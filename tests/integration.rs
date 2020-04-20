#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

fn fresh_stream_id(prefix: &str) -> String {
    let uuid = uuid::Uuid::new_v4();

    format!("{}-{}", prefix, uuid)
}

pub mod tcp {
    use super::fresh_stream_id;
    use eventstore::{Retry, Slice};
    use futures::channel::oneshot;
    use futures::SinkExt;
    use std::collections::HashMap;
    use std::error::Error;
    use std::net::ToSocketAddrs;
    use std::time::Duration;
    use tokio_test::block_on;

    fn generate_events(event_type: &str, cnt: usize) -> Vec<eventstore::EventData> {
        let mut events = Vec::with_capacity(cnt);

        for idx in 1..cnt + 1 {
            let payload = json!({
                "event_index": idx,
            });

            let data = eventstore::EventData::json(event_type, payload).unwrap();
            events.push(data);
        }

        events
    }

    fn generate_event(event_type: &str) -> eventstore::EventData {
        generate_events(event_type, 1)
            .pop()
            .expect("Can't be empty")
    }

    // We write an event into a stream.
    async fn test_write_events(connection: &eventstore::Connection) -> Result<(), Box<dyn Error>> {
        let stream_id = fresh_stream_id("write-events");
        let event = generate_event("write-events-test");

        let result = connection
            .write_events(stream_id)
            .push_event(event)
            .execute()
            .await?;

        debug!("Write response: {:?}", result);

        Ok(())
    }

    // We write an event into a stream then try to read it back.
    async fn test_read_event(connection: &eventstore::Connection) -> Result<(), Box<dyn Error>> {
        let stream_id = fresh_stream_id("read_event");
        let event = generate_event("read_event_test");
        let _ = connection
            .write_events(stream_id.as_str())
            .push_event(event)
            .execute()
            .await?;

        let result = connection
            .read_event(stream_id.as_str(), 0)
            .execute()
            .await?;

        debug!("Read response: {:?}", result);

        match result {
            eventstore::ReadEventStatus::Success(ref result) => {
                let event = result.event.get_original_event();
                let value: serde_json::Value = serde_json::from_slice(&event.data).unwrap();

                debug!("Payload as JSON {:?}", value);
            }

            _ => panic!("Something went wrong when reading stream {}", stream_id),
        }

        Ok(())
    }

    // We write metadata to a stream then try to read it back.
    async fn test_write_and_read_stream_metadata(
        connection: &eventstore::Connection,
    ) -> Result<(), Box<dyn Error>> {
        let stream_id = fresh_stream_id("metadata");
        let duration = Duration::from_secs(2 * 3_600) + Duration::from_millis(300);
        let metadata = eventstore::StreamMetadata::builder()
            .max_age(duration)
            .max_count(1000)
            .insert_custom_property("foo".to_owned(), "Bar!")
            .build();

        let result = connection
            .write_stream_metadata(stream_id.as_str(), metadata)
            .execute()
            .await?;

        debug!("Write stream metadata {:?}", result);

        let result = connection
            .read_stream_metadata(stream_id.as_str())
            .execute()
            .await?;

        debug!("Read stream metadata {:?}", result);

        match result {
            eventstore::StreamMetadataResult::Success(result) => {
                let read_max_age = result.metadata.max_age.unwrap();

                assert_eq!(read_max_age, duration);
            }

            _ => panic!(
                "Something went wrong when reading stream {} metadata",
                stream_id
            ),
        }

        Ok(())
    }

    // We write a stream using a transaction. The write will be only be taken into
    // account by the server after the `commit` call.
    async fn test_transaction(connection: &eventstore::Connection) -> Result<(), Box<dyn Error>> {
        let stream_id = fresh_stream_id("transaction");
        let event = generate_event("transaction_test");
        let transaction = connection
            .start_transaction(stream_id.as_str())
            .execute()
            .await?;

        transaction.write_single(event).await?;

        let result = transaction.commit().await?;

        debug!("Transaction commit result {:?}", result);

        Ok(())
    }

