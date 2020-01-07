use std::fmt;
use std::io::Error;

use uuid::Uuid;

use crate::internal::package::Pkg;
use crate::types::{Endpoint, OperationError};
use futures::channel::mpsc;

#[derive(Debug)]
pub enum OpMsg {
    Recv(Pkg),
    Failed(OperationError),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lifetime<A> {
    OneTime(A),
    KeepAlive(A),
}

impl<A> Lifetime<A> {
    pub fn is_keep_alive(&self) -> bool {
        if let Lifetime::KeepAlive(_) = self {
            return true;
        }

        false
    }

    pub fn inner(self) -> A {
        match self {
            Lifetime::KeepAlive(a) => a,
            Lifetime::OneTime(a) => a,
        }
    }
}

pub type Mailbox = mpsc::Sender<OpMsg>;

pub enum Msg {
    Start,
    Shutdown,
    Tick,
    Establish(Endpoint),
    Established(Uuid),
    Arrived(Pkg),
    ConnectionClosed(Uuid, Error),
    Transmit(Lifetime<Pkg>, Mailbox),
    Send(Pkg),
    Marker, // Use as checkpoint detection.
}

impl fmt::Debug for Msg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Msg::*;

        match self {
            Start => writeln!(f, "Start"),
            Shutdown => writeln!(f, "Shutdown"),
            Tick => writeln!(f, "Tick"),
            Establish(ept) => writeln!(f, "Establish({:?})", ept),
            Established(id) => writeln!(f, "Established({:?})", id),
            Arrived(pkg) => writeln!(f, "Arrived({:?})", pkg),
            ConnectionClosed(id, e) => writeln!(f, "ConnectionClosed({:?}, {:?})", id, e),
            Transmit(pkg, _) => writeln!(f, "Transmit({:?})", pkg),
            Send(pkg) => writeln!(f, "Send({:?})", pkg),
            Marker => writeln!(f, "Marker"),
        }
    }
}
