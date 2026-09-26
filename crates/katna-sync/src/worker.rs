// SPDX-License-Identifier: GPL-3.0-or-later

//! The account worker (plan task 1.4): keeps one account in sync for as long
//! as the daemon runs.
//!
//! 1. Connect. On failure, wait and try again, doubling the wait each time
//!    up to [`WorkerConfig::retry_max`]. A refused password is not retried
//!    on its own: it would only lock the account on many servers.
//! 2. Sync every folder ([`engine::sync_account`]), then download the bodies
//!    of messages in the offline window ([`bodies::download_bodies`]).
//! 3. Loop: send queued changes ([`ops::replay`]), bring the inbox and its
//!    bodies up to date, then wait on it with IDLE until the server reports
//!    a change, [`WorkerConfig::idle_timeout`] passes (RFC 2177 asks clients
//!    to renew IDLE before 29 minutes) or a refused change is due again.
//!    Every [`WorkerConfig::full_sync_interval`], sync all folders again.
//! 4. Any network or protocol error ends the session; go back to 1.
//!
//! [`Handle::sync_now`] ends any wait: it starts a full sync, or reconnects
//! at once. [`Handle::reconnect`] drops the connection without waiting for
//! it (after a network change or a resume, it may be dead) and connects
//! again at once. [`Handle::send_changes`] sends queued changes now.
//! [`Handle::fetch_body`] downloads one message now. Dropping the
//! [`Handle`] logs out and ends the worker.
//! On a metered network ([`Handle::set_metered`]) step 2 and 3 skip the
//! bodies; headers, flags and changes stay in sync.
//!
//! POP3 accounts get [`run_pop3`] instead: POP3 has no push, so it checks
//! the maildrop every [`WorkerConfig::pop3_interval`] and when asked, and
//! holds a connection only while checking (the server locks the maildrop
//! for the length of a session).
//!
//! The worker owns its [`Store`] handle, so several workers can run at once;
//! SQLite serialises their writes.

use std::{
    future::Future,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use async_channel::{Receiver, Sender};
use async_io::Timer;
use futures_lite::FutureExt;
use katna_core::{AccountId, Pop3Keep};
use katna_store::{FolderRole, MessageId, Store};

use crate::{
    Credentials, Endpoint, Error, MailBackend, Result,
    bodies::{self, OfflineWindow},
    engine::{self, FolderReport},
    imap::ImapBackend,
    net::Tls,
    ops::{self, ReplayReport},
    pop3::{
        Maildrop, Pop3Client,
        sync::{self as pop3_sync, PROGRESS_EVERY, Pop3Report},
    },
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

/// Opens POP3 sessions for [`run_pop3`].
pub trait Pop3Connect: Send + Sync + 'static {
    type Maildrop: Maildrop;

    fn connect(&self) -> impl Future<Output = Result<Self::Maildrop>> + Send;
}

/// Connects to a POP3 server with a password.
#[derive(Clone)]
pub struct Pop3Connector {
    pub endpoint: Endpoint,
    pub credentials: Credentials,
    pub tls: Tls,
}

impl Pop3Connect for Pop3Connector {
    type Maildrop = Pop3Client;

    async fn connect(&self) -> Result<Pop3Client> {
        Pop3Client::connect(&self.endpoint, &self.credentials, self.tls.clone()).await
    }
}

/// Timing of a worker, and what it keeps offline.
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
    /// Messages whose bodies are downloaded ahead of time.
    pub offline: OfflineWindow,
    /// How often a POP3 account checks for new mail.
    pub pop3_interval: Duration,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            idle_timeout: Duration::from_secs(25 * 60),
            full_sync_interval: Duration::from_secs(15 * 60),
            retry_min: Duration::from_secs(2),
            retry_max: Duration::from_secs(5 * 60),
            offline: OfflineWindow::default(),
            pop3_interval: Duration::from_secs(5 * 60),
        }
    }
}

