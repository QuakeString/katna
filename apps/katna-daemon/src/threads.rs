// SPDX-License-Identifier: GPL-3.0-or-later

//! Long-running work on threads of its own.
//!
//! The sync engine and the store call SQLite and blob files straight from
//! async code. smol's shared executor has one thread, so while one
//! account's call waits (a big first sync, a slow disk, a lock held up to
//! the busy timeout) everything else on it would wait too: the other
//! accounts, the outbox, the contacts, tasks, notes and calendar loops and
//! the D-Bus signals. Each account worker, the outbox and those loops
//! therefore run on their own thread, with the shared executor left to
//! short work.

use std::{future::Future, pin::Pin};

use async_channel::{Receiver, Sender};
use futures_lite::FutureExt;

type Work = Pin<Box<dyn Future<Output = ()> + Send>>;

/// Work running on its own thread. Dropping it cancels the work at its
/// next await, as dropping a `smol::Task` does.
pub(crate) struct Threaded {
    done: Receiver<()>,
    _cancel: Sender<()>,
}

impl Threaded {
    /// Waits until the work ends.
    pub(crate) async fn finished(self) {
        let Self { done, _cancel } = self;
        let _ = done.recv().await;
    }
}

/// Runs `work` on a new thread called `name`.
pub(crate) fn spawn(name: &str, work: impl Future<Output = ()> + Send + 'static) -> Threaded {
    let (cancel, cancelled) = async_channel::bounded::<()>(1);
    let (finished, done) = async_channel::bounded(1);
    start(
        name,
        Box::pin(async move {
            work.or(async {
                let _ = cancelled.recv().await;
            })
            .await;
            let _ = finished.try_send(());
        }),
    );
    Threaded {
        done,
        _cancel: cancel,
    }
}

/// Runs `work` on a new thread called `name` until it ends by itself.
pub(crate) fn detach(name: &str, work: impl Future<Output = ()> + Send + 'static) {
    start(name, Box::pin(work));
}

fn start(name: &str, work: Work) {
    // The work goes over once the thread exists, so that it can still run
    // on the shared executor when no thread can be made.
    let (give, take) = std::sync::mpsc::sync_channel::<Work>(1);
    let started = std::thread::Builder::new()
        .name(name.to_owned())
        .spawn(move || {
            if let Ok(work) = take.recv() {
                smol::block_on(work);
            }
        });
    match started {
        Ok(_) => {
            let _ = give.send(work);
        }
        Err(err) => {
            tracing::warn!(%err, name, "no thread of its own; sharing the executor");
            smol::spawn(work).detach();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::*;

    #[test]
    fn blocking_work_holds_up_only_its_own_thread() {
        let blocked = spawn("blocked", async {
            std::thread::sleep(Duration::from_secs(2));
        });
        let started = Instant::now();
        smol::block_on(async {
            let other = spawn("other", async {
                async_io::Timer::after(Duration::from_millis(50)).await;
            });
            other.finished().await;
        });
        assert!(
            started.elapsed() < Duration::from_secs(1),
            "{:?}",
            started.elapsed()
        );
        drop(blocked);
    }

    #[test]
    fn dropping_cancels_and_finished_waits() {
        let (tx, rx) = async_channel::bounded::<()>(1);
        let waiting = spawn("waiting", async move {
            let _ = rx.recv().await;
        });
        drop(waiting);
        // Cancelled: the receiver is gone, so sending fails soon.
        let deadline = Instant::now() + Duration::from_secs(5);
        while !tx.is_closed() {
            assert!(Instant::now() < deadline, "not cancelled");
            std::thread::sleep(Duration::from_millis(5));
        }
        smol::block_on(spawn("quick", async {}).finished());
    }
}
