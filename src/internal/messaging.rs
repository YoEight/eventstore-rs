use std::io::Error;
use std::fmt;

use uuid::Uuid;

use crate::internal::operations;
use crate::internal::package::Pkg;
use crate::types::Endpoint;

pub(crate) enum Msg {
    Start,
    Shutdown,
    Tick,
    Establish(Endpoint),
    Established(Uuid),
    Arrived(Pkg),
    ConnectionClosed(Uuid, Error),
    NewOp(operations::OperationWrapper),
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
            NewOp(op) => writeln!(f, "NewOp({:?})", op.id),
            Send(pkg) => writeln!(f, "Send({:?})", pkg),
            Marker => writeln!(f, "Marker"),
        }
    }
}

impl Msg {
    pub(crate) fn new_op(op: operations::OperationWrapper) -> Msg {
        Msg::NewOp(op)
    }
}
