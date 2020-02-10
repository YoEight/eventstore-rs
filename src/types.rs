//! Common types used across the library.
use std::cmp::Ordering;
use std::collections::HashMap;
use std::net::{SocketAddr, ToSocketAddrs};
use std::ops::Deref;
use std::time::Duration;

use bytes::Bytes;
use futures::channel::mpsc::Sender;
use futures::sink::SinkExt;
use protobuf::Chars;
use serde::de::Deserialize;
use serde::ser::Serialize;
use serde_json;
use uuid::{BytesError, Uuid};

use crate::internal::command::Cmd;
use crate::internal::messages;
use crate::internal::messaging::Msg;
use crate::internal::package::Pkg;
use futures::Stream;

#[derive(Debug, Clone)]
pub enum OperationError {
    WrongExpectedVersion(String, ExpectedVersion),
    StreamDeleted(String),
    InvalidTransaction,
    AccessDenied(String),
    ProtobufDecodingError(String),
    ServerError(Option<String>),
    InvalidOperation(String),
    StreamNotFound(String),
    AuthenticationRequired,
    Aborted,
    WrongClientImpl(Option<Cmd>),
    ConnectionHasDropped,
    NotImplemented,
}

impl std::error::Error for OperationError {}

impl std::fmt::Display for OperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use OperationError::*;

        match self {
            WrongExpectedVersion(stream, exp) => {
                writeln!(f, "expected version {:?} for stream {}", exp, stream)
            }
            StreamDeleted(stream) => writeln!(f, "stream {} deleted", stream),
            InvalidTransaction => writeln!(f, "invalid transaction"),
            AccessDenied(info) => writeln!(f, "access denied: {}", info),
            ProtobufDecodingError(error) => writeln!(f, "protobuf decoding error: {}", error),
            ServerError(error) => writeln!(f, "server error: {:?}", error),
            InvalidOperation(info) => writeln!(f, "invalid operation: {}", info),
            StreamNotFound(stream) => writeln!(f, "stream {} not found", stream),
            AuthenticationRequired => writeln!(f, "authentication required"),
            Aborted => writeln!(f, "aborted"),
            WrongClientImpl(info) => writeln!(f, "wrong client impl: {:?}", info),
            ConnectionHasDropped => writeln!(f, "connection has dropped"),
            NotImplemented => writeln!(f, "not implemented"),
        }
    }
}

/// Represents a reconnection strategy when a connection has dropped or is
/// about to be created.
#[derive(Copy, Clone, Debug)]
pub enum Retry {
    Undefinately,
    Only(usize),
}

impl Retry {
    pub(crate) fn to_usize(&self) -> usize {
        match *self {
            Retry::Undefinately => usize::max_value(),
            Retry::Only(x) => x,
        }
    }
}

/// Holds login and password information.
#[derive(Clone, Debug)]
pub struct Credentials {
    pub(crate) login: Bytes,
    pub(crate) password: Bytes,
}

impl Credentials {
    /// Creates a new `Credentials` instance.
    pub fn new<S>(login: S, password: S) -> Credentials
    where
        S: Into<Bytes>,
    {
        Credentials {
            login: login.into(),
            password: password.into(),
        }
    }

    pub(crate) fn network_size(&self) -> usize {
        self.login.len() + self.password.len() + 2 // Including 2 length bytes.
    }
}

/// Determines whether any link event encountered in the stream will be
/// resolved. See the discussion on [Resolved Events](https://eventstore.org/docs/dotnet-api/reading-events/index.html#resolvedevent)
/// for more information on this.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkTos {
    ResolveLink,
    NoResolution,
}

impl LinkTos {
    pub(crate) fn raw_resolve_lnk_tos(self) -> bool {
        match self {
            LinkTos::ResolveLink => true,
            LinkTos::NoResolution => false,
        }
    }

    pub(crate) fn from_bool(raw: bool) -> LinkTos {
        if raw {
            LinkTos::ResolveLink
        } else {
            LinkTos::NoResolution
        }
    }
}

/// Global connection settings.
#[derive(Clone, Debug)]
pub struct Settings {
    /// Maximum delay of inactivity before the client sends a heartbeat request.
    pub heartbeat_delay: Duration,

    /// Maximum delay the server has to issue a heartbeat response.
    pub heartbeat_timeout: Duration,

    /// Delay in which an operation will be retried if no response arrived.
    pub operation_timeout: Duration,

    /// Retry strategy when an operation has timeout.
    pub operation_retry: Retry,

    /// Retry strategy when failing to connect.
    pub connection_retry: Retry,

    /// 'Credentials' to use if other `Credentials` are not explicitly supplied
    /// when issuing commands.
    pub default_user: Option<Credentials>,

