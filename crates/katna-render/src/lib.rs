// SPDX-License-Identifier: GPL-3.0-or-later

//! HTML sanitizing and message rendering. See `docs/ARCHITECTURE.md` §12.
//!
//! So far only the plain-text view of plan task 3.5: headers, the text of
//! the message (HTML converted to text) and the attachment list. Sanitized
//! HTML, remote-content blocking and authentication banners come later.

mod plain;

pub use plain::{Address, Attachment, MAX_BODY_BYTES, MessageView, message_view};
