use futures::sync::mpsc::Sender;
use futures::{ Future, Sink, Stream };
use internal::data::EventData;
use internal::messaging::Msg;
use internal::metadata::StreamMetadata;
use internal::operations;
use types;

type Task<A> = Box<Future<Item=A, Error=operations::OperationError>>;

fn single_value_future<S: 'static, A: 'static>(stream: S) -> Task<A>
    where S: Stream<Item = Result<A, operations::OperationError>, Error = ()>
{
    let fut = stream.into_future().then(|res| {
        match res {
            Ok((Some(x), _)) => x,
            _                => unreachable!(),
        }
    });
    Box::new(fut)
}

pub struct WriteEvents {
    stream: String,
    events: Vec<EventData>,
    require_master: bool,
    version: types::ExpectedVersion,
    creds: Option<types::Credentials>,
    sender: Sender<Msg>,
}

impl WriteEvents {
    pub fn new(sender: Sender<Msg>, stream: String) -> WriteEvents {
        WriteEvents {
            stream,
            events: Vec::new(),
            require_master: false,
            version: types::ExpectedVersion::Any,
            creds: None,
            sender,
        }
    }

    pub fn set_events(mut self, events: Vec<EventData>) -> WriteEvents {
        self.events = events;

        self
    }

    pub fn push_event(mut self, event: EventData) -> WriteEvents {
        self.events.push(event);

        self
    }

    pub fn append_events<T>(mut self, events: T) -> WriteEvents
        where T: IntoIterator<Item=EventData>
    {
        self.events.extend(events);

        self
    }

    pub fn require_master(mut self, value: bool) -> WriteEvents {
        self.require_master = value;

        self
    }

    pub fn expected_version(mut self, version: types::ExpectedVersion) -> WriteEvents {
        self.version = version;

        self
    }

    pub fn credentials(mut self, creds: types::Credentials) -> WriteEvents {
        self.creds = Some(creds);

        self
    }

    pub fn execute(self) -> Task<types::WriteResult> {
        let     (rcv, promise) = operations::Promise::new(1);
        let mut op             = operations::WriteEvents::new(promise, self.creds);

        op.set_event_stream_id(self.stream);
        op.set_expected_version(self.version);
        op.set_events(self.events);
        op.set_require_master(self.require_master);

        self.sender.send(Msg::NewOp(operations::Op::Write(op))).wait().unwrap();

        single_value_future(rcv)
    }
}

pub struct ReadEvent {
    stream: String,
    event_number: i64,
    resolve_link_tos: bool,
    require_master: bool,
    creds: Option<types::Credentials>,
    sender: Sender<Msg>,
}

impl ReadEvent {
    pub fn new(sender: Sender<Msg>, stream: String, event_number: i64) -> ReadEvent {
        ReadEvent {
            stream,
            event_number,
            sender,
            resolve_link_tos: false,
            require_master: false,
            creds: None,
        }
    }

    pub fn resolve_link_tos(mut self, value: bool) -> ReadEvent {
        self.resolve_link_tos = value;

        self
    }

    pub fn require_master(mut self, value: bool) -> ReadEvent {
        self.require_master = value;

        self
    }

    pub fn credentials(mut self, value: types::Credentials) -> ReadEvent {
        self.creds = Some(value);

        self
    }

    pub fn execute(self) -> Task<types::ReadEventStatus<types::ReadEventResult>> {
        let (rcv, promise) = operations::Promise::new(1);
        let mut op         = operations::ReadEvent::new(promise, self.creds);

        op.set_event_stream_id(self.stream);
        op.set_event_number(self.event_number);
        op.set_resolve_link_tos(self.resolve_link_tos);
        op.set_require_master(self.require_master);

        self.sender.send(Msg::NewOp(operations::Op::Read(op))).wait().unwrap();

        single_value_future(rcv)
    }
}

pub struct WriteStreamData {
    metadata: StreamMetadata,
    inner: WriteEvents,
}

