// SPDX-License-Identifier: GPL-3.0-or-later

//! The POP3 client against a small scripted server, and POP3 sync and its
//! worker into a real store.

use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use futures_lite::FutureExt;
use katna_core::{AccountId, AccountKind, Paths, Pop3Keep};
use katna_store::{FolderRole, Mode, Store};
use katna_sync::{
    Credentials, Endpoint, Error, Security,
    net::Tls,
    pop3::{Pop3Client, sync},
    worker::{self, Event, Pop3Connector, WorkerConfig},
};

/// One message on the fake server.
#[derive(Clone)]
struct Stored {
    uidl: String,
    raw: String,
}

#[derive(Default)]
struct Maildrop {
    messages: Vec<Stored>,
    /// Every command received, passwords masked.
    log: Vec<String>,
    /// Refuse RETR of this UIDL by closing the connection.
    break_on: Option<String>,
}

/// A POP3 server on 127.0.0.1 that serves `maildrop`, one session at a
/// time, as long as the test runs.
struct FakePop3 {
    port: u16,
    maildrop: Arc<Mutex<Maildrop>>,
}

impl FakePop3 {
    fn start(messages: &[(&str, &str)]) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let maildrop = Arc::new(Mutex::new(Maildrop {
            messages: messages
                .iter()
                .map(|(uidl, raw)| Stored {
                    uidl: uidl.to_string(),
                    raw: raw.to_string(),
                })
                .collect(),
            ..Maildrop::default()
        }));
        let shared = maildrop.clone();
        thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(stream) = stream else { return };
                let _ = serve(stream, &shared);
            }
        });
        Self { port, maildrop }
    }

    fn endpoint(&self) -> Endpoint {
        Endpoint::new("127.0.0.1", self.port, Security::Plain)
    }

    fn connect(&self, password: &str) -> Result<Pop3Client, Error> {
        smol::block_on(Pop3Client::connect(
            &self.endpoint(),
            &Credentials::new("alice", password),
            Tls::insecure_for_local_tests(),
        ))
    }

    fn uidls(&self) -> Vec<String> {
        let drop = self.maildrop.lock().unwrap();
        drop.messages.iter().map(|m| m.uidl.clone()).collect()
    }

    fn add(&self, uidl: &str, raw: &str) {
        self.maildrop.lock().unwrap().messages.push(Stored {
            uidl: uidl.into(),
            raw: raw.into(),
        });
    }

    fn log(&self) -> Vec<String> {
        self.maildrop.lock().unwrap().log.clone()
    }
}

fn serve(stream: TcpStream, shared: &Mutex<Maildrop>) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut out = stream;
    // Message numbers and deletions are fixed for the session.
    let session: Vec<Stored> = shared.lock().unwrap().messages.clone();
    let mut deleted = vec![false; session.len()];
    let mut user_ok = false;
    let mut logged_in = false;
    out.write_all(b"+OK fake POP3 ready\r\n")?;
    let mut line = String::new();
    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            return Ok(()); // no QUIT: nothing is deleted
        }
        let command = line.trim_end().to_owned();
        let (verb, arg) = command.split_once(' ').unwrap_or((&command, ""));
        let verb = verb.to_ascii_uppercase();
        shared.lock().unwrap().log.push(match verb.as_str() {
            "PASS" => "PASS ***".to_owned(),
            _ => command.clone(),
        });
        let message = |arg: &str| -> Option<usize> {
            let n: usize = arg.split(' ').next()?.parse().ok()?;
            (1..=session.len())
                .contains(&n)
                .then_some(n - 1)
                .filter(|&i| !deleted[i])
        };
        let reply = match verb.as_str() {
            "CAPA" => "+OK\r\nUSER\r\nUIDL\r\nTOP\r\n.\r\n".to_owned(),
            "USER" => {
                user_ok = arg == "alice";
                "+OK\r\n".to_owned()
            }
            "PASS" if user_ok && arg == "secret" => {
                logged_in = true;
                "+OK logged in\r\n".to_owned()
            }
            "PASS" => "-ERR [AUTH] invalid login\r\n".to_owned(),
            _ if !logged_in => "-ERR log in first\r\n".to_owned(),
            "UIDL" | "LIST" => {
                let mut reply = "+OK\r\n".to_owned();
                for (i, stored) in session.iter().enumerate() {
                    if deleted[i] {
                        continue;
                    }
                    let word = match verb.as_str() {
                        "UIDL" => stored.uidl.clone(),
                        _ => stored.raw.len().to_string(),
                    };
                    reply.push_str(&format!("{} {word}\r\n", i + 1));
                }
                reply + ".\r\n"
            }
            "RETR" | "TOP" => match message(arg) {
                Some(i) => {
                    let stored = &session[i];
                    if shared.lock().unwrap().break_on.as_deref() == Some(&stored.uidl) {
                        return Ok(());
                    }
                    let raw = match verb.as_str() {
                        "TOP" => {
                            stored.raw.split("\r\n\r\n").next().unwrap().to_owned() + "\r\n\r\n"
                        }
                        _ => stored.raw.clone(),
                    };
                    let mut reply = "+OK\r\n".to_owned();
                    for line in raw.split_inclusive("\r\n") {
                        if line.starts_with('.') {
                            reply.push('.');
                        }
                        reply.push_str(line);
                    }
                    reply + ".\r\n"
                }
                None => "-ERR no such message\r\n".to_owned(),
            },
            "DELE" => match message(arg) {
                Some(i) => {
                    deleted[i] = true;
                    "+OK\r\n".to_owned()
                }
                None => "-ERR no such message\r\n".to_owned(),
            },
            "QUIT" => {
                let gone: Vec<String> = session
                    .iter()
                    .zip(&deleted)
                    .filter(|(_, d)| **d)
                    .map(|(m, _)| m.uidl.clone())
                    .collect();
                shared
                    .lock()
                    .unwrap()
                    .messages
                    .retain(|m| !gone.contains(&m.uidl));
                out.write_all(b"+OK bye\r\n")?;
                return Ok(());
            }
            _ => "-ERR unknown command\r\n".to_owned(),
        };
        out.write_all(reply.as_bytes())?;
    }
}

