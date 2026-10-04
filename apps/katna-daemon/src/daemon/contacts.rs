// SPDX-License-Identifier: GPL-3.0-or-later

//! Keeps each account's contacts in `pim.db` (`docs/ARCHITECTURE.md`
//! §8.6): Google's People API for Gmail (Google's CardDAV server when the
//! API is not available), Microsoft Graph for Outlook, and CardDAV for the
//! rest, found from the account's address. A pass runs
//! shortly after start, every 15 minutes, and when woken (Sync now, a new
//! sign-in); only what changed since the last pass is read where the
//! service allows.

use std::collections::HashMap;
use std::sync::{Arc, Weak};
use std::time::{Duration, Instant};

use async_channel::Receiver;
use futures_lite::FutureExt;
use katna_core::config::AppKind;
use katna_core::contact::Card;
use katna_core::{Account, AccountId, AccountKind, OAuthProvider};
use katna_dbus::contacts_state;
use katna_store::{BookSource, BookState, BookSync, ContactRef, SyncedContact};
use katna_sync::{
    Error as SyncError,
    carddav::{self, CardDav},
    contacts::{GoogleContacts, MicrosoftContacts},
    methods::{self, Data, Dav, Method},
    net::Tls,
    oauth::Provider,
};

use super::{CommandError, Daemon, Notice};

/// Wait after start, so the first sync of mail goes first.
const FIRST: Duration = Duration::from_secs(20);
/// How often every account is read again.
const EVERY: Duration = Duration::from_secs(15 * 60);
/// How often an account's CardDAV address books are looked for again.
const LOOK_AGAIN: Duration = Duration::from_secs(24 * 3600);
/// Pictures fetched in one pass, so a first sync of a large book does not
/// hold the next pass up; the rest come in later passes.
const PICTURES: usize = 100;

/// Runs until the daemon is gone.
pub(crate) async fn run(daemon: Weak<Daemon>, wake: Receiver<()>) {
    let mut looked: HashMap<AccountId, Instant> = HashMap::new();
    let mut pause = FIRST;
    loop {
        async {
            let _ = wake.recv().await;
        }
        .or(async {
            async_io::Timer::after(pause).await;
        })
        .await;
        pause = EVERY;
        let Some(daemon) = daemon.upgrade() else {
            return;
        };
        if daemon.closing() {
            return;
        }
        // Turned off in Settings > Apps: nothing syncs until it is on again,
        // which wakes this.
        if !daemon.app_on(AppKind::Contacts) {
            continue;
        }
        // "Try again" looks for the address books from scratch.
        for id in std::mem::take(&mut *daemon.contacts_recheck.lock().unwrap()) {
            looked.remove(&id);
        }
        let accounts = match daemon.store().accounts() {
            Ok(accounts) => accounts,
            Err(err) => {
                tracing::warn!(%err, "contacts: cannot list accounts");
                continue;
            }
        };
        let mut changed = false;
        // An account taken offline keeps its last status until it is back.
        for account in accounts
            .iter()
            .filter(|a| a.kind.is_mail() && !daemon.is_offline(a.id))
        {
            if daemon.closing() {
                return;
            }
            let status = match sync_account(&daemon, account, &mut looked).await {
                Ok((c, status)) => {
                    changed |= c;
                    status
                }
                Err(err) => {
                    tracing::info!(account = %account.id, %err, "contacts: sync failed");
                    (contacts_state::ERROR, err)
                }
            };
            // A new reason shows in Contacts at once.
            let old = daemon
                .contacts_status
                .lock()
                .unwrap()
                .insert(account.id, status.clone());
            changed |= old.as_ref() != Some(&status);
        }
        // A removed account's books went with it.
        looked.retain(|id, _| accounts.iter().any(|a| a.id == *id));
        daemon
            .contacts_status
            .lock()
            .unwrap()
            .retain(|id, _| accounts.iter().any(|a| a.id == *id));
        if changed {
            let _ = daemon.notices().try_send(Notice::ContactsChanged);
        }
    }
}

