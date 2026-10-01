// SPDX-License-Identifier: GPL-3.0-or-later

//! PostgreSQL storage: installs, tracking IDs with their link targets, and
//! events. Times are milliseconds since the Unix epoch.

use std::time::Duration;

use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod, Runtime};
use serde::Serialize;
use tokio_postgres::NoTls;

use crate::classify::{Kind, Source};

/// A storage error.
#[derive(Debug, thiserror::Error)]
pub enum DbError {
    /// No connection could be had from the pool.
    #[error("database connection: {0}")]
    Pool(#[from] deadpool_postgres::PoolError),
    /// A query failed.
    #[error("database: {0}")]
    Postgres(#[from] tokio_postgres::Error),
}

/// Schema changes, applied in order. Never edit one that has shipped; add
/// a new one.
const MIGRATIONS: &[&str] = &[
    // 1: installs, tracking IDs, events.
    "CREATE TABLE installs (
         id TEXT PRIMARY KEY,
         token_hash BYTEA NOT NULL UNIQUE,
         created_at BIGINT NOT NULL,
         last_seen BIGINT NOT NULL
     );
     CREATE TABLE tracks (
         id TEXT PRIMARY KEY,
         install_id TEXT NOT NULL REFERENCES installs (id) ON DELETE CASCADE,
         created_at BIGINT NOT NULL,
         links TEXT[] NOT NULL
     );
     CREATE INDEX tracks_install_created ON tracks (install_id, created_at);
     CREATE INDEX tracks_created ON tracks (created_at);
     CREATE TABLE events (
         seq BIGSERIAL PRIMARY KEY,
         install_id TEXT NOT NULL REFERENCES installs (id) ON DELETE CASCADE,
         track_id TEXT NOT NULL REFERENCES tracks (id) ON DELETE CASCADE,
         kind TEXT NOT NULL,
         link INTEGER,
         source TEXT NOT NULL,
         at BIGINT NOT NULL
     );
     CREATE INDEX events_install_seq ON events (install_id, seq);
     CREATE INDEX events_track ON events (track_id);",
    // 2: Katna accounts. An install signed in to an account is one of its
    // devices.
    "CREATE TABLE accounts (
         id TEXT PRIMARY KEY,
         email TEXT NOT NULL UNIQUE,
         password_hash TEXT NOT NULL,
         verified_at BIGINT,
         created_at BIGINT NOT NULL
     );
     CREATE INDEX accounts_unverified ON accounts (created_at) WHERE verified_at IS NULL;
     ALTER TABLE installs
         ADD COLUMN account_id TEXT REFERENCES accounts (id) ON DELETE SET NULL,
         ADD COLUMN device_name TEXT NOT NULL DEFAULT '',
         ADD COLUMN signed_in_at BIGINT;
     CREATE INDEX installs_account ON installs (account_id);
     CREATE TABLE account_codes (
         account_id TEXT NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
         purpose TEXT NOT NULL,
         code_hash BYTEA NOT NULL,
         expires_at BIGINT NOT NULL,
         attempts INTEGER NOT NULL DEFAULT 0,
         PRIMARY KEY (account_id, purpose)
     );",
    // 3: link targets stored once per request rather than once per
    // recipient, and each account's use per day (kept after its installs
    // go, so forgetting an install does not reset the daily limits).
    "CREATE TABLE link_sets (
         id BIGSERIAL PRIMARY KEY,
         install_id TEXT NOT NULL REFERENCES installs (id) ON DELETE CASCADE,
         created_at BIGINT NOT NULL,
         links TEXT[] NOT NULL
     );
     CREATE INDEX link_sets_created ON link_sets (created_at);
     ALTER TABLE tracks ADD COLUMN link_set BIGINT REFERENCES link_sets (id) ON DELETE CASCADE;
     CREATE INDEX tracks_link_set ON tracks (link_set);
     CREATE TABLE track_usage (
         account_id TEXT NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
         created_at BIGINT NOT NULL,
         ids INTEGER NOT NULL,
         link_bytes BIGINT NOT NULL
     );
     CREATE INDEX track_usage_account_created ON track_usage (account_id, created_at);",
    // 4: wrong guesses of emailed codes per account, kept across new codes
    // and restarts.
    "CREATE TABLE code_failures (
         account_id TEXT NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
         at BIGINT NOT NULL
     );
     CREATE INDEX code_failures_account_at ON code_failures (account_id, at);",
    // 5: Katna AI: each account's free month and paid time, and what it
    // cost per calendar month (UTC, as yyyymm), per account and for all.
    "CREATE TABLE ai_plans (
         account_id TEXT PRIMARY KEY REFERENCES accounts (id) ON DELETE CASCADE,
         first_use BIGINT NOT NULL,
         paid_until BIGINT
     );
     CREATE TABLE ai_usage (
         account_id TEXT NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
         month INTEGER NOT NULL,
         requests BIGINT NOT NULL DEFAULT 0,
         cost_micros BIGINT NOT NULL DEFAULT 0,
         PRIMARY KEY (account_id, month)
     );
     CREATE TABLE ai_spend (
         month INTEGER PRIMARY KEY,
         requests BIGINT NOT NULL DEFAULT 0,
         cost_micros BIGINT NOT NULL DEFAULT 0
     );",
    // 6: Katna AI's settings from the admin page (JSON), over those of the
    // environment.
    "CREATE TABLE ai_settings (
         id INTEGER PRIMARY KEY CHECK (id = 1),
         value TEXT NOT NULL,
         updated_at BIGINT NOT NULL,
         updated_by TEXT NOT NULL
     );",
    // 7: the admin page's own sign-in, apart from Katna accounts: a
    // password set on the server (`katna-server admin-password`), and the
    // code mailed for the second step with its wrong guesses.
    "CREATE TABLE admins (
         email TEXT PRIMARY KEY,
         password_hash TEXT NOT NULL,
         code_hash BYTEA,
         code_expires_at BIGINT,
         code_attempts INTEGER NOT NULL DEFAULT 0,
         failures BIGINT[] NOT NULL DEFAULT '{}',
         updated_at BIGINT NOT NULL
     );",
];

