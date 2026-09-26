// SPDX-License-Identifier: GPL-3.0-or-later

//! The daemon on a private session bus, driven through the client proxy.
//!
//! Needs `dbus-daemon` (package `dbus` on Arch, `dbus-daemon` on Debian and
//! Ubuntu). The `#[ignore]`d tests also need the dev servers:
//! `docker compose -f dev/compose.yaml up -d`, then
//! `cargo test -p katna-daemon --test dbus -- --ignored --test-threads 1`.

use std::{
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use async_io::Timer;
use futures_lite::{FutureExt, StreamExt};
use katna_core::{AccountKind, AccountSettings, Paths, Server};
use katna_daemon::{Instance, StartError, secrets::Secrets};
use katna_dbus::{NewImapAccount, NewPop3Account, PimProxy, ServerSpec, send_state, state};
use katna_import::{Flags, IncomingMessage, MessageSink, StoreSink, parse_message};
use katna_search::{Query, SearchIndex, SearchOptions};
use katna_store::{Added, MessageFlags, Mode, NewMessage, Store};
use katna_sync::{
    Credentials, Endpoint, MailBackend, Security, imap::ImapBackend, net::Tls, worker::WorkerConfig,
};

/// A private `dbus-daemon`, killed on drop.
struct Bus {
    child: Child,
    address: String,
}

impl Bus {
    fn start() -> Self {
        let mut child = Command::new("dbus-daemon")
            .args(["--session", "--nofork", "--print-address=1"])
            .stdout(Stdio::piped())
            .spawn()
            .expect("dbus-daemon is installed");
        let mut address = String::new();
        BufReader::new(child.stdout.take().unwrap())
            .read_line(&mut address)
            .unwrap();
        Self {
            child,
            address: address.trim().to_owned(),
        }
    }

    async fn connect(&self) -> zbus::Connection {
        zbus::connection::Builder::address(self.address.as_str())
            .unwrap()
            .build()
            .await
            .unwrap()
    }
}

impl Drop for Bus {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

async fn start(bus: &Bus, paths: &Paths, secrets: Secrets) -> Result<Instance, StartError> {
    Instance::start(
        paths.clone(),
        secrets,
        WorkerConfig::default(),
        bus.connect().await,
    )
    .await
}

async fn within<T>(what: &str, seconds: u64, future: impl Future<Output = T>) -> T {
    future
        .or(async {
            Timer::after(Duration::from_secs(seconds)).await;
            panic!("{what}: nothing within {seconds} s");
        })
        .await
}

fn error_name(err: &zbus::Error) -> String {
    match err {
        zbus::Error::MethodError(name, _, _) => name.to_string(),
        other => format!("{other:?}"),
    }
}

fn imap(host: &str, port: u16, security: &str) -> NewImapAccount {
    NewImapAccount {
        display_name: String::new(),
        address: "alice@katna.test".into(),
        imap: ServerSpec {
            host: host.into(),
            port,
            security: security.into(),
            username: String::new(),
            accept_invalid_certs: true,
        },
        smtp: ServerSpec::default(),
    }
}

#[test]
fn rejects_bad_accounts_and_unknown_ids() {
    let bus = Bus::start();
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths::with_root(tmp.path());
    smol::block_on(async {
        let instance = start(&bus, &paths, Secrets::memory()).await.unwrap();
        let client = bus.connect().await;
        let pim = PimProxy::new(&client).await.unwrap();
        assert!(pim.accounts().await.unwrap().is_empty());

        let mut no_address = imap("127.0.0.1", 993, "tls");
        no_address.address.clear();
        let err = pim.add_imap_account(&no_address, "pw").await.unwrap_err();
        assert_eq!(error_name(&err), "org.freedesktop.DBus.Error.InvalidArgs");

        let err = pim
            .add_imap_account(&imap("127.0.0.1", 993, "ssl"), "pw")
            .await
            .unwrap_err();
        assert_eq!(error_name(&err), "org.freedesktop.DBus.Error.InvalidArgs");
        assert!(err.to_string().contains("tls, starttls or plain"), "{err}");

        // Nothing listens on port 1.
        let err = pim
            .add_imap_account(&imap("127.0.0.1", 1, "tls"), "pw")
            .await
            .unwrap_err();
        assert_eq!(error_name(&err), "org.freedesktop.DBus.Error.Failed");
        assert!(err.to_string().contains("could not connect"), "{err}");
        assert!(pim.accounts().await.unwrap().is_empty());

        let err = pim.sync_now(99).await.unwrap_err();
        assert_eq!(error_name(&err), "org.freedesktop.DBus.Error.UnknownObject");
        assert!(!pim.remove_account(99).await.unwrap());
        pim.sync_now(0).await.unwrap();
        instance.shutdown().await;
    });
}

#[test]
fn discovers_servers() {
    let bus = Bus::start();
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths::with_root(tmp.path());
    smol::block_on(async {
        let instance = start(&bus, &paths, Secrets::memory()).await.unwrap();
        let pim = PimProxy::new(&bus.connect().await).await.unwrap();
        let (account, source) = pim.discover_account(" ada@gmail.com ").await.unwrap();
        assert_eq!(source, "built-in");
        assert_eq!(account.address, "ada@gmail.com");
        assert_eq!(
            (
                account.imap.host.as_str(),
                account.imap.port,
                account.imap.security.as_str()
            ),
            ("imap.gmail.com", 993, "tls")
        );
        assert_eq!(account.smtp.host, "smtp.gmail.com");
        let err = pim.discover_account("not an address").await.unwrap_err();
        assert_eq!(error_name(&err), "org.freedesktop.DBus.Error.Failed");
        instance.shutdown().await;
    });
}

#[test]
fn only_one_daemon_per_bus() {
    let bus = Bus::start();
    let tmp = tempfile::tempdir().unwrap();
    smol::block_on(async {
        let first = start(
            &bus,
            &Paths::with_root(tmp.path().join("a")),
            Secrets::memory(),
        )
        .await
        .unwrap();
        let second = start(
            &bus,
            &Paths::with_root(tmp.path().join("b")),
            Secrets::memory(),
        )
        .await;
        assert!(
            matches!(second, Err(StartError::AlreadyRunning)),
            "{:?}",
            second.err()
        );
        first.shutdown().await;
    });
}

#[test]
fn imported_accounts_are_listed_but_not_synced() {
    let bus = Bus::start();
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths::with_root(tmp.path());
    let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
    store
        .add_account(AccountKind::Local, "enron", "enron@local")
        .unwrap();
    drop(store);
    smol::block_on(async {
        let instance = start(&bus, &paths, Secrets::memory()).await.unwrap();
        let pim = PimProxy::new(&bus.connect().await).await.unwrap();
        let accounts = pim.accounts().await.unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].kind, "local");
        assert_eq!(accounts[0].state, state::NOT_SYNCED);
        instance.shutdown().await;
    });
}

