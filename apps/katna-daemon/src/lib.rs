// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna background service. See `docs/ARCHITECTURE.md` §9.
//!
//! The daemon is the only process that writes the store and talks to mail
//! servers. It runs one sync worker per account, keeps the search index up
//! to date and serves `in.invenia.katna.Pim1` on the session bus, with
//! KRunner's and GNOME Shell's search; owning the bus name keeps it to a
//! single instance.

mod agenda;
mod crash_upload;
pub mod daemon;
mod desktop;
mod desktop_search;
pub mod install;
pub mod katna_account;
mod mail_app;
mod notify;
mod on_demand;
pub mod secrets;
pub mod service;
pub mod system;
mod threads;
mod tracking;
pub mod translate;
pub mod update;
mod updates;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use async_channel::{Receiver, Sender};
use futures_lite::FutureExt;
use katna_core::{AccountId, Paths, ids};
use katna_search::{IndexEvent, Indexer, IndexerOptions, IndexerWaker};
use katna_store::{Mode, Store};
use katna_sync::worker::WorkerConfig;
use zbus::fdo::{RequestNameFlags, RequestNameReply};

use crate::{
    daemon::{Daemon, Notice},
    secrets::Secrets,
    service::PimService,
    system::SystemEvent,
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
    paths: Paths,
    connection: zbus::Connection,
    /// `None` if the index could not be opened; mail still syncs.
    indexer: Option<Indexer>,
    backfill: Backfill,
    /// Sent when the user quits from the tray.
    quit: Receiver<()>,
}

/// Why [`Instance::serve`] returned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ended {
    Stopped,
    /// All data was deleted; the next client starts a new daemon.
    Deleted,
}

/// Threads and classifies mail stored before threading existed, on its own
/// thread with its own store handle, one small transaction at a time.
struct Backfill {
    stop: Arc<AtomicBool>,
    thread: Option<std::thread::JoinHandle<()>>,
}

