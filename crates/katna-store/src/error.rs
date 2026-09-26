// SPDX-License-Identifier: GPL-3.0-or-later

//! Error type of `katna-store`.

use std::io;
use std::path::PathBuf;

use crate::blob::BlobHash;

/// Errors from opening, migrating or using the store.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An SQLite call failed.
    #[error("database error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    /// A file operation (blob files, directories) failed.
    #[error("{path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    /// Creating the data directories failed.
    #[error(transparent)]
    Core(#[from] katna_core::Error),

    /// A database opened read-only does not exist yet.
    #[error("{path}: database not found (is katna-daemon running?)")]
    NotFound { path: PathBuf },

    /// The database was written by a newer Katna.
    #[error("{path}: schema version {found} is newer than this Katna supports ({supported})")]
    SchemaTooNew {
        path: PathBuf,
        found: u32,
        supported: u32,
    },

    /// A read-only database has an older schema; the daemon must migrate it
    /// first.
    #[error("{path}: schema version {found} needs migration to {expected} by katna-daemon")]
    SchemaOutdated {
        path: PathBuf,
        found: u32,
        expected: u32,
    },

    /// A write was attempted on a store opened read-only.
    #[error("the store is open read-only")]
    ReadOnly,

    /// A blob's content does not match its hash.
    #[error("blob {0} is corrupt")]
    CorruptBlob(BlobHash),

    /// A row holds a value this version cannot interpret.
    #[error("invalid value in database: {0}")]
    InvalidData(String),
}

impl Error {
    pub(crate) fn io(path: impl Into<PathBuf>, source: io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }
}

/// Result type of `katna-store`.
pub type Result<T, E = Error> = std::result::Result<T, E>;
