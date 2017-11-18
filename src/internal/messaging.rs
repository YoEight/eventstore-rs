use uuid::Uuid;

use internal::package::Pkg;

pub enum Msg {
    Start,
    Shutdown,
    Established(Uuid),
    Arrived(Pkg),
}