    /// Default connection name.
    pub connection_name: Option<String>,

    /// The period used to check pending command. Those checks include if the
    /// the connection has timeout or if the command was issued with a
    /// different connection.
    pub operation_check_period: Duration,
}

impl Default for Settings {
    fn default() -> Self {
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

/// Constants used for expected version control.
/// The use of expected version can be a bit tricky especially when discussing
/// assurances given by the GetEventStore server.
///
/// The GetEventStore server will assure idempotency for all operations using
/// any value in `ExpectedVersion` except `ExpectedVersion::Any`. When using
/// `ExpectedVersion::Any`, the GetEventStore server will do its best to assure
/// idempotency but will not guarantee idempotency.
#[derive(Copy, Clone, Debug)]
pub enum ExpectedVersion {
    /// This write should not conflict with anything and should always succeed.
    Any,

    /// The stream should exist. If it or a metadata stream does not exist,
    /// treats that as a concurrency problem.
    StreamExists,

    /// The stream being written to should not yet exist. If it does exist,
    /// treats that as a concurrency problem.
    NoStream,

    /// States that the last event written to the stream should have an event
    /// number matching your expected value.
    Exact(i64),
}

impl ExpectedVersion {
    pub(crate) fn to_i64(self) -> i64 {
        match self {
            ExpectedVersion::Any => -2,
            ExpectedVersion::StreamExists => -4,
            ExpectedVersion::NoStream => -1,
            ExpectedVersion::Exact(n) => n,
        }
    }

    pub(crate) fn from_i64(ver: i64) -> ExpectedVersion {
        match ver {
            -2 => ExpectedVersion::Any,
            -4 => ExpectedVersion::StreamExists,
            -1 => ExpectedVersion::NoStream,
            _ => ExpectedVersion::Exact(ver),
        }
    }
}

/// A structure referring to a potential logical record position in the
/// GetEventStore transaction file.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    /// Commit position of the record.
    pub commit: i64,

    /// Prepare position of the record.
    pub prepare: i64,
}

impl Position {
    /// Points to the begin of the transaction file.
    pub fn start() -> Position {
        Position {
            commit: 0,
            prepare: 0,
        }
    }

    /// Points to the end of the transaction file.
    pub fn end() -> Position {
        Position {
            commit: -1,
            prepare: -1,
        }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Position) -> Ordering {
        self.commit
            .cmp(&other.commit)
            .then(self.prepare.cmp(&other.prepare))
    }
}

/// Returned after writing to a stream.
#[derive(Debug)]
pub struct WriteResult {
    /// Next expected version of the stream.
    pub next_expected_version: i64,

    /// `Position` of the write.
    pub position: Position,
}

/// Enumeration detailing the possible outcomes of reading a stream.
#[derive(Debug)]
pub enum ReadEventStatus<A> {
    NotFound,
    NoStream,
    Deleted,
    Success(A),
}

/// Represents the result of looking up a specific event number from a stream.
#[derive(Debug)]
pub struct ReadEventResult {
    /// Stream where the event orignates from.
    pub stream_id: String,

    /// Sequence number of the event.
    pub event_number: i64,

    /// Event data.
    pub event: ResolvedEvent,
}

/// Represents a previously written event.
#[derive(Debug)]
pub struct RecordedEvent {
    /// The event stream that events belongs to.
    pub event_stream_id: String,

    /// Unique identifier representing this event.
    pub event_id: Uuid,

    /// Number of this event in the stream.
    pub event_number: i64,

    /// Type of this event.
    pub event_type: String,

    /// Payload of this event.
    pub data: Bytes,

    /// Representing the metadata associated with this event.
    pub metadata: Bytes,

    /// Indicates wheter the content is internally marked as JSON.
    pub is_json: bool,

    /// Representing when this event was created in the database system.
    /// TODO - Gives back an UTC time instead.
    pub created: Option<i64>,

    /// Representing when this event was created in the database system in
    /// epoch time.
    pub created_epoch: Option<i64>,
}

fn decode_bytes_error(err: BytesError) -> ::std::io::Error {
    ::std::io::Error::new(::std::io::ErrorKind::Other, format!("BytesError {}", err))
}

impl RecordedEvent {
    pub(crate) fn new(mut event: messages::EventRecord) -> ::std::io::Result<RecordedEvent> {
        let event_stream_id = event.take_event_stream_id().deref().to_owned();
        let event_id = guid::to_uuid(event.get_event_id()).map_err(decode_bytes_error)?;
        let event_number = event.get_event_number();
        let event_type = event.take_event_type().deref().to_owned();
        let data = event.take_data();
        let metadata = event.take_metadata();

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

    /// Tries to decode this event payload as a JSON object.
    pub fn as_json<'a, T>(&'a self) -> serde_json::Result<T>
    where
        T: Deserialize<'a>,
    {
        serde_json::from_slice(&self.data[..])
    }
}

/// A structure representing a single event or an resolved link event.
#[derive(Debug)]
pub struct ResolvedEvent {
    /// The event, or the resolved link event if this `ResolvedEvent` is a link
    /// event.
    pub event: Option<RecordedEvent>,