#[test]
fn indexes_the_store_for_search() {
    let bus = Bus::start();
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths::with_root(tmp.path());
    let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
    let import = |store: &mut Store, subject: &str| {
        let raw = format!(
            "From: kenneth.lay@enron.com\r\nSubject: {subject}\r\n\
             Date: Mon, 14 May 2001 16:39:00 -0700\r\n\r\nThe budget.\r\n"
        );
        let account = StoreSink::local_account(store, "enron").unwrap();
        let message = IncomingMessage {
            folder: "inbox".into(),
            flags: Flags::default(),
            parsed: parse_message(raw.as_bytes()).unwrap(),
            raw: raw.into_bytes(),
        };
        StoreSink::new(store, account.id).write(&[message]).unwrap();
    };
    import(&mut store, "2001 budget");
    smol::block_on(async {
        let instance = start(&bus, &paths, Secrets::memory()).await.unwrap();
        // The index exists as soon as the daemon has started.
        let index = SearchIndex::open_read_only(&paths.index_dir()).unwrap();
        let found = |text: &str| {
            let query = Query::parse(text).unwrap();
            index
                .search(&query, &SearchOptions::default())
                .unwrap()
                .hits
                .len()
        };
        within("first index", 20, async {
            while found("budget") == 0 {
                Timer::after(Duration::from_millis(50)).await;
            }
        })
        .await;

        // Mail added by another writer is found by the fallback poll.
        import(&mut store, "Budget review");
        within("the new message", 20, async {
            while found("review") == 0 {
                Timer::after(Duration::from_millis(100)).await;
            }
        })
        .await;
        instance.shutdown().await;
    });
}

