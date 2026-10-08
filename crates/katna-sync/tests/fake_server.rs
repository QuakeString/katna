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
    AttachmentPart, Credentials, Endpoint, Error, Flags, FolderChange, MailBackend, Security,
    connection::{self, Connection},
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

/// With QRESYNC enabled, the flag fetch also reports expunges
/// (`VANISHED (EARLIER)`), and pushed expunges arrive as `VANISHED`.
#[test]
fn qresync_reports_vanished_mail() {
    let caps = "IMAP4rev1 AUTH=PLAIN SASL-IR IDLE ENABLE CONDSTORE QRESYNC";
    let (endpoint, server) = serve(caps, |s| {
        let tag = s.expect("ENABLE");
        s.send(&format!("* ENABLED QRESYNC\r\n{tag} OK enabled\r\n"));

        let (tag, rest) = s.command();
        assert_eq!(rest, "UID FETCH 1:10 (UID FLAGS) (CHANGEDSINCE 5 VANISHED)");
        s.send(&format!(
            "* VANISHED (EARLIER) 3:4,7\r\n\
             * 2 FETCH (UID 5 FLAGS (\\Seen) MODSEQ (9))\r\n\
             * 1 FETCH (UID 1 FLAGS () MODSEQ (8))\r\n{tag} OK done\r\n"
        ));

        s.quiet_noop();
        let tag = s.expect("IDLE");
        s.send("+ idling\r\n");
        s.send("* VANISHED 8\r\n");
        assert_eq!(s.line(), "DONE");
        s.ok(&tag);
    });
    smol::block_on(async {
        let mut imap = connect(&endpoint).await;
        let changes = imap.fetch_flags(1, 10, Some(5)).await.unwrap();
        let uids: Vec<_> = changes
            .flags
            .iter()
            .map(|f| (f.uid, f.flags.seen))
            .collect();
        assert_eq!(uids, [(1, false), (5, true)]);
        assert_eq!(changes.vanished, Some(vec![3..=4, 7..=7]));
        let wait = imap
            .wait_for_changes(Duration::from_secs(30), never())
            .await
            .unwrap();
        let expected = FolderChange::Vanished(std::iter::once(8..=8).collect());
        assert_eq!(wait.changes, [expected]);
    });
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
        Connection::logout(&conn).await.unwrap();
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

/// Gmail's thread IDs and `X-GM-RAW` search go out as raw commands,
/// because imap-codec can neither build nor parse them. Written from
/// Gmail's documented responses; not run against Gmail itself.
#[test]
fn gmail_thread_ids_and_search() {
    let (endpoint, server) = serve("IMAP4rev1 AUTH=PLAIN SASL-IR IDLE X-GM-EXT-1", |s| {
        let (tag, rest) = s.command();
        assert!(rest.starts_with("UID FETCH 1:2 ("), "{rest}");
        assert!(rest.contains(" References "), "{rest}");
        assert!(rest.contains(" List-Unsubscribe "), "{rest}");
        s.send(
            "* 1 FETCH (UID 1 FLAGS (\\Seen) RFC822.SIZE 100 \
                 BODY[HEADER.FIELDS (SUBJECT)] {15}\r\nSubject: hi\r\n\r\n)\r\n\
                 * 2 FETCH (UID 2 FLAGS () RFC822.SIZE 200 \
                 BODY[HEADER.FIELDS (SUBJECT)] {15}\r\nSubject: yo\r\n\r\n)\r\n",
        );
        s.ok(&tag);

        let (tag, rest) = s.command();
        assert_eq!(rest, "UID FETCH 1:2 (UID BODYSTRUCTURE)");
        s.ok(&tag);

        let (tag, rest) = s.command();
        assert_eq!(rest, "UID FETCH 1:2 (UID X-GM-THRID X-GM-MSGID)");
        s.send(&format!(
            "* 1 FETCH (X-GM-THRID 1278455344230334865 X-GM-MSGID 1278455344230334866 UID 1)\r\n\
                 * 3 EXISTS\r\n\
                 * 2 FETCH (UID 2 X-GM-THRID 99 X-GM-MSGID 100)\r\n{tag} OK Success\r\n"
        ));

        // Importance is a label on Gmail.
        let (tag, rest) = s.command();
        assert_eq!(rest, "UID FETCH 1:2 (UID X-GM-LABELS)");
        s.send(&format!(
            "* 1 FETCH (X-GM-LABELS (\\Inbox \"\\\\Important\") UID 1)\r\n\
                 * 2 FETCH (UID 2 X-GM-LABELS (\\Inbox Important))\r\n{tag} OK Success\r\n"
        ));
        let (tag, rest) = s.command();
        assert_eq!(rest, "UID STORE 2 +X-GM-LABELS (\\Important)");
        s.send(&format!(
            "* 2 FETCH (UID 2 X-GM-LABELS (\\Inbox \\Important))\r\n{tag} OK Success\r\n"
        ));

        let (tag, rest) = s.command();
        assert_eq!(rest, "UID SEARCH UID 1:* X-GM-RAW \"category:promotions\"");
        s.send(&format!(
            "* SEARCH 2 1\r\n{tag} OK SEARCH completed (Success)\r\n"
        ));

        let (tag, _) = s.command();
        s.send(&format!("{tag} BAD Could not parse command\r\n"));
        s.quiet_noop();
    });
    smol::block_on(async {
        let mut imap = connect(&endpoint).await;
        let headers = imap.fetch_headers(1, Some(2)).await.unwrap();
        let ids: Vec<_> = headers
            .iter()
            .map(|h| (h.uid, h.gm_thread_id, h.gm_msgid))
            .collect();
        assert_eq!(
            ids,
            [
                (
                    1,
                    Some(1_278_455_344_230_334_865),
                    Some(1_278_455_344_230_334_866)
                ),
                (2, Some(99), Some(100))
            ]
        );
        assert_eq!(headers[0].header, b"Subject: hi\r\n\r\n");
        assert_eq!(headers[0].flags.keywords, [katna_sync::IMPORTANT]);
        assert!(headers[1].flags.keywords.is_empty());
        let important = Flags {
            keywords: vec![katna_sync::IMPORTANT.to_owned()],
            ..Flags::default()
        };
        imap.store_flags(&[2], &important, true).await.unwrap();
        let found = imap.gmail_search(1, "category:promotions").await.unwrap();
        assert_eq!(found, Some(vec![1, 2]));
        let err = imap.gmail_search(1, "category:social").await.unwrap_err();
        assert!(matches!(err, Error::Rejected(_)), "{err:?}");
        assert!(
            imap.gmail_search(1, "a\"b").await.is_err(),
            "quotes cannot be sent"
        );
        imap.poll_changes().await.unwrap();
    });
    server.join().unwrap();
}

/// Attachments come from `BODYSTRUCTURE`, in a command of its own: a
/// structure imap-codec cannot parse loses that message's attachment list,
/// not the message.
#[test]
fn attachments_from_the_body_structure() {
    let (endpoint, server) = serve(CAPS, |s| {
        let (tag, rest) = s.command();
        assert!(rest.starts_with("UID FETCH 1:3 ("), "{rest}");
        s.send(
            "* 1 FETCH (UID 1 RFC822.SIZE 100 BODY[HEADER.FIELDS (SUBJECT)] {14}\r\nSubject: a\r\n\r\n)\r\n\
             * 2 FETCH (UID 2 RFC822.SIZE 100 BODY[HEADER.FIELDS (SUBJECT)] {14}\r\nSubject: b\r\n\r\n)\r\n\
             * 3 FETCH (UID 3 RFC822.SIZE 100 BODY[HEADER.FIELDS (SUBJECT)] {14}\r\nSubject: c\r\n\r\n)\r\n",
        );
        s.ok(&tag);

        let (tag, rest) = s.command();
        assert_eq!(rest, "UID FETCH 1:3 (UID BODYSTRUCTURE)");
        // 1: text, a PDF with an RFC 2231 name, an HTML body with an
        // inline logo, and a forwarded message. 2: versions of the text
        // only (Gmail's AMP one, an invitation's calendar).
        // 3: a structure imap-codec rejects.
        s.send(concat!(
            "* 1 FETCH (UID 1 BODYSTRUCTURE (",
            "(\"TEXT\" \"PLAIN\" (\"CHARSET\" \"utf-8\") NIL NIL \"7BIT\" 20 1 NIL NIL NIL NIL)",
            "(\"APPLICATION\" \"PDF\" (\"NAME\" \"rates.pdf\") NIL NIL \"BASE64\" 4000 NIL ",
            "(\"ATTACHMENT\" (\"FILENAME*\" \"utf-8''%E2%82%AC%20rates.pdf\")) NIL NIL)",
            "((\"TEXT\" \"HTML\" (\"CHARSET\" \"utf-8\") NIL NIL \"7BIT\" 30 1 NIL NIL NIL NIL)",
            "(\"IMAGE\" \"PNG\" (\"NAME\" \"logo.png\") \"<logo>\" NIL \"BASE64\" 800 NIL ",
            "(\"INLINE\" (\"FILENAME\" \"logo.png\")) NIL NIL) ",
            "\"RELATED\" (\"BOUNDARY\" \"r\") NIL NIL NIL)",
            "(\"MESSAGE\" \"RFC822\" NIL NIL NIL \"7BIT\" 500 ",
            "(NIL \"Fwd\" NIL NIL NIL NIL NIL NIL NIL NIL) ",
            "(\"TEXT\" \"PLAIN\" (\"CHARSET\" \"us-ascii\") NIL NIL \"7BIT\" 10 1) 20 NIL NIL NIL NIL) ",
            "\"MIXED\" (\"BOUNDARY\" \"b\") NIL NIL NIL))\r\n",
            "* 2 FETCH (UID 2 BODYSTRUCTURE (",
            "(\"TEXT\" \"PLAIN\" (\"CHARSET\" \"us-ascii\") NIL NIL \"7BIT\" 10 1 NIL NIL NIL NIL)",
            "(\"TEXT\" \"HTML\" (\"CHARSET\" \"utf-8\") NIL NIL \"7BIT\" 30 1 NIL NIL NIL NIL)",
            "(\"TEXT\" \"X-AMP-HTML\" (\"CHARSET\" \"utf-8\") NIL NIL \"7BIT\" 90 1 NIL NIL NIL NIL)",
            "(\"TEXT\" \"CALENDAR\" (\"METHOD\" \"REQUEST\") NIL NIL \"7BIT\" 70 1 NIL NIL NIL NIL) ",
            "\"ALTERNATIVE\" (\"BOUNDARY\" \"a\") NIL NIL NIL))\r\n",
            "* 3 FETCH (UID 3 BODYSTRUCTURE (\"TEXT\" \"PLAIN\" bogus))\r\n",
        ));
        s.ok(&tag);
        s.quiet_noop();
    });
    smol::block_on(async {
        let mut imap = connect(&endpoint).await;
        let headers = imap.fetch_headers(1, Some(3)).await.unwrap();
        let uids: Vec<_> = headers.iter().map(|h| h.uid).collect();
        assert_eq!(uids, [1, 2, 3], "no message is lost to its structure");
        let pdf = AttachmentPart {
            part: "2".into(),
            mime: "application/pdf".into(),
            filename: Some("€ rates.pdf".into()),
            size: 3000,
        };
        let forwarded = AttachmentPart {
            part: "4".into(),
            mime: "message/rfc822".into(),
            filename: None,
            size: 500,
        };
        assert_eq!(headers[0].attachments, Some(vec![pdf, forwarded]));
        assert_eq!(headers[1].attachments, Some(vec![]));
        assert_eq!(headers[2].attachments, None);
        imap.poll_changes().await.unwrap();
    });
    server.join().unwrap();
}

#[test]
fn other_servers_have_no_gmail_search() {
    let (endpoint, server) = serve(CAPS, |s| s.quiet_noop());
    smol::block_on(async {
        let mut imap = connect(&endpoint).await;
        assert_eq!(imap.gmail_search(1, "category:social").await.unwrap(), None);
        imap.poll_changes().await.unwrap();
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

/// "More results on server": a search box query as IMAP `SEARCH`, words
/// that are not ASCII as UTF-8 literals; on Gmail as `X-GM-RAW`.
#[test]
fn server_search_commands() {
    use katna_sync::server_search::{Criterion, Field};
    let query = Criterion::And(vec![
        Criterion::Text(Field::Any, "budget".into()),
        Criterion::Text(Field::From, "ada@example.org".into()),
        Criterion::After(1_704_067_200),
    ]);
    let (endpoint, server) = serve(CAPS, |s| {
        let (tag, rest) = s.command();
        assert_eq!(
            rest,
            "UID SEARCH TEXT budget FROM ada@example.org SENTSINCE \"01-Jan-2024\""
        );
        s.send(&format!("* SEARCH 7 3\r\n{tag} OK done\r\n"));

        let (tag, rest) = s.command();
        assert_eq!(rest, "UID SEARCH CHARSET UTF-8 TEXT {5}");
        s.send("+ go\r\n");
        let mut literal = vec![0; 5];
        s.reader.read_exact(&mut literal).unwrap();
        assert_eq!(literal, "café".as_bytes());
        assert_eq!(s.line(), "");
        s.send(&format!("* SEARCH\r\n{tag} OK done\r\n"));
        s.quiet_noop();
    });
    smol::block_on(async {
        let mut imap = connect(&endpoint).await;
        assert_eq!(imap.search(&query).await.unwrap(), Some(vec![3, 7]));
        let cafe = Criterion::Text(Field::Any, "café".into());
        assert_eq!(imap.search(&cafe).await.unwrap(), Some(vec![]));
        // Attachments have no IMAP form.
        assert_eq!(imap.search(&Criterion::HasAttachment).await.unwrap(), None);
        imap.poll_changes().await.unwrap();
    });
    server.join().unwrap();

    let (endpoint, server) = serve("IMAP4rev1 AUTH=PLAIN SASL-IR IDLE X-GM-EXT-1", |s| {
        let (tag, rest) = s.command();
        assert_eq!(
            rest,
            "UID SEARCH X-GM-RAW \"(\\\"q3 plan\\\" has:attachment)\""
        );
        s.send(&format!(
            "* SEARCH 4\r\n{tag} OK SEARCH completed (Success)\r\n"
        ));
        s.quiet_noop();
    });
    smol::block_on(async {
        let mut imap = connect(&endpoint).await;
        let query = Criterion::And(vec![
            Criterion::Text(Field::Any, "q3 plan".into()),
            Criterion::HasAttachment,
        ]);
        assert_eq!(imap.search(&query).await.unwrap(), Some(vec![4]));
        imap.poll_changes().await.unwrap();
    });
    server.join().unwrap();
}
