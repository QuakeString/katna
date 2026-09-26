// SPDX-License-Identifier: GPL-3.0-or-later

//! The account worker against an in-memory mail server.

mod common;

use std::time::Duration;

use async_channel::Receiver;
use async_io::Timer;
use common::FakeServer;
use futures_lite::FutureExt;
use katna_sync::worker::{self, Event, Handle, WorkerConfig};

fn config() -> WorkerConfig {
    WorkerConfig {
        idle_timeout: Duration::from_secs(10),
        full_sync_interval: Duration::from_secs(60),
        retry_min: Duration::from_millis(20),
        retry_max: Duration::from_millis(80),
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
    async fn next(&self) -> Event {
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
