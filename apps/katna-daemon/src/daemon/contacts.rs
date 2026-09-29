// SPDX-License-Identifier: GPL-3.0-or-later

//! Keeps each account's contacts in `pim.db` (`docs/ARCHITECTURE.md`
//! §8.6): Google's People API for Gmail, Microsoft Graph for Outlook, and
//! CardDAV for the rest, found from the account's address. A pass runs
//! shortly after start, every 15 minutes, and when woken (Sync now, a new
//! sign-in); only what changed since the last pass is read where the
//! service allows.

use std::collections::HashMap;
use std::sync::{Arc, Weak};
use std::time::{Duration, Instant};

use async_channel::Receiver;
use futures_lite::FutureExt;
use katna_core::{Account, AccountId, OAuthProvider};
use katna_store::{BookSource, BookState, BookSync};
use katna_sync::{
    Error as SyncError,
    carddav::{self, CardDav},
    contacts::{GoogleContacts, MicrosoftContacts},
    net::Tls,
};

use super::{Daemon, Notice};

/// Wait after start, so the first sync of mail goes first.
const FIRST: Duration = Duration::from_secs(20);
/// How often every account is read again.
const EVERY: Duration = Duration::from_secs(15 * 60);
/// How often an account without a CardDAV address book is looked at again.
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
        let accounts = match daemon.store().accounts() {
            Ok(accounts) => accounts,
            Err(err) => {
                tracing::warn!(%err, "contacts: cannot list accounts");
                continue;
            }
        };
        let mut changed = false;
        for account in accounts.iter().filter(|a| a.kind.is_mail()) {
            if daemon.closing() {
                return;
            }
            match sync_account(&daemon, account, &mut looked).await {
                Ok(c) => changed |= c,
                Err(err) => {
                    tracing::info!(account = %account.id, %err, "contacts: sync failed");
                }
            }
        }
        // A removed account's books went with it.
        looked.retain(|id, _| accounts.iter().any(|a| a.id == *id));
        if changed {
            let _ = daemon.notices().try_send(Notice::ContactsChanged);
        }
    }
}

/// Syncs the address books of `account`; returns whether anything changed.
async fn sync_account(
    daemon: &Arc<Daemon>,
    account: &Account,
    looked: &mut HashMap<AccountId, Instant>,
) -> Result<bool, String> {
    let settings = daemon
        .store()
        .account_settings(account.id)
        .map_err(|e| e.to_string())?
        .unwrap_or_default();
    let tls = Tls::system().map_err(|e| e.to_string())?;
    match settings.oauth {
        Some(provider) => {
            let source = match provider {
                OAuthProvider::Google => BookSource::Google,
                OAuthProvider::Microsoft => BookSource::Microsoft,
            };
            let book = daemon
                .store()
                .ensure_address_book(Some(account.id), source, "", "")
                .map_err(|e| e.to_string())?;
            let tokens = daemon.oauth_tokens(account.id, provider).await?;
            let found = match provider {
                OAuthProvider::Google => {
                    let google = GoogleContacts::new(tokens, tls.clone());
                    match google.allowed().await {
                        Ok(true) => {
                            let token = sync_token(daemon, book);
                            google.sync(token.as_deref()).await
                        }
                        Ok(false) => Err(SyncError::Auth("contacts not allowed".into())),
                        Err(e) => Err(e),
                    }
                }
                OAuthProvider::Microsoft => {
                    let microsoft = MicrosoftContacts::new(tokens, tls.clone());
                    match microsoft.allowed().await {
                        Ok(true) => microsoft.sync().await,
                        Ok(false) => Err(SyncError::Auth("contacts not allowed".into())),
                        Err(e) => Err(e),
                    }
                }
            };
            let mut changed = save(daemon, book, found)?;
            if provider == OAuthProvider::Google {
                changed |= fetch_pictures(daemon, book, &tls).await;
            }
            Ok(changed)
        }
        None => {
            let Some(password) = daemon
                .secrets
                .password(account.id)
                .await
                .map_err(|e| e.to_string())?
            else {
                return Ok(false);
            };
            let user = settings
                .imap
                .as_ref()
                .map(|s| s.username.clone())
                .filter(|u| !u.is_empty())
                .unwrap_or_else(|| account.address.clone());
            let dav = CardDav::new(&user, &password, tls);
            let books: Vec<(i64, String)> = daemon
                .store()
                .address_books()
                .map_err(|e| e.to_string())?
                .into_iter()
                .filter(|b| b.account == Some(account.id) && b.source == BookSource::CardDav)
                .map(|b| (b.id, b.remote_id))
                .collect();
            let due = looked
                .get(&account.id)
                .is_none_or(|at| at.elapsed() >= LOOK_AGAIN);
            let mut changed = false;
            let books = if due {
                looked.insert(account.id, Instant::now());
                let starts = carddav::start_urls(
                    None,
                    &account.address,
                    settings.imap.as_ref().map(|s| s.host.as_str()),
                );
                match dav.discover(&starts).await {
                    Ok(found) => {
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
                        let ids: Vec<i64> = kept.iter().map(|(id, _)| *id).collect();
                        changed |= store
                            .remove_address_books_except(account.id, &ids)
                            .map_err(|e| e.to_string())?
                            > 0;
                        kept
                    }
                    // Keeps the books it had: the server may be down.
                    Err(err) => {
                        tracing::debug!(account = %account.id, %err, "contacts: no CardDAV");
                        books
                    }
                }
            } else {
                books
            };
            for (book, url) in books {
                let token = sync_token(daemon, book);
                let known = daemon
                    .store()
                    .contact_etags(book)
                    .map_err(|e| e.to_string())?;
                let found = dav.sync(&url, token.as_deref(), &known).await;
                changed |= save(daemon, book, found)?;
            }
            Ok(changed)
        }
    }
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
