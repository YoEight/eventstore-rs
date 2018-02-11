use core::option::Option;
use std::net::SocketAddrV4;
use std::thread::{ spawn, JoinHandle };

use chan::{ Sender, Receiver, async };
use time::{ Duration, Timespec, get_time };
use timer::Timer;
use uuid::Uuid;

use internal::command::Cmd;
use internal::connection::Connection;
use internal::endpoint::Endpoint;
use internal::messaging::Msg;
use internal::messages;
use internal::package::Pkg;
use internal::registry::{ Registry, Outcome };
use internal::types::{ Credentials, Settings };

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

#[derive(PartialEq, Eq)]
enum ConnectionState {
    Init,
    Connecting,
    Connected,
    Closed,
}

struct InitReq {
    correlation: Uuid,
    started: Timespec,
}

impl InitReq {
    fn new(id: Uuid) -> InitReq {
        InitReq {
            correlation: id,
            started: get_time(),
        }
    }
}

#[derive(PartialEq, Eq)]
enum Phase {
    Reconnecting,
    EndpointDiscovery,
    Establishing,
    Authentication,
    Identification,
}

struct Attempt {
    started: Timespec,
    tries: u32,
}

impl Attempt {
    fn new() -> Attempt {
        Attempt {
            started: get_time(),
            tries: 0,
        }
    }

    fn new_try(&self) -> Attempt {
        Attempt {
            started: get_time(),
            tries: self.tries + 1,
        }
    }
}

struct StaticDiscovery {
    addr: SocketAddrV4,
}

impl Discovery for StaticDiscovery {
    fn discover(&mut self, _: Option<&Endpoint>) -> Endpoint {
        Endpoint {
            addr: self.addr,
        }
    }
}

impl StaticDiscovery {
    fn new(addr: SocketAddrV4) -> StaticDiscovery {
        StaticDiscovery {
            addr: addr,
        }
    }
}

trait Discovery {
    fn discover(&mut self, last: Option<&Endpoint>) -> Endpoint;
}

enum Report {
    Continue,
    Quit,
}

struct Driver {
    registry: Registry,
    candidate: Option<Connection>,
    tracker: HealthTracker,
    attempt_opt: Option<Attempt>,
    state: ConnectionState,
    phase: Phase,
    last_endpoint: Option<Endpoint>,
    discovery: Box<Discovery>,
    connection_name: Option<String>,
    default_user: Option<Credentials>,
    operation_timeout: Duration,
    init_req_opt: Option<InitReq>,
    reconnect_delay: Duration,
    max_reconnect: u32,
}

impl Driver {
    fn new(setts: Settings, disc: Box<Discovery>) -> Driver {
        Driver {
            registry: Registry::new(&setts),
            candidate: None,
            tracker: HealthTracker::new(&setts),
            attempt_opt: None,
            state: ConnectionState::Init,
            phase: Phase::Reconnecting,
            last_endpoint: None,
            discovery: disc,
            connection_name: setts.connection_name,
            default_user: setts.default_user,
            operation_timeout: setts.operation_timeout,
            init_req_opt: None,
            reconnect_delay: Duration::seconds(3),
            max_reconnect: 3,
        }
    }

    fn start(&mut self, sender: Sender<Msg>) {
        self.attempt_opt = Some(Attempt::new());
        self.state       = ConnectionState::Connecting;
        self.phase       = Phase::Reconnecting;

        self.discover(sender);
    }

    fn discover(&mut self, sender: Sender<Msg>) {
        if self.state == ConnectionState::Connecting && self.phase == Phase::Reconnecting {
            let endpoint = self.discovery.discover(self.last_endpoint.as_ref());

            self.phase = Phase::EndpointDiscovery;

            // TODO - Will be performed in a different thread.
            sender.send(Msg::Establish(endpoint));
        }
    }

    fn on_establish(&mut self, sender: Sender<Msg>, endpoint: Endpoint) {
        if self.state == ConnectionState::Connecting && self.phase == Phase::EndpointDiscovery {
            self.phase         = Phase::Establishing;
            self.candidate     = Some(Connection::new(sender, endpoint.addr));
            self.last_endpoint = Some(endpoint);
        }
    }

    fn authenticate(&mut self, creds: Credentials) {
        if self.state == ConnectionState::Connecting && self.phase == Phase::Establishing {
            let pkg = Pkg::authenticate(creds);

            self.init_req_opt = Some(InitReq::new(pkg.correlation));
            self.phase        = Phase::Authentication;

            if let Some(conn) = self.candidate.as_ref() {
                conn.enqueue(pkg);
            }
        }
    }

    fn identify_client(&mut self) {
        if self.state == ConnectionState::Connecting && (self.phase == Phase::Authentication || self.phase == Phase::Establishing) {
            let pkg = Pkg::identify_client(&self.connection_name);

            self.init_req_opt = Some(InitReq::new(pkg.correlation));
            self.phase        = Phase::Identification;

            if let Some(conn) = self.candidate.as_ref() {
                conn.enqueue(pkg);
            }
        }
    }

