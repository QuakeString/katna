// SPDX-License-Identifier: GPL-3.0-or-later

//! The account worker against an in-memory mail server.

mod common;

use std::time::Duration;

use async_channel::Receiver;
use async_io::Timer;
use common::FakeServer;
use futures_lite::FutureExt;
use katna_store::MessageFlags;
use katna_sync::{
    bodies::OfflineWindow,
    ops,
    worker::{self, Event, Handle, WorkerConfig},
};

fn config() -> WorkerConfig {
    WorkerConfig {
        idle_timeout: Duration::from_secs(10),
        full_sync_interval: Duration::from_secs(60),
        retry_min: Duration::from_millis(20),
        retry_max: Duration::from_millis(80),
        offline: OfflineWindow {
            days: None,
            max_size: u64::MAX,
        },
        pop3_interval: Duration::from_secs(60),
    }
}

struct Running {
    events: Receiver<Event>,
    handle: Handle,
    task: smol::Task<()>,
    _tmp: tempfile::TempDir,
}

fn start(server: &FakeServer, config: WorkerConfig) -> Running {
    let (tmp, store, account) = common::store();
    let (events_tx, events) = async_channel::unbounded();
    let (handle, control) = worker::control();
    let task = smol::spawn(worker::run(
        server.clone(),
        store,
        account,
        config,
        events_tx,
        control,
    ));
    Running {
        events,
        handle,
        task,
        _tmp: tmp,
    }
}

impl Running {
    /// Another handle on the worker's store, as the daemon has.
    fn store(&self) -> katna_store::Store {
        katna_store::Store::open(
            &katna_core::Paths::with_root(self._tmp.path()),
            katna_store::Mode::ReadWrite,
        )
        .unwrap()
    }

    /// The next event other than [`Event::BodiesStored`].
    async fn next(&self) -> Event {
        loop {
            match self.next_any().await {
                Event::BodiesStored(_) => {}
                event => return event,
            }
        }
    }

    async fn next_any(&self) -> Event {
        self.events
            .recv()
            .or(async {
                Timer::after(Duration::from_secs(5)).await;
                panic!("no event within 5 s");
            })
            .await
            .unwrap()
    }

    async fn stop(self) {
        drop(self.handle);
        self.task
            .or(async {
                Timer::after(Duration::from_secs(5)).await;
                panic!("worker did not stop");
            })
            .await;
    }
}

fn added(event: &Event) -> usize {
    match event {
        Event::Synced(reports) => reports.iter().map(|r| r.added).sum(),
        other => panic!("expected Synced, got {other:?}"),
    }
}

#[test]
fn syncs_then_picks_up_new_mail_by_push() {
    let server = FakeServer::default();
    server.create("INBOX", 1);
    server.create("Archive", 1);
    server.deliver("INBOX", "one");
    server.deliver("Archive", "two");
    smol::block_on(async {
        let worker = start(&server, config());
        assert!(matches!(worker.next().await, Event::Connected));
        assert_eq!(added(&worker.next().await), 2);

        Timer::after(Duration::from_millis(50)).await;
        server.deliver("INBOX", "three");
        let event = worker.next().await;
        assert_eq!(added(&event), 1, "{event:?}");

        worker.stop().await;
        assert_eq!(server.log().last().map(String::as_str), Some("LOGOUT"));
        assert_eq!(server.state().connects, 1);
    });
}

#[test]
fn reconnects_with_growing_waits() {
    let server = FakeServer::default();
    server.create("INBOX", 1);
    server.state().refuse = 3;
    smol::block_on(async {
        let worker = start(&server, config());
        let mut waits = Vec::new();
        for _ in 0..3 {
            match worker.next().await {
                Event::Disconnected { retry_in, .. } => waits.push(retry_in.as_millis()),
                other => panic!("expected Disconnected, got {other:?}"),
            }
        }
        assert_eq!(waits, [20, 40, 80]);
        assert!(matches!(worker.next().await, Event::Connected));
        assert!(matches!(worker.next().await, Event::Synced(_)));

        // A network change breaks the session; mail arrives meanwhile.
        server.break_connections();
        server.deliver("INBOX", "while away");
        match worker.next().await {
            Event::Disconnected { retry_in, .. } => {
                assert_eq!(retry_in.as_millis(), 20, "a good session resets the wait");
            }
            other => panic!("expected Disconnected, got {other:?}"),
        }
        assert!(matches!(worker.next().await, Event::Connected));
        assert_eq!(added(&worker.next().await), 1);
        worker.stop().await;
    });
}

