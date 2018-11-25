//! Common types used across the library.
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::Read;
use std::time::Duration;

use bytes::{ Bytes, BytesMut, BufMut, Buf };
use futures::{ Future, Stream, Sink };
use futures::stream::iter_ok;
use futures::sync::mpsc::{ Receiver, Sender };
use futures::sync::oneshot;
use protobuf::Chars;
use serde::de::Deserialize;
use serde::ser::Serialize;
use serde_json;
use uuid::{ Uuid, BytesError };

use internal::command::Cmd;
use internal::messages;
use internal::messaging::Msg;
use internal::package::Pkg;

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
            Retry::Only(x)      => x,
        }
    }
}

/// Holds login and password information.
#[derive(Clone, Debug)]
pub struct Credentials {
    login: Bytes,
    password: Bytes,
}

impl Credentials {
    /// Creates a new `Credentials` instance.
    pub fn new<S>(login: S, password: S) -> Credentials
        where S: Into<Bytes>
    {
        Credentials {
            login: login.into(),
            password: password.into(),
        }
    }

    pub(crate) fn write_to_bytes_mut(&self, dst: &mut BytesMut) {
        dst.put_u8(self.login.len() as u8);
        dst.put(&self.login);
        dst.put_u8(self.password.len() as u8);
        dst.put(&self.password);
    }

    pub(crate) fn parse_from_buf<B>(buf: &mut B) -> ::std::io::Result<Credentials>
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
            ExpectedVersion::Any          => -2,
            ExpectedVersion::StreamExists => -4,
            ExpectedVersion::NoStream     => -1,
            ExpectedVersion::Exact(n)     => n,
        }
    }

    pub(crate) fn from_i64(ver: i64) -> ExpectedVersion {
        match ver {
            -2 => ExpectedVersion::Any,
            -4 => ExpectedVersion::StreamExists,
            -1 => ExpectedVersion::NoStream,
            _  => ExpectedVersion::Exact(ver),
        }
    }
}

/// A structure referring to a potential logical record position in the
/// GetEventStore transaction file.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    /// Commit position of the record.
    pub commit:  i64,

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
        self.commit.cmp(&other.commit).then(self.prepare.cmp(&other.prepare))
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
    pub event_stream_id: Chars,

    /// Unique identifier representing this event.
    pub event_id: Uuid,

    /// Number of this event in the stream.
    pub event_number: i64,

    /// Type of this event.
    pub event_type: Chars,

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
        let event_stream_id = event.take_event_stream_id();
        let event_id        = Uuid::from_slice(event.get_event_id()).map_err(decode_bytes_error)?;
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

    /// Tries to decode this event payload as a JSON object.
    pub fn as_json<'a, T>(&'a self) -> serde_json::Result<T>
        where T: Deserialize<'a>
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

    pub(crate) fn new_from_indexed(mut msg: messages::ResolvedIndexedEvent) -> ::std::io::Result<ResolvedEvent> {
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
    /// TODO - It's impossible for `get_original_event` to be undefined.
    pub fn get_original_event(&self) -> Option<&RecordedEvent> {
        self.link.as_ref().or_else(|| self.event.as_ref())
    }

    /// Returns the stream id of the original event.
    pub fn get_original_stream_id(&self) -> Option<&Chars> {
        self.get_original_event().map(|event| &event.event_stream_id)
    }
}

/// Represents stream metadata as a series of properties for system data and
/// user-defined metadata.
#[derive(Debug, Clone)]
pub enum StreamMetadataResult {
    Deleted { stream: Chars },
    NotFound { stream: Chars },
    Success(Box<VersionedMetadata>),
}

/// Represents a stream metadata.
#[derive(Debug, Clone)]
pub struct VersionedMetadata {
    /// Metadata's stream.
    pub stream: Chars,

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
    Events { events: Vec<ResolvedEvent>, next: Option<A> },
}

