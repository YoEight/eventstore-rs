/// Provides a TCP client for [GetEventStore] datatbase.

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

mod connection;
mod discovery;
#[cfg(feature = "es6")]
pub mod es6;
mod internal;
pub mod types;

pub use connection::{Connection, ConnectionBuilder};

pub use internal::commands;

pub use types::*;