    // We read stream events by batch. We also test if we can properly read a
    // stream thoroughly.
    async fn test_read_stream_events(
        connection: &eventstore::Connection,
    ) -> Result<(), Box<dyn Error>> {
        let stream_id = fresh_stream_id("read-stream-events");
        let events = generate_events("read-stream-events-test", 10);

        let _ = connection
            .write_events(stream_id.as_str())
            .append_events(events)
            .execute()
            .await?;

        let mut pos = 0;
        let mut idx = 0;

        loop {
            let result = connection
                .read_stream(stream_id.as_str())
                .start_from(pos)
                .max_count(1)
                .execute()
                .await?;

            match result {
                eventstore::ReadStreamStatus::Success(slice) => match slice.events() {
                    eventstore::LocatedEvents::EndOfStream => {
                        break;
                    }

                    eventstore::LocatedEvents::Events { mut events, next } => {
                        let event = events.pop().unwrap();
                        let event = event.get_original_event();
                        let obj: HashMap<String, i64> = event.as_json().unwrap();
                        let value = obj.get("event_index").unwrap();

                        idx = *value;

                        match next {
                            Some(n) => pos = n,
                            None => {
                                break;
                            }
                        }
                    }
                },

                eventstore::ReadStreamStatus::Error(error) => {
                    panic!("ReadStream error: {:?}", error);
                }
            }
        }

        assert_eq!(pos, 9);
        assert_eq!(idx, 10);

        Ok(())
    }

    // Like `test_read_stream_events` but use `ReadStreamEvents::until_end_of_stream`
    async fn test_iterate_over_forward(
        connection: &eventstore::Connection,
    ) -> Result<(), Box<dyn Error>> {
        use futures::stream::TryStreamExt;

        let stream_id = fresh_stream_id("until-end-of-stream-forward");
        let events = generate_events("until-end-of-stream-forward-test", 10);

        let _ = connection
            .write_events(stream_id.as_str())
            .append_events(events)
            .execute()
            .await?;

        let mut iter = connection
            .read_stream(stream_id.as_str())
            .start_from_beginning()
            .max_count(1)
            .iterate_over();

        let mut pos = 0;
        let mut idx = 0;

        while let Some(event) = iter.try_next().await? {
            let event = event.get_original_event();
            let obj = event.as_json::<HashMap<String, i64>>()?;
            let value = obj.get("event_index").unwrap();

            idx = *value;
            pos += 1;
        }

        assert_eq!(pos, 10);
        assert_eq!(idx, 10);

        Ok(())
    }

    // Like `test_until_end_of_stream_forward` but backward.
    async fn test_iterate_over_backward(
        connection: &eventstore::Connection,
    ) -> Result<(), Box<dyn Error>> {
        use futures::stream::TryStreamExt;

        let stream_id = fresh_stream_id("until-end-of-stream-backward");
        let events = generate_events("until-end-of-stream-backward-test", 10);

        let _ = connection
            .write_events(stream_id.as_str())
            .append_events(events)
            .execute()
            .await?;

        let mut iter = connection
            .read_stream(stream_id.as_str())
            .start_from_end_of_stream()
            .max_count(1)
            .iterate_over();

        let mut pos = 0;
        let mut idx = 0;

        while let Some(event) = iter.try_next().await? {
            let event = event.get_original_event();
            let obj = event.as_json::<HashMap<String, i64>>()?;
            let value = obj.get("event_index").unwrap();

            idx = *value;
            pos += 1;
        }

        assert_eq!(pos, 10);
        assert_eq!(idx, 1);

        Ok(())
    }

    // We read $all system stream. We cannot write on $all stream. It's very
    // unlikely for $all stream to be empty. From a personal note, I never saw that
    // stream empty even right after booting the server.
    async fn test_read_all_stream(
        connection: &eventstore::Connection,
    ) -> Result<(), Box<dyn Error>> {
        let result = connection.read_all().max_count(10).execute().await?;

        debug!("Read $all events result {:?}", result);

        Ok(())
    }

    // We write an event into a stream then delete that stream.
    async fn test_delete_stream(connection: &eventstore::Connection) -> Result<(), Box<dyn Error>> {
        let stream_id = fresh_stream_id("delete");
        let event = generate_event("delete-test");

        let _ = connection
            .write_events(stream_id.as_str())
            .push_event(event)
            .execute()
            .await?;

        let result = connection
            .delete_stream(stream_id.as_str())
            .execute()
            .await?;

        debug!("Delete stream [{}] result: {:?}", stream_id, result);

        Ok(())
    }

