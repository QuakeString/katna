// SPDX-License-Identifier: GPL-3.0-or-later

//! The daemon on a private session bus, driven through the client proxy.
//!
//! Needs `dbus-daemon` (package `dbus` on Arch, `dbus-daemon` on Debian and
//! Ubuntu). The `#[ignore]`d tests also need the dev servers:
//! `docker compose -f dev/compose.yaml up -d`, then
//! `cargo test -p katna-daemon --test dbus -- --ignored --test-threads 1`.
//! Windows runs the bus end to end in `katna-dbus`'s `windows_bus` test.
#![cfg(unix)]

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
        let (account, source, sign_in, password) =
            pim.discover_account(" ada@gmail.com ").await.unwrap();
        assert_eq!(source, "built-in");
        assert_eq!((sign_in.as_str(), password), ("google", true));
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

        // Microsoft's own addresses only sign in in the browser.
        let (account, _, sign_in, password) =
            pim.discover_account("kay@outlook.com").await.unwrap();
        assert_eq!((sign_in.as_str(), password), ("microsoft", false));
        assert_eq!(account.imap.host, "outlook.office365.com");
        let err = pim.sign_in("yahoo", 0, "").await.unwrap_err();
        assert_eq!(error_name(&err), "org.freedesktop.DBus.Error.InvalidArgs");
        if !katna_core::OAuthProvider::Microsoft.available() {
            let err = pim.sign_in("microsoft", 0, "").await.unwrap_err();
            assert_eq!(error_name(&err), "org.freedesktop.DBus.Error.Failed");
        }
        assert!(!pim.cancel_sign_in().await.unwrap());
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

