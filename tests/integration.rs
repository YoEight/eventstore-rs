extern crate eventstore;
extern crate futures;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
extern crate env_logger;
extern crate uuid;

use std::time::Duration;
use std::thread::spawn;
use futures::Future;
use eventstore::client::Client;
use eventstore::types::{ self, Credentials, Settings, EventData, StreamMetadata, SubscriptionEnv };
use uuid::Uuid;

struct TestSub {
    count: usize,
    max: usize,
}

impl types::SubscriptionConsumer for TestSub {
    fn when_confirmed(&mut self, id: Uuid, last_commit_position: i64, last_event_number: i64) {
        debug!("Subscription confirmed: {}, last_commit_position: {}, last_event_number: {}",
            id, last_commit_position, last_event_number);
    }

    fn when_event_appeared<E>(&mut self, _: &mut E, event: types::ResolvedEvent)
        -> types::OnEventAppeared
        where E: SubscriptionEnv
    {
        let event     = event.get_original_event().unwrap();
        let num       = &event.event_number;
        let stream_id = &event.event_stream_id;

        debug!("Event appeared, stream_id {}, num {}", stream_id, num);

        self.count += 1;

        if self.count == self.max {
            types::OnEventAppeared::Drop
        } else {
            types::OnEventAppeared::Continue
        }
    }

    fn when_dropped(&mut self) {
        debug!("Subscription dropped!");
    }
}

struct PersistentTestSub {
    count: usize,
    max: usize,
}

impl types::SubscriptionConsumer for PersistentTestSub {
    fn when_confirmed(&mut self, id: Uuid, last_commit_position: i64, last_event_number: i64) {
        debug!("Subscription confirmed: {}, last_commit_position: {}, last_event_number: {}",
            id, last_commit_position, last_event_number);
    }

    fn when_event_appeared<E>(&mut self, env: &mut E, event: types::ResolvedEvent)
        -> types::OnEventAppeared
        where E: SubscriptionEnv
    {
        let event     = event.get_original_event().unwrap();
        let num       = &event.event_number;
        let stream_id = &event.event_stream_id;

        debug!("Event appeared, stream_id {}, num {}", stream_id, num);

        self.count += 1;

        env.push_ack(event.event_id);

        if self.count == self.max {
            types::OnEventAppeared::Drop
        } else {
            types::OnEventAppeared::Continue
        }
    }

    fn when_dropped(&mut self) {
        debug!("Subscription dropped!");
    }
}

fn fresh_stream_id(prefix: &str) -> String {
    let uuid = Uuid::new_v4();

    format!("{}:{}", prefix, uuid)
}

fn generate_events(event_type: &str, cnt: usize) -> Vec<EventData> {
    let mut events = Vec::with_capacity(cnt);

    for idx in 1..cnt+1 {
        let payload = json!({
            "event_index": idx,
        });

        let data = EventData::json(event_type, payload);
        events.push(data);
    }

    events
}

fn generate_event(event_type: &str) -> EventData {
    generate_events(event_type, 1).pop().expect("Can't be empty")
}

// We write an event into a stream.
fn test_write_events(client: &Client) {
    let stream_id = fresh_stream_id("write-events");
    let event     = generate_event("write-events-test");

    let result = client.write_events(stream_id)
                       .push_event(event)
                       .execute()
                       .wait()
                       .unwrap();

    debug!("Write response: {:?}", result);
}

// We write an event into a stream then try to read it back.
fn test_read_event(client: &Client) {
    let stream_id = fresh_stream_id("read_event");
    let event     = generate_event("read_event_test");
    let _         = client.write_events(stream_id.as_str())
                          .push_event(event)
                          .execute()
                          .wait()
                          .unwrap();

    let result = client.read_event(stream_id.as_str(), 0)
                       .execute()
                       .wait()
                       .unwrap();

    debug!("Read response: {:?}", result);

    match result {
        types::ReadEventStatus::Success(ref result) => {
            let event = result.event.get_original_event().unwrap();
            let value: serde_json::Value = serde_json::from_slice(&event.data).unwrap();

            debug!("Payload as JSON {:?}", value);
        },

        _ => panic!("Something went wrong when reading stream {}", stream_id),
    }

}

