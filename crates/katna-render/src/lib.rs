// SPDX-License-Identifier: GPL-3.0-or-later

//! HTML sanitizing and message rendering. See `docs/ARCHITECTURE.md` §12.
//!
//! [`message_view`] gives the headers, the text of a message (HTML
//! converted to text, for replies and quoting) and its attachments.
//! [`message_document`] lays out its HTML body for the reading pane (see
//! [`html`]). [`sender_authenticated`] reads what the user's provider
//! found of the sender's DKIM and DMARC.

mod auth;
pub mod html;
mod plain;
pub mod print;
mod rich;

pub use auth::sender_authenticated;
pub use plain::{
    Address, Attachment, AttachmentFile, MAX_BODY_BYTES, MessageView, attachment_file, message_view,
};
pub use rich::message_document;
