use std::ops::Deref;

use futures::{ Future, Stream };
use futures::sync::mpsc;
use protobuf::{ Chars, RepeatedField };

use internal::command::Cmd;
use internal::messages;
use internal::package::Pkg;
use types::{ self, Slice };

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

impl OperationError {
    fn to_io_error(self) -> ::std::io::Error {
        let msg = format!("internal operation error: {:?}", self);

        ::std::io::Error::new(::std::io::ErrorKind::Other, msg)
    }
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

    pub fn is_done(&self) -> bool {
        match *self {
            Outcome::Done => true,
            _             => false,
        }
    }
}

pub type Decision = ::std::io::Result<Outcome>;

fn decision_is_continuing(value: &Decision) -> bool {
    match *value {
        Ok(ref outcome) => outcome.is_continuing(),
        _               => false,
    }
}

fn decision_is_retrying(value: &Decision) -> bool {
    match *value {
        Ok(ref outcome) => outcome.is_retrying(),
        _               => false,
    }
}

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


fn op_continue() -> Decision {
    Ok(Outcome::Continue(None))
}

fn op_send(pkg: Pkg) -> Decision {
    Ok(Outcome::Continue(Some(pkg)))
}

fn op_retry() -> Decision {
    Ok(Outcome::Retry)
}

