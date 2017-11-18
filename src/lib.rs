extern crate chan;
extern crate core;
extern crate bytes;
extern crate uuid;

mod internal;

use core::option::Option;
use std::io::{ Cursor, Read };
use std::net::{ TcpStream, SocketAddrV4 };
use std::thread::{ spawn, JoinHandle };

use chan::{ Sender, Receiver, async };
use uuid::Uuid;

use internal::connection::Connection;
use internal::messaging::Msg;
use internal::package::Pkg;

pub struct Client {
    worker: JoinHandle<()>,
    sender: Sender<Msg>,
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
