// SPDX-License-Identifier: GPL-3.0-or-later

//! Drafts, as in Gmail: closing a message saves it in its account's
//! Drafts folder, and only Discard throws it away, with Undo. A draft
//! opened from Drafts is written on in the compose window, and sending it
//! deletes it. Every save of one message carries the same `Message-ID`,
//! so the daemon replaces the copy saved before, here and on the server.

use std::sync::atomic::{AtomicU64, Ordering};

use gpui::{Context, Window};
use katna_core::AccountId;
use katna_i18n::tr;
use katna_store::MessageId;
use mail_parser::MessageParser;

use super::{Attachment, Draft, Kind, MailWindow, Unsent, body_parts, scheduled};
use crate::daemon::{self, Command};
use crate::outgoing::{self, Mailbox, Outgoing};
use katna_ui::rich::html;

/// A `Message-ID` for a new message's drafts, without angle brackets.
pub(super) fn new_message_id() -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos() as u64);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!(
        "katna-draft.{now:x}.{:x}.{n}@katna.invalid",
        std::process::id()
    )
}

/// The addresses of `text` that are addresses; a draft keeps what it can.
fn addresses(text: &str) -> Vec<Mailbox> {
    outgoing::parse_addresses(text).unwrap_or_else(|_| {
        text.split([',', ';'])
            .filter_map(|entry| outgoing::parse_addresses(entry).ok())
            .flatten()
            .collect()
    })
}

/// `unsent` as the draft saved in `account`.
fn draft_raw(unsent: &Unsent, from: Mailbox, message_id: &str) -> Vec<u8> {
    let domain = from
        .email
        .rsplit_once('@')
        .map_or("katna.local", |(_, d)| d)
        .to_owned();
    let draft = &unsent.draft;
    let (mut html_body, inline) = body_parts(&draft.body, unsent.plain, &domain);
    // Files already in Drive stay in the draft as their links.
    let mut body = html::to_plain(&draft.body);
    body.push_str(&super::drive::links_text(&unsent.drive));
    if let Some(html) = &mut html_body {
        html.push_str(&super::drive::links_html(&unsent.drive));
    }
    outgoing::build(&Outgoing {
        from: Some(from),
        to: addresses(&draft.to),
        cc: addresses(&draft.cc),
        bcc: addresses(&draft.bcc),
        subject: draft.subject.clone(),
        body,
        in_reply_to: unsent.thread.in_reply_to.clone(),
        references: unsent.thread.references.clone(),
        html: html_body,
        inline,
        attachments: unsent.attachments.iter().map(Attachment::part).collect(),
        date: None,
        message_id: Some(message_id.to_owned()),
    })
}

impl MailWindow {
    /// What the open message holds, to save or open again.
    fn snapshot(&self, cx: &gpui::App) -> Option<Unsent> {
        let compose = self.compose.as_ref().filter(|c| !c.closing)?;
        let from = compose
            .from
            .or_else(|| self.compose_account(compose.kind).map(|a| a.id));
        Some(Unsent {
            draft: compose.fields(cx),
            thread: compose.thread.clone(),
            sealing: compose.sealing,
            signature: compose.signature,
            attachments: compose.attachments.clone(),
            drive: compose.drive.clone(),
            plain: compose.plain(cx),
            from,
            answering: compose.answering,
            unarchive: None,
            message_id: Some(compose.message_id.clone()),
            saved: compose.saved,
        })
    }

    /// The close button: closes the message, saved as a draft when
    /// anything was written.
    pub(in crate::window) fn close_compose_saving(&mut self, cx: &mut Context<Self>) {
        let touched = self
            .compose
            .as_ref()
            .is_some_and(|c| !c.closing && c.touched(cx));
        let unsent = touched.then(|| self.snapshot(cx)).flatten();
        self.close_compose(cx);
        if let Some(unsent) = unsent {
            self.save_draft(unsent, cx);
        }
    }

    /// Discard: closes the message and deletes the draft it was saved as,
    /// with Undo to open it again.
    pub(in crate::window) fn discard_compose(&mut self, cx: &mut Context<Self>) {
        let touched = self
            .compose
            .as_ref()
            .is_some_and(|c| !c.closing && c.touched(cx));
        let Some(mut unsent) = self.snapshot(cx) else {
            return;
        };
        self.close_compose(cx);
        if let (Some(account), Some(message_id)) = (unsent.saved, unsent.message_id.clone()) {
            self.discard_saved(account, message_id, cx);
        }
        if touched || unsent.saved.is_some() {
            // Undo opens it written, not yet saved again.
            unsent.saved = None;
            self.writing.closed_draft = Some(unsent);
            self.show_snackbar(tr!("compose-discarded"), Some(Command::ReopenDraft), cx);
        }
    }

