// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna Notes in an account's Notes folder, against an in-memory server.

mod common;

use common::{FakeServer, store as setup};
use katna_core::AccountId;
use katna_store::{Note, Store};
use katna_sync::connection::{self, Connection};
use katna_sync::notes::{NOTES_FOLDER, NotesSynced, parse_note, sync_notes};

fn sync(server: &FakeServer, store: &mut Store, account: AccountId) -> NotesSynced {
    let (conn, task): (Connection, _) = connection::spawn(server.connection());
    let task = smol::spawn(task);
    let done = smol::block_on(sync_notes(&conn, store, account, "alice@example.org")).unwrap();
    drop(conn);
    smol::block_on(task);
    done
}

fn note(account: AccountId, title: &str) -> Note {
    Note {
        account_id: Some(account.0),
        title: title.to_owned(),
        body: "☐ milk".to_owned(),
        color: 4,
        ..Note::default()
    }
}

#[test]
fn without_notes_the_folder_is_not_made() {
    let server = FakeServer::default();
    server.create("INBOX", 1);
    let (_tmp, mut store, account) = setup();
    assert_eq!(sync(&server, &mut store, account), NotesSynced::default());
    assert!(!server.state().folders.contains_key(NOTES_FOLDER));
}

#[test]
fn a_note_goes_up_and_comes_back_with_its_uid() {
    let server = FakeServer::default();
    server.create("INBOX", 1);
    let (_tmp, mut store, account) = setup();
    let id = store.save_note(&note(account, "Groceries")).unwrap();
    let done = sync(&server, &mut store, account);
    assert_eq!(done.uploaded, 1);
    assert_eq!(done.downloaded, 0);
    let uids = server.uids(NOTES_FOLDER);
    assert_eq!(uids.len(), 1);
    let saved = store.note(id).unwrap().unwrap();
    assert_eq!(saved.server_uid, Some(i64::from(uids[0])));
    assert!(!saved.dirty);
    assert_eq!(store.account_notes(account.0).unwrap().len(), 1);

    // Changed here: the copy is replaced.
    let mut changed = saved.clone();
    changed.title = "Groceries for Sunday".to_owned();
    store.save_note(&changed).unwrap();
    sync(&server, &mut store, account);
    let uids = server.uids(NOTES_FOLDER);
    assert_eq!(uids.len(), 1);
    let raw = {
        let state = server.state();
        let message = &state.folders[NOTES_FOLDER].messages[&uids[0]];
        [message.header.clone(), message.body.clone()].concat()
    };
    let remote = parse_note(&raw).unwrap();
    assert_eq!(remote.title, "Groceries for Sunday");
    assert_eq!(remote.color, 4);

    // In Trash here: gone there.
    store.trash_notes(&[id], true).unwrap();
    let done = sync(&server, &mut store, account);
    assert_eq!(done.deleted_there, 1);
    assert!(server.uids(NOTES_FOLDER).is_empty());
    assert!(store.note(id).unwrap().is_some(), "still in Trash here");
}

#[test]
fn notes_written_and_deleted_on_a_phone_follow() {
    let server = FakeServer::default();
    server.create("INBOX", 1);
    server.create(NOTES_FOLDER, 1);
    let uid = server.deliver_header(
        NOTES_FOLDER,
        "Subject: Shopping\r\nX-Uniform-Type-Identifier: com.apple.mail-note\r\n\
         X-Universally-Unique-Identifier: 12345678-AAAA-4BBB-8CCC-1234567890AB\r\n\
         Date: Mon, 28 Sep 2026 10:00:00 +0000\r\nContent-Type: text/html; charset=utf-8\r\n\r\n",
        "<html><body><div>Shopping</div><div>Milk</div></body></html>\r\n",
    );
    server.deliver(NOTES_FOLDER, "not a note");
    let (_tmp, mut store, account) = setup();
    let done = sync(&server, &mut store, account);
    assert_eq!(done.downloaded, 1);
    let notes = store.account_notes(account.0).unwrap();
    assert_eq!(notes.len(), 1);
    assert_eq!(
        (notes[0].title.as_str(), notes[0].body.as_str()),
        ("Shopping", "Milk")
    );

    server.expunge(NOTES_FOLDER, uid);
    let done = sync(&server, &mut store, account);
    assert_eq!(done.deleted_here, 1);
    assert!(store.account_notes(account.0).unwrap().is_empty());
    assert_eq!(
        server.uids(NOTES_FOLDER).len(),
        1,
        "other mail is left alone"
    );
}
