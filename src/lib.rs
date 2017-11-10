extern crate core;
extern crate bytes;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate uuid;

use core::option::Option;
use core::result::Result;
use std::io::{ Error, Cursor };

use bytes::{ Buf, BytesMut, LittleEndian };
use bytes::buf::BufMut;
use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;
use tokio_io::codec::{ Encoder, Decoder };
use uuid::Uuid;

pub struct Pkg {
    pub cmd:         u8,
    pub correlation: Uuid,
}

impl Pkg {
    fn new(cmd: u8, correlation: Uuid) -> Pkg {
        Pkg {
            cmd:         cmd,
            correlation: correlation,
        }
    }

    fn size(&self) -> u32 {
        18
    }
}

pub struct PkgCodec;

impl Encoder for PkgCodec {
    type Item  = Pkg;
    type Error = Error;

    fn encode(&mut self, item: Pkg, dest: &mut BytesMut) -> Result<(), Error> {
        dest.put_u32::<LittleEndian>(item.size());
        dest.put_u8(item.cmd);
        dest.put_u8(0); // Package credential flag.
        dest.put_slice(item.correlation.as_bytes());

        Result::Ok(())
    }
}

impl Decoder for PkgCodec {
    type Item  = Pkg;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Pkg>, Error> {
        // Checks if the frame size has been sent.
        if src.len() < 4 {
            Result::Ok(Option::None)
        } else {
            let mut frame_cursor = Cursor::new(src.split_to(3));
            let frame            = frame_cursor.get_u32::<LittleEndian>() as usize;

            // Checks if all the message payload has been sent.
            if src.len() < frame {
                Result::Ok(Option::None)
            } else {
                let cmd         = src[0];
                let correlation = Uuid::from_bytes(&src[2..18]).unwrap();

                Result::Ok(Option::Some(Pkg::new(cmd, correlation)))
            }
        }
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
