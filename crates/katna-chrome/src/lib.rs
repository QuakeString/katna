// SPDX-License-Identifier: GPL-3.0-or-later

//! Window decorations (server-side on KDE, client-side on GNOME) and theme
//! tokens. See `docs/ARCHITECTURE.md` §13.1 and `docs/spikes/S1-window-chrome.md`.
//!
//! [`desktop`], [`geometry`] and [`tokens`] have no GPUI types; [`frame`]
//! draws the chrome with GPUI.

pub mod desktop;
pub mod frame;
pub mod geometry;
pub mod tokens;

pub use desktop::{DecorationMode, Desktop, Environment, Preset, Session};
pub use frame::{Bar, WindowChrome, window_options};
pub use tokens::ChromeTokens;
