use std::net::SocketAddr;
use std::time::Duration;

use futures::{ Future, Stream, Sink };
use futures::sync::mpsc::{ Receiver, Sender, channel };
use protobuf::Chars;
use tokio::runtime::{ Runtime, Shutdown };

use crate::discovery;
use crate::internal::driver::{ Driver, Report };
use crate::internal::messaging::Msg;
use crate::internal::commands;
use crate::internal::operations::OperationError;
use crate::types::{ self, StreamMetadata, Settings, GossipSeedClusterSettings };

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
pub struct Connection {
    shutdown: Option<Shutdown>,
    sender: Sender<Msg>,
    settings: Settings,
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
        where S: Into<protobuf::Chars>
    {
        self.settings.connection_name = Some(name.into());
        self
    }

    /// The period used to check pending command. Those checks include if the
    /// the connection has timeout or if the command was issued with a
    /// different connection.
    pub fn operation_check_period(mut self, period: Duration) -> Self {
        self.settings.operation_check_period = period;
        self
    }

    /// Creates a connection to a single EventStore node. The connection will
    /// start right away.
    pub fn single_node_connection(self, addr: SocketAddr) -> Connection {
        self.start_common_with_runtime(DiscoveryProcess::Static(addr), None)
    }

    /// Creates a connection to a single EventStore node. The connection will
    /// start right away.
    pub fn single_node_connection_with_runtime(self, addr: SocketAddr, runtime: &mut Runtime) -> Connection {
        self.start_common_with_runtime(DiscoveryProcess::Static(addr), Some(runtime))
    }

    /// Creates a connection to a cluster of EventStore nodes. The connection will
    /// start right away. Those `GossipSeed` should be the external HTTP endpoint
    /// of a node. The standard external HTTP endpoint is running on `2113`.
    pub fn cluster_nodes_through_gossip_connection(self, setts: GossipSeedClusterSettings)
        -> Connection
    {
        self.start_common_with_runtime(DiscoveryProcess::ClusterThroughGossip(setts), None)
    }

    /// Creates a connection to a cluster of EventStore nodes. The connection will
    /// start right away. Those `GossipSeed` should be the external HTTP endpoint
    /// of a node. The standard external HTTP endpoint is running on `2113`.
    pub fn cluster_nodes_through_gossip_connection_with_runtime(self, setts: GossipSeedClusterSettings, runtime: &mut Runtime)
        -> Connection
    {
        self.start_common_with_runtime(DiscoveryProcess::ClusterThroughGossip(setts), Some(runtime))
    }

    fn start_common_with_runtime(self, discovery: DiscoveryProcess, runtime_opt: Option<&mut Runtime>) -> Connection {
        let client = Connection::with_runtime(self.settings, discovery, runtime_opt);

        client.start();

        client
    }
}

const DEFAULT_BOX_SIZE: usize = 500;

