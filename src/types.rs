use std::collections::HashMap;
use std::io::Read;
use std::time::Duration;

use bytes::{ Bytes, BytesMut, BufMut, Buf };
use futures::{ Future, Stream, Sink };
use futures::sync::mpsc::{ Receiver, Sender };
use futures::sync::oneshot;
use protobuf::Chars;
use serde::de::Deserialize;
use serde::ser::Serialize;
use serde_json;
use uuid::{ Uuid, ParseError };

use internal::command::Cmd;
use internal::messages;
use internal::messaging::Msg;
use internal::package::Pkg;

#[derive(Copy, Clone)]
pub enum Retry {
    Undefinately,
    Only(usize),
}

impl Retry {
    pub fn to_usize(&self) -> usize {
        match *self {
            Retry::Undefinately => usize::max_value(),
            Retry::Only(x)      => x,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Credentials {
    login: Bytes,
    password: Bytes,
}

impl Credentials {
    pub fn new<S>(login: S, password: S) -> Credentials
        where S: Into<Bytes>
    {
        Credentials {
            login: login.into(),
            password: password.into(),
        }
    }

    pub fn write_to_bytes_mut(&self, dst: &mut BytesMut) {
        dst.put_u8(self.login.len() as u8);
        dst.put(&self.login);
        dst.put_u8(self.password.len() as u8);
        dst.put(&self.password);
    }

    pub fn parse_from_buf<B>(buf: &mut B) -> ::std::io::Result<Credentials>
        where B: Buf + Read
    {
        let     login_len = buf.get_u8() as usize;
        let mut login     = Vec::with_capacity(login_len);

        let mut take = Read::take(buf, login_len as u64);
        take.read_to_end(&mut login)?;
        let buf = take.into_inner();

        let     passw_len = buf.get_u8() as usize;
        let mut password  = Vec::with_capacity(passw_len);

        let mut take = Read::take(buf, passw_len as u64);
        take.read_to_end(&mut password)?;

        let creds = Credentials {
            login: login.into(),
            password: password.into(),
        };

        Ok(creds)
    }

    pub fn network_size(&self) -> usize {
        self.login.len() + self.password.len() + 2 // Including 2 length bytes.
    }
}

#[derive(Clone)]
pub struct Settings {
    pub heartbeat_delay: Duration,
    pub heartbeat_timeout: Duration,
    pub operation_timeout: Duration,
    pub operation_retry: Retry,
    pub connection_retry: Retry,
    pub default_user: Option<Credentials>,
    pub connection_name: Option<String>,
    pub operation_check_period: Duration,
}

impl Settings {
    pub fn default() -> Settings {
        Settings {
            heartbeat_delay: Duration::from_millis(750),
            heartbeat_timeout: Duration::from_millis(1500),
            operation_timeout: Duration::from_secs(7),
            operation_retry: Retry::Only(3),
            connection_retry: Retry::Only(3),
            default_user: None,
            connection_name: None,
            operation_check_period: Duration::from_secs(1),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ExpectedVersion {
    Any,
    StreamExists,
    NoStream,
    Exact(i64),
}

impl ExpectedVersion {
    pub fn to_i64(self) -> i64 {
        match self {
            ExpectedVersion::Any          => -2,
            ExpectedVersion::StreamExists => -4,
            ExpectedVersion::NoStream     => -1,
            ExpectedVersion::Exact(n)     => n,
        }
    }

    pub fn from_i64(ver: i64) -> ExpectedVersion {
        match ver {
            -2 => ExpectedVersion::Any,
            -4 => ExpectedVersion::StreamExists,
            -1 => ExpectedVersion::NoStream,
            _  => ExpectedVersion::Exact(ver),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub commit:  i64,
    pub prepare: i64,
}

impl Position {
    pub fn start() -> Position {
        Position {
            commit: 0,
            prepare: 0,
        }
    }

    pub fn end() -> Position {
        Position {
            commit: -1,
            prepare: -1,
        }
    }
}

#[derive(Debug)]
pub struct WriteResult {
    pub next_expected_version: i64,
    pub position: Position,
}

#[derive(Debug)]
pub enum ReadEventStatus<A> {
    NotFound,
    NoStream,
    Deleted,
    Success(A),
}

#[derive(Debug)]
pub struct ReadEventResult {
    pub stream_id: String,
    pub event_number: i64,
    pub event: ResolvedEvent,
}

#[derive(Debug)]
pub struct RecordedEvent {
    pub event_stream_id: Chars,
    pub event_id: Uuid,
    pub event_number: i64,
    pub event_type: Chars,
    pub data: Bytes,
    pub metadata: Bytes,
    pub is_json: bool,
    pub created: Option<i64>,
    pub created_epoch: Option<i64>,
}

fn decode_parse_error(err: ParseError) -> ::std::io::Error {
    ::std::io::Error::new(::std::io::ErrorKind::Other, format!("ParseError {}", err))
}

impl RecordedEvent {
    pub fn new(mut event: messages::EventRecord) -> ::std::io::Result<RecordedEvent> {
        let event_stream_id = event.take_event_stream_id();
        let event_id        = Uuid::from_bytes(event.get_event_id()).map_err(decode_parse_error)?;
        let event_number    = event.get_event_number();
        let event_type      = event.take_event_type();
        let data            = event.take_data();
        let metadata        = event.take_metadata();

        let created = {
            if event.has_created() {
                Some(event.get_created())
            } else {
                None
            }
        };

        let created_epoch = {
            if event.has_created_epoch() {
                Some(event.get_created_epoch())
            } else {
                None
            }
        };

        let is_json = event.get_data_content_type() == 1;

        let record = RecordedEvent {
            event_stream_id,
            event_id,
            event_number,
            event_type,
            data,
            metadata,
            created,
            created_epoch,
            is_json,
        };

        Ok(record)
    }

    pub fn as_json<'a, T>(&'a self) -> serde_json::Result<T>
        where T: Deserialize<'a>
    {
        serde_json::from_slice(&self.data[..])
    }
}

#[derive(Debug)]
pub struct ResolvedEvent {
    pub event: Option<RecordedEvent>,
    pub link: Option<RecordedEvent>,
    pub position: Option<Position>,
}

impl ResolvedEvent {
    pub fn new(mut msg: messages::ResolvedEvent) -> ::std::io::Result<ResolvedEvent> {
        let event = {
            if msg.has_event() {
                let record = RecordedEvent::new(msg.take_event())?;
                Ok(Some(record))
            } else {
                Ok::<Option<RecordedEvent>, ::std::io::Error>(None)
            }
        }?;

        let link = {
            if msg.has_link() {
                let record = RecordedEvent::new(msg.take_link())?;
                Ok(Some(record))
            } else {
                Ok::<Option<RecordedEvent>, ::std::io::Error>(None)
            }
        }?;

        let position = Position {
            commit: msg.get_commit_position(),
            prepare: msg.get_prepare_position(),
        };

        let position = Some(position);

        let resolved = ResolvedEvent {
            event,
            link,
            position,
        };

        Ok(resolved)
    }

    pub fn new_from_indexed(mut msg: messages::ResolvedIndexedEvent) -> ::std::io::Result<ResolvedEvent> {
        let event = {
            if msg.has_event() {
                let record = RecordedEvent::new(msg.take_event())?;
                Ok(Some(record))
            } else {
                Ok::<Option<RecordedEvent>, ::std::io::Error>(None)
            }
        }?;

        let link = {
            if msg.has_link() {
                let record = RecordedEvent::new(msg.take_link())?;
                Ok(Some(record))
            } else {
                Ok::<Option<RecordedEvent>, ::std::io::Error>(None)
            }
        }?;

        let position = None;

        let resolved = ResolvedEvent {
            event,
            link,
            position,
        };

        Ok(resolved)
    }

    pub fn is_resolved(&self) -> bool {
        self.event.is_some() && self.link.is_some()
    }

    pub fn get_original_event(&self) -> Option<&RecordedEvent> {
        self.link.as_ref().or(self.event.as_ref())
    }

    pub fn get_original_stream_id(&self) -> Option<&Chars> {
        self.get_original_event().map(|event| &event.event_stream_id)
    }
}

#[derive(Debug)]
pub enum StreamMetadataResult {
    Deleted { stream: Chars },
    NotFound { stream: Chars },
    Success { stream: Chars, version: i64, metadata: StreamMetadata },
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct TransactionId(pub i64);

impl TransactionId {
    pub fn new(id: i64) -> TransactionId {
        TransactionId(id)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ReadDirection {
    Forward,
    Backward,
}

#[derive(Debug)]
pub enum LocatedEvents<A> {
    EndOfStream,
    Events { events: Vec<ResolvedEvent>, next: Option<A> },
}

impl <A> LocatedEvents<A> {
    pub fn is_end_of_stream(&self) -> bool {
        match *self {
            LocatedEvents::EndOfStream            => true,
            LocatedEvents::Events { ref next, ..} => next.is_some(),
        }
    }
}

pub trait Slice {
    type Location;

    fn from(&self) -> Self::Location;
    fn direction(&self) -> ReadDirection;
    fn events(self) -> LocatedEvents<Self::Location>;
}

#[derive(Debug)]
pub enum ReadStreamError {
    NoStream(Chars),
    StreamDeleted(Chars),
    NotModified(Chars),
    Error(Chars),
    AccessDenied(Chars),
}

#[derive(Debug)]
pub enum ReadStreamStatus<A> {
    Success(A),
    Error(ReadStreamError),
}

#[derive(Debug)]
pub struct StreamSlice {
    _from: i64,
    _direction: ReadDirection,
    _events: Vec<ResolvedEvent>,
    _next_num_opt: Option<i64>,
}

impl StreamSlice {
    pub fn new(
        direction: ReadDirection,
        from: i64,
        events: Vec<ResolvedEvent>,
        next_num_opt: Option<i64>) -> StreamSlice
    {
        StreamSlice {
            _from: from,
            _direction: direction,
            _events: events,
            _next_num_opt: next_num_opt,
        }
    }
}

impl Slice for StreamSlice {
    type Location = i64;

    fn from(&self) -> i64 {
        self._from
    }

    fn direction(&self) -> ReadDirection {
        self._direction
    }

    fn events(self) -> LocatedEvents<i64> {
        if self._events.is_empty() {
            LocatedEvents::EndOfStream
        } else {
            match self._next_num_opt {
                None => LocatedEvents::Events {
                    events: self._events,
                    next: None,
                },

                Some(next_num) => LocatedEvents::Events {
                    events: self._events,
                    next: Some(next_num),
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct AllSlice {
    from: Position,
    direction: ReadDirection,
    events: Vec<ResolvedEvent>,
    next: Position,
}

impl AllSlice {
    pub fn new(
        direction: ReadDirection,
        from: Position,
        events: Vec<ResolvedEvent>,
        next: Position) -> AllSlice
    {
        AllSlice {
            from,
            direction,
            events,
            next,
        }
    }
}

impl Slice for AllSlice {
    type Location = Position;

    fn from(&self) -> Position {
        self.from
    }

    fn direction(&self) -> ReadDirection {
        self.direction
    }

    fn events(self) -> LocatedEvents<Position> {
        if self.events.is_empty() {
            LocatedEvents::EndOfStream
        } else {
            LocatedEvents::Events {
                events: self.events,
                next: Some(self.next),
            }
        }
    }
}

enum Payload {
    Json(Bytes),
    Binary(Bytes),
}

pub struct EventData {
    event_type: Chars,
    payload: Payload,
    id_opt: Option<Uuid>,
    metadata_payload_opt: Option<Payload>,
}

impl EventData {
    pub fn json<P, S>(event_type: S, payload: P) -> EventData
        where P: Serialize,
              S: Into<Chars>
    {
        let bytes = Bytes::from(serde_json::to_vec(&payload).unwrap());

        EventData {
            event_type: event_type.into(),
            payload: Payload::Json(bytes),
            id_opt: None,
            metadata_payload_opt: None,
        }
    }

    pub fn binary<S>(event_type: S, payload: Bytes) -> EventData
        where S: Into<Chars>
    {
        EventData {
            event_type: event_type.into(),
            payload: Payload::Binary(payload),
            id_opt: None,
            metadata_payload_opt: None,
        }
    }

    pub fn id(self, value: Uuid) -> EventData {
        EventData { id_opt: Some(value), ..self }
    }

    pub fn metadata_as_json<P>(self, payload: P) -> EventData
        where P: Serialize
    {
        let bytes    = Bytes::from(serde_json::to_vec(&payload).unwrap());
        let json_bin = Some(Payload::Json(bytes));

        EventData { metadata_payload_opt: json_bin, ..self }
    }

    pub fn metadata_as_binary(self, payload: Bytes) -> EventData {
        let content_bin = Some(Payload::Binary(payload));

        EventData { metadata_payload_opt: content_bin, ..self }
    }

    pub fn build(self) -> messages::NewEvent {
        let mut new_event = messages::NewEvent::new();
        let     id        = self.id_opt.unwrap_or_else(|| Uuid::new_v4());

        new_event.set_event_id(Bytes::from(&id.as_bytes()[..]));

        match self.payload {
            Payload::Json(bin) => {
                new_event.set_data_content_type(1);
                new_event.set_data(bin);
            },

            Payload::Binary(bin) => {
                new_event.set_data_content_type(0);
                new_event.set_data(bin);
            },
        }

        match self.metadata_payload_opt {
            Some(Payload::Json(bin)) => {
                new_event.set_metadata_content_type(1);
                new_event.set_metadata(bin);
            },

            Some(Payload::Binary(bin)) => {
                new_event.set_metadata_content_type(0);
                new_event.set_metadata(bin);
            },

            None => new_event.set_metadata_content_type(0),
        }

        new_event.set_event_type(self.event_type);

        new_event
    }
}

#[derive(Default)]
pub struct StreamMetadataBuilder {
    max_count: Option<u64>,
    max_age: Option<Duration>,
    truncate_before: Option<u64>,
    cache_control: Option<Duration>,
    acl: Option<StreamAcl>,
    properties: HashMap<String, serde_json::Value>,
}

impl StreamMetadataBuilder {
    pub fn new() -> StreamMetadataBuilder {
        Default::default()
    }

    pub fn max_count(self, value: u64) -> StreamMetadataBuilder {
        StreamMetadataBuilder { max_count: Some(value), ..self }
    }

    pub fn max_age(self, value: Duration) -> StreamMetadataBuilder {
        StreamMetadataBuilder { max_age: Some(value), ..self }
    }

    pub fn truncate_before(self, value: u64) -> StreamMetadataBuilder {
        StreamMetadataBuilder { truncate_before: Some(value), ..self }
    }

    pub fn cache_control(self, value: Duration) -> StreamMetadataBuilder {
        StreamMetadataBuilder { cache_control: Some(value), ..self }
    }

    pub fn acl(self, value: StreamAcl) -> StreamMetadataBuilder {
        StreamMetadataBuilder { acl: Some(value), ..self }
    }

    pub fn insert_custom_property<V>(mut self, key: String, value: V) -> StreamMetadataBuilder
        where V: Serialize
    {
        let serialized = serde_json::to_value(value).unwrap();
        let _          = self.properties.insert(key, serialized);

        self
    }

    pub fn build(self) -> StreamMetadata {
        StreamMetadata {
            max_count: self.max_count,
            max_age: self.max_age,
            truncate_before: self.truncate_before,
            cache_control: self.cache_control,
            acl: self.acl.unwrap_or_default(),
            custom_properties: self.properties,
        }
    }
}

#[derive(Debug, Default)]
pub struct StreamMetadata {
    pub max_count: Option<u64>,
    pub max_age: Option<Duration>,
    pub truncate_before: Option<u64>,
    pub cache_control: Option<Duration>,
    pub acl: StreamAcl,
    pub custom_properties: HashMap<String, serde_json::Value>,
}

impl StreamMetadata {
    pub fn builder() -> StreamMetadataBuilder {
        StreamMetadataBuilder::new()
    }
}

#[derive(Default, Debug)]
pub struct StreamAcl {
    pub read_roles: Vec<String>,
    pub write_roles: Vec<String>,
    pub delete_roles: Vec<String>,
    pub meta_read_roles: Vec<String>,
    pub meta_write_roles: Vec<String>,
}

pub(crate) enum SubEvent {
    Confirmed {
        id: Uuid,
        last_commit_position: i64,
        last_event_number: i64
    },

    EventAppeared(ResolvedEvent),
    HasBeenConfirmed(oneshot::Sender<()>),
    Dropped
}

pub struct Subscription {
    pub(crate) inner: Sender<SubEvent>,
    pub(crate) receiver: Receiver<SubEvent>,
    pub(crate) sender: Sender<Msg>,
}

impl Subscription {
    pub fn consume<C>(self, consumer: C) -> C
        where C: SubscriptionConsumer
    {
        self.consume_async(consumer).wait().unwrap()
    }

    pub fn consume_async<C>(self, init: C) -> impl Future<Item=C, Error=()>
        where C: SubscriptionConsumer
    {
        struct State<A: SubscriptionConsumer> {
            consumer: A,
            confirmed: bool,
            confirmation_requests: Vec<oneshot::Sender<()>>,
        }

        impl <A: SubscriptionConsumer> State<A> {
            fn new(consumer: A) -> State<A> {
                State {
                    consumer,
                    confirmed: false,
                    confirmation_requests: Vec::new(),
                }
            }

            fn drain_requests(&mut self) {
                for req in self.confirmation_requests.drain(..) {
                    let _ = req.send(());
                }
            }
        }


        let mut sub_id_opt = None;
        let     sender     = self.sender.clone();

        self.receiver.fold(State::new(init), move |mut state, event| {
            match event {
                SubEvent::Confirmed { id, last_commit_position, last_event_number } => {
                    sub_id_opt = Some(id);

                    state.drain_requests();
                    state.consumer.when_confirmed(id, last_commit_position, last_event_number);
                },

                SubEvent::EventAppeared(evt) => {
                    if let OnEventAppeared::Drop = state.consumer.when_event_appeared(evt) {
                        let id  = sub_id_opt.take().expect("impossible situation when dropping subscription");
                        let pkg = Pkg::new(Cmd::UnsubscribeFromStream, id);

                        sender.clone().send(Msg::Send(pkg)).wait().unwrap();
                    }
                },

                SubEvent::Dropped => {
                    state.consumer.when_dropped();
                    state.drain_requests();
                },

                SubEvent::HasBeenConfirmed(req) => {
                    if state.confirmed {
                        let _ = req.send(());
                    } else {
                        state.confirmation_requests.push(req);
                    }
                },
            }

            Ok::<State<C>, ()>(state)
        }).map(|state| state.consumer)
    }

    /// You shouldn't have to use that function as it makes no sense to
    /// wait for a confirmation from the server. However, for testing
    /// purpose or weirdos, we expose that function. it returns will a
    /// a future waiting the subscription to be confirmed by the server.
    pub fn confirmation(&self) -> impl Future<Item=(), Error=()> {
        let (tx, rcv) = oneshot::channel();
        let _         = self.inner.clone().send(SubEvent::HasBeenConfirmed(tx)).wait();

        rcv.map_err(|_| ())
    }
}

pub enum OnEventAppeared {
    Continue,
    Drop,
}

pub trait SubscriptionConsumer {
    fn when_confirmed(&mut self, id: Uuid, last_commit_position: i64, last_event_number: i64);
    fn when_event_appeared(&mut self, event: ResolvedEvent) -> OnEventAppeared;
    fn when_dropped(&mut self);
}
