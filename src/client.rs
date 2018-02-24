use core::option::Option;
use std::io::{ Error, ErrorKind };
use std::net::SocketAddrV4;
use std::thread::{ spawn, JoinHandle };

use futures::{ Async, Poll, Future, Stream, Sink };
use futures::stream::poll_fn;
use futures::sync::mpsc::{ Sender, Receiver, channel };
use time::{ Duration, Timespec, get_time };
use timer::{ Timer, Guard };
use tokio::executor::current_thread;
use tokio_core::reactor::{ Core, Handle };
use futures::future::lazy;
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
                        println!("Closing connection [{}] due to HEARTBEAT TIMEOUT at pkgNum {}.", conn.id, self.pkg_num);

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

struct TickRate {
    period: Duration,
    state: Timespec,
}

impl TickRate {
    fn new(period: Duration) -> TickRate {
        TickRate {
            period: period,
            state: get_time(),
        }
    }
}

impl Stream for TickRate {
    type Item  = Msg;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Msg>, ()> {
        let now = get_time();

        if now - self.state >= self.period {
            self.state = now;

            Ok(Async::Ready(Some(Msg::Tick)))
        } else {
            Ok(Async::NotReady)
        }
    }
}

struct Driver {
    handle: Handle,
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
    fn new(setts: Settings, disc: Box<Discovery>, sender: Sender<Msg>, handle: Handle) -> Driver {
        Driver {
            handle: handle,
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

    fn start(&mut self, timer: &Timer) {
        self.attempt_opt = Some(Attempt::new());
        self.state       = ConnectionState::Connecting;
        self.phase       = Phase::Reconnecting;

        let tx          = self.sender.clone();
        let tick_period = Duration::milliseconds(200);
        let mut state   = get_time();
        let     tick    = TickRate::new(tick_period);

        let tick = tick.fold(self.sender.clone(), |sender, msg| {
            sender.send(msg).map_err(|_| ())
        });

        self.handle.spawn(tick.then(|_| Ok(())));

        self.discover();
    }

    fn discover(&mut self) {
        if self.state == ConnectionState::Connecting && self.phase == Phase::Reconnecting {
            let endpoint = self.discovery.discover(self.last_endpoint.as_ref());

            self.phase = Phase::EndpointDiscovery;

            // TODO - Properly handle endpoint discovery asynchronously.
            self.handle.spawn(
                self.sender.clone().send(Msg::Establish(endpoint)).then(|_| Ok(())));

            self.tracker.reset();
        }
    }

    fn on_establish(&mut self, endpoint: Endpoint) {
        if self.state == ConnectionState::Connecting && self.phase == Phase::EndpointDiscovery {
            self.phase         = Phase::Establishing;
            self.candidate     = Some(Connection::new(self.sender.clone(), endpoint.addr, self.handle.clone()));
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
                println!("Connection established: {}.", id);
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
            println!("CloseConnection: {}.", error);
            self.tcp_connection_close(&conn_id, error);
        }
    }

    fn tcp_connection_close(&mut self, conn_id: &Uuid, err: Error) {
        println!("Connection [{}] error. Cause: {}.", conn_id, err);

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
            if let Some(ref conn) = self.candidate {
                println!("Connection identified: {}.", conn.id);
            }

            self.init_req_opt         = None;
            self.attempt_opt          = None;
            self.last_operation_check = get_time();
            self.state                = ConnectionState::Connected;
        } else if (pkg.cmd == Cmd::Authenticated || pkg.cmd == Cmd::NotAuthenticated) && self.state == ConnectionState::Connecting && self.phase == Phase::Authentication {
            if pkg.cmd == Cmd::NotAuthenticated {
                println!("Not authenticated.");
            }

            self.identify_client();
        } else {
            if self.state == ConnectionState::Connected {
                match pkg.cmd {
                    Cmd::HeartbeatRequest => {
                        let mut resp = pkg.copy_headers_only();

                        resp.cmd = Cmd::HeartbeatResponse;

                        if let Some(ref conn) = self.candidate {
                            conn.enqueue(resp);
                        }
                    },

                    Cmd::HeartbeatResponse => (),

                    _ => {
                        if self.registry.handle(&pkg) {
                            println!("Package [{}] received: command [{}].",
                                pkg.correlation, pkg.cmd.to_u8())
                        } else {
                            println!("Package [{}] not handled: command [{}].",
                                pkg.correlation, pkg.cmd.to_u8())
                        }
                    },
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
            if let Some(conn) = self.candidate.take() {
                self.tcp_connection_close(&conn.id, heartbeat_timeout_error());
            }
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
                        self.discover();

                        Report::Continue
                    } else {
                        Report::Quit
                    }
                } else {
                    Report::Continue
                }
            } else if self.phase == Phase::Authentication {
                if self.has_init_req_timeout(&now) {
                    println!("Authentication has timeout.");

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

pub struct Client {
    worker: JoinHandle<()>,
    sender: Sender<Msg>,
}

impl Client {
    pub fn new(settings: Settings, addr: SocketAddrV4) -> Client {
        let (sender, recv) = channel(500);
        let disc           = Box::new(StaticDiscovery::new(addr));

        let tx     = sender.clone();
        let handle = spawn(move || {
            let mut core   = Core::new().unwrap();
            let     handle = core.handle();

            let mut driver = Driver::new(settings, disc, sender, handle.clone());
            let     timer  = Timer::new();
            let mut ticker = None;

            let worker = recv.for_each(move |msg| {
                match msg {
                    Msg::Start => {
                        ticker = Some(driver.start(&timer));
                    },

                    Msg::Shutdown => {
                        println!("Shutting down...");
                        return Err(());
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
                            return Err(())
                        }
                    },
                };

                Ok(())
            });

            core.run(worker).unwrap();
        });

        Client {
            worker: handle,
            sender: tx,
        }
    }

    pub fn start(&self) {
        self.sender.clone().send(Msg::Start).wait().unwrap();
    }

    pub fn shutdown(&self) {
        self.sender.clone().send(Msg::Shutdown).wait().unwrap();
    }

    pub fn wait_till_closed(self) {
        self.worker.join().unwrap();
    }
}

fn heartbeat_timeout_error() -> Error {
    Error::new(ErrorKind::Other, "Heartbeat timeout error.")
}
