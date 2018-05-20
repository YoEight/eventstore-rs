use std::collections::HashMap;

use bytes::BytesMut;
use uuid::Uuid;

use internal::command::Cmd;
use internal::connection::Connection;
use internal::operations::{ OperationError, OperationWrapper, OperationId, Tracking, Session };
use internal::package::Pkg;

struct Checking {
    key: Uuid,
    is_checking: bool,
}

impl Checking {
    fn delete(id: Uuid) -> Checking {
        Checking {
            key: id,
            is_checking: false,
        }
    }

    fn check(id: Uuid) -> Checking {
        Checking {
            key: id,
            is_checking: true,
        }
    }
}

struct Request {
    session: OperationId,
    conn_id: Uuid,
    tracker: Tracking,
}

impl Request {
    fn new(session: OperationId, cmd: Cmd, conn_id: Uuid) -> Request {
        Request {
            session,
            conn_id,
            tracker: Tracking::new(cmd),
        }
    }

    fn get_id(&self) -> Uuid {
        self.tracker.get_id()
    }
}

struct SessionImpl<'a> {
    id: OperationId,
    assocs: &'a mut HashMap<Uuid, Request>,
    conn: &'a Connection,
    runnings: &'a mut Vec<Uuid>,
}

impl<'a> SessionImpl<'a> {
    fn new(
        id: OperationId,
        assocs: &'a mut HashMap<Uuid, Request>,
        conn: &'a Connection,
        runnings: &'a mut Vec<Uuid>) -> SessionImpl<'a>
    {
        SessionImpl {
            id,
            assocs,
            conn,
            runnings,
        }
    }
}

fn terminate(assocs: &mut HashMap<Uuid, Request>, runnings: Vec<Uuid>) {
    for id in runnings {
        assocs.remove(&id);
    }
}

impl<'a> Session for SessionImpl<'a> {
    fn new_request(&mut self, cmd: Cmd) -> Uuid {
        let req = Request::new(self.id, cmd, self.conn.id);
        let id  = req.get_id();

        self.assocs.insert(id, req);
        self.runnings.push(id);

        id
    }

    fn pop(&mut self, id: &Uuid) -> ::std::io::Result<Tracking> {
        match self.assocs.remove(id) {
            Some(req) => {
                let pos = self.runnings
                              .iter()
                              .position(|x| x == id).unwrap();

                self.runnings.remove(pos);

                Ok(req.tracker)
            },

            None => {
                let error = ::std::io::Error::new(::std::io::ErrorKind::Other, "Tracker must exists");
                Err(error)
            },
        }
    }

    fn reuse(&mut self, tracker: Tracking) {
        let id  = tracker.get_id();
        let req = Request {
            session: self.id,
            conn_id: self.conn.id,
            tracker,
        };

        self.runnings.push(id);
        self.assocs.insert(id, req);
    }

    fn using(&mut self, id: &Uuid) -> ::std::io::Result<&mut Tracking> {
        match self.assocs.get_mut(id) {
            Some(req) => Ok(&mut req.tracker),
            None      => {
                let error = ::std::io::Error::new(::std::io::ErrorKind::Other, "Tracker must exists");
                Err(error)
            },
        }
    }

    fn requests(&self) -> Vec<&Tracking> {
        self.assocs.values().map(|req| &req.tracker).collect()
    }

    fn terminate(&mut self) {
        terminate(self.assocs, self.runnings.drain(..).collect());
    }
}

struct Requests {
    sessions: HashMap<OperationId, OperationWrapper>,
    session_request_ids: HashMap<OperationId, Vec<Uuid>>,
    assocs: HashMap<Uuid, Request>,
    buffer: BytesMut,
}

impl Requests {
    fn new() -> Requests {
        Requests {
            sessions: HashMap::new(),
            session_request_ids: HashMap::new(),
            assocs: HashMap::new(),
            buffer: BytesMut::new(),
        }
    }

    fn register(&mut self, conn: &Connection, mut op: OperationWrapper) {
        let mut runnings = Vec::new();
        let     success = {
            let session = SessionImpl::new(op.id, &mut self.assocs, conn, &mut runnings);

            match op.send(&mut self.buffer, session).map(|out| out.produced_pkgs()) {
                Ok(pkgs) => {
                    conn.enqueue_all(pkgs);

                    true
                },

                Err(e) => {
                    error!("Exception occured when issuing requests: {}", e);

                    false
                },
            }
        };

        if !success {
            terminate(&mut self.assocs, runnings);
        } else {
            self.session_request_ids.insert(op.id, runnings);
            self.sessions.insert(op.id, op);
        }
    }

