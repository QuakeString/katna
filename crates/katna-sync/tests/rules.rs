// SPDX-License-Identifier: GPL-3.0-or-later

//! Mail rules on new incoming mail, against an in-memory mail server: what
//! the daemon does after each sync (`katna_sync::rules::Watch`).

mod common;

use common::{FakeServer, store as setup};
use katna_core::AccountId;
use katna_store::rules::{Action, Comparator, Condition, Field, Rule};
use katna_store::{FolderId, MessageFlags, MessageId, Store, StoredMessage};
use katna_sync::{
    bodies::{self, OfflineWindow},
    engine, ops,
    rules::{self, Context, HOLD, Watch},
};

/// The `Date` of the fake server's mail.
const NOW: i64 = 1_790_416_800;

fn live(now: i64) -> Context {
    Context {
        now,
        live: true,
        can_send: true,
    }
}

fn sync(server: &FakeServer, store: &mut Store, account: AccountId) {
    let mut conn = server.connection();
    smol::block_on(engine::sync_account(&mut conn, store, account)).unwrap();
}

fn replay(server: &FakeServer, store: &mut Store, account: AccountId) {
    let mut conn = server.connection();
    smol::block_on(ops::replay(&mut conn, store, account, NOW)).unwrap();
}

fn download(server: &FakeServer, store: &mut Store, account: AccountId) {
    let inbox = folder(store, account, "INBOX");
    let mut conn = server.connection();
    smol::block_on(bodies::download_bodies(
        &mut conn,
        store,
        inbox,
        "INBOX",
        &OfflineWindow::default(),
        NOW,
    ))
    .unwrap();
}

fn folder(store: &Store, account: AccountId, path: &str) -> FolderId {
    store
        .folders(account)
        .unwrap()
        .into_iter()
        .find(|f| f.path == path)
        .unwrap()
        .id
}

fn in_folder(store: &Store, folder: FolderId) -> Vec<StoredMessage> {
    store.messages_in_folder(folder).unwrap()
}

fn subjects(list: Vec<StoredMessage>) -> Vec<String> {
    list.into_iter().map(|m| m.subject).collect()
}

fn by_subject(store: &Store, account: AccountId, subject: &str) -> StoredMessage {
    let ids: Vec<MessageId> = (1..=store.latest_message(account).unwrap().0)
        .map(MessageId)
        .collect();
    store
        .messages_by_id(&ids)
        .unwrap()
        .into_iter()
        .find(|m| m.subject == subject)
        .unwrap()
}

fn subject_has(value: &str, actions: Vec<Action>, account: AccountId) -> Rule {
    Rule {
        name: format!("Subject has {value}"),
        conditions: vec![Condition {
            field: Field::Subject,
            comparator: Comparator::Contains,
            value: value.to_owned(),
        }],
        actions,
        accounts: vec![account.0],
        ..Rule::default()
    }
}

/// A server with INBOX, Bills, Archive and Trash.
fn server() -> FakeServer {
    let server = FakeServer::default();
    for name in ["INBOX", "Bills", "Archive", "Trash"] {
        server.create(name, 1);
    }
    server
}

/// A store synced with [`server`] (one message in INBOX), and a watch
/// started as the daemon starts one: new mail is what comes next.
fn synced() -> (tempfile::TempDir, Store, AccountId, FakeServer, Watch) {
    let (tmp, mut store, account) = setup();
    let server = server();
    server.deliver("INBOX", "Seed");
    sync(&server, &mut store, account);
    let watch = Watch::new(&store, account).unwrap();
    (tmp, store, account, server, watch)
}

