// SPDX-License-Identifier: GPL-3.0-or-later

//! The account worker (plan task 1.4): keeps one account in sync for as long
//! as the daemon runs.
//!
//! 1. Connect. On failure, wait and try again, doubling the wait each time
//!    up to [`WorkerConfig::retry_max`]. A refused password is not retried:
//!    it would only lock the account on many servers.
//! 2. Sync every folder ([`engine::sync_account`]).
//! 3. Loop: bring the inbox up to date, then wait on it with IDLE until the
//!    server reports a change or [`WorkerConfig::idle_timeout`] passes
//!    (RFC 2177 asks clients to renew IDLE before 29 minutes). Every
//!    [`WorkerConfig::full_sync_interval`], sync all folders again.
//! 4. Any network or protocol error ends the session; go back to 1.
//!
//! The worker owns its [`Store`] handle, so several workers can run at once;
//! SQLite serialises their writes.

use std::{future::Future, time::Duration};

use async_channel::{Receiver, Sender};
use async_io::Timer;
use futures_lite::FutureExt;
use katna_core::AccountId;
use katna_store::{FolderRole, Store};

use crate::{
    Credentials, Endpoint, Error, MailBackend, Result,
    engine::{self, FolderReport},
    imap::ImapBackend,
    net::Tls,
};

/// Opens new connections for a worker.
pub trait Connector: Send + Sync + 'static {
    type Backend: MailBackend;

    fn connect(&self) -> impl Future<Output = Result<Self::Backend>> + Send;
}

/// Connects to an IMAP server with a password.
#[derive(Clone)]
pub struct ImapConnector {
    pub endpoint: Endpoint,
    pub credentials: Credentials,
    pub tls: Tls,
}

impl Connector for ImapConnector {
    type Backend = ImapBackend;

    async fn connect(&self) -> Result<ImapBackend> {
        ImapBackend::connect(&self.endpoint, &self.credentials, self.tls.clone()).await
    }
}

/// Timing of a worker.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkerConfig {
    /// How long one IDLE may last before it is renewed.
    pub idle_timeout: Duration,
    /// How often every folder is synced, not only the inbox.
    pub full_sync_interval: Duration,
    /// First wait before reconnecting.
    pub retry_min: Duration,
    /// Longest wait before reconnecting.
    pub retry_max: Duration,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            idle_timeout: Duration::from_secs(25 * 60),
            full_sync_interval: Duration::from_secs(15 * 60),
            retry_min: Duration::from_secs(2),
            retry_max: Duration::from_secs(5 * 60),
        }
    }
}

/// What a worker reports while it runs.
#[derive(Debug)]
pub enum Event {
    Connected,
    /// A sync changed something, or a full sync finished.
    Synced(Vec<FolderReport>),
    /// The connection failed or broke. The worker reconnects after `retry_in`.
    Disconnected {
        error: String,
        retry_in: Duration,
    },
    /// The server refused the password. The worker waits for [`Stop`].
    AuthFailed(String),
}

/// Tells a worker to stop. Dropping it stops the worker too.
pub struct StopHandle(#[allow(dead_code)] Sender<()>);

/// The worker's side of a [`StopHandle`].
#[derive(Clone)]
pub struct Stop(Receiver<()>);

/// A new stop signal.
pub fn stop_signal() -> (StopHandle, Stop) {
    let (tx, rx) = async_channel::bounded(1);
    (StopHandle(tx), Stop(rx))
}

impl Stop {
    /// Completes once the handle is dropped. Cancel-safe.
    pub async fn wait(&self) {
        // Nothing is ever sent; `recv` fails when the sender goes away.
        let _ = self.0.recv().await;
    }

    fn is_set(&self) -> bool {
        self.0.is_closed()
    }
}

/// Runs the worker for `account` until `stop` fires.
pub async fn run<C: Connector>(
    connector: C,
    mut store: Store,
    account: AccountId,
    config: WorkerConfig,
    events: Sender<Event>,
    stop: Stop,
) {
    let mut delay = config.retry_min;
    while !stop.is_set() {
        let connected = async { Some(connector.connect().await) }
            .or(async {
                stop.wait().await;
                None
            })
            .await;
        let error = match connected {
            None => return,
            Some(Err(Error::Auth(message))) => {
                tracing::warn!(%account, %message, "login refused; not retrying");
                let _ = events.try_send(Event::AuthFailed(message));
                stop.wait().await;
                return;
            }
            Some(Err(error)) => error,
            Some(Ok(mut backend)) => {
                let _ = events.try_send(Event::Connected);
                let mut synced = false;
                match session(
                    &mut backend,
                    &mut store,
                    account,
                    &config,
                    &events,
                    &stop,
                    &mut synced,
                )
                .await
                {
                    Ok(()) => {
                        if let Err(error) = backend.logout().await {
                            tracing::debug!(%error, "logout on stop");
                        }
                        return;
                    }
                    Err(error) => {
                        // A session that got as far as a full sync counts
                        // as a success: start the next wait short again.
                        if synced {
                            delay = config.retry_min;
                        }
                        error
                    }
                }
            }
        };
        tracing::info!(%account, %error, retry_in = ?delay, "disconnected");
        let _ = events.try_send(Event::Disconnected {
            error: error.to_string(),
            retry_in: delay,
        });
        let stopped = sleep_or_stop(delay, &stop).await;
        if stopped {
            return;
        }
        delay = (delay * 2).min(config.retry_max);
    }
}

/// One connected session. Returns `Ok` when stopped, `Err` when the
/// connection is no longer usable.
async fn session<B: MailBackend>(
    backend: &mut B,
    store: &mut Store,
    account: AccountId,
    config: &WorkerConfig,
    events: &Sender<Event>,
    stop: &Stop,
    synced: &mut bool,
) -> Result<()> {
    let mut last_full = std::time::Instant::now();
    let mut first = true;
    loop {
        if first || last_full.elapsed() >= config.full_sync_interval {
            let reports = engine::sync_account(backend, store, account).await?;
            let _ = events.try_send(Event::Synced(reports));
            *synced = true;
            first = false;
            last_full = std::time::Instant::now();
        }
        let Some(inbox) = store
            .folders(account)?
            .into_iter()
            .find(|f| f.role == Some(FolderRole::Inbox))
        else {
            // Nothing to watch: wait for the next full sync.
            let next = config
                .full_sync_interval
                .saturating_sub(last_full.elapsed());
            if sleep_or_stop(next, stop).await {
                return Ok(());
            }
            continue;
        };

        // Catch up on the inbox, which also selects it for IDLE.
        let report = engine::sync_folder(backend, store, account, inbox.id, &inbox.path).await?;
        if report.added + report.flags_changed + report.removed > 0 || report.reset {
            let _ = events.try_send(Event::Synced(vec![report]));
        }

        let until_full = config
            .full_sync_interval
            .saturating_sub(last_full.elapsed());
        if until_full.is_zero() {
            continue;
        }
        let wait = backend
            .wait_for_changes(config.idle_timeout.min(until_full), stop.wait())
            .await?;
        if wait.interrupted.is_some() {
            return Ok(());
        }
    }
}

/// Sleeps for `duration`. Returns `true` if `stop` fired first.
async fn sleep_or_stop(duration: Duration, stop: &Stop) -> bool {
    async {
        Timer::after(duration).await;
        false
    }
    .or(async {
        stop.wait().await;
        true
    })
    .await
}
