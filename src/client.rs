use core::option::Option;
use std::net::SocketAddrV4;
use std::thread::{ spawn, JoinHandle };

use chan::{ Sender, Receiver, async };
use time::Duration;
use timer::Timer;
use uuid::Uuid;

use internal::connection::Connection;
use internal::messaging::Msg;

pub struct Client {
    worker: JoinHandle<()>,
    sender: Sender<Msg>,
    timer:  Timer,
}

enum ConnState {
    Disconnected,
    Pending(Connection),
    Connected(Connection),
}

struct Internal {
   sender: Sender<Msg>,
   receiver: Receiver<Msg>,
   connected: bool,
   connection: Option<Connection>,
}

impl Internal {
    fn new() -> Internal {
        let (sender, recv) = async();

        Internal {
            sender: sender,
            receiver: recv,
            connected: false,
            connection: Option::None,
        }
    }

    fn clone_sender(&self) -> Sender<Msg> {
        self.sender.clone()
    }

    fn clone_receiver(&self) -> Receiver<Msg> {
        self.receiver.clone()
    }

    fn recv_msg(&self) -> Option<Msg> {
        self.receiver.recv()
    }

    fn is_connected(&self) -> bool {
        self.connected
    }

    fn set_pending(&mut self, conn: Connection) {
        self.connected  = false;
        self.connection = Option::Some(conn);
    }

    fn switch_to_connected_if_same_connection(&mut self, cid: Uuid) {
        match self.connection {
            Option::Some(ref conn) if conn.id == cid => {
                self.connected = true;
            },
            _ => ()
        }
    }

    fn with_connection<F>(&self, func: F)
        where F: FnOnce(&Connection)
    {
        match self.connection {
            Option::Some(ref conn) => func(conn),
            _                      => (),
        }
    }

    fn when_connected<F>(&self, func: F)
        where F: FnOnce(&Connection)
    {
        match self.connection {
            Option::Some(ref conn) if self.connected => func(conn),
            _                                        => (),
        }
    }

}

impl Client {
    pub fn new(addr: SocketAddrV4) -> Client {
        let mut state  = Internal::new();
        let     sender = state.clone_sender();
        let     handle = spawn(move || Client::worker_thread(&mut state, addr));

        Client {
            worker: handle,
            sender: sender,
            timer:  Timer::new(),
        }
    }

    pub fn start(&self) {
        let tx = self.sender.clone();

        self.sender.send(Msg::Start);
        self.timer.schedule_repeating(Duration::milliseconds(200), move || {
           tx.send(Msg::Tick);
        });
    }

    pub fn shutdown(&self) {
        self.sender.send(Msg::Shutdown);
    }

    pub fn wait_till_closed(self) {
        self.worker.join().unwrap();
    }

    fn worker_thread(state: &mut Internal, addr: SocketAddrV4) {
        let mut keep_going = true;

        while keep_going {
            let msg_opt = state.recv_msg();

            match msg_opt {
                Option::Some(msg) => match msg {
                    Msg::Start => {
                        let bus  = state.clone_sender();
                        let conn = Connection::new(bus, addr);

                        state.set_pending(conn);
                    },

                    Msg::Shutdown => {
                        keep_going = false;
                        println!("Shutting down...");
                    },

                    Msg::Established(id) => {
                        state.switch_to_connected_if_same_connection(id);
                    },

                    Msg::Arrived(pkg) => {
                        state.when_connected(|conn| {
                            match pkg.cmd {
                                0x01 => {
                                    println!("Heartbeat request received");

                                    let mut resp = pkg.copy_headers_only();

                                    resp.cmd = 0x02;

                                    conn.enqueue(resp);
                                },

                                unknown => {
                                    println!("Unknown command [{}].", unknown);
                                }
                            }
                        });
                    },

                    Msg::Tick => {
                    
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
