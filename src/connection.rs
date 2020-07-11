use std::net::SocketAddr;
use std::time::Duration;

use futures::channel::mpsc::{channel, Receiver, Sender};
use futures::sink::SinkExt;
use futures::stream::StreamExt;

use crate::discovery;
use crate::internal::commands;
use crate::internal::driver::{Driver, Report};
use crate::internal::messaging::{Msg, OpMsg};
use crate::types::{self, GossipSeedClusterSettings, OperationError, Settings, StreamMetadata};

/// Represents a connection to a single node. `Client` maintains a full duplex
/// connection to the EventStore server. An EventStore connection operates
/// quite differently than say a SQL connection. Normally when you use an
/// EventStore connection you want to keep the connection open for a much
/// longer of time than when you use a SQL connection.
///
/// Another difference is that with the EventStore connection, all operations
/// are handled in a full async manner (even if you call the synchronous
/// behaviors). Many threads can use an EventStore connection at the same time
/// or a single thread can make many asynchronous requests. To get the most
/// performance out of the connection, it is generally recommended to use it
/// in this way.
#[derive(Clone)]
pub struct Connection {
    sender: Sender<Msg>,
}

/// Helps constructing a connection to the server.
pub struct ConnectionBuilder {
    pub settings: Settings,
}

impl ConnectionBuilder {
    /// Maximum delay of inactivity before the client sends a heartbeat request.
    pub fn heartbeat_delay(mut self, delay: Duration) -> Self {
        self.settings.heartbeat_delay = delay;
        self
    }

    /// Maximum delay the server has to issue a heartbeat response.
    pub fn heartbeat_timeout(mut self, timeout: Duration) -> Self {
        self.settings.heartbeat_timeout = timeout;
        self
    }

    /// Delay in which an operation will be retried if no response arrived.
    pub fn operation_timeout(mut self, timeout: Duration) -> Self {
        self.settings.operation_timeout = timeout;
        self
    }

    /// Retry strategy when an operation has timeout.
    pub fn operation_retry(mut self, strategy: types::Retry) -> Self {
        self.settings.operation_retry = strategy;
        self
    }

    /// Retry strategy when failing to connect.
    pub fn connection_retry(mut self, strategy: types::Retry) -> Self {
        self.settings.connection_retry = strategy;
        self
    }

    /// 'Credentials' to use if other `Credentials` are not explicitly supplied
    /// when issuing commands.
    pub fn with_default_user(mut self, user: types::Credentials) -> Self {
        self.settings.default_user = Some(user);
        self
    }

    /// Default connection name.
    pub fn with_connection_name<S>(mut self, name: S) -> Self
    where
        S: AsRef<str>,
    {
        self.settings.connection_name = Some(name.as_ref().to_owned());
        self
    }

    /// The period used to check pending command. Those checks include if the
    /// the connection has timeout or if the command was issued with a
    /// different connection.
    pub fn operation_check_period(mut self, period: Duration) -> Self {
        self.settings.operation_check_period = period;
        self
    }

    /// Maximum delay to create a successful connection to a node.
    pub fn connection_timeout(mut self, period: Duration) -> Self {
        self.settings.connection_timeout = period;
        self
    }

    /// Maximum delay to physically connect to a node. This property differs from
    /// `connection_timeout` by referencing the delay to have a connected socket to a node, whereas
    /// `connection_timeout` refers to the whole connection, validation included.
    pub fn socket_connection_timeout(mut self, period: Duration) -> Self {
        self.settings.socket_connection_timeout = period;
        self
    }

    #[cfg(feature = "tls")]
    /// Enable secure connection with the server/cluster.
    pub fn enable_secure_connection(mut self, config: crate::SecureSettings) -> Self {
        self.settings.tls_client_config = Some(config);
        self
    }

    /// Creates a connection to a single EventStore node. The connection will
    /// start right away.
    pub async fn single_node_connection(self, addr: SocketAddr) -> Connection {
        self.start_common_with_runtime(DiscoveryProcess::Static(addr))
            .await
    }

    /// Creates a connection to a cluster of EventStore nodes. The connection will
    /// start right away. Those `GossipSeed` should be the external HTTP endpoint
    /// of a node. The standard external HTTP endpoint is running on `2113`.
    pub async fn cluster_nodes_through_gossip_connection(
        self,
        setts: GossipSeedClusterSettings,
    ) -> Connection {
        self.start_common_with_runtime(DiscoveryProcess::ClusterThroughGossip(setts))
            .await
    }

    async fn start_common_with_runtime(self, discovery: DiscoveryProcess) -> Connection {
        let mut client = Connection::make(self.settings, discovery);

        client.start().await;

        client
    }
}

const DEFAULT_BOX_SIZE: usize = 500;

