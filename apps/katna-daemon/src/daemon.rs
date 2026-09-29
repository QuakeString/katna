// SPDX-License-Identifier: GPL-3.0-or-later

//! The daemon's state: one sync worker per account, what each is doing, the
//! outbox, and the account commands behind the D-Bus API.

use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex, MutexGuard, OnceLock, Weak,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use async_channel::{Receiver, Sender};
use futures_lite::FutureExt;
use katna_core::{
    Account, AccountId, AccountKind, AccountSettings, Config, Paths, Pop3Keep, Security, Server,
    config::Metered,
};
use katna_dbus::{
    AccountStatus, NewImapAccount, NewPop3Account, OutboxItem, ServerSpec, TemplateItem, state,
};
use katna_i18n::tr;
use katna_search::IndexerWaker;
use katna_store::{
    FolderId, Forgotten, MessageFlags, MessageId, Mode, SendState, Store, Template, TemplateFile,
    Translation,
};
use katna_sync::{
    Credentials, Endpoint, MailBackend, MailSender,
    autoconfig::{Discovered, Discovery},
    bodies,
    connection::Connection,
    net::Tls,
    oauth::TokenSource,
    ops::{self, ChangeError},
    outbox::{self, OutboxConfig, OutboxEvent, OutboxHandle, Outgoing, QueueError},
    pictures::Pictures,
    pop3::{self, Pop3Client},
    smtp::SmtpSender,
    tracking,
    worker::{self, Connector, Event, ImapConnector, Pop3Connector, WorkerConfig},
};

use crate::threads::{self, Threaded};
use crate::translate::{self, KatnaServer, TranslateError};
use crate::{desktop, notify::NewMailNotices, on_demand::OnDemand, secrets::Secrets};

pub(crate) mod alarms;
mod calendar;
mod contact_labels;
mod contacts;
mod contacts_import;
mod drive;
mod meet;
mod notes;
mod other_contacts;
mod reminders;

pub use reminders::{SNOOZED, is_snoozed_path};
pub(crate) use sign_in::open_in_browser;
mod sign_in;
mod tasks;

/// The longest account name taken.
const MAX_ACCOUNT_NAME: usize = 200;
/// The longest template name, in characters.
const MAX_TEMPLATE_NAME: usize = 200;
/// The most a template's attachments may hold, in bytes (the session bus
/// carries 32 MB at most).
const MAX_TEMPLATE_FILES: usize = 20_000_000;

/// A template name, trimmed; not empty and not too long.
fn template_name(name: &str) -> Result<String, CommandError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(CommandError::InvalidArgs(
            "a template needs a name".to_owned(),
        ));
    }
    if name.chars().count() > MAX_TEMPLATE_NAME {
        return Err(CommandError::InvalidArgs(format!(
            "a template name is at most {MAX_TEMPLATE_NAME} characters"
        )));
    }
    Ok(name.to_owned())
}

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
    /// Workers now act metered, or stopped doing so.
    MeteredChanged(bool),
    /// The Katna account this computer is signed in to changed.
    KatnaAccountChanged,
    /// A tracked message was opened or a link in it followed.
    TrackingChanged,
    /// Where an update of Katna stands changed.
    UpdateChanged,
    /// A Google Drive upload moved on.
    DriveChanged(i64),
    /// Calendars, their events or the calendar sync's state changed.
    CalendarChanged,
    /// Saved contacts changed.
    ContactsChanged,
    /// Task sync brought changes from a task service.
    TasksChanged,
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
    task: Threaded,
}

struct Sending {
    handle: OutboxHandle,
    task: Threaded,
}