#[test]
fn wrong_password_is_retried_only_when_asked() {
    let server = FakeServer::default();
    server.create("INBOX", 1);
    server.state().wrong_password = true;
    smol::block_on(async {
        let worker = start(&server, config());
        assert!(matches!(worker.next().await, Event::AuthFailed(_)));
        Timer::after(Duration::from_millis(200)).await;
        assert_eq!(server.state().connects, 1);

        // The user fixed the password.
        server.state().wrong_password = false;
        worker.handle.sync_now();
        assert!(matches!(worker.next().await, Event::Connected));
        assert!(matches!(worker.next().await, Event::Synced(_)));
        worker.stop().await;
    });
}

#[test]
fn sync_now_ends_the_wait() {
    let server = FakeServer::default();
    server.create("INBOX", 1);
    server.create("Archive", 1);
    smol::block_on(async {
        let worker = start(&server, config());
        assert!(matches!(worker.next().await, Event::Connected));
        assert!(matches!(worker.next().await, Event::Synced(_)));

        // New mail in a folder IDLE does not watch shows up on request.
        Timer::after(Duration::from_millis(50)).await;
        server.clear_log();
        server.remove_folder("Archive");
        server.create("Archive", 1);
        server.deliver("Archive", "elsewhere");
        worker.handle.sync_now();
        let event = worker.next().await;
        assert_eq!(added(&event), 1, "{event:?}");
        assert!(server.log().contains(&"LIST".to_owned()));
        worker.stop().await;
    });
}

#[test]
fn sync_now_reconnects_at_once() {
    let server = FakeServer::default();
    server.create("INBOX", 1);
    server.state().refuse = 1;
    let config = WorkerConfig {
        retry_min: Duration::from_secs(60),
        ..config()
    };
    smol::block_on(async {
        let worker = start(&server, config);
        assert!(matches!(worker.next().await, Event::Disconnected { .. }));
        worker.handle.sync_now();
        assert!(matches!(worker.next().await, Event::Connected));
        worker.stop().await;
    });
}

#[test]
fn renews_idle_and_syncs_everything_periodically() {
    let server = FakeServer::default();
    server.create("INBOX", 1);
    let config = WorkerConfig {
        idle_timeout: Duration::from_millis(30),
        full_sync_interval: Duration::from_millis(150),
        ..config()
    };
    smol::block_on(async {
        let worker = start(&server, config);
        assert!(matches!(worker.next().await, Event::Connected));
        assert!(matches!(worker.next().await, Event::Synced(_)));
        // The second full sync.
        assert!(matches!(worker.next().await, Event::Synced(_)));
        worker.stop().await;

        let log = server.log();
        let count = |name: &str| log.iter().filter(|c| c.as_str() == name).count();
        assert!(count("IDLE") >= 4, "{log:?}");
        assert!(count("LIST") >= 2, "{log:?}");
        assert_eq!(server.state().connects, 1);
    });
}