    /// The link event if this `ResolvedEvent` is a link event.
    pub link: Option<RecordedEvent>,

    /// Possible `Position` of that event in the server transaction file.
    pub position: Option<Position>,
}

impl ResolvedEvent {
    pub(crate) fn new(mut msg: messages::ResolvedEvent) -> ::std::io::Result<ResolvedEvent> {
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

    pub(crate) fn new_from_indexed(
        mut msg: messages::ResolvedIndexedEvent,
    ) -> ::std::io::Result<ResolvedEvent> {
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

    /// If it's a link event with its associated resolved event.
    pub fn is_resolved(&self) -> bool {
        self.event.is_some() && self.link.is_some()
    }

    /// Returns the event that was read or which triggered the subscription.
    /// If this `ResolvedEvent` represents a link event, the link will be the
    /// orginal event, otherwise it will be the event.
    ///
    pub fn get_original_event(&self) -> &RecordedEvent {
        self.link.as_ref().unwrap_or_else(|| {
            self.event
                .as_ref()
                .expect("[get_original_event] Not supposed to happen!")
        })
    }

    /// Returns the stream id of the original event.
    pub fn get_original_stream_id(&self) -> &str {
        let event = self.get_original_event();

        event.event_stream_id.deref()
    }
}

/// Represents stream metadata as a series of properties for system data and
/// user-defined metadata.
#[derive(Debug, Clone)]
pub enum StreamMetadataResult {
    Deleted { stream: String },
    NotFound { stream: String },
    Success(Box<VersionedMetadata>),
}

/// Represents a stream metadata.
#[derive(Debug, Clone)]
pub struct VersionedMetadata {
    /// Metadata's stream.
    pub stream: String,

    /// Metadata's version.
    pub version: i64,

    /// Metadata properties.
    pub metadata: StreamMetadata,
}

/// The id of a transaction.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct TransactionId(pub i64);

impl TransactionId {
    pub(crate) fn new(id: i64) -> TransactionId {
        TransactionId(id)
    }
}

/// Represents the direction of read operation (both from '$all' and a regular
/// stream).
#[derive(Copy, Clone, Debug)]
pub enum ReadDirection {
    Forward,
    Backward,
}

/// Indicates either if we reach the end of a stream or a slice of events when
/// reading a stream. `LocatedEvents` is polymorphic so it can be used by
/// either '$all' stream (that requires `Position`) and regular stream that
/// requires event sequence numer.
#[derive(Debug)]
pub enum LocatedEvents<A> {
    /// Indicates the end of the stream has been reached.
    EndOfStream,

    /// Gives the read events and a possible starting position for the next
    /// batch. if `next` is `None`, it means we have reached the end of the
    /// stream.
    Events {
        events: Vec<ResolvedEvent>,
        next: Option<A>,
    },
}

impl<A> LocatedEvents<A> {
    /// Indicates if we have reached the end of the stream we read.
    pub fn is_end_of_stream(&self) -> bool {
        match *self {
            LocatedEvents::EndOfStream => true,
            LocatedEvents::Events { ref next, .. } => next.is_some(),
        }
    }
}

/// Gathers common slice operations.
pub trait Slice {
    /// What kind of location this slice supports.
    type Location: Copy;

    /// Returns the starting point of that slice.
    fn from(&self) -> Self::Location;

    /// Returns the read direction used to fetch that slice.
    fn direction(&self) -> ReadDirection;

    /// Returns the events fetched by that slice.
    fn events(self) -> LocatedEvents<Self::Location>;
}

/// Represents the errors that can arise when reading a stream.
#[derive(Debug, Clone)]
pub enum ReadStreamError {
    NoStream(String),
    StreamDeleted(String),
    NotModified(String),
    Error(String),
    AccessDenied(String),
}

/// Represents the result of reading a stream.
#[derive(Debug, Clone)]
pub enum ReadStreamStatus<A> {
    Success(A),
    Error(ReadStreamError),
}

/// Represents the slice returned when reading a regular stream.
#[derive(Debug)]
pub struct StreamSlice {
    _from: i64,
    _direction: ReadDirection,
    _events: Vec<ResolvedEvent>,
    _next_num_opt: Option<i64>,
}

impl StreamSlice {
    pub(crate) fn new(
        direction: ReadDirection,
        from: i64,
        events: Vec<ResolvedEvent>,
        next_num_opt: Option<i64>,
    ) -> StreamSlice {
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
                },
            }
        }
    }
}