/// Where an account's contacts sync stands: a [`contacts_state`] and a
/// detail for people.
type Status = (&'static str, String);

/// Syncs the address books of `account`; returns whether anything changed
/// and where it stands.
///
/// The best way for the account comes first and the others follow when it
/// is not available ([`methods`]): Google's People API, then Google's
/// CardDAV server with the same sign-in; Microsoft Graph; CardDAV with the
/// password. What worked is remembered and replaces the account's address
/// books from any other way, so nobody shows twice.
async fn sync_account(
    daemon: &Arc<Daemon>,
    account: &Account,
    looked: &mut HashMap<AccountId, Instant>,
) -> Result<(bool, Status), String> {
    let settings = daemon
        .store()
        .account_settings(account.id)
        .map_err(|e| e.to_string())?
        .unwrap_or_default();
    let provider = settings.oauth;
    // Google and Microsoft let Katna into contacts only through their own
    // sign-in, not with a mail password.
    if account.kind == AccountKind::Imap
        && provider.is_none()
        && let Some(own) = settings
            .imap
            .as_ref()
            .and_then(|imap| Provider::for_imap_host(&imap.host))
    {
        return Ok((
            false,
            (contacts_state::USE_SIGN_IN, own.as_str().to_owned()),
        ));
    }
    let tls = Tls::system().map_err(|e| e.to_string())?;
    let now = unix_now();
    let order = methods::order(&daemon.store(), account.id, Data::Contacts, provider, now);
    // Other contacts only come from Google's API, whatever way the
    // account's own contacts come.
    let others = provider == Some(OAuthProvider::Google)
        && super::other_contacts::sync(daemon, account, &tls).await;
    let due = looked
        .get(&account.id)
        .is_none_or(|at| at.elapsed() >= LOOK_AGAIN);
    if due {
        looked.insert(account.id, Instant::now());
    }
    // Why the service's own API was refused, for the Allow banner.
    let mut refused: Option<(BookSource, String)> = None;
    // Why a CardDAV server refused the sign-in or password.
    let mut dav_refused: Option<String> = None;
    // What the server answered when no address book was found.
    let mut missing = String::new();
    // Why the service's own API is switched off for Katna.
    let mut off: Option<String> = None;
    for method in order {
        let changed = match (method, provider) {
            (Method::Api, Some(provider)) => {
                match sync_api(daemon, account, provider, &tls).await {
                    Ok(changed) => Some(changed),
                    Err(SyncError::Auth(why)) => {
                        let source = match provider {
                            OAuthProvider::Google => BookSource::Google,
                            // Zoho is never an account's own sign-in.
                            OAuthProvider::Microsoft | OAuthProvider::Zoho => BookSource::Microsoft,
                        };
                        refused = Some((source, why));
                        None
                    }
                    // Switched off in Katna's Google Cloud project:
                    // CardDAV may still be on.
                    Err(SyncError::NotEnabled(why)) => {
                        off = Some(why);
                        None
                    }
                    // The network or the service: not a reason to go another way.
                    Err(err) => return Err(err.to_string()),
                }
            }
            (Method::Dav, Some(OAuthProvider::Google)) => {
                let starts = [methods::google_dav_start(Dav::Card, &account.address)];
                sync_card_dav(
                    daemon,
                    account,
                    &tls,
                    &starts,
                    due,
                    &mut dav_refused,
                    &mut missing,
                )
                .await?
            }
            (Method::Dav, None) => {
                let starts = carddav::start_urls(
                    None,
                    &account.address,
                    settings.imap.as_ref().map(|s| s.host.as_str()),
                );
                sync_card_dav(
                    daemon,
                    account,
                    &tls,
                    &starts,
                    due,
                    &mut dav_refused,
                    &mut missing,
                )
                .await?
            }
            _ => None,
        };
        if let Some(changed) = changed {
            methods::remember(&mut daemon.store(), account.id, Data::Contacts, method, now);
            // A book the server stopped letting Katna into.
            let refused = daemon
                .store()
                .address_books()
                .map_err(|e| e.to_string())?
                .iter()
                .any(|b| b.account == Some(account.id) && b.state == BookState::NeedsPermission);
            let status = if refused {
                (contacts_state::NEEDS_SIGN_IN, String::new())
            } else {
                (contacts_state::OK, String::new())
            };
            return Ok((changed || others, status));
        }
        tracing::debug!(account = %account.id, ?method, "contacts: this way is not available");
    }
    // No way worked: a refused API asks the user to allow contacts.
    match (refused, dav_refused) {
        (Some((source, why)), _) => {
            let book = daemon
                .store()
                .ensure_address_book(Some(account.id), source, "", "")
                .map_err(|e| e.to_string())?;
            let changed = save(daemon, book, Err(SyncError::Auth(why.clone())))? || others;
            Ok((changed, (contacts_state::NEEDS_SIGN_IN, why)))
        }
        (None, Some(why)) => Ok((others, (contacts_state::NEEDS_SIGN_IN, why))),
        (None, None) => match off {
            Some(why) => Ok((others, (contacts_state::NOT_ENABLED, why))),
            None => Ok((others, (contacts_state::NONE, missing))),
        },
    }
}

/// Syncs the account's contacts through Google's People API or Microsoft
/// Graph; returns whether anything changed.
async fn sync_api(
    daemon: &Arc<Daemon>,
    account: &Account,
    provider: OAuthProvider,
    tls: &Tls,
) -> katna_sync::Result<bool> {
    let tokens = daemon
        .oauth_tokens(account.id, provider)
        .await
        .map_err(SyncError::Auth)?;
    let source = match provider {
        OAuthProvider::Google => BookSource::Google,
        OAuthProvider::Microsoft => BookSource::Microsoft,
        // Never an account's own sign-in; no contacts come from Zoho.
        OAuthProvider::Zoho => return Ok(false),
    };
    let found = match provider {
        OAuthProvider::Google => {
            let google = GoogleContacts::new(tokens, tls.clone());
            if !google.allowed().await? {
                return Err(SyncError::Auth("contacts not allowed".into()));
            }
            let token = api_token(daemon, account.id, source);
            google.sync(token.as_deref()).await?
        }
        OAuthProvider::Microsoft => {
            let microsoft = MicrosoftContacts::new(tokens, tls.clone());
            if !microsoft.allowed().await? {
                return Err(SyncError::Auth("contacts not allowed".into()));
            }
            microsoft.sync().await?
        }
        OAuthProvider::Zoho => return Ok(false),
    };
    let failed = |e: String| SyncError::Protocol(e);
    let book = daemon
        .store()
        .ensure_address_book(Some(account.id), source, "", "")
        .map_err(|e| failed(e.to_string()))?;
    let mut changed = save(daemon, book, Ok(found)).map_err(failed)?;
    // Books another way brought would show everyone twice.
    changed |= daemon
        .store()
        .remove_address_books_except(account.id, &[book])
        .map_err(|e| failed(e.to_string()))?
        > 0;
    if provider == OAuthProvider::Google {
        changed |= fetch_pictures(daemon, book, tls).await;
    }
    Ok(changed)
}

/// Where the account's last API read left off, without making its
/// address book before the API has answered.
fn api_token(daemon: &Daemon, account: AccountId, source: BookSource) -> Option<String> {
    daemon
        .store()
        .address_books()
        .ok()?
        .into_iter()
        .find(|b| b.account == Some(account) && b.source == source)?
        .sync_token
}

/// The account's CardDAV address books: their ids and URLs.
fn dav_books(daemon: &Daemon, account: AccountId) -> Result<Vec<(i64, String)>, String> {
    Ok(daemon
        .store()
        .address_books()
        .map_err(|e| e.to_string())?
        .into_iter()
        .filter(|b| b.account == Some(account) && b.source == BookSource::CardDav)
        .map(|b| (b.id, b.remote_id))
        .collect())
}

/// Syncs the account's CardDAV address books, looking for them from
/// `starts` when `due` or none are known. `None` when the account has no
/// way in or no address book there (a server that refused the password
/// says why in `refused`, one without CardDAV in `missing`); otherwise
/// whether anything changed. The
/// account's books from any other way go.
async fn sync_card_dav(
    daemon: &Arc<Daemon>,
    account: &Account,
    tls: &Tls,
    starts: &[String],
    due: bool,
    refused: &mut Option<String>,
    missing: &mut String,
) -> Result<Option<bool>, String> {
    let Some(dav) = card_dav(daemon, account.id, tls.clone()).await? else {
        return Ok(None);
    };
    let mut books = dav_books(daemon, account.id)?;
    if due || books.is_empty() {
        match dav.discover(starts).await {
            Ok(found) if !found.is_empty() => {
                let store = daemon.store();
                let mut kept = Vec::new();
                for collection in &found {
                    let id = store
                        .ensure_address_book(
                            Some(account.id),
                            BookSource::CardDav,
                            &collection.url,
                            &collection.name,
                        )
                        .map_err(|e| e.to_string())?;
                    kept.push((id, collection.url.clone()));
                }
                books = kept;
            }
            Ok(_) => {
                tracing::debug!(account = %account.id, "contacts: no CardDAV address book");
            }
            Err(SyncError::Auth(why)) => {
                tracing::debug!(account = %account.id, %why, "contacts: CardDAV refused");
                *refused = Some(why);
            }
            // Keeps the books it had: the server may be down.
            Err(err) => {
                tracing::debug!(account = %account.id, %err, "contacts: no CardDAV");
                *missing = err.to_string();
            }
        }
    }
    if books.is_empty() {
        return Ok(None);
    }
    let ids: Vec<i64> = books.iter().map(|(id, _)| *id).collect();
    let mut changed = daemon
        .store()
        .remove_address_books_except(account.id, &ids)
        .map_err(|e| e.to_string())?
        > 0;
    for (book, url) in books {
        let token = sync_token(daemon, book);
        let known = daemon
            .store()
            .contact_etags(book)
            .map_err(|e| e.to_string())?;
        let found = dav.sync(&url, token.as_deref(), &known).await;
        changed |= save(daemon, book, found)?;
    }
    Ok(Some(changed))
}

fn unix_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() as i64)
}