/// Translation without a server: mail in the reading language, unknown
/// messages and stored translations need no network.
#[test]
fn translates_from_the_store_and_never_sends_mail_in_the_reading_language() {
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
    let spanish = "Hola Ana, gracias por tu mensaje. Nos vemos el martes en la oficina \
                   para hablar del nuevo proyecto.";
    let raw = format!("Subject: Hola\r\n\r\n{spanish}\r\n");
    let added = batch
        .add_message(
            account,
            inbox,
            &NewMessage {
                raw: raw.as_bytes(),
                message_id_hdr: None,
                subject: Some("Hola"),
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
    batch.commit().unwrap();
    let Added::Message(id) = added else {
        unreachable!()
    };
    let kept = katna_store::Translation {
        source: "es".into(),
        text: "Hi Ana, thanks for your message.".into(),
    };
    store
        .save_translation(id, "en", spanish, &kept, 1_790_000_000)
        .unwrap();
    drop(store);

    smol::block_on(async {
        let instance = start(&bus, &paths, Secrets::memory()).await.unwrap();
        let pim = PimProxy::new(&bus.connect().await).await.unwrap();
        let (source, text, problem) = pim.translate(id.0, spanish, "es", "en").await.unwrap();
        assert_eq!((source.as_str(), text.as_str()), ("es", kept.text.as_str()));
        assert_eq!(problem, "");

        let english = "Hi Sam, thanks for the notes from the meeting yesterday. I will send \
                       the plan to the whole team before Friday.";
        let (_, text, problem) = pim.translate(id.0, english, "en", "en").await.unwrap();
        assert_eq!(problem, katna_dbus::translate_problem::SAME_LANGUAGE);
        assert!(text.is_empty());

        let (_, _, problem) = pim.translate(999_999, spanish, "es", "en").await.unwrap();
        assert_eq!(problem, katna_dbus::translate_problem::FAILED);
        let (_, _, problem) = pim
            .translate(id.0, spanish, "es", "EN; drop")
            .await
            .unwrap();
        assert_eq!(problem, katna_dbus::translate_problem::FAILED);
        instance.shutdown().await;
    });
}

/// KRunner and GNOME Shell find people as they are typed, and mail whose
/// subject or sender has every word; `mail:` or a trigger word searches
/// everything.
#[test]
fn answers_krunner_and_gnome_search() {
    use std::collections::HashMap;
    use zbus::zvariant::OwnedValue;

    type Match = (
        String,
        String,
        String,
        i32,
        f64,
        HashMap<String, OwnedValue>,
    );

    let bus = Bus::start();
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths::with_root(tmp.path());
    let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
    let account = StoreSink::local_account(&mut store, "enron").unwrap();
    let messages: Vec<IncomingMessage> = [
        (
            "Kenneth Lay <kenneth.lay@enron.com>",
            "2001 budget",
            "The numbers.",
        ),
        (
            "jeff.skilling@enron.com",
            "Lunch",
            "The budget, over lunch.",
        ),
    ]
    .into_iter()
    .map(|(from, subject, body)| {
        let raw = format!(
            "From: {from}\r\nTo: me@enron.com\r\nSubject: {subject}\r\n\
             Date: Mon, 14 May 2001 16:39:00 -0700\r\n\r\n{body}\r\n"
        );
        IncomingMessage {
            folder: "inbox".into(),
            flags: Flags::default(),
            parsed: parse_message(raw.as_bytes()).unwrap(),
            raw: raw.into_bytes(),
        }
    })
    .collect();
    StoreSink::new(&mut store, account.id)
        .write(&messages)
        .unwrap();
    drop(store);
    smol::block_on(async {
        let instance = start(&bus, &paths, Secrets::memory()).await.unwrap();
        let connection = bus.connect().await;
        let krunner = |query: &'static str| {
            let connection = connection.clone();
            async move {
                let reply = connection
                    .call_method(
                        Some(katna_core::ids::DAEMON_BUS_NAME),
                        katna_core::ids::RUNNER_OBJECT_PATH,
                        Some("org.kde.krunner1"),
                        "Match",
                        &(query,),
                    )
                    .await
                    .unwrap();
                reply.body().deserialize::<Vec<Match>>().unwrap()
            }
        };
        // The index and the address book fill in the background.
        let people = within("people", 20, async {
            loop {
                let found = krunner("kenn").await;
                if found.iter().any(|m| m.0.starts_with('c')) {
                    break found;
                }
                Timer::after(Duration::from_millis(100)).await;
            }
        })
        .await;
        assert_eq!(people[0].0, "ckenneth.lay@enron.com");
        assert_eq!(people[0].1, "Kenneth Lay");
        let subtext = String::try_from(people[0].5["subtext"].try_clone().unwrap()).unwrap();
        assert_eq!(subtext, "kenneth.lay@enron.com");

        let mail = within("mail", 20, async {
            loop {
                let found = krunner("budget").await;
                if !found.is_empty() {
                    break found;
                }
                Timer::after(Duration::from_millis(100)).await;
            }
        })
        .await;
        // Only the message with "budget" in its subject.
        assert_eq!(mail.len(), 1, "{mail:?}");
        assert_eq!(mail[0].1, "2001 budget");
        assert_eq!(mail[0].2, "mail-unread");
        // Too short, or not plain words: nothing.
        assert!(krunner("bu").await.is_empty());
        // `mail:` searches the text too, as Katna Mail's search box does.
        assert_eq!(krunner("mail: budget").await.len(), 2);
        // So does a trigger word from Settings: "k" and "m" at first.
        assert_eq!(krunner("k budget").await.len(), 2);
        assert!(krunner("k bu").await.is_empty());
        let mut config = katna_core::config::Config::default();
        config.general.search_triggers = vec!["find".into()];
        config.save(&paths.config_file()).unwrap();
        PimProxy::new(&connection)
            .await
            .unwrap()
            .reload_config()
            .await
            .unwrap();
        assert_eq!(krunner("Find budget").await.len(), 2);
        assert!(krunner("k budget").await.is_empty());

        let gnome = |method: &'static str, body: Vec<String>| {
            let connection = connection.clone();
            async move {
                connection
                    .call_method(
                        Some(katna_core::ids::DAEMON_BUS_NAME),
                        katna_core::ids::SEARCH_PROVIDER_OBJECT_PATH,
                        Some("org.gnome.Shell.SearchProvider2"),
                        method,
                        &(body,),
                    )
                    .await
                    .unwrap()
            }
        };
        let ids: Vec<String> = gnome("GetInitialResultSet", vec!["2001".into(), "budget".into()])
            .await
            .body()
            .deserialize()
            .unwrap();
        assert_eq!(ids.len(), 1, "{ids:?}");
        let metas: Vec<HashMap<String, OwnedValue>> = gnome("GetResultMetas", ids.clone())
            .await
            .body()
            .deserialize()
            .unwrap();
        let name = String::try_from(metas[0]["name"].try_clone().unwrap()).unwrap();
        let description = String::try_from(metas[0]["description"].try_clone().unwrap()).unwrap();
        assert_eq!(name, "2001 budget");
        assert_eq!(description, "From Kenneth Lay");
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

        pim.set_pinned(&[ids[1].0], true).await.unwrap();
        let pinned = reader.pinned().unwrap();
        assert_eq!(pinned.len(), 1);
        assert_eq!(pinned[0].message, ids[1]);
        pim.set_pinned(&[ids[1].0], false).await.unwrap();
        assert!(reader.pinned().unwrap().is_empty());

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

/// Stores a message in `folder`; `reply_to` makes it an answer.
fn add_mail(
    batch: &mut katna_store::MailBatch<'_>,
    account: katna_core::AccountId,
    folder: katna_store::FolderId,
    message_id: &str,
    date: i64,
    reply_to: Option<&str>,
    flags: MessageFlags,
) -> katna_store::MessageId {
    let raw = format!("Message-ID: {message_id}\r\nSubject: Offer\r\n\r\nHello.\r\n");
    let added = batch
        .add_message(
            account,
            folder,
            &NewMessage {
                raw: raw.as_bytes(),
                message_id_hdr: Some(message_id),
                subject: Some("Offer"),
                date: Some(date),
                flags,
                has_attachments: false,
                list_id: None,
                snippet: None,
                participants: &[],
                in_reply_to: reply_to,
                references: &[],
                category: None,
            },
        )
        .unwrap();
    let Added::Message(id) = added else {
        unreachable!()
    };
    id
}

#[test]
fn snoozes_and_reminds_across_restarts() {
    use katna_store::FolderRole;
    let bus = Bus::start();
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths::with_root(tmp.path());
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
    let account = store
        .add_account(AccountKind::Local, "enron", "enron@local")
        .unwrap()
        .id;
    let mut batch = store.mail_batch().unwrap();
    let inbox = batch
        .upsert_folder(account, "INBOX", Some(FolderRole::Inbox))
        .unwrap();
    let sent = batch
        .upsert_folder(account, "Sent", Some(FolderRole::Sent))
        .unwrap();
    let seen = MessageFlags::SEEN;
    let first = add_mail(&mut batch, account, inbox, "<1@x>", now - 900, None, seen);
    let second = add_mail(&mut batch, account, inbox, "<2@x>", now - 800, None, seen);
    // Sent with a reminder: one got no answer, one did.
    let unanswered = add_mail(&mut batch, account, sent, "<3@x>", now - 700, None, seen);
    let answered = add_mail(&mut batch, account, sent, "<4@x>", now - 600, None, seen);
    add_mail(
        &mut batch,
        account,
        inbox,
        "<5@x>",
        now - 500,
        Some("<4@x>"),
        seen,
    );
    batch.commit().unwrap();
    for (outbox, header) in [(900_001, "<3@x>"), (900_002, "<4@x>")] {
        let follow_up = katna_meta::FollowUp {
            account: account.0,
            message_id: header.into(),
            subject: "Offer".into(),
            remind_at: now + 3600,
            after: 3600,
        };
        katna_meta::set_follow_up(&mut store, outbox, &follow_up).unwrap();
    }
    drop(store);

    let folder_of = |reader: &Store, id| -> Vec<String> {
        reader.messages_by_id(&[id]).unwrap()[0]
            .locations
            .iter()
            .map(|l| l.path.clone())
            .collect()
    };
    smol::block_on(async {
        let instance = start(&bus, &paths, Secrets::memory()).await.unwrap();
        let client = bus.connect().await;
        let pim = PimProxy::new(&client).await.unwrap();
        let reader = Store::open(&paths, Mode::ReadOnly).unwrap();

        let err = pim.snooze(&[first.0], now).await.unwrap_err();
        assert_eq!(error_name(&err), "org.freedesktop.DBus.Error.InvalidArgs");
        let err = pim.snooze(&[999_999], now + 3600).await.unwrap_err();
        assert_eq!(error_name(&err), "org.freedesktop.DBus.Error.UnknownObject");

        // Snoozed: in the Snoozed folder, made on this computer.
        pim.snooze(&[first.0], now + 3600).await.unwrap();
        assert_eq!(folder_of(&reader, first), ["Snoozed"]);
        let snooze = katna_meta::snooze_of(&reader, first).unwrap().unwrap();
        assert_eq!((snooze.until, snooze.back_to), (now + 3600, inbox.0));
        // Sent mail is not snoozed.
        pim.snooze(&[unanswered.0], now + 3600).await.unwrap();
        assert_eq!(folder_of(&reader, unanswered), ["Sent"]);
        // Unsnoozed (Undo): back as it was.
        pim.unsnooze(&[first.0]).await.unwrap();
        assert_eq!(folder_of(&reader, first), ["INBOX"]);
        assert!(
            reader.messages_by_id(&[first]).unwrap()[0]
                .flags
                .contains(seen)
        );
        assert_eq!(katna_meta::snooze_of(&reader, first).unwrap(), None);

        pim.snooze(&[first.0, second.0], now + 3600).await.unwrap();
        instance.shutdown().await;
    });

    // The time passes while the daemon is not running.
    let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
    for id in [first, second] {
        let mut snooze = katna_meta::snooze_of(&store, id).unwrap().unwrap();
        snooze.until = now - 10;
        katna_meta::set_snooze(&mut store, id, &snooze).unwrap();
    }
    for outbox in [900_001, 900_002] {
        let mut follow_up = katna_meta::follow_up_of(&store, outbox).unwrap().unwrap();
        follow_up.remind_at = now - 10;
        katna_meta::set_follow_up(&mut store, outbox, &follow_up).unwrap();
    }
    drop(store);

    smol::block_on(async {
        let instance = start(&bus, &paths, Secrets::memory()).await.unwrap();
        let reader = Store::open(&paths, Mode::ReadOnly).unwrap();
        within("snoozed mail back", 10, async {
            while !katna_meta::snoozed(&reader).unwrap().is_empty()
                || !katna_meta::follow_ups(&reader).unwrap().is_empty()
            {
                Timer::after(Duration::from_millis(50)).await;
            }
        })
        .await;
        for id in [first, second] {
            assert_eq!(folder_of(&reader, id), ["INBOX"]);
            let flags = reader.messages_by_id(&[id]).unwrap()[0].flags;
            assert!(!flags.contains(seen), "back unread");
        }
        let mut folders = folder_of(&reader, unanswered);
        folders.sort();
        assert_eq!(folders, ["INBOX", "Sent"], "the reminder is in the inbox");
        assert!(
            !reader.messages_by_id(&[unanswered]).unwrap()[0]
                .flags
                .contains(seen)
        );
        assert_eq!(
            folder_of(&reader, answered),
            ["Sent"],
            "answered: no reminder"
        );
        let mut surfaced: Vec<_> = katna_meta::surfaced(&reader)
            .unwrap()
            .into_iter()
            .map(|(id, _)| id)
            .collect();
        surfaced.sort();
        assert_eq!(surfaced, [first, second, unanswered]);
        instance.shutdown().await;
    });
}

#[test]
fn follow_ups_wait_on_outgoing_mail() {
    let bus = Bus::start();
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths::with_root(tmp.path());
    let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
    let account = store
        .add_account(AccountKind::Imap, "Alice", "alice@katna.test")
        .unwrap()
        .id;
    let smtp = Server {
        host: "127.0.0.1".into(),
        port: 1,
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
    let message = b"From: alice@katna.test\r\nTo: bob@katna.test\r\n\
        Subject: Lunch\r\n\r\nNoon?\r\n";

    smol::block_on(async {
        let instance = start(&bus, &paths, Secrets::memory()).await.unwrap();
        let client = bus.connect().await;
        let pim = PimProxy::new(&client).await.unwrap();
        let reader = Store::open(&paths, Mode::ReadOnly).unwrap();

        let err = pim.set_follow_up(424_242, 86_400).await.unwrap_err();
        assert_eq!(error_name(&err), "org.freedesktop.DBus.Error.InvalidArgs");

        let id = pim.queue_send(account.0, message, 3600).await.unwrap();
        pim.set_follow_up(id, 3 * 86_400).await.unwrap();
        let send_at = pim.outbox().await.unwrap()[0].send_at;
        let follow_up = katna_meta::follow_up_of(&reader, id).unwrap().unwrap();
        assert_eq!(follow_up.remind_at, send_at + 3 * 86_400);
        assert_eq!(follow_up.subject, "Lunch");
        assert!(
            follow_up.message_id.contains('@'),
            "{}",
            follow_up.message_id
        );
        // 0 takes it back; so does Undo send.
        pim.set_follow_up(id, 0).await.unwrap();
        assert_eq!(katna_meta::follow_up_of(&reader, id).unwrap(), None);
        pim.set_follow_up(id, 86_400).await.unwrap();
        assert!(pim.undo_send(id).await.unwrap());
        assert_eq!(katna_meta::follow_up_of(&reader, id).unwrap(), None);
        let err = pim.set_follow_up(id, 86_400).await.unwrap_err();
        assert_eq!(error_name(&err), "org.freedesktop.DBus.Error.InvalidArgs");
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

        // Templates: saved, renamed and deleted on this computer.
        let mut template = katna_dbus::TemplateItem {
            name: "Welcome".to_owned(),
            text: "Hi {first name}".to_owned(),
            attachments: vec![katna_dbus::TemplateFileItem {
                name: "a.txt".to_owned(),
                mime: "text/plain".to_owned(),
                data: b"a".to_vec(),
            }],
            ..Default::default()
        };
        let err = pim
            .save_template(&katna_dbus::TemplateItem {
                name: " ".to_owned(),
                ..template.clone()
            })
            .await
            .unwrap_err();
        assert_eq!(error_name(&err), "org.freedesktop.DBus.Error.InvalidArgs");
        template.id = pim.save_template(&template).await.unwrap();
        assert!(pim.rename_template(template.id, "Hello").await.unwrap());
        assert!(pim.delete_template(template.id).await.unwrap());
        assert!(!pim.delete_template(template.id).await.unwrap());

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

/// What the fake notification server was asked.
#[derive(Debug, PartialEq)]
enum Asked {
    Notify {
        summary: String,
        body: String,
        actions: Vec<String>,
        origin: String,
    },
    Close(u32),
}

/// The desktop's notification server, as far as the daemon uses it.
struct FakeNotifications {
    asked: async_channel::Sender<Asked>,
    next: u32,
}

#[zbus::interface(name = "org.freedesktop.Notifications")]
impl FakeNotifications {
    #[allow(clippy::too_many_arguments)]
    fn notify(
        &mut self,
        _app_name: &str,
        _replaces_id: u32,
        _app_icon: &str,
        summary: &str,
        body: &str,
        actions: Vec<String>,
        hints: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        _expire_timeout: i32,
    ) -> u32 {
        let origin = hints
            .get("x-kde-origin-name")
            .and_then(|v| String::try_from(v.clone()).ok())
            .unwrap_or_default();
        self.next += 1;
        let _ = self.asked.try_send(Asked::Notify {
            summary: summary.into(),
            body: body.into(),
            actions,
            origin,
        });
        self.next
    }

    fn close_notification(&self, id: u32) {
        let _ = self.asked.try_send(Asked::Close(id));
    }

    fn get_capabilities(&self) -> Vec<String> {
        vec!["actions".into(), "body".into()]
    }

    #[zbus(signal)]
    async fn action_invoked(
        emitter: &zbus::object_server::SignalEmitter<'_>,
        id: u32,
        action_key: &str,
    ) -> zbus::Result<()>;
}

/// New mail in the inbox becomes a desktop notification; its Mark as read
/// button marks it read; `notifications.new_mail = false` turns them off.
#[test]
#[ignore = "needs the dev servers: docker compose -f dev/compose.yaml up -d"]
fn notifies_about_new_mail_on_dev_servers() {
    use katna_core::Config;
    use zbus::object_server::SignalEmitter;

    let imap_port = port("KATNA_STALWART_IMAPS_PORT", 10993);
    let bus = Bus::start();
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths::with_root(tmp.path());
    smol::block_on(async {
        let (asked_tx, asked) = async_channel::unbounded();
        let desktop = bus.connect().await;
        desktop
            .object_server()
            .at(
                "/org/freedesktop/Notifications",
                FakeNotifications {
                    asked: asked_tx,
                    next: 0,
                },
            )
            .await
            .unwrap();
        desktop
            .request_name("org.freedesktop.Notifications")
            .await
            .unwrap();

        let instance = start(&bus, &paths, Secrets::memory()).await.unwrap();
        let client = bus.connect().await;
        let pim = PimProxy::new(&client).await.unwrap();
        let account = imap("127.0.0.1", imap_port, "tls");
        let id = pim.add_imap_account(&account, "katna-dev").await.unwrap();
        wait_until_online(&pim, id).await;
        Timer::after(Duration::from_millis(300)).await;
        assert!(asked.is_empty(), "mail already there is not news");

        let mut other = ImapBackend::connect(
            &Endpoint::new("127.0.0.1", imap_port, Security::Tls),
            &Credentials::new("alice@katna.test", "katna-dev"),
            Tls::insecure_for_local_tests(),
        )
        .await
        .unwrap();
        let mut append = async |subject: &str| {
            // No Date header: the server's arrival time counts.
            let message = format!(
                "From: Carol Example <carol@katna.test>\r\nTo: <alice@katna.test>\r\n\
                 Subject: {subject}\r\nMessage-ID: <{subject}@katna.test>\r\n\r\nSee you at 3.\r\n"
            );
            other.append("INBOX", message.into_bytes()).await.unwrap();
        };
        let subject = unique("notify");
        append(&subject).await;
        let mut shown = within("Notify", 15, asked.recv()).await.unwrap();
        // The preview is there when the body was downloaded in the same
        // sync.
        if let Asked::Notify { body, .. } = &mut shown {
            let preview = format!("{subject}\nSee you at 3.");
            assert!(*body == subject || *body == preview, "{body}");
            *body = subject.clone();
        }
        assert_eq!(
            shown,
            Asked::Notify {
                summary: "Carol Example".into(),
                body: subject.clone(),
                actions: [
                    "default",
                    "Open",
                    "reply-all",
                    "Reply all",
                    "mark-read",
                    "Mark as read",
                    "archive",
                    "Archive"
                ]
                .map(String::from)
                .to_vec(),
                origin: "alice@katna.test".into(),
            }
        );

        // Mark as read: the mail is read in the store, and the
        // notification closes.
        let reader = Store::open(&paths, Mode::ReadOnly).unwrap();
        // The daemon records the notification when Notify returns; a
        // click cannot come sooner.
        Timer::after(Duration::from_millis(300)).await;
        let emitter = SignalEmitter::new(&desktop, "/org/freedesktop/Notifications").unwrap();
        FakeNotifications::action_invoked(&emitter, 1, "mark-read")
            .await
            .unwrap();
        assert_eq!(
            within("CloseNotification", 5, asked.recv()).await.unwrap(),
            Asked::Close(1)
        );
        let inbox = reader
            .folders(katna_core::AccountId(id))
            .unwrap()
            .into_iter()
            .find(|f| f.path == "INBOX")
            .unwrap();
        let read = reader
            .messages_in_folder(inbox.id)
            .unwrap()
            .into_iter()
            .find(|m| m.subject == subject)
            .unwrap();
        assert!(read.flags.contains(MessageFlags::SEEN));

        // Turned off in the settings: nothing more.
        let mut config = Config::default();
        config.notifications.new_mail = false;
        config.save(&paths.config_file()).unwrap();
        pim.reload_config().await.unwrap();
        let mut changed = pim.receive_mail_changed().await.unwrap();
        let quiet = unique("quiet");
        append(&quiet).await;
        within("MailChanged", 15, async {
            loop {
                changed.next().await.unwrap();
                let all = reader.messages_in_folder(inbox.id).unwrap();
                if all.iter().any(|m| m.subject == quiet) {
                    return;
                }
            }
        })
        .await;
        Timer::after(Duration::from_millis(500)).await;
        assert!(asked.is_empty(), "{:?}", asked.try_recv());

        other.logout().await.unwrap();
        assert!(pim.remove_account(id).await.unwrap());
        instance.shutdown().await;
    });
}

#[test]
fn deletes_all_data_and_exits() {
    let bus = Bus::start();
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths::with_root(tmp.path());
    let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
    let account = store
        .add_account(AccountKind::Local, "enron", "enron@local")
        .unwrap();
    drop(store);
    std::fs::create_dir_all(paths.config_dir()).unwrap();
    std::fs::write(paths.config_file(), "[mail]\n").unwrap();
    let secrets = Secrets::memory();
    let Secrets::Memory(saved) = &secrets else {
        unreachable!()
    };
    let saved = saved.clone();
    saved.lock().unwrap().insert(account.id, "pw".into());
    // A password left from an account removed earlier.
    saved
        .lock()
        .unwrap()
        .insert(katna_core::AccountId(99), "old".into());
    smol::block_on(async {
        let instance = start(&bus, &paths, secrets).await.unwrap();
        let (stop, stopped) = async_channel::bounded::<()>(1);
        let served = smol::spawn(instance.serve(async move {
            let _ = stopped.recv().await;
        }));
        let pim = PimProxy::new(&bus.connect().await).await.unwrap();
        within("deleting", 20, pim.delete_all_data()).await.unwrap();
        assert!(!paths.data_dir().exists());
        assert!(!paths.cache_dir().exists());
        assert!(!paths.config_file().exists());
        assert!(saved.lock().unwrap().is_empty());
        let ended = within("exiting", 10, served).await;
        assert_eq!(ended, katna_daemon::Ended::Deleted);
        drop(stop);

        // A new daemon starts with nothing stored.
        let instance = start(&bus, &paths, Secrets::memory()).await.unwrap();
        assert!(pim.accounts().await.unwrap().is_empty());
        instance.shutdown().await;
    });
}

/// A StatusNotifierItem tooltip: icon name, pixmaps, title, text.
type Tooltip = (String, Vec<(i32, i32, Vec<u8>)>, String, String);

/// A dbusmenu layout: ID, properties, children (each a variant).
type MenuLayout = (
    i32,
    std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
    Vec<zbus::zvariant::OwnedValue>,
);

/// Plasma's `org.kde.StatusNotifierWatcher`: records who registers.
struct FakeWatcher {
    registered: async_channel::Sender<String>,
}

#[zbus::interface(name = "org.kde.StatusNotifierWatcher")]
impl FakeWatcher {
    fn register_status_notifier_item(&self, service: String) {
        let _ = self.registered.try_send(service);
    }
}

/// The unread count reaches the taskbar icon and the tray, whose menu
/// quits the daemon.
#[test]
fn shows_the_unread_count_on_the_taskbar_and_in_the_tray() {
    use zbus::zvariant::{OwnedValue, Value};

    let bus = Bus::start();
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths::with_root(tmp.path());
    let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
    let account = store
        .add_account(AccountKind::Local, "local", "me@local")
        .unwrap()
        .id;
    let mut batch = store.mail_batch().unwrap();
    let inbox = batch
        .upsert_folder(account, "INBOX", Some(katna_store::FolderRole::Inbox))
        .unwrap();
    for (subject, flags) in [
        ("one", MessageFlags::empty()),
        ("two", MessageFlags::empty()),
        ("three", MessageFlags::SEEN),
    ] {
        let raw = format!("Subject: {subject}\r\n\r\nHello.\r\n");
        batch
            .add_message(
                account,
                inbox,
                &NewMessage {
                    raw: raw.as_bytes(),
                    message_id_hdr: None,
                    subject: Some(subject),
                    date: None,
                    flags,
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
    }
    batch.commit().unwrap();
    drop(store);

    smol::block_on(async {
        let panel = bus.connect().await;
        let (registered_tx, registered) = async_channel::unbounded();
        panel
            .object_server()
            .at(
                "/StatusNotifierWatcher",
                FakeWatcher {
                    registered: registered_tx,
                },
            )
            .await
            .unwrap();
        panel
            .request_name("org.kde.StatusNotifierWatcher")
            .await
            .unwrap();
        let rule = zbus::MatchRule::builder()
            .msg_type(zbus::message::Type::Signal)
            .interface("com.canonical.Unity.LauncherEntry")
            .unwrap()
            .member("Update")
            .unwrap()
            .build();
        let mut updates = zbus::MessageStream::for_match_rule(rule, &panel, None)
            .await
            .unwrap();

        let instance = start(&bus, &paths, Secrets::memory()).await.unwrap();

        let update = within("taskbar count", 10, updates.next())
            .await
            .unwrap()
            .unwrap();
        let (uri, props): (String, std::collections::HashMap<String, OwnedValue>) =
            update.body().deserialize().unwrap();
        assert_eq!(uri, "application://in.invenia.katna.Mail.desktop");
        assert_eq!(props["count"], OwnedValue::from(2i64));
        assert_eq!(props["count-visible"], OwnedValue::from(true));

        let item = within("tray registration", 10, registered.recv())
            .await
            .unwrap();
        assert!(item.starts_with("org.kde.StatusNotifierItem-"), "{item}");
        let property = async |name: &str| -> OwnedValue {
            let reply = panel
                .call_method(
                    Some(item.as_str()),
                    "/StatusNotifierItem",
                    Some("org.freedesktop.DBus.Properties"),
                    "Get",
                    &("org.kde.StatusNotifierItem", name),
                )
                .await
                .unwrap();
            reply.body().deserialize().unwrap()
        };
        let text = |value: OwnedValue| String::try_from(value).unwrap();
        assert_eq!(text(property("Id").await), "in.invenia.katna.Mail");
        // The count is drawn into the pixmaps, so there is no icon name.
        assert_eq!(text(property("IconName").await), "");
        let tooltip: Tooltip = property("ToolTip").await.try_into().unwrap();
        assert_eq!(tooltip.3, "2 unread messages");
        let menu: zbus::zvariant::OwnedObjectPath = property("Menu").await.try_into().unwrap();

        let layout = panel
            .call_method(
                Some(item.as_str()),
                menu.as_str(),
                Some("com.canonical.dbusmenu"),
                "GetLayout",
                &(0i32, -1i32, vec!["label"]),
            )
            .await
            .unwrap();
        let (_, (_, _, children)): (u32, MenuLayout) = layout.body().deserialize().unwrap();
        let mut labels = Vec::new();
        let mut quit = None;
        for child in children {
            let structure: zbus::zvariant::Structure = Value::from(child).downcast().unwrap();
            let fields = structure.fields();
            let id: i32 = fields[0].try_clone().unwrap().downcast().unwrap();
            let props: std::collections::HashMap<String, OwnedValue> =
                fields[1].try_clone().unwrap().downcast().unwrap();
            if let Some(label) = props.get("label") {
                let label: String = label.try_clone().unwrap().try_into().unwrap();
                if label == "_Quit" {
                    quit = Some(id);
                }
                labels.push(label);
            }
        }
        assert_eq!(
            labels,
            ["Open _Inbox", "_New Message", "_Preferences", "_Quit"]
        );

        // Quit asks the (absent) app to close, then stops the daemon.
        panel
            .call_method(
                Some(item.as_str()),
                menu.as_str(),
                Some("com.canonical.dbusmenu"),
                "Event",
                &(quit.unwrap(), "clicked", Value::from(0i32), 0u32),
            )
            .await
            .unwrap();
        within("quit", 10, instance.quit_requested()).await;
        instance.shutdown().await;
    });
}

/// New folders (Gmail's labels) on the server, at the top and nested, with
/// names outside ASCII; taken names and separators in names are refused.
#[test]
#[ignore = "needs the dev servers: docker compose -f dev/compose.yaml up -d"]
fn creates_folders_on_dev_servers() {
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
        smol::block_on(async {
            let instance = start(&bus, &paths, Secrets::memory()).await.unwrap();
            let client = bus.connect().await;
            let pim = PimProxy::new(&client).await.unwrap();
            let account = imap("127.0.0.1", imap_port, security);
            let id = pim.add_imap_account(&account, "katna-dev").await.unwrap();
            wait_until_online(&pim, id).await;
            let reader = Store::open(&paths, Mode::ReadOnly).unwrap();
            let folders = || reader.folders(katna_core::AccountId(id)).unwrap();

            let top = unique("Projects");
            let parent = pim.create_folder(id, &format!(" {top} "), 0).await.unwrap();
            let stored = folders();
            let parent_path = &stored.iter().find(|f| f.id.0 == parent).unwrap().path;
            assert_eq!(parent_path, &top, "{name}: trimmed");

            let child = pim
                .create_folder(id, "Rechnungen für Café", parent)
                .await
                .unwrap();
            let stored = folders();
            let child_path = stored
                .iter()
                .find(|f| f.id.0 == child)
                .unwrap()
                .path
                .clone();
            assert!(child_path.starts_with(&top), "{name}: {child_path}");
            assert!(
                child_path.ends_with("Rechnungen für Café"),
                "{name}: {child_path}"
            );
            let separator = &child_path[top.len()..child_path.len() - "Rechnungen für Café".len()];
            assert_eq!(separator.chars().count(), 1, "{name}: {child_path}");

            // The next sync finds both on the server, under the same names.
            pim.sync_now(id).await.unwrap();
            Timer::after(Duration::from_secs(2)).await;
            wait_until_online(&pim, id).await;
            let after = folders();
            for folder in [parent, child] {
                assert!(after.iter().any(|f| f.id.0 == folder), "{name}: {after:?}");
            }
            let other = ImapBackend::connect(
                &Endpoint::new(
                    "127.0.0.1",
                    imap_port,
                    if security == "tls" {
                        Security::Tls
                    } else {
                        Security::StartTls
                    },
                ),
                &Credentials::new("alice@katna.test", "katna-dev"),
                Tls::insecure_for_local_tests(),
            )
            .await
            .unwrap();
            let mut other = other;
            let listed = other.list_folders().await.unwrap();
            assert!(listed.iter().any(|f| f.name == child_path), "{name}");
            other.logout().await.unwrap();

            for (bad, parent) in [
                (top.to_uppercase(), 0),
                (format!("a{separator}b"), 0),
                ("  ".to_owned(), 0),
                ("fine".to_owned(), 999_999),
            ] {
                let err = pim.create_folder(id, &bad, parent).await.unwrap_err();
                let expected = if parent == 0 {
                    "org.freedesktop.DBus.Error.InvalidArgs"
                } else {
                    "org.freedesktop.DBus.Error.UnknownObject"
                };
                assert_eq!(error_name(&err), expected, "{name}: {bad:?}: {err}");
            }

            assert!(pim.remove_account(id).await.unwrap());
            instance.shutdown().await;
        });
    }
}

/// Mail older than the offline window is downloaded when opened, at once
/// and several at a time, on a connection apart from the sync's.
#[test]
#[ignore = "needs the dev servers: docker compose -f dev/compose.yaml up -d"]
fn downloads_old_mail_when_opened() {
    let imap_port = port("KATNA_STALWART_IMAPS_PORT", 10993);
    let bus = Bus::start();
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths::with_root(tmp.path());
    smol::block_on(async {
        let mut other = ImapBackend::connect(
            &Endpoint::new("127.0.0.1", imap_port, Security::Tls),
            &Credentials::new("alice@katna.test", "katna-dev"),
            Tls::insecure_for_local_tests(),
        )
        .await
        .unwrap();
        let subjects: Vec<String> = (0..3).map(|i| unique(&format!("old{i}"))).collect();
        for subject in &subjects {
            let message = format!(
                "From: <bob@katna.test>\r\nTo: <alice@katna.test>\r\n\
                 Date: Tue, 1 May 2001 10:00:00 +0000\r\nSubject: {subject}\r\n\
                 Message-ID: <{subject}@katna.test>\r\n\r\nFrom long ago.\r\n"
            );
            other.append("INBOX", message.into_bytes()).await.unwrap();
        }
        other.logout().await.unwrap();

        let instance = start(&bus, &paths, Secrets::memory()).await.unwrap();
        let client = bus.connect().await;
        let pim = PimProxy::new(&client).await.unwrap();
        let account = imap("127.0.0.1", imap_port, "tls");
        let id = pim.add_imap_account(&account, "katna-dev").await.unwrap();
        wait_until_online(&pim, id).await;
        let reader = Store::open(&paths, Mode::ReadOnly).unwrap();
        let inbox = reader
            .folders(katna_core::AccountId(id))
            .unwrap()
            .into_iter()
            .find(|f| f.path == "INBOX")
            .unwrap();
        let old: Vec<_> = reader
            .messages_in_folder(inbox.id)
            .unwrap()
            .into_iter()
            .filter(|m| subjects.contains(&m.subject))
            .collect();
        assert_eq!(old.len(), 3);
        assert!(
            old.iter().all(|m| m.blob_hash.is_none()),
            "outside the window"
        );

        let started = std::time::Instant::now();
        // Opened one after another, faster than they download.
        use futures_lite::future::zip;
        let [a, b, c] = &old[..] else { unreachable!() };
        let (a, (b, c)) = zip(
            pim.fetch_body(a.id.0),
            zip(pim.fetch_body(b.id.0), pim.fetch_body(c.id.0)),
        )
        .await;
        a.unwrap();
        b.unwrap();
        c.unwrap();
        for message in &old {
            let stored = &reader.messages_by_id(&[message.id]).unwrap()[0];
            assert!(stored.blob_hash.is_some(), "{}", stored.subject);
        }
        assert!(
            started.elapsed() < Duration::from_secs(10),
            "{:?}",
            started.elapsed()
        );

        assert!(pim.remove_account(id).await.unwrap());
        instance.shutdown().await;
    });
}

/// Reset cache deletes downloaded mail and sender pictures, keeps the
/// account, and downloads the mail again.
#[test]
#[ignore = "needs the dev servers: docker compose -f dev/compose.yaml up -d"]
fn resets_the_cache_on_dev_servers() {
    let imap_port = port("KATNA_STALWART_IMAPS_PORT", 10993);
    let bus = Bus::start();
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths::with_root(tmp.path());
    std::fs::create_dir_all(paths.config_dir()).unwrap();
    std::fs::write(paths.config_file(), "[sync]\noffline_days = 0\n").unwrap();
    let pictures = paths.cache_dir().join("pictures");
    std::fs::create_dir_all(&pictures).unwrap();
    std::fs::write(pictures.join("katna.test.pic"), "picture").unwrap();
    smol::block_on(async {
        let mut other = ImapBackend::connect(
            &Endpoint::new("127.0.0.1", imap_port, Security::Tls),
            &Credentials::new("alice@katna.test", "katna-dev"),
            Tls::insecure_for_local_tests(),
        )
        .await
        .unwrap();
        let subjects: Vec<String> = (0..2).map(|i| unique(&format!("cache{i}"))).collect();
        for subject in &subjects {
            let message = format!(
                "From: <bob@katna.test>\r\nTo: <alice@katna.test>\r\nSubject: {subject}\r\n\
                 Message-ID: <{subject}@katna.test>\r\n\r\nKept on the server.\r\n"
            );
            other.append("INBOX", message.into_bytes()).await.unwrap();
        }
        other.logout().await.unwrap();

        let instance = start(&bus, &paths, Secrets::memory()).await.unwrap();
        let client = bus.connect().await;
        let pim = PimProxy::new(&client).await.unwrap();
        let account = imap("127.0.0.1", imap_port, "tls");
        let id = pim.add_imap_account(&account, "katna-dev").await.unwrap();
        wait_until_online(&pim, id).await;
        let reader = Store::open(&paths, Mode::ReadOnly).unwrap();
        let inbox = reader
            .folders(katna_core::AccountId(id))
            .unwrap()
            .into_iter()
            .find(|f| f.path == "INBOX")
            .unwrap();
        let ours = || -> Vec<_> {
            reader
                .messages_in_folder(inbox.id)
                .unwrap()
                .into_iter()
                .filter(|m| subjects.contains(&m.subject))
                .collect()
        };
        let downloaded = || ours().iter().all(|m| m.blob_hash.is_some());
        within("bodies downloaded", 30, async {
            while !downloaded() {
                Timer::after(Duration::from_millis(50)).await;
            }
        })
        .await;
        let ids: Vec<_> = ours().iter().map(|m| m.id).collect();
        assert_eq!(ids.len(), 2);

        let (messages, bytes) = pim.reset_cache().await.unwrap();
        assert!(messages >= 2, "{messages}");
        assert!(bytes > 0);
        assert!(!pictures.exists());
        // The same messages, now without their bodies at first.
        let after: Vec<_> = ours().iter().map(|m| m.id).collect();
        assert_eq!(after, ids);
        assert_eq!(pim.accounts().await.unwrap().len(), 1);
        within("bodies downloaded again", 30, async {
            while !downloaded() {
                Timer::after(Duration::from_millis(50)).await;
            }
        })
        .await;

        assert!(pim.remove_account(id).await.unwrap());
        instance.shutdown().await;
    });
}

#[test]
fn labels_contacts_on_this_computer() {
    let bus = Bus::start();
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths::with_root(tmp.path());
    smol::block_on(async {
        let instance = start(&bus, &paths, Secrets::memory()).await.unwrap();
        let pim = PimProxy::new(&bus.connect().await).await.unwrap();
        let card = katna_core::contact::Card {
            name: katna_core::contact::Name {
                given: "Asha".into(),
                ..Default::default()
            },
            emails: vec![katna_core::contact::Typed::new("asha@rao.in", "home")],
            ..Default::default()
        };
        let card = serde_json::to_string(&card).unwrap();
        let id = pim.save_contact(0, 0, &card).await.unwrap();
        let labels = |paths: &Paths| {
            let store = Store::open(paths, Mode::ReadOnly).unwrap();
            store.saved_contacts().unwrap()[0].labels.clone()
        };

        pim.set_contact_labels(id, &["Family".into(), " family ".into(), "Work".into()])
            .await
            .unwrap();
        assert_eq!(labels(&paths), ["Family", "Work"]);
        pim.set_contact_labels(id, &["Work".into()]).await.unwrap();
        assert_eq!(labels(&paths), ["Work"]);

        pim.rename_contact_label("work", "Office").await.unwrap();
        assert_eq!(labels(&paths), ["Office"]);
        pim.rename_contact_label("Office", "").await.unwrap();
        assert!(labels(&paths).is_empty());
        let store = Store::open(&paths, Mode::ReadOnly).unwrap();
        assert!(store.contact_labels().unwrap().is_empty());
        instance.shutdown().await;
    });
}