    fn handle_pkg(&mut self, conn: &Connection, pkg: Pkg) {
        let pkg_id  = pkg.correlation;
        let pkg_cmd = pkg.cmd;

        if let Some(req) = self.assocs.remove(&pkg_id) {
            debug!("Package [{}] received: command {:?}.", pkg_id, pkg_cmd.to_u8());

            let session_id   = req.session;
            let session_over = {
                let runnings = self.session_request_ids
                                   .get_mut(&req.session)
                                   .expect("No session associated to request!");

                let op = self.sessions
                             .get_mut(&session_id)
                             .expect("Unknown session!");

                // We insert back the request to let the operation decides if it
                // want to keep it.
                self.assocs.insert(pkg_id, req);

                let errored = {
                    let session =
                        SessionImpl::new(
                            session_id, &mut self.assocs, conn, runnings);

                    match op.receive(&mut self.buffer, session, pkg) {
                        Ok(outcome) => {
                            let pkgs = outcome.produced_pkgs();

                            if !pkgs.is_empty() {
                                conn.enqueue_all(pkgs);
                            }

                            None
                        },

                        Err(e) => {
                            error!("An error occured when running operation: {}", e);
                            let msg = format!("Exception raised: {}", e);

                            Some(msg)
                        },
                    }
                };

                if let Some(msg) = errored {
                    op.failed(OperationError::InvalidOperation(msg));
                    terminate(&mut self.assocs, runnings.drain(..).collect());
                }

                runnings.is_empty()
            };

            if session_over {
                self.sessions.remove(&session_id);
                self.session_request_ids.remove(&session_id);
            }
        } else {
            warn!("Package [{}] not handled: cmd {:?}.", pkg_id, pkg_cmd);
        }
    }

    fn check_and_retry(&mut self, conn: &Connection) {
        let mut process_later = Vec::new();

        for (key, req) in &self.assocs {
            if req.conn_id != conn.id {
                process_later.push(Checking::delete(*key));
            } else {
                process_later.push(Checking::check(*key));
            }
        }

        for status in process_later {
            if let Some(mut req) = self.assocs.remove(&status.key) {

                if status.is_checking {
                    let terminated_requests = {
                        let op = self.sessions
                                     .get_mut(&req.session)
                                     .expect("No session associated to request");

                        let runnings = self.session_request_ids
                                           .get_mut(&req.session)
                                           .expect("No session attached to request");

                        let result = {
                            let session =
                                SessionImpl::new(
                                    req.session, &mut self.assocs, conn, runnings);

                            op.check_and_retry(&mut self.buffer, session)
                        };

                        match result {
                            Ok(outcome) => {
                                if outcome.is_done() {
                                    runnings.drain(..).collect()
                                } else {
                                    let pkgs = outcome.produced_pkgs();

                                    if !pkgs.is_empty() {
                                        conn.enqueue_all(pkgs);
                                    }

                                    Vec::new()
                                }
                            },

                            Err(e) => {
                                error!("Exception raised when checking out operation: {}", e);
                                let msg = format!("Exception raised: {}", e);

                                op.failed(OperationError::InvalidOperation(msg));
                                runnings.drain(..).collect()
                            },
                        }
                    };

                    if !terminated_requests.is_empty() {
                        terminate(&mut self.assocs, terminated_requests);
                        self.sessions.remove(&req.session);
                        self.session_request_ids.remove(&req.session);
                    }
                } else {
                    let mut op = self.sessions
                                     .remove(&req.session)
                                     .expect("No session associated with request!");

                    let runnings = self.session_request_ids
                                       .remove(&req.session)
                                       .expect("No session attached to request");

                    op.failed(OperationError::Aborted);
                    terminate(&mut self.assocs, runnings);
                }
            }
        }
    }
}

pub(crate) struct Registry {
    requests: Requests,
    awaiting: Vec<OperationWrapper>,
}

impl Registry {
    pub(crate) fn new() -> Registry {
        Registry {
            requests: Requests::new(),
            awaiting: Vec::new(),
        }
    }

    pub(crate) fn register(&mut self, op: OperationWrapper, conn: Option<&Connection>) {
        match conn {
            None       => self.awaiting.push(op),
            Some(conn) => self.requests.register(conn, op),
        }
    }

    pub(crate) fn handle(&mut self, pkg: Pkg, conn: &Connection) {
        self.requests.handle_pkg(conn, pkg);
    }

    pub(crate) fn check_and_retry(&mut self, conn: &Connection) {
        self.requests.check_and_retry(conn);

        while let Some(op) = self.awaiting.pop() {
            self.register(op, Some(conn));
        }
    }
}
