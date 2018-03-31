use futures::sync::mpsc::Sender;
use futures::{ Future, Sink, Stream };
use internal::data::EventData;
use internal::messaging::Msg;
use internal::metadata::StreamMetadata;
use internal::operations;
use internal::types;

type Task<A> = Box<Future<Item=A, Error=operations::OperationError>>;

fn single_value_future<S: 'static, A: 'static>(stream: S) -> Task<A>
    where S: Stream<Item = Result<A, operations::OperationError>, Error = ()>
{
    let fut = stream.take(1).collect().then(|res| {
        match res {
            Ok(mut xs) => xs.remove(0),
            _          => unreachable!(),
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
        let event = EventData::new_json(None, "$metadata".to_owned(), self.metadata);

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
