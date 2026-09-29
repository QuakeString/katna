// SPDX-License-Identifier: GPL-3.0-or-later

//! Calendar sync (`docs/ARCHITECTURE.md` §18): every account's calendars
//! and events into `pim.db`, at start, every five minutes and on
//! `SyncNow`, the best way first and others when that one is not
//! available ([`katna_sync::methods`]): Google Calendar, then Google's
//! CalDAV, for Google sign-ins; Microsoft Graph for Microsoft ones; CalDAV
//! on the provider's server for others ([`katna_sync::calendar`]). The
//! way that worked is remembered per account. One task
//! goes through the accounts in turn with its own store connection, apart
//! from the mail workers, and says `CalendarChanged` (and the clock's
//! `Changed`) only when something changed.
//!
//! Changes made in the apps (`EditEvent`) are written to the store at
//! once ([`katna_sync::calendar::edit::apply`]) and then sent to the
//! calendar's service by another task, one after the other, with the
//! services the sync uses (never while it syncs). When a service refuses a
//! change, its calendar syncs again, which undoes the change here.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, Weak},
    time::Duration,
};

use async_channel::{Receiver, Sender};
use futures_lite::FutureExt;
use katna_core::{Account, AccountId, AccountKind, OAuthProvider};
use katna_dbus::calendar_state;
use katna_store::{
    Mode, Store,
    calendar::{CalendarSource, EventChange},
};
use katna_sync::{
    calendar::{
        CalendarError,
        caldav::CalDav,
        edit::{self, Applied, EditError, Remote},
        google::GoogleCalendar,
        graph::GraphCalendar,
    },
    methods::{self, Data, Method},
    net::Tls,
};

use super::{CommandError, Daemon, Notice};

/// How often calendars sync.
const INTERVAL: Duration = Duration::from_secs(5 * 60);

/// The first sync waits this long after start, out of the way of the
/// first mail sync.
const FIRST_DELAY: Duration = Duration::from_secs(10);

/// State of the calendar sync, shared with the D-Bus API.
pub(crate) struct Calendars {
    wake: (Sender<()>, Receiver<()>),
    /// Each account's last result: a [`calendar_state`] and a detail.
    status: Mutex<HashMap<AccountId, (&'static str, String)>>,
    /// The accounts' services, for the sync and for sending changes; held
    /// by one of them at a time.
    services: smol::lock::Mutex<Services>,
    /// Changes written to the store, to send to their services.
    changes: (Sender<Applied>, Receiver<Applied>),
}

impl Default for Calendars {
    fn default() -> Self {
        Self {
            wake: async_channel::bounded(1),
            status: Mutex::default(),
            services: smol::lock::Mutex::default(),
            changes: async_channel::unbounded(),
        }
    }
}

/// Where an account's calendars come from.
enum Service {
    Google(GoogleCalendar),
    Microsoft(GraphCalendar),
    CalDav(CalDav),
}

impl Service {
    async fn sync(&self, store: &mut Store, account: &Account) -> Result<bool, CalendarError> {
        match self {
            Self::Google(google) => google.sync(store, account.id).await,
            Self::Microsoft(graph) => graph.sync(store, account.id, &account.address).await,
            Self::CalDav(dav) => dav.sync(store, account.id, &account.address).await,
        }
    }

    fn remote(&self) -> Remote<'_> {
        match self {
            Self::Google(google) => Remote::Google(google),
            Self::Microsoft(graph) => Remote::Microsoft(graph),
            Self::CalDav(dav) => Remote::CalDav(dav),
        }
    }
}

/// The services of the accounts, one per way tried, kept between rounds
/// (CalDAV remembers where the calendars are) and made again when what
/// they depend on changes (a new sign-in, another server or password).
type Services = HashMap<(AccountId, Method), (String, Service)>;

impl Daemon {
    /// Has the calendar sync go through every account now.
    pub(crate) fn wake_calendars(&self) {
        let _ = self.calendars.wake.0.try_send(());
    }

    /// Where each account's calendar sync stands.
    pub fn calendar_status(&self) -> Result<Vec<(i64, String, String)>, CommandError> {
        let accounts = self.store().accounts()?;
        let status = self.calendars.status.lock().unwrap();
        Ok(accounts
            .into_iter()
            .map(|account| {
                let (state, detail) = status.get(&account.id).cloned().unwrap_or_else(|| {
                    let state = if account.kind == AccountKind::Imap {
                        calendar_state::OK
                    } else {
                        calendar_state::NONE
                    };
                    (state, String::new())
                });
                (account.id.0, state.to_owned(), detail)
            })
            .collect())
    }

