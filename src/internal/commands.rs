//! Commands this client supports.
use std::collections::HashMap;
use std::ops::Deref;

use futures::sync::mpsc::{ self, Sender };
use futures::{ Future, Sink, Stream };
use protobuf::Chars;
use serde_json;

use internal::messaging::Msg;
use internal::operations::{ self, OperationError };
use internal::timespan::Timespan;
use types;

fn single_value_future<S, A>(stream: S) -> impl Future<Item=A, Error=OperationError>
    where S: Stream<Item = Result<A, operations::OperationError>, Error = ()>
{
    stream.into_future().then(|res| {
        match res {
            Ok((Some(x), _)) => x,
            _ => {
                warn!("Operation stream-receiver was disposed too early. \
                      It shouldn't happen but not a big of a deal neither. \
                      Worth investigating though as it means the code \
                      went on unexpected path.");

                Err(operations::OperationError::Aborted)
            },
        }
    })
}

/// Command that sends events to a given stream.
pub struct WriteEvents<'a> {
    stream: Chars,
    events: Vec<types::EventData>,
    require_master: bool,
    version: types::ExpectedVersion,
    creds: Option<types::Credentials>,
    settings: &'a types::Settings,
    pub(crate) sender: Sender<Msg>,
}

impl <'a> WriteEvents<'a> {
    pub(crate) fn new<S>(sender: Sender<Msg>, stream: S, settings: &types::Settings) -> WriteEvents
        where S: Into<Chars>
    {
        WriteEvents {
            stream: stream.into(),
            events: Vec::new(),
            require_master: false,
            version: types::ExpectedVersion::Any,
            creds: None,
            settings,
            sender,
        }
    }

    /// Sets events to write in the command. This function will replace
    /// previously added events.
    pub fn set_events(self, events: Vec<types::EventData>) -> WriteEvents<'a> {
        WriteEvents { events, ..self}
    }

    /// Adds an event to the current list of events to send to the server.
    pub fn push_event(mut self, event: types::EventData) -> WriteEvents<'a> {
        self.events.push(event);

        self
    }

    /// Extends the current set of events to send the the server with the
    /// given iterator.
    pub fn append_events<T>(mut self, events: T) -> WriteEvents<'a>
        where T: IntoIterator<Item=types::EventData>
    {
        self.events.extend(events);

        self
    }

    /// Asks the server receiving the command to be the master of the cluster
    /// in order to perform the write. Default: `false`.
    pub fn require_master(self, require_master: bool) -> WriteEvents<'a> {
        WriteEvents { require_master, ..self }
    }

    /// Asks the server to check that the stream receiving the event is at
    /// the given expected version. Default: `types::ExpectedVersion::Any`.
    pub fn expected_version(self, version: types::ExpectedVersion) -> WriteEvents<'a> {
        WriteEvents { version, ..self }
    }

    /// Performs the command with the given credentials.
    pub fn credentials(self, creds: types::Credentials) -> WriteEvents<'a> {
        WriteEvents { creds: Some(creds), ..self }
    }

    /// Sends asynchronously the write command to the server.
    pub fn execute(self) -> impl Future<Item=types::WriteResult, Error=OperationError> {
        let     (rcv, promise) = operations::Promise::new(1);
        let mut op             = operations::WriteEvents::new(promise);

        op.set_event_stream_id(self.stream);
        op.set_expected_version(self.version);
        op.set_events(self.events);
        op.set_require_master(self.require_master);

        let op = operations::OperationWrapper::new(op,
                                                   self.creds,
                                                   self.settings.operation_retry.to_usize(),
                                                   self.settings.operation_timeout);

        self.sender.send(Msg::new_op(op)).wait().unwrap();

        single_value_future(rcv)
    }
}

/// Command that reads an event from a given stream.
pub struct ReadEvent<'a> {
    stream: Chars,
    event_number: i64,
    resolve_link_tos: bool,
    require_master: bool,
    creds: Option<types::Credentials>,
    settings: &'a types::Settings,
    pub(crate) sender: Sender<Msg>,
}

impl <'a> ReadEvent<'a> {
    pub(crate) fn new<S>(
        sender: Sender<Msg>,
        stream: S,
        event_number: i64,
        settings: &types::Settings) -> ReadEvent
        where S: Into<Chars>
    {
        ReadEvent {
            stream: stream.into(),
            event_number,
            sender,
            resolve_link_tos: false,
            require_master: false,
            creds: None,
            settings,
        }
    }

    /// When using projections, you can have links placed into another stream.
    /// If you set `true`, the server will resolve those links and will return
    /// the event that the link points to. Default: [NoResolution](../types/enum.LinkTos.html).
    pub fn resolve_link_tos(self, tos: types::LinkTos) -> ReadEvent<'a> {
        let resolve_link_tos = tos.raw_resolve_lnk_tos();

