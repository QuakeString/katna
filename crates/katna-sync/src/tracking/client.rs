// SPDX-License-Identifier: GPL-3.0-or-later

//! Talking to Katna Server (`server/katna-server/README.md`): registering
//! this install, asking for tracking IDs, and reading the event stream.
//! `https` only, except `http` to this computer for tests.

use std::time::Duration;

use serde::Deserialize;

use crate::net::{Conn, Tls};
use crate::{Error, Result};

/// How long one request may take.
const TIMEOUT: Duration = Duration::from_secs(30);
/// Largest answer read.
const MAX_BODY: usize = 1024 * 1024;

/// Where the server is.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Server {
    host: String,
    port: u16,
    tls: bool,
    /// `https://host[:port]`, used in tracked mail.
    base: String,
}

impl Server {
    /// Parses `https://host[:port]`, or `http://127.0.0.1:port` (also
    /// `localhost` and `[::1]`) for tests. `None` for anything else,
    /// including an empty string (tracking off).
    pub fn parse(url: &str) -> Option<Self> {
        let url = url.trim().trim_end_matches('/');
        let (tls, rest) = if let Some(rest) = url.strip_prefix("https://") {
            (true, rest)
        } else {
            (false, url.strip_prefix("http://")?)
        };
        if rest.is_empty() || rest.contains(['/', '?', '#', '@', ' ']) {
            return None;
        }
        let (host, port) = match rest.rsplit_once(':') {
            Some((host, port)) if !host.ends_with(':') => (host, port.parse().ok()?),
            _ => (rest, if tls { 443 } else { 80 }),
        };
        let host = host.trim_start_matches('[').trim_end_matches(']');
        if host.is_empty() || (!tls && !matches!(host, "127.0.0.1" | "localhost" | "::1")) {
            return None;
        }
        Some(Self {
            host: host.to_owned(),
            port,
            tls,
            base: url.to_owned(),
        })
    }

    /// The server's address as tracked mail uses it.
    pub fn base(&self) -> &str {
        &self.base
    }
}

/// A registered install.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Registration {
    pub install: String,
    pub token: String,
}

/// An open or click, as the server reports it.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ServerEvent {
    /// Increasing number; the stream resumes after the last one kept.
    pub seq: i64,
    /// The tracking ID.
    pub id: String,
    /// `open` or `click`.
    pub kind: String,
    /// For a click, the link's number.
    #[serde(default)]
    pub link: Option<i64>,
    /// `person`, `apple_proxy` or `scanner`.
    pub source: String,
    /// Milliseconds since the Unix epoch.
    pub at: i64,
}

/// A client for one server.
#[derive(Clone)]
pub struct Client {
    server: Server,
    tls: Tls,
}

impl Client {
    pub fn new(server: Server, tls: Tls) -> Self {
        Self { server, tls }
    }

    pub fn server(&self) -> &Server {
        &self.server
    }

    /// Registers a new install.
    pub async fn register(&self) -> Result<Registration> {
        let (status, body) = self.call("POST", "/api/v1/installs", None, b"").await?;
        check(status, &body)?;
        serde_json::from_slice(&body).map_err(|e| Error::Protocol(format!("tracking server: {e}")))
    }

    /// `count` new tracking IDs whose links go to `links`.
    pub async fn create(&self, token: &str, count: usize, links: &[String]) -> Result<Vec<String>> {
        #[derive(Deserialize)]
        struct Created {
            ids: Vec<String>,
        }
        let body = serde_json::json!({ "count": count, "links": links }).to_string();
        let (status, answer) = self
            .call("POST", "/api/v1/tracks", Some(token), body.as_bytes())
            .await?;
        check(status, &answer)?;
        let created: Created = serde_json::from_slice(&answer)
            .map_err(|e| Error::Protocol(format!("tracking server: {e}")))?;
        if created.ids.len() != count {
            return Err(Error::Protocol(
                "tracking server: wrong number of IDs".into(),
            ));
        }
        // IDs go into the HTML of mail sent in the user's name: only the
        // server's own form (32 lowercase hex digits) is used.
        if !created.ids.iter().all(|id| is_track_id(id)) {
            return Err(Error::Protocol("tracking server: malformed ID".into()));
        }
        Ok(created.ids)
    }