impl <A> LocatedEvents<A> {
    /// Indicates if we have reached the end of the stream we read.
    pub fn is_end_of_stream(&self) -> bool {
        match *self {
            LocatedEvents::EndOfStream            => true,
            LocatedEvents::Events { ref next, ..} => next.is_some(),
        }
    }
}

/// Gathers common slice operations.
pub trait Slice {
    /// What kind of location this slice supports.
    type Location;

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
    NoStream(Chars),
    StreamDeleted(Chars),
    NotModified(Chars),
    Error(Chars),
    AccessDenied(Chars),
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

/// Holds data of event about to be sent to the server.
pub struct EventData {
    event_type: Chars,
    payload: Payload,
    id_opt: Option<Uuid>,
    metadata_payload_opt: Option<Payload>,
}

impl EventData {
    /// Creates an event with a JSON payload.
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

    /// Creates an event with a raw binary payload.
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

    /// Set an id to this event. By default, the id will be generated by the
    /// server.
    pub fn id(self, value: Uuid) -> EventData {
        EventData { id_opt: Some(value), ..self }
    }

    /// Assignes a JSON metadata to this event.
    pub fn metadata_as_json<P>(self, payload: P) -> EventData
        where P: Serialize
    {
        let bytes    = Bytes::from(serde_json::to_vec(&payload).unwrap());
        let json_bin = Some(Payload::Json(bytes));

        EventData { metadata_payload_opt: json_bin, ..self }
    }

    /// Assignes a raw binary metadata to this event.
    pub fn metadata_as_binary(self, payload: Bytes) -> EventData {
        let content_bin = Some(Payload::Binary(payload));

        EventData { metadata_payload_opt: content_bin, ..self }
    }

