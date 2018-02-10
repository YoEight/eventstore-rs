use std::io::{ Cursor, Read, Error, ErrorKind, Result };
use std::net::TcpStream;
use std::result::Result::{ Ok };

use bytes::{ Buf, BytesMut, LittleEndian };
use bytes::buf::BufMut;
use protobuf::Message;
use uuid::{ Uuid, ParseError };

use internal::command::Cmd;
use internal::messages;

pub struct Pkg {
    pub cmd:         Cmd,
    pub correlation: Uuid,
    pub payload:     Vec<u8>,
}

static CLIENT_VERSION: i32 = 1;

impl Pkg {
    pub fn new(cmd: Cmd, correlation: Uuid) -> Pkg {
        Pkg {
            cmd:         cmd,
            correlation: correlation,
            payload:     Vec::new(),
        }
    }

    pub fn set_payload(&mut self, payload: Vec<u8>) {
        self.payload = payload;
    }

    pub fn size(&self) -> usize {
        18 + self.payload.len()
    }

    pub fn heartbeat_request() -> Pkg {
        Pkg::new(Cmd::HeartbeatRequest, Uuid::new_v4())
    }

    pub fn identify_client(name_opt: &Option<String>) -> Pkg {
        let     corr_id = Uuid::new_v4();
        let mut pkg     = Pkg::new(Cmd::IdentifyClient, corr_id);
        let mut msg     = messages::IdentifyClient::new();
        let     name    = match *name_opt {
            Some(ref name) => name.clone(),
            None           => format!("ES-{}", Uuid::new_v4()),
        };

        msg.set_connection_name(name);
        msg.set_version(CLIENT_VERSION);
        msg.write_to_vec(&mut pkg.payload);

        pkg
    }

    // Copies the Pkg except its payload.
    pub fn copy_headers_only(&self) -> Pkg {
        Pkg {
            cmd:         self.cmd,
            correlation: self.correlation,
            payload:     Vec::new(),
        }
    }

    pub fn to_bytes(&self) -> BytesMut {
        let     capacity = 4 + self.size(); // frame size + package size.
        let mut bytes    = BytesMut::with_capacity(capacity);

        bytes.put_u32::<LittleEndian>(self.size() as u32);
        bytes.put_u8(self.cmd.to_u8());
        bytes.put_u8(0); // Package credential flag.
        bytes.put_slice(self.correlation.as_bytes());
        bytes.put_slice(self.payload.as_slice());

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

        let     cmd         = Cmd::from_u8(pkg_buf[0]);
        let     correlation = Uuid::from_bytes(&pkg_buf[2..18]).map_err(|e| to_error(e))?;
        let mut pkg         = Pkg::new(cmd, correlation);

        if frame_size > 18 {
            let payload = &pkg_buf[18..frame_size];

            pkg.set_payload(payload.to_vec());
        }

        Ok(pkg)
    }
}
