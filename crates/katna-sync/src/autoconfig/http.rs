// SPDX-License-Identifier: GPL-3.0-or-later

//! Just enough HTTPS to fetch a configuration file or an image: `GET`,
//! `Connection: close`, `Content-Length` or chunked bodies, and a few
//! redirects; and a `POST` for crash reports. Only `https` URLs, so a
//! network in the middle cannot hand us its servers or see what is
//! fetched or sent.

use std::time::Duration;

use futures_lite::FutureExt;

use crate::{
    Error, Result,
    net::{Conn, Reach, Tls},
};

/// Largest body [`get`] accepts; configuration files are a few kilobytes.
const MAX_BODY: usize = 1024 * 1024;
const MAX_REDIRECTS: usize = 3;

/// Fetches `url` and returns the body of a `200` answer, or `None` for any
/// other status (404 is the usual "no configuration here").
pub async fn get(url: &str, tls: &Tls, timeout: Duration) -> Result<Option<Vec<u8>>> {
    get_limited(url, tls, timeout, MAX_BODY).await
}

/// Like [`get`], for bodies of at most `max_body` bytes.
pub async fn get_limited(
    url: &str,
    tls: &Tls,
    timeout: Duration,
    max_body: usize,
) -> Result<Option<Vec<u8>>> {
    fetch(url, tls, timeout, max_body, false, Reach::Any).await
}

/// Like [`get_limited`], for a URL that mail, DNS or a web page named
/// (a remote image, a sender picture): only port 443 of a public address,
/// on every redirect too ([`Reach::Public`]).
pub async fn get_public(
    url: &str,
    tls: &Tls,
    timeout: Duration,
    max_body: usize,
) -> Result<Option<Vec<u8>>> {
    fetch(url, tls, timeout, max_body, false, Reach::Public).await
}

/// Like [`get_limited`], but a longer body is cut instead of refused, and
/// reading stops at the end of an HTML page's `<head>`: for reading what a
/// page names in its head.
pub async fn get_head(
    url: &str,
    tls: &Tls,
    timeout: Duration,
    max_body: usize,
) -> Result<Option<Vec<u8>>> {
    fetch(url, tls, timeout, max_body, true, Reach::Any).await
}

/// [`get_head`] with the limits of [`get_public`].
pub async fn get_head_public(
    url: &str,
    tls: &Tls,
    timeout: Duration,
    max_body: usize,
) -> Result<Option<Vec<u8>>> {
    fetch(url, tls, timeout, max_body, true, Reach::Public).await
}

async fn fetch(
    url: &str,
    tls: &Tls,
    timeout: Duration,
    max_body: usize,
    head: bool,
    reach: Reach,
) -> Result<Option<Vec<u8>>> {
    let fetch = async {
        let mut url = url.to_owned();
        for _ in 0..=MAX_REDIRECTS {
            match get_once(&url, tls, max_body, head, reach).await? {
                Answer::Body(body) => return Ok(Some(body)),
                Answer::Redirect(to) => url = resolve(&url, &to)?,
                Answer::Status(status) => {
                    tracing::debug!(url, status, "nothing there");
                    return Ok(None);
                }
            }
        }
        Err(Error::Protocol(format!("{url}: too many redirects")))
    };
    fetch
        .or(async {
            async_io::Timer::after(timeout).await;
            Err(Error::Timeout(timeout))
        })
        .await
}

/// Downloads `url`, following a few redirects, handing the body to `sink`
/// piece by piece with the whole body's length, without keeping it in
/// memory: for update packages. The answer must be a `200` with a
/// `Content-Length` of at most `max_size` bytes. Returns the length.
pub async fn download(
    url: &str,
    tls: &Tls,
    max_size: u64,
    sink: &mut (dyn FnMut(&[u8], u64) -> std::io::Result<()> + Send),
) -> Result<u64> {
    match download_with(url, &[], tls, max_size, sink).await? {
        Ok(size) => Ok(size),
        Err(status) => Err(Error::Protocol(format!("{url}: HTTP status {status}"))),
    }
}

