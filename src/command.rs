use futures::sync::mpsc::Sender;
use futures::{ Future, Sink, Stream };
use internal::data::EventData;
use internal::messaging::Msg;
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
    pub stream: String,
    pub events: Vec<EventData>,
    pub require_master: bool,
    pub version: types::ExpectedVersion,
    pub creds: Option<types::Credentials>,
    pub sender: Sender<Msg>,
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

    pub fn append_events(mut self, events: &mut Vec<EventData>) -> WriteEvents {
        self.events.append(events);

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

    pub fn credentials(mut self, creds: Option<types::Credentials>) -> WriteEvents {
        self.creds = creds;

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
