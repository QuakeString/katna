// SPDX-License-Identifier: GPL-3.0-or-later

//! Shared GPUI components for Katna apps. GPUI types stay in this crate,
//! `katna-chrome` and the GUI apps. See `docs/ARCHITECTURE.md` §13.

pub mod frost;
pub mod motion;
pub mod native;
pub mod rich;
pub mod ripple;
pub mod scale;
pub mod scrollbar;
pub mod text_area;
pub mod text_input;
pub mod tooltip;

pub use motion::Spring;
pub use rich::RichEditor;
pub use ripple::Ripple;
pub use scale::{px, unpx};
pub use scrollbar::ScrollBar;
pub use text_area::{TEXT_AREA_CONTEXT, TextArea};
pub use text_input::{InputEvent, InputGrammarMenu, TextInput};
pub use tooltip::{Tooltip, UiFont};
