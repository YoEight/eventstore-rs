use uuid::Uuid;

use serde::ser::Serialize;
use serde_json::to_vec;

use internal::messages;

enum Payload {
    Json(Vec<u8>),
    Binary(Vec<u8>),
}

pub struct EventData {
    event_type: String,
    payload: Payload,
    id_opt: Option<Uuid>,
    metadata_payload_opt: Option<Payload>,
}

impl EventData {
    pub fn json<P>(event_type: String, payload: P) -> EventData
        where P: Serialize
    {
        EventData {
            event_type,
            payload: Payload::Json(to_vec(&payload).unwrap()),
            id_opt: None,
            metadata_payload_opt: None,
        }
    }

    pub fn binary(event_type: String, payload: Vec<u8>) -> EventData {
        EventData {
            event_type,
            payload: Payload::Binary(payload),
            id_opt: None,
            metadata_payload_opt: None,
        }
    }

    pub fn id(mut self, value: Uuid) -> EventData {
        self.id_opt = Some(value);

        self
    }

    pub fn metadata_as_json<P>(mut self, payload: P) -> EventData
        where P: Serialize
    {
        self.metadata_payload_opt = Some(Payload::Json(to_vec(&payload).unwrap()));

        self
    }

    pub fn metadata_as_binary(mut self, payload: Vec<u8>) -> EventData {
        self.metadata_payload_opt = Some(Payload::Binary(payload));

        self
    }

    pub fn build(self) -> messages::NewEvent {
        let mut new_event = messages::NewEvent::new();
        let     id        = self.id_opt.unwrap_or_else(|| Uuid::new_v4());

        new_event.set_event_id(id.as_bytes().to_vec());

        match self.payload {
            Payload::Json(bin) => {
                new_event.set_data_content_type(1);
                new_event.set_data(bin);
            },

            Payload::Binary(bin) => {
                new_event.set_data_content_type(0);
                new_event.set_data(bin);
            },
        }

        match self.metadata_payload_opt {
            Some(Payload::Json(bin)) => {
                new_event.set_metadata_content_type(1);
                new_event.set_metadata(bin);
            },

            Some(Payload::Binary(bin)) => {
                new_event.set_metadata_content_type(0);
                new_event.set_metadata(bin);
            },

            None => new_event.set_metadata_content_type(0),
        }

        new_event.set_event_type(self.event_type);

        new_event
    }
}