#[test]
fn downloads_bodies_and_fetches_on_request() {
    let server = FakeServer::default();
    server.create("INBOX", 1);
    server.create("Archive", 1);
    server.deliver("INBOX", "one");
    server.deliver("Archive", "two");
    let (tmp, store, account) = common::store();
    let paths = katna_core::Paths::with_root(tmp.path());
    let config = WorkerConfig {
        offline: OfflineWindow {
            days: None,
            // The fake server's messages are 1000 bytes: nothing fits.
            max_size: 10,
        },
        ..config()
    };
    let (events_tx, events) = async_channel::unbounded();
    let (handle, control) = worker::control();
    let task = smol::spawn(worker::run(
        server.clone(),
        store,
        account,
        config,
        events_tx,
        control,
    ));
    let worker = Running {
        events,
        handle,
        task,
        _tmp: tmp,
    };
    smol::block_on(async {
        assert!(matches!(worker.next().await, Event::Connected));
        assert!(matches!(worker.next().await, Event::Synced(_)));

        let reader = katna_store::Store::open(&paths, katna_store::Mode::ReadOnly).unwrap();
        let archive = reader
            .folders(account)
            .unwrap()
            .into_iter()
            .find(|f| f.path == "Archive")
            .unwrap();
        let message = reader.messages_in_folder(archive.id).unwrap()[0].id;
        assert!(
            reader.messages_by_id(&[message]).unwrap()[0]
                .blob_hash
                .is_none()
        );

        Timer::after(Duration::from_millis(50)).await;
        worker.handle.fetch_body(message).await.unwrap();
        assert!(matches!(worker.next_any().await, Event::BodiesStored(1)));
        let stored = &reader.messages_by_id(&[message]).unwrap()[0];
        assert_eq!(stored.snippet.as_deref(), Some("Body of two."));

        // Unknown messages fail without breaking the connection.
        let err = worker
            .handle
            .fetch_body(katna_store::MessageId(999))
            .await
            .unwrap_err();
        assert!(matches!(err, katna_sync::Error::Rejected(_)), "{err:?}");

        // The inbox is watched again afterwards.
        server.deliver("INBOX", "three");
        assert_eq!(added(&worker.next().await), 1);
        assert_eq!(server.state().connects, 1);
        worker.stop().await;
    });
}

#[test]
fn metered_network_waits_with_bodies() {
    let server = FakeServer::default();
    server.create("INBOX", 1);
    server.create("Archive", 1);
    server.deliver("INBOX", "one");
    server.deliver("Archive", "two");
    let (tmp, store, account) = common::store();
    let paths = katna_core::Paths::with_root(tmp.path());
    let (events_tx, events) = async_channel::unbounded();
    let (handle, control) = worker::control();
    handle.set_metered(true);
    let task = smol::spawn(worker::run(
        server.clone(),
        store,
        account,
        config(),
        events_tx,
        control,
    ));
    let worker = Running {
        events,
        handle,
        task,
        _tmp: tmp,
    };
    smol::block_on(async {
        assert!(matches!(worker.next_any().await, Event::Connected));
        assert_eq!(added(&worker.next_any().await), 2);
        let reader = katna_store::Store::open(&paths, katna_store::Mode::ReadOnly).unwrap();
        let bodies = || {
            let ids: Vec<_> = reader
                .folders(account)
                .unwrap()
                .into_iter()
                .flat_map(|f| reader.messages_in_folder(f.id).unwrap())
                .map(|m| m.id)
                .collect();
            reader
                .messages_by_id(&ids)
                .unwrap()
                .iter()
                .filter(|m| m.blob_hash.is_some())
                .count()
        };

        // New mail still arrives, without its body.
        server.deliver("INBOX", "three");
        assert_eq!(added(&worker.next_any().await), 1);
        assert_eq!(bodies(), 0);

        // What the user opens is still fetched.
        let inbox = reader
            .folders(account)
            .unwrap()
            .into_iter()
            .find(|f| f.path == "INBOX")
            .unwrap();
        let message = reader.messages_in_folder(inbox.id).unwrap()[0].id;
        worker.handle.fetch_body(message).await.unwrap();
        assert!(matches!(worker.next_any().await, Event::BodiesStored(1)));
        assert_eq!(bodies(), 1);

        // Off the metered network, the worker catches up at once.
        worker.handle.set_metered(false);
        assert!(matches!(worker.next_any().await, Event::Synced(_)));
        assert!(matches!(worker.next_any().await, Event::BodiesStored(2)));
        assert_eq!(bodies(), 3);
        assert_eq!(server.state().connects, 1);
        worker.stop().await;
    });
}

