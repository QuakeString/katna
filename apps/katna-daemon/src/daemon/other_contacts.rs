// SPDX-License-Identifier: GPL-3.0-or-later

//! Google's "Other contacts" (`docs/ARCHITECTURE.md` §8.6): the people a
//! Gmail account mailed but never saved, read with each contacts pass and
//! saved into the account's contacts on request.

use std::sync::Arc;

use katna_core::{Account, OAuthProvider};
use katna_store::{BookSource, BookState, BookSync};
use katna_sync::{Error as SyncError, contacts::GoogleContacts, net::Tls};

use super::contacts::failed;
use super::{CommandError, Daemon, Notice};

/// Reads `account`'s other contacts when it is a Google account; returns
/// whether anything changed. A sign-in that did not allow them is noted,
/// not an error.
pub(super) async fn sync(daemon: &Arc<Daemon>, account: &Account, tls: &Tls) -> bool {
    let Ok(tokens) = daemon.oauth_tokens(account.id, OAuthProvider::Google).await else {
        return false;
    };
    let google = GoogleContacts::new(tokens, tls.clone());
    let found = match google.other_allowed().await {
        Ok(true) => {
            let token = daemon
                .store()
                .other_contacts_token(account.id)
                .ok()
                .flatten();
            google.other_contacts(token.as_deref()).await
        }
        Ok(false) => Err(SyncError::Auth("other contacts not allowed".into())),
        Err(err) => Err(err),
    };
    let result = match found {
        Ok(sync) => daemon.store().save_other_contacts(account.id, &sync),
        Err(SyncError::Auth(why)) => {
            tracing::info!(account = %account.id, %why, "contacts: other contacts not allowed");
            daemon
                .store()
                .set_other_contacts_state(account.id, BookState::NeedsPermission)
        }
        Err(err) => {
            tracing::info!(account = %account.id, %err, "contacts: other contacts not read");
            return false;
        }
    };
    result.unwrap_or_else(|err| {
        tracing::warn!(%err, "contacts: cannot keep other contacts");
        false
    })
}

impl Daemon {
    /// Saves other contact `id` in its account's contacts. Returns the new
    /// card's id, or 0 when it comes with the next sync (the account's
    /// contacts are read another way than Google's API).
    pub async fn save_other_contact(&self, id: i64) -> Result<i64, CommandError> {
        let other = self
            .store()
            .other_contact(id)?
            .ok_or_else(|| CommandError::InvalidArgs(format!("no other contact {id}")))?;
        let tokens = self
            .oauth_tokens(other.account, OAuthProvider::Google)
            .await
            .map_err(CommandError::AuthFailed)?;
        let tls = Tls::system().map_err(|e| CommandError::Failed(e.to_string()))?;
        let saved = GoogleContacts::new(tokens, tls)
            .copy_other(&other.remote_id)
            .await
            .map_err(failed)?;
        let book = self.store().address_books()?.into_iter().find(|b| {
            b.account == Some(other.account)
                && b.source == BookSource::Google
                && b.remote_id.is_empty()
        });
        let Some(book) = book else {
            tracing::info!(id, "other contact saved; it comes with the next sync");
            self.wake_contacts();
            return Ok(0);
        };
        let remote_id = saved.remote_id.clone();
        let sync = BookSync {
            contacts: vec![saved],
            ..BookSync::default()
        };
        let card = {
            let mut store = self.store();
            store.save_book_sync(book.id, &sync)?;
            store.contact_id(book.id, &remote_id)?
        }
        .ok_or_else(|| CommandError::Failed("the saved contact went missing".into()))?;
        tracing::info!(id, card, "other contact saved");
        let _ = self.notices().try_send(Notice::ContactsChanged);
        Ok(card)
    }
}
