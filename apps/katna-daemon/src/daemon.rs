// SPDX-License-Identifier: GPL-3.0-or-later

//! The daemon's state: one sync worker per account, what each is doing, the
//! outbox, and the account commands behind the D-Bus API.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard, Weak},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use async_channel::{Receiver, Sender};
use futures_lite::FutureExt;
use katna_core::{
    Account, AccountId, AccountKind, AccountSettings, Paths, Pop3Keep, Security, Server,
};
use katna_dbus::{AccountStatus, NewImapAccount, NewPop3Account, OutboxItem, ServerSpec, state};
use katna_store::{FolderId, MessageFlags, MessageId, Mode, SendState, Store};
use katna_sync::{
    Credentials, Endpoint, MailBackend,
    autoconfig::Discovery,
    net::Tls,
    ops::{self, ChangeError},
    outbox::{self, OutboxConfig, OutboxEvent, OutboxHandle, Outgoing, QueueError},
    pop3::{self, Pop3Client},
    smtp::SmtpSender,
    worker::{self, Connector, Event, ImapConnector, Pop3Connector, WorkerConfig},
};

use crate::secrets::Secrets;

/// How long a stopping worker may take to log out.
const STOP_TIMEOUT: Duration = Duration::from_secs(10);

/// Something D-Bus clients should hear about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Notice {
    AccountsChanged,
    StatusChanged(AccountId),
    MailChanged(AccountId),
    /// An outbox entry changed state.
    OutboxChanged(i64),
}

/// Why a command failed. Mapped to `org.freedesktop.DBus.Error.*` names.
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("{0}")]
    InvalidArgs(String),
    #[error("{0}")]
    AuthFailed(String),
    #[error("no account {0}")]
    UnknownAccount(i64),
    #[error("no message {0}")]
    UnknownMessage(i64),
    #[error("no folder {0}")]
    UnknownFolder(i64),
    #[error("{0}")]
    Failed(String),
}

impl From<ChangeError> for CommandError {
    fn from(err: ChangeError) -> Self {
        match err {
            ChangeError::UnknownMessage(id) => Self::UnknownMessage(id),
            ChangeError::UnknownFolder(id) => Self::UnknownFolder(id),
            ChangeError::NotPossible(reason) => Self::Failed(reason),
            ChangeError::Store(err) => err.into(),
        }
    }
}

impl From<QueueError> for CommandError {
    fn from(err: QueueError) -> Self {
        match err {
            QueueError::Invalid(reason) => Self::InvalidArgs(reason),
            QueueError::Store(err) => err.into(),
        }
    }
}

impl From<katna_store::Error> for CommandError {
    fn from(err: katna_store::Error) -> Self {
        Self::Failed(format!("store: {err}"))
    }
}

impl From<crate::secrets::Error> for CommandError {
    fn from(err: crate::secrets::Error) -> Self {
        Self::Failed(err.to_string())
    }
}

#[derive(Debug, Clone)]
struct Status {
    state: &'static str,
    detail: String,
    last_sync: i64,
}

impl Status {
    fn new(state: &'static str, detail: impl Into<String>) -> Self {
        Self {
            state,
            detail: detail.into(),
            last_sync: 0,
        }
    }
}

struct Running {
    handle: worker::Handle,
    task: smol::Task<()>,
}

struct Sending {
    handle: OutboxHandle,
    task: smol::Task<()>,
}

/// Shared state of a running daemon.
pub struct Daemon {
    paths: Paths,
    store: Mutex<Store>,
    secrets: Secrets,
    config: WorkerConfig,
    workers: Mutex<HashMap<AccountId, Running>>,
    status: Mutex<HashMap<AccountId, Status>>,
    outbox: Mutex<Option<Sending>>,
    /// Why each outbox entry's last try failed.
    send_errors: Mutex<HashMap<i64, String>>,
    notices: Sender<Notice>,
}

impl Daemon {
    /// Opens the store for writing. Workers start with [`Daemon::start`].
    pub fn new(
        paths: Paths,
        secrets: Secrets,
        config: WorkerConfig,
    ) -> katna_store::Result<(Arc<Self>, Receiver<Notice>)> {
        let store = Store::open(&paths, Mode::ReadWrite)?;
        let (notices, receiver) = async_channel::unbounded();
        let daemon = Arc::new(Self {
            paths,
            store: Mutex::new(store),
            secrets,
            config,
            workers: Mutex::default(),
            status: Mutex::default(),
            outbox: Mutex::default(),
            send_errors: Mutex::default(),
            notices,
        });
        Ok((daemon, receiver))
    }

