// SPDX-License-Identifier: GPL-3.0-or-later

//! The OneDrive client against a fake Microsoft (token endpoint and Graph)
//! on the loopback.

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use katna_core::OAuthProvider;

use super::*;
use crate::oauth::Provider;

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

    fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).into_owned()
    }
}

#[derive(Default)]
struct State {
    seen: Vec<Seen>,
    stored: Vec<u8>,
    /// Drops the connection in the middle of this many chunk uploads.
    drop_chunks: usize,
    /// The sign-in never allowed OneDrive.
    no_consent: bool,
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

fn answer(stream: &mut TcpStream, status: u16, body: &str) {
    let text = format!(
        "HTTP/1.1 {status} X\r\nContent-Type: application/json\r\n\
         Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    let _ = stream.write_all(text.as_bytes());
}

/// Starts a fake Microsoft; returns its address and state.
fn fake_microsoft(state: State) -> (String, Arc<Mutex<State>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
    let state = Arc::new(Mutex::new(state));
    let shared = state.clone();
    let session = format!("{base}/up/session-1");
    thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let Some(request) = read_request(&mut stream) else {
                continue;
            };
            let mut state = shared.lock().unwrap();
            state.seen.push(request.clone());
            let bearer = request.header("Authorization").is_some();
            match (request.method.as_str(), request.path.as_str()) {
                ("POST", "/token") => {
                    if state.no_consent {
                        answer(
                            &mut stream,
                            400,
                            r#"{"error":"invalid_grant","error_description":"AADSTS65001: The user or administrator has not consented to use the application."}"#,
                        );
                    } else {
                        answer(
                            &mut stream,
                            200,
                            r#"{"access_token":"graph-1","expires_in":3600,"refresh_token":"rt-2"}"#,
                        );
                    }
                }
                ("POST", path) if path.ends_with(":/createUploadSession") => {
                    assert!(bearer);
                    answer(&mut stream, 200, &format!(r#"{{"uploadUrl":"{session}"}}"#));
                }
                ("PUT", "/up/session-1") => {
                    assert!(!bearer, "the session address carries its own authorisation");
                    let range = request.header("Content-Range").unwrap().to_owned();
                    let total: usize = range.rsplit('/').next().unwrap().parse().unwrap();
                    let start: usize = range["bytes ".len()..]
                        .split('-')
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap();
                    assert_eq!(start, state.stored.len(), "uploads go on where OneDrive is");
                    if state.drop_chunks > 0 {
                        state.drop_chunks -= 1;
                        let half = request.body.len() / 2 / (320 * 1024) * (320 * 1024);
                        state.stored.extend_from_slice(&request.body[..half]);
                        continue;
                    }
                    state.stored.extend_from_slice(&request.body);
                    if state.stored.len() == total {
                        answer(
                            &mut stream,
                            201,
                            r#"{"id":"item-1","webUrl":"https://onedrive.test/item-1"}"#,
                        );
                    } else {
                        answer(
                            &mut stream,
                            202,
                            &format!(r#"{{"nextExpectedRanges":["{}-"]}}"#, state.stored.len()),
                        );
                    }
                }
                ("GET", "/up/session-1") => {
                    answer(
                        &mut stream,
                        200,
                        &format!(r#"{{"nextExpectedRanges":["{}-"]}}"#, state.stored.len()),
                    );
                }
                ("POST", "/me/drive/items/item-1/invite") => {
                    if request.text().contains("@outside.test") {
                        answer(
                            &mut stream,
                            400,
                            r#"{"error":{"code":"invalidRequest","message":"One or more recipients could not be resolved."}}"#,
                        );
                    } else {
                        answer(&mut stream, 200, r#"{"value":[{"id":"p1"}]}"#);
                    }
                }
                ("POST", "/me/drive/items/item-1/createLink") => {
                    answer(
                        &mut stream,
                        200,
                        r#"{"link":{"webUrl":"https://1drv.test/v/anyone"}}"#,
                    );
                }
                ("DELETE", "/me/drive/items/item-1") => answer(&mut stream, 204, ""),
                _ => answer(&mut stream, 404, "{}"),
            }
        }
    });
    (base, state)
}

fn onedrive(base: &str) -> OneDrive {
    let provider = Provider {
        kind: OAuthProvider::Microsoft,
        auth_url: "https://login.test/authorize".into(),
        token_url: format!("{base}/token"),
        client_id: "katna-test".into(),
        client_secret: String::new(),
        scope: "https://outlook.test/IMAP.AccessAsUser.All offline_access".into(),
        consent: MICROSOFT_FILES.into(),
        redirect_host: "localhost",
        tls: Tls::insecure_for_local_tests(),
    };
    let tokens = TokenSource::new(provider, "rt-1".into(), None);
    OneDrive::with_api(Arc::new(tokens), Tls::insecure_for_local_tests(), base)
}

fn file_of(size: usize) -> (tempfile::NamedTempFile, Vec<u8>) {
    let data: Vec<u8> = (0..size).map(|i| (i * 7 % 251) as u8).collect();
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(&data).unwrap();
    (file, data)
}

#[test]
fn uploads_to_katnas_folder_in_chunks() {
    let (base, state) = fake_microsoft(State::default());
    let (file, data) = file_of(CHUNK * 2 + 1234);
    let reported = Mutex::new(Vec::new());
    let uploaded = smol::block_on(onedrive(&base).upload(
        file.path(),
        "holiday video.mp4",
        &|done, size| reported.lock().unwrap().push((done, size)),
    ))
    .unwrap();
    assert_eq!(uploaded.id, "item-1");
    assert_eq!(uploaded.link, "https://onedrive.test/item-1");
    let state = state.lock().unwrap();
    assert_eq!(state.stored, data);
    // Graph's token comes from the refresh token, for Graph only.
    let token = &state.seen[0];
    assert_eq!(token.path, "/token");
    let form = token.text();
    assert!(
        form.contains("graph.microsoft.com%2FFiles.ReadWrite"),
        "{form}"
    );
    assert!(!form.contains("outlook"), "{form}");
    let start = &state.seen[1];
    assert_eq!(
        start.path,
        "/me/drive/special/approot:/holiday%20video.mp4:/createUploadSession"
    );
    assert_eq!(start.header("Authorization"), Some("Bearer graph-1"));
    assert!(start.text().contains("rename"));
    // Token, start, three chunks.
    assert_eq!(state.seen.len(), 5);
    let reported = reported.lock().unwrap();
    assert_eq!(
        reported.last(),
        Some(&(data.len() as u64, data.len() as u64))
    );
    assert!(reported.windows(2).all(|w| w[0].0 <= w[1].0));
}

#[test]
fn a_broken_connection_resumes_where_onedrive_stopped() {
    let (base, state) = fake_microsoft(State {
        drop_chunks: 1,
        ..State::default()
    });
    let (file, data) = file_of(CHUNK + 5000);
    smol::block_on(onedrive(&base).upload(file.path(), "big.zip", &|_, _| {})).unwrap();
    let state = state.lock().unwrap();
    assert_eq!(state.stored, data, "no byte twice, none missing");
    assert_eq!(state.seen.iter().filter(|s| s.method == "GET").count(), 1);
}

#[test]
fn shares_quietly_and_names_who_it_could_not_share_with() {
    let (base, state) = fake_microsoft(State::default());
    let onedrive = onedrive(&base);
    let refused = smol::block_on(onedrive.share(
        "item-1",
        &["ana@outlook.test".into(), "bo@outside.test".into()],
    ))
    .unwrap();
    assert_eq!(refused, ["bo@outside.test"]);
    let link = smol::block_on(onedrive.share_with_link("item-1")).unwrap();
    assert_eq!(link, "https://1drv.test/v/anyone");
    smol::block_on(onedrive.remove("item-1")).unwrap();
    let state = state.lock().unwrap();
    let invite = state.seen[1].text();
    assert!(invite.contains(r#""sendInvitation":false"#), "{invite}");
    assert!(invite.contains(r#""read""#), "{invite}");
    assert!(state.seen[3].text().contains(r#""scope":"anonymous""#));
    assert_eq!(state.seen[4].method, "DELETE");
}

#[test]
fn a_sign_in_without_onedrive_asks_to_allow_it() {
    let (base, _) = fake_microsoft(State {
        no_consent: true,
        ..State::default()
    });
    let onedrive = onedrive(&base);
    assert!(!smol::block_on(onedrive.allowed()).unwrap());
    let (file, _) = file_of(10);
    let err = smol::block_on(onedrive.upload(file.path(), "a.bin", &|_, _| {})).unwrap_err();
    assert!(matches!(err, Error::Auth(_)), "{err}");
}

#[test]
fn next_offsets() {
    assert_eq!(next_offset(&["26-".into()]), Some(26));
    assert_eq!(next_offset(&["77-99".into(), "26-50".into()]), Some(26));
    assert_eq!(next_offset(&[]), None);
}
