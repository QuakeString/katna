// SPDX-License-Identifier: GPL-3.0-or-later

//! Our own POP3 client (RFC 1939, with CAPA from RFC 2449 and STLS from
//! RFC 2595). POP3 is small enough that a protocol library would cost more
//! than it saves, and Pimalaya has none.
//!
//! [`Pop3Client`] is one authenticated session. The sync logic in [`sync`]
//! only sees the [`Maildrop`] trait, so tests can use an in-memory mailbox.

pub mod sync;

use std::{future::Future, ops::Range};

use crate::{
    Credentials, Endpoint, Error, Result, Security,
    net::{Conn, Tls},
};

/// One message in the maildrop: its number in this session and its
/// server-wide unique ID.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DropEntry {
    /// Message number, valid until the session ends.
    pub number: u32,
    /// UIDL: stays the same across sessions.
    pub uidl: String,
    /// Size in bytes as the server counts it (CRLF line ends).
    pub size: u64,
}

/// A POP3 session as the sync logic uses it.
pub trait Maildrop: Send {
    /// Every message with its UIDL and size.
    fn entries(&mut self) -> impl Future<Output = Result<Vec<DropEntry>>> + Send;

    /// The whole message `number`.
    fn message(&mut self, number: u32) -> impl Future<Output = Result<Vec<u8>>> + Send;

    /// Marks message `number` for deletion. The server removes it only when
    /// the session ends with [`Maildrop::finish`]; a broken connection
    /// keeps it.
    fn delete(&mut self, number: u32) -> impl Future<Output = Result<()>> + Send;

    /// Ends the session, which carries out the deletions.
    fn finish(self) -> impl Future<Output = Result<()>> + Send;
}

/// One authenticated POP3 session.
pub struct Pop3Client {
    conn: Conn,
    /// Bytes read from the server, from `pos` on not yet used.
    pending: Vec<u8>,
    pos: usize,
    capabilities: Vec<String>,
}

impl Pop3Client {
    /// Connects, negotiates TLS and logs in with USER and PASS.
    pub async fn connect(endpoint: &Endpoint, creds: &Credentials, tls: Tls) -> Result<Self> {
        let mut conn = Conn::new(tls);
        match endpoint.security {
            Security::Tls => conn.connect_tls(&endpoint.host, endpoint.port).await?,
            Security::StartTls | Security::Plain => {
                conn.connect_tcp(&endpoint.host, endpoint.port).await?
            }
        }
        let mut client = Self {
            conn,
            pending: Vec::new(),
            pos: 0,
            capabilities: Vec::new(),
        };
        client.status().await?;
        client.capabilities = client.capa().await?;

        if endpoint.security == Security::StartTls {
            if !client.has("STLS") {
                return Err(Error::Tls("the server does not offer STLS".into()));
            }
            client.command("STLS").await?;
            if client.pos < client.pending.len() {
                return Err(Error::Protocol("data after STLS answer".into()));
            }
            client.conn.upgrade_tls().await?;
            // RFC 2595: capabilities may change after STLS.
            client.capabilities = client.capa().await?;
        }

        client.login(creds).await?;
        // Some servers only list UIDL and TOP after login (RFC 2449 §5).
        client.capabilities = client.capa().await?;
        Ok(client)
    }

    /// The server's capabilities (CAPA), in upper case, after login.
    pub fn capabilities(&self) -> &[String] {
        &self.capabilities
    }

    fn has(&self, name: &str) -> bool {
        self.capabilities
            .iter()
            .any(|cap| cap.split_whitespace().next() == Some(name))
    }

    async fn login(&mut self, creds: &Credentials) -> Result<()> {
        // CR and LF in a login would start a new command.
        if [&creds.user, &creds.password]
            .iter()
            .any(|s| s.contains(['\r', '\n']))
        {
            return Err(Error::Auth("line break in user name or password".into()));
        }
        self.command(&format!("USER {}", creds.user))
            .await
            .map_err(auth_error)?;
        self.command(&format!("PASS {}", creds.password))
            .await
            .map_err(auth_error)?;
        Ok(())
    }

