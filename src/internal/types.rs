use time::Duration;

#[derive(Copy, Clone)]
pub enum Retry {
    Undefinately,
    Only(u32),
}

impl Retry {
    pub fn to_u32(&self) -> u32 {
        match *self {
            Retry::Undefinately => u32::max_value(),
            Retry::Only(x)      => x,
        }
    }
}

#[derive(Clone)]
pub struct Credentials {
    pub login: String,
    pub password: String,
}

pub struct Settings {
    pub heartbeat_delay: Duration,
    pub heartbeat_timeout: Duration,
    pub operation_timeout: Duration,
    pub operation_retry: Retry,
    pub connection_retry: Retry,
    pub default_user: Option<Credentials>,
    pub connection_name: Option<String>,
    pub operation_check_period: Duration,
}

impl Settings {
    pub fn default() -> Settings {
        Settings {
            heartbeat_delay: Duration::milliseconds(750),
            heartbeat_timeout: Duration::milliseconds(1500),
            operation_timeout: Duration::seconds(7),
            operation_retry: Retry::Only(3),
            connection_retry: Retry::Only(3),
            default_user: None,
            connection_name: None,
            operation_check_period: Duration::seconds(1),
        }
    }
}

pub enum ExpectedVersion {
    Any,
    StreamExists,
    NoStream,
    Exact(i64),
}

impl ExpectedVersion {
    pub fn to_i64(self) -> i64 {
        match self {
            ExpectedVersion::Any          => -2,
            ExpectedVersion::StreamExists => -4,
            ExpectedVersion::NoStream     => -1,
            ExpectedVersion::Exact(n)     => n,
        }
    }

    pub fn from_i64(ver: i64) -> ExpectedVersion {
        match ver {
            -2 => ExpectedVersion::Any,
            -4 => ExpectedVersion::StreamExists,
            -1 => ExpectedVersion::NoStream,
            _  => ExpectedVersion::Exact(ver),
        }
    }
}

pub struct Position {
    pub commit:  i64,
    pub prepare: i64,
}

pub struct WriteResult {
    pub next_expected_version: i64,
    pub position: Position,
}