/// The account's CardDAV login: its password with its IMAP user name.
pub(super) async fn card_dav(
    daemon: &Daemon,
    account: AccountId,
    tls: Tls,
) -> Result<Option<CardDav>, String> {
    if daemon.is_offline(account) {
        return Err("the account is offline".into());
    }
    let oauth = daemon
        .store()
        .account_settings(account)
        .map_err(|e| e.to_string())?
        .and_then(|s| s.oauth);
    match oauth {
        // Google's CardDAV server takes the account's sign-in.
        Some(OAuthProvider::Google) => {
            let tokens = daemon.oauth_tokens(account, OAuthProvider::Google).await?;
            // A sign-in from before Katna asked for it cannot use it.
            if !tokens
                .has_scope(katna_sync::oauth::GOOGLE_CARDDAV)
                .await
                .map_err(|e| e.to_string())?
            {
                return Ok(None);
            }
            let token = tokens.access_token().await.map_err(|e| e.to_string())?;
            return Ok(Some(CardDav::bearer(&token, tls)));
        }
        Some(OAuthProvider::Microsoft | OAuthProvider::Zoho) => return Ok(None),
        None => {}
    }
    let Some(password) = daemon
        .secrets
        .password(account)
        .await
        .map_err(|e| e.to_string())?
    else {
        return Ok(None);
    };
    let address = daemon
        .store()
        .accounts()
        .map_err(|e| e.to_string())?
        .into_iter()
        .find(|a| a.id == account)
        .map(|a| a.address)
        .unwrap_or_default();
    let user = daemon
        .store()
        .account_settings(account)
        .map_err(|e| e.to_string())?
        .and_then(|s| s.imap)
        .map(|s| s.username)
        .filter(|u| !u.is_empty())
        .unwrap_or(address);
    Ok(Some(CardDav::new(&user, &password, tls)))
}

