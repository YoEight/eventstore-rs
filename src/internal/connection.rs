use std::io::{ self, Write, Cursor };
use std::net::{ SocketAddr, SocketAddrV4 };
use std::string::FromUtf8Error;
use std::thread::{ JoinHandle, spawn };

use bytes::{ Buf, BufMut, BytesMut, LittleEndian };
use futures::{ Future, Stream, Sink };
use futures::sync::mpsc::{ Sender, Receiver, channel };
use futures::future::{ self, lazy };
use tokio_io::AsyncRead;
use tokio_io::codec::{ Decoder, Encoder };
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use uuid::{ Uuid, ParseError };

use internal::command::Cmd;
use internal::messaging::Msg;
use internal::package::{ Pkg, PKG_MANDATORY_SIZE };
use internal::types::Credentials;

pub struct Connection {
    pub id:     Uuid,
    sender: Sender<Pkg>,
    handle: Handle,
}

struct PkgCodec {
    state: DecodeState,
    frame_size: usize,
}

impl PkgCodec {
    fn new() -> PkgCodec {
        PkgCodec {
            state: DecodeState::Frame,
            frame_size: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum DecodeState {
    Frame,
    Payload,
}

fn decode_parse_error(err: ParseError) -> io::Error {
    io::Error::new(io::ErrorKind::Other, format!("ParseError {}", err))
}

fn decode_utf8_error(err: FromUtf8Error) -> io::Error {
    io::Error::new(io::ErrorKind::Other,
        format!("Wrong UTF-8 parsing: {}", err))
}

impl Decoder for PkgCodec {
    type Item  = Pkg;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Pkg>, Self::Error> {

        if self.state == DecodeState::Frame {
            if src.len() < 4 {
                return Ok(None)
            }

            let frame_size = {
                let mut frame_cursor = Cursor::new(&src[0..4]);

                frame_cursor.get_u32::<LittleEndian>() as usize
            };

            self.frame_size = frame_size;
            self.state      = DecodeState::Payload;

            src.advance(4);
        }

        if src.len() < self.frame_size {
            return Ok(None)
        }

        let     cmd            = Cmd::from_u8(src[0]);
        let     auth_flag      = src[1];
        let     correlation    = Uuid::from_bytes(&src[2..18]).map_err(decode_parse_error)?;
        let mut pkg            = Pkg::new(cmd, correlation);
        let mut payload_offset = PKG_MANDATORY_SIZE;

        let creds_opt = {
            if auth_flag == 0x01 {
                let login_len = src[18] as usize;
                let login_off = 19 + login_len;
                let login_vec = Vec::from(&src[19..login_off]);
                let login     = String::from_utf8(login_vec).map_err(|e| decode_utf8_error(e))?;
                let passw_len = src[login_off] as usize;
                let passw_off = login_off + 1 + passw_len;
                let passw_vec = Vec::from(&src[login_off+1..passw_off]);
                let passw     = String::from_utf8(passw_vec).map_err(|e| decode_utf8_error(e))?;
                let cred      = Credentials { login: login, password: passw };

                payload_offset = passw_off;

                Some(cred)
            } else {
                None
            }
        };

        pkg.creds_opt = creds_opt;

        if self.frame_size > payload_offset {
            let payload = &src[payload_offset..self.frame_size];

            pkg.set_payload(payload.to_vec());
        }

        self.state      = DecodeState::Frame;
        self.frame_size = 0;

        // We notify Tokio we have consumed all the bytes we wanted.
        // Otherwise, it will complain there are remaining bytes in the buffer.
        src.clear();

        Ok(Some(pkg))
    }
}

impl Encoder for PkgCodec {
    type Item  = Pkg;
    type Error = io::Error;

    fn encode(&mut self, item: Pkg, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let     size      = item.size();
        let     auth_flag = {
            if item.creds_opt.is_some() {
                0x01
            } else {
                0x00
            }
        };

        dst.put_u32::<LittleEndian>(size as u32);
        dst.put_u8(item.cmd.to_u8());
        dst.put_u8(auth_flag);
        dst.put_slice(item.correlation.as_bytes());

        if let Some(creds) = item.creds_opt.as_ref() {
            let login_len = creds.login.len();
            let passw_len = creds.password.len();

            dst.put_u8(login_len as u8);
            dst.put_slice(creds.login.as_bytes());
            dst.put_u8(passw_len as u8);
            dst.put_slice(creds.password.as_bytes());
        }

        dst.put_slice(item.payload.as_slice());

        Ok(())
    }
}

#[derive(Debug)]
enum ConnErr {
    ConnectError(io::Error),
    MpscClose,
}

fn io_error_only<A>(res: Result<A, ConnErr>) -> io::Result<()> {
    match res {
        Ok(_)    => Ok(()),
        Err(typ) => match typ {
            ConnErr::MpscClose       => Ok(()),
            ConnErr::ConnectError(e) => Err(e),
        },
    }
}

impl Connection {
    pub fn new(bus: Sender<Msg>, addr: SocketAddrV4, handle: Handle) -> Connection {
        let (sender, recv) = channel(500);
        let id             = Uuid::new_v4();
        let cloned_handle  = handle.clone();
        let cloned_bus     = bus.clone();

        let conn_fut =
                TcpStream::connect(&SocketAddr::V4(addr), &handle)
                    .map_err(ConnErr::ConnectError)
                    .and_then(move |stream|{
                        bus.send(Msg::Established(id))
                            .map_err(|_| ConnErr::MpscClose)
                            .map(|bus| (bus, stream))
                    });

        let conn_fut =
            conn_fut.and_then(move |(bus, stream)| {
                let (txing, rcving) = stream.framed(PkgCodec::new()).split();

                let rcving =
                    rcving.map_err(ConnErr::ConnectError)
                        .fold(bus.clone(), |bus, pkg| {
                            bus.send(Msg::Arrived(pkg))
                                .map_err(|_| ConnErr::MpscClose)
                        });

                let cloned = bus.clone();
                let rcving =
                    rcving.then(io_error_only)
                        .or_else(move |e| {
                            cloned.send(Msg::ConnectionClosed(id, e))
                                .then(|_| Ok(()))
                        });

                let txing =
                    recv.map_err(|_| ConnErr::MpscClose)
                        .fold(txing, |sender, pkg| {
                            sender.send(pkg).map_err(ConnErr::ConnectError)
                        });

                let cloned = bus.clone();
                let txing  =
                    txing.then(io_error_only)
                        .or_else(move |e| {
                            cloned.send(Msg::ConnectionClosed(id, e))
                                .then(|_| Ok(()))
                        });

                cloned_handle.spawn(rcving);
                cloned_handle.spawn(txing);

                Ok(())
            });

        let conn_fut =
            conn_fut.then(io_error_only)
                .or_else(move |e| {
                    cloned_bus.send(Msg::ConnectionClosed(id, e))
                        .then(|_| Ok(()))
                });

        handle.spawn(conn_fut);

        Connection {
            id,
            sender,
            handle,
        }
    }

    pub fn enqueue(&self, pkg: Pkg) {
        self.handle.spawn(self.sender.clone().send(pkg).then(|_| Ok(())))
    }
}
