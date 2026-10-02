// SPDX-License-Identifier: GPL-3.0-or-later

use katna_core::{AccountKind, Paths};

use super::*;
use crate::mail::{NewMessage, NewParticipant};
use crate::{FolderRole, Mode, ParticipantRole};

fn add(
    batch: &mut MailBatch<'_>,
    account: katna_core::AccountId,
    folder: crate::FolderId,
    id: &str,
) -> MessageId {
    let from = [NewParticipant {
        role: ParticipantRole::From,
        email_norm: "ann@example.org",
        domain: "example.org",
        display_name: None,
    }];
    let added = batch
        .add_message(
            account,
            folder,
            &NewMessage {
                raw: id.as_bytes(),
                message_id_hdr: Some(id),
                subject: Some("Goa"),
                date: Some(1_800_000_000),
                flags: crate::MessageFlags::empty(),
                has_attachments: false,
                list_id: None,
                snippet: None,
                participants: &from,
                in_reply_to: None,
                references: &[],
                category: None,
            },
        )
        .unwrap();
    match added {
        crate::mail::Added::Message(id)
        | crate::mail::Added::Location(id)
        | crate::mail::Added::Duplicate(id) => id,
    }
}

fn rig() -> (tempfile::TempDir, Store, Vec<MessageId>) {
    let tmp = tempfile::tempdir().unwrap();
    let mut store = Store::open(&Paths::with_root(tmp.path()), Mode::ReadWrite).unwrap();
    let account = store
        .add_account(AccountKind::Local, "Test", "me@example.org")
        .unwrap()
        .id;
    let mut batch = store.mail_batch().unwrap();
    let inbox = batch
        .upsert_folder(account, "INBOX", Some(FolderRole::Inbox))
        .unwrap();
    let ids = (0..3)
        .map(|n| add(&mut batch, account, inbox, &format!("m{n}@x")))
        .collect();
    batch.commit().unwrap();
    (tmp, store, ids)
}

#[test]
fn a_conversation_keeps_one_summary_of_each_kind() {
    let (_tmp, mut store, ids) = rig();
    let mut batch = store.mail_batch().unwrap();
    batch
        .save_summary(
            ids[1],
            &ids,
            SummaryKind::All,
            2,
            "{\"gist\":\"a\"}",
            "katna",
            1,
        )
        .unwrap();
    batch
        .save_summary(
            ids[2],
            &ids,
            SummaryKind::New,
            1,
            "{\"gist\":\"b\"}",
            "own",
            2,
        )
        .unwrap();
    batch.commit().unwrap();
    let kept = store.conversation_summaries(&ids).unwrap();
    assert_eq!(kept.len(), 2);
    assert_eq!(kept[0].kind, SummaryKind::New);
    assert_eq!(kept[0].service, "own");
    assert_eq!(kept[1].message, ids[1]);
    assert_eq!(kept[1].mails, 2);

    // A newer summary of the whole conversation replaces the older one.
    let mut batch = store.mail_batch().unwrap();
    batch
        .save_summary(
            ids[2],
            &ids,
            SummaryKind::All,
            3,
            "{\"gist\":\"c\"}",
            "katna",
            3,
        )
        .unwrap();
    batch.commit().unwrap();
    let kept = store.conversation_summaries(&ids).unwrap();
    assert_eq!(kept.len(), 2);
    assert_eq!(kept[0].body, "{\"gist\":\"c\"}");
    assert_eq!(kept[0].mails, 3);
    assert!(store.conversation_summaries(&[]).unwrap().is_empty());
}

#[test]
fn a_summary_goes_with_its_mail() {
    let (_tmp, mut store, ids) = rig();
    let mut batch = store.mail_batch().unwrap();
    batch
        .save_summary(ids[0], &ids, SummaryKind::All, 1, "{}", "katna", 1)
        .unwrap();
    batch.commit().unwrap();
    store
        .mail
        .execute("DELETE FROM message WHERE id = ?1", [ids[0].0])
        .unwrap();
    assert!(store.conversation_summaries(&ids).unwrap().is_empty());
}