async fn connection_state_machine(
    mut sender: Sender<Msg>,
    mut recv: Receiver<Msg>,
    mut driver: Driver,
) {
    async fn closing(sender: &mut Sender<Msg>, driver: &mut Driver) {
        driver.close_connection();
        let _ = sender.send(Msg::Marker).await;

        info!("Closing the connection...");
        info!("Start clearing uncomplete operations...");
    }

    // Live state
    while let Some(msg) = recv.next().await {
        match msg {
            Msg::Start => driver.start().await,
            Msg::Establish(endpoint) => driver.on_establish(endpoint),
            Msg::Established(id) => driver.on_established(id).await,
            Msg::ConnectionClosed(conn_id, error) => driver.on_connection_closed(conn_id, &error),
            Msg::Arrived(pkg) => driver.on_package_arrived(pkg).await,
            Msg::Transmit(pkg, mailbox) => driver.on_transmit(mailbox, pkg).await,
            Msg::Send(pkg) => driver.on_send_pkg(pkg).await,

            Msg::Tick => {
                if let Report::Quit = driver.on_tick().await {
                    closing(&mut sender, &mut driver).await;
                    break;
                }
            }

            // It's impossible to receive `Msg::Marker` at `State::Live` state.
            // However we can hit two birds with one stone with pattern-matching
            // coverage checker.
            Msg::Shutdown | Msg::Marker => {
                info!("User-shutdown request received.");
                closing(&mut sender, &mut driver).await;
                break;
            }
        }
    }

    // Closing state
    while let Some(msg) = recv.next().await {
        match msg {
            Msg::Transmit(_, mut mailbox) => {
                let _ = mailbox.send(OpMsg::Failed(OperationError::Aborted)).await;
            }

            Msg::Arrived(pkg) => driver.on_package_arrived(pkg).await,
            Msg::Marker => {
                // We've reached the end of our checkpoint, we can properly
                // aborts uncompleted operations.
                driver.abort().await;
                info!("Connection closed properly.");

                break;
            }

            _ => {}
        }
    }
}

enum DiscoveryProcess {
    Static(SocketAddr),
    ClusterThroughGossip(GossipSeedClusterSettings),
}

impl Connection {
    /// Return a connection builder.
    pub fn builder() -> ConnectionBuilder {
        ConnectionBuilder {
            settings: Default::default(),
        }
    }

    fn make(settings: Settings, discovery: DiscoveryProcess) -> Connection {
        let sender = Self::initialize(settings, discovery);

        Connection { sender }
    }

    fn initialize(settings: Settings, discovery: DiscoveryProcess) -> Sender<Msg> {
        let (sender, recv) = channel(DEFAULT_BOX_SIZE);
        let (start_discovery, run_discovery) = futures::channel::mpsc::channel(DEFAULT_BOX_SIZE);
        let cloned_sender = sender.clone();

        match discovery {
            DiscoveryProcess::Static(addr) => {
                let endpoint = types::Endpoint::from_addr(addr);
                let action = discovery::constant::discover(run_discovery, sender.clone(), endpoint);

                tokio::spawn(action);
            }

            DiscoveryProcess::ClusterThroughGossip(setts) => {
                #[cfg(feature = "tls")]
                {
                    let secure_mode = settings.tls_client_config.is_some();
                    let action = discovery::cluster::discover(
                        run_discovery,
                        sender.clone(),
                        setts,
                        secure_mode,
                    );

                    tokio::spawn(action);
                }
                #[cfg(not(feature = "tls"))]
                {
                    let action =
                        discovery::cluster::discover(run_discovery, sender.clone(), setts, false);

                    tokio::spawn(action);
                }
            }
        };

        let driver = Driver::new(settings, start_discovery, sender.clone());

        tokio::spawn(connection_state_machine(cloned_sender, recv, driver));
        sender
    }

    async fn start(&mut self) {
        let _ = self.sender.send(Msg::Start).await;
    }

    /// Sends events to a given stream.
    pub fn write_events<S>(&self, stream: S) -> commands::WriteEvents
    where
        S: AsRef<str>,
    {
        commands::WriteEvents::new(self.sender.clone(), stream)
    }

    /// Sets the metadata for a stream.
    pub fn write_stream_metadata<S>(
        &self,
        stream: S,
        metadata: StreamMetadata,
    ) -> commands::WriteStreamMetadata
    where
        S: AsRef<str>,
    {
        commands::WriteStreamMetadata::new(self.sender.clone(), stream, metadata)
    }

    /// Reads a single event from a given stream.
    pub fn read_event<S>(&self, stream: S, event_number: i64) -> commands::ReadEvent
    where
        S: AsRef<str>,
    {
        commands::ReadEvent::new(self.sender.clone(), stream, event_number)
    }

    /// Gets the metadata of a stream.
    pub fn read_stream_metadata<S>(&self, stream: S) -> commands::ReadStreamMetadata
    where
        S: AsRef<str>,
    {
        commands::ReadStreamMetadata::new(self.sender.clone(), stream)
    }

