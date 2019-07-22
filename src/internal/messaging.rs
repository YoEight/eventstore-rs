use std::io::Error;

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

impl Msg {
    pub(crate) fn new_op(op: operations::OperationWrapper) -> Msg {
        Msg::NewOp(op)
    }
}
