// SPDX-License-Identifier: GPL-3.0-or-later

//! Desktop integration: portals, desktop detection, settings, tray and badges.
//! See `docs/ARCHITECTURE.md` §13.2 and §15.

pub mod colors;
#[cfg(windows)]
pub mod credentials;
pub mod dbusmenu;
pub mod font;
pub mod icon;
pub mod launcher;
#[cfg(windows)]
pub mod mail_handler;
pub mod mimeapps;
#[cfg(windows)]
pub mod toasts;
#[cfg(not(windows))]
pub mod tray;
#[cfg(windows)]
#[path = "tray_windows.rs"]
pub mod tray;