        ReadEvent { resolve_link_tos, ..self }
    }

    /// Asks the server receiving the command to be the master of the cluster
    /// in order to perform the write. Default: `false`.
    pub fn require_master(self, require_master: bool) -> ReadEvent<'a> {
        ReadEvent { require_master, ..self }
    }

    /// Performs the command with the given credentials.
    pub fn credentials(self, value: types::Credentials) -> ReadEvent<'a> {
        ReadEvent { creds: Some(value), ..self }
    }

    /// Sends asynchronously the read command to the server.
    pub fn execute(self) -> impl Future<Item=types::ReadEventStatus<types::ReadEventResult>, Error=OperationError> {
        let (rcv, promise) = operations::Promise::new(1);
        let mut op         = operations::ReadEvent::new(promise);

        op.set_event_stream_id(self.stream);
        op.set_event_number(self.event_number);
        op.set_resolve_link_tos(self.resolve_link_tos);
        op.set_require_master(self.require_master);

        let op = operations::OperationWrapper::new(op,
                                                   self.creds,
                                                   self.settings.operation_retry.to_usize(),
                                                   self.settings.operation_timeout);

        self.sender.send(Msg::new_op(op)).wait().unwrap();

        single_value_future(rcv)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct StreamMetadataInternal {
    #[serde(rename = "$maxCount")]
    max_count: Option<u64>,

    #[serde(rename = "$maxAge")]
    max_age: Option<Timespan>,

    #[serde(rename = "$tb")]
    truncate_before: Option<u64>,

    #[serde(rename = "$cacheControl")]
    cache_control: Option<Timespan>,

    #[serde(rename = "$acl")]
    acl: StreamAclInternal,

    #[serde(flatten)]
    custom_properties: HashMap<String, serde_json::Value>,
}

impl StreamMetadataInternal {
    fn from_metadata(metadata: types::StreamMetadata) -> StreamMetadataInternal {
        StreamMetadataInternal {
            max_count: metadata.max_count,
            max_age: metadata.max_age.map(Timespan::from_duration),
            truncate_before: metadata.truncate_before,
            cache_control: metadata.cache_control.map(Timespan::from_duration),
            acl: StreamAclInternal::from_acl(metadata.acl),
            custom_properties: metadata.custom_properties,
        }
    }

    fn build_metadata(self) -> types::StreamMetadata {
        types::StreamMetadata {
            max_count: self.max_count,
            max_age: self.max_age.map(|t| t.build_duration()),
            truncate_before: self.truncate_before,
            cache_control: self.cache_control.map(|t| t.build_duration()),
            acl: self.acl.build_acl(),
            custom_properties: self.custom_properties,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct StreamAclInternal {
    #[serde(rename = "$r")]
    read_roles: Vec<String>,

    #[serde(rename = "$w")]
    write_roles: Vec<String>,

    #[serde(rename = "$d")]
    delete_roles: Vec<String>,

    #[serde(rename = "$mr")]
    meta_read_roles: Vec<String>,

    #[serde(rename = "$mw")]
    meta_write_roles: Vec<String>,
}

impl StreamAclInternal {
    fn from_acl(acl: types::StreamAcl) -> StreamAclInternal {
        StreamAclInternal {
            read_roles: acl.read_roles,
            write_roles: acl.write_roles,
            delete_roles: acl.delete_roles,
            meta_read_roles: acl.meta_read_roles,
            meta_write_roles: acl.meta_write_roles,
        }
    }

    fn build_acl(self) -> types::StreamAcl {
        types::StreamAcl {
            read_roles: self.read_roles,
            write_roles: self.write_roles,
            delete_roles: self.delete_roles,
            meta_read_roles: self.meta_read_roles,
            meta_write_roles: self.meta_write_roles,
        }
    }
}

/// Write stream metadata command.
pub struct WriteStreamMetadata<'a> {
    metadata: types::StreamMetadata,
    inner: WriteEvents<'a>,
}

impl <'a> WriteStreamMetadata<'a> {
    pub(crate) fn new<S>(
        sender: Sender<Msg>,
        stream: S,
        metadata: types::StreamMetadata,
        settings: &types::Settings) -> WriteStreamMetadata
        where S: Into<Chars>
    {
        WriteStreamMetadata {
            metadata,
            inner: WriteEvents::new(sender, format!("$${}", stream.into().deref()), settings),
        }
    }

    /// Asks the server receiving the command to be the master of the cluster
    /// in order to perform the write. Default: `false`.
    pub fn require_master(mut self, value: bool) -> WriteStreamMetadata<'a> {
        self.inner = self.inner.require_master(value);

        self
    }

    /// Asks the server to check that the stream receiving the event is at
    /// the given expected version. Default: `types::ExpectedVersion::Any`.
    pub fn expected_version(mut self, value: types::ExpectedVersion) -> WriteStreamMetadata<'a> {
        self.inner = self.inner.expected_version(value);

        self
    }

    /// Performs the command with the given credentials.
    pub fn credentials(mut self, value: types::Credentials) -> WriteStreamMetadata<'a> {
        self.inner = self.inner.credentials(value);

        self
    }

    /// Sends asynchronously the write command to the server.
    pub fn execute(self) -> impl Future<Item=types::WriteResult, Error=OperationError> {
        let metadata = StreamMetadataInternal::from_metadata(self.metadata);
        let event    = types::EventData::json("$metadata", metadata);

        self.inner.push_event(event)
                  .execute()
    }
}

/// Reads a stream metadata command.
pub struct ReadStreamMetadata<'a> {
    stream: Chars,
    inner: ReadEvent<'a>,
}

impl <'a> ReadStreamMetadata<'a> {
    pub(crate) fn new<S>(sender: Sender<Msg>, stream: S, settings: &types::Settings) -> ReadStreamMetadata
        where S: Into<Chars>
    {
        let stream_chars = stream.into();
        let name         = format!("$${}", stream_chars.deref());

        ReadStreamMetadata {
            stream: stream_chars,
            inner: ReadEvent::new(sender, name, -1, settings),
        }
    }

    /// Asks the server receiving the command to be the master of the cluster
    /// in order to perform the write. Default: `false`.
    pub fn require_master(mut self, value: bool) -> ReadStreamMetadata<'a> {
        self.inner = self.inner.require_master(value);