    /// A sender for notices from outside the workers (the backfill).
    pub(crate) fn notifier(&self) -> Sender<Notice> {
        self.notices.clone()
    }

    /// Starts a worker for every account, and the outbox.
    pub async fn start(self: &Arc<Self>) -> Result<(), CommandError> {
        let accounts = self.store().accounts()?;
        tracing::info!(accounts = accounts.len(), "starting");
        for account in accounts {
            self.start_account(&account).await;
        }
        self.start_outbox()?;
        Ok(())
    }

    /// Stops the outbox and every worker; each logs out.
    pub async fn shutdown(&self) {
        let sending = self.outbox.lock().unwrap().take();
        if let Some(sending) = sending {
            // A message being handed over finishes first.
            drop(sending.handle);
            wait_for(AccountId(0), sending.task).await;
        }
        let workers: Vec<_> = self.workers().drain().collect();
        // Signal every worker first, so they log out at the same time.
        let tasks: Vec<_> = workers
            .into_iter()
            .map(|(account, running)| {
                drop(running.handle);
                (account, running.task)
            })
            .collect();
        for (account, task) in tasks {
            wait_for(account, task).await;
        }
    }

    /// Every account with its sync state.
    pub fn accounts(&self) -> Result<Vec<AccountStatus>, CommandError> {
        let accounts = self.store().accounts()?;
        let status = self.status.lock().unwrap();
        Ok(accounts
            .into_iter()
            .map(|account| {
                let current = status
                    .get(&account.id)
                    .cloned()
                    .unwrap_or_else(|| Status::new(state::NOT_SYNCED, ""));
                AccountStatus {
                    id: account.id.0,
                    kind: account.kind.to_string(),
                    display_name: account.display_name,
                    address: account.address,
                    state: current.state.to_owned(),
                    detail: current.detail,
                    last_sync: current.last_sync,
                }
            })
            .collect())
    }

    /// Checks the login, then adds the account and starts syncing it.
    pub async fn add_imap_account(
        self: &Arc<Self>,
        new: NewImapAccount,
        password: String,
    ) -> Result<AccountId, CommandError> {
        let address = new.address.trim().to_owned();
        if address.is_empty() {
            return Err(CommandError::InvalidArgs("the address is empty".into()));
        }
        let imap = server(&new.imap, &address)?
            .ok_or_else(|| CommandError::InvalidArgs("the IMAP server is missing".into()))?;
        let smtp = server(&new.smtp, &address)?;
        check_login(AccountKind::Imap, &imap, &password).await?;

        let settings = AccountSettings {
            imap: Some(imap),
            smtp,
            ..AccountSettings::default()
        };
        self.save_account(
            AccountKind::Imap,
            &new.display_name,
            address,
            settings,
            password,
        )
        .await
    }

    /// Checks the login, then adds the POP3 account and starts checking
    /// it for mail.
    pub async fn add_pop3_account(
        self: &Arc<Self>,
        new: NewPop3Account,
        password: String,
    ) -> Result<AccountId, CommandError> {
        let address = new.address.trim().to_owned();
        if address.is_empty() {
            return Err(CommandError::InvalidArgs("the address is empty".into()));
        }
        let pop3 = server(&new.pop3, &address)?
            .ok_or_else(|| CommandError::InvalidArgs("the POP3 server is missing".into()))?;
        let smtp = server(&new.smtp, &address)?;
        check_login(AccountKind::Pop3, &pop3, &password).await?;

        let settings = AccountSettings {
            pop3: Some(pop3),
            smtp,
            pop3_keep: Pop3Keep {
                leave_on_server: new.leave_on_server,
                days: (new.keep_days > 0).then_some(new.keep_days),
                delete_with_local: new.delete_with_local,
            },
            ..AccountSettings::default()
        };
        self.save_account(
            AccountKind::Pop3,
            &new.display_name,
            address,
            settings,
            password,
        )
        .await
    }

