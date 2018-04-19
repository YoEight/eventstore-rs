use std::io::Error;

use uuid::Uuid;

use discovery::Endpoint;
use internal::operations;
use internal::package::Pkg;

pub enum Msg {
    Start,
    Shutdown,
    Tick,
    Establish(Endpoint),
    Established(Uuid),
    Arrived(Pkg),
    ConnectionClosed(Uuid, Error),
    NewOp(operations::Exchange),
    Send(Pkg),
}

impl Msg {
    pub fn new_op<O>(op: O) -> Msg
        where O: operations::Operation + Sync + Send + 'static
    {
        Msg::NewOp(Box::new(op))
    }
}
