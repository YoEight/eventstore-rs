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
            _                => unreachable!(),
        }
    })
}

pub struct WriteEvents {
    stream: Chars,
    events: Vec<types::EventData>,
    require_master: bool,
    version: types::ExpectedVersion,
    creds: Option<types::Credentials>,
    sender: Sender<Msg>,
}

impl WriteEvents {
    pub fn new<S>(sender: Sender<Msg>, stream: S) -> WriteEvents
        where S: Into<Chars>
    {
        WriteEvents {
            stream: stream.into(),
            events: Vec::new(),
            require_master: false,
            version: types::ExpectedVersion::Any,
            creds: None,
            sender,
        }
    }

    pub fn set_events(self, events: Vec<types::EventData>) -> WriteEvents {
        WriteEvents { events, ..self}
    }

    pub fn push_event(mut self, event: types::EventData) -> WriteEvents {
        self.events.push(event);

        self
    }

    pub fn append_events<T>(mut self, events: T) -> WriteEvents
        where T: IntoIterator<Item=types::EventData>
    {
        self.events.extend(events);

        self
    }

    pub fn require_master(self, require_master: bool) -> WriteEvents {
        WriteEvents { require_master, ..self }
    }

    pub fn expected_version(self, version: types::ExpectedVersion) -> WriteEvents {
        WriteEvents { version, ..self }
    }

    pub fn credentials(self, creds: types::Credentials) -> WriteEvents {
        WriteEvents { creds: Some(creds), ..self }
    }

    pub fn execute(self) -> impl Future<Item=types::WriteResult, Error=OperationError> {
        let     (rcv, promise) = operations::Promise::new(1);
        let mut op             = operations::WriteEvents::new(promise, self.creds);

        op.set_event_stream_id(self.stream);
        op.set_expected_version(self.version);
        op.set_events(self.events);
        op.set_require_master(self.require_master);

        self.sender.send(Msg::new_op(op)).wait().unwrap();

        single_value_future(rcv)
    }
}

pub struct ReadEvent {
    stream: Chars,
    event_number: i64,
    resolve_link_tos: bool,
    require_master: bool,
    creds: Option<types::Credentials>,
    sender: Sender<Msg>,
}

impl ReadEvent {
    pub fn new<S>(sender: Sender<Msg>, stream: S, event_number: i64) -> ReadEvent
        where S: Into<Chars>
    {
        ReadEvent {
            stream: stream.into(),
            event_number,
            sender,
            resolve_link_tos: false,
            require_master: false,
            creds: None,
        }
    }

    pub fn resolve_link_tos(self, resolve_link_tos: bool) -> ReadEvent {
        ReadEvent { resolve_link_tos, ..self }
    }

    pub fn require_master(self, require_master: bool) -> ReadEvent {
        ReadEvent { require_master, ..self }
    }

    pub fn credentials(self, value: types::Credentials) -> ReadEvent {
        ReadEvent { creds: Some(value), ..self }
    }

    pub fn execute(self) -> impl Future<Item=types::ReadEventStatus<types::ReadEventResult>, Error=OperationError> {
        let (rcv, promise) = operations::Promise::new(1);
        let mut op         = operations::ReadEvent::new(promise, self.creds);

        op.set_event_stream_id(self.stream);
        op.set_event_number(self.event_number);
        op.set_resolve_link_tos(self.resolve_link_tos);
        op.set_require_master(self.require_master);

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

    fn to_metadata(self) -> types::StreamMetadata {
        types::StreamMetadata {
            max_count: self.max_count,
            max_age: self.max_age.map(|t| t.to_duration()),
            truncate_before: self.truncate_before,
            cache_control: self.cache_control.map(|t| t.to_duration()),
            acl: self.acl.to_acl(),
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

    fn to_acl(self) -> types::StreamAcl {
        types::StreamAcl {
            read_roles: self.read_roles,
            write_roles: self.write_roles,
            delete_roles: self.delete_roles,
            meta_read_roles: self.meta_read_roles,
            meta_write_roles: self.meta_write_roles,
        }
    }
}

pub struct WriteStreamData {
    metadata: types::StreamMetadata,
    inner: WriteEvents,
}

impl WriteStreamData {
    pub fn new<S>(sender: Sender<Msg>, stream: S, metadata: types::StreamMetadata) -> WriteStreamData
        where S: Into<Chars>
    {
        WriteStreamData {
            metadata,
            inner: WriteEvents::new(sender, format!("$${}", stream.into().deref())),
        }
    }

    pub fn require_master(mut self, value: bool) -> WriteStreamData {
        self.inner = self.inner.require_master(value);

        self
    }

    pub fn expected_version(mut self, value: types::ExpectedVersion) -> WriteStreamData {
        self.inner = self.inner.expected_version(value);

        self
    }

    pub fn credentials(mut self, value: types::Credentials) -> WriteStreamData {
        self.inner = self.inner.credentials(value);

        self
    }

    pub fn execute(self) -> impl Future<Item=types::WriteResult, Error=OperationError> {
        let metadata = StreamMetadataInternal::from_metadata(self.metadata);
        let event    = types::EventData::json("$metadata", metadata);

        self.inner.push_event(event)
                  .execute()
    }
}

pub struct ReadStreamData {
    stream: Chars,
    inner: ReadEvent,
}

impl ReadStreamData {
    pub fn new<S>(sender: Sender<Msg>, stream: S) -> ReadStreamData
        where S: Into<Chars>
    {
        let stream_chars = stream.into();
        let name         = format!("$${}", stream_chars.deref());

        ReadStreamData {
            stream: stream_chars,
            inner: ReadEvent::new(sender, name, -1),
        }
    }

    pub fn require_master(mut self, value: bool) -> ReadStreamData {
        self.inner = self.inner.require_master(value);

        self
    }

    pub fn credentials(mut self, value: types::Credentials) -> ReadStreamData {
        self.inner = self.inner.credentials(value);

        self
    }

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

                    types::StreamMetadataResult::Success {
                        stream: stream,
                        version: result.event_number,
                        metadata: metadata_internal.to_metadata(),
                    }
                },

                types::ReadEventStatus::NotFound | types::ReadEventStatus::NoStream => {
                    types::StreamMetadataResult::NotFound { stream: stream }
                },

                types::ReadEventStatus::Deleted => {
                    types::StreamMetadataResult::Deleted { stream: stream }
                },
            }
        })
    }
}