/// Like [`download`], sending `headers` (a token) to `url` only, never on
/// to where it redirects, and allowing plain HTTP to the loopback for
/// tests. A status other than `200` or a redirect comes back as `Err` in
/// the `Ok`, for the caller to read: for drive files.
pub async fn download_with(
    url: &str,
    headers: &[(&str, &str)],
    tls: &Tls,
    max_size: u64,
    sink: &mut (dyn FnMut(&[u8], u64) -> std::io::Result<()> + Send),
) -> Result<std::result::Result<u64, u16>> {
    let mut url = url.to_owned();
    let mut headers = headers;
    for _ in 0..=MAX_REDIRECTS {
        match download_once(&url, headers, tls, max_size, sink).await? {
            Downloaded::Done(size) => return Ok(Ok(size)),
            Downloaded::Refused(status) => return Ok(Err(status)),
            Downloaded::Redirect(to) => {
                url = resolve(&url, &to)?;
                headers = &[];
            }
        }
    }
    Err(Error::Protocol(format!("{url}: too many redirects")))
}

enum Downloaded {
    Done(u64),
    Redirect(String),
    Refused(u16),
}

async fn download_once(
    url: &str,
    headers: &[(&str, &str)],
    tls: &Tls,
    max_size: u64,
    sink: &mut (dyn FnMut(&[u8], u64) -> std::io::Result<()> + Send),
) -> Result<Downloaded> {
    let (parts, plain) = match url.strip_prefix("http://") {
        Some(rest) => {
            let parts = parse_url_loopback(rest)?;
            if !matches!(parts.host, "localhost" | "127.0.0.1") {
                return Err(Error::Protocol(format!("{url}: http only to localhost")));
            }
            (parts, true)
        }
        None => (parse_url(url)?, false),
    };
    let mut conn = Conn::new(tls.clone());
    if plain {
        conn.connect_tcp(parts.host, parts.port).await?;
    } else {
        conn.connect_tls(parts.host, parts.port).await?;
    }
    let mut request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nUser-Agent: Katna\r\nAccept: */*\r\n\
         Connection: close\r\n",
        parts.path, parts.host
    );
    for (name, value) in headers {
        if name.contains(['\r', '\n', ':']) || value.contains(['\r', '\n']) {
            return Err(Error::Protocol(format!("{url}: bad header {name}")));
        }
        request.push_str(&format!("{name}: {value}\r\n"));
    }
    request.push_str("\r\n");
    conn.write_all(request.as_bytes()).await?;
    let mut response = Vec::new();
    while !response.windows(4).any(|w| w == b"\r\n\r\n") {
        let chunk = conn.read_raw().await?;
        if chunk.is_empty() {
            return Err(Error::Closed(format!("{url}: no answer")));
        }
        response.extend_from_slice(chunk);
        if response.len() > 64 * 1024 {
            return Err(Error::Protocol(format!("{url}: header too large")));
        }
    }
    let head = parse_head(&response)?;
    match head.status {
        200 => {}
        301 | 302 | 303 | 307 | 308 => {
            let _ = conn.close().await;
            return head
                .location
                .map(Downloaded::Redirect)
                .ok_or_else(|| Error::Protocol("HTTP: redirect without Location".into()));
        }
        other => {
            let _ = conn.close().await;
            return Ok(Downloaded::Refused(other));
        }
    }
    if head.chunked {
        return Err(Error::Protocol(format!("{url}: no Content-Length")));
    }
    let total =
        head.length
            .ok_or_else(|| Error::Protocol(format!("{url}: no Content-Length")))? as u64;
    if total > max_size {
        return Err(Error::Protocol(format!(
            "{url}: {total} bytes is too large"
        )));
    }
    let first = &head.rest[..head.rest.len().min(total as usize)];
    sink(first, total)?;
    let mut done = first.len() as u64;
    while done < total {
        let chunk = conn.read_raw().await?;
        if chunk.is_empty() {
            return Err(Error::Closed(format!(
                "{url}: ended after {done} of {total} bytes"
            )));
        }
        let take = chunk.len().min((total - done) as usize);
        sink(&chunk[..take], total)?;
        done += take as u64;
    }
    let _ = conn.close().await;
    Ok(Downloaded::Done(total))
}

/// Posts `body` to `url` with the headers `headers` and returns the answer's
/// status code. Redirects are not followed.
pub async fn post(
    url: &str,
    headers: &[(&str, &str)],
    body: &[u8],
    tls: &Tls,
    timeout: Duration,
) -> Result<u16> {
    let post = async {
        let parts = parse_url(url)?;
        let mut request = format!(
            "POST {} HTTP/1.1\r\nHost: {}\r\nUser-Agent: Katna\r\nContent-Length: {}\r\n\
             Connection: close\r\n",
            parts.path,
            parts.host,
            body.len()
        );
        for (name, value) in headers {
            if name.contains(['\r', '\n', ':']) || value.contains(['\r', '\n']) {
                return Err(Error::Protocol(format!("{url}: bad header {name}")));
            }
            request.push_str(&format!("{name}: {value}\r\n"));
        }
        request.push_str("\r\n");
        let mut conn = Conn::new(tls.clone());
        conn.connect_tls(parts.host, parts.port).await?;
        conn.write_all(request.as_bytes()).await?;
        conn.write_all(body).await?;
        let mut response = Vec::new();
        // The status line is all that is needed.
        while !response.windows(2).any(|w| w == b"\r\n") {
            let chunk = conn.read().await?;
            if chunk.is_empty() || response.len() > 64 * 1024 {
                break;
            }
            response.extend_from_slice(chunk);
        }
        let _ = conn.close().await;
        status(&response).ok_or_else(|| Error::Protocol(format!("{url}: no HTTP status")))
    };
    post.or(async {
        async_io::Timer::after(timeout).await;
        Err(Error::Timeout(timeout))
    })
    .await
}

/// The code of an `HTTP/1.1 200 OK` line.
fn status(response: &[u8]) -> Option<u16> {
    let line = response.split(|b| *b == b'\r').next()?;
    let line = std::str::from_utf8(line).ok()?;
    let mut words = line.split(' ');
    words.next()?.starts_with("HTTP/").then_some(())?;
    words.next()?.parse().ok()
}

enum Answer {
    Body(Vec<u8>),
    Redirect(String),
    Status(u16),
}

/// The parts of an `https://host[:port]/path` URL.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Url<'a> {
    pub host: &'a str,
    pub port: u16,
    /// Path and query, starting with `/`.
    pub path: &'a str,
}

pub(crate) fn parse_url(url: &str) -> Result<Url<'_>> {
    // The URL goes into the request line as it is: a line break in it
    // would start a second request (URLs come from mail, DNS and web
    // pages), and a space would end the path early.
    if url.bytes().any(|b| b <= b' ' || b == 0x7f) {
        return Err(Error::Protocol(format!(
            "{}: spaces or control characters in the URL",
            url.escape_debug()
        )));
    }
    let rest = url
        .strip_prefix("https://")
        .ok_or_else(|| Error::Protocol(format!("{url}: only https URLs are fetched")))?;
    let (authority, path) = match rest.find(['/', '?']) {
        Some(at) if rest.as_bytes()[at] == b'/' => rest.split_at(at),
        Some(_) => return Err(Error::Protocol(format!("{url}: no path"))),
        None => (rest, "/"),
    };
    let (host, port) = match authority.rsplit_once(':') {
        Some((host, port)) => (
            host,
            port.parse()
                .map_err(|_| Error::Protocol(format!("{url}: bad port")))?,
        ),
        None => (authority, 443),
    };
    if host.is_empty() || host.contains(['@', ' ']) {
        return Err(Error::Protocol(format!("{url}: bad host")));
    }
    Ok(Url { host, port, path })
}

/// A `Location` relative to the URL it came from.
fn resolve(base: &str, location: &str) -> Result<String> {
    if location.starts_with("https://") {
        return Ok(location.to_owned());
    }
    if location.starts_with('/') && !location.starts_with("//") {
        let url = parse_url(base)?;
        return Ok(format!("https://{}:{}{location}", url.host, url.port));
    }
    Err(Error::Protocol(format!(
        "{base}: will not follow a redirect to {location}"
    )))
}

/// With `head`, reading stops after `</head>` or `max_body` bytes, and the
/// body is cut there. With [`Reach::Public`], only port 443 is used.
async fn get_once(
    url: &str,
    tls: &Tls,
    max_body: usize,
    head: bool,
    reach: Reach,
) -> Result<Answer> {
    let parts = parse_url(url)?;
    if reach == Reach::Public && parts.port != 443 {
        return Err(Error::Protocol(format!("{url}: only port 443 is used")));
    }
    let mut conn = Conn::with_reach(tls.clone(), reach);
    conn.connect_tls(parts.host, parts.port).await?;
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nUser-Agent: Katna\r\nAccept: */*\r\n\
         Connection: close\r\n\r\n",
        parts.path, parts.host
    );
    conn.write_all(request.as_bytes()).await?;
    let mut response = Vec::new();
    loop {
        let chunk = conn.read().await?;
        if chunk.is_empty() {
            break;
        }
        let from = response.len().saturating_sub(6);
        response.extend_from_slice(chunk);
        if head
            && (response.len() >= max_body
                || response[from..]
                    .windows(7)
                    .any(|w| w.eq_ignore_ascii_case(b"</head>")))
        {
            break;
        }
        if response.len() > max_body + 64 * 1024 {
            return Err(Error::Protocol(format!("{url}: answer too large")));
        }
    }
    let _ = conn.close().await;
    parse_response(&response, max_body, head)
}

