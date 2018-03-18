use uuid::Uuid;

use futures::sync::mpsc;
use protobuf::{ RepeatedField, Message };
use protobuf::core::parse_from_bytes;

use internal::command::Cmd;
use internal::data::EventData;
use internal::messages;
use internal::package::Pkg;
use internal::types;

use self::messages::OperationResult;

#[derive(Debug)]
pub enum OperationError {
    WrongExpectedVersion(String, types::ExpectedVersion),
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
    Read(ReadEvent),
}

impl Op {
    // FIXME - Currently, I don't know how to encode existential types so it
    // can be shared across several threads.
    pub fn to_operation(self) -> Box<Operation> {
        match self {
            Op::Write(w) => Box::new(w),
            Op::Read(r)  => Box::new(r),
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
    promise: Promise<types::WriteResult>,
    creds: Option<types::Credentials>,
    state: State,
}

impl WriteEvents {
    pub fn new(promise: Promise<types::WriteResult>, creds: Option<types::Credentials>) -> WriteEvents {
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

    pub fn set_expected_version(&mut self, exp_ver: types::ExpectedVersion) {
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
                let     size    = self.inner.compute_size() as usize;
                let mut pkg     = Pkg::new(Cmd::WriteEvents, Uuid::new_v4());
                let mut payload = Vec::with_capacity(size);

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
                                    let position = types::Position {
                                        commit: response.get_commit_position(),
                                        prepare: response.get_prepare_position(),
                                    };

                                    let result = types::WriteResult {
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
                                    let exp       = types::ExpectedVersion::from_i64(exp_i64);

                                    self.promise.reject(OperationError::WrongExpectedVersion(stream_id, exp));

                                    op_done()
                                },

                                OperationResult::StreamDeleted => {
                                    let stream_id = self.inner.get_event_stream_id().to_string();

                                    self.promise.reject(OperationError::StreamDeleted(stream_id));

                                    op_done()
                                },

                                OperationResult::InvalidTransaction => {
                                    self.promise.reject(OperationError::InvalidTransaction);

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
        self.state = State::CreatePkg;
    }
}

pub struct ReadEvent {
    inner: messages::ReadEvent,
    promise: Promise<types::ReadEventStatus<types::ReadEventResult>>,
    creds: Option<types::Credentials>,
    state: State,
}

impl ReadEvent {
    pub fn new(promise: Promise<types::ReadEventStatus<types::ReadEventResult>>, creds: Option<types::Credentials>) -> ReadEvent {
        ReadEvent {
            inner: messages::ReadEvent::new(),
            promise,
            creds,
            state: State::CreatePkg,
        }
    }

    pub fn set_event_stream_id(&mut self, stream_id: String) {
        self.inner.set_event_stream_id(stream_id);
    }

    pub fn set_event_number(&mut self, event_number: i64) {
        self.inner.set_event_number(event_number);
    }

    pub fn set_resolve_link_tos(&mut self, tos: bool) {
        self.inner.set_resolve_link_tos(tos);
    }

    pub fn set_require_master(&mut self, require_master: bool) {
        self.inner.set_require_master(require_master);
    }
}

impl Operation for ReadEvent {
    fn poll(&mut self, input: Option<&Pkg>) -> Decision {
        match self.state {
            State::CreatePkg => {
                let     size    = self.inner.compute_size() as usize;
                let mut pkg     = Pkg::new(Cmd::ReadEvent, Uuid::new_v4());
                let mut payload = Vec::with_capacity(size);

                self.inner.write_to_vec(&mut payload)?;

                pkg.set_payload(payload);
                pkg.creds_opt = self.creds.clone();

                self.state = State::Awaiting;
                op_send(pkg)
            },

            State::Awaiting => {
                if let Some(pkg) = input {
                    match pkg.cmd {
                        Cmd::ReadEventCompleted => {
                            let mut response: messages::ReadEventCompleted =
                                    parse_from_bytes(&pkg.payload.as_slice())?;

                            match response.get_result() {
                                messages::ReadEventCompleted_ReadEventResult::Success => {
                                    let event        = response.take_event();
                                    let event        = types::ResolvedEvent::new_from_indexed(event)?;
                                    let event_number = self.inner.get_event_number();
                                    let stream_id    = self.inner.get_event_stream_id().to_owned();

                                    let result = types::ReadEventResult {
                                        stream_id,
                                        event_number,
                                        event,
                                    };

                                    let result = types::ReadEventStatus::Success(result);

                                    self.promise.accept(result);
                                    op_done()
                                },

                                messages::ReadEventCompleted_ReadEventResult::NotFound => {
                                    self.promise.accept(types::ReadEventStatus::NotFound);
                                    op_done()
                                },

                                messages::ReadEventCompleted_ReadEventResult::NoStream => {
                                    self.promise.accept(types::ReadEventStatus::NoStream);
                                    op_done()
                                },

                                messages::ReadEventCompleted_ReadEventResult::StreamDeleted => {
                                    self.promise.accept(types::ReadEventStatus::Deleted);
                                    op_done()
                                },

                                messages::ReadEventCompleted_ReadEventResult::Error => {
                                    let error = response.take_error();
                                    let error = OperationError::ServerError(Some(error));

                                    self.promise.reject(error);
                                    op_done()
                                },

                                messages::ReadEventCompleted_ReadEventResult::AccessDenied => {
                                    let stream_id = self.inner.get_event_stream_id().to_owned();
                                    let error     = OperationError::AccessDenied(stream_id);

                                    self.promise.reject(error);
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
        self.state = State::CreatePkg;
    }
}