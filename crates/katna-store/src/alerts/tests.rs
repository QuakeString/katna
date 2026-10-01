// SPDX-License-Identifier: GPL-3.0-or-later

use katna_core::{AccountKind, Paths};

use super::*;
use crate::mail::{NewMessage, NewParticipant};
use crate::{Mode, ParticipantRole};

const NOW: i64 = 1_800_000_000;

struct Mail<'a> {
    id: &'a str,
    from: &'a str,
    reply_to: Option<&'a str>,
    category: Option<MailCategory>,
    flags: MessageFlags,
}

impl<'a> Mail<'a> {
    fn new(id: &'a str) -> Self {
        Self {
            id,
            from: "ann@example.org",
            reply_to: None,
            category: None,
            flags: MessageFlags::empty(),
        }
    }
}

fn add(
    batch: &mut MailBatch<'_>,
    account: AccountId,
    folder: FolderId,
    mail: Mail<'_>,
) -> MessageId {
    let from = [NewParticipant {
        role: ParticipantRole::From,
        email_norm: mail.from,
        domain: mail.from.rsplit('@').next().unwrap_or_default(),
        display_name: None,
    }];
    let references: Vec<&str> = mail.reply_to.into_iter().collect();
    let added = batch
        .add_message(
            account,
            folder,
            &NewMessage {
                raw: mail.id.as_bytes(),
                message_id_hdr: Some(mail.id),
                subject: Some("Hello"),
                date: Some(NOW),
                flags: mail.flags,
                has_attachments: false,
                list_id: None,
                snippet: None,
                participants: &from,
                in_reply_to: mail.reply_to,
                references: &references,
                category: mail.category,
            },
        )
        .unwrap();
    match added {
        crate::mail::Added::Message(id)
        | crate::mail::Added::Location(id)
        | crate::mail::Added::Duplicate(id) => id,
    }
}

struct Rig {
    _tmp: tempfile::TempDir,
    store: Store,
    account: AccountId,
    inbox: FolderId,
    work: FolderId,
}

fn rig() -> Rig {
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
    let work = batch.upsert_folder(account, "Work", None).unwrap();
    batch.commit().unwrap();
    Rig {
        _tmp: tmp,
        store,
        account,
        inbox,
        work,
    }
}

fn ringing(store: &Store, account: AccountId, now: i64) -> Vec<MessageId> {
    store
        .new_ringing_mail(account, MessageId(0), 0, 100, now)
        .unwrap()
}

#[test]
fn only_the_inboxs_primary_tab_rings_by_default() {
    let mut r = rig();
    let mut batch = r.store.mail_batch().unwrap();
    let plain = add(&mut batch, r.account, r.inbox, Mail::new("a@x"));
    let primary = add(
        &mut batch,
        r.account,
        r.inbox,
        Mail {
            category: Some(MailCategory::Primary),
            ..Mail::new("b@x")
        },
    );
    add(
        &mut batch,
        r.account,
        r.inbox,
        Mail {
            category: Some(MailCategory::Promotions),
            ..Mail::new("c@x")
        },
    );
    add(
        &mut batch,
        r.account,
        r.inbox,
        Mail {
            flags: MessageFlags::SEEN,
            ..Mail::new("d@x")
        },
    );
    add(&mut batch, r.account, r.work, Mail::new("e@x"));
    batch.commit().unwrap();
    assert_eq!(ringing(&r.store, r.account, NOW), [plain, primary]);
    assert_eq!(r.store.counted_unread(NOW).unwrap(), 2);
}

#[test]
fn bells_turn_folders_and_tabs_on_and_off() {
    let mut r = rig();
    let mut batch = r.store.mail_batch().unwrap();
    let plain = add(&mut batch, r.account, r.inbox, Mail::new("a@x"));
    let promo = add(
        &mut batch,
        r.account,
        r.inbox,
        Mail {
            category: Some(MailCategory::Promotions),
            ..Mail::new("c@x")
        },
    );
    let work = add(&mut batch, r.account, r.work, Mail::new("e@x"));
    batch.set_bell(r.work, None, Bell::ON).unwrap();
    batch
        .set_bell(
            r.inbox,
            Some(MailCategory::Promotions),
            Bell {
                notify: false,
                count: true,
            },
        )
        .unwrap();
    batch
        .set_bell(r.inbox, Some(MailCategory::Primary), Bell::OFF)
        .unwrap();
    batch.commit().unwrap();
    assert_eq!(ringing(&r.store, r.account, NOW), [work]);
    assert_eq!(r.store.counted_unread(NOW).unwrap(), 2);
    let _ = (plain, promo);

    // Back to the default: nothing is kept.
    let mut batch = r.store.mail_batch().unwrap();
    batch
        .set_bell(r.inbox, Some(MailCategory::Primary), Bell::ON)
        .unwrap();
    batch.set_bell(r.work, None, Bell::OFF).unwrap();
    batch.commit().unwrap();
    assert_eq!(r.store.folder_bells().unwrap().len(), 1);
    assert_eq!(ringing(&r.store, r.account, NOW), [plain]);
}

