// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna Server: Katna accounts ([`accounts`]) and open and click tracking
//! for mail sent with Katna
//! (`docs/ARCHITECTURE.md` §16.1).
//!
//! The daemon asks for one random tracking ID per recipient and puts
//! `/o/<id>.png` (the open pixel) and `/l/<id>/<n>` (link `n`) into that
//! recipient's copy. When the recipient's mail program fetches the pixel or
//! a link is followed, the server records an event and streams it to the
//! install that asked for the ID.
//!
//! The server stores random IDs, link targets (so it can never redirect
//! anywhere it was not told to), times and a coarse label of who fetched:
//! a person, Apple's privacy proxy or a security scanner. It never sees
//! subjects, recipients or message content, and it does not keep IP
//! addresses or user agents; they are only read to pick the label.

pub mod accounts;
pub mod ai;
pub mod auth;
pub mod classify;
pub mod config;
pub mod db;
pub mod ids;
pub mod limits;
pub mod mailer;
pub mod routes;
pub mod stream;
pub mod translate;

pub use config::Config;
pub use routes::{AppState, router};