    // We create a volatile subscription on a stream then write events into that
    // same stream. We check our subscription consumer internal state to see it
    // has consumed all the expected events.
    async fn test_volatile_subscription(
        connection: &eventstore::Connection,
    ) -> Result<(), Box<dyn Error>> {
        use futures::stream::StreamExt;

        let stream_id = fresh_stream_id("volatile");
        let mut sub = connection
            .subcribe_to_stream(stream_id.as_str())
            .execute_with_sub_events();
        let events = generate_events("volatile-test", 3);

        let (tx, recv) = oneshot::channel();

        // Used as concurrency control, we don't want to write events before the subscription
        // has been confirmed by the server.
        let (mut confirmation_tx, mut confirmation_recv) = futures::channel::mpsc::unbounded();

        tokio::spawn(async move {
            let mut count = 0usize;

            while let Some(evt) = sub.next().await {
                match evt {
                    eventstore::SubEvent::Confirmed => {
                        let _ = confirmation_tx.send(()).await;
                    }

                    eventstore::SubEvent::EventAppeared { .. } => {
                        count += 1;

                        if count == 3 {
                            break;
                        }
                    }

                    eventstore::SubEvent::Dropped => {
                        break;
                    }
                }
            }

            tx.send(count).unwrap();
        });

        let _ = confirmation_recv.next().await;

        let _ = connection
            .write_events(stream_id)
            .append_events(events)
            .execute()
            .await?;

        let count = recv.await?;

        assert_eq!(
            count, 3,
            "We are testing proper state after volatile subscription: got {} expected {}.",
            count, 3
        );

        Ok(())
    }

    // We write events into a stream. Then, we issue a catchup subscription. After,
    // we write another batch of events into the same stream. The goal is to make
    // sure we receive events written prior and after our subscription request.
    // To assess we received all the events we expected, we test our subscription
    // internal state value.
    async fn test_catchup_subscription(
        connection: &eventstore::Connection,
    ) -> Result<(), Box<dyn Error>> {
        use futures::stream::StreamExt;

        let stream_id = fresh_stream_id("catchup");
        let events_before = generate_events("catchup-test-before", 3);
        let events_after = generate_events("catchup-test-after", 3);

        let _ = connection
            .write_events(stream_id.clone())
            .append_events(events_before)
            .execute()
            .await?;

        let mut sub = connection
            .subscribe_to_stream_from(stream_id.clone())
            .execute_with_sub_events();

        let (tx, recv) = oneshot::channel();
        // Used as concurrency control, we don't want to write events before the subscription
        // has been confirmed by the server.
        let (mut confirmation_tx, mut confirmation_recv) = futures::channel::mpsc::unbounded();

        tokio::spawn(async move {
            let mut count = 0usize;

            while let Some(evt) = sub.next().await {
                match evt {
                    eventstore::SubEvent::Confirmed => {
                        let _ = confirmation_tx.send(()).await;
                    }

                    eventstore::SubEvent::EventAppeared { .. } => {
                        count += 1;

                        if count == 6 {
                            break;
                        }
                    }

                    eventstore::SubEvent::Dropped => {
                        break;
                    }
                }
            }

            tx.send(count).unwrap();
        });

        let _ = confirmation_recv.next().await;

        let _ = connection
            .write_events(stream_id)
            .append_events(events_after)
            .execute()
            .await?;

        let count = recv.await?;

        assert_eq!(
            count, 6,
            "We are testing proper state after catchup subscription: got {} expected {}.",
            count, 6
        );

        Ok(())
    }

    // $all stream being a special system stream, we can not test as precisely as
    // we did in `test_catchup_subscription`
    // h
    async fn test_catchup_all_subscription(connection: &eventstore::Connection) {
        use futures::stream::StreamExt;

        let mut sub = connection.subscribe_to_all_from().execute();
        let mut count = 0usize;

        while let Some(_) = sub.next().await {
            count += 1;

            if count == 10 {
                break;
            }
        }

        assert_eq!(count, 10, "We are testing proper state after $all catchup");
    }

    // We test we can successfully create a persistent subscription.
    async fn test_create_persistent_subscription(
        connection: &eventstore::Connection,
    ) -> Result<(), Box<dyn Error>> {
        let stream_id = fresh_stream_id("create_persistent_sub");
        let result = connection
            .create_persistent_subscription(stream_id, "a_group_name".to_string())
            .execute()
            .await?;

        assert_eq!(
            result,
            eventstore::PersistActionResult::Success,
            "We expect create a persistent subscription to succeed",
        );

        Ok(())
    }

