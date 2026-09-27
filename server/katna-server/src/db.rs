// SPDX-License-Identifier: GPL-3.0-or-later

//! PostgreSQL storage: installs, tracking IDs with their link targets, and
//! events. Times are milliseconds since the Unix epoch.

use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
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
];

/// Wrong guesses allowed for one emailed code.
pub const CODE_ATTEMPTS: i32 = 5;

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
    /// No code, expired, or too many wrong guesses.
    Gone,
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
    /// Link targets, in the order the daemon numbered them.
    pub links: Vec<String>,
}

/// The database.
#[derive(Clone)]
pub struct Db {
    pool: Pool,
}

impl Db {
    /// Opens a pool of connections to `url`. Connections are made when
    /// first needed.
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
            .build()
            .expect("a pool without timeouts builds without a runtime");
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
        let row = client
            .query_opt(
                "SELECT i.id, i.last_seen, i.account_id, a.verified_at IS NOT NULL
                 FROM installs i LEFT JOIN accounts a ON a.id = i.account_id
                 WHERE i.token_hash = $1",
                &[&token_hash],
            )
            .await?;
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
    /// one counts against [`CODE_ATTEMPTS`].
    pub async fn check_code(
        &self,
        account: &str,
        purpose: &str,
        code_hash: &[u8],
        now: i64,
    ) -> Result<CodeCheck, DbError> {
        let mut client = self.pool.get().await?;
        let tx = client.transaction().await?;
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
    /// `before`, and expired codes. Returns how many accounts went.
    pub async fn purge_accounts(&self, before: i64, now: i64) -> Result<u64, DbError> {
        let client = self.pool.get().await?;
        client
            .execute("DELETE FROM account_codes WHERE expires_at < $1", &[&now])
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

    /// How many tracking IDs `install` created since `since`.
    pub async fn tracks_since(&self, install: &str, since: i64) -> Result<i64, DbError> {
        let row = self
            .pool
            .get()
            .await?
            .query_one(
                "SELECT count(*) FROM tracks WHERE install_id = $1 AND created_at >= $2",
                &[&install, &since],
            )
            .await?;
        Ok(row.get(0))
    }

    /// Stores new tracking IDs, each with the same link targets.
    pub async fn create_tracks(
        &self,
        install: &str,
        ids: &[String],
        links: &[String],
        now: i64,
    ) -> Result<(), DbError> {
        let mut client = self.pool.get().await?;
        let tx = client.transaction().await?;
        let insert = tx
            .prepare(
                "INSERT INTO tracks (id, install_id, created_at, links) VALUES ($1, $2, $3, $4)",
            )
            .await?;
        for id in ids {
            tx.execute(&insert, &[id, &install, &now, &links]).await?;
        }
        tx.commit().await?;
        Ok(())
    }

    /// Deletes one of `install`'s tracking IDs with its events. Returns
    /// whether it existed.
    pub async fn delete_track(&self, install: &str, id: &str) -> Result<bool, DbError> {
        let deleted = self
            .pool
            .get()
            .await?
            .execute(
                "DELETE FROM tracks WHERE id = $1 AND install_id = $2",
                &[&id, &install],
            )
            .await?;
        Ok(deleted > 0)
    }

    /// Looks a tracking ID up.
    pub async fn track(&self, id: &str) -> Result<Option<Track>, DbError> {
        let row = self
            .pool
            .get()
            .await?
            .query_opt(
                "SELECT install_id, created_at, links FROM tracks WHERE id = $1",
                &[&id],
            )
            .await?;
        Ok(row.map(|row| Track {
            install: row.get(0),
            created_at: row.get(1),
            links: row.get(2),
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
        let row = self
            .pool
            .get()
            .await?
            .query_one(
                "INSERT INTO events (install_id, track_id, kind, link, source, at)
                 VALUES ($1, $2, $3, $4, $5, $6) RETURNING seq",
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

    /// Deletes tracking IDs created before `before` (with their events) and
    /// installs not seen since then. Returns how many of each went.
    pub async fn purge(&self, before: i64) -> Result<(u64, u64), DbError> {
        let client = self.pool.get().await?;
        let tracks = client
            .execute("DELETE FROM tracks WHERE created_at < $1", &[&before])
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

/// Milliseconds since the Unix epoch.
pub fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_millis() as i64)
        .unwrap_or(0)
}
