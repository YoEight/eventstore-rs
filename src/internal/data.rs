use uuid::Uuid;

use internal::messages;

pub struct EventData {
    inner: messages::NewEvent,
}

impl EventData {
    pub fn new() -> EventData {
        EventData {
            inner: messages::NewEvent::new(),
        }
    }

    pub fn set_id(&mut self, uuid: Uuid) {
        self.inner.set_event_id(uuid.as_bytes().to_vec());
    }

    pub fn set_type(&mut self, tpe: String) {
        self.inner.set_event_type(tpe);
    }

    pub fn set_json(&mut self, json: Vec<u8>) {
        self.inner.set_data_content_type(1);
        self.inner.set_data(json);
    }

    pub fn set_binary(&mut self, raw: Vec<u8>) {
        self.inner.set_data_content_type(0);
        self.inner.set_data(raw);
    }

    pub fn set_meta_json(&mut self, json: Vec<u8>) {
        self.inner.set_metadata_content_type(1);
        self.inner.set_metadata(json);
    }

    pub fn set_meta_raw(&mut self, raw: Vec<u8>) {
        self.inner.set_metadata_content_type(0);
        self.inner.set_metadata(raw);
    }

    pub fn build(self) -> messages::NewEvent {
        self.inner
    }
}