// SPDX-License-Identifier: GPL-3.0-or-later

//! The Drive client against a fake Drive on the loopback.

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use katna_core::OAuthProvider;

use super::*;
use crate::oauth::Provider;

/// A request as the fake Drive saw it.
#[derive(Debug, Clone)]
struct Seen {
    method: String,
    path: String,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

impl Seen {
    fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }
}

#[derive(Default)]
struct State {
    seen: Vec<Seen>,
    /// The bytes of the upload so far.
    stored: Vec<u8>,
    /// Drops the connection in the middle of this many chunk uploads.
    drop_chunks: usize,
    /// Answers with 403 and no Drive scope.
    no_scope: bool,
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

fn answer(stream: &mut TcpStream, status: u16, headers: &[(&str, String)], body: &str) {
    let mut text = format!("HTTP/1.1 {status} X\r\n");
    for (name, value) in headers {
        text.push_str(&format!("{name}: {value}\r\n"));
    }
    text.push_str(&format!(
        "Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    ));
    let _ = stream.write_all(text.as_bytes());
}

/// Starts a fake Drive; returns its address and state.
fn fake_drive(state: State) -> (String, Arc<Mutex<State>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
    let state = Arc::new(Mutex::new(state));
    let shared = state.clone();
    let session = format!("{base}/session/1");
    thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let Some(request) = read_request(&mut stream) else {
                continue;
            };
            let mut state = shared.lock().unwrap();
            state.seen.push(request.clone());
            if state.no_scope {
                answer(
                    &mut stream,
                    403,
                    &[],
                    r#"{"error":{"code":403,"message":"Request had insufficient authentication scopes.","errors":[{"reason":"insufficientPermissions"}]}}"#,
                );
                continue;
            }
            match (request.method.as_str(), request.path.as_str()) {
                ("POST", path)
                    if path.starts_with("/upload/drive/v3/files?uploadType=resumable") =>
                {
                    answer(&mut stream, 200, &[("Location", session.clone())], "");
                }
                ("PUT", "/session/1") => {
                    let range = request.header("Content-Range").unwrap().to_owned();
                    let total: usize = range.rsplit('/').next().unwrap().parse().unwrap();
                    if !range.starts_with("bytes */") {
                        let start: usize = range["bytes ".len()..]
                            .split('-')
                            .next()
                            .unwrap()
                            .parse()
                            .unwrap();
                        assert_eq!(start, state.stored.len(), "uploads go on where Drive is");
                        if state.drop_chunks > 0 {
                            // Half of it arrived, then the line broke.
                            state.drop_chunks -= 1;
                            let half = request.body.len() / 2 / (256 * 1024) * (256 * 1024);
                            state.stored.extend_from_slice(&request.body[..half]);
                            continue;
                        }
                        state.stored.extend_from_slice(&request.body);
                    }
                    if state.stored.len() == total {
                        answer(
                            &mut stream,
                            200,
                            &[],
                            r#"{"id":"file-1","webViewLink":"https://drive.test/file/d/file-1/view"}"#,
                        );
                    } else {
                        let range = if state.stored.is_empty() {
                            Vec::new()
                        } else {
                            vec![("Range", format!("bytes=0-{}", state.stored.len() - 1))]
                        };
                        answer(&mut stream, 308, &range, "");
                    }
                }
                ("POST", path) if path.starts_with("/drive/v3/files/file-1/permissions") => {
                    let body = String::from_utf8_lossy(&request.body);
                    if body.contains("@outside.test") {
                        answer(
                            &mut stream,
                            400,
                            &[],
                            r#"{"error":{"code":400,"message":"Bad Request. User message: \"You are trying to invite someone@outside.test. Since there is no Google account associated with this email address, you must check the Notify people box to invite this recipient.\"","errors":[{"reason":"invalidSharingRequest"}]}}"#,
                        );
                    } else {
                        answer(&mut stream, 200, &[], r#"{"id":"p1"}"#);
                    }
                }
                ("PATCH", path) if path.starts_with("/drive/v3/files/file-1") => {
                    answer(&mut stream, 200, &[], r#"{"id":"file-1"}"#);
                }
                _ => answer(&mut stream, 404, &[], "{}"),
            }
        }
    });
    (base, state)
}

fn provider() -> Provider {
    Provider {
        kind: OAuthProvider::Google,
        auth_url: "https://accounts.test/auth".into(),
        token_url: "http://127.0.0.1:1/token".into(),
        client_id: "katna-test".into(),
        client_secret: "not-secret".into(),
        scope: format!("https://mail.test/ {GOOGLE_DRIVE_FILE}"),
        redirect_host: "127.0.0.1",
        tls: Tls::insecure_for_local_tests(),
    }
}

fn drive(api: &str) -> Drive {
    let tokens = TokenSource::new(provider(), "rt".into(), None)
        .with_access_token("at-1".into(), Duration::from_secs(3600))
        .with_scope(Some(format!(
            "https://mail.test/ {GOOGLE_DRIVE_FILE} openid"
        )));
    Drive::with_api(Arc::new(tokens), Tls::insecure_for_local_tests(), api)
}

fn file_of(size: usize) -> (tempfile::NamedTempFile, Vec<u8>) {
    let data: Vec<u8> = (0..size).map(|i| (i * 7 % 251) as u8).collect();
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(&data).unwrap();
    (file, data)
}

#[test]
fn uploads_in_chunks_and_gives_the_link() {
    let (api, state) = fake_drive(State::default());
    let (file, data) = file_of(CHUNK * 2 + 1234);
    let reported = Mutex::new(Vec::new());
    let uploaded = smol::block_on(drive(&api).upload(
        file.path(),
        "holiday video.mp4",
        "video/mp4",
        &|done, size| reported.lock().unwrap().push((done, size)),
    ))
    .unwrap();
    assert_eq!(uploaded.id, "file-1");
    assert_eq!(uploaded.link, "https://drive.test/file/d/file-1/view");
    let state = state.lock().unwrap();
    assert_eq!(state.stored, data);
    let start = &state.seen[0];
    assert_eq!(start.header("Authorization"), Some("Bearer at-1"));
    assert_eq!(start.header("X-Upload-Content-Type"), Some("video/mp4"));
    let size = data.len().to_string();
    assert_eq!(start.header("X-Upload-Content-Length"), Some(size.as_str()));
    assert!(String::from_utf8_lossy(&start.body).contains("holiday video.mp4"));
    // Three chunks after the start.
    assert_eq!(state.seen.len(), 4);
    let reported = reported.lock().unwrap();
    assert_eq!(reported.first(), Some(&(0, data.len() as u64)));
    assert_eq!(
        reported.last(),
        Some(&(data.len() as u64, data.len() as u64))
    );
    assert!(
        reported.windows(2).all(|w| w[0].0 <= w[1].0),
        "progress only grows"
    );
}

#[test]
fn a_broken_connection_resumes_where_drive_stopped() {
    let (api, state) = fake_drive(State {
        drop_chunks: 1,
        ..State::default()
    });
    let (file, data) = file_of(CHUNK + 5000);
    smol::block_on(drive(&api).upload(file.path(), "big.zip", "application/zip", &|_, _| {}))
        .unwrap();
    let state = state.lock().unwrap();
    assert_eq!(state.stored, data, "no byte twice, none missing");
    // Start, the broken chunk, the question how much arrived, the rest.
    let asked = state
        .seen
        .iter()
        .filter(|s| {
            s.header("Content-Range")
                .is_some_and(|r| r.starts_with("bytes */"))
        })
        .count();
    assert_eq!(asked, 1);
}

#[test]
fn shares_and_names_who_it_could_not_share_with() {
    let (api, state) = fake_drive(State::default());
    let drive = drive(&api);
    let refused = smol::block_on(drive.share(
        "file-1",
        &["ana@gmail.test".into(), "bo@outside.test".into()],
    ))
    .unwrap();
    assert_eq!(refused, ["bo@outside.test"]);
    smol::block_on(drive.share_with_link("file-1")).unwrap();
    smol::block_on(drive.remove("file-1")).unwrap();
    let state = state.lock().unwrap();
    let first = &state.seen[0];
    assert!(first.path.contains("sendNotificationEmail=false"));
    assert!(String::from_utf8_lossy(&first.body).contains(r#""role":"reader""#));
    assert!(String::from_utf8_lossy(&state.seen[2].body).contains(r#""type":"anyone""#));
    assert_eq!(state.seen[3].method, "PATCH");
    assert!(String::from_utf8_lossy(&state.seen[3].body).contains("trashed"));
}

#[test]
fn a_missing_drive_scope_asks_to_sign_in_again() {
    let (api, _) = fake_drive(State {
        no_scope: true,
        ..State::default()
    });
    let (file, _) = file_of(10);
    let err = smol::block_on(drive(&api).upload(
        file.path(),
        "a.bin",
        "application/octet-stream",
        &|_, _| {},
    ))
    .unwrap_err();
    assert!(matches!(err, Error::Auth(_)), "{err}");
}

#[test]
fn knows_whether_sign_in_allowed_drive() {
    let drive = drive("http://127.0.0.1:1");
    assert!(smol::block_on(drive.allowed()).unwrap());
    let old = TokenSource::new(provider(), "rt".into(), None)
        .with_scope(Some("https://mail.google.com/ openid email".into()));
    assert_eq!(old.granted(GOOGLE_DRIVE_FILE), Some(false));
}

#[test]
fn next_offsets() {
    assert_eq!(next_offset(None), 0);
    assert_eq!(next_offset(Some("bytes=0-262143")), 262_144);
    assert_eq!(next_offset(Some("junk")), 0);
}