    /// CAPA's list, or empty when the server has no CAPA.
    async fn capa(&mut self) -> Result<Vec<String>> {
        match self.command("CAPA").await {
            Ok(_) => {}
            Err(Error::Rejected(_)) => return Ok(Vec::new()),
            Err(err) => return Err(err),
        }
        let body = self.multiline().await?;
        Ok(lines(&body)
            .map(|line| String::from_utf8_lossy(line).trim().to_ascii_uppercase())
            .filter(|line| !line.is_empty())
            .collect())
    }

    /// `UIDL` and `LIST`, joined by message number.
    pub async fn entries(&mut self) -> Result<Vec<DropEntry>> {
        self.command("UIDL").await.map_err(|err| match err {
            Error::Rejected(text) => Error::Protocol(format!("the server has no UIDL: {text}")),
            err => err,
        })?;
        let uidls = self.multiline().await?;
        self.command("LIST").await?;
        let sizes = self.multiline().await?;
        join_listings(&uidls, &sizes)
    }

    /// The header and the first `lines` lines of the body (`TOP`).
    pub async fn top(&mut self, number: u32, lines: u32) -> Result<Vec<u8>> {
        self.command(&format!("TOP {number} {lines}")).await?;
        self.multiline().await
    }

    /// The whole message (`RETR`).
    pub async fn retr(&mut self, number: u32) -> Result<Vec<u8>> {
        self.command(&format!("RETR {number}")).await?;
        self.multiline().await
    }

    /// Marks a message for deletion at the end of the session (`DELE`).
    pub async fn dele(&mut self, number: u32) -> Result<()> {
        self.command(&format!("DELE {number}")).await.map(drop)
    }

    /// Ends the session (`QUIT`); the server now removes deleted messages.
    pub async fn quit(mut self) -> Result<()> {
        self.command("QUIT").await?;
        self.conn.close().await
    }

    /// Sends one command and reads its status line. Returns the text after
    /// `+OK`; `-ERR` becomes [`Error::Rejected`].
    async fn command(&mut self, command: &str) -> Result<String> {
        let verb = command.split(' ').next().unwrap_or_default();
        tracing::trace!(verb, "POP3 command");
        self.conn
            .write_all(format!("{command}\r\n").as_bytes())
            .await?;
        self.status().await
    }

    async fn status(&mut self) -> Result<String> {
        let line = self.line().await?;
        parse_status(&self.pending[line])
    }

    /// Reads a multi-line answer up to the `.` line, removing the
    /// byte-stuffing. Line ends stay as the server sent them.
    async fn multiline(&mut self) -> Result<Vec<u8>> {
        let mut out = Vec::new();
        loop {
            let range = self.line().await?;
            let line = &self.pending[range];
            let text = line
                .strip_suffix(b"\n")
                .map(|l| l.strip_suffix(b"\r").unwrap_or(l))
                .unwrap_or(line);
            if text == b"." {
                return Ok(out);
            }
            out.extend_from_slice(line.strip_prefix(b".").unwrap_or(line));
        }
    }

    /// The next line, with its line end, as a range of `pending`.
    async fn line(&mut self) -> Result<Range<usize>> {
        loop {
            if let Some(end) = self.pending[self.pos..].iter().position(|&b| b == b'\n') {
                let line = self.pos..self.pos + end + 1;
                self.pos = line.end;
                return Ok(line);
            }
            // Keep the unfinished line; drop what was used.
            self.pending.drain(..self.pos);
            self.pos = 0;
            let chunk = self.conn.read().await?;
            if chunk.is_empty() {
                return Err(Error::Closed(
                    "the POP3 server closed the connection".into(),
                ));
            }
            self.pending.extend_from_slice(chunk);
        }
    }
}

