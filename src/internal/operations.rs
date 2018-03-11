use uuid::Uuid;

use futures::sync::mpsc;
use protobuf::{ RepeatedField, Message };
use protobuf::core::parse_from_bytes;

use internal::command::Cmd;
use internal::data::EventData;
use internal::messages;
use internal::package::Pkg;
use internal::types::{ ExpectedVersion, Position, WriteResult, Credentials };

use self::messages::OperationResult;

#[derive(Debug)]
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
    WrongClientImpl(Cmd),
}

pub enum Outcome {
    Done,
    Continue(Option<Pkg>),
}

impl Outcome {
    pub fn produced_pkg(self) -> Option<Pkg> {
        match self {
            Outcome::Done              => None,
            Outcome::Continue(pkg_opt) => pkg_opt,
        }
    }

    pub fn is_continuing(&self) -> bool {
        if let Outcome::Continue(_) = *self {
            true
        } else {
            false
        }
    }
}

pub type Decision = ::std::io::Result<Outcome>;

pub struct Promise<A> {
    inner: mpsc::Sender<Result<A, OperationError>>,
}

impl <A> Promise<A> {
    pub fn new(buffer: usize) -> (mpsc::Receiver<Result<A, OperationError>>, Promise<A>) {
        let (tx, rcv) = mpsc::channel(buffer);
        let this      = Promise { inner: tx };

        (rcv, this)
    }

    fn accept(&mut self, value: A) {
        let _ = self.inner.try_send(Ok(value));
    }

    fn reject(&mut self, error: OperationError) {
        let _ = self.inner.try_send(Err(error));
    }
}

fn op_done() -> Decision {
    Ok(Outcome::Done)
}

fn op_continue() -> Decision {
    Ok(Outcome::Continue(None))
}

fn op_send(pkg: Pkg) -> Decision {
    Ok(Outcome::Continue(Some(pkg)))
}

pub enum Op {
    Write(WriteEvents),
}

impl Op {
    // FIXME - Currently, I don't know how to encode existential types so it
    // can be shared across several threads.
    pub fn to_operation(self) -> Box<Operation> {
        match self {
            Op::Write(w) => Box::new(w)
        }
    }
}

pub trait Operation {
    fn poll(&mut self, input: Option<&Pkg>) -> Decision;
    fn failed(&mut self, error: OperationError);
    fn retry(&mut self);
}

enum State {
    CreatePkg,
    Awaiting,
}

pub struct WriteEvents {
    inner: messages::WriteEvents,
    promise: Promise<WriteResult>,
    creds: Option<Credentials>,
    state: State,
}

impl WriteEvents {
    pub fn new(promise: Promise<WriteResult>, creds: Option<Credentials>) -> WriteEvents {
        WriteEvents {
            inner: messages::WriteEvents::new(),
            promise,
            creds,
            state: State::CreatePkg,
        }
    }

    pub fn set_event_stream_id(&mut self, stream_id: String) {
        self.inner.set_event_stream_id(stream_id);
    }

    pub fn set_expected_version(&mut self, exp_ver: ExpectedVersion) {
        self.inner.set_expected_version(exp_ver.to_i64());
    }

    pub fn set_events(&mut self, events: Vec<EventData>) {
        let mut repeated = RepeatedField::new();

        for event in events {
            repeated.push(event.build());
        }

        self.inner.set_events(repeated);
    }

    pub fn set_require_master(&mut self, require_master: bool) {
        self.inner.set_require_master(require_master);
    }
}

impl Operation for WriteEvents {
    fn poll(&mut self, input: Option<&Pkg>) -> Decision {
        match self.state {
            State::CreatePkg => {
                let mut pkg     = Pkg::new(Cmd::WriteEvents, Uuid::new_v4());
                let mut payload = Vec::new();

                self.inner.write_to_vec(&mut payload)?;

                pkg.set_payload(payload);
                pkg.creds_opt = self.creds.clone();

                self.state = State::Awaiting;
                op_send(pkg)
            },

            State::Awaiting => {
                if let Some(pkg) = input {
                    match pkg.cmd {
                        Cmd::WriteEventsCompleted => {
                            let response: messages::WriteEventsCompleted =
                                    parse_from_bytes(&pkg.payload.as_slice())?;

                            match response.get_result() {
                                OperationResult::Success => {
                                    let position = Position {
                                        commit: response.get_commit_position(),
                                        prepare: response.get_prepare_position(),
                                    };

                                    let result = WriteResult {
                                        next_expected_version: response.get_last_event_number(),
                                        position: position,
                                    };

                                    self.promise.accept(result);

                                    op_done()
                                },

                                OperationResult::PrepareTimeout => op_continue(),
                                OperationResult::ForwardTimeout => op_continue(),
                                OperationResult::CommitTimeout  => op_continue(),

                                OperationResult::WrongExpectedVersion => {
                                    let stream_id = self.inner.get_event_stream_id().to_string();
                                    let exp_i64   = self.inner.get_expected_version();
                                    let exp       = ExpectedVersion::from_i64(exp_i64);

                                    self.promise.reject(OperationError::WrongExpectedVersion(stream_id, exp));

                                    op_done()
                                },

                                OperationResult::StreamDeleted => {
                                    let stream_id = self.inner.get_event_stream_id().to_string();

                                    self.promise.reject(OperationError::StreamDeleted(stream_id));

                                    op_done()
                                },

                                OperationResult::InvalidTransaction => {
                                    // self.promise.reject(OperationError::InvalidTransaction);

                                    op_done()
                                }

                                OperationResult::AccessDenied => {
                                    let stream_id = self.inner.get_event_stream_id().to_string();

                                    self.promise.reject(OperationError::AccessDenied(stream_id));

                                    op_done()
                                },
                            }
                        },

                        _ => {
                            self.promise.reject(OperationError::WrongClientImpl(pkg.cmd));

                            op_done()
                        },
                    }
                } else {
                    op_done()
                }
            },
        }
    }

    fn failed(&mut self, error: OperationError) {
        self.promise.reject(error);
    }

    fn retry(&mut self) {
        self.state = State::Awaiting;
    }
}