#[test]
fn new_mail_is_moved_and_flagged_but_not_the_mail_of_the_first_sync() {
    let (_tmp, mut store, account) = setup();
    let server = server();
    server.deliver("INBOX", "Invoice old");
    // A new account: the watch starts before its first sync.
    let mut watch = Watch::new(&store, account).unwrap();
    sync(&server, &mut store, account);
    let bills = folder(&store, account, "Bills");
    store
        .save_rule(&subject_has(
            "invoice",
            vec![
                Action::Move { folder: bills.0 },
                Action::MarkRead,
                Action::Star,
            ],
            account,
        ))
        .unwrap();
    // The first sync's mail is old: the run only notes where new mail
    // starts.
    let first = watch.run(&mut store, account, &live(NOW), true).unwrap();
    assert_eq!(first, rules::Outcome::default());
    let inbox = folder(&store, account, "INBOX");
    assert_eq!(subjects(in_folder(&store, inbox)), ["Invoice old"]);

    server.deliver("INBOX", "Invoice new");
    server.deliver("INBOX", "Hello");
    sync(&server, &mut store, account);
    let out = watch.run(&mut store, account, &live(NOW), true).unwrap();
    assert_eq!(out.changed, 1);
    assert_eq!(out.accounts, [account]);
    assert!(out.failed.is_empty() && out.held.is_empty() && out.quiet.is_empty());

    let moved = by_subject(&store, account, "Invoice new");
    assert_eq!(moved.flags, MessageFlags::SEEN | MessageFlags::FLAGGED);
    assert_eq!(subjects(in_folder(&store, bills)), ["Invoice new"]);
    assert_eq!(subjects(in_folder(&store, inbox)), ["Invoice old", "Hello"]);
    let old = by_subject(&store, account, "Invoice old");
    assert_eq!(old.flags, MessageFlags::empty());

    // The changes reach the server like the user's own.
    replay(&server, &mut store, account);
    assert_eq!(server.uids("Bills").len(), 1);
    let uid = server.uids("Bills")[0];
    assert!(server.flags("Bills", uid).seen && server.flags("Bills", uid).flagged);
    assert_eq!(server.uids("INBOX").len(), 2);

    // Mail the rules ran on is not new again, even back in the inbox.
    sync(&server, &mut store, account);
    ops::move_messages(&mut store, &[moved.id], inbox).unwrap();
    let again = watch.run(&mut store, account, &live(NOW), true).unwrap();
    assert_eq!(again.changed, 0);
}

#[test]
fn mail_from_the_account_itself_and_old_mail_are_left_alone() {
    let (_tmp, mut store, account, server, mut watch) = synced();
    store
        .save_rule(&subject_has("invoice", vec![Action::Archive], account))
        .unwrap();
    server.deliver_header(
        "INBOX",
        "From: alice@example.org\r\nTo: bob@example.org\r\nSubject: Invoice sent\r\n\
         Date: Sat, 26 Sep 2026 10:00:00 +0000\r\nMessage-ID: <own@example.org>\r\n\r\n",
        "Mine.\r\n",
    );
    server.deliver_header(
        "INBOX",
        "From: bob@example.org\r\nSubject: Invoice from last year\r\n\
         Date: Fri, 26 Sep 2025 10:00:00 +0000\r\nMessage-ID: <old@example.org>\r\n\r\n",
        "Old.\r\n",
    );
    sync(&server, &mut store, account);
    let out = watch.run(&mut store, account, &live(NOW), true).unwrap();
    assert_eq!(out.changed, 0);
    let inbox = folder(&store, account, "INBOX");
    assert_eq!(in_folder(&store, inbox).len(), 3);
}

#[test]
fn rules_run_in_order_and_stop() {
    let (_tmp, mut store, account, server, mut watch) = synced();
    let first = store
        .save_rule(&Rule {
            stop: true,
            ..subject_has("news", vec![Action::MarkRead], account)
        })
        .unwrap();
    let second = store
        .save_rule(&subject_has("news", vec![Action::Star], account))
        .unwrap();
    let other_account = store
        .save_rule(&subject_has(
            "news",
            vec![Action::MarkImportant],
            AccountId(account.0 + 1),
        ))
        .unwrap();
    store
        .reorder_rules(&[second, first, other_account])
        .unwrap();
    server.deliver("INBOX", "Weekly news");
    sync(&server, &mut store, account);
    watch.run(&mut store, account, &live(NOW), true).unwrap();
    let news = by_subject(&store, account, "Weekly news");
    assert_eq!(news.flags, MessageFlags::SEEN | MessageFlags::FLAGGED);

    // With the stopping rule first, the other doesn't run.
    store.reorder_rules(&[first, second]).unwrap();
    server.deliver("INBOX", "Daily news");
    sync(&server, &mut store, account);
    watch.run(&mut store, account, &live(NOW), true).unwrap();
    let news = by_subject(&store, account, "Daily news");
    assert_eq!(news.flags, MessageFlags::SEEN);
}

