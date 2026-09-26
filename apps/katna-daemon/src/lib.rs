// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna background service. See `docs/ARCHITECTURE.md` §9.
//!
//! The daemon is the only process that writes the store and talks to mail
//! servers. It runs one sync worker per account, keeps the search index up
//! to date and serves `in.invenia.katna.Pim1` on the session bus; owning
//! the bus name keeps it to a single instance.

pub mod daemon;
pub mod install;
pub mod secrets;
pub mod service;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use async_channel::{Receiver, Sender};
use katna_core::{AccountId, Paths, ids};
use katna_search::{IndexEvent, Indexer, IndexerOptions, IndexerWaker};
use katna_store::{Mode, Store};
use katna_sync::worker::WorkerConfig;
use zbus::fdo::{RequestNameFlags, RequestNameReply};

use crate::{
    daemon::{Daemon, Notice},
    secrets::Secrets,
    service::PimService,
};

/// Why the daemon could not start.
#[derive(Debug, thiserror::Error)]
pub enum StartError {
    #[error("another katna-daemon is already running")]
    AlreadyRunning,
    #[error("store: {0}")]
    Store(#[from] katna_store::Error),
    #[error("D-Bus: {0}")]
    DBus(#[from] zbus::Error),
    #[error("{0}")]
    Start(#[from] daemon::CommandError),
}

/// A daemon serving on a bus connection.
pub struct Instance {
    pub daemon: Arc<Daemon>,
    connection: zbus::Connection,
    /// `None` if the index could not be opened; mail still syncs.
    indexer: Option<Indexer>,
    backfill: Backfill,
}

/// Threads and classifies mail stored before threading existed, on its own
/// thread with its own store handle, one small transaction at a time.
struct Backfill {
    stop: Arc<AtomicBool>,
    thread: Option<std::thread::JoinHandle<()>>,
}

/// Pause between backfill batches, so sync gets the write lock often.
const BACKFILL_PAUSE: Duration = Duration::from_millis(20);

impl Backfill {
    fn start(paths: &Paths, notices: Sender<Notice>) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let paths = paths.clone();
        let stopped = stop.clone();
        let thread = std::thread::Builder::new()
            .name("katna-backfill".into())
            .spawn(move || {
                if let Err(err) = run_backfill(&paths, &stopped, &notices) {
                    tracing::warn!(%err, "threading old mail stopped");
                }
            });
        let thread = match thread {
            Ok(thread) => Some(thread),
            Err(err) => {
                tracing::warn!(%err, "cannot start threading old mail");
                None
            }
        };
        Self { stop, thread }
    }

    async fn stop(mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(thread) = self.thread.take() {
            smol::unblock(move || {
                let _ = thread.join();
            })
            .await;
        }
    }
}

fn run_backfill(
    paths: &Paths,
    stop: &AtomicBool,
    notices: &Sender<Notice>,
) -> katna_store::Result<()> {
    let mut store = Store::open(paths, Mode::ReadWrite)?;
    let left = store.unthreaded_count()?;
    if left == 0 {
        return Ok(());
    }
    let started = Instant::now();
    tracing::info!(left, "threading old mail");
    let accounts: Vec<AccountId> = store.accounts()?.into_iter().map(|a| a.id).collect();
    let mut last_notice = Instant::now();
    let changed = katna_import::backfill::run(&mut store, katna_import::backfill::BATCH, |_| {
        // Tell the apps now and then, not after every batch.
        if last_notice.elapsed() > Duration::from_secs(2) {
            for &account in &accounts {
                let _ = notices.try_send(Notice::MailChanged(account));
            }
            last_notice = Instant::now();
        }
        std::thread::sleep(BACKFILL_PAUSE);
        !stop.load(Ordering::Relaxed)
    })?;
    for &account in &accounts {
        let _ = notices.try_send(Notice::MailChanged(account));
    }
    tracing::info!(
        changed,
        seconds = started.elapsed().as_secs_f32(),
        "threaded old mail"
    );
    Ok(())
}

impl Instance {
    /// Serves the API on `connection`, takes the bus name and starts every
    /// account's worker.
    pub async fn start(
        paths: Paths,
        secrets: Secrets,
        config: WorkerConfig,
        connection: zbus::Connection,
    ) -> Result<Self, StartError> {
        let index_paths = paths.clone();
        let (daemon, notices) = Daemon::new(paths, secrets, config)?;
        connection
            .object_server()
            .at(ids::PIM_OBJECT_PATH, PimService::new(daemon.clone()))
            .await?;
        // Taken after the object is there, so activated calls find it.
        let reply = connection
            .request_name_with_flags(ids::DAEMON_BUS_NAME, RequestNameFlags::DoNotQueue.into())
            .await;
        match reply {
            Ok(RequestNameReply::PrimaryOwner | RequestNameReply::AlreadyOwner) => {}
            Ok(RequestNameReply::Exists | RequestNameReply::InQueue)
            | Err(zbus::Error::NameTaken) => return Err(StartError::AlreadyRunning),
            Err(err) => return Err(err.into()),
        }
        // Only the daemon that owns the bus name may write the index.
        let indexer = start_indexer(&index_paths);
        let (forward, forwarded) = async_channel::unbounded();
        smol::spawn(watch_mail(
            notices,
            forward,
            indexer.as_ref().map(Indexer::waker),
        ))
        .detach();
        smol::spawn(service::emit_signals(connection.clone(), forwarded)).detach();
        daemon.start().await?;
        let backfill = Backfill::start(&index_paths, daemon.notifier());
        Ok(Self {
            daemon,
            connection,
            indexer,
            backfill,
        })
    }

    /// Releases the bus name and stops every worker and the indexer.
    pub async fn shutdown(self) {
        if let Err(err) = self.connection.release_name(ids::DAEMON_BUS_NAME).await {
            tracing::debug!(%err, "releasing the bus name");
        }
        self.backfill.stop().await;
        self.daemon.shutdown().await;
        if let Some(indexer) = self.indexer {
            // Commits what it has indexed; at most one batch more.
            smol::unblock(move || indexer.stop()).await;
        }
    }
}

/// Starts indexing the store for search. Without an index, search is
/// unavailable but mail still syncs, so a failure is only logged.
fn start_indexer(paths: &Paths) -> Option<Indexer> {
    let started = Indexer::start(paths, IndexerOptions::default(), |event| match event {
        IndexEvent::Progress { indexed } => tracing::debug!(indexed, "indexing"),
        IndexEvent::Updated(stats) => tracing::debug!(?stats, "index updated"),
        IndexEvent::Failed(err) => tracing::warn!(%err, "indexing failed; retrying later"),
    });
    match started {
        Ok(indexer) => Some(indexer),
        Err(err) => {
            tracing::error!(%err, "search index unavailable");
            None
        }
    }
}

/// Passes `notices` on to `forward` and wakes the indexer when mail changed.
async fn watch_mail(
    notices: Receiver<Notice>,
    forward: Sender<Notice>,
    indexer: Option<IndexerWaker>,
) {
    while let Ok(notice) = notices.recv().await {
        if let (Notice::MailChanged(_), Some(indexer)) = (notice, &indexer) {
            indexer.changed();
        }
        if forward.send(notice).await.is_err() {
            break;
        }
    }
}
