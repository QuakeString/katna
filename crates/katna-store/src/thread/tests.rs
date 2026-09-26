// SPDX-License-Identifier: GPL-3.0-or-later

//! Threading, the conversation list API and the backfill hooks.

use katna_core::{AccountId, AccountKind, MailCategory, Paths};
use rusqlite::OptionalExtension;

use crate::{
    Added, Backfill, ChangeOp, DbKind, FolderId, FolderRole, MessageFlags, MessageId, Mode,
    NewMessage, NewParticipant, ObjectKind, ParticipantRole, RemoteMessage, Store, ThreadEntry,
    ThreadId,
};

const DAY: i64 = 24 * 60 * 60;
const T0: i64 = 1_790_000_000;

struct Fixture {
    _tmp: tempfile::TempDir,
    paths: Paths,
    store: Store,
    account: AccountId,
}

fn fixture() -> Fixture {
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths::with_root(tmp.path());
    let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
    let account = store
        .add_account(AccountKind::Imap, "Work", "me@example.org")
        .unwrap()
        .id;
    Fixture {
        _tmp: tmp,
        paths,
        store,
        account,
    }
}

/// A message to store.
#[derive(Clone, Copy)]
struct Mail<'a> {
    id: &'a str,
    subject: &'a str,
    date: i64,
    in_reply_to: Option<&'a str>,
    references: &'a [&'a str],
    from: &'a str,
    flags: MessageFlags,
}

fn mail<'a>(id: &'a str, subject: &'a str, date: i64) -> Mail<'a> {
    Mail {
        id,
        subject,
        date,
        in_reply_to: None,
        references: &[],
        from: "ada@example.org",
        flags: MessageFlags::SEEN,
    }
}

impl Fixture {
    fn folder(&mut self, path: &str, role: Option<FolderRole>) -> FolderId {
        let mut batch = self.store.mail_batch().unwrap();
        let folder = batch.upsert_folder(self.account, path, role).unwrap();
        batch.commit().unwrap();
        folder
    }

    /// Stores `m` as the importer does (with a raw message).
    fn add(&mut self, folder: FolderId, m: &Mail<'_>) -> MessageId {
        let raw = format!("Message-ID: <{}>\r\nSubject: {}\r\n\r\n", m.id, m.subject);
        let from = [NewParticipant {
            role: ParticipantRole::From,
            email_norm: m.from,
            domain: m.from.rsplit_once('@').unwrap().1,
            display_name: None,
        }];
        let mut batch = self.store.mail_batch().unwrap();
        let added = batch
            .add_message(
                self.account,
                folder,
                &NewMessage {
                    raw: raw.as_bytes(),
                    message_id_hdr: Some(m.id),
                    subject: Some(m.subject),
                    date: Some(m.date),
                    flags: m.flags,
                    has_attachments: false,
                    list_id: None,
                    snippet: None,
                    participants: &from,
                    in_reply_to: m.in_reply_to,
                    references: m.references,
                    category: Some(MailCategory::Primary),
                },
            )
            .unwrap();
        batch.commit().unwrap();
        let Added::Message(id) = added else {
            panic!("new message expected");
        };
        id
    }

    /// Stores `m` as sync does (headers only).
    fn remote(
        &mut self,
        folder: FolderId,
        uid: u32,
        m: &Mail<'_>,
        thrid: Option<u64>,
    ) -> MessageId {
        let mut batch = self.store.mail_batch().unwrap();
        let added = batch
            .add_remote_message(
                self.account,
                folder,
                &RemoteMessage {
                    uid,
                    message_id_hdr: Some(m.id),
                    subject: Some(m.subject),
                    date: Some(m.date),
                    size: 100,
                    flags: m.flags,
                    keywords: &[],
                    has_attachments: false,
                    list_id: None,
                    participants: &[],
                    in_reply_to: m.in_reply_to,
                    references: m.references,
                    gm_thread_id: thrid,
                    gm_msgid: None,
                    category: None,
                    attachments: &[],
                },
            )
            .unwrap();
        batch.commit().unwrap();
        let Added::Message(id) = added else {
            panic!("new message expected");
        };
        id
    }

    fn thread_of(&self, id: MessageId) -> Option<ThreadId> {
        self.store.messages_by_id(&[id]).unwrap()[0].thread_id
    }

