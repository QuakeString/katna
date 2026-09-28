// SPDX-License-Identifier: GPL-3.0-or-later

//! Configuration, XDG paths, accounts, secrets, identifiers and errors shared by all
//! Katna components. See `docs/ARCHITECTURE.md` §3 and §5.

pub mod account;
pub mod category;
pub mod config;
pub mod crash;
pub mod error;
pub mod ids;
pub mod image;
pub mod logging;
pub mod paths;
pub mod sentry;
pub mod subject;
pub mod update;
pub mod window;

pub use account::{
    Account, AccountId, AccountKind, AccountSettings, OAuthProvider, Pop3Keep, Security, Server,
};
pub use category::{MailCategory, MailFacts, classify};
pub use config::Config;
pub use error::{Error, Result};
pub use paths::Paths;