/// The start of an HTTP/1.1 response.
struct Head<'a> {
    status: u16,
    chunked: bool,
    length: Option<usize>,
    location: Option<String>,
    /// A `Range` header (Google's resumable uploads say how much they
    /// have in it).
    range: Option<String>,
    /// An `ETag` header (a CalDAV server's version of what it took).
    etag: Option<String>,
    /// What follows the header.
    rest: &'a [u8],
}

fn parse_head(response: &[u8]) -> Result<Head<'_>> {
    let bad = |what: &str| Error::Protocol(format!("HTTP: {what}"));
    let end = response
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .ok_or_else(|| bad("no end of header"))?;
    let head = std::str::from_utf8(&response[..end]).map_err(|_| bad("header is not UTF-8"))?;
    let mut lines = head.split("\r\n");
    let status: u16 = lines
        .next()
        .and_then(|line| line.split(' ').nth(1))
        .and_then(|code| code.parse().ok())
        .ok_or_else(|| bad("no status"))?;
    let mut parsed = Head {
        status,
        chunked: false,
        length: None,
        location: None,
        range: None,
        etag: None,
        rest: &response[end + 4..],
    };
    for line in lines {
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        let value = value.trim();
        match name.trim().to_ascii_lowercase().as_str() {
            "transfer-encoding" => parsed.chunked = value.eq_ignore_ascii_case("chunked"),
            "content-length" => parsed.length = value.parse::<usize>().ok(),
            "location" => parsed.location = Some(value.to_owned()),
            "range" => parsed.range = Some(value.to_owned()),
            "etag" => parsed.etag = Some(value.to_owned()),
            _ => {}
        }
    }
    Ok(parsed)
}