        self
    }

    /// Performs the command with the given credentials.
    pub fn credentials(mut self, value: types::Credentials) -> ReadStreamMetadata<'a> {
        self.inner = self.inner.credentials(value);

        self
    }

    /// Sends asynchronously the read command to the server.
    pub fn execute(self) -> impl Future<Item=types::StreamMetadataResult, Error=OperationError> {
        let stream = self.stream;

        self.inner.execute().map(|res| {
            match res {
                types::ReadEventStatus::Success(result) => {
                    let metadata_internal: StreamMetadataInternal =
                        result.event
                              .get_original_event()
                              .unwrap()
                              .as_json()
                              .unwrap();

                    let versioned = types::VersionedMetadata {
                        stream,
                        version: result.event_number,
                        metadata: metadata_internal.build_metadata(),
                    };

                    types::StreamMetadataResult::Success(Box::new(versioned))
                },

                types::ReadEventStatus::NotFound | types::ReadEventStatus::NoStream => {
                    types::StreamMetadataResult::NotFound { stream }
                },

                types::ReadEventStatus::Deleted => {
                    types::StreamMetadataResult::Deleted { stream }
                },
            }
        })
    }
}

/// Command that starts a transaction on a stream.
pub struct TransactionStart<'a> {
    stream: Chars,
    version: types::ExpectedVersion,
    require_master: bool,
    creds_opt: Option<types::Credentials>,
    settings: &'a types::Settings,
    pub(crate) sender: Sender<Msg>,
}

impl <'a> TransactionStart<'a> {
    pub(crate) fn new<S>(
        sender: Sender<Msg>,
        stream: S,
        settings: &'a types::Settings) -> TransactionStart
        where S: Into<Chars>
    {
        TransactionStart {
            stream: stream.into(),
            require_master: false,
            version: types::ExpectedVersion::Any,
            creds_opt: None,
            settings,
            sender,
        }
    }

    /// Asks the server receiving the command to be the master of the cluster
    /// in order to perform the write. Default: `false`.
    pub fn require_master(self, require_master: bool) -> TransactionStart<'a> {
        TransactionStart { require_master, ..self }
    }

    /// Asks the server to check that the stream receiving the event is at
    /// the given expected version. Default: `types::ExpectedVersion::Any`.
    pub fn expected_version(self, version: types::ExpectedVersion) -> TransactionStart<'a> {
        TransactionStart { version, ..self }
    }

    /// Performs the command with the given credentials.
    pub fn credentials(self, value: types::Credentials) -> TransactionStart<'a> {
        TransactionStart { creds_opt: Some(value), ..self }
    }

    /// Sends asnychronously the start transaction command to the server.
    pub fn execute(self) -> impl Future<Item=Transaction, Error=OperationError> {
        let cloned_creds   = self.creds_opt.clone();
        let (rcv, promise) = operations::Promise::new(1);
        let mut op         = operations::TransactionStart::new(promise);
        let     stream     = self.stream.clone();

        op.set_event_stream_id(self.stream);
        op.set_require_master(self.require_master);
        op.set_expected_version(self.version);

        let require_master = self.require_master;
        let version        = self.version;
        let sender         = self.sender.clone();
        let op             = operations::OperationWrapper::new(op,
                                                               self.creds_opt,
                                                               self.settings.operation_retry.to_usize(),
                                                               self.settings.operation_timeout);

        self.sender.send(Msg::new_op(op)).wait().unwrap();

        let settings = self.settings.clone();

        single_value_future(rcv).map(move |id| {
            Transaction {
                stream,
                id,
                sender,
                require_master,
                creds: cloned_creds,
                version,
                settings: settings.clone(),
            }
        })
    }
}

/// Represents a multi-requests transaction with the GetEventStore server.
pub struct Transaction {
    stream: Chars,
    id: types::TransactionId,
    version: types::ExpectedVersion,
    require_master: bool,
    pub(crate) sender: Sender<Msg>,
    settings: types::Settings,
    creds: Option<types::Credentials>,
}

impl Transaction {
    /// Returns the a `Transaction` id.
    pub fn get_id(&self) -> types::TransactionId {
        self.id
    }

    /// Like `write` but specific to a single event.
    pub fn write_single(&self, event: types::EventData) -> impl Future<Item=(), Error=OperationError> {
        self.write(::std::iter::once(event))
    }

    /// Asynchronously write to transaction in the GetEventStore server.
    pub fn write<I>(&self, events: I) -> impl Future<Item=(), Error=OperationError>
        where I: IntoIterator<Item=types::EventData>
    {
        let (rcv, promise) = operations::Promise::new(1);
        let mut op = operations::TransactionWrite::new(promise, self.stream.clone());

        op.set_transaction_id(self.id);
        op.set_events(events);
        op.set_require_master(self.require_master);

        let op = operations::OperationWrapper::new(op,
                                                   self.creds.clone(),
                                                   self.settings.operation_retry.to_usize(),
                                                   self.settings.operation_timeout);

        self.sender.clone().send(Msg::new_op(op)).wait().unwrap();

        single_value_future(rcv)
    }

    /// Asynchronously commit this transaction.
    pub fn commit(self) -> impl Future<Item=types::WriteResult, Error=OperationError> {
        let (rcv, promise) = operations::Promise::new(1);
        let mut op =
            operations::TransactionCommit::new(promise, self.stream.clone(), self.version);

        op.set_transaction_id(self.id);
        op.set_require_master(self.require_master);

        let op = operations::OperationWrapper::new(op,
                                                   self.creds,
                                                   self.settings.operation_retry.to_usize(),
                                                   self.settings.operation_timeout);

        self.sender.send(Msg::new_op(op)).wait().unwrap();

        single_value_future(rcv)
    }