    /// Stores a checked account and its password, and starts its worker.
    async fn save_account(
        self: &Arc<Self>,
        kind: AccountKind,
        display_name: &str,
        address: String,
        settings: AccountSettings,
        password: String,
    ) -> Result<AccountId, CommandError> {
        let name = match display_name.trim() {
            "" => address.clone(),
            name => name.to_owned(),
        };
        let account = {
            let mut store = self.store();
            let account = store.add_account(kind, &name, &address)?;
            store.set_account_settings(account.id, &settings)?;
            if kind == AccountKind::Pop3 {
                // So the apps show its folders before the first check.
                pop3::sync::ensure_folders(&mut store, account.id)
                    .map_err(|err| CommandError::Failed(err.to_string()))?;
            }
            account
        };
        if let Err(err) = self
            .secrets
            .set_password(account.id, &address, &password)
            .await
        {
            // Without the password the account could never sync.
            self.store().remove_account(account.id)?;
            return Err(CommandError::Failed(format!(
                "{err}. Is a keyring (KWallet or GNOME Keyring) running?"
            )));
        }
        tracing::info!(account = %account.id, %address, "account added");
        let _ = self.notices.try_send(Notice::AccountsChanged);
        self.start_account(&account).await;
        Ok(account.id)
    }

    /// Finds the servers of `address`.
    pub async fn discover_account(
        &self,
        address: &str,
    ) -> Result<(NewImapAccount, &'static str), CommandError> {
        let discovery =
            Discovery::system().map_err(|err| CommandError::Failed(format!("TLS setup: {err}")))?;
        let found = discovery
            .discover(address)
            .await
            .map_err(CommandError::Failed)?;
        tracing::info!(
            address,
            source = found.source.as_str(),
            host = found.imap.host,
            "discovered"
        );
        let account = NewImapAccount {
            display_name: String::new(),
            address: address.trim().to_owned(),
            imap: spec(&found.imap),
            smtp: found.smtp.as_ref().map(spec).unwrap_or_default(),
        };
        Ok((account, found.source.as_str()))
    }

    /// Checks and saves a new password, then restarts the account's worker.
    pub async fn set_password(
        self: &Arc<Self>,
        id: AccountId,
        password: String,
    ) -> Result<(), CommandError> {
        let account = self.account(id)?;
        let settings = self.store().account_settings(id)?.unwrap_or_default();
        let incoming = match account.kind {
            AccountKind::Pop3 => settings.pop3,
            _ => settings.imap,
        }
        .ok_or_else(|| {
            CommandError::InvalidArgs(format!("account {id} has no server to log in to"))
        })?;
        check_login(account.kind, &incoming, &password).await?;
        self.secrets
            .set_password(id, &account.address, &password)
            .await?;
        self.start_account(&account).await;
        Ok(())
    }

    /// Stops the account's worker and deletes the account, its mail and its
    /// password. Returns whether it existed.
    pub async fn remove_account(&self, id: AccountId) -> Result<bool, CommandError> {
        let running = self.workers().remove(&id);
        if let Some(running) = running {
            stop(id, running).await;
        }
        let existed = {
            let mut store = self.store();
            let folders = store.folders(id)?;
            let mut batch = store.mail_batch()?;
            for folder in &folders {
                batch.remove_folder(folder.id)?;
            }
            // Account IDs can be reused; its changes and mail must not
            // go out from the next account.
            batch.clear_ops(id)?;
            batch.clear_outbox(id)?;
            batch.clear_pop3(id)?;
            batch.commit()?;
            store.remove_account(id)?
        };
        if let Err(err) = self.secrets.delete(id).await {
            tracing::warn!(account = %id, %err, "could not delete the password");
        }
        self.status.lock().unwrap().remove(&id);
        if existed {
            tracing::info!(account = %id, "account removed");
            let _ = self.notices.try_send(Notice::AccountsChanged);
            let _ = self.notices.try_send(Notice::MailChanged(id));
        }
        Ok(existed)
    }