/// Represents a slice of the '$all' stream.
#[derive(Debug)]
pub struct AllSlice {
    from: Position,
    direction: ReadDirection,
    events: Vec<ResolvedEvent>,
    next: Position,
}

impl AllSlice {
    pub(crate) fn new(
        direction: ReadDirection,
        from: Position,
        events: Vec<ResolvedEvent>,
        next: Position,
    ) -> AllSlice {
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

pub enum Payload {
    Json(Bytes),
    Binary(Bytes),
}

impl Payload {
    pub fn is_json(&self) -> bool {
        match *self {
            Payload::Json(_) => true,
            _ => false,
        }
    }

    pub fn into_inner(self) -> Bytes {
        match self {
            Payload::Json(bytes) => bytes,
            Payload::Binary(bytes) => bytes,
        }
    }
}

/// Holds data of event about to be sent to the server.
pub struct EventData {
    event_type: Chars,
    payload: Payload,
    id_opt: Option<Uuid>,
    metadata_payload_opt: Option<Payload>,
}

impl EventData {
    /// Creates an event with a JSON payload.
    pub fn json<P, S>(event_type: S, payload: P) -> serde_json::Result<EventData>
    where
        P: Serialize,
        S: AsRef<str>,
    {
        let data = serde_json::to_vec(&payload)?;
        let bytes = Bytes::from(data);

        Ok(EventData {
            event_type: event_type.as_ref().into(),
            payload: Payload::Json(bytes),
            id_opt: None,
            metadata_payload_opt: None,
        })
    }

    /// Creates an event with a raw binary payload.
    pub fn binary<S>(event_type: S, payload: Bytes) -> EventData
    where
        S: AsRef<str>,
    {
        EventData {
            event_type: event_type.as_ref().into(),
            payload: Payload::Binary(payload),
            id_opt: None,
            metadata_payload_opt: None,
        }
    }

    /// Set an id to this event. By default, the id will be generated by the
    /// server.
    pub fn id(self, value: Uuid) -> EventData {
        EventData {
            id_opt: Some(value),
            ..self
        }
    }

    /// Assignes a JSON metadata to this event.
    pub fn metadata_as_json<P>(self, payload: P) -> EventData
    where
        P: Serialize,
    {
        let bytes = Bytes::from(serde_json::to_vec(&payload).unwrap());
        let json_bin = Some(Payload::Json(bytes));

        EventData {
            metadata_payload_opt: json_bin,
            ..self
        }
    }

    /// Assignes a raw binary metadata to this event.
    pub fn metadata_as_binary(self, payload: Bytes) -> EventData {
        let content_bin = Some(Payload::Binary(payload));

        EventData {
            metadata_payload_opt: content_bin,
            ..self
        }
    }

    pub(crate) fn build(self) -> messages::NewEvent {
        let mut new_event = messages::NewEvent::new();
        let id = self.id_opt.unwrap_or_else(Uuid::new_v4);

        new_event.set_event_id(guid::from_uuid(id));

        match self.payload {
            Payload::Json(bin) => {
                new_event.set_data_content_type(1);
                new_event.set_data(bin);
            }

            Payload::Binary(bin) => {
                new_event.set_data_content_type(0);
                new_event.set_data(bin);
            }
        }

        match self.metadata_payload_opt {
            Some(Payload::Json(bin)) => {
                new_event.set_metadata_content_type(1);
                new_event.set_metadata(bin);
            }

            Some(Payload::Binary(bin)) => {
                new_event.set_metadata_content_type(0);
                new_event.set_metadata(bin);
            }

            None => new_event.set_metadata_content_type(0),
        }

        new_event.set_event_type(self.event_type);

        new_event
    }
}

/// Used to facilitate the creation of a stream's metadata.
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
    /// Creates a `StreamMetadata` initialized with default values.
    pub fn new() -> StreamMetadataBuilder {
        Default::default()
    }

    /// Sets a sliding window based on the number of items in the stream.
    /// When data reaches a certain length it disappears automatically
    /// from the stream and is considered eligible for scavenging.
    pub fn max_count(self, value: u64) -> StreamMetadataBuilder {
        StreamMetadataBuilder {
            max_count: Some(value),
            ..self
        }
    }

    /// Sets a sliding window based on dates. When data reaches a certain age
    /// it disappears automatically from the stream and is considered
    /// eligible for scavenging.
    pub fn max_age(self, value: Duration) -> StreamMetadataBuilder {
        StreamMetadataBuilder {
            max_age: Some(value),
            ..self
        }
    }

    /// Sets the event number from which previous events can be scavenged.
    pub fn truncate_before(self, value: u64) -> StreamMetadataBuilder {
        StreamMetadataBuilder {
            truncate_before: Some(value),
            ..self
        }
    }

    /// This controls the cache of the head of a stream. Most URIs in a stream
    /// are infinitely cacheable but the head by default will not cache. It
    /// may be preferable in some situations to set a small amount of caching
    /// on the head to allow intermediaries to handle polls (say 10 seconds).
    pub fn cache_control(self, value: Duration) -> StreamMetadataBuilder {
        StreamMetadataBuilder {
            cache_control: Some(value),
            ..self
        }
    }

    /// Sets the ACL of a stream.
    pub fn acl(self, value: StreamAcl) -> StreamMetadataBuilder {
        StreamMetadataBuilder {
            acl: Some(value),
            ..self
        }
    }

    /// Adds user-defined property in the stream metadata.
    pub fn insert_custom_property<V>(mut self, key: String, value: V) -> StreamMetadataBuilder
    where
        V: Serialize,
    {
        let serialized = serde_json::to_value(value).unwrap();
        let _ = self.properties.insert(key, serialized);

        self
    }

    /// Returns a properly configured `StreamMetaData`.
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

/// Represents stream metadata with strongly types properties for system values
/// and a dictionary-like interface for custom values.
#[derive(Debug, Default, Clone)]
pub struct StreamMetadata {
    /// A sliding window based on the number of items in the stream. When data reaches
    /// a certain length it disappears automatically from the stream and is considered
    /// eligible for scavenging.
    pub max_count: Option<u64>,

    /// A sliding window based on dates. When data reaches a certain age it disappears
    /// automatically from the stream and is considered eligible for scavenging.
    pub max_age: Option<Duration>,

    /// The event number from which previous events can be scavenged. This is
    /// used to implement soft-deletion of streams.
    pub truncate_before: Option<u64>,

    /// Controls the cache of the head of a stream. Most URIs in a stream are infinitely
    /// cacheable but the head by default will not cache. It may be preferable
    /// in some situations to set a small amount of caching on the head to allow
    /// intermediaries to handle polls (say 10 seconds).
    pub cache_control: Option<Duration>,

    /// The access control list for the stream.
    pub acl: StreamAcl,

    /// An enumerable of key-value pairs of keys to JSON value for
    /// user-provided metadata.
    pub custom_properties: HashMap<String, serde_json::Value>,
}

impl StreamMetadata {
    /// Initializes a fresh stream metadata builder.
    pub fn builder() -> StreamMetadataBuilder {
        StreamMetadataBuilder::new()
    }
}

/// Represents an access control list for a stream.
#[derive(Default, Debug, Clone)]
pub struct StreamAcl {
    /// Roles and users permitted to read the stream.
    pub read_roles: Option<Vec<String>>,

    /// Roles and users permitted to write to the stream.
    pub write_roles: Option<Vec<String>>,

    /// Roles and users permitted to delete to the stream.
    pub delete_roles: Option<Vec<String>>,

    /// Roles and users permitted to read stream metadata.
    pub meta_read_roles: Option<Vec<String>>,

    /// Roles and users permitted to write stream metadata.
    pub meta_write_roles: Option<Vec<String>>,
}

/// Read part of a persistent subscription, isomorphic to a stream of events.
pub struct PersistentSubRead {
    pub(crate) inner: Box<dyn Stream<Item = PersistentSubEvent> + Send + Unpin>,
}

impl PersistentSubRead {
    pub fn into_inner(self) -> Box<dyn Stream<Item = PersistentSubEvent> + Send + Unpin> {
        self.inner
    }

    pub async fn read_next(&mut self) -> Option<PersistentSubEvent> {
        use futures::stream::StreamExt;

        self.inner.next().await
    }
}

/// Write part of a persistent subscription. Used to either acknowledge or report error on a persistent
/// subscription.
pub struct PersistentSubWrite {
    pub(crate) sub_id: protobuf::Chars,
    pub(crate) sender: Sender<Msg>,
}

impl PersistentSubWrite {
    /// Acknowledges a batch of event ids have been processed successfully.
    async fn ack<I>(&mut self, ids: I)
    where
        I: Iterator<Item = Uuid>,
    {
        let mut msg = messages::PersistentSubscriptionAckEvents::new();

        msg.set_processed_event_ids(ids.map(guid::from_uuid).collect());
        msg.set_subscription_id(self.sub_id.clone());

        let pkg = Pkg::from_message(Cmd::PersistentSubscriptionAckEvents, None, &msg)
            .expect("We expect serializing ack message will succeed");

        let _ = self.sender.send(Msg::Send(pkg)).await;
    }

    /// Acknowledges a batch of event ids has failed during process.
    async fn nak<I, S>(&mut self, ids: I, action: NakAction, reason: S)
    where
        I: Iterator<Item = Uuid>,
        S: AsRef<str>,
    {
        let mut msg = messages::PersistentSubscriptionNakEvents::new();

        msg.set_processed_event_ids(ids.map(guid::from_uuid).collect());
        msg.set_subscription_id(self.sub_id.clone());
        msg.set_message(reason.as_ref().into());
        msg.set_action(action.build_internal_nak_action());

        let pkg = Pkg::from_message(Cmd::PersistentSubscriptionNakEvents, None, &msg)
            .expect("We expect serializing nak message will succeed");

        let _ = self.sender.send(Msg::Send(pkg)).await;
    }

    /// Acknowledges a batch of `ResolvedEvent`s has been processed successfully.
    pub async fn ack_events<I>(&mut self, events: I)
    where
        I: Iterator<Item = PersistentSubEvent>,
    {
        self.ack(events.map(|event| event.inner.get_original_event().event_id))
            .await
    }

    /// Acknowledges a batch of `ResolvedEvent`'s has failed during process.
    pub async fn nak_events<I, S>(&mut self, events: I, action: NakAction, reason: S)
    where
        I: Iterator<Item = PersistentSubEvent>,
        S: AsRef<str>,
    {
        let ids = events.map(|event| event.inner.get_original_event().event_id);

        self.nak(ids, action, reason).await
    }

    /// Acknowledges a `ResolvedEvent` has been processed successfully.
    pub async fn ack_event(&mut self, event: PersistentSubEvent) {
        self.ack_events(vec![event].into_iter()).await
    }

    /// Acknowledges a `ResolvedEvent` has failed during process.
    pub async fn nak_event<S>(&mut self, event: PersistentSubEvent, action: NakAction, reason: S)
    where
        S: AsRef<str>,
    {
        self.nak_events(vec![event].into_iter(), action, reason)
            .await
    }
}

pub(crate) enum SubEvent {
    Confirmed,

    EventAppeared {
        event: Box<ResolvedEvent>,
        retry_count: usize,
    },

    Dropped,
}

#[derive(Debug)]
pub struct PersistentSubEvent {
    pub inner: ResolvedEvent,
    pub retry_count: usize,
}

/// Gathers every possible Nak actions.
#[derive(Debug, PartialEq, Eq)]
pub enum NakAction {
    /// Client unknown on action. Let server decide.
    Unknown,

    /// Park message do not resend. Put on poison queue.
    Park,

    /// Explicity retry the message.
    Retry,

    /// Skip this message do not resend do not put in poison queue.
    Skip,

    /// Stop the subscription.
    Stop,
}

impl NakAction {
    fn build_internal_nak_action(self) -> messages::PersistentSubscriptionNakEvents_NakAction {
        match self {
            NakAction::Unknown => messages::PersistentSubscriptionNakEvents_NakAction::Unknown,
            NakAction::Retry => messages::PersistentSubscriptionNakEvents_NakAction::Retry,
            NakAction::Skip => messages::PersistentSubscriptionNakEvents_NakAction::Skip,
            NakAction::Park => messages::PersistentSubscriptionNakEvents_NakAction::Park,
            NakAction::Stop => messages::PersistentSubscriptionNakEvents_NakAction::Stop,
        }
    }
}

/// System supported consumer strategies for use with persistent subscriptions.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SystemConsumerStrategy {
    /// Distributes events to a single client until the bufferSize is reached.
    /// After which the next client is selected in a round robin style,
    /// and the process is repeated.
    DispatchToSingle,

    /// Distributes events to all clients evenly. If the client buffer-size
    /// is reached the client is ignored until events are
    /// acknowledged/not acknowledged.
    RoundRobin,

    /// For use with an indexing projection such as the system $by_category
    /// projection. Event Store inspects event for its source stream id,
    /// hashing the id to one of 1024 buckets assigned to individual clients.
    /// When a client disconnects it's buckets are assigned to other clients.
    /// When a client connects, it is assigned some of the existing buckets.
    /// This naively attempts to maintain a balanced workload.
    /// The main aim of this strategy is to decrease the likelihood of
    /// concurrency and ordering issues while maintaining load balancing.
    /// This is not a guarantee, and you should handle the usual ordering
    /// and concurrency issues.
    Pinned,
}

impl SystemConsumerStrategy {
    pub(crate) fn as_str(&self) -> &str {
        match *self {
            SystemConsumerStrategy::DispatchToSingle => "DispatchToSingle",
            SystemConsumerStrategy::RoundRobin => "RoundRobin",
            SystemConsumerStrategy::Pinned => "Pinned",
        }
    }
}

/// Gathers every persistent subscription property.
#[derive(Debug, Clone, Copy)]
pub struct PersistentSubscriptionSettings {
    /// Whether or not the persistent subscription shoud resolve 'linkTo'
    /// events to their linked events.
    pub resolve_link_tos: bool,

