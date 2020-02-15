use std::collections::HashMap;
use std::time::{Duration, Instant};

use futures::sink::SinkExt;
use uuid::Uuid;

use crate::internal::command::Cmd;
use crate::internal::messages;
use crate::internal::messaging::{self, Lifetime, OpMsg};
use crate::internal::package::Pkg;
use crate::types::{Endpoint, OperationError, Retry};

#[derive(Debug)]
struct Request {
    original: Pkg,
    conn_id: Uuid,
    retries: usize,
    started: Instant,
    mailbox: messaging::Mailbox,
    keep_alive_until: Option<Cmd>,
}

impl Request {
    fn keep_alive(&self) -> bool {
        self.keep_alive_until.is_some()
    }

    fn inner_lifetime(self) -> (messaging::Mailbox, Lifetime<Pkg>) {
        let lifetime = if let Some(cmd) = self.keep_alive_until {
            Lifetime::KeepAlive(cmd, self.original)
        } else {
            Lifetime::OneTime(self.original)
        };

        (self.mailbox, lifetime)
    }
}

#[derive(Debug)]
struct Waiting {
    pkg: Lifetime<Pkg>,
    mailbox: messaging::Mailbox,
}

pub struct Registry {
    requests: HashMap<Uuid, Request>,
    waitings: Vec<Waiting>,
    timeout: Duration,
    max_retries: Retry,
}

impl Registry {
    pub fn new(timeout: Duration, max_retries: Retry) -> Self {
        Registry {
            requests: HashMap::new(),
            waitings: Vec::new(),
            timeout,
            max_retries,
        }
    }

    pub fn register(&mut self, conn_id: Uuid, mailbox: messaging::Mailbox, pkg: Lifetime<Pkg>) {
        let keep_alive_until = pkg.keep_alive_until();
        let req = Request {
            mailbox,
            conn_id,
            original: pkg.inner(),
            retries: 1,
            started: Instant::now(),
            keep_alive_until,
        };

        self.requests.insert(req.original.correlation, req);
    }

    pub fn postpone(&mut self, mailbox: messaging::Mailbox, pkg: Lifetime<Pkg>) {
        let req = Waiting { pkg, mailbox };

        self.waitings.push(req);
    }