    /// Syncs every folder of `id` now, or of every account.
    pub async fn sync_now(self: &Arc<Self>, id: Option<AccountId>) -> Result<(), CommandError> {
        let accounts = match id {
            Some(id) => vec![self.account(id)?],
            None => self.store().accounts()?,
        };
        for account in accounts {
            let running = self
                .workers()
                .get(&account.id)
                .map(|running| running.handle.sync_now())
                .is_some();
            if !running {
                // For example, the password was missing at startup.
                self.start_account(&account).await;
            }
        }
        Ok(())
    }

    /// Downloads one message through its account's worker.
    pub async fn fetch_body(&self, message: MessageId) -> Result<(), CommandError> {
        let (account, stored) = {
            let store = self.store();
            let stored = store
                .messages_by_id(&[message])?
                .into_iter()
                .next()
                .ok_or(CommandError::UnknownMessage(message.0))?;
            let location = store.remote_location(message)?;
            (location.map(|(account, ..)| account), stored)
        };
        if stored.blob_hash.is_some() {
            return Ok(());
        }
        let account = account.ok_or_else(|| {
            CommandError::Failed(format!("message {} is not on a server", message.0))
        })?;
        let fetcher = self
            .workers()
            .get(&account)
            .map(|running| running.handle.fetcher())
            .ok_or_else(|| CommandError::Failed(format!("account {account} is not syncing")))?;
        match fetcher.fetch_body(message).await {
            Ok(()) => Ok(()),
            Err(katna_sync::Error::Rejected(reason)) => Err(CommandError::Failed(reason)),
            Err(err) => Err(CommandError::Failed(format!(
                "could not download message {}: {err}",
                message.0
            ))),
        }
    }

    /// Adds and removes flags on messages.
    pub fn set_flags(
        &self,
        messages: &[MessageId],
        add: MessageFlags,
        remove: MessageFlags,
    ) -> Result<(), CommandError> {
        self.change(|store| ops::set_flags(store, messages, add, remove))
    }

    /// Moves messages to another folder of their account.
    pub fn move_messages(&self, messages: &[MessageId], to: FolderId) -> Result<(), CommandError> {
        self.change(|store| ops::move_messages(store, messages, to))
    }

    /// Moves messages to the trash, or deletes them if they are there.
    pub fn delete_messages(&self, messages: &[MessageId]) -> Result<(), CommandError> {
        self.change(|store| ops::delete_messages(store, messages))
    }

    /// Moves messages to their account's archive.
    pub fn archive_messages(&self, messages: &[MessageId]) -> Result<(), CommandError> {
        self.change(|store| ops::archive_messages(store, messages))
    }

    /// Queues `raw` from `account` to be sent after `delay` seconds.
    pub fn queue_send(
        &self,
        account: AccountId,
        raw: &[u8],
        delay: u32,
    ) -> Result<i64, CommandError> {
        let has_smtp = self
            .store()
            .account_settings(account)?
            .is_some_and(|settings| settings.smtp.is_some());
        if !has_smtp {
            self.account(account)?;
            return Err(CommandError::InvalidArgs(format!(
                "account {account} has no SMTP server"
            )));
        }
        let id = outbox::queue(&mut self.store(), account, raw, delay, unix_now())?;
        tracing::info!(id, %account, delay, "queued to send");
        let _ = self.notices.try_send(Notice::OutboxChanged(id));
        if let Some(sending) = self.outbox.lock().unwrap().as_ref() {
            sending.handle.wake();
        }
        Ok(id)
    }

    /// Takes a queued message back, if it is not being sent yet.
    pub fn undo_send(&self, id: i64) -> Result<bool, CommandError> {
        let undone = outbox::cancel(&mut self.store(), id)?;
        if undone {
            tracing::info!(id, "send undone");
            let _ = self.notices.try_send(Notice::OutboxChanged(id));
        }
        Ok(undone)
    }

    /// Forgets a cancelled or failed message.
    pub fn discard_send(&self, id: i64) -> Result<bool, CommandError> {
        let discarded = outbox::discard(&mut self.store(), id)?;
        if discarded {
            self.send_errors.lock().unwrap().remove(&id);
            let _ = self.notices.try_send(Notice::OutboxChanged(id));
        }
        Ok(discarded)
    }

