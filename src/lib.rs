/// Provides a TCP client for [GetEventStore] datatbase.

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

mod discovery;
mod internal;
mod connection;
pub mod types;

pub use connection::{
    Connection,
    ConnectionBuilder,
};

pub use internal::{
    commands,
    operations::OperationError
};

pub use types::*;
