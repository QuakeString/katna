// SPDX-License-Identifier: GPL-3.0-or-later

//! Google's "Other contacts": people a Gmail account mailed but never
//! saved (`docs/ARCHITECTURE.md` §8.6). Read-only; saving one copies it
//! into the account's contacts.

use katna_core::AccountId;
use rusqlite::{OptionalExtension, TransactionBehavior, params};

use crate::Store;
use crate::contacts::{BookState, BookSync};
use crate::error::Result;

/// One other contact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtherContact {
    pub id: i64,
    pub account: AccountId,
    pub remote_id: String,
    pub name: String,
    pub sort_key: String,
    /// Lower case.
    pub emails: Vec<String>,
    pub phone: String,
}

impl Store {
    /// Saves what a read of `account`'s other contacts found. Returns
    /// whether anything changed.
    pub fn save_other_contacts(&mut self, account: AccountId, sync: &BookSync) -> Result<bool> {
        self.check_writable()?;
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let mut changed = false;
        if sync.full {
            let keep = serde_json::to_string(
                &sync
                    .contacts
                    .iter()
                    .map(|c| c.remote_id.as_str())
                    .collect::<Vec<_>>(),
            )
            .unwrap_or_else(|_| "[]".to_owned());
            changed |= tx.execute(
                "DELETE FROM other_contact WHERE account_id = ?1
                 AND remote_id NOT IN (SELECT value FROM json_each(?2))",
                params![account.0, keep],
            )? > 0;
        }
        for gone in &sync.deleted {
            changed |= tx.execute(
                "DELETE FROM other_contact WHERE account_id = ?1 AND remote_id = ?2",
                params![account.0, gone],
            )? > 0;
        }
        for contact in &sync.contacts {
            let emails = serde_json::to_string(&contact.card.email_keys())
                .unwrap_or_else(|_| "[]".to_owned());
            changed |= tx.execute(
                "INSERT INTO other_contact (account_id, remote_id, name, sort_key, emails, phone)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                 ON CONFLICT (account_id, remote_id) DO UPDATE SET
                   name = excluded.name, sort_key = excluded.sort_key,
                   emails = excluded.emails, phone = excluded.phone
                 WHERE name <> excluded.name OR sort_key <> excluded.sort_key
                   OR emails <> excluded.emails OR phone <> excluded.phone",
                params![
                    account.0,
                    contact.remote_id,
                    contact.card.display_name(),
                    contact.card.sort_key(),
                    emails,
                    contact.card.phones.first().map_or("", |p| p.value.as_str()),
                ],
            )? > 0;
        }
        let before: Option<String> = tx
            .query_row(
                "SELECT state FROM other_contact_sync WHERE account_id = ?1",
                [account.0],
                |row| row.get(0),
            )
            .optional()?;
        changed |= before.as_deref() != Some(BookState::Ok.as_str());
        tx.execute(
            "INSERT INTO other_contact_sync (account_id, sync_token, state) VALUES (?1, ?2, 'ok')
             ON CONFLICT (account_id) DO UPDATE SET
               sync_token = COALESCE(excluded.sync_token, sync_token), state = 'ok'",
            params![account.0, sync.sync_token],
        )?;
        tx.commit()?;
        Ok(changed)
    }

    /// Where the last read of `account`'s other contacts left off.
    pub fn other_contacts_token(&self, account: AccountId) -> Result<Option<String>> {
        Ok(self
            .pim
            .prepare_cached("SELECT sync_token FROM other_contact_sync WHERE account_id = ?1")?
            .query_row([account.0], |row| row.get(0))
            .optional()?
            .flatten())
    }

    /// Notes that `account`'s other contacts could not be read, and
    /// forgets the ones kept. Returns whether that changed anything.
    pub fn set_other_contacts_state(&self, account: AccountId, state: BookState) -> Result<bool> {
        self.check_writable()?;
        let mut changed = self.pim.execute(
            "INSERT INTO other_contact_sync (account_id, state) VALUES (?1, ?2)
             ON CONFLICT (account_id) DO UPDATE SET state = excluded.state
             WHERE state <> excluded.state",
            params![account.0, state.as_str()],
        )? > 0;
        if state == BookState::NeedsPermission {
            changed |= self.remove_other_contacts(account)?;
        }
        Ok(changed)
    }

