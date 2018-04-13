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
    NewOp(operations::Op),
}
