use std::io::{ self, Cursor };
use std::net::SocketAddr;

use bytes::{ Bytes, Buf, BufMut, BytesMut, ByteOrder, LittleEndian };
use futures::{ Future, Stream, Sink };
use futures::sync::mpsc::{ Sender, channel };
use futures::stream::iter_ok;
use tokio::spawn;
use tokio::codec::{ Decoder, Encoder };
use tokio::net::TcpStream;
use tokio::timer::timeout::Timeout;
use tokio::prelude;
use uuid::{ Uuid, BytesError };

use crate::internal::command::Cmd;
use crate::internal::messaging::Msg;
use crate::internal::package::Pkg;
use crate::types::Credentials;

pub(crate) struct Connection {
    pub(crate) id: Uuid,
    pub(crate) desc: String,
    sender: Sender<Pkg>,
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

fn decode_bytes_error(err: BytesError) -> io::Error {
    io::Error::new(io::ErrorKind::Other, format!("BytesError {}", err))
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

            let correlation = Uuid::from_slice(&correlation_bytes).map_err(decode_bytes_error)?;

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
        use std::mem::size_of;

        let size = item.size();

        dst.reserve(size + size_of::<u32>());

        let auth_flag = {
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

#[inline]
fn timeout_error() -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Interrupted, "Connection timeout")
}

#[inline]
fn bus_closed() -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, "Bus closed")
}

impl Connection {
    pub(crate) fn new(bus: Sender<Msg>, addr: SocketAddr) -> Connection {
        let bus = bus.sink_map_err(|_| ());
        let (sender, recv) = channel(500);
        let id = Uuid::new_v4();
        let desc = format!("{:?}", addr);
        let delay = std::time::Duration::from_secs(5);

        let conn_fut =
            Timeout::new(TcpStream::connect(&addr), delay).then(move |result| {
                match result {
                    Ok(stream) => {
                        let (txing, rcving) = PkgCodec::new().framed(stream).split();

                        let input =
                            prelude::stream::once(Ok(Msg::Established(id)))
                                .select(rcving.map(Msg::Arrived))
                                .or_else(move |e| Ok(Msg::ConnectionClosed(id, e)));

                        let reading = input.forward(bus.clone());
                        let writing = recv.map_err(|_| bus_closed()).forward(txing);

                        tokio::spawn(reading.map(|_| ()));
                        tokio::spawn(writing.then(move |result| {
                            if let Err(e) = result {
                                let action =
                                    bus.send(Msg::ConnectionClosed(id, e))
                                        .then(|_| Ok::<(), ()>(()));

                                tokio::spawn(action);
                            }

                            Ok::<(), ()>(())
                        }));
                    },

                    Err(err) => {
                        let real_reason = if let Some(reason) = err.into_inner() {
                            reason
                        } else {
                            timeout_error()
                        };

                        let action =
                            bus.send(Msg::ConnectionClosed(id, real_reason))
                                .map(|_| ());

                        tokio::spawn(action);
                    },
                };

                Ok::<(), ()>(())
            });

        tokio::spawn(conn_fut);

        Connection {
            id,
            desc,
            sender,
        }
    }

    pub fn enqueue(&self, pkg: Pkg) {
        spawn(self.sender.clone().send(pkg).then(|_| Ok(())));
    }

    pub fn enqueue_all(&self, pkgs: Vec<Pkg>) {
        spawn(self.sender.clone().send_all(iter_ok(pkgs)).then(|_| Ok(())));
    }
}