fn sync_token(daemon: &Daemon, book: i64) -> Option<String> {
    daemon
        .store()
        .address_books()
        .ok()?
        .into_iter()
        .find(|b| b.id == book)?
        .sync_token
}

/// Saves what a sync found, or notes why it failed.
fn save(daemon: &Daemon, book: i64, found: katna_sync::Result<BookSync>) -> Result<bool, String> {
    let mut store = daemon.store();
    match found {
        Ok(sync) => store.save_book_sync(book, &sync).map_err(|e| e.to_string()),
        Err(SyncError::Auth(why)) => {
            let before = store
                .address_books()
                .ok()
                .and_then(|books| books.into_iter().find(|b| b.id == book))
                .map(|b| b.state);
            store
                .set_address_book_state(book, BookState::NeedsPermission)
                .map_err(|e| e.to_string())?;
            tracing::info!(book, %why, "contacts: the account has to allow them");
            Ok(before != Some(BookState::NeedsPermission))
        }
        Err(err) => {
            store
                .set_address_book_state(book, BookState::Failed)
                .map_err(|e| e.to_string())?;
            Err(err.to_string())
        }
    }
}

/// Fetches the pictures of `book` not fetched yet; returns whether any
/// came.
async fn fetch_pictures(daemon: &Daemon, book: i64, tls: &Tls) -> bool {
    let todo = match daemon.store().contact_photos_to_fetch(book) {
        Ok(todo) => todo,
        Err(err) => {
            tracing::warn!(%err, "contacts: cannot list pictures");
            return false;
        }
    };
    let mut any = false;
    for (contact, url) in todo.into_iter().take(PICTURES) {
        if daemon.closing() {
            break;
        }
        match katna_sync::contacts::photo_at(&url, tls).await {
            Ok(data) => match daemon.store().set_contact_photo(contact, &url, &data) {
                Ok(()) => any = true,
                Err(err) => tracing::warn!(%err, "contacts: cannot keep a picture"),
            },
            Err(err) => tracing::debug!(%err, "contacts: picture not fetched"),
        }
    }
    any
}

