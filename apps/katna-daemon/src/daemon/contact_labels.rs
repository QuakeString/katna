// SPDX-License-Identifier: GPL-3.0-or-later

//! Contact labels (`docs/ARCHITECTURE.md` §8.6): putting people on a label
//! and taking them off, and renaming or deleting a label. Each service
//! keeps them its own way: Google as contact groups, Microsoft as the
//! contact's categories, CardDAV as the card's `CATEGORIES`, and a card on
//! this computer by name. Apple-style group cards (`group:`) are read, not
//! changed.

use katna_core::OAuthProvider;
use katna_store::{BookSource, BookSync, ContactRef, SyncedContact, SyncedGroup};
use katna_sync::{
    contacts::{GoogleContacts, MicrosoftContacts},
    net::Tls,
};

use super::contacts::{card_dav, failed};
use super::{CommandError, Daemon, Notice};

/// An Apple-style group card's label, kept in the group, not the card.
fn apple(remote: &str) -> bool {
    remote.starts_with("group:")
}

fn same(a: &str, b: &str) -> bool {
    a.trim().to_lowercase() == b.trim().to_lowercase()
}

/// `labels` without blanks or repeats, in their order.
fn tidy(labels: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for label in labels {
        let label = label.trim().to_owned();
        if !label.is_empty() && !out.iter().any(|l| same(l, &label)) {
            out.push(label);
        }
    }
    out
}

impl Daemon {
    /// Gives card `contact` the labels named `labels`, making the ones its
    /// address book does not have yet.
    pub async fn set_contact_labels(
        &self,
        contact: i64,
        labels: Vec<String>,
    ) -> Result<(), CommandError> {
        let card = self
            .store()
            .contact_ref(contact)?
            .ok_or_else(|| CommandError::InvalidArgs(format!("no contact {contact}")))?;
        let groups = self.store().contact_groups(card.book.id)?;
        let name_of = |remote: &str| {
            groups
                .iter()
                .find(|g| g.remote_id == remote)
                .map_or_else(|| remote.to_owned(), |g| g.name.clone())
        };
        let wanted = tidy(labels);
        let remove: Vec<String> = card
            .groups
            .iter()
            .filter(|g| !apple(g) && !wanted.iter().any(|w| same(w, &name_of(g))))
            .cloned()
            .collect();
        let add: Vec<String> = wanted
            .into_iter()
            .filter(|w| !card.groups.iter().any(|g| same(w, &name_of(g))))
            .collect();
        if remove.is_empty() && add.is_empty() {
            return Ok(());
        }
        self.write_labels(&card, &groups, &remove, &add).await?;
        let _ = self.notices().try_send(Notice::ContactsChanged);
        Ok(())
    }

    /// Renames label `old` to `new` in every address book, or takes it
    /// away when `new` is empty; the people on it stay.
    pub async fn rename_contact_label(&self, old: &str, new: &str) -> Result<(), CommandError> {
        let new = new.trim();
        if old.trim().is_empty() {
            return Err(CommandError::InvalidArgs("a label without a name".into()));
        }
        let books = self.store().address_books()?;
        let mut first_error = None;
        let mut changed = false;
        for book in books {
            let groups = self.store().contact_groups(book.id)?;
            let Some(group) = groups
                .iter()
                .find(|g| !apple(&g.remote_id) && same(&g.name, old))
                .cloned()
            else {
                continue;
            };
            let done = match (book.source, book.account) {
                // Google keeps the label itself: one change covers everyone.
                (BookSource::Google, Some(account)) => {
                    async {
                        let google = self.google(account).await?;
                        if new.is_empty() {
                            google
                                .delete_group(&group.remote_id)
                                .await
                                .map_err(failed)?;
                            self.store()
                                .remove_contact_group(book.id, &group.remote_id)?;
                        } else {
                            let renamed = google
                                .rename_group(&group.remote_id, new)
                                .await
                                .map_err(failed)?;
                            let sync = BookSync {
                                groups: Some(vec![renamed]),
                                ..BookSync::default()
                            };
                            self.store().save_book_sync(book.id, &sync)?;
                        }
                        Ok::<_, CommandError>(())
                    }
                    .await
                }
                // Elsewhere the label lives on each card.
                _ => {
                    async {
                        let add: Vec<String> = if new.is_empty() {
                            Vec::new()
                        } else {
                            vec![new.to_owned()]
                        };
                        let remove = [group.remote_id.clone()];
                        let members = self.store().contacts_in_group(book.id, &group.remote_id)?;
                        for id in members {
                            let Some(card) = self.store().contact_ref(id)? else {
                                continue;
                            };
                            self.write_labels(&card, &groups, &remove, &add).await?;
                        }
                        self.store()
                            .remove_contact_group(book.id, &group.remote_id)?;
                        Ok::<_, CommandError>(())
                    }
                    .await
                }
            };
            match done {
                Ok(()) => changed = true,
                Err(err) => {
                    tracing::info!(book = book.id, %err, "contacts: label not changed");
                    first_error.get_or_insert(err);
                }
            }
        }
        if changed {
            let _ = self.notices().try_send(Notice::ContactsChanged);
        }
        first_error.map_or(Ok(()), Err)
    }

