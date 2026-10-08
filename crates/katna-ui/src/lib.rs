// SPDX-License-Identifier: GPL-3.0-or-later

//! Shared GPUI components for Katna apps. GPUI types stay in this crate,
//! `katna-chrome` and the GUI apps. See `docs/ARCHITECTURE.md` §13.

pub mod anchored;
pub mod direction;
pub mod frost;
pub mod glow;
pub mod icons;
pub mod motion;
pub mod native;
pub mod rich;
pub mod ripple;
pub mod scale;
pub mod schemes;
pub mod scrollbar;
pub mod text_area;
pub mod text_input;
pub mod tokens;
pub mod tooltip;
pub mod window_drag;

/// How strongly hint text in an empty field shows, against the field's
/// own text colour: faint, so it never reads as something typed.
pub const PLACEHOLDER_OPACITY: f32 = 0.42;

pub use anchored::anchored;
pub use direction::{Direction, directed};
pub use glow::Glow;
pub use icons::icon_svg;
pub use motion::Spring;
pub use rich::RichEditor;
pub use ripple::Ripple;
pub use scale::{px, unpx};
pub use scrollbar::ScrollBar;
pub use text_area::{TEXT_AREA_CONTEXT, TextArea};
pub use text_input::{InputEvent, InputGrammarMenu, TextInput};
pub use tooltip::{Tooltip, UiFont};
pub use window_drag::WindowDrag;