    // On purpose, this function does nothing. GetEventStore doesn't have a rollback operation.
    // This function is there mainly because of how transactions are perceived, meaning a
    // transaction comes with a `commit` and a `rollback` functions.
    pub fn rollback(self) {}
}

/// A command that reads several events from a stream. It can read events
/// forward or backward.
pub struct ReadStreamEvents<'a> {
    stream: Chars,
    max_count: i32,
    start: i64,
    require_master: bool,
    resolve_link_tos: bool,
    direction: types::ReadDirection,
    pub(crate) sender: Sender<Msg>,
    creds: Option<types::Credentials>,
    settings: &'a types::Settings,
}

impl <'a> ReadStreamEvents<'a> {
    pub(crate) fn new<S>(sender: Sender<Msg>, stream: S, settings: &types::Settings) -> ReadStreamEvents
        where S: Into<Chars>
    {
        ReadStreamEvents {
            stream: stream.into(),
            max_count: 500,
            start: 0,
            require_master: false,
            resolve_link_tos: false,
            direction: types::ReadDirection::Forward,
            sender,
            creds: None,
            settings,
        }
    }

    /// Asks the command to read forward (toward the end of the stream).
    /// That's the default behavior.
    pub fn forward(self) -> ReadStreamEvents<'a> {
        ReadStreamEvents { direction: types::ReadDirection::Forward, ..self }
    }

    /// Asks the command to read backward (toward the begining of the stream).
    pub fn backward(self) -> ReadStreamEvents<'a> {
        ReadStreamEvents { direction: types::ReadDirection::Backward, ..self }
    }

    /// Performs the command with the given credentials.
    pub fn credentials(self, value: types::Credentials) -> ReadStreamEvents<'a> {
        ReadStreamEvents { creds: Some(value), ..self }
    }

    /// Max batch size.
    pub fn max_count(self, max_count: i32) -> ReadStreamEvents<'a> {
        ReadStreamEvents { max_count, ..self }
    }

    /// Starts the read ot the given event number. By default, it starts at
    /// 0.
    pub fn start_from(self, start: i64) -> ReadStreamEvents<'a> {
        ReadStreamEvents { start, ..self }
    }

    /// Asks the server receiving the command to be the master of the cluster
    /// in order to perform the write. Default: `false`.
    pub fn require_master(self, require_master: bool) -> ReadStreamEvents<'a> {
        ReadStreamEvents { require_master, ..self }
    }

    /// When using projections, you can have links placed into another stream.
    /// If you set `true`, the server will resolve those links and will return
    /// the event that the link points to. Default: [NoResolution](../types/enum.LinkTos.html).
    pub fn resolve_link_tos(self, tos: types::LinkTos) -> ReadStreamEvents<'a> {
        let resolve_link_tos = tos.raw_resolve_lnk_tos();

        ReadStreamEvents { resolve_link_tos, ..self }
    }

    /// Sends asynchronously the read command to the server.
    pub fn execute(self) -> impl Future<Item=types::ReadStreamStatus<types::StreamSlice>, Error=OperationError> {
        let     (rcv, promise) = operations::Promise::new(1);
        let mut op             = operations::ReadStreamEvents::new(promise, self.direction);

        op.set_event_stream_id(self.stream);
        op.set_from_event_number(self.start);
        op.set_max_count(self.max_count);
        op.set_require_master(self.require_master);
        op.set_resolve_link_tos(self.resolve_link_tos);

        let op = operations::OperationWrapper::new(op,
                                                   self.creds,
                                                   self.settings.operation_retry.to_usize(),
                                                   self.settings.operation_timeout);

        self.sender.send(Msg::new_op(op)).wait().unwrap();

        single_value_future(rcv)
    }
}

/// Like `ReadStreamEvents` but specialized to system stream '$all'.
pub struct ReadAllEvents<'a> {
    max_count: i32,
    start: types::Position,
    require_master: bool,
    resolve_link_tos: bool,
    direction: types::ReadDirection,
    pub(crate) sender: Sender<Msg>,
    creds: Option<types::Credentials>,
    settings: &'a types::Settings,
}

impl <'a> ReadAllEvents<'a> {
    pub(crate) fn new(sender: Sender<Msg>, settings: &types::Settings) -> ReadAllEvents {
        ReadAllEvents {
            max_count: 500,
            start: types::Position::start(),
            require_master: false,
            resolve_link_tos: false,
            direction: types::ReadDirection::Forward,
            sender,
            creds: None,
            settings,
        }
    }