    /// Saves `unsent` in its account's Drafts folder. When that fails the
    /// snackbar says so, and its Undo opens the message again.
    fn save_draft(&mut self, mut unsent: Unsent, cx: &mut Context<Self>) {
        let account = unsent
            .from
            .and_then(|id| self.accounts.iter().find(|a| a.id == id));
        let Some(account) = account else {
            self.writing.closed_draft = Some(unsent);
            self.show_snackbar(
                tr!("compose-draft-failed", error = tr!("compose-no-account")),
                Some(Command::ReopenDraft),
                cx,
            );
            return;
        };
        let from = Mailbox {
            name: Some(account.display_name.trim().to_owned()).filter(|n| !n.is_empty()),
            email: account.address.clone(),
        };
        let id = account.id;
        let message_id = unsent.message_id.get_or_insert_with(new_message_id).clone();
        let raw = draft_raw(&unsent, from, &message_id);
        // Saved before in another account (From changed): it moves.
        let moved = unsent.saved.filter(|saved| *saved != id);
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn({
                    let message_id = message_id.clone();
                    async move {
                        let connection = match connection {
                            Some(connection) => connection,
                            None => daemon::connect().await?,
                        };
                        daemon::save_draft(&connection, id.0, &raw).await?;
                        if let Some(old) = moved {
                            daemon::discard_draft(&connection, old.0, &message_id).await?;
                        }
                        Ok::<_, String>(())
                    }
                })
                .await;
            this.update(cx, |this, cx| {
                match result {
                    Ok(()) => this.show_snackbar(tr!("compose-draft-saved"), None, cx),
                    Err(err) => {
                        tracing::warn!(%err, "saving a draft");
                        unsent.saved = unsent.saved.or(Some(id)).filter(|_| moved.is_none());
                        this.writing.closed_draft = Some(unsent);
                        this.show_snackbar(
                            tr!("compose-draft-failed", error = err),
                            Some(Command::ReopenDraft),
                            cx,
                        );
                    }
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// Deletes every saved copy of the draft `message_id` of `account`.
    fn discard_saved(&mut self, account: AccountId, message_id: String, cx: &mut Context<Self>) {
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::discard_draft(&connection, account.0, &message_id).await
                })
                .await;
            if let Err(err) = result {
                this.update(cx, |this, cx| this.show_snackbar(err, None, cx))
                    .ok();
            }
        })
        .detach();
    }

    /// Undo on "Draft discarded" or a draft that could not be saved: the
    /// message opens again, unless another is being written.
    pub(in crate::window) fn reopen_closed_draft(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self
            .compose
            .as_ref()
            .is_some_and(|c| !c.closing && c.touched(cx))
        {
            self.show_snackbar(tr!("compose-open-elsewhere"), None, cx);
            return;
        }
        if let Some(unsent) = self.writing.closed_draft.take() {
            self.reopen_message(unsent, Draft::default(), window, cx);
        }
    }

    /// Opens draft `id` from the Drafts folder in the compose window,
    /// downloading it first if its text is not here yet. A message being
    /// written is saved first.
    pub(in crate::window) fn open_draft(
        &mut self,
        id: MessageId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let open = self.draft_message_id(id);
        if self.compose.as_ref().is_some_and(|c| {
            !c.closing && c.saved.is_some() && open.as_ref() == Some(&c.message_id)
        }) {
            // Already open.
            return;
        }
        self.close_compose_saving(cx);
        let raw = self.mail.as_ref().ok().and_then(|mail| mail.raw(id));
        if let Some(raw) = raw {
            self.open_draft_raw(id, &raw, window, cx);
            return;
        }
        let done = self.download(id, cx);
        cx.spawn_in(window, async move |this, cx| {
            let _ = done.recv().await;
            this.update_in(cx, |this, window, cx| {
                match this.mail.as_ref().ok().and_then(|mail| mail.raw(id)) {
                    Some(raw) => this.open_draft_raw(id, &raw, window, cx),
                    None => this.show_snackbar(tr!("compose-draft-not-opened"), None, cx),
                }
            })
            .ok();
        })
        .detach();
    }

    /// The `Message-ID` of draft `id`, if its text is here.
    fn draft_message_id(&self, id: MessageId) -> Option<String> {
        let raw = self.mail.as_ref().ok()?.raw(id)?;
        Some(
            MessageParser::default()
                .parse(&raw)?
                .message_id()?
                .to_owned(),
        )
    }

    fn open_draft_raw(
        &mut self,
        id: MessageId,
        raw: &[u8],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(mut unsent) = scheduled::unsent_from_raw(raw) else {
            self.show_snackbar(tr!("compose-draft-not-opened"), None, cx);
            return;
        };
        let account = self
            .mail
            .as_ref()
            .ok()
            .and_then(|mail| mail.message_account(id));
        unsent.from = account;
        unsent.saved = account;
        // Drafts from other apps without one are saved with a new one; the
        // old copy then stays in Drafts.
        unsent.message_id = MessageParser::default()
            .parse(raw)
            .and_then(|m| m.message_id().map(str::to_owned));
        let start = unsent.draft.clone();
        self.reopen_message(unsent, start, window, cx);
        if let Some(compose) = &mut self.compose {
            compose.kind = if compose.thread.in_reply_to.is_some() {
                Kind::Reply
            } else {
                Kind::New
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drafts_keep_their_message_id_and_what_was_typed() {
        let message_id = new_message_id();
        assert_ne!(message_id, new_message_id());
        let unsent = Unsent {
            draft: Draft {
                to: "Kay <kay@example.com>, not an address".to_owned(),
                cc: String::new(),
                bcc: "boss@example.com".to_owned(),
                subject: "Plan".to_owned(),
                body: html::from_plain("Half written\n"),
            },
            thread: Default::default(),
            sealing: Default::default(),
            signature: None,
            attachments: Vec::new(),
            drive: Vec::new(),
            plain: true,
            from: None,
            answering: None,
            unarchive: None,
            message_id: None,
            saved: None,
        };
        let from = Mailbox {
            name: None,
            email: "me@example.com".to_owned(),
        };
        let raw = draft_raw(&unsent, from, &message_id);
        let parsed = MessageParser::default().parse(&raw).unwrap();
        assert_eq!(parsed.message_id(), Some(message_id.as_str()));
        let reopened = scheduled::unsent_from_raw(&raw).unwrap();
        assert_eq!(reopened.draft.to, "Kay <kay@example.com>");
        assert_eq!(reopened.draft.bcc, "boss@example.com");
        assert_eq!(reopened.draft.subject, "Plan");
        assert_eq!(html::to_plain(&reopened.draft.body).trim(), "Half written");
    }
}
