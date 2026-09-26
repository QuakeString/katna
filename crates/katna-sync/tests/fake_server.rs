// SPDX-License-Identifier: GPL-3.0-or-later

//! Protocol edge cases against a scripted IMAP server on localhost. No real
//! server needed, so these run in CI.

use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use futures_lite::FutureExt;
use katna_sync::{
    Credentials, Endpoint, Error, Flags, FolderChange, MailBackend, Security, connection,
    imap::ImapBackend,
    net::{Conn, Tls},
};

const CAPS: &str = "IMAP4rev1 AUTH=PLAIN SASL-IR IDLE";

/// The server side of one connection, run on its own thread.
struct Session {
    reader: BufReader<TcpStream>,
    writer: TcpStream,
    caps: &'static str,
}

impl Session {
    fn send(&mut self, text: &str) {
        self.writer.write_all(text.as_bytes()).unwrap();
        self.writer.flush().unwrap();
    }

    fn line(&mut self) -> String {
        let mut line = String::new();
        self.reader.read_line(&mut line).unwrap();
        assert!(line.ends_with("\r\n"), "incomplete line {line:?}");
        line.truncate(line.len() - 2);
        line
    }

    /// Reads the next command, answering CAPABILITY on the way. Returns
    /// the tag and the rest of the line.
    fn command(&mut self) -> (String, String) {
        loop {
            let line = self.line();
            let (tag, rest) = line.split_once(' ').expect("tagged command");
            if rest.eq_ignore_ascii_case("CAPABILITY") {
                let caps = self.caps;
                self.send(&format!("* CAPABILITY {caps}\r\n{tag} OK done\r\n"));
                continue;
            }
            return (tag.to_owned(), rest.to_owned());
        }
    }

    /// Reads a command and checks its name. Returns the tag.
    fn expect(&mut self, name: &str) -> String {
        let (tag, rest) = self.command();
        let got = rest.split(' ').next().unwrap();
        assert!(
            got.eq_ignore_ascii_case(name),
            "expected {name}, got {rest:?}"
        );
        tag
    }

    fn ok(&mut self, tag: &str) {
        self.send(&format!("{tag} OK done\r\n"));
    }

    /// Greeting and SASL PLAIN login.
    fn login(&mut self) {
        let caps = self.caps;
        self.send(&format!("* OK [CAPABILITY {caps}] fake server ready\r\n"));
        let (tag, rest) = self.command();
        assert!(
            rest.to_ascii_uppercase().starts_with("AUTHENTICATE PLAIN"),
            "{rest}"
        );
        if !rest.contains("PLAIN ") {
            self.send("+ \r\n");
            self.line();
        }
        self.send(&format!("{tag} OK [CAPABILITY {caps}] logged in\r\n"));
    }

    /// Answers the NOOP that starts every IDLE wait.
    fn quiet_noop(&mut self) {
        let tag = self.expect("NOOP");
        self.ok(&tag);
    }
}

/// Starts a one-connection server running `script` after the login.
fn serve(
    caps: &'static str,
    script: impl FnOnce(&mut Session) + Send + 'static,
) -> (Endpoint, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let handle = thread::spawn(move || {
        let (stream, _) = listener.accept().unwrap();
        let mut session = Session {
            reader: BufReader::new(stream.try_clone().unwrap()),
            writer: stream,
            caps,
        };
        session.login();
        script(&mut session);
    });
    (Endpoint::new("127.0.0.1", port, Security::Plain), handle)
}

async fn connect(endpoint: &Endpoint) -> ImapBackend {
    let creds = Credentials::new("alice", "secret");
    ImapBackend::connect(endpoint, &creds, Tls::insecure_for_local_tests())
        .await
        .unwrap()
}

fn never() -> impl Future<Output = ()> + Send {
    futures_lite::future::pending()
}

#[test]
fn noop_reports_updates() {
    let (endpoint, server) = serve(CAPS, |s| {
        let tag = s.expect("NOOP");
        s.send(&format!(
            "* 12 EXISTS\r\n* 3 EXPUNGE\r\n* 5 FETCH (FLAGS (\\Seen $Forwarded))\r\n{tag} OK done\r\n"
        ));
    });
    let changes = smol::block_on(async { connect(&endpoint).await.poll_changes().await.unwrap() });
    assert_eq!(
        changes,
        vec![
            FolderChange::Exists(12),
            FolderChange::Expunged(3),
            FolderChange::FlagsChanged {
                seq: 5,
                flags: Flags {
                    seen: true,
                    keywords: vec!["$Forwarded".into()],
                    ..Flags::default()
                }
            },
        ]
    );
    server.join().unwrap();
}