    /// Starts a transaction on a given stream.
    pub fn start_transaction<S>(&self, stream: S) -> commands::TransactionStart
    where
        S: AsRef<str>,
    {
        commands::TransactionStart::new(self.sender.clone(), stream)
    }

    /// Reads events from a given stream. The reading can be done forward and
    /// backward.
    pub fn read_stream<S>(&self, stream: S) -> commands::ReadStreamEvents
    where
        S: AsRef<str>,
    {
        commands::ReadStreamEvents::new(self.sender.clone(), stream)
    }

    /// Reads events for the system stream `$all`. The reading can be done
    /// forward and backward.
    pub fn read_all(&self) -> commands::ReadAllEvents {
        commands::ReadAllEvents::new(self.sender.clone())
    }

    /// Deletes a given stream. By default, the server performs a soft delete,
    /// More information can be found on the [Deleting streams and events]
    /// page.
    ///
    /// [Deleting stream and events]: https://eventstore.org/docs/server/deleting-streams-and-events/index.html
    pub fn delete_stream<S>(&self, stream: S) -> commands::DeleteStream
    where
        S: AsRef<str>,
    {
        commands::DeleteStream::new(self.sender.clone(), stream)
    }

    /// Subscribes to a given stream. You will get notified of each new events
    /// written to this stream.
    pub fn subcribe_to_stream<S>(&self, stream_id: S) -> commands::SubscribeToStream
    where
        S: AsRef<str>,
    {
        commands::SubscribeToStream::new(self.sender.clone(), stream_id)
    }

    /// Subscribes to a given stream. This kind of subscription specifies a
    /// starting point (by default, the beginning of a stream). For a regular
    /// stream, that starting point will be an event number. For the system
    /// stream `$all`, it will be a position in the transaction file
    /// (see [`subscribe_to_all_from`]). This subscription will fetch every event
    /// until the end of the stream, then will dispatch subsequently written
    /// events.
    ///
    /// For example, if a starting point of 50 is specified when a stream has
    /// 100 events in it, the subscriber can expect to see events 51 through
    /// 100, and then any events subsequenttly written events until such time
    /// as the subscription is dropped or closed.
    ///
    /// [`subscribe_to_all_from`]: #method.subscribe_to_all_from
    pub fn subscribe_to_stream_from<S>(&self, stream: S) -> commands::RegularCatchupSubscribe
    where
        S: AsRef<str>,
    {
        commands::RegularCatchupSubscribe::new(self.sender.clone(), stream)
    }

    /// Like [`subscribe_to_stream_from`] but specific to system `$all` stream.
    ///
    /// [`subscribe_to_stream_from`]: #method.subscribe_to_stream_from
    pub fn subscribe_to_all_from(&self) -> commands::AllCatchupSubscribe {
        commands::AllCatchupSubscribe::new(self.sender.clone())
    }

    /// Creates a persistent subscription group on a stream.
    ///
    /// Persistent subscriptions are special kind of subscription where the
    /// server remembers the state of the subscription. This allows for many
    /// different modes of operations compared to a regular or catchup
    /// subscription where the client holds the subscription state.
    pub fn create_persistent_subscription<S>(
        &self,
        stream_id: S,
        group_name: S,
    ) -> commands::CreatePersistentSubscription
    where
        S: AsRef<str>,
    {
        commands::CreatePersistentSubscription::new(stream_id, group_name, self.sender.clone())
    }

    /// Updates a persistent subscription group on a stream.
    pub fn update_persistent_subscription<S>(
        &self,
        stream_id: S,
        group_name: S,
    ) -> commands::UpdatePersistentSubscription
    where
        S: AsRef<str>,
    {
        commands::UpdatePersistentSubscription::new(stream_id, group_name, self.sender.clone())
    }

    /// Deletes a persistent subscription group on a stream.
    pub fn delete_persistent_subscription<S>(
        &self,
        stream_id: S,
        group_name: S,
    ) -> commands::DeletePersistentSubscription
    where
        S: AsRef<str>,
    {
        commands::DeletePersistentSubscription::new(stream_id, group_name, self.sender.clone())
    }

    /// Connects to a persistent subscription group on a stream.
    pub fn connect_persistent_subscription<S>(
        &self,
        stream_id: S,
        group_name: S,
    ) -> commands::ConnectToPersistentSubscription
    where
        S: AsRef<str>,
    {
        commands::ConnectToPersistentSubscription::new(stream_id, group_name, self.sender.clone())
    }

    /// Closes the connection to the server.
    ///
    /// When closing a connection, a `Connection` might have ongoing operations
    /// running. `shutdown` makes sure the `Connection` has handled
    /// everything properly when returning.
    ///
    /// `shutdown` blocks the current thread.
    pub async fn shutdown(mut self) {
        let _ = self.sender.send(Msg::Shutdown).await;
    }
}
