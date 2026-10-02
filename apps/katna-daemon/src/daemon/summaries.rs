// SPDX-License-Identifier: GPL-3.0-or-later

//! Conversations summed up by AI (`docs/ARCHITECTURE.md` §16.5): asked of
//! the service the settings name, then kept in `mail.db` so opening the
//! conversation again shows the summary without asking.

use katna_ai::summary::SummarizeRequest;
use katna_ai::wire::Plan;
use katna_store::{MessageId, SummaryKind};

use super::{Daemon, settings, unix_now};
use crate::ai::AiError;

impl Daemon {
    /// Sums up `request`'s conversation, whose newest mail sent is
    /// `newest`, and keeps the summary. Returns it as JSON
    /// (`katna_ai::summary::Summary`) with where the account stands.
    pub async fn ai_summarize(
        &self,
        newest: MessageId,
        request: &SummarizeRequest,
    ) -> Result<(String, Plan), AiError> {
        let settings = settings(&self.paths).ai;
        let (mut summary, plan) = crate::ai::summarize(&settings, &self.secrets, request)
            .await
            .inspect_err(|err| tracing::info!(%err, "summing up a conversation"))?;
        if request.catch_up {
            summary.new = request.mails.iter().filter(|m| m.new).count() as u32;
        }
        let body =
            serde_json::to_string(&summary).map_err(|err| AiError::Failed(err.to_string()))?;
        let kind = if request.catch_up && request.mails.iter().any(|m| m.new) {
            SummaryKind::New
        } else {
            SummaryKind::All
        };
        let service = if plan.kind == crate::ai::OWN {
            "own"
        } else {
            "katna"
        };
        let kept = (|| -> Result<(), katna_store::Error> {
            let mut store = self.store();
            let Some(stored) = store.messages_by_id(&[newest])?.into_iter().next() else {
                return Ok(());
            };
            let siblings = match stored.thread_id {
                Some(thread) => store.thread_messages(thread)?,
                None => vec![newest],
            };
            let mut batch = store.mail_batch()?;
            batch.save_summary(
                newest,
                &siblings,
                kind,
                request.mails.len() as u32,
                &body,
                service,
                unix_now(),
            )?;
            batch.commit()
        })();
        if let Err(err) = kept {
            // The summary still shows; it is asked again next time.
            tracing::warn!(%err, "keeping a summary");
        }
        Ok((body, plan))
    }
}