/// Shared state of a running daemon.
pub struct Daemon {
    paths: Paths,
    store: Mutex<Store>,
    secrets: Secrets,
    config: WorkerConfig,
    workers: Mutex<HashMap<AccountId, Running>>,
    /// Whether workers act metered (the network as NetworkManager sees
    /// it, or the user's setting); new workers start with it.
    metered: AtomicBool,
    /// What NetworkManager last said.
    network_metered: AtomicBool,
    /// The user's `sync.metered` setting.
    metered_setting: Mutex<Metered>,
    /// The user's `sync.offline_days` setting; `None` for all mail.
    offline_days: Mutex<Option<u32>>,
    /// The user's `general.language` setting, as last applied.
    language: Mutex<String>,
    status: Mutex<HashMap<AccountId, Status>>,
    outbox: Mutex<Option<Sending>>,
    /// Why each outbox entry's last try failed.
    send_errors: Mutex<HashMap<i64, String>>,
    /// How long each account's SMTP server holds mail (FUTURERELEASE),
    /// once asked; `None` when it cannot.
    hold_limits: Mutex<HashMap<AccountId, Option<u64>>>,
    /// Whether each account's SMTP server sends delivery receipts, once
    /// asked.
    delivery_receipts: Mutex<HashMap<AccountId, bool>>,
    notices: Sender<Notice>,
    /// Connections for messages the user opens.
    on_demand: OnDemand,
    /// Desktop notifications, once a session bus has a server for them.
    new_mail: OnceLock<Arc<NewMailNotices>>,
    /// The taskbar count and the tray, once they run.
    desktop: OnceLock<desktop::Handle>,
    /// KRunner's and GNOME's search, for its trigger words.
    finder: OnceLock<Arc<crate::desktop_search::Finder>>,
    /// Set once all data is being deleted: nothing starts any more.
    closing: AtomicBool,
    /// Set while the cache is being reset: workers start once it is done.
    resetting: AtomicBool,
    /// Asks the search indexer to start over, once it runs.
    indexer: OnceLock<IndexerWaker>,
    /// Asks the [`crate::Instance`] to delete the files and exit; it
    /// answers on the sender inside.
    delete_requests: (Sender<DeleteDone>, Receiver<DeleteDone>),
    /// Tells the crash report sender that settings changed.
    crash_uploads: (Sender<()>, Receiver<()>),
    /// Access tokens of the accounts that sign in with OAuth2, shared by
    /// their connections.
    tokens: Mutex<HashMap<AccountId, Arc<TokenSource>>>,
    /// Refresh tokens a provider replaced, to save in the Secret Service.
    rotated: (Sender<Rotated>, Receiver<Rotated>),
    /// Ends the browser sign-in under way, if any.
    signing_in: Mutex<Option<Sender<()>>>,
    /// Files going up to Google Drive for messages.
    uploads: drive::Uploads,
    /// The calendar sync.
    calendars: calendar::Calendars,
    /// Tells the tracking event stream to look again (a tracked message
    /// went out, or settings changed).
    tracking_wake: (Sender<()>, Receiver<()>),
    /// Accounts whose notes changed here, for the notes sync
    /// ([`notes::run`]).
    notes_wake: (Sender<AccountId>, Receiver<AccountId>),
    /// Wakes the contacts sync.
    contacts_wake: (Sender<()>, Receiver<()>),
    /// Where each account's contacts sync stands: a
    /// `katna_dbus::contacts_state` and a detail.
    contacts_status: Mutex<HashMap<AccountId, (&'static str, String)>>,
    /// The languages Katna Server translates between, once asked.
    translation_languages: crate::translate::Languages,
    /// Wakes the scheduler of snooze and reminders, once it runs.
    scheduler: OnceLock<katna_meta::Waker>,
    /// Checks for, and downloads, new versions of Katna.
    updates: crate::updates::Updates,
    /// Has task sync run a round soon.
    task_sync_wake: (Sender<()>, Receiver<()>),
    /// Each account's last task sync result, for the Tasks page.
    tasks_status: Mutex<HashMap<AccountId, tasks::Status>>,
}

/// A refresh token that replaced the account's old one.
type Rotated = (AccountId, String);

/// Where the [`crate::Instance`] reports whether it deleted every file.
pub(crate) type DeleteDone = Sender<Result<(), String>>;

impl Daemon {
    /// Opens the store for writing. Workers start with [`Daemon::start`].
    pub fn new(
        paths: Paths,
        secrets: Secrets,
        config: WorkerConfig,
    ) -> katna_store::Result<(Arc<Self>, Receiver<Notice>)> {
        let store = Store::open(&paths, Mode::ReadWrite)?;
        let saved = settings(&paths);
        let sync = saved.sync;
        let setting = sync.metered;
        let (notices, receiver) = async_channel::unbounded();
        let daemon = Arc::new(Self {
            paths,
            store: Mutex::new(store),
            secrets,
            config,
            workers: Mutex::default(),
            metered: AtomicBool::new(setting.decide(false)),
            network_metered: AtomicBool::new(false),
            metered_setting: Mutex::new(setting),
            offline_days: Mutex::new(sync.offline_window()),
            language: Mutex::new(saved.general.language),
            status: Mutex::default(),
            outbox: Mutex::default(),
            send_errors: Mutex::default(),
            hold_limits: Mutex::default(),
            delivery_receipts: Mutex::default(),
            notices,
            on_demand: OnDemand::default(),
            new_mail: OnceLock::new(),
            desktop: OnceLock::new(),
            finder: OnceLock::new(),
            closing: AtomicBool::new(false),
            resetting: AtomicBool::new(false),
            indexer: OnceLock::new(),
            delete_requests: async_channel::bounded(1),
            crash_uploads: async_channel::bounded(1),
            tokens: Mutex::default(),
            rotated: async_channel::unbounded(),
            signing_in: Mutex::default(),
            uploads: drive::Uploads::default(),
            calendars: calendar::Calendars::default(),
            tracking_wake: async_channel::bounded(1),
            notes_wake: async_channel::unbounded(),
            contacts_wake: async_channel::bounded(1),
            contacts_status: Mutex::default(),
            translation_languages: Default::default(),
            scheduler: OnceLock::new(),
            updates: crate::updates::Updates::default(),
            task_sync_wake: async_channel::bounded(1),
            tasks_status: Mutex::default(),
        });
        Ok((daemon, receiver))
    }

    /// A sender for notices from outside the workers (the backfill).
    pub(crate) fn notifier(&self) -> Sender<Notice> {
        self.notices.clone()
    }

    /// Shows new-mail notifications through the desktop's notification
    /// server on `connection` (the session bus), as `notifications.new_mail`
    /// says. Call before [`Daemon::start`].
    pub async fn notify_new_mail(self: &Arc<Self>, connection: &zbus::Connection) {
        let notifications = settings(&self.paths).notifications;
        match NewMailNotices::new(connection, &notifications).await {
            Ok(notices) => {
                if self.new_mail.set(Arc::new(notices)).is_ok() {
                    smol::spawn(NewMailNotices::serve_actions(Arc::downgrade(self))).detach();
                }
            }
            Err(err) => tracing::warn!(%err, "no desktop notifications"),
        }
    }

    /// Where settings changes for the taskbar count and the tray go.
    pub(crate) fn set_desktop(&self, handle: desktop::Handle) {
        let _ = self.desktop.set(handle);
    }

    /// Where the desktop search's trigger words go when settings change.
    pub(crate) fn set_finder(&self, finder: Arc<crate::desktop_search::Finder>) {
        let _ = self.finder.set(finder);
    }

    /// The search indexer, for [`Daemon::reset_cache`].
    pub(crate) fn set_indexer(&self, waker: IndexerWaker) {
        let _ = self.indexer.set(waker);
    }

    pub(crate) fn new_mail_notices(&self) -> Option<Arc<NewMailNotices>> {
        self.new_mail.get().cloned()
    }

    /// Starts a worker for every account, the outbox and the scheduler of
    /// snooze and reminders.
    pub async fn start(self: &Arc<Self>) -> Result<(), CommandError> {
        let accounts = self.store().accounts()?;
        tracing::info!(accounts = accounts.len(), "starting");
        for account in accounts {
            self.start_account(&account).await;
        }
        self.start_outbox()?;
        self.start_calendars();
        smol::spawn(sign_in::save_rotated(
            Arc::downgrade(self),
            self.rotated.1.clone(),
        ))
        .detach();
        self.start_scheduler();
        smol::spawn(crate::crash_upload::run(
            Arc::downgrade(self),
            self.crash_uploads.1.clone(),
        ))
        .detach();
        smol::spawn(crate::tracking::run(
            Arc::downgrade(self),
            self.tracking_wake.1.clone(),
        ))
        .detach();
        smol::spawn(crate::updates::run(Arc::downgrade(self))).detach();
        // Their database calls block too (crate::threads).
        threads::detach(
            "katna-contacts",
            contacts::run(Arc::downgrade(self), self.contacts_wake.1.clone()),
        );
        threads::detach(
            "katna-tasks",
            tasks::run(Arc::downgrade(self), self.task_sync_wake.1.clone()),
        );
        threads::detach(
            "katna-notes",
            notes::run(Arc::downgrade(self), self.notes_wake.1.clone()),
        );
        threads::detach("katna-alarms", alarms::run(Arc::downgrade(self)));
        Ok(())
    }

