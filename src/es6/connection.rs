use std::time::Duration;

use crate::es6::commands::{self, persistent, streams};
use crate::types::{self, Settings};

use http::uri::Uri;
use tonic::transport::Channel;

struct NoVerification;

impl rustls::ServerCertVerifier for NoVerification {
    fn verify_server_cert(
        &self,
        _roots: &rustls::RootCertStore,
        _presented_certs: &[rustls::Certificate],
        _dns_name: webpki::DNSNameRef,
        _ocsp_response: &[u8],
    ) -> Result<rustls::ServerCertVerified, rustls::TLSError> {
        Ok(rustls::ServerCertVerified::assertion())
    }
}

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
    settings: Settings,
    streams: streams::streams_client::StreamsClient<Channel>,
    persistent: persistent::persistent_subscriptions_client::PersistentSubscriptionsClient<Channel>,
}

/// Helps constructing a connection to the server.
pub struct ConnectionBuilder {
    pub settings: Settings,
    disable_certs_validation: bool,
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

    /// Disable the use of certificate validation.
    ///
    /// # Warning
    ///
    /// You should think very carefully before using this method. If
    /// invalid certificates are trusted, *any* certificate for *any* site
    /// will be trusted for use. This includes expired certificates. This
    /// introduces significant vulnerabilities, and should only be used
    /// as a last resort.
    pub fn disable_server_certificate_validation(mut self) -> Self {
        self.disable_certs_validation = true;
        self
    }

    /// Creates a connection to a single EventStore node. The connection will
    /// start right away.
    pub async fn single_node_connection(
        self,
        uri: Uri,
    ) -> Result<Connection, Box<dyn std::error::Error>> {
        Connection::initialize(self.settings, self.disable_certs_validation, uri).await
    }
}

impl Connection {
    /// Return a connection builder.
    pub fn builder() -> ConnectionBuilder {
        ConnectionBuilder {
            settings: Default::default(),
            disable_certs_validation: false,
        }
    }

    async fn initialize(
        settings: Settings,
        disable_certs_validation: bool,
        uri: http::uri::Uri,
    ) -> Result<Connection, Box<dyn std::error::Error>> {
        let mut channel = Channel::builder(uri);

        if disable_certs_validation {
            let mut rustls_config = rustls::ClientConfig::new();
            let protocols = vec![(b"h2".to_vec())];

            rustls_config.set_protocols(protocols.as_slice());

            rustls_config
                .dangerous()
                .set_certificate_verifier(std::sync::Arc::new(NoVerification));

            let client_config =
                tonic::transport::ClientTlsConfig::new().rustls_client_config(rustls_config);

            channel = channel.tls_config(client_config);
        }

        let channel = channel.connect().await?;

        let conn = Connection{
            settings,
            streams: streams::streams_client::StreamsClient::new(channel.clone()),
            persistent: persistent::persistent_subscriptions_client::PersistentSubscriptionsClient::new(channel),
        };

        Ok(conn)
    }

    /// Sends events to a given stream.
    pub fn write_events(&self, stream: String) -> commands::WriteEvents {
        commands::WriteEvents::new(
            self.streams.clone(),
            stream,
            self.settings.default_user.clone(),
        )
    }

    /// Reads events from a given stream. The reading can be done forward and
    /// backward.
    pub fn read_stream(&self, stream: String) -> commands::ReadStreamEvents {
        commands::ReadStreamEvents::new(
            self.streams.clone(),
            stream,
            self.settings.default_user.clone(),
        )
    }

    /// Reads events for the system stream `$all`. The reading can be done
    /// forward and backward.
    pub fn read_all(&self) -> commands::ReadAllEvents {
        commands::ReadAllEvents::new(self.streams.clone(), self.settings.default_user.clone())
    }

    /// Deletes a given stream. By default, the server performs a soft delete,
    /// More information can be found on the [Deleting streams and events]
    /// page.
    ///
    /// [Deleting stream and events]: https://eventstore.org/docs/server/deleting-streams-and-events/index.html
    pub fn delete_stream(&self, stream: String) -> commands::DeleteStream {
        commands::DeleteStream::new(
            self.streams.clone(),
            stream,
            self.settings.default_user.clone(),
        )
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
    pub fn subscribe_to_stream_from(&self, stream: String) -> commands::RegularCatchupSubscribe {
        commands::RegularCatchupSubscribe::new(
            self.streams.clone(),
            stream,
            self.settings.default_user.clone(),
        )
    }

    /// Like [`subscribe_to_stream_from`] but specific to system `$all` stream.
    ///
    /// [`subscribe_to_stream_from`]: #method.subscribe_to_stream_from
    pub fn subscribe_to_all_from(&self) -> commands::AllCatchupSubscribe {
        commands::AllCatchupSubscribe::new(self.streams.clone(), self.settings.default_user.clone())
    }

    /// Creates a persistent subscription group on a stream.
    ///
    /// Persistent subscriptions are special kind of subscription where the
    /// server remembers the state of the subscription. This allows for many
    /// different modes of operations compared to a regular or catchup
    /// subscription where the client holds the subscription state.
    pub fn create_persistent_subscription(
        &self,
        stream_id: String,
        group_name: String,
    ) -> commands::CreatePersistentSubscription {
        commands::CreatePersistentSubscription::new(
            self.persistent.clone(),
            stream_id,
            group_name,
            self.settings.default_user.clone(),
        )
    }

    /// Updates a persistent subscription group on a stream.
    pub fn update_persistent_subscription(
        &self,
        stream_id: String,
        group_name: String,
    ) -> commands::UpdatePersistentSubscription {
        commands::UpdatePersistentSubscription::new(
            self.persistent.clone(),
            stream_id,
            group_name,
            self.settings.default_user.clone(),
        )
    }

    /// Deletes a persistent subscription group on a stream.
    pub fn delete_persistent_subscription(
        &self,
        stream_id: String,
        group_name: String,
    ) -> commands::DeletePersistentSubscription {
        commands::DeletePersistentSubscription::new(
            self.persistent.clone(),
            stream_id,
            group_name,
            self.settings.default_user.clone(),
        )
    }

    /// Connects to a persistent subscription group on a stream.
    pub fn connect_persistent_subscription(
        &self,
        stream_id: String,
        group_name: String,
    ) -> commands::ConnectToPersistentSubscription {
        commands::ConnectToPersistentSubscription::new(
            self.persistent.clone(),
            stream_id,
            group_name,
            self.settings.default_user.clone(),
        )
    }

    /// Closes the connection to the server.
    ///
    /// When closing a connection, a `Connection` might have ongoing operations
    /// running. `shutdown` makes sure the `Connection` has handled
    /// everything properly when returning.
    ///
    /// `shutdown` blocks the current thread.
    pub fn shutdown(self) {}
}
