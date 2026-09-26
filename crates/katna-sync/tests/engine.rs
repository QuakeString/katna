// SPDX-License-Identifier: GPL-3.0-or-later

//! The level-1 sync engine against an in-memory mail server.

mod common;

use common::{FakeServer, store as setup};
use katna_core::{AccountId, MailCategory};
use katna_store::{FolderRole as StoreRole, Store};
use katna_sync::engine::{self, CHUNK, FolderReport};

fn sync(server: &FakeServer, store: &mut Store, account: AccountId) -> Vec<FolderReport> {
    server.clear_log();
    let mut conn = server.connection();
    smol::block_on(engine::sync_account(&mut conn, store, account)).unwrap()
}

fn report(path: &str, added: usize, flags_changed: usize, removed: usize) -> FolderReport {
    FolderReport {
        path: path.into(),
        added,
        flags_changed,
        removed,
        reset: false,
        backfilled: 0,
    }
}

#[test]
fn first_sync_then_incremental_changes() {
    let (_tmp, mut store, account) = setup();
    let server = FakeServer::default();
    server.create("INBOX", 1);
    server.create("Archive", 1);
    for i in 0..3 {
        server.deliver("INBOX", &format!("hello-{i}"));
    }

    let reports = sync(&server, &mut store, account);
    assert_eq!(
        reports,
        vec![report("Archive", 0, 0, 0), report("INBOX", 3, 0, 0)]
    );
    let folders = store.folders(account).unwrap();
    let inbox = folders.iter().find(|f| f.path == "INBOX").unwrap();
    assert_eq!(inbox.role, Some(StoreRole::Inbox));
    assert_eq!(inbox.uidvalidity, Some(1));
    assert_eq!(inbox.highestmodseq, Some(3));
    assert_eq!(store.folder_uids(inbox.id).unwrap(), vec![1, 2, 3]);

    // Nothing changed: no flag fetch, no header fetch past UIDNEXT.
    let reports = sync(&server, &mut store, account);
    assert_eq!(reports[1], report("INBOX", 0, 0, 0));
    assert!(
        !server
            .log()
            .iter()
            .any(|c| c.starts_with("FLAGS") || c.starts_with("HEADERS")),
        "{:?}",
        server.log()
    );

    // One new, one read, one expunged.
    server.deliver("INBOX", "hello-3");
    server.set_seen("INBOX", 1);
    server.expunge("INBOX", 2);
    let reports = sync(&server, &mut store, account);
    assert_eq!(reports[1], report("INBOX", 1, 1, 1));
    assert!(
        server.log().contains(&"FLAGS 1:3 since Some(3)".to_owned()),
        "{:?}",
        server.log()
    );
    assert_eq!(store.folder_uids(inbox.id).unwrap(), vec![1, 3, 4]);
    let message = store
        .messages_in_folder(inbox.id)
        .unwrap()
        .into_iter()
        .find(|m| m.subject == "hello-0")
        .unwrap();
    assert!(message.flags.contains(katna_store::MessageFlags::SEEN));
    let from = message.first(katna_store::ParticipantRole::From).unwrap();
    assert_eq!(from.email_norm, "bob@example.org");
    assert_eq!(from.display_name.as_deref(), Some("Bob"));
}

#[test]
fn big_folder_is_fetched_in_chunks() {
    let (_tmp, mut store, account) = setup();
    let server = FakeServer::default();
    server.create("INBOX", 1);
    let total = CHUNK * 2 + 10;
    for i in 0..total {
        server.deliver("INBOX", &format!("m{i}"));
    }
    let reports = sync(&server, &mut store, account);
    assert_eq!(reports[0].added, total as usize);
    let log = server.log();
    let fetches: Vec<_> = log.iter().filter(|c| c.starts_with("HEADERS")).collect();
    assert_eq!(
        fetches,
        [
            &format!("HEADERS 1:Some({CHUNK})"),
            &format!("HEADERS {}:Some({})", CHUNK + 1, CHUNK * 2),
            &format!("HEADERS {}:Some({total})", CHUNK * 2 + 1),
        ]
    );
}

