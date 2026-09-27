// SPDX-License-Identifier: GPL-3.0-or-later

//! Languages (`docs/ARCHITECTURE.md` §13.10).
//!
//! The interface's text lives in Fluent files, one folder per binary and
//! one file per area (`i18n/<translation>/<binary>/<area>.ftl`; a small
//! binary may have a single `<binary>.ftl`), embedded in each binary by its
//! build script. A binary calls [`init`] once
//! with its files, then [`apply`] with the user's choice (and again when it
//! changes). Everything else asks for text with [`tr!`]:
//!
//! ```ignore
//! let label = tr!("compose");
//! let unread = tr!("unread-count", count = 12);
//! ```
//!
//! A message missing from the chosen language falls back to English, so a
//! partly translated language still shows everything.

mod catalog;
#[cfg(feature = "format")]
pub mod format;
mod languages;
mod pseudo;
mod system;

pub use catalog::{Args, Sources, apply, current, english, init, lookup, rtl};
pub use fluent_bundle::FluentValue;
pub use languages::{Language, Status, all, find, fold, picker};
pub use system::{Resolved, resolve, system_formats, system_language};

/// The environment variable that overrides the language setting, for
/// testing: a tag from `i18n/languages.toml`, or a pseudo-language
/// (`qps-ploc` accents and lengthens every string, `qps-plocm` also
/// mirrors the layout).
pub const OVERRIDE_VAR: &str = "KATNA_LANGUAGE";

/// The text of message `id` in the current language, with its variables:
///
/// ```ignore
/// tr!("compose")
/// tr!("unread-count", count = unread)
/// tr!("wrote", name = sender.as_str(), date = when)
/// ```
///
/// Each variable is anything [`FluentValue`] can be made from (strings
/// and numbers). A number picks the plural form and is written with the
/// language's digits.
#[macro_export]
macro_rules! tr {
    ($id:expr) => {
        $crate::lookup($id, None)
    };
    ($id:expr, $($name:ident = $value:expr),+ $(,)?) => {{
        let mut args = $crate::Args::new();
        $(args.set(stringify!($name), $crate::FluentValue::from($value));)+
        $crate::lookup($id, Some(&args))
    }};
}