#[test]
fn changes_imported_mail_in_the_store() {
    let bus = Bus::start();
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths::with_root(tmp.path());
    let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
    let account = store
        .add_account(AccountKind::Local, "enron", "enron@local")
        .unwrap()
        .id;
    let mut batch = store.mail_batch().unwrap();
    let inbox = batch.ensure_folder(account, "inbox").unwrap();
    let old = batch.ensure_folder(account, "old").unwrap();
    let mut ids = Vec::new();
    for subject in ["one", "two"] {
        let raw = format!("Subject: {subject}\r\n\r\nHello.\r\n");
        let added = batch
            .add_message(
                account,
                inbox,
                &NewMessage {
                    raw: raw.as_bytes(),
                    message_id_hdr: None,
                    subject: Some(subject),
                    date: None,
                    flags: MessageFlags::empty(),
                    has_attachments: false,
                    list_id: None,
                    snippet: None,
                    participants: &[],
                    in_reply_to: None,
                    references: &[],
                    category: None,
                },
            )
            .unwrap();
        let Added::Message(id) = added else {
            unreachable!()
        };
        ids.push(id);
    }
    batch.commit().unwrap();
    drop(store);

    smol::block_on(async {
        let instance = start(&bus, &paths, Secrets::memory()).await.unwrap();
        let client = bus.connect().await;
        let pim = PimProxy::new(&client).await.unwrap();
        let reader = Store::open(&paths, Mode::ReadOnly).unwrap();
        let flags = |id| reader.messages_by_id(&[id]).unwrap()[0].flags;
        let mut changed = pim.receive_mail_changed().await.unwrap();

        pim.set_flags(&[ids[0].0, ids[1].0], &["seen", "flagged"], &[])
            .await
            .unwrap();
        within("MailChanged", 5, changed.next()).await.unwrap();
        assert_eq!(flags(ids[0]), MessageFlags::SEEN | MessageFlags::FLAGGED);
        pim.set_flags(&[ids[0].0], &[], &["flagged"]).await.unwrap();
        assert_eq!(flags(ids[0]), MessageFlags::SEEN);
        assert_eq!(flags(ids[1]), MessageFlags::SEEN | MessageFlags::FLAGGED);

        pim.move_messages(&[ids[0].0], old.0).await.unwrap();
        assert_eq!(reader.messages_in_folder(old).unwrap()[0].id, ids[0]);
        // No trash: deleted for good.
        pim.delete_messages(&[ids[1].0]).await.unwrap();
        assert!(reader.messages_by_id(&[ids[1]]).unwrap().is_empty());
        assert!(reader.messages_in_folder(inbox).unwrap().is_empty());
        // Nothing waits for a server that does not exist.
        assert_eq!(reader.next_op_due(account).unwrap(), None);

        let err = pim
            .set_flags(&[ids[0].0], &["urgent"], &[])
            .await
            .unwrap_err();
        assert_eq!(error_name(&err), "org.freedesktop.DBus.Error.InvalidArgs");
        let err = pim.delete_messages(&[999_999]).await.unwrap_err();
        assert_eq!(error_name(&err), "org.freedesktop.DBus.Error.UnknownObject");
        let err = pim.move_messages(&[ids[0].0], 999_999).await.unwrap_err();
        assert_eq!(error_name(&err), "org.freedesktop.DBus.Error.UnknownObject");
        let err = pim.archive_messages(&[ids[0].0]).await.unwrap_err();
        assert_eq!(error_name(&err), "org.freedesktop.DBus.Error.Failed");
        assert!(err.to_string().contains("no archive folder"), "{err}");
        instance.shutdown().await;
    });
}

