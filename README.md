# eventstore-rs
Rust GetEventStore TCP Client.

[Talk and exchange ideas in our dedicated Discord Server]

## State of implemented features

- [x] Can connect to GetEventStore 4.* servers.
- [x] Connection health tracking.
- [x] Operation timeout detection and retry.
- [x] Write events.
- [x] Read events (including `$all` stream).
- [x] Read/Write stream metadata.
- [x] Transactions.
- [x] Delete stream.
- [x] Volatile Subscriptions.
- [x] Catchup Subscriptions.
- [x] Persistent Subscriptions.
- [ ] Support connection to server clusters.
- [ ] Support SSL connection.

# Example

```rust
extern crate eventstore;
extern crate futures;
#[macro_use]
extern crate serde_json;

use eventstore::{ Connection, EventData };
use futures::Future;

fn main() {
    let connection = Connection::builder()
        .start("127.0.0.1:1113")
        .unwrap();

    // It is not mandatory to use JSON as a data format however GetEventStore
    // provides great additional values if you do so.
    let payload = json!({
        "is_rust_a_nice_language": true,
    });

    let event = EventData::json("language-poll", payload);

    // All the operations are asynchronous but for the sake of this example
    // we decide to wait until the server sends a response.
    let result = connection
        .write_events("language-stream")
        .push_event(event)
        .execute()
        .wait()
        .unwrap();

    // Do something productive with the result.
}
```

## Notes

That library was tested on Linux and OSX.

Contributions and bug reports are welcome!

MIT License

-Yorick Laupa

[GetEventStore]: https://eventstore.org/
[Talk and exchange ideas in our dedicated Discord Server]: https://discord.gg/x7q37jJ