    /// Every outbox entry.
    pub fn outbox(&self) -> Result<Vec<OutboxItem>, CommandError> {
        let entries = self.store().outbox()?;
        let errors = self.send_errors.lock().unwrap();
        Ok(entries
            .into_iter()
            .map(|entry| OutboxItem {
                id: entry.id,
                account: entry.account.0,
                message: entry.message.0,
                subject: entry.subject,
                send_at: entry.send_at,
                state: entry.state.as_str().to_owned(),
                detail: errors.get(&entry.id).cloned().unwrap_or_default(),
            })
            .collect())
    }

    fn start_outbox(self: &Arc<Self>) -> Result<(), CommandError> {
        let store = Store::open(&self.paths, Mode::ReadWrite)?;
        let (handle, control) = outbox::control();
        let (events, received) = async_channel::unbounded();
        let smtp = SmtpAccounts(Arc::downgrade(self));
        let task = smol::spawn(outbox::run(
            smtp,
            store,
            OutboxConfig::default(),
            events,
            control,
        ));
        smol::spawn(self.clone().forward_sends(received)).detach();
        *self.outbox.lock().unwrap() = Some(Sending { handle, task });
        Ok(())
    }

    /// Turns outbox events into notices, and has the worker file sent mail.
    async fn forward_sends(self: Arc<Self>, events: Receiver<OutboxEvent>) {
        while let Ok(event) = events.recv().await {
            if event.detail.is_empty() {
                self.send_errors.lock().unwrap().remove(&event.id);
            } else {
                self.send_errors
                    .lock()
                    .unwrap()
                    .insert(event.id, event.detail);
            }
            if event.state == SendState::Sent
                && let Some(running) = self.workers().get(&event.account)
            {
                running.handle.send_changes();
            }
            let _ = self.notices.try_send(Notice::OutboxChanged(event.id));
        }
    }

    /// Makes a change in the store, tells clients, and has the workers send
    /// it to the servers.
    fn change(
        &self,
        change: impl FnOnce(&mut Store) -> Result<Vec<AccountId>, ChangeError>,
    ) -> Result<(), CommandError> {
        let accounts = change(&mut self.store())?;
        let workers = self.workers();
        for account in accounts {
            let _ = self.notices.try_send(Notice::MailChanged(account));
            // Without a worker the change waits for the next start.
            if let Some(running) = workers.get(&account) {
                running.handle.send_changes();
            }
        }
        Ok(())
    }

    fn account(&self, id: AccountId) -> Result<Account, CommandError> {
        self.store()
            .accounts()?
            .into_iter()
            .find(|account| account.id == id)
            .ok_or(CommandError::UnknownAccount(id.0))
    }

    /// Starts (or restarts) the worker of `account`.
    async fn start_account(self: &Arc<Self>, account: &Account) {
        let old = self.workers().remove(&account.id);
        if let Some(old) = old {
            stop(account.id, old).await;
        }
        match self.connector(account).await {
            Ok(Some(link)) => self.spawn_worker(account.id, link),
            Ok(None) => self.set_status(account.id, Status::new(state::NOT_SYNCED, "")),
            Err(detail) => {
                tracing::warn!(account = %account.id, %detail, "not syncing");
                self.set_status(account.id, Status::new(state::AUTH_FAILED, detail));
            }
        }
    }

    /// How to reach the account's IMAP or POP3 server; `None` if it has
    /// none.
    async fn connector(&self, account: &Account) -> Result<Option<Link>, String> {
        let settings = self
            .store()
            .account_settings(account.id)
            .map_err(|err| err.to_string())?
            .unwrap_or_default();
        let server = match account.kind {
            AccountKind::Imap => settings.imap,
            AccountKind::Pop3 => settings.pop3,
            _ => None,
        };
        let Some(server) = server else {
            return Ok(None);
        };
        let password = self
            .secrets
            .password(account.id)
            .await
            .map_err(|err| err.to_string())?
            .ok_or("no password saved; set one with katnactl password")?;
        let (endpoint, tls) = endpoint(&server)?;
        let credentials = Credentials::new(server.username.clone(), &password);
        Ok(Some(match account.kind {
            AccountKind::Pop3 => Link::Pop3(
                Pop3Connector {
                    endpoint,
                    credentials,
                    tls,
                },
                settings.pop3_keep,
            ),
            _ => Link::Imap(ImapConnector {
                endpoint,
                credentials,
                tls,
            }),
        }))
    }

