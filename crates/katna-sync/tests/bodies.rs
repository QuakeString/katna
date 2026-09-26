// SPDX-License-Identifier: GPL-3.0-or-later

//! Sync level 3 against an in-memory mail server.

mod common;

use common::{FakeServer, store as setup};
use katna_core::AccountId;
use katna_store::{FolderId, MessageId, Store};
use katna_sync::{
    Error,
    bodies::{self, BODY_CHUNK, OfflineWindow},
    engine,
};

/// The `Date` of every message the fake server delivers.
const DELIVERED: i64 = 1_790_416_800;
const DAY: i64 = 86_400;

fn sync(server: &FakeServer, store: &mut Store, account: AccountId) -> FolderId {
    let mut conn = server.connection();
    smol::block_on(engine::sync_account(&mut conn, store, account)).unwrap();
    store
        .folders(account)
        .unwrap()
        .into_iter()
        .find(|f| f.path == "INBOX")
        .unwrap()
        .id
}

fn download(
    server: &FakeServer,
    store: &mut Store,
    inbox: FolderId,
    window: &OfflineWindow,
    now: i64,
) -> usize {
    server.clear_log();
    let mut conn = server.connection();
    smol::block_on(bodies::download_bodies(
        &mut conn, store, inbox, "INBOX", window, now,
    ))
    .unwrap()
}

fn thirty_days() -> OfflineWindow {
    OfflineWindow {
        days: Some(30),
        max_size: 100_000,
    }
}

#[test]
fn downloads_the_window_once() {
    let (_tmp, mut store, account) = setup();
    let server = FakeServer::default();
    server.create("INBOX", 1);
    for i in 0..3 {
        server.deliver("INBOX", &format!("hello-{i}"));
    }
    let inbox = sync(&server, &mut store, account);

    assert_eq!(
        download(&server, &mut store, inbox, &thirty_days(), DELIVERED + DAY),
        3
    );
    assert_eq!(
        server.log(),
        ["SELECT INBOX", "BODIES [3, 2, 1]"],
        "newest first"
    );
    let messages = store.messages_in_folder(inbox).unwrap();
    for message in &messages {
        let raw = store
            .blobs()
            .get(&message.blob_hash.unwrap())
            .unwrap()
            .unwrap();
        assert!(raw.ends_with(format!("Body of {}.\r\n", message.subject).as_bytes()));
        assert_eq!(
            message.snippet.as_deref(),
            Some(format!("Body of {}.", message.subject).as_str())
        );
    }

    // Nothing left: not even a SELECT.
    assert_eq!(
        download(&server, &mut store, inbox, &thirty_days(), DELIVERED + DAY),
        0
    );
    assert!(server.log().is_empty(), "{:?}", server.log());
}

#[test]
fn old_and_big_messages_stay_on_the_server() {
    let (_tmp, mut store, account) = setup();
    let server = FakeServer::default();
    server.create("INBOX", 1);
    server.deliver("INBOX", "old");
    let inbox = sync(&server, &mut store, account);

    let later = DELIVERED + 31 * DAY;
    assert_eq!(
        download(&server, &mut store, inbox, &thirty_days(), later),
        0
    );
    let small = OfflineWindow {
        days: None,
        max_size: 999,
    };
    assert_eq!(download(&server, &mut store, inbox, &small, later), 0);
    assert!(server.log().is_empty());

    let everything = OfflineWindow {
        days: None,
        max_size: 1000,
    };
    assert_eq!(download(&server, &mut store, inbox, &everything, later), 1);
}

#[test]
fn big_windows_are_fetched_in_chunks() {
    let (_tmp, mut store, account) = setup();
    let server = FakeServer::default();
    server.create("INBOX", 1);
    let total = BODY_CHUNK as usize * 2 + 3;
    for i in 0..total {
        server.deliver("INBOX", &format!("m{i}"));
    }
    let inbox = sync(&server, &mut store, account);
    assert_eq!(
        download(&server, &mut store, inbox, &thirty_days(), DELIVERED),
        total
    );
    let fetches = server
        .log()
        .iter()
        .filter(|c| c.starts_with("BODIES"))
        .count();
    assert_eq!(fetches, 3);
}

#[test]
fn expunged_messages_do_not_loop() {
    let (_tmp, mut store, account) = setup();
    let server = FakeServer::default();
    server.create("INBOX", 1);
    server.deliver("INBOX", "kept");
    let gone = server.deliver("INBOX", "gone");
    let inbox = sync(&server, &mut store, account);
    server.expunge("INBOX", gone);
    assert_eq!(
        download(&server, &mut store, inbox, &thirty_days(), DELIVERED),
        1
    );
    let bodies = server
        .log()
        .iter()
        .filter(|c| c.starts_with("BODIES"))
        .count();
    assert_eq!(bodies, 1);
}

#[test]
fn fetches_one_message_on_request() {
    let (_tmp, mut store, account) = setup();
    let server = FakeServer::default();
    server.create("INBOX", 1);
    server.create("Archive", 1);
    server.deliver("Archive", "wanted");
    let gone = server.deliver("Archive", "gone");
    sync(&server, &mut store, account);
    let archive = store
        .folders(account)
        .unwrap()
        .into_iter()
        .find(|f| f.path == "Archive")
        .unwrap();
    let messages = store.messages_in_folder(archive.id).unwrap();

    let mut conn = server.connection();
    smol::block_on(bodies::fetch_body(
        &mut conn,
        &mut store,
        account,
        messages[0].id,
    ))
    .unwrap();
    let stored = &store.messages_by_id(&[messages[0].id]).unwrap()[0];
    assert_eq!(stored.snippet.as_deref(), Some("Body of wanted."));

    server.expunge("Archive", gone);
    let err = smol::block_on(bodies::fetch_body(
        &mut conn,
        &mut store,
        account,
        messages[1].id,
    ))
    .unwrap_err();
    assert!(matches!(err, Error::Rejected(_)), "{err:?}");
    let err = smol::block_on(bodies::fetch_body(
        &mut conn,
        &mut store,
        AccountId(account.0 + 1),
        messages[0].id,
    ))
    .unwrap_err();
    assert!(matches!(err, Error::Rejected(_)), "{err:?}");
    let err = smol::block_on(bodies::fetch_body(
        &mut conn,
        &mut store,
        account,
        MessageId(999),
    ))
    .unwrap_err();
    assert!(matches!(err, Error::Rejected(_)), "{err:?}");
}
