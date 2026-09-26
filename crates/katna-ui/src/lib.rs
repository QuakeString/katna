// SPDX-License-Identifier: GPL-3.0-or-later

//! Shared GPUI components for Katna apps. GPUI types stay in this crate,
//! `katna-chrome` and the GUI apps. See `docs/ARCHITECTURE.md` §13.

pub mod text_input;

pub use text_input::{InputEvent, TextInput};
