use uuid::Uuid;

use protobuf::{ RepeatedField, Message };
use protobuf::core::parse_from_bytes;

use internal::command::Cmd;
use internal::data::EventData;
use internal::messages;
use internal::package::Pkg;
use internal::types::{ ExpectedVersion, Position, WriteResult };

use self::messages::OperationResult;

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

pub enum Decision {
    Done,
    Continue,
}

pub trait Operation {
    fn create(&self, correlation: Uuid) -> Pkg;
    fn inspect(&mut self, pkg: Pkg) -> Decision;
}

pub trait Promise<A> {
    fn accept(&self, result: A);
    fn reject(&self, error: OperationError);
}

pub struct WriteEvents {
    inner: messages::WriteEvents,
    promise: Box<Promise<WriteResult>>,
}

impl WriteEvents {
    pub fn new(promise: Box<Promise<WriteResult>>) -> WriteEvents {
        WriteEvents {
            inner: messages::WriteEvents::new(),
            promise: promise,
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
    fn create(&self, correlation: Uuid) -> Pkg {
        let mut pkg     = Pkg::new(Cmd::WriteEvents, correlation);
        let mut payload = Vec::new();

        self.inner.write_to_vec(&mut payload).unwrap();

        pkg.set_payload(payload);

        pkg
    }

    fn inspect(&mut self, pkg: Pkg) -> Decision {
        match pkg.cmd {
            Cmd::WriteEventsCompleted => {
                let response: messages::WriteEventsCompleted =
                        parse_from_bytes(&pkg.payload.as_slice()).unwrap();

                match response.get_result() {
                    OperationResult::Success => {
                        let position = Position {
                            commit: response.get_commit_position(),
                            prepare: response.get_prepare_position(),
                        };

                        let result = WriteResult {
                            next_expected_version: response.get_current_version(),
                            position: position,
                        };

                        self.promise.accept(result);

                        Decision::Done
                    },

                    OperationResult::PrepareTimeout => Decision::Continue,
                    OperationResult::ForwardTimeout => Decision::Continue,
                    OperationResult::CommitTimeout  => Decision::Continue,

                    OperationResult::WrongExpectedVersion => {
                        let stream_id = self.inner.get_event_stream_id().to_string();
                        let exp_i64   = self.inner.get_expected_version();
                        let exp       = ExpectedVersion::from_i64(exp_i64);

                        self.promise.reject(OperationError::WrongExpectedVersion(stream_id, exp));

                        Decision::Done
                    },

                    OperationResult::StreamDeleted => {
                        let stream_id = self.inner.get_event_stream_id().to_string();

                        self.promise.reject(OperationError::StreamDeleted(stream_id));

                        Decision::Done
                    },

                    OperationResult::InvalidTransaction => {
                        self.promise.reject(OperationError::InvalidTransaction);

                        Decision::Done
                    }

                    OperationResult::AccessDenied => {
                        let stream_id = self.inner.get_event_stream_id().to_string();

                        self.promise.reject(OperationError::AccessDenied(stream_id));

                        Decision::Done
                    },
                }
            },

            _ => {
                self.promise.reject(OperationError::WrongClientImpl(pkg.cmd));

                Decision::Done
            },
        }
    }
}
