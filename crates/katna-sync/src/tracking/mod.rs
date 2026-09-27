// SPDX-License-Identifier: GPL-3.0-or-later

//! Open and click tracking (`docs/ARCHITECTURE.md` §11, §16.1): tracked
//! copies of a message and the Katna Server client.

pub mod client;
pub mod rewrite;

pub use client::{Client, EventStream, Registration, Server, ServerEvent};
