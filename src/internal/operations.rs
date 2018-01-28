use uuid::Uuid;

use protobuf::{ RepeatedField, Message };
use protobuf::core::parse_from_bytes;

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
    WrongClientImpl(u8),
}

pub enum Decision {
    Success,
    Retry,
    Failed(OperationError),
}

pub trait Operation {
    fn create(&self, correlation: Uuid) -> Pkg;
    fn inspect(&self, pkg: Pkg) -> Decision;
}

pub struct WriteEvents {
    inner: messages::WriteEvents,
}

impl WriteEvents {
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
        let mut pkg     = Pkg::new(0x82, correlation);
        let mut payload = Vec::new();

        self.inner.write_to_vec(&mut payload).unwrap();

        pkg.set_payload(payload);

        pkg
    }

    fn inspect(&self, pkg: Pkg) -> Decision {
        match pkg.cmd {
            0x83 => {
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

                        Decision::Success
                    },

                    OperationResult::PrepareTimeout => Decision::Retry,
                    OperationResult::ForwardTimeout => Decision::Retry,
                    OperationResult::CommitTimeout  => Decision::Retry,

                    OperationResult::WrongExpectedVersion => {
                        let stream_id = self.inner.get_event_stream_id().to_string();
                        let exp_i64   = self.inner.get_expected_version();
                        let exp       = ExpectedVersion::from_i64(exp_i64);

                        Decision::Failed(OperationError::WrongExpectedVersion(stream_id, exp))
                    },

                    OperationResult::StreamDeleted => {
                        let stream_id = self.inner.get_event_stream_id().to_string();

                        Decision::Failed(OperationError::StreamDeleted(stream_id))
                    },

                    OperationResult::InvalidTransaction =>
                        Decision::Failed(OperationError::InvalidTransaction),

                    OperationResult::AccessDenied => {
                        let stream_id = self.inner.get_event_stream_id().to_string();

                        Decision::Failed(OperationError::AccessDenied(stream_id))
                    },
                }
            },

            _ => Decision::Failed(OperationError::WrongClientImpl(pkg.cmd)),
        }
    }
}