/// Wrong guesses allowed for one emailed code.
pub const CODE_ATTEMPTS: i32 = 5;

/// Wrong guesses allowed per account in 24 hours, over all its codes (a
/// new code does not start this over).
pub const CODE_FAILURES_PER_DAY: i64 = 10;

/// How long a request waits for a database connection.
const POOL_WAIT: Duration = Duration::from_secs(5);

/// An install as a request's token finds it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstallAuth {
    /// The install.
    pub id: String,
    /// The Katna account it is signed in to.
    pub account: Option<String>,
    /// Whether that account's address is confirmed.
    pub verified: bool,
}

/// A Katna account as stored.
#[derive(Clone, Debug)]
pub struct Account {
    /// Random ID.
    pub id: String,
    /// Address, lowercased.
    pub email: String,
    /// Argon2 hash in PHC form.
    pub password_hash: String,
    /// When the address was confirmed.
    pub verified_at: Option<i64>,
    /// When the account was created.
    pub created_at: i64,
}

/// One device (install) signed in to an account.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Device {
    /// The install's ID.
    pub id: String,
    /// The name the device gave (its host name).
    pub name: String,
    /// When it signed in.
    pub signed_in_at: i64,
    /// When it was last seen (to the hour).
    pub last_seen: i64,
}

/// What checking an emailed code found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodeCheck {
    /// Right; the code is used up.
    Right,
    /// Wrong; it may be tried again.
    Wrong,
    /// No code, expired, or too many wrong guesses at it.
    Gone,
    /// Too many wrong guesses at the account's codes in the last 24 hours;
    /// the code was not checked.
    Locked,
}

/// A recorded open or click, as the event stream sends it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Event {
    /// Increasing number; the stream resumes after the last one seen.
    pub seq: i64,
    /// The install the tracking ID belongs to (not sent).
    #[serde(skip)]
    pub install: String,
    /// The tracking ID.
    pub id: String,
    /// `open` or `click`.
    pub kind: &'static str,
    /// For a click, the link's number in the message (0-based).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<i32>,
    /// `person`, `apple_proxy` or `scanner`.
    pub source: &'static str,
    /// When, in milliseconds since the Unix epoch.
    pub at: i64,
}

/// A tracking ID as stored.
#[derive(Clone, Debug)]
pub struct Track {
    /// The install that created it.
    pub install: String,
    /// When it was created (the mail was sent).
    pub created_at: i64,
    /// The link target asked for, when there is one with that number.
    pub link: Option<String>,
}

/// An account's limits on new tracking IDs per 24 hours.
#[derive(Clone, Copy, Debug)]
pub struct TrackLimits {
    /// Tracking IDs.
    pub ids: u32,
    /// Bytes of link targets, counted once per request.
    pub link_bytes: u64,
}

/// Which daily limit a request for tracking IDs would pass.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverLimit {
    /// Too many tracking IDs.
    Ids,
    /// Too many bytes of link targets.
    LinkBytes,
}

/// An account's time with Katna AI.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AiPlan {
    /// Its first use, which started the free month.
    pub first_use: i64,
    /// The end of the time paid for, if any.
    pub paid_until: Option<i64>,
}

/// What Katna AI cost this month, in millionths of a US dollar.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AiSpent {
    /// For one account.
    pub account: i64,
    /// For all accounts together.
    pub everyone: i64,
}

/// Katna AI's use, for the admin page.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct AiStats {
    /// Requests this month.
    pub requests: i64,
    /// What they cost, in millionths of a US dollar.
    pub cost_micros: i64,
    /// Accounts in their free month.
    pub trial_accounts: i64,
    /// Accounts with paid time left.
    pub paid_accounts: i64,
    /// Accounts at their monthly cap.
    pub capped_accounts: i64,
    /// Cost per month, oldest first: `(yyyymm, millionths)`.
    pub months: Vec<(i32, i64)>,
}

/// The database.
#[derive(Clone)]
pub struct Db {
    pool: Pool,
}