pub type Exchange = Box<Operation + Sync + Send>;

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

    fn report_error(&mut self, error: types::ReadStreamError) {
        self.promise.accept(types::ReadStreamStatus::Error(error))
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

                                self.report_error(types::ReadStreamError::NoStream(stream));

                                op_done()
                            },

                            ReadStreamEventsCompleted_ReadStreamResult::StreamDeleted => {
                                let stream = self.inner.take_event_stream_id();

                                self.report_error(types::ReadStreamError::StreamDeleted(stream));

                                op_done()
                            },

                            ReadStreamEventsCompleted_ReadStreamResult::AccessDenied => {
                                let stream = self.inner.take_event_stream_id();

                                self.report_error(types::ReadStreamError::AccessDenied(stream));

                                op_done()
                            },

                            ReadStreamEventsCompleted_ReadStreamResult::NotModified => {
                                let stream = self.inner.take_event_stream_id();

                                self.report_error(types::ReadStreamError::NotModified(stream));

                                op_done()
                            },

                            ReadStreamEventsCompleted_ReadStreamResult::Error => {
                                let error_msg = response.take_error();

                                self.report_error(types::ReadStreamError::Error(error_msg));

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

    fn report_error(&mut self, error: types::ReadStreamError) {
        self.promise.accept(types::ReadStreamStatus::Error(error))
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
                                self.report_error(
                                    types::ReadStreamError::AccessDenied("$all".into()));

                                op_done()
                            },

                            ReadAllEventsCompleted_ReadAllResult::NotModified => {
                                self.report_error(
                                    types::ReadStreamError::NotModified("$all".into()));

                                op_done()
                            },

                            ReadAllEventsCompleted_ReadAllResult::Error => {
                                let error_msg = response.take_error();

                                self.report_error(types::ReadStreamError::Error(error_msg));

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

pub struct SubscribeToStream {
    sub_bus: mpsc::Sender<types::SubEvent>,
    inner: messages::SubscribeToStream,
    creds: Option<types::Credentials>,
    state: State,
}

impl SubscribeToStream {
    pub(crate) fn new(sub_bus: mpsc::Sender<types::SubEvent>, creds: Option<types::Credentials>)
        -> SubscribeToStream
    {
        SubscribeToStream {
            sub_bus,
            creds,
            inner: messages::SubscribeToStream::new(),
            state: State::CreatePkg,
        }
    }

    pub fn set_event_stream_id(&mut self, stream_id: Chars) {
        self.inner.set_event_stream_id(stream_id);
    }

    pub fn set_resolve_link_tos(&mut self, value: bool) {
        self.inner.set_resolve_link_tos(value);
    }

    fn publish(&mut self, event: types::SubEvent) {
        if let Err(_) = self.sub_bus.try_send(event) {
            print!("ERROR: Max unprocessed events limit reached!");
        }
    }
}

impl Operation for SubscribeToStream {
    fn poll(&mut self, input: Option<Pkg>) -> Decision {
        match self.state {
            State::CreatePkg => {
                let pkg = Pkg::from_message(
                    Cmd::SubscribeToStream, self.creds.clone(), &self.inner)?;

                self.state = State::Awaiting;
                op_send(pkg)
            },

            State::Awaiting => {
                if let Some(pkg) = input {
                    match pkg.cmd {
                        Cmd::SubscriptionConfirmed => {
                            let response: messages::SubscriptionConfirmation =
                                pkg.to_message()?;

                            let last_commit_position = response.get_last_commit_position();
                            let last_event_number    = response.get_last_event_number();

                            let confirmed = types::SubEvent::Confirmed {
                                id: pkg.correlation,
                                last_commit_position,
                                last_event_number,
                            };

                            self.publish(confirmed);

                            op_continue()
                        },

                        Cmd::StreamEventAppeared => {
                            let mut response: messages::StreamEventAppeared =
                                pkg.to_message()?;

                            let event    = types::ResolvedEvent::new(response.take_event())?;
                            let appeared = types::SubEvent::EventAppeared(event);

                            self.publish(appeared);

                            op_continue()
                        }

                        Cmd::SubscriptionDropped => {
                            self.publish(types::SubEvent::Dropped);

                            op_done()
                        },

                        _ => {
                            // Will never happened, has error in subscription is
                            // reported through `Cmd::SubscriptionDropped`
                            // command.
                            op_done()
                        },
                    }
                } else {
                    op_done()
                }
            },
        }
    }

    fn failed(&mut self, _: OperationError) {
        self.publish(types::SubEvent::Dropped);
    }

    fn retry(&mut self) {
        self.state = State::CreatePkg;
    }
}

pub(crate) trait StreamPull {
    fn pull(&mut self, dest: &mut PullDest, pkg: Option<Pkg>) -> Decision;
}

enum PullState {
    Prepare,
    Await,
}

enum PullStatus {
    Success(Option<i64>),
    Failed,
}

struct PullProcess {
    receiver: Receiver<types::ReadStreamStatus<types::StreamSlice>>,
    inner: ReadStreamEvents,
}

impl PullProcess {
    fn retry(&mut self) {
        self.inner.retry();
    }
}

pub(crate) struct PullDest {
    buffer_opt: Option<Vec<types::ResolvedEvent>>,
    error_opt: Option<types::ReadStreamError>,
}

fn blocking_pull_error(error: &types::ReadStreamError) -> bool {
    match *error {
        types::ReadStreamError::NoStream(_) => false,
        _                                   => true,
    }
}

impl PullDest {
    fn new() -> PullDest {
        PullDest {
            buffer_opt: None,
            error_opt: None,
        }
    }
}

fn single_value_future<S, A>(stream: S) -> impl Future<Item=A, Error=OperationError>
    where S: Stream<Item = Result<A, OperationError>, Error = ()>
{
    stream.into_future().then(|res| {
        match res {
            Ok((Some(x), _)) => x,
            _                => unreachable!(),
        }
    })
}

pub(crate) struct RegularStreamPull {
    stream_id: Chars,
    process_opt: Option<PullProcess>,
    pos: i64,
    resolve_link_tos: bool,
    require_master: bool,
    batch_size: i32,
    creds_opt: Option<types::Credentials>,
    state: PullState,
}

impl RegularStreamPull {
    pub(crate) fn new(
        stream_id: Chars,
        resolve_link_tos: bool,
        require_master: bool,
        batch_size: i32,
        start_from: i64,
        creds_opt: Option<types::Credentials>) -> RegularStreamPull
    {
        RegularStreamPull {
            stream_id,
            resolve_link_tos,
            require_master,
            batch_size,
            process_opt: None,
            pos: start_from,
            creds_opt,
            state: PullState::Prepare,
        }
    }

    fn prepare(&mut self) -> Decision {
        let (receiver, sender) = Promise::new(1);
        let mut inner          = ReadStreamEvents::new(sender, types::ReadDirection::Forward, self.creds_opt.clone());

        inner.set_event_stream_id(self.stream_id.clone());
        inner.set_from_event_number(self.pos);
        inner.set_max_count(self.batch_size);
        inner.set_require_master(self.require_master);
        inner.set_resolve_link_tos(self.resolve_link_tos);

        let pkg = inner.poll(None);

        let process = PullProcess {
            receiver,
            inner,
        };

        self.process_opt = Some(process);

        pkg
    }

    fn accept_response(&mut self, dest: &mut PullDest, pkg: Pkg) -> Decision {
        if let Some(mut process) = self.process_opt.take() {
            let outcome = process.inner.poll(Some(pkg))?;

            if outcome.is_retrying() {
                return Ok(outcome);
            }

            let fut = single_value_future(process.receiver).map(|status| {
                match status {
                    types::ReadStreamStatus::Success(slice) => {
                        let events = slice.events();

                        if let types::LocatedEvents::Events { events, next } = events {
                            dest.buffer_opt = Some(events);

                            PullStatus::Success(next)
                        } else {
                            PullStatus::Success(None)
                        }
                    },

                    types::ReadStreamStatus::Error(error) => {
                        if blocking_pull_error(&error) {
                            dest.error_opt = Some(error);
                        }

                        PullStatus::Failed
                    },
                }
            });

            match fut.wait() {
                Ok(status) => {
                    match status {
                        PullStatus::Success(next_pos) => {
                            if let Some(pos) = next_pos {
                                self.pos = pos;

                                op_done()
                            } else {
                                op_continue()
                            }
                        },

                        PullStatus::Failed => {
                            op_done()
                        }
                    }
                },

                Err(op_error) => {
                    Err(op_error.to_io_error())
                },
            }
        } else {
            op_done()
        }
    }
}

impl StreamPull for RegularStreamPull {
    fn pull(&mut self, dest: &mut PullDest, pkg_opt: Option<Pkg>) -> Decision {
        match self.state {
            PullState::Prepare => {
                self.state = PullState::Await;

                self.prepare()
            },

            PullState::Await => {
                match pkg_opt {
                    Some(pkg) => {
                        let decision = self.accept_response(dest, pkg);

                        if decision_is_continuing(&decision) {
                            self.state = PullState::Prepare;
                            self.pull(dest, None)
                        } else {
                            if decision_is_retrying(&decision) {
                                let inner = self.process_opt
                                                .as_mut()
                                                .expect("Process must be available");

                                inner.retry();
                            }

                            decision
                        }
                    },

                    None => {
                        op_done()
                    },
                }
            },
        }
    }
}

pub(crate) struct CatchupSubscribe<P: StreamPull> {
    stream_id_opt: Option<Chars>, // If `None`, it means we target $all stream.
    resolve_link_tos: bool,
    sub_bus: mpsc::Sender<types::SubEvent>,
    puller: P,
    dest: PullDest,
    creds_opt: Option<types::Credentials>,
    sub_opt: Option<SubscribeToStream>,
    state: CatchupState,
}

fn max_unprocessed_events_limit_error_reached() -> ::std::io::Error {
    let msg = format!("Max unprocessed events limit reached!");

    ::std::io::Error::new(::std::io::ErrorKind::Other, msg)
}

impl <P: StreamPull> CatchupSubscribe<P> {
    pub(crate) fn new(
        stream_id_opt: Option<Chars>,
        resolve_link_tos: bool,
        sub_bus: mpsc::Sender<types::SubEvent>,
        creds_opt: Option<types::Credentials>,
        puller: P) -> CatchupSubscribe<P>
    {
        CatchupSubscribe {
            stream_id_opt,
            resolve_link_tos,
            sub_bus,
            puller,
            dest: PullDest::new(),
            sub_opt: None,
            creds_opt,
            state: CatchupState::CatchingUp,
        }
    }

    fn notify_sub(&mut self, msg: types::SubEvent) -> ::std::io::Result<()> {
        if let Err(_) = self.sub_bus.try_send(msg) {
            Err(max_unprocessed_events_limit_error_reached())
        } else {
            Ok(())
        }
    }

    fn notify_sub_drop(&mut self) -> ::std::io::Result<()> {
        self.notify_sub(types::SubEvent::Dropped)
    }

    fn notify_event_appeared(&mut self, event: types::ResolvedEvent)
        -> ::std::io::Result<()>
    {
        let msg = types::SubEvent::EventAppeared(event);

        self.notify_sub(msg)
    }

    fn notify_events_appeared(&mut self, events: Vec<types::ResolvedEvent>)
        -> ::std::io::Result<()>
    {
        // Initially, we used `send_all` but considering it might block the
        // driver if we reach the maximum capacity of the queue. Besides,
        // `Sender` is the sink that does nothing regarding to flushing,
        for event in events {
            self.notify_event_appeared(event)?;
        }

        Ok(())
    }

    fn prepare_sub_request(&mut self) -> Decision {
        let mut sub       = SubscribeToStream::new(self.sub_bus.clone(), self.creds_opt.clone());
        let     stream_id = self.stream_id_opt.clone().unwrap_or_default();

        sub.set_event_stream_id(stream_id);
        sub.set_resolve_link_tos(self.resolve_link_tos);

        let pkg = sub.poll(None);
        self.sub_opt = Some(sub);

        pkg
    }
}

#[derive(Copy, Clone)]
enum CatchupState {
    CatchingUp,
    Subscribe,
    Live,
}

impl <P: StreamPull> Operation for CatchupSubscribe<P> {
    fn poll(&mut self, input: Option<Pkg>) -> Decision {
        match self.state {
            CatchupState::CatchingUp => {
                let outcome = self.puller.pull(&mut self.dest, input)?;

                // Safe because if we shouldn't expect events or something went
                // wrong, it will be a `None`.
                if let Some(events) = self.dest.buffer_opt.take() {
                    self.notify_events_appeared(events)?;
                }

                if outcome.is_done() {
                    if let None = self.dest.error_opt.take() {
                        self.state = CatchupState::Subscribe;

                        self.poll(None)
                    } else {
                        self.notify_sub_drop()?;

                        Ok(outcome)
                    }
                } else {
                    Ok(outcome)
                }
            },

            CatchupState::Subscribe => {
                // When we have to send the subscription request.
                if input.is_none() {
                    self.prepare_sub_request()
                } else {
                    // If everything went smootly, we should receive a confirmation
                    // response.
                    self.state = CatchupState::Live;
                    let sub = self.sub_opt.as_mut().expect("Sub reference must be available on confirmation");

                    // If things didn't go well, `sub` will ask to drop the
                    // transaction with the server for us.
                    sub.poll(input)
                }
            },

            CatchupState::Live => {
                // At this point we let the subcription driving the transaction
                // with the server for us.
                let sub = self.sub_opt.as_mut().expect("Sub reference must be available when live");

                sub.poll(input)
            },
        }
    }

    fn failed(&mut self, _: OperationError) {
        // It would mean either the catching up phase or our subscription
        // request was invalid. All we can do is notify the user the
        // subscription has been dropped.
        let _ = self.notify_sub_drop();
    }

    fn retry(&mut self) {
        // Nothing to do. Everything will be handle properly by `poll`
        // as we didn't update `CatchupSubscribe` stage state.
    }
}
