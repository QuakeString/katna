// SPDX-License-Identifier: GPL-3.0-or-later

//! SMTP sender: Pimalaya's `io-smtp` coroutines driven by [`Conn`].
//! Pimalaya names end at this module's boundary.

use std::{borrow::Cow, fmt::Display};

use io_sasl::{mechanism::Sasl, rfc4616::plain::SaslPlainCreds, xoauth2::SaslXoauth2Creds};
use io_smtp::{
    coroutine::{SmtpCoroutine, SmtpCoroutineState as S, SmtpYield},
    message::SmtpMessageSend,
    rfc3461::{
        capability::DSN,
        parameter::{SmtpDsnNotify, SmtpDsnRet},
    },
    rfc5321::{
        SmtpAtom, SmtpDomain, SmtpEhloDomain, SmtpForwardPath, SmtpLocalPart, SmtpMailbox,
        SmtpParameter, SmtpReversePath, data::SmtpData, ehlo::SmtpEhlo, mail::SmtpMail,
        quit::SmtpQuit, rcpt::SmtpRcpt,
    },
    session::{
        SmtpSessionOpen, SmtpSessionOpenOptions, SmtpSessionOpenYield as O, SmtpSessionTransport,
    },
};

use crate::{
    Credentials, Endpoint, Error, MailSender, Result, Security,
    backend::Login,
    net::{Conn, Tls},
};

/// One authenticated SMTP submission connection.
pub struct SmtpSender {
    conn: Conn,
    capabilities: Vec<String>,
    /// The capabilities are the ones listed after login.
    asked_again: bool,
    /// Ask for delivery status notifications (RFC 3461) where the server
    /// offers them.
    receipts: bool,
}

impl SmtpSender {
    /// Connects, negotiates TLS, says EHLO and authenticates with PLAIN, or
    /// XOAUTH2 for OAuth2 accounts. A refused access token is renewed once.
    pub async fn connect(endpoint: &Endpoint, creds: &Credentials, tls: Tls) -> Result<Self> {
        match Self::connect_once(endpoint, creds, tls.clone()).await {
            Err(Error::Auth(_)) if creds.retry_after_refusal() => {
                Self::connect_once(endpoint, creds, tls).await
            }
            result => result,
        }
    }

