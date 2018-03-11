use std::collections::HashMap;
use std::time::{ Duration, Instant };

use uuid::Uuid;

use internal::connection::Connection;
use internal::operations::{ Operation, OperationError };
use internal::package::Pkg;
use internal::types::{ Settings, Retry };

struct Register {
    // When the operation has started.
    started: Instant,

    // How many times, that operation has been tried so far.
    tries: u32,

    // Lasting session are meant for special operation like subscriptions which
    // usually keep the same correlation until the end of the universe, unless
    // if the user want to unsubscribe.
    lasting_session: bool,

    // When the operation has sent its request to server, indicates what was
    // the connection id. If it's 'None', it means the operation didn't sent
    // anything yet.
    conn_id: Option<Uuid>,

    // Actual operation state-machine.
    op: Box<Operation>,
}

pub enum Outcome {
    Handled,
    NotHandled,
}

struct Checking {
    key: Uuid,
    is_retry: bool,
}

impl Checking {
    fn delete(id: Uuid) -> Checking {
        Checking {
            key: id,
            is_retry: false,
        }
    }

    fn retry(id: Uuid) -> Checking {
        Checking {
            key: id,
            is_retry: true,
        }
    }
}

impl Register {
    fn new(op: Box<Operation>) -> Register {
        Register {
            started: Instant::now(),
            tries: 0,
            lasting_session: false,
            conn_id: None,
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
        // It's the first time this operation run, so we expect it to produce
        // a 'Pkg' so we can start a transaction with the server.
        match reg.op.poll(None) {
            Ok(outcome) => {
                if let Some(pkg) = outcome.produced_pkg() {
                    reg.tries  += 1;
                    reg.started = Instant::now();
                    reg.conn_id = Some(conn.id);
                    self.pending.insert(pkg.correlation, reg);
                    conn.enqueue(pkg);
                }
            },

            Err(e) => {
                println!("Something bad happened: {}", e);
            },
        }
    }

    pub fn handle(&mut self, pkg: &Pkg, conn: &Connection) -> bool {
        if let Some(mut reg) = self.pending.remove(&pkg.correlation) {

            // We notified the operation we receive some 'Pkg' from the server
            // that might interest it.
            match reg.op.poll(Some(pkg)) {
                Ok(outcome) => {
                    if outcome.is_continuing() {
                        if let Some(new) = outcome.produced_pkg() {
                            // This operation issued a new transaction request
                            // with the server, so we update its id in the
                            // registry.
                            self.pending.insert(new.correlation, reg);

                            // In case the operation was previously in a long
                            // term transaction.
                            reg.lasting_session = false;
                            reg.conn_id         = Some(conn.id);

                            conn.enqueue(new);
                        } else {
                            // This operation wants to keep its old correlation
                            // id, so we insert it back with its previous
                            // value.
                            self.pending.insert(pkg.correlation, reg);
                            reg.lasting_session = true;
                        }
                    }
                },

                Err(e) => {
                    let msg = format!("Exception raised: {}", e);

                    reg.op.failed(OperationError::InvalidOperation(msg));
                },
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

        for (key, reg) in self.pending.iter() {
            let not_same_conn = reg.conn_id != Some(conn.id);
            let has_timeout   = reg.started.elapsed() >= self.operation_timeout;

            if (has_timeout && !reg.lasting_session) || not_same_conn {
                // Basically, we have a subscription and the connection has
                // changed since the subscription has been confirmed.
                if reg.lasting_session {
                    to_process.push(Checking::delete(*key));
                    continue
                }

                match self.operation_retry {
                    Retry::Undefinately => {
                        to_process.push(Checking::retry(*key));
                    }

                    Retry::Only(n) => {
                        if reg.tries + 1 > n {
                            to_process.push(Checking::delete(*key));
                        } else {
                            to_process.push(Checking::retry(*key));
                        }
                    },
                }
            }
        }

        for status in to_process {
            if let Some(mut reg) = self.pending.remove(&status.key) {
                if status.is_retry {
                    self.send_register(conn, reg);
                } else {
                    reg.op.failed(OperationError::Aborted);
                }
            }
        }
    }
}