/// How long after starting the desktop search reads the addresses.
const DESKTOP_SEARCH_WARM_UP: Duration = Duration::from_secs(20);
/// When the daemon switches on Katna's GNOME Shell clock extension.
const GNOME_EXTENSION_DELAY: Duration = Duration::from_secs(10);

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
    let accounts: Vec<AccountId> = store.accounts()?.into_iter().map(|a| a.id).collect();
    let tell = || {
        for &account in &accounts {
            let _ = notices.try_send(Notice::MailChanged(account));
        }
    };
    let mut last_notice = Instant::now();
    let mut keep_going = |_| {
        // Tell the apps now and then, not after every batch.
        if last_notice.elapsed() > Duration::from_secs(2) {
            tell();
            last_notice = Instant::now();
        }
        std::thread::sleep(BACKFILL_PAUSE);
        !stop.load(Ordering::Relaxed)
    };

    let left = store.unthreaded_count()?;
    if left > 0 {
        let started = Instant::now();
        tracing::info!(left, "threading old mail");
        let changed = katna_import::backfill::run(
            &mut store,
            katna_import::backfill::BATCH,
            &mut keep_going,
        )?;
        tell();
        tracing::info!(
            changed,
            seconds = started.elapsed().as_secs_f32(),
            "threaded old mail"
        );
    }
    if stop.load(Ordering::Relaxed) {
        return Ok(());
    }

    // Attachment lists for downloaded mail stored without one; Gmail's AMP
    // body is not a file.
    let started = Instant::now();
    let dropped = store.forget_body_parts(&katna_import::mime::BODY_TEXT)?;
    let listed = katna_import::backfill::run_attachments(
        &mut store,
        katna_import::backfill::BATCH,
        &mut keep_going,
    )?;
    if dropped > 0 || listed > 0 {
        tell();
        tracing::info!(
            dropped,
            listed,
            seconds = started.elapsed().as_secs_f32(),
            "listed attachments of old mail"
        );
    }
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
        let finder = desktop_search::Finder::new(
            index_paths.clone(),
            daemon::settings(&index_paths).general.search_triggers,
        );
        daemon.set_finder(finder.clone());
        desktop_search::serve(&connection, finder.clone()).await?;
        agenda::serve(&connection, daemon.clone()).await?;
        let (shell, shell_paths) = (connection.clone(), index_paths.clone());
        smol::spawn(async move {
            // Out of the way of the first sync after login.
            smol::Timer::after(GNOME_EXTENSION_DELAY).await;
            agenda::enable_gnome_extension(&shell, &shell_paths).await;
        })
        .detach();
        let warm = finder.clone();
        smol::spawn(async move {
            // Out of the way of the first sync after login.
            smol::Timer::after(DESKTOP_SEARCH_WARM_UP).await;
            warm.warm_up();
        })
        .detach();
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
        if let Some(indexer) = &indexer {
            daemon.set_indexer(indexer.waker());
        }
        let (desktop, desktop_events) = desktop::channel();
        let (quit_sender, quit) = async_channel::bounded(1);
        daemon.set_desktop(desktop.clone());
        smol::spawn(desktop::run(
            connection.clone(),
            index_paths.clone(),
            daemon::settings(&index_paths).general,
            desktop.clone(),
            desktop_events,
            quit_sender,
        ))
        .detach();
        let (forward, forwarded) = async_channel::unbounded();
        smol::spawn(watch_mail(
            notices,
            forward,
            indexer.as_ref().map(Indexer::waker),
            desktop,
            finder,
        ))
        .detach();
        smol::spawn(service::emit_signals(connection.clone(), forwarded)).detach();
        // Windows has no notification server: the daemon is its own.
        #[cfg(windows)]
        if let Err(err) = katna_platform::toasts::serve(&connection).await {
            tracing::warn!(%err, "no toasts");
        }
        daemon.notify_new_mail(&connection).await;
        daemon.start().await?;
        let backfill = Backfill::start(&index_paths, daemon.notifier());
        Ok(Self {
            daemon,
            paths: index_paths,
            connection,
            indexer,
            backfill,
            quit,
        })
    }

    /// Waits until the user quits Katna from the tray. The daemon starts
    /// again at the next login or when the app needs it (D-Bus activation).
    pub async fn quit_requested(&self) {
        let _ = self.quit.recv().await;
    }

    /// Serves until `stop` finishes or the user quits from the tray, then
    /// shuts down; or until a client asks to delete all data, which it
    /// does before exiting.
    pub async fn serve(self, stop: impl Future<Output = ()>) -> Ended {
        let quit = self.quit.clone();
        let stop = stop.or(async move {
            let _ = quit.recv().await;
            tracing::info!("quit from the tray");
        });
        let requests = self.daemon.delete_requests();
        let delete = async {
            match requests.recv().await {
                Ok(done) => Some(done),
                Err(_) => std::future::pending().await,
            }
        };
        let request = async {
            stop.await;
            None
        }
        .or(delete)
        .await;
        match request {
            None => {
                self.shutdown().await;
                Ended::Stopped
            }
            Some(done) => {
                self.delete_all_data(done).await;
                Ended::Deleted
            }
        }
    }

    /// Stops what writes files, deletes them and tells `done`. The daemon
    /// has stopped every account and deleted the passwords already.
    async fn delete_all_data(self, done: daemon::DeleteDone) {
        self.backfill.stop().await;
        if let Some(indexer) = self.indexer {
            smol::unblock(move || indexer.stop()).await;
        }
        let paths = self.paths.clone();
        let deleted = smol::unblock(move || paths.delete_all_data()).await;
        match &deleted {
            Ok(()) => tracing::warn!("all data deleted"),
            Err(err) => tracing::error!(%err, "deleting all data"),
        }
        let _ = done.send(deleted.map_err(|err| err.to_string())).await;
        // Let the answer go out before the bus name. The bus name is kept
        // until the files are gone, so no new daemon opens them meanwhile.
        smol::Timer::after(Duration::from_millis(300)).await;
        if let Err(err) = self.connection.release_name(ids::DAEMON_BUS_NAME).await {
            tracing::debug!(%err, "releasing the bus name");
        }
    }

    /// Has every worker reconnect at once when the machine wakes up or
    /// the network comes back, and hold back body downloads while the
    /// network is metered, as the system bus `system` reports.
    pub fn watch_system(&self, system: zbus::Connection) {
        let daemon = Arc::downgrade(&self.daemon);
        smol::spawn(async move {
            let watched = system::watch(system, |event| {
                tracing::debug!(?event, "system event");
                let Some(daemon) = daemon.upgrade() else {
                    return;
                };
                match event {
                    SystemEvent::Resumed | SystemEvent::NetworkUp => daemon.network_changed(),
                    SystemEvent::Metered(metered) => daemon.set_metered(metered),
                }
            })
            .await;
            if let Err(err) = watched {
                tracing::warn!(%err, "not watching suspend and network changes");
            }
        })
        .detach();
    }

    /// Releases the bus name and stops every worker and the indexer.
    pub async fn shutdown(self) {
        if let Err(err) = self.connection.release_name(ids::DAEMON_BUS_NAME).await {
            tracing::debug!(%err, "releasing the bus name");
        }
        let started = Instant::now();
        self.backfill.stop().await;
        tracing::debug!(ms = started.elapsed().as_millis(), "backfill stopped");
        self.daemon.shutdown().await;
        tracing::debug!(ms = started.elapsed().as_millis(), "accounts stopped");
        if let Some(indexer) = self.indexer {
            // Commits what it has indexed; at most one batch more.
            smol::unblock(move || indexer.stop()).await;
        }
        tracing::debug!(ms = started.elapsed().as_millis(), "indexer stopped");
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

/// Passes `notices` on to `forward`, and wakes the indexer, updates the
/// unread counts and has the desktop search read the addresses again when
/// mail or saved contacts changed.
async fn watch_mail(
    notices: Receiver<Notice>,
    forward: Sender<Notice>,
    indexer: Option<IndexerWaker>,
    desktop: desktop::Handle,
    finder: Arc<desktop_search::Finder>,
) {
    while let Ok(notice) = notices.recv().await {
        if let Notice::MailChanged(_) = notice {
            if let Some(indexer) = &indexer {
                indexer.changed();
            }
            desktop.mail_changed();
            finder.mail_changed();
        }
        // Saved names are in the book too.
        if let Notice::ContactsChanged = notice {
            finder.mail_changed();
        }
        if forward.send(notice).await.is_err() {
            break;
        }
    }
}