#[test]
fn mutes_silence_accounts_folders_conversations_and_senders() {
    let mut r = rig();
    let mut batch = r.store.mail_batch().unwrap();
    let first = add(&mut batch, r.account, r.inbox, Mail::new("a@x"));
    let reply = add(
        &mut batch,
        r.account,
        r.inbox,
        Mail {
            reply_to: Some("a@x"),
            from: "bob@example.org",
            ..Mail::new("b@x")
        },
    );
    let other = add(
        &mut batch,
        r.account,
        r.inbox,
        Mail {
            from: "news@shop.example",
            ..Mail::new("c@x")
        },
    );
    batch.commit().unwrap();
    let thread = r.store.messages_by_id(&[first]).unwrap()[0]
        .thread_id
        .unwrap();
    assert_eq!(ringing(&r.store, r.account, NOW), [first, reply, other]);

    let mut batch = r.store.mail_batch().unwrap();
    batch
        .mute(&MuteTarget::Thread(thread), "Hello", None, false, NOW)
        .unwrap();
    batch
        .mute(
            &MuteTarget::Sender("News@Shop.example ".into()),
            "Shop",
            Some(NOW + 3600),
            false,
            NOW,
        )
        .unwrap();
    batch.commit().unwrap();
    assert!(ringing(&r.store, r.account, NOW).is_empty());
    assert_eq!(r.store.counted_unread(NOW).unwrap(), 0);
    assert_eq!(r.store.muted_threads(NOW).unwrap(), HashSet::from([thread]));
    assert_eq!(r.store.next_mute_end(NOW).unwrap(), Some(NOW + 3600));
    assert_eq!(r.store.still_ringing(&[first, other], NOW).unwrap(), []);

    // The sender's mute ends; the conversation's lasts.
    assert_eq!(ringing(&r.store, r.account, NOW + 3600), [other]);
    let mut batch = r.store.mail_batch().unwrap();
    assert_eq!(batch.drop_ended_mutes(NOW + 3600).unwrap(), 1);
    assert!(batch.unmute(&MuteTarget::Thread(thread)).unwrap());
    batch
        .mute(&MuteTarget::Account(r.account), "", None, false, NOW)
        .unwrap();
    batch.commit().unwrap();
    assert!(ringing(&r.store, r.account, NOW).is_empty());
    let mutes = r.store.mutes(NOW).unwrap();
    assert_eq!(mutes.len(), 1);
    assert_eq!(mutes[0].target, MuteTarget::Account(r.account));

    let mut batch = r.store.mail_batch().unwrap();
    batch.unmute(&MuteTarget::Account(r.account)).unwrap();
    batch
        .mute(
            &MuteTarget::Folder(r.inbox),
            "Inbox",
            Some(NOW + 60),
            false,
            NOW,
        )
        .unwrap();
    batch.commit().unwrap();
    assert!(ringing(&r.store, r.account, NOW).is_empty());
    assert_eq!(ringing(&r.store, r.account, NOW + 60).len(), 3);
}

#[test]
fn the_services_mute_flag_mutes_and_unmutes_conversations() {
    let mut r = rig();
    let mut batch = r.store.mail_batch().unwrap();
    let message = add(
        &mut batch,
        r.account,
        r.inbox,
        Mail {
            flags: MessageFlags::MUTED,
            ..Mail::new("a@x")
        },
    );
    assert!(batch.follow_server_mutes(NOW).unwrap());
    assert!(!batch.follow_server_mutes(NOW).unwrap());
    batch.commit().unwrap();
    let thread = r.store.messages_by_id(&[message]).unwrap()[0]
        .thread_id
        .unwrap();
    let mute = r
        .store
        .mute_of(&MuteTarget::Thread(thread), NOW)
        .unwrap()
        .unwrap();
    assert!(mute.server);
    assert!(ringing(&r.store, r.account, NOW).is_empty());

    // Unmuted at the service.
    r.store
        .mail_batch()
        .map(|mut batch| {
            batch
                .set_message_flags(message, MessageFlags::empty())
                .unwrap();
            assert!(batch.follow_server_mutes(NOW).unwrap());
            batch.commit().unwrap();
        })
        .unwrap();
    assert!(r.store.mutes(NOW).unwrap().is_empty());

    // Katna's own mute of a conversation the service cannot mute stays.
    let mut batch = r.store.mail_batch().unwrap();
    batch
        .mute(&MuteTarget::Thread(thread), "Hello", None, false, NOW)
        .unwrap();
    assert!(!batch.follow_server_mutes(NOW).unwrap());
    batch.commit().unwrap();
    assert_eq!(r.store.mutes(NOW).unwrap().len(), 1);
}
