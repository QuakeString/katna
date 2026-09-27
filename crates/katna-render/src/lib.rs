// SPDX-License-Identifier: GPL-3.0-or-later

//! HTML sanitizing and message rendering. See `docs/ARCHITECTURE.md` §12.
//!
//! [`message_view`] gives the headers, the text of a message (HTML
//! converted to text, for replies and quoting) and its attachments.
//! [`message_document`] lays out its HTML body for the reading pane (see
//! [`html`]). Authentication banners come later.

pub mod html;
mod plain;
pub mod print;
mod rich;

pub use plain::{
    Address, Attachment, AttachmentFile, MAX_BODY_BYTES, MessageView, attachment_file, message_view,
};
pub use rich::message_document;
