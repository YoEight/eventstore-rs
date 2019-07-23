#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

use eventstore::Slice;
use std::collections::HashMap;
use std::time::Duration;
use std::thread::spawn;
use futures::{ Future, Stream };
use uuid::Uuid;

struct TestSub {
    count: usize,
    max: usize,
}

impl eventstore::SubscriptionConsumer for TestSub {
    fn when_confirmed(&mut self, id: Uuid, last_commit_position: i64, last_event_number: i64) {
        debug!("Subscription confirmed: {}, last_commit_position: {}, last_event_number: {}",
            id, last_commit_position, last_event_number);
    }

    fn when_event_appeared<E>(&mut self, _: &mut E, event: Box<eventstore::ResolvedEvent>)
        -> eventstore::OnEventAppeared
        where E: eventstore::SubscriptionEnv
    {
        let event     = event.get_original_event().unwrap();
        let num       = &event.event_number;
        let stream_id = &event.event_stream_id;

        debug!("Event appeared, stream_id {}, num {}", stream_id, num);

        self.count += 1;

        if self.count == self.max {
            eventstore::OnEventAppeared::Drop
        } else {
            eventstore::OnEventAppeared::Continue
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

impl eventstore::SubscriptionConsumer for PersistentTestSub {
    fn when_confirmed(&mut self, id: Uuid, last_commit_position: i64, last_event_number: i64) {
        debug!("Subscription confirmed: {}, last_commit_position: {}, last_event_number: {}",
            id, last_commit_position, last_event_number);
    }

    fn when_event_appeared<E>(&mut self, env: &mut E, event: Box<eventstore::ResolvedEvent>)
        -> eventstore::OnEventAppeared
        where E: eventstore::SubscriptionEnv
    {
        let event     = event.get_original_event().unwrap();
        let num       = &event.event_number;
        let stream_id = &event.event_stream_id;

        debug!("Event appeared, stream_id {}, num {}", stream_id, num);

        self.count += 1;

        env.push_ack(event.event_id);

        if self.count == self.max {
            eventstore::OnEventAppeared::Drop
        } else {
            eventstore::OnEventAppeared::Continue
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

fn generate_events(event_type: &str, cnt: usize) -> Vec<eventstore::EventData> {
    let mut events = Vec::with_capacity(cnt);

    for idx in 1..cnt+1 {
        let payload = json!({
            "event_index": idx,
        });

        let data = eventstore::EventData::json(event_type, payload).unwrap();
        events.push(data);
    }

    events
}

fn generate_event(event_type: &str) -> eventstore::EventData {
    generate_events(event_type, 1).pop().expect("Can't be empty")
}

// We write an event into a stream.
fn test_write_events(connection: &eventstore::Connection) {
    let stream_id = fresh_stream_id("write-events");
    let event     = generate_event("write-events-test");

    let result = connection.write_events(stream_id)
                       .push_event(event)
                       .execute()
                       .wait()
                       .unwrap();

    debug!("Write response: {:?}", result);
}

// We write an event into a stream then try to read it back.
fn test_read_event(connection: &eventstore::Connection) {
    let stream_id = fresh_stream_id("read_event");
    let event     = generate_event("read_event_test");
    let _         = connection.write_events(stream_id.as_str())
                          .push_event(event)
                          .execute()
                          .wait()
                          .unwrap();

    let result = connection.read_event(stream_id.as_str(), 0)
                       .execute()
                       .wait()
                       .unwrap();

    debug!("Read response: {:?}", result);

    match result {
        eventstore::ReadEventStatus::Success(ref result) => {
            let event = result.event.get_original_event().unwrap();
            let value: serde_json::Value = serde_json::from_slice(&event.data).unwrap();

            debug!("Payload as JSON {:?}", value);
        },

        _ => panic!("Something went wrong when reading stream {}", stream_id),
    }

}

// We write metadata to a stream then try to read it back.
fn test_write_and_read_stream_metadata(connection: &eventstore::Connection) {
    let stream_id = fresh_stream_id("metadata");
    let duration  = Duration::from_secs(2 * 3_600) + Duration::from_millis(300);
    let metadata  =
        eventstore::StreamMetadata::builder()
            .max_age(duration)
            .max_count(1000)
            .insert_custom_property("foo".to_owned(), "Bar!")
            .build();

    let result =
        connection.write_stream_metadata(stream_id.as_str(), metadata)
              .execute()
              .wait()
              .unwrap();

    debug!("Write stream metadata {:?}", result);

    let result =
        connection.read_stream_metadata(stream_id.as_str())
              .execute()
              .wait()
              .unwrap();

    debug!("Read stream metadata {:?}", result);

    match result {
        eventstore::StreamMetadataResult::Success(result) => {
            let read_max_age = result.metadata.max_age.unwrap();

            assert_eq!(read_max_age, duration);
        },

        _ => panic!("Something went wrong when reading stream {} metadata", stream_id),
    }
}

// We write a stream using a transaction. The write will be only be taken into
// account by the server after the `commit` call.
fn test_transaction(connection: &eventstore::Connection) {
    let stream_id   = fresh_stream_id("transaction");
    let event       = generate_event("transaction_test");
    let transaction =
        connection.start_transaction(stream_id.as_str())
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

// We read stream events by batch. We also test if we can properly read a
// stream thoroughly.
fn test_read_stream_events(connection: &eventstore::Connection) {
    let stream_id = fresh_stream_id("read-stream-events");
    let events    = generate_events("read-stream-events-test", 10);

    let _ = connection.write_events(stream_id.as_str())
                  .append_events(events)
                  .execute()
                  .wait()
                  .unwrap();

    let mut pos = 0;
    let mut idx = 0;

    loop {
        let result =
            connection.read_stream(stream_id.as_str())
                  .start_from(pos)
                  .max_count(1)
                  .execute()
                  .wait()
                  .unwrap();

        match result {
            eventstore::ReadStreamStatus::Success(slice) => match slice.events() {
                eventstore::LocatedEvents::EndOfStream => {
                    break;
                },

                eventstore::LocatedEvents::Events { mut events, next } => {
                    let event = events.pop().unwrap();
                    let event = event.get_original_event().unwrap();
                    let obj: HashMap<String, i64> = event.as_json().unwrap();
                    let value = obj.get("event_index").unwrap();

                    idx = *value;

                    match next {
                        Some(n) => { pos = n },
                        None    => { break; },
                    }
                },
            },

            eventstore::ReadStreamStatus::Error(error) => {
                panic!("ReadStream error: {:?}", error);
            },
        }
    }

    assert_eq!(pos, 9);
    assert_eq!(idx, 10);
}

// Like `test_read_stream_events` but use `ReadStreamEvents::until_end_of_stream`
fn test_iterate_over_forward(connection: &eventstore::Connection) {
    let stream_id = fresh_stream_id("until-end-of-stream-forward");
    let events    = generate_events("until-end-of-stream-forward-test", 10);

    let _ = connection.write_events(stream_id.as_str())
                  .append_events(events)
                  .execute()
                  .wait()
                  .unwrap();

    let iter = connection.read_stream(stream_id.as_str())
                  .start_from_beginning()
                  .max_count(1)
                  .iterate_over()
                  .wait();

    let mut pos = 0;
    let mut idx = 0;

    for event in iter {
        let event = event.unwrap();
        let event = event.get_original_event().unwrap();
        let obj: HashMap<String, i64> = event.as_json().unwrap();
        let value = obj.get("event_index").unwrap();

        idx = *value;
        pos += 1;
    }

    assert_eq!(pos, 10);
    assert_eq!(idx, 10);
}

// Like `test_until_end_of_stream_forward` but backward.
fn test_iterate_over_backward(connection: &eventstore::Connection) {
    let stream_id = fresh_stream_id("until-end-of-stream-backward");
    let events    = generate_events("until-end-of-stream-backward-test", 10);

    let _ = connection.write_events(stream_id.as_str())
                  .append_events(events)
                  .execute()
                  .wait()
                  .unwrap();

    let iter = connection.read_stream(stream_id.as_str())
                  .start_from_end_of_stream()
                  .max_count(1)
                  .iterate_over()
                  .wait();

    let mut pos = 0;
    let mut idx = 0;

    for event in iter {
        let event = event.unwrap();
        let event = event.get_original_event().unwrap();
        let obj: HashMap<String, i64> = event.as_json().unwrap();
        let value = obj.get("event_index").unwrap();

        idx = *value;
        pos += 1;
    }

    assert_eq!(pos, 10);
    assert_eq!(idx, 1);
}

// We read $all system stream. We cannot write on $all stream. It's very
// unlikely for $all stream to be empty. From a personal note, I never saw that
// stream empty even right after booting the server.
fn test_read_all_stream(connection: &eventstore::Connection) {
    let result =
        connection.read_all()
              .max_count(10)
              .execute()
              .wait()
              .unwrap();

    debug!("Read $all events result {:?}", result);
}

// We write an event into a stream then delete that stream.
fn test_delete_stream(connection: &eventstore::Connection) {
    let stream_id = fresh_stream_id("delete");
    let event     = generate_event("delete-test");

    let _ = connection.write_events(stream_id.as_str())
                  .push_event(event)
                  .execute()
                  .wait()
                  .unwrap();

    let result = connection.delete_stream(stream_id.as_str())
                       .execute()
                       .wait()
                       .unwrap();

    debug!("Delete stream [{}] result: {:?}", stream_id, result);
}

// We create a volatile subscription on a stream then write events into that
// same stream. We check our subscription consumer internal state to see it
// has consumed all the expected events.
fn test_volatile_subscription(connection: &eventstore::Connection) {
    let stream_id = fresh_stream_id("volatile");
    let sub       = connection.subcribe_to_stream(stream_id.as_str()).execute();
    let events    = generate_events("volatile-test", 3);
    let confirmation = sub.confirmation();

    let handle = spawn(move || {
        sub.consume(TestSub { count: 0, max: 3 })
    });

    let _ = confirmation.wait();

    let _ = connection.write_events(stream_id)
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
fn test_catchup_subscription(connection: &eventstore::Connection) {
    let stream_id     = fresh_stream_id("catchup");
    let events_before = generate_events("catchup-test-before", 3);
    let events_after  = generate_events("catchup-test-after", 3);

    let _ = connection.write_events(stream_id.clone())
                  .append_events(events_before)
                  .execute()
                  .wait()
                  .unwrap();

    let sub = connection.subscribe_to_stream_from(stream_id.clone()).execute();

    let handle = spawn(move || {
        sub.consume(TestSub { count: 0, max: 6 })
    });

    let _ = connection.write_events(stream_id)
                  .append_events(events_after)
                  .execute()
                  .wait()
                  .unwrap();

    let test_sub = handle.join().unwrap();

    assert_eq!(test_sub.count, 6, "We are testing proper state after catchup subscription: got {} expected {}.", test_sub.count, 3);
}

// $all stream being a special system stream, we can not test as precisely as
// we did in `test_catchup_subscription`
fn test_catchup_all_subscription(connection: &eventstore::Connection) {
    let sub = connection.subscribe_to_all_from().execute();
    let tmp = sub.consume(TestSub { count: 0, max: 10 });

    assert_eq!(
        tmp.count,
        10,
        "We are testing proper state after $all catchup"
    );
}

// We test we can successfully create a persistent subscription.
fn test_create_persistent_subscription(connection: &eventstore::Connection) {
    let stream_id = fresh_stream_id("create_persistent_sub");
    let result = connection
        .create_persistent_subscription(stream_id, "a_group_name".to_string())
        .execute()
        .wait()
        .unwrap();

    assert_eq!(
        result,
        eventstore::PersistActionResult::Success,
        "We expect create a persistent subscription to succeed",
    );
}

// We test we can successfully update a persistent subscription.
fn test_update_persistent_subscription(connection: &eventstore::Connection) {
    let stream_id = fresh_stream_id("update_persistent_sub");
    let result = connection
        .create_persistent_subscription(stream_id.clone(), "a_group_name".to_string())
        .execute()
        .wait()
        .unwrap();

    assert_eq!(
        result,
        eventstore::PersistActionResult::Success,
        "We expect create a persistent subscription to succeed",
    );

    let mut setts = eventstore::PersistentSubscriptionSettings::default();

    setts.max_retry_count = 1000;

    let result = connection
        .update_persistent_subscription(stream_id, "a_group_name".to_string())
        .settings(setts)
        .execute()
        .wait()
        .unwrap();

    assert_eq!(
        result,
        eventstore::PersistActionResult::Success,
        "We expect updating a persistent subscription to succeed",
    );
}

// We test we can successfully delete a persistent subscription.
fn test_delete_persistent_subscription(connection: &eventstore::Connection) {
    let stream_id = fresh_stream_id("delete_persistent_sub");
    let result = connection
        .create_persistent_subscription(stream_id.clone(), "a_group_name".to_string())
        .execute()
        .wait()
        .unwrap();

    assert_eq!(
        result,
        eventstore::PersistActionResult::Success,
        "We expect create a persistent subscription to succeed",
    );

    let result = connection
        .delete_persistent_subscription(stream_id, "a_group_name".to_string())
        .execute()
        .wait()
        .unwrap();

    assert_eq!(
        result,
        eventstore::PersistActionResult::Success,
        "We expect deleting a persistent subscription to succeed",
    );
}

fn test_persistent_subscription(connection: &eventstore::Connection) {
    let stream_id = fresh_stream_id("persistent_subscription");
    let events    = generate_events("persistent_subscription_test", 5);

    let _ = connection
        .create_persistent_subscription(stream_id.clone(), "a_group_name".to_string())
        .execute()
        .wait()
        .unwrap();

    let _ = connection.write_events(stream_id.clone())
                  .append_events(events)
                  .execute()
                  .wait()
                  .unwrap();

    let sub = connection.connect_persistent_subscription(stream_id.clone(), "a_group_name".to_string())
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
    use std::env;

    env_logger::init();

    let host = env::var("EVENTSTORE_HOST").unwrap_or("127.0.0.1".to_string());
    let conn_str = format!("{}:1113", host);

    info!("Connection string: {}", conn_str);

    let connection = eventstore::Connection::builder()
        .with_default_user(eventstore::Credentials::new("admin", "changeit"))
        .single_node_connection(conn_str.parse().unwrap());

    test_write_events(&connection);
    test_read_event(&connection);
    test_write_and_read_stream_metadata(&connection);
    test_transaction(&connection);
    test_read_stream_events(&connection);
    test_iterate_over_forward(&connection);
    test_iterate_over_backward(&connection);
    test_read_all_stream(&connection);
    test_delete_stream(&connection);
    test_volatile_subscription(&connection);
    test_catchup_subscription(&connection);
    test_catchup_all_subscription(&connection);
    test_create_persistent_subscription(&connection);
    test_update_persistent_subscription(&connection);
    test_delete_persistent_subscription(&connection);
    test_persistent_subscription(&connection);

    connection.shutdown();
}
