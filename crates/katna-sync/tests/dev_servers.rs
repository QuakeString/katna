// SPDX-License-Identifier: GPL-3.0-or-later

//! Integration tests against the local servers in `dev/compose.yaml`.
//!
//! They need the servers running, so they are ignored by default:
//!
//! ```sh
//! (cd dev && docker compose up -d)
//! cargo test -p katna-sync --test dev_servers -- --ignored --test-threads 1
//! ```
//!
//! Ports follow the `KATNA_*_PORT` variables from `dev/README.md`. Every
//! test writes only to folders it creates (`katna-test-…`) or sends mail to
//! the test accounts.

use std::{
    env,
    io::{Read, Write},
    net::TcpStream,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use katna_sync::{
    Credentials, Endpoint, Error, FolderChange, FolderRole, MailBackend, MailSender, Security,
    connection::{self, Connection},
    imap::ImapBackend,
    net::Tls,
    smtp::SmtpSender,
};

const USER: &str = "alice@katna.test";
const PASSWORD: &str = "katna-dev";

fn port(var: &str, default: u16) -> u16 {
    env::var(var)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

/// The IMAP endpoints to test: Stalwart over TLS, Dovecot over STARTTLS
/// and over TLS.
fn imap_servers() -> Vec<(&'static str, Endpoint)> {
    vec![
        (
            "stalwart",
            Endpoint::new(
                "127.0.0.1",
                port("KATNA_STALWART_IMAPS_PORT", 10993),
                Security::Tls,
            ),
        ),
        (
            "dovecot-starttls",
            Endpoint::new(
                "127.0.0.1",
                port("KATNA_DOVECOT_IMAP_PORT", 20143),
                Security::StartTls,
            ),
        ),
        (
            "dovecot-tls",
            Endpoint::new(
                "127.0.0.1",
                port("KATNA_DOVECOT_IMAPS_PORT", 20993),
                Security::Tls,
            ),
        ),
    ]
}

fn creds() -> Credentials {
    Credentials::new(USER, PASSWORD)
}

fn tls() -> Tls {
    Tls::insecure_for_local_tests()
}

async fn connect(endpoint: &Endpoint) -> ImapBackend {
    ImapBackend::connect(endpoint, &creds(), tls())
        .await
        .unwrap_or_else(|e| panic!("connect to {endpoint:?}: {e}"))
}

/// Connects and moves the connection into its own task on smol's pool.
async fn spawn(endpoint: &Endpoint) -> Connection {
    let (conn, task) = connection::spawn(connect(endpoint).await);
    smol::spawn(task).detach();
    conn
}

fn unique(prefix: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{prefix}-{nanos}")
}

fn message(subject: &str, to: &str) -> Vec<u8> {
    format!(
        "From: Alice <{USER}>\r\nTo: <{to}>\r\nSubject: {subject}\r\n\
         Date: Sat, 26 Sep 2026 10:00:00 +0000\r\nMessage-ID: <{subject}@katna.test>\r\n\
         \r\nHello from the katna-sync tests.\r\n"
    )
    .into_bytes()
}

#[test]
#[ignore = "needs the dev/compose.yaml servers"]
fn lists_folders_and_fetches_seeded_envelopes() {
    for (name, endpoint) in imap_servers() {
        smol::block_on(async {
            let mut imap = connect(&endpoint).await;
            let caps = imap.capabilities();
            assert!(caps.iter().any(|c| c == "IDLE"), "{name}: {caps:?}");

            let folders = imap.list_folders().await.unwrap();
            let inbox = folders.iter().find(|f| f.name == "INBOX").unwrap();
            assert_eq!(inbox.role, Some(FolderRole::Inbox), "{name}");
            assert!(folders.iter().any(|f| f.name == "Projects"), "{name}");

            let status = imap.select("INBOX").await.unwrap();
            assert!(status.exists >= 6, "{name}: {status:?}");
            assert!(status.uid_validity.is_some(), "{name}: {status:?}");
            assert!(status.highest_modseq.is_some(), "{name}: no CONDSTORE");

            let started = Instant::now();
            let envelopes = imap.fetch_envelopes(1, None).await.unwrap();
            let elapsed = started.elapsed();
            assert_eq!(envelopes.len() as u32, status.exists, "{name}");
            assert!(envelopes.windows(2).all(|w| w[0].uid < w[1].uid));
            let welcome = envelopes
                .iter()
                .find(|e| e.subject.as_deref() == Some("Welcome to the Katna test server"))
                .unwrap_or_else(|| panic!("{name}: welcome mail missing"));
            assert!(!welcome.from.is_empty() && welcome.size > 0, "{welcome:?}");
            println!("{name}: {} envelopes in {elapsed:?}", envelopes.len());

            imap.logout().await.unwrap();
        });
    }
}

#[test]
#[ignore = "needs the dev/compose.yaml servers"]
fn refused_command_keeps_the_connection() {
    for (name, endpoint) in imap_servers() {
        smol::block_on(async {
            let mut imap = connect(&endpoint).await;
            let err = imap.select("katna-no-such-folder").await.unwrap_err();
            assert!(matches!(err, Error::Rejected(_)), "{name}: {err:?}");
            assert!(!err.is_fatal());
            imap.select("INBOX").await.unwrap();
            imap.poll_changes().await.unwrap();
            imap.logout().await.unwrap();
        });
    }
}

#[test]
#[ignore = "needs the dev/compose.yaml servers"]
fn wrong_password_is_an_auth_error() {
    // Dovecot accepts any user name, so a wrong password is the test.
    for (name, endpoint) in imap_servers() {
        smol::block_on(async {
            let bad = Credentials::new(USER, "not-the-password");
            let err = match ImapBackend::connect(&endpoint, &bad, tls()).await {
                Ok(_) => panic!("{name}: login succeeded"),
                Err(err) => err,
            };
            assert!(matches!(err, Error::Auth(_)), "{name}: {err:?}");
        });
    }
}

#[test]
#[ignore = "needs the dev/compose.yaml servers"]
fn idle_wakes_up_on_append_from_another_connection() {
    for (name, endpoint) in imap_servers() {
        smol::block_on(async {
            let folder = unique("katna-test-idle");
            let watcher = spawn(&endpoint).await;
            watcher.create_folder(&folder).await.unwrap();
            assert_eq!(watcher.select(&folder).await.unwrap().exists, 0);

            let waiting = {
                let watcher = watcher.clone();
                smol::spawn(async move {
                    let started = Instant::now();
                    let changes = watcher
                        .wait_for_changes(Duration::from_secs(30))
                        .await
                        .unwrap();
                    (changes, started.elapsed())
                })
            };
            // Let the IDLE start before appending.
            async_io::Timer::after(Duration::from_millis(300)).await;
            let mut writer = connect(&endpoint).await;
            let appended = Instant::now();
            writer
                .append(&folder, message(&unique("idle"), USER))
                .await
                .unwrap();

            let (changes, waited) = waiting.await;
            assert!(
                changes.contains(&FolderChange::Exists(1)),
                "{name}: {changes:?}"
            );
            assert!(waited < Duration::from_secs(10), "{name}: {waited:?}");
            println!(
                "{name}: IDLE woke {:?} after the append",
                appended.elapsed()
            );

            // The connection is ready for more after IDLE.
            let envelopes = watcher.fetch_envelopes(1, None).await.unwrap();
            assert_eq!(envelopes.len(), 1, "{name}");
            writer.logout().await.unwrap();
            watcher.logout().await.unwrap();
        });
    }
}

/// Stalwart 0.16 reports a change made between two commands on NOOP only,
/// not when IDLE starts, so the wait must ask first.
#[test]
#[ignore = "needs the dev/compose.yaml servers"]
fn wait_reports_changes_made_before_it_started() {
    for (name, endpoint) in imap_servers() {
        smol::block_on(async {
            let folder = unique("katna-test-before");
            let watcher = spawn(&endpoint).await;
            watcher.create_folder(&folder).await.unwrap();
            watcher.select(&folder).await.unwrap();

            let mut writer = connect(&endpoint).await;
            writer
                .append(&folder, message(&unique("before"), USER))
                .await
                .unwrap();
            writer.logout().await.unwrap();
            // Dovecot batches notifications for up to half a second.
            async_io::Timer::after(Duration::from_millis(600)).await;

            let started = Instant::now();
            let changes = watcher
                .wait_for_changes(Duration::from_secs(10))
                .await
                .unwrap();
            assert!(
                changes.contains(&FolderChange::Exists(1)),
                "{name}: {changes:?}"
            );
            assert!(started.elapsed() < Duration::from_secs(2), "{name}");
            watcher.logout().await.unwrap();
        });
    }
}

#[test]
#[ignore = "needs the dev/compose.yaml servers"]
fn request_ends_idle_and_runs_next() {
    for (name, endpoint) in imap_servers() {
        smol::block_on(async {
            let conn = spawn(&endpoint).await;
            conn.select("INBOX").await.unwrap();
            let waiting = {
                let conn = conn.clone();
                smol::spawn(async move { conn.wait_for_changes(Duration::from_secs(120)).await })
            };
            async_io::Timer::after(Duration::from_millis(300)).await;

            let started = Instant::now();
            let folders = conn.list_folders().await.unwrap();
            let elapsed = started.elapsed();
            assert!(!folders.is_empty());
            assert!(elapsed < Duration::from_secs(5), "{name}: {elapsed:?}");
            assert_eq!(waiting.await.unwrap(), vec![], "{name}");
            println!("{name}: IDLE ended and LIST ran in {elapsed:?}");

            // Still in IDLE-capable shape.
            conn.poll_changes().await.unwrap();
            Connection::logout(&conn).await.unwrap();
            assert!(matches!(conn.select("INBOX").await, Err(Error::Closed(_))));
        });
    }
}

#[test]
#[ignore = "needs the dev/compose.yaml servers"]
fn dropping_every_handle_logs_out() {
    let (_, endpoint) = &imap_servers()[0];
    smol::block_on(async {
        let (conn, task) = connection::spawn(connect(endpoint).await);
        let task = smol::spawn(task);
        conn.select("INBOX").await.unwrap();
        let waiter = {
            let conn = conn.clone();
            smol::spawn(async move { conn.wait_for_changes(Duration::from_secs(120)).await })
        };
        async_io::Timer::after(Duration::from_millis(300)).await;
        drop(conn);
        // The waiter still holds a handle, so the task keeps running until
        // its wait ends; cancelling the waiter drops the last handle.
        drop(waiter);
        let started = Instant::now();
        task.await;
        assert!(started.elapsed() < Duration::from_secs(5));
    });
}

/// S2 problem 10: Dovecot splits the `+` continuation of APPEND across
/// reads, which breaks io-imap unless we hand it whole lines.
#[test]
#[ignore = "needs the dev/compose.yaml servers"]
fn many_appends_over_starttls() {
    let (_, endpoint) = &imap_servers()[1];
    smol::block_on(async {
        let folder = unique("katna-test-append");
        let mut imap = connect(endpoint).await;
        imap.create_folder(&folder).await.unwrap();
        let started = Instant::now();
        for i in 0..500 {
            imap.append(&folder, message(&format!("append-{i}"), USER))
                .await
                .unwrap_or_else(|e| panic!("append {i}: {e}"));
        }
        println!("500 appends in {:?}", started.elapsed());
        assert_eq!(imap.select(&folder).await.unwrap().exists, 500);
        imap.logout().await.unwrap();
    });
}

#[test]
#[ignore = "needs the dev/compose.yaml servers"]
fn stalwart_delivers_submitted_mail_and_idle_sees_it() {
    smol::block_on(async {
        let imap = Endpoint::new(
            "127.0.0.1",
            port("KATNA_STALWART_IMAPS_PORT", 10993),
            Security::Tls,
        );
        let smtp = Endpoint::new(
            "127.0.0.1",
            port("KATNA_STALWART_SUBMISSIONS_PORT", 10465),
            Security::Tls,
        );
        let watcher = spawn(&imap).await;
        let before = watcher.select("INBOX").await.unwrap().exists;

        let mut sender = SmtpSender::connect(&smtp, &creds(), tls()).await.unwrap();
        let subject = unique("smtp-stalwart");
        sender
            .send(USER, &[USER], message(&subject, USER))
            .await
            .unwrap();
        sender.quit().await.unwrap();

        let deadline = Instant::now() + Duration::from_secs(30);
        let mut seen = false;
        while !seen && Instant::now() < deadline {
            let changes = watcher
                .wait_for_changes(Duration::from_secs(5))
                .await
                .unwrap();
            seen = changes
                .iter()
                .any(|c| matches!(c, FolderChange::Exists(n) if *n > before));
        }
        assert!(seen, "the sent mail never arrived in INBOX");
        let envelopes = watcher.fetch_envelopes(1, None).await.unwrap();
        assert!(
            envelopes
                .iter()
                .any(|e| e.subject.as_deref() == Some(subject.as_str()))
        );
        watcher.logout().await.unwrap();
    });
}

#[test]
#[ignore = "needs the dev/compose.yaml servers"]
fn dovecot_submission_relays_to_mailpit() {
    smol::block_on(async {
        let smtp = Endpoint::new(
            "127.0.0.1",
            port("KATNA_DOVECOT_SUBMISSION_PORT", 20587),
            Security::StartTls,
        );
        let mut sender = SmtpSender::connect(&smtp, &creds(), tls()).await.unwrap();
        let subject = unique("smtp-dovecot");
        let to = "someone@example.org";
        sender
            .send(USER, &[to], message(&subject, to))
            .await
            .unwrap();
        sender.quit().await.unwrap();

        let deadline = Instant::now() + Duration::from_secs(15);
        loop {
            if mailpit_has(&subject) {
                break;
            }
            assert!(Instant::now() < deadline, "Mailpit never got {subject}");
            async_io::Timer::after(Duration::from_millis(200)).await;
        }
    });
}

/// Asks Mailpit's API whether a message with this subject arrived.
fn mailpit_has(subject: &str) -> bool {
    let port = port("KATNA_MAILPIT_HTTP_PORT", 8025);
    let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
    let request =
        format!("GET /api/v1/search?query=subject:{subject} HTTP/1.0\r\nHost: 127.0.0.1\r\n\r\n");
    stream.write_all(request.as_bytes()).unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    response.contains(subject)
}

/// Sync level 1 into a fresh store, then an incremental run. Runs as a task
/// on smol's pool to show the engine's futures are `Send`.
#[test]
#[ignore = "needs the dev/compose.yaml servers"]
fn level_one_sync_into_the_store() {
    use katna_core::{AccountKind, Paths};
    use katna_store::{Mode, Store};
    use katna_sync::engine;

    for (name, endpoint) in imap_servers() {
        let tmp = tempfile::tempdir().unwrap();
        let mut store = Store::open(&Paths::with_root(tmp.path()), Mode::ReadWrite).unwrap();
        let account = store.add_account(AccountKind::Imap, name, USER).unwrap().id;
        smol::block_on(async {
            let mut conn = spawn(&endpoint).await;
            let (mut conn, mut store, first) = smol::spawn(async move {
                let started = Instant::now();
                let reports = engine::sync_account(&mut conn, &mut store, account)
                    .await
                    .unwrap();
                println!("{name}: first sync in {:?}", started.elapsed());
                (conn, store, reports)
            })
            .await;
            let inbox = first.iter().find(|r| r.path == "INBOX").unwrap();
            assert!(inbox.added >= 6, "{name}: {first:?}");
            let projects = first.iter().find(|r| r.path == "Projects").unwrap();
            assert_eq!(projects.added, 1, "{name}");

            let folders = store.folders(account).unwrap();
            let inbox_id = folders.iter().find(|f| f.path == "INBOX").unwrap().id;
            let messages = store.messages_in_folder(inbox_id).unwrap();
            let subjects: Vec<&str> = messages.iter().map(|m| m.subject.as_str()).collect();
            assert!(
                subjects.contains(&"Weekly digest \u{2014} caf\u{e9} edition"),
                "{name}: encoded words decoded: {subjects:?}"
            );
            assert!(messages.iter().all(|m| m.date.is_some()), "{name}");
            assert!(
                messages.iter().any(|m| m.has_attachments),
                "{name}: the attachment sample"
            );

            // Incremental: one new message, nothing else.
            conn.append("INBOX", message(&unique("level1"), USER))
                .await
                .unwrap();
            let second = engine::sync_account(&mut conn, &mut store, account)
                .await
                .unwrap();
            let inbox = second.iter().find(|r| r.path == "INBOX").unwrap();
            assert_eq!(
                (inbox.added, inbox.removed, inbox.reset),
                (1, 0, false),
                "{name}: {second:?}"
            );
            Connection::logout(&conn).await.unwrap();
        });
    }
}

#[test]
#[ignore = "needs the dev servers: docker compose -f dev/compose.yaml up -d"]
fn worker_syncs_new_mail_by_push() {
    use async_io::Timer;
    use futures_lite::FutureExt;
    use katna_core::{AccountKind, Paths};
    use katna_store::{Mode, Store};
    use katna_sync::worker::{self, Event, ImapConnector, WorkerConfig};

    for (name, endpoint) in imap_servers() {
        let tmp = tempfile::tempdir().unwrap();
        let mut store = Store::open(&Paths::with_root(tmp.path()), Mode::ReadWrite).unwrap();
        let account = store.add_account(AccountKind::Imap, name, USER).unwrap().id;
        let connector = ImapConnector {
            endpoint: endpoint.clone(),
            credentials: creds(),
            tls: tls(),
        };
        let (events_tx, events) = async_channel::unbounded();
        let (handle, control) = worker::control();
        smol::block_on(async {
            let task = smol::spawn(worker::run(
                connector,
                store,
                account,
                WorkerConfig::default(),
                events_tx,
                control,
            ));
            // The next event other than a body download.
            let next = || async {
                loop {
                    let event = events
                        .recv()
                        .or(async {
                            Timer::after(Duration::from_secs(10)).await;
                            panic!("{name}: no worker event within 10 s");
                        })
                        .await
                        .unwrap();
                    if !matches!(event, Event::BodiesStored(_)) {
                        return event;
                    }
                }
            };
            assert!(matches!(next().await, Event::Connected), "{name}");
            assert!(matches!(next().await, Event::Synced(_)), "{name}");

            // Let the worker settle into IDLE, then deliver.
            Timer::after(Duration::from_millis(300)).await;
            let other = spawn(&endpoint).await;
            let started = Instant::now();
            other
                .append("INBOX", message(&unique("worker"), USER))
                .await
                .unwrap();
            match next().await {
                Event::Synced(reports) => {
                    println!("{name}: worker synced new mail in {:?}", started.elapsed());
                    assert_eq!(reports.len(), 1, "{name}: {reports:?}");
                    assert_eq!(reports[0].path, "INBOX");
                    assert_eq!(reports[0].added, 1, "{name}");
                }
                other => panic!("{name}: expected Synced, got {other:?}"),
            }
            Connection::logout(&other).await.unwrap();
            drop(handle);
            task.await;
        });
    }
}

#[test]
#[ignore = "needs the dev servers: docker compose -f dev/compose.yaml up -d"]
fn bodies_are_downloaded_without_marking_mail_read() {
    use katna_core::{AccountKind, Paths};
    use katna_store::{Mode, Store};
    use katna_sync::{
        bodies::{self, OfflineWindow},
        engine,
    };

    for (name, endpoint) in imap_servers() {
        let tmp = tempfile::tempdir().unwrap();
        let mut store = Store::open(&Paths::with_root(tmp.path()), Mode::ReadWrite).unwrap();
        let account = store.add_account(AccountKind::Imap, name, USER).unwrap().id;
        smol::block_on(async {
            let mut conn = connect(&endpoint).await;
            engine::sync_account(&mut conn, &mut store, account)
                .await
                .unwrap();
            let inbox = store
                .folders(account)
                .unwrap()
                .into_iter()
                .find(|f| f.path == "INBOX")
                .unwrap();
            let unread_before: Vec<_> = store
                .messages_in_folder(inbox.id)
                .unwrap()
                .into_iter()
                .filter(|m| !m.flags.contains(katna_store::MessageFlags::SEEN))
                .map(|m| m.id)
                .collect();

            let everything = OfflineWindow {
                days: None,
                max_size: u64::MAX,
            };
            let started = Instant::now();
            let stored =
                bodies::download_bodies(&mut conn, &mut store, inbox.id, "INBOX", &everything, 0)
                    .await
                    .unwrap();
            println!("{name}: {stored} bodies in {:?}", started.elapsed());
            let messages = store.messages_in_folder(inbox.id).unwrap();
            assert_eq!(stored, messages.len(), "{name}");
            for message in &messages {
                let raw = store
                    .blobs()
                    .get(&message.blob_hash.unwrap())
                    .unwrap()
                    .unwrap();
                assert_eq!(
                    raw.len() as u64,
                    message.size,
                    "{name}: {}",
                    message.subject
                );
            }
            let attachment = messages.iter().find(|m| m.has_attachments);
            assert!(attachment.is_some(), "{name}: the attachment sample");

            // BODY.PEEK leaves \Seen alone.
            let report = engine::sync_folder(&mut conn, &mut store, account, inbox.id, "INBOX")
                .await
                .unwrap();
            assert_eq!(report.flags_changed, 0, "{name}");
            let still_unread = store
                .messages_by_id(&unread_before)
                .unwrap()
                .iter()
                .filter(|m| !m.flags.contains(katna_store::MessageFlags::SEEN))
                .count();
            assert_eq!(still_unread, unread_before.len(), "{name}");
            conn.logout().await.unwrap();
        });
    }
}

#[test]
#[ignore = "needs the dev servers: docker compose -f dev/compose.yaml up -d"]
fn queued_changes_reach_the_server() {
    use katna_core::{AccountKind, Paths};
    use katna_store::{FolderId, MessageFlags, Mode, Store};
    use katna_sync::{engine, ops};

    const NOW: i64 = 1_790_416_800;

    /// Subject → flags of the messages in `path`.
    fn contents(
        store: &Store,
        account: katna_core::AccountId,
        path: &str,
    ) -> Vec<(String, MessageFlags)> {
        let folder = store
            .folders(account)
            .unwrap()
            .into_iter()
            .find(|f| f.path == path)
            .unwrap();
        let mut messages: Vec<_> = store
            .messages_in_folder(folder.id)
            .unwrap()
            .into_iter()
            .map(|m| (m.subject, m.flags))
            .collect();
        messages.sort_by(|a, b| a.0.cmp(&b.0));
        messages
    }

    for (name, endpoint) in imap_servers() {
        let tmp = tempfile::tempdir().unwrap();
        let open = |dir: &str| {
            let mut store =
                Store::open(&Paths::with_root(tmp.path().join(dir)), Mode::ReadWrite).unwrap();
            let account = store.add_account(AccountKind::Imap, name, USER).unwrap().id;
            (store, account)
        };
        let (mut store, account) = open("app");
        let (from, to) = (unique("katna-test-ops-a"), unique("katna-test-ops-b"));
        let subjects = ["keep", "move", "delete"].map(|s| format!("{from}-{s}"));
        smol::block_on(async {
            let mut conn = connect(&endpoint).await;
            for path in [&from, &to] {
                conn.create_folder(path).await.unwrap();
            }
            for subject in &subjects {
                conn.append(&from, message(subject, USER)).await.unwrap();
            }
            engine::sync_account(&mut conn, &mut store, account)
                .await
                .unwrap();
            let folder = |path: &str| -> FolderId {
                store
                    .folders(account)
                    .unwrap()
                    .into_iter()
                    .find(|f| f.path == path)
                    .unwrap()
                    .id
            };
            let (from_id, to_id) = (folder(&from), folder(&to));
            let id = |subject: &str| {
                store
                    .messages_in_folder(from_id)
                    .unwrap()
                    .into_iter()
                    .find(|m| m.subject == subject)
                    .unwrap()
                    .id
            };
            let (keep, moved, deleted) = (id(&subjects[0]), id(&subjects[1]), id(&subjects[2]));

            ops::set_flags(
                &mut store,
                &[keep, moved],
                MessageFlags::SEEN | MessageFlags::FLAGGED,
                MessageFlags::empty(),
            )
            .unwrap();
            ops::set_flags(
                &mut store,
                &[keep],
                MessageFlags::empty(),
                MessageFlags::SEEN,
            )
            .unwrap();
            ops::move_messages(&mut store, &[moved], to_id).unwrap();
            // Twice: to the trash, then gone, so the trash stays clean.
            ops::delete_messages(&mut store, &[deleted]).unwrap();
            let started = Instant::now();
            let report = ops::replay(&mut conn, &mut store, account, NOW)
                .await
                .unwrap();
            println!("{name}: {report:?} in {:?}", started.elapsed());
            assert_eq!((report.retried, report.failed), (0, 0), "{name}");
            assert!(
                report.resync.is_empty(),
                "{name}: MOVE/COPYUID reports UIDs"
            );
            ops::delete_messages(&mut store, &[deleted]).unwrap();
            let report = ops::replay(&mut conn, &mut store, account, NOW)
                .await
                .unwrap();
            assert_eq!((report.done, report.failed), (1, 0), "{name}");
            assert!(store.messages_by_id(&[deleted]).unwrap().is_empty());

            // Our own sync sees nothing new: the UIDs we recorded match.
            let reports = engine::sync_account(&mut conn, &mut store, account)
                .await
                .unwrap();
            for report in reports.iter().filter(|r| r.path == from || r.path == to) {
                assert_eq!(
                    (report.added, report.removed, report.flags_changed),
                    (0, 0, 0),
                    "{name}: {report:?}"
                );
            }

            // A fresh store sees the same thing on the server.
            let (mut fresh, fresh_account) = open("fresh");
            engine::sync_account(&mut conn, &mut fresh, fresh_account)
                .await
                .unwrap();
            let flagged = MessageFlags::FLAGGED;
            let both = MessageFlags::SEEN | MessageFlags::FLAGGED;
            assert_eq!(
                contents(&fresh, fresh_account, &from),
                [(subjects[0].clone(), flagged)],
                "{name}"
            );
            assert_eq!(
                contents(&fresh, fresh_account, &to),
                [(subjects[1].clone(), both)],
                "{name}"
            );
            for (path, folder) in [(&from, &from), (&to, &to)] {
                assert_eq!(
                    contents(&store, account, path),
                    contents(&fresh, fresh_account, folder),
                    "{name}"
                );
            }
            let trash = fresh
                .folders(fresh_account)
                .unwrap()
                .into_iter()
                .find(|f| f.role == Some(katna_store::FolderRole::Trash));
            if let Some(trash) = trash {
                assert!(
                    !contents(&fresh, fresh_account, &trash.path)
                        .iter()
                        .any(|(subject, _)| *subject == subjects[2]),
                    "{name}: expunged from the trash"
                );
            }
            conn.logout().await.unwrap();
        });
    }
}

/// The POP3 endpoints: Stalwart over TLS and Dovecot over STLS.
fn pop3_servers() -> Vec<(&'static str, Endpoint, Endpoint)> {
    vec![
        (
            "stalwart",
            Endpoint::new(
                "127.0.0.1",
                port("KATNA_STALWART_POP3S_PORT", 10995),
                Security::Tls,
            ),
            imap_servers()[0].1.clone(),
        ),
        (
            "dovecot-stls",
            Endpoint::new(
                "127.0.0.1",
                port("KATNA_DOVECOT_POP3_PORT", 20110),
                Security::StartTls,
            ),
            imap_servers()[1].1.clone(),
        ),
    ]
}

#[test]
#[ignore = "needs the dev servers"]
fn pop3_downloads_what_imap_delivered() {
    use katna_core::{AccountKind, Paths, Pop3Keep};
    use katna_store::{Mode, Store};
    use katna_sync::pop3::{Pop3Client, sync};

    for (name, pop3, imap) in pop3_servers() {
        smol::block_on(async {
            let err = Pop3Client::connect(&pop3, &Credentials::new(USER, "wrong"), tls()).await;
            assert!(matches!(err, Err(Error::Auth(_))), "{name}");

            let subject = unique("pop3");
            let mut backend = connect(&imap).await;
            backend
                .append("INBOX", message(&subject, USER))
                .await
                .unwrap();
            backend.logout().await.unwrap();

            let mut client = Pop3Client::connect(&pop3, &creds(), tls()).await.unwrap();
            assert!(client.capabilities().iter().any(|c| c == "UIDL"), "{name}");
            let entries = client.entries().await.unwrap();
            let newest = entries.last().expect("mail in the maildrop");
            let raw = client.retr(newest.number).await.unwrap();
            assert!(
                String::from_utf8_lossy(&raw).contains(&subject),
                "{name}: the newest message is the one just delivered"
            );
            let header = client.top(newest.number, 0).await.unwrap();
            assert!(header.len() < raw.len() && raw.starts_with(&header[..header.len() - 2]));
            client.quit().await.unwrap();

            // Into the store, leaving everything on the server.
            let tmp = tempfile::tempdir().unwrap();
            let mut store = Store::open(&Paths::with_root(tmp.path()), Mode::ReadWrite).unwrap();
            let account = store.add_account(AccountKind::Pop3, name, USER).unwrap().id;
            let keep = Pop3Keep::default();
            let client = Pop3Client::connect(&pop3, &creds(), tls()).await.unwrap();
            let report = sync::sync_account(client, &mut store, account, &keep, 0, |_| {})
                .await
                .unwrap();
            assert_eq!(report.added, entries.len(), "{name}");
            assert_eq!(report.deleted, 0, "{name}");
            let client = Pop3Client::connect(&pop3, &creds(), tls()).await.unwrap();
            let again = sync::sync_account(client, &mut store, account, &keep, 0, |_| {})
                .await
                .unwrap();
            assert_eq!(again, sync::Pop3Report::default(), "{name}");
        });
    }
}
