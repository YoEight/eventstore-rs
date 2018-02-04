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

#[derive(Copy, Clone)]
enum HeartbeatStatus {
    Init,
    Delay(u32, Timespec),
    Timeout(u32, Timespec),
}

enum Heartbeat {
    Valid,
    Failure,
}

struct HealthTracker {
    pkg_num: u32,
    state: HeartbeatStatus,
    heartbeat_delay: Duration,
    heartbeat_timeout: Duration,
}

impl HealthTracker {
    fn new(setts: &Settings) -> HealthTracker {
        HealthTracker {
            pkg_num: 0,
            state: HeartbeatStatus::Init,
            heartbeat_delay: setts.heartbeat_delay,
            heartbeat_timeout: setts.heartbeat_timeout,
        }
    }

    fn incr_pkg_num(&mut self) {
        self.pkg_num += 1;
    }

    fn manage_heartbeat(&mut self, conn: &Connection) -> Heartbeat {
        match self.state {
            HeartbeatStatus::Init => {
                self.state = HeartbeatStatus::Delay(self.pkg_num, get_time());

                Heartbeat::Valid
            },

            HeartbeatStatus::Delay(num, start) => {
                let now = get_time();

                if self.pkg_num != num {
                    self.state = HeartbeatStatus::Delay(self.pkg_num, now);
                } else {
                    if now - start >= self.heartbeat_delay {
                        self.state = HeartbeatStatus::Timeout(self.pkg_num, now);
                        conn.enqueue(Pkg::heartbeat_request());
                    }
                }

                Heartbeat::Valid
            },

            HeartbeatStatus::Timeout(num, start) => {
                let now = get_time();

                if self.pkg_num != num {
                    self.state = HeartbeatStatus::Delay(self.pkg_num, now);

                    Heartbeat::Valid
                } else {
                    if now - start >= self.heartbeat_timeout {
                        Heartbeat::Failure
                    } else {
                        Heartbeat::Valid
                    }
                }
            },
        }
    }
}

fn worker_thread(settings: &Settings, addr: SocketAddrV4, sender: Sender<Msg>, receiver: Receiver<Msg>) {
    let mut registry  = Registry::new(settings);
    let mut connected = false;
    let mut conn_opt  = None;
    let mut tracker   = HealthTracker::new(settings);
    let     timer     = Timer::new();

    loop {
        if let Some(msg) = receiver.recv() {
            match msg {
                Msg::Start => {
                    let tx1  = sender.clone();
                    let tx2  = sender.clone();
                    let conn = Connection::new(tx1, addr);

                    timer.schedule_repeating(Duration::milliseconds(200), move || {
                        tx2.send(Msg::Tick);
                    });

                    connected = false;
                    conn_opt  = Some(conn);
                },

                Msg::Shutdown => {
                    println!("Shutting down...");
                    break;
                },

                Msg::Established(id) => {
                    if let Some(ref conn) = conn_opt {
                        if conn.id == id {
                            connected = true;
                        }
                    }
                },

                Msg::Arrived(pkg) => {
                    tracker.incr_pkg_num();

                    if connected {
                        match pkg.cmd {
                            Cmd::HeartbeatRequest => {
                                println!("Heartbeat request received");

                                let mut resp = pkg.copy_headers_only();

                                resp.cmd = Cmd::HeartbeatResponse;
                                if let Some(ref conn) = conn_opt {
                                    conn.enqueue(resp);
                                }
                            },

                            _ => registry.handle(pkg),
                        }
                    }
                },

                Msg::Tick => {
                    if connected {
                        if let Some(ref conn) = conn_opt {
                            if let Heartbeat::Valid = tracker.manage_heartbeat(conn) {
                                registry.check_and_retry(conn);
                            } else {
                                println!("Heartbeat TIMEOUT");
                                break;
                            }
                        }
                    }
                },
            }
        } else {
            println!("Main bus closed");
            break;
        }
    }
}

pub struct Client {
    worker: JoinHandle<()>,
    sender: Sender<Msg>,
}

impl Client {
    pub fn new(settings: Settings, addr: SocketAddrV4) -> Client {
        let (sender, recv) = async();

        let tx     = sender.clone();
        let handle = spawn(move || worker_thread(&settings, addr, sender, recv));

        Client {
            worker: handle,
            sender: tx,
        }
    }

    pub fn start(&self) {
        self.sender.send(Msg::Start);
    }

    pub fn shutdown(&self) {
        self.sender.send(Msg::Shutdown);
    }

    pub fn wait_till_closed(self) {
        self.worker.join().unwrap();
    }
}