    async fn connect_once(endpoint: &Endpoint, creds: &Credentials, tls: Tls) -> Result<Self> {
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
        let sasl = match creds.login().await? {
            Login::Password(password) => Sasl::Plain(SaslPlainCreds {
                authzid: None,
                authcid: creds.user.clone(),
                passwd: password.into(),
            }),
            Login::Token(token) => Sasl::Xoauth2(SaslXoauth2Creds {
                username: creds.user.clone(),
                token: token.into(),
            }),
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
            asked_again: false,
            receipts: false,
        })
    }

    /// The server's EHLO keywords after login.
    pub fn capabilities(&self) -> &[String] {
        &self.capabilities
    }

    /// Says EHLO again, once, for the capabilities listed after login.
    async fn ask_again(&mut self) -> Result<()> {
        if !self.asked_again {
            // Stalwart lists FUTURERELEASE only to a logged-in client, and
            // io-smtp keeps the list from before login. EHLO again: like
            // RSET, it keeps the login.
            let domain = SmtpEhloDomain::from(SmtpDomain(Cow::Borrowed("localhost")));
            let capabilities = self.run(SmtpEhlo::new(domain)).await?;
            self.capabilities = capabilities.iter().map(|c| c.to_string()).collect();
            self.asked_again = true;
        }
        Ok(())
    }

    /// Sends `message` in one transaction with these `MAIL FROM`
    /// parameters, and delivery status notifications asked for when
    /// [`Self::receipts`] is on and the server offers them.
    async fn transaction(
        &mut self,
        from: &str,
        to: &[&str],
        message: Vec<u8>,
        mut parameters: Vec<SmtpParameter<'static>>,
    ) -> Result<()> {
        let receipts = self.receipts && delivery_receipts(&self.capabilities);
        if receipts {
            // The report carries the headers, not the whole message.
            parameters.push(SmtpDsnRet::Hdrs.into_parameter());
        }
        let reverse = SmtpReversePath::from(mailbox(from)?);
        self.run(SmtpMail::new(reverse, parameters)).await?;
        for addr in to {
            let forward = SmtpForwardPath::from(mailbox(addr)?);
            let notify = if receipts {
                vec![
                    (SmtpDsnNotify::SUCCESS | SmtpDsnNotify::FAILURE | SmtpDsnNotify::DELAY)
                        .into_parameter(),
                ]
            } else {
                Vec::new()
            };
            self.run(SmtpRcpt::new(forward, notify)).await?;
        }
        self.run(SmtpData::new(message)).await
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
        if self.receipts {
            self.ask_again().await?;
            return self.transaction(from, to, message, Vec::new()).await;
        }
        let reverse = SmtpReversePath::from(mailbox(from)?);
        let forward = to
            .iter()
            .map(|addr| mailbox(addr).map(SmtpForwardPath::from))
            .collect::<Result<Vec<_>>>()?;
        self.run(SmtpMessageSend::new(reverse, forward, message))
            .await
    }

    async fn hold_limit(&mut self) -> Result<Option<u64>> {
        if let Some(limit) = future_release(&self.capabilities) {
            return Ok(Some(limit));
        }
        self.ask_again().await?;
        Ok(future_release(&self.capabilities))
    }

    async fn offers_receipts(&mut self) -> Result<bool> {
        if !delivery_receipts(&self.capabilities) {
            self.ask_again().await?;
        }
        Ok(delivery_receipts(&self.capabilities))
    }

    fn ask_for_receipts(&mut self, on: bool) {
        self.receipts = on;
    }

    async fn send_held(
        &mut self,
        from: &str,
        to: &[&str],
        message: Vec<u8>,
        until: i64,
    ) -> Result<()> {
        // RFC 4865: the server keeps the message until `HOLDUNTIL`.
        let keyword = SmtpAtom::parse(b"HOLDUNTIL")
            .map_err(|_| Error::Protocol("HOLDUNTIL keyword".into()))?;
        let hold = SmtpParameter {
            keyword,
            value: Some(Cow::Owned(rfc3339_utc(until))),
        };
        self.transaction(from, to, message, vec![hold]).await
    }

    async fn quit(mut self) -> Result<()> {
        self.run(SmtpQuit::new()).await?;
        self.conn.close().await
    }
}

/// The longest hold, in seconds, the server's `FUTURERELEASE` keyword
/// allows (RFC 4865), if it has one.
pub fn future_release(capabilities: &[String]) -> Option<u64> {
    capabilities.iter().find_map(|line| {
        let mut words = line.split_whitespace();
        words
            .next()
            .is_some_and(|word| word.eq_ignore_ascii_case("FUTURERELEASE"))
            .then(|| words.next()?.parse().ok())
            .flatten()
            .filter(|&max| max > 0)
    })
}

/// Whether the server sends delivery status notifications (RFC 3461).
pub fn delivery_receipts(capabilities: &[String]) -> bool {
    capabilities.iter().any(|line| {
        line.split_whitespace()
            .next()
            .is_some_and(|word| word.eq_ignore_ascii_case(DSN))
    })
}

/// `at` (Unix seconds) as an RFC 3339 time in UTC, for example
/// `2026-09-28T09:00:00Z`.
fn rfc3339_utc(at: i64) -> String {
    let (year, month, day) = crate::outbox::civil_date(at.div_euclid(86_400));
    let secs = at.rem_euclid(86_400);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        secs / 3600,
        secs / 60 % 60,
        secs % 60
    )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn future_release_reads_the_longest_hold() {
        let caps = |lines: &[&str]| lines.iter().map(|l| l.to_string()).collect::<Vec<_>>();
        assert_eq!(
            future_release(&caps(&[
                "PIPELINING",
                "FUTURERELEASE 604800 2026-10-04T17:09:25Z"
            ])),
            Some(604_800)
        );
        assert_eq!(future_release(&caps(&["PIPELINING", "DSN"])), None);
        assert_eq!(future_release(&caps(&["FUTURERELEASE"])), None);
        assert!(delivery_receipts(&caps(&["PIPELINING", "DSN"])));
        assert!(!delivery_receipts(&caps(&["SIZE 35882577", "8BITMIME"])));
        assert_eq!(rfc3339_utc(1_790_416_800), "2026-09-26T10:00:00Z");
        assert_eq!(rfc3339_utc(951_782_400), "2000-02-29T00:00:00Z");
    }
}
