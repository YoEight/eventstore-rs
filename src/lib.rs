extern crate core;
extern crate bytes;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate uuid;

use core::option::Option;
use core::result::Result;
use std::io::{ Error, ErrorKind };

use bytes::BytesMut;
use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;
use tokio_io::codec::{ Encoder, Decoder };
use uuid::Uuid;

pub struct Pkg {
    pub cmd:         u8,
    pub correlation: Uuid,
}

pub struct PkgCodec;

impl Encoder for PkgCodec {
    type Item  = Pkg;
    type Error = Error;

    fn encode(&mut self, _: Pkg, _: &mut BytesMut) -> Result<(), Error> {
        let e = Error::new(ErrorKind::Other, "not implemented yet");

        Result::Err(e)
    }
}

impl Decoder for PkgCodec {
    type Item  = Pkg;
    type Error = Error;

    fn decode(&mut self, _: &mut BytesMut) -> Result<Option<Pkg>, Error> {
        let e = Error::new(ErrorKind::Other, "not implemented yet");

        Result::Err(e)
    }
}

pub fn workbench() {
    let core   = Core::new().unwrap();
    let handle = core.handle();
    let host   = "127.0.0.1".parse().unwrap();

    let _ = TcpStream::connect(&host, &handle);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut core = Core::new().unwrap();
        let handle   = core.handle();
        let host = "127.0.0.1".parse();

        let _ = TcpStream::connect(host, handle);

        assert_eq!(2 + 2, 4);

    }
}
