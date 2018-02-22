use std::collections::HashMap;

use time::{ Duration, Timespec, get_time };

use uuid::Uuid;

use internal::connection::Connection;
use internal::operations::{ Operation, Decision };
use internal::package::Pkg;
use internal::types::{ Settings, Retry };

struct Register {
    started: Timespec,
    tries:   u32,
    op:      Box<Operation>,
}

pub enum Outcome {
    Handled,
    NotHandled,
}

enum Checking {
    Delete(Uuid),
    Retry(Uuid),
}

impl Register {
    fn new(op: Box<Operation>) -> Register {
        Register {
            started: get_time(),
            tries: 0,
            op: op,
        }
    }
}

pub struct Registry {
    awaiting: Vec<Box<Operation>>,
    pending: HashMap<Uuid, Register>,
    operation_timeout: Duration,
    operation_retry: Retry,
}

impl Registry {
    pub fn new(setts: &Settings) -> Registry {
        Registry {
            awaiting: Vec::new(),
            pending: HashMap::new(),
            operation_timeout: setts.operation_timeout,
            operation_retry: setts.operation_retry,
        }
    }

    pub fn register(&mut self, op: Box<Operation>, conn: Option<&Connection>) {
        match conn {
            None => self.awaiting.push(op),

            Some(conn) => {
                let reg = Register::new(op);

                self.send_register(conn, reg);
            }
        }
    }

    fn send_register(&mut self, conn: &Connection, mut reg: Register) {
        let correlation = Uuid::new_v4();
        let pkg         = reg.op.create(correlation);
        let now         = get_time();

        reg.tries  += 1;
        reg.started =  now;
        self.pending.insert(correlation, reg);
        conn.enqueue(pkg);
    }

    pub fn handle(&mut self, pkg: &Pkg) -> bool {
        if let Some(mut reg) = self.pending.remove(&pkg.correlation) {
            if let Decision::Continue = reg.op.inspect(pkg) {
                self.register(reg.op, None)
            }

            true
        } else {
            false
        }
    }

    pub fn check_and_retry(&mut self, conn: &Connection) {
        let mut to_process = Vec::new();

        while let Some(op) = self.awaiting.pop() {
            self.register(op, Some(conn));
        }

        let now = get_time();

        for (key, reg) in self.pending.iter() {
            if now - reg.started >= self.operation_timeout {
                match self.operation_retry {
                    Retry::Undefinately => {
                        to_process.push(Checking::Retry(*key));
                    }

                    Retry::Only(n) => {
                        if reg.tries + 1 > n {
                            to_process.push(Checking::Delete(*key));
                        } else {
                            to_process.push(Checking::Retry(*key));
                        }
                    },
                }
            }
        }

        for status in to_process {
            match status {
                Checking::Delete(key) => {
                    self.pending.remove(&key);
                },

                Checking::Retry(key) => {
                    if let Some(reg) = self.pending.remove(&key) {
                        self.send_register(conn, reg);
                    }
                },
            }
        }
    }
}