    /// Has the tracking event stream look again.
    pub(crate) fn wake_tracking(&self) {
        let _ = self.tracking_wake.0.try_send(());
    }

    /// Syncs contacts now rather than at the next pass.
    pub(crate) fn wake_contacts(&self) {
        let _ = self.contacts_wake.0.try_send(());
    }

    /// Katna Server, for tracking ([`crate::katna_account::server_url`]).
    pub(crate) fn tracking_client(&self) -> Option<tracking::Client> {
        let server = tracking::Server::parse(&crate::katna_account::server_url())?;
        let tls = Tls::system()
            .map_err(|err| tracing::warn!(%err, "TLS setup failed; tracking waits"))
            .ok()?;
        Some(tracking::Client::new(server, tls))
    }

    /// This computer's Katna Server token, once it has registered. The
    /// server only takes it for tracking while signed in to a Katna
    /// account with a confirmed address (§16.2).
    pub(crate) async fn tracking_token(&self) -> Option<String> {
        let session = crate::katna_account::Session::new(&self.secrets).ok()?;
        match session.token().await {
            Ok(token) => token,
            Err(error) => {
                tracing::warn!(%error, "no Katna Server token");
                None
            }
        }
    }

    pub(crate) fn updates(&self) -> &crate::updates::Updates {
        &self.updates
    }

    pub(crate) fn notices(&self) -> &Sender<Notice> {
        &self.notices
    }

    pub(crate) fn paths(&self) -> &Paths {
        &self.paths
    }