/// The body after `head`; with `cut`, the response may end early and the
/// body is cut to `max_body`.
fn parse_body(head: &Head<'_>, max_body: usize, cut: bool) -> Result<Vec<u8>> {
    let bad = |what: &str| Error::Protocol(format!("HTTP: {what}"));
    let body = head.rest;
    let mut body = if head.chunked {
        dechunk(body, cut).ok_or_else(|| bad("broken chunked body"))?
    } else {
        match head.length {
            Some(length) if length <= body.len() => body[..length].to_vec(),
            Some(_) if cut => body.to_vec(),
            Some(_) => return Err(bad("body shorter than Content-Length")),
            None => body.to_vec(),
        }
    };
    if body.len() > max_body {
        if !cut {
            return Err(bad("body too large"));
        }
        body.truncate(max_body);
    }
    Ok(body)
}

/// Splits a whole HTTP/1.1 response into status, headers and body; with
/// `cut`, the response may end early and the body is cut to `max_body`.
fn parse_response(response: &[u8], max_body: usize, cut: bool) -> Result<Answer> {
    let head = parse_head(response)?;
    match head.status {
        200 => {}
        301 | 302 | 303 | 307 | 308 => {
            return head
                .location
                .map(Answer::Redirect)
                .ok_or_else(|| Error::Protocol("HTTP: redirect without Location".into()));
        }
        other => return Ok(Answer::Status(other)),
    }
    parse_body(&head, max_body, cut).map(Answer::Body)
}