#[test]
fn idle_returns_pushed_update() {
    let (endpoint, server) = serve(CAPS, |s| {
        s.quiet_noop();
        let tag = s.expect("IDLE");
        s.send("+ idling\r\n");
        thread::sleep(Duration::from_millis(100));
        s.send("* 4 EXISTS\r\n");
        assert_eq!(s.line(), "DONE");
        s.ok(&tag);
    });
    let started = Instant::now();
    let wait = smol::block_on(async {
        let mut imap = connect(&endpoint).await;
        imap.wait_for_changes(Duration::from_secs(30), never())
            .await
            .unwrap()
    });
    assert_eq!(wait.changes, vec![FolderChange::Exists(4)]);
    assert!(wait.interrupted.is_none());
    assert!(started.elapsed() < Duration::from_secs(5));
    server.join().unwrap();
}

/// io-imap's own IDLE drops this update (S2 problem 2).
#[test]
fn idle_keeps_updates_sent_after_done() {
    let (endpoint, server) = serve(CAPS, |s| {
        s.quiet_noop();
        let tag = s.expect("IDLE");
        s.send("+ idling\r\n");
        assert_eq!(s.line(), "DONE");
        s.send(&format!("* 9 EXISTS\r\n{tag} OK IDLE terminated\r\n"));
    });
    let wait = smol::block_on(async {
        let mut imap = connect(&endpoint).await;
        imap.wait_for_changes(Duration::from_millis(200), never())
            .await
            .unwrap()
    });
    assert_eq!(wait.changes, vec![FolderChange::Exists(9)]);
    server.join().unwrap();
}

#[test]
fn changes_before_idle_skip_the_idle() {
    let (endpoint, server) = serve(CAPS, |s| {
        let tag = s.expect("NOOP");
        s.send(&format!("* 2 EXISTS\r\n{tag} OK done\r\n"));
        let tag = s.expect("LOGOUT");
        s.send(&format!("* BYE bye\r\n{tag} OK done\r\n"));
    });
    smol::block_on(async {
        let mut imap = connect(&endpoint).await;
        let wait = imap
            .wait_for_changes(Duration::from_secs(30), never())
            .await
            .unwrap();
        assert_eq!(wait.changes, vec![FolderChange::Exists(2)]);
        imap.logout().await.unwrap();
    });
    server.join().unwrap();
}

#[test]
fn request_from_another_handle_ends_idle() {
    let (endpoint, server) = serve(CAPS, |s| {
        s.quiet_noop();
        let tag = s.expect("IDLE");
        s.send("+ idling\r\n");
        // Nothing happens until the client needs the connection.
        assert_eq!(s.line(), "DONE");
        s.ok(&tag);
        let tag = s.expect("LIST");
        s.send(&format!(
            "* LIST () \"/\" INBOX\r\n* LIST (\\Sent) \"/\" Sent\r\n{tag} OK done\r\n"
        ));
        let tag = s.expect("LOGOUT");
        s.send(&format!("* BYE bye\r\n{tag} OK done\r\n"));
    });
    smol::block_on(async {
        let (conn, task) = connection::spawn(connect(&endpoint).await);
        let task = smol::spawn(task);
        let waiter = {
            let conn = conn.clone();
            smol::spawn(async move { conn.wait_for_changes(Duration::from_secs(60)).await })
        };
        async_io::Timer::after(Duration::from_millis(200)).await;
        let folders = conn.list_folders().await.unwrap();
        assert_eq!(folders.len(), 2);
        assert_eq!(waiter.await.unwrap(), vec![]);
        conn.logout().await.unwrap();
        task.await;
        assert!(conn.is_closed());
    });
    server.join().unwrap();
}