    /// Asks the command to read forward (toward the end of the stream).
    /// That's the default behavior.
    pub fn forward(self) -> ReadAllEvents<'a> {
        ReadAllEvents { direction: types::ReadDirection::Forward, ..self }
    }

    /// Asks the command to read backward (toward the begining of the stream).
    pub fn backward(self) -> ReadAllEvents<'a> {
        ReadAllEvents { direction: types::ReadDirection::Backward, ..self }
    }

    /// Performs the command with the given credentials.
    pub fn credentials(self, value: types::Credentials) -> ReadAllEvents<'a> {
        ReadAllEvents { creds: Some(value), ..self }
    }

    /// Max batch size.
    pub fn max_count(self, max_count: i32) -> ReadAllEvents<'a> {
        ReadAllEvents { max_count, ..self }
    }

    /// Starts the read ot the given event number. By default, it starts at
    /// `types::Position::start`.
    pub fn start_from(self, start: types::Position) -> ReadAllEvents<'a> {
        ReadAllEvents { start, ..self }
    }

    /// Asks the server receiving the command to be the master of the cluster
    /// in order to perform the write. Default: `false`.
    pub fn require_master(self, require_master: bool) -> ReadAllEvents<'a> {
        ReadAllEvents { require_master, ..self }
    }

    /// When using projections, you can have links placed into another stream.
    /// If you set `true`, the server will resolve those links and will return
    /// the event that the link points to. Default: [NoResolution](../types/enum.LinkTos.html).
    pub fn resolve_link_tos(self, tos: types::LinkTos) -> ReadAllEvents<'a> {
        let resolve_link_tos = tos.raw_resolve_lnk_tos();

        ReadAllEvents { resolve_link_tos, ..self }
    }

    /// Sends asynchronously the read command to the server.
    pub fn execute(self) -> impl Future<Item=types::ReadStreamStatus<types::AllSlice>, Error=OperationError> {
        let     (rcv, promise) = operations::Promise::new(1);
        let mut op             = operations::ReadAllEvents::new(promise, self.direction);

        op.set_from_position(self.start);
        op.set_max_count(self.max_count);
        op.set_require_master(self.require_master);
        op.set_resolve_link_tos(self.resolve_link_tos);

        let op = operations::OperationWrapper::new(op,
                                                   self.creds,
                                                   self.settings.operation_retry.to_usize(),
                                                   self.settings.operation_timeout);

        self.sender.send(Msg::new_op(op)).wait().unwrap();

        single_value_future(rcv)
    }
}

/// Command that deletes a stream. More information on [Deleting stream and events].
///
/// [Deleting stream and events]: https://eventstore.org/docs/server/deleting-streams-and-events/index.html
pub struct DeleteStream<'a> {
    stream: Chars,
    require_master: bool,
    version: types::ExpectedVersion,
    creds: Option<types::Credentials>,
    hard_delete: bool,
    pub(crate) sender: Sender<Msg>,
    settings: &'a types::Settings,
}

impl <'a> DeleteStream<'a> {
    pub(crate) fn new<S>(sender: Sender<Msg>, stream: S, settings: &types::Settings) -> DeleteStream
        where S: Into<Chars>
    {
        DeleteStream {
            stream: stream.into(),
            require_master: false,
            hard_delete: false,
            version: types::ExpectedVersion::Any,
            creds: None,
            sender,
            settings,
        }
    }

    /// Asks the server receiving the command to be the master of the cluster
    /// in order to perform the write. Default: `false`.
    pub fn require_master(self, require_master: bool) -> DeleteStream<'a> {
        DeleteStream { require_master, ..self }
    }

    /// Asks the server to check that the stream receiving the event is at
    /// the given expected version. Default: `types::ExpectedVersion::Any`.
    pub fn expected_version(self, version: types::ExpectedVersion) -> DeleteStream<'a> {
        DeleteStream { version, ..self }
    }

    /// Performs the command with the given credentials.
    pub fn credentials(self, value: types::Credentials) -> DeleteStream<'a> {
        DeleteStream { creds: Some(value), ..self }
    }

    /// Makes use of Truncate before. When a stream is deleted, its Truncate
    /// before is set to the streams current last event number. When a soft
    /// deleted stream is read, the read will return a StreamNotFound. After
    /// deleting the stream, you are able to write to it again, continuing from
    /// where it left off.
    ///
    /// That is the default behavior.
    pub fn soft_delete(self) -> DeleteStream<'a> {
        DeleteStream { hard_delete: false, ..self }
    }

    /// A hard delete writes a tombstone event to the stream, permanently
    /// deleting it. The stream cannot be recreated or written to again.
    /// Tombstone events are written with the event type '$streamDeleted'. When
    /// a hard deleted stream is read, the read will return a StreamDeleted.
    pub fn hard_delete(self) -> DeleteStream<'a> {
        DeleteStream { hard_delete: true, ..self }
    }

    /// Sends asynchronously the delete command to the server.
    pub fn execute(self) -> impl Future<Item=types::Position, Error=OperationError> {
        let     (rcv, promise) = operations::Promise::new(1);
        let mut op             = operations::DeleteStream::new(promise);

        op.set_event_stream_id(self.stream);
        op.set_expected_version(self.version);
        op.set_require_master(self.require_master);
        op.set_hard_delete(self.hard_delete);

        let op = operations::OperationWrapper::new(op,
                                                   self.creds,
                                                   self.settings.operation_retry.to_usize(),
                                                   self.settings.operation_timeout);

        self.sender.send(Msg::new_op(op)).wait().unwrap();

        single_value_future(rcv)
    }
}

/// Represents a volatile subscription. For example, if a stream has 100 events
/// in it when a subscriber connects, the subscriber can expect to see event
/// number 101 onwards until the time the subscription is closed or dropped.
///
/// * Notes
/// If the connection drops, the command will not try to resume the subscription.
/// If you need this behavior, use a catchup subscription instead.
pub struct SubscribeToStream<'a> {
    stream_id: Chars,
    pub(crate) sender: Sender<Msg>,
    resolve_link_tos: bool,
    creds: Option<types::Credentials>,
    settings: &'a types::Settings,
}

impl <'a> SubscribeToStream<'a> {
    pub(crate) fn new<S>(sender: Sender<Msg>, stream_id: S, settings: &types::Settings)
        -> SubscribeToStream
        where S: Into<Chars>
    {
        SubscribeToStream {
            stream_id: stream_id.into(),
            resolve_link_tos: false,
            creds: None,
            sender,
            settings,
        }
    }