/// The next worker event; fails the test after ten seconds.
async fn recv(events: &async_channel::Receiver<Event>) -> Event {
    let event = async { events.recv().await.ok() };
    let timeout = async {
        async_io::Timer::after(Duration::from_secs(10)).await;
        None
    };
    event.or(timeout).await.expect("a worker event")
}

fn mail(n: u32) -> String {
    format!(
        "From: Bob <bob@example.org>\r\nTo: alice@example.org\r\n\
         Message-ID: <{n}@example.org>\r\nSubject: Note {n}\r\n\r\nLine one\r\n.dot line\r\n"
    )
}

fn store() -> (tempfile::TempDir, Store, AccountId) {
    let tmp = tempfile::tempdir().unwrap();
    let mut store = Store::open(&Paths::with_root(tmp.path()), Mode::ReadWrite).unwrap();
    let account = store
        .add_account(AccountKind::Pop3, "pop", "alice@example.org")
        .unwrap()
        .id;
    (tmp, store, account)
}

fn check(
    server: &FakePop3,
    store: &mut Store,
    account: AccountId,
    keep: &Pop3Keep,
    now: i64,
) -> sync::Pop3Report {
    let client = server.connect("secret").unwrap();
    smol::block_on(sync::sync_account(
        client,
        store,
        account,
        keep,
        now,
        |_| {},
    ))
    .unwrap()
}

fn inbox_subjects(store: &Store, account: AccountId) -> Vec<String> {
    let inbox = store
        .folders(account)
        .unwrap()
        .into_iter()
        .find(|f| f.role == Some(FolderRole::Inbox))
        .unwrap();
    let mut subjects: Vec<String> = store
        .messages_in_folder(inbox.id)
        .unwrap()
        .into_iter()
        .map(|m| m.subject)
        .collect();
    subjects.sort();
    subjects
}

#[test]
fn client_speaks_pop3() {
    let server = FakePop3::start(&[("a1", &mail(1)), ("b2", &mail(2))]);
    assert!(matches!(server.connect("wrong"), Err(Error::Auth(_))));

    let mut client = server.connect("secret").unwrap();
    assert!(client.capabilities().contains(&"UIDL".to_owned()));
    smol::block_on(async {
        let entries = client.entries().await.unwrap();
        assert_eq!(
            entries
                .iter()
                .map(|e| (e.number, e.uidl.as_str(), e.size))
                .collect::<Vec<_>>(),
            [
                (1, "a1", mail(1).len() as u64),
                (2, "b2", mail(2).len() as u64)
            ]
        );
        // Byte-stuffed lines come back as sent.
        assert_eq!(client.retr(2).await.unwrap(), mail(2).as_bytes());
        let header = client.top(1, 0).await.unwrap();
        assert!(header.ends_with(b"Subject: Note 1\r\n\r\n"));
        assert!(matches!(client.retr(9).await, Err(Error::Rejected(_))));
        client.dele(1).await.unwrap();
        client.quit().await.unwrap();
    });
    assert_eq!(server.uidls(), ["b2"]);
    assert!(server.log().contains(&"PASS ***".to_owned()));
}

