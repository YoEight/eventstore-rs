use std::collections::HashMap;
use std::ops::Deref;
use std::time::{ Duration, Instant };

use bytes::{ BufMut, BytesMut };
use futures::{ Future, Stream };
use futures::sync::mpsc;
use protobuf::{ Chars, RepeatedField };
use uuid::Uuid;

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
    Continue(Vec<Pkg>),
}

impl Outcome {
    pub fn produced_pkgs(self) -> Vec<Pkg> {
        match self {
            Outcome::Done           => Vec::new(),
            Outcome::Continue(pkgs) => pkgs,
        }
    }

    pub fn is_continuing(&self) -> bool {
        if let Outcome::Continue(_) = *self {
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
    trackers: HashMap<Uuid, Tracking>,
}

struct Tracking {
    id: Uuid,
    attempts: usize,
    started: Instant,
    lasting: bool,
}

impl Default for Tracking {
    fn default() -> Tracking {
        Tracking {
            id: Uuid::new_v4(),
            attempts: 0,
            started: Instant::now(),
            lasting: false,
        }
    }
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
            trackers: HashMap::new(),
            creds,
            max_retry,
            timeout,
        }
    }

    fn send_req(&mut self, dest: &mut BytesMut, prev_tracker: Option<Tracking>) -> Decision {
        let mut tracker     = prev_tracker.unwrap_or_default();
        let     new_req_id  = Uuid::new_v4();
        let     prev_req_id = {
            if tracker.attempts > 0 {
                Some(tracker.id)
            } else {
                None
            }
        };

        let is_new_req = prev_req_id.is_none();
        let req        = self.inner.request(prev_req_id, new_req_id);

        if is_new_req {
            tracker.id = new_req_id;
        }

        tracker.attempts += 1;

        self.trackers.insert(new_req_id, tracker);

        req.send(new_req_id, self.creds.clone(), dest)
    }

