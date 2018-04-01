use std::time::Duration;

use serde::de::Deserialize;
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

#[derive(Debug)]
pub struct Position {
    pub commit:  i64,
    pub prepare: i64,
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
