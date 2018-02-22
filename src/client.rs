use core::option::Option;
use std::io::{ Error, ErrorKind };
use std::net::SocketAddrV4;
use std::thread::{ spawn, JoinHandle };

use chan::{ Sender, Receiver, async };
use time::{ Duration, Timespec, get_time };
use timer::{ Timer, Guard };
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

    fn reset(&mut self) {
        self.state = HeartbeatStatus::Init;

        println!("HealthTracker: Resetting");
    }

    fn manage_heartbeat(&mut self, conn: &Connection) -> Heartbeat {
        match self.state {
            HeartbeatStatus::Init => {
                self.state = HeartbeatStatus::Delay(self.pkg_num, get_time());

                println!("HealthTracker: Delay");
                Heartbeat::Valid
            },

            HeartbeatStatus::Delay(num, start) => {
                let now = get_time();

                if self.pkg_num != num {
                    self.state = HeartbeatStatus::Delay(self.pkg_num, now);
                    println!("HealthTracker: Delay");
                } else {
                    println!("Delay duration: {}", now - start);
                    if now - start >= self.heartbeat_delay {
                        self.state = HeartbeatStatus::Timeout(self.pkg_num, now);
                        conn.enqueue(Pkg::heartbeat_request());
                        println!("HealthTracker: Timeout");
                    }
                }

                Heartbeat::Valid
            },

            HeartbeatStatus::Timeout(num, start) => {
                let now = get_time();

                if self.pkg_num != num {
                    self.state = HeartbeatStatus::Delay(self.pkg_num, now);
                    println!("HealthTracker: Delay");

                    Heartbeat::Valid
                } else {
                    println!("Timeout duration: {}", now - start);
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

#[derive(PartialEq, Eq)]
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
    sender: Sender<Msg>,
    operation_check_period: Duration,
    last_operation_check: Timespec,
}

impl Driver {
    fn new(setts: Settings, disc: Box<Discovery>, sender: Sender<Msg>) -> Driver {
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
            max_reconnect: setts.connection_retry.to_u32(),
            sender: sender,
            operation_check_period: setts.operation_check_period,
            last_operation_check: get_time(),
        }
    }

    fn start(&mut self, timer: &Timer) -> Guard {
        self.attempt_opt = Some(Attempt::new());
        self.state       = ConnectionState::Connecting;
        self.phase       = Phase::Reconnecting;

        let tx    = self.sender.clone();
        let guard = timer.schedule_repeating(Duration::milliseconds(200), move || {
            tx.send(Msg::Tick);
        });

        self.discover();

        guard
    }

    fn discover(&mut self) {
        if self.state == ConnectionState::Connecting && self.phase == Phase::Reconnecting {
            println!("Driver: Discover");
            let endpoint = self.discovery.discover(self.last_endpoint.as_ref());

            self.phase = Phase::EndpointDiscovery;

            // TODO - Will be performed in a different thread.
            self.sender.send(Msg::Establish(endpoint));
            self.tracker.reset();
        }
    }

    fn on_establish(&mut self, endpoint: Endpoint) {
        if self.state == ConnectionState::Connecting && self.phase == Phase::EndpointDiscovery {
            println!("Driver: Establish");
            self.phase         = Phase::Establishing;
            self.candidate     = Some(Connection::new(self.sender.clone(), endpoint.addr));
            self.last_endpoint = Some(endpoint);
        }
    }

    fn authenticate(&mut self, creds: Credentials) {
        if self.state == ConnectionState::Connecting && self.phase == Phase::Establishing {
            println!("Driver: Authenticate");
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
            println!("Driver: Identify");
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
                println!("Driver: Established");
                self.tracker.reset();

                match self.default_user.clone() {
                    Some(creds) => self.authenticate(creds),
                    None        => self.identify_client(),
                }
            }
        }
    }

    fn is_same_connection(&self, conn_id: &Uuid) -> bool {
        match self.candidate {
            Some(ref conn) => conn.id == *conn_id,
            None           => false,
        }
    }

    fn on_connection_closed(&mut self, conn_id: Uuid, error: Error) {
        if self.is_same_connection(&conn_id) {
            self.tcp_connection_close(error);
        }
    }

    fn tcp_connection_close(&mut self, _: Error) {
        match self.state {
            ConnectionState::Connected => {
                self.attempt_opt = Some(Attempt::new());
                self.state       = ConnectionState::Connecting;
                self.phase       = Phase::Reconnecting;
            },

            ConnectionState::Connecting => {
                self.state = ConnectionState::Connecting;
                self.phase = Phase::Reconnecting;
            },

            _ => (),
        }
    }

    fn on_package_arrived(&mut self, pkg: Pkg) {
        self.tracker.incr_pkg_num();

        if pkg.cmd == Cmd::ClientIdentified && self.state == ConnectionState::Connecting && self.phase == Phase::Identification {
            self.init_req_opt         = None;
            self.attempt_opt          = None;
            self.last_operation_check = get_time();
            self.state                = ConnectionState::Connected;
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

                    Cmd::HeartbeatResponse => { println!("Heartbeat response"); },

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

    fn conn_has_timeout(&self, now: &Timespec) -> bool {
        if let Some(att) = self.attempt_opt.as_ref() {
            *now - att.started >= self.reconnect_delay
        } else {
            false
        }
    }

    fn start_new_attempt(&mut self, now: Timespec) -> bool {
        if let Some(att) = self.attempt_opt.as_mut() {
            att.tries   += 1;
            att.started = now;

            att.tries <= self.max_reconnect
        } else {
            false
        }
    }

    fn close_connection(&mut self) {
        self.state = ConnectionState::Closed;
    }

    fn manage_heartbeat(&mut self) {
        let has_timeout =
                if let Some(ref conn) = self.candidate {
                    match self.tracker.manage_heartbeat(conn) {
                        Heartbeat::Valid   => false,
                        Heartbeat::Failure => true,
                    }
                } else {
                    false
                };

        if has_timeout {
            self.tcp_connection_close(heartbeat_timeout_error());
        }
    }

    fn on_tick(&mut self) -> Report {
        let now = get_time();

        if self.state == ConnectionState::Init || self.state == ConnectionState::Closed {

            Report::Continue

        } else if self.state == ConnectionState::Connecting {
            if self.phase == Phase::Reconnecting {
                let now = get_time();

                if self.conn_has_timeout(&now) {
                    if self.start_new_attempt(now) {
                        println!("New connection attempt");
                        self.discover();

                        Report::Continue
                    } else {
                        println!("Connection max attempt reached.");
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

                self.manage_heartbeat();

                Report::Continue

            } else if self.phase == Phase::Identification {
                if self.has_init_req_timeout(&now) {
                    Report::Quit
                } else {
                    self.manage_heartbeat();

                    Report::Continue
                }
            } else {
                Report::Continue
            }

        } else {
            // Connected state
            if let Some(ref conn) = self.candidate {
                let now = get_time();

                if now - self.last_operation_check >= self.operation_check_period {
                    self.registry.check_and_retry(conn);

                    self.last_operation_check = now;
                }
            }

            self.manage_heartbeat();

            Report::Continue
        }
    }
}

fn worker_thread(settings: Settings, disc: Box<Discovery>, sender: Sender<Msg>, receiver: Receiver<Msg>) {
    let mut driver = Driver::new(settings, disc, sender);
    let     timer  = Timer::new();
    let mut ticker = None;

    loop {
        if let Some(msg) = receiver.recv() {
            match msg {
                Msg::Start => {
                    ticker = Some(driver.start(&timer));
                },

                Msg::Shutdown => {
                    println!("Shutting down...");
                    break;
                },

                Msg::Establish(endpoint) =>
                    driver.on_establish(endpoint),

                Msg::Established(id) => {
                    driver.on_established(id);
                },

                Msg::ConnectionClosed(conn_id, error) => {
                    driver.on_connection_closed(conn_id, error);
                },

                Msg::Arrived(pkg) => {
                    driver.on_package_arrived(pkg);
                },

                Msg::Tick => {
                    if let Report::Quit = driver.on_tick() {
                        driver.close_connection();
                        break
                    }
                },
            }
        } else {
            println!("Main bus closed");
            break;
        }
    }

    println!("Driver has terminated.");
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

fn heartbeat_timeout_error() -> Error {
    Error::new(ErrorKind::Other, "Heartbeat timeout error.")
}