    fn spawn_worker(self: &Arc<Self>, id: AccountId, link: Link) {
        let store = match Store::open(&self.paths, Mode::ReadWrite) {
            Ok(store) => store,
            Err(err) => {
                self.set_status(id, Status::new(state::OFFLINE, err.to_string()));
                return;
            }
        };
        let (handle, control) = worker::control();
        let (events, received) = async_channel::unbounded();
        let config = self.config.clone();
        let task = match link {
            Link::Imap(connector) => {
                smol::spawn(worker::run(connector, store, id, config, events, control))
            }
            Link::Pop3(connector, keep) => smol::spawn(worker::run_pop3(
                connector, store, id, keep, config, events, control,
            )),
        };
        // Ends when the worker does and drops its event sender.
        smol::spawn(self.clone().forward(id, received)).detach();
        self.set_status(id, Status::new(state::CONNECTING, ""));
        self.workers().insert(id, Running { handle, task });
    }

    /// Turns a worker's events into status and notices.
    async fn forward(self: Arc<Self>, id: AccountId, events: Receiver<Event>) {
        while let Ok(event) = events.recv().await {
            let mut status = self
                .status
                .lock()
                .unwrap()
                .get(&id)
                .cloned()
                .unwrap_or_else(|| Status::new(state::CONNECTING, ""));
            match event {
                Event::Connected => {
                    status.state = state::CONNECTING;
                    status.detail.clear();
                }
                Event::Synced(reports) => {
                    status.state = state::ONLINE;
                    status.detail.clear();
                    status.last_sync = unix_now();
                    let changed = reports.iter().any(|report| {
                        report.added + report.flags_changed + report.removed + report.backfilled > 0
                            || report.reset
                    });
                    if changed {
                        let _ = self.notices.try_send(Notice::MailChanged(id));
                    }
                }
                Event::BodiesStored(_) => {
                    let _ = self.notices.try_send(Notice::MailChanged(id));
                    continue;
                }
                Event::ChangesSent(report) => {
                    // Refused changes were undone in the store.
                    if report.failed > 0 {
                        let _ = self.notices.try_send(Notice::MailChanged(id));
                    }
                    continue;
                }
                Event::Disconnected { error, retry_in } => {
                    status.state = state::OFFLINE;
                    status.detail = format!("{error}; retrying in {} s", retry_in.as_secs());
                }
                Event::AuthFailed(message) => {
                    status.state = state::AUTH_FAILED;
                    status.detail = message;
                }
            }
            // A removed account's last events must not bring it back.
            if self.workers().contains_key(&id) {
                self.set_status(id, status);
            }
        }
    }

    fn set_status(&self, id: AccountId, status: Status) {
        self.status.lock().unwrap().insert(id, status);
        let _ = self.notices.try_send(Notice::StatusChanged(id));
    }

    fn store(&self) -> MutexGuard<'_, Store> {
        self.store.lock().unwrap()
    }

    fn workers(&self) -> MutexGuard<'_, HashMap<AccountId, Running>> {
        self.workers.lock().unwrap()
    }
}

/// How a worker reaches its account's incoming server.
enum Link {
    Imap(ImapConnector),
    Pop3(Pop3Connector, Pop3Keep),
}

/// Drops the handle and waits for the worker to log out.
async fn stop(account: AccountId, running: Running) {
    drop(running.handle);
    wait_for(account, running.task).await;
}

async fn wait_for(account: AccountId, task: smol::Task<()>) {
    let stopped = async {
        task.await;
        true
    }
    .or(async {
        async_io::Timer::after(STOP_TIMEOUT).await;
        false
    })
    .await;
    if !stopped {
        tracing::warn!(%account, "worker did not stop in time");
    }
}

/// A server as D-Bus sends it.
fn spec(server: &Server) -> ServerSpec {
    ServerSpec {
        host: server.host.clone(),
        port: server.port,
        security: server.security.to_string(),
        username: server.username.clone(),
        accept_invalid_certs: server.accept_invalid_certs,
    }
}

