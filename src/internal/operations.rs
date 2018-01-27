use uuid::Uuid;

use protobuf::{ RepeatedField, Message };

use internal::data::EventData;
use internal::messages;
use internal::package::Pkg;
use internal::types::ExpectedVersion;

trait Operation {
    fn create(&self, correlation: Uuid) -> Pkg;
    fn inspect(&self, pkg: Pkg) -> bool;
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

    fn inspect(&self, _: Pkg) -> bool {
        false
    }
}