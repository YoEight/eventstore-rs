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

pub struct StaticDiscovery {
    addr: SocketAddrV4,
}

impl Discovery for StaticDiscovery {
    fn discover(&mut self, _: Option<&Endpoint>) -> Endpoint {
        Endpoint {
            addr: self.addr,
        }
    }
}

impl StaticDiscovery {
    pub fn new(addr: SocketAddrV4) -> StaticDiscovery {
        StaticDiscovery {
            addr: addr,
        }
    }
}

pub trait Discovery {
    fn discover(&mut self, last: Option<&Endpoint>) -> Endpoint;
}
