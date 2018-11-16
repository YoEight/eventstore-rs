use std::net::SocketAddr;

pub struct Endpoint {
    pub addr: SocketAddr,
}

impl Endpoint {
    pub fn from_addr(addr: SocketAddr) -> Endpoint {
        Endpoint {
            addr,
        }
    }
}

pub struct StaticDiscovery {
    addr: SocketAddr,
}

impl Discovery for StaticDiscovery {
    fn discover(&mut self, _: Option<&Endpoint>) -> Endpoint {
        Endpoint {
            addr: self.addr,
        }
    }
}

impl StaticDiscovery {
    pub fn new(addr: SocketAddr) -> StaticDiscovery {
        StaticDiscovery {
            addr,
        }
    }
}

pub trait Discovery {
    fn discover(&mut self, last: Option<&Endpoint>) -> Endpoint;
}