    /// Shows or hides calendar `id`'s events.
    pub fn set_calendar_hidden(&self, id: i64, hidden: bool) -> Result<(), CommandError> {
        if !self.store().set_calendar_hidden(id, hidden)? {
            return Err(CommandError::InvalidArgs(format!("no calendar {id}")));
        }
        tracing::info!(calendar = id, hidden, "calendar shown or hidden");
        let _ = self.notices.try_send(Notice::CalendarChanged);
        Ok(())
    }

    /// Forgets the calendars of `account`, which went away.
    pub(super) fn forget_calendars(&self, account: AccountId) {
        self.calendars.status.lock().unwrap().remove(&account);
        match self.store().remove_account_calendars(account) {
            Ok(0) => {}
            Ok(_) => {
                let _ = self.notices.try_send(Notice::CalendarChanged);
            }
            Err(err) => tracing::warn!(%account, %err, "could not remove the calendars"),
        }
    }

    /// Adds, changes or deletes events as `json` (an [`EventChange`])
    /// says: in the store at once, then on the calendar's service. Returns
    /// the event row added or changed, or 0.
    pub fn edit_event(&self, json: &str) -> Result<i64, CommandError> {
        let change: EventChange = serde_json::from_str(json)
            .map_err(|err| CommandError::InvalidArgs(format!("not an event change: {err}")))?;
        let applied = {
            let mut store = self.store();
            edit::apply(&mut store, &change, unix_now()).map_err(|err| match err {
                EditError::Invalid(message) => CommandError::InvalidArgs(message),
                EditError::Store(err) => err.into(),
            })?
        };
        let id = applied.id;
        tracing::info!(id, steps = applied.steps.len(), "event changed");
        let _ = self.notices.try_send(Notice::CalendarChanged);
        if !applied.steps.is_empty() {
            let _ = self.calendars.changes.0.try_send(applied);
        }
        Ok(id)
    }

    /// Starts the calendar sync, and sending changes made here. Changes
    /// that weren't sent before the daemon last stopped are given up (the
    /// first sync brings back what the services have), and a calendar on
    /// this computer is made when there is none.
    pub(super) fn start_calendars(self: &Arc<Self>) {
        {
            let mut store = self.store();
            match store.forget_pending_events(None) {
                Ok(calendars) if !calendars.is_empty() => {
                    tracing::warn!(?calendars, "event changes not sent before; syncing again");
                }
                Ok(_) => {}
                Err(err) => tracing::warn!(%err, "could not look for unsent event changes"),
            }
            match store.add_local_calendar_if_none() {
                Ok(Some(id)) => tracing::info!(calendar = id, "made a calendar on this computer"),
                Ok(None) => {}
                Err(err) => tracing::warn!(%err, "could not make a calendar on this computer"),
            }
        }
        smol::spawn(run(Arc::downgrade(self), self.calendars.wake.1.clone())).detach();
        smol::spawn(send_changes(
            Arc::downgrade(self),
            self.calendars.changes.1.clone(),
        ))
        .detach();
    }

