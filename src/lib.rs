extern crate chan;
extern crate core;
extern crate bytes;
extern crate uuid;

use core::option::Option;
use core::result::Result;
use std::io::{ Error, Cursor };
use std::net::{ TcpStream, SocketAddrV4 };
use std::thread::{ spawn, JoinHandle };

use bytes::{ Buf, BytesMut, LittleEndian };
use bytes::buf::BufMut;
use chan::{ Sender, Receiver, async };
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

    // Copies the Pkg except its payload.
    fn copy_headers_only(&self) -> Pkg {
        Pkg {
            cmd:         self.cmd,
            correlation: self.correlation,
        }
    }
}

// pub struct PkgCodec;
//
// impl Encoder for PkgCodec {
//     type Item  = Pkg;
//     type Error = Error;
//
//     fn encode(&mut self, item: Pkg, dest: &mut BytesMut) -> Result<(), Error> {
//         dest.put_u32::<LittleEndian>(item.size());
//         dest.put_u8(item.cmd);
//         dest.put_u8(0); // Package credential flag.
//         dest.put_slice(item.correlation.as_bytes());
//
//         Result::Ok(())
//     }
// }
//
// impl Decoder for PkgCodec {
//     type Item  = Pkg;
//     type Error = Error;
//
//     fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Pkg>, Error> {
//         // Checks if the frame size has been sent.
//         if src.len() < 4 {
//             Result::Ok(Option::None)
//         } else {
//             let mut frame_cursor = Cursor::new(src.split_to(3));
//             let frame            = frame_cursor.get_u32::<LittleEndian>() as usize;
//
//             // Checks if all the message payload has been sent.
//             if src.len() < frame {
//                 Result::Ok(Option::None)
//             } else {
//                 let cmd         = src[0];
//                 let correlation = Uuid::from_bytes(&src[2..18]).unwrap();
//
//                 Result::Ok(Option::Some(Pkg::new(cmd, correlation)))
//             }
//         }
//     }
// }

enum Msg {
    Start,
    Shutdown,
    Established(Uuid),
}

pub struct Client {
    worker: JoinHandle<()>,
    sender: Sender<Msg>,
}

struct Connection {
    id:     Uuid,
    sender: Sender<Pkg>,
    worker: JoinHandle<()>,
}

impl Connection {
    fn new(bus: Sender<Msg>, addr: SocketAddrV4) -> Connection {
        let (sender, recv) = async();
        let id             = Uuid::new_v4();
        let worker         = spawn(move || Connection::create_conn(id, recv, bus, addr));

        Connection {
            id:     id,
            sender: sender,
            worker: worker,
        }
    }

    fn create_conn(id: Uuid, rx: Receiver<Pkg>, bus: Sender<Msg>, addr: SocketAddrV4) {
        let stream = TcpStream::connect(addr).unwrap();

        bus.send(Msg::Established(id));
    }
}

impl Client {
    pub fn new(addr: SocketAddrV4) -> Client {
        let (sender, recv) = async();
        let tx             = sender.clone();
        let handle         = spawn(move || Client::worker_thread(addr, tx, recv));

        Client {
            worker: handle,
            sender: sender,
        }
    }

    fn worker_thread(addr: SocketAddrV4, bus: Sender<Msg>, queue: Receiver<Msg>) {
        let mut keep_going  = true;
        let mut connection = Option::None;
        let mut connected  = false;

        while keep_going {
            let msg_opt = queue.recv();

            match msg_opt {
                Option::Some(msg) => match msg {
                    Msg::Start => {
                        connection = Option::Some(Connection::new(bus.clone(), addr));
                    },

                    Msg::Shutdown => {
                        keep_going = false;
                        println!("Shutting down...");
                    },

                    Msg::Established(id) => {
                        for conn in &connection {
                            if conn.id == id {
                                connected = true;
                            }
                        }
                    },
                },

                Option::None => {
                    println!("Main bus closed");
                    keep_going = false;
                }
            }
        }
    }
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