    // We test we can successfully update a persistent subscription.
    async fn test_update_persistent_subscription(
        connection: &eventstore::Connection,
    ) -> Result<(), Box<dyn Error>> {
        let stream_id = fresh_stream_id("update_persistent_sub");
        let result = connection
            .create_persistent_subscription(stream_id.clone(), "a_group_name".to_string())
            .execute()
            .await?;

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
            .await?;

        assert_eq!(
            result,
            eventstore::PersistActionResult::Success,
            "We expect updating a persistent subscription to succeed",
        );

        Ok(())
    }

    // We test we can successfully delete a persistent subscription.
    async fn test_delete_persistent_subscription(
        connection: &eventstore::Connection,
    ) -> Result<(), Box<dyn Error>> {
        let stream_id = fresh_stream_id("delete_persistent_sub");
        let result = connection
            .create_persistent_subscription(stream_id.clone(), "a_group_name".to_string())
            .execute()
            .await?;

        assert_eq!(
            result,
            eventstore::PersistActionResult::Success,
            "We expect create a persistent subscription to succeed",
        );

        let result = connection
            .delete_persistent_subscription(stream_id, "a_group_name".to_string())
            .execute()
            .await?;

        assert_eq!(
            result,
            eventstore::PersistActionResult::Success,
            "We expect deleting a persistent subscription to succeed",
        );

        Ok(())
    }

    async fn test_persistent_subscription(
        connection: &eventstore::Connection,
    ) -> Result<(), Box<dyn Error>> {
        let stream_id = fresh_stream_id("persistent_subscription");
        let events = generate_events("persistent_subscription_test", 5);

        let _ = connection
            .create_persistent_subscription(stream_id.clone(), "a_group_name".to_string())
            .execute()
            .await?;

        let _ = connection
            .write_events(stream_id.clone())
            .append_events(events)
            .execute()
            .await?;

        let (mut sub_read, mut sub_write) = connection
            .connect_persistent_subscription(stream_id.clone(), "a_group_name".to_string())
            .execute();

        let mut count = 0usize;

        while let Some(event) = sub_read.read_next().await {
            count += 1;
            sub_write.ack_event(event).await;

            if count == 5 {
                break;
            }
        }

        assert_eq!(
            count, 5,
            "We are testing proper state after persistent subscription: got {} expected {}",
            count, 5
        );

        Ok(())
    }

    #[test]
    fn all_round_operation_test() {
        block_on(async {
            use std::env;

            let _ = env_logger::try_init();

            let host = env::var("EVENTSTORE_HOST").unwrap_or("127.0.0.1".to_string());
            let conn_str = format!("{}:1113", host);

            info!("Connection string: {}", conn_str);

            let endpoint = conn_str.to_socket_addrs().unwrap().next().unwrap();

            let connection = eventstore::Connection::builder()
                .with_default_user(eventstore::Credentials::new("admin", "changeit"))
                .single_node_connection(endpoint)
                .await;

            test_write_events(&connection).await?;
            test_read_event(&connection).await?;
            test_write_and_read_stream_metadata(&connection).await?;
            test_transaction(&connection).await?;
            test_read_stream_events(&connection).await?;
            test_iterate_over_forward(&connection).await?;
            test_iterate_over_backward(&connection).await?;
            test_read_all_stream(&connection).await?;
            test_delete_stream(&connection).await?;
            test_volatile_subscription(&connection).await?;
            test_catchup_subscription(&connection).await?;
            test_catchup_all_subscription(&connection).await;
            test_create_persistent_subscription(&connection).await?;
            test_update_persistent_subscription(&connection).await?;
            test_delete_persistent_subscription(&connection).await?;
            test_persistent_subscription(&connection).await?;

            connection.shutdown().await;

            Ok(()) as Result<(), Box<dyn Error>>
        })
        .unwrap();
    }