/// What a worker reports while it runs.
#[derive(Debug)]
pub enum Event {
    Connected,
    /// A sync changed something, or a full sync finished.
    Synced(Vec<FolderReport>),
    /// Bodies of this many messages were downloaded.
    BodiesStored(usize),
    /// Queued changes were sent. Failed ones were undone in the store.
    ChangesSent(ReplayReport),
    /// The connection failed or broke. The worker reconnects after `retry_in`.
    Disconnected {
        error: String,
        retry_in: Duration,
    },
    /// The server refused the password. The worker tries again only when
    /// asked to ([`Handle::sync_now`]).
    AuthFailed(String),
}

/// A request to download one message, answered when it is stored.
struct BodyRequest {
    message: MessageId,
    done: Sender<Result<()>>,
}

/// The daemon's side of a worker: asks it to sync now or fetch a message,
/// and stops it when dropped.
pub struct Handle {
    _stop: Sender<()>,
    wake: Sender<()>,
    reconnect: Sender<()>,
    metered: Arc<AtomicBool>,
    changes: Sender<()>,
    bodies: Sender<BodyRequest>,
}

impl Handle {
    /// Asks the worker to sync every folder now. A worker waiting to
    /// reconnect, or stopped by a refused password, tries again at once.
    pub fn sync_now(&self) {
        // A full channel already holds a request.
        let _ = self.wake.try_send(());
    }

    /// Asks the worker to drop its connection and connect again now,
    /// because the network changed or the machine woke up. Unlike
    /// [`Handle::sync_now`], this does not wait for a command on the old
    /// connection, which may hang until it times out. A worker stopped by
    /// a refused password stays stopped.
    pub fn reconnect(&self) {
        let _ = self.reconnect.try_send(());
    }

    /// Tells the worker whether the network is metered. While it is, the
    /// worker keeps mail and changes in sync but downloads no bodies ahead
    /// of time; a body the user opens is still fetched. When the network
    /// stops being metered, the worker syncs at once and catches up.
    pub fn set_metered(&self, metered: bool) {
        let was = self.metered.swap(metered, Ordering::Relaxed);
        if was && !metered {
            self.sync_now();
        }
    }

    /// Asks the worker to send the changes queued with [`ops`] now. While
    /// offline they wait for the next connection.
    pub fn send_changes(&self) {
        let _ = self.changes.try_send(());
    }

    /// Downloads the full message `message` now. Fails with
    /// [`Error::Closed`] while the worker is offline.
    pub async fn fetch_body(&self, message: MessageId) -> Result<()> {
        self.fetcher().fetch_body(message).await
    }

    /// A cheap handle for [`Handle::fetch_body`] that can be awaited
    /// without borrowing the [`Handle`].
    pub fn fetcher(&self) -> Fetcher {
        Fetcher(self.bodies.clone())
    }
}

/// Asks a worker for message bodies. Does not keep the worker running.
#[derive(Clone)]
pub struct Fetcher(Sender<BodyRequest>);

impl Fetcher {
    /// See [`Handle::fetch_body`].
    pub async fn fetch_body(&self, message: MessageId) -> Result<()> {
        let (done, answer) = async_channel::bounded(1);
        let stopped = || Error::Closed("the account's worker stopped".into());
        self.0
            .send(BodyRequest { message, done })
            .await
            .map_err(|_| stopped())?;
        answer.recv().await.map_err(|_| stopped())?
    }
}

/// The worker's side of a [`Handle`].
pub struct Control {
    stop: Receiver<()>,
    wake: Receiver<()>,
    reconnect: Receiver<()>,
    metered: Arc<AtomicBool>,
    changes: Receiver<()>,
    bodies: Receiver<BodyRequest>,
}

/// What ended a wait on a [`Control`].
enum Signal {
    Stop,
    Wake,
    Changes,
    Fetch(BodyRequest),
}

