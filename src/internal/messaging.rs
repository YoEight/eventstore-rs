use std::io::Error;

use uuid::Uuid;

use discovery::Endpoint;
use internal::operations;
use internal::package::Pkg;

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
}

impl Msg {
    pub(crate) fn new_op(op: operations::OperationWrapper) -> Msg {
        Msg::NewOp(op)
    }
}