#[test]
fn downloads_new_mail_once_and_keeps_it_on_the_server() {
    let server = FakePop3::start(&[("a1", &mail(1)), ("b2", &mail(2))]);
    let (_tmp, mut store, account) = store();
    let keep = Pop3Keep::default();

    let report = check(&server, &mut store, account, &keep, 100);
    assert_eq!(
        report,
        sync::Pop3Report {
            added: 2,
            deleted: 0
        }
    );
    assert_eq!(inbox_subjects(&store, account), ["Note 1", "Note 2"]);
    let paths: Vec<_> = store
        .folders(account)
        .unwrap()
        .into_iter()
        .map(|f| f.path)
        .collect();
    assert_eq!(paths, ["INBOX", "Sent", "Trash"]);

    server.add("c3", &mail(3));
    let report = check(&server, &mut store, account, &keep, 200);
    assert_eq!(
        report,
        sync::Pop3Report {
            added: 1,
            deleted: 0
        }
    );
    assert_eq!(server.uidls(), ["a1", "b2", "c3"]);
    let retrs = server
        .log()
        .iter()
        .filter(|c| c.starts_with("RETR"))
        .count();
    assert_eq!(retrs, 3);

    // The body is stored whole, without the byte-stuffing.
    let messages = store.messages_after(katna_store::MessageId(0), 10).unwrap();
    let hash = messages[0].blob_hash.unwrap();
    let raw = store.blobs().get(&hash).unwrap().unwrap();
    assert!(raw.ends_with(b"\r\n.dot line\r\n"));
}

#[test]
fn deletes_on_the_server_as_the_account_says() {
    let server = FakePop3::start(&[("a1", &mail(1)), ("b2", &mail(2))]);
    let (_tmp, mut store, account) = store();
    let keep = Pop3Keep {
        days: Some(7),
        ..Pop3Keep::default()
    };
    check(&server, &mut store, account, &keep, 0);

    // Deleted for good in Katna: gone from the server too.
    let inbox = store.folders(account).unwrap()[0].id;
    let first = store.messages_in_folder(inbox).unwrap()[0].id;
    let mut batch = store.mail_batch().unwrap();
    batch.remove_from_folder(first, inbox).unwrap();
    batch.commit().unwrap();
    let report = check(&server, &mut store, account, &keep, 60);
    assert_eq!(
        report,
        sync::Pop3Report {
            added: 0,
            deleted: 1
        }
    );
    assert_eq!(server.uidls().len(), 1);
    assert_eq!(store.pop3_uidls(account).unwrap().len(), 1);

    // After seven days the rest goes, but stays in Katna.
    let report = check(&server, &mut store, account, &keep, 7 * 86_400);
    assert_eq!(report.deleted, 1);
    assert!(server.uidls().is_empty());
    assert!(store.pop3_uidls(account).unwrap().is_empty());
    assert_eq!(inbox_subjects(&store, account).len(), 1);
}

#[test]
fn fetch_and_delete_survives_a_broken_connection() {
    let server = FakePop3::start(&[("a1", &mail(1)), ("b2", &mail(2)), ("c3", &mail(3))]);
    let (_tmp, mut store, account) = store();
    let keep = Pop3Keep {
        leave_on_server: false,
        ..Pop3Keep::default()
    };
    // Newest first: c3 is stored, then b2 breaks the session.
    server.maildrop.lock().unwrap().break_on = Some("b2".into());
    let client = server.connect("secret").unwrap();
    let result = smol::block_on(sync::sync_account(
        client,
        &mut store,
        account,
        &keep,
        0,
        |_| {},
    ));
    assert!(result.is_err());
    assert_eq!(inbox_subjects(&store, account), ["Note 3"]);
    assert_eq!(server.uidls().len(), 3, "no QUIT, nothing deleted");

    server.maildrop.lock().unwrap().break_on = None;
    let report = check(&server, &mut store, account, &keep, 10);
    assert_eq!(
        report,
        sync::Pop3Report {
            added: 2,
            deleted: 3
        }
    );
    assert!(server.uidls().is_empty());
    assert!(store.pop3_uidls(account).unwrap().is_empty());
    assert_eq!(inbox_subjects(&store, account).len(), 3);
}

#[test]
fn worker_checks_again_when_asked() {
    let server = FakePop3::start(&[("a1", &mail(1))]);
    let (_tmp, store, account) = store();
    let connector = Pop3Connector {
        endpoint: server.endpoint(),
        credentials: Credentials::new("alice", "secret"),
        tls: Tls::insecure_for_local_tests(),
    };
    let config = WorkerConfig {
        pop3_interval: Duration::from_secs(3600),
        ..WorkerConfig::default()
    };
    let (handle, control) = worker::control();
    let (events_tx, events) = async_channel::unbounded();
    let task = smol::spawn(worker::run_pop3(
        connector,
        store,
        account,
        Pop3Keep::default(),
        config,
        events_tx,
        control,
    ));
    smol::block_on(async {
        assert!(matches!(recv(&events).await, Event::Connected));
        let Event::Synced(reports) = recv(&events).await else {
            panic!("sync expected");
        };
        assert_eq!(reports[0].added, 1);

        server.add("b2", &mail(2));
        handle.sync_now();
        assert!(matches!(recv(&events).await, Event::Connected));
        let Event::Synced(reports) = recv(&events).await else {
            panic!("sync expected");
        };
        assert_eq!(reports[0].added, 1);
        drop(handle);
        task.await;
    });
}

