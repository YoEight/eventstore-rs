/// Provides a TCP client for [GetEventStore] datatbase.

extern crate core;
extern crate bytes;
extern crate uuid;
extern crate timer;
extern crate time;
extern crate futures;
extern crate protobuf;
extern crate tokio;

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