    #[test]
    fn offline_server_test() {
        block_on(async {
            let host = std::env::var("EVENTSTORE_HOST").unwrap_or("127.0.0.1".to_string());
            let conn_str = format!("{}:1114", host); // Be sure your server doesn't run on port 1114

            info!("Connection string: {}", conn_str);

            let endpoint = conn_str.to_socket_addrs().unwrap().next().unwrap();

            let connection = eventstore::Connection::builder()
                .connection_timeout(Duration::from_secs(0))
                .connection_retry(Retry::Only(0))
                .with_default_user(eventstore::Credentials::new("admin", "changeit"))
                .single_node_connection(endpoint)
                .await;

            let stream_id = fresh_stream_id("stream-id");
            let group_name = "group-name";

            assert!(connection.read_all().execute().await.is_err());
            assert!(connection.read_all().execute().await.is_err());
            assert!(connection
                .read_event(stream_id.as_str(), 0)
                .execute()
                .await
                .is_err());
            assert!(connection
                .read_stream(stream_id.as_str())
                .execute()
                .await
                .is_err());
            assert!(connection
                .create_persistent_subscription(stream_id.as_str(), group_name)
                .execute()
                .await
                .is_err());
        });
    }
}
#[cfg(feature = "es6")]
pub mod es6 {
    use super::fresh_stream_id;
    use eventstore::es6;
    use futures::channel::oneshot;
    use futures::stream::{self, TryStreamExt};
    use std::collections::HashMap;
    use std::error::Error;
    use tokio_test::block_on;

    fn generate_events(event_type: String, cnt: usize) -> Vec<es6::types::EventData> {
        let mut events = Vec::with_capacity(cnt);

        for idx in 1..cnt + 1 {
            let payload = json!({
                "event_index": idx,
            });

            let data =
                eventstore::es6::types::EventData::json(event_type.clone(), payload).unwrap();
            events.push(data);
        }

        events
    }

    async fn test_write_events(
        connection: &es6::connection::Connection,
    ) -> Result<(), Box<dyn Error>> {
        let stream_id = fresh_stream_id("write_events");
        let events = generate_events("es6-write-events-test".to_string(), 3);

        let result = connection
            .write_events(stream_id)
            .send(stream::iter(events))
            .await?;

        debug!("Write response: {:?}", result);

        Ok(())
    }

    // We read stream events by batch. We also test if we can properly read a
    // stream thoroughly.
    async fn test_read_stream_events(
        connection: &es6::connection::Connection,
    ) -> Result<(), Box<dyn Error>> {
        let stream_id = fresh_stream_id("read_stream_events");
        let events = generate_events("es6-read-stream-events-test".to_string(), 10);

        let _ = connection
            .write_events(stream_id.clone())
            .send(stream::iter(events))
            .await?;

        let mut pos = 0usize;
        let mut idx = 0i64;

        let mut stream = connection
            .read_stream(stream_id)
            .start_from_beginning()
            .execute(10)
            .await?;

        while let Some(event) = stream.try_next().await? {
            let event = event.get_original_event();
            let obj: HashMap<String, i64> = event.as_json().unwrap();
            let value = obj.get("event_index").unwrap();

            idx = *value;
            pos += 1;
        }

        assert_eq!(pos, 10);
        assert_eq!(idx, 10);

        Ok(())
    }

    // We write an event into a stream then delete that stream.
    async fn test_delete_stream(
        connection: &es6::connection::Connection,
    ) -> Result<(), Box<dyn Error>> {
        let stream_id = fresh_stream_id("delete");
        let events = generate_events("delete-test".to_string(), 1);

        let _ = connection
            .write_events(stream_id.clone())
            .send(stream::iter(events))
            .await?;

        let result = connection
            .delete_stream(stream_id.clone())
            .execute()
            .await?;

        debug!("Delete stream [{}] result: {:?}", stream_id, result);

        Ok(())
    }

    // We write events into a stream. Then, we issue a catchup subscription. After,
    // we write another batch of events into the same stream. The goal is to make
    // sure we receive events written prior and after our subscription request.
    // To assess we received all the events we expected, we test our subscription
    // internal state value.
    async fn test_subscription(
        connection: &es6::connection::Connection,
    ) -> Result<(), Box<dyn Error>> {
        let stream_id = fresh_stream_id("catchup");
        let events_before = generate_events("catchup-test-before".to_string(), 3);
        let events_after = generate_events("catchup-test-after".to_string(), 3);

        let _ = connection
            .write_events(stream_id.clone())
            .send(stream::iter(events_before))
            .await?;

        let mut sub = connection
            .subscribe_to_stream_from(stream_id.clone())
            .execute()
            .await?;

        let (tx, recv) = oneshot::channel();

        tokio::spawn(async move {
            let mut count = 0usize;
            let max = 6usize;

            while let Some(_) = sub.try_next().await? {
                count += 1;

                if count == max {
                    break;
                }
            }

            tx.send(count).unwrap();
            Ok(()) as Result<(), tonic::Status>
        });

        let _ = connection
            .write_events(stream_id)
            .send(stream::iter(events_after))
            .await?;

        let test_count = recv.await?;

        assert_eq!(
            test_count, 6,
            "We are testing proper state after catchup subscription: got {} expected {}.",
            test_count, 6
        );

        Ok(())
    }