/// A change made on the Contacts page failed.
pub(super) fn failed(err: SyncError) -> CommandError {
    match err {
        SyncError::Auth(why) => CommandError::AuthFailed(why),
        other => CommandError::Failed(other.to_string()),
    }
}

impl Daemon {
    /// Where each account's contacts sync stands; a mail account not
    /// synced yet is `ok` (about to be).
    pub fn contacts_status(&self) -> Result<Vec<(i64, String, String)>, CommandError> {
        let accounts = self.store().accounts()?;
        let status = self.contacts_status.lock().unwrap();
        Ok(accounts
            .into_iter()
            .map(|account| {
                let (state, detail) = status.get(&account.id).cloned().unwrap_or_else(|| {
                    // A mail archive on this computer has no address book.
                    let state = if account.kind.is_mail() && account.kind != AccountKind::Local {
                        contacts_state::OK
                    } else {
                        contacts_state::NONE
                    };
                    (state, String::new())
                });
                (account.id.0, state.to_owned(), detail)
            })
            .collect())
    }

    /// Saves `card` (a `katna_core::contact::Card` as JSON) over saved
    /// card `contact`, or as a new card in address book `book` (0: the
    /// book on this computer) when `contact` is 0. The account's service
    /// is written first, so what is saved is what it keeps. Returns the
    /// card's id.
    pub async fn save_contact(
        &self,
        contact: i64,
        book: i64,
        card: &str,
    ) -> Result<i64, CommandError> {
        let card: Card = serde_json::from_str(card)
            .map_err(|e| CommandError::InvalidArgs(format!("contact card: {e}")))?;
        if card.is_empty() {
            return Err(CommandError::InvalidArgs("an empty contact".into()));
        }
        let (book, old) = if contact != 0 {
            let old = self
                .store()
                .contact_ref(contact)?
                .ok_or_else(|| CommandError::InvalidArgs(format!("no contact {contact}")))?;
            (old.book.clone(), Some(old))
        } else {
            let id = match book {
                0 => self.store().local_address_book()?,
                id => id,
            };
            let book = self
                .store()
                .address_book(id)?
                .ok_or_else(|| CommandError::InvalidArgs(format!("no address book {id}")))?;
            (book, None)
        };
        let remote = old.as_ref().map(|o| o.remote_id.as_str());
        let etag = old.as_ref().and_then(|o| o.etag.as_deref());
        // Contacts are saved on their service straight away, so not
        // while it is offline.
        if book.source != BookSource::Local
            && book.account.is_some_and(|account| self.is_offline(account))
        {
            return Err(CommandError::Failed("the account is offline".into()));
        }
        let tls = || Tls::system().map_err(|e| CommandError::Failed(e.to_string()));
        let saved = match (book.source, book.account) {
            (BookSource::Local, _) | (_, None) => SyncedContact {
                remote_id: remote
                    .map_or_else(|| format!("local:{}", carddav::new_uid()), str::to_owned),
                etag: None,
                card,
                raw: None,
                starred: old.as_ref().is_some_and(|o| o.starred),
                groups: old.as_ref().map(|o| o.groups.clone()).unwrap_or_default(),
                photo: None,
            },
            (BookSource::Google, Some(account)) => {
                let tokens = self
                    .oauth_tokens(account, OAuthProvider::Google)
                    .await
                    .map_err(CommandError::AuthFailed)?;
                GoogleContacts::new(tokens, tls()?)
                    .save(remote, etag, &card)
                    .await
                    .map_err(failed)?
            }
            (BookSource::Microsoft, Some(account)) => {
                let tokens = self
                    .oauth_tokens(account, OAuthProvider::Microsoft)
                    .await
                    .map_err(CommandError::AuthFailed)?;
                MicrosoftContacts::new(tokens, tls()?)
                    .save(remote, &card)
                    .await
                    .map_err(failed)?
            }
            (BookSource::CardDav, Some(account)) => {
                let dav = card_dav(self, account, tls()?)
                    .await
                    .map_err(CommandError::Failed)?
                    .ok_or_else(|| CommandError::AuthFailed("no password saved".into()))?;
                let groups = old.as_ref().map(|o| o.groups.clone()).unwrap_or_default();
                let (apple, categories): (Vec<String>, Vec<String>) =
                    groups.into_iter().partition(|g| g.starts_with("group:"));
                let raw = old.as_ref().and_then(|o| o.raw.as_deref());
                let mut saved = dav
                    .save(&book.remote_id, remote, etag, raw, &categories, &card)
                    .await
                    .map_err(failed)?;
                // Group cards are not re-read with one card: keep its own.
                saved.groups.extend(apple);
                saved
            }
        };
        let remote_id = saved.remote_id.clone();
        let sync = BookSync {
            contacts: vec![saved],
            ..BookSync::default()
        };
        let id = {
            let mut store = self.store();
            store.save_book_sync(book.id, &sync)?;
            store.contact_id(book.id, &remote_id)?
        }
        .ok_or_else(|| CommandError::Failed("the saved contact went missing".into()))?;
        tracing::info!(id, book = book.id, "contact saved");
        let _ = self.notices().try_send(Notice::ContactsChanged);
        Ok(id)
    }