    /// Deletes a tracking ID and its events on the server.
    pub async fn forget(&self, token: &str, id: &str) -> Result<()> {
        let (status, body) = self
            .call("DELETE", &format!("/api/v1/tracks/{id}"), Some(token), b"")
            .await?;
        if status == 404 {
            return Ok(());
        }
        check(status, &body)
    }

    /// Opens the event stream after event `after`.
    pub async fn events(&self, token: &str, after: i64) -> Result<EventStream> {
        let mut conn = self.connect().await?;
        let request = format!(
            "GET /api/v1/events HTTP/1.1\r\nHost: {}\r\nUser-Agent: Katna\r\n\
             Accept: text/event-stream\r\nAuthorization: Bearer {token}\r\n\
             Last-Event-ID: {after}\r\nConnection: close\r\n\r\n",
            self.server.host
        );
        conn.write_all(request.as_bytes()).await?;
        let mut head = Vec::new();
        let rest = loop {
            let chunk = conn.read().await?;
            if chunk.is_empty() {
                return Err(Error::Closed("tracking server".into()));
            }
            head.extend_from_slice(chunk);
            if let Some(at) = head.windows(4).position(|w| w == b"\r\n\r\n") {
                break head.split_off(at + 4);
            }
            if head.len() > 64 * 1024 {
                return Err(Error::Protocol("tracking server: header too long".into()));
            }
        };
        let (status, headers) = parse_head(&head)?;
        if status != 200 {
            return Err(status_error(status, ""));
        }
        let chunked = headers.chunked;
        let mut stream = EventStream {
            conn,
            body: Body::new(chunked),
            lines: SseParser::default(),
        };
        stream.feed(&rest);
        Ok(stream)
    }

    async fn connect(&self) -> Result<Conn> {
        let mut conn = Conn::new(self.tls.clone());
        if self.server.tls {
            conn.connect_tls(&self.server.host, self.server.port)
                .await?;
        } else {
            conn.connect_tcp(&self.server.host, self.server.port)
                .await?;
        }
        Ok(conn)
    }

    async fn call(
        &self,
        method: &str,
        path: &str,
        token: Option<&str>,
        body: &[u8],
    ) -> Result<(u16, Vec<u8>)> {
        let call = async {
            let mut conn = self.connect().await?;
            let mut request = format!(
                "{method} {path} HTTP/1.1\r\nHost: {}\r\nUser-Agent: Katna\r\n\
                 Accept: application/json\r\nConnection: close\r\nContent-Length: {}\r\n",
                self.server.host,
                body.len()
            );
            if !body.is_empty() {
                request.push_str("Content-Type: application/json\r\n");
            }
            if let Some(token) = token {
                request.push_str(&format!("Authorization: Bearer {token}\r\n"));
            }
            request.push_str("\r\n");
            conn.write_all(request.as_bytes()).await?;
            conn.write_all(body).await?;
            let mut response = Vec::new();
            loop {
                let chunk = conn.read().await?;
                if chunk.is_empty() {
                    break;
                }
                response.extend_from_slice(chunk);
                if response.len() > MAX_BODY + 64 * 1024 {
                    return Err(Error::Protocol("tracking server: answer too large".into()));
                }
            }
            let _ = conn.close().await;
            let at = response
                .windows(4)
                .position(|w| w == b"\r\n\r\n")
                .ok_or_else(|| Error::Protocol("tracking server: no header".into()))?;
            let (status, headers) = parse_head(&response[..at + 4])?;
            let mut body = Body::new(headers.chunked);
            let mut out = Vec::new();
            body.push(&response[at + 4..], &mut out);
            if let Some(length) = headers.length {
                out.truncate(length);
            }
            Ok((status, out))
        };
        futures_lite::FutureExt::or(call, async {
            async_io::Timer::after(TIMEOUT).await;
            Err(Error::Timeout(TIMEOUT))
        })
        .await
    }
}