    /// The service of `account`, made again when what it depends on
    /// changed. `Err` says why there is none.
    async fn service<'a>(
        &self,
        services: &'a mut Services,
        account: &Account,
        method: Method,
    ) -> Result<Option<&'a Service>, String> {
        let at = (account.id, method);
        match self.calendar_service(account, method).await? {
            Some((key, service)) => {
                let keep = services.get(&at).is_some_and(|(old, _)| *old == key);
                if !keep {
                    services.insert(at, (key, service));
                }
                Ok(Some(&services[&at].1))
            }
            None => {
                services.remove(&at);
                Ok(None)
            }
        }
    }

    /// Sends the steps of `applied` to their calendars' services, in order.
    async fn send_change(&self, store: &mut Store, applied: &Applied) -> Result<(), String> {
        for step in &applied.steps {
            let calendar = store
                .calendar(step.calendar())
                .map_err(|err| err.to_string())?
                .ok_or("the calendar is gone")?;
            let Some(account) = calendar.account else {
                continue;
            };
            let account = self
                .store()
                .accounts()
                .map_err(|err| err.to_string())?
                .into_iter()
                .find(|a| a.id == account)
                .ok_or("the account is gone")?;
            // The way the calendar came: its events' IDs are that service's.
            let method = match calendar.source {
                CalendarSource::CalDav => Method::Dav,
                CalendarSource::Google | CalendarSource::Microsoft => Method::Api,
                CalendarSource::Local => continue,
            };
            let mut services = self.calendars.services.lock().await;
            let service = self
                .service(&mut services, &account, method)
                .await?
                .ok_or("the account has no calendars Katna can reach")?;
            edit::push(&service.remote(), store, &calendar, step)
                .await
                .map_err(|err| err.to_string())?;
        }
        Ok(())
    }

    /// Where `account`'s calendars come from, and what that depends on;
    /// `None` when it has none Katna can reach.
    async fn calendar_service(
        &self,
        account: &Account,
        method: Method,
    ) -> Result<Option<(String, Service)>, String> {
        if account.kind != AccountKind::Imap {
            return Ok(None);
        }
        let settings = self
            .store()
            .account_settings(account.id)
            .map_err(|err| err.to_string())?
            .unwrap_or_default();
        if let Some(provider) = settings.oauth {
            let tokens = self.oauth_tokens(account.id, provider).await?;
            let key = format!("{provider:?} {:p}", Arc::as_ptr(&tokens));
            let tls = Tls::system().map_err(|err| format!("TLS setup: {err}"))?;
            return Ok(match (provider, method) {
                (OAuthProvider::Google, Method::Api) => {
                    Some((key, Service::Google(GoogleCalendar::new(tokens, tls))))
                }
                (OAuthProvider::Google, Method::Dav) => {
                    let dav = CalDav::google(tokens, &account.address, tls);
                    Some((key, Service::CalDav(dav)))
                }
                (OAuthProvider::Microsoft, Method::Api) => {
                    Some((key, Service::Microsoft(GraphCalendar::new(tokens, tls))))
                }
                (OAuthProvider::Microsoft, Method::Dav) => None,
            });
        }
        if method != Method::Dav {
            return Ok(None);
        }
        let Some(server) = settings.imap else {
            return Ok(None);
        };
        let password = self
            .secrets
            .password(account.id)
            .await
            .map_err(|err| err.to_string())?
            .ok_or("no password saved")?;
        let (_, tls) = super::endpoint(&server)?;
        let key = format!(
            "caldav {} {} {} {} {password}",
            account.address, server.host, server.username, server.accept_invalid_certs
        );
        let dav = CalDav::for_account(
            &account.address,
            &server.host,
            &server.username,
            &password,
            tls,
        );
        Ok(Some((key, Service::CalDav(dav))))
    }

    /// Syncs `account`'s calendars, the best way first and the others
    /// when it is not available; returns what to show and whether
    /// anything changed.
    async fn sync_calendars(
        &self,
        store: &mut Store,
        services: &mut Services,
        account: &Account,
    ) -> ((&'static str, String), bool) {
        let provider = match self.store().account_settings(account.id) {
            Ok(settings) => settings.and_then(|s| s.oauth),
            Err(err) => return ((calendar_state::ERROR, err.to_string()), false),
        };
        let now = unix_now();
        // What to show when no way works: the most useful reason.
        let mut shown = (calendar_state::NONE, String::new());
        for method in methods::order(store, account.id, Data::Calendar, provider, now) {
            let service = match self.service(services, account, method).await {
                Ok(Some(service)) => service,
                Ok(None) => continue,
                Err(detail) => return ((calendar_state::ERROR, detail), false),
            };
            let (state, detail) = match service.sync(store, account).await {
                Ok(changed) => {
                    methods::remember(store, account.id, Data::Calendar, method, now);
                    return ((calendar_state::OK, String::new()), changed);
                }
                Err(CalendarError::NeedsSignIn(detail)) => (calendar_state::NEEDS_SIGN_IN, detail),
                Err(CalendarError::NotEnabled(detail)) => (calendar_state::NOT_ENABLED, detail),
                Err(CalendarError::NotOffered) => (calendar_state::NONE, String::new()),
                // The network or the server: not a reason to go another way.
                Err(CalendarError::Failed(err)) => {
                    tracing::warn!(account = %account.id, ?method, %err, "calendar sync failed");
                    return ((calendar_state::ERROR, err.to_string()), false);
                }
            };
            tracing::debug!(account = %account.id, ?method, state, detail, "calendar way not available");
            if shown.0 == calendar_state::NONE {
                shown = (state, detail);
            }
        }
        (shown, false)
    }
}

