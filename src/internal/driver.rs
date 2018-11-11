use core::option::Option;
use std::io::{ Error, ErrorKind };
use std::time::{ Duration, Instant };

use futures::{ Future, Stream, Sink };
use futures::sync::mpsc::Sender;
use tokio::spawn;
use tokio::timer::Interval;
use uuid::Uuid;

use internal::discovery::{ Endpoint, Discovery };
use internal::command::Cmd;
use internal::connection::Connection;
use internal::messaging::Msg;
use internal::operations::OperationWrapper;
use internal::package::Pkg;
use internal::registry::Registry;
use types::{ Credentials, Settings };

#[derive(Copy, Clone)]
enum HeartbeatStatus {
    Init,
    Delay(u32, Instant),
    Timeout(u32, Instant),
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
                self.state = HeartbeatStatus::Delay(
                        self.pkg_num, Instant::now());

                Heartbeat::Valid
            },

            HeartbeatStatus::Delay(num, start) => {

                if self.pkg_num != num {
                    self.state = HeartbeatStatus::Delay(
                        self.pkg_num, Instant::now());
                } else {
                    if start.elapsed() >= self.heartbeat_delay {
                        self.state = HeartbeatStatus::Timeout(
                            self.pkg_num, Instant::now());

                        conn.enqueue(Pkg::heartbeat_request());
                    }
                }

                Heartbeat::Valid
            },

            HeartbeatStatus::Timeout(num, start) => {

                if self.pkg_num != num {
                    self.state = HeartbeatStatus::Delay(
                        self.pkg_num, Instant::now());

                    Heartbeat::Valid
                } else {
                    if start.elapsed() >= self.heartbeat_timeout {
                        error!("Closing connection [{}] due to HEARTBEAT TIMEOUT at pkgNum {}.", conn.id, self.pkg_num);

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

impl ConnectionState {
    fn is_connected(&self) -> bool {
        *self == ConnectionState::Connected
    }
}

struct InitReq {
    correlation: Uuid,
    started: Instant,
}

impl InitReq {
    fn new(id: Uuid) -> InitReq {
        InitReq {
            correlation: id,
            started: Instant::now(),
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
    started: Instant,
    tries: usize,
}

impl Attempt {
    fn new() -> Attempt {
        Attempt {
            started: Instant::now(),
            tries: 0,
        }
    }
}

#[derive(PartialEq, Eq)]
pub(crate) enum Report {
    Continue,
    Quit,
}

pub(crate) struct Driver {
    registry: Registry,
    candidate: Option<Connection>,
    tracker: HealthTracker,
    attempt_opt: Option<Attempt>,
    state: ConnectionState,
    phase: Phase,
    last_endpoint: Option<Endpoint>,
    discovery: Box<Discovery + Send>,
    connection_name: Option<String>,
    default_user: Option<Credentials>,
    operation_timeout: Duration,
    init_req_opt: Option<InitReq>,
    reconnect_delay: Duration,
    max_reconnect: usize,
    sender: Sender<Msg>,
    operation_check_period: Duration,
    last_operation_check: Instant,
}

impl Driver {
    pub(crate) fn new(setts: &Settings, disc: Box<Discovery + Send>, sender: Sender<Msg>) -> Driver {
        Driver {
            registry: Registry::new(),
            candidate: None,
            tracker: HealthTracker::new(&setts),
            attempt_opt: None,
            state: ConnectionState::Init,
            phase: Phase::Reconnecting,
            last_endpoint: None,
            discovery: disc,
            connection_name: setts.connection_name.clone(),
            default_user: setts.default_user.clone(),
            operation_timeout: setts.operation_timeout,
            init_req_opt: None,
            reconnect_delay: Duration::from_secs(3),
            max_reconnect: setts.connection_retry.to_usize(),
            sender: sender,
            operation_check_period: setts.operation_check_period,
            last_operation_check: Instant::now(),
        }
    }

    pub(crate) fn start(&mut self) {
        self.attempt_opt = Some(Attempt::new());
        self.state       = ConnectionState::Connecting;
        self.phase       = Phase::Reconnecting;

        let tick_period = Duration::from_millis(200);
        let tick        = Interval::new(Instant::now(), tick_period).map_err(|_| ());

        let tick = tick.fold(self.sender.clone(), |sender, _| {
            sender.send(Msg::Tick).map_err(|_| ())
        });

        spawn(tick.then(|_| Ok(())));

        self.discover();
    }

    fn discover(&mut self) {
        if self.state == ConnectionState::Connecting && self.phase == Phase::Reconnecting {
            let endpoint = self.discovery.discover(self.last_endpoint.as_ref());

            self.phase = Phase::EndpointDiscovery;

            // TODO - Properly handle endpoint discovery asynchronously.
            spawn(
                self.sender.clone().send(Msg::Establish(endpoint)).then(|_| Ok(())));

            self.tracker.reset();
        }
    }

    pub(crate) fn on_establish(&mut self, endpoint: Endpoint) {
        if self.state == ConnectionState::Connecting && self.phase == Phase::EndpointDiscovery {
            self.phase         = Phase::Establishing;
            self.candidate     = Some(Connection::new(self.sender.clone(), endpoint.addr));
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

    pub(crate) fn on_established(&mut self, id: Uuid) {
        if self.state == ConnectionState::Connecting && self.phase == Phase::Establishing {
            let same_connection =
                match self.candidate {
                    Some(ref conn) => conn.id == id,
                    None           => false,
                };

            if same_connection {
                info!("Connection established: {}.", id);
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

    pub(crate) fn on_connection_closed(&mut self, conn_id: Uuid, error: Error) {
        if self.is_same_connection(&conn_id) {
            info!("CloseConnection: {}.", error);
            self.tcp_connection_close(&conn_id, error);
        }
    }

    fn tcp_connection_close(&mut self, conn_id: &Uuid, err: Error) {
        info!("Connection [{}] error. Cause: {}.", conn_id, err);

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

    pub(crate) fn on_package_arrived(&mut self, pkg: Pkg) {
        self.tracker.incr_pkg_num();

        if pkg.cmd == Cmd::ClientIdentified && self.state == ConnectionState::Connecting && self.phase == Phase::Identification {
            if let Some(req) = self.init_req_opt.take() {
                if req.correlation == pkg.correlation {
                    if let Some(ref conn) = self.candidate {
                        info!("Connection identified: {}.", conn.id);
                    }

                    self.attempt_opt          = None;
                    self.last_operation_check = Instant::now();
                    self.state                = ConnectionState::Connected;
                }
            }
        } else if (pkg.cmd == Cmd::Authenticated || pkg.cmd == Cmd::NotAuthenticated) && self.state == ConnectionState::Connecting && self.phase == Phase::Authentication {
            if let Some(req) = self.init_req_opt.take(){
                if req.correlation == pkg.correlation {
                    if pkg.cmd == Cmd::NotAuthenticated {
                        warn!("Not authenticated.");
                    }

                    self.identify_client();
                }
            }
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
                        // It will be always 'Some' when receiving a package.
                        if let Some(ref conn) = self.candidate {
                            self.registry.handle(pkg, conn);
                        }
                    },
                }
            }
        }
    }

    pub(crate) fn on_new_op(&mut self, operation: OperationWrapper) {
        let conn_opt = {
            if self.state.is_connected() {
                // Will be always 'Some' when connected.
                self.candidate.as_ref()
            } else {
                None
            }
        };

        self.registry.register(operation, conn_opt);
    }

    fn has_init_req_timeout(&self) -> bool {
        if let Some(ref req) = self.init_req_opt {
            req.started.elapsed() >= self.operation_timeout
        } else {
            false
        }
    }

    fn conn_has_timeout(&self) -> bool {
        if let Some(att) = self.attempt_opt.as_ref() {
            att.started.elapsed() >= self.reconnect_delay
        } else {
            false
        }
    }

    fn start_new_attempt(&mut self) -> Option<usize> {
        if let Some(att) = self.attempt_opt.as_mut() {
            att.tries   += 1;
            att.started = Instant::now();

            if att.tries <= self.max_reconnect {
                Some(att.tries)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub(crate) fn close_connection(&mut self) {
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

    pub(crate) fn on_tick(&mut self) -> Report {

        if self.state == ConnectionState::Init || self.state == ConnectionState::Closed {
            return Report::Continue;
        }

        if self.state == ConnectionState::Connecting {
            if self.phase == Phase::Reconnecting {
                if self.conn_has_timeout() {
                    if let Some(attempt_cnt) = self.start_new_attempt() {
                        info!("Starting new connection attempt ({}).", attempt_cnt);

                        self.discover();
                    } else {
                        error!("Maximum reconnection attempt count reached!");
                        return Report::Quit;
                    }
                }
            } else if self.phase == Phase::Authentication {
                if self.has_init_req_timeout() {
                    warn!("Authentication has timeout.");

                    self.identify_client();
                }

                self.manage_heartbeat();
            } else if self.phase == Phase::Identification {
                if self.has_init_req_timeout() {
                    return Report::Quit;
                } else {
                    self.manage_heartbeat();
                }
            }
        } else {
            // Connected state
            if let Some(ref conn) = self.candidate {
                if self.last_operation_check.elapsed() >= self.operation_check_period {
                    self.registry.check_and_retry(conn);

                    self.last_operation_check = Instant::now();
                }
            }

            self.manage_heartbeat();
        }

        Report::Continue
    }

    pub(crate) fn on_send_pkg(&mut self, pkg: Pkg) {
        if self.state == ConnectionState::Connected {
            if let Some(ref conn) = self.candidate {
                conn.enqueue(pkg);
            }
        }
    }

    pub(crate) fn abort(&mut self) {
        self.registry.abort();
    }
}

fn heartbeat_timeout_error() -> Error {
    Error::new(ErrorKind::Other, "Heartbeat timeout error.")
}