    /// Where the subscription should start from (event number).
    pub start_from: i64,

    /// Whether or not in depth latency statistics should be tracked on this
    /// subscription.
    pub extra_stats: bool,

    /// The amount of time after which a message should be considered to be
    /// timeout and retried.
    pub msg_timeout: Duration,

    /// The maximum number of retries (due to timeout) before a message get
    /// considered to be parked.
    pub max_retry_count: u16,

    /// The size of the buffer listenning to live messages as they happen.
    pub live_buf_size: u16,

    /// The number of events read at a time when paging in history.
    pub read_batch_size: u16,

    /// The number of events to cache when paging through history.
    pub history_buf_size: u16,

    /// The amount of time to try checkpoint after.
    pub checkpoint_after: Duration,

    /// The minimum number of messages to checkpoint.
    pub min_checkpoint_count: u16,

    /// The maximum number of messages to checkpoint. If this number is reached
    /// , a checkpoint will be forced.
    pub max_checkpoint_count: u16,

    /// The maximum number of subscribers allowed.
    pub max_subs_count: u16,

    /// The strategy to use for distributing events to client consumers.
    pub named_consumer_strategy: SystemConsumerStrategy,
}

impl PersistentSubscriptionSettings {
    pub fn default() -> PersistentSubscriptionSettings {
        PersistentSubscriptionSettings {
            resolve_link_tos: false,
            start_from: -1, // Means the stream doesn't exist yet.
            extra_stats: false,
            msg_timeout: Duration::from_secs(30),
            max_retry_count: 10,
            live_buf_size: 500,
            read_batch_size: 20,
            history_buf_size: 500,
            checkpoint_after: Duration::from_secs(2),
            min_checkpoint_count: 10,
            max_checkpoint_count: 1000,
            max_subs_count: 0, // Means their is no limit.
            named_consumer_strategy: SystemConsumerStrategy::RoundRobin,
        }
    }
}

impl Default for PersistentSubscriptionSettings {
    fn default() -> PersistentSubscriptionSettings {
        PersistentSubscriptionSettings::default()
    }
}

/// Represents the different scenarios that could happen when performing
/// a persistent subscription.
#[derive(Debug, Eq, PartialEq)]
pub enum PersistActionResult {
    Success,
    Failure(PersistActionError),
}

impl PersistActionResult {
    /// Checks if the persistent action succeeded.
    pub fn is_success(&self) -> bool {
        match *self {
            PersistActionResult::Success => true,
            _ => false,
        }
    }

