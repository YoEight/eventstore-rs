use std::io::{ Cursor, Read, Error, ErrorKind, Result };
use std::net::TcpStream;
use std::result::Result::{ Ok };
use std::string::FromUtf8Error;

use bytes::{ Buf, BytesMut, LittleEndian };
use bytes::buf::BufMut;
use protobuf::Message;
use uuid::{ Uuid, ParseError };

use internal::command::Cmd;
use internal::messages;
use internal::types::Credentials;

pub struct Pkg {
    pub cmd:         Cmd,
    pub creds_opt:   Option<Credentials>,
    pub correlation: Uuid,
    pub payload:     Vec<u8>,
}

static CLIENT_VERSION: i32 = 1;

impl Pkg {
    pub fn new(cmd: Cmd, correlation: Uuid) -> Pkg {
        Pkg {
            cmd:         cmd,
            creds_opt:   None,
            correlation: correlation,
            payload:     Vec::new(),
        }
    }

    pub fn set_payload(&mut self, payload: Vec<u8>) {
        self.payload = payload;
    }

    pub fn set_credentials(&mut self, creds: Credentials) {
        self.creds_opt = Some(creds);
    }

    pub fn size(&self) -> usize {
        let creds_size = {
            match self.creds_opt {
                Some(ref creds) => creds.login.len() + creds.password.len() + 2, // Including 2 length bytes.
                None            => 0,
            }
        };

        18 + self.payload.len() + creds_size
    }

    pub fn heartbeat_request() -> Pkg {
        Pkg::new(Cmd::HeartbeatRequest, Uuid::new_v4())
    }

    pub fn authenticate(creds: Credentials) -> Pkg {
        let corr_id = Uuid::new_v4();
        let mut pkg = Pkg::new(Cmd::Authenticate, corr_id);

        pkg.set_credentials(creds);

        pkg
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
            creds_opt:   None,
        }
    }

    pub fn to_bytes(&self) -> BytesMut {
        let     size      = self.size();
        let     capacity  = 4 + size; // frame size + package size.
        let mut bytes     = BytesMut::with_capacity(capacity);
        let     auth_flag = {
            if self.creds_opt.is_some() {
                0x01
            } else {
                0x00
            }
        };

        bytes.put_u32::<LittleEndian>(size as u32);
        bytes.put_u8(self.cmd.to_u8());
        bytes.put_u8(auth_flag);
        bytes.put_slice(self.correlation.as_bytes());

        if let Some(creds) = self.creds_opt.as_ref() {
            let login_len = creds.login.len();
            let passw_len = creds.password.len();

            bytes.put_u8(login_len as u8);
            bytes.put_slice(creds.login.as_bytes());
            bytes.put_u8(passw_len as u8);
            bytes.put_slice(creds.password.as_bytes());
        }

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

        fn from_utf8_error(err: FromUtf8Error) -> Error {
            Error::new(ErrorKind::Other, format!("Wrong UTF-8 parsing: {}", err))
        }

        let     cmd            = Cmd::from_u8(pkg_buf[0]);
        let     auth_flag      = pkg_buf[1];
        let     correlation    = Uuid::from_bytes(&pkg_buf[2..18]).map_err(|e| to_error(e))?;
        let mut pkg            = Pkg::new(cmd, correlation);
        let mut payload_offset = 18;

        let creds_opt = {
            if auth_flag == 0x01 {
                let login_len = pkg_buf[18] as usize;
                let login_off = 19 + login_len;
                let login_vec = Vec::from(&pkg_buf[19..login_off]);
                let login     = String::from_utf8(login_vec).map_err(|e| from_utf8_error(e))?;
                let passw_len = pkg_buf[login_off] as usize;
                let passw_off = login_off + 1 + passw_len;
                let passw_vec = Vec::from(&pkg_buf[login_off+1..passw_off]);
                let passw     = String::from_utf8(passw_vec).map_err(|e| from_utf8_error(e))?;
                let cred      = Credentials { login: login, password: passw };

                payload_offset = passw_off;

                Some(cred)
            } else {
                None
            }
        };

        if frame_size > payload_offset {
            let payload = &pkg_buf[payload_offset..frame_size];

            pkg.set_payload(payload.to_vec());
        }

        Ok(pkg)
    }
}