/// Sends a request with a JSON body (or none) and returns the answer's
/// status and body, whatever the status; redirects are not followed. For
/// APIs such as Katna Server's. Besides `https`, `http` is allowed to
/// `localhost` and `127.0.0.1` only, for a server under test.
pub async fn request(
    method: &str,
    url: &str,
    headers: &[(&str, &str)],
    json: Option<&[u8]>,
    tls: &Tls,
    timeout: Duration,
) -> Result<(u16, Vec<u8>)> {
    let body = json.map(|json| ("application/json", json));
    let reply = exchange(method, url, headers, body, None, tls, timeout).await?;
    Ok((reply.status, reply.body))
}

/// The URL in environment variable `var`, when it names a server under
/// test on this computer (`http://127.0.0.1:…` or `http://localhost:…`);
/// anything else is ignored, so it can never send tokens elsewhere.
pub fn test_url(var: &str) -> Option<String> {
    let url = std::env::var(var).ok()?;
    let url = url.trim().trim_end_matches('/');
    (url.starts_with("http://127.0.0.1:") || url.starts_with("http://localhost:"))
        .then(|| url.to_owned())
}

/// What [`exchange`] got back.
#[derive(Debug)]
pub struct Reply {
    pub status: u16,
    pub location: Option<String>,
    pub range: Option<String>,
    pub etag: Option<String>,
    pub body: Vec<u8>,
}

/// Called with how many bytes of the body have gone out.
pub type Progress<'a> = &'a (dyn Fn(usize) + Sync);

/// Like [`request`], with a body of any type, and `sent` told as it goes
/// out; the answer keeps its `Location` and `Range` headers. For uploads.
pub async fn exchange(
    method: &str,
    url: &str,
    headers: &[(&str, &str)],
    body: Option<(&str, &[u8])>,
    sent: Option<Progress<'_>>,
    tls: &Tls,
    timeout: Duration,
) -> Result<Reply> {
    exchange_limited(method, url, headers, body, sent, tls, timeout, MAX_BODY).await
}

/// Like [`exchange`], reading answers of up to `max_body` bytes: for
/// calendar sync and address books, whose listings can be large.
#[allow(clippy::too_many_arguments)]
pub async fn exchange_limited(
    method: &str,
    url: &str,
    headers: &[(&str, &str)],
    body: Option<(&str, &[u8])>,
    sent: Option<Progress<'_>>,
    tls: &Tls,
    timeout: Duration,
    max_body: usize,
) -> Result<Reply> {
    let exchange = async {
        let (parts, plain) = match url.strip_prefix("http://") {
            Some(rest) => {
                let parts = parse_url_loopback(rest)?;
                if !matches!(parts.host, "localhost" | "127.0.0.1") {
                    return Err(Error::Protocol(format!("{url}: http only to localhost")));
                }
                (parts, true)
            }
            None => (parse_url(url)?, false),
        };
        if !method.bytes().all(|b| b.is_ascii_uppercase()) {
            return Err(Error::Protocol(format!("bad method {method}")));
        }
        let mut request = format!(
            "{method} {} HTTP/1.1\r\nHost: {}\r\nUser-Agent: Katna\r\n\
             Accept: application/json\r\nConnection: close\r\n",
            parts.path, parts.host
        );
        if let Some((kind, body)) = body {
            if kind.contains(['\r', '\n']) {
                return Err(Error::Protocol(format!("{url}: bad content type")));
            }
            request.push_str(&format!(
                "Content-Type: {kind}\r\nContent-Length: {}\r\n",
                body.len()
            ));
        } else if method != "GET" {
            request.push_str("Content-Length: 0\r\n");
        }
        for (name, value) in headers {
            if name.contains(['\r', '\n', ':']) || value.contains(['\r', '\n']) {
                return Err(Error::Protocol(format!("{url}: bad header {name}")));
            }
            request.push_str(&format!("{name}: {value}\r\n"));
        }
        request.push_str("\r\n");
        let mut conn = Conn::new(tls.clone());
        if plain {
            conn.connect_tcp(parts.host, parts.port).await?;
        } else {
            conn.connect_tls(parts.host, parts.port).await?;
        }
        conn.write_all(request.as_bytes()).await?;
        if let Some((_, body)) = body {
            // In pieces, so progress shows while a large body goes out.
            let mut done = 0;
            for piece in body.chunks(256 * 1024) {
                conn.write_all(piece).await?;
                done += piece.len();
                if let Some(sent) = sent {
                    sent(done);
                }
            }
        }
        let response = read_answer(&mut conn, url, max_body + 64 * 1024).await?;
        let _ = conn.close().await;
        let head = parse_head(&response)?;
        let body = parse_body(&head, max_body, false)?;
        Ok(Reply {
            status: head.status,
            location: head.location,
            range: head.range,
            etag: head.etag,
            body,
        })
    };
    exchange
        .or(async {
            async_io::Timer::after(timeout).await;
            Err(Error::Timeout(timeout))
        })
        .await
}