    pub(crate) fn build(self) -> messages::NewEvent {
        let mut new_event = messages::NewEvent::new();
        let     id        = self.id_opt.unwrap_or_else(Uuid::new_v4);

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

    /// Sets the maximum number of events allowed in the stream.
    pub fn max_count(self, value: u64) -> StreamMetadataBuilder {
        StreamMetadataBuilder { max_count: Some(value), ..self }
    }

    /// Sets the maximum age of events allowed in the stream.
    pub fn max_age(self, value: Duration) -> StreamMetadataBuilder {
        StreamMetadataBuilder { max_age: Some(value), ..self }
    }

    /// Sets the event number from which previous events can be scavenged.
    pub fn truncate_before(self, value: u64) -> StreamMetadataBuilder {
        StreamMetadataBuilder { truncate_before: Some(value), ..self }
    }

    /// Sets the amount of time for which the stream head is cacheable.
    pub fn cache_control(self, value: Duration) -> StreamMetadataBuilder {
        StreamMetadataBuilder { cache_control: Some(value), ..self }
    }

    /// Sets the ACL of a stream.
    pub fn acl(self, value: StreamAcl) -> StreamMetadataBuilder {
        StreamMetadataBuilder { acl: Some(value), ..self }
    }

    /// Adds user-defined property in the stream metadata.
    pub fn insert_custom_property<V>(mut self, key: String, value: V) -> StreamMetadataBuilder
        where V: Serialize
    {
        let serialized = serde_json::to_value(value).unwrap();
        let _          = self.properties.insert(key, serialized);

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
    /// The maximum number of events allowed in the stream.
    pub max_count: Option<u64>,

    /// The maximum age of events allowed in the stream.
    pub max_age: Option<Duration>,

    /// The event number from which previous events can be scavenged. This is
    /// used to implement soft-deletion of streams.
    pub truncate_before: Option<u64>,

    /// The amount of time for which the stream head is cacheable.
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
    pub read_roles: Vec<String>,

    /// Roles and users permitted to write to the stream.
    pub write_roles: Vec<String>,

    /// Roles and users permitted to delete to the stream.
    pub delete_roles: Vec<String>,

    /// Roles and users permitted to read stream metadata.
    pub meta_read_roles: Vec<String>,

    /// Roles and users permitted to write stream metadata.
    pub meta_write_roles: Vec<String>,
}

pub(crate) enum SubEvent {
    Confirmed {
        id: Uuid,
        last_commit_position: i64,
        last_event_number: i64,
        // If defined, it means we are in a persistent subscription.
        persistent_id: Option<Chars>,
    },

    EventAppeared {
        event: Box<ResolvedEvent>,
        retry_count: usize,
    },

    HasBeenConfirmed(oneshot::Sender<()>),
    Dropped
}

impl SubEvent {
    pub(crate) fn event_appeared(&self) -> Option<&ResolvedEvent> {
        match self {
            SubEvent::EventAppeared{ ref event, .. } => Some(event),
            _                                        => None,
        }
    }

    pub(crate) fn new_event_appeared(event: ResolvedEvent) -> SubEvent {
        SubEvent::EventAppeared {
            event: Box::new(event),
            retry_count: 0,
        }
    }
}

struct State<A: SubscriptionConsumer> {
    consumer: A,
    confirmation_id: Option<Uuid>,
    persistent_id: Option<Chars>,
    confirmation_requests: Vec<oneshot::Sender<()>>,
    buffer: BytesMut,
}

impl <A: SubscriptionConsumer> State<A> {
    fn new(consumer: A) -> State<A> {
        State {
            consumer,
            confirmation_id: None,
            persistent_id: None,
            confirmation_requests: Vec::new(),
            buffer: BytesMut::new(),
        }
    }

    fn drain_requests(&mut self) {
        for req in self.confirmation_requests.drain(..) {
            let _ = req.send(());
        }
    }
}

enum OnEvent {
    Continue,
    Stop,
}

impl OnEvent {
    fn is_stop(&self) -> bool {
        match *self {
            OnEvent::Continue => false,
            OnEvent::Stop     => true,
        }
    }
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
    fn build_internal_nak_action(self)
        -> messages::PersistentSubscriptionNakEvents_NakAction
    {
        match self {
            NakAction::Unknown => messages::PersistentSubscriptionNakEvents_NakAction::Unknown,
            NakAction::Retry   => messages::PersistentSubscriptionNakEvents_NakAction::Retry,
            NakAction::Skip    => messages::PersistentSubscriptionNakEvents_NakAction::Skip,
            NakAction::Park    => messages::PersistentSubscriptionNakEvents_NakAction::Park,
            NakAction::Stop    => messages::PersistentSubscriptionNakEvents_NakAction::Stop,
        }
    }
}

fn on_event<C>(
    sender: &Sender<Msg>,
    state: &mut State<C>,
    event: SubEvent
) -> OnEvent
    where
        C: SubscriptionConsumer
{
    match event {
        SubEvent::Confirmed { id, last_commit_position, last_event_number, persistent_id } => {
            state.confirmation_id = Some(id);
            state.persistent_id = persistent_id;
            state.drain_requests();
            state.consumer.when_confirmed(id, last_commit_position, last_event_number);
        },

        SubEvent::EventAppeared { event, retry_count } => {
            let decision = match state.persistent_id.as_ref() {
                Some(sub_id) => {
                    let mut env  = PersistentSubscriptionEnv::new(retry_count);
                    let decision = state.consumer.when_event_appeared(&mut env, event);

                    let acks = env.acks;

                    if !acks.is_empty() {
                        let mut msg = messages::PersistentSubscriptionAckEvents::new();

                        msg.set_subscription_id(sub_id.clone());

                        for id in acks {
                            // Reserves enough to store an UUID (which is 16 bytes long).
                            state.buffer.reserve(16);
                            state.buffer.put_slice(id.as_bytes());

                            let bytes = state.buffer.take().freeze();
                            msg.mut_processed_event_ids().push(bytes);
                        }

                        let pkg = Pkg::from_message(
                            Cmd::PersistentSubscriptionAckEvents,
                            None,
                            &msg
                        ).unwrap();

                        sender.clone().send(Msg::Send(pkg)).wait().unwrap();
                    }

                    let naks     = env.naks;
                    let mut pkgs = Vec::new();

                    if !naks.is_empty() {
                        for naked in naks {
                            let mut msg       = messages::PersistentSubscriptionNakEvents::new();
                            let mut bytes_vec = Vec::with_capacity(naked.ids.len());

                            msg.set_subscription_id(sub_id.clone());

                            for id in naked.ids {
                                // Reserves enough to store an UUID (which is 16 bytes long).
                                state.buffer.reserve(16);
                                state.buffer.put_slice(id.as_bytes());

                                let bytes = state.buffer.take().freeze();
                                bytes_vec.push(bytes);
                            }

                            msg.set_processed_event_ids(bytes_vec);
                            msg.set_message(naked.message);
                            msg.set_action(naked.action.build_internal_nak_action());

                            let pkg = Pkg::from_message(
                                Cmd::PersistentSubscriptionAckEvents,
                                None,
                                &msg
                            ).unwrap();

                            pkgs.push(pkg);
                        }

                        let pkgs = pkgs.into_iter().map(Msg::Send);

                        sender.clone().send_all(iter_ok(pkgs)).wait().unwrap();
                    }

                    decision
                },

                None => {
                   state.consumer.when_event_appeared(&mut NoopSubscriptionEnv, event)
                },
            };

            if let OnEventAppeared::Drop = decision {
                let id  = state.confirmation_id.expect("impossible situation when dropping subscription");
                let pkg = Pkg::new(Cmd::UnsubscribeFromStream, id);

                sender.clone().send(Msg::Send(pkg)).wait().unwrap();
                return OnEvent::Stop;
            }
        },

        SubEvent::Dropped => {
            state.consumer.when_dropped();
            state.drain_requests();
        },

        SubEvent::HasBeenConfirmed(req) => {
            if state.confirmation_id.is_some() {
                let _ = req.send(());
            } else {
                state.confirmation_requests.push(req);
            }
        },
    };

    OnEvent::Continue
}

/// Represents the common operations supported by a subscription.
pub struct Subscription {
    pub(crate) inner: Sender<SubEvent>,
    pub(crate) receiver: Receiver<SubEvent>,
    pub(crate) sender: Sender<Msg>,
}

impl Subscription {
    /// Consumes synchronously the events comming from a subscription.
    pub fn consume<C>(self, consumer: C) -> C
        where C: SubscriptionConsumer
    {
        let mut state = State::new(consumer);

        for event in self.receiver.wait() {
            if let Ok(event) = event {
                let decision = on_event(&self.sender, &mut state, event);

                if decision.is_stop() {
                    break;
                }
            } else {
                // It means the queue has been closed by the operation.
                break;
            }
        }

        state.consumer
    }

    /// Consumes asynchronously the events comming from a subscription.
    pub fn consume_async<C>(self, init: C) -> impl Future<Item=C, Error=()>
        where C: SubscriptionConsumer
    {
        let sender = self.sender.clone();

        self.receiver.fold(State::new(init), move |mut state, event| {
            match on_event(&sender, &mut state, event) {
                OnEvent::Continue => Ok::<State<C>, ()>(state),
                OnEvent::Stop     => Err(()),
            }
        }).map(|state| state.consumer)
    }

    /// You shouldn't have to use that function as it makes no sense to
    /// wait for a confirmation from the server. However, for testing
    /// purpose or weirdos, we expose that function. it returns
    /// a future waiting the subscription to be confirmed by the server.
    pub fn confirmation(&self) -> impl Future<Item=(), Error=()> {
        let (tx, rcv) = oneshot::channel();
        let _         = self.inner.clone().send(SubEvent::HasBeenConfirmed(tx)).wait();

        rcv.map_err(|_| ())
    }
}

/// Outcome to returns when a subscription dispatches an event.
#[derive(Debug, PartialEq, Eq)]
pub enum OnEventAppeared {
    /// States to continue processing the subscription.
    Continue,

    /// Asks to drop the subscription.
    Drop,
}

struct NakedEvents {
    ids: Vec<Uuid>,
    action: NakAction,
    message: Chars,
}

/// Set of operations supported when consumming events from a subscription.
/// Most of those operations are only available for persistent subscription,
/// and will be no-op when used with either volatile or catchup subscriptions.
///
/// * Notes
/// All the buffers used by `SubscriptionEnv` will get flushed once the instance
/// goes out of scope.
pub trait SubscriptionEnv {
    /// Add an event id to ack list.
    ///
    /// * Notes
    /// For persistent subscription only.
    fn push_ack(&mut self, Uuid);

    /// Add an event ids to the nak list. It asks for `Vec` so you can have
    /// different `NakAction` for different event set.
    ///
    /// * Notes
    /// For persistent subscription only.
    fn push_nak_with_message<S: Into<Chars>>(&mut self, Vec<Uuid>, NakAction, S);

    /// Get the number of time that event has been retried.
    ///
    /// * Notes
    /// For persistent subscription only.
    fn current_event_retry_count(&self) -> usize;

    /// Like `push_nak_with_message` but uses an empty message for `NakAction`.
    fn push_nak(&mut self, ids: Vec<Uuid>, action: NakAction) {
        self.push_nak_with_message(ids, action, "");
    }
}

struct NoopSubscriptionEnv;

impl SubscriptionEnv for NoopSubscriptionEnv {
    fn push_ack(&mut self, _: Uuid) {}
    fn push_nak_with_message<S: Into<Chars>>(&mut self, _: Vec<Uuid>, _: NakAction, _: S) {}
    fn current_event_retry_count(&self) -> usize { 0 }
}

struct PersistentSubscriptionEnv {
    acks: Vec<Uuid>,
    naks: Vec<NakedEvents>,
    retry_count: usize,
}

impl PersistentSubscriptionEnv {
    fn new(retry_count: usize) -> PersistentSubscriptionEnv {
        PersistentSubscriptionEnv {
            acks: Vec::new(),
            naks: Vec::new(),
            retry_count,
        }
    }
}

impl SubscriptionEnv for PersistentSubscriptionEnv {
    fn push_ack(&mut self, id: Uuid) {
        self.acks.push(id);
    }

    fn push_nak_with_message<S: Into<Chars>>(&mut self, ids: Vec<Uuid>, action: NakAction, message: S) {
        let naked = NakedEvents {
            ids,
            action,
            message: message.into(),
        };

        self.naks.push(naked);
    }

    fn current_event_retry_count(&self) -> usize {
        self.retry_count
    }
}

/// Represents the lifecycle of a subscription.
pub trait SubscriptionConsumer {
    /// Called when the subscription has been confirmed by the server.
    /// Usually, it's the first out of all callbacks to be called.
    ///
    /// * Notes
    /// It's possible to have `when_event_appeared` called before
    /// `when_confirmed` for catchup subscriptions as catchup subscriptions
    /// mixes several operations at the same time.
    fn when_confirmed(&mut self, Uuid, i64, i64);

    /// Called when the subscription has received an event from the server.
    fn when_event_appeared<E>(&mut self, &mut E, Box<ResolvedEvent>) -> OnEventAppeared
        where E: SubscriptionEnv;

    /// Called when the subscrition has been dropped whether by the server or
    /// the user themself.
    fn when_dropped(&mut self);
}

/// System supported consumer strategies for use with persistent subscriptions.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SystemConsumerStrategy {
    /// Distributes events to a single client until it is full. Then round
    /// robin to the next client.
    DispatchToSingle,

    /// Distributes events to each c lient in a round robin fashion.
    RoundRobin,
}

impl SystemConsumerStrategy {
    pub(crate) fn as_str(&self) -> &str {
        match *self {
            SystemConsumerStrategy::DispatchToSingle => "DispatchToSingle",
            SystemConsumerStrategy::RoundRobin       => "RoundRobin",
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
            max_retry_count: 500,
            live_buf_size: 500,
            read_batch_size: 10,
            history_buf_size: 20,
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
            _                            => false,
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
