// SPDX-License-Identifier: GPL-3.0-or-later

//! Calendar sync (`docs/ARCHITECTURE.md` §18): every account's calendars
//! and events into `pim.db`, at start, every five minutes and on
//! `SyncNow`, through its provider's API: Google Calendar for Google
//! accounts, Microsoft Graph for Microsoft ones, CalDAV on the IMAP
//! server of others that offer it ([`katna_sync::calendar`]). One task
//! goes through the accounts in turn with its own store connection, apart
//! from the mail workers, and says `CalendarChanged` (and the clock's
//! `Changed`) only when something changed.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, Weak},
    time::Duration,
};

use async_channel::{Receiver, Sender};
use futures_lite::FutureExt;
use katna_core::{Account, AccountId, AccountKind, OAuthProvider};
use katna_dbus::calendar_state;
use katna_store::{Mode, Store};
use katna_sync::{
    calendar::{CalendarError, caldav::CalDav, google::GoogleCalendar, graph::GraphCalendar},
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
}

impl Default for Calendars {
    fn default() -> Self {
        Self {
            wake: async_channel::bounded(1),
            status: Mutex::default(),
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
}

/// The services of the accounts, kept between rounds (CalDAV remembers
/// where the calendars are) and made again when what they depend on
/// changes (a new sign-in, another server or password).
type Services = HashMap<AccountId, (String, Service)>;

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

    /// Starts the calendar sync.
    pub(super) fn start_calendars(self: &Arc<Self>) {
        smol::spawn(run(Arc::downgrade(self), self.calendars.wake.1.clone())).detach();
    }

    /// Where `account`'s calendars come from, and what that depends on;
    /// `None` when it has none Katna can reach.
    async fn calendar_service(
        &self,
        account: &Account,
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
            return Ok(Some(match provider {
                OAuthProvider::Google => (key, Service::Google(GoogleCalendar::new(tokens, tls))),
                OAuthProvider::Microsoft => {
                    (key, Service::Microsoft(GraphCalendar::new(tokens, tls)))
                }
            }));
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
            "caldav {} {} {} {password}",
            server.host, server.username, server.accept_invalid_certs
        );
        let dav = CalDav::new(&server.host, &server.username, &password, tls);
        Ok(Some((key, Service::CalDav(dav))))
    }

    /// Syncs `account`'s calendars; returns what to show and whether
    /// anything changed.
    async fn sync_calendars(
        &self,
        store: &mut Store,
        services: &mut Services,
        account: &Account,
    ) -> ((&'static str, String), bool) {
        let service = match self.calendar_service(account).await {
            Ok(Some((key, service))) => {
                let keep = services
                    .get(&account.id)
                    .is_some_and(|(old, _)| *old == key);
                if !keep {
                    services.insert(account.id, (key, service));
                }
                &services[&account.id].1
            }
            Ok(None) => {
                services.remove(&account.id);
                return ((calendar_state::NONE, String::new()), false);
            }
            Err(detail) => return ((calendar_state::ERROR, detail), false),
        };
        match service.sync(store, account).await {
            Ok(changed) => ((calendar_state::OK, String::new()), changed),
            Err(CalendarError::NeedsSignIn(detail)) => {
                ((calendar_state::NEEDS_SIGN_IN, detail), false)
            }
            Err(CalendarError::NotEnabled(detail)) => {
                ((calendar_state::NOT_ENABLED, detail), false)
            }
            Err(CalendarError::NotOffered) => ((calendar_state::NONE, String::new()), false),
            Err(CalendarError::Failed(err)) => {
                tracing::warn!(account = %account.id, %err, "calendar sync failed");
                ((calendar_state::ERROR, err.to_string()), false)
            }
        }
    }
}

/// Syncs every account's calendars, then waits for the next round or a
/// wake-up, until the daemon goes away.
async fn run(daemon: Weak<Daemon>, wake: Receiver<()>) {
    let mut services = Services::new();
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
        daemon.sync_all_calendars(store, &mut services).await;
    }
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
        services.retain(|id, _| accounts.iter().any(|a| a.id == *id));
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
