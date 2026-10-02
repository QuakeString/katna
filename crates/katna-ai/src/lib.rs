// SPDX-License-Identifier: GPL-3.0-or-later

//! Writing help from an AI service (`docs/ARCHITECTURE.md` §16.5):
//! rephrasing the text the user selected in a message, and finishing the
//! sentence being written, summing up a conversation, and writing a
//! first draft of a reply ([`draft`]). No network and no GPUI here: what to ask
//! ([`prompt`]), how each kind of service wants it asked and answers
//! ([`provider`]), and what Katna Mail, the daemon and Katna Server say
//! to each other about it ([`wire`]). The daemon sends the requests for a
//! key of the user's own; Katna Server sends them for Katna AI.

pub mod draft;
pub mod prompt;
pub mod provider;
pub mod summary;
pub mod wire;

pub use prompt::{Prompt, Tone};
pub use provider::{Call, Kind, PRESETS, Preset, ProviderError};