fn connection_state_machine(sender: Sender<Msg>, recv: Receiver<Msg>, mut driver: Driver)
    -> impl Future<Item=(), Error=()>
{
    enum State {
        Live,
        Clearing,
    }

    fn start_closing<E>(sender: &Sender<Msg>, driver: &mut Driver)
        -> Result<State, E>
    {
        driver.close_connection();

        let action = sender
            .clone()
            .send(Msg::Marker)
            .map(|_| ())
            .map_err(|_| ());

        tokio::spawn(action);

        info!("Closing the connection...");
        info!("Start clearing uncomplete operations...");

        Ok(State::Clearing)
    }

    recv.fold(State::Live, move |acc, msg| {
        if let State::Live = &acc {
            match msg {
                Msg::Start => driver.start(),
                Msg::Establish(endpoint) => driver.on_establish(endpoint),
                Msg::Established(id)  => driver.on_established(id),
                Msg::ConnectionClosed(conn_id, error) => driver.on_connection_closed(conn_id, &error),
                Msg::Arrived(pkg) => driver.on_package_arrived(pkg),
                Msg::NewOp(op) => driver.on_new_op(op),
                Msg::Send(pkg) => driver.on_send_pkg(pkg),

                Msg::Tick => {
                    if let Report::Quit = driver.on_tick() {
                        return start_closing(&sender, &mut driver);
                    }
                },

                // It's impossible to receive `Msg::Marker` at `State::Live` state.
                // However we can hit two birds with one stone with pattern-matching
                // coverage checker.
                Msg::Shutdown | Msg::Marker => {
                    info!("User-shutdown request received.");

                    return start_closing(&sender, &mut driver);
                },
            }
        } else {
            match msg {
                Msg::NewOp(mut op) => op.failed(OperationError::Aborted),
                Msg::Arrived(pkg) => driver.on_package_arrived(pkg),
                Msg::Marker => {
                    // We've reached the end of our checkpoint, we can properly
                    // aborts uncompleted operations.
                    driver.abort();
                    info!("Connection closed properly.");

                    return Err(());
                },

                _ => {},
            }
        }

        Ok(acc)
    }).map(|_| ())
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

    fn with_runtime(settings: Settings, discovery: DiscoveryProcess, runtime_opt: Option<&mut Runtime>)
        -> Connection
    {
        if let Some(runtime) = runtime_opt {
            let sender = Self::initialize(&settings, discovery, runtime);

            Connection {
                shutdown: None,
                sender,
                settings,
            }
        } else {
            let mut runtime = Runtime::new().unwrap();
            let sender = Self::initialize(&settings, discovery, &mut runtime);
            let shutdown = runtime.shutdown_on_idle();

            Connection {
                shutdown: Some(shutdown),
                sender,
                settings,
            }
        }
    }

    fn initialize(settings: &Settings, discovery: DiscoveryProcess, runtime: &mut Runtime) -> Sender<Msg> {
        let (sender, recv) = channel(DEFAULT_BOX_SIZE);
        let (start_discovery, run_discovery) = channel(DEFAULT_BOX_SIZE);
        let cloned_sender = sender.clone();
        let driver = Driver::new(&settings, start_discovery, sender.clone());

        match discovery {
            DiscoveryProcess::Static(addr) => {
                let endpoint = types::Endpoint::from_addr(addr);
                let action = discovery::constant::discover(run_discovery, sender.clone(), endpoint);

                runtime.spawn(action);
            },

            DiscoveryProcess::ClusterThroughGossip(setts) => {
                let action = discovery::cluster::discover(run_discovery, sender.clone(), setts);

                runtime.spawn(action);
            },
        };

        let action = connection_state_machine(cloned_sender, recv, driver);
        let _ = runtime.spawn(action);
        sender
    }

    fn start(&self) {
        self.sender.clone().send(Msg::Start).wait().unwrap();
    }

    /// Sends events to a given stream.
    pub fn write_events<S>(&self, stream: S) -> commands::WriteEvents
        where S: Into<Chars>
    {
        commands::WriteEvents::new(self.sender.clone(), stream, &self.settings)
    }

    /// Sets the metadata for a stream.
    pub fn write_stream_metadata<S>(&self, stream: S, metadata: StreamMetadata)
        -> commands::WriteStreamMetadata
        where S: Into<Chars>
    {
        commands::WriteStreamMetadata::new(self.sender.clone(), stream, metadata, &self.settings)
    }

    /// Reads a single event from a given stream.
    pub fn read_event<S>(&self, stream: S, event_number: i64) -> commands::ReadEvent
        where S: Into<Chars>
    {
        commands::ReadEvent::new(self.sender.clone(), stream, event_number, &self.settings)
    }

    /// Gets the metadata of a stream.
    pub fn read_stream_metadata<S>(&self, stream: S) -> commands::ReadStreamMetadata
        where S: Into<Chars>
    {
        commands::ReadStreamMetadata::new(self.sender.clone(), stream, &self.settings)
    }

    /// Starts a transaction on a given stream.
    pub fn start_transaction<S>(&self, stream: S) -> commands::TransactionStart
        where S: Into<Chars>
    {
        commands::TransactionStart::new(self.sender.clone(), stream, &self.settings)
    }

    /// Reads events from a given stream. The reading can be done forward and
    /// backward.
    pub fn read_stream<S>(&self, stream: S) -> commands::ReadStreamEvents
        where S: Into<Chars>
    {
        commands::ReadStreamEvents::new(self.sender.clone(), stream, &self.settings)
    }

    /// Reads events for the system stream `$all`. The reading can be done
    /// forward and backward.
    pub fn read_all(&self) -> commands::ReadAllEvents {
        commands::ReadAllEvents::new(self.sender.clone(), &self.settings)
    }

    /// Deletes a given stream. By default, the server performs a soft delete,
    /// More information can be found on the [Deleting streams and events]
    /// page.
    ///
    /// [Deleting stream and events]: https://eventstore.org/docs/server/deleting-streams-and-events/index.html
    pub fn delete_stream<S>(&self, stream: S) -> commands::DeleteStream
        where S: Into<Chars>
    {
        commands::DeleteStream::new(self.sender.clone(), stream, &self.settings)
    }

    /// Subscribes to a given stream. You will get notified of each new events
    /// written to this stream.
    pub fn subcribe_to_stream<S>(&self, stream_id: S) -> commands::SubscribeToStream
        where S: Into<Chars>
    {
        commands::SubscribeToStream::new(self.sender.clone(), stream_id, &self.settings)
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
        where S: Into<Chars>
    {
        commands::RegularCatchupSubscribe::new(self.sender.clone(), stream, &self.settings)
    }

    /// Like [`subscribe_to_stream_from`] but specific to system `$all` stream.
    ///
    /// [`subscribe_to_stream_from`]: #method.subscribe_to_stream_from
    pub fn subscribe_to_all_from(&self) -> commands::AllCatchupSubscribe
    {
        commands::AllCatchupSubscribe::new(self.sender.clone(), &self.settings)
    }

    /// Creates a persistent subscription group on a stream.
    ///
    /// Persistent subscriptions are special kind of subscription where the
    /// server remembers the state of the subscription. This allows for many
    /// different modes of operations compared to a regular or catchup
    /// subscription where the client holds the subscription state.
    pub fn create_persistent_subscription<S>(&self, stream_id: S, group_name: S) -> commands::CreatePersistentSubscription
        where S: Into<Chars>
    {
        commands::CreatePersistentSubscription::new(stream_id, group_name, self.sender.clone(), &self.settings)
    }

    /// Updates a persistent subscription group on a stream.
    pub fn update_persistent_subscription<S>(&self, stream_id: S, group_name: S) -> commands::UpdatePersistentSubscription
        where S: Into<Chars>
    {
        commands::UpdatePersistentSubscription::new(stream_id, group_name, self.sender.clone(), &self.settings)
    }

    /// Deletes a persistent subscription group on a stream.
    pub fn delete_persistent_subscription<S>(&self, stream_id: S, group_name: S) -> commands::DeletePersistentSubscription
        where S: Into<Chars>
    {
        commands::DeletePersistentSubscription::new(stream_id, group_name, self.sender.clone(), &self.settings)
    }

    /// Connects to a persistent subscription group on a stream.
    pub fn connect_persistent_subscription<S>(&self, stream_id: S, group_name: S) -> commands::ConnectToPersistentSubscription
        where S: Into<Chars>
    {
        commands::ConnectToPersistentSubscription::new(stream_id, group_name, self.sender.clone(), &self.settings)
    }

    /// Closes the connection to the server.
    ///
    /// When closing a connection, a `Connection` might have ongoing operations
    /// running. `shutdown` makes sure the `Connection` has handled
    /// everything properly when returning.
    ///
    /// `shutdown` blocks the current thread.
    pub fn shutdown(self) {
        self.sender.send(Msg::Shutdown).wait().unwrap();
        self.shutdown.wait().unwrap();
    }
}