/// Syncs every account's calendars, then waits for the next round or a
/// wake-up, until the daemon goes away.
async fn run(daemon: Weak<Daemon>, wake: Receiver<()>) {
    let mut store: Option<Store> = None;
    let mut delay = FIRST_DELAY;
    loop {
        let woken = async {
            let _ = wake.recv().await;
        }
        .or(async {
            smol::Timer::after(delay).await;
        });
        woken.await;
        delay = INTERVAL;
        let Some(daemon) = daemon.upgrade() else {
            return;
        };
        if daemon.closing() {
            return;
        }
        if daemon.resetting.load(std::sync::atomic::Ordering::SeqCst) {
            continue;
        }
        let store = match &mut store {
            Some(store) => store,
            None => match Store::open(&daemon.paths, Mode::ReadWrite) {
                Ok(opened) => store.insert(opened),
                Err(err) => {
                    tracing::warn!(%err, "calendar sync has no store");
                    continue;
                }
            },
        };
        let mut services = daemon.calendars.services.lock().await;
        daemon.sync_all_calendars(store, &mut services).await;
    }
}

/// Sends each change made here to its calendar's service, until the
/// daemon goes away. A change a service refused is given up: its calendar
/// syncs again, bringing back what the service has.
async fn send_changes(daemon: Weak<Daemon>, changes: Receiver<Applied>) {
    let mut store: Option<Store> = None;
    while let Ok(applied) = changes.recv().await {
        let Some(daemon) = daemon.upgrade() else {
            return;
        };
        let store = match &mut store {
            Some(store) => store,
            None => match Store::open(&daemon.paths, Mode::ReadWrite) {
                Ok(opened) => store.insert(opened),
                Err(err) => {
                    tracing::warn!(%err, "no store to send event changes from");
                    daemon.give_up_change(None, &applied);
                    continue;
                }
            },
        };
        match daemon.send_change(store, &applied).await {
            Ok(()) => {
                if let Err(err) = store.events_pushed(&applied.rows) {
                    tracing::warn!(%err, "could not mark event changes sent");
                }
            }
            Err(err) => {
                tracing::warn!(id = applied.id, %err, "the calendar refused an event change");
                daemon.give_up_change(Some(store), &applied);
            }
        }
    }
}

impl Daemon {
    /// Gives up sending `applied`: its calendars sync again.
    fn give_up_change(&self, store: Option<&mut Store>, applied: &Applied) {
        let forgot = match store {
            Some(store) => store.forget_pending_events(Some(&applied.rows)),
            None => self.store().forget_pending_events(Some(&applied.rows)),
        };
        if let Err(err) = forgot {
            tracing::warn!(%err, "could not give up an event change");
        }
        self.wake_calendars();
    }
}

fn unix_now() -> i64 {
    jiff::Timestamp::now().as_second()
}

impl Daemon {
    /// One round of [`run`].
    async fn sync_all_calendars(&self, store: &mut Store, services: &mut Services) {
        let accounts = match self.store().accounts() {
            Ok(accounts) => accounts,
            Err(err) => {
                tracing::warn!(%err, "no accounts for calendar sync");
                return;
            }
        };
        services.retain(|(id, _), _| accounts.iter().any(|a| a.id == *id));
        let mut changed = false;
        // Calendars of accounts removed while the daemon was away.
        if let Ok(calendars) = store.calendars() {
            let mut gone: Vec<AccountId> = calendars
                .iter()
                .filter_map(|c| c.account)
                .filter(|id| !accounts.iter().any(|a| a.id == *id))
                .collect();
            gone.dedup();
            for account in gone {
                changed |= store.remove_account_calendars(account).unwrap_or(0) > 0;
            }
        }
        for account in &accounts {
            if self.closing() {
                return;
            }
            let (status, synced) = self.sync_calendars(store, services, account).await;
            changed |= synced;
            let old = self
                .calendars
                .status
                .lock()
                .unwrap()
                .insert(account.id, status.clone());
            if old.as_ref() != Some(&status) {
                if status.0 != calendar_state::OK {
                    tracing::info!(account = %account.id, state = status.0, detail = status.1, "calendar sync");
                }
                changed = true;
            }
        }
        if changed {
            let _ = self.notices.try_send(Notice::CalendarChanged);
        }
    }
}
