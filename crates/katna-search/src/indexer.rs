// SPDX-License-Identifier: GPL-3.0-or-later

//! Keeps the index up to date in the background, for `katna-daemon`.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use katna_core::Paths;
use katna_store::{DbKind, Mode, Store};

use crate::error::{Error, Result};
use crate::index::{IndexOptions, SearchIndex, UpdateStats};

/// What the indexer did, for status and D-Bus signals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndexEvent {
    /// Part of a long update is committed and searchable: this many
    /// messages so far.
    Progress { indexed: u64 },
    /// An update finished and changed the index.
    Updated(UpdateStats),
    /// An update failed; the indexer tries again at the next change or
    /// poll.
    Failed(String),
}

/// Options of [`Indexer::start`].
#[derive(Debug, Clone)]
pub struct IndexerOptions {
    pub index: IndexOptions,
    /// How often the store's change journal is checked without a
    /// [`changed`](Indexer::changed) call, so changes by any writer are
    /// picked up.
    pub poll_every: Duration,
}

impl Default for IndexerOptions {
    fn default() -> Self {
        Self {
            index: IndexOptions::default(),
            poll_every: Duration::from_secs(5),
        }
    }
}

enum Wake {
    Changed,
    /// Empty the index first, then index everything again.
    Rebuild,
    Stop,
}

/// A thread that indexes the store's messages: all of them the first time,
/// then what changes. It reads the store with its own read-only connection,
/// so it never blocks the writer. Apps that opened the index with
/// [`SearchIndex::open_read_only`] see each commit by themselves.
pub struct Indexer {
    wake: mpsc::Sender<Wake>,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl Indexer {
    /// Opens (creating or rebuilding if needed) the index in `paths` and
    /// starts indexing. `events` is called on the indexer's thread.
    pub fn start(
        paths: &Paths,
        options: IndexerOptions,
        mut events: impl FnMut(IndexEvent) + Send + 'static,
    ) -> Result<Self> {
        let dir = paths.index_dir();
        let index = SearchIndex::open(&dir)?;
        let paths = paths.clone();
        let stop = Arc::new(AtomicBool::new(false));
        let mut index_options = options.index;
        index_options.stop = Some(stop.clone());
        let (wake, woken) = mpsc::channel();
        let thread = thread::Builder::new()
            .name("katna-indexer".into())
            .spawn(move || {
                let mut store = None;
                let mut indexed_seq = None;
                let mut rebuild = false;
                loop {
                    if std::mem::take(&mut rebuild) {
                        match index.clear() {
                            Ok(()) => indexed_seq = None,
                            Err(err) => events(IndexEvent::Failed(err.to_string())),
                        }
                    }
                    run_once(
                        &paths,
                        &index,
                        &index_options,
                        &mut store,
                        &mut indexed_seq,
                        &mut events,
                    );
                    match woken.recv_timeout(options.poll_every) {
                        Ok(Wake::Stop) | Err(RecvTimeoutError::Disconnected) => break,
                        Ok(Wake::Rebuild) => rebuild = true,
                        Ok(Wake::Changed) | Err(RecvTimeoutError::Timeout) => {}
                    }
                    // Many changes in a row make one update.
                    loop {
                        match woken.try_recv() {
                            Ok(Wake::Changed) => {}
                            Ok(Wake::Rebuild) => rebuild = true,
                            Ok(Wake::Stop) | Err(mpsc::TryRecvError::Disconnected) => return,
                            Err(mpsc::TryRecvError::Empty) => break,
                        }
                    }
                }
            })
            .map_err(|source| Error::Io { path: dir, source })?;
        Ok(Self {
            wake,
            stop,
            thread: Some(thread),
        })
    }

    /// Tells the indexer the store changed, for example after a sync. Cheap;
    /// calls while it is busy make one more update.
    pub fn changed(&self) {
        let _ = self.wake.send(Wake::Changed);
    }

    /// A handle that can only call [`changed`](Self::changed), for other
    /// tasks and threads.
    pub fn waker(&self) -> IndexerWaker {
        IndexerWaker(self.wake.clone())
    }

    /// Stops the indexer: a running update commits what it has done (at most
    /// one batch of messages more) and the thread exits. Blocks until then.
    pub fn stop(mut self) {
        self.shut_down();
    }

    fn shut_down(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        let _ = self.wake.send(Wake::Stop);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

/// See [`Indexer::waker`]. Does nothing once the indexer has stopped.
#[derive(Clone)]
pub struct IndexerWaker(mpsc::Sender<Wake>);

impl IndexerWaker {
    /// See [`Indexer::changed`].
    pub fn changed(&self) {
        let _ = self.0.send(Wake::Changed);
    }

    /// Has the indexer empty the index and index every message again, for
    /// example after downloaded mail was deleted.
    pub fn rebuild(&self) {
        let _ = self.0.send(Wake::Rebuild);
    }
}

impl Drop for Indexer {
    fn drop(&mut self) {
        self.shut_down();
    }
}

/// One update, if the journal moved since the last one.
fn run_once(
    paths: &Paths,
    index: &SearchIndex,
    options: &IndexOptions,
    store: &mut Option<Store>,
    indexed_seq: &mut Option<i64>,
    events: &mut impl FnMut(IndexEvent),
) {
    let result = (|| -> Result<Option<UpdateStats>> {
        if store.is_none() {
            *store = Some(Store::open(paths, Mode::ReadOnly)?);
        }
        let Some(store) = store.as_ref() else {
            return Ok(None);
        };
        let latest = store.latest_change(DbKind::Mail)?;
        if *indexed_seq == Some(latest) {
            return Ok(None);
        }
        let stats = index.update(store, options, |indexed| {
            if indexed > 0 {
                events(IndexEvent::Progress { indexed });
            }
        })?;
        if !options
            .stop
            .as_ref()
            .is_some_and(|stop| stop.load(Ordering::Relaxed))
        {
            *indexed_seq = Some(latest);
        }
        Ok(Some(stats))
    })();
    match result {
        Ok(Some(stats)) if stats != UpdateStats::default() => events(IndexEvent::Updated(stats)),
        Ok(_) => {}
        Err(err) => {
            tracing::warn!(error = %err, "indexing failed");
            // Reopen the store next time, in case it was replaced.
            *store = None;
            events(IndexEvent::Failed(err.to_string()));
        }
    }
}