    /// Performs the command with the given credentials.
    pub fn credentials(self, value: types::Credentials) -> SubscribeToStream<'a> {
        SubscribeToStream { creds: Some(value), ..self }
    }

    /// When using projections, you can have links placed into another stream.
    /// If you set `true`, the server will resolve those links and will return
    /// the event that the link points to. Default: [NoResolution](../types/enum.LinkTos.html).
    pub fn resolve_link_tos(self, tos: types::LinkTos) -> SubscribeToStream<'a> {
        let resolve_link_tos = tos.raw_resolve_lnk_tos();

        SubscribeToStream { resolve_link_tos, ..self }
    }

    /// Sends the volatile subscription request to the server asynchronously
    /// even if the subscription is available right away.
    pub fn execute(self) -> types::Subscription {
        let sender   = self.sender.clone();
        let (tx, rx) = mpsc::channel(operations::DEFAULT_BOUNDED_SIZE);
        let tx_dup   = tx.clone();
        let mut op   = operations::SubscribeToStream::new(tx);

        op.set_event_stream_id(self.stream_id);
        op.set_resolve_link_tos(self.resolve_link_tos);

        let op = operations::OperationWrapper::new(op,
                                                   self.creds,
                                                   self.settings.operation_retry.to_usize(),
                                                   self.settings.operation_timeout);

        self.sender.send(Msg::new_op(op)).wait().unwrap();

        types::Subscription {
            inner: tx_dup,
            receiver: rx,
            sender,
        }
    }
}

/// Subscribes to a given stream. This kind of subscription specifies a
/// starting point (by default, the beginning of a stream). For a regular
/// stream, that starting point will be an event number. For the system
/// stream `$all`, it will be a position in the transaction file
/// (see `subscribe_to_all_from`). This subscription will fetch every event
/// until the end of the stream, then will dispatch subsequently written
/// events.
///
/// For example, if a starting point of 50 is specified when a stream has
/// 100 events in it, the subscriber can expect to see events 51 through
/// 100, and then any events subsequenttly written events until such time
/// as the subscription is dropped or closed.
///
/// * Notes
/// Catchup subscription are resilient to connection drops.
/// Basically, if the connection drops. The command will restart its
/// catching up phase from the begining and then emit a new volatile
/// subscription request.
///
/// All this process happens without the user has to do anything.
pub struct RegularCatchupSubscribe<'a> {
    stream_id: Chars,
    resolve_link_tos: bool,
    require_master: bool,
    batch_size: u16,
    start_pos: i64,
    creds_opt: Option<types::Credentials>,
    pub(crate) sender: Sender<Msg>,
    settings: &'a types::Settings,
}

impl <'a> RegularCatchupSubscribe<'a> {
    pub(crate) fn new<S: Into<Chars>>(sender: Sender<Msg>, stream: S, settings: &types::Settings) -> RegularCatchupSubscribe {
        RegularCatchupSubscribe {
            stream_id: stream.into(),
            resolve_link_tos: false,
            require_master: false,
            batch_size: 500,
            start_pos: 0,
            sender,
            creds_opt: None,
            settings,
        }
    }

    /// When using projections, you can have links placed into another stream.
    /// If you set `true`, the server will resolve those links and will return
    /// the event that the link points to. Default: [NoResolution](../types/enum.LinkTos.html).
    pub fn resolve_link_tos(self, tos: types::LinkTos) -> RegularCatchupSubscribe<'a> {
        let resolve_link_tos = tos.raw_resolve_lnk_tos();

        RegularCatchupSubscribe { resolve_link_tos, ..self }
    }

    /// Asks the server receiving the command to be the master of the cluster
    /// in order to perform the write. Default: `false`.
    pub fn require_master(self, require_master: bool) -> RegularCatchupSubscribe<'a> {
        RegularCatchupSubscribe { require_master, ..self }
    }

    /// For example, if a starting point of 50 is specified when a stream has
    /// 100 events in it, the subscriber can expect to see events 51 through
    /// 100, and then any events subsequenttly written events until such time
    /// as the subscription is dropped or closed.
    ///
    /// By default, it will start from the event number 0.
    pub fn start_position(self, start_pos: i64) -> RegularCatchupSubscribe<'a> {
        RegularCatchupSubscribe { start_pos, ..self }
    }

    /// Performs the command with the given credentials.
    pub fn credentials(self, creds: types::Credentials) -> RegularCatchupSubscribe<'a> {
        RegularCatchupSubscribe { creds_opt: Some(creds), ..self }
    }

    /// Preforms the catching up phase of the subscription asynchronously. When
    /// it will reach the head of stream, the command will emit a volatile
    /// subscription request.
    pub fn execute(self) -> types::Subscription {
        let sender     = self.sender.clone();
        let (tx, rx)   = mpsc::channel(operations::DEFAULT_BOUNDED_SIZE);
        let tx_dup     = tx.clone();

        let inner = operations::RegularCatchup::new(
            self.stream_id.clone(),
            self.start_pos,
            self.require_master,
            self.resolve_link_tos,
            self.batch_size,
        );

        let op = operations::CatchupWrapper::new(
            inner,
            &self.stream_id,
            self.resolve_link_tos,
            tx
        );

        let op = operations::OperationWrapper::new(
            op,
            self.creds_opt,
            self.settings.operation_retry.to_usize(),
            self.settings.operation_timeout
        );

        self.sender.send(Msg::new_op(op)).wait().unwrap();

        types::Subscription {
            inner: tx_dup,
            receiver: rx,
            sender,
        }
    }
}

