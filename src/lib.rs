//! Provides a TCP client for [GetEventStore] datatbase.
//!
//! # Example
//! ```rust
//! extern crate eventstore;
//! extern crate futures;
//! #[macro_use]
//! extern crate serde_json;
//!
//! use eventstore::Client;
//! use eventstore::types::{ Settings, EventData };
//! use futures::future::Future;
//!
//! fn main() {
//!     // No connection has started yet.
//!     let client = Client::new(
//!         Settings::default(),
//!         "127.0.0.1:1113".parse().unwrap(),
//!     );
//!
//!     // Now the asynchronous connection procedure will start.
//!     client.start();
//!
//!     // It is not mandatory to use JSON as a data format however GetEventStore
//!     // provides great additional values if you do so.
//!     let payload = json!({
//!         "is_rust_a_nice_language": true,
//!     });
//!
//!     let event = EventData::json("language-poll", payload);
//!
//!     // All the operations are asynchronous but for the sake of this example
//!     // we decide to wait until the server sends a response.
//!     let result = client
//!         .write_events("language-stream")
//!         .push_event(event)
//!         .execute()
//!         .wait()
//!         .unwrap();
//!
//!     // Do something productive with the result.
//! }
//! ```
//! [GetEventStore]: https://eventstore.org/

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
extern crate serde_json;

#[macro_use]
extern crate log;

mod internal;
mod client;

pub use client::{
    Client,
    Handle,
};
pub use internal::{ commands };
pub mod types;