fn check(status: u16, body: &[u8]) -> Result<()> {
    if (200..300).contains(&status) {
        Ok(())
    } else {
        Err(status_error(status, &String::from_utf8_lossy(body)))
    }
}

fn status_error(status: u16, body: &str) -> Error {
    let body: String = body.chars().take(200).collect();
    match status {
        401 => Error::Auth(format!("tracking server: {body}")),
        400..=499 => Error::Rejected(format!("tracking server {status}: {body}")),
        _ => Error::Closed(format!("tracking server {status}")),
    }
}

struct Head {
    chunked: bool,
    length: Option<usize>,
}

fn parse_head(head: &[u8]) -> Result<(u16, Head)> {
    let text = std::str::from_utf8(head)
        .map_err(|_| Error::Protocol("tracking server: bad header".into()))?;
    let mut lines = text.split("\r\n");
    let status = lines
        .next()
        .and_then(|line| line.split(' ').nth(1))
        .and_then(|code| code.parse().ok())
        .ok_or_else(|| Error::Protocol("tracking server: no status".into()))?;
    let mut parsed = Head {
        chunked: false,
        length: None,
    };
    for line in lines {
        if let Some((name, value)) = line.split_once(':') {
            match name.trim().to_ascii_lowercase().as_str() {
                "transfer-encoding" => {
                    parsed.chunked = value.trim().eq_ignore_ascii_case("chunked")
                }
                "content-length" => parsed.length = value.trim().parse().ok(),
                _ => {}
            }
        }
    }
    Ok((status, parsed))
}

/// Decodes a body as it arrives, chunked or not.
struct Body {
    chunked: bool,
    pending: Vec<u8>,
    /// Bytes left in the current chunk.
    left: usize,
    done: bool,
}

impl Body {
    fn new(chunked: bool) -> Self {
        Self {
            chunked,
            pending: Vec::new(),
            left: 0,
            done: false,
        }
    }

    /// Adds `data` and appends what is decoded to `out`.
    fn push(&mut self, data: &[u8], out: &mut Vec<u8>) {
        if !self.chunked {
            out.extend_from_slice(data);
            return;
        }
        self.pending.extend_from_slice(data);
        let mut at = 0;
        while !self.done && at < self.pending.len() {
            if self.left > 0 {
                let take = self.left.min(self.pending.len() - at);
                out.extend_from_slice(&self.pending[at..at + take]);
                at += take;
                self.left -= take;
                continue;
            }
            // A size line, possibly after the previous chunk's CRLF.
            let Some(end) = self.pending[at..].windows(2).position(|w| w == b"\r\n") else {
                break;
            };
            let line = &self.pending[at..at + end];
            at += end + 2;
            if line.is_empty() {
                continue;
            }
            let size = std::str::from_utf8(line)
                .ok()
                .and_then(|l| usize::from_str_radix(l.split(';').next()?.trim(), 16).ok());
            match size {
                Some(0) | None => self.done = true,
                Some(size) => self.left = size,
            }
        }
        self.pending.drain(..at);
    }
}

/// Splits server-sent events into `data` payloads.
#[derive(Default)]
struct SseParser {
    line: Vec<u8>,
    data: String,
    ready: std::collections::VecDeque<String>,
}

impl SseParser {
    fn push(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            if byte != b'\n' {
                self.line.push(byte);
                continue;
            }
            let line = String::from_utf8_lossy(&self.line)
                .trim_end_matches('\r')
                .to_owned();
            self.line.clear();
            if line.is_empty() {
                if !self.data.is_empty() {
                    self.ready.push_back(std::mem::take(&mut self.data));
                }
            } else if let Some(data) = line.strip_prefix("data:") {
                if !self.data.is_empty() {
                    self.data.push('\n');
                }
                self.data.push_str(data.strip_prefix(' ').unwrap_or(data));
            }
            // `id:`, `event:` and `:` comments (keep-alives) need nothing.
        }
    }
}

