use core::option::Option;
use std::net::SocketAddrV4;
use std::thread::{ spawn, JoinHandle };

use chan::{ Sender, Receiver, async };
use time::{ Duration, Timespec, get_time };
use timer::Timer;
use uuid::Uuid;

use internal::command::Cmd;
use internal::connection::Connection;
use internal::messaging::Msg;
use internal::messages;
use internal::package::Pkg;
use internal::registry::{ Registry, Outcome };
use internal::types::Settings;

use protobuf;

pub struct Client {
    worker: JoinHandle<()>,
    sender: Sender<Msg>,
    timer:  Timer,
    settings: Settings,
}

struct Internal {
   sender: Sender<Msg>,
   receiver: Receiver<Msg>,
   pkg_num: u32,
   connected: bool,
   connection: Option<Connection>,
   heartbeat: Option<HeartbeatStatus>,
   settings: Settings,
}

#[derive(Copy, Clone)]
enum HeartbeatStatus {
    Delay(u32, Timespec),
    Timeout(u32, Timespec),
}

enum Heartbeat {
    Valid,
    Failure,
}

impl Internal {
    fn new(settings: Settings) -> Internal {
        let (sender, recv) = async();

        Internal {
            sender: sender,
            receiver: recv,
            pkg_num: 0,
            connected: false,
            connection: Option::None,
            heartbeat: Option::None,
            settings: settings,
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

    fn manage_heartbeat(&mut self) -> Heartbeat {
        match self.heartbeat {
            Some(status) => {
                match status {
                    HeartbeatStatus::Delay(num, start) => {
                        if self.pkg_num != num {
                            let new_status = HeartbeatStatus::Delay(self.pkg_num, get_time());

                            self.heartbeat = Option::Some(new_status);
                        } else {
                            let now = get_time();

                            if now - start >= self.settings.heartbeat_delay {
                                let new_status = HeartbeatStatus::Timeout(self.pkg_num, now);
                                let req        = Pkg::heartbeat_request();

                                self.heartbeat = Option::Some(new_status);
                                self.when_connected(move |conn| conn.enqueue(req));
                            }
                        }

                        Heartbeat::Valid
                    },

                    HeartbeatStatus::Timeout(num, start) => {
                        if self.pkg_num != num {
                            let new_status = HeartbeatStatus::Delay(self.pkg_num, get_time());

                            self.heartbeat = Some(new_status);

                            Heartbeat::Valid
                        } else {
                            let now = get_time();

                            if now - start >= self.settings.heartbeat_timeout {
                                Heartbeat::Failure
                            } else {
                                Heartbeat::Valid
                            }
                        }
                    },
                }
            },

            _ => {
                let new_status = HeartbeatStatus::Delay(self.pkg_num, get_time());

                self.heartbeat = Some(new_status);

                Heartbeat::Valid
            },
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
    pub fn new(settings: Settings, addr: SocketAddrV4) -> Client {
        let mut state  = Internal::new(settings);
        let     sender = state.clone_sender();
        let     handle = spawn(move || Client::worker_thread(&mut state, addr));

        Client {
            worker: handle,
            sender: sender,
            timer:  Timer::new(),
            settings: settings,
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
        let mut registry = Registry::new(state.settings);

        loop {
            let msg_opt = state.recv_msg();

            match msg_opt {
                Option::Some(msg) => match msg {
                    Msg::Start => {
                        let bus  = state.clone_sender();
                        let conn = Connection::new(bus, addr);

                        state.set_pending(conn);
                    },

                    Msg::Shutdown => {
                        println!("Shutting down...");
                        break;
                    },

                    Msg::Established(id) => {
                        state.switch_to_connected_if_same_connection(id);
                    },

                    Msg::Arrived(pkg) => {
                        state.pkg_num += 1;

                        state.when_connected(|conn| {
                            match pkg.cmd {
                                Cmd::HeartbeatRequest => {
                                    println!("Heartbeat request received");

                                    let mut resp = pkg.copy_headers_only();

                                    resp.cmd = Cmd::HeartbeatResponse;

                                    conn.enqueue(resp);
                                },

                                _ => registry.handle(pkg),
                            }
                        });
                    },

                    Msg::Tick => {
                        if state.connected  {
                            if let Heartbeat::Valid = state.manage_heartbeat() {
                                if let Some(ref conn) = state.connection {
                                    registry.check_and_retry(conn);
                                }
                            } else {
                                println!("Heartbeat TIMEOUT");
                                break;
                            }
                        }
                    },
                },

                Option::None => {
                    println!("Main bus closed");
                    break;
                }
            }
        }
    }
}