    async fn google(&self, account: katna_core::AccountId) -> Result<GoogleContacts, CommandError> {
        let tokens = self
            .oauth_tokens(account, OAuthProvider::Google)
            .await
            .map_err(CommandError::AuthFailed)?;
        Ok(GoogleContacts::new(tokens, tls()?))
    }

    /// Takes `card` off the labels `remove` (by `remote_id`) and puts it on
    /// the labels named `add`, at its service and in the store.
    async fn write_labels(
        &self,
        card: &ContactRef,
        groups: &[SyncedGroup],
        remove: &[String],
        add: &[String],
    ) -> Result<(), CommandError> {
        let book = &card.book;
        let kept: Vec<String> = card
            .groups
            .iter()
            .filter(|g| !remove.contains(g))
            .cloned()
            .collect();
        let name_of = |remote: &str| {
            groups
                .iter()
                .find(|g| g.remote_id == remote)
                .map_or_else(|| remote.to_owned(), |g| g.name.clone())
        };
        let mut made: Vec<SyncedGroup> = Vec::new();
        let saved = match (book.source, book.account) {
            (BookSource::Google, Some(account)) => {
                let google = self.google(account).await?;
                let mut final_groups = kept.clone();
                for name in add {
                    let remote = match groups.iter().find(|g| same(&g.name, name)) {
                        Some(g) => g.remote_id.clone(),
                        None => {
                            let group = google.create_group(name).await.map_err(failed)?;
                            let remote = group.remote_id.clone();
                            made.push(group);
                            remote
                        }
                    };
                    google
                        .modify_group(&remote, &[&card.remote_id], &[])
                        .await
                        .map_err(failed)?;
                    final_groups.push(remote);
                }
                for remote in remove {
                    google
                        .modify_group(remote, &[], &[&card.remote_id])
                        .await
                        .map_err(failed)?;
                }
                // Its new etag; Google may show the labels a little later,
                // so Katna keeps the ones it just set.
                let mut saved = google.person(&card.remote_id).await.map_err(failed)?;
                saved.groups = final_groups;
                saved
            }
            (BookSource::Microsoft, Some(account)) => {
                let tokens = self
                    .oauth_tokens(account, OAuthProvider::Microsoft)
                    .await
                    .map_err(CommandError::AuthFailed)?;
                let mut categories: Vec<String> = kept.iter().map(|g| name_of(g)).collect();
                categories.extend(add.iter().cloned());
                MicrosoftContacts::new(tokens, tls()?)
                    .set_categories(&card.remote_id, &categories)
                    .await
                    .map_err(failed)?
            }
            (BookSource::CardDav, Some(account)) => {
                let dav = card_dav(self, account, tls()?)
                    .await
                    .map_err(CommandError::Failed)?
                    .ok_or_else(|| CommandError::AuthFailed("no password saved".into()))?;
                let (apples, mut categories): (Vec<String>, Vec<String>) =
                    kept.into_iter().partition(|g| apple(g));
                categories.extend(add.iter().cloned());
                let mut saved = dav
                    .save(
                        &book.remote_id,
                        Some(&card.remote_id),
                        card.etag.as_deref(),
                        card.raw.as_deref(),
                        &categories,
                        &card.card,
                    )
                    .await
                    .map_err(failed)?;
                saved.groups.extend(apples);
                saved
            }
            // On this computer a label is its name.
            _ => {
                let mut final_groups = kept;
                final_groups.extend(add.iter().cloned());
                SyncedContact {
                    remote_id: card.remote_id.clone(),
                    etag: card.etag.clone(),
                    card: card.card.clone(),
                    raw: card.raw.clone(),
                    starred: card.starred,
                    groups: final_groups,
                    photo: None,
                }
            }
        };
        let sync = BookSync {
            contacts: vec![saved],
            groups: (!made.is_empty()).then_some(made),
            ..BookSync::default()
        };
        let mut store = self.store();
        store.save_book_sync(book.id, &sync)?;
        // Google keeps a label nobody has; elsewhere it goes with its last card.
        if book.source != BookSource::Google {
            store.remove_empty_contact_groups(book.id)?;
        }
        Ok(())
    }
}

fn tls() -> Result<Tls, CommandError> {
    Tls::system().map_err(|e| CommandError::Failed(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_are_tidied() {
        assert_eq!(
            tidy(vec![
                " Family ".into(),
                "family".into(),
                String::new(),
                "Work".into()
            ]),
            ["Family", "Work"]
        );
        assert!(same("Family", " FAMILY"));
        assert!(apple("group:abc"));
    }
}