/// An open event stream.
pub struct EventStream {
    conn: Conn,
    body: Body,
    lines: SseParser,
}

impl EventStream {
    fn feed(&mut self, data: &[u8]) {
        let mut decoded = Vec::new();
        self.body.push(data, &mut decoded);
        self.lines.push(&decoded);
    }

    /// The next event. `Ok(None)` when nothing, not even a keep-alive,
    /// arrived within `idle` (the server sends one every 30 s), or the
    /// server ended the stream: reconnect then.
    pub async fn next(&mut self, idle: Duration) -> Result<Option<ServerEvent>> {
        loop {
            while let Some(data) = self.lines.ready.pop_front() {
                match serde_json::from_str(&data) {
                    Ok(event) => return Ok(Some(event)),
                    Err(err) => tracing::debug!(%err, "skipping an unreadable tracking event"),
                }
            }
            if self.body.done {
                return Ok(None);
            }
            let Some(chunk) = self.conn.read_timeout(idle).await? else {
                return Ok(None);
            };
            if chunk.is_empty() {
                return Ok(None);
            }
            let chunk = chunk.to_vec();
            self.feed(&chunk);
        }
    }
}

/// Whether `id` looks like a tracking ID the server makes: 32 lowercase
/// hex digits.
fn is_track_id(id: &str) -> bool {
    id.len() == 32 && id.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

#[cfg(test)]
mod tests {
    use super::is_track_id;

    #[test]
    fn only_well_formed_track_ids() {
        assert!(is_track_id(&"0a".repeat(16)));
        assert!(!is_track_id(&"0A".repeat(16)));
        assert!(!is_track_id("\"><img src=x>"));
        assert!(!is_track_id(&"a".repeat(33)));
    }

    use super::*;

    #[test]
    fn server_addresses() {
        let server = Server::parse("https://server.katna.invenia.in/").unwrap();
        assert_eq!(
            (server.host.as_str(), server.port, server.tls),
            ("server.katna.invenia.in", 443, true)
        );
        assert_eq!(server.base(), "https://server.katna.invenia.in");
        let local = Server::parse("http://127.0.0.1:18090").unwrap();
        assert_eq!((local.port, local.tls), (18090, false));
        assert!(Server::parse("").is_none());
        assert!(Server::parse("http://server.katna.invenia.in").is_none());
        assert!(Server::parse("https://a.org/path").is_none());
        assert!(Server::parse("ftp://a.org").is_none());
    }

    #[test]
    fn chunked_events_in_pieces() {
        let event = r#"{"seq":7,"id":"ab","kind":"click","link":1,"source":"person","at":5}"#;
        let sse = format!(": keep-alive\n\nid: 7\nevent: track\ndata: {event}\n\n");
        let chunked = format!("{:x}\r\n{sse}\r\n0\r\n\r\n", sse.len());
        let mut body = Body::new(true);
        let mut parser = SseParser::default();
        for piece in chunked.as_bytes().chunks(5) {
            let mut out = Vec::new();
            body.push(piece, &mut out);
            parser.push(&out);
        }
        assert!(body.done);
        let data = parser.ready.pop_front().unwrap();
        let parsed: ServerEvent = serde_json::from_str(&data).unwrap();
        assert_eq!(parsed.seq, 7);
        assert_eq!(parsed.link, Some(1));
        assert!(parser.ready.is_empty());
    }

    #[test]
    fn status_errors() {
        assert!(matches!(status_error(401, ""), Error::Auth(_)));
        assert!(matches!(status_error(429, "x"), Error::Rejected(_)));
        assert!(status_error(502, "").is_transient());
    }
}