#[test]
fn queues_undoes_and_retries_outgoing_mail() {
    let bus = Bus::start();
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths::with_root(tmp.path());
    // Nothing listens on this port once the listener is gone.
    let closed = std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
    let local = store
        .add_account(AccountKind::Local, "enron", "enron@local")
        .unwrap()
        .id;
    let account = store
        .add_account(AccountKind::Imap, "Alice", "alice@katna.test")
        .unwrap()
        .id;
    let smtp = Server {
        host: "127.0.0.1".into(),
        port: closed,
        security: katna_core::Security::Tls,
        username: "alice@katna.test".into(),
        accept_invalid_certs: true,
    };
    let settings = AccountSettings {
        smtp: Some(smtp),
        ..AccountSettings::default()
    };
    store.set_account_settings(account, &settings).unwrap();
    drop(store);
    let secrets = Secrets::memory();
    let Secrets::Memory(passwords) = &secrets else {
        unreachable!()
    };
    passwords
        .lock()
        .unwrap()
        .insert(account, "katna-dev".into());
    let message = b"From: alice@katna.test\r\nTo: bob@katna.test\r\n\
        Subject: Lunch\r\n\r\nNoon?\r\n";

    smol::block_on(async {
        let instance = start(&bus, &paths, secrets).await.unwrap();
        let client = bus.connect().await;
        let pim = PimProxy::new(&client).await.unwrap();
        let mut changed = pim.receive_outbox_changed().await.unwrap();

        let err = pim.queue_send(local.0, message, 0).await.unwrap_err();
        assert_eq!(error_name(&err), "org.freedesktop.DBus.Error.InvalidArgs");
        assert!(err.to_string().contains("no SMTP server"), "{err}");
        let err = pim
            .queue_send(account.0, b"Subject: x\r\n\r\n", 0)
            .await
            .unwrap_err();
        assert_eq!(error_name(&err), "org.freedesktop.DBus.Error.InvalidArgs");

        // Undo send: the message waits for its delay.
        let id = pim.queue_send(account.0, message, 3600).await.unwrap();
        let signal = within("OutboxChanged", 5, changed.next()).await.unwrap();
        assert_eq!(signal.args().unwrap().id, id);
        let outbox = pim.outbox().await.unwrap();
        assert_eq!(outbox.len(), 1);
        assert_eq!(
            (
                outbox[0].account,
                outbox[0].subject.as_str(),
                outbox[0].state.as_str()
            ),
            (account.0, "Lunch", send_state::QUEUED)
        );
        assert!(!pim.discard_send(id).await.unwrap(), "still queued");
        assert!(pim.undo_send(id).await.unwrap());
        assert!(!pim.undo_send(id).await.unwrap());
        assert_eq!(pim.outbox().await.unwrap()[0].state, send_state::CANCELLED);
        assert!(pim.discard_send(id).await.unwrap());
        assert!(pim.outbox().await.unwrap().is_empty());

        // Offline: it goes back in the queue and says why.
        let id = pim.queue_send(account.0, message, 0).await.unwrap();
        within("retry", 10, async {
            loop {
                changed.next().await;
                let outbox = pim.outbox().await.unwrap();
                if let Some(item) = outbox.iter().find(|item| item.id == id)
                    && item.state == send_state::QUEUED
                    && item.detail.starts_with("offline")
                {
                    break;
                }
            }
        })
        .await;

        // Removing the account takes its outgoing mail with it.
        assert!(pim.remove_account(account.0).await.unwrap());
        assert!(pim.outbox().await.unwrap().is_empty());
        instance.shutdown().await;
    });
}