#[test]
fn changed_uidvalidity_downloads_the_folder_again() {
    let (_tmp, mut store, account) = setup();
    let server = FakeServer::default();
    server.create("INBOX", 1);
    server.deliver("INBOX", "old-1");
    server.deliver("INBOX", "old-2");
    sync(&server, &mut store, account);

    // The server rebuilt the folder: new UIDVALIDITY, UIDs start again.
    server.create("INBOX", 2);
    server.deliver("INBOX", "new-1");
    let reports = sync(&server, &mut store, account);
    assert_eq!(
        reports[0],
        FolderReport {
            reset: true,
            ..report("INBOX", 1, 0, 0)
        }
    );
    let inbox = &store.folders(account).unwrap()[0];
    assert_eq!(inbox.uidvalidity, Some(2));
    let subjects: Vec<_> = store
        .messages_in_folder(inbox.id)
        .unwrap()
        .into_iter()
        .map(|m| m.subject)
        .collect();
    assert_eq!(subjects, ["new-1"]);
}

fn header(id: &str, extra: &str) -> String {
    format!(
        "From: Bob <bob@example.org>\r\nTo: alice@example.org\r\nSubject: Plan {id}\r\n\
         Date: Sat, 26 Sep 2026 10:00:00 +0000\r\nMessage-ID: <{id}@example.org>\r\n{extra}\r\n"
    )
}

/// `(thread, category)` of every message in the folder, in UID order.
fn threads_and_categories(
    store: &Store,
    folder: katna_store::FolderId,
) -> Vec<(Option<katna_store::ThreadId>, Option<MailCategory>)> {
    store
        .messages_in_folder(folder)
        .unwrap()
        .into_iter()
        .map(|m| (m.thread_id, m.category))
        .collect()
}

#[test]
fn new_messages_are_threaded_and_classified() {
    let (_tmp, mut store, account) = setup();
    let server = FakeServer::default();
    server.create("INBOX", 1);
    server.deliver_header("INBOX", &header("a", ""), "");
    server.deliver_header(
        "INBOX",
        &header("b", "In-Reply-To: <a@example.org>\r\n"),
        "",
    );
    server.deliver_header(
        "INBOX",
        &header(
            "c",
            "List-Unsubscribe: <https://x.example/u>\r\nX-Campaign: 1\r\n",
        ),
        "",
    );
    sync(&server, &mut store, account);
    let inbox = store.folders(account).unwrap()[0].id;
    let got = threads_and_categories(&store, inbox);
    assert_eq!(got[0].0, got[1].0);
    assert_ne!(got[0].0, got[2].0);
    let categories: Vec<_> = got.iter().map(|(_, c)| *c).collect();
    assert_eq!(
        categories,
        [
            Some(MailCategory::Primary),
            Some(MailCategory::Primary),
            Some(MailCategory::Promotions)
        ]
    );
    assert!(
        server.log().iter().any(|c| c.starts_with("GMAIL 1:")),
        "asks, and learns it is not Gmail"
    );
}

