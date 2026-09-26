// SPDX-License-Identifier: GPL-3.0-or-later

//! Just enough HTTPS to fetch a configuration file or an image: `GET`,
//! `Connection: close`, `Content-Length` or chunked bodies, and a few
//! redirects. Only `https` URLs, so a network in the middle cannot hand us
//! its servers or see what is fetched.

use std::time::Duration;

use futures_lite::FutureExt;

use crate::{Error, Result, net::Conn, net::Tls};

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
    fetch(url, tls, timeout, max_body, false).await
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
    fetch(url, tls, timeout, max_body, true).await
}

async fn fetch(
    url: &str,
    tls: &Tls,
    timeout: Duration,
    max_body: usize,
    head: bool,
) -> Result<Option<Vec<u8>>> {
    let fetch = async {
        let mut url = url.to_owned();
        for _ in 0..=MAX_REDIRECTS {
            match get_once(&url, tls, max_body, head).await? {
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
/// body is cut there.
async fn get_once(url: &str, tls: &Tls, max_body: usize, head: bool) -> Result<Answer> {
    let parts = parse_url(url)?;
    let mut conn = Conn::new(tls.clone());
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

/// Splits a whole HTTP/1.1 response into status, headers and body; with
/// `cut`, the response may end early and the body is cut to `max_body`.
fn parse_response(response: &[u8], max_body: usize, cut: bool) -> Result<Answer> {
    let bad = |what: &str| Error::Protocol(format!("HTTP: {what}"));
    let end = response
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .ok_or_else(|| bad("no end of header"))?;
    let head = std::str::from_utf8(&response[..end]).map_err(|_| bad("header is not UTF-8"))?;
    let body = &response[end + 4..];
    let mut lines = head.split("\r\n");
    let status: u16 = lines
        .next()
        .and_then(|line| line.split(' ').nth(1))
        .and_then(|code| code.parse().ok())
        .ok_or_else(|| bad("no status"))?;
    let mut chunked = false;
    let mut length = None;
    let mut location = None;
    for line in lines {
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        let value = value.trim();
        match name.trim().to_ascii_lowercase().as_str() {
            "transfer-encoding" => chunked = value.eq_ignore_ascii_case("chunked"),
            "content-length" => length = value.parse::<usize>().ok(),
            "location" => location = Some(value.to_owned()),
            _ => {}
        }
    }
    match status {
        200 => {}
        301 | 302 | 303 | 307 | 308 => {
            return location
                .map(Answer::Redirect)
                .ok_or_else(|| bad("redirect without Location"));
        }
        other => return Ok(Answer::Status(other)),
    }
    let mut body = if chunked {
        dechunk(body, cut).ok_or_else(|| bad("broken chunked body"))?
    } else {
        match length {
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
    Ok(Answer::Body(body))
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
