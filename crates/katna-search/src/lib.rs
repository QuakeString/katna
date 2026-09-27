// SPDX-License-Identifier: GPL-3.0-or-later

//! Full-text search index, query language and ranking. See `docs/ARCHITECTURE.md` §7.
//!
//! - [`schema`]: the tantivy schema and tokenizer, versioned by [`SCHEMA_VERSION`].
//! - [`document`]: raw message → searchable text → index document.
//! - [`SearchIndex`]: indexes a [`katna_store::Store`] (fully the first time,
//!   then from its change journal) and searches it.
//! - [`Indexer`]: keeps the index up to date on a background thread
//!   (`katna-daemon`); apps search with [`SearchIndex::open_read_only`].
//! - [`Query`]: the query language, compiled to tantivy queries by [`compile`].
//! - [`contacts`]: recipient suggestions from the addresses in the mail.
//!
//! The index is disposable: it can be deleted and rebuilt from the store.

pub mod compile;
pub mod contacts;
pub mod document;
mod error;
mod highlight;
mod index;
mod indexer;
pub mod query;
pub mod schema;
mod suggest;

pub use error::{Error, Result};
pub use index::{
    Hit, IndexOptions, IndexState, SearchIndex, SearchOptions, SearchResults, Snippet, Sort,
    UpdateStats,
};
pub use indexer::{IndexEvent, Indexer, IndexerOptions, IndexerWaker};
pub use query::{Filter, ParseError, Query, TextField};
pub use schema::SCHEMA_VERSION;