impl Db {
    /// Opens a pool of connections to `url`. Connections are made when
    /// first needed; a request that waits more than a few seconds for one
    /// fails rather than queueing without end.
    pub fn connect(url: &str) -> Result<Self, tokio_postgres::Error> {
        let config: tokio_postgres::Config = url.parse()?;
        let manager = Manager::from_config(
            config,
            NoTls,
            ManagerConfig {
                recycling_method: RecyclingMethod::Fast,
            },
        );
        let pool = Pool::builder(manager)
            .max_size(16)
            .runtime(Runtime::Tokio1)
            .wait_timeout(Some(POOL_WAIT))
            .create_timeout(Some(POOL_WAIT))
            .recycle_timeout(Some(POOL_WAIT))
            .build()
            .expect("a pool with a runtime builds");
        Ok(Self { pool })
    }

    /// Brings the schema up to date. Several servers starting at once take
    /// turns through an advisory lock.
    pub async fn migrate(&self) -> Result<(), DbError> {
        let mut client = self.pool.get().await?;
        let tx = client.transaction().await?;
        tx.batch_execute(
            "SELECT pg_advisory_xact_lock(7243901);
             CREATE TABLE IF NOT EXISTS schema_version (version INTEGER NOT NULL);",
        )
        .await?;
        let current: i32 = match tx
            .query_opt("SELECT version FROM schema_version", &[])
            .await?
        {
            Some(row) => row.get(0),
            None => {
                tx.execute("INSERT INTO schema_version (version) VALUES (0)", &[])
                    .await?;
                0
            }
        };
        for (index, migration) in MIGRATIONS.iter().enumerate().skip(current as usize) {
            tx.batch_execute(migration).await?;
            let version = index as i32 + 1;
            tx.execute("UPDATE schema_version SET version = $1", &[&version])
                .await?;
            tracing::info!(version, "database schema updated");
        }
        tx.commit().await?;
        Ok(())
    }

    /// Checks that the database answers.
    pub async fn ping(&self) -> Result<(), DbError> {
        self.pool.get().await?.execute("SELECT 1", &[]).await?;
        Ok(())
    }

    /// Stores a new install.
    pub async fn create_install(
        &self,
        id: &str,
        token_hash: &[u8],
        now: i64,
    ) -> Result<(), DbError> {
        self.pool
            .get()
            .await?
            .execute(
                "INSERT INTO installs (id, token_hash, created_at, last_seen) VALUES ($1, $2, $3, $3)",
                &[&id, &token_hash, &now],
            )
            .await?;
        Ok(())
    }

    /// The install a token belongs to, noting that it was seen (at most
    /// once an hour, to spare writes).
    pub async fn install_for_token(
        &self,
        token_hash: &[u8],
        now: i64,
    ) -> Result<Option<String>, DbError> {
        Ok(self
            .install_auth(token_hash, now)
            .await?
            .map(|install| install.id))
    }

    /// The install a token belongs to with its account, noting that it was
    /// seen (at most once an hour, to spare writes).
    pub async fn install_auth(
        &self,
        token_hash: &[u8],
        now: i64,
    ) -> Result<Option<InstallAuth>, DbError> {
        let client = self.pool.get().await?;
        let statement = client
            .prepare_cached(
                "SELECT i.id, i.last_seen, i.account_id, a.verified_at IS NOT NULL
                 FROM installs i LEFT JOIN accounts a ON a.id = i.account_id
                 WHERE i.token_hash = $1",
            )
            .await?;
        let row = client.query_opt(&statement, &[&token_hash]).await?;
        let Some(row) = row else { return Ok(None) };
        let id: String = row.get(0);
        let last_seen: i64 = row.get(1);
        let account: Option<String> = row.get(2);
        let verified = account.is_some() && row.get::<_, bool>(3);
        if now - last_seen > 3_600_000 {
            client
                .execute(
                    "UPDATE installs SET last_seen = $2 WHERE id = $1",
                    &[&id, &now],
                )
                .await?;
        }
        Ok(Some(InstallAuth {
            id,
            account,
            verified,
        }))
    }

    /// Stores a new account. Returns `false` when the address already has
    /// one.
    pub async fn create_account(
        &self,
        id: &str,
        email: &str,
        password_hash: &str,
        now: i64,
    ) -> Result<bool, DbError> {
        let created = self
            .pool
            .get()
            .await?
            .execute(
                "INSERT INTO accounts (id, email, password_hash, created_at)
                 VALUES ($1, $2, $3, $4) ON CONFLICT (email) DO NOTHING",
                &[&id, &email, &password_hash, &now],
            )
            .await?;
        Ok(created > 0)
    }

    /// The account with this (lowercased) address.
    pub async fn account_by_email(&self, email: &str) -> Result<Option<Account>, DbError> {
        self.account_where("email", email).await
    }

    /// The account with this ID.
    pub async fn account(&self, id: &str) -> Result<Option<Account>, DbError> {
        self.account_where("id", id).await
    }

    async fn account_where(&self, column: &str, value: &str) -> Result<Option<Account>, DbError> {
        let query = format!(
            "SELECT id, email, password_hash, verified_at, created_at FROM accounts WHERE {column} = $1"
        );
        let row = self.pool.get().await?.query_opt(&query, &[&value]).await?;
        Ok(row.map(|row| Account {
            id: row.get(0),
            email: row.get(1),
            password_hash: row.get(2),
            verified_at: row.get(3),
            created_at: row.get(4),
        }))
    }