/// The parts of `host[:port]/path` after `http://`.
fn parse_url_loopback(rest: &str) -> Result<Url<'_>> {
    let (authority, path) = match rest.find('/') {
        Some(at) => rest.split_at(at),
        None => (rest, "/"),
    };
    let (host, port) = match authority.rsplit_once(':') {
        Some((host, port)) => (
            host,
            port.parse()
                .map_err(|_| Error::Protocol(format!("{rest}: bad port")))?,
        ),
        None => (authority, 80),
    };
    Ok(Url { host, port, path })
}

/// Largest answer [`post_form`] reads; token answers are a few kilobytes.
const MAX_FORM_ANSWER: usize = 256 * 1024;

/// Posts an `application/x-www-form-urlencoded` form to `url` and returns
/// the answer's status and body, whatever the status: OAuth2 token
/// endpoints explain a refusal in the body. Redirects are not followed.
/// Besides `https`, it takes `http` to a loopback address with a port,
/// which only a test's own token server uses.
pub async fn post_form(
    url: &str,
    form: &[(&str, &str)],
    tls: &Tls,
    timeout: Duration,
) -> Result<(u16, Vec<u8>)> {
    let post = async {
        let (secure, https) = match url.strip_prefix("http://") {
            Some(rest) => (false, format!("https://{rest}")),
            None => (true, url.to_owned()),
        };
        let parts = parse_url(&https)?;
        if !secure {
            let loopback = ["127.0.0.1", "localhost", "[::1]"].contains(&parts.host);
            let port_given = https["https://".len()..]
                .split(['/', '?'])
                .next()
                .and_then(|authority| authority.rsplit_once(':'))
                .is_some_and(|(_, port)| port.parse::<u16>().is_ok());
            if !loopback || !port_given {
                return Err(Error::Protocol(format!("{url}: only https URLs are used")));
            }
        }
        let body = form_encode(form);
        let request = format!(
            "POST {} HTTP/1.1\r\nHost: {}\r\nUser-Agent: Katna\r\nAccept: application/json\r\n\
             Content-Type: application/x-www-form-urlencoded\r\nContent-Length: {}\r\n\
             Connection: close\r\n\r\n{body}",
            parts.path,
            parts.host,
            body.len()
        );
        let mut conn = Conn::new(tls.clone());
        if secure {
            conn.connect_tls(parts.host, parts.port).await?;
        } else {
            conn.connect_tcp(parts.host.trim_matches(['[', ']']), parts.port)
                .await?;
        }
        conn.write_all(request.as_bytes()).await?;
        let response = read_answer(&mut conn, url, MAX_FORM_ANSWER).await?;
        let _ = conn.close().await;
        let head = parse_head(&response)?;
        let body = parse_body(&head, MAX_FORM_ANSWER, false)?;
        Ok((head.status, body))
    };
    post.or(async {
        async_io::Timer::after(timeout).await;
        Err(Error::Timeout(timeout))
    })
    .await
}

