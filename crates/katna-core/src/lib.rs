// SPDX-License-Identifier: GPL-3.0-or-later

//! Configuration, XDG paths, accounts, secrets, identifiers and errors shared by all
//! Katna components. See `docs/ARCHITECTURE.md` §3 and §5.

pub mod account;
pub mod config;
pub mod error;
pub mod ids;
pub mod logging;
pub mod paths;

pub use account::{Account, AccountId, AccountKind, AccountSettings, Security, Server};
pub use config::Config;
pub use error::{Error, Result};
pub use paths::Paths;