/// Like `RegularCatchupSubscribe` but specific to the system stream '$all'.
pub struct AllCatchupSubscribe<'a> {
    resolve_link_tos: bool,
    require_master: bool,
    batch_size: u16,
    start_pos: types::Position,
    creds_opt: Option<types::Credentials>,
    pub(crate) sender: Sender<Msg>,
    settings: &'a types::Settings,
}

impl <'a> AllCatchupSubscribe<'a> {
    pub(crate) fn new(sender: Sender<Msg>, settings: &types::Settings) -> AllCatchupSubscribe {
        AllCatchupSubscribe {
            resolve_link_tos: false,
            require_master: false,
            batch_size: 500,
            start_pos: types::Position::start(),
            sender,
            creds_opt: None,
            settings,
        }
    }

    /// When using projections, you can have links placed into another stream.
    /// If you set `true`, the server will resolve those links and will return
    /// the event that the link points to. Default: [NoResolution](../types/enum.LinkTos.html).
    pub fn resolve_link_tos(self, tos: types::LinkTos) -> AllCatchupSubscribe<'a> {
        let resolve_link_tos = tos.raw_resolve_lnk_tos();

        AllCatchupSubscribe { resolve_link_tos, ..self }
    }

    /// Asks the server receiving the command to be the master of the cluster
    /// in order to perform the write. Default: `false`.
    pub fn require_master(self, require_master: bool) -> AllCatchupSubscribe<'a> {
        AllCatchupSubscribe { require_master, ..self }
    }

    /// Starting point in the transaction journal log. By default, it will start at
    /// `types::Position::start`.
    pub fn start_position(self, start_pos: types::Position) -> AllCatchupSubscribe<'a> {
        AllCatchupSubscribe { start_pos, ..self }
    }

    /// Performs the command with the given credentials.
    pub fn credentials(self, creds: types::Credentials) -> AllCatchupSubscribe<'a> {
        AllCatchupSubscribe { creds_opt: Some(creds), ..self }
    }

    /// Preforms the catching up phase of the subscription asynchronously. When
    /// it will reach the head of stream, the command will emit a volatile
    /// subscription request.
    pub fn execute(self) -> types::Subscription {
        let sender     = self.sender.clone();
        let (tx, rx)   = mpsc::channel(operations::DEFAULT_BOUNDED_SIZE);
        let tx_dup     = tx.clone();

        let inner = operations::AllCatchup::new(
            self.start_pos,
            self.require_master,
            self.resolve_link_tos,
            self.batch_size,
        );

        let op = operations::CatchupWrapper::new(
            inner,
            &"".into(),
            self.resolve_link_tos,
            tx
        );

        let op = operations::OperationWrapper::new(
            op,
            self.creds_opt,
            self.settings.operation_retry.to_usize(),
            self.settings.operation_timeout
        );

        self.sender.send(Msg::new_op(op)).wait().unwrap();

        types::Subscription {
            inner: tx_dup,
            receiver: rx,
            sender,
        }
    }
}

/// A command that creates a persistent subscription for a given group.
pub struct CreatePersistentSubscription<'a> {
    stream_id: Chars,
    group_name: Chars,
    sub_settings: types::PersistentSubscriptionSettings,
    settings: &'a types::Settings,
    creds: Option<types::Credentials>,
    pub(crate) sender: Sender<Msg>,
}

impl<'a> CreatePersistentSubscription<'a> {
    pub(crate)
    fn new<S>(
        stream_id: S,
        group_name: S,
        sender: Sender<Msg>,
        settings: &'a types::Settings,
    ) -> CreatePersistentSubscription
        where S: Into<Chars>
    {
        CreatePersistentSubscription {
            stream_id: stream_id.into(),
            group_name: group_name.into(),
            sender,
            settings,
            creds: None,
            sub_settings: types::PersistentSubscriptionSettings::default(),
        }
    }

    /// Performs the command with the given credentials.
    pub fn credentials(
        self,
        creds: types::Credentials,
        ) -> CreatePersistentSubscription<'a>
    {
        CreatePersistentSubscription { creds: Some(creds), ..self }
    }

    /// Creates a persistent subscription based on the given
    /// `types::PersistentSubscriptionSettings`.
    pub fn settings(
        self,
        sub_settings: types::PersistentSubscriptionSettings
    ) -> CreatePersistentSubscription<'a>
    {
        CreatePersistentSubscription { sub_settings, ..self }
    }

    /// Sends the persistent subscription creation command asynchronously to
    /// the server.
    pub fn execute(self)
        -> impl Future<Item=types::PersistActionResult, Error=OperationError>
    {
        let     (rcv, promise) = operations::Promise::new(1);
        let mut op             = operations::CreatePersistentSubscription::new(promise);

        op.set_subscription_group_name(self.group_name);
        op.set_event_stream_id(self.stream_id);
        op.set_settings(&self.sub_settings);

        let op = operations::OperationWrapper::new(op,
                                                   self.creds,
                                                   self.settings.operation_retry.to_usize(),
                                                   self.settings.operation_timeout);

        self.sender.send(Msg::new_op(op)).wait().unwrap();

        single_value_future(rcv)
    }
}

/// Command that updates an already existing subscription's settings.
pub struct UpdatePersistentSubscription<'a> {
    stream_id: Chars,
    group_name: Chars,
    sub_settings: types::PersistentSubscriptionSettings,
    settings: &'a types::Settings,
    creds: Option<types::Credentials>,
    pub(crate) sender: Sender<Msg>,
}