pub struct TransactionStart {
    stream: Chars,
    version: types::ExpectedVersion,
    require_master: bool,
    creds_opt: Option<types::Credentials>,
    sender: Sender<Msg>,
}

impl TransactionStart {
    pub fn new<S>(sender: Sender<Msg>, stream: S) -> TransactionStart
        where S: Into<Chars>
    {
        TransactionStart {
            stream: stream.into(),
            require_master: false,
            version: types::ExpectedVersion::Any,
            creds_opt: None,
            sender,
        }
    }

    pub fn require_master(self, require_master: bool) -> TransactionStart {
        TransactionStart { require_master, ..self }
    }

    pub fn version(self, version: types::ExpectedVersion) -> TransactionStart {
        TransactionStart { version, ..self }
    }

    pub fn credentials(self, value: types::Credentials) -> TransactionStart {
        TransactionStart { creds_opt: Some(value), ..self }
    }

    pub fn execute(self) -> impl Future<Item=Transaction, Error=OperationError> {
        let cloned_creds   = self.creds_opt.clone();
        let (rcv, promise) = operations::Promise::new(1);
        let mut op         = operations::TransactionStart::new(promise, self.creds_opt);
        let     stream     = self.stream.clone();

        op.set_event_stream_id(self.stream);
        op.set_require_master(self.require_master);
        op.set_expected_version(self.version);

        let require_master = self.require_master;
        let version        = self.version;
        let sender         = self.sender.clone();

        self.sender.send(Msg::new_op(op)).wait().unwrap();

        single_value_future(rcv).map(move |id| {
            Transaction {
                stream,
                id,
                sender,
                require_master,
                creds: cloned_creds,
                version,
            }
        })
    }
}

pub struct Transaction {
    stream: Chars,
    id: types::TransactionId,
    version: types::ExpectedVersion,
    require_master: bool,
    sender: Sender<Msg>,
    creds: Option<types::Credentials>,
}

impl Transaction {
    pub fn get_id(&self) -> types::TransactionId {
        self.id
    }

    pub fn write_single(&self, event: types::EventData) -> impl Future<Item=(), Error=OperationError> {
        self.write(::std::iter::once(event))
    }

    pub fn write<I>(&self, events: I) -> impl Future<Item=(), Error=OperationError>
        where I: IntoIterator<Item=types::EventData>
    {
        let (rcv, promise) = operations::Promise::new(1);
        let mut op = operations::TransactionWrite::new(
            promise, self.stream.clone(), self.creds.clone());

        op.set_transaction_id(self.id);
        op.set_events(events);
        op.set_require_master(self.require_master);

        self.sender.clone().send(Msg::new_op(op)).wait().unwrap();

        single_value_future(rcv)
    }

    pub fn commit(self) -> impl Future<Item=types::WriteResult, Error=OperationError> {
        let (rcv, promise) = operations::Promise::new(1);
        let mut op =
            operations::TransactionCommit::new(
                promise, self.stream.clone(), self.version, self.creds.clone());

        op.set_transaction_id(self.id);
        op.set_require_master(self.require_master);

        self.sender.send(Msg::new_op(op)).wait().unwrap();

        single_value_future(rcv)
    }

