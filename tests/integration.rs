extern crate eventstore;
extern crate futures;
#[macro_use]
extern crate serde_json;
extern crate uuid;

use futures::Future;
use eventstore::client::Client;
use eventstore::internal::data::EventData;
use eventstore::internal::metadata::StreamMetadata;
use eventstore::internal::timespan::Timespan;
use eventstore::types::{ self, Credentials, Settings };
use uuid::Uuid;

#[test]
fn all_round_operation_test() {
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

    let fut =
        client.write_events("languages".to_owned())
              .push_event(EventData::json("foo-type".to_owned(), foo))
              .execute();

    let result = fut.wait().unwrap();

    println!("Write response: {:?}", result);

    let result = client.read_event("languages".to_owned(), 0)
                       .require_master(true)
                       .resolve_link_tos(true)
                       .execute()
                       .wait()
                       .unwrap();

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

    let result =
        client.write_stream_metadata("languages".to_owned(), metadata)
              .require_master(true)
              .execute()
              .wait()
              .unwrap();

    println!("Write stream metadata {:?}", result);

    let result =
        client.read_stream_metadata("languages".to_owned())
              .execute()
              .wait()
              .unwrap();

    println!("Read stream metadata {:?}", result);

    match result {
        types::StreamMetadataResult::Success { metadata,.. } => {
            let read_max_age = metadata.max_age.unwrap();

            assert_eq!(read_max_age, timespan);
            println!("Deserialized metadata {:?}", metadata);
        },

        _ => unreachable!(),
    }

    let transaction =
        client.start_transaction("languages-transaction".to_owned())
              .execute()
              .wait()
              .unwrap();

    let data = json!({
        "transactions_are_working_nicely": true,
    });

    transaction.write_single(EventData::json("foo-type-transaction".to_owned(), data))
               .wait()
               .unwrap();

    let result =
        transaction.commit()
                   .wait()
                   .unwrap();

    println!("Transaction commit result {:?}", result);

    let result =
        client.read_stream("languages".to_owned())
              .execute()
              .wait()
              .unwrap();

    println!("Read stream events result {:?}", result);

    let result =
        client.read_all()
              .max_count(10)
              .execute()
              .wait()
              .unwrap();

    println!(" Read $all events result {:?}", result);

    let uuid      = Uuid::new_v4();
    let stream_id = format!("foo:{}", uuid);
    let foo = json!({
        "is_rust_a_nice_language": true,
        "is_haskell_still_better": true,
    });

    let _ = client.write_events(stream_id.clone())
                  .push_event(EventData::json("foo-type".to_owned(), foo))
                  .execute()
                  .wait()
                  .unwrap();

    let result = client.delete_stream(stream_id.clone())
                       .hard_delete(true)
                       .execute()
                       .wait()
                       .unwrap();

    println!("Delete stream [{}] result: {:?}", stream_id, result);
}