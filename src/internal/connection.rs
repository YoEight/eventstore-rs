use std::io::{ self, Cursor };
use std::net::{ SocketAddr, SocketAddrV4 };

use bytes::{ Bytes, Buf, BufMut, BytesMut, ByteOrder, LittleEndian };
use futures::{ Future, Stream, Sink };
use futures::sync::mpsc::{ Sender, channel };
use futures::stream::iter_ok;
use tokio_io::AsyncRead;
use tokio_io::codec::{ Decoder, Encoder };
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use uuid::{ Uuid, ParseError };

use internal::command::Cmd;
use internal::messaging::Msg;
use internal::package::Pkg;
use types::Credentials;

pub(crate) struct Connection {
    pub(crate) id:     Uuid,
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

impl Decoder for PkgCodec {
    type Item  = Pkg;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Pkg>, Self::Error> {

        if self.state == DecodeState::Frame {
            if src.len() < 4 {
                return Ok(None)
            }

            self.frame_size = LittleEndian::read_u32(&src[0..4]) as usize;
            self.state      = DecodeState::Payload;

            src.advance(4);
        }

        if src.len() < self.frame_size {
            return Ok(None)
        }

        let pkg = {
            let mut cursor            = Cursor::new(&src[0..self.frame_size]);
            let     cmd               = Cmd::from_u8(cursor.get_u8());
            let     auth_flag         = cursor.get_u8();
            let mut correlation_bytes = [0; 16];

            cursor.copy_to_slice(&mut correlation_bytes);

            let correlation = Uuid::from_bytes(&correlation_bytes).map_err(decode_parse_error)?;

            // Client-wise, the server doesn't send back authentication
            // information. For a matter of consistency, we still implement
            // `Credentials` parsing, even if it will never be used at the client
            // level.
            let creds_opt = {
                if auth_flag == 0x01 {
                    let creds = Credentials::parse_from_buf(&mut cursor)?;

                    Some(creds)
                } else {
                    None
                }
            };

            let payload = {
                if self.frame_size > cursor.position() as usize {
                    cursor.collect()
                } else {
                    Bytes::new()
                }
            };

            Pkg {
                cmd,
                creds_opt,
                correlation,
                payload,
            }
        };

        src.advance(self.frame_size);

        self.state      = DecodeState::Frame;
        self.frame_size = 0;

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

        dst.put_u32_le(size as u32);
        dst.put_u8(item.cmd.to_u8());
        dst.put_u8(auth_flag);
        dst.put_slice(item.correlation.as_bytes());

        if let Some(creds) = item.creds_opt.as_ref() {
            creds.write_to_bytes_mut(dst);
        }

        dst.put(item.payload);

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
    pub(crate) fn new(bus: Sender<Msg>, addr: SocketAddrV4, handle: Handle) -> Connection {
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

    pub fn enqueue_all(&self, pkgs: Vec<Pkg>) {
        self.handle.spawn(self.sender.clone().send_all(iter_ok(pkgs)).then(|_| Ok(())))
    }
}
