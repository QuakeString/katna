// SPDX-License-Identifier: GPL-3.0-or-later

//! Renaming and deleting folders against an in-memory mail server.

mod common;

use common::{FakeServer, store as setup};
use katna_core::AccountId;
use katna_store::{FolderId, MessageFlags, MessageId, Store};
use katna_sync::{
    engine,
    folders::{self, FolderError},
    ops,
};

fn sync(server: &FakeServer, store: &mut Store, account: AccountId) -> Vec<engine::FolderReport> {
    let mut conn = server.connection();
    smol::block_on(engine::sync_account(&mut conn, store, account)).unwrap()
}

fn rename(
    server: &FakeServer,
    store: &mut Store,
    folder: FolderId,
    name: &str,
) -> Result<String, FolderError> {
    let mut conn = server.connection();
    smol::block_on(folders::rename_folder(&mut conn, store, folder, name))
}

fn delete(server: &FakeServer, store: &mut Store, folder: FolderId) -> Result<u32, FolderError> {
    let mut conn = server.connection();
    smol::block_on(folders::delete_folder(&mut conn, store, folder))
}

fn folder(store: &Store, account: AccountId, path: &str) -> FolderId {
    store
        .folders(account)
        .unwrap()
        .into_iter()
        .find(|f| f.path == path)
        .unwrap_or_else(|| panic!("no folder {path}"))
        .id
}

fn paths(store: &Store, account: AccountId) -> Vec<String> {
    store
        .folders(account)
        .unwrap()
        .into_iter()
        .map(|f| f.path)
        .collect()
}

fn server_folders(server: &FakeServer) -> Vec<String> {
    server.state().folders.keys().cloned().collect()
}

fn messages(store: &Store, folder: FolderId) -> Vec<MessageId> {
    store
        .messages_in_folder(folder)
        .unwrap()
        .iter()
        .map(|m| m.id)
        .collect()
}

/// INBOX, Trash, Work (one message) and Work/2026 (two), synced.
fn setup_synced() -> (tempfile::TempDir, Store, AccountId, FakeServer) {
    let (tmp, mut store, account) = setup();
    let server = FakeServer::default();
    for name in ["INBOX", "Trash", "Work", "Work/2026"] {
        server.create(name, 1);
    }
    server.deliver("INBOX", "hello");
    server.deliver("Work", "plan");
    server.deliver("Work/2026", "budget");
    server.deliver("Work/2026", "review");
    sync(&server, &mut store, account);
    (tmp, store, account, server)
}

#[test]
fn a_rename_keeps_the_parent_and_takes_the_folders_inside_along() {
    let (_tmp, mut store, account, server) = setup_synced();
    let work = folder(&store, account, "Work");
    let year = folder(&store, account, "Work/2026");
    let kept = messages(&store, year);

    server.clear_log();
    assert_eq!(rename(&server, &mut store, work, "Jobs").unwrap(), "Jobs");
    assert!(server.log().contains(&"RENAME Work Jobs".to_owned()));
    assert_eq!(
        server_folders(&server),
        ["INBOX", "Jobs", "Jobs/2026", "Trash"]
    );
    assert_eq!(
        paths(&store, account),
        ["INBOX", "Jobs", "Jobs/2026", "Trash"]
    );
    assert_eq!(folder(&store, account, "Jobs/2026"), year, "same folder");
    assert_eq!(messages(&store, year), kept, "same messages");

    // Only the last part changes.
    assert_eq!(
        rename(&server, &mut store, year, "2027").unwrap(),
        "Jobs/2027"
    );
    assert_eq!(
        server_folders(&server),
        ["INBOX", "Jobs", "Jobs/2027", "Trash"]
    );

    // The next sync finds nothing new or gone.
    for report in sync(&server, &mut store, account) {
        assert_eq!((report.added, report.removed), (0, 0), "{report:?}");
    }
    assert_eq!(messages(&store, year), kept);
}

#[test]
fn renames_that_cannot_be() {
    let (_tmp, mut store, account, server) = setup_synced();
    let (inbox, trash, work) = (
        folder(&store, account, "INBOX"),
        folder(&store, account, "Trash"),
        folder(&store, account, "Work"),
    );
    server.clear_log();
    for special in [inbox, trash] {
        assert!(matches!(
            rename(&server, &mut store, special, "Other"),
            Err(FolderError::Invalid(_))
        ));
    }
    assert!(server.log().is_empty(), "refused before asking the server");
    assert!(matches!(
        rename(&server, &mut store, work, "a/b"),
        Err(FolderError::Invalid(_))
    ));
    assert!(matches!(
        rename(&server, &mut store, work, "inbox"),
        Err(FolderError::Invalid(_))
    ));
    assert!(matches!(
        rename(&server, &mut store, FolderId(999), "x"),
        Err(FolderError::UnknownFolder(999))
    ));

    // A change on its way to the server still needs the old name.
    let id = messages(&store, work)[0];
    ops::set_flags(&mut store, &[id], MessageFlags::SEEN, MessageFlags::empty()).unwrap();
    assert!(matches!(
        rename(&server, &mut store, work, "Jobs"),
        Err(FolderError::Failed(_))
    ));

    // Offline: the server cannot be asked.
    server.break_connections();
    let mut conn = server.connection();
    server.break_connections();
    let offline = smol::block_on(folders::rename_folder(&mut conn, &mut store, work, "Jobs"));
    assert!(matches!(offline, Err(FolderError::Failed(_))));
    assert_eq!(
        paths(&store, account),
        ["INBOX", "Trash", "Work", "Work/2026"]
    );
}