    pub fn rollback(self) {
        // On purpose, this function does nothing. GetEventStore doesn't have a rollback operation.
        // This function is there mainly because of how transactions are perceived, meaning a
        // transaction comes with a commit and a rollback functions.
    }
}

pub struct ReadStreamEvents {
    stream: Chars,
    max_count: i32,
    start: i64,
    require_master: bool,
    resolve_link_tos: bool,
    direction: types::ReadDirection,
    sender: Sender<Msg>,
    creds: Option<types::Credentials>,
}

impl ReadStreamEvents {
    pub fn new<S>(sender: Sender<Msg>, stream: S) -> ReadStreamEvents
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
        }
    }

    pub fn forward(self) -> ReadStreamEvents {
        ReadStreamEvents { direction: types::ReadDirection::Forward, ..self }
    }

    pub fn backward(self) -> ReadStreamEvents {
        ReadStreamEvents { direction: types::ReadDirection::Backward, ..self }
    }

    pub fn credentials(self, value: types::Credentials) -> ReadStreamEvents {
        ReadStreamEvents { creds: Some(value), ..self }
    }

    pub fn max_count(self, max_count: i32) -> ReadStreamEvents {
        ReadStreamEvents { max_count, ..self }
    }

    pub fn start_from(self, start: i64) -> ReadStreamEvents {
        ReadStreamEvents { start, ..self }
    }

    pub fn require_master(self, require_master: bool) -> ReadStreamEvents {
        ReadStreamEvents { require_master, ..self }
    }

    pub fn resolve_link_tos(self, resolve_link_tos: bool) -> ReadStreamEvents {
        ReadStreamEvents { resolve_link_tos, ..self }
    }

    pub fn execute(self) -> impl Future<Item=types::ReadStreamStatus<types::StreamSlice>, Error=OperationError> {
        let     (rcv, promise) = operations::Promise::new(1);
        let mut op             = operations::ReadStreamEvents::new(promise, self.direction, self.creds);

        op.set_event_stream_id(self.stream);
        op.set_from_event_number(self.start);
        op.set_max_count(self.max_count);
        op.set_require_master(self.require_master);
        op.set_resolve_link_tos(self.resolve_link_tos);

        self.sender.send(Msg::new_op(op)).wait().unwrap();

        single_value_future(rcv)
    }
}

pub struct ReadAllEvents {
    max_count: i32,
    start: types::Position,
    require_master: bool,
    resolve_link_tos: bool,
    direction: types::ReadDirection,
    sender: Sender<Msg>,
    creds: Option<types::Credentials>,
}

impl ReadAllEvents {
    pub fn new(sender: Sender<Msg>) -> ReadAllEvents {
        ReadAllEvents {
            max_count: 500,
            start: types::Position::start(),
            require_master: false,
            resolve_link_tos: false,
            direction: types::ReadDirection::Forward,
            sender,
            creds: None,
        }
    }

    pub fn forward(self) -> ReadAllEvents {
        ReadAllEvents { direction: types::ReadDirection::Forward, ..self }
    }

    pub fn backward(self) -> ReadAllEvents {
        ReadAllEvents { direction: types::ReadDirection::Backward, ..self }
    }

    pub fn credentials(self, value: types::Credentials) -> ReadAllEvents {
        ReadAllEvents { creds: Some(value), ..self }
    }

    pub fn max_count(self, max_count: i32) -> ReadAllEvents {
        ReadAllEvents { max_count, ..self }
    }

    pub fn start_from(self, start: types::Position) -> ReadAllEvents {
        ReadAllEvents { start, ..self }
    }

    pub fn require_master(self, require_master: bool) -> ReadAllEvents {
        ReadAllEvents { require_master, ..self }
    }

    pub fn resolve_link_tos(self, resolve_link_tos: bool) -> ReadAllEvents {
        ReadAllEvents { resolve_link_tos, ..self }
    }

    pub fn execute(self) -> impl Future<Item=types::ReadStreamStatus<types::AllSlice>, Error=OperationError> {
        let     (rcv, promise) = operations::Promise::new(1);
        let mut op             = operations::ReadAllEvents::new(promise, self.direction, self.creds);

        op.set_from_position(self.start);
        op.set_max_count(self.max_count);
        op.set_require_master(self.require_master);
        op.set_resolve_link_tos(self.resolve_link_tos);

        self.sender.send(Msg::new_op(op)).wait().unwrap();

        single_value_future(rcv)
    }
}

pub struct DeleteStream {
    stream: Chars,
    require_master: bool,
    version: types::ExpectedVersion,
    creds: Option<types::Credentials>,
    hard_delete: bool,
    sender: Sender<Msg>,
}

