use core::option::Option;
use std::net::SocketAddrV4;
use std::thread::{ spawn, JoinHandle };

use chan::{ Sender, Receiver, async };
use time::Duration;
use timer::Timer;

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
   connection: ConnState,
}

impl Internal {
    fn new() -> Internal {
        let (sender, recv) = async();

        Internal {
            sender: sender,
            receiver: recv,
            connection: ConnState::Disconnected,
        }
    }

    fn clone_sender(&self) -> Sender<Msg> {
        self.sender.clone()
    }

    fn clone_receiver(&self) -> Receiver<Msg> {
        self.receiver.clone()
    }

    fn is_connected(&self) -> bool {
        match self.connection {
            ConnState::Connected(_) => true,
            _                       => false,
        }
    }

    fn set_pending(&mut self, conn: Connection) {
        self.connection = ConnState::Pending(conn);
    }

    fn with_connection<F>(&self, func: F)
        where F: FnOnce(&Connection)
    {
        match self.connection {
            ConnState::Pending(ref conn)   => func(conn),
            ConnState::Connected(ref conn) => func(conn),
            _                              => (),
        }
    }

    fn when_connected<F>(&self, func: F)
        where F: FnOnce(&Connection)
    {
        match self.connection {
            ConnState::Connected(ref conn) => func(conn),
            _                              => (),
        }
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
                                    println!("Heartbeat request received");

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
