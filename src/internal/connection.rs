use futures::channel::mpsc::{channel, Sender};
use futures::sink::SinkExt;
use futures::stream::iter;
use futures::stream::StreamExt;
use std::net::SocketAddr;
use tokio::io::{split, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio_byteorder::{AsyncWriteBytesExt, LittleEndian};
use uuid::{BytesError, Uuid};

use crate::internal::command::Cmd;
use crate::internal::messaging::Msg;
use crate::internal::package::Pkg;

pub struct Connection {
    pub id: Uuid,
    pub desc: String,
    sender: Sender<Pkg>,
}

fn decode_bytes_error(err: BytesError) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, format!("BytesError {}", err))
}

async fn decode_pkg(reading: &mut ReadHalf<TcpStream>) -> std::io::Result<Pkg> {
    let frame_size = tokio_byteorder::AsyncReadBytesExt::read_u32::<LittleEndian>(reading).await?;
    let mut src = AsyncReadExt::take(reading, frame_size.into());
    let cmd = AsyncReadExt::read_u8(&mut src).await?;
    let cmd = Cmd::from_u8(cmd);
    // Parses the authentication flag. The server always sends 0 on responses.
    let _ = AsyncReadExt::read_u8(&mut src).await?;
    let mut uuid = [0; 16];

    src.read_exact(&mut uuid).await?;

    let correlation = Uuid::from_slice(&uuid).map_err(decode_bytes_error)?;
    let mut payload: Vec<u8> = Vec::with_capacity(src.limit() as usize);

    src.read_to_end(&mut payload).await?;

    let pkg = Pkg {
        cmd,
        creds_opt: None,
        correlation,
        payload: payload.into(),
    };

    Ok(pkg)
}

async fn encode_pkg(dest: &mut WriteHalf<TcpStream>, pkg: Pkg) -> std::io::Result<()> {
    let size = pkg.size();
    let auth_flag = if pkg.creds_opt.is_some() { 0x01 } else { 0x00 };

    AsyncWriteBytesExt::write_u32::<LittleEndian>(dest, size as u32).await?;
    AsyncWriteExt::write_u8(dest, pkg.cmd.to_u8()).await?;
    AsyncWriteExt::write_u8(dest, auth_flag).await?;
    dest.write_all(pkg.correlation.as_bytes()).await?;

    if let Some(creds) = pkg.creds_opt.as_ref() {
        AsyncWriteExt::write_u8(dest, creds.login.len() as u8).await?;
        dest.write_all(creds.login.as_ref()).await?;
        AsyncWriteExt::write_u8(dest, creds.password.len() as u8).await?;
        dest.write_all(creds.password.as_ref()).await?;
    }

    dest.write_all(pkg.payload.as_ref()).await?;
    dest.flush().await?;

    Ok(())
}

#[inline]
fn timeout_error() -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Interrupted, "Connection timeout")
}

impl Connection {
    pub fn new(mut bus: Sender<Msg>, addr: SocketAddr) -> Connection {
        let (sender, mut recv) = channel(500);
        let id = Uuid::new_v4();
        let desc = format!("{:?}", addr);
        let delay = std::time::Duration::from_secs(5);

        tokio::spawn(async move {
            let result = tokio::time::timeout(delay, TcpStream::connect(&addr)).await;
            match result {
                Ok(result) => match result {
                    Ok(stream) => {
                        let _ = bus.send(Msg::Established(id)).await;
                        let (mut reading, mut writing) = split(stream);
                        let mut reading_bus = bus.clone();

                        tokio::spawn(async move {
                            loop {
                                let result = decode_pkg(&mut reading).await;

                                match result {
                                    Ok(pkg) => {
                                        let _ = reading_bus.send(Msg::Arrived(pkg)).await;
                                    }

                                    Err(e) => {
                                        let _ =
                                            reading_bus.send(Msg::ConnectionClosed(id, e)).await;

                                        break;
                                    }
                                }
                            }
                        });

                        let mut writing_bus = bus.clone();

                        tokio::spawn(async move {
                            while let Some(pkg) = recv.next().await {
                                if let Err(e) = encode_pkg(&mut writing, pkg).await {
                                    let _ = writing_bus.send(Msg::ConnectionClosed(id, e)).await;

                                    break;
                                }
                            }
                        });
                    }

                    Err(err) => {
                        let _ = bus.send(Msg::ConnectionClosed(id, err)).await;
                    }
                },

                Err(_) => {
                    let _ = bus.send(Msg::ConnectionClosed(id, timeout_error())).await;
                }
            }
        });

        Connection { id, desc, sender }
    }

    pub async fn enqueue(&mut self, pkg: Pkg) {
        let _ = self.sender.send(pkg).await;
    }

    pub async fn enqueue_all(&mut self, pkgs: Vec<Pkg>) {
        let stream = pkgs.into_iter().map(Ok);
        let _ = self.sender.send_all(&mut iter(stream)).await;
    }
}