/// Reads the answer to a `Connection: close` request up to where its own
/// framing (`Content-Length`, or a chunked body's last chunk) says it
/// ends, or else to the end of the connection. Stopping there means a
/// server, or a network box on the way, that drops the connection without
/// TLS's `close_notify` (seen from Google's token server on a mobile
/// network) costs nothing once the whole answer is in.
async fn read_answer(conn: &mut Conn, url: &str, limit: usize) -> Result<Vec<u8>> {
    let mut response = Vec::new();
    loop {
        let chunk = match conn.read_raw().await {
            Ok(chunk) => chunk,
            Err(Error::Io(err)) if err.kind() == std::io::ErrorKind::UnexpectedEof => {
                if answer_complete(&response) {
                    break;
                }
                return Err(Error::Closed(format!(
                    "{url}: the connection ended in the middle of the answer"
                )));
            }
            Err(err) => return Err(err),
        };
        if chunk.is_empty() {
            break;
        }
        response.extend_from_slice(chunk);
        if answer_complete(&response) {
            break;
        }
        if response.len() > limit {
            return Err(Error::Protocol(format!("{url}: answer too large")));
        }
    }
    Ok(response)
}

/// Whether `response` holds a whole answer by its own framing.
fn answer_complete(response: &[u8]) -> bool {
    let Ok(head) = parse_head(response) else {
        return false;
    };
    // These never have a body.
    if matches!(head.status, 204 | 304) {
        return true;
    }
    if head.chunked {
        dechunk(head.rest, false).is_some()
    } else {
        head.length.is_some_and(|length| head.rest.len() >= length)
    }
}

/// `name=value&…`, escaped for a form or a URL query.
pub fn form_encode(fields: &[(&str, &str)]) -> String {
    fields
        .iter()
        .map(|(name, value)| format!("{}={}", escape(name), escape(value)))
        .collect::<Vec<_>>()
        .join("&")
}

/// Percent-escapes everything but RFC 3986's unreserved characters.
pub fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for byte in text.bytes() {
        if byte.is_ascii_alphanumeric() || b"-._~".contains(&byte) {
            out.push(char::from(byte));
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
    }
    out
}