    /// All data is being deleted.
    pub(crate) fn closing(&self) -> bool {
        self.closing.load(Ordering::SeqCst)
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
        let accounts: Vec<(Account, String)> = {
            let store = self.store();
            store
                .accounts()?
                .into_iter()
                .map(|account| {
                    let sign_in = store
                        .account_settings(account.id)
                        .ok()
                        .flatten()
                        .and_then(|settings| settings.oauth)
                        .map(|provider| provider.as_str().to_owned())
                        .unwrap_or_default();
                    (account, sign_in)
                })
                .collect()
        };
        let status = self.status.lock().unwrap();
        Ok(accounts
            .into_iter()
            .map(|(account, sign_in)| {
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
                    sign_in,
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
            None,
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
            None,
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
        grant: Option<&katna_sync::oauth::Grant>,
    ) -> Result<AccountId, CommandError> {
        if self.closing.load(Ordering::SeqCst) {
            // Katna Mail shows this in the add-account dialog.
            return Err(CommandError::Failed(tr!("daemon-deleting-data")));
        }
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
        if let (Some(grant), Some(provider)) = (grant, settings.oauth) {
            // Its first connections use the token the sign-in brought.
            match self.token_source(account.id, provider, password, Some(grant)) {
                Ok(tokens) => {
                    self.tokens
                        .lock()
                        .unwrap()
                        .insert(account.id, Arc::new(tokens));
                }
                Err(err) => tracing::warn!(%err, "no token source"),
            }
        }
        let _ = self.notices.try_send(Notice::AccountsChanged);
        self.start_account(&account).await;
        self.wake_calendars();
        self.wake_task_sync();
        Ok(account.id)
    }

    /// Finds the servers of `address`.
    pub async fn discover_account(
        &self,
        address: &str,
    ) -> Result<(NewImapAccount, Discovered), CommandError> {
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
        Ok((account, found))
    }

    /// Downloads a remote image for the reading pane.
    pub async fn fetch_image(&self, url: &str) -> Result<Vec<u8>, CommandError> {
        let pictures = Pictures::system(self.paths.cache_dir())
            .map_err(|err| CommandError::Failed(format!("TLS setup: {err}")))?;
        pictures
            .image(url)
            .await
            .map_err(|err| CommandError::Failed(err.to_string()))
    }

    /// This computer's Katna account on Katna Server.
    pub fn katna(&self) -> Result<crate::katna_account::Session<'_>, CommandError> {
        crate::katna_account::Session::new(&self.secrets)
    }

    /// Tells the apps the Katna account changed.
    pub fn katna_changed(&self) {
        let _ = self.notices.try_send(Notice::KatnaAccountChanged);
    }

    /// The picture of the sender `address`, or empty. Only looked up when
    /// the user's provider authenticated mail from the address's domain
    /// (DMARC or aligned DKIM in its `Authentication-Results`), so a forged
    /// `From` makes the daemon fetch nothing.
    pub async fn sender_picture(&self, address: &str) -> Result<Vec<u8>, CommandError> {
        let domain = address
            .rsplit_once('@')
            .map(|(_, domain)| domain.trim().trim_end_matches('.').to_ascii_lowercase())
            .unwrap_or_default();
        let authenticated = !domain.is_empty()
            && self
                .store()
                .sender_domain_authenticated(&domain)
                .unwrap_or_else(|err| {
                    tracing::warn!(%err, "reading sender authentication");
                    false
                });
        if !authenticated {
            return Ok(Vec::new());
        }
        let pictures = Pictures::system(self.paths.cache_dir())
            .map_err(|err| CommandError::Failed(format!("TLS setup: {err}")))?;
        Ok(pictures.sender(address).await)
    }

    /// Translates `text`, the plain text of `message` in language `source`
    /// (found by the app; `auto` when unclear), into `target` (LibreTranslate
    /// codes such as `en`): the language it was in and the translation,
    /// from the store when this text was translated before. Mail already
    /// in `target` is never sent.
    pub async fn translate(
        &self,
        message: MessageId,
        text: &str,
        source: &str,
        target: &str,
    ) -> Result<Translation, TranslateError> {
        let valid = |code: &str| {
            (2..=8).contains(&code.len())
                && code.bytes().all(|b| b.is_ascii_lowercase() || b == b'-')
        };
        if !valid(source) || !valid(target) {
            return Err(TranslateError::Server(format!(
                "bad language {source:?} or {target:?}"
            )));
        }
        // Before anything reaches the network, the store included.
        if katna_translate::same_language(source, target) {
            return Err(TranslateError::SameLanguage);
        }
        {
            let store = self.store();
            let known = store
                .messages_by_id(&[message])
                .map_err(|err| TranslateError::Server(err.to_string()))?;
            if known.is_empty() {
                return Err(TranslateError::Server(format!("no message {}", message.0)));
            }
            let cached = store
                .translation(message, target, text)
                .map_err(|err| TranslateError::Server(err.to_string()))?;
            if let Some(cached) = cached {
                return Ok(cached);
            }
        }
        let server = KatnaServer::connect(&self.secrets).await?;
        let done = translate::translate(&server, &self.translation_languages, text, source, target)
            .await?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs().try_into().unwrap_or(i64::MAX));
        if let Err(err) = self
            .store()
            .save_translation(message, target, text, &done, now)
        {
            tracing::warn!(%err, "keeping a translation");
        }
        tracing::info!(from = done.source, to = target, "message translated");
        Ok(done)
    }

    /// The languages Katna Server can translate into `target`.
    pub async fn translation_sources(&self, target: &str) -> Result<Vec<String>, TranslateError> {
        let server = KatnaServer::connect(&self.secrets).await?;
        self.translation_languages
            .sources(&server, target)
            .await
            .inspect_err(|err| tracing::info!(%err, "asking the server for its languages"))
    }

    /// Renames an account. An empty name goes back to the name its own
    /// mail carries, else its address.
    pub fn rename_account(&self, id: AccountId, name: &str) -> Result<(), CommandError> {
        let account = self.account(id)?;
        let name = match name.trim() {
            "" => self
                .store()
                .name_in_own_mail(id, &account.address)?
                .unwrap_or_else(|| account.address.clone()),
            name if name.chars().count() > MAX_ACCOUNT_NAME => {
                return Err(CommandError::InvalidArgs(format!(
                    "an account name is at most {MAX_ACCOUNT_NAME} characters"
                )));
            }
            name => name.to_owned(),
        };
        self.store().rename_account(id, &name)?;
        tracing::info!(account = %id, "account renamed");
        let _ = self.notices.try_send(Notice::AccountsChanged);
        Ok(())
    }

    /// An account known only by its address takes the name its own mail
    /// is sent under, once there is some.
    fn name_from_mail(&self, id: AccountId) {
        let Ok(account) = self.account(id) else {
            return;
        };
        let name = account.display_name.trim();
        if !name.is_empty() && !name.eq_ignore_ascii_case(account.address.trim()) {
            return;
        }
        let found = self.store().name_in_own_mail(id, &account.address);
        let Ok(Some(name)) = found else {
            return;
        };
        if self.store().rename_account(id, &name).is_ok() {
            let _ = self.notices.try_send(Notice::AccountsChanged);
        }
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
        // A password replaces an OAuth2 sign-in.
        if settings.oauth.is_some() {
            let mut settings = self.store().account_settings(id)?.unwrap_or_default();
            settings.oauth = None;
            self.store().set_account_settings(id, &settings)?;
            self.tokens.lock().unwrap().remove(&id);
        }
        self.start_account(&account).await;
        self.wake_task_sync();
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
            batch.set_quota(id, None, 0)?;
            batch.commit()?;
            store.remove_account(id)?
        };
        if let Err(err) = self.secrets.delete(id).await {
            tracing::warn!(account = %id, %err, "could not delete the password");
        }
        self.tokens.lock().unwrap().remove(&id);
        self.forget_calendars(id);
        let _ = std::fs::remove_file(sign_in::provider_picture(&self.paths, id));
        self.status.lock().unwrap().remove(&id);
        if let Some(notices) = self.new_mail_notices() {
            notices.forget(id);
        }
        if existed {
            tracing::info!(account = %id, "account removed");
            let _ = self.notices.try_send(Notice::AccountsChanged);
            let _ = self.notices.try_send(Notice::MailChanged(id));
        }
        Ok(existed)
    }

    /// Deletes everything Katna keeps on this computer: stops every
    /// account, deletes every saved password, then has the
    /// [`crate::Instance`] delete the databases, the search index, the
    /// cache and the settings file and exit. Mail servers are not touched.
    pub async fn delete_all_data(&self) -> Result<(), CommandError> {
        if self.closing.swap(true, Ordering::SeqCst) {
            return Err(CommandError::Failed(
                "all data is already being deleted".into(),
            ));
        }
        tracing::warn!("deleting all data, as the user asked");
        self.shutdown().await;
        let accounts = self.store().accounts()?;
        for account in accounts {
            if let Err(err) = self.secrets.delete(account.id).await {
                tracing::warn!(account = %account.id, %err, "could not delete the password");
            }
        }
        // Also passwords left from accounts removed before.
        self.secrets.delete_all().await?;
        let (done, result) = async_channel::bounded(1);
        self.delete_requests
            .0
            .send(done)
            .await
            .map_err(|_| CommandError::Failed("the service is stopping".into()))?;
        result
            .recv()
            .await
            .map_err(|_| CommandError::Failed("the service stopped".into()))?
            .map_err(CommandError::Failed)
    }

