use std::collections::HashMap;

use time::{ Duration, Timespec, get_time };

use uuid::Uuid;

use internal::connection::Connection;
use internal::operations::Operation;
use internal::types::{ Settings, Retry };

struct Register {
    started: Timespec,
    tries:   u32,
    op:      Box<Operation>,
}

impl Register {
    fn new(op: Box<Operation>) -> Register {
        Register {
            started: get_time(),
            tries: 0,
            op: op,
        }
    }

    fn elapsed(self) -> Duration {
        get_time() - self.started
    }
}

struct Registry {
    settings: Settings,
    awaiting: Vec<Box<Operation>>,
    pending: HashMap<Uuid, Register>,
}

impl Registry {
    pub fn new(settings: Settings) -> Registry {
        Registry {
            settings: settings,
            awaiting: Vec::new(),
            pending: HashMap::new(),
        }
    }

    pub fn register(&mut self, op: Box<Operation>, conn: Option<&Connection>) {
        match conn {
            None => self.awaiting.push(op),

            Some(conn) => {
                let     correlation = Uuid::new_v4();
                let     pkg         = op.create(correlation);
                let mut reg         = Register::new(op);

                reg.tries += 1;
                self.pending.insert(correlation, reg);
                conn.enqueue(pkg);
            }
        }
    }

    pub fn check_and_retry(&mut self, conn: &Connection) {
        // let mut to_retry  = vec![];
        let mut to_delete = vec![];

        while let Some(op) = self.awaiting.pop() {
            self.register(op, Some(conn));
        }

        let now = get_time();

        for (key, reg) in self.pending.iter_mut() {
            if now - reg.started >= self.settings.operation_timeout {
                match self.settings.operation_retry {
                    Retry::Undefinately => {
                        reg.tries   += 1;
                        reg.started = now;

                        to_delete.push(key);
                    }

                    Retry::Only(_) => (),
                }
            }
        }

        // Won't work because self.pending already mutable.
        // for key in to_delete {
        //     let _ = self.pending.remove(key);
        // }
    }
}