#[test]
fn a_rule_that_needs_the_body_waits_for_it_and_can_keep_quiet() {
    let (_tmp, mut store, account, server, mut watch) = synced();
    store
        .save_rule(&Rule {
            name: "Quiet reports".into(),
            conditions: vec![Condition {
                field: Field::Body,
                comparator: Comparator::Contains,
                value: "body of report".into(),
            }],
            actions: vec![Action::DontNotify, Action::MarkReadAfter { days: 2 }],
            accounts: vec![account.0],
            ..Rule::default()
        })
        .unwrap();
    server.deliver("INBOX", "Report");
    sync(&server, &mut store, account);
    let report = by_subject(&store, account, "Report");

    // Only headers so far: it waits, and isn't notified meanwhile.
    let out = watch.run(&mut store, account, &live(NOW), true).unwrap();
    assert_eq!(out.held, [report.id]);
    assert!(watch.waiting());

    download(&server, &mut store, account);
    let out = watch
        .run(&mut store, account, &live(NOW + 5), true)
        .unwrap();
    assert!(out.held.is_empty() && !watch.waiting());
    assert_eq!(out.quiet, [report.id]);
    assert_eq!(out.changed, 1);
    assert!(
        katna_meta::due(&store, NOW + 4 + 2 * 86_400)
            .unwrap()
            .is_empty()
    );
    let due = katna_meta::due(&store, NOW + 5 + 2 * 86_400).unwrap();
    assert_eq!(due, [katna_meta::Due::ReadAfter(report.id)]);
}

#[test]
fn mail_waits_for_its_body_only_so_long() {
    let (_tmp, mut store, account, server, mut watch) = synced();
    store
        .save_rule(&Rule {
            name: "Body".into(),
            conditions: vec![Condition {
                field: Field::Body,
                comparator: Comparator::NotContains,
                value: "never there".into(),
            }],
            actions: vec![Action::Star],
            accounts: vec![account.0],
            ..Rule::default()
        })
        .unwrap();
    server.deliver("INBOX", "Big");
    sync(&server, &mut store, account);
    let out = watch.run(&mut store, account, &live(NOW), true).unwrap();
    assert_eq!(out.held.len(), 1);
    let out = watch
        .run(&mut store, account, &live(NOW + HOLD), true)
        .unwrap();
    assert!(out.held.is_empty());
    assert_eq!(out.changed, 1, "the snippet stood in for the body");

    // Without waiting (a metered connection), the rules run at once.
    server.deliver("INBOX", "Metered");
    sync(&server, &mut store, account);
    let out = watch.run(&mut store, account, &live(NOW), false).unwrap();
    assert!(out.held.is_empty());
    assert_eq!(out.changed, 1);
}

#[test]
fn a_rule_whose_folder_is_gone_is_switched_off_with_the_reason() {
    let (_tmp, mut store, account, server, mut watch) = synced();
    let id = store
        .save_rule(&subject_has(
            "invoice",
            vec![Action::MarkRead, Action::Move { folder: 9999 }],
            account,
        ))
        .unwrap();
    server.deliver("INBOX", "Invoice 1");
    server.deliver("INBOX", "Invoice 2");
    sync(&server, &mut store, account);
    let out = watch.run(&mut store, account, &live(NOW), true).unwrap();
    assert_eq!(out.failed.len(), 1);
    assert_eq!(out.failed[0].0, id);
    let rule = store.rule(id).unwrap().unwrap();
    assert!(!rule.enabled);
    assert_eq!(
        rule.last_error.as_deref(),
        Some("folder 9999 no longer exists")
    );
    // Switched off, it no longer runs.
    server.deliver("INBOX", "Invoice 3");
    sync(&server, &mut store, account);
    let out = watch.run(&mut store, account, &live(NOW), true).unwrap();
    assert!(out.failed.is_empty());
    assert_eq!(
        by_subject(&store, account, "Invoice 3").flags,
        MessageFlags::empty()
    );
}

