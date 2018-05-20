use std::collections::HashMap;

use bytes::BytesMut;
use uuid::Uuid;

use internal::connection::Connection;
use internal::operations::{ OperationError, OperationWrapper, OperationId, Outcome };
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

struct Session {
    op: OperationWrapper,
}

impl Session {
    fn new(op: OperationWrapper) -> Session {
        Session {
            op,
        }
    }

    fn issue_requests(&mut self, buffer: &mut BytesMut) -> ::std::io::Result<Vec<Pkg>> {
        self.op.poll(buffer, None).map(|outcome| outcome.produced_pkgs())
    }

    fn report_error(&mut self, error: OperationError) {
        self.op.failed(error);
    }

    fn accept_pkg(&mut self, buffer: &mut BytesMut, pkg: Pkg) -> ::std::io::Result<Outcome> {
        self.op.poll(buffer, Some(pkg))
    }
}

type SessionAssocs = HashMap<OperationId, Session>;
type Requests      = HashMap<Uuid, Request>;

struct Sessions {
    buffer: BytesMut,
    assocs: SessionAssocs,
    requests: Requests,
}

impl Sessions {
    fn new() -> Sessions {
        Sessions {
            buffer: BytesMut::new(),
            assocs: HashMap::new(),
            requests: HashMap::new(),
        }
    }

    fn insert_session(&mut self, session: Session) {
        self.assocs.insert(session.op.id, session);
    }

    fn send_pkgs(requests: &mut Requests, session: &mut Session, conn: &Connection, pkgs: Vec<Pkg>) {
        for pkg in &pkgs {
            let req = Request::new(session.op.id, conn);

            requests.insert(pkg.correlation, req);
            debug!("Request issued: {} cmd: {:?}", pkg.correlation, pkg.cmd);
        }

        conn.enqueue_all(pkgs)
    }

    fn get_session_mut<'a>(assocs: &'a mut SessionAssocs, session_id: &OperationId) -> &'a mut Session {
        assocs.get_mut(session_id).expect("Session must be defined at this point")
    }

    fn handle_request_response(&mut self, pkg: Pkg, conn: &Connection) {
        let pkg_id  = pkg.correlation;
        let pkg_cmd = pkg.cmd;

        if let Some(req) = self.requests.remove(&pkg.correlation) {
            debug!("Package [{}] received: command {:?}.", pkg_id, pkg_cmd.to_u8());

            let session_id = req.session;
            let session_completed = {
                let session = Sessions::get_session_mut(&mut self.assocs, &session_id);

                // We notified the operation we've received a 'Pkg' from the server
                // that might interest it.
                match session.accept_pkg(&mut self.buffer, pkg) {
                    Ok(outcome) => {
                        if outcome.is_continuing() {
                            let pkgs = outcome.produced_pkgs();

                            if pkgs.is_empty() {
                                debug!("Operation is continuing on id: {}", pkg_id);
                                // This operation wants to keep its old correlation
                                // id, so we insert the request back.
                                self.requests.insert(pkg_id, req);
                            } else {
                                debug!("Operation has been terminated on id: {}, cmd: {:?}", pkg_id, pkg_cmd);
                                // This operation issued a new transaction requests
                                // with the server,
                                Sessions::send_pkgs(&mut self.requests, session, conn, pkgs);
                            }
                        }
                    },

                    Err(e) => {
                        let msg = format!("Exception raised: {}", e);

                        session.report_error(OperationError::InvalidOperation(msg));
                    },
                };

                session.op.is_completed()
            };

            if session_completed {
                debug!("Session related to {} has been closed", pkg_id);
                // The given operation has no longer opened requests, so we can
                // drop it safely.
                let _ = self.assocs.remove(&session_id);
            }
        } else {
            warn!("Package [{}] not handled: cmd {:?}.", pkg_id, pkg_cmd);
        }
    }

    fn terminate_session(&mut self, session: &mut Session) {
        session.report_error(OperationError::Aborted);
    }

    fn check_and_retry(&mut self, conn: &Connection) {
        let mut process_later = Vec::new();

        for (key, req) in &self.requests {
            if req.conn_id != conn.id {
                process_later.push(Checking::delete(*key));
            } else {
                process_later.push(Checking::check(*key));
            }
        }

        for status in process_later {
            if let Some(mut req) = self.requests.remove(&status.key) {
                if status.is_checking {
                    let delete_session = {
                        let session = Sessions::get_session_mut(&mut self.assocs, &req.session);
                        match session.op.check_and_retry(&mut self.buffer) {
                            Ok(outcome) => {
                                if outcome.is_done() {
                                    Some(req.session)
                                } else {
                                    let pkgs = outcome.produced_pkgs();

                                    if pkgs.is_empty() {
                                        self.requests.insert(status.key, req);
                                    } else {
                                        Sessions::send_pkgs(&mut self.requests, session, conn, pkgs);
                                    }

                                    None
                                }
                            },

                            Err(e) => {
                                error!("Exception raised when checking out operation: {}", e);
                                Some(req.session)
                            },
                        }
                    };

                    if let Some(session_id) = delete_session {
                        let mut session = self.assocs.remove(&session_id).expect("Session must exist.");
                        self.terminate_session(&mut session);
                    }
                } else {
                    let mut session = self.assocs.remove(&req.session).expect("Session must exist.");

                    self.terminate_session(&mut session);
                }
            }
        }
    }
}

struct Request {
    session: OperationId,
    conn_id: Uuid,
}

impl Request {
    fn new(session: OperationId, conn: &Connection) -> Request {
        Request {
            session,
            conn_id: conn.id,
        }
    }
}

pub(crate) struct Registry {
    sessions: Sessions,
    awaiting: Vec<OperationWrapper>,
}

impl Registry {
    pub(crate) fn new() -> Registry {
        Registry {
            sessions: Sessions::new(),
            awaiting: Vec::new(),
        }
    }

    fn push_session(&mut self, op: OperationWrapper, conn: &Connection) {
        let mut session = Session::new(op);

        match session.issue_requests(&mut self.sessions.buffer) {
            Ok(pkgs) => {
                Sessions::send_pkgs(&mut self.sessions.requests, &mut session, conn, pkgs);

                self.sessions.insert_session(session);
            },

            Err(e) => {
                // Don't need to cleanup the session at this stage
                // because the session never got the chance to be
                // inserted in the session maps and having running
                // requests.
                error!("Exception occured when issuing requests: {}", e);
            },
        };
    }

    pub(crate) fn register(&mut self, op: OperationWrapper, conn: Option<&Connection>) {
        match conn {
            None       => self.awaiting.push(op),
            Some(conn) => self.push_session(op, conn),
        }
    }

    pub(crate) fn handle(&mut self, pkg: Pkg, conn: &Connection) {
        self.sessions.handle_request_response(pkg, conn);
    }

    pub(crate) fn check_and_retry(&mut self, conn: &Connection) {
        self.sessions.check_and_retry(conn);

        while let Some(op) = self.awaiting.pop() {
            self.register(op, Some(conn));
        }
    }
}