    /// Marks the account's address as confirmed.
    pub async fn set_verified(&self, account: &str, now: i64) -> Result<(), DbError> {
        self.pool
            .get()
            .await?
            .execute(
                "UPDATE accounts SET verified_at = COALESCE(verified_at, $2) WHERE id = $1",
                &[&account, &now],
            )
            .await?;
        Ok(())
    }

    /// Replaces the account's password hash.
    pub async fn set_password(&self, account: &str, password_hash: &str) -> Result<(), DbError> {
        self.pool
            .get()
            .await?
            .execute(
                "UPDATE accounts SET password_hash = $2 WHERE id = $1",
                &[&account, &password_hash],
            )
            .await?;
        Ok(())
    }

    /// Deletes the account, its devices and all their data.
    pub async fn delete_account(&self, account: &str) -> Result<(), DbError> {
        let mut client = self.pool.get().await?;
        let tx = client.transaction().await?;
        tx.execute("DELETE FROM installs WHERE account_id = $1", &[&account])
            .await?;
        tx.execute("DELETE FROM accounts WHERE id = $1", &[&account])
            .await?;
        tx.commit().await?;
        Ok(())
    }

    /// Signs `install` in to `account` as a device called `name`.
    pub async fn sign_in(
        &self,
        install: &str,
        account: &str,
        name: &str,
        now: i64,
    ) -> Result<(), DbError> {
        self.pool
            .get()
            .await?
            .execute(
                "UPDATE installs SET account_id = $2, device_name = $3, signed_in_at = $4
                 WHERE id = $1",
                &[&install, &account, &name, &now],
            )
            .await?;
        Ok(())
    }

    /// Signs `install` out of `account`. Returns whether it was signed in
    /// to it.
    pub async fn sign_out(&self, account: &str, install: &str) -> Result<bool, DbError> {
        let signed_out = self
            .pool
            .get()
            .await?
            .execute(
                "UPDATE installs SET account_id = NULL, signed_in_at = NULL
                 WHERE id = $1 AND account_id = $2",
                &[&install, &account],
            )
            .await?;
        Ok(signed_out > 0)
    }

    /// Whether `install` is still signed in to `account` and its address is
    /// confirmed (an open event stream checks this now and then).
    pub async fn still_signed_in(&self, install: &str, account: &str) -> Result<bool, DbError> {
        Ok(self
            .pool
            .get()
            .await?
            .query_opt(
                "SELECT 1 FROM installs i JOIN accounts a ON a.id = i.account_id
                 WHERE i.id = $1 AND i.account_id = $2 AND a.verified_at IS NOT NULL",
                &[&install, &account],
            )
            .await?
            .is_some())
    }

    /// Signs every device of `account` but `keep` out.
    pub async fn sign_out_others(&self, account: &str, keep: &str) -> Result<u64, DbError> {
        Ok(self
            .pool
            .get()
            .await?
            .execute(
                "UPDATE installs SET account_id = NULL, signed_in_at = NULL
                 WHERE account_id = $1 AND id <> $2",
                &[&account, &keep],
            )
            .await?)
    }

    /// The devices signed in to `account`, most recently signed in first.
    pub async fn devices(&self, account: &str) -> Result<Vec<Device>, DbError> {
        let rows = self
            .pool
            .get()
            .await?
            .query(
                "SELECT id, device_name, COALESCE(signed_in_at, created_at), last_seen
                 FROM installs WHERE account_id = $1 ORDER BY signed_in_at DESC, id",
                &[&account],
            )
            .await?;
        Ok(rows
            .into_iter()
            .map(|row| Device {
                id: row.get(0),
                name: row.get(1),
                signed_in_at: row.get(2),
                last_seen: row.get(3),
            })
            .collect())
    }

    /// Stores a new emailed code for `account`, replacing an earlier one
    /// for the same purpose.
    pub async fn put_code(
        &self,
        account: &str,
        purpose: &str,
        code_hash: &[u8],
        expires_at: i64,
    ) -> Result<(), DbError> {
        self.pool
            .get()
            .await?
            .execute(
                "INSERT INTO account_codes (account_id, purpose, code_hash, expires_at)
                 VALUES ($1, $2, $3, $4)
                 ON CONFLICT (account_id, purpose)
                 DO UPDATE SET code_hash = $3, expires_at = $4, attempts = 0",
                &[&account, &purpose, &code_hash, &expires_at],
            )
            .await?;
        Ok(())
    }