#[test]
fn a_deleted_folder_leaves_its_mail_in_the_trash() {
    let (_tmp, mut store, account, server) = setup_synced();
    let (trash, work) = (
        folder(&store, account, "Trash"),
        folder(&store, account, "Work"),
    );
    let year = folder(&store, account, "Work/2026");
    let mut moving = messages(&store, work);
    moving.extend(messages(&store, year));

    server.clear_log();
    assert_eq!(delete(&server, &mut store, work).unwrap(), 3);
    let log = server.log();
    let at = |command: &str| log.iter().position(|c| c == command).unwrap();
    assert!(at("DELETE Work/2026") < at("DELETE Work"), "deepest first");
    assert!(at("MOVE [1] Trash") < at("DELETE Work/2026"));
    assert_eq!(server_folders(&server), ["INBOX", "Trash"]);
    assert_eq!(server.uids("Trash"), [1, 2, 3]);

    assert_eq!(paths(&store, account), ["INBOX", "Trash"]);
    let mut in_trash = messages(&store, trash);
    in_trash.sort();
    moving.sort();
    assert_eq!(in_trash, moving, "the same messages, now in the Trash");
    assert_eq!(store.folder_uids(trash).unwrap(), [1, 2, 3]);
    for report in sync(&server, &mut store, account) {
        assert_eq!((report.added, report.removed), (0, 0), "{report:?}");
    }
}

#[test]
fn without_a_trash_a_deleted_folder_takes_its_mail_along() {
    let (_tmp, mut store, account) = setup();
    let server = FakeServer::default();
    for name in ["INBOX", "Old"] {
        server.create(name, 1);
    }
    server.deliver("Old", "gone");
    sync(&server, &mut store, account);
    let old = folder(&store, account, "Old");

    assert_eq!(delete(&server, &mut store, old).unwrap(), 0);
    assert_eq!(server_folders(&server), ["INBOX"]);
    assert_eq!(paths(&store, account), ["INBOX"]);
    assert_eq!(
        store.latest_message(account).unwrap(),
        MessageId(0),
        "no message left"
    );
}

#[test]
fn special_folders_are_not_deleted() {
    let (_tmp, mut store, account) = setup();
    let server = FakeServer::default();
    // Not marked, but named like special folders where those live.
    for name in [
        "INBOX",
        "INBOX/Sent Items",
        "Junk",
        "Stuff",
        "Stuff/Archive",
    ] {
        server.create(name, 1);
    }
    sync(&server, &mut store, account);
    for path in ["INBOX", "INBOX/Sent Items", "Junk"] {
        let id = folder(&store, account, path);
        assert!(
            matches!(
                delete(&server, &mut store, id),
                Err(FolderError::Invalid(_))
            ),
            "{path}"
        );
    }
    // Deeper down, the name is only a name.
    let stuff = folder(&store, account, "Stuff");
    assert_eq!(delete(&server, &mut store, stuff).unwrap(), 0);
    assert_eq!(
        server_folders(&server),
        ["INBOX", "INBOX/Sent Items", "Junk"]
    );
}

#[test]
fn deleting_a_gmail_label_keeps_the_mail() {
    let (_tmp, mut store, account) = setup();
    let server = FakeServer::default();
    server.set_gmail(true);
    for name in [
        "INBOX",
        "[Gmail]/All Mail",
        "Trash",
        "Receipts",
        "Receipts/2026",
    ] {
        server.create(name, 1);
    }
    let a = server.deliver("[Gmail]/All Mail", "a");
    server.label("[Gmail]/All Mail", a, 101, "Receipts");
    server.label("[Gmail]/All Mail", a, 101, "INBOX");
    let b = server.deliver("[Gmail]/All Mail", "b");
    server.label("[Gmail]/All Mail", b, 102, "Receipts/2026");
    sync(&server, &mut store, account);
    let (all, inbox, receipts) = (
        folder(&store, account, "[Gmail]/All Mail"),
        folder(&store, account, "INBOX"),
        folder(&store, account, "Receipts"),
    );
    assert_eq!(messages(&store, all).len(), 2);

    server.clear_log();
    assert_eq!(delete(&server, &mut store, receipts).unwrap(), 0);
    assert!(
        !server.log().iter().any(|c| c.starts_with("MOVE")),
        "nothing goes to the Trash"
    );
    assert_eq!(
        server_folders(&server),
        ["INBOX", "Trash", "[Gmail]/All Mail"]
    );
    assert_eq!(
        paths(&store, account),
        ["INBOX", "Trash", "[Gmail]/All Mail"]
    );
    assert_eq!(messages(&store, all).len(), 2, "both stay in All Mail");
    assert_eq!(
        messages(&store, inbox).len(),
        1,
        "and in their other labels"
    );
}