    fn tracker_must_exist_mut<'a>(trackers: &'a mut HashMap<Uuid, Tracking>, id: Uuid)
        -> ::std::io::Result<&'a mut Tracking>
    {
        let error = ::std::io::Error::new(::std::io::ErrorKind::Other, "Tracker must exists");

        trackers.get_mut(&id).ok_or(error)
    }

    fn tracker_must_exist(&mut self, id: Uuid)
        -> ::std::io::Result<Tracking>
    {
        let error = ::std::io::Error::new(::std::io::ErrorKind::Other, "Tracker must exists");

        self.trackers.remove(&id).ok_or(error)
    }


    pub(crate) fn poll(&mut self, dest: &mut BytesMut, input: Option<Pkg>) -> Decision {
        match input {
            // It means this operation was newly created and has to issue its
            // first package to the server.
            None => self.send_req(dest, None),

            // At this point, it means this operation send a package to
            // the server already.
            Some(pkg) => {
                let corr_id = pkg.correlation;

                if self.inner.is_valid_response(pkg.cmd) {
                    let res = self.inner.respond(pkg)?;

                    match res {
                        ImplResult::Retry    => self.retry(dest, corr_id),
                        ImplResult::Continue => self.send_req(dest, None),
                        ImplResult::Done     => op_done(),
                        ImplResult::Awaiting => {
                            let tracker = OperationWrapper::tracker_must_exist_mut(&mut self.trackers, corr_id)?;

                            tracker.lasting = true;

                            op_continue()
                        },
                    }
                } else {
                    self.failed(OperationError::WrongClientImpl(pkg.cmd));

                    op_done()
                }
            },
        }
    }

    pub(crate) fn failed(&mut self, error: OperationError) {
        self.inner.report_operation_error(error);
    }

    pub(crate) fn retry(&mut self, dest: &mut BytesMut, id: Uuid) -> Decision {
        let tracker = self.tracker_must_exist(id)?;

        if tracker.attempts + 1 >= self.max_retry {
            self.failed(OperationError::Aborted);

            return op_done();
        }

        self.send_req(dest, Some(tracker))
    }

    pub(crate) fn check_and_retry(&mut self, dest: &mut BytesMut) -> Decision {
        let mut to_retry = Vec::new();

        for (key, tracker) in &self.trackers {
            if !tracker.lasting && tracker.started.elapsed() >= self.timeout {
                to_retry.push(*key);
            }
        }

        if to_retry.is_empty() {
            op_continue()
        } else {
            let mut pkgs = Vec::new();

            for key in to_retry {
                let decision = self.retry(dest, key);

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
}

pub(crate) struct Request<'a> {
    cmd: Cmd,
    msg: &'a ::protobuf::Message,
}

impl <'a> Request<'a> {
    fn send(self, id: Uuid, creds: Option<types::Credentials>, dest: &mut BytesMut) -> Decision {
        dest.reserve(self.msg.compute_size() as usize);

        self.msg.write_to_writer(&mut dest.writer())?;

        let pkg = Pkg {
            cmd: self.cmd,
            correlation: id,
            creds_opt: creds,
            payload: dest.take().freeze(),
        };

        op_send(pkg)
    }
}

pub(crate) enum ImplResult {
    Retry,
    Continue,
    Awaiting,
    Done,
}

impl ImplResult {
    fn retrying() -> ::std::io::Result<ImplResult> {
        Ok(ImplResult::Retry)
    }

    fn awaiting() -> ::std::io::Result<ImplResult> {
        Ok(ImplResult::Awaiting)
    }

    fn continuing() -> ::std::io::Result<ImplResult> {
        Ok(ImplResult::Continue)
    }

    fn done() -> ::std::io::Result<ImplResult> {
        Ok(ImplResult::Done)
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
    fn request(&mut self, prev_id: Option<Uuid>, new_id: Uuid) -> Request;
    fn is_valid_response(&self, cmd: Cmd) -> bool;
    fn respond(&mut self, pkg: Pkg) -> ::std::io::Result<ImplResult>;
    fn report_operation_error(&mut self, error: OperationError);
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
    fn request(&mut self, _: Option<Uuid>, _: Uuid) -> Request {
        Request {
            cmd: Cmd::WriteEvents,
            msg: &self.inner,
        }
    }

    fn is_valid_response(&self, cmd: Cmd) -> bool {
        Cmd::WriteEventsCompleted == cmd
    }

    fn respond(&mut self, pkg: Pkg) -> ::std::io::Result<ImplResult> {
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
    fn request(&mut self, _: Option<Uuid>, _: Uuid) -> Request {
        Request {
            cmd: Cmd::ReadEvent,
            msg: &self.inner,
        }
    }

    fn is_valid_response(&self, cmd: Cmd) -> bool {
        Cmd::ReadEventCompleted == cmd
    }

    fn respond(&mut self, pkg: Pkg) -> ::std::io::Result<ImplResult> {
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
    fn request(&mut self, _: Option<Uuid>, _: Uuid) -> Request {
        Request {
            cmd: Cmd::TransactionStart,
            msg: &self.inner,
        }
    }

    fn is_valid_response(&self, cmd: Cmd) -> bool {
        Cmd::TransactionStartCompleted == cmd
    }

    fn respond(&mut self, pkg: Pkg) -> ::std::io::Result<ImplResult> {
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
    fn request(&mut self, _: Option<Uuid>, _: Uuid) -> Request {
        Request {
            cmd: Cmd::TransactionWrite,
            msg: &self.inner,
        }
    }

    fn is_valid_response(&self, cmd: Cmd) -> bool {
        Cmd::TransactionWriteCompleted == cmd
    }

    fn respond(&mut self, pkg: Pkg) -> ::std::io::Result<ImplResult> {
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
    fn request(&mut self, _: Option<Uuid>, _: Uuid) -> Request {
        Request {
            cmd: Cmd::TransactionCommit,
            msg: &self.inner,
        }
    }

    fn is_valid_response(&self, cmd: Cmd) -> bool {
        Cmd::TransactionCommitCompleted == cmd
    }

    fn respond(&mut self, pkg: Pkg) -> ::std::io::Result<ImplResult> {
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
    fn request(&mut self, _: Option<Uuid>, _: Uuid) -> Request {
        Request {
            cmd: self.request_cmd,
            msg: &self.inner,
        }
    }

    fn is_valid_response(&self, cmd: Cmd) -> bool {
        self.response_cmd == cmd
    }

    fn respond(&mut self, pkg: Pkg) -> ::std::io::Result<ImplResult> {
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
    fn request(&mut self, _: Option<Uuid>, _: Uuid) -> Request {
        Request {
            cmd: self.request_cmd,
            msg: &self.inner,
        }
    }

    fn is_valid_response(&self, cmd: Cmd) -> bool {
        self.response_cmd == cmd
    }

    fn respond(&mut self, pkg: Pkg) -> ::std::io::Result<ImplResult> {
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
    fn request(&mut self, _: Option<Uuid>, _: Uuid) -> Request {
        Request {
            cmd: Cmd::DeleteStream,
            msg: &self.inner,
        }
    }

    fn is_valid_response(&self, cmd: Cmd) -> bool {
        Cmd::DeleteStreamCompleted == cmd
    }

    fn respond(&mut self, pkg: Pkg) -> ::std::io::Result<ImplResult> {
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
    _creds: Option<types::Credentials>,
    state: SubState,
}

impl SubscribeToStream {
    pub(crate) fn new(sub_bus: mpsc::Sender<types::SubEvent>, _creds: Option<types::Credentials>)
        -> SubscribeToStream
    {
        SubscribeToStream {
            sub_bus,
            _creds,
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
            print!("ERROR: Max unprocessed events limit reached!");
        }
    }
}

impl OperationImpl for SubscribeToStream {
    fn request(&mut self, _: Option<Uuid>, _: Uuid) -> Request {
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

    fn respond(&mut self, pkg: Pkg) -> ::std::io::Result<ImplResult> {
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

pub(crate) trait StreamPull {
    fn pull(&mut self, buf: &mut BytesMut, dest: &mut PullDest, pkg: Option<Pkg>) -> Decision;
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
    inner: OperationWrapper,
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
    operation_retry: usize,
    operation_timeout: Duration,
}

impl RegularStreamPull {
    pub(crate) fn new(
        stream_id: Chars,
        resolve_link_tos: bool,
        require_master: bool,
        batch_size: i32,
        start_from: i64,
        creds_opt: Option<types::Credentials>,
        settings: &types::Settings) -> RegularStreamPull
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
            operation_retry: settings.operation_retry.to_usize(),
            operation_timeout: settings.operation_timeout,
        }
    }

    fn prepare(&mut self, dest: &mut BytesMut) -> Decision {
        let (receiver, sender) = Promise::new(1);
        let mut inner          = ReadStreamEvents::new(sender, types::ReadDirection::Forward);

        inner.set_event_stream_id(self.stream_id.clone());
        inner.set_from_event_number(self.pos);
        inner.set_max_count(self.batch_size);
        inner.set_require_master(self.require_master);
        inner.set_resolve_link_tos(self.resolve_link_tos);

        let mut inner = OperationWrapper::new(inner,
                                              self.creds_opt.clone(),
                                              self.operation_retry,
                                              self.operation_timeout);

        let pkg     = inner.poll(dest, None);
        let process = PullProcess {
            receiver,
            inner,
        };

        self.process_opt = Some(process);

        pkg
    }

    fn accept_response(&mut self, buf: &mut BytesMut, dest: &mut PullDest, pkg: Pkg) -> Decision {
        if let Some(mut process) = self.process_opt.take() {
            let outcome = process.inner.poll(buf, Some(pkg))?;

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
    fn pull(&mut self, buf: &mut BytesMut, dest: &mut PullDest, pkg_opt: Option<Pkg>) -> Decision {
        match self.state {
            PullState::Prepare => {
                self.state = PullState::Await;

                self.prepare(buf)
            },

            PullState::Await => {
                match pkg_opt {
                    Some(pkg) => {
                        let corr_id  = pkg.correlation;
                        let decision = self.accept_response(buf, dest, pkg);

                        if decision_is_continuing(&decision) {
                            self.state = PullState::Prepare;
                            return self.pull(buf, dest, None);
                        }

                        decision
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
    sub_opt: Option<OperationWrapper>,
    state: CatchupState,
    settings: types::Settings,
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
        puller: P,
        settings: types::Settings) -> CatchupSubscribe<P>
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
            settings,
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

    fn prepare_sub_request(&mut self, dest: &mut BytesMut) -> Decision {
        let mut sub       = SubscribeToStream::new(self.sub_bus.clone(), self.creds_opt.clone());
        let     stream_id = self.stream_id_opt.clone().unwrap_or_default();

        sub.set_event_stream_id(stream_id);
        sub.set_resolve_link_tos(self.resolve_link_tos);

        let mut sub = OperationWrapper::new(sub,
                                            self.creds_opt.clone(),
                                            self.settings.operation_retry.to_usize(),
                                            self.settings.operation_timeout);

        let pkg = sub.poll(dest, None);
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

pub(crate) trait Operation {
    fn poll(&mut self, dest: &mut BytesMut, input: Option<Pkg>) -> Decision;
    fn failed(&mut self, error: OperationError);
    fn retry(&mut self, id: Uuid);
}

impl <P: StreamPull> Operation for CatchupSubscribe<P> {
    fn poll(&mut self, dest: &mut BytesMut, input: Option<Pkg>) -> Decision {
        match self.state {
            CatchupState::CatchingUp => {
                let outcome = self.puller.pull(dest, &mut self.dest, input)?;

                // Safe because if we shouldn't expect events or something went
                // wrong, it will be a `None`.
                if let Some(events) = self.dest.buffer_opt.take() {
                    self.notify_events_appeared(events)?;
                }

                if outcome.is_done() {
                    if let None = self.dest.error_opt.take() {
                        self.state = CatchupState::Subscribe;

                        self.poll(dest, None)
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
                    self.prepare_sub_request(dest)
                } else {
                    // If everything went smootly, we should receive a confirmation
                    // response.
                    self.state = CatchupState::Live;
                    let sub = self.sub_opt.as_mut().expect("Sub reference must be available on confirmation");

                    // If things didn't go well, `sub` will ask to drop the
                    // transaction with the server for us.
                    sub.poll(dest, input)
                }
            },

            CatchupState::Live => {
                // At this point we let the subcription driving the transaction
                // with the server for us.
                let sub = self.sub_opt.as_mut().expect("Sub reference must be available when live");

                sub.poll(dest, input)
            },
        }
    }

    fn failed(&mut self, _: OperationError) {
        // It would mean either the catching up phase or our subscription
        // request was invalid. All we can do is notify the user the
        // subscription has been dropped.
        let _ = self.notify_sub_drop();
    }

    fn retry(&mut self, _: Uuid) {
        // Nothing to do. Everything will be handle properly by `poll`
        // as we didn't update `CatchupSubscribe` stage state.
    }
}
