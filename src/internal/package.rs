use std::io::{ Cursor, Read };
use std::net::TcpStream;

use bytes::{ Buf, BytesMut, LittleEndian };
use bytes::buf::BufMut;
use uuid::Uuid;

pub struct Pkg {
    pub cmd:         u8,
    pub correlation: Uuid,
}

impl Pkg {
    pub fn new(cmd: u8, correlation: Uuid) -> Pkg {
        Pkg {
            cmd:         cmd,
            correlation: correlation,
        }
    }

    pub fn size(&self) -> u32 {
        18
    }

    // Copies the Pkg except its payload.
    pub fn copy_headers_only(&self) -> Pkg {
        Pkg {
            cmd:         self.cmd,
            correlation: self.correlation,
        }
    }

    pub fn to_bytes(&self) -> BytesMut {
        // FIXME - Use with_capacity instead.
        let mut bytes = BytesMut::new();

        bytes.put_u32::<LittleEndian>(self.size());
        bytes.put_u8(self.cmd);
        bytes.put_u8(0); // Package credential flag.
        bytes.put_slice(self.correlation.as_bytes());

        bytes
    }

    pub fn from_stream(stream: &mut TcpStream) -> Pkg {
        let mut frame: [u8; 4] = [0; 4];

        stream.read_exact(&mut frame).unwrap();

        let mut frame_cursor = Cursor::new(frame);
        let     frame_size   = frame_cursor.get_u32::<LittleEndian>() as usize;
        let mut pkg_buf      = vec![0; frame_size];

        stream.read_exact(&mut pkg_buf).unwrap();

        let cmd         = pkg_buf[0];
        let correlation = Uuid::from_bytes(&pkg_buf[2..18]).unwrap();

        Pkg::new(cmd, correlation)
    }
}