    /// Checks if the persistent action failed.
    pub fn is_failure(&self) -> bool {
        !self.is_success()
    }
}

/// Enumerates all persistent action exceptions.
#[derive(Debug, Eq, PartialEq)]
pub enum PersistActionError {
    /// The action failed.
    Fail,

    /// Happens when creating a persistent subscription on a stream with a
    /// group name already taken.
    AlreadyExists,

    /// An operation tried to do something on a persistent subscription or a
    /// stream that don't exist.
    DoesNotExist,

    /// The current user is not allowed to operate on the supplied stream or
    /// persistent subscription.
    AccessDenied,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Endpoint {
    pub addr: SocketAddr,
}

impl std::fmt::Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.addr)
    }
}

impl Endpoint {
    pub(crate) fn from_addr(addr: SocketAddr) -> Endpoint {
        Endpoint { addr }
    }
}

/// Represents a source of cluster gossip.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct GossipSeed {
    /// The endpoint for the external HTTP endpoint of the gossip seed. The
    /// HTTP endpoint is used rather than the TCP endpoint because it is
    /// required for the client to exchange gossip with the server.
    /// standard port which should be used here in 2113.
    pub(crate) endpoint: Endpoint,
}

impl std::fmt::Display for GossipSeed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "endpoint: {}", self.endpoint.addr)
    }
}