    pub async fn handle(&mut self, pkg: Pkg) -> Option<Endpoint> {
        if let Some(mut req) = self.requests.remove(&pkg.correlation) {
            debug!(
                "Package [{}]: command {:?} received {:?}.",
                req.original.correlation, req.original.cmd, pkg.cmd,
            );

            match pkg.cmd {
                Cmd::BadRequest => {
                    let msg = pkg.build_text();

                    error!("Bad request for command {:?}: {}.", req.original.cmd, msg,);

                    let error = OperationError::ServerError(Some(msg));
                    let _ = req.mailbox.send(OpMsg::Failed(error)).await;
                }

                Cmd::NotAuthenticated => {
                    error!(
                        "Not authenticated for command {:?} [{:?}].",
                        req.original.cmd, req.original.correlation,
                    );

                    let error = OperationError::AuthenticationRequired;
                    let _ = req.mailbox.send(OpMsg::Failed(error)).await;
                }

                Cmd::NotHandled => {
                    warn!(
                        "Not handled request {:?} id {}.",
                        req.original.cmd, req.original.correlation,
                    );

                    let result = pkg.to_message::<messages::NotHandled>().and_then(|msg| {
                        if let messages::NotHandled_NotHandledReason::NotMaster = msg.get_reason() {
                            let master_info =
                                protobuf::parse_from_bytes::<messages::NotHandled_MasterInfo>(
                                    msg.get_additional_info(),
                                )?;

                            // TODO - Support reconnection on the secure port when we are going to
                            // implement SSL connection.
                            let addr_str = format!(
                                "{}:{}",
                                master_info.get_external_tcp_address(),
                                master_info.get_external_tcp_port(),
                            );
                            let addr = addr_str.parse().map_err(|e| {
                                std::io::Error::new(
                                    std::io::ErrorKind::InvalidInput,
                                    format!("Failed parsing ip address: {}", e),
                                )
                            })?;

                            let external_tcp_port = Endpoint::from_addr(addr);

                            Ok(Some(external_tcp_port))
                        } else {
                            Ok(None)
                        }
                    });

                    match result {
                        Ok(endpoint_opt) => {
                            let orig_cmd = req.original.cmd;
                            let (mailbox, orig_pkg) = req.inner_lifetime();

                            self.postpone(mailbox, orig_pkg);

                            if let Some(endpoint) = endpoint_opt {
                                warn!(
                                    "Received a non master error on command {:?} id {} retrying on [{:?}]",
                                    orig_cmd,
                                    pkg.correlation,
                                    endpoint,
                                );

                                return Some(endpoint);
                            } else {
                                warn!(
                                    "The server has either not started or is too busy.
                                      Retrying command {:?} id {}.",
                                    orig_cmd, pkg.correlation,
                                );
                            }
                        }

                        Err(error) => {
                            error!(
                                "Decoding error: can't decode NotHandled message: {}.",
                                error,
                            );

                            let msg = format!(
                                "Decoding error: can't decode NotHandled message: {}.",
                                error,
                            );

                            let error = OperationError::ProtobufDecodingError(msg);
                            let _ = req.mailbox.send(OpMsg::Failed(error)).await;
                        }
                    }
                }

                _ => {
                    let resp_cmd = pkg.cmd;
                    let _ = req.mailbox.send(OpMsg::Recv(pkg)).await;

                    if let Some(cmd) = req.keep_alive_until {
                        if cmd != resp_cmd {
                            self.requests.insert(req.original.correlation, req);
                        }
                    }
                }
            }

            None
        } else {
            warn!("No operation associated to package {:?}", pkg);

            None
        }
    }

    pub async fn check_and_retry(&mut self, conn_id: Uuid) -> Vec<Pkg> {
        debug!("Enter check_and_retry process…");

        let mut pkgs: Vec<Pkg> = vec![];
        let max_retries = self.max_retries.to_usize();

        for key in self.requests.keys().copied().collect::<Vec<Uuid>>() {
            let mut req = self.requests.remove(&key).expect("impossible situation");

            if req.conn_id != conn_id {
                let _ = req
                    .mailbox
                    .send(OpMsg::Failed(OperationError::ConnectionHasDropped))
                    .await;

                continue;
            }

            if !req.keep_alive() && req.started.elapsed() >= self.timeout {
                if req.retries + 1 > max_retries {
                    error!(
                        "Command {:?} [{:?}]: maximum retries threshold reached [{}], aborted!",
                        req.original.cmd, req.original.correlation, max_retries,
                    );

                    let _ = req
                        .mailbox
                        .send(OpMsg::Failed(OperationError::Aborted))
                        .await;

                    continue;
                } else {
                    req.retries += 1;
                    req.started = Instant::now();

                    warn!(
                        "Command {:?} [{:?}] has timeout. Retrying (attempt {}/{})",
                        req.original.cmd, req.original.correlation, req.retries, max_retries,
                    );

                    pkgs.push(req.original.clone());
                }
            }

            self.requests.insert(key, req);
        }

        while let Some(req) = self.waitings.pop() {
            pkgs.push(req.pkg.clone().inner());
            self.register(conn_id, req.mailbox, req.pkg);
        }

        debug!("check_and_retry process completed.");

        pkgs
    }

    pub async fn abort(&mut self) {
        for (_, mut req) in self.requests.drain() {
            let _ = req
                .mailbox
                .send(OpMsg::Failed(OperationError::Aborted))
                .await;
        }

        for mut req in self.waitings.drain(..) {
            let _ = req
                .mailbox
                .send(OpMsg::Failed(OperationError::Aborted))
                .await;
        }
    }
}
