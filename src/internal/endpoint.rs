use std::net::SocketAddrV4;

pub struct Endpoint {
    pub addr: SocketAddrV4,
}

impl Endpoint {
    pub fn from_addr(addr: SocketAddrV4) -> Endpoint {
        Endpoint {
            addr: addr,
        }
    }
}