    /// Checks a code typed for `account`. A right code is used up; a wrong
    /// one counts against [`CODE_ATTEMPTS`] for the code and
    /// [`CODE_FAILURES_PER_DAY`] for the account. Checks for one account
    /// take turns, so parallel guesses cannot pass either limit.
    pub async fn check_code(
        &self,
        account: &str,
        purpose: &str,
        code_hash: &[u8],
        now: i64,
    ) -> Result<CodeCheck, DbError> {
        let mut client = self.pool.get().await?;
        let tx = client.transaction().await?;
        tx.execute(
            "SELECT pg_advisory_xact_lock(hashtextextended($1, 7243903))",
            &[&account],
        )
        .await?;
        let failures: i64 = tx
            .query_one(
                "SELECT count(*) FROM code_failures WHERE account_id = $1 AND at > $2",
                &[&account, &(now - 86_400_000)],
            )
            .await?
            .get(0);
        if failures >= CODE_FAILURES_PER_DAY {
            return Ok(CodeCheck::Locked);
        }
        let row = tx
            .query_opt(
                "SELECT code_hash, expires_at, attempts FROM account_codes
                 WHERE account_id = $1 AND purpose = $2 FOR UPDATE",
                &[&account, &purpose],
            )
            .await?;
        let Some(row) = row else {
            return Ok(CodeCheck::Gone);
        };
        let stored: Vec<u8> = row.get(0);
        let expires_at: i64 = row.get(1);
        let attempts: i32 = row.get(2);
        let check = if expires_at < now || attempts >= CODE_ATTEMPTS {
            CodeCheck::Gone
        } else if constant_time_eq(&stored, code_hash) {
            CodeCheck::Right
        } else {
            CodeCheck::Wrong
        };
        if check == CodeCheck::Wrong {
            tx.execute(
                "UPDATE account_codes SET attempts = attempts + 1
                 WHERE account_id = $1 AND purpose = $2",
                &[&account, &purpose],
            )
            .await?;
            tx.execute(
                "INSERT INTO code_failures (account_id, at) VALUES ($1, $2)",
                &[&account, &now],
            )
            .await?;
        } else {
            tx.execute(
                "DELETE FROM account_codes WHERE account_id = $1 AND purpose = $2",
                &[&account, &purpose],
            )
            .await?;
        }
        tx.commit().await?;
        Ok(check)
    }

    /// Deletes accounts whose address was never confirmed, created before
    /// `before`, expired codes and wrong guesses older than a day. Returns
    /// how many accounts went.
    pub async fn purge_accounts(&self, before: i64, now: i64) -> Result<u64, DbError> {
        let client = self.pool.get().await?;
        client
            .execute("DELETE FROM account_codes WHERE expires_at < $1", &[&now])
            .await?;
        client
            .execute(
                "DELETE FROM code_failures WHERE at < $1",
                &[&(now - 86_400_000)],
            )
            .await?;
        Ok(client
            .execute(
                "DELETE FROM accounts WHERE verified_at IS NULL AND created_at < $1",
                &[&before],
            )
            .await?)
    }

    /// Deletes an install with its tracking IDs and events.
    pub async fn delete_install(&self, install: &str) -> Result<(), DbError> {
        self.pool
            .get()
            .await?
            .execute("DELETE FROM installs WHERE id = $1", &[&install])
            .await?;
        Ok(())
    }

    /// Stores new tracking IDs for `install` of `account`, all sharing the
    /// link targets `links`, unless that would take the account past
    /// `limits` for the 24 hours before `now`. Requests of one account take
    /// turns, so parallel ones cannot pass the limits together.
    pub async fn create_tracks(
        &self,
        account: &str,
        install: &str,
        ids: &[String],
        links: &[String],
        now: i64,
        limits: TrackLimits,
    ) -> Result<Result<(), OverLimit>, DbError> {
        let link_bytes: i64 = links.iter().map(|link| link.len() as i64).sum();
        let mut client = self.pool.get().await?;
        let tx = client.transaction().await?;
        tx.execute(
            "SELECT pg_advisory_xact_lock(hashtextextended($1, 7243902))",
            &[&account],
        )
        .await?;
        let used = tx
            .query_one(
                "SELECT COALESCE(sum(ids), 0)::BIGINT, COALESCE(sum(link_bytes), 0)::BIGINT
                 FROM track_usage WHERE account_id = $1 AND created_at >= $2",
                &[&account, &(now - 86_400_000)],
            )
            .await?;
        let (used_ids, used_bytes): (i64, i64) = (used.get(0), used.get(1));
        if used_ids + ids.len() as i64 > i64::from(limits.ids) {
            return Ok(Err(OverLimit::Ids));
        }
        if used_bytes + link_bytes > i64::try_from(limits.link_bytes).unwrap_or(i64::MAX) {
            return Ok(Err(OverLimit::LinkBytes));
        }
        tx.execute(
            "INSERT INTO track_usage (account_id, created_at, ids, link_bytes) VALUES ($1, $2, $3, $4)",
            &[&account, &now, &(ids.len() as i32), &link_bytes],
        )
        .await?;
        let link_set: i64 = tx
            .query_one(
                "INSERT INTO link_sets (install_id, created_at, links) VALUES ($1, $2, $3) RETURNING id",
                &[&install, &now, &links],
            )
            .await?
            .get(0);
        // One statement for all the IDs, not a round trip each.
        tx.execute(
            "INSERT INTO tracks (id, install_id, created_at, links, link_set)
             SELECT id, $2, $3, '{}', $4 FROM unnest($1::text[]) AS id",
            &[&ids, &install, &now, &link_set],
        )
        .await?;
        tx.commit().await?;
        Ok(Ok(()))
    }

