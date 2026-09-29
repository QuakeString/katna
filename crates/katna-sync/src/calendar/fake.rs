// SPDX-License-Identifier: GPL-3.0-or-later

//! A fake HTTP server on the loopback for the calendar clients' tests:
//! a token endpoint and whatever API the test answers for. Never a real
//! server.

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

/// A request the fake server got.
#[derive(Debug, Clone)]
pub struct Seen {
    pub method: String,
    /// Path and query.
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Seen {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }

    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).into_owned()
    }

    /// The path without its query.
    pub fn route(&self) -> &str {
        self.path.split('?').next().unwrap_or_default()
    }

    /// Query parameter `name`, still escaped.
    pub fn query(&self, name: &str) -> Option<&str> {
        let (_, query) = self.path.split_once('?')?;
        query
            .split('&')
            .filter_map(|pair| pair.split_once('='))
            .find(|(n, _)| *n == name)
            .map(|(_, v)| v)
    }
}

/// An answer: status, content type, body and a `Location`, if any.
pub struct Answer {
    pub status: u16,
    pub kind: &'static str,
    pub body: String,
    pub location: Option<String>,
}

impl Answer {
    pub fn json(status: u16, body: impl Into<String>) -> Self {
        Self {
            status,
            kind: "application/json",
            body: body.into(),
            location: None,
        }
    }

    pub fn xml(status: u16, body: impl Into<String>) -> Self {
        Self {
            status,
            kind: "application/xml; charset=utf-8",
            body: body.into(),
            location: None,
        }
    }

    pub fn redirect(to: &str) -> Self {
        Self {
            status: 301,
            kind: "text/plain",
            body: String::new(),
            location: Some(to.to_owned()),
        }
    }
}

fn read_request(stream: &mut TcpStream) -> Option<Seen> {
    let mut data = Vec::new();
    let mut buf = [0; 64 * 1024];
    loop {
        let n = stream.read(&mut buf).ok()?;
        if n == 0 {
            return None;
        }
        data.extend_from_slice(&buf[..n]);
        let Some(end) = data.windows(4).position(|w| w == b"\r\n\r\n") else {
            continue;
        };
        let head = String::from_utf8_lossy(&data[..end]).into_owned();
        let mut lines = head.split("\r\n");
        let mut first = lines.next()?.split(' ');
        let method = first.next()?.to_owned();
        let path = first.next()?.to_owned();
        let headers: Vec<(String, String)> = lines
            .filter_map(|l| l.split_once(':'))
            .map(|(n, v)| (n.trim().to_owned(), v.trim().to_owned()))
            .collect();
        let length = headers
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case("content-length"))
            .map_or(0, |(_, v)| v.parse::<usize>().unwrap());
        while data.len() < end + 4 + length {
            let n = stream.read(&mut buf).ok()?;
            if n == 0 {
                return None;
            }
            data.extend_from_slice(&buf[..n]);
        }
        return Some(Seen {
            method,
            path,
            headers,
            body: data[end + 4..end + 4 + length].to_vec(),
        });
    }
}

fn write_answer(stream: &mut TcpStream, answer: &Answer) {
    let location = answer
        .location
        .as_ref()
        .map(|l| format!("Location: {l}\r\n"))
        .unwrap_or_default();
    let text = format!(
        "HTTP/1.1 {} X\r\nContent-Type: {}\r\n{location}\
         Content-Length: {}\r\nConnection: close\r\n\r\n{}",
        answer.status,
        answer.kind,
        answer.body.len(),
        answer.body
    );
    let _ = stream.write_all(text.as_bytes());
}

/// Starts a server that answers every request with `handler`; returns
/// its address (`http://127.0.0.1:port`) and the requests it saw.
pub fn serve(
    handler: impl Fn(&Seen) -> Answer + Send + 'static,
) -> (String, Arc<Mutex<Vec<Seen>>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
    let seen = Arc::new(Mutex::new(Vec::new()));
    let shared = seen.clone();
    thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let Some(request) = read_request(&mut stream) else {
                continue;
            };
            shared.lock().unwrap().push(request.clone());
            let answer = handler(&request);
            write_answer(&mut stream, &answer);
        }
    });
    (base, seen)
}
