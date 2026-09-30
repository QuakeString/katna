// SPDX-License-Identifier: GPL-3.0-or-later

//! The level-1 sync engine against an in-memory mail server.

mod common;

use common::{FakeServer, store as setup};
use katna_core::{AccountId, MailCategory};
use katna_store::{FolderRole as StoreRole, Store};
use katna_sync::{
    AttachmentPart,
    engine::{self, CHUNK, FolderReport},
};

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
        vec![report("INBOX", 3, 0, 0), report("Archive", 0, 0, 0)]
    );
    let folders = store.folders(account).unwrap();
    let inbox = folders.iter().find(|f| f.path == "INBOX").unwrap();
    assert_eq!(inbox.role, Some(StoreRole::Inbox));
    assert_eq!(inbox.uidvalidity, Some(1));
    assert_eq!(inbox.highestmodseq, Some(3));
    assert_eq!(store.folder_uids(inbox.id).unwrap(), vec![1, 2, 3]);

    // Nothing changed: no flag fetch, no header fetch past UIDNEXT.
    let reports = sync(&server, &mut store, account);
    assert_eq!(reports[0], report("INBOX", 0, 0, 0));
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
    assert_eq!(reports[0], report("INBOX", 1, 1, 1));
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
fn qresync_reports_expunges_without_a_uid_list() {
    let (_tmp, mut store, account) = setup();
    let server = FakeServer::default();
    server.state().qresync = true;
    server.create("INBOX", 1);
    for i in 0..5 {
        server.deliver("INBOX", &format!("hello-{i}"));
    }
    sync(&server, &mut store, account);
    let inbox = store.folders(account).unwrap()[0].id;

    // Two expunged, one new, one read.
    server.expunge("INBOX", 2);
    server.expunge("INBOX", 4);
    server.deliver("INBOX", "hello-5");
    server.set_seen("INBOX", 1);
    let reports = sync(&server, &mut store, account);
    assert_eq!(reports[0], report("INBOX", 1, 1, 2));
    assert_eq!(store.folder_uids(inbox).unwrap(), vec![1, 3, 5, 6]);
    assert!(
        !server.log().iter().any(|c| c == "UIDS"),
        "the UID list is not needed: {:?}",
        server.log()
    );

    // A server that forgot an expunge: the counts give it away.
    server.expunge("INBOX", 3);
    server
        .state()
        .folders
        .get_mut("INBOX")
        .unwrap()
        .vanished
        .clear();
    server.set_seen("INBOX", 5);
    let reports = sync(&server, &mut store, account);
    assert_eq!(reports[0], report("INBOX", 0, 1, 1));
    assert!(
        server.log().iter().any(|c| c == "UIDS"),
        "{:?}",
        server.log()
    );
    assert_eq!(store.folder_uids(inbox).unwrap(), vec![1, 5, 6]);
}

#[test]
fn big_folder_is_fetched_newest_first_in_chunks() {
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
            &format!("HEADERS {}:Some({total})", total - CHUNK + 1),
            &format!("HEADERS {}:Some({})", total - 2 * CHUNK + 1, total - CHUNK),
            &format!("HEADERS 1:Some({})", total - 2 * CHUNK),
        ]
    );
}

