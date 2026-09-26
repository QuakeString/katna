// SPDX-License-Identifier: GPL-3.0-or-later

//! SMTP sender: Pimalaya's `io-smtp` coroutines driven by [`Conn`].
//! Pimalaya names end at this module's boundary.

use std::{borrow::Cow, fmt::Display};

use io_sasl::rfc4616::plain::SaslPlainCreds;
use io_smtp::{
    coroutine::{SmtpCoroutine, SmtpCoroutineState as S, SmtpYield},
    message::SmtpMessageSend,
    rfc5321::{
        SmtpDomain, SmtpEhloDomain, SmtpForwardPath, SmtpLocalPart, SmtpMailbox, SmtpReversePath,
        quit::SmtpQuit,
    },
    session::{
        SmtpSessionOpen, SmtpSessionOpenOptions, SmtpSessionOpenYield as O, SmtpSessionTransport,
    },
};

use crate::{
    Credentials, Endpoint, Error, MailSender, Result, Security,
    net::{Conn, Tls},
};

/// One authenticated SMTP submission connection.
pub struct SmtpSender {
    conn: Conn,
    capabilities: Vec<String>,
}

impl SmtpSender {
    /// Connects, negotiates TLS, says EHLO and authenticates with PLAIN.
    pub async fn connect(endpoint: &Endpoint, creds: &Credentials, tls: Tls) -> Result<Self> {
        let transport = match endpoint.security {
            Security::Tls => SmtpSessionTransport::Tls {
                host: endpoint.host.clone(),
                port: endpoint.port,
            },
            Security::StartTls | Security::Plain => SmtpSessionTransport::Tcp {
                host: endpoint.host.clone(),
                port: endpoint.port,
            },
        };
        let opts = SmtpSessionOpenOptions {
            starttls: endpoint.security == Security::StartTls,
        };
        let sasl = SaslPlainCreds {
            authzid: None,
            authcid: creds.user.clone(),
            passwd: creds.password.clone().into(),
        };
        // A real client sends its own FQDN or an address literal.
        let domain = SmtpEhloDomain::from(SmtpDomain(Cow::Borrowed("localhost")));
        let mut co = SmtpSessionOpen::new(transport, domain, Some(sasl), opts);

        let mut conn = Conn::new(tls);
        let mut state = co.resume(None);
        let session = loop {
            state = match state {
                S::Yielded(O::WantsTcpConnect { host, port }) => {
                    conn.connect_tcp(&host, port).await?;
                    co.resume(None)
                }
                S::Yielded(O::WantsTlsConnect { host, port }) => {
                    conn.connect_tls(&host, port).await?;
                    co.resume(None)
                }
                S::Yielded(O::WantsTlsUpgrade) => {
                    conn.upgrade_tls().await?;
                    co.resume(None)
                }
                S::Yielded(O::WantsUnixConnect(path)) => {
                    return Err(Error::Protocol(format!(
                        "unix sockets not supported: {path}"
                    )));
                }
                S::Yielded(O::WantsWrite(bytes)) => {
                    conn.write_all(&bytes).await?;
                    co.resume(None)
                }
                S::Yielded(O::WantsRead) => {
                    let bytes = conn.read().await?;
                    co.resume(Some(bytes))
                }
                S::Complete(Ok(session)) => break session,
                S::Complete(Err(err)) => return Err(Error::Auth(err.to_string())),
            };
        };

        Ok(Self {
            conn,
            capabilities: session.capabilities.iter().map(|c| c.to_string()).collect(),
        })
    }

    /// The server's EHLO keywords after login.
    pub fn capabilities(&self) -> &[String] {
        &self.capabilities
    }

    async fn run<C, T, E>(&mut self, mut co: C) -> Result<T>
    where
        C: SmtpCoroutine<Yield = SmtpYield, Return = std::result::Result<T, E>>,
        E: Display,
    {
        let conn = &mut self.conn;
        let mut state = co.resume(None);
        loop {
            state = match state {
                S::Yielded(SmtpYield::WantsWrite(bytes)) => {
                    conn.write_all(&bytes).await?;
                    co.resume(None)
                }
                S::Yielded(SmtpYield::WantsRead) => {
                    let bytes = conn.read().await?;
                    co.resume(Some(bytes))
                }
                S::Complete(result) => return result.map_err(command_error),
            };
        }
    }
}

impl MailSender for SmtpSender {
    async fn send(&mut self, from: &str, to: &[&str], message: Vec<u8>) -> Result<()> {
        let reverse = SmtpReversePath::from(mailbox(from)?);
        let forward = to
            .iter()
            .map(|addr| mailbox(addr).map(SmtpForwardPath::from))
            .collect::<Result<Vec<_>>>()?;
        self.run(SmtpMessageSend::new(reverse, forward, message))
            .await
    }

    async fn quit(mut self) -> Result<()> {
        self.run(SmtpQuit::new()).await?;
        self.conn.close().await
    }
}

/// io-smtp's errors for refused commands (4xx and 5xx replies) all say
/// "rejected"; keep those apart, since the message itself may be the
/// problem and retrying it will not help.
fn command_error(err: impl Display) -> Error {
    let text = err.to_string();
    if text.contains("rejected") {
        Error::Rejected(text)
    } else if text.contains("EOF") {
        Error::Closed(text)
    } else {
        Error::Protocol(text)
    }
}

fn mailbox(addr: &str) -> Result<SmtpMailbox<'static>> {
    let (local, domain) = addr
        .rsplit_once('@')
        .ok_or_else(|| Error::Protocol(format!("not an address: {addr}")))?;
    let domain = SmtpDomain::parse(domain.as_bytes())
        .map_err(|_| Error::Protocol(format!("bad domain in {addr}")))?;
    Ok(SmtpMailbox {
        local_part: SmtpLocalPart(Cow::Owned(local.to_owned())),
        domain: SmtpEhloDomain::from(SmtpDomain(Cow::Owned(domain.0.into_owned()))),
    })
}
