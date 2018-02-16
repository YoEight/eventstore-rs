use std::io::Error;

use uuid::Uuid;

use internal::endpoint::Endpoint;
use internal::package::Pkg;

pub enum Msg {
    Start,
    Shutdown,
    Tick,
    Establish(Endpoint),
    Established(Uuid),
    Arrived(Pkg),
    ConnectionClosed(Uuid, Error),
}
