use self::messages::{
    CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult,
    DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult, OperationResult,
    ReadAllEventsCompleted_ReadAllResult, ReadStreamEventsCompleted_ReadStreamResult,
    UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult,
};
use crate::internal::command::Cmd;
use crate::internal::messages;
use crate::internal::messaging::{Lifetime, Msg, OpMsg};
use crate::internal::package::Pkg;
use crate::types::{self, Credentials, OperationError};
use bytes::{buf::BufMutExt, BytesMut};
use futures::channel::{mpsc, oneshot};
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use protobuf::{Chars, RepeatedField};
use std::ops::Deref;
use uuid::Uuid;

pub const DEFAULT_BOUNDED_SIZE: usize = 500;

fn create_pkg<M>(
    cmd: Cmd,
    creds_opt: Option<Credentials>,
    correlation: Option<Uuid>,
    msg: &mut M,
    dest: &mut BytesMut,
) -> std::io::Result<Pkg>
where
    M: protobuf::Message,
{
    dest.reserve(msg.compute_size() as usize);
    msg.write_to_writer(&mut dest.writer())?;

    let pkg = Pkg {
        cmd,
        creds_opt,
        correlation: correlation.unwrap_or_else(Uuid::new_v4),
        payload: dest.split().freeze(),
    };

    Ok(pkg)
}

pub struct WriteEvents {
    inner: messages::WriteEvents,
}

