use std::ops::Deref;
use std::time::{ Duration, Instant };

use bytes::{ BufMut, BytesMut };
use futures::{ Future, Stream, Sink };
use futures::stream::iter_ok;
use futures::sync::mpsc;
use protobuf::{ Chars, RepeatedField };
use uuid::Uuid;

use internal::command::Cmd;
use internal::messages;
use internal::package::Pkg;
use types::{ self, Slice };

use self::messages::{ OperationResult, ReadStreamEventsCompleted_ReadStreamResult, ReadAllEventsCompleted_ReadAllResult };

#[derive(Debug, Clone)]
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
    WrongClientImpl(Option<Cmd>),
    ConnectionHasDropped,
}

impl OperationError {
    fn wrong_client_impl() -> OperationError {
        OperationError::WrongClientImpl(None)
    }

    fn wrong_client_impl_on_cmd(cmd: Cmd) -> OperationError {
        OperationError::WrongClientImpl(Some(cmd))
    }
}

pub enum Outcome {
    Done,
    Continue(Vec<Pkg>),
}

impl Outcome {
    pub fn produced_pkgs(self) -> Vec<Pkg> {
        match self {
            Outcome::Done           => Vec::new(),
            Outcome::Continue(pkgs) => pkgs,
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

fn decision_is_done(value: &Decision) -> bool {
    match *value {
        Ok(ref outcome) => outcome.is_done(),
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
    Ok(Outcome::Continue(Vec::new()))
}

fn op_send(pkg: Pkg) -> Decision {
    Ok(Outcome::Continue(vec![pkg]))
}

fn op_send_pkgs(pkgs: Vec<Pkg>) -> Decision {
    Ok(Outcome::Continue(pkgs))
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub(crate) struct OperationId(Uuid);

impl OperationId {
    fn new() -> OperationId {
        OperationId(Uuid::new_v4())
    }
}

pub(crate) struct OperationWrapper {
    // Operation unique id, useful for registry sessions.
    pub(crate) id: OperationId,
    max_retry: usize,
    timeout: Duration,
    inner: Box<OperationImpl + Sync + Send>,
    creds: Option<types::Credentials>,
}

pub(crate) struct Tracking {
    id: Uuid,
    cmd: Cmd,
    attempts: usize,
    started: Instant,
    lasting: bool,
}

impl Tracking {
    pub(crate) fn new(cmd: Cmd) -> Tracking {
        Tracking {
            cmd,
            id: Uuid::new_v4(),
            attempts: 0,
            started: Instant::now(),
            lasting: false,
        }
    }

    pub(crate) fn get_id(&self) -> Uuid {
        self.id
    }
}

pub(crate) trait ReqBuffer {
    fn push_req(&mut self, req: Request) -> ::std::io::Result<()>;
}

/// Used to allow an operation to support multiple exchanges at the same time
/// with the server.
struct VecReqBuffer<'a, A: 'a + Session> {
    session: &'a mut A,
    dest: &'a mut BytesMut,
    creds: Option<types::Credentials>,
    pkgs: Vec<Pkg>,
}

impl<'a, A: Session> VecReqBuffer<'a, A> {
    fn new(session: &'a mut A, dest: &'a mut BytesMut, creds: Option<types::Credentials>)
        -> VecReqBuffer<'a, A>
    {
        VecReqBuffer {
            session,
            dest,
            creds,
            pkgs: Vec::new(),
        }
    }
}

impl<'a, A: Session> ReqBuffer for VecReqBuffer<'a, A> {
    fn push_req(&mut self, req: Request) -> ::std::io::Result<()> {
        let id  = self.session.new_request(req.cmd);
        let pkg = req.produce_pkg(id, self.creds.clone(), self.dest)?;

        self.pkgs.push(pkg);

        Ok(())
    }
}

pub(crate) trait Session {
    fn new_request(&mut self, Cmd) -> Uuid;
    fn pop(&mut self, &Uuid) -> ::std::io::Result<Tracking>;
    fn reuse(&mut self, Tracking);
    fn using(&mut self, &Uuid) -> ::std::io::Result<&mut Tracking>;
    fn requests(&self) -> Vec<&Tracking>;
    fn terminate(&mut self);
}

impl OperationWrapper {
    pub(crate) fn new<A>(
        op: A,
        creds: Option<types::Credentials>,
        max_retry: usize,
        timeout: Duration) -> OperationWrapper
        where A: OperationImpl + Sync + Send + 'static
    {
        OperationWrapper {
            id: OperationId::new(),
            inner: Box::new(op),
            creds,
            max_retry,
            timeout,
        }
    }

    pub(crate) fn send<A: Session>(&mut self, dest: &mut BytesMut, session: A)
        -> Decision
    {
        self.poll(dest, session, None)
    }

    pub(crate) fn receive<A: Session>(&mut self, dest: &mut BytesMut, session: A, pkg: Pkg)
        -> Decision
    {
        self.poll(dest, session, Some(pkg))
    }

    fn poll<A: Session>(&mut self, dest: &mut BytesMut, mut session: A, input: Option<Pkg>) -> Decision {
        match input {
            // It means this operation was newly created and has to issue its
            // first package to the server.
            None => {
                let req      = self.inner.initial_request();
                let id       = session.new_request(req.cmd);
                let decision = req.send(id, self.creds.clone(), dest);

                decision
            },

            // At this point, it means this operation send a package to
            // the server already.
            Some(pkg) => {
                let corr_id = pkg.correlation;

                if self.inner.is_valid_response(pkg.cmd) {
                    let (pkgs, result) = {
                        let mut buffer = VecReqBuffer::new(&mut session, dest, self.creds.clone());
                        let     result = self.inner.respond(&mut buffer, pkg)?;

                        (buffer.pkgs, result)
                    };

                    match result {
                        ImplResult::Retry => {
                            return self.retry(dest, &mut session, corr_id)
                        },

                        ImplResult::Done => {
                            session.pop(&corr_id)?;
                        },

                        ImplResult::Awaiting => {
                            let tracker = session.using(&corr_id)?;

                            tracker.lasting = true;
                        },

                        ImplResult::Terminate => {
                            session.terminate();

                            return op_done();
                        },
                    };

                    op_send_pkgs(pkgs)
                } else {
                    self.failed(OperationError::wrong_client_impl_on_cmd(pkg.cmd));

                    op_done()
                }
            },
        }
    }

    pub(crate) fn failed(&mut self, error: OperationError) {
        self.inner.report_operation_error(error);
    }

    pub(crate) fn retry<A: Session>(&mut self, dest: &mut BytesMut, session: &mut A, id: Uuid) -> Decision {
        let mut tracker = session.pop(&id)?;

        if tracker.attempts + 1 >= self.max_retry {
            self.failed(OperationError::Aborted);
            session.terminate();

            return op_done();
        }

        tracker.attempts += 1;
        tracker.id       = Uuid::new_v4();

        let req      = self.inner.retry(tracker.cmd);
        let decision = req.send(tracker.id, self.creds.clone(), dest);

        session.reuse(tracker);

        decision
    }

    pub(crate) fn check_and_retry<A: Session>(&mut self, dest: &mut BytesMut, mut session: A) -> Decision {
        let mut to_retry = Vec::new();

        for tracker in session.requests() {
            if !tracker.lasting && tracker.started.elapsed() >= self.timeout {
                to_retry.push(tracker.id);
            }
        }

        if to_retry.is_empty() {
            op_continue()
        } else {
            let mut pkgs = Vec::new();

            for key in to_retry {
                let decision = self.retry(dest, &mut session, key);

                if decision_is_done(&decision) {
                    return decision;
                } else {
                    let outcome = decision?;

                    pkgs.append(&mut outcome.produced_pkgs());
                }
            }

            op_send_pkgs(pkgs)
        }
    }

    pub(crate) fn connection_has_dropped<A>(
        &mut self,
        dest: &mut BytesMut,
        mut session: A
    ) -> Decision
    where
        A: Session
    {
        let pkgs = {
            let mut buffer = VecReqBuffer::new(
                &mut session, dest, self.creds.clone());

            self.inner.connection_has_dropped(&mut buffer)?;

            buffer.pkgs
        };

        if pkgs.is_empty() {
            op_done()
        } else {
            op_send_pkgs(pkgs)
        }
    }
}

pub(crate) struct Request<'a> {
    cmd: Cmd,
    msg: &'a ::protobuf::Message,
}

impl <'a> Request<'a> {
    fn produce_pkg(self, id: Uuid, creds: Option<types::Credentials>, dest: &mut BytesMut)
        -> ::std::io::Result<Pkg>
    {
        dest.reserve(self.msg.compute_size() as usize);

        self.msg.write_to_writer(&mut dest.writer())?;

        let pkg = Pkg {
            cmd: self.cmd,
            correlation: id,
            creds_opt: creds,
            payload: dest.take().freeze(),
        };

        Ok(pkg)
    }

    fn send(self, id: Uuid, creds: Option<types::Credentials>, dest: &mut BytesMut)
        -> Decision
    {
        let pkg = self.produce_pkg(id, creds, dest)?;

        op_send(pkg)
    }
}

pub(crate) enum ImplResult {
    Retry,
    Awaiting,
    Done,
    Terminate,
}

impl ImplResult {
    fn retrying() -> ::std::io::Result<ImplResult> {
        Ok(ImplResult::Retry)
    }

    fn awaiting() -> ::std::io::Result<ImplResult> {
        Ok(ImplResult::Awaiting)
    }

    fn done() -> ::std::io::Result<ImplResult> {
        Ok(ImplResult::Done)
    }

    fn terminate() -> ::std::io::Result<ImplResult> {
        Ok(ImplResult::Terminate)
    }

    fn is_done(&self) -> bool {
        match *self {
            ImplResult::Done => true,
            _                => false,
        }
    }
}


pub(crate) trait OperationImpl {
    /// Issues a new `Request` for the server. `prev_id` indicates what was
    /// the previous id for that `Request`. `prev_id` is useful for operation
    /// that handles multiple smaller operations at the same time. `prev_id`
    /// ease the process of `retry` in this context. The implementation in this
    /// case is required to keep track of its last attempt in order to know
    /// which request it has to re-issue if `prev_id` is defined.
    ///
    /// 'new_id' indicates the correlation id that will be used for that new
    /// request.
    fn initial_request(&self) -> Request;
    fn is_valid_response(&self, cmd: Cmd) -> bool;
    fn respond(&mut self, buffer: &mut ReqBuffer, pkg: Pkg) -> ::std::io::Result<ImplResult>;
    fn report_operation_error(&mut self, error: OperationError);

    fn retry(&self, _: Cmd) -> Request {
        self.initial_request()
    }

    fn connection_has_dropped(
        &mut self,
        _: &mut ReqBuffer
    ) -> ::std::io::Result<()>
    {
        self.report_operation_error(OperationError::ConnectionHasDropped);

        Ok(())
    }
}

pub struct WriteEvents {
    inner: messages::WriteEvents,
    promise: Promise<types::WriteResult>,
}

impl WriteEvents {
    pub fn new(promise: Promise<types::WriteResult>) -> WriteEvents {
        WriteEvents {
            inner: messages::WriteEvents::new(),
            promise,
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

impl OperationImpl for WriteEvents {
    fn initial_request(&self) -> Request {
        Request {
            cmd: Cmd::WriteEvents,
            msg: &self.inner,
        }
    }

    fn is_valid_response(&self, cmd: Cmd) -> bool {
        Cmd::WriteEventsCompleted == cmd
    }

    fn respond(&mut self, _: &mut ReqBuffer, pkg: Pkg) -> ::std::io::Result<ImplResult> {
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

                ImplResult::done()
            },

            OperationResult::PrepareTimeout | OperationResult::ForwardTimeout | OperationResult::CommitTimeout => {
                ImplResult::retrying()
            }

            OperationResult::WrongExpectedVersion => {
                let stream_id = self.inner.take_event_stream_id();
                let exp_i64   = self.inner.get_expected_version();
                let exp       = types::ExpectedVersion::from_i64(exp_i64);

                self.promise.reject(OperationError::WrongExpectedVersion(stream_id, exp));

                ImplResult::done()
            },

            OperationResult::StreamDeleted => {
                let stream_id = self.inner.take_event_stream_id();

                self.promise.reject(OperationError::StreamDeleted(stream_id));

                ImplResult::done()
            },

            OperationResult::InvalidTransaction => {
                self.promise.reject(OperationError::InvalidTransaction);

                ImplResult::done()
            }

            OperationResult::AccessDenied => {
                let stream_id = self.inner.take_event_stream_id();

                self.promise.reject(OperationError::AccessDenied(stream_id));

                ImplResult::done()
            },
        }
    }

    fn report_operation_error(&mut self, error: OperationError) {
        self.promise.reject(error);
    }
}

pub struct ReadEvent {
    inner: messages::ReadEvent,
    promise: Promise<types::ReadEventStatus<types::ReadEventResult>>,
}

impl ReadEvent {
    pub fn new(promise: Promise<types::ReadEventStatus<types::ReadEventResult>>) -> ReadEvent {
        ReadEvent {
            inner: messages::ReadEvent::new(),
            promise,
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

pub struct TransactionStart {
    inner: messages::TransactionStart,
    promise: Promise<types::TransactionId>,
}

impl TransactionStart {
    pub fn new(promise: Promise<types::TransactionId>) -> TransactionStart {
        TransactionStart {
            inner: messages::TransactionStart::new(),
            promise,
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

impl OperationImpl for ReadEvent {
    fn initial_request(&self) -> Request {
        Request {
            cmd: Cmd::ReadEvent,
            msg: &self.inner,
        }
    }

    fn is_valid_response(&self, cmd: Cmd) -> bool {
        Cmd::ReadEventCompleted == cmd
    }

    fn respond(&mut self, _: &mut ReqBuffer, pkg: Pkg) -> ::std::io::Result<ImplResult> {
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
            },

            messages::ReadEventCompleted_ReadEventResult::NotFound => {
                self.promise.accept(types::ReadEventStatus::NotFound);
            },

            messages::ReadEventCompleted_ReadEventResult::NoStream => {
                self.promise.accept(types::ReadEventStatus::NoStream);
            },

            messages::ReadEventCompleted_ReadEventResult::StreamDeleted => {
                self.promise.accept(types::ReadEventStatus::Deleted);
            },

            messages::ReadEventCompleted_ReadEventResult::Error => {
                let error = response.take_error();
                let error = OperationError::ServerError(Some(error));

                self.promise.reject(error);
            },

            messages::ReadEventCompleted_ReadEventResult::AccessDenied => {
                let stream_id = self.inner.take_event_stream_id();
                let error     = OperationError::AccessDenied(stream_id);

                self.promise.reject(error);
            },
        }

        ImplResult::done()
    }

    fn report_operation_error(&mut self, error: OperationError) {
        self.promise.reject(error)
    }
}



impl OperationImpl for TransactionStart {
    fn initial_request(&self) -> Request {
        Request {
            cmd: Cmd::TransactionStart,
            msg: &self.inner,
        }
    }

    fn is_valid_response(&self, cmd: Cmd) -> bool {
        Cmd::TransactionStartCompleted == cmd
    }

    fn respond(&mut self, _: &mut ReqBuffer, pkg: Pkg) -> ::std::io::Result<ImplResult> {
        let response: messages::TransactionStartCompleted =
                 pkg.to_message()?;

        match response.get_result() {
            OperationResult::Success => {
                let id = response.get_transaction_id();
                self.promise.accept(types::TransactionId::new(id));

                ImplResult::done()
            },

            OperationResult::PrepareTimeout | OperationResult::ForwardTimeout | OperationResult::CommitTimeout => {
                ImplResult::retrying()
            },

            OperationResult::WrongExpectedVersion => {
                let stream_id = self.inner.take_event_stream_id();
                let exp_i64   = self.inner.get_expected_version();
                let exp       = types::ExpectedVersion::from_i64(exp_i64);

                self.promise.reject(OperationError::WrongExpectedVersion(stream_id, exp));

                ImplResult::done()
            },

            OperationResult::StreamDeleted => {
                let stream_id = self.inner.take_event_stream_id();

                self.promise.reject(OperationError::StreamDeleted(stream_id));

                ImplResult::done()
            },

            OperationResult::InvalidTransaction => {
                self.promise.reject(OperationError::InvalidTransaction);

                ImplResult::done()
            }

            OperationResult::AccessDenied => {
                let stream_id = self.inner.take_event_stream_id();

                self.promise.reject(OperationError::AccessDenied(stream_id));

                ImplResult::done()
            },
        }
    }

    fn report_operation_error(&mut self, error: OperationError) {
        self.promise.reject(error)
    }
}

pub struct TransactionWrite {
    stream: Chars,
    promise: Promise<()>,
    inner: messages::TransactionWrite,
}

impl TransactionWrite {
    pub fn new(promise: Promise<()>, stream: Chars) -> TransactionWrite {
        TransactionWrite {
            stream,
            promise,
            inner: messages::TransactionWrite::new(),
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

impl OperationImpl for TransactionWrite {
    fn initial_request(&self) -> Request {
        Request {
            cmd: Cmd::TransactionWrite,
            msg: &self.inner,
        }
    }

    fn is_valid_response(&self, cmd: Cmd) -> bool {
        Cmd::TransactionWriteCompleted == cmd
    }

    fn respond(&mut self, _: &mut ReqBuffer, pkg: Pkg) -> ::std::io::Result<ImplResult> {
        let response: messages::TransactionWriteCompleted =
                pkg.to_message()?;

        match response.get_result() {
            OperationResult::Success => {
                self.promise.accept(());

                ImplResult::done()
            },

            OperationResult::PrepareTimeout | OperationResult::ForwardTimeout | OperationResult::CommitTimeout => {
                ImplResult::retrying()
            },

            OperationResult::WrongExpectedVersion => {
                // You can't have a wrong expected version on a transaction
                // because, the write hasn't been committed yet.
                unreachable!()
            },

            OperationResult::StreamDeleted => {
                let stream = self.stream.clone();
                self.promise.reject(OperationError::StreamDeleted(stream));

                ImplResult::done()
            },

            OperationResult::InvalidTransaction => {
                self.promise.reject(OperationError::InvalidTransaction);

                ImplResult::done()
            }

            OperationResult::AccessDenied => {
                let stream = self.stream.clone();
                self.promise.reject(OperationError::AccessDenied(stream));

                ImplResult::done()
            },
        }
    }

    fn report_operation_error(&mut self, error: OperationError) {
        self.promise.reject(error)
    }
}

pub struct TransactionCommit {
    stream: Chars,
    version: types::ExpectedVersion,
    promise: Promise<types::WriteResult>,
    inner: messages::TransactionCommit,
}

impl TransactionCommit {
    pub fn new(
        promise: Promise<types::WriteResult>,
        stream: Chars,
        version: types::ExpectedVersion) -> TransactionCommit
    {
        TransactionCommit {
            stream,
            promise,
            version,
            inner: messages::TransactionCommit::new(),
        }
    }

    pub fn set_transaction_id(&mut self, value: types::TransactionId) {
        self.inner.set_transaction_id(value.0);
    }

    pub fn set_require_master(&mut self, value: bool) {
        self.inner.set_require_master(value);
    }
}

impl OperationImpl for TransactionCommit {
    fn initial_request(&self) -> Request {
        Request {
            cmd: Cmd::TransactionCommit,
            msg: &self.inner,
        }
    }

    fn is_valid_response(&self, cmd: Cmd) -> bool {
        Cmd::TransactionCommitCompleted == cmd
    }

    fn respond(&mut self, _: &mut ReqBuffer, pkg: Pkg) -> ::std::io::Result<ImplResult> {
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

                ImplResult::done()
            },

            OperationResult::PrepareTimeout | OperationResult::ForwardTimeout | OperationResult::CommitTimeout => {
                ImplResult::retrying()
            }

            OperationResult::WrongExpectedVersion => {
                let stream = self.stream.clone();

                self.promise.reject(OperationError::WrongExpectedVersion(stream, self.version));

                ImplResult::done()
            },

            OperationResult::StreamDeleted => {
                let stream = Chars::from(self.stream.deref());

                self.promise.reject(OperationError::StreamDeleted(stream));

                ImplResult::done()
            },

            OperationResult::InvalidTransaction => {
                self.promise.reject(OperationError::InvalidTransaction);

                ImplResult::done()
            }

            OperationResult::AccessDenied => {
                let stream = self.stream.clone();

                self.promise.reject(OperationError::AccessDenied(stream));

                ImplResult::done()
            },
        }
    }

    fn report_operation_error(&mut self, error: OperationError) {
        self.promise.reject(error);
    }
}

pub struct ReadStreamEvents {
    promise: Promise<types::ReadStreamStatus<types::StreamSlice>>,
    direction: types::ReadDirection,
    request_cmd: Cmd,
    response_cmd: Cmd,
    inner: messages::ReadStreamEvents,
}

impl ReadStreamEvents {
    pub fn new(
        promise: Promise<types::ReadStreamStatus<types::StreamSlice>>,
        direction: types::ReadDirection) -> ReadStreamEvents
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

impl OperationImpl for ReadStreamEvents {
    fn initial_request(&self) -> Request {
        Request {
            cmd: self.request_cmd,
            msg: &self.inner,
        }
    }

    fn is_valid_response(&self, cmd: Cmd) -> bool {
        self.response_cmd == cmd
    }

    fn respond(&mut self, _: &mut ReqBuffer, pkg: Pkg) -> ::std::io::Result<ImplResult> {
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
            },

            ReadStreamEventsCompleted_ReadStreamResult::NoStream => {
                let stream = self.inner.take_event_stream_id();

                self.report_error(types::ReadStreamError::NoStream(stream));
            },

            ReadStreamEventsCompleted_ReadStreamResult::StreamDeleted => {
                let stream = self.inner.take_event_stream_id();

                self.report_error(types::ReadStreamError::StreamDeleted(stream));
            },

            ReadStreamEventsCompleted_ReadStreamResult::AccessDenied => {
                let stream = self.inner.take_event_stream_id();

                self.report_error(types::ReadStreamError::AccessDenied(stream));
            },

            ReadStreamEventsCompleted_ReadStreamResult::NotModified => {
                let stream = self.inner.take_event_stream_id();

                self.report_error(types::ReadStreamError::NotModified(stream));
            },

            ReadStreamEventsCompleted_ReadStreamResult::Error => {
                let error_msg = response.take_error();

                self.report_error(types::ReadStreamError::Error(error_msg));
            },
        };

        ImplResult::done()
    }

    fn report_operation_error(&mut self, error: OperationError) {
        self.promise.reject(error);
    }
}

pub struct ReadAllEvents {
    promise: Promise<types::ReadStreamStatus<types::AllSlice>>,
    direction: types::ReadDirection,
    request_cmd: Cmd,
    response_cmd: Cmd,
    inner: messages::ReadAllEvents,
}

impl ReadAllEvents {
    pub fn new(
        promise: Promise<types::ReadStreamStatus<types::AllSlice>>,
        direction: types::ReadDirection) -> ReadAllEvents
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

impl OperationImpl for ReadAllEvents {
    fn initial_request(&self) -> Request {
        Request {
            cmd: self.request_cmd,
            msg: &self.inner,
        }
    }

    fn is_valid_response(&self, cmd: Cmd) -> bool {
        self.response_cmd == cmd
    }

    fn respond(&mut self, _: &mut ReqBuffer, pkg: Pkg) -> ::std::io::Result<ImplResult> {
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
            },

            ReadAllEventsCompleted_ReadAllResult::AccessDenied => {
                self.report_error(
                    types::ReadStreamError::AccessDenied("$all".into()));
            },

            ReadAllEventsCompleted_ReadAllResult::NotModified => {
                self.report_error(
                    types::ReadStreamError::NotModified("$all".into()));
            },

            ReadAllEventsCompleted_ReadAllResult::Error => {
                let error_msg = response.take_error();

                self.report_error(types::ReadStreamError::Error(error_msg));
            },
        };

        ImplResult::done()
    }

    fn report_operation_error(&mut self, error: OperationError) {
        self.promise.reject(error)
    }
}

pub struct DeleteStream {
    inner: messages::DeleteStream,
    promise: Promise<types::Position>,
}

impl DeleteStream {
    pub fn new(promise: Promise<types::Position>) -> DeleteStream {
        DeleteStream {
            inner: messages::DeleteStream::new(),
            promise,
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

impl OperationImpl for DeleteStream {
    fn initial_request(&self) -> Request {
        Request {
            cmd: Cmd::DeleteStream,
            msg: &self.inner,
        }
    }

    fn is_valid_response(&self, cmd: Cmd) -> bool {
        Cmd::DeleteStreamCompleted == cmd
    }

    fn respond(&mut self, _: &mut ReqBuffer, pkg: Pkg) -> ::std::io::Result<ImplResult> {
        let response: messages::DeleteStreamCompleted =
                pkg.to_message()?;

        match response.get_result() {
            OperationResult::Success => {
                let position = types::Position {
                    commit: response.get_commit_position(),
                    prepare: response.get_prepare_position(),
                };

                self.promise.accept(position);

                ImplResult::done()
            },

            OperationResult::PrepareTimeout | OperationResult::ForwardTimeout | OperationResult::CommitTimeout => {
                ImplResult::retrying()
            }

            OperationResult::WrongExpectedVersion => {
                let stream_id = self.inner.take_event_stream_id();
                let exp_i64   = self.inner.get_expected_version();
                let exp       = types::ExpectedVersion::from_i64(exp_i64);

                self.promise.reject(OperationError::WrongExpectedVersion(stream_id, exp));

                ImplResult::done()
            },

            OperationResult::StreamDeleted => {
                let stream_id = self.inner.take_event_stream_id();

                self.promise.reject(OperationError::StreamDeleted(stream_id));

                ImplResult::done()
            },

            OperationResult::InvalidTransaction => {
                self.promise.reject(OperationError::InvalidTransaction);

                ImplResult::done()
            }

            OperationResult::AccessDenied => {
                let stream_id = self.inner.take_event_stream_id();

                self.promise.reject(OperationError::AccessDenied(stream_id));

                ImplResult::done()
            },
        }
    }

    fn report_operation_error(&mut self, error: OperationError) {
        self.promise.reject(error)
    }
}

enum SubState {
    Requesting,
    Confirmed,
}

pub struct SubscribeToStream {
    sub_bus: mpsc::Sender<types::SubEvent>,
    inner: messages::SubscribeToStream,
    state: SubState,
}

impl SubscribeToStream {
    pub(crate) fn new(sub_bus: mpsc::Sender<types::SubEvent>)
        -> SubscribeToStream
    {
        SubscribeToStream {
            sub_bus,
            inner: messages::SubscribeToStream::new(),
            state: SubState::Requesting,
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
            error!("ERROR: Max unprocessed events limit reached!");
        }
    }
}

impl OperationImpl for SubscribeToStream {
    fn initial_request(&self) -> Request {
        Request {
            cmd: Cmd::SubscribeToStream,
            msg: &self.inner,
        }
    }

    fn is_valid_response(&self, cmd: Cmd) -> bool {
        if Cmd::SubscriptionDropped == cmd {
            return true
        }

        match self.state {
            SubState::Requesting => Cmd::SubscriptionConfirmed == cmd,
            SubState::Confirmed  => Cmd::StreamEventAppeared == cmd,
        }
    }

    fn respond(&mut self, _: &mut ReqBuffer, pkg: Pkg) -> ::std::io::Result<ImplResult> {
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

                self.state = SubState::Confirmed;
                self.publish(confirmed);

                ImplResult::awaiting()
            },

            Cmd::StreamEventAppeared => {
                let mut response: messages::StreamEventAppeared =
                    pkg.to_message()?;

                let event    = types::ResolvedEvent::new(response.take_event())?;
                let appeared = types::SubEvent::EventAppeared(event);

                self.publish(appeared);

                ImplResult::awaiting()
            }

            Cmd::SubscriptionDropped => {
                self.publish(types::SubEvent::Dropped);

                ImplResult::done()
            },

            _ => {
                // Will never happened, has error in subscription is
                // reported through `Cmd::SubscriptionDropped`
                // command.
                unreachable!()
            },
        }
    }

    fn report_operation_error(&mut self, _: OperationError) {
        self.publish(types::SubEvent::Dropped);
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

struct OperationExtractor<A, O: OperationImpl> {
    recv: mpsc::Receiver<Result<A, OperationError>>,
    inner: O,
}

impl <A, O: OperationImpl> OperationImpl for OperationExtractor<A, O> {
    fn initial_request(&self) -> Request {
        self.inner.initial_request()
    }

    fn is_valid_response(&self, cmd: Cmd) -> bool {
        self.inner.is_valid_response(cmd)
    }

    fn respond(&mut self, buffer: &mut ReqBuffer, pkg: Pkg) -> ::std::io::Result<ImplResult> {
        self.inner.respond(buffer, pkg)
    }

    fn report_operation_error(&mut self, error: OperationError) {
        self.inner.report_operation_error(error)
    }

    fn retry(&self, cmd: Cmd) -> Request {
        self.inner.retry(cmd)
    }
}

impl <A, O: OperationImpl> OperationExtractor<A, O> {
    fn new<F>(maker: F) -> OperationExtractor<A, O>
        where F: FnOnce(Promise<A>) -> O
    {
        let (recv, promise) = Promise::new(500);

        OperationExtractor {
            recv,
            inner: maker(promise),
        }
    }

    fn get_result(self) -> Result<A, OperationError> {
        single_value_future(self.recv).wait()
    }
}

pub(crate) struct Catchup {
    puller: Option<OperationExtractor<types::ReadStreamStatus<types::StreamSlice>, ReadStreamEvents>>,
    recv: mpsc::Receiver<types::SubEvent>,
    sender: mpsc::Sender<types::SubEvent>,
    sub: SubscribeToStream,
    resolve_link_tos: bool,
    require_master: bool,
    stream_id: Chars,
    event_number: i64,
    max_count: i32,
    // Number of events the subscription received already.
    flying_event_count: usize,
    has_caught_up: bool,
    last_position: Option<types::Position>,
    last_event_number: Option<i64>,
}

impl Catchup {
    pub(crate) fn new(
        stream_id: Chars,
        from: i64,
        max_count: i32,
        require_master: bool,
        resolve_link_tos: bool,
        sender: mpsc::Sender<types::SubEvent>) -> Catchup
    {
        let (tx, recv) = mpsc::channel(500);
        let mut sub    = SubscribeToStream::new(tx);

        sub.set_event_stream_id(stream_id.clone());
        sub.set_resolve_link_tos(resolve_link_tos);

        Catchup {
            puller: None,
            sub,
            stream_id,
            event_number: from,
            max_count,
            sender,
            recv,
            require_master,
            resolve_link_tos,
            flying_event_count: 0,
            has_caught_up: false,
            last_position: None,
            last_event_number: None,
        }
    }

    fn propagate_events(&mut self) {
        use std::mem;

        let mut events = Vec::new();

        // We need to move `self.recv` for a very small amount of time.
        // It would not be possible to do it without tricking the
        // Rust move semantic.
        unsafe {
            let mut recv = mem::replace(&mut self.recv, mem::uninitialized());
            let mut cpt  = 0;

            while cpt < self.flying_event_count {
                let (evt_opt, next) = recv.into_future().wait().ok().unwrap();
                let evt             = evt_opt.unwrap();

                recv =  next;
                cpt  += 1;

                let can_be_dispatched = match evt.event_appeared() {
                    Some(event) => {
                        let can_be_dispatched = match event.get_original_event() {
                            Some(event) => match self.last_event_number {
                                Some(ref old) => old < &event.event_number,
                                None          => true,
                            },

                            None => unreachable!(),
                        };

                        // We update catchup tracking state if we can dispatch
                        // that event.
                        if can_be_dispatched {
                            self.last_position     = event.position;
                            self.last_event_number = event.get_original_event().map(|e| e.event_number);
                        }

                        can_be_dispatched
                    },

                    None => true,
                };

                if can_be_dispatched {
                    events.push(evt);
                }
            }

            mem::forget(mem::replace(&mut self.recv, recv));
        }

        self.flying_event_count = 0;

        if !events.is_empty() {
            self.sender.clone().send_all(iter_ok(events)).wait().unwrap();
        }
    }

    fn pull(&mut self, buffer: &mut ReqBuffer)
        -> ::std::io::Result<()>
    {
        let stream_id        = self.stream_id.clone();
        let event_number     = self.event_number;
        let max_count        = self.max_count;
        let require_master   = self.require_master;
        let resolve_link_tos = self.resolve_link_tos;

        let extractor = OperationExtractor::new(|promise| {
            let mut op = ReadStreamEvents::new(promise, types::ReadDirection::Forward);

            op.set_event_stream_id(stream_id);
            op.set_from_event_number(event_number);
            op.set_max_count(max_count);
            op.set_require_master(require_master);
            op.set_resolve_link_tos(resolve_link_tos);

            op
        });

        buffer.push_req(extractor.initial_request())?;
        self.puller = Some(extractor);

        Ok(())
    }

    fn is_sub_pkg(&self, cmd: Cmd) -> bool {
        match cmd {
            Cmd::SubscriptionDropped | Cmd::StreamEventAppeared | Cmd::SubscriptionConfirmed
                => true,

            _ => false,
        }
    }
}

impl OperationImpl for Catchup {
    fn initial_request(&self) -> Request {
        self.sub.initial_request()
    }

    fn is_valid_response(&self, cmd: Cmd) -> bool {
        let valid_for_puller = self.puller.as_ref().map_or(false, |p| p.is_valid_response(cmd));

        self.sub.is_valid_response(cmd) || valid_for_puller
    }

    fn respond(&mut self, buffer: &mut ReqBuffer, pkg: Pkg) -> ::std::io::Result<ImplResult> {
        if self.is_sub_pkg(pkg.cmd) {
            let cmd    = pkg.cmd;
            let result = self.sub.respond(buffer, pkg)?;

            self.flying_event_count += 1;

            // Once we receive our subscription confirmation, we can start
            // reading the stream through.
            if cmd == Cmd::SubscriptionConfirmed {
                self.propagate_events();
                self.pull(buffer)?;
            } else {
                // We propagate live event only if we already caugh up the head of
                // the stream. Otherwise, we accumulate.
                if self.has_caught_up {
                    self.propagate_events();
                }
            }

            return Ok(result);
        }

        if let Some(mut puller) = self.puller.take() {
            let outcome = puller.respond(buffer, pkg)?;

            if outcome.is_done() {
                match puller.get_result() {
                    Err(error) => {
                        self.report_operation_error(error);

                        ImplResult::terminate()
                    },

                    Ok(read_result) => match read_result {
                        types::ReadStreamStatus::Error(error) =>  match error {
                            types::ReadStreamError::NoStream(_) | types::ReadStreamError::NotModified(_) => {
                                self.has_caught_up = true;
                                self.propagate_events();

                                ImplResult::done()
                            },

                            types::ReadStreamError::StreamDeleted(stream) => {
                                self.report_operation_error(OperationError::StreamDeleted(stream));

                                ImplResult::terminate()
                            },

                            types::ReadStreamError::AccessDenied(stream) => {
                                self.report_operation_error(OperationError::AccessDenied(stream));

                                ImplResult::terminate()
                            },

                            types::ReadStreamError::Error(msg) => {
                                self.report_operation_error(OperationError::ServerError(Some(msg)));

                                ImplResult::terminate()
                            },
                        },

                        types::ReadStreamStatus::Success(slice) => match slice.events() {
                            types::LocatedEvents::EndOfStream => {
                                self.has_caught_up = true;
                                self.propagate_events();

                                ImplResult::done()
                            },

                            types::LocatedEvents::Events { events, next } => {
                                for event in &events {
                                    self.last_position     = event.position;
                                    self.last_event_number = event.get_original_event()
                                                                  .map(|e| e.event_number);
                                }

                                let stream = iter_ok(events).map(types::SubEvent::EventAppeared);
                                let _      = self.sender.clone().send_all(stream).wait();

                                if let Some(next) = next {
                                    self.event_number = next;
                                    self.pull(buffer)?;
                                } else {
                                    self.has_caught_up = true;
                                    self.propagate_events();
                                }

                                ImplResult::done()
                            },
                        }
                    },
                }
            } else {
                Ok(outcome)
            }
        } else {
            warn!("Catchup subscription is in wrong state. \
                  Submit an issue in https://github.com/YoEight/eventstore-rs");

            self.report_operation_error(OperationError::wrong_client_impl());

            ImplResult::done()
        }
    }

    fn report_operation_error(&mut self, error: OperationError) {
        self.sub.report_operation_error(error.clone());

        if let Some(mut puller) = self.puller.take() {
            puller.report_operation_error(error);
        }
    }
}