// We write metadata to a stream then try to read it back.
fn test_write_and_read_stream_metadata(client: &Client) {
    let stream_id = fresh_stream_id("metadata");
    let duration  = Duration::from_secs(2 * 3_600) + Duration::from_millis(300);
    let metadata  =
        StreamMetadata::builder()
            .max_age(duration)
            .max_count(1000)
            .insert_custom_property("foo".to_owned(), "Bar!")
            .build();

    let result =
        client.write_stream_metadata(stream_id.as_str(), metadata)
              .execute()
              .wait()
              .unwrap();

    debug!("Write stream metadata {:?}", result);

    let result =
        client.read_stream_metadata(stream_id.as_str())
              .execute()
              .wait()
              .unwrap();

    debug!("Read stream metadata {:?}", result);

    match result {
        types::StreamMetadataResult::Success { metadata,.. } => {
            let read_max_age = metadata.max_age.unwrap();

            assert_eq!(read_max_age, duration);
        },

        _ => panic!("Something went wrong when reading stream {} metadata", stream_id),
    }
}

// We write a stream using a transaction. The write will be only be taken into
// account by the server after the `commit` call.
fn test_transaction(client: &Client) {
    let stream_id   = fresh_stream_id("transaction");
    let event       = generate_event("transaction_test");
    let transaction =
        client.start_transaction(stream_id.as_str())
              .execute()
              .wait()
              .unwrap();

    transaction.write_single(event)
               .wait()
               .unwrap();

    let result =
        transaction.commit()
                   .wait()
                   .unwrap();

    debug!("Transaction commit result {:?}", result);
}

// We read stream events by batch.
fn test_read_stream_events(client: &Client) {
    let stream_id = fresh_stream_id("read-stream-events");
    let events    = generate_events("read-stream-events-test", 10);

    let _ = client.write_events(stream_id.as_str())
                  .append_events(events)
                  .execute()
                  .wait()
                  .unwrap();

    let result =
        client.read_stream(stream_id.as_str())
              .execute()
              .wait()
              .unwrap();

    debug!("Read stream events result {:?}", result);
}

// We read $all system stream. We cannot write on $all stream. It's very
// unlikely for $all stream to be empty. From a personal note, I never saw that
// stream empty even right after booting the server.
fn test_read_all_stream(client: &Client) {
    let result =
        client.read_all()
              .max_count(10)
              .execute()
              .wait()
              .unwrap();

    debug!("Read $all events result {:?}", result);
}

// We write an event into a stream then delete that stream.
fn test_delete_stream(client: &Client) {
    let stream_id = fresh_stream_id("delete");
    let event     = generate_event("delete-test");

    let _ = client.write_events(stream_id.as_str())
                  .push_event(event)
                  .execute()
                  .wait()
                  .unwrap();

    let result = client.delete_stream(stream_id.as_str())
                       .hard_delete(true)
                       .execute()
                       .wait()
                       .unwrap();

    debug!("Delete stream [{}] result: {:?}", stream_id, result);
}

// We create a volatile subscription on a stream then write events into that
// same stream. We check our subscription consumer internal state to see it
// has consumed all the expected events.
fn test_volatile_subscription(client: &Client) {
    let stream_id = fresh_stream_id("volatile");
    let sub       = client.subcribe_to_stream(stream_id.as_str()).execute();
    let events    = generate_events("volatile-test", 3);
    let confirmation = sub.confirmation();

    let handle = spawn(move || {
        sub.consume(TestSub { count: 0, max: 3 })
    });

    let _ = confirmation.wait();

    let _ = client.write_events(stream_id)
                  .append_events(events)
                  .execute()
                  .wait()
                  .unwrap();

    let test_sub = handle.join().unwrap();

    assert_eq!(test_sub.count, 3, "We are testing proper state after volatile subscription: got {} expected {}.", test_sub.count, 3);
}

// We write events into a stream. Then, we issue a catchup subscription. After,
// we write another batch of events into the same stream. The goal is to make
// sure we receive events written prior and after our subscription request.
// To assess we received all the events we expected, we test our subscription
// internal state value.
fn test_catchup_subscription(client: &Client) {
    let stream_id     = fresh_stream_id("catchup");
    let events_before = generate_events("catchup-test-before", 3);
    let events_after  = generate_events("catchup-test-after", 3);

    let _ = client.write_events(stream_id.clone())
                  .append_events(events_before)
                  .execute()
                  .wait()
                  .unwrap();

    let sub = client.subscribe_to_stream_from(stream_id.clone()).execute();

    let handle = spawn(move || {
        sub.consume(TestSub { count: 0, max: 6 })
    });

    let _ = client.write_events(stream_id)
                  .append_events(events_after)
                  .execute()
                  .wait()
                  .unwrap();

    let test_sub = handle.join().unwrap();

    assert_eq!(test_sub.count, 6, "We are testing proper state after catchup subscription: got {} expected {}.", test_sub.count, 3);
}