impl DeleteStream {
    pub fn new<S>(sender: Sender<Msg>, stream: S) -> DeleteStream
        where S: Into<Chars>
    {
        DeleteStream {
            stream: stream.into(),
            require_master: false,
            hard_delete: false,
            version: types::ExpectedVersion::Any,
            creds: None,
            sender,
        }
    }

    pub fn require_master(self, require_master: bool) -> DeleteStream {
        DeleteStream { require_master, ..self }
    }

    pub fn expected_version(self, version: types::ExpectedVersion) -> DeleteStream {
        DeleteStream { version, ..self }
    }

    pub fn credentials(self, value: types::Credentials) -> DeleteStream {
        DeleteStream { creds: Some(value), ..self }
    }

    pub fn hard_delete(self, hard_delete: bool) -> DeleteStream {
        DeleteStream { hard_delete, ..self }
    }

    pub fn execute(self) -> impl Future<Item=types::Position, Error=OperationError> {
        let     (rcv, promise) = operations::Promise::new(1);
        let mut op             = operations::DeleteStream::new(promise, self.creds);

        op.set_event_stream_id(self.stream);
        op.set_expected_version(self.version);
        op.set_require_master(self.require_master);
        op.set_hard_delete(self.hard_delete);

        self.sender.send(Msg::new_op(op)).wait().unwrap();

        single_value_future(rcv)
    }
}

pub struct SubscribeToStream {
    stream_id: Chars,
    sender: Sender<Msg>,
    resolve_link_tos: bool,
    creds: Option<types::Credentials>,
}

impl SubscribeToStream {
    pub(crate) fn new<S>(sender: Sender<Msg>, stream_id: S)
        -> SubscribeToStream
        where S: Into<Chars>
    {
        SubscribeToStream {
            stream_id: stream_id.into(),
            resolve_link_tos: false,
            creds: None,
            sender,
        }
    }

    pub fn credentials(self, value: types::Credentials) -> SubscribeToStream {
        SubscribeToStream { creds: Some(value), ..self }
    }

    pub fn resolve_link_tos(self, resolve_link_tos: bool) -> SubscribeToStream {
        SubscribeToStream { resolve_link_tos, ..self }
    }

    pub fn execute(self) -> types::Subscription {
        let sender   = self.sender.clone();
        let (tx, rx) = mpsc::channel(500);
        let mut op   = operations::SubscribeToStream::new(tx, self.creds);

        op.set_event_stream_id(self.stream_id);
        op.set_resolve_link_tos(self.resolve_link_tos);

        self.sender.send(Msg::new_op(op)).wait().unwrap();

        types::Subscription {
            receiver: rx,
            sender,
        }
    }
}

pub struct RegularCatchupSubscribe {
    stream_id: Chars,
    resolve_link_tos: bool,
    require_master: bool,
    batch_size: u16,
    start_pos: i64,
    creds_opt: Option<types::Credentials>,
    sender: Sender<Msg>,
}

impl RegularCatchupSubscribe {
    pub(crate) fn new<S: Into<Chars>>(sender: Sender<Msg>, stream: S) -> RegularCatchupSubscribe {
        RegularCatchupSubscribe {
            stream_id: stream.into(),
            resolve_link_tos: false,
            require_master: false,
            batch_size: 500,
            start_pos: 0,
            sender,
            creds_opt: None,
        }
    }

    pub fn resolve_link_tos(self, resolve_link_tos: bool) -> RegularCatchupSubscribe {
        RegularCatchupSubscribe { resolve_link_tos, ..self }
    }

    pub fn require_master(self, require_master: bool) -> RegularCatchupSubscribe {
        RegularCatchupSubscribe { require_master, ..self }
    }

    pub fn start_position(self, start_pos: i64) -> RegularCatchupSubscribe {
        RegularCatchupSubscribe { start_pos, ..self }
    }

    pub fn credentials(self, creds: types::Credentials) -> RegularCatchupSubscribe {
        RegularCatchupSubscribe { creds_opt: Some(creds), ..self }
    }

    pub fn execute(self) -> types::Subscription {
        let sender   = self.sender.clone();
        let (tx, rx) = mpsc::channel(500);

        let puller = operations::RegularStreamPull::new(
            self.stream_id.clone(), self.resolve_link_tos, self.require_master, self.batch_size as i32, self.start_pos, self.creds_opt.clone());
        let op = operations::CatchupSubscribe::new(
            Some(self.stream_id), self.resolve_link_tos, tx, self.creds_opt, puller);

        self.sender.send(Msg::new_op(op)).wait().unwrap();

        types::Subscription {
            receiver: rx,
            sender,
        }
    }
}
