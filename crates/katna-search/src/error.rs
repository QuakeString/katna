// SPDX-License-Identifier: GPL-3.0-or-later

use std::path::PathBuf;

/// Errors of the search index.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("search index: {0}")]
    Index(#[from] tantivy::TantivyError),
    #[error(transparent)]
    Store(#[from] katna_store::Error),
    #[error("query: {0}")]
    Query(#[from] crate::query::ParseError),
    #[error("no search index in {0}; build it first")]
    NotFound(PathBuf),
    #[error("search index version {found} in {path} does not match version {expected}; rebuild it")]
    SchemaVersion {
        path: PathBuf,
        found: u32,
        expected: u32,
    },
    #[error("search index state: {0}")]
    State(String),
    #[error("{path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
