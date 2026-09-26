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
use katna_core::{AccountKind, Paths};
use katna_daemon::{Instance, StartError, secrets::Secrets};
use katna_dbus::{NewImapAccount, PimProxy, ServerSpec, state};
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
