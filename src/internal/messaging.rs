use uuid::Uuid;

use internal::package::Pkg;

pub enum Msg {
    Start,
    Shutdown,
    Tick,
    Established(Uuid),
    Arrived(Pkg),
}
