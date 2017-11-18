use core::option::Option;
use std::net::SocketAddrV4;
use std::thread::{ spawn, JoinHandle };

use chan::{ Sender, Receiver, async };

use internal::connection::Connection;
use internal::messaging::Msg;

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
        let mut keep_going = true;
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

                    Msg::Arrived(pkg) => {
                        if connected {
                            match pkg.cmd {
                                0x01 => {
                                    for conn in &connection {
                                        let mut resp = pkg.copy_headers_only();

                                        resp.cmd = 0x02;

                                        conn.enqueue(resp);
                                    }
                                },

                                unknown => {
                                    println!("Unknown command [{}].", unknown);
                                }
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
