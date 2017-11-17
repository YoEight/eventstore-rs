use uuid::Uuid;

pub enum Msg {
    Start,
    Shutdown,
    Established(Uuid),
}
