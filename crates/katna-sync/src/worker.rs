// SPDX-License-Identifier: GPL-3.0-or-later

//! The account worker (plan task 1.4): keeps one account in sync for as long
//! as the daemon runs.
//!
//! 1. Connect. On failure, wait and try again, doubling the wait each time
//!    up to [`WorkerConfig::retry_max`]. A refused password is not retried
//!    on its own: it would only lock the account on many servers.
//! 2. Sync every folder ([`engine::sync_account`]).
//! 3. Loop: bring the inbox up to date, then wait on it with IDLE until the
//!    server reports a change or [`WorkerConfig::idle_timeout`] passes
//!    (RFC 2177 asks clients to renew IDLE before 29 minutes). Every
//!    [`WorkerConfig::full_sync_interval`], sync all folders again.
//! 4. Any network or protocol error ends the session; go back to 1.
//!
//! [`Handle::sync_now`] ends any wait: it starts a full sync, or reconnects
//! at once. Dropping the [`Handle`] logs out and ends the worker.
//!
//! The worker owns its [`Store`] handle, so several workers can run at once;
//! SQLite serialises their writes.

use std::{
    future::Future,
    time::{Duration, Instant},
};

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
    /// The server refused the password. The worker tries again only when
    /// asked to ([`Handle::sync_now`]).
    AuthFailed(String),
}

/// The daemon's side of a worker: asks it to sync now, and stops it when
/// dropped.
pub struct Handle {
    _stop: Sender<()>,
    wake: Sender<()>,
}

impl Handle {
    /// Asks the worker to sync every folder now. A worker waiting to
    /// reconnect, or stopped by a refused password, tries again at once.
    pub fn sync_now(&self) {
        // A full channel already holds a request.
        let _ = self.wake.try_send(());
    }
}

/// The worker's side of a [`Handle`].
pub struct Control {
    stop: Receiver<()>,
    wake: Receiver<()>,
}

/// What ended a wait on a [`Control`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Signal {
    Stop,
    Wake,
}

/// A new handle and the control it drives.
pub fn control() -> (Handle, Control) {
    let (stop_tx, stop) = async_channel::bounded(1);
    let (wake, wake_rx) = async_channel::bounded(1);
    (
        Handle {
            _stop: stop_tx,
            wake,
        },
        Control {
            stop,
            wake: wake_rx,
        },
    )
}

impl Control {
    /// Completes when the handle asks for something. Cancel-safe.
    async fn signal(&self) -> Signal {
        // Nothing is sent on `stop`; `recv` fails when the handle is dropped.
        async {
            let _ = self.stop.recv().await;
            Signal::Stop
        }
        .or(async {
            match self.wake.recv().await {
                Ok(()) => Signal::Wake,
                Err(_) => Signal::Stop,
            }
        })
        .await
    }

    /// Completes once the handle is dropped. Cancel-safe.
    async fn stopped(&self) {
        let _ = self.stop.recv().await;
    }

    fn is_stopped(&self) -> bool {
        self.stop.is_closed()
    }
}

/// Runs the worker for `account` until its [`Handle`] is dropped.
pub async fn run<C: Connector>(
    connector: C,
    mut store: Store,
    account: AccountId,
    config: WorkerConfig,
    events: Sender<Event>,
    control: Control,
) {
    let mut delay = config.retry_min;
    while !control.is_stopped() {
        let connected = async { Some(connector.connect().await) }
            .or(async {
                control.stopped().await;
                None
            })
            .await;
        let error = match connected {
            None => return,
            Some(Err(Error::Auth(message))) => {
                tracing::warn!(%account, %message, "login refused; waiting for sync now");
                let _ = events.try_send(Event::AuthFailed(message));
                // Retrying on its own could lock the account on many
                // servers; only an explicit request tries again.
                match control.signal().await {
                    Signal::Stop => return,
                    Signal::Wake => {
                        delay = config.retry_min;
                        continue;
                    }
                }
            }
            Some(Err(error)) => error,
            Some(Ok(mut backend)) => {
                let _ = events.try_send(Event::Connected);
                let mut synced = false;
                let result = session(
                    &mut backend,
                    &mut store,
                    account,
                    &config,
                    &events,
                    &control,
                    &mut synced,
                )
                .await;
                match result {
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
        match sleep_or_signal(delay, &control).await {
            Some(Signal::Stop) => return,
            // Sync now: reconnect at once, starting the waits over.
            Some(Signal::Wake) => delay = config.retry_min,
            None => delay = (delay * 2).min(config.retry_max),
        }
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
    control: &Control,
    synced: &mut bool,
) -> Result<()> {
    let mut last_full = Instant::now();
    let mut full = true;
    loop {
        if full || last_full.elapsed() >= config.full_sync_interval {
            let reports = engine::sync_account(backend, store, account).await?;
            let _ = events.try_send(Event::Synced(reports));
            *synced = true;
            full = false;
            last_full = Instant::now();
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
            match sleep_or_signal(next, control).await {
                Some(Signal::Stop) => return Ok(()),
                Some(Signal::Wake) => full = true,
                None => {}
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
            .wait_for_changes(config.idle_timeout.min(until_full), control.signal())
            .await?;
        match wait.interrupted {
            Some(Signal::Stop) => return Ok(()),
            Some(Signal::Wake) => full = true,
            None => {}
        }
    }
}

/// Sleeps for `duration`. Returns the signal that cut it short, if any.
async fn sleep_or_signal(duration: Duration, control: &Control) -> Option<Signal> {
    async {
        Timer::after(duration).await;
        None
    }
    .or(async { Some(control.signal().await) })
    .await
}
