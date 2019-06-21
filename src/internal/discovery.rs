use std::net::SocketAddr;
use std::io;
use futures::Future;
use futures::future::{ self, FutureResult };

pub struct Endpoint {
    pub addr: SocketAddr,
}

pub struct StaticDiscovery {
    addr: SocketAddr,
}

impl Discovery for StaticDiscovery {
    type Fut = FutureResult<Endpoint, io::Error>;

    fn discover(&mut self, _: Option<&Endpoint>) -> Self::Fut {
        let endpoint = Endpoint {
            addr: self.addr,
        };

        future::ok(endpoint)
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
    type Fut: Future<Item=Endpoint, Error=io::Error> + Send + 'static;

    fn discover(&mut self, last: Option<&Endpoint>) -> Self::Fut;
}
