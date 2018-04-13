use std::time::Duration;

use serde::de::Deserialize;
use serde::ser::Serialize;
use serde_json;
use uuid::{ Uuid, ParseError };

use internal::messages;
use internal::metadata::StreamMetadata;

#[derive(Copy, Clone)]
pub enum Retry {
    Undefinately,
    Only(u32),
}

impl Retry {
    pub fn to_u32(&self) -> u32 {
        match *self {
            Retry::Undefinately => u32::max_value(),
            Retry::Only(x)      => x,
        }
    }
}

#[derive(Clone)]
pub struct Credentials {
    pub login: String,
    pub password: String,
}

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
    pub event_stream_id: String,
    pub event_id: Uuid,
    pub event_number: i64,
    pub event_type: String,
    pub data: Vec<u8>,
    pub metadata: Vec<u8>,
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
        serde_json::from_slice(self.data.as_slice())
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

    pub fn get_original_stream_id(&self) -> Option<&String> {
        self.get_original_event().map(|event| &event.event_stream_id)
    }
}

#[derive(Debug)]
pub enum StreamMetadataResult {
    Deleted { stream: String },
    NotFound { stream: String },
    Success { stream: String, version: i64, metadata: StreamMetadata },
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

pub trait Slice {
    type Location;

    fn from(&self) -> Self::Location;
    fn direction(&self) -> ReadDirection;
    fn events(self) -> LocatedEvents<Self::Location>;
}

#[derive(Debug)]
pub enum ReadStreamStatus<A> {
    Success(A),
    NoStream(String),
    StreamDeleled(String),
    NotModified(String),
    Error(String),
    AccessDenied(String),
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
    Json(Vec<u8>),
    Binary(Vec<u8>),
}

pub struct EventData {
    event_type: String,
    payload: Payload,
    id_opt: Option<Uuid>,
    metadata_payload_opt: Option<Payload>,
}

impl EventData {
    pub fn json<P>(event_type: String, payload: P) -> EventData
        where P: Serialize
    {
        EventData {
            event_type,
            payload: Payload::Json(serde_json::to_vec(&payload).unwrap()),
            id_opt: None,
            metadata_payload_opt: None,
        }
    }

    pub fn binary(event_type: String, payload: Vec<u8>) -> EventData {
        EventData {
            event_type,
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
        let json_bin = serde_json::to_vec(&payload).unwrap();
        let json_bin = Some(Payload::Json(json_bin));

        EventData { metadata_payload_opt: json_bin, ..self }
    }

    pub fn metadata_as_binary(self, payload: Vec<u8>) -> EventData {
        let content_bin = Some(Payload::Binary(payload));

        EventData { metadata_payload_opt: content_bin, ..self }
    }

    pub fn build(self) -> messages::NewEvent {
        let mut new_event = messages::NewEvent::new();
        let     id        = self.id_opt.unwrap_or_else(|| Uuid::new_v4());

        new_event.set_event_id(id.as_bytes().to_vec());

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