fn port(var: &str, default: u16) -> u16 {
    std::env::var(var)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

fn unique(prefix: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{prefix}-{nanos}")
}

async fn wait_until_online(pim: &PimProxy<'_>, id: i64) {
    let mut changes = pim.receive_sync_status_changed().await.unwrap();
    within("online", 30, async {
        loop {
            let accounts = pim.accounts().await.unwrap();
            let account = accounts.iter().find(|a| a.id == id).unwrap();
            if account.state == state::ONLINE {
                assert!(account.last_sync > 0);
                return;
            }
            assert_ne!(account.state, state::AUTH_FAILED, "{account:?}");
            changes.next().await;
        }
    })
    .await;
}

#[test]
#[ignore = "needs the dev servers: docker compose -f dev/compose.yaml up -d"]
fn adds_syncs_restarts_and_removes_dev_accounts() {
    let servers = [
        ("stalwart", port("KATNA_STALWART_IMAPS_PORT", 10993), "tls"),
        (
            "dovecot",
            port("KATNA_DOVECOT_IMAP_PORT", 20143),
            "starttls",
        ),
    ];
    for (name, imap_port, security) in servers {
        let bus = Bus::start();
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        let secrets = Secrets::memory();
        let Secrets::Memory(passwords) = &secrets else {
            unreachable!()
        };
        let passwords = passwords.clone();
        smol::block_on(async {
            let instance = start(&bus, &paths, secrets).await.unwrap();
            let client = bus.connect().await;
            let pim = PimProxy::new(&client).await.unwrap();
            let account = imap("127.0.0.1", imap_port, security);

            let err = pim.add_imap_account(&account, "wrong").await.unwrap_err();
            assert_eq!(
                error_name(&err),
                "org.freedesktop.DBus.Error.AuthFailed",
                "{name}: {err}"
            );

            let id = pim.add_imap_account(&account, "katna-dev").await.unwrap();
            wait_until_online(&pim, id).await;
            let reader = Store::open(&paths, Mode::ReadOnly).unwrap();
            let folders = reader.folders(katna_core::AccountId(id)).unwrap();
            assert!(folders.iter().any(|f| f.path == "INBOX"), "{name}");
            let before = reader.message_count().unwrap();
            assert!(before > 0, "{name}");

            // New mail: the daemon's IDLE sees it and signals.
            let mut changed = pim.receive_mail_changed().await.unwrap();
            let endpoint = Endpoint::new(
                "127.0.0.1",
                imap_port,
                if security == "tls" {
                    Security::Tls
                } else {
                    Security::StartTls
                },
            );
            let mut other = ImapBackend::connect(
                &endpoint,
                &Credentials::new("alice@katna.test", "katna-dev"),
                Tls::insecure_for_local_tests(),
            )
            .await
            .unwrap();
            let subject = unique("daemon");
            let message = format!(
                "From: <alice@katna.test>\r\nTo: <alice@katna.test>\r\nSubject: {subject}\r\n\
                 Message-ID: <{subject}@katna.test>\r\n\r\nHello.\r\n"
            );
            other.append("INBOX", message.into_bytes()).await.unwrap();
            // Body downloads signal too; wait for the one with the new mail.
            within("MailChanged", 10, async {
                while reader.message_count().unwrap() == before {
                    let signal = changed.next().await.unwrap();
                    assert_eq!(signal.args().unwrap().account, id);
                }
            })
            .await;
            assert_eq!(reader.message_count().unwrap(), before + 1, "{name}");
            other.logout().await.unwrap();

            // Every message can be downloaded on request; bodies in the
            // offline window are there already.
            let inbox = folders.iter().find(|f| f.path == "INBOX").unwrap();
            for message in reader.messages_in_folder(inbox.id).unwrap() {
                pim.fetch_body(message.id.0).await.unwrap();
                let stored = &reader.messages_by_id(&[message.id]).unwrap()[0];
                assert!(stored.blob_hash.is_some(), "{name}: {}", stored.subject);
            }
            let err = pim.fetch_body(999_999).await.unwrap_err();
            assert_eq!(error_name(&err), "org.freedesktop.DBus.Error.UnknownObject");

            // A change goes out at once; flag a message and put it back.
            let first = reader.messages_in_folder(inbox.id).unwrap()[0].id;
            let account_id = katna_core::AccountId(id);
            for (add, remove) in [(&["flagged"][..], &[][..]), (&[], &["flagged"])] {
                pim.set_flags(&[first.0], add, remove).await.unwrap();
                within("change sent", 10, async {
                    while !reader.due_ops(account_id, i64::MAX, 1).unwrap().is_empty() {
                        Timer::after(Duration::from_millis(10)).await;
                    }
                })
                .await;
            }

            // A restarted daemon picks the account up from the store and
            // the keyring.
            instance.shutdown().await;
            drop(pim);
            drop(client);
            let instance = start(&bus, &paths, Secrets::Memory(passwords.clone()))
                .await
                .unwrap();
            let client = bus.connect().await;
            let pim = PimProxy::new(&client).await.unwrap();
            wait_until_online(&pim, id).await;
            pim.sync_now(id).await.unwrap();

            assert!(pim.remove_account(id).await.unwrap());
            assert!(pim.accounts().await.unwrap().is_empty());
            assert!(
                reader
                    .folders(katna_core::AccountId(id))
                    .unwrap()
                    .is_empty()
            );
            assert_eq!(reader.message_count().unwrap(), 0, "{name}");
            assert!(passwords.lock().unwrap().is_empty());
            instance.shutdown().await;
        });
    }
}

#[test]
#[ignore = "needs the dev servers: docker compose -f dev/compose.yaml up -d"]
fn pop3_accounts_on_dev_servers() {
    let servers = [
        (
            "stalwart",
            port("KATNA_STALWART_POP3S_PORT", 10995),
            "tls",
            Endpoint::new(
                "127.0.0.1",
                port("KATNA_STALWART_IMAPS_PORT", 10993),
                Security::Tls,
            ),
        ),
        (
            "dovecot",
            port("KATNA_DOVECOT_POP3_PORT", 20110),
            "starttls",
            Endpoint::new(
                "127.0.0.1",
                port("KATNA_DOVECOT_IMAP_PORT", 20143),
                Security::StartTls,
            ),
        ),
    ];
    for (name, pop3_port, security, imap) in servers {
        let bus = Bus::start();
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        let secrets = Secrets::memory();
        let Secrets::Memory(passwords) = &secrets else {
            unreachable!()
        };
        let passwords = passwords.clone();
        smol::block_on(async {
            let instance = start(&bus, &paths, secrets).await.unwrap();
            let client = bus.connect().await;
            let pim = PimProxy::new(&client).await.unwrap();
            let account = NewPop3Account {
                address: "alice@katna.test".into(),
                pop3: ServerSpec {
                    host: "127.0.0.1".into(),
                    port: pop3_port,
                    security: security.into(),
                    username: String::new(),
                    accept_invalid_certs: true,
                },
                leave_on_server: true,
                delete_with_local: false,
                ..NewPop3Account::default()
            };
            let err = pim.add_pop3_account(&account, "wrong").await.unwrap_err();
            assert_eq!(
                error_name(&err),
                "org.freedesktop.DBus.Error.AuthFailed",
                "{name}: {err}"
            );

            let id = pim.add_pop3_account(&account, "katna-dev").await.unwrap();
            wait_until_online(&pim, id).await;
            let listed = pim.accounts().await.unwrap();
            assert_eq!(listed[0].kind, "pop3");
            let reader = Store::open(&paths, Mode::ReadOnly).unwrap();
            let account_id = katna_core::AccountId(id);
            let folders = reader.folders(account_id).unwrap();
            let paths_found: Vec<_> = folders.iter().map(|f| f.path.as_str()).collect();
            assert_eq!(paths_found, ["INBOX", "Sent", "Trash"], "{name}");
            let before = reader.message_count().unwrap();
            assert!(before > 0, "{name}");

            // New mail shows after `sync now`.
            let subject = unique("pop3-daemon");
            let mut other = ImapBackend::connect(
                &imap,
                &Credentials::new("alice@katna.test", "katna-dev"),
                Tls::insecure_for_local_tests(),
            )
            .await
            .unwrap();
            let message = format!(
                "From: <alice@katna.test>\r\nTo: <alice@katna.test>\r\nSubject: {subject}\r\n\
                 Message-ID: <{subject}@katna.test>\r\n\r\nHello.\r\n"
            );
            other.append("INBOX", message.into_bytes()).await.unwrap();
            other.logout().await.unwrap();
            pim.sync_now(id).await.unwrap();
            within("new POP3 mail", 20, async {
                while reader.message_count().unwrap() == before {
                    Timer::after(Duration::from_millis(50)).await;
                }
            })
            .await;
            let inbox = &folders[0];
            let stored = reader.messages_in_folder(inbox.id).unwrap();
            let new = stored.iter().find(|m| m.subject == subject).unwrap();
            assert!(new.blob_hash.is_some(), "{name}: POP3 mail is stored whole");

            assert!(pim.remove_account(id).await.unwrap());
            assert!(reader.folders(account_id).unwrap().is_empty());
            assert!(reader.pop3_uidls(account_id).unwrap().is_empty());
            assert!(passwords.lock().unwrap().is_empty());
            instance.shutdown().await;
        });
    }
}

#[test]
#[ignore = "needs the dev servers: docker compose -f dev/compose.yaml up -d"]
fn sends_through_dev_servers() {
    let servers = [
        (
            "stalwart",
            port("KATNA_STALWART_IMAPS_PORT", 10993),
            "tls",
            port("KATNA_STALWART_SUBMISSIONS_PORT", 10465),
            "tls",
        ),
        (
            "dovecot",
            port("KATNA_DOVECOT_IMAP_PORT", 20143),
            "starttls",
            port("KATNA_DOVECOT_SUBMISSION_PORT", 20587),
            "starttls",
        ),
    ];
    for (name, imap_port, security, smtp_port, smtp_security) in servers {
        let bus = Bus::start();
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        smol::block_on(async {
            let instance = start(&bus, &paths, Secrets::memory()).await.unwrap();
            let client = bus.connect().await;
            let pim = PimProxy::new(&client).await.unwrap();
            let mut account = imap("127.0.0.1", imap_port, security);
            account.smtp = ServerSpec {
                host: "127.0.0.1".into(),
                port: smtp_port,
                security: smtp_security.into(),
                username: String::new(),
                accept_invalid_certs: true,
            };
            let id = pim.add_imap_account(&account, "katna-dev").await.unwrap();
            wait_until_online(&pim, id).await;
            let reader = Store::open(&paths, Mode::ReadOnly).unwrap();
            let account_id = katna_core::AccountId(id);
            let sent = reader
                .folders(account_id)
                .unwrap()
                .into_iter()
                .find(|f| f.role == Some(katna_store::FolderRole::Sent));

            let subject = unique("send");
            let message = format!(
                "From: Alice <alice@katna.test>\r\nTo: alice@katna.test\r\n\
                 Bcc: bob@katna.test\r\nSubject: {subject}\r\n\r\nSent by Katna.\r\n"
            );
            let started = std::time::Instant::now();
            let outbox_id = pim.queue_send(id, message.as_bytes(), 0).await.unwrap();
            // Sent, then filed: the outbox forgets it.
            within("sent and filed", 30, async {
                // One read per round: the item can leave between two reads.
                while let Some(item) = pim.outbox().await.unwrap().first() {
                    assert_eq!(item.id, outbox_id);
                    assert_ne!(item.state, send_state::FAILED, "{name}: {item:?}");
                    Timer::after(Duration::from_millis(20)).await;
                }
            })
            .await;
            eprintln!("{name}: sent and filed in {:?}", started.elapsed());

            let has = |folder: katna_store::FolderId| {
                reader
                    .messages_in_folder(folder)
                    .unwrap()
                    .into_iter()
                    .find(|m| m.subject == subject)
            };
            if let Some(sent) = &sent {
                let copy = within("copy in Sent", 30, async {
                    loop {
                        if let Some(copy) = has(sent.id) {
                            return copy;
                        }
                        Timer::after(Duration::from_millis(50)).await;
                    }
                })
                .await;
                assert!(copy.flags.contains(MessageFlags::SEEN), "{name}");
                let copies = reader.messages_in_folder(sent.id).unwrap();
                let copies = copies.iter().filter(|m| m.subject == subject).count();
                assert_eq!(copies, 1, "{name}: the server filed one too");
                pim.fetch_body(copy.id.0).await.unwrap();
                let stored = &reader.messages_by_id(&[copy.id]).unwrap()[0];
                let raw = reader
                    .blobs()
                    .get(&stored.blob_hash.unwrap())
                    .unwrap()
                    .unwrap();
                let raw = String::from_utf8_lossy(&raw);
                assert!(
                    raw.contains("Bcc: bob@katna.test"),
                    "{name}: the sender's copy keeps Bcc"
                );
                assert!(raw.contains("Message-ID: <"), "{name}");
            } else {
                eprintln!("{name}: no Sent folder; nothing filed");
            }

            if name == "stalwart" {
                // Delivered back to alice, without the Bcc line.
                let inbox = reader
                    .folders(account_id)
                    .unwrap()
                    .into_iter()
                    .find(|f| f.path == "INBOX")
                    .unwrap();
                let delivered = within("delivery", 30, async {
                    loop {
                        if let Some(message) = has(inbox.id) {
                            return message;
                        }
                        pim.sync_now(id).await.unwrap();
                        Timer::after(Duration::from_millis(200)).await;
                    }
                })
                .await;
                pim.fetch_body(delivered.id.0).await.unwrap();
                let stored = &reader.messages_by_id(&[delivered.id]).unwrap()[0];
                let raw = reader
                    .blobs()
                    .get(&stored.blob_hash.unwrap())
                    .unwrap()
                    .unwrap();
                let raw = String::from_utf8_lossy(&raw);
                assert!(
                    !raw.contains("bob@katna.test"),
                    "{name}: Bcc leaked:\n{raw}"
                );
            } else {
                within("Mailpit", 30, async {
                    while !mailpit_has(&subject) {
                        Timer::after(Duration::from_millis(200)).await;
                    }
                })
                .await;
            }

            assert!(pim.remove_account(id).await.unwrap());
            instance.shutdown().await;
        });
    }
}

/// Asks Mailpit's API whether a message with this subject arrived.
fn mailpit_has(subject: &str) -> bool {
    use std::io::{Read, Write};
    let port = port("KATNA_MAILPIT_HTTP_PORT", 8025);
    let mut stream = std::net::TcpStream::connect(("127.0.0.1", port)).unwrap();
    let request =
        format!("GET /api/v1/search?query=subject:{subject} HTTP/1.0\r\nHost: 127.0.0.1\r\n\r\n");
    stream.write_all(request.as_bytes()).unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    response.contains(subject)
}

/// logind and NetworkManager as far as the daemon listens to them.
struct FakeLogin;

#[zbus::interface(name = "org.freedesktop.login1.Manager")]
impl FakeLogin {
    #[zbus(signal)]
    async fn prepare_for_sleep(
        emitter: &zbus::object_server::SignalEmitter<'_>,
        start: bool,
    ) -> zbus::Result<()>;
}

struct FakeNetworkManager {
    metered: u32,
}

#[zbus::interface(name = "org.freedesktop.NetworkManager")]
impl FakeNetworkManager {
    #[zbus(signal)]
    async fn state_changed(
        emitter: &zbus::object_server::SignalEmitter<'_>,
        state: u32,
    ) -> zbus::Result<()>;

    #[zbus(property)]
    fn metered(&self) -> u32 {
        self.metered
    }
}

#[test]
fn watches_suspend_network_and_metering_on_the_system_bus() {
    use katna_daemon::system::{self, SystemEvent};
    use zbus::object_server::SignalEmitter;

    let system_bus = Bus::start();
    smol::block_on(async {
        let services = system_bus.connect().await;
        services
            .object_server()
            .at("/org/freedesktop/login1", FakeLogin)
            .await
            .unwrap();
        services
            .object_server()
            // "Guess yes", as on a phone hotspot.
            .at(
                "/org/freedesktop/NetworkManager",
                FakeNetworkManager { metered: 3 },
            )
            .await
            .unwrap();
        services
            .request_name("org.freedesktop.login1")
            .await
            .unwrap();
        services
            .request_name("org.freedesktop.NetworkManager")
            .await
            .unwrap();

        let (events_tx, events) = async_channel::unbounded();
        let watcher = system_bus.connect().await;
        smol::spawn(system::watch(watcher, move |event| {
            let _ = events_tx.try_send(event);
        }))
        .detach();
        // Let the watcher add its match rules.
        Timer::after(Duration::from_millis(300)).await;

        let login = SignalEmitter::new(&services, "/org/freedesktop/login1").unwrap();
        let network = SignalEmitter::new(&services, "/org/freedesktop/NetworkManager").unwrap();
        FakeLogin::prepare_for_sleep(&login, true).await.unwrap();
        FakeNetworkManager::state_changed(&network, 20)
            .await
            .unwrap(); // disconnected
        FakeLogin::prepare_for_sleep(&login, false).await.unwrap();
        FakeNetworkManager::state_changed(&network, 70)
            .await
            .unwrap(); // global
        FakeNetworkManager::state_changed(&network, 60)
            .await
            .unwrap(); // site
        FakeNetworkManager::state_changed(&network, 70)
            .await
            .unwrap();
        let manager = services
            .object_server()
            .interface::<_, FakeNetworkManager>("/org/freedesktop/NetworkManager")
            .await
            .unwrap();
        for metered in [4, 2] {
            // "Guess no", then "no": only the first is a change.
            manager.get_mut().await.metered = metered;
            manager
                .get()
                .await
                .metered_changed(manager.signal_emitter())
                .await
                .unwrap();
        }

        let mut seen = Vec::new();
        within("system events", 5, async {
            while seen.len() < 5 {
                seen.push(events.recv().await.unwrap());
            }
        })
        .await;
        assert_eq!(
            seen,
            [
                SystemEvent::Metered(true),
                SystemEvent::Resumed,
                SystemEvent::NetworkUp,
                SystemEvent::NetworkUp,
                SystemEvent::Metered(false),
            ]
        );
        Timer::after(Duration::from_millis(100)).await;
        assert!(events.is_empty(), "going to sleep or down is not an event");
    });
}

/// `sync.metered` wins over NetworkManager, and `ReloadConfig` applies a
/// saved change at once.
#[test]
fn metered_setting_overrides_the_network() {
    use katna_core::{Config, config::Metered};

    let system_bus = Bus::start();
    let session_bus = Bus::start();
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths::with_root(tmp.path());
    let save = |metered: Metered| {
        let mut config = Config::default();
        config.sync.metered = metered;
        config.save(&paths.config_file()).unwrap();
    };
    save(Metered::Never);
    smol::block_on(async {
        let services = system_bus.connect().await;
        services
            .object_server()
            // "Guess yes", as on a phone hotspot.
            .at(
                "/org/freedesktop/NetworkManager",
                FakeNetworkManager { metered: 3 },
            )
            .await
            .unwrap();
        services
            .request_name("org.freedesktop.NetworkManager")
            .await
            .unwrap();

        let instance = start(&session_bus, &paths, Secrets::memory())
            .await
            .unwrap();
        instance.watch_system(system_bus.connect().await);
        let client = session_bus.connect().await;
        let pim = PimProxy::new(&client).await.unwrap();
        let mut changes = pim.receive_metered_changed().await.unwrap();
        // Let the watcher read NetworkManager.
        Timer::after(Duration::from_millis(500)).await;
        assert!(
            !pim.metered().await.unwrap(),
            "never, whatever the network says"
        );

        let mut next = async || {
            within("MeteredChanged", 5, changes.next())
                .await
                .unwrap()
                .args()
                .unwrap()
                .metered
        };
        save(Metered::Auto);
        pim.reload_config().await.unwrap();
        assert!(next().await, "auto follows the network");
        assert!(pim.metered().await.unwrap());

        save(Metered::Always);
        pim.reload_config().await.unwrap();
        let manager = services
            .object_server()
            .interface::<_, FakeNetworkManager>("/org/freedesktop/NetworkManager")
            .await
            .unwrap();
        manager.get_mut().await.metered = 2;
        manager
            .get()
            .await
            .metered_changed(manager.signal_emitter())
            .await
            .unwrap();
        Timer::after(Duration::from_millis(300)).await;
        assert!(
            pim.metered().await.unwrap(),
            "always, whatever the network says"
        );

        save(Metered::Auto);
        pim.reload_config().await.unwrap();
        assert!(!next().await, "the network is not metered any more");
        assert!(!pim.metered().await.unwrap());
    });
}
