// SPDX-License-Identifier: GPL-3.0-or-later

//! How full each account's mail storage is, as its server reports it
//! (IMAP QUOTA). The daemon writes it on each full sync; the folder pane
//! shows it.

use katna_core::AccountId;
use rusqlite::{OptionalExtension, params};

use crate::Store;
use crate::error::Result;
use crate::mail::MailBatch;

/// Used and total storage of an account, in bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorageQuota {
    pub used: u64,
    pub limit: u64,
}

impl StorageQuota {
    /// The used share, from 0 to 1.
    pub fn fraction(self) -> f32 {
        if self.limit == 0 {
            return 0.0;
        }
        (self.used as f64 / self.limit as f64).clamp(0.0, 1.0) as f32
    }
}

impl Store {
    /// The last storage quota the server of `account` reported, if any.
    pub fn quota(&self, account: AccountId) -> Result<Option<StorageQuota>> {
        Ok(self
            .mail
            .prepare_cached("SELECT used, quota_max FROM quota WHERE account_id = ?1")?
            .query_row([account.0], |row| {
                Ok(StorageQuota {
                    used: row.get::<_, i64>(0)?.max(0) as u64,
                    limit: row.get::<_, i64>(1)?.max(0) as u64,
                })
            })
            .optional()?)
    }
}

impl MailBatch<'_> {
    /// Records the storage quota of `account` at `now`, or forgets it
    /// (`None`). Returns whether the numbers changed.
    pub fn set_quota(
        &mut self,
        account: AccountId,
        quota: Option<StorageQuota>,
        now: i64,
    ) -> Result<bool> {
        let tx = self.tx();
        let old = tx
            .prepare_cached("SELECT used, quota_max FROM quota WHERE account_id = ?1")?
            .query_row([account.0], |row| {
                Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?))
            })
            .optional()?;
        let new = quota.map(|q| (clamp(q.used), clamp(q.limit)));
        match new {
            Some((used, limit)) => {
                tx.prepare_cached(
                    "INSERT INTO quota (account_id, used, quota_max, checked_at)
                     VALUES (?1, ?2, ?3, ?4)
                     ON CONFLICT (account_id) DO UPDATE
                     SET used = ?2, quota_max = ?3, checked_at = ?4",
                )?
                .execute(params![account.0, used, limit, now])?;
            }
            None => {
                tx.prepare_cached("DELETE FROM quota WHERE account_id = ?1")?
                    .execute([account.0])?;
            }
        }
        Ok(old != new)
    }
}

fn clamp(bytes: u64) -> i64 {
    i64::try_from(bytes).unwrap_or(i64::MAX)
}

#[cfg(test)]
mod tests {
    use katna_core::{AccountKind, Paths};

    use super::*;
    use crate::Mode;

    #[test]
    fn quota_is_kept_per_account() {
        let tmp = tempfile::tempdir().unwrap();
        let mut store = Store::open(&Paths::with_root(tmp.path()), Mode::ReadWrite).unwrap();
        let account = store
            .add_account(AccountKind::Imap, "Work", "me@example.org")
            .unwrap()
            .id;
        assert_eq!(store.quota(account).unwrap(), None);
        let quota = StorageQuota {
            used: 34,
            limit: 100,
        };
        let mut batch = store.mail_batch().unwrap();
        assert!(batch.set_quota(account, Some(quota), 1).unwrap());
        assert!(!batch.set_quota(account, Some(quota), 2).unwrap());
        batch.commit().unwrap();
        assert_eq!(store.quota(account).unwrap(), Some(quota));
        assert!((quota.fraction() - 0.34).abs() < 1e-6);
        let mut batch = store.mail_batch().unwrap();
        assert!(batch.set_quota(account, None, 3).unwrap());
        batch.commit().unwrap();
        assert_eq!(store.quota(account).unwrap(), None);
    }
}