    /// Deletes what Katna downloaded and can download again: the bodies
    /// and attachments of mail still on the server, the search index and
    /// the sender pictures. Then syncs, which downloads the mail of the
    /// offline window again; other mail downloads when opened. Accounts,
    /// settings, flags, labels, pins and mail that exists only on this
    /// computer (drafts, the outbox, changes not yet on the server) stay.
    /// Nothing changes on the servers. Returns what was deleted.
    pub async fn reset_cache(self: &Arc<Self>) -> Result<Forgotten, CommandError> {
        if self.closing() {
            return Err(CommandError::Failed("all data is being deleted".into()));
        }
        if self.resetting.swap(true, Ordering::SeqCst) {
            return Err(CommandError::Failed(
                "the cache is already being reset".into(),
            ));
        }
        tracing::info!("resetting the cache, as the user asked");
        // Workers store bodies; none may run while they are deleted.
        let workers: Vec<_> = self.workers().drain().collect();
        for (account, running) in workers {
            stop(account, running).await;
        }
        let paths = self.paths.clone();
        let forgotten = smol::unblock(move || forget_downloaded(&paths)).await;
        if let Some(indexer) = self.indexer.get() {
            indexer.rebuild();
        }
        self.resetting.store(false, Ordering::SeqCst);
        match &forgotten {
            Ok(forgotten) => tracing::info!(
                messages = forgotten.messages,
                bytes = forgotten.bytes,
                "cache reset"
            ),
            Err(err) => tracing::warn!(%err, "resetting the cache"),
        }
        let accounts = self.store().accounts()?;
        for account in &accounts {
            self.start_account(account).await;
            let _ = self.notices.try_send(Notice::MailChanged(account.id));
        }
        forgotten
    }

    /// Requests from [`Daemon::delete_all_data`].
    pub(crate) fn delete_requests(&self) -> Receiver<DeleteDone> {
        self.delete_requests.1.clone()
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
        self.wake_calendars();
        self.wake_contacts();
        self.wake_task_sync();
        Ok(())
    }

    /// The network came back or the machine woke up: every worker drops
    /// its connection, which may be dead, and connects again at once.
    /// (Mail the outbox could not send is tried again every 30 s anyway.)
    pub fn network_changed(&self) {
        tracing::info!("network changed; reconnecting");
        for running in self.workers().values() {
            running.handle.reconnect();
        }
        // After a resume, snoozes may have ended while the timers slept.
        self.wake_scheduler();
    }

    /// NetworkManager says the network became metered or stopped being so.
    /// What workers do also depends on the user's setting
    /// ([`Daemon::reload_config`]).
    pub fn set_metered(&self, metered: bool) {
        tracing::info!(metered, "network metering changed");
        self.network_metered.store(metered, Ordering::Relaxed);
        self.apply_metered();
    }

    /// Whether workers act metered now.
    pub fn metered(&self) -> bool {
        self.metered.load(Ordering::Relaxed)
    }

    /// Reads the settings file again and applies what the daemon uses from
    /// it (`sync.metered`, `sync.offline_days`, `notifications`,
    /// the `general` language, tray and badge switches, search trigger words,
    /// `feedback.send_crash_reports`). Katna Mail calls this after saving
    /// settings.
    pub fn reload_config(&self) -> Result<(), CommandError> {
        let config = Config::load(&self.paths.config_file())
            .map_err(|err| CommandError::InvalidArgs(err.to_string()))?;
        tracing::info!(
            metered = ?config.sync.metered,
            offline_days = config.sync.offline_days,
            new_mail = config.notifications.new_mail,
            sound = config.notifications.sound,
            "settings reloaded"
        );
        *self.metered_setting.lock().unwrap() = config.sync.metered;
        let days = config.sync.offline_window();
        if std::mem::replace(&mut *self.offline_days.lock().unwrap(), days) != days {
            for running in self.workers().values() {
                running.handle.set_offline_days(days);
            }
        }
        if let Some(notices) = self.new_mail_notices() {
            notices.set(&config.notifications);
        }
        // Before the tray hears of it, so it rebuilds in the new language.
        let language = &config.general.language;
        if std::mem::replace(&mut *self.language.lock().unwrap(), language.clone()) != *language {
            katna_i18n::apply(language);
        }
        if let Some(finder) = self.finder.get() {
            finder.set_triggers(config.general.search_triggers.clone());
        }
        if let Some(desktop) = self.desktop.get() {
            desktop.settings(config.general.clone());
        }
        self.apply_metered();
        // Sending crash reports may have been turned on.
        let _ = self.crash_uploads.0.try_send(());
        // Downloading updates may have been turned on.
        self.updates.settings_changed();
        Ok(())
    }

    /// Workers download bodies ahead of time only when not metered, and
    /// catch up when that ends.
    fn apply_metered(&self) {
        let setting = *self.metered_setting.lock().unwrap();
        let metered = setting.decide(self.network_metered.load(Ordering::Relaxed));
        if self.metered.swap(metered, Ordering::Relaxed) == metered {
            return;
        }
        for running in self.workers().values() {
            running.handle.set_metered(metered);
        }
        let _ = self.notices.try_send(Notice::MeteredChanged(metered));
    }

    /// Downloads one message now, for a user who opened it. Runs on a
    /// connection of its own, kept for a while for the next one, so it
    /// never waits for the account's sync to finish.
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
        let session = self.on_demand.session(account);
        let mut connection = session.connection.lock().await;
        let mut store = Store::open(&self.paths, Mode::ReadWrite)?;
        // A kept connection may have been closed by the server meanwhile:
        // then once more on a new one.
        let mut result = Ok(());
        for _ in 0..2 {
            let fresh = connection.as_ref().is_none_or(Connection::is_closed);
            if fresh {
                *connection = Some(self.connect_on_demand(account).await?);
            }
            let mut handle = connection.clone().expect("connected above");
            result = bodies::fetch_body(&mut handle, &mut store, account, message).await;
            if !result.as_ref().is_err_and(katna_sync::Error::is_fatal) {
                break;
            }
            *connection = None;
            if fresh {
                break;
            }
        }
        drop(connection);
        self.on_demand.used(account, &session);
        match result {
            Ok(()) => {
                let _ = self.notices.try_send(Notice::MailChanged(account));
                Ok(())
            }
            Err(katna_sync::Error::Rejected(reason)) => Err(CommandError::Failed(reason)),
            Err(err) if err.is_fatal() => Err(CommandError::Failed(format!(
                "lost the connection to the mail server ({err})"
            ))),
            Err(err) => Err(CommandError::Failed(format!(
                "the mail server did not send it ({err})"
            ))),
        }
    }