    /// Deletes one of `install`'s tracking IDs with its events. Returns
    /// whether it existed.
    pub async fn delete_track(&self, install: &str, id: &str) -> Result<bool, DbError> {
        let deleted = self
            .pool
            .get()
            .await?
            .execute(
                "WITH gone AS (
                     DELETE FROM tracks WHERE id = $1 AND install_id = $2 RETURNING link_set
                 ), unused AS (
                     DELETE FROM link_sets s USING gone
                     WHERE s.id = gone.link_set
                       AND NOT EXISTS (SELECT 1 FROM tracks t WHERE t.link_set = s.id AND t.id <> $1)
                 )
                 SELECT 1 FROM gone",
                &[&id, &install],
            )
            .await?;
        Ok(deleted > 0)
    }

    /// Looks a tracking ID up, with its link target number `link` (from 0)
    /// when asked for; only that one target is read.
    pub async fn track(&self, id: &str, link: Option<u32>) -> Result<Option<Track>, DbError> {
        // SQL arrays count from 1; 0 reads as no target.
        let index = link
            .and_then(|n| n.checked_add(1))
            .and_then(|n| i32::try_from(n).ok())
            .unwrap_or(0);
        let client = self.pool.get().await?;
        let statement = client
            .prepare_cached(
                "SELECT t.install_id, t.created_at, (COALESCE(s.links, t.links))[$2]
                 FROM tracks t LEFT JOIN link_sets s ON s.id = t.link_set
                 WHERE t.id = $1",
            )
            .await?;
        let row = client.query_opt(&statement, &[&id, &index]).await?;
        Ok(row.map(|row| Track {
            install: row.get(0),
            created_at: row.get(1),
            link: row.get(2),
        }))
    }

    /// Records an event and returns it with its number.
    pub async fn insert_event(
        &self,
        install: &str,
        id: &str,
        kind: Kind,
        link: Option<i32>,
        source: Source,
        at: i64,
    ) -> Result<Event, DbError> {
        let client = self.pool.get().await?;
        let statement = client
            .prepare_cached(
                "INSERT INTO events (install_id, track_id, kind, link, source, at)
                 VALUES ($1, $2, $3, $4, $5, $6) RETURNING seq",
            )
            .await?;
        let row = client
            .query_one(
                &statement,
                &[&install, &id, &kind.as_str(), &link, &source.as_str(), &at],
            )
            .await?;
        Ok(Event {
            seq: row.get(0),
            install: install.to_owned(),
            id: id.to_owned(),
            kind: kind.as_str(),
            link,
            source: source.as_str(),
            at,
        })
    }

    /// Up to `limit` of `install`'s events after `after`, oldest first.
    pub async fn events_after(
        &self,
        install: &str,
        after: i64,
        limit: i64,
    ) -> Result<Vec<Event>, DbError> {
        let rows = self
            .pool
            .get()
            .await?
            .query(
                "SELECT seq, track_id, kind, link, source, at FROM events
                 WHERE install_id = $1 AND seq > $2 ORDER BY seq LIMIT $3",
                &[&install, &after, &limit],
            )
            .await?;
        Ok(rows
            .into_iter()
            .filter_map(|row| {
                Some(Event {
                    seq: row.get(0),
                    install: install.to_owned(),
                    id: row.get(1),
                    kind: Kind::parse(row.get(2))?.as_str(),
                    link: row.get(3),
                    source: Source::parse(row.get(4))?.as_str(),
                    at: row.get(5),
                })
            })
            .collect())
    }

    /// The account's time with Katna AI; a first use at `now` starts the
    /// free month.
    pub async fn ai_plan(&self, account: &str, now: i64) -> Result<AiPlan, DbError> {
        let row = self
            .pool
            .get()
            .await?
            .query_one(
                "INSERT INTO ai_plans (account_id, first_use) VALUES ($1, $2)
                 ON CONFLICT (account_id) DO UPDATE SET account_id = EXCLUDED.account_id
                 RETURNING first_use, paid_until",
                &[&account, &now],
            )
            .await?;
        Ok(AiPlan {
            first_use: row.get(0),
            paid_until: row.get(1),
        })
    }

    /// Marks the account's Katna AI paid for until `until`.
    pub async fn set_ai_paid_until(
        &self,
        account: &str,
        until: i64,
        now: i64,
    ) -> Result<(), DbError> {
        self.pool
            .get()
            .await?
            .execute(
                "INSERT INTO ai_plans (account_id, first_use, paid_until) VALUES ($1, $3, $2)
                 ON CONFLICT (account_id) DO UPDATE SET paid_until = $2",
                &[&account, &until, &now],
            )
            .await?;
        Ok(())
    }

    /// What Katna AI cost in `month` (yyyymm), for `account` and for all.
    pub async fn ai_spent(&self, account: &str, month: i32) -> Result<AiSpent, DbError> {
        let row = self
            .pool
            .get()
            .await?
            .query_one(
                "SELECT
                     COALESCE((SELECT cost_micros FROM ai_usage
                               WHERE account_id = $1 AND month = $2), 0),
                     COALESCE((SELECT cost_micros FROM ai_spend WHERE month = $2), 0)",
                &[&account, &month],
            )
            .await?;
        Ok(AiSpent {
            account: row.get(0),
            everyone: row.get(1),
        })
    }

    /// Counts one Katna AI request by `account` in `month` that cost
    /// `cost_micros`.
    pub async fn ai_record(
        &self,
        account: &str,
        month: i32,
        cost_micros: i64,
    ) -> Result<(), DbError> {
        let mut client = self.pool.get().await?;
        let tx = client.transaction().await?;
        tx.execute(
            "INSERT INTO ai_usage (account_id, month, requests, cost_micros) VALUES ($1, $2, 1, $3)
             ON CONFLICT (account_id, month) DO UPDATE SET
                 requests = ai_usage.requests + 1,
                 cost_micros = ai_usage.cost_micros + $3",
            &[&account, &month, &cost_micros],
        )
        .await?;
        tx.execute(
            "INSERT INTO ai_spend (month, requests, cost_micros) VALUES ($1, 1, $2)
             ON CONFLICT (month) DO UPDATE SET
                 requests = ai_spend.requests + 1,
                 cost_micros = ai_spend.cost_micros + $2",
            &[&month, &cost_micros],
        )
        .await?;
        tx.commit().await?;
        Ok(())
    }

    /// Sets the admin page password of `email` (a hash), dropping any
    /// code mailed before.
    pub async fn set_admin_password(
        &self,
        email: &str,
        password_hash: &str,
        now: i64,
    ) -> Result<(), DbError> {
        self.pool
            .get()
            .await?
            .execute(
                "INSERT INTO admins (email, password_hash, updated_at) VALUES ($1, $2, $3)
                 ON CONFLICT (email) DO UPDATE SET password_hash = $2, updated_at = $3,
                     code_hash = NULL, code_expires_at = NULL, code_attempts = 0",
                &[&email, &password_hash, &now],
            )
            .await?;
        Ok(())
    }

    /// The admin page password hash of `email`, if one is set.
    pub async fn admin_password(&self, email: &str) -> Result<Option<String>, DbError> {
        Ok(self
            .pool
            .get()
            .await?
            .query_opt(
                "SELECT password_hash FROM admins WHERE email = $1",
                &[&email],
            )
            .await?
            .map(|row| row.get(0)))
    }

    /// Keeps the hash of a code mailed to the admin `email`.
    pub async fn put_admin_code(
        &self,
        email: &str,
        code_hash: &[u8],
        expires_at: i64,
    ) -> Result<(), DbError> {
        self.pool
            .get()
            .await?
            .execute(
                "UPDATE admins SET code_hash = $2, code_expires_at = $3, code_attempts = 0
                 WHERE email = $1",
                &[&email, &code_hash, &expires_at],
            )
            .await?;
        Ok(())
    }

    /// Checks a code typed by the admin `email`, within the same limits as
    /// [`Db::check_code`]: [`CODE_ATTEMPTS`] per code and
    /// [`CODE_FAILURES_PER_DAY`] for the address.
    pub async fn check_admin_code(
        &self,
        email: &str,
        code_hash: &[u8],
        now: i64,
    ) -> Result<CodeCheck, DbError> {
        let mut client = self.pool.get().await?;
        let tx = client.transaction().await?;
        let row = tx
            .query_opt(
                "SELECT code_hash, code_expires_at, code_attempts,
                     (SELECT count(*) FROM unnest(failures) AS at WHERE at > $2)
                 FROM admins WHERE email = $1 FOR UPDATE",
                &[&email, &(now - 86_400_000)],
            )
            .await?;
        let Some(row) = row else {
            return Ok(CodeCheck::Gone);
        };
        let stored: Option<Vec<u8>> = row.get(0);
        let expires_at: Option<i64> = row.get(1);
        let attempts: i32 = row.get(2);
        let failures: i64 = row.get(3);
        if failures >= CODE_FAILURES_PER_DAY {
            return Ok(CodeCheck::Locked);
        }
        let check = match (stored, expires_at) {
            (Some(stored), Some(expires_at)) if expires_at >= now && attempts < CODE_ATTEMPTS => {
                if constant_time_eq(&stored, code_hash) {
                    CodeCheck::Right
                } else {
                    CodeCheck::Wrong
                }
            }
            _ => CodeCheck::Gone,
        };
        if check == CodeCheck::Wrong {
            tx.execute(
                "UPDATE admins SET code_attempts = code_attempts + 1,
                     failures = array_append(
                         ARRAY(SELECT at FROM unnest(failures) AS at WHERE at > $3), $2)
                 WHERE email = $1",
                &[&email, &now, &(now - 86_400_000)],
            )
            .await?;
        } else {
            tx.execute(
                "UPDATE admins SET code_hash = NULL, code_expires_at = NULL, code_attempts = 0
                 WHERE email = $1",
                &[&email],
            )
            .await?;
        }
        tx.commit().await?;
        Ok(check)
    }

    /// Katna AI's settings saved from the admin page, as JSON.
    pub async fn ai_settings(&self) -> Result<Option<String>, DbError> {
        Ok(self
            .pool
            .get()
            .await?
            .query_opt("SELECT value FROM ai_settings WHERE id = 1", &[])
            .await?
            .map(|row| row.get(0)))
    }

    /// Saves Katna AI's settings (JSON), changed by `by`.
    pub async fn set_ai_settings(&self, value: &str, by: &str, now: i64) -> Result<(), DbError> {
        self.pool
            .get()
            .await?
            .execute(
                "INSERT INTO ai_settings (id, value, updated_at, updated_by) VALUES (1, $1, $2, $3)
                 ON CONFLICT (id) DO UPDATE SET value = $1, updated_at = $2, updated_by = $3",
                &[&value, &now, &by],
            )
            .await?;
        Ok(())
    }

    /// Katna AI's use in `months` (yyyymm, the last one this month) at
    /// `now`, with a free time of `trial_ms` and a cap per account of
    /// `cap_micros`.
    pub async fn ai_stats(
        &self,
        months: &[i32],
        now: i64,
        trial_ms: i64,
        cap_micros: i64,
    ) -> Result<AiStats, DbError> {
        let month = *months.last().unwrap_or(&month_of(now));
        let client = self.pool.get().await?;
        let row = client
            .query_one(
                "SELECT
                     COALESCE((SELECT requests FROM ai_spend WHERE month = $1), 0),
                     COALESCE((SELECT cost_micros FROM ai_spend WHERE month = $1), 0),
                     (SELECT count(*) FROM ai_plans
                      WHERE (paid_until IS NULL OR paid_until <= $2) AND first_use > $2 - $3),
                     (SELECT count(*) FROM ai_plans WHERE paid_until > $2),
                     (SELECT count(*) FROM ai_usage WHERE month = $1 AND cost_micros >= $4)",
                &[&month, &now, &trial_ms, &cap_micros],
            )
            .await?;
        let spent: Vec<(i32, i64)> = client
            .query(
                "SELECT month, cost_micros FROM ai_spend WHERE month = ANY($1)",
                &[&months],
            )
            .await?
            .iter()
            .map(|row| (row.get(0), row.get(1)))
            .collect();
        Ok(AiStats {
            requests: row.get(0),
            cost_micros: row.get(1),
            trial_accounts: row.get(2),
            paid_accounts: row.get(3),
            capped_accounts: row.get(4),
            months: months
                .iter()
                .map(|m| {
                    let cost = spent.iter().find(|(s, _)| s == m).map_or(0, |(_, c)| *c);
                    (*m, cost)
                })
                .collect(),
        })
    }

    /// Deletes tracking IDs created before `before` (with their events) and
    /// installs not seen since then. Returns how many of each went.
    pub async fn purge(&self, before: i64) -> Result<(u64, u64), DbError> {
        let client = self.pool.get().await?;
        let tracks = client
            .execute("DELETE FROM tracks WHERE created_at < $1", &[&before])
            .await?;
        client
            .execute("DELETE FROM link_sets WHERE created_at < $1", &[&before])
            .await?;
        client
            .execute(
                "DELETE FROM track_usage WHERE created_at < $1",
                &[&(now_ms() - 2 * 86_400_000)],
            )
            .await?;
        let installs = client
            .execute("DELETE FROM installs WHERE last_seen < $1", &[&before])
            .await?;
        Ok((tracks, installs))
    }
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