#[test]
fn a_cut_off_first_sync_fills_in_the_older_mail_next_time() {
    let (_tmp, mut store, account) = setup();
    let server = FakeServer::default();
    server.create("INBOX", 1);
    let total = CHUNK * 2 + 10;
    for i in 0..total {
        server.deliver("INBOX", &format!("m{i}"));
    }
    // The newest chunk is stored, then the connection drops.
    server.state().header_fetches_left = Some(1);
    let mut conn = server.connection();
    assert!(smol::block_on(engine::sync_account(&mut conn, &mut store, account)).is_err());
    let inbox = store.folders(account).unwrap()[0].id;
    let stored = store.folder_uids(inbox).unwrap();
    assert_eq!(stored.first(), Some(&(total - CHUNK + 1)));
    assert_eq!(stored.last(), Some(&total));

    // Mail that arrived meanwhile comes first, then the older mail.
    server.deliver("INBOX", "late");
    let reports = sync(&server, &mut store, account);
    assert_eq!(reports[0].added, total as usize - CHUNK as usize + 1);
    let log = server.log();
    let fetches: Vec<_> = log.iter().filter(|c| c.starts_with("HEADERS")).collect();
    assert_eq!(
        fetches,
        [
            &format!("HEADERS {}:Some({})", total + 1, total + 1),
            &format!("HEADERS {}:Some({})", total - 2 * CHUNK + 1, total - CHUNK),
            &format!("HEADERS 1:Some({})", total - 2 * CHUNK),
        ]
    );
    assert_eq!(
        store.folder_uids(inbox).unwrap(),
        (1..=total + 1).collect::<Vec<_>>()
    );

    // Nothing is left to fetch.
    let reports = sync(&server, &mut store, account);
    assert_eq!(reports[0].added, 0);
    assert!(!server.log().iter().any(|c| c.starts_with("HEADERS")));
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
fn the_providers_authentication_verdict_is_kept() {
    let (_tmp, mut store, account) = setup();
    let server = FakeServer::default();
    server.create("INBOX", 1);
    // A forged sender with the provider's failing verdict on top and the
    // sender's own passing one below it.
    server.deliver_header(
        "INBOX",
        &format!(
            "Authentication-Results: mx.provider.test; dmarc=fail header.from=bank.example\r\n\
             Authentication-Results: mx.provider.test; dmarc=pass header.from=bank.example\r\n\
             {}",
            header("a", "").replace("bob@example.org", "ceo@bank.example")
        ),
        "",
    );
    server.deliver_header(
        "INBOX",
        &format!(
            "Authentication-Results: mx.provider.test;\r\n dkim=pass header.d=example.org;\r\n \
             dmarc=pass (p=none) header.from=example.org\r\n{}",
            header("b", "")
        ),
        "",
    );
    sync(&server, &mut store, account);
    assert!(store.sender_domain_authenticated("example.org").unwrap());
    assert!(!store.sender_domain_authenticated("bank.example").unwrap());
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

    // The same mail in All Mail lands in the same tabs as in the inbox.
    server.create("[Gmail]/All Mail", 1);
    for (name, label) in [("a", None), ("c", None), ("d", Some("social"))] {
        let uid = server.deliver_header(
            "[Gmail]/All Mail",
            &header(
                name,
                if name == "c" {
                    "List-Unsubscribe: <https://x.example/u>\r\nX-Campaign: 1\r\n"
                } else {
                    ""
                },
            ),
            "",
        );
        server.set_gmail_labels("[Gmail]/All Mail", uid, Some(10), label);
    }
    sync(&server, &mut store, account);
    let all = store
        .folders(account)
        .unwrap()
        .into_iter()
        .find(|f| f.path == "[Gmail]/All Mail")
        .unwrap()
        .id;
    let categories: Vec<_> = threads_and_categories(&store, all)
        .iter()
        .map(|(_, c)| *c)
        .collect();
    assert_eq!(
        categories,
        [
            Some(MailCategory::Primary),
            Some(MailCategory::Primary),
            Some(MailCategory::Social)
        ]
    );
}

#[test]
fn gmail_labels_are_one_message_in_several_folders() {
    let (_tmp, mut store, account) = setup();
    let server = FakeServer::default();
    server.set_gmail(true);
    server.create("INBOX", 1);
    server.create("[Gmail]/All Mail", 1);
    let a = server.deliver("INBOX", "a");
    let b = server.deliver("INBOX", "b");
    server.label("INBOX", a, 101, "[Gmail]/All Mail");
    server.label("INBOX", b, 102, "[Gmail]/All Mail");
    // Archived: only in All Mail.
    let c = server.deliver("[Gmail]/All Mail", "c");
    server.label("[Gmail]/All Mail", c, 103, "[Gmail]/All Mail");
    server.expunge("[Gmail]/All Mail", c);

    let reports = sync(&server, &mut store, account);
    let added: Vec<_> = reports.iter().map(|r| (r.path.as_str(), r.added)).collect();
    assert_eq!(added, [("INBOX", 2), ("[Gmail]/All Mail", 3)]);
    let folders = store.folders(account).unwrap();
    let id_of = |path: &str| folders.iter().find(|f| f.path == path).unwrap().id;
    let (inbox, all) = (id_of("INBOX"), id_of("[Gmail]/All Mail"));
    let ids = |store: &Store, folder| {
        let mut ids: Vec<_> = store
            .messages_in_folder(folder)
            .unwrap()
            .iter()
            .map(|m| m.id)
            .collect();
        ids.sort();
        ids
    };
    let in_inbox = ids(&store, inbox);
    let in_all = ids(&store, all);
    assert_eq!(in_inbox.len(), 2);
    assert_eq!(in_all.len(), 3);
    assert!(
        in_inbox.iter().all(|id| in_all.contains(id)),
        "stored once, in both folders"
    );

    // Read in one folder is read in the other: it is one message.
    server.set_seen("INBOX", a);
    sync(&server, &mut store, account);
    let message = store.messages_by_id(&in_inbox[..1]).unwrap().remove(0);
    assert!(message.flags.contains(katna_store::MessageFlags::SEEN));

    // Archived by another client: gone from the inbox, still in All Mail.
    server.expunge("INBOX", a);
    let reports = sync(&server, &mut store, account);
    assert_eq!(reports[0].removed, 1);
    assert_eq!(ids(&store, inbox).len(), 1);
    assert_eq!(ids(&store, all), in_all);

    // Archived here: the message only leaves the inbox, and the server is
    // asked to move it from there, not from All Mail.
    let b_id = ids(&store, inbox)[0];
    katna_sync::ops::archive_messages(&mut store, &[b_id]).unwrap();
    assert!(ids(&store, inbox).is_empty());
    assert_eq!(ids(&store, all), in_all);
    let due = store.due_ops(account, i64::MAX, 10).unwrap();
    assert_eq!(due.len(), 1);
    assert!(
        due[0].op_json.contains("\"from_path\":\"INBOX\""),
        "{}",
        due[0].op_json
    );
}

#[test]
fn gmail_copies_stored_before_v4_are_merged() {
    let (_tmp, mut store, account) = setup();
    let server = FakeServer::default();
    server.create("INBOX", 1);
    server.create("[Gmail]/All Mail", 1);
    let a = server.deliver("INBOX", "a");
    let b = server.deliver("INBOX", "b");
    let a_all = server.label("INBOX", a, 101, "[Gmail]/All Mail");
    server.label("INBOX", b, 102, "[Gmail]/All Mail");

    // Synced the way Katna did before schema v4: without Gmail's message
    // IDs, so every label is its own message.
    sync(&server, &mut store, account);
    let folders = store.folders(account).unwrap();
    let id_of = |path: &str| folders.iter().find(|f| f.path == path).unwrap().id;
    let (inbox, all) = (id_of("INBOX"), id_of("[Gmail]/All Mail"));
    let count = |store: &Store| store.messages_in_folder(all).unwrap().len();
    let everything = |store: &Store| {
        store
            .messages_after(katna_store::MessageId(0), 100)
            .unwrap()
    };
    assert_eq!(everything(&store).len(), 4);
    let mut batch = store.mail_batch().unwrap();
    for folder in &folders {
        batch
            .set_folder_state(folder.id, folder.uidvalidity, folder.highestmodseq, None)
            .unwrap();
    }
    batch.commit().unwrap();
    // Starred in All Mail only, as a failed Undo left it.
    let mut batch = store.mail_batch().unwrap();
    batch
        .set_remote_flags(all, a_all, katna_store::MessageFlags::FLAGGED, &[])
        .unwrap();
    batch.commit().unwrap();

    // This version asks for the IDs once and merges the copies.
    server.set_gmail(true);
    let reports = sync(&server, &mut store, account);
    let merged: usize = reports.iter().map(|r| r.backfilled).sum();
    assert!(merged >= 2, "{reports:?}");
    let messages = everything(&store);
    assert_eq!(messages.len(), 2, "one message per Gmail message");
    assert_eq!(count(&store), 2);
    assert_eq!(store.messages_in_folder(inbox).unwrap().len(), 2);
    let a_row = messages
        .iter()
        .find(|m| m.subject == "a")
        .expect("a is still there");
    assert_eq!(a_row.locations.len(), 2, "in the inbox and All Mail");
    assert!(
        a_row.flags.contains(katna_store::MessageFlags::FLAGGED),
        "the star set on one copy is kept"
    );

    // Only once: the next sync does not ask again.
    server.clear_log();
    sync(&server, &mut store, account);
    assert!(
        !server
            .log()
            .iter()
            .any(|line| line.starts_with("X-GM-MSGID")),
        "{:?}",
        server.log()
    );
}

#[test]
fn attachments_come_from_the_structure() {
    let (_tmp, mut store, account) = setup();
    let server = FakeServer::default();
    server.create("INBOX", 1);
    let mixed = "Content-Type: multipart/mixed; boundary=b\r\n";
    let pdf = server.deliver_header("INBOX", &header("pdf", mixed), "");
    let none = server.deliver_header("INBOX", &header("none", mixed), "");
    let guess = server.deliver_header("INBOX", &header("guess", mixed), "");
    let part = AttachmentPart {
        part: "2".into(),
        mime: "application/pdf".into(),
        filename: Some("rates.pdf".into()),
        size: 51_200,
    };
    server.set_attachments("INBOX", pdf, Some(vec![part]));
    // A structure without attachments beats the header's guess.
    server.set_attachments("INBOX", none, Some(Vec::new()));
    server.set_attachments("INBOX", guess, None);
    sync(&server, &mut store, account);

    let inbox = store.folders(account).unwrap()[0].id;
    let messages = store.messages_in_folder(inbox).unwrap();
    let got: Vec<_> = messages
        .iter()
        .map(|m| {
            let names: Vec<_> = store
                .attachments(m.id)
                .unwrap()
                .into_iter()
                .map(|a| (a.part, a.mime, a.filename, a.size))
                .collect();
            (m.has_attachments, names)
        })
        .collect();
    assert_eq!(
        got,
        [
            (
                true,
                vec![(
                    "2".into(),
                    "application/pdf".into(),
                    Some("rates.pdf".into()),
                    51_200
                )]
            ),
            (false, vec![]),
            (true, vec![]),
        ]
    );
}

#[test]
fn mail_stored_without_a_list_gets_its_structure_again() {
    let (_tmp, mut store, account) = setup();
    let server = FakeServer::default();
    server.set_gmail(true);
    server.create("INBOX", 1);
    server.create("Scratch", 1);
    let mixed = "Content-Type: multipart/mixed; boundary=b\r\n";
    let old = server.deliver_header("INBOX", &header("old", mixed), "");
    let labelled = server.deliver_header("INBOX", &header("labelled", mixed), "");
    // Gives it its Gmail ID; All Mail is not there yet.
    server.label("INBOX", labelled, 201, "Scratch");
    server.remove_folder("Scratch");
    // Stored as sync did before it read structures: the header's guess
    // and no list.
    sync(&server, &mut store, account);
    let inbox = store.folders(account).unwrap()[0].id;
    let unlisted = |store: &Store| -> Vec<bool> {
        store
            .messages_in_folder(inbox)
            .unwrap()
            .iter()
            .map(|m| m.has_attachments && store.attachments(m.id).unwrap().is_empty())
            .collect()
    };
    assert_eq!(unlisted(&store), [true, true]);

    let pdf = AttachmentPart {
        part: "2".into(),
        mime: "application/pdf".into(),
        filename: Some("rates.pdf".into()),
        size: 3000,
    };
    // The old one's structure can be read now. The labelled one's only in
    // All Mail, where it shows up under its label.
    server.set_attachments("INBOX", old, Some(vec![pdf.clone()]));
    server.set_attachments("INBOX", labelled, Some(vec![pdf]));
    server.create("[Gmail]/All Mail", 1);
    server.label("INBOX", labelled, 201, "[Gmail]/All Mail");
    server.set_attachments("INBOX", labelled, None);
    let reports = sync(&server, &mut store, account);
    assert_eq!(unlisted(&store), [false, false], "{reports:?}");
    let names: Vec<_> = store
        .messages_in_folder(inbox)
        .unwrap()
        .iter()
        .flat_map(|m| store.attachments(m.id).unwrap())
        .map(|a| (a.part, a.filename))
        .collect();
    assert_eq!(
        names,
        [
            ("2".into(), Some("rates.pdf".into())),
            ("2".into(), Some("rates.pdf".into()))
        ]
    );

    sync(&server, &mut store, account);
    assert!(
        !server.log().iter().any(|l| l.starts_with("HEADERS 1:")),
        "nothing is fetched again: {:?}",
        server.log()
    );
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
