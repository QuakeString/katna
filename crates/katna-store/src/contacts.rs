// SPDX-License-Identifier: GPL-3.0-or-later

//! Saved contacts (`address_book`, `contact` and their tables in `pim.db`):
//! what the daemon syncs from each account's address books, and what the
//! Contacts page, the contact panel and the address suggestions read.
//!
//! The same person saved in several accounts is one [`SavedContact`] when
//! the cards share an email address (`docs/ARCHITECTURE.md` §8.6).

use std::collections::{BTreeSet, HashMap};
use std::fmt;
use std::str::FromStr;

use katna_core::AccountId;
use katna_core::contact::Card;
use rusqlite::{OptionalExtension, TransactionBehavior, params};

use crate::Store;
use crate::db::unix_now;
use crate::error::{Error, Result};

/// Where an address book syncs from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BookSource {
    /// Google People API.
    Google,
    /// Microsoft Graph contacts.
    Microsoft,
    CardDav,
    /// Kept on this computer only.
    Local,
}

impl BookSource {
    const ALL: [Self; 4] = [Self::Google, Self::Microsoft, Self::CardDav, Self::Local];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Google => "google",
            Self::Microsoft => "microsoft",
            Self::CardDav => "carddav",
            Self::Local => "local",
        }
    }
}

impl fmt::Display for BookSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for BookSource {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::ALL
            .into_iter()
            .find(|k| k.as_str() == s)
            .ok_or_else(|| Error::InvalidData(format!("unknown address book source {s:?}")))
    }
}

/// How an address book's last sync went.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum BookState {
    #[default]
    Ok,
    /// The account was signed in before Katna asked for its contacts: the
    /// user has to allow them.
    NeedsPermission,
    Failed,
}

impl BookState {
    const ALL: [Self; 3] = [Self::Ok, Self::NeedsPermission, Self::Failed];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::NeedsPermission => "needs-permission",
            Self::Failed => "failed",
        }
    }
}

impl FromStr for BookState {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::ALL
            .into_iter()
            .find(|k| k.as_str() == s)
            .ok_or_else(|| Error::InvalidData(format!("unknown address book state {s:?}")))
    }
}

/// An address book.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressBook {
    pub id: i64,
    /// `None` for the book on this computer.
    pub account: Option<AccountId>,
    pub source: BookSource,
    /// Empty for Google's one book; a Microsoft folder id; a CardDAV
    /// collection URL.
    pub remote_id: String,
    pub name: String,
    pub sync_token: Option<String>,
    pub synced_at: Option<i64>,
    pub state: BookState,
}

/// A contact as a sync reads it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SyncedContact {
    /// Its name in the source: a `resourceName`, a Graph id or an href.
    pub remote_id: String,
    pub etag: Option<String>,
    pub card: Card,
    /// The source's own form, kept for writing changes back.
    pub raw: Option<String>,
    pub starred: bool,
    /// The labels it has, by their `remote_id`; one not among the
    /// book's labels is made, named as its `remote_id` (a vCard category).
    pub groups: Vec<String>,
    /// Its picture, when the card carries it (CardDAV).
    pub photo: Option<Vec<u8>>,
}

/// A label as a sync reads it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncedGroup {
    pub remote_id: String,
    pub name: String,
}

/// What one sync of an address book found.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BookSync {
    /// Every contact of the book: the ones not listed are removed.
    pub full: bool,
    pub contacts: Vec<SyncedContact>,
    /// Removed since the last sync, by `remote_id` (changes only).
    pub deleted: Vec<String>,
    /// Cards a full sync found unchanged and did not read again, by
    /// `remote_id`: they stay.
    pub unchanged: Vec<String>,
    /// Labels the sync read: made or renamed.
    pub groups: Option<Vec<SyncedGroup>>,
    /// `groups` is every label of the book: the others are removed.
    pub groups_complete: bool,
    /// Where the next sync picks up.
    pub sync_token: Option<String>,
}

/// One person on the Contacts page: every saved card that shares an email
/// address with another, over all accounts, as one.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SavedContact {
    /// The cards, the first the one whose name is shown.
    pub ids: Vec<i64>,
    pub name: String,
    pub sort_key: String,
    pub emails: Vec<String>,
    pub phone: String,
    pub job: String,
    pub starred: bool,
    pub labels: Vec<String>,
    /// The accounts that hold it, `None` for this computer.
    pub accounts: Vec<Option<AccountId>>,
}

/// One saved card with where it is kept.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredCard {
    pub id: i64,
    pub book: i64,
    pub account: Option<AccountId>,
    pub source: BookSource,
    pub card: Card,
    pub starred: bool,
    pub labels: Vec<String>,
}

