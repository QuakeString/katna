// SPDX-License-Identifier: GPL-3.0-or-later

//! The operation queue against an in-memory mail server.

mod common;

use common::{FakeServer, store as setup};
use katna_core::AccountId;
use katna_store::{FolderId, MessageFlags, MessageId, Store};
use katna_sync::{
    engine,
    ops::{self, ChangeError, RETRY_AFTER, ReplayReport},
};

const NOW: i64 = 1_790_416_800;

fn sync(server: &FakeServer, store: &mut Store, account: AccountId) -> Vec<engine::FolderReport> {
    let mut conn = server.connection();
    smol::block_on(engine::sync_account(&mut conn, store, account)).unwrap()
}

fn replay(server: &FakeServer, store: &mut Store, account: AccountId, now: i64) -> ReplayReport {
    let mut conn = server.connection();
    smol::block_on(ops::replay(&mut conn, store, account, now)).unwrap()
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

fn message_at(store: &Store, folder: FolderId, index: usize) -> MessageId {
    store.messages_in_folder(folder).unwrap()[index].id
}

fn flags_of(store: &Store, id: MessageId) -> MessageFlags {
    store.messages_by_id(&[id]).unwrap()[0].flags
}

/// A server with INBOX (two messages), Archive and Trash, synced.
fn setup_synced() -> (tempfile::TempDir, Store, AccountId, FakeServer) {
    let (tmp, mut store, account) = setup();
    let server = FakeServer::default();
    for name in ["INBOX", "Archive", "Trash"] {
        server.create(name, 1);
    }
    server.deliver("INBOX", "one");
    server.deliver("INBOX", "two");
    sync(&server, &mut store, account);
    (tmp, store, account, server)
}

#[test]
fn flags_change_locally_then_on_the_server() {
    let (_tmp, mut store, account, server) = setup_synced();
    let inbox = folder(&store, account, "INBOX");
    let id = message_at(&store, inbox, 0);

    let touched = ops::set_flags(
        &mut store,
        &[id],
        MessageFlags::SEEN | MessageFlags::FLAGGED,
        MessageFlags::empty(),
    )
    .unwrap();
    assert_eq!(touched, [account]);
    assert_eq!(
        flags_of(&store, id),
        MessageFlags::SEEN | MessageFlags::FLAGGED
    );
    assert!(!server.flags("INBOX", 1).seen, "nothing sent yet");

    server.clear_log();
    let report = replay(&server, &mut store, account, NOW);
    assert_eq!(report.done, 1);
    assert_eq!(
        server.log(),
        ["SELECT INBOX", "STORE [1] +\\Seen \\Flagged"]
    );
    let flags = server.flags("INBOX", 1);
    assert!(flags.seen && flags.flagged);
    assert!(store.due_ops(account, NOW, 10).unwrap().is_empty());

    // The server's echo of our own change changes nothing locally.
    let reports = sync(&server, &mut store, account);
    let inbox_report = reports.iter().find(|r| r.path == "INBOX").unwrap();
    assert_eq!(inbox_report.flags_changed, 0);

    // Removing only sends what changes; setting it again queues nothing.
    ops::set_flags(&mut store, &[id], MessageFlags::SEEN, MessageFlags::FLAGGED).unwrap();
    server.clear_log();
    replay(&server, &mut store, account, NOW);
    assert_eq!(server.log(), ["SELECT INBOX", "STORE [1] -\\Flagged"]);
    assert!(!server.flags("INBOX", 1).flagged);
    ops::set_flags(&mut store, &[id], MessageFlags::SEEN, MessageFlags::empty()).unwrap();
    assert!(store.due_ops(account, NOW, 10).unwrap().is_empty());
}

#[test]
fn a_move_is_undone_before_the_server_has_it() {
    let (_tmp, mut store, account, server) = setup_synced();
    let (inbox, archive) = (
        folder(&store, account, "INBOX"),
        folder(&store, account, "Archive"),
    );
    let id = message_at(&store, inbox, 0);

    // Archived, then moved back at once (Undo), before any replay.
    ops::archive_messages(&mut store, &[id]).unwrap();
    ops::move_messages(&mut store, &[id], inbox).unwrap();
    assert_eq!(store.messages_in_folder(archive).unwrap().len(), 0);
    assert!(
        store
            .messages_in_folder(inbox)
            .unwrap()
            .iter()
            .any(|m| m.id == id)
    );

    // The second move takes the UID the first one got on the server.
    let report = replay(&server, &mut store, account, NOW);
    assert_eq!((report.done, report.failed), (2, 0));
    assert!(server.uids("Archive").is_empty());
    assert_eq!(server.uids("INBOX").len(), 2);
    assert!(store.due_ops(account, NOW, 10).unwrap().is_empty());
    let location = store
        .locations(id)
        .unwrap()
        .into_iter()
        .find(|l| l.folder == inbox)
        .unwrap();
    assert!(location.uid.is_some());
}

#[test]
fn moves_keep_the_message_when_the_server_reports_uids() {
    let (_tmp, mut store, account, server) = setup_synced();
    let (inbox, archive) = (
        folder(&store, account, "INBOX"),
        folder(&store, account, "Archive"),
    );
    let id = message_at(&store, inbox, 0);

    ops::archive_messages(&mut store, &[id]).unwrap();
    assert_eq!(store.messages_in_folder(inbox).unwrap().len(), 1);
    assert_eq!(store.messages_in_folder(archive).unwrap()[0].id, id);

    let report = replay(&server, &mut store, account, NOW);
    assert_eq!((report.done, report.resync.len()), (1, 0));
    assert_eq!(server.uids("INBOX"), [2]);
    assert_eq!(server.uids("Archive"), [1]);
    assert_eq!(store.folder_uids(archive).unwrap(), [1]);

    // Nothing to add or remove afterwards; the same message row stays.
    for report in sync(&server, &mut store, account) {
        assert_eq!((report.added, report.removed), (0, 0), "{report:?}");
    }
    assert_eq!(store.messages_in_folder(archive).unwrap()[0].id, id);

    // Flags set while the move is pending follow the message.
    let other = message_at(&store, inbox, 0);
    ops::archive_messages(&mut store, &[other]).unwrap();
    ops::set_flags(
        &mut store,
        &[other],
        MessageFlags::SEEN,
        MessageFlags::empty(),
    )
    .unwrap();
    let report = replay(&server, &mut store, account, NOW);
    assert_eq!(report.done, 2);
    assert!(server.flags("Archive", 2).seen);
}

#[test]
fn moves_without_uidplus_sync_the_target_again() {
    let (_tmp, mut store, account, server) = setup_synced();
    server.state().no_uidplus = true;
    let (inbox, archive) = (
        folder(&store, account, "INBOX"),
        folder(&store, account, "Archive"),
    );
    let id = message_at(&store, inbox, 0);
    ops::move_messages(&mut store, &[id], archive).unwrap();

    let report = replay(&server, &mut store, account, NOW);
    assert_eq!(report.resync, [(archive, "Archive".to_owned())]);
    assert!(store.messages_in_folder(archive).unwrap().is_empty());
    let reports = sync(&server, &mut store, account);
    let archived = reports.iter().find(|r| r.path == "Archive").unwrap();
    assert_eq!(archived.added, 1);
}

#[test]
fn delete_goes_to_trash_then_away() {
    let (_tmp, mut store, account, server) = setup_synced();
    let (inbox, trash) = (
        folder(&store, account, "INBOX"),
        folder(&store, account, "Trash"),
    );
    let id = message_at(&store, inbox, 0);

    ops::delete_messages(&mut store, &[id]).unwrap();
    replay(&server, &mut store, account, NOW);
    assert_eq!(server.uids("Trash"), [1]);
    assert_eq!(store.messages_in_folder(trash).unwrap()[0].id, id);

    ops::delete_messages(&mut store, &[id]).unwrap();
    assert!(
        store.messages_by_id(&[id]).unwrap().is_empty(),
        "gone at once"
    );
    server.clear_log();
    replay(&server, &mut store, account, NOW);
    assert_eq!(server.log(), ["SELECT Trash", "EXPUNGE [1]"]);
    assert!(server.uids("Trash").is_empty());
}

#[test]
fn refused_changes_are_retried_then_undone() {
    let (_tmp, mut store, account, server) = setup_synced();
    let (inbox, archive) = (
        folder(&store, account, "INBOX"),
        folder(&store, account, "Archive"),
    );
    let (first, second) = (message_at(&store, inbox, 0), message_at(&store, inbox, 1));
    server.state().refuse_changes = true;
    ops::set_flags(
        &mut store,
        &[first],
        MessageFlags::SEEN,
        MessageFlags::empty(),
    )
    .unwrap();
    ops::move_messages(&mut store, &[second], archive).unwrap();

    let report = replay(&server, &mut store, account, NOW);
    assert_eq!(report.retried, 2);
    assert_eq!(
        replay(&server, &mut store, account, NOW),
        ReplayReport::default()
    );
    assert_eq!(
        replay(&server, &mut store, account, NOW + RETRY_AFTER).retried,
        2
    );
    let report = replay(&server, &mut store, account, NOW + 2 * RETRY_AFTER);
    assert_eq!(report.failed, 2);
    assert_eq!(store.next_op_due(account).unwrap(), None);

    // The move is undone at once; the flag with the next sync.
    assert_eq!(store.folder_uids(inbox).unwrap(), [1, 2]);
    assert!(store.messages_in_folder(archive).unwrap().is_empty());
    sync(&server, &mut store, account);
    assert_eq!(flags_of(&store, first), MessageFlags::empty());
}

#[test]
fn pins_are_local_and_capped() {
    let (_tmp, mut store, account) = setup();
    let server = FakeServer::default();
    server.create("INBOX", 1);
    for n in 0..=ops::MAX_PINS {
        server.deliver("INBOX", &format!("subject {n}"));
    }
    sync(&server, &mut store, account);
    let inbox = folder(&store, account, "INBOX");
    let ids: Vec<MessageId> = (0..=ops::MAX_PINS)
        .map(|n| message_at(&store, inbox, n))
        .collect();

    let (ten, eleventh) = ids.split_at(ops::MAX_PINS);
    assert_eq!(
        ops::set_pinned(&mut store, ten, true, NOW).unwrap(),
        [account]
    );
    assert_eq!(store.pinned().unwrap().len(), ops::MAX_PINS);
    // Pinning again is fine; an 11th conversation is not.
    assert!(
        ops::set_pinned(&mut store, &ten[..1], true, NOW)
            .unwrap()
            .is_empty()
    );
    assert!(matches!(
        ops::set_pinned(&mut store, eleventh, true, NOW),
        Err(ChangeError::NotPossible(_))
    ));
    assert_eq!(store.pinned().unwrap().len(), ops::MAX_PINS);

    // Unpinning one makes room. Nothing goes to the server.
    ops::set_pinned(&mut store, &ten[..1], false, NOW).unwrap();
    ops::set_pinned(&mut store, eleventh, true, NOW + 1).unwrap();
    let pinned = store.pinned().unwrap();
    assert_eq!(pinned[0].message, eleventh[0], "newest pin first");
    assert!(!pinned.iter().any(|p| p.message == ten[0]));
    assert!(store.due_ops(account, NOW, 10).unwrap().is_empty());
    assert!(matches!(
        ops::set_pinned(&mut store, &[MessageId(999)], true, NOW),
        Err(ChangeError::UnknownMessage(999))
    ));
}

#[test]
fn impossible_changes_are_errors() {
    let (_tmp, mut store, account) = setup();
    let server = FakeServer::default();
    server.create("INBOX", 1);
    server.deliver("INBOX", "one");
    sync(&server, &mut store, account);
    let id = message_at(&store, folder(&store, account, "INBOX"), 0);

    assert!(matches!(
        ops::archive_messages(&mut store, &[id]),
        Err(ChangeError::NotPossible(_))
    ));
    assert!(matches!(
        ops::set_flags(
            &mut store,
            &[MessageId(99)],
            MessageFlags::SEEN,
            MessageFlags::empty()
        ),
        Err(ChangeError::UnknownMessage(99))
    ));
    assert!(matches!(
        ops::move_messages(&mut store, &[id], FolderId(99)),
        Err(ChangeError::UnknownFolder(99))
    ));
    // No Trash: delete for good.
    ops::delete_messages(&mut store, &[id]).unwrap();
    replay(&server, &mut store, account, NOW);
    assert!(server.uids("INBOX").is_empty());
}