impl<'a> UpdatePersistentSubscription<'a> {
    pub(crate)
    fn new<S>(
        stream_id: S,
        group_name: S,
        sender: Sender<Msg>,
        settings: &'a types::Settings,
    ) -> UpdatePersistentSubscription
        where S: Into<Chars>
    {
        UpdatePersistentSubscription {
            stream_id: stream_id.into(),
            group_name: group_name.into(),
            sender,
            settings,
            creds: None,
            sub_settings: types::PersistentSubscriptionSettings::default(),
        }
    }

    /// Performs the command with the given credentials.
    pub fn credentials(
        self,
        creds: types::Credentials,
        ) -> UpdatePersistentSubscription<'a>
    {
        UpdatePersistentSubscription { creds: Some(creds), ..self }
    }

    /// Updates a persistent subscription using the given
    /// `types::PersistentSubscriptionSettings`.
    pub fn settings(
        self,
        sub_settings: types::PersistentSubscriptionSettings
    ) -> UpdatePersistentSubscription<'a>
    {
        UpdatePersistentSubscription { sub_settings, ..self }
    }

    /// Sends the persistent subscription update command asynchronously to
    /// the server.
    pub fn execute(self)
        -> impl Future<Item=types::PersistActionResult, Error=OperationError>
    {
        let     (rcv, promise) = operations::Promise::new(1);
        let mut op             = operations::UpdatePersistentSubscription::new(promise);

        op.set_subscription_group_name(self.group_name);
        op.set_event_stream_id(self.stream_id);
        op.set_settings(self.sub_settings);

        let op = operations::OperationWrapper::new(op,
                                                   self.creds,
                                                   self.settings.operation_retry.to_usize(),
                                                   self.settings.operation_timeout);

        self.sender.send(Msg::new_op(op)).wait().unwrap();

        single_value_future(rcv)
    }
}

/// Command that  deletes a persistent subscription.
pub struct DeletePersistentSubscription<'a> {
    stream_id: Chars,
    group_name: Chars,
    settings: &'a types::Settings,
    creds: Option<types::Credentials>,
    pub(crate) sender: Sender<Msg>,
}

impl<'a> DeletePersistentSubscription<'a> {
    pub(crate)
    fn new<S>(
        stream_id: S,
        group_name: S,
        sender: Sender<Msg>,
        settings: &'a types::Settings,
    ) -> DeletePersistentSubscription
        where S: Into<Chars>
    {
        DeletePersistentSubscription {
            stream_id: stream_id.into(),
            group_name: group_name.into(),
            sender,
            settings,
            creds: None,
        }
    }

    /// Performs the command with the given credentials.
    pub fn credentials(
        self,
        creds: types::Credentials,
        ) -> DeletePersistentSubscription<'a>
    {
        DeletePersistentSubscription { creds: Some(creds), ..self }
    }

    /// Sends the persistent subscription deletion command asynchronously to
    /// the server.
    pub fn execute(self)
        -> impl Future<Item=types::PersistActionResult, Error=OperationError>
    {
        let     (rcv, promise) = operations::Promise::new(1);
        let mut op             = operations::DeletePersistentSubscription::new(promise);

        op.set_subscription_group_name(self.group_name);
        op.set_event_stream_id(self.stream_id);

        let op = operations::OperationWrapper::new(op,
                                                   self.creds,
                                                   self.settings.operation_retry.to_usize(),
                                                   self.settings.operation_timeout);

        self.sender.send(Msg::new_op(op)).wait().unwrap();

        single_value_future(rcv)
    }
}

/// A subscription model where the server remembers the state of the
/// consumption of a stream. This allows for many different modes of operations
/// compared to a regular subscription where the client hols the subscription
/// state.
pub struct ConnectToPersistentSubscription<'a> {
    stream_id: Chars,
    group_name: Chars,
    settings: &'a types::Settings,
    batch_size: u16,
    creds: Option<types::Credentials>,
    pub(crate) sender: Sender<Msg>,
}

impl<'a> ConnectToPersistentSubscription<'a> {
    pub(crate) fn new<S>(
        stream_id: S,
        group_name: S,
        sender: Sender<Msg>,
        settings: &'a types::Settings,
    ) -> ConnectToPersistentSubscription
        where S: Into<Chars>
    {
        ConnectToPersistentSubscription {
            stream_id: stream_id.into(),
            group_name: group_name.into(),
            sender,
            settings,
            batch_size: 10,
            creds: None,
        }
    }

    /// Performs the command with the given credentials.
    pub fn credentials(self, creds: types::Credentials)
        -> ConnectToPersistentSubscription<'a>
    {
        ConnectToPersistentSubscription { creds: Some(creds), ..self }
    }

    /// The buffer size to use  for the persistent subscription.
    pub fn batch_size(self, batch_size: u16)
        -> ConnectToPersistentSubscription<'a>
    {
        ConnectToPersistentSubscription { batch_size, ..self }
    }

    /// Sends the persistent subscription connection request to the server
    /// asynchronously even if the subscription is available right away.
    pub fn execute(self) -> types::Subscription {
        let sender   = self.sender.clone();
        let (tx, rx) = mpsc::channel(operations::DEFAULT_BOUNDED_SIZE);
        let tx_dup   = tx.clone();
        let mut op   = operations::ConnectToPersistentSubscription::new(tx);

        op.set_event_stream_id(self.stream_id);
        op.set_group_name(self.group_name);
        op.set_buffer_size(self.batch_size);

        let op = operations::OperationWrapper::new(op,
                                                   self.creds,
                                                   self.settings.operation_retry.to_usize(),
                                                   self.settings.operation_timeout);

        self.sender.send(Msg::new_op(op)).wait().unwrap();

        types::Subscription {
            inner: tx_dup,
            receiver: rx,
            sender,
        }
    }

}