/// A saved card as the daemon needs it to write a change back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContactRef {
    pub id: i64,
    pub book: AddressBook,
    pub remote_id: String,
    pub etag: Option<String>,
    pub card: Card,
    pub raw: Option<String>,
    pub starred: bool,
    /// Its labels, by their `remote_id`.
    pub groups: Vec<String>,
}

/// A label with how many people have it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContactLabel {
    pub name: String,
    pub count: usize,
}

impl Store {
    /// Every address book, by account and name.
    pub fn address_books(&self) -> Result<Vec<AddressBook>> {
        let mut stmt = self.pim.prepare_cached(
            "SELECT id, account_id, source, remote_id, name, sync_token, synced_at, state
             FROM address_book ORDER BY account_id, name COLLATE NOCASE, id",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, Option<i64>>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, Option<i64>>(6)?,
                row.get::<_, String>(7)?,
            ))
        })?;
        rows.map(|row| {
            let (id, account, source, remote_id, name, sync_token, synced_at, state) = row?;
            Ok(AddressBook {
                id,
                account: account.map(AccountId),
                source: source.parse()?,
                remote_id,
                name,
                sync_token,
                synced_at,
                state: state.parse()?,
            })
        })
        .collect()
    }

    /// The address book `remote_id` of `account`, made if it is new; its
    /// name is brought up to date. Returns its id.
    pub fn ensure_address_book(
        &self,
        account: Option<AccountId>,
        source: BookSource,
        remote_id: &str,
        name: &str,
    ) -> Result<i64> {
        self.check_writable()?;
        let found: Option<i64> = self
            .pim
            .prepare_cached(
                "SELECT id FROM address_book WHERE account_id IS ?1 AND remote_id = ?2",
            )?
            .query_row(params![account.map(|a| a.0), remote_id], |row| row.get(0))
            .optional()?;
        if let Some(id) = found {
            self.pim.execute(
                "UPDATE address_book SET name = ?2, source = ?3 WHERE id = ?1 AND
                 (name <> ?2 OR source <> ?3)",
                params![id, name, source.as_str()],
            )?;
            return Ok(id);
        }
        self.pim.execute(
            "INSERT INTO address_book (account_id, source, remote_id, name)
             VALUES (?1, ?2, ?3, ?4)",
            params![account.map(|a| a.0), source.as_str(), remote_id, name],
        )?;
        Ok(self.pim.last_insert_rowid())
    }

    /// Removes `account`'s address books other than `keep` (collections
    /// deleted on the server), with their contacts. Returns how many went.
    pub fn remove_address_books_except(&self, account: AccountId, keep: &[i64]) -> Result<usize> {
        self.check_writable()?;
        let keep = serde_json::to_string(keep).unwrap_or_else(|_| "[]".to_owned());
        Ok(self.pim.execute(
            "DELETE FROM address_book WHERE account_id = ?1
             AND id NOT IN (SELECT value FROM json_each(?2))",
            params![account.0, keep],
        )?)
    }

    /// Notes how the last sync of `book` went.
    pub fn set_address_book_state(&self, book: i64, state: BookState) -> Result<()> {
        self.check_writable()?;
        self.pim.execute(
            "UPDATE address_book SET state = ?2 WHERE id = ?1",
            params![book, state.as_str()],
        )?;
        Ok(())
    }

    /// Forgets where `book`'s sync left off, so the next one reads it all.
    pub fn reset_address_book_sync(&self, book: i64) -> Result<()> {
        self.check_writable()?;
        self.pim.execute(
            "UPDATE address_book SET sync_token = NULL WHERE id = ?1",
            [book],
        )?;
        Ok(())
    }

    /// Saves what one sync of `book` found, in one transaction, and marks
    /// the book synced. Returns whether anything changed.
    pub fn save_book_sync(&mut self, book: i64, sync: &BookSync) -> Result<bool> {
        self.check_writable()?;
        let now = unix_now();
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let mut changed = false;

        let mut group_ids: HashMap<String, i64> = HashMap::new();
        if let Some(groups) = &sync.groups {
            if sync.groups_complete {
                let names = serde_json::to_string(
                    &groups
                        .iter()
                        .map(|g| g.remote_id.as_str())
                        .collect::<Vec<_>>(),
                )
                .unwrap_or_else(|_| "[]".to_owned());
                changed |= tx.execute(
                    "DELETE FROM contact_group WHERE book_id = ?1
                     AND remote_id NOT IN (SELECT value FROM json_each(?2))",
                    params![book, names],
                )? > 0;
            }
            for group in groups {
                changed |= tx.execute(
                    "INSERT INTO contact_group (book_id, remote_id, name) VALUES (?1, ?2, ?3)
                     ON CONFLICT (book_id, remote_id) DO UPDATE SET name = excluded.name
                     WHERE name <> excluded.name",
                    params![book, group.remote_id, group.name],
                )? > 0;
            }
        }
        {
            let mut stmt =
                tx.prepare_cached("SELECT remote_id, id FROM contact_group WHERE book_id = ?1")?;
            let rows = stmt.query_map([book], |row| Ok((row.get(0)?, row.get(1)?)))?;
            for row in rows {
                let (remote, id) = row?;
                group_ids.insert(remote, id);
            }
        }

        if sync.full {
            let kept = serde_json::to_string(
                &sync
                    .contacts
                    .iter()
                    .map(|c| c.remote_id.as_str())
                    .chain(sync.unchanged.iter().map(String::as_str))
                    .collect::<Vec<_>>(),
            )
            .unwrap_or_else(|_| "[]".to_owned());
            changed |= tx.execute(
                "DELETE FROM contact WHERE book_id = ?1
                 AND remote_id NOT IN (SELECT value FROM json_each(?2))",
                params![book, kept],
            )? > 0;
        }
        for remote in &sync.deleted {
            changed |= tx.execute(
                "DELETE FROM contact WHERE book_id = ?1 AND remote_id = ?2",
                params![book, remote],
            )? > 0;
        }

        for contact in &sync.contacts {
            let card_json = serde_json::to_string(&contact.card)
                .map_err(|e| Error::InvalidData(format!("contact card: {e}")))?;
            let old: Option<(i64, Option<String>, String, bool)> = tx
                .prepare_cached(
                    "SELECT id, etag, card_json, starred FROM contact
                     WHERE book_id = ?1 AND remote_id = ?2",
                )?
                .query_row(params![book, contact.remote_id], |row| {
                    Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
                })
                .optional()?;
            let same = old.as_ref().is_some_and(|(_, etag, json, starred)| {
                *etag == contact.etag && *json == card_json && *starred == contact.starred
            });
            let id = match old {
                Some((id, ..)) if same => id,
                Some((id, ..)) => {
                    tx.execute(
                        "UPDATE contact SET etag = ?2, display_name = ?3, sort_key = ?4,
                         job = ?5, phone = ?6, starred = ?7, card_json = ?8, raw = ?9,
                         updated_at = ?10 WHERE id = ?1",
                        params![
                            id,
                            contact.etag,
                            contact.card.display_name(),
                            contact.card.sort_key(),
                            contact.card.job(),
                            contact.card.phones.first().map_or("", |p| p.value.as_str()),
                            contact.starred,
                            card_json,
                            contact.raw,
                            now,
                        ],
                    )?;
                    changed = true;
                    id
                }
                None => {
                    tx.execute(
                        "INSERT INTO contact (book_id, remote_id, etag, display_name,
                         sort_key, job, phone, starred, card_json, raw, updated_at)
                         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                        params![
                            book,
                            contact.remote_id,
                            contact.etag,
                            contact.card.display_name(),
                            contact.card.sort_key(),
                            contact.card.job(),
                            contact.card.phones.first().map_or("", |p| p.value.as_str()),
                            contact.starred,
                            card_json,
                            contact.raw,
                            now,
                        ],
                    )?;
                    changed = true;
                    tx.last_insert_rowid()
                }
            };
            if !same {
                tx.execute("DELETE FROM contact_address WHERE contact_id = ?1", [id])?;
                for (position, email) in contact.card.email_keys().iter().enumerate() {
                    tx.execute(
                        "INSERT OR IGNORE INTO contact_address (contact_id, email_norm, position)
                         VALUES (?1, ?2, ?3)",
                        params![id, email, position as i64],
                    )?;
                }
            }
            if let Some(photo) = &contact.photo {
                tx.execute(
                    "INSERT INTO contact_photo (contact_id, source, data) VALUES (?1, 'card', ?2)
                     ON CONFLICT (contact_id) DO UPDATE SET source = 'card', data = excluded.data
                     WHERE data <> excluded.data",
                    params![id, photo],
                )?;
            }
            // Labels can change without the card changing (Google keeps
            // them apart), so memberships are always brought up to date.
            let mut wanted: BTreeSet<i64> = BTreeSet::new();
            for group in &contact.groups {
                let gid = match group_ids.get(group) {
                    Some(&gid) => gid,
                    None => {
                        tx.execute(
                            "INSERT INTO contact_group (book_id, remote_id, name) VALUES (?1, ?2, ?2)",
                            params![book, group],
                        )?;
                        let gid = tx.last_insert_rowid();
                        group_ids.insert(group.clone(), gid);
                        gid
                    }
                };
                wanted.insert(gid);
            }
            let have: BTreeSet<i64> = tx
                .prepare_cached("SELECT group_id FROM contact_group_member WHERE contact_id = ?1")?
                .query_map([id], |row| row.get(0))?
                .collect::<rusqlite::Result<_>>()?;
            if wanted != have {
                tx.execute(
                    "DELETE FROM contact_group_member WHERE contact_id = ?1",
                    [id],
                )?;
                for group in &wanted {
                    tx.execute(
                        "INSERT INTO contact_group_member (group_id, contact_id) VALUES (?1, ?2)",
                        params![group, id],
                    )?;
                }
                changed = true;
            }
        }

        tx.execute(
            "UPDATE address_book SET synced_at = ?2, state = 'ok',
             sync_token = COALESCE(?3, sync_token) WHERE id = ?1",
            params![book, now, sync.sync_token],
        )?;
        tx.commit()?;
        Ok(changed)
    }

    /// The ETags of `book`'s cards, by `remote_id`.
    pub fn contact_etags(&self, book: i64) -> Result<HashMap<String, String>> {
        let mut stmt = self.pim.prepare_cached(
            "SELECT remote_id, etag FROM contact WHERE book_id = ?1 AND etag IS NOT NULL",
        )?;
        let rows = stmt.query_map([book], |row| Ok((row.get(0)?, row.get(1)?)))?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// The cards of `book` whose picture is at a URL not fetched yet.
    pub fn contact_photos_to_fetch(&self, book: i64) -> Result<Vec<(i64, String)>> {
        let mut stmt = self.pim.prepare_cached(
            "SELECT c.id, json_extract(c.card_json, '$.photo_url') AS url
             FROM contact c LEFT JOIN contact_photo p ON p.contact_id = c.id
             WHERE c.book_id = ?1 AND url IS NOT NULL AND url <> ''
               AND (p.source IS NULL OR p.source <> url)",
        )?;
        let rows = stmt.query_map([book], |row| Ok((row.get(0)?, row.get(1)?)))?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Keeps `data` as card `contact`'s picture, fetched from `source`.
    pub fn set_contact_photo(&self, contact: i64, source: &str, data: &[u8]) -> Result<()> {
        self.check_writable()?;
        self.pim.execute(
            "INSERT INTO contact_photo (contact_id, source, data) VALUES (?1, ?2, ?3)
             ON CONFLICT (contact_id) DO UPDATE SET source = excluded.source, data = excluded.data",
            params![contact, source, data],
        )?;
        Ok(())
    }

    /// The cards that have a picture.
    pub fn contacts_with_photos(&self) -> Result<std::collections::HashSet<i64>> {
        let mut stmt = self
            .pim
            .prepare_cached("SELECT contact_id FROM contact_photo")?;
        let rows = stmt.query_map([], |row| row.get(0))?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// The picture of the first of `ids` that has one.
    pub fn contact_photo(&self, ids: &[i64]) -> Result<Option<Vec<u8>>> {
        let mut stmt = self
            .pim
            .prepare_cached("SELECT data FROM contact_photo WHERE contact_id = ?1")?;
        for id in ids {
            if let Some(data) = stmt.query_row([id], |row| row.get(0)).optional()? {
                return Ok(Some(data));
            }
        }
        Ok(None)
    }

    /// Everyone saved, one entry per person, in A–Z order.
    pub fn saved_contacts(&self) -> Result<Vec<SavedContact>> {
        struct Row {
            id: i64,
            account: Option<AccountId>,
            name: String,
            sort_key: String,
            job: String,
            phone: String,
            starred: bool,
        }
        let rows: Vec<Row> = self
            .pim
            .prepare_cached(
                "SELECT c.id, b.account_id, c.display_name, c.sort_key, c.job, c.phone, c.starred
                 FROM contact c JOIN address_book b ON b.id = c.book_id
                 ORDER BY c.sort_key, c.id",
            )?
            .query_map([], |row| {
                Ok(Row {
                    id: row.get(0)?,
                    account: row.get::<_, Option<i64>>(1)?.map(AccountId),
                    name: row.get(2)?,
                    sort_key: row.get(3)?,
                    job: row.get(4)?,
                    phone: row.get(5)?,
                    starred: row.get(6)?,
                })
            })?
            .collect::<rusqlite::Result<_>>()?;
        let mut emails: HashMap<i64, Vec<String>> = HashMap::new();
        {
            let mut stmt = self.pim.prepare_cached(
                "SELECT contact_id, email_norm FROM contact_address ORDER BY contact_id, position",
            )?;
            for row in stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))? {
                let (id, email): (i64, String) = row?;
                emails.entry(id).or_default().push(email);
            }
        }
        let mut labels: HashMap<i64, Vec<String>> = HashMap::new();
        {
            let mut stmt = self.pim.prepare_cached(
                "SELECT m.contact_id, g.name FROM contact_group_member m
                 JOIN contact_group g ON g.id = m.group_id ORDER BY g.name COLLATE NOCASE",
            )?;
            for row in stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))? {
                let (id, name): (i64, String) = row?;
                labels.entry(id).or_default().push(name);
            }
        }

        // Cards that share an address are one person (union-find).
        let index: HashMap<i64, usize> = rows.iter().enumerate().map(|(i, r)| (r.id, i)).collect();
        let mut parent: Vec<usize> = (0..rows.len()).collect();
        fn root(parent: &mut [usize], mut i: usize) -> usize {
            while parent[i] != i {
                parent[i] = parent[parent[i]];
                i = parent[i];
            }
            i
        }
        let mut owner: HashMap<&str, usize> = HashMap::new();
        for (id, list) in &emails {
            let Some(&i) = index.get(id) else { continue };
            for email in list {
                match owner.get(email.as_str()) {
                    Some(&j) => {
                        let (a, b) = (root(&mut parent, i), root(&mut parent, j));
                        if a != b {
                            parent[a.max(b)] = a.min(b);
                        }
                    }
                    None => {
                        owner.insert(email, i);
                    }
                }
            }
        }

        let mut people: Vec<SavedContact> = Vec::new();
        let mut slot: HashMap<usize, usize> = HashMap::new();
        for (i, row) in rows.iter().enumerate() {
            let r = root(&mut parent, i);
            let at = *slot.entry(r).or_insert_with(|| {
                people.push(SavedContact {
                    name: row.name.clone(),
                    sort_key: row.sort_key.clone(),
                    ..SavedContact::default()
                });
                people.len() - 1
            });
            let person = &mut people[at];
            person.ids.push(row.id);
            for email in emails.get(&row.id).into_iter().flatten() {
                if !person.emails.contains(email) {
                    person.emails.push(email.clone());
                }
            }
            if person.phone.is_empty() {
                person.phone = row.phone.clone();
            }
            if person.job.is_empty() {
                person.job = row.job.clone();
            }
            person.starred |= row.starred;
            for label in labels.get(&row.id).into_iter().flatten() {
                if !person.labels.contains(label) {
                    person.labels.push(label.clone());
                }
            }
            if !person.accounts.contains(&row.account) {
                person.accounts.push(row.account);
            }
        }
        Ok(people)
    }

    /// The cards `ids`, with where each is kept.
    pub fn saved_cards(&self, ids: &[i64]) -> Result<Vec<StoredCard>> {
        let mut out = Vec::with_capacity(ids.len());
        let mut stmt = self.pim.prepare_cached(
            "SELECT c.book_id, b.account_id, b.source, c.card_json, c.starred
             FROM contact c JOIN address_book b ON b.id = c.book_id WHERE c.id = ?1",
        )?;
        let mut labels = self.pim.prepare_cached(
            "SELECT g.name FROM contact_group_member m JOIN contact_group g ON g.id = m.group_id
             WHERE m.contact_id = ?1 ORDER BY g.name COLLATE NOCASE",
        )?;
        for &id in ids {
            let found = stmt
                .query_row([id], |row| {
                    Ok((
                        row.get::<_, i64>(0)?,
                        row.get::<_, Option<i64>>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, bool>(4)?,
                    ))
                })
                .optional()?;
            let Some((book, account, source, json, starred)) = found else {
                continue;
            };
            let card = serde_json::from_str(&json)
                .map_err(|e| Error::InvalidData(format!("contact {id}: {e}")))?;
            out.push(StoredCard {
                id,
                book,
                account: account.map(AccountId),
                source: source.parse()?,
                card,
                starred,
                labels: labels
                    .query_map([id], |row| row.get(0))?
                    .collect::<rusqlite::Result<_>>()?,
            });
        }
        Ok(out)
    }

    /// The saved cards with the address `email` (any case).
    pub fn saved_cards_for(&self, email: &str) -> Result<Vec<StoredCard>> {
        let ids: Vec<i64> = self
            .pim
            .prepare_cached(
                "SELECT contact_id FROM contact_address WHERE email_norm = ?1 ORDER BY contact_id",
            )?
            .query_map([email.trim().to_lowercase()], |row| row.get(0))?
            .collect::<rusqlite::Result<_>>()?;
        self.saved_cards(&ids)
    }

    /// Every label with how many cards have it, by name; labels of the
    /// same name in several books count as one.
    pub fn contact_labels(&self) -> Result<Vec<ContactLabel>> {
        let mut stmt = self.pim.prepare_cached(
            "SELECT g.name, COUNT(DISTINCT m.contact_id) FROM contact_group g
             LEFT JOIN contact_group_member m ON m.group_id = g.id
             GROUP BY g.name COLLATE NOCASE ORDER BY g.name COLLATE NOCASE",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(ContactLabel {
                name: row.get(0)?,
                count: row.get::<_, i64>(1)?.max(0) as usize,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// The labels of `book`.
    pub fn contact_groups(&self, book: i64) -> Result<Vec<SyncedGroup>> {
        let mut stmt = self.pim.prepare_cached(
            "SELECT remote_id, name FROM contact_group WHERE book_id = ?1
             ORDER BY name COLLATE NOCASE",
        )?;
        let rows = stmt.query_map([book], |row| {
            Ok(SyncedGroup {
                remote_id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// The cards of `book` with label `remote_id`.
    pub fn contacts_in_group(&self, book: i64, remote_id: &str) -> Result<Vec<i64>> {
        let mut stmt = self.pim.prepare_cached(
            "SELECT m.contact_id FROM contact_group_member m
             JOIN contact_group g ON g.id = m.group_id
             WHERE g.book_id = ?1 AND g.remote_id = ?2 ORDER BY m.contact_id",
        )?;
        let rows = stmt.query_map(params![book, remote_id], |row| row.get(0))?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Forgets label `remote_id` of `book`; its cards stay.
    pub fn remove_contact_group(&self, book: i64, remote_id: &str) -> Result<bool> {
        self.check_writable()?;
        Ok(self.pim.execute(
            "DELETE FROM contact_group WHERE book_id = ?1 AND remote_id = ?2",
            params![book, remote_id],
        )? > 0)
    }

    /// Forgets the labels of `book` nobody has, for services where a
    /// label lives only on its cards.
    pub fn remove_empty_contact_groups(&self, book: i64) -> Result<usize> {
        self.check_writable()?;
        Ok(self.pim.execute(
            "DELETE FROM contact_group WHERE book_id = ?1 AND NOT EXISTS
             (SELECT 1 FROM contact_group_member m WHERE m.group_id = contact_group.id)",
            [book],
        )?)
    }

    /// The address book `id`.
    pub fn address_book(&self, id: i64) -> Result<Option<AddressBook>> {
        Ok(self.address_books()?.into_iter().find(|b| b.id == id))
    }

    /// The book on this computer, made the first time it is wanted.
    pub fn local_address_book(&self) -> Result<i64> {
        self.ensure_address_book(None, BookSource::Local, "", "")
    }

    /// Card `id` with its book, for writing a change back.
    pub fn contact_ref(&self, id: i64) -> Result<Option<ContactRef>> {
        let found = self
            .pim
            .prepare_cached(
                "SELECT book_id, remote_id, etag, card_json, raw, starred FROM contact
                 WHERE id = ?1",
            )?
            .query_row([id], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, Option<String>>(4)?,
                    row.get::<_, bool>(5)?,
                ))
            })
            .optional()?;
        let Some((book, remote_id, etag, json, raw, starred)) = found else {
            return Ok(None);
        };
        let Some(book) = self.address_book(book)? else {
            return Ok(None);
        };
        let card = serde_json::from_str(&json)
            .map_err(|e| Error::InvalidData(format!("contact {id}: {e}")))?;
        let groups = self
            .pim
            .prepare_cached(
                "SELECT g.remote_id FROM contact_group_member m
                 JOIN contact_group g ON g.id = m.group_id WHERE m.contact_id = ?1",
            )?
            .query_map([id], |row| row.get(0))?
            .collect::<rusqlite::Result<_>>()?;
        Ok(Some(ContactRef {
            id,
            book,
            remote_id,
            etag,
            card,
            raw,
            starred,
            groups,
        }))
    }

    /// The id of the card `remote_id` in `book`.
    pub fn contact_id(&self, book: i64, remote_id: &str) -> Result<Option<i64>> {
        Ok(self
            .pim
            .prepare_cached("SELECT id FROM contact WHERE book_id = ?1 AND remote_id = ?2")?
            .query_row(params![book, remote_id], |row| row.get(0))
            .optional()?)
    }

    /// Every saved address with the name saved for it: for the address
    /// suggestions and names in the mail list. Starred cards first, so
    /// their name wins where an address is saved twice.
    pub fn saved_names(&self) -> Result<Vec<(String, String)>> {
        let mut stmt = self.pim.prepare_cached(
            "SELECT a.email_norm, c.display_name FROM contact_address a
             JOIN contact c ON c.id = a.contact_id
             ORDER BY c.starred DESC, a.position, c.id",
        )?;
        let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// The saved cards with a birthday, as `(id, name, birthday)`: the
    /// birthday is `YYYY-MM-DD`, or `--MM-DD` without the year.
    pub fn contact_birthdays(&self) -> Result<Vec<(i64, String, String)>> {
        let mut stmt = self.pim.prepare_cached(
            "SELECT id, display_name, json_extract(card_json, '$.birthday') AS birthday
             FROM contact WHERE birthday IS NOT NULL AND birthday != ''
             ORDER BY sort_key, id",
        )?;
        let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }
}

#[cfg(test)]
mod tests {
    use katna_core::contact::{Name, Typed};
    use katna_core::{AccountKind, Paths};

    use super::*;
    use crate::Mode;

    fn store() -> (tempfile::TempDir, Store, AccountId, AccountId) {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        paths.create_dirs().unwrap();
        let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
        let a = store
            .add_account(AccountKind::Imap, "Home", "me@gmail.com")
            .unwrap()
            .id;
        let b = store
            .add_account(AccountKind::Imap, "Work", "me@work.in")
            .unwrap()
            .id;
        (tmp, store, a, b)
    }

    fn card(given: &str, email: &str) -> Card {
        Card {
            name: Name {
                given: given.into(),
                ..Name::default()
            },
            emails: vec![Typed::new(email, "work")],
            ..Card::default()
        }
    }

    fn synced(remote: &str, card: Card) -> SyncedContact {
        SyncedContact {
            remote_id: remote.into(),
            etag: Some(format!("e-{remote}")),
            card,
            ..SyncedContact::default()
        }
    }

    #[test]
    fn full_sync_saves_and_sweeps_then_changes_apply() {
        let (_tmp, mut store, a, _) = store();
        let book = store
            .ensure_address_book(Some(a), BookSource::Google, "", "Contacts")
            .unwrap();
        assert_eq!(
            store
                .ensure_address_book(Some(a), BookSource::Google, "", "Contacts")
                .unwrap(),
            book
        );
        let first = BookSync {
            full: true,
            contacts: vec![
                synced("people/1", card("Asha", "asha@x.in")),
                synced("people/2", card("Bilal", "bilal@x.in")),
            ],
            groups: Some(vec![SyncedGroup {
                remote_id: "contactGroups/9".into(),
                name: "Suppliers".into(),
            }]),
            groups_complete: true,
            sync_token: Some("t1".into()),
            ..BookSync::default()
        };
        assert!(store.save_book_sync(book, &first).unwrap());
        assert!(!store.save_book_sync(book, &first).unwrap(), "nothing new");
        assert_eq!(
            store.address_books().unwrap()[0].sync_token.as_deref(),
            Some("t1")
        );

        let mut labelled = synced("people/2", card("Bilal", "bilal@x.in"));
        labelled.groups = vec!["contactGroups/9".into()];
        labelled.starred = true;
        let change = BookSync {
            contacts: vec![labelled],
            deleted: vec!["people/1".into()],
            sync_token: Some("t2".into()),
            ..BookSync::default()
        };
        assert!(store.save_book_sync(book, &change).unwrap());
        let people = store.saved_contacts().unwrap();
        assert_eq!(people.len(), 1);
        assert_eq!(people[0].name, "Bilal");
        assert!(people[0].starred);
        assert_eq!(people[0].labels, ["Suppliers"]);
        assert_eq!(
            store.contact_labels().unwrap(),
            [ContactLabel {
                name: "Suppliers".into(),
                count: 1
            }]
        );

        assert_eq!(store.contact_groups(book).unwrap()[0].name, "Suppliers");
        let bilal = store.contact_id(book, "people/2").unwrap().unwrap();
        assert_eq!(
            store.contacts_in_group(book, "contactGroups/9").unwrap(),
            [bilal]
        );
        assert_eq!(store.remove_empty_contact_groups(book).unwrap(), 0);
        assert!(store.remove_contact_group(book, "contactGroups/9").unwrap());
        assert!(store.contact_labels().unwrap().is_empty());
        assert!(store.saved_contacts().unwrap()[0].labels.is_empty());

        let sweep = BookSync {
            full: true,
            ..BookSync::default()
        };
        assert!(store.save_book_sync(book, &sweep).unwrap());
        assert!(store.saved_contacts().unwrap().is_empty());
    }

    #[test]
    fn one_person_across_accounts_by_shared_address() {
        let (_tmp, mut store, a, b) = store();
        let home = store
            .ensure_address_book(Some(a), BookSource::Google, "", "")
            .unwrap();
        let work = store
            .ensure_address_book(Some(b), BookSource::CardDav, "https://dav/x/", "Work")
            .unwrap();
        let mut both = card("Arjun", "arjun@acme.co");
        both.emails.push(Typed::new("Arjun.M@gmail.com", "home"));
        both.phones.push(Typed::new("+91 99", "mobile"));
        store
            .save_book_sync(
                home,
                &BookSync {
                    full: true,
                    contacts: vec![synced("people/1", both)],
                    ..BookSync::default()
                },
            )
            .unwrap();
        store
            .save_book_sync(
                work,
                &BookSync {
                    full: true,
                    contacts: vec![
                        synced("/x/1.vcf", card("Arjun Mehta", "arjun.m@gmail.com")),
                        synced("/x/2.vcf", card("Chen", "chen@lotus.cn")),
                    ],
                    ..BookSync::default()
                },
            )
            .unwrap();
        let people = store.saved_contacts().unwrap();
        assert_eq!(people.len(), 2);
        let arjun = &people[0];
        assert_eq!(arjun.name, "Arjun");
        assert_eq!(arjun.ids.len(), 2);
        assert_eq!(arjun.emails, ["arjun@acme.co", "arjun.m@gmail.com"]);
        assert_eq!(arjun.phone, "+91 99");
        assert_eq!(arjun.accounts, [Some(a), Some(b)]);
        let cards = store.saved_cards(&arjun.ids).unwrap();
        assert_eq!(cards[1].source, BookSource::CardDav);
        assert_eq!(store.saved_cards_for("ARJUN.M@gmail.com").unwrap().len(), 2);
        assert_eq!(store.saved_names().unwrap().len(), 4);

        // A collection removed on the server goes with its cards.
        store.remove_address_books_except(b, &[]).unwrap();
        assert_eq!(store.saved_contacts().unwrap()[0].ids.len(), 1);
    }

    #[test]
    fn birthdays_are_read_from_the_cards() {
        let (_tmp, mut store, a, _) = store();
        let book = store
            .ensure_address_book(Some(a), BookSource::Google, "", "Contacts")
            .unwrap();
        let mut asha = card("Asha", "asha@x.in");
        asha.birthday = "1990-03-14".into();
        let mut bilal = card("Bilal", "bilal@x.in");
        bilal.birthday = "--07-02".into();
        let sync = BookSync {
            full: true,
            contacts: vec![
                synced("people/1", asha),
                synced("people/2", bilal),
                synced("people/3", card("Chen", "chen@x.in")),
            ],
            ..BookSync::default()
        };
        store.save_book_sync(book, &sync).unwrap();
        let found: Vec<(String, String)> = store
            .contact_birthdays()
            .unwrap()
            .into_iter()
            .map(|(_, name, day)| (name, day))
            .collect();
        assert_eq!(
            found,
            [
                ("Asha".into(), "1990-03-14".into()),
                ("Bilal".into(), "--07-02".into())
            ]
        );
    }

    #[test]
    fn photos_are_fetched_once_per_url() {
        let (_tmp, mut store, a, _) = store();
        let book = store
            .ensure_address_book(Some(a), BookSource::Google, "", "")
            .unwrap();
        let mut with_photo = card("Asha", "asha@x.in");
        with_photo.photo_url = "https://photo/1".into();
        store
            .save_book_sync(
                book,
                &BookSync {
                    full: true,
                    contacts: vec![synced("people/1", with_photo)],
                    ..BookSync::default()
                },
            )
            .unwrap();
        let todo = store.contact_photos_to_fetch(book).unwrap();
        assert_eq!(todo.len(), 1);
        store
            .set_contact_photo(todo[0].0, &todo[0].1, b"png")
            .unwrap();
        assert!(store.contact_photos_to_fetch(book).unwrap().is_empty());
        assert_eq!(store.contact_photo(&[todo[0].0]).unwrap().unwrap(), b"png");
    }
}