impl Maildrop for Pop3Client {
    async fn entries(&mut self) -> Result<Vec<DropEntry>> {
        Pop3Client::entries(self).await
    }

    async fn message(&mut self, number: u32) -> Result<Vec<u8>> {
        self.retr(number).await
    }

    async fn delete(&mut self, number: u32) -> Result<()> {
        self.dele(number).await
    }

    async fn finish(self) -> Result<()> {
        self.quit().await
    }
}

fn auth_error(err: Error) -> Error {
    match err {
        Error::Rejected(text) => Error::Auth(text),
        err => err,
    }
}

/// `+OK text` → `text`; `-ERR text` → [`Error::Rejected`].
fn parse_status(line: &[u8]) -> Result<String> {
    let line = String::from_utf8_lossy(line);
    let line = line.trim_end();
    if let Some(rest) = line.strip_prefix("+OK") {
        Ok(rest.trim_start().to_owned())
    } else if let Some(rest) = line.strip_prefix("-ERR") {
        Err(Error::Rejected(rest.trim_start().to_owned()))
    } else {
        Err(Error::Protocol(format!("unexpected POP3 answer: {line:?}")))
    }
}

fn lines(body: &[u8]) -> impl Iterator<Item = &[u8]> {
    body.split(|&b| b == b'\n')
        .map(|line| line.strip_suffix(b"\r").unwrap_or(line))
        .filter(|line| !line.is_empty())
}

/// `n word` pairs of a UIDL or LIST answer.
fn listing(body: &[u8]) -> Result<Vec<(u32, String)>> {
    lines(body)
        .map(|line| {
            let text = String::from_utf8_lossy(line);
            let mut parts = text.split_whitespace();
            let number = parts.next().and_then(|n| n.parse().ok());
            match (number, parts.next()) {
                (Some(number), Some(word)) => Ok((number, word.to_owned())),
                _ => Err(Error::Protocol(format!("bad POP3 listing line: {text:?}"))),
            }
        })
        .collect()
}

fn join_listings(uidls: &[u8], sizes: &[u8]) -> Result<Vec<DropEntry>> {
    let sizes: std::collections::HashMap<u32, u64> = listing(sizes)?
        .into_iter()
        .map(|(n, size)| {
            size.parse()
                .map(|size| (n, size))
                .map_err(|_| Error::Protocol(format!("bad size for message {n}: {size}")))
        })
        .collect::<Result<_>>()?;
    let mut entries: Vec<DropEntry> = listing(uidls)?
        .into_iter()
        .map(|(number, uidl)| DropEntry {
            number,
            uidl,
            size: sizes.get(&number).copied().unwrap_or(0),
        })
        .collect();
    entries.sort_by_key(|e| e.number);
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_status_lines() {
        assert_eq!(parse_status(b"+OK ready\r\n").unwrap(), "ready");
        assert_eq!(parse_status(b"+OK\r\n").unwrap(), "");
        assert!(matches!(
            parse_status(b"-ERR [AUTH] bad password\r\n"),
            Err(Error::Rejected(text)) if text == "[AUTH] bad password"
        ));
        assert!(matches!(
            parse_status(b"* OK imap\r\n"),
            Err(Error::Protocol(_))
        ));
    }

    #[test]
    fn joins_uidl_and_list() {
        let uidls = b"2 bbb\r\n1 aaa\r\n";
        let sizes = b"1 120\r\n2 3400\r\n";
        assert_eq!(
            join_listings(uidls, sizes).unwrap(),
            [
                DropEntry {
                    number: 1,
                    uidl: "aaa".into(),
                    size: 120
                },
                DropEntry {
                    number: 2,
                    uidl: "bbb".into(),
                    size: 3400
                },
            ]
        );
        assert!(join_listings(b"1\r\n", b"").is_err());
        assert!(join_listings(b"1 a\r\n", b"1 big\r\n").is_err());
    }
}