    fn on_established(&mut self, id: Uuid) {
        if self.state == ConnectionState::Connecting && self.phase == Phase::Establishing {
            let same_connection = {
                match self.candidate {
                    Some(ref conn) => conn.id == id,
                    None           => false,
                }
            };

            if same_connection {
                match self.default_user.clone() {
                    Some(creds) => self.authenticate(creds),
                    None        => self.identify_client(),
                }
            }
        }
    }

    fn on_package_arrived(&mut self, pkg: Pkg) {
         self.tracker.incr_pkg_num();

         if pkg.cmd == Cmd::ClientIdentified && self.state == ConnectionState::Connecting && self.phase == Phase::Identification {
             self.init_req_opt = None;
             self.attempt_opt  = None;
             self.state        = ConnectionState::Connected;
         } else if (pkg.cmd == Cmd::Authenticated || pkg.cmd == Cmd::NotAuthenticated) && self.state == ConnectionState::Connecting && self.phase == Phase::Authentication {
             if pkg.cmd == Cmd::NotAuthenticated {
                 println!("warn: Not authenticated.");
             }

             self.identify_client();
         } else {
             if self.state == ConnectionState::Connected {
                 match pkg.cmd {
                     Cmd::HeartbeatRequest => {
                         println!("Heartbeat request received");

                         let mut resp = pkg.copy_headers_only();

                         resp.cmd = Cmd::HeartbeatResponse;

                         if let Some(ref conn) = self.candidate {
                             conn.enqueue(resp);
                         }
                     },

                     _ => self.registry.handle(pkg),
                 }
             }
         }
    }

    fn has_init_req_timeout(&self, now: &Timespec) -> bool {
        if let Some(ref req) = self.init_req_opt {
            *now - req.started >= self.operation_timeout
        } else {
            false
        }
    }

    fn on_tick(&mut self) -> Report {
        let now = get_time();

        if self.state == ConnectionState::Init || self.state == ConnectionState::Closed {

            Report::Continue

        } else if self.state == ConnectionState::Connecting {
            if self.phase == Phase::Reconnecting {
                if let Some(att) = self.attempt_opt.as_mut() {
                    if now - att.started >= self.reconnect_delay {
                        att.tries += 1;

                        if att.tries > self.max_reconnect {
                            Report::Quit
                        } else {
                            att.started = get_time();

                            // TODO - Implement automatic reconnection when the previous
                            // connection took too many times.
                            // self.discover(sender);

                            Report::Continue
                        }
                    } else {
                        Report::Quit
                    }
                } else {
                    Report::Continue
                }
            } else if self.phase == Phase::Authentication {
                if self.has_init_req_timeout(&now) {
                    println!("warn: authentication has timeout.");

                    self.identify_client();
                }

                Report::Continue

            } else if self.phase == Phase::Identification {
                if self.has_init_req_timeout(&now) {
                    Report::Quit
                } else {
                    Report::Continue
                }
            } else {
                Report::Continue
            }

        } else {
            // Connected state
            if let Some(ref conn) = self.candidate {
                if let Heartbeat::Valid = self.tracker.manage_heartbeat(conn) {
                    self.registry.check_and_retry(conn);

                    Report::Continue
                } else {
                    println!("Heartbeat TIMEOUT");

                    Report::Quit
                }
            } else {
                // Impossible situation, if at 'Connected' state, it means we must
                // have a valued candidate property.
                Report::Continue
            }
        }
    }
}

fn worker_thread(settings: Settings, disc: Box<Discovery>, sender: Sender<Msg>, receiver: Receiver<Msg>) {
    let mut driver = Driver::new(settings, disc);
    let     timer  = Timer::new();

    loop {
        if let Some(msg) = receiver.recv() {
            match msg {
                Msg::Start => {
                    let tx1 = sender.clone();
                    let tx2 = sender.clone();

                    timer.schedule_repeating(Duration::milliseconds(200), move || {
                        tx1.send(Msg::Tick);
                    });

                    driver.start(tx2);
                },

                Msg::Shutdown => {
                    println!("Shutting down...");
                    break;
                },

                Msg::Establish(endpoint) =>
                    driver.on_establish(sender.clone(), endpoint),

                Msg::Established(id) => {
                    driver.on_established(id);
                }

                Msg::Arrived(pkg) => {
                    driver.on_package_arrived(pkg);
                },

                Msg::Tick => {
                    if let Report::Quit = driver.on_tick() {
                        break
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
        let disc           = Box::new(StaticDiscovery::new(addr));

        let tx     = sender.clone();
        let handle = spawn(move || worker_thread(settings, disc, sender, recv));

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
