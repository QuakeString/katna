// SPDX-License-Identifier: GPL-3.0-or-later

//! Read receipts and delivery reports for the user's own mail are marked
//! read as they arrive: Katna Mail shows them as ticks on the mail they
//! answer, not as messages, so they must not count as unread or notify
//! (`docs/ARCHITECTURE.md` §16.1). They stay in the mailbox. Bounces are
//! left alone: a failed delivery is news.

use katna_store::{MessageFlags, MessageId, Store};

use crate::ops;

/// Marks read each receipt in `found` that answers a message in a folder
/// here: a receipt with the `Message-ID`s it may answer (the report's, or
/// its `In-Reply-To` and `References`).
pub(crate) fn quiet(store: &mut Store, found: &[(MessageId, Vec<String>)]) {
    let ids: Vec<MessageId> = found
        .iter()
        .filter(|(_, answers)| {
            answers
                .iter()
                .any(|id| store.message_with_header(id).ok().flatten().is_some())
        })
        .map(|(id, _)| *id)
        .collect();
    if ids.is_empty() {
        return;
    }
    if let Err(err) = ops::set_flags(store, &ids, MessageFlags::SEEN, MessageFlags::empty()) {
        tracing::warn!("marking receipts read: {err}");
    }
}