#[test]
fn gmail_thread_ids_and_categories_win() {
    let (_tmp, mut store, account) = setup();
    let server = FakeServer::default();
    server.set_gmail(true);
    server.create("INBOX", 1);
    let a = server.deliver_header("INBOX", &header("a", ""), "");
    // Cites a, but Gmail put it in its own thread.
    let b = server.deliver_header(
        "INBOX",
        &header("b", "In-Reply-To: <a@example.org>\r\n"),
        "",
    );
    // Looks like marketing to our classifier; Gmail says Primary.
    let c = server.deliver_header(
        "INBOX",
        &header(
            "c",
            "List-Unsubscribe: <https://x.example/u>\r\nX-Campaign: 1\r\n",
        ),
        "",
    );
    let d = server.deliver_header("INBOX", &header("d", ""), "");
    server.set_gmail_labels("INBOX", a, Some(10), None);
    server.set_gmail_labels("INBOX", b, Some(20), Some("updates"));
    server.set_gmail_labels("INBOX", c, Some(30), None);
    server.set_gmail_labels("INBOX", d, Some(10), Some("social"));

    let reports = sync(&server, &mut store, account);
    assert_eq!(reports[0].added, 4);
    assert_eq!(reports[0].backfilled, 3, "b, c and d changed tab");
    let inbox = store.folders(account).unwrap()[0].id;
    let got = threads_and_categories(&store, inbox);
    assert_ne!(got[0].0, got[1].0, "X-GM-THRID beats In-Reply-To");
    assert_eq!(got[0].0, got[3].0, "same X-GM-THRID");
    let categories: Vec<_> = got.iter().map(|(_, c)| *c).collect();
    assert_eq!(
        categories,
        [
            Some(MailCategory::Primary),
            Some(MailCategory::Updates),
            Some(MailCategory::Primary),
            Some(MailCategory::Social)
        ]
    );
    let searches: Vec<_> = server
        .log()
        .into_iter()
        .filter(|c| c.starts_with("GMAIL"))
        .collect();
    assert_eq!(
        searches,
        [
            "GMAIL 1: category:social",
            "GMAIL 1: category:promotions",
            "GMAIL 1: category:updates",
            "GMAIL 1: category:forums",
        ]
    );

    // Afterwards only new messages are asked about, and only when there
    // are some.
    sync(&server, &mut store, account);
    assert!(!server.log().iter().any(|c| c.starts_with("GMAIL")));
    let e = server.deliver_header("INBOX", &header("e", ""), "");
    server.set_gmail_labels("INBOX", e, Some(50), Some("promotions"));
    sync(&server, &mut store, account);
    assert!(
        server
            .log()
            .contains(&"GMAIL 5: category:promotions".to_owned()),
        "{:?}",
        server.log()
    );
    let got = threads_and_categories(&store, inbox);
    assert_eq!(got[4].1, Some(MailCategory::Promotions));
}

#[test]
fn messages_from_before_threading_get_their_headers_again() {
    let (_tmp, mut store, account) = setup();
    let server = FakeServer::default();
    server.create("INBOX", 1);
    server.deliver_header("INBOX", &header("a", ""), "");
    server.deliver_header("INBOX", &header("b", "References: <a@example.org>\r\n"), "");
    sync(&server, &mut store, account);
    let inbox = store.folders(account).unwrap()[0].id;
    let before = threads_and_categories(&store, inbox);

    store.reset_threads().unwrap();
    let reports = sync(&server, &mut store, account);
    assert_eq!(reports[0].backfilled, 2);
    assert!(
        server.log().contains(&"HEADERS 1:Some(2)".to_owned()),
        "{:?}",
        server.log()
    );
    let after = threads_and_categories(&store, inbox);
    assert_eq!(after[0].0, after[1].0);
    assert_eq!(
        after.iter().map(|(_, c)| *c).collect::<Vec<_>>(),
        before.iter().map(|(_, c)| *c).collect::<Vec<_>>()
    );
    let reports = sync(&server, &mut store, account);
    assert_eq!(reports[0].backfilled, 0);
    assert!(!server.log().iter().any(|c| c.starts_with("HEADERS")));
}

#[test]
fn deleted_folder_is_removed() {
    let (_tmp, mut store, account) = setup();
    let server = FakeServer::default();
    server.create("INBOX", 1);
    server.create("Old", 1);
    server.deliver("Old", "bye");
    sync(&server, &mut store, account);
    assert_eq!(store.folders(account).unwrap().len(), 2);

    server.remove_folder("Old");
    sync(&server, &mut store, account);
    let folders = store.folders(account).unwrap();
    assert_eq!(folders.len(), 1);
    assert_eq!(folders[0].path, "INBOX");
}