    async fn test_create_persistent_subscription(
        connection: &es6::connection::Connection,
    ) -> Result<(), Box<dyn Error>> {
        let stream_id = fresh_stream_id("create_persistent_sub");

        connection
            .create_persistent_subscription(stream_id, "a_group_name".to_string())
            .execute()
            .await?;

        Ok(())
    }

    // We test we can successfully update a persistent subscription.
    async fn test_update_persistent_subscription(
        connection: &es6::connection::Connection,
    ) -> Result<(), Box<dyn Error>> {
        let stream_id = fresh_stream_id("update_persistent_sub");

        connection
            .create_persistent_subscription(stream_id.clone(), "a_group_name".to_string())
            .execute()
            .await?;

        let mut setts = es6::types::PersistentSubscriptionSettings::default();

        setts.max_retry_count = 1000;

        connection
            .update_persistent_subscription(stream_id, "a_group_name".to_string())
            .settings(setts)
            .execute()
            .await?;

        Ok(())
    }

    // We test we can successfully delete a persistent subscription.
    async fn test_delete_persistent_subscription(
        connection: &es6::connection::Connection,
    ) -> Result<(), Box<dyn Error>> {
        let stream_id = fresh_stream_id("delete_persistent_sub");
        connection
            .create_persistent_subscription(stream_id.clone(), "a_group_name".to_string())
            .execute()
            .await?;

        connection
            .delete_persistent_subscription(stream_id, "a_group_name".to_string())
            .execute()
            .await?;

        Ok(())
    }

    async fn test_persistent_subscription(
        connection: &es6::connection::Connection,
    ) -> Result<(), Box<dyn Error>> {
        let stream_id = fresh_stream_id("persistent_subscription");
        let events = generate_events("es6-persistent-subscription-test".to_string(), 5);

        connection
            .create_persistent_subscription(stream_id.clone(), "a_group_name".to_string())
            .execute()
            .await?;

        let _ = connection
            .write_events(stream_id.clone())
            .send(stream::iter(events))
            .await?;

        let (mut read, mut write) = connection
            .connect_persistent_subscription(stream_id.clone(), "a_group_name".to_string())
            .execute()
            .await?;

        let mut count = 0usize;
        let max = 5usize;

        while let Some(event) = read.try_next().await? {
            write.ack_event(event).await?;

            count += 1;

            if count == max {
                break;
            }
        }

        assert_eq!(
            count, 5,
            "We are testing proper state after persistent subscription: got {} expected {}",
            count, 5
        );

        Ok(())
    }

    #[test]
    #[should_panic]
    // Expect to panic because the lastest ES6 preview isn't updated on dockerhub yet.
    fn es6_preview_test() {
        block_on(async {
            use std::env;

            let _ = env_logger::try_init();

            let host = env::var("EVENTSTORE_HOST").unwrap_or("localhost".to_string());
            let uri = format!("https://{}:2113/", host).parse()?;

            let connection = eventstore::es6::connection::Connection::builder()
                .with_default_user(eventstore::Credentials::new("admin", "changeit"))
                .disable_server_certificate_validation()
                .single_node_connection(uri)
                .await?;

            test_write_events(&connection).await?;
            test_read_stream_events(&connection).await?;
            test_delete_stream(&connection).await?;
            test_subscription(&connection).await?;
            test_create_persistent_subscription(&connection).await?;
            test_update_persistent_subscription(&connection).await?;
            test_delete_persistent_subscription(&connection).await?;
            test_persistent_subscription(&connection).await?;

            Ok(()) as Result<(), Box<dyn std::error::Error>>
        })
        .unwrap();
    }
}