    /// Deletes saved cards `ids` from their accounts' services (Google and
    /// Microsoft keep them in their trash) and from here. Tries every card
    /// and returns the first failure.
    pub async fn delete_contacts(&self, ids: &[i64]) -> Result<(), CommandError> {
        let mut first_error = None;
        let mut changed = false;
        for &id in ids {
            let Some(old) = self.store().contact_ref(id)? else {
                continue;
            };
            let gone = match (old.book.source, old.book.account) {
                (BookSource::Local, _) | (_, None) => Ok(()),
                (source, Some(account)) => self.delete_remote(source, account, &old).await,
            };
            match gone {
                Ok(()) => {
                    let sync = BookSync {
                        deleted: vec![old.remote_id.clone()],
                        ..BookSync::default()
                    };
                    changed |= self.store().save_book_sync(old.book.id, &sync)?;
                    tracing::info!(id, "contact deleted");
                }
                Err(err) => {
                    tracing::info!(id, %err, "contact not deleted");
                    first_error.get_or_insert(err);
                }
            }
        }
        if changed {
            let _ = self.notices().try_send(Notice::ContactsChanged);
        }
        first_error.map_or(Ok(()), Err)
    }

    async fn delete_remote(
        &self,
        source: BookSource,
        account: AccountId,
        old: &ContactRef,
    ) -> Result<(), CommandError> {
        if self.is_offline(account) {
            return Err(CommandError::Failed("the account is offline".into()));
        }
        let tls = Tls::system().map_err(|e| CommandError::Failed(e.to_string()))?;
        match source {
            BookSource::Google | BookSource::Microsoft => {
                let provider = if source == BookSource::Google {
                    OAuthProvider::Google
                } else {
                    OAuthProvider::Microsoft
                };
                let tokens = self
                    .oauth_tokens(account, provider)
                    .await
                    .map_err(CommandError::AuthFailed)?;
                if source == BookSource::Google {
                    GoogleContacts::new(tokens, tls)
                        .delete(&old.remote_id)
                        .await
                } else {
                    MicrosoftContacts::new(tokens, tls)
                        .delete(&old.remote_id)
                        .await
                }
                .map_err(failed)
            }
            BookSource::CardDav => {
                let dav = card_dav(self, account, tls)
                    .await
                    .map_err(CommandError::Failed)?
                    .ok_or_else(|| CommandError::AuthFailed("no password saved".into()))?;
                dav.delete(&old.book.remote_id, &old.remote_id, old.etag.as_deref())
                    .await
                    .map_err(failed)
            }
            BookSource::Local => Ok(()),
        }
    }
}
