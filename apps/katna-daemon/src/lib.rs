// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna background service. See `docs/ARCHITECTURE.md` §9.
//!
//! The daemon is the only process that writes the store and talks to mail
//! servers. It runs one sync worker per account, keeps the search index and
//! the threads up to date and serves `in.invenia.katna.Pim1` on the session bus; owning
//! the bus name keeps it to a single instance.

pub mod daemon;
pub mod install;
pub mod secrets;
pub mod service;

use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    time::{Duration, Instant},
};

use async_channel::{Receiver, Sender};
use katna_core::{AccountId, Paths, ids};
use katna_search::{IndexEvent, Indexer, IndexerOptions, IndexerWaker};
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
    threader: Threader,
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
        let threader = Threader::start(index_paths.clone(), forward.clone());
        smol::spawn(watch_mail(
            notices,
            forward,
            indexer.as_ref().map(Indexer::waker),
            threader.wake.clone(),
        ))
        .detach();
        smol::spawn(service::emit_signals(connection.clone(), forwarded)).detach();
        daemon.start().await?;
        Ok(Self {
            daemon,
            connection,
            indexer,
            threader,
        })
    }

    /// Releases the bus name and stops every worker and the indexer.
    pub async fn shutdown(self) {
        if let Err(err) = self.connection.release_name(ids::DAEMON_BUS_NAME).await {
            tracing::debug!(%err, "releasing the bus name");
        }
        self.daemon.shutdown().await;
        let threader = self.threader;
        smol::unblock(move || threader.stop()).await;
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

/// Passes `notices` on to `forward` and wakes the indexer and the threader
/// when mail changed.
async fn watch_mail(
    notices: Receiver<Notice>,
    forward: Sender<Notice>,
    indexer: Option<IndexerWaker>,
    threader: mpsc::SyncSender<()>,
) {
    while let Ok(notice) = notices.recv().await {
        if let Notice::MailChanged(_) = notice {
            if let Some(indexer) = &indexer {
                indexer.changed();
            }
            // Already awake if the channel is full.
            let _ = threader.try_send(());
        }
        if forward.send(notice).await.is_err() {
            break;
        }
    }
}

/// Threads imported and older mail on its own thread with its own store
/// connection (`katna_sync::threads`): once at start, then whenever mail
/// changed. IMAP sync threads new mail itself; this catches the rest.
struct Threader {
    wake: mpsc::SyncSender<()>,
    stop: Arc<AtomicBool>,
    thread: std::thread::JoinHandle<()>,
}

impl Threader {
    /// How often a long run tells clients about new threads.
    const NOTIFY_EVERY: Duration = Duration::from_secs(5);

    fn start(paths: Paths, signals: Sender<Notice>) -> Self {
        let (wake, woken) = mpsc::sync_channel(1);
        let _ = wake.try_send(());
        let stop = Arc::new(AtomicBool::new(false));
        let stopping = stop.clone();
        let thread = std::thread::Builder::new()
            .name("katna-threader".into())
            .spawn(move || Self::run(&paths, &woken, &signals, &stopping))
            .expect("spawning a thread");
        Self { wake, stop, thread }
    }

    fn run(paths: &Paths, woken: &mpsc::Receiver<()>, signals: &Sender<Notice>, stop: &AtomicBool) {
        let mut store = match katna_store::Store::open(paths, katna_store::Mode::ReadWrite) {
            Ok(store) => store,
            Err(err) => {
                tracing::error!(%err, "threading unavailable");
                return;
            }
        };
        while woken.recv().is_ok() && !stop.load(Ordering::Relaxed) {
            let mut changed: Vec<AccountId> = Vec::new();
            let mut told = Instant::now();
            let mut threaded = 0;
            while !stop.load(Ordering::Relaxed) {
                match katna_sync::threads::thread_pending(&mut store) {
                    Ok(accounts) if accounts.is_empty() => break,
                    Ok(accounts) => {
                        threaded += katna_sync::threads::BATCH;
                        for account in accounts {
                            if !changed.contains(&account) {
                                changed.push(account);
                            }
                        }
                    }
                    Err(err) => {
                        tracing::warn!(%err, "threading failed; retrying on the next change");
                        break;
                    }
                }
                if told.elapsed() >= Self::NOTIFY_EVERY {
                    tracing::debug!(threaded, "threading");
                    Self::tell(signals, &mut changed);
                    told = Instant::now();
                }
            }
            Self::tell(signals, &mut changed);
        }
    }

    fn tell(signals: &Sender<Notice>, changed: &mut Vec<AccountId>) {
        for account in changed.drain(..) {
            let _ = signals.send_blocking(Notice::MailChanged(account));
        }
    }

    /// Stops after the batch it is on.
    fn stop(self) {
        self.stop.store(true, Ordering::Relaxed);
        // A full channel wakes it just as well.
        let _ = self.wake.try_send(());
        if self.thread.join().is_err() {
            tracing::error!("the threader panicked");
        }
    }
}
