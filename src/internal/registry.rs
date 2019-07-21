use std::collections::{ HashMap, HashSet };
use std::iter::IntoIterator;

use bytes::BytesMut;
use uuid::Uuid;

use crate::internal::command::Cmd;
use crate::internal::connection::Connection;
use crate::internal::messages;
use crate::internal::operations::{ OperationError, OperationWrapper, OperationId, Tracking, Session };
use crate::internal::package::Pkg;

#[derive(Copy, Clone)]
struct Request {
    session: OperationId,
    tracker: Tracking,
}

impl Request {
    fn new(session: OperationId, cmd: Cmd, conn_id: Uuid) -> Request {
        Request {
            session,
            tracker: Tracking::new(cmd, conn_id),
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
    runnings: &'a mut HashSet<Uuid>,
}

impl<'a> SessionImpl<'a> {
    fn new(
        id: OperationId,
        assocs: &'a mut HashMap<Uuid, Request>,
        conn: &'a Connection,
        runnings: &'a mut HashSet<Uuid>) -> SessionImpl<'a>
    {
        SessionImpl {
            id,
            assocs,
            conn,
            runnings,
        }
    }
}

fn terminate<I>(assocs: &mut HashMap<Uuid, Request>, runnings: I)
    where
        I: IntoIterator<Item=Uuid>
{
    for id in runnings {
        assocs.remove(&id);
    }
}

impl<'a> Session for SessionImpl<'a> {
    fn new_request(&mut self, cmd: Cmd) -> Uuid {
        let req = Request::new(self.id, cmd, self.conn.id);
        let id  = req.get_id();

        self.assocs.insert(id, req);
        self.runnings.insert(id);

        id
    }

    fn pop(&mut self, id: &Uuid) -> ::std::io::Result<Tracking> {
        match self.assocs.remove(id) {
            Some(req) => {
                self.runnings.remove(id);

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
            tracker,
        };

        self.runnings.insert(id);
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
        terminate(self.assocs, self.runnings.drain());
    }

    fn connection_id(&self) -> Uuid {
        self.conn.id
    }

    fn has_running_requests(&self) -> bool {
        !self.runnings.is_empty()
    }
}

struct Requests {
    sessions: HashMap<OperationId, OperationWrapper>,
    session_request_ids: HashMap<OperationId, HashSet<Uuid>>,
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
        use std::mem;

        let mut requests = mem::replace(&mut self.assocs, HashMap::new());
        let mut runnings = HashSet::new();
        let session = SessionImpl::new(op.id, &mut requests, conn, &mut runnings);

        match op.send(&mut self.buffer, session).map(|out| out.produced_pkgs()) {
            Ok(pkgs) => {
                conn.enqueue_all(pkgs);

                self.session_request_ids.insert(op.id, runnings);
                self.sessions.insert(op.id, op);
            },

            Err(e) => {
                error!("Exception occured when issuing requests: {}", e);

                terminate(&mut self.assocs, runnings);
            },
        }

        mem::replace(&mut self.assocs, requests);
    }

    fn handle_pkg(&mut self, conn: &Connection, pkg: Pkg) {
        use std::mem;

        struct Resp {
            operation: OperationWrapper,
            request: Request,
            runnings: HashSet<Uuid>,
        }

        enum Out {
            Failed,
            Handled,
        }

        let mut sessions = mem::replace(&mut self.sessions, HashMap::new());
        let mut sessions_requests = mem::replace(&mut self.session_request_ids, HashMap::new());
        let mut requests = mem::replace(&mut self.assocs, HashMap::new());

        let extract_resp = requests.get(&pkg.correlation).copied().and_then(|request|
        {
            sessions.remove(&request.session).and_then(|operation|
            {
                sessions_requests.remove(&request.session).map(|runnings|
                {
                    Resp {
                        operation,
                        request,
                        runnings,
                    }
                })
            })
        });

        let pkg_id  = pkg.correlation;
        let pkg_cmd = pkg.cmd;

        if let Some(mut resp) = extract_resp {
            let original_cmd = resp.request.tracker.get_cmd();
            let session_id   = resp.request.session;

            debug!("Package [{}]: command {:?} received {:?}.", pkg_id, original_cmd, pkg_cmd);

            let out = {
                let mut session =
                    SessionImpl::new(
                        session_id, &mut requests, conn, &mut resp.runnings);

                match pkg.cmd {
                    Cmd::BadRequest => {
                        let msg = pkg.build_text();

                        error!("Bad request for command {:?}: {}.", original_cmd, msg);

                        resp.operation.failed(OperationError::ServerError(Some(msg)));

                        Out::Failed
                    },

                    Cmd::NotAuthenticated => {
                        error!("Not authenticated for command {:?}.", original_cmd);

                        resp.operation.failed(OperationError::AuthenticationRequired);

                        Out::Failed
                    },

                    Cmd::NotHandled => {
                        warn!("Not handled request {:?} id {}.", original_cmd, pkg_id);

                        let msg: ::std::io::Result<messages::NotHandled> =
                            pkg.to_message();

                        match msg {
                            Ok(not_handled) => {
                                match not_handled.get_reason() {
                                    messages::NotHandled_NotHandledReason::NotMaster => {
                                        warn!("Received a non master error on command {:?} id {}.
                                              This driver doesn't support cluster connection yet.", original_cmd, pkg_id);

                                        resp.operation.failed(OperationError::NotImplemented);

                                        Out::Failed
                                    },

                                    _ => {
                                        warn!("The server has either not started or is too busy.
                                              Retrying command {:?} id {}.", original_cmd, pkg_id);

                                        match resp.operation.retry(&mut self.buffer, &mut session, pkg_id) {
                                            Ok(outcome) => {
                                                let pkgs = outcome.produced_pkgs();

                                                if !pkgs.is_empty() {
                                                    conn.enqueue_all(pkgs);
                                                }

                                                Out::Handled
                                            },

                                            Err(error) => {
                                                error!(
                                                    "An error occured when retrying command {:?} id {}: {}.",
                                                    original_cmd, pkg_id, error
                                                );

                                                Out::Failed
                                            },
                                        }
                                    },
                                }
                            },

                            Err(error) => {
                                error!("Decoding error: can't decode NotHandled message: {}.", error);

                                Out::Failed
                            },
                        }
                    },

                    _ => match resp.operation.receive(&mut self.buffer, session, pkg) {
                        Ok(outcome) => {
                            let pkgs = outcome.produced_pkgs();

                            if !pkgs.is_empty() {
                                conn.enqueue_all(pkgs);
                            }

                            Out::Handled
                        },

                        Err(e) => {
                            error!("An error occured when running operation: {}", e);
                            let msg = format!("Exception raised: {}", e);

                            resp.operation.failed(OperationError::InvalidOperation(msg));

                            Out::Failed
                        },
                    },
                }
            };

            if let Out::Failed = out {
                terminate(&mut requests, resp.runnings.drain());
            }

            if !resp.runnings.is_empty() {
                sessions.insert(session_id, resp.operation);
                sessions_requests.insert(session_id, resp.runnings);
            }
        } else {
            warn!("Package [{}] not handled: cmd {:?}.", pkg_id, pkg_cmd);
        }

        mem::replace(&mut self.sessions, sessions);
        mem::replace(&mut self.session_request_ids, sessions_requests);
        mem::replace(&mut self.assocs, requests);
    }

    fn check_and_retry(&mut self, conn: &Connection) {
        use std::mem;

        let mut sessions = mem::replace(&mut self.sessions, HashMap::new());
        let mut sessions_requests = mem::replace(&mut self.session_request_ids, HashMap::new());
        let mut requests = mem::replace(&mut self.assocs, HashMap::new());

        sessions.retain(|op_id, op|
        {
            if let Some(mut runnings) = sessions_requests.remove(&op_id) {
                let result = {
                    let session = SessionImpl::new(
                        op.id, &mut requests, conn, &mut runnings);

                    op.check_and_retry(&mut self.buffer, session)
                };

                match result {
                    Ok(outcome) => {
                        if outcome.is_done() {
                            for id in runnings.drain() {
                                requests.remove(&id);
                            }

                            return false;
                        }

                        let pkgs = outcome.produced_pkgs();

                        if !pkgs.is_empty() {
                            conn.enqueue_all(pkgs);
                        }

                        sessions_requests.insert(*op_id, runnings);

                        return true;
                    },

                    Err(e) => {
                        error!("Exception raised when checking out operation {:?}: {}", op_id, e);

                        let msg = format!("Exception raised: {}", e);

                        op.failed(OperationError::InvalidOperation(msg));

                        for id in runnings.drain() {
                            requests.remove(&id);
                        }

                        return false;
                    },
                }
            }

            warn!("No running requests associated to session {:?}. It means we didn't clean
                  the session up. Session disposed.", op_id);

            false
        });

        mem::replace(&mut self.sessions, sessions);
        mem::replace(&mut self.session_request_ids, sessions_requests);
        mem::replace(&mut self.assocs, requests);
    }

    pub(crate) fn abort(&mut self) {
        for op in self.sessions.values_mut() {
            op.failed(OperationError::Aborted);
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
        debug!("Enter check_and_retry process…");

        self.requests.check_and_retry(conn);

        while let Some(op) = self.awaiting.pop() {
            self.register(op, Some(conn));
        }

        debug!("check_and_retry process completed.");
    }

    pub(crate) fn abort(&mut self) {
        self.requests.abort();

        for op in self.awaiting.iter_mut() {
            op.failed(OperationError::Aborted);
        }
    }
}