    /// Forgets `account`'s other contacts and where their read left off.
    pub fn remove_other_contacts(&self, account: AccountId) -> Result<bool> {
        self.check_writable()?;
        let gone = self.pim.execute(
            "DELETE FROM other_contact WHERE account_id = ?1",
            [account.0],
        )?;
        self.pim.execute(
            "UPDATE other_contact_sync SET sync_token = NULL WHERE account_id = ?1",
            [account.0],
        )?;
        Ok(gone > 0)
    }

    /// Every other contact, by name.
    pub fn other_contacts(&self) -> Result<Vec<OtherContact>> {
        let mut stmt = self.pim.prepare_cached(
            "SELECT id, account_id, remote_id, name, sort_key, emails, phone FROM other_contact
             ORDER BY sort_key, id",
        )?;
        let rows = stmt.query_map([], |row| {
            let emails: String = row.get(5)?;
            Ok(OtherContact {
                id: row.get(0)?,
                account: AccountId(row.get(1)?),
                remote_id: row.get(2)?,
                name: row.get(3)?,
                sort_key: row.get(4)?,
                emails: serde_json::from_str(&emails).unwrap_or_default(),
                phone: row.get(6)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// The accounts whose sign-in has to allow their other contacts.
    pub fn other_contacts_blocked(&self) -> Result<Vec<AccountId>> {
        let mut stmt = self.pim.prepare_cached(
            "SELECT account_id FROM other_contact_sync WHERE state = 'needs-permission'
             ORDER BY account_id",
        )?;
        let rows = stmt.query_map([], |row| Ok(AccountId(row.get(0)?)))?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Other contact `id`.
    pub fn other_contact(&self, id: i64) -> Result<Option<OtherContact>> {
        Ok(self.other_contacts()?.into_iter().find(|c| c.id == id))
    }
}

#[cfg(test)]
mod tests {
    use katna_core::contact::{Card, Name, Typed};
    use katna_core::{AccountKind, Paths};

    use super::*;
    use crate::{Mode, SyncedContact};

    fn other(id: &str, given: &str, email: &str) -> SyncedContact {
        SyncedContact {
            remote_id: id.into(),
            card: Card {
                name: Name {
                    given: given.into(),
                    ..Name::default()
                },
                emails: vec![Typed::new(email, "")],
                ..Card::default()
            },
            ..SyncedContact::default()
        }
    }

    #[test]
    fn keeps_reads_and_forgets_other_contacts() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        paths.create_dirs().unwrap();
        let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
        let a = store
            .add_account(AccountKind::Imap, "Home", "me@gmail.com")
            .unwrap()
            .id;

        let full = BookSync {
            full: true,
            contacts: vec![
                other("otherContacts/1", "Ravi", "Ravi@Shop.in"),
                other("otherContacts/2", "Asha", "asha@x.in"),
            ],
            sync_token: Some("t1".into()),
            ..BookSync::default()
        };
        assert!(store.save_other_contacts(a, &full).unwrap());
        assert!(!store.save_other_contacts(a, &full).unwrap());
        let all = store.other_contacts().unwrap();
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].name, "Asha");
        assert_eq!(all[1].emails, ["ravi@shop.in"]);
        assert_eq!(
            store.other_contacts_token(a).unwrap().as_deref(),
            Some("t1")
        );
        let ravi = store.other_contact(all[1].id).unwrap().unwrap();
        assert_eq!(ravi.remote_id, "otherContacts/1");

        let change = BookSync {
            deleted: vec!["otherContacts/2".into()],
            ..BookSync::default()
        };
        assert!(store.save_other_contacts(a, &change).unwrap());
        assert_eq!(store.other_contacts().unwrap().len(), 1);
        assert_eq!(
            store.other_contacts_token(a).unwrap().as_deref(),
            Some("t1")
        );

        assert!(
            store
                .set_other_contacts_state(a, BookState::NeedsPermission)
                .unwrap()
        );
        assert!(store.other_contacts().unwrap().is_empty());
        assert_eq!(store.other_contacts_token(a).unwrap(), None);
        assert_eq!(store.other_contacts_blocked().unwrap(), [a]);
        assert!(store.save_other_contacts(a, &full).unwrap());
        assert!(store.other_contacts_blocked().unwrap().is_empty());
    }
}