/// The calendar month (UTC) of `ms` since the Unix epoch, as yyyymm.
pub fn month_of(ms: i64) -> i32 {
    // Howard Hinnant's days-to-civil.
    let days = ms.div_euclid(86_400_000) + 719_468;
    let era = days.div_euclid(146_097);
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted = (5 * day_of_year + 2) / 153;
    let month = if shifted < 10 {
        shifted + 3
    } else {
        shifted - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    (year * 100 + month) as i32
}

/// The `count` calendar months up to and including that of `ms`, oldest
/// first, as yyyymm.
pub fn months_to(ms: i64, count: usize) -> Vec<i32> {
    let last = month_of(ms);
    let (mut year, mut month) = (last / 100, last % 100);
    let mut months = vec![last];
    while months.len() < count {
        month -= 1;
        if month == 0 {
            month = 12;
            year -= 1;
        }
        months.push(year * 100 + month);
    }
    months.reverse();
    months
}

/// Milliseconds since the Unix epoch.
pub fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_millis() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn months() {
        assert_eq!(month_of(0), 197_001);
        // 2026-10-01 00:00:00 UTC, and the millisecond before.
        assert_eq!(month_of(1_790_812_800_000), 202_610);
        assert_eq!(month_of(1_790_812_799_999), 202_609);
        // 2024-02-29.
        assert_eq!(month_of(1_709_164_800_000), 202_402);
        assert_eq!(month_of(1_709_251_200_000), 202_403);
        assert_eq!(
            months_to(1_709_251_200_000, 6),
            [202_310, 202_311, 202_312, 202_401, 202_402, 202_403]
        );
    }
}