impl WriteEvents {
    pub fn new() -> Self {
        WriteEvents {
            inner: messages::WriteEvents::new(),
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

    pub async fn execute(
        mut self,
        creds_opt: Option<Credentials>,
        mut bus: mpsc::Sender<Msg>,
    ) -> Result<types::WriteResult, OperationError> {
        let (respond, promise) = oneshot::channel();

        tokio::spawn(async move {
            let (mailbox, mut recv) = mpsc::channel(DEFAULT_BOUNDED_SIZE);
            let mut buffer = BytesMut::new();
            let pkg = create_pkg(
                Cmd::WriteEvents,
                creds_opt,
                None,
                &mut self.inner,
                &mut buffer,
            )?;

            let _ = bus
                .send(Msg::Transmit(
                    Lifetime::OneTime(pkg.clone()),
                    mailbox.clone(),
                ))
                .await;

            while let Some(msg) = recv.next().await {
                match msg {
                    OpMsg::Recv(resp) => {
                        let response = resp.to_message::<messages::WriteEventsCompleted>()?;

                        match response.get_result() {
                            OperationResult::Success => {
                                let position = types::Position {
                                    commit: response.get_commit_position(),
                                    prepare: response.get_prepare_position(),
                                };

                                let result = types::WriteResult {
                                    next_expected_version: response.get_last_event_number(),
                                    position,
                                };

                                let _ = respond.send(Ok(result));
                                break;
                            }

                            OperationResult::PrepareTimeout
                            | OperationResult::ForwardTimeout
                            | OperationResult::CommitTimeout => {
                                let _ = bus
                                    .send(Msg::Transmit(
                                        Lifetime::OneTime(pkg.clone()),
                                        mailbox.clone(),
                                    ))
                                    .await;
                            }

                            OperationResult::WrongExpectedVersion => {
                                let stream_id = self.inner.take_event_stream_id().to_string();
                                let exp_i64 = self.inner.get_expected_version();
                                let exp = types::ExpectedVersion::from_i64(exp_i64);
                                let error = OperationError::WrongExpectedVersion(stream_id, exp);
                                let _ = respond.send(Err(error));

                                break;
                            }

                            OperationResult::StreamDeleted => {
                                let stream_id = self.inner.take_event_stream_id().to_string();
                                let error = OperationError::StreamDeleted(stream_id);
                                let _ = respond.send(Err(error));

                                break;
                            }

                            OperationResult::InvalidTransaction => {
                                let _ = respond.send(Err(OperationError::InvalidTransaction));

                                break;
                            }

                            OperationResult::AccessDenied => {
                                let stream_id = self.inner.take_event_stream_id().to_string();
                                let error = OperationError::AccessDenied(stream_id);
                                let _ = respond.send(Err(error));

                                break;
                            }
                        }
                    }

                    OpMsg::Failed(error) => {
                        let _ = respond.send(Err(error));
                        break;
                    }
                }
            }

            Ok(()) as std::io::Result<()>
        });

        promise.await.unwrap()
    }
}

pub struct ReadEvent {
    inner: messages::ReadEvent,
}

impl ReadEvent {
    pub fn new() -> Self {
        ReadEvent {
            inner: messages::ReadEvent::new(),
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

    pub async fn execute(
        mut self,
        creds_opt: Option<Credentials>,
        mut bus: mpsc::Sender<Msg>,
    ) -> Result<types::ReadEventStatus<types::ReadEventResult>, OperationError> {
        let (respond, promise) = oneshot::channel();

        tokio::spawn(async move {
            let (mailbox, mut recv) = mpsc::channel(DEFAULT_BOUNDED_SIZE);
            let mut buffer = BytesMut::new();
            let pkg = create_pkg(
                Cmd::ReadEvent,
                creds_opt,
                None,
                &mut self.inner,
                &mut buffer,
            )?;

            let _ = bus
                .send(Msg::Transmit(Lifetime::OneTime(pkg), mailbox))
                .await;

            if let Some(msg) = recv.next().await {
                match msg {
                    OpMsg::Recv(resp) => {
                        let mut response = resp.to_message::<messages::ReadEventCompleted>()?;

                        match response.get_result() {
                            messages::ReadEventCompleted_ReadEventResult::Success => {
                                let event = response.take_event();
                                let event = types::ResolvedEvent::new_from_indexed(event)?;
                                let event_number = self.inner.get_event_number();
                                let stream_id = self.inner.get_event_stream_id().to_owned();

                                let result = types::ReadEventResult {
                                    stream_id,
                                    event_number,
                                    event,
                                };

                                let result = types::ReadEventStatus::Success(result);
                                let _ = respond.send(Ok(result));
                            }

                            messages::ReadEventCompleted_ReadEventResult::NotFound => {
                                let _ = respond.send(Ok(types::ReadEventStatus::NotFound));
                            }

                            messages::ReadEventCompleted_ReadEventResult::NoStream => {
                                let _ = respond.send(Ok(types::ReadEventStatus::NoStream));
                            }

                            messages::ReadEventCompleted_ReadEventResult::StreamDeleted => {
                                let _ = respond.send(Ok(types::ReadEventStatus::Deleted));
                            }

                            messages::ReadEventCompleted_ReadEventResult::Error => {
                                let error = response.take_error().to_string();
                                let error = OperationError::ServerError(Some(error));
                                let _ = respond.send(Err(error));
                            }

                            messages::ReadEventCompleted_ReadEventResult::AccessDenied => {
                                let stream_id = self.inner.take_event_stream_id().to_string();
                                let error = OperationError::AccessDenied(stream_id);
                                let _ = respond.send(Err(error));
                            }
                        }
                    }

                    OpMsg::Failed(error) => {
                        let _ = respond.send(Err(error));
                    }
                }
            }

            Ok(()) as std::io::Result<()>
        });

        promise.await.unwrap()
    }
}

pub struct TransactionStart {
    inner: messages::TransactionStart,
}

impl TransactionStart {
    pub fn new() -> Self {
        TransactionStart {
            inner: messages::TransactionStart::new(),
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

    pub async fn execute(
        mut self,
        creds_opt: Option<Credentials>,
        mut bus: mpsc::Sender<Msg>,
    ) -> Result<types::TransactionId, OperationError> {
        let (respond, promise) = oneshot::channel();

        tokio::spawn(async move {
            let (mailbox, mut recv) = mpsc::channel(DEFAULT_BOUNDED_SIZE);
            let mut buffer = BytesMut::new();
            let pkg = create_pkg(
                Cmd::TransactionStart,
                creds_opt,
                None,
                &mut self.inner,
                &mut buffer,
            )?;

            let _ = bus
                .send(Msg::Transmit(
                    Lifetime::OneTime(pkg.clone()),
                    mailbox.clone(),
                ))
                .await;

            while let Some(msg) = recv.next().await {
                match msg {
                    OpMsg::Recv(resp) => {
                        let response = resp.to_message::<messages::TransactionStartCompleted>()?;

                        match response.get_result() {
                            OperationResult::Success => {
                                let id = response.get_transaction_id();
                                let _ = respond.send(Ok(types::TransactionId::new(id)));

                                break;
                            }

                            OperationResult::PrepareTimeout
                            | OperationResult::ForwardTimeout
                            | OperationResult::CommitTimeout => {
                                let _ = bus
                                    .send(Msg::Transmit(
                                        Lifetime::OneTime(pkg.clone()),
                                        mailbox.clone(),
                                    ))
                                    .await;
                            }

                            OperationResult::WrongExpectedVersion => {
                                let stream_id = self.inner.take_event_stream_id().to_string();
                                let exp_i64 = self.inner.get_expected_version();
                                let exp = types::ExpectedVersion::from_i64(exp_i64);
                                let error = OperationError::WrongExpectedVersion(stream_id, exp);
                                let _ = respond.send(Err(error));

                                break;
                            }

                            OperationResult::StreamDeleted => {
                                let stream_id = self.inner.take_event_stream_id().to_string();
                                let error = OperationError::StreamDeleted(stream_id);
                                let _ = respond.send(Err(error));

                                break;
                            }

                            OperationResult::InvalidTransaction => {
                                let _ = respond.send(Err(OperationError::InvalidTransaction));

                                break;
                            }

                            OperationResult::AccessDenied => {
                                let stream_id = self.inner.take_event_stream_id().to_string();
                                let error = OperationError::AccessDenied(stream_id);
                                let _ = respond.send(Err(error));

                                break;
                            }
                        }
                    }

                    OpMsg::Failed(error) => {
                        let _ = respond.send(Err(error));
                        break;
                    }
                }
            }

            Ok(()) as std::io::Result<()>
        });

        promise.await.unwrap()
    }
}

pub struct TransactionWrite {
    stream: Chars,
    inner: messages::TransactionWrite,
}

impl TransactionWrite {
    pub fn new(stream: Chars) -> Self {
        TransactionWrite {
            stream,
            inner: messages::TransactionWrite::new(),
        }
    }

    pub fn set_transaction_id(&mut self, value: types::TransactionId) {
        self.inner.set_transaction_id(value.0)
    }

    pub fn set_events<I>(&mut self, events: I)
    where
        I: IntoIterator<Item = types::EventData>,
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

    pub async fn execute(
        mut self,
        creds_opt: Option<Credentials>,
        mut bus: mpsc::Sender<Msg>,
    ) -> Result<(), OperationError> {
        let (respond, promise) = oneshot::channel();

        tokio::spawn(async move {
            let (mailbox, mut recv) = mpsc::channel(DEFAULT_BOUNDED_SIZE);
            let mut buffer = BytesMut::new();
            let pkg = create_pkg(
                Cmd::TransactionWrite,
                creds_opt,
                None,
                &mut self.inner,
                &mut buffer,
            )?;

            let _ = bus
                .send(Msg::Transmit(
                    Lifetime::OneTime(pkg.clone()),
                    mailbox.clone(),
                ))
                .await;

            while let Some(msg) = recv.next().await {
                match msg {
                    OpMsg::Recv(resp) => {
                        let response = resp.to_message::<messages::TransactionWriteCompleted>()?;

                        match response.get_result() {
                            OperationResult::Success => {
                                let _ = respond.send(Ok(()));

                                break;
                            }

                            OperationResult::PrepareTimeout
                            | OperationResult::ForwardTimeout
                            | OperationResult::CommitTimeout => {
                                let _ = bus
                                    .send(Msg::Transmit(
                                        Lifetime::OneTime(pkg.clone()),
                                        mailbox.clone(),
                                    ))
                                    .await;
                            }

                            OperationResult::WrongExpectedVersion => {
                                // You can't have a wrong expected version on a transaction
                                // because, the write hasn't been committed yet.
                                unreachable!()
                            }

                            OperationResult::StreamDeleted => {
                                let stream_id = self.stream.deref().into();
                                let error = OperationError::StreamDeleted(stream_id);
                                let _ = respond.send(Err(error));

                                break;
                            }

                            OperationResult::InvalidTransaction => {
                                let _ = respond.send(Err(OperationError::InvalidTransaction));

                                break;
                            }

                            OperationResult::AccessDenied => {
                                let stream_id = self.stream.deref().into();
                                let error = OperationError::AccessDenied(stream_id);
                                let _ = respond.send(Err(error));

                                break;
                            }
                        }
                    }

                    OpMsg::Failed(error) => {
                        let _ = respond.send(Err(error));
                        break;
                    }
                }
            }

            Ok(()) as std::io::Result<()>
        });

        promise.await.unwrap()
    }
}

pub struct TransactionCommit {
    stream: Chars,
    version: types::ExpectedVersion,
    inner: messages::TransactionCommit,
}

impl TransactionCommit {
    pub fn new(stream: Chars, version: types::ExpectedVersion) -> TransactionCommit {
        TransactionCommit {
            stream,
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

    pub async fn execute(
        mut self,
        creds_opt: Option<Credentials>,
        mut bus: mpsc::Sender<Msg>,
    ) -> Result<types::WriteResult, OperationError> {
        let (respond, promise) = oneshot::channel();

        tokio::spawn(async move {
            let (mailbox, mut recv) = mpsc::channel(DEFAULT_BOUNDED_SIZE);
            let mut buffer = BytesMut::new();
            let pkg = create_pkg(
                Cmd::TransactionCommit,
                creds_opt,
                None,
                &mut self.inner,
                &mut buffer,
            )?;

            let _ = bus
                .send(Msg::Transmit(
                    Lifetime::OneTime(pkg.clone()),
                    mailbox.clone(),
                ))
                .await;

            while let Some(msg) = recv.next().await {
                match msg {
                    OpMsg::Recv(resp) => {
                        let response = resp.to_message::<messages::TransactionCommitCompleted>()?;

                        match response.get_result() {
                            OperationResult::Success => {
                                let position = types::Position {
                                    commit: response.get_commit_position(),
                                    prepare: response.get_prepare_position(),
                                };

                                let result = types::WriteResult {
                                    next_expected_version: response.get_last_event_number(),
                                    position,
                                };

                                let _ = respond.send(Ok(result));

                                break;
                            }

                            OperationResult::PrepareTimeout
                            | OperationResult::ForwardTimeout
                            | OperationResult::CommitTimeout => {
                                let _ = bus
                                    .send(Msg::Transmit(
                                        Lifetime::OneTime(pkg.clone()),
                                        mailbox.clone(),
                                    ))
                                    .await;
                            }

                            OperationResult::WrongExpectedVersion => {
                                let stream = self.stream.deref().into();
                                let error =
                                    OperationError::WrongExpectedVersion(stream, self.version);
                                let _ = respond.send(Err(error));

                                break;
                            }

                            OperationResult::StreamDeleted => {
                                let stream_id = self.stream.deref().into();
                                let error = OperationError::StreamDeleted(stream_id);
                                let _ = respond.send(Err(error));

                                break;
                            }

                            OperationResult::InvalidTransaction => {
                                let _ = respond.send(Err(OperationError::InvalidTransaction));

                                break;
                            }

                            OperationResult::AccessDenied => {
                                let stream_id = self.stream.deref().into();
                                let error = OperationError::AccessDenied(stream_id);
                                let _ = respond.send(Err(error));

                                break;
                            }
                        }
                    }

                    OpMsg::Failed(error) => {
                        let _ = respond.send(Err(error));
                        break;
                    }
                }
            }

            Ok(()) as std::io::Result<()>
        });

        promise.await.unwrap()
    }
}

pub struct ReadStreamEvents {
    direction: types::ReadDirection,
    request_cmd: Cmd,
    pub inner: messages::ReadStreamEvents,
}

impl ReadStreamEvents {
    pub fn new(direction: types::ReadDirection) -> Self {
        let request_cmd = match direction {
            types::ReadDirection::Forward => Cmd::ReadStreamEventsForward,
            types::ReadDirection::Backward => Cmd::ReadStreamEventsBackward,
        };

        ReadStreamEvents {
            direction,
            request_cmd,
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

    pub async fn execute(
        mut self,
        creds_opt: Option<Credentials>,
        mut bus: mpsc::Sender<Msg>,
    ) -> Result<types::ReadStreamStatus<types::StreamSlice>, OperationError> {
        let (respond, promise) = oneshot::channel();

        tokio::spawn(async move {
            let (mailbox, mut recv) = mpsc::channel(DEFAULT_BOUNDED_SIZE);
            let mut buffer = BytesMut::new();
            let pkg = create_pkg(
                self.request_cmd,
                creds_opt,
                None,
                &mut self.inner,
                &mut buffer,
            )?;

            let _ = bus
                .send(Msg::Transmit(Lifetime::OneTime(pkg), mailbox))
                .await;

            if let Some(msg) = recv.next().await {
                match msg {
                    OpMsg::Recv(resp) => {
                        let mut response =
                            resp.to_message::<messages::ReadStreamEventsCompleted>()?;

                        match response.get_result() {
                            ReadStreamEventsCompleted_ReadStreamResult::Success => {
                                let is_eof = response.get_is_end_of_stream();
                                let events = response.take_events().into_vec();
                                let mut resolveds = Vec::with_capacity(events.len());

                                for event in events {
                                    let resolved = types::ResolvedEvent::new_from_indexed(event)?;

                                    resolveds.push(resolved);
                                }

                                let next_num_opt = {
                                    if !is_eof {
                                        Some(response.get_next_event_number())
                                    } else {
                                        None
                                    }
                                };

                                let from = self.inner.get_from_event_number();
                                let slice = types::StreamSlice::new(
                                    self.direction,
                                    from,
                                    resolveds,
                                    next_num_opt,
                                );
                                let result = types::ReadStreamStatus::Success(slice);
                                let _ = respond.send(Ok(result));
                            }

                            ReadStreamEventsCompleted_ReadStreamResult::NoStream => {
                                let stream = self.inner.take_event_stream_id().to_string();
                                let error = types::ReadStreamError::NoStream(stream);
                                let error = types::ReadStreamStatus::Error(error);
                                let _ = respond.send(Ok(error));
                            }

                            ReadStreamEventsCompleted_ReadStreamResult::StreamDeleted => {
                                let stream = self.inner.take_event_stream_id().to_string();
                                let error = types::ReadStreamError::StreamDeleted(stream);
                                let error = types::ReadStreamStatus::Error(error);
                                let _ = respond.send(Ok(error));
                            }

                            ReadStreamEventsCompleted_ReadStreamResult::AccessDenied => {
                                let stream = self.inner.take_event_stream_id().to_string();
                                let error = types::ReadStreamError::AccessDenied(stream);
                                let error = types::ReadStreamStatus::Error(error);
                                let _ = respond.send(Ok(error));
                            }

                            ReadStreamEventsCompleted_ReadStreamResult::NotModified => {
                                let stream = self.inner.take_event_stream_id().to_string();
                                let error = types::ReadStreamError::NotModified(stream);
                                let error = types::ReadStreamStatus::Error(error);
                                let _ = respond.send(Ok(error));
                            }

                            ReadStreamEventsCompleted_ReadStreamResult::Error => {
                                let error = response.take_error().to_string();
                                let error = OperationError::ServerError(Some(error));
                                let _ = respond.send(Err(error));
                            }
                        }
                    }

                    OpMsg::Failed(error) => {
                        let _ = respond.send(Err(error));
                    }
                }
            }

            Ok(()) as std::io::Result<()>
        });

        promise.await.unwrap()
    }
}

pub struct ReadAllEvents {
    direction: types::ReadDirection,
    request_cmd: Cmd,
    inner: messages::ReadAllEvents,
}

impl ReadAllEvents {
    pub fn new(direction: types::ReadDirection) -> Self {
        let request_cmd = match direction {
            types::ReadDirection::Forward => Cmd::ReadAllEventsForward,
            types::ReadDirection::Backward => Cmd::ReadAllEventsBackward,
        };

        ReadAllEvents {
            direction,
            request_cmd,
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

    pub async fn execute(
        mut self,
        creds_opt: Option<Credentials>,
        mut bus: mpsc::Sender<Msg>,
    ) -> Result<types::ReadStreamStatus<types::AllSlice>, OperationError> {
        let (respond, promise) = oneshot::channel();

        tokio::spawn(async move {
            let (mailbox, mut recv) = mpsc::channel(DEFAULT_BOUNDED_SIZE);
            let mut buffer = BytesMut::new();
            let pkg = create_pkg(
                self.request_cmd,
                creds_opt,
                None,
                &mut self.inner,
                &mut buffer,
            )?;

            let _ = bus
                .send(Msg::Transmit(Lifetime::OneTime(pkg), mailbox))
                .await;

            if let Some(msg) = recv.next().await {
                match msg {
                    OpMsg::Recv(resp) => {
                        let mut response = resp.to_message::<messages::ReadAllEventsCompleted>()?;

                        match response.get_result() {
                            ReadAllEventsCompleted_ReadAllResult::Success => {
                                let commit = response.get_commit_position();
                                let prepare = response.get_prepare_position();
                                let nxt_commit = response.get_next_commit_position();
                                let nxt_prepare = response.get_next_prepare_position();
                                let events = response.take_events().into_vec();
                                let mut resolveds = Vec::with_capacity(events.len());

                                for event in events {
                                    let resolved = types::ResolvedEvent::new(event)?;

                                    resolveds.push(resolved);
                                }

                                let from = types::Position { commit, prepare };

                                let next = types::Position {
                                    commit: nxt_commit,
                                    prepare: nxt_prepare,
                                };

                                let slice =
                                    types::AllSlice::new(self.direction, from, resolveds, next);
                                let result = types::ReadStreamStatus::Success(slice);
                                let _ = respond.send(Ok(result));
                            }

                            ReadAllEventsCompleted_ReadAllResult::AccessDenied => {
                                let error = types::ReadStreamError::AccessDenied("$all".into());
                                let error = types::ReadStreamStatus::Error(error);
                                let _ = respond.send(Ok(error));
                            }

                            ReadAllEventsCompleted_ReadAllResult::NotModified => {
                                let error = types::ReadStreamError::NotModified("$all".into());
                                let error = types::ReadStreamStatus::Error(error);
                                let _ = respond.send(Ok(error));
                            }

                            ReadAllEventsCompleted_ReadAllResult::Error => {
                                let error = response.take_error().to_string();
                                let error = OperationError::ServerError(Some(error));
                                let _ = respond.send(Err(error));
                            }
                        }
                    }

                    OpMsg::Failed(error) => {
                        let _ = respond.send(Err(error));
                    }
                }
            }

            Ok(()) as std::io::Result<()>
        });

        promise.await.unwrap()
    }
}

pub struct DeleteStream {
    inner: messages::DeleteStream,
}

impl DeleteStream {
    pub fn new() -> Self {
        DeleteStream {
            inner: messages::DeleteStream::new(),
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

    pub async fn execute(
        mut self,
        creds_opt: Option<Credentials>,
        mut bus: mpsc::Sender<Msg>,
    ) -> Result<types::Position, OperationError> {
        let (respond, promise) = oneshot::channel();

        tokio::spawn(async move {
            let (mailbox, mut recv) = mpsc::channel(DEFAULT_BOUNDED_SIZE);
            let mut buffer = BytesMut::new();
            let pkg = create_pkg(
                Cmd::DeleteStream,
                creds_opt,
                None,
                &mut self.inner,
                &mut buffer,
            )?;

            let _ = bus
                .send(Msg::Transmit(
                    Lifetime::OneTime(pkg.clone()),
                    mailbox.clone(),
                ))
                .await;

            while let Some(msg) = recv.next().await {
                match msg {
                    OpMsg::Recv(resp) => {
                        let response = resp.to_message::<messages::DeleteStreamCompleted>()?;

                        match response.get_result() {
                            OperationResult::Success => {
                                let position = types::Position {
                                    commit: response.get_commit_position(),
                                    prepare: response.get_prepare_position(),
                                };

                                let _ = respond.send(Ok(position));
                                break;
                            }

                            OperationResult::PrepareTimeout
                            | OperationResult::ForwardTimeout
                            | OperationResult::CommitTimeout => {
                                let _ = bus
                                    .send(Msg::Transmit(
                                        Lifetime::OneTime(pkg.clone()),
                                        mailbox.clone(),
                                    ))
                                    .await;
                            }

                            OperationResult::WrongExpectedVersion => {
                                let stream_id = self.inner.take_event_stream_id().to_string();
                                let exp_i64 = self.inner.get_expected_version();
                                let exp = types::ExpectedVersion::from_i64(exp_i64);
                                let error = OperationError::WrongExpectedVersion(stream_id, exp);
                                let _ = respond.send(Err(error));

                                break;
                            }

                            OperationResult::StreamDeleted => {
                                let stream_id = self.inner.take_event_stream_id().to_string();
                                let error = OperationError::StreamDeleted(stream_id);
                                let _ = respond.send(Err(error));

                                break;
                            }

                            OperationResult::InvalidTransaction => {
                                let _ = respond.send(Err(OperationError::InvalidTransaction));

                                break;
                            }

                            OperationResult::AccessDenied => {
                                let stream_id = self.inner.take_event_stream_id().to_string();
                                let error = OperationError::AccessDenied(stream_id);
                                let _ = respond.send(Err(error));

                                break;
                            }
                        }
                    }

                    OpMsg::Failed(error) => {
                        let _ = respond.send(Err(error));
                        break;
                    }
                }
            }

            Ok(()) as std::io::Result<()>
        });

        promise.await.unwrap()
    }
}

pub struct SubscribeToStream {
    inner: messages::SubscribeToStream,
}

impl SubscribeToStream {
    pub fn new() -> SubscribeToStream {
        SubscribeToStream {
            inner: messages::SubscribeToStream::new(),
        }
    }

    pub fn set_event_stream_id(&mut self, stream_id: Chars) {
        self.inner.set_event_stream_id(stream_id);
    }

    pub fn set_resolve_link_tos(&mut self, value: bool) {
        self.inner.set_resolve_link_tos(value);
    }

    pub fn execute(
        mut self,
        creds_opt: Option<Credentials>,
        mut bus: mpsc::Sender<Msg>,
    ) -> types::Subscription {
        let (mut respond, promise) = mpsc::channel(DEFAULT_BOUNDED_SIZE);
        let bus_cloned = bus.clone();

        tokio::spawn(async move {
            let (mailbox, mut recv) = mpsc::channel(DEFAULT_BOUNDED_SIZE);
            let mut buffer = BytesMut::new();
            let pkg = create_pkg(
                Cmd::SubscribeToStream,
                creds_opt,
                None,
                &mut self.inner,
                &mut buffer,
            )?;
            let sub_id = pkg.correlation;
            let _ = bus
                .send(Msg::Transmit(Lifetime::KeepAlive(pkg), mailbox))
                .await;

            while let Some(msg) = recv.next().await {
                match msg {
                    OpMsg::Recv(resp) => {
                        match resp.cmd {
                            Cmd::SubscriptionConfirmed => {
                                info!("Subscription [{}] is confirmed", sub_id);

                                let response =
                                    resp.to_message::<messages::SubscriptionConfirmation>()?;
                                let last_commit_position = response.get_last_commit_position();
                                let last_event_number = response.get_last_event_number();
                                let confirmed = types::SubEvent::Confirmed {
                                    id: sub_id,
                                    last_commit_position,
                                    last_event_number,
                                    persistent_id: None,
                                };

                                let _ = respond.send(confirmed).await;
                            }

                            Cmd::StreamEventAppeared => {
                                let mut response =
                                    resp.to_message::<messages::StreamEventAppeared>()?;
                                let event = types::ResolvedEvent::new(response.take_event())?;

                                debug!("Subscription [{}] event appeared [{:?}]", sub_id, event);

                                let appeared = types::SubEvent::EventAppeared {
                                    event: Box::new(event),
                                    retry_count: 0,
                                };

                                let _ = respond.send(appeared).await;
                            }

                            Cmd::SubscriptionDropped => {
                                info!("Subscription [{}] has dropped", sub_id);

                                let _ = respond.send(types::SubEvent::Dropped).await;
                                break;
                            }

                            _ => {
                                // Will never happened, has error in subscription is
                                // reported through `Cmd::SubscriptionDropped`
                                // command.
                                unreachable!()
                            }
                        }
                    }

                    OpMsg::Failed(error) => {
                        error!(
                            "Subscription [{:?}] has dropped because of error: {}",
                            sub_id, error
                        );

                        let _ = respond.send(types::SubEvent::Dropped).await;
                        break;
                    }
                }
            }

            Ok(()) as std::io::Result<()>
        });

        types::Subscription {
            receiver: promise,
            sender: bus_cloned,
        }
    }
}

enum CatchupLiveLoop {
    Continue,
    Break,
}

enum CatchupLoop<A> {
    Continue(A),
    Break,
}

enum Mode<A> {
    Catchup(A),
    Live(A),
}

impl<A> Mode<A> {
    fn offset(&self) -> A
    where
        A: Copy,
    {
        match *self {
            Mode::Catchup(a) => a,
            Mode::Live(a) => a,
        }
    }
}

type CatchupLoopResult<Offset> =
    std::io::Result<CatchupLoop<(Vec<types::ResolvedEvent>, Option<Offset>)>>;

trait Track {
    type Offset: Copy + Send;

    fn get_offset(&self, event: &types::ResolvedEvent) -> Self::Offset;
}

trait Catchup: Track {
    type Msg: protobuf::Message;
    type Resp: protobuf::Message;

    fn update_read_msg(&self, msg: &mut Self::Msg, offset: Self::Offset);

    fn iterate(&self, sub_id: Uuid, resp: &mut Self::Resp) -> CatchupLoopResult<Self::Offset>;
}

struct RegularTrack {
    stream: protobuf::Chars,
}

impl Track for RegularTrack {
    type Offset = i64;

    fn get_offset(&self, event: &types::ResolvedEvent) -> Self::Offset {
        event.get_original_event().event_number
    }
}

impl Catchup for RegularTrack {
    type Msg = messages::ReadStreamEvents;
    type Resp = messages::ReadStreamEventsCompleted;

    fn update_read_msg(&self, msg: &mut Self::Msg, offset: Self::Offset) {
        msg.set_from_event_number(offset);
    }

    fn iterate(&self, sub_id: Uuid, response: &mut Self::Resp) -> CatchupLoopResult<Self::Offset> {
        match response.get_result() {
            ReadStreamEventsCompleted_ReadStreamResult::Success => {
                let is_eos = response.get_is_end_of_stream();
                let events = response.take_events().into_vec();
                let mut resolved_events = Vec::with_capacity(events.len());
                let next = if is_eos {
                    None
                } else {
                    Some(response.get_next_event_number())
                };

                for event in events {
                    let resolved = types::ResolvedEvent::new_from_indexed(event)?;

                    resolved_events.push(resolved);
                }

                Ok(CatchupLoop::Continue((resolved_events, next)))
            }

            ReadStreamEventsCompleted_ReadStreamResult::NoStream => {
                Ok(CatchupLoop::Continue((vec![], None)))
            }

            ReadStreamEventsCompleted_ReadStreamResult::StreamDeleted => {
                error!(
                    "Catchup subscription [{:?}] has dropped because stream [{:?}] is deleted",
                    sub_id,
                    self.stream.to_string(),
                );

                Ok(CatchupLoop::Break)
            }

            ReadStreamEventsCompleted_ReadStreamResult::AccessDenied => {
                error!(
                    "Catchup subscription [{:?}] has dropped because of access denied on [{:?}]",
                    sub_id,
                    self.stream.to_string(),
                );

                Ok(CatchupLoop::Break)
            }

            ReadStreamEventsCompleted_ReadStreamResult::NotModified => {
                Ok(CatchupLoop::Continue((vec![], None)))
            }

            ReadStreamEventsCompleted_ReadStreamResult::Error => {
                let error = response.take_error().to_string();

                error!(
                    "Catchup subscription [{:?}] has dropped because of a server error: {:?}",
                    sub_id, error,
                );

                Ok(CatchupLoop::Break)
            }
        }
    }
}

struct AllTrack();

impl Track for AllTrack {
    type Offset = types::Position;

    fn get_offset(&self, event: &types::ResolvedEvent) -> Self::Offset {
        event
            .position
            .expect("position property should be defined when reading from $all stream")
    }
}

impl Catchup for AllTrack {
    type Msg = messages::ReadAllEvents;
    type Resp = messages::ReadAllEventsCompleted;

    fn update_read_msg(&self, msg: &mut Self::Msg, offset: Self::Offset) {
        msg.set_commit_position(offset.commit);
        msg.set_prepare_position(offset.prepare);
    }

    fn iterate(&self, sub_id: Uuid, response: &mut Self::Resp) -> CatchupLoopResult<Self::Offset> {
        match response.get_result() {
            ReadAllEventsCompleted_ReadAllResult::Success => {
                let nxt_commit = response.get_next_commit_position();
                let nxt_prepare = response.get_next_prepare_position();
                let events = response.take_events().into_vec();
                let mut resolved_events = Vec::with_capacity(events.len());
                let next = types::Position {
                    commit: nxt_commit,
                    prepare: nxt_prepare,
                };

                for event in events {
                    let resolved = types::ResolvedEvent::new(event)?;

                    resolved_events.push(resolved);
                }

                if !resolved_events.is_empty() {
                    Ok(CatchupLoop::Continue((resolved_events, Some(next))))
                } else {
                    // ^ Stands for end of stream
                    Ok(CatchupLoop::Continue((resolved_events, None)))
                }
            }

            ReadAllEventsCompleted_ReadAllResult::AccessDenied => {
                error!(
                    "Catchup subscription [{:?}] has dropped because of access denied on [$all]",
                    sub_id,
                );

                Ok(CatchupLoop::Break)
            }

            ReadAllEventsCompleted_ReadAllResult::NotModified => {
                Ok(CatchupLoop::Continue((vec![], None)))
            }

            ReadAllEventsCompleted_ReadAllResult::Error => {
                let error = response.take_error().to_string();

                error!(
                    "Catchup subscription [{:?}] has dropped because of a server error: {:?}",
                    sub_id, error,
                );

                Ok(CatchupLoop::Break)
            }
        }
    }
}

fn catchup_impl<C>(
    track: C,
    creds_opt: Option<Credentials>,
    start_position: <C as Track>::Offset,
    read_cmd: Cmd,
    mut read_msg: <C as Catchup>::Msg,
    mut sub_msg: messages::SubscribeToStream,
    mut bus: mpsc::Sender<Msg>,
) -> types::Subscription
where
    C: Catchup + std::marker::Sync + std::marker::Send + 'static,
{
    use futures::stream::iter;

    let (mut respond, promise) = mpsc::channel(DEFAULT_BOUNDED_SIZE);
    let bus_cloned = bus.clone();

    tokio::spawn(async move {
        let (mailbox, mut recv) = mpsc::channel(DEFAULT_BOUNDED_SIZE);
        let mut buffer = BytesMut::new();
        let mut mode = Mode::Catchup(start_position);

        let mut sub_pkg = create_pkg(
            Cmd::SubscribeToStream,
            creds_opt.clone(),
            None,
            &mut sub_msg,
            &mut buffer,
        )?;

        let mut read_pkg = create_pkg(
            read_cmd,
            creds_opt.clone(),
            None,
            &mut read_msg,
            &mut buffer,
        )?;

        let mut read_id = read_pkg.correlation;
        let mut sub_id = sub_pkg.correlation;
        let _ = bus
            .send(Msg::Transmit(Lifetime::OneTime(read_pkg), mailbox.clone()))
            .await;

        let _ = bus
            .send(Msg::Transmit(
                Lifetime::KeepAlive(sub_pkg.clone()),
                mailbox.clone(),
            ))
            .await;

        while let Some(msg) = recv.next().await {
            match msg {
                OpMsg::Recv(resp) => {
                    if sub_id == resp.correlation {
                        let outcome =
                            handle_catchup_sub(sub_id, &track, &resp, &mut respond, &mut mode)
                                .await?;

                        if let CatchupLiveLoop::Break = outcome {
                            break;
                        }
                    // I know I didn't do that check for other operations but considering in
                    // some situations we can have multiple pending requests, better be
                    // cautious.
                    } else if read_id == resp.correlation {
                        let mut response = resp.to_message::<<C as Catchup>::Resp>()?;

                        match track.iterate(sub_id, &mut response)? {
                            CatchupLoop::Continue((batch, next_opt)) => {
                                let last = batch.last().map(|evt| track.get_offset(evt));

                                if !batch.is_empty() {
                                    let events = batch.into_iter().map(|event| {
                                        let appeared = types::SubEvent::EventAppeared {
                                            event: Box::new(event),
                                            retry_count: 0,
                                        };

                                        Ok(appeared)
                                    });

                                    let _ = respond.send_all(&mut iter(events)).await;

                                    if let Some(next) = next_opt {
                                        mode = Mode::Catchup(next);

                                        track.update_read_msg(&mut read_msg, next);
                                        read_pkg = create_pkg(
                                            read_cmd,
                                            creds_opt.clone(),
                                            None,
                                            &mut read_msg,
                                            &mut buffer,
                                        )?;

                                        read_id = read_pkg.correlation;

                                        let _ = bus
                                            .send(Msg::Transmit(
                                                Lifetime::OneTime(read_pkg),
                                                mailbox.clone(),
                                            ))
                                            .await;
                                    } else {
                                        // Means we reach end of stream.
                                        let offset =
                                            last.expect("Batch was tested non empty earlier");

                                        mode = Mode::Live(offset);
                                    }
                                } else if let Mode::Catchup(offset) = mode {
                                    mode = Mode::Live(offset);
                                }
                            }

                            CatchupLoop::Break => {
                                let _ = respond.send(types::SubEvent::Dropped).await;

                                break;
                            }
                        }
                    }
                }

                OpMsg::Failed(error) => {
                    if let OperationError::ConnectionHasDropped = error {
                        let next = mode.offset();

                        track.update_read_msg(&mut read_msg, next);
                        read_pkg = create_pkg(
                            read_cmd,
                            creds_opt.clone(),
                            None,
                            &mut read_msg,
                            &mut buffer,
                        )?;

                        read_id = read_pkg.correlation;

                        let _ = bus
                            .send(Msg::Transmit(Lifetime::OneTime(read_pkg), mailbox.clone()))
                            .await;

                        sub_id = Uuid::new_v4();
                        sub_pkg.correlation = sub_id;

                        let _ = bus
                            .send(Msg::Transmit(
                                Lifetime::KeepAlive(sub_pkg.clone()),
                                mailbox.clone(),
                            ))
                            .await;

                        continue;
                    }

                    error!(
                        "subscription [{:?}] has dropped because of error: {}",
                        sub_id, error
                    );

                    let _ = respond.send(types::SubEvent::Dropped).await;
                    break;
                }
            }
        }

        Ok(()) as std::io::Result<()>
    });

    types::Subscription {
        receiver: promise,
        sender: bus_cloned,
    }
}

async fn handle_catchup_sub<T: Track>(
    sub_id: Uuid,
    track: &T,
    resp: &Pkg,
    respond: &mut mpsc::Sender<types::SubEvent>,
    mode: &mut Mode<<T as Track>::Offset>,
) -> std::io::Result<CatchupLiveLoop> {
    match resp.cmd {
        Cmd::SubscriptionConfirmed => {
            info!("Catchup subscription [{}] is confirmed", sub_id);

            let response = resp.to_message::<messages::SubscriptionConfirmation>()?;
            let last_commit_position = response.get_last_commit_position();
            let last_event_number = response.get_last_event_number();
            let confirmed = types::SubEvent::Confirmed {
                id: sub_id,
                last_commit_position,
                last_event_number,
                persistent_id: None,
            };

            let _ = respond.send(confirmed).await;
        }

        Cmd::StreamEventAppeared => {
            if let Mode::Live(_) = mode {
                let mut response = resp.to_message::<messages::StreamEventAppeared>()?;
                let event = types::ResolvedEvent::new(response.take_event())?;
                let offset = track.get_offset(&event);

                debug!(
                    "Catchup subscription [{}] event appeared [{:?}]",
                    sub_id, event
                );

                let appeared = types::SubEvent::EventAppeared {
                    event: Box::new(event),
                    retry_count: 0,
                };

                let _ = respond.send(appeared).await;
                *mode = Mode::Live(offset);
            }
        }

        Cmd::SubscriptionDropped => {
            info!("Catchup subscription [{}] has dropped", sub_id);

            let _ = respond.send(types::SubEvent::Dropped).await;

            return Ok(CatchupLiveLoop::Break);
        }

        _ => {
            // Will never happened, has error in subscription is
            // reported through `Cmd::SubscriptionDropped`
            // command.
            unreachable!()
        }
    }

    Ok(CatchupLiveLoop::Continue)
}

pub struct CatchupRegularSubscription {
    pub require_master: bool,
    pub start_position: i64,
    pub resolve_link_tos: bool,
    pub batch_size: i32,
    pub stream: protobuf::Chars,
}

impl CatchupRegularSubscription {
    pub fn execute(
        self,
        creds_opt: Option<Credentials>,
        bus: mpsc::Sender<Msg>,
    ) -> types::Subscription {
        let track = RegularTrack {
            stream: self.stream,
        };

        let mut sub_msg = messages::SubscribeToStream::new();
        let mut read_msg = messages::ReadStreamEvents::new();

        sub_msg.set_event_stream_id(track.stream.clone());
        sub_msg.set_resolve_link_tos(self.resolve_link_tos);
        read_msg.set_event_stream_id(track.stream.clone());
        read_msg.set_from_event_number(self.start_position);
        read_msg.set_max_count(self.batch_size);
        read_msg.set_resolve_link_tos(self.resolve_link_tos);
        read_msg.set_require_master(self.require_master);

        catchup_impl(
            track,
            creds_opt,
            self.start_position,
            Cmd::ReadStreamEventsForward,
            read_msg,
            sub_msg,
            bus,
        )
    }
}

pub struct CatchupAllSubscription {
    pub require_master: bool,
    pub start_position: types::Position,
    pub resolve_link_tos: bool,
    pub batch_size: i32,
}

impl CatchupAllSubscription {
    pub fn execute(
        self,
        creds_opt: Option<Credentials>,
        bus: mpsc::Sender<Msg>,
    ) -> types::Subscription {
        let track = AllTrack();
        let mut sub_msg = messages::SubscribeToStream::new();
        let mut read_msg = messages::ReadAllEvents::new();

        sub_msg.set_resolve_link_tos(self.resolve_link_tos);
        sub_msg.set_event_stream_id("".into());
        read_msg.set_commit_position(self.start_position.commit);
        read_msg.set_prepare_position(self.start_position.prepare);
        read_msg.set_max_count(self.batch_size);
        read_msg.set_resolve_link_tos(self.resolve_link_tos);
        read_msg.set_require_master(self.require_master);

        catchup_impl(
            track,
            creds_opt,
            self.start_position,
            Cmd::ReadAllEventsForward,
            read_msg,
            sub_msg,
            bus,
        )
    }
}
pub struct CreatePersistentSubscription {
    inner: messages::CreatePersistentSubscription,
}

impl CreatePersistentSubscription {
    pub fn new() -> Self {
        CreatePersistentSubscription {
            inner: messages::CreatePersistentSubscription::new(),
        }
    }

    pub fn set_subscription_group_name(&mut self, name: Chars) {
        self.inner.set_subscription_group_name(name);
    }

    pub fn set_event_stream_id(&mut self, stream_id: Chars) {
        self.inner.set_event_stream_id(stream_id);
    }

    pub fn set_settings(&mut self, settings: &types::PersistentSubscriptionSettings) {
        self.inner.set_resolve_link_tos(settings.resolve_link_tos);
        self.inner.set_start_from(settings.start_from);
        self.inner
            .set_message_timeout_milliseconds(settings.msg_timeout.as_millis() as i32);
        self.inner.set_record_statistics(settings.extra_stats);
        self.inner
            .set_live_buffer_size(i32::from(settings.live_buf_size));
        self.inner
            .set_read_batch_size(i32::from(settings.read_batch_size));
        self.inner
            .set_buffer_size(i32::from(settings.history_buf_size));
        self.inner
            .set_max_retry_count(i32::from(settings.max_retry_count));
        self.inner.set_prefer_round_robin(false); // Legacy way of picking strategy.
        self.inner
            .set_checkpoint_after_time(settings.checkpoint_after.as_millis() as i32);
        self.inner
            .set_checkpoint_max_count(i32::from(settings.max_checkpoint_count));
        self.inner
            .set_checkpoint_min_count(i32::from(settings.min_checkpoint_count));
        self.inner
            .set_subscriber_max_count(i32::from(settings.max_subs_count));
        self.inner
            .set_named_consumer_strategy(settings.named_consumer_strategy.as_str().into());
    }

    pub async fn execute(
        mut self,
        creds_opt: Option<Credentials>,
        mut bus: mpsc::Sender<Msg>,
    ) -> Result<types::PersistActionResult, OperationError> {
        let (respond, promise) = oneshot::channel();

        tokio::spawn(async move {
            let (mailbox, mut recv) = mpsc::channel(DEFAULT_BOUNDED_SIZE);
            let mut buffer = BytesMut::new();
            let pkg = create_pkg(
                Cmd::CreatePersistentSubscription,
                creds_opt,
                None,
                &mut self.inner,
                &mut buffer,
            )?;

            let _ = bus
                .send(Msg::Transmit(Lifetime::OneTime(pkg), mailbox))
                .await;

            if let Some(msg) = recv.next().await {
                match msg {
                    OpMsg::Recv(resp) => {
                        let response =
                            resp.to_message::<messages::CreatePersistentSubscriptionCompleted>()?;

                        let result = match response.get_result() {
                            CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult::Success => {
                                types::PersistActionResult::Success
                            },

                            CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult::AlreadyExists => {
                                types::PersistActionResult::Failure(types::PersistActionError::AlreadyExists)
                            },

                            CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult::Fail => {
                                types::PersistActionResult::Failure(types::PersistActionError::Fail)
                            },

                            CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult::AccessDenied => {
                                types::PersistActionResult::Failure(types::PersistActionError::AccessDenied)
                            },
                        };

                        let _ = respond.send(Ok(result));
                    }

                    OpMsg::Failed(error) => {
                        let _ = respond.send(Err(error));
                    }
                }
            }

            Ok(()) as std::io::Result<()>
        });

        promise.await.unwrap()
    }
}

pub struct UpdatePersistentSubscription {
    inner: messages::UpdatePersistentSubscription,
}

impl UpdatePersistentSubscription {
    pub fn new() -> Self {
        UpdatePersistentSubscription {
            inner: messages::UpdatePersistentSubscription::new(),
        }
    }

    pub fn set_subscription_group_name(&mut self, name: Chars) {
        self.inner.set_subscription_group_name(name);
    }

    pub fn set_event_stream_id(&mut self, stream_id: Chars) {
        self.inner.set_event_stream_id(stream_id);
    }

    pub fn set_settings(&mut self, settings: types::PersistentSubscriptionSettings) {
        self.inner.set_resolve_link_tos(settings.resolve_link_tos);
        self.inner.set_start_from(settings.start_from);
        self.inner
            .set_message_timeout_milliseconds(settings.msg_timeout.as_millis() as i32);
        self.inner.set_record_statistics(settings.extra_stats);
        self.inner
            .set_live_buffer_size(i32::from(settings.live_buf_size));
        self.inner
            .set_read_batch_size(i32::from(settings.read_batch_size));
        self.inner
            .set_buffer_size(i32::from(settings.history_buf_size));
        self.inner
            .set_max_retry_count(i32::from(settings.max_retry_count));
        self.inner.set_prefer_round_robin(false); // Legacy way of picking strategy.
        self.inner
            .set_checkpoint_after_time(settings.checkpoint_after.as_millis() as i32);
        self.inner
            .set_checkpoint_max_count(i32::from(settings.max_checkpoint_count));
        self.inner
            .set_checkpoint_min_count(i32::from(settings.min_checkpoint_count));
        self.inner
            .set_subscriber_max_count(i32::from(settings.max_subs_count));
        self.inner
            .set_named_consumer_strategy(settings.named_consumer_strategy.as_str().into());
    }

    pub async fn execute(
        mut self,
        creds_opt: Option<Credentials>,
        mut bus: mpsc::Sender<Msg>,
    ) -> Result<types::PersistActionResult, OperationError> {
        let (respond, promise) = oneshot::channel();

        tokio::spawn(async move {
            let (mailbox, mut recv) = mpsc::channel(DEFAULT_BOUNDED_SIZE);
            let mut buffer = BytesMut::new();
            let pkg = create_pkg(
                Cmd::UpdatePersistentSubscription,
                creds_opt,
                None,
                &mut self.inner,
                &mut buffer,
            )?;

            let _ = bus
                .send(Msg::Transmit(Lifetime::OneTime(pkg), mailbox))
                .await;

            if let Some(msg) = recv.next().await {
                match msg {
                    OpMsg::Recv(resp) => {
                        let response =
                            resp.to_message::<messages::UpdatePersistentSubscriptionCompleted>()?;

                        let result = match response.get_result() {
                            UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult::Success => {
                                types::PersistActionResult::Success
                            },

                            UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult::DoesNotExist => {
                                types::PersistActionResult::Failure(types::PersistActionError::DoesNotExist)
                            },

                            UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult::Fail => {
                                types::PersistActionResult::Failure(types::PersistActionError::Fail)
                            },

                            UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult::AccessDenied => {
                                types::PersistActionResult::Failure(types::PersistActionError::AccessDenied)
                            },
                        };

                        let _ = respond.send(Ok(result));
                    }

                    OpMsg::Failed(error) => {
                        let _ = respond.send(Err(error));
                    }
                }
            }

            Ok(()) as std::io::Result<()>
        });

        promise.await.unwrap()
    }
}

pub struct DeletePersistentSubscription {
    inner: messages::DeletePersistentSubscription,
}

impl DeletePersistentSubscription {
    pub fn new() -> Self {
        DeletePersistentSubscription {
            inner: messages::DeletePersistentSubscription::new(),
        }
    }

    pub fn set_subscription_group_name(&mut self, name: Chars) {
        self.inner.set_subscription_group_name(name);
    }

    pub fn set_event_stream_id(&mut self, stream_id: Chars) {
        self.inner.set_event_stream_id(stream_id);
    }

    pub async fn execute(
        mut self,
        creds_opt: Option<Credentials>,
        mut bus: mpsc::Sender<Msg>,
    ) -> Result<types::PersistActionResult, OperationError> {
        let (respond, promise) = oneshot::channel();

        tokio::spawn(async move {
            let (mailbox, mut recv) = mpsc::channel(DEFAULT_BOUNDED_SIZE);
            let mut buffer = BytesMut::new();
            let pkg = create_pkg(
                Cmd::DeletePersistentSubscription,
                creds_opt,
                None,
                &mut self.inner,
                &mut buffer,
            )?;

            let _ = bus
                .send(Msg::Transmit(Lifetime::OneTime(pkg), mailbox))
                .await;

            if let Some(msg) = recv.next().await {
                match msg {
                    OpMsg::Recv(resp) => {
                        let response =
                            resp.to_message::<messages::DeletePersistentSubscriptionCompleted>()?;

                        let result = match response.get_result() {
                            DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult::Success => {
                                types::PersistActionResult::Success
                            },

                            DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult::DoesNotExist => {
                                types::PersistActionResult::Failure(types::PersistActionError::DoesNotExist)
                            },

                            DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult::Fail => {
                                types::PersistActionResult::Failure(types::PersistActionError::Fail)
                            },

                            DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult::AccessDenied => {
                                types::PersistActionResult::Failure(types::PersistActionError::AccessDenied)
                            },
                        };

                        let _ = respond.send(Ok(result));
                    }

                    OpMsg::Failed(error) => {
                        let _ = respond.send(Err(error));
                    }
                }
            }

            Ok(()) as std::io::Result<()>
        });

        promise.await.unwrap()
    }
}

pub struct ConnectToPersistentSubscription {
    inner: messages::ConnectToPersistentSubscription,
}

impl ConnectToPersistentSubscription {
    pub fn new() -> ConnectToPersistentSubscription {
        ConnectToPersistentSubscription {
            inner: messages::ConnectToPersistentSubscription::new(),
        }
    }

    pub fn set_event_stream_id(&mut self, stream_id: Chars) {
        self.inner.set_event_stream_id(stream_id);
    }

    pub fn set_group_name(&mut self, group_name: Chars) {
        self.inner.set_subscription_id(group_name);
    }

    pub fn set_buffer_size(&mut self, size: u16) {
        self.inner.set_allowed_in_flight_messages(i32::from(size));
    }

    pub fn execute(
        mut self,
        creds_opt: Option<Credentials>,
        mut bus: mpsc::Sender<Msg>,
    ) -> types::Subscription {
        let (mut respond, promise) = mpsc::channel(DEFAULT_BOUNDED_SIZE);
        let bus_cloned = bus.clone();

        tokio::spawn(async move {
            let (mailbox, mut recv) = mpsc::channel(DEFAULT_BOUNDED_SIZE);
            let mut buffer = BytesMut::new();
            let pkg = create_pkg(
                Cmd::ConnectToPersistentSubscription,
                creds_opt,
                None,
                &mut self.inner,
                &mut buffer,
            )?;

            let sub_id = pkg.correlation;
            let _ = bus
                .send(Msg::Transmit(Lifetime::KeepAlive(pkg), mailbox))
                .await;

            while let Some(msg) = recv.next().await {
                match msg {
                    OpMsg::Recv(resp) => {
                        match resp.cmd {
                            Cmd::PersistentSubscriptionConfirmation => {
                                info!(
                                    "Persistent subscription connection [{}] is confirmed",
                                    sub_id
                                );

                                let mut response = resp
                                    .to_message::<messages::PersistentSubscriptionConfirmation>(
                                )?;
                                let last_commit_position = response.get_last_commit_position();
                                let last_event_number = response.get_last_event_number();
                                let persistent_id = response.take_subscription_id().to_string();

                                let confirmed = types::SubEvent::Confirmed {
                                    id: sub_id,
                                    last_commit_position,
                                    last_event_number,
                                    persistent_id: Some(persistent_id),
                                };

                                let _ = respond.send(confirmed).await;
                            }

                            Cmd::PersistentSubscriptionStreamEventAppeared => {
                                let mut response = resp.to_message::<messages::PersistentSubscriptionStreamEventAppeared>()?;
                                let event =
                                    types::ResolvedEvent::new_from_indexed(response.take_event())?;
                                let retry_count = response.get_retryCount() as usize;

                                debug!(
                                    "Persistent subscription [{}] event appeared [{:?}]",
                                    sub_id, event
                                );

                                let appeared = types::SubEvent::EventAppeared {
                                    event: Box::new(event),
                                    retry_count,
                                };

                                let _ = respond.send(appeared).await;
                            }

                            Cmd::SubscriptionDropped => {
                                info!("Persistent subscription [{}] has dropped", sub_id);

                                let _ = respond.send(types::SubEvent::Dropped).await;
                                break;
                            }

                            _ => {
                                // Will never happened, has error in subscription is
                                // reported through `Cmd::SubscriptionDropped`
                                // command.
                                unreachable!()
                            }
                        }
                    }

                    OpMsg::Failed(error) => {
                        error!(
                            "Persistent subscription [{:?}] has dropped because of error: {}",
                            sub_id, error
                        );

                        let _ = respond.send(types::SubEvent::Dropped).await;
                        break;
                    }
                }
            }

            Ok(()) as std::io::Result<()>
        });

        types::Subscription {
            receiver: promise,
            sender: bus_cloned,
        }
    }
}
