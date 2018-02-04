#[derive(Copy, Clone)]
pub enum Cmd {
    HeartbeatRequest,
    HeartbeatResponse,
    WriteEvents,
    WriteEventsCompleted,
    Unknown(u8),
}

impl Cmd {
    pub fn to_u8(self) -> u8 {
        match self {
            Cmd::HeartbeatRequest     => 0x01,
            Cmd::HeartbeatResponse    => 0x02,
            Cmd::WriteEvents          => 0x82,
            Cmd::WriteEventsCompleted => 0x83,
            Cmd::Unknown(cmd)         => cmd,
        }
    }

    pub fn from_u8(cmd: u8) -> Cmd {
        match cmd {
            0x01 => Cmd::HeartbeatRequest,
            0x02 => Cmd::HeartbeatResponse,
            0x82 => Cmd::WriteEvents,
            0x83 => Cmd::WriteEventsCompleted,
            _    => Cmd::Unknown(cmd),
        }
    }
}