#[test]
fn sends_changes_when_asked_and_after_reconnecting() {
    let server = FakeServer::default();
    server.create("INBOX", 1);
    server.create("Archive", 1);
    server.deliver("INBOX", "one");
    server.deliver("INBOX", "two");
    smol::block_on(async {
        let worker = start(&server, config());
        assert!(matches!(worker.next().await, Event::Connected));
        assert!(matches!(worker.next().await, Event::Synced(_)));
        let mut store = worker.store();
        let account = store.accounts().unwrap()[0].id;
        let folders = store.folders(account).unwrap();
        let inbox = folders.iter().find(|f| f.path == "INBOX").unwrap().id;
        let archive = folders.iter().find(|f| f.path == "Archive").unwrap().id;
        let messages = store.messages_in_folder(inbox).unwrap();

        // Sent while the worker idles.
        Timer::after(Duration::from_millis(50)).await;
        ops::set_flags(
            &mut store,
            &[messages[0].id],
            MessageFlags::SEEN,
            MessageFlags::empty(),
        )
        .unwrap();
        worker.handle.send_changes();
        match worker.next().await {
            Event::ChangesSent(report) => assert_eq!(report.done, 1),
            other => panic!("{other:?}"),
        }
        assert!(server.flags("INBOX", 1).seen);

        // Queued while offline; asking does not stop the worker, and the
        // change goes out once it is back.
        server.state().refuse = 1;
        server.break_connections();
        assert!(matches!(worker.next().await, Event::Disconnected { .. }));
        ops::move_messages(&mut store, &[messages[1].id], archive).unwrap();
        worker.handle.send_changes();
        assert!(matches!(worker.next().await, Event::Disconnected { .. }));
        assert!(matches!(worker.next().await, Event::Connected));
        match worker.next().await {
            Event::ChangesSent(report) => assert_eq!(report.done, 1),
            other => panic!("{other:?}"),
        }
        assert_eq!(server.uids("Archive"), [1]);
        assert_eq!(server.state().connects, 3);
        worker.stop().await;
    });
}

#[test]
fn refused_changes_wait_until_due() {
    let server = FakeServer::default();
    server.create("INBOX", 1);
    server.deliver("INBOX", "one");
    smol::block_on(async {
        let worker = start(&server, config());
        assert!(matches!(worker.next().await, Event::Connected));
        assert!(matches!(worker.next().await, Event::Synced(_)));
        let mut store = worker.store();
        let account = store.accounts().unwrap()[0].id;
        let inbox = store.folders(account).unwrap()[0].id;
        let message = store.messages_in_folder(inbox).unwrap()[0].id;

        server.state().refuse_changes = true;
        ops::set_flags(
            &mut store,
            &[message],
            MessageFlags::FLAGGED,
            MessageFlags::empty(),
        )
        .unwrap();
        worker.handle.send_changes();
        match worker.next().await {
            Event::ChangesSent(report) => assert_eq!(report.retried, 1),
            other => panic!("{other:?}"),
        }
        // Asking again does not send it before it is due.
        worker.handle.send_changes();
        Timer::after(Duration::from_millis(50)).await;
        assert!(worker.events.is_empty());
        assert!(!server.flags("INBOX", 1).flagged);

        // Make it due now.
        let queued = store.due_ops(account, i64::MAX, 10).unwrap()[0].id;
        let mut batch = store.mail_batch().unwrap();
        batch.retry_op(queued, 0).unwrap();
        batch.commit().unwrap();
        server.state().refuse_changes = false;
        worker.handle.send_changes();
        match worker.next().await {
            Event::ChangesSent(report) => assert_eq!(report.done, 1),
            other => panic!("{other:?}"),
        }
        assert!(server.flags("INBOX", 1).flagged);
        worker.stop().await;
    });
}

#[test]
fn reconnect_drops_the_connection_and_connects_at_once() {
    let server = FakeServer::default();
    server.create("INBOX", 1);
    server.deliver("INBOX", "one");
    let config = WorkerConfig {
        // A plain disconnect would wait a minute; a reconnect must not.
        retry_min: Duration::from_secs(60),
        ..config()
    };
    smol::block_on(async {
        let worker = start(&server, config);
        assert!(matches!(worker.next().await, Event::Connected));
        assert_eq!(added(&worker.next().await), 1);

        // Waiting in IDLE: the old connection is dropped, not logged out.
        Timer::after(Duration::from_millis(50)).await;
        worker.handle.reconnect();
        match worker.next().await {
            Event::Disconnected { retry_in, .. } => assert_eq!(retry_in, Duration::ZERO),
            other => panic!("expected Disconnected, got {other:?}"),
        }
        assert!(matches!(worker.next().await, Event::Connected));
        assert!(matches!(worker.next().await, Event::Synced(_)));
        assert_eq!(server.state().connects, 2);

        worker.stop().await;
    });
}