impl GossipSeed {
    /// Creates a gossip seed.
    pub fn new<A>(addrs: A) -> std::io::Result<GossipSeed>
    where
        A: ToSocketAddrs,
    {
        let mut iter = addrs.to_socket_addrs()?;

        if let Some(addr) = iter.next() {
            let endpoint = Endpoint { addr };

            Ok(GossipSeed::from_endpoint(endpoint))
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Failed to resolve socket address.",
            ))
        }
    }

    pub(crate) fn from_endpoint(endpoint: Endpoint) -> GossipSeed {
        GossipSeed { endpoint }
    }

    pub(crate) fn from_socket_addr(addr: SocketAddr) -> GossipSeed {
        GossipSeed::from_endpoint(Endpoint::from_addr(addr))
    }

    pub(crate) fn url(self) -> std::io::Result<reqwest::Url> {
        let url_str = format!("http://{}/gossip?format=json", self.endpoint.addr);

        reqwest::Url::parse(&url_str).map_err(|error| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Wrong url [{}]: {}", url_str, error),
            )
        })
    }
}

/// Indicates which order of preferred nodes for connecting to.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum NodePreference {
    /// When attempting connnection, prefers master node.
    /// TODO - Not implemented yet.
    Master,

    /// When attempting connnection, prefers slave node.
    /// TODO - Not implemented yet.
    Slave,

    /// When attempting connnection, has no node preference.
    Random,
}

