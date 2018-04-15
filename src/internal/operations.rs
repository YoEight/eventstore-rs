use std::ops::Deref;

use futures::sync::mpsc;
use protobuf::{ Chars, RepeatedField };

use internal::command::Cmd;
use internal::messages;
use internal::package::Pkg;
use types;

use self::messages::{ OperationResult, ReadStreamEventsCompleted_ReadStreamResult, ReadAllEventsCompleted_ReadAllResult };

#[derive(Debug)]
pub enum OperationError {
    WrongExpectedVersion(Chars, types::ExpectedVersion),
    StreamDeleted(Chars),
    InvalidTransaction,
    AccessDenied(Chars),
    ProtobufDecodingError(String),
    ServerError(Option<Chars>),
    InvalidOperation(String),
    StreamNotFound(Chars),
    AuthenticationRequired,
    Aborted,
    WrongClientImpl(Cmd),
}

pub enum Outcome {
    Done,
    Continue(Option<Pkg>),
    Retry,
}

impl Outcome {
    pub fn produced_pkg(self) -> Option<Pkg> {
        match self {
            Outcome::Done              => None,
            Outcome::Continue(pkg_opt) => pkg_opt,
            Outcome::Retry             => None,
        }
    }

    pub fn is_continuing(&self) -> bool {
        if let Outcome::Continue(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_retrying(&self) -> bool {
        if let Outcome::Retry = *self {
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

pub type Receiver<A> = mpsc::Receiver<Result<A, OperationError>>;

impl <A> Promise<A> {
    pub fn new(buffer: usize) -> (Receiver<A>, Promise<A>) {
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


fn _op_continue() -> Decision {
    Ok(Outcome::Continue(None))
}

fn op_send(pkg: Pkg) -> Decision {
    Ok(Outcome::Continue(Some(pkg)))
}

fn op_retry() -> Decision {
    Ok(Outcome::Retry)
}

pub enum Op {
    Write(WriteEvents),
    Read(ReadEvent),
    TransactionStart(TransactionStart),
    TransactionWrite(TransactionWrite),
    TransactionCommit(TransactionCommit),
    ReadStreams(ReadStreamEvents),
    ReadAll(ReadAllEvents),
    Delete(DeleteStream),
}

impl Op {
    // FIXME - Currently, I don't know how to encode existential types so it
    // can be shared across several threads.
    pub fn to_operation(self) -> Box<Operation> {
        match self {
            Op::Write(w)             => Box::new(w),
            Op::Read(r)              => Box::new(r),
            Op::TransactionStart(t)  => Box::new(t),
            Op::TransactionWrite(t)  => Box::new(t),
            Op::TransactionCommit(t) => Box::new(t),
            Op::ReadStreams(r)       => Box::new(r),
            Op::ReadAll(r)           => Box::new(r),
            Op::Delete(d)            => Box::new(d),
        }
    }
}

pub trait Operation {
    fn poll(&mut self, input: Option<Pkg>) -> Decision;
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

    pub fn set_event_stream_id(&mut self, stream_id: Chars) {
        self.inner.set_event_stream_id(stream_id);
    }

    pub fn set_expected_version(&mut self, exp_ver: types::ExpectedVersion) {
        self.inner.set_expected_version(exp_ver.to_i64());
    }

    pub fn set_events(&mut self, events: Vec<types::EventData>) {
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
    fn poll(&mut self, input: Option<Pkg>) -> Decision {
        match self.state {
            State::CreatePkg => {
                let pkg = Pkg::from_message(
                    Cmd::WriteEvents, self.creds.clone(), &self.inner)?;

                self.state = State::Awaiting;
                op_send(pkg)
            },

            State::Awaiting => {
                if let Some(pkg) = input {
                    match pkg.cmd {
                        Cmd::WriteEventsCompleted => {
                            let response: messages::WriteEventsCompleted =
                                    pkg.to_message()?;

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

                                OperationResult::PrepareTimeout | OperationResult::ForwardTimeout | OperationResult::CommitTimeout => {
                                    op_retry()
                                }

                                OperationResult::WrongExpectedVersion => {
                                    let stream_id = self.inner.take_event_stream_id();
                                    let exp_i64   = self.inner.get_expected_version();
                                    let exp       = types::ExpectedVersion::from_i64(exp_i64);

                                    self.promise.reject(OperationError::WrongExpectedVersion(stream_id, exp));

                                    op_done()
                                },

                                OperationResult::StreamDeleted => {
                                    let stream_id = self.inner.take_event_stream_id();

                                    self.promise.reject(OperationError::StreamDeleted(stream_id));

                                    op_done()
                                },

                                OperationResult::InvalidTransaction => {
                                    self.promise.reject(OperationError::InvalidTransaction);

                                    op_done()
                                }

                                OperationResult::AccessDenied => {
                                    let stream_id = self.inner.take_event_stream_id();

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

    pub fn set_event_stream_id(&mut self, stream_id: Chars) {
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
    fn poll(&mut self, input: Option<Pkg>) -> Decision {
        match self.state {
            State::CreatePkg => {
                let pkg = Pkg::from_message(
                    Cmd::ReadEvent, self.creds.clone(), &self.inner)?;

                self.state = State::Awaiting;
                op_send(pkg)
            },

            State::Awaiting => {
                if let Some(pkg) = input {
                    match pkg.cmd {
                        Cmd::ReadEventCompleted => {
                            let mut response: messages::ReadEventCompleted =
                                    pkg.to_message()?;

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
                                    let stream_id = self.inner.take_event_stream_id();
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

pub struct TransactionStart {
    inner: messages::TransactionStart,
    promise: Promise<types::TransactionId>,
    creds: Option<types::Credentials>,
    state: State,
}

impl TransactionStart {
    pub fn new(promise: Promise<types::TransactionId>, creds: Option<types::Credentials>) -> TransactionStart {
        TransactionStart {
            inner: messages::TransactionStart::new(),
            promise,
            creds,
            state: State::CreatePkg,
        }
    }

    pub fn set_event_stream_id(&mut self, value: Chars) {
        self.inner.set_event_stream_id(value);
    }

    pub fn set_expected_version(&mut self, value: types::ExpectedVersion) {
        self.inner.set_expected_version(value.to_i64());
    }

    pub fn set_require_master(&mut self, value: bool) {
        self.inner.set_require_master(value);
    }
}

impl Operation for TransactionStart {
    fn poll(&mut self, input: Option<Pkg>) -> Decision {
        match self.state {
            State::CreatePkg => {
                let pkg = Pkg::from_message(
                    Cmd::TransactionStart, self.creds.clone(), &self.inner)?;

                self.state = State::Awaiting;
                op_send(pkg)
            },

            State::Awaiting => {
                if let Some(pkg) = input {
                    match pkg.cmd {
                        Cmd::TransactionStartCompleted => {
                            let response: messages::TransactionStartCompleted =
                                    pkg.to_message()?;

                            match response.get_result() {
                                OperationResult::Success => {
                                    let id = response.get_transaction_id();
                                    self.promise.accept(types::TransactionId::new(id));

                                    op_done()
                                },

                                OperationResult::PrepareTimeout | OperationResult::ForwardTimeout | OperationResult::CommitTimeout => {
                                    op_retry()
                                },

                                OperationResult::WrongExpectedVersion => {
                                    let stream_id = self.inner.take_event_stream_id();
                                    let exp_i64   = self.inner.get_expected_version();
                                    let exp       = types::ExpectedVersion::from_i64(exp_i64);

                                    self.failed(OperationError::WrongExpectedVersion(stream_id, exp));

                                    op_done()
                                },

                                OperationResult::StreamDeleted => {
                                    let stream_id = self.inner.take_event_stream_id();

                                    self.failed(OperationError::StreamDeleted(stream_id));

                                    op_done()
                                },

                                OperationResult::InvalidTransaction => {
                                    self.failed(OperationError::InvalidTransaction);

                                    op_done()
                                }

                                OperationResult::AccessDenied => {
                                    let stream_id = self.inner.take_event_stream_id();

                                    self.failed(OperationError::AccessDenied(stream_id));

                                    op_done()
                                },
                            }
                        },

                        _ => {
                            self.failed(OperationError::WrongClientImpl(pkg.cmd));
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
        self.promise.reject(error)
    }

    fn retry(&mut self) {
        self.state = State::CreatePkg;
    }
}

pub struct TransactionWrite {
    stream: Chars,
    promise: Promise<()>,
    inner: messages::TransactionWrite,
    creds: Option<types::Credentials>,
    state: State,
}

impl TransactionWrite {
    pub fn new(promise: Promise<()>, stream: Chars, creds: Option<types::Credentials>) -> TransactionWrite {
        TransactionWrite {
            stream,
            promise,
            creds,
            inner: messages::TransactionWrite::new(),
            state: State::CreatePkg,
        }
    }

    pub fn set_transaction_id(&mut self, value: types::TransactionId) {
        self.inner.set_transaction_id(value.0)
    }

    pub fn set_events<I>(&mut self, events: I)
        where I: IntoIterator<Item=types::EventData>
    {
        let mut repeated = RepeatedField::new();

        for event in events {
            repeated.push(event.build());
        }

        self.inner.set_events(repeated);
    }

    pub fn set_require_master(&mut self, value: bool) {
        self.inner.set_require_master(value);
    }
}

impl Operation for TransactionWrite {
    fn poll(&mut self, input: Option<Pkg>) -> Decision {
        match self.state {
            State::CreatePkg => {
                let pkg = Pkg::from_message(
                    Cmd::TransactionWrite, self.creds.clone(), &self.inner)?;

                self.state = State::Awaiting;
                op_send(pkg)
            },

            State::Awaiting => {
                if let Some(pkg) = input {
                    match pkg.cmd {
                        Cmd::TransactionWriteCompleted => {
                            let response: messages::TransactionWriteCompleted =
                                    pkg.to_message()?;

                            match response.get_result() {
                                OperationResult::Success => {
                                    self.promise.accept(());

                                    op_done()
                                },

                                OperationResult::PrepareTimeout | OperationResult::ForwardTimeout | OperationResult::CommitTimeout => {
                                    op_retry()
                                },

                                OperationResult::WrongExpectedVersion => {
                                    // You can't have a wrong expected version on a transaction
                                    // because, the write hasn't been committed yet.
                                    unreachable!()
                                },

                                OperationResult::StreamDeleted => {
                                    let stream = Chars::from(self.stream.deref());
                                    self.failed(OperationError::StreamDeleted(stream));

                                    op_done()
                                },

                                OperationResult::InvalidTransaction => {
                                    self.failed(OperationError::InvalidTransaction);

                                    op_done()
                                }

                                OperationResult::AccessDenied => {
                                    let stream = self.stream.clone();
                                    self.failed(OperationError::AccessDenied(stream));

                                    op_done()
                                },
                            }
                        },

                        _ => {
                            self.failed(OperationError::WrongClientImpl(pkg.cmd));
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

pub struct TransactionCommit {
    stream: Chars,
    version: types::ExpectedVersion,
    promise: Promise<types::WriteResult>,
    inner: messages::TransactionCommit,
    creds: Option<types::Credentials>,
    state: State,
}

impl TransactionCommit {
    pub fn new(
        promise: Promise<types::WriteResult>,
        stream: Chars,
        version: types::ExpectedVersion,
        creds: Option<types::Credentials>) -> TransactionCommit
    {
        TransactionCommit {
            stream,
            promise,
            version,
            inner: messages::TransactionCommit::new(),
            creds,
            state: State::CreatePkg,
        }
    }

    pub fn set_transaction_id(&mut self, value: types::TransactionId) {
        self.inner.set_transaction_id(value.0);
    }

    pub fn set_require_master(&mut self, value: bool) {
        self.inner.set_require_master(value);
    }
}

impl Operation for TransactionCommit {
    fn poll(&mut self, input: Option<Pkg>) -> Decision {
        match self.state {
            State::CreatePkg => {
                let pkg = Pkg::from_message(
                    Cmd::TransactionCommit, self.creds.clone(), &self.inner)?;

                self.state = State::Awaiting;
                op_send(pkg)
            },

            State::Awaiting => {
                if let Some(pkg) = input {
                    match pkg.cmd {
                        Cmd::TransactionCommitCompleted => {
                            let response: messages::TransactionCommitCompleted =
                                    pkg.to_message()?;

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

                                OperationResult::PrepareTimeout | OperationResult::ForwardTimeout | OperationResult::CommitTimeout => {
                                    op_retry()
                                }

                                OperationResult::WrongExpectedVersion => {
                                    let stream = Chars::from(self.stream.deref());

                                    self.promise.reject(OperationError::WrongExpectedVersion(stream, self.version));

                                    op_done()
                                },

                                OperationResult::StreamDeleted => {
                                    let stream = Chars::from(self.stream.deref());

                                    self.promise.reject(OperationError::StreamDeleted(stream));

                                    op_done()
                                },

                                OperationResult::InvalidTransaction => {
                                    self.promise.reject(OperationError::InvalidTransaction);

                                    op_done()
                                }

                                OperationResult::AccessDenied => {
                                    let stream = Chars::from(self.stream.deref());

                                    self.promise.reject(OperationError::AccessDenied(stream));

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

pub struct ReadStreamEvents {
    promise: Promise<types::ReadStreamStatus<types::StreamSlice>>,
    direction: types::ReadDirection,
    request_cmd: Cmd,
    response_cmd: Cmd,
    inner: messages::ReadStreamEvents,
    creds: Option<types::Credentials>,
    state: State,
}

impl ReadStreamEvents {
    pub fn new(
        promise: Promise<types::ReadStreamStatus<types::StreamSlice>>,
        direction: types::ReadDirection,
        creds: Option<types::Credentials>) -> ReadStreamEvents
    {
        let request_cmd = match direction {
            types::ReadDirection::Forward  => Cmd::ReadStreamEventsForward,
            types::ReadDirection::Backward => Cmd::ReadStreamEventsBackward,
        };

        let response_cmd = match direction {
            types::ReadDirection::Forward => Cmd::ReadStreamEventsForwardCompleted,
            types::ReadDirection::Backward => Cmd::ReadStreamEventsBackwardCompleted,
        };

        ReadStreamEvents {
            promise,
            direction,
            request_cmd,
            response_cmd,
            inner: messages::ReadStreamEvents::new(),
            creds,
            state: State::CreatePkg,
        }
    }

    pub fn set_event_stream_id(&mut self, value: Chars) {
        self.inner.set_event_stream_id(value);
    }

    pub fn set_from_event_number(&mut self, value: i64) {
        self.inner.set_from_event_number(value);
    }

    pub fn set_max_count(&mut self, value: i32) {
        self.inner.set_max_count(value);
    }

    pub fn set_resolve_link_tos(&mut self, value: bool) {
        self.inner.set_resolve_link_tos(value);
    }

    pub fn set_require_master(&mut self, value: bool) {
        self.inner.set_require_master(value);
    }
}

impl Operation for ReadStreamEvents {
    fn poll(&mut self, input: Option<Pkg>) -> Decision {
        match self.state {
            State::CreatePkg => {
                let pkg = Pkg::from_message(
                    self.request_cmd, self.creds.clone(), &self.inner)?;

                self.state = State::Awaiting;
                op_send(pkg)
            },

            State::Awaiting => {
                if let Some(pkg) = input {
                    if pkg.cmd == self.response_cmd {
                        let mut response: messages::ReadStreamEventsCompleted =
                                pkg.to_message()?;

                        match response.get_result() {
                            ReadStreamEventsCompleted_ReadStreamResult::Success => {
                                let     is_eof    = response.get_is_end_of_stream();
                                let     events    = response.take_events().into_vec();
                                let mut resolveds = Vec::with_capacity(events.len());

                                for event in events {
                                    let resolved = types::ResolvedEvent::new_from_indexed(event)?;

                                    resolveds.push(resolved);
                                }

                                let next_num_opt = {
                                    if is_eof {
                                        Some(response.get_next_event_number())
                                    } else {
                                        None
                                    }
                                };

                                let from  = self.inner.get_from_event_number();
                                let slice = types::StreamSlice::new(
                                    self.direction, from, resolveds, next_num_opt);
                                let result = types::ReadStreamStatus::Success(slice);

                                self.promise.accept(result);

                                op_done()
                            },

                            ReadStreamEventsCompleted_ReadStreamResult::NoStream => {
                                let stream = self.inner.take_event_stream_id();

                                self.promise.accept(types::ReadStreamStatus::NoStream(stream));

                                op_done()
                            },

                            ReadStreamEventsCompleted_ReadStreamResult::StreamDeleted => {
                                let stream = self.inner.take_event_stream_id();

                                self.promise.accept(types::ReadStreamStatus::StreamDeleled(stream));

                                op_done()
                            },

                            ReadStreamEventsCompleted_ReadStreamResult::AccessDenied => {
                                let stream = self.inner.take_event_stream_id();

                                self.promise.accept(types::ReadStreamStatus::AccessDenied(stream));

                                op_done()
                            },

                            ReadStreamEventsCompleted_ReadStreamResult::NotModified => {
                                let stream = self.inner.take_event_stream_id();

                                self.promise.accept(types::ReadStreamStatus::NotModified(stream));

                                op_done()
                            },

                            ReadStreamEventsCompleted_ReadStreamResult::Error => {
                                let error_msg = response.take_error();

                                self.promise.accept(types::ReadStreamStatus::Error(error_msg));

                                op_done()
                            },
                        }
                    } else {
                        self.promise.reject(OperationError::WrongClientImpl(pkg.cmd));

                        op_done()
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

pub struct ReadAllEvents {
    promise: Promise<types::ReadStreamStatus<types::AllSlice>>,
    direction: types::ReadDirection,
    request_cmd: Cmd,
    response_cmd: Cmd,
    inner: messages::ReadAllEvents,
    creds: Option<types::Credentials>,
    state: State,
}

impl ReadAllEvents {
    pub fn new(
        promise: Promise<types::ReadStreamStatus<types::AllSlice>>,
        direction: types::ReadDirection,
        creds: Option<types::Credentials>) -> ReadAllEvents
    {
        let request_cmd = match direction {
            types::ReadDirection::Forward  => Cmd::ReadAllEventsForward,
            types::ReadDirection::Backward => Cmd::ReadAllEventsBackward,
        };

        let response_cmd = match direction {
            types::ReadDirection::Forward => Cmd::ReadAllEventsForwardCompleted,
            types::ReadDirection::Backward => Cmd::ReadAllEventsBackwardCompleted,
        };

        ReadAllEvents {
            promise,
            direction,
            request_cmd,
            response_cmd,
            inner: messages::ReadAllEvents::new(),
            creds,
            state: State::CreatePkg,
        }
    }

    pub fn set_from_position(&mut self, value: types::Position) {
        self.inner.set_commit_position(value.commit);
        self.inner.set_prepare_position(value.prepare);
    }

    pub fn set_max_count(&mut self, value: i32) {
        self.inner.set_max_count(value);
    }

    pub fn set_resolve_link_tos(&mut self, value: bool) {
        self.inner.set_resolve_link_tos(value);
    }

    pub fn set_require_master(&mut self, value: bool) {
        self.inner.set_require_master(value);
    }
}

impl Operation for ReadAllEvents {
    fn poll(&mut self, input: Option<Pkg>) -> Decision {
        match self.state {
            State::CreatePkg => {
                let pkg = Pkg::from_message(
                    self.request_cmd, self.creds.clone(), &self.inner)?;

                self.state = State::Awaiting;
                op_send(pkg)
            },

            State::Awaiting => {
                if let Some(pkg) = input {
                    if pkg.cmd == self.response_cmd {
                        let mut response: messages::ReadAllEventsCompleted =
                                pkg.to_message()?;

                        match response.get_result() {
                            ReadAllEventsCompleted_ReadAllResult::Success => {
                                let     commit      = response.get_commit_position();
                                let     prepare     = response.get_prepare_position();
                                let     nxt_commit  = response.get_next_commit_position();
                                let     nxt_prepare = response.get_next_prepare_position();
                                let     events      = response.take_events().into_vec();
                                let mut resolveds   = Vec::with_capacity(events.len());

                                for event in events {
                                    let resolved = types::ResolvedEvent::new(event)?;

                                    resolveds.push(resolved);
                                }

                                let from = types::Position {
                                    commit,
                                    prepare,
                                };

                                let next = types::Position {
                                    commit: nxt_commit,
                                    prepare: nxt_prepare,
                                };

                                let slice = types::AllSlice::new(
                                    self.direction, from, resolveds, next);
                                let result = types::ReadStreamStatus::Success(slice);

                                self.promise.accept(result);

                                op_done()
                            },

                            ReadAllEventsCompleted_ReadAllResult::AccessDenied => {
                                self.promise.accept(
                                    types::ReadStreamStatus::AccessDenied(Chars::from("$all")));

                                op_done()
                            },

                            ReadAllEventsCompleted_ReadAllResult::NotModified => {
                                self.promise.accept(
                                    types::ReadStreamStatus::NotModified(Chars::from("$all")));

                                op_done()
                            },

                            ReadAllEventsCompleted_ReadAllResult::Error => {
                                let error_msg = response.take_error();

                                self.promise.accept(types::ReadStreamStatus::Error(error_msg));

                                op_done()
                            },
                        }
                    } else {
                        self.promise.reject(OperationError::WrongClientImpl(pkg.cmd));

                        op_done()
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

pub struct DeleteStream {
    inner: messages::DeleteStream,
    promise: Promise<types::Position>,
    creds: Option<types::Credentials>,
    state: State,
}

impl DeleteStream {
    pub fn new(promise: Promise<types::Position>, creds: Option<types::Credentials>) -> DeleteStream {
        DeleteStream {
            inner: messages::DeleteStream::new(),
            promise,
            creds,
            state: State::CreatePkg,
        }
    }

    pub fn set_event_stream_id(&mut self, stream_id: Chars) {
        self.inner.set_event_stream_id(stream_id);
    }

    pub fn set_expected_version(&mut self, exp_ver: types::ExpectedVersion) {
        self.inner.set_expected_version(exp_ver.to_i64());
    }

    pub fn set_require_master(&mut self, require_master: bool) {
        self.inner.set_require_master(require_master);
    }

    pub fn set_hard_delete(&mut self, value: bool) {
        self.inner.set_hard_delete(value);
    }
}

impl Operation for DeleteStream {
    fn poll(&mut self, input: Option<Pkg>) -> Decision {
        match self.state {
            State::CreatePkg => {
                let pkg = Pkg::from_message(
                    Cmd::DeleteStream, self.creds.clone(), &self.inner)?;

                self.state = State::Awaiting;
                op_send(pkg)
            },

            State::Awaiting => {
                if let Some(pkg) = input {
                    match pkg.cmd {
                        Cmd::DeleteStreamCompleted => {
                            let response: messages::DeleteStreamCompleted =
                                    pkg.to_message()?;

                            match response.get_result() {
                                OperationResult::Success => {
                                    let position = types::Position {
                                        commit: response.get_commit_position(),
                                        prepare: response.get_prepare_position(),
                                    };

                                    self.promise.accept(position);

                                    op_done()
                                },

                                OperationResult::PrepareTimeout | OperationResult::ForwardTimeout | OperationResult::CommitTimeout => {
                                    op_retry()
                                }

                                OperationResult::WrongExpectedVersion => {
                                    let stream_id = self.inner.take_event_stream_id();
                                    let exp_i64   = self.inner.get_expected_version();
                                    let exp       = types::ExpectedVersion::from_i64(exp_i64);

                                    self.promise.reject(OperationError::WrongExpectedVersion(stream_id, exp));

                                    op_done()
                                },

                                OperationResult::StreamDeleted => {
                                    let stream_id = self.inner.take_event_stream_id();

                                    self.promise.reject(OperationError::StreamDeleted(stream_id));

                                    op_done()
                                },

                                OperationResult::InvalidTransaction => {
                                    self.promise.reject(OperationError::InvalidTransaction);

                                    op_done()
                                }

                                OperationResult::AccessDenied => {
                                    let stream_id = self.inner.take_event_stream_id();

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