// SPDX-License-Identifier: GPL-3.0-or-later

//! Rich text: the formatted document the compose window edits, and its
//! HTML and plain-text forms.

pub mod doc;
pub mod html;

pub use doc::{
    Align, Block, CharStyle, Doc, Font, Image, ImageSize, List, Para, ParaStyle, Path, Pos, Size,
    Table,
};