/// Parses a server from D-Bus; `None` if its host is empty.
fn server(spec: &ServerSpec, address: &str) -> Result<Option<Server>, CommandError> {
    let host = spec.host.trim();
    if host.is_empty() {
        return Ok(None);
    }
    if spec.port == 0 {
        return Err(CommandError::InvalidArgs(format!("no port for {host}")));
    }
    let security: Security = spec.security.parse().map_err(CommandError::InvalidArgs)?;
    let username = match spec.username.trim() {
        "" => address.to_owned(),
        name => name.to_owned(),
    };
    Ok(Some(Server {
        host: host.to_owned(),
        port: spec.port,
        security,
        username,
        accept_invalid_certs: spec.accept_invalid_certs,
    }))
}

fn endpoint(server: &Server) -> Result<(Endpoint, Tls), String> {
    let security = match server.security {
        Security::Tls => katna_sync::Security::Tls,
        Security::StartTls => katna_sync::Security::StartTls,
        Security::Plain => katna_sync::Security::Plain,
    };
    let tls = if server.accept_invalid_certs {
        Tls::insecure_for_local_tests()
    } else {
        Tls::system().map_err(|err| format!("TLS setup: {err}"))?
    };
    Ok((
        Endpoint::new(server.host.clone(), server.port, security),
        tls,
    ))
}

/// The outbox's way to each account's SMTP server.
struct SmtpAccounts(Weak<Daemon>);

impl SmtpAccounts {
    fn settings(&self, account: AccountId) -> katna_sync::Result<(Arc<Daemon>, AccountSettings)> {
        let daemon = self
            .0
            .upgrade()
            .ok_or_else(|| katna_sync::Error::Closed("the daemon is stopping".into()))?;
        let settings = daemon
            .store()
            .account_settings(account)
            .map_err(|err| katna_sync::Error::Protocol(format!("store: {err}")))?
            .unwrap_or_default();
        Ok((daemon, settings))
    }
}

impl Outgoing for SmtpAccounts {
    type Sender = SmtpSender;

    async fn connect(&self, account: AccountId) -> katna_sync::Result<SmtpSender> {
        let (daemon, settings) = self.settings(account)?;
        let smtp = settings.smtp.ok_or_else(|| {
            katna_sync::Error::Rejected(format!("account {account} has no SMTP server"))
        })?;
        let password = daemon
            .secrets
            .password(account)
            .await
            .map_err(|err| katna_sync::Error::Auth(err.to_string()))?
            .ok_or_else(|| katna_sync::Error::Auth("no password saved".into()))?;
        drop(daemon);
        let (endpoint, tls) = endpoint(&smtp).map_err(katna_sync::Error::Tls)?;
        let credentials = Credentials::new(smtp.username.clone(), &password);
        SmtpSender::connect(&endpoint, &credentials, tls).await
    }

    fn files_sent_mail(&self, account: AccountId) -> bool {
        // Gmail files everything sent through its SMTP server in Sent Mail.
        self.settings(account).is_ok_and(|(_, settings)| {
            settings.imap.is_some_and(|imap| {
                let host = imap.host.to_ascii_lowercase();
                ["gmail.com", "googlemail.com"]
                    .iter()
                    .any(|domain| host.strip_suffix(domain).is_some_and(|h| h.ends_with('.')))
            })
        })
    }
}

/// Logs in once to check the server and password.
async fn check_login(
    kind: AccountKind,
    server: &Server,
    password: &str,
) -> Result<(), CommandError> {
    let (endpoint, tls) = endpoint(server).map_err(CommandError::Failed)?;
    let credentials = Credentials::new(server.username.clone(), password);
    let result = match kind {
        AccountKind::Pop3 => match Pop3Client::connect(&endpoint, &credentials, tls).await {
            Ok(client) => {
                let _ = client.quit().await;
                Ok(())
            }
            Err(err) => Err(err),
        },
        _ => {
            let connector = ImapConnector {
                endpoint,
                credentials,
                tls,
            };
            match connector.connect().await {
                Ok(backend) => {
                    let _ = backend.logout().await;
                    Ok(())
                }
                Err(err) => Err(err),
            }
        }
    };
    match result {
        Ok(()) => Ok(()),
        Err(katna_sync::Error::Auth(message)) => Err(CommandError::AuthFailed(format!(
            "{} refused the login: {message}",
            server.host
        ))),
        Err(err) => Err(CommandError::Failed(format!(
            "could not connect to {}:{}: {err}",
            server.host, server.port
        ))),
    }
}

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs() as i64)
}