impl WriteStreamData {
    pub fn new(sender: Sender<Msg>, stream: String, metadata: StreamMetadata) -> WriteStreamData {
        WriteStreamData {
            metadata,
            inner: WriteEvents::new(sender, format!("$${}", stream)),
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

    pub fn execute(self) -> Task<types::WriteResult> {
        let event = EventData::json("$metadata".to_owned(), self.metadata);

        self.inner.push_event(event)
                  .execute()
    }
}

pub struct ReadStreamData {
    stream: String,
    inner: ReadEvent,
}

impl ReadStreamData {
    pub fn new(sender: Sender<Msg>, stream: String) -> ReadStreamData {
        let name = format!("$${}", stream);

        ReadStreamData {
            stream,
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

    pub fn execute(self) -> Task<types::StreamMetadataResult> {
        let stream = self.stream;
        let fut    = self.inner.execute().map(|res| {
            match res {
                types::ReadEventStatus::Success(result) => {
                    let metadata =
                        result.event
                              .get_original_event()
                              .unwrap()
                              .as_json()
                              .unwrap();

                    types::StreamMetadataResult::Success {
                        stream: stream,
                        version: result.event_number,
                        metadata,
                    }
                },

                types::ReadEventStatus::NotFound | types::ReadEventStatus::NoStream => {
                    types::StreamMetadataResult::NotFound { stream: stream }
                },

                types::ReadEventStatus::Deleted => {
                    types::StreamMetadataResult::Deleted { stream: stream }
                },
            }
        });

        Box::new(fut)
    }
}

pub struct TransactionStart {
    stream: String,
    version: types::ExpectedVersion,
    require_master: bool,
    creds_opt: Option<types::Credentials>,
    sender: Sender<Msg>,
}

impl TransactionStart {
    pub fn new(sender: Sender<Msg>, stream: String) -> TransactionStart {
        TransactionStart {
            stream,
            require_master: false,
            version: types::ExpectedVersion::Any,
            creds_opt: None,
            sender,
        }
    }

    pub fn require_master(mut self, value: bool) -> TransactionStart {
        self.require_master = value;

        self
    }

    pub fn version(mut self, value: types::ExpectedVersion) -> TransactionStart {
        self.version = value;

        self
    }

    pub fn credentials(mut self, value: types::Credentials) -> TransactionStart {
        self.creds_opt = Some(value);

        self
    }

    pub fn execute(self) -> Task<Transaction> {
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

        self.sender.send(Msg::NewOp(operations::Op::TransactionStart(op))).wait().unwrap();

        let fut = single_value_future(rcv).map(move |id| {
            Transaction {
                stream,
                id,
                sender,
                require_master,
                creds: cloned_creds,
                version,
            }
        });

        Box::new(fut)
    }
}

pub struct Transaction {
    stream: String,
    id: types::TransactionId,
    version: types::ExpectedVersion,
    require_master: bool,
    sender: Sender<Msg>,
    creds: Option<types::Credentials>,
}

impl Transaction {
    pub fn get_it(&self) -> types::TransactionId {
        self.id
    }

    pub fn write_single(&self, event: EventData) -> Task<()> {
        self.write(::std::iter::once(event))
    }

    pub fn write<I>(&self, events: I) -> Task<()>
        where I: IntoIterator<Item=EventData>
    {
        let (rcv, promise) = operations::Promise::new(1);
        let mut op = operations::TransactionWrite::new(
            promise, self.stream.clone(), self.creds.clone());

        op.set_transaction_id(self.id);
        op.set_events(events);
        op.set_require_master(self.require_master);

        self.sender.clone().send(Msg::NewOp(operations::Op::TransactionWrite(op))).wait().unwrap();

        single_value_future(rcv)
    }

    pub fn commit(self) -> Task<types::WriteResult> {
        let (rcv, promise) = operations::Promise::new(1);
        let mut op =
            operations::TransactionCommit::new(
                promise, self.stream.clone(), self.version, self.creds.clone());

        op.set_transaction_id(self.id);
        op.set_require_master(self.require_master);

        self.sender.clone().send(Msg::NewOp(operations::Op::TransactionCommit(op))).wait().unwrap();

        single_value_future(rcv)
    }

    pub fn rollback(self) {
        // On purpose, this function does nothing. GetEventStore doesn't have a rollback operation.
        // This function is there mainly because of how transactions are perceived, meaning a
        // transaction comes with a commit and a rollback functions.
    }
}

pub struct ReadStreamEvents {
    stream: String,
    max_count: i32,
    start: i64,
    require_master: bool,
    resolve_link_tos: bool,
    direction: types::ReadDirection,
    sender: Sender<Msg>,
    creds: Option<types::Credentials>,
}

impl ReadStreamEvents {
    pub fn new(sender: Sender<Msg>, stream: String) -> ReadStreamEvents {
        ReadStreamEvents {
            stream,
            max_count: 500,
            start: 0,
            require_master: false,
            resolve_link_tos: false,
            direction: types::ReadDirection::Forward,
            sender,
            creds: None,
        }
    }

    pub fn forward(mut self) -> ReadStreamEvents {
        self.direction = types::ReadDirection::Forward;

        self
    }

    pub fn backward(mut self) -> ReadStreamEvents {
        self.direction = types::ReadDirection::Backward;

        self
    }

    pub fn credentials(mut self, value: types::Credentials) -> ReadStreamEvents {
        self.creds = Some(value);

        self
    }

    pub fn max_count(mut self, value: i32) -> ReadStreamEvents {
        self.max_count = value;

        self
    }

    pub fn start_from(mut self, value: i64) -> ReadStreamEvents {
        self.start = value;

        self
    }

    pub fn require_master(mut self, value: bool) -> ReadStreamEvents {
        self.require_master = value;

        self
    }

    pub fn resolve_link_tos(mut self, value: bool) -> ReadStreamEvents {
        self.resolve_link_tos = value;

        self
    }

    pub fn execute(self) -> Task<types::ReadStreamStatus<types::StreamSlice>> {
        let     (rcv, promise) = operations::Promise::new(1);
        let mut op             = operations::ReadStreamEvents::new(promise, self.direction, self.creds);

        op.set_event_stream_id(self.stream);
        op.set_from_event_number(self.start);
        op.set_max_count(self.max_count);
        op.set_require_master(self.require_master);
        op.set_resolve_link_tos(self.resolve_link_tos);

        self.sender.send(Msg::NewOp(operations::Op::ReadStreams(op))).wait().unwrap();

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

    pub fn forward(mut self) -> ReadAllEvents {
        self.direction = types::ReadDirection::Forward;

        self
    }

    pub fn backward(mut self) -> ReadAllEvents {
        self.direction = types::ReadDirection::Backward;

        self
    }

    pub fn credentials(mut self, value: types::Credentials) -> ReadAllEvents {
        self.creds = Some(value);

        self
    }

    pub fn max_count(mut self, value: i32) -> ReadAllEvents {
        self.max_count = value;

        self
    }

    pub fn start_from(mut self, value: types::Position) -> ReadAllEvents {
        self.start = value;

        self
    }

    pub fn require_master(mut self, value: bool) -> ReadAllEvents {
        self.require_master = value;

        self
    }

    pub fn resolve_link_tos(mut self, value: bool) -> ReadAllEvents {
        self.resolve_link_tos = value;

        self
    }

    pub fn execute(self) -> Task<types::ReadStreamStatus<types::AllSlice>> {
        let     (rcv, promise) = operations::Promise::new(1);
        let mut op             = operations::ReadAllEvents::new(promise, self.direction, self.creds);

        op.set_from_position(self.start);
        op.set_max_count(self.max_count);
        op.set_require_master(self.require_master);
        op.set_resolve_link_tos(self.resolve_link_tos);

        self.sender.send(Msg::NewOp(operations::Op::ReadAll(op))).wait().unwrap();

        single_value_future(rcv)
    }
}

pub struct DeleteStream {
    stream: String,
    require_master: bool,
    version: types::ExpectedVersion,
    creds: Option<types::Credentials>,
    hard_delete: bool,
    sender: Sender<Msg>,
}

impl DeleteStream {
    pub fn new(sender: Sender<Msg>, stream: String) -> DeleteStream {
        DeleteStream {
            stream,
            require_master: false,
            hard_delete: false,
            version: types::ExpectedVersion::Any,
            creds: None,
            sender,
        }
    }

    pub fn require_master(mut self, value: bool) -> DeleteStream {
        self.require_master = value;

        self
    }

    pub fn expected_version(mut self, version: types::ExpectedVersion) -> DeleteStream {
        self.version = version;

        self
    }

    pub fn credentials(mut self, creds: types::Credentials) -> DeleteStream {
        self.creds = Some(creds);

        self
    }

    pub fn hard_delete(mut self, value: bool) -> DeleteStream {
        self.hard_delete = value;

        self
    }

    pub fn execute(self) -> Task<types::Position> {
        let     (rcv, promise) = operations::Promise::new(1);
        let mut op             = operations::DeleteStream::new(promise, self.creds);

        op.set_event_stream_id(self.stream);
        op.set_expected_version(self.version);
        op.set_require_master(self.require_master);
        op.set_hard_delete(self.hard_delete);

        self.sender.send(Msg::NewOp(operations::Op::Delete(op))).wait().unwrap();

        single_value_future(rcv)
    }
}