#[derive(Copy, Clone, Debug)]
pub enum Cmd {
    HeartbeatRequest,
    HeartbeatResponse,
    IdentifyClient,
    ClientIdentified,
    Authenticate,
    Authenticated,
    NotAuthenticated,
    WriteEvents,
    WriteEventsCompleted,
    Unknown(u8),
}

impl PartialEq for Cmd {
    fn eq(&self, other: &Cmd) -> bool {
        self.to_u8() == other.to_u8()
    }
}

impl Eq for Cmd {}

impl Cmd {
    pub fn to_u8(&self) -> u8 {
        match *self {
            Cmd::HeartbeatRequest     => 0x01,
            Cmd::HeartbeatResponse    => 0x02,
            Cmd::IdentifyClient       => 0xF5,
            Cmd::ClientIdentified     => 0xF6,
            Cmd::Authenticate         => 0xF2,
            Cmd::Authenticated        => 0xF3,
            Cmd::NotAuthenticated     => 0xF4,
            Cmd::WriteEvents          => 0x82,
            Cmd::WriteEventsCompleted => 0x83,
            Cmd::Unknown(cmd)         => cmd,
        }
    }

    pub fn from_u8(cmd: u8) -> Cmd {
        match cmd {
            0x01 => Cmd::HeartbeatRequest,
            0x02 => Cmd::HeartbeatResponse,
            0xF5 => Cmd::IdentifyClient,
            0xF6 => Cmd::ClientIdentified,
            0xF2 => Cmd::Authenticate,
            0xF3 => Cmd::Authenticated,
            0xF4 => Cmd::NotAuthenticated,
            0x82 => Cmd::WriteEvents,
            0x83 => Cmd::WriteEventsCompleted,
            _    => Cmd::Unknown(cmd),
        }
    }
}
