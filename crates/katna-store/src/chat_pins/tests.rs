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
fn the_newest_pin_comes_first_and_five_is_the_most() {
    let (_tmp, mut store, ids) = rig();
    let mut batch = store.mail_batch().unwrap();
    let what = [
        Pinned::Mail,
        Pinned::File(0),
        Pinned::Text("18 to 22".into()),
        Pinned::File(1),
        Pinned::Text("before the new-year prices".into()),
    ];
    for (n, what) in what.iter().enumerate() {
        let pinned = batch
            .pin_in_chat(ids[n % 3], &ids, what, "label", n as i64)
            .unwrap();
        assert!(pinned.is_some());
    }
    // A sixth, and the same thing twice, are refused.
    assert_eq!(
        batch
            .pin_in_chat(ids[0], &ids, &Pinned::Text("more".into()), "", 9)
            .unwrap(),
        None
    );
    batch.commit().unwrap();
    let pins = store.chat_pins(&ids).unwrap();
    assert_eq!(pins.len(), 5);
    assert_eq!(
        pins[0].what,
        Pinned::Text("before the new-year prices".into())
    );
    assert_eq!(pins[4].what, Pinned::Mail);
    // Only this conversation's messages are asked.
    assert!(store.chat_pins(&ids[2..]).unwrap().len() < 5);
}

#[test]
fn pins_come_off_and_take_a_new_order() {
    let (_tmp, mut store, ids) = rig();
    let mut batch = store.mail_batch().unwrap();
    let a = batch
        .pin_in_chat(ids[0], &ids, &Pinned::Mail, "a", 1)
        .unwrap()
        .unwrap();
    let b = batch
        .pin_in_chat(ids[1], &ids, &Pinned::Mail, "b", 2)
        .unwrap()
        .unwrap();
    assert_eq!(
        batch
            .pin_in_chat(ids[1], &ids, &Pinned::Mail, "b", 3)
            .unwrap(),
        None
    );
    batch.order_chat_pins(&[a, b]).unwrap();
    batch.commit().unwrap();
    let order: Vec<i64> = store
        .chat_pins(&ids)
        .unwrap()
        .iter()
        .map(|p| p.id)
        .collect();
    assert_eq!(order, [a, b]);
    let mut batch = store.mail_batch().unwrap();
    assert!(batch.unpin_in_chat(a).unwrap());
    assert!(!batch.unpin_in_chat(a).unwrap());
    batch.commit().unwrap();
    assert_eq!(store.chat_pins(&ids).unwrap().len(), 1);
}
