extern crate core;
extern crate bytes;
extern crate uuid;
extern crate timer;
extern crate time;
extern crate futures;
extern crate protobuf;
extern crate tokio;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_timer;

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

// #[macro_use]
// extern crate log;

pub mod internal;
pub mod client;
// pub mod internal {
//     mod command;
//     mod messages;
//     mod messaging;
//     mod operations;
//     mod package;
//     pub mod types;
//     mod registry;
// }

#[cfg(test)]
mod tests {
    use std::thread;
    use futures::Future;
    use client::Client;
    use internal::types::{ self, Credentials, Settings, ExpectedVersion };
    use internal::data::EventData;
    use internal::metadata::StreamMetadata;
    use internal::timespan::Timespan;
    use serde_json;
    use time::Duration;

    #[test]
    fn it_works() {
        let mut settings = Settings::default();
        let login        = "admin".to_owned();
        let passw        = "changeit".to_owned();

        settings.default_user = Some(Credentials { login: login, password: passw });

        let client = Client::new(settings, "127.0.0.1:1113".parse().unwrap());

        client.start();

        let foo = json!({
            "is_rust_a_nice_language": true,
            "is_haskell_still_better": true,
        });

        let payload = EventData::new_json(None, "foo-type".to_owned(), foo);

        let fut = client.write_event("languages".to_owned(), payload, true, ExpectedVersion::Any, None);

        let result = fut.wait().unwrap();

        println!("Write response: {:?}", result);

        let fut    = client.read_event("languages".to_owned(), 0, true, true, None);
        let result = fut.wait().unwrap();

        println!("Read response: {:?}", result);

        match result {
            types::ReadEventStatus::Success(ref res) => {
                let event = res.event.get_original_event().unwrap();
                let value: serde_json::Value = serde_json::from_slice(&event.data).unwrap();

                println!("Payload as JSON {:?}", value);
            },

            _ => unreachable!(),
        }

        let timespan =
            Timespan::builder()
                .hours(2)
                .milliseconds(300)
                .build();

        let metadata = StreamMetadata::builder()
            .max_age(timespan)
            .max_count(1000)
            .insert_custom_property("foo".to_owned(), "Bar!")
            .build();

        let result = client.write_stream_metadata(
                "languages".to_owned(), metadata, true, ExpectedVersion::Any, None);

        let result = result.wait().unwrap();

        println!("Write stream metadata {:?}", result);
        //     thread::sleep(Duration::from_millis(1000));
        // }
    }
}
