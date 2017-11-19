use std::io::{ Cursor, Read, Error, ErrorKind, Result };
use std::net::TcpStream;
use std::result::Result::{ Ok };

use bytes::{ Buf, BytesMut, LittleEndian };
use bytes::buf::BufMut;
use uuid::{ Uuid, ParseError };

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

    pub fn from_stream(stream: &mut TcpStream) -> Result<Pkg> {
        let mut frame: [u8; 4] = [0; 4];

        stream.read_exact(&mut frame)?;

        let mut frame_cursor = Cursor::new(frame);
        let     frame_size   = frame_cursor.get_u32::<LittleEndian>() as usize;
        let mut pkg_buf      = vec![0; frame_size];

        stream.read_exact(&mut pkg_buf)?;

        fn to_error(err: ParseError) -> Error {
            Error::new(ErrorKind::Other, format!("ParseError {}", err))
        }

        let cmd         = pkg_buf[0];
        let correlation = Uuid::from_bytes(&pkg_buf[2..18]).map_err(|e| to_error(e))?;

        Ok(Pkg::new(cmd, correlation))
    }
}