    /// A new connection to the account's IMAP server, for
    /// [`Daemon::fetch_body`].
    async fn connect_on_demand(&self, account: AccountId) -> Result<Connection, CommandError> {
        let connector = match self.connector(&self.account(account)?).await {
            Ok(Some(Link::Imap(connector))) => connector,
            Ok(Some(Link::Pop3(..))) => {
                return Err(CommandError::Failed(
                    "POP3 messages are always downloaded whole".into(),
                ));
            }
            Ok(None) => {
                return Err(CommandError::Failed(format!(
                    "account {account} has no server"
                )));
            }
            Err(detail) => return Err(CommandError::Failed(detail)),
        };
        let backend = connector.connect().await.map_err(|err| {
            CommandError::Failed(format!("could not reach the mail server ({err})"))
        })?;
        let (handle, task) = katna_sync::connection::spawn(backend);
        smol::spawn(task).detach();
        Ok(handle)
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

    /// Pins messages to the top of the list, or unpins them.
    pub fn set_pinned(&self, messages: &[MessageId], on: bool) -> Result<(), CommandError> {
        self.change(|store| ops::set_pinned(store, messages, on, unix_now()))
    }

    /// Creates a folder (a label, on Gmail) called `name` on the account's
    /// server, inside `parent` when given, and stores it. Needs the server:
    /// fails while offline. Returns the new folder.
    pub async fn create_folder(
        &self,
        account: AccountId,
        name: &str,
        parent: Option<FolderId>,
    ) -> Result<FolderId, CommandError> {
        let name = name.trim();
        if name.is_empty() {
            return Err(CommandError::InvalidArgs("the name is empty".into()));
        }
        if name.chars().count() > MAX_FOLDER_NAME || name.chars().any(char::is_control) {
            return Err(CommandError::InvalidArgs(format!(
                "a name has at most {MAX_FOLDER_NAME} characters and no line breaks"
            )));
        }
        let account = self.account(account)?;
        let parent = match parent {
            Some(id) => Some(
                self.store()
                    .folders(account.id)?
                    .into_iter()
                    .find(|f| f.id == id)
                    .ok_or(CommandError::UnknownFolder(id.0))?,
            ),
            None => None,
        };
        let connector = match self.connector(&account).await {
            Ok(Some(Link::Imap(connector))) => connector,
            Ok(_) => {
                return Err(CommandError::InvalidArgs(
                    "only IMAP accounts have folders on the server".into(),
                ));
            }
            Err(detail) => return Err(CommandError::Failed(detail)),
        };
        let mut backend = connector.connect().await.map_err(|err| {
            CommandError::Failed(format!("could not reach the mail server: {err}"))
        })?;
        let created =
            create_on_server(&mut backend, name, parent.as_ref().map(|f| &f.path[..])).await;
        let _ = backend.logout().await;
        let path = created?;
        let id = {
            let mut store = self.store();
            let mut batch = store.mail_batch()?;
            let id = batch.upsert_folder(account.id, &path, None)?;
            batch.commit()?;
            id
        };
        tracing::info!(account = %account.id, path, "folder created");
        let _ = self.notices.try_send(Notice::MailChanged(account.id));
        if let Some(running) = self.workers().get(&account.id) {
            running.handle.sync_now();
        }
        Ok(id)
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
        self.check_smtp(account)?;
        let id = outbox::queue(&mut self.store(), account, raw, delay, unix_now())?;
        tracing::info!(id, %account, delay, "queued to send");
        self.queued(id);
        Ok(id)
    }

    /// Queues `raw` from `account` to go out at `at`, handed over after
    /// `delay` seconds (see [`outbox::schedule`]).
    pub fn schedule_send(
        &self,
        account: AccountId,
        raw: &[u8],
        delay: u32,
        at: i64,
    ) -> Result<i64, CommandError> {
        self.check_smtp(account)?;
        let id = outbox::schedule(&mut self.store(), account, raw, delay, at, unix_now())?;
        tracing::info!(id, %account, delay, at, "scheduled to send");
        self.queued(id);
        Ok(id)
    }

    /// How long the SMTP server of `account` holds mail, in seconds; 0
    /// when it cannot. Logs in to ask the first time.
    pub async fn server_hold_limit(
        self: &Arc<Self>,
        account: AccountId,
    ) -> Result<u64, CommandError> {
        self.check_smtp(account)?;
        if let Some(limit) = self.hold_limits.lock().unwrap().get(&account) {
            return Ok(limit.unwrap_or(0));
        }
        let failed = |err: katna_sync::Error| CommandError::Failed(err.to_string());
        let mut sender = SmtpAccounts(Arc::downgrade(self))
            .connect(account)
            .await
            .map_err(failed)?;
        let limit = sender.hold_limit().await.map_err(failed)?;
        if let Err(error) = sender.quit().await {
            tracing::debug!(%error, "SMTP QUIT after asking");
        }
        self.hold_limits.lock().unwrap().insert(account, limit);
        Ok(limit.unwrap_or(0))
    }

    /// Whether the SMTP server of `account` sends delivery receipts
    /// (RFC 3461 `DSN`). Logs in to ask the first time.
    pub async fn server_delivery_receipts(
        self: &Arc<Self>,
        account: AccountId,
    ) -> Result<bool, CommandError> {
        self.check_smtp(account)?;
        if let Some(&offered) = self.delivery_receipts.lock().unwrap().get(&account) {
            return Ok(offered);
        }
        let failed = |err: katna_sync::Error| CommandError::Failed(err.to_string());
        let mut sender = SmtpAccounts(Arc::downgrade(self))
            .connect(account)
            .await
            .map_err(failed)?;
        let offered = sender.offers_receipts().await.map_err(failed)?;
        if let Err(error) = sender.quit().await {
            tracing::debug!(%error, "SMTP QUIT after asking");
        }
        self.delivery_receipts
            .lock()
            .unwrap()
            .insert(account, offered);
        Ok(offered)
    }

    fn check_smtp(&self, account: AccountId) -> Result<(), CommandError> {
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
        Ok(())
    }

    /// Tells clients about queued mail and wakes the outbox.
    fn queued(&self, id: i64) {
        // A reminder left by an earlier entry with this ID is not this one's.
        if let Err(error) = katna_meta::clear_follow_up(&mut self.store(), id) {
            tracing::warn!(%error, id, "clearing an old reply reminder");
        }
        let _ = self.notices.try_send(Notice::OutboxChanged(id));
        if let Some(sending) = self.outbox.lock().unwrap().as_ref() {
            sending.handle.wake();
        }
    }

    /// Like [`Daemon::queue_send`], as one tracked copy per recipient.
    pub fn queue_tracked_send(
        &self,
        account: AccountId,
        raw: &[u8],
        delay: u32,
    ) -> Result<i64, CommandError> {
        self.check_smtp(account)?;
        let id = outbox::queue_with(&mut self.store(), account, raw, delay, unix_now(), true)?;
        tracing::info!(id, %account, delay, "queued to send, tracked");
        self.queued(id);
        Ok(id)
    }

    /// Saves a mail template (a new one for ID 0). Returns its ID.
    pub fn save_template(&self, template: TemplateItem) -> Result<i64, CommandError> {
        let name = template_name(&template.name)?;
        let size: usize = template.attachments.iter().map(|f| f.data.len()).sum();
        if size > MAX_TEMPLATE_FILES {
            return Err(CommandError::InvalidArgs(format!(
                "a template's attachments are at most {} MB",
                MAX_TEMPLATE_FILES / 1_000_000
            )));
        }
        let template = Template {
            id: template.id,
            name,
            subject: template.subject,
            html: template.html,
            text: template.text,
            attachments: template
                .attachments
                .into_iter()
                .map(|f| TemplateFile {
                    name: f.name,
                    mime: f.mime,
                    data: f.data,
                })
                .collect(),
        };
        let id = self.store().save_template(&template)?;
        tracing::info!(id, "template saved");
        Ok(id)
    }

    /// Renames a template. Returns whether it exists.
    pub fn rename_template(&self, id: i64, name: &str) -> Result<bool, CommandError> {
        let name = template_name(name)?;
        Ok(self.store().rename_template(id, &name)?)
    }

    /// Deletes a template. Returns whether it existed.
    pub fn delete_template(&self, id: i64) -> Result<bool, CommandError> {
        let deleted = self.store().delete_template(id)?;
        if deleted {
            tracing::info!(id, "template deleted");
        }
        Ok(deleted)
    }

    /// Takes a queued message back, if it is not being sent yet.
    pub fn undo_send(&self, id: i64) -> Result<bool, CommandError> {
        let undone = outbox::cancel(&mut self.store(), id)?;
        if undone {
            katna_meta::clear_follow_up(&mut self.store(), id)?;
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

    /// Saves a draft of `account` in its Drafts folder, in place of the
    /// copies saved before. Returns the saved message's ID.
    pub fn save_draft(&self, account: AccountId, raw: &[u8]) -> Result<i64, CommandError> {
        self.account(account)?;
        let mut saved = None;
        self.change(|store| {
            let (id, _) = ops::save_draft(store, account, raw, unix_now())?;
            saved = Some(id);
            Ok(vec![account])
        })?;
        tracing::info!(%account, "draft saved");
        Ok(saved.map_or(0, |id| id.0))
    }

    /// Deletes every saved copy of the draft `message_id` of `account`.
    pub fn discard_draft(&self, account: AccountId, message_id: &str) -> Result<(), CommandError> {
        self.account(account)?;
        self.change(|store| {
            ops::discard_draft(store, account, message_id)?;
            Ok(vec![account])
        })?;
        tracing::info!(%account, "draft discarded");
        Ok(())
    }

    /// Every outbox entry.
    pub fn outbox(&self) -> Result<Vec<OutboxItem>, CommandError> {
        let entries = self.store().outbox()?;
        let errors = self.send_errors.lock().unwrap();
        Ok(entries
            .into_iter()
            .map(|entry| {
                // Scheduled mail shows when it goes out, not when it is
                // handed over.
                let held = entry.state == SendState::Sent && entry.hold_until.is_some();
                OutboxItem {
                    id: entry.id,
                    account: entry.account.0,
                    message: entry.message.0,
                    subject: entry.subject,
                    send_at: match entry.state {
                        SendState::Queued | SendState::Sent => {
                            entry.hold_until.unwrap_or(entry.send_at)
                        }
                        _ => entry.send_at,
                    },
                    state: if held {
                        katna_dbus::send_state::HELD.to_owned()
                    } else {
                        entry.state.as_str().to_owned()
                    },
                    detail: errors.get(&entry.id).cloned().unwrap_or_default(),
                }
            })
            .collect())
    }

    fn start_outbox(self: &Arc<Self>) -> Result<(), CommandError> {
        let store = Store::open(&self.paths, Mode::ReadWrite)?;
        let (handle, control) = outbox::control();
        let (events, received) = async_channel::unbounded();
        let smtp = SmtpAccounts(Arc::downgrade(self));
        let task = threads::spawn(
            "katna-outbox",
            outbox::run(smtp, store, OutboxConfig::default(), events, control),
        );
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
            if event.state == SendState::Sent {
                if let Some(running) = self.workers().get(&event.account) {
                    running.handle.send_changes();
                }
                // A tracked message may have gone out: follow its events.
                self.wake_tracking();
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
        // Mail read or moved here needs no notification any more.
        if let Some(notices) = self.new_mail_notices() {
            let handled = notices.handled(&self.store(), None);
            if !handled.is_empty() {
                smol::spawn(async move { notices.close(handled).await }).detach();
            }
        }
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
        // A reset starts every worker once it is done.
        if self.closing.load(Ordering::SeqCst) || self.resetting.load(Ordering::SeqCst) {
            return;
        }
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
            AccountKind::Imap => settings.imap.clone(),
            AccountKind::Pop3 => settings.pop3.clone(),
            _ => None,
        };
        let Some(server) = server else {
            return Ok(None);
        };
        let credentials = self.credentials(account.id, &server, &settings).await?;
        let (endpoint, tls) = endpoint(&server)?;
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
        if let Some(notices) = self.new_mail_notices() {
            notices.watch(&self.store(), id);
        }
        let (handle, control) = worker::control();
        handle.set_metered(self.metered.load(Ordering::Relaxed));
        let (events, received) = async_channel::unbounded();
        let mut config = self.config.clone();
        config.offline.days = *self.offline_days.lock().unwrap();
        // A thread per account: its database calls block (crate::threads).
        let name = format!("katna-sync-{id}");
        let task = match link {
            Link::Imap(connector) => threads::spawn(
                &name,
                worker::run(connector, store, id, config, events, control),
            ),
            Link::Pop3(connector, keep) => threads::spawn(
                &name,
                worker::run_pop3(connector, store, id, keep, config, events, control),
            ),
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
                    // The first sync since start is news, so that a new
                    // account's folders show even when they are empty.
                    let first = status.last_sync == 0;
                    status.last_sync = unix_now();
                    let changed = reports.iter().any(|report| {
                        report.added + report.flags_changed + report.removed + report.backfilled > 0
                            || report.reset
                    });
                    if changed || first {
                        let _ = self.notices.try_send(Notice::MailChanged(id));
                    }
                    if first {
                        self.name_from_mail(id);
                    }
                    if let Some(notices) = self.new_mail_notices() {
                        notices.synced(&self.store, id).await;
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
                // The folder pane shows how full the account is.
                Event::QuotaChanged => {
                    let _ = self.notices.try_send(Notice::MailChanged(id));
                    continue;
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

    pub(crate) fn store(&self) -> MutexGuard<'_, Store> {
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
/// Forgets the downloaded mail of every IMAP account and the translations,
/// and deletes the sender pictures, for [`Daemon::reset_cache`]. POP3 servers may no longer
/// have their mail, and imported mail has no server.
fn forget_downloaded(paths: &Paths) -> Result<Forgotten, CommandError> {
    let mut store = Store::open(paths, Mode::ReadWrite)?;
    let accounts: Vec<AccountId> = store
        .accounts()?
        .into_iter()
        .filter(|account| account.kind == AccountKind::Imap)
        .map(|account| account.id)
        .collect();
    let forgotten = store.forget_downloaded_mail(&accounts)?;
    store.forget_translations()?;
    let pictures = Pictures::cache_dir(paths.cache_dir());
    match std::fs::remove_dir_all(&pictures) {
        Err(err) if err.kind() != std::io::ErrorKind::NotFound => {
            tracing::warn!(path = %pictures.display(), %err, "deleting sender pictures");
        }
        _ => {}
    }
    Ok(forgotten)
}

async fn stop(account: AccountId, running: Running) {
    drop(running.handle);
    wait_for(account, running.task).await;
}

async fn wait_for(account: AccountId, task: Threaded) {
    let stopped = async {
        task.finished().await;
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
        let smtp = settings.smtp.clone().ok_or_else(|| {
            katna_sync::Error::Rejected(format!("account {account} has no SMTP server"))
        })?;
        let credentials = daemon
            .credentials(account, &smtp, &settings)
            .await
            .map_err(katna_sync::Error::Auth)?;
        drop(daemon);
        let (endpoint, tls) = endpoint(&smtp).map_err(katna_sync::Error::Tls)?;
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

    fn tracking(&self) -> Option<tracking::Client> {
        self.0.upgrade()?.tracking_client()
    }

    async fn tracking_token(&self) -> katna_sync::Result<String> {
        let daemon = self
            .0
            .upgrade()
            .ok_or_else(|| katna_sync::Error::Closed("the daemon is stopping".into()))?;
        daemon
            .tracking_token()
            .await
            .ok_or_else(|| katna_sync::Error::Rejected("not signed in to a Katna account".into()))
    }
}

/// Logs in once to check the server and password.
async fn check_login(
    kind: AccountKind,
    server: &Server,
    password: &str,
) -> Result<(), CommandError> {
    let credentials = Credentials::new(server.username.clone(), password);
    check_credentials(kind, server, credentials).await
}

/// Logs in once to check the server and the password or token.
async fn check_credentials(
    kind: AccountKind,
    server: &Server,
    credentials: Credentials,
) -> Result<(), CommandError> {
    let (endpoint, tls) = endpoint(server).map_err(CommandError::Failed)?;
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

/// Longest folder name, in characters (Gmail's limit for labels).
const MAX_FOLDER_NAME: usize = 225;

/// Creates `name` inside `parent` (a path) with the server's separator,
/// and subscribes to it. Returns its path.
async fn create_on_server(
    backend: &mut impl MailBackend,
    name: &str,
    parent: Option<&str>,
) -> Result<String, CommandError> {
    let failed = |err: katna_sync::Error| CommandError::Failed(err.to_string());
    let existing = backend.list_folders().await.map_err(failed)?;
    let separator = parent
        .and_then(|parent| existing.iter().find(|f| f.name == parent))
        .or_else(|| existing.iter().find(|f| f.delimiter.is_some()))
        .and_then(|f| f.delimiter);
    if let Some(separator) = separator
        && name.contains(separator)
    {
        return Err(CommandError::InvalidArgs(format!(
            "a name cannot contain \u{201c}{separator}\u{201d}; nest it under another \
             folder instead"
        )));
    }
    let path = match (parent, separator) {
        (Some(parent), Some(separator)) => format!("{parent}{separator}{name}"),
        (Some(_), None) => {
            return Err(CommandError::InvalidArgs(
                "this server does not nest folders".into(),
            ));
        }
        (None, _) => name.to_owned(),
    };
    // Gmail and most servers ignore case in names.
    if let Some(taken) = existing
        .iter()
        .find(|f| f.name.to_lowercase() == path.to_lowercase())
    {
        return Err(CommandError::InvalidArgs(format!(
            "\u{201c}{}\u{201d} already exists",
            taken.name
        )));
    }
    backend
        .create_folder(&path)
        .await
        .map_err(|err| CommandError::Failed(format!("the mail server did not create it: {err}")))?;
    Ok(path)
}

/// The settings file, or the defaults when it cannot be read.
pub(crate) fn settings(paths: &Paths) -> Config {
    Config::load(&paths.config_file()).unwrap_or_else(|err| {
        tracing::warn!(%err, "settings unreadable; using the defaults");
        Config::default()
    })
}