#[test]
fn label_forward_and_trash() {
    // Labels are Gmail's.
    let (_tmp, mut store, account) = setup();
    let server = server();
    server.create("[Gmail]/All Mail", 1);
    server.deliver("INBOX", "Seed");
    sync(&server, &mut store, account);
    let mut watch = Watch::new(&store, account).unwrap();
    let bills = folder(&store, account, "Bills");
    store
        .save_rule(&subject_has(
            "spam",
            vec![
                Action::AddLabel { folder: bills.0 },
                Action::Forward {
                    to: "boss@example.net".into(),
                },
                Action::Trash,
            ],
            account,
        ))
        .unwrap();
    server.deliver("INBOX", "Spam offer");
    sync(&server, &mut store, account);
    download(&server, &mut store, account);
    let out = watch.run(&mut store, account, &live(NOW), true).unwrap();
    assert_eq!(out.changed, 1);
    assert_eq!(out.outbox.len(), 1);
    let spam = by_subject(&store, account, "Spam offer");
    let (inbox, trash) = (
        folder(&store, account, "INBOX"),
        folder(&store, account, "Trash"),
    );
    assert!(in_folder(&store, trash).iter().any(|m| m.id == spam.id));
    assert!(in_folder(&store, bills).iter().any(|m| m.id == spam.id));
    assert!(!in_folder(&store, inbox).iter().any(|m| m.id == spam.id));

    // An account that can't send fails the rule.
    server.deliver("INBOX", "More spam");
    sync(&server, &mut store, account);
    download(&server, &mut store, account);
    let cannot = Context {
        can_send: false,
        ..live(NOW)
    };
    let out = watch.run(&mut store, account, &cannot, true).unwrap();
    assert_eq!(out.failed.len(), 1);
    assert_eq!(out.failed[0].1, "the account cannot send mail");
}

#[test]
fn a_label_on_an_account_without_labels_fails_the_rule() {
    let (_tmp, mut store, account, server, mut watch) = synced();
    let bills = folder(&store, account, "Bills");
    let id = store
        .save_rule(&subject_has(
            "news",
            vec![Action::AddLabel { folder: bills.0 }],
            account,
        ))
        .unwrap();
    server.deliver("INBOX", "News");
    sync(&server, &mut store, account);
    let out = watch.run(&mut store, account, &live(NOW), true).unwrap();
    assert_eq!(out.failed, [(id, "only Gmail accounts have labels".to_owned())]);
    assert!(!store.rule(id).unwrap().unwrap().enabled);
}

#[test]
fn forwarded_mail_carries_the_original() {
    let mailbox = |name: Option<&str>, email: &str| katna_sync::quick_reply::Mailbox {
        name: name.map(str::to_owned),
        email: email.to_owned(),
    };
    let forwarded = rules::forwarded(
        b"Subject: Spam offer\r\n\r\nHi\r\n",
        &mailbox(Some("Alice"), "alice@example.org"),
        &mailbox(None, "boss@example.net"),
        "Spam offer",
    );
    let text = String::from_utf8(forwarded).unwrap();
    assert!(text.starts_with("From: Alice <alice@example.org>\r\nTo: boss@example.net\r\n"));
    assert!(text.contains("Subject: Fwd: Spam offer\r\n"));
    assert!(text.contains("Content-Type: message/rfc822\r\n"));
    assert!(text.contains("\r\n\r\nSubject: Spam offer\r\n\r\nHi\r\n--katna-forward-"));
    let again = rules::forwarded(
        b"Subject: x\r\n\r\n",
        &mailbox(None, "a@example.org"),
        &mailbox(None, "b@example.org"),
        "FWD: x",
    );
    assert!(
        String::from_utf8(again)
            .unwrap()
            .contains("Subject: FWD: x\r\n")
    );
}

#[test]
fn also_apply_to_these_runs_over_recent_inbox_mail() {
    let (_tmp, mut store, account) = setup();
    let server = server();
    server.deliver("INBOX", "Invoice 1");
    server.deliver("INBOX", "Invoice 2");
    server.deliver("INBOX", "Hello");
    sync(&server, &mut store, account);
    let bills = folder(&store, account, "Bills");
    let rule = Rule {
        enabled: false,
        ..subject_has("invoice", vec![Action::Move { folder: bills.0 }], account)
    };
    let id = store.save_rule(&rule).unwrap();
    let rule = store.rule(id).unwrap().unwrap();
    let preview = store.rule_preview(&rule, 30, NOW, |_| None).unwrap();
    assert_eq!(preview, 2);
    let out = rules::apply_recent(&mut store, &rule, 30, NOW).unwrap();
    assert_eq!(out.changed, 2);
    assert_eq!(out.accounts, [account]);
    assert_eq!(in_folder(&store, bills).len(), 2);
    // Again: nothing left to change.
    let out = rules::apply_recent(&mut store, &rule, 30, NOW).unwrap();
    assert_eq!(out.changed, 0);
    // Mail older than the days asked for is left alone.
    let inbox = folder(&store, account, "INBOX");
    let ids: Vec<MessageId> = in_folder(&store, bills).iter().map(|m| m.id).collect();
    ops::move_messages(&mut store, &ids, inbox).unwrap();
    let out = rules::apply_recent(&mut store, &rule, 1, NOW + 10 * 86_400).unwrap();
    assert_eq!(out.changed, 0);
}
