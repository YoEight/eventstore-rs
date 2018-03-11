use uuid::Uuid;

use serde_json::{ to_vec, Value };

use internal::messages;

pub struct EventData {
    inner: messages::NewEvent,
}

impl EventData {
    pub fn new_json(id_opt: Option<Uuid>, event_type: String, payload: Value) -> EventData {
        let mut this =
            EventData {
                inner: messages::NewEvent::new(),
            };

        let id = match id_opt {
            Some(u) => u,
            None    => Uuid::new_v4(),
        };

        this.set_id(id);
        this.set_type(event_type);
        this.set_json(payload);
        this.inner.set_metadata_content_type(0);

        this
    }

    pub fn new_binary(id_opt: Option<Uuid>, event_type: String, payload: Vec<u8>) -> EventData {
        let mut this =
            EventData {
                inner: messages::NewEvent::new(),
            };

        let id = match id_opt {
            Some(u) => u,
            None    => Uuid::new_v4(),
        };

        this.set_id(id);
        this.set_type(event_type);
        this.set_binary(payload);
        this.inner.set_metadata_content_type(0);

        this
    }

    pub fn set_id(&mut self, uuid: Uuid) {
        self.inner.set_event_id(uuid.as_bytes().to_vec());
    }

    pub fn set_type(&mut self, tpe: String) {
        self.inner.set_event_type(tpe);
    }

    pub fn set_json(&mut self, json: Value) {
        self.inner.set_data_content_type(1);
        self.inner.set_data(to_vec(&json).unwrap());
    }

    pub fn set_binary(&mut self, raw: Vec<u8>) {
        self.inner.set_data_content_type(0);
        self.inner.set_data(raw);
    }

    pub fn set_meta_json(&mut self, json: Value) {
        self.inner.set_metadata_content_type(1);
        self.inner.set_metadata(to_vec(&json).unwrap());
    }

    pub fn set_meta_raw(&mut self, raw: Vec<u8>) {
        self.inner.set_metadata_content_type(0);
        self.inner.set_metadata(raw);
    }

    pub fn build(self) -> messages::NewEvent {
        self.inner
    }
}