/// A new handle and the control it drives.
pub fn control() -> (Handle, Control) {
    let (stop_tx, stop) = async_channel::bounded(1);
    let (wake, wake_rx) = async_channel::bounded(1);
    let (reconnect, reconnect_rx) = async_channel::bounded(1);
    let (changes, changes_rx) = async_channel::bounded(1);
    let (bodies, bodies_rx) = async_channel::unbounded();
    let metered = Arc::new(AtomicBool::new(false));
    (
        Handle {
            _stop: stop_tx,
            wake,
            reconnect,
            metered: metered.clone(),
            changes,
            bodies,
        },
        Control {
            stop,
            wake: wake_rx,
            reconnect: reconnect_rx,
            metered,
            changes: changes_rx,
            bodies: bodies_rx,
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
        .or(async {
            match self.changes.recv().await {
                Ok(()) => Signal::Changes,
                Err(_) => Signal::Stop,
            }
        })
        .or(async {
            match self.bodies.recv().await {
                Ok(request) => Signal::Fetch(request),
                Err(_) => Signal::Stop,
            }
        })
        .await
    }

    /// Like [`Self::signal`], but turns body requests away with `offline`
    /// and leaves changes queued until a stop or wake-up comes.
    async fn signal_offline(&self, offline: &str) -> Signal {
        loop {
            match self.signal().await {
                Signal::Fetch(request) => {
                    let _ = request
                        .done
                        .try_send(Err(Error::Closed(offline.to_owned())));
                }
                Signal::Changes => {}
                other => return other,
            }
        }
    }

    /// Completes once the handle is dropped. Cancel-safe.
    async fn stopped(&self) {
        let _ = self.stop.recv().await;
    }

    /// Completes on [`Handle::reconnect`]; never once the handle is
    /// dropped (stopping is [`Self::stopped`]'s job). Cancel-safe.
    async fn reconnect_requested(&self) {
        if self.reconnect.recv().await.is_err() {
            std::future::pending::<()>().await;
        }
    }

    /// Forgets reconnect requests made before the current connection.
    fn clear_reconnect(&self) {
        while self.reconnect.try_recv().is_ok() {}
    }

    /// Whether bodies should wait for an unmetered network.
    fn metered(&self) -> bool {
        self.metered.load(Ordering::Relaxed)
    }

    fn is_stopped(&self) -> bool {
        self.stop.is_closed()
    }
}

/// Why a connection was dropped on [`Handle::reconnect`].
const RECONNECTING: &str = "reconnecting after a network change";

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
        // A request to reconnect drops whatever the connection is doing.
        let mut reconnecting = false;
        control.clear_reconnect();
        let connected = async { Some(connector.connect().await) }
            .or(async {
                control.stopped().await;
                None
            })
            .or(async {
                control.reconnect_requested().await;
                reconnecting = true;
                Some(Err(Error::Closed(RECONNECTING.into())))
            })
            .await;
        let error = match connected {
            None => return,
            Some(Err(Error::Auth(message))) => {
                tracing::warn!(%account, %message, "login refused; waiting for sync now");
                let _ = events.try_send(Event::AuthFailed(message.clone()));
                // Retrying on its own could lock the account on many
                // servers; only an explicit request tries again.
                match control
                    .signal_offline(&format!("the server refused the login: {message}"))
                    .await
                {
                    Signal::Wake => {
                        delay = config.retry_min;
                        continue;
                    }
                    _ => return,
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
                .or(async {
                    control.reconnect_requested().await;
                    reconnecting = true;
                    Err(Error::Closed(RECONNECTING.into()))
                })
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
        if reconnecting {
            tracing::info!(%account, "reconnecting");
            let _ = events.try_send(Event::Disconnected {
                error: error.to_string(),
                retry_in: Duration::ZERO,
            });
            delay = config.retry_min;
            continue;
        }
        tracing::info!(%account, %error, retry_in = ?delay, "disconnected");
        let _ = events.try_send(Event::Disconnected {
            error: error.to_string(),
            retry_in: delay,
        });
        let offline = format!("offline: {error}");
        let wait = async {
            Timer::after(delay).await;
            None
        }
        .or(async { Some(control.signal_offline(&offline).await) })
        .or(async {
            control.reconnect_requested().await;
            Some(Signal::Wake)
        });
        match wait.await {
            None => delay = (delay * 2).min(config.retry_max),
            // Sync now or a network change: reconnect at once, starting the
            // waits over.
            Some(Signal::Wake) => delay = config.retry_min,
            Some(_) => return,
        }
    }
}

/// Runs the POP3 worker for `account` until its [`Handle`] is dropped.
/// Reports each check as a sync of `INBOX`.
pub async fn run_pop3<C: Pop3Connect>(
    connector: C,
    mut store: Store,
    account: AccountId,
    keep: Pop3Keep,
    config: WorkerConfig,
    events: Sender<Event>,
    control: Control,
) {
    let mut delay = config.retry_min;
    loop {
        // Stopping mid-session is safe: every stored message is committed,
        // and without QUIT the server deletes nothing.
        control.clear_reconnect();
        let checked =
            async { Some(pop3_check(&connector, &mut store, account, &keep, &events).await) }
                .or(async {
                    control.stopped().await;
                    None
                })
                .or(async {
                    // A check on a dead connection would hang until it
                    // times out; start over.
                    control.reconnect_requested().await;
                    Some(Err(Error::Closed(RECONNECTING.into())))
                })
                .await;
        let wait = match checked {
            None => return,
            Some(Ok(report)) => {
                delay = config.retry_min;
                tracing::debug!(%account, ?report, "POP3 check");
                let _ = events.try_send(Event::Synced(vec![FolderReport {
                    path: pop3_sync::FOLDERS[0].0.to_owned(),
                    added: report.added,
                    ..FolderReport::default()
                }]));
                config.pop3_interval
            }
            Some(Err(Error::Auth(message))) => {
                tracing::warn!(%account, %message, "login refused; waiting for sync now");
                let _ = events.try_send(Event::AuthFailed(message));
                Duration::MAX
            }
            Some(Err(Error::Closed(reason))) if reason == RECONNECTING => {
                delay = config.retry_min;
                Duration::ZERO
            }
            Some(Err(error)) => {
                tracing::info!(%account, %error, retry_in = ?delay, "POP3 check failed");
                let _ = events.try_send(Event::Disconnected {
                    error: error.to_string(),
                    retry_in: delay,
                });
                let wait = delay;
                delay = (delay * 2).min(config.retry_max);
                wait
            }
        };
        if !pop3_wait(&control, wait).await {
            return;
        }
    }
}

async fn pop3_check<C: Pop3Connect>(
    connector: &C,
    store: &mut Store,
    account: AccountId,
    keep: &Pop3Keep,
    events: &Sender<Event>,
) -> Result<Pop3Report> {
    let maildrop = connector.connect().await?;
    let _ = events.try_send(Event::Connected);
    pop3_sync::sync_account(maildrop, store, account, keep, unix_now(), |_| {
        let _ = events.try_send(Event::BodiesStored(PROGRESS_EVERY));
    })
    .await
}

/// Waits `wait`, or until a wake-up. Returns `false` when stopped.
async fn pop3_wait(control: &Control, wait: Duration) -> bool {
    let deadline = Instant::now().checked_add(wait);
    loop {
        let left = deadline.map_or(Duration::MAX, |d| {
            d.saturating_duration_since(Instant::now())
        });
        if left.is_zero() {
            return true;
        }
        let signal = async {
            Timer::after(left).await;
            None
        }
        .or(async { Some(control.signal().await) })
        .or(async {
            control.reconnect_requested().await;
            Some(Signal::Wake)
        })
        .await;
        match signal {
            None | Some(Signal::Wake) => return true,
            Some(Signal::Stop) => return false,
            // Changes to POP3 mail are local only.
            Some(Signal::Changes) => {}
            Some(Signal::Fetch(request)) => {
                // Never asked: POP3 messages are stored whole.
                let _ = request.done.try_send(Err(Error::Rejected(
                    "POP3 messages are always downloaded whole".into(),
                )));
            }
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
        // Changes go out first, so a sync never overwrites them.
        let report = ops::replay(backend, store, account, unix_now()).await?;
        let resync = report.resync.clone();
        if report != ReplayReport::default() {
            let _ = events.try_send(Event::ChangesSent(report));
        }
        if full || last_full.elapsed() >= config.full_sync_interval {
            let reports = engine::sync_account(backend, store, account).await?;
            let _ = events.try_send(Event::Synced(reports));
            *synced = true;
            full = false;
            last_full = Instant::now();
            if !control.metered() {
                download_all(backend, store, account, config, events).await?;
            }
        } else {
            // Moves the server gave no new UIDs for.
            let mut reports = Vec::new();
            for (folder, path) in resync {
                reports.push(engine::sync_folder(backend, store, account, folder, &path).await?);
            }
            if !reports.is_empty() {
                let _ = events.try_send(Event::Synced(reports));
            }
        }
        let Some(inbox) = store
            .folders(account)?
            .into_iter()
            .find(|f| f.role == Some(FolderRole::Inbox))
        else {
            // Nothing to watch: wait for the next full sync.
            let next = config
                .full_sync_interval
                .saturating_sub(last_full.elapsed())
                .min(until_retry(store, account)?);
            let wait = async {
                Timer::after(next).await;
                None
            }
            .or(async { Some(control.signal().await) });
            match wait.await {
                None => {}
                Some(Signal::Stop) => return Ok(()),
                Some(Signal::Wake) => full = true,
                Some(Signal::Changes) => {}
                Some(Signal::Fetch(request)) => {
                    serve(backend, store, account, events, request).await?;
                }
            }
            continue;
        };

        // Catch up on the inbox, which also selects it for IDLE.
        let report = engine::sync_folder(backend, store, account, inbox.id, &inbox.path).await?;
        if report.added + report.flags_changed + report.removed > 0 || report.reset {
            let _ = events.try_send(Event::Synced(vec![report]));
        }
        if !control.metered() {
            let stored = bodies::download_bodies(
                backend,
                store,
                inbox.id,
                &inbox.path,
                &config.offline,
                unix_now(),
            )
            .await?;
            if stored > 0 {
                let _ = events.try_send(Event::BodiesStored(stored));
            }
        }

        let max_wait = config
            .full_sync_interval
            .saturating_sub(last_full.elapsed())
            .min(config.idle_timeout)
            .min(until_retry(store, account)?);
        if max_wait.is_zero() {
            continue;
        }
        let wait = backend.wait_for_changes(max_wait, control.signal()).await?;
        match wait.interrupted {
            None => {}
            Some(Signal::Stop) => return Ok(()),
            Some(Signal::Wake) => full = true,
            Some(Signal::Changes) => {}
            // Fetching selects another folder; the next round of the loop
            // selects the inbox again.
            Some(Signal::Fetch(request)) => {
                serve(backend, store, account, events, request).await?;
            }
        }
    }
}

/// How long until a refused change is tried again; [`Duration::MAX`] when
/// none waits.
fn until_retry(store: &Store, account: AccountId) -> Result<Duration> {
    Ok(match store.next_op_due(account)? {
        // Anything due now was just sent; the rest waits at least a second
        // so a busy server does not spin the loop.
        Some(due) => Duration::from_secs(due.saturating_sub(unix_now()).max(1) as u64),
        None => Duration::MAX,
    })
}

/// Downloads the offline window of every folder, the inbox first.
async fn download_all<B: MailBackend>(
    backend: &mut B,
    store: &mut Store,
    account: AccountId,
    config: &WorkerConfig,
    events: &Sender<Event>,
) -> Result<()> {
    let mut folders = store.folders(account)?;
    folders.sort_by_key(|f| f.role != Some(FolderRole::Inbox));
    let now = unix_now();
    let mut stored = 0;
    for folder in folders {
        match bodies::download_bodies(
            backend,
            store,
            folder.id,
            &folder.path,
            &config.offline,
            now,
        )
        .await
        {
            Ok(count) => stored += count,
            Err(Error::Rejected(reason)) => {
                tracing::info!(path = folder.path, %reason, "skipping bodies");
            }
            Err(error) => return Err(error),
        }
    }
    if stored > 0 {
        let _ = events.try_send(Event::BodiesStored(stored));
    }
    Ok(())
}

/// Answers one body request. Only errors that break the connection end
/// the session.
async fn serve<B: MailBackend>(
    backend: &mut B,
    store: &mut Store,
    account: AccountId,
    events: &Sender<Event>,
    request: BodyRequest,
) -> Result<()> {
    let result = bodies::fetch_body(backend, store, account, request.message).await;
    let fatal = match &result {
        Ok(()) => {
            let _ = events.try_send(Event::BodiesStored(1));
            None
        }
        Err(error) if error.is_fatal() => Some(error.to_string()),
        Err(_) => None,
    };
    let _ = request.done.try_send(result);
    match fatal {
        Some(message) => Err(Error::Closed(message)),
        None => Ok(()),
    }
}

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs() as i64)
}