    /// `(message_count, last_date)` of a thread, or `None` if it is gone.
    fn stats(&self, thread: ThreadId) -> Option<(i64, Option<i64>)> {
        self.store
            .mail
            .query_row(
                "SELECT message_count, last_date FROM thread WHERE id = ?1",
                [thread.0],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()
            .unwrap()
    }

    fn count(&self, table: &str) -> i64 {
        self.store
            .mail
            .query_row(&format!("SELECT count(*) FROM {table}"), [], |row| {
                row.get(0)
            })
            .unwrap()
    }
}

#[test]
fn replies_join_the_thread_they_answer() {
    let mut f = fixture();
    let inbox = f.folder("INBOX", Some(FolderRole::Inbox));
    let a = f.add(inbox, &mail("a@x", "Budget", T0));
    let b = f.add(
        inbox,
        &Mail {
            in_reply_to: Some("a@x"),
            ..mail("b@x", "Re: Budget", T0 + DAY)
        },
    );
    let c = f.add(
        inbox,
        &Mail {
            references: &["a@x", "b@x"],
            ..mail("c@x", "Totally different subject", T0 + 2 * DAY)
        },
    );
    let other = f.add(inbox, &mail("d@x", "Lunch", T0 + 3 * DAY));
    let thread = f.thread_of(a).unwrap();
    assert_eq!(f.thread_of(b), Some(thread));
    assert_eq!(f.thread_of(c), Some(thread));
    assert_ne!(f.thread_of(other), Some(thread));
    assert_eq!(f.stats(thread), Some((3, Some(T0 + 2 * DAY))));
    assert_eq!(f.store.thread_messages(thread).unwrap(), [a, b, c]);
}

#[test]
fn replies_that_arrive_first_are_joined_and_threads_merge() {
    let mut f = fixture();
    let inbox = f.folder("INBOX", Some(FolderRole::Inbox));
    // b answers a, c answers x; neither a nor x is here yet.
    let b = f.add(
        inbox,
        &Mail {
            in_reply_to: Some("a@x"),
            ..mail("b@x", "Re: Plan", T0 + DAY)
        },
    );
    let c = f.add(
        inbox,
        &Mail {
            in_reply_to: Some("x@x"),
            ..mail("c@x", "Re: Other plan", T0 + 2 * DAY)
        },
    );
    let (tb, tc) = (f.thread_of(b).unwrap(), f.thread_of(c).unwrap());
    assert_ne!(tb, tc);
    let seen = f.store.latest_change(DbKind::Mail).unwrap();

    // a arrives: b was waiting for it.
    let a = f.add(inbox, &mail("a@x", "Plan", T0));
    assert_eq!(f.thread_of(a), Some(tb));
    assert_eq!(f.stats(tb), Some((2, Some(T0 + DAY))));

    // x arrives and cites b: it links both threads, which merge into the
    // older one.
    let x = f.add(
        inbox,
        &Mail {
            references: &["a@x", "b@x"],
            ..mail("x@x", "Re: Plan", T0 + DAY / 2)
        },
    );
    for id in [a, b, c, x] {
        assert_eq!(f.thread_of(id), Some(tb), "{id}");
    }
    assert_eq!(f.stats(tb), Some((4, Some(T0 + 2 * DAY))));
    assert_eq!(f.stats(tc), None, "merged away");
    assert_eq!(f.count("thread"), 1);
    assert_eq!(f.store.thread_messages(tb).unwrap(), [a, x, b, c]);
    let changes: Vec<_> = f
        .store
        .changes_since(DbKind::Mail, seen, 100)
        .unwrap()
        .into_iter()
        .filter(|c| c.kind == ObjectKind::Thread)
        .map(|c| (ThreadId(c.object_id), c.op))
        .collect();
    assert!(changes.contains(&(tc, ChangeOp::Delete)), "{changes:?}");
    assert!(changes.contains(&(tb, ChangeOp::Update)), "{changes:?}");
    assert_eq!(f.count("thread_ref"), 0, "every awaited message arrived");
}

#[test]
fn reply_subjects_join_recent_threads_only() {
    let mut f = fixture();
    let inbox = f.folder("INBOX", Some(FolderRole::Inbox));
    let first = f.add(inbox, &mail("1@x", "Quarterly numbers", T0));
    let reply = f.add(
        inbox,
        &mail("2@x", "RE: [finance] Re: quarterly  NUMBERS", T0 + 10 * DAY),
    );
    let forward = f.add(inbox, &mail("3@x", "Fwd: Quarterly numbers", T0 + 35 * DAY));
    let late = f.add(inbox, &mail("4@x", "Re: Quarterly numbers", T0 + 90 * DAY));
    let fresh = f.add(inbox, &mail("5@x", "Quarterly numbers", T0 + 91 * DAY));
    let thread = f.thread_of(first).unwrap();
    assert_eq!(f.thread_of(reply), Some(thread));
    assert_eq!(
        f.thread_of(forward),
        Some(thread),
        "within 30 days of the thread's last message"
    );
    assert_ne!(f.thread_of(late), Some(thread));
    assert_ne!(f.thread_of(fresh), f.thread_of(late), "no prefix: new");
}

#[test]
fn deletes_keep_counts_and_remove_empty_threads() {
    let mut f = fixture();
    let inbox = f.folder("INBOX", Some(FolderRole::Inbox));
    let a = f.remote(inbox, 1, &mail("a@x", "Hi", T0), None);
    let b = f.remote(
        inbox,
        2,
        &Mail {
            in_reply_to: Some("a@x"),
            ..mail("b@x", "Re: Hi", T0 + DAY)
        },
        None,
    );
    let thread = f.thread_of(a).unwrap();
    assert_eq!(f.thread_of(b), Some(thread));
    assert_eq!(f.stats(thread), Some((2, Some(T0 + DAY))));

    let mut batch = f.store.mail_batch().unwrap();
    batch.remove_remote_messages(inbox, &[2]).unwrap();
    batch.commit().unwrap();
    assert_eq!(f.stats(thread), Some((1, Some(T0))), "last date goes back");

    let mut batch = f.store.mail_batch().unwrap();
    batch.remove_from_folder(a, inbox).unwrap();
    batch.commit().unwrap();
    assert_eq!(f.stats(thread), None);
    assert_eq!(f.count("thread"), 0);
}

#[test]
fn server_copies_count_once_and_trash_is_hidden() {
    let mut f = fixture();
    let inbox = f.folder("INBOX", Some(FolderRole::Inbox));
    let all = f.folder("[Gmail]/All Mail", Some(FolderRole::All));
    let trash = f.folder("[Gmail]/Trash", Some(FolderRole::Trash));
    let a_inbox = f.remote(inbox, 1, &mail("a@x", "Hi", T0), None);
    let a_all = f.remote(all, 1, &mail("a@x", "Hi", T0), None);
    let reply = Mail {
        in_reply_to: Some("a@x"),
        ..mail("b@x", "Re: Hi", T0 + DAY)
    };
    let b_all = f.remote(all, 2, &reply, None);
    let c_trash = f.remote(
        trash,
        1,
        &Mail {
            in_reply_to: Some("b@x"),
            ..mail("c@x", "Re: Hi", T0 + 2 * DAY)
        },
        None,
    );
    let thread = f.thread_of(a_inbox).unwrap();
    for id in [a_all, b_all, c_trash] {
        assert_eq!(f.thread_of(id), Some(thread));
    }
    assert_eq!(f.stats(thread), Some((4, Some(T0 + 2 * DAY))), "rows");
    assert_eq!(f.store.thread_messages(thread).unwrap(), [a_inbox, b_all]);

    // Only the trashed message left: then it is shown.
    let mut batch = f.store.mail_batch().unwrap();
    for (id, folder) in [(a_inbox, inbox), (a_all, all), (b_all, all)] {
        batch.remove_from_folder(id, folder).unwrap();
    }
    batch.commit().unwrap();
    assert_eq!(f.store.thread_messages(thread).unwrap(), [c_trash]);
}

#[test]
fn gmail_thread_ids_are_authoritative() {
    let mut f = fixture();
    let all = f.folder("[Gmail]/All Mail", Some(FolderRole::All));
    // Threaded before the thread ID was known, then adopted.
    let old = f.remote(all, 1, &mail("a@x", "Trip", T0), None);
    let first = f.remote(
        all,
        2,
        &Mail {
            in_reply_to: Some("a@x"),
            ..mail("b@x", "Re: Trip", T0 + DAY)
        },
        Some(77),
    );
    assert_eq!(f.thread_of(first), f.thread_of(old));
    let same = f.remote(
        all,
        3,
        &mail("c@x", "Unrelated subject", T0 + 2 * DAY),
        Some(77),
    );
    assert_eq!(f.thread_of(same), f.thread_of(old));
    // Gmail split this one off although it cites the thread.
    let split = f.remote(
        all,
        4,
        &Mail {
            in_reply_to: Some("a@x"),
            ..mail("d@x", "Re: Trip", T0 + 3 * DAY)
        },
        Some(u64::MAX - 1),
    );
    assert_ne!(f.thread_of(split), f.thread_of(old));
    let again = f.remote(all, 5, &mail("e@x", "x", T0), Some(u64::MAX - 1));
    assert_eq!(f.thread_of(again), f.thread_of(split), "big IDs round-trip");
}

#[test]
fn folder_lists_summaries_and_tab_counts() {
    let mut f = fixture();
    let inbox = f.folder("INBOX", Some(FolderRole::Inbox));
    let sent = f.folder("Sent", Some(FolderRole::Sent));
    let a = f.add(inbox, &mail("a@x", "Hi", T0));
    let b = f.add(
        sent,
        &Mail {
            in_reply_to: Some("a@x"),
            from: "me@example.org",
            ..mail("b@x", "Re: Hi", T0 + DAY)
        },
    );
    let c = f.add(
        inbox,
        &Mail {
            in_reply_to: Some("b@x"),
            flags: MessageFlags::empty(),
            ..mail("c@x", "Re: Hi", T0 + 2 * DAY)
        },
    );
    let promo = f.add(
        inbox,
        &Mail {
            from: "deals@shop.example",
            flags: MessageFlags::empty(),
            ..mail("p@x", "Sale", T0 + 3 * DAY)
        },
    );
    let old = f.add(inbox, &mail("o@x", "Old", T0 - DAY));
    let mut batch = f.store.mail_batch().unwrap();
    batch
        .set_message_flags(c, MessageFlags::FLAGGED | MessageFlags::IMPORTANT)
        .unwrap();
    batch.commit().unwrap();
    f.store
        .mail
        .execute(
            "UPDATE message SET category = ?2 WHERE id = ?1",
            [promo.0, MailCategory::Promotions.to_storage()],
        )
        .unwrap();
    // A message from before threading.
    f.store
        .mail
        .execute(
            "UPDATE message SET thread_id = NULL, category = NULL WHERE id = ?1",
            [old.0],
        )
        .unwrap();

    let app = Store::open(&f.paths, Mode::ReadOnly).unwrap();
    let thread = app.messages_by_id(&[a]).unwrap()[0].thread_id.unwrap();
    let promo_thread = app.messages_by_id(&[promo]).unwrap()[0].thread_id.unwrap();
    let entries = app.folder_threads(inbox, None).unwrap();
    assert_eq!(
        entries,
        [
            ThreadEntry {
                thread: Some(promo_thread),
                latest: promo
            },
            ThreadEntry {
                thread: Some(thread),
                latest: c
            },
            ThreadEntry {
                thread: None,
                latest: old
            },
        ]
    );
    assert_eq!(
        app.folder_threads(inbox, Some(&[MailCategory::Primary]))
            .unwrap(),
        entries[1..]
    );
    assert_eq!(
        app.folder_threads(inbox, Some(&[MailCategory::Promotions]))
            .unwrap(),
        entries[..1]
    );
    assert!(
        app.folder_threads(inbox, Some(&[MailCategory::Social]))
            .unwrap()
            .is_empty()
    );
    // A tab of several categories, such as Focused and Other's Other.
    assert_eq!(
        app.folder_threads(
            inbox,
            Some(&[MailCategory::Primary, MailCategory::Promotions])
        )
        .unwrap(),
        entries
    );
    assert_eq!(app.folder_thread_messages(sent, thread).unwrap(), [b]);
    // Without conversations, the tabs hold single messages.
    assert_eq!(
        app.folder_messages_in(inbox, &[MailCategory::Promotions])
            .unwrap(),
        [promo]
    );
    let primary = app
        .folder_messages_in(inbox, &[MailCategory::Primary])
        .unwrap();
    assert!(primary.contains(&c) && primary.contains(&old) && !primary.contains(&promo));
    assert!(
        app.folder_messages_in(inbox, &[MailCategory::Social])
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        app.folder_threads(sent, None).unwrap(),
        [ThreadEntry {
            thread: Some(thread),
            latest: b
        }]
    );

    let summaries = app
        .thread_summaries(&[thread, ThreadId(999), promo_thread], inbox)
        .unwrap();
    assert_eq!(summaries.len(), 2);
    let s = &summaries[0];
    assert_eq!(s.thread, thread);
    assert_eq!(s.message_count, 3);
    assert!(s.unread && s.flagged && s.important && !s.has_attachments);
    let senders: Vec<_> = s
        .senders
        .iter()
        .map(|s| (s.email.as_str(), s.unread))
        .collect();
    assert_eq!(
        senders,
        [("ada@example.org", true), ("me@example.org", false)]
    );
    let sent_view = &app.thread_summaries(&[thread], sent).unwrap()[0];
    assert!(!sent_view.unread, "the unread message is not in Sent");

    assert_eq!(
        app.category_unread(inbox).unwrap(),
        [
            (MailCategory::Primary, 1),
            (MailCategory::Promotions, 1),
            (MailCategory::Social, 0),
            (MailCategory::Updates, 0),
            (MailCategory::Forums, 0),
        ]
    );
}

#[test]
fn backfill_threads_old_messages_once() {
    let mut f = fixture();
    let inbox = f.folder("INBOX", Some(FolderRole::Inbox));
    let a = f.add(inbox, &mail("a@x", "Hi", T0));
    let b = f.add(inbox, &mail("b@x", "Something else", T0 + DAY));
    let c = f.remote(inbox, 9, &mail("c@x", "Re: Hi", T0 + 2 * DAY), None);
    // As a store from before threading: no threads, no categories.
    f.store
        .mail
        .execute("UPDATE message SET thread_id = NULL, category = NULL", [])
        .unwrap();
    assert_eq!(f.count("thread"), 0, "emptied threads are removed");
    assert_eq!(f.store.unthreaded_count().unwrap(), 3);
    let with_body = f.store.unthreaded_with_body(MessageId(0), 10).unwrap();
    assert_eq!(
        with_body.iter().map(|(id, _)| *id).collect::<Vec<_>>(),
        [a, b],
        "c has no body"
    );
    assert_eq!(f.store.uids_needing_headers(inbox, 10).unwrap(), [9]);

    // Out of order: b's headers say it answers a.
    let mut batch = f.store.mail_batch().unwrap();
    let facts = Backfill {
        in_reply_to: Some("a@x"),
        category: Some(MailCategory::Updates),
        ..Backfill::default()
    };
    assert!(batch.backfill_message(b, &facts).unwrap());
    assert!(!batch.backfill_message(b, &facts).unwrap(), "idempotent");
    assert!(batch.backfill_message(a, &Backfill::default()).unwrap());
    let primary = Backfill {
        category: Some(MailCategory::Primary),
        ..Backfill::default()
    };
    assert!(batch.backfill_remote(inbox, 9, &primary).unwrap());
    assert!(!batch.backfill_remote(inbox, 10, &primary).unwrap());
    batch.commit().unwrap();

    let thread = f.thread_of(a).unwrap();
    assert_eq!(f.thread_of(b), Some(thread));
    assert_eq!(f.thread_of(c), Some(thread), "by subject");
    assert_eq!(f.stats(thread), Some((3, Some(T0 + 2 * DAY))));
    let stored = f.store.messages_by_id(&[b, a]).unwrap();
    assert_eq!(stored[0].category, Some(MailCategory::Updates));
    assert_eq!(stored[1].category, None, "nothing to fill in");
    assert!(f.store.uids_needing_headers(inbox, 10).unwrap().is_empty());
    assert_eq!(f.store.unthreaded_count().unwrap(), 1);
}

#[test]
fn gmail_categories_are_set_per_uid() {
    let mut f = fixture();
    let inbox = f.folder("INBOX", Some(FolderRole::Inbox));
    let a = f.remote(inbox, 1, &mail("a@x", "A", T0), None);
    let b = f.remote(inbox, 2, &mail("b@x", "B", T0), None);
    let seen = f.store.latest_change(DbKind::Mail).unwrap();
    let mut batch = f.store.mail_batch().unwrap();
    let changed = batch
        .set_categories(
            inbox,
            &[
                (1, MailCategory::Social),
                (2, MailCategory::Primary),
                (3, MailCategory::Updates),
            ],
        )
        .unwrap();
    assert_eq!(changed, 2);
    assert_eq!(
        batch
            .set_categories(inbox, &[(1, MailCategory::Social)])
            .unwrap(),
        0
    );
    batch.commit().unwrap();
    let stored = f.store.messages_by_id(&[a, b]).unwrap();
    assert_eq!(stored[0].category, Some(MailCategory::Social));
    assert_eq!(stored[1].category, Some(MailCategory::Primary));
    let kinds: Vec<_> = f
        .store
        .changes_since(DbKind::Mail, seen, 10)
        .unwrap()
        .into_iter()
        .map(|c| c.kind)
        .collect();
    assert_eq!(kinds, [ObjectKind::Thread, ObjectKind::Thread]);
}
