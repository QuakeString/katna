// SPDX-License-Identifier: GPL-3.0-or-later

//! Read receipts and delivery reports for the user's own mail are marked
//! read as they arrive: Katna Mail shows them as ticks on the mail they
//! answer, not as messages, so they must not count as unread or notify
//! (`docs/ARCHITECTURE.md` §16.1). They stay in the mailbox, and the store
//! marks them so the lists leave them out. Bounces are left alone: a
//! failed delivery is news.

use katna_store::{MessageFlags, MessageId, Store};

use crate::ops;

/// Marks read, and as receipt mail, each receipt in `found` that answers a
/// message in a folder here: a receipt with the `Message-ID`s it may
/// answer (the report's, or its `In-Reply-To` and `References`).
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
    let marked = store.mail_batch().and_then(|mut batch| {
        for &id in &ids {
            batch.mark_receipt_mail(id)?;
        }
        batch.commit()
    });
    if let Err(err) = marked {
        tracing::warn!("marking receipt mail: {err}");
    }
    if let Err(err) = ops::set_flags(store, &ids, MessageFlags::SEEN, MessageFlags::empty()) {
        tracing::warn!("marking receipts read: {err}");
    }
}

/// Finds the receipts among downloaded mail stored before receipts were
/// marked, and quiets them as [`quiet`] does. Returns how many it found.
pub fn backfill(store: &mut Store) -> katna_store::Result<usize> {
    let mut found = Vec::new();
    for (id, hash) in store.possible_receipt_mail()? {
        let Some(raw) = store.blobs().get(&hash)? else {
            continue;
        };
        let Some(report) = katna_import::report::parse(&raw) else {
            continue;
        };
        if !report
            .outcomes
            .iter()
            .any(|(_, o)| *o == katna_import::report::Outcome::Failed)
        {
            found.push((id, vec![report.original]));
        }
    }
    quiet(store, &found);
    Ok(found.len())
}