// $all stream being a special system stream, we can not test as precisely as
// we did in `test_catchup_subscription`
fn test_catchup_all_subscription(client: &Client) {
    let sub = client.subscribe_to_all_from().execute();
    let tmp = sub.consume(TestSub { count: 0, max: 10 });

    assert_eq!(
        tmp.count,
        10,
        "We are testing proper state after $all catchup"
    );
}

// We test we can successfully create a persistent subscription.
fn test_create_persistent_subscription(client: &Client) {
    let stream_id = fresh_stream_id("create_persistent_sub");
    let result = client
        .create_persistent_subscription(stream_id, "a_group_name".to_string())
        .execute()
        .wait()
        .unwrap();

    assert_eq!(
        result,
        types::PersistActionResult::Success,
        "We expect create a persistent subscription to succeed",
    );
}

// We test we can successfully update a persistent subscription.
fn test_update_persistent_subscription(client: &Client) {
    let stream_id = fresh_stream_id("update_persistent_sub");
    let result = client
        .create_persistent_subscription(stream_id.clone(), "a_group_name".to_string())
        .execute()
        .wait()
        .unwrap();

    assert_eq!(
        result,
        types::PersistActionResult::Success,
        "We expect create a persistent subscription to succeed",
    );

    let mut setts = types::PersistentSubscriptionSettings::default();

    setts.max_retry_count = 1000;

    let result = client
        .update_persistent_subscription(stream_id, "a_group_name".to_string())
        .settings(setts)
        .execute()
        .wait()
        .unwrap();

    assert_eq!(
        result,
        types::PersistActionResult::Success,
        "We expect updating a persistent subscription to succeed",
    );
}

// We test we can successfully delete a persistent subscription.
fn test_delete_persistent_subscription(client: &Client) {
    let stream_id = fresh_stream_id("delete_persistent_sub");
    let result = client
        .create_persistent_subscription(stream_id.clone(), "a_group_name".to_string())
        .execute()
        .wait()
        .unwrap();

    assert_eq!(
        result,
        types::PersistActionResult::Success,
        "We expect create a persistent subscription to succeed",
    );

    let result = client
        .delete_persistent_subscription(stream_id, "a_group_name".to_string())
        .execute()
        .wait()
        .unwrap();

    assert_eq!(
        result,
        types::PersistActionResult::Success,
        "We expect deleting a persistent subscription to succeed",
    );
}

fn test_persistent_subscription(client: &Client) {
    let stream_id = fresh_stream_id("persistent_subscription");
    let events    = generate_events("persistent_subscription_test", 5);

    let _ = client
        .create_persistent_subscription(stream_id.clone(), "a_group_name".to_string())
        .execute()
        .wait()
        .unwrap();

    let _ = client.write_events(stream_id.clone())
                  .append_events(events)
                  .execute()
                  .wait()
                  .unwrap();

    let sub = client.connect_persistent_subscription(stream_id.clone(), "a_group_name".to_string())
                    .execute();

    let test_sub = sub.consume(PersistentTestSub { count: 0, max: 5 });

    assert_eq!(
        test_sub.count,
        5,
        "We are testing proper state after persistent subscription: got {} expected {}", test_sub.count, 5
    );
}

#[test]
fn all_round_operation_test() {
    env_logger::init();

    let mut settings = Settings::default();
    let login        = "admin";
    let password     = "changeit";

    settings.default_user = Some(Credentials::new(login, password));

    let client = Client::new(settings, "127.0.0.1:1113".parse().unwrap());

    client.start();

    test_write_events(&client);
    test_read_event(&client);
    test_write_and_read_stream_metadata(&client);
    test_transaction(&client);
    test_read_stream_events(&client);
    test_read_all_stream(&client);
    test_delete_stream(&client);
    test_volatile_subscription(&client);
    test_catchup_subscription(&client);
    test_catchup_all_subscription(&client);
    test_create_persistent_subscription(&client);
    test_update_persistent_subscription(&client);
    test_delete_persistent_subscription(&client);
    test_persistent_subscription(&client);
}
