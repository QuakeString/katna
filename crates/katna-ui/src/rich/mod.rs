// SPDX-License-Identifier: GPL-3.0-or-later

//! Rich text: the formatted document the compose window edits
//! ([`doc`]), its HTML and plain-text forms ([`html`]), and the editor
//! ([`RichEditor`]).

pub mod doc;
mod editor;
pub mod html;
mod layout;

pub use doc::{
    Align, Block, CharStyle, Doc, Font, Image, ImageSize, List, Para, ParaStyle, Path, Pos, Size,
    Table,
};
pub(crate) use editor::{GRAMMAR_WAIT, HINT_WAIT};
pub use editor::{
    GrammarCheck, GrammarFix, GrammarIssue, Palette, RICH_TEXT_CONTEXT, RichEditor, RichEvent,
    SpellCheck, Suggest, TableEdit, bind_keys, image_mime, insert_signature_doc,
};