/// The data of a chunked body; with `cut`, what arrived of a body that
/// was not read to its end.
fn dechunk(mut data: &[u8], cut: bool) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    loop {
        let Some(line_end) = data.windows(2).position(|w| w == b"\r\n") else {
            return cut.then_some(out);
        };
        let size = std::str::from_utf8(&data[..line_end]).ok()?;
        let size = size.split(';').next()?.trim();
        let size = usize::from_str_radix(size, 16).ok()?;
        data = &data[line_end + 2..];
        if size == 0 {
            return Some(out);
        }
        match data.get(..size) {
            Some(chunk) => out.extend_from_slice(chunk),
            None if cut => {
                out.extend_from_slice(data);
                return Some(out);
            }
            None => return None,
        }
        data = match data.get(size + 2..) {
            Some(rest) => rest,
            None if cut => return Some(out),
            None => return None,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_end_where_their_framing_says() {
        let whole = b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\n{}";
        assert!(answer_complete(whole));
        assert!(!answer_complete(&whole[..whole.len() - 1]));
        assert!(!answer_complete(b"HTTP/1.1 200 OK\r\nContent-Len"));
        let chunked = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n2\r\n{}\r\n0\r\n\r\n";
        assert!(answer_complete(chunked));
        assert!(!answer_complete(&chunked[..chunked.len() - 7]));
        assert!(answer_complete(b"HTTP/1.1 204 No Content\r\n\r\n"));
        // Without framing, only the end of the connection ends it.
        assert!(!answer_complete(b"HTTP/1.1 200 OK\r\n\r\n{}"));
    }

    /// A server that keeps the connection open after its answer, as one
    /// whose close the network loses does, still answers at once.
    #[test]
    fn a_whole_answer_needs_no_close() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = std::thread::spawn(move || {
            use std::io::{Read, Write};
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0; 4096];
            let _ = stream.read(&mut request).unwrap();
            stream
                .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 18\r\n\r\n{\"access_token\":1}")
                .unwrap();
            std::thread::sleep(Duration::from_secs(5));
        });
        let tls = Tls::insecure_for_local_tests();
        let started = std::time::Instant::now();
        let (status, body) = async_io::block_on(post_form(
            &format!("http://127.0.0.1:{port}/token"),
            &[("code", "c")],
            &tls,
            Duration::from_secs(3),
        ))
        .unwrap();
        assert_eq!(
            (status, body.as_slice()),
            (200, &b"{\"access_token\":1}"[..])
        );
        assert!(started.elapsed() < Duration::from_secs(2));
        server.join().unwrap();
    }

    #[test]
    fn status_lines() {
        assert_eq!(status(b"HTTP/1.1 200 OK\r\nServer: x\r\n"), Some(200));
        assert_eq!(status(b"HTTP/1.1 429 Too Many Requests\r\n"), Some(429));
        assert_eq!(status(b"garbage"), None);
        assert_eq!(status(b""), None);
    }

    #[test]
    fn urls() {
        assert_eq!(
            parse_url("https://autoconfig.example.org/mail/config-v1.1.xml?a=b").unwrap(),
            Url {
                host: "autoconfig.example.org",
                port: 443,
                path: "/mail/config-v1.1.xml?a=b"
            }
        );
        assert_eq!(parse_url("https://127.0.0.1:8443").unwrap().path, "/");
        assert!(parse_url("http://example.org/").is_err());
        assert!(parse_url("https://user@example.org/").is_err());
        // No second request smuggled in through a line break.
        assert!(parse_url("https://example.org/a\r\nX-Evil: 1\r\n\r\nGET /b").is_err());
        assert!(parse_url("https://example.org/a b").is_err());
        assert!(parse_url("https://example.org\t/a").is_err());
        assert_eq!(
            resolve("https://a.org/x", "/y").unwrap(),
            "https://a.org:443/y"
        );
        assert!(resolve("https://a.org/x", "http://b.org/").is_err());
    }

    #[test]
    fn bodies() {
        let plain = b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello!!";
        assert!(
            matches!(parse_response(plain, MAX_BODY, false).unwrap(), Answer::Body(b) if b == b"hello")
        );
        let chunked = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n\
                        3\r\nhel\r\n2;x=y\r\nlo\r\n0\r\n\r\n";
        assert!(
            matches!(parse_response(chunked, MAX_BODY, false).unwrap(), Answer::Body(b) if b == b"hello")
        );
        let moved = b"HTTP/1.1 301 Moved\r\nLocation: https://b.org/c\r\n\r\n";
        assert!(
            matches!(parse_response(moved, MAX_BODY, false).unwrap(), Answer::Redirect(l) if l == "https://b.org/c")
        );
        let missing = b"HTTP/1.0 404 Not Found\r\n\r\n";
        assert!(matches!(
            parse_response(missing, MAX_BODY, false).unwrap(),
            Answer::Status(404)
        ));
        assert!(parse_response(b"HTTP/1.1 200 OK\r\n", MAX_BODY, false).is_err());
    }

    #[test]
    fn forms() {
        assert_eq!(
            form_encode(&[("a", "x y"), ("redirect_uri", "http://127.0.0.1:5/")]),
            "a=x%20y&redirect_uri=http%3A%2F%2F127.0.0.1%3A5%2F"
        );
    }

    #[test]
    fn any_status_keeps_its_body() {
        let refused = b"HTTP/1.1 403 Forbidden\r\nContent-Length: 18\r\n\r\n{\"code\":\"sign_in\"}";
        let head = parse_head(refused).unwrap();
        assert_eq!(head.status, 403);
        assert_eq!(
            parse_body(&head, MAX_BODY, false).unwrap(),
            b"{\"code\":\"sign_in\"}"
        );
        let url = parse_url_loopback("127.0.0.1:8080/api/v1/account").unwrap();
        assert_eq!(
            (url.host, url.port, url.path),
            ("127.0.0.1", 8080, "/api/v1/account")
        );
    }

    #[test]
    fn cut_bodies() {
        // A page read only up to its head: cut instead of refused.
        let long = b"HTTP/1.1 200 OK\r\nContent-Length: 99999\r\n\r\n<head>x</head><body>";
        assert!(parse_response(long, MAX_BODY, false).is_err());
        assert!(
            matches!(parse_response(long, 10, true).unwrap(), Answer::Body(b) if b == b"<head>x</h")
        );
        let chunked = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n\
                        3\r\nhel\r\n20\r\nlo wor";
        assert!(parse_response(chunked, MAX_BODY, false).is_err());
        assert!(
            matches!(parse_response(chunked, MAX_BODY, true).unwrap(), Answer::Body(b) if b == b"hello wor")
        );
    }
}