#[test]
fn worker_stops_on_a_refused_password() {
    let server = FakePop3::start(&[]);
    let (_tmp, store, account) = store();
    let connector = Pop3Connector {
        endpoint: server.endpoint(),
        credentials: Credentials::new("alice", "wrong"),
        tls: Tls::insecure_for_local_tests(),
    };
    let (handle, control) = worker::control();
    let (events_tx, events) = async_channel::unbounded();
    let task = smol::spawn(worker::run_pop3(
        connector,
        store,
        account,
        Pop3Keep::default(),
        WorkerConfig::default(),
        events_tx,
        control,
    ));
    smol::block_on(async {
        assert!(matches!(recv(&events).await, Event::AuthFailed(_)));
        drop(handle);
        task.await;
    });
    let logins = server
        .log()
        .iter()
        .filter(|c| c.starts_with("PASS"))
        .count();
    assert_eq!(logins, 1, "a refused password is not retried on its own");
}

#[test]
fn sent_mail_is_filed_locally() {
    let (_tmp, mut store, account) = store();
    sync::ensure_folders(&mut store, account).unwrap();
    let raw = b"From: alice@example.org\r\nTo: bob@example.org\r\nSubject: Hi\r\n\r\nHi\r\n";
    let id = katna_sync::outbox::queue(&mut store, account, raw, 0, 0).unwrap();
    let message = store.outbox_entry(id).unwrap().unwrap().message;
    assert!(!katna_sync::ops::file_sent(&mut store, message).unwrap());
    assert_eq!(store.next_op_due(account).unwrap(), None);
    assert!(store.outbox().unwrap().is_empty());
    let sent = store
        .folders(account)
        .unwrap()
        .into_iter()
        .find(|f| f.role == Some(FolderRole::Sent))
        .unwrap();
    let filed = store.messages_in_folder(sent.id).unwrap();
    assert_eq!(filed.len(), 1);
    assert_eq!(filed[0].id, message);
    assert!(filed[0].flags.contains(katna_store::MessageFlags::SEEN));
}

#[test]
fn worker_checks_again_after_a_network_change() {
    let server = FakePop3::start(&[("a1", &mail(1))]);
    let (_tmp, store, account) = store();
    let connector = Pop3Connector {
        endpoint: server.endpoint(),
        credentials: Credentials::new("alice", "secret"),
        tls: Tls::insecure_for_local_tests(),
    };
    let config = WorkerConfig {
        pop3_interval: Duration::from_secs(3600),
        ..WorkerConfig::default()
    };
    let (handle, control) = worker::control();
    let (events_tx, events) = async_channel::unbounded();
    let task = smol::spawn(worker::run_pop3(
        connector,
        store,
        account,
        Pop3Keep::default(),
        config,
        events_tx,
        control,
    ));
    smol::block_on(async {
        assert!(matches!(recv(&events).await, Event::Connected));
        assert!(matches!(recv(&events).await, Event::Synced(_)));
        server.add("b2", &mail(2));
        handle.reconnect();
        assert!(matches!(recv(&events).await, Event::Connected));
        let Event::Synced(reports) = recv(&events).await else {
            panic!("sync expected");
        };
        assert_eq!(reports[0].added, 1);
        drop(handle);
        task.await;
    });
}

#[test]
fn drafts_get_a_local_folder() {
    let (_tmp, mut store, account) = store();
    let raw =
        b"From: me@example.org\r\nSubject: Plan\r\nMessage-ID: <d1@example.org>\r\n\r\nHi.\r\n";
    let (id, queued) = katna_sync::ops::save_draft(&mut store, account, raw, 100).unwrap();
    assert!(!queued, "nothing goes to a POP3 server");
    let drafts = store
        .folders(account)
        .unwrap()
        .into_iter()
        .find(|f| f.path == "Drafts")
        .unwrap();
    assert_eq!(drafts.role, Some(katna_store::FolderRole::Drafts));
    let ids: Vec<_> = store
        .messages_in_folder(drafts.id)
        .unwrap()
        .into_iter()
        .map(|m| m.id)
        .collect();
    assert_eq!(ids, [id]);
    katna_sync::ops::discard_draft(&mut store, account, "d1@example.org").unwrap();
    assert!(store.messages_in_folder(drafts.id).unwrap().is_empty());
}