#[test]
fn bye_during_idle_closes_the_connection() {
    let (endpoint, server) = serve(CAPS, |s| {
        s.quiet_noop();
        s.expect("IDLE");
        s.send("+ idling\r\n");
        s.send("* BYE server shutting down\r\n");
    });
    smol::block_on(async {
        let (conn, task) = connection::spawn(connect(&endpoint).await);
        let task = smol::spawn(task);
        let err = conn
            .wait_for_changes(Duration::from_secs(30))
            .await
            .unwrap_err();
        assert!(matches!(err, Error::Closed(_)), "{err:?}");
        task.await;
        assert!(matches!(conn.poll_changes().await, Err(Error::Closed(_))));
    });
    server.join().unwrap();
}

#[test]
fn without_idle_waits_then_polls() {
    let (endpoint, server) = serve("IMAP4rev1 AUTH=PLAIN SASL-IR", |s| {
        let tag = s.expect("NOOP");
        s.send(&format!("* 7 EXISTS\r\n{tag} OK done\r\n"));
    });
    let started = Instant::now();
    let wait = smol::block_on(async {
        let mut imap = connect(&endpoint).await;
        imap.wait_for_changes(Duration::from_millis(300), never())
            .await
            .unwrap()
    });
    assert!(started.elapsed() >= Duration::from_millis(300));
    assert_eq!(wait.changes, vec![FolderChange::Exists(7)]);
    server.join().unwrap();
}

/// S2 problem 10: the `+` continuation arrives in two pieces.
#[test]
fn append_survives_a_split_continuation() {
    let (endpoint, server) = serve(CAPS, |s| {
        let (tag, rest) = s.command();
        assert!(rest.starts_with("APPEND"), "{rest}");
        let size: usize = rest
            .rsplit_once('{')
            .and_then(|(_, n)| n.strip_suffix('}'))
            .expect("synchronising literal")
            .parse()
            .unwrap();
        s.send("+ O");
        thread::sleep(Duration::from_millis(100));
        s.send("K\r\n");
        let mut literal = vec![0; size];
        s.reader.read_exact(&mut literal).unwrap();
        assert_eq!(literal, b"Subject: hi\r\n\r\nhello\r\n");
        assert_eq!(s.line(), "");
        s.ok(&tag);
        s.quiet_noop();
    });
    smol::block_on(async {
        let mut imap = connect(&endpoint).await;
        imap.append("INBOX", b"Subject: hi\r\n\r\nhello\r\n".to_vec())
            .await
            .unwrap();
        // The tagged OK was read by APPEND, not left for the next command.
        imap.poll_changes().await.unwrap();
    });
    server.join().unwrap();
}

#[test]
fn rejected_command_is_not_fatal() {
    let (endpoint, server) = serve(CAPS, |s| {
        let tag = s.expect("SELECT");
        s.send(&format!("{tag} NO [NONEXISTENT] no such folder\r\n"));
        let tag = s.expect("SELECT");
        s.send(&format!(
            "* 3 EXISTS\r\n* OK [UIDVALIDITY 42] ok\r\n* OK [UIDNEXT 4] ok\r\n{tag} OK [READ-WRITE] done\r\n"
        ));
    });
    smol::block_on(async {
        let mut imap = connect(&endpoint).await;
        let err = imap.select("Missing").await.unwrap_err();
        assert!(matches!(err, Error::Rejected(_)), "{err:?}");
        let status = imap.select("INBOX").await.unwrap();
        assert_eq!(status.exists, 3);
        assert_eq!(status.uid_validity, Some(42));
        assert_eq!(status.uid_next, Some(4));
    });
    server.join().unwrap();
}

/// A read cancelled halfway through a line keeps the bytes it already has.
#[test]
fn cancelled_read_keeps_partial_line() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream.write_all(b"* 1 EXI").unwrap();
        thread::sleep(Duration::from_millis(400));
        stream.write_all(b"STS\r\n").unwrap();
        thread::sleep(Duration::from_millis(200));
    });
    smol::block_on(async {
        let mut conn = Conn::new(Tls::insecure_for_local_tests());
        conn.connect_tcp("127.0.0.1", port).await.unwrap();
        let cancelled = async {
            conn.read_timeout(Duration::from_secs(10))
                .await
                .map(|_| true)
        }
        .or(async {
            async_io::Timer::after(Duration::from_millis(150)).await;
            Ok(false)
        })
        .await
        .unwrap();
        assert!(!cancelled, "the read should still be waiting mid-line");
        let bytes = conn.read().await.unwrap();
        assert_eq!(bytes, b"* 1 EXISTS\r\n");
    });
    server.join().unwrap();
}