impl std::fmt::Display for NodePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::NodePreference::*;

        match self {
            Master => write!(f, "Master"),
            Slave => write!(f, "Slave"),
            Random => write!(f, "Random"),
        }
    }
}

#[derive(Debug)]
/// Contains settings related to a cluster of fixed nodes.
pub struct GossipSeedClusterSettings {
    pub(crate) seeds: vec1::Vec1<GossipSeed>,
    pub(crate) preference: NodePreference,
    pub(crate) gossip_timeout: Duration,
    pub(crate) max_discover_attempts: usize,
}

impl GossipSeedClusterSettings {
    /// Creates a `GossipSeedClusterSettings` from a non-empty list of gossip
    /// seeds.
    pub fn new(seeds: vec1::Vec1<GossipSeed>) -> GossipSeedClusterSettings {
        GossipSeedClusterSettings {
            seeds,
            preference: NodePreference::Random,
            gossip_timeout: Duration::from_secs(1),
            max_discover_attempts: 10,
        }
    }

    /// Maximum duration a node should take when requested a gossip request.
    pub fn set_gossip_timeout(self, gossip_timeout: Duration) -> GossipSeedClusterSettings {
        GossipSeedClusterSettings {
            gossip_timeout,
            ..self
        }
    }

    /// Maximum number of retries during a discovery process. Discovery process
    /// is when the client tries to figure out the best node to connect to.
    pub fn set_max_discover_attempts(self, max_attempt: usize) -> GossipSeedClusterSettings {
        GossipSeedClusterSettings {
            max_discover_attempts: max_attempt,
            ..self
        }
    }
}

mod guid {
    use bytes::Bytes;
    use uuid::{BytesError, Uuid};

    pub(crate) fn from_uuid(uuid: Uuid) -> Bytes {
        let b = uuid.as_bytes();

        Bytes::from(vec![
            b[3], b[2], b[1], b[0], b[5], b[4], b[7], b[6], b[8], b[9], b[10], b[11], b[12], b[13],
            b[14], b[15],
        ])
    }

    pub(crate) fn to_uuid(b: &[u8]) -> Result<Uuid, BytesError> {
        Uuid::from_slice(&[
            b[3], b[2], b[1], b[0], b[5], b[4], b[7], b[6], b[8], b[9], b[10], b[11], b[12], b[13],
            b[14], b[15],
        ])
    }

    #[cfg(test)]
    mod test {
        use bytes::Bytes;
        use uuid::Uuid;

        #[test]
        fn test_from_uuid() {
            let uuid = Uuid::from_bytes([
                60, 213, 58, 216, 84, 211, 79, 74, 177, 22, 31, 9, 149, 122, 243, 48,
            ]);
            let expected_guid = Bytes::from_static(&[
                216, 58, 213, 60, 211, 84, 74, 79, 177, 22, 31, 9, 149, 122, 243, 48,
            ]);

            assert_eq!(super::from_uuid(uuid), expected_guid);
        }

        #[test]
        fn test_to_uuid() {
            let guid = &[
                216, 58, 213, 60, 211, 84, 74, 79, 177, 22, 31, 9, 149, 122, 243, 48,
            ];
            let expected_uuid = Uuid::from_bytes([
                60, 213, 58, 216, 84, 211, 79, 74, 177, 22, 31, 9, 149, 122, 243, 48,
            ]);

            assert_eq!(super::to_uuid(guid), Ok(expected_uuid));
        }
    }
}
