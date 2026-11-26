// SPDX-License-Identifier: GPL-3.0-or-later

//! Unsubscribe in one click (`docs/ARCHITECTURE.md` §9.5): from an open
//! mail's header or the right-click menu of one or more mail lines. The
//! daemon does it in the background the sender's own way (a one-click
//! POST, else the list's unsubscribe mail); a short note says how it
//! went. A list that only offers a web page gets an Open page button:
//! Katna never clicks through pages itself. Nothing goes through Katna's
//! servers.

use gpui::{AnyElement, Context, IntoElement as _, StatefulInteractiveElement as _, Styled as _};
use katna_dbus::unsubscribed;
use katna_i18n::tr;
use katna_store::MessageId;

use super::{EntryKey, MailWindow};
use crate::daemon::{self, Command};
use crate::data::Unsubscribable;
use crate::theme::Theme;
use crate::widgets::line_chip;

/// Where unsubscribing from a sender stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Unsub {
    Working,
    Done,
}

/// At most this many messages are looked at for one right-click.
const MAX_MESSAGES: usize = 200;

impl MailWindow {
    /// The Unsubscribe chip of an open message from `found`'s sender, or
    /// its Unsubscribed mark once done.
    pub(super) fn unsubscribe_chip(
        &self,
        ix: usize,
        found: &Unsubscribable,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if !found.ways.any() {
            return None;
        }
        let state = self.unsubscribed.get(&found.address).copied();
        let (name, label, color) = match state {
            Some(Unsub::Done) => ("check", tr!("unsubscribe-done-chip"), th.text_faint),
            Some(Unsub::Working) => ("unsubscribe", tr!("unsubscribe-working"), th.text_faint),
            None => ("unsubscribe", tr!("unsubscribe-chip"), th.text_dim),
        };
        let chip = line_chip(
            ("part-unsubscribe", ix),
            ("part-unsubscribe-glow", ix),
            name,
            label,
            color,
            th,
        )
        .flex_shrink_0()
        .self_center();
        Some(match state {
            Some(_) => chip.into_any_element(),
            None => {
                let found = found.clone();
                chip.cursor_pointer()
                    .on_click(cx.listener(move |this, _, _, cx| {
                        cx.stop_propagation();
                        this.unsubscribe_from(vec![found.clone()], cx);
                    }))
                    .into_any_element()
            }
        })
    }

    /// Unsubscribes from the senders of mail lines `keys` (Unsubscribe on
    /// the right-click menu): from each sender's newest message that says
    /// how.
    pub(super) fn unsubscribe_lines(&mut self, keys: Vec<EntryKey>, cx: &mut Context<Self>) {
        let Ok(mail) = &self.mail else {
            return;
        };
        let mut ids: Vec<MessageId> = keys
            .iter()
            .flat_map(|key| mail.entry_messages(*key))
            .collect();
        // Newest first, so each sender's latest way is the one used.
        ids.sort_by_key(|id| std::cmp::Reverse(id.0));
        ids.truncate(MAX_MESSAGES);
        let mut found: Vec<Unsubscribable> = Vec::new();
        let mut senders = 0;
        let mut seen = std::collections::HashSet::new();
        for id in ids {
            let Some(this) = mail.unsubscribe_ways(id) else {
                continue;
            };
            if seen.insert(this.address.clone()) {
                senders += 1;
            }
            if this.ways.any() && !found.iter().any(|f| f.address == this.address) {
                found.push(this);
            }
        }
        if found.is_empty() {
            let text = if senders == 0 {
                tr!("unsubscribe-nothing")
            } else {
                tr!("unsubscribe-no-way", count = senders)
            };
            self.show_snackbar(text, None, cx);
            return;
        }
        self.unsubscribe_from(found, cx);
    }

    /// Unsubscribes from the senders of `found` in the background, then
    /// says how it went.
    pub(super) fn unsubscribe_from(&mut self, found: Vec<Unsubscribable>, cx: &mut Context<Self>) {
        let found: Vec<Unsubscribable> = found
            .into_iter()
            .filter(|f| self.unsubscribed.get(&f.address) != Some(&Unsub::Done))
            .collect();
        if found.is_empty() {
            return;
        }
        let quiet: Vec<i64> = found
            .iter()
            .filter(|f| f.ways.quiet())
            .map(|f| f.id.0)
            .collect();
        for f in found.iter().filter(|f| f.ways.quiet()) {
            self.unsubscribed.insert(f.address.clone(), Unsub::Working);
        }
        cx.notify();
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = if quiet.is_empty() {
                Ok(Vec::new())
            } else {
                cx.background_executor()
                    .spawn(async move {
                        let connection = match connection {
                            Some(connection) => connection,
                            None => daemon::connect().await?,
                        };
                        daemon::unsubscribe(&connection, &quiet).await
                    })
                    .await
            };
            this.update(cx, |this, cx| this.unsubscribed_from(&found, result, cx))
                .ok();
        })
        .detach();
    }

    /// Says how unsubscribing from `found` went.
    fn unsubscribed_from(
        &mut self,
        found: &[Unsubscribable],
        result: Result<Vec<(i64, String)>, String>,
        cx: &mut Context<Self>,
    ) {
        let outcomes = match result {
            Ok(outcomes) => outcomes,
            Err(err) => {
                tracing::warn!("unsubscribing: {err}");
                Vec::new()
            }
        };
        let outcome = |f: &Unsubscribable| {
            outcomes
                .iter()
                .find(|(id, _)| *id == f.id.0)
                .map(|(_, o)| o.as_str())
                .unwrap_or(unsubscribed::NONE)
        };
        let mut done: Vec<&Unsubscribable> = Vec::new();
        let mut pages: Vec<&Unsubscribable> = Vec::new();
        let mut failed: Vec<&Unsubscribable> = Vec::new();
        for f in found {
            match outcome(f) {
                unsubscribed::DONE | unsubscribed::SENT => {
                    self.unsubscribed.insert(f.address.clone(), Unsub::Done);
                    done.push(f);
                }
                o => {
                    self.unsubscribed.remove(&f.address);
                    if o == unsubscribed::FAILED || f.ways.quiet() {
                        failed.push(f);
                    } else {
                        pages.push(f);
                    }
                }
            }
        }
        let left: Vec<&Unsubscribable> = failed.iter().chain(&pages).copied().collect();
        let links: Vec<String> = left.iter().filter_map(|f| f.ways.page.clone()).collect();
        let text = match (done.as_slice(), left.as_slice()) {
            ([one], []) => tr!("unsubscribe-done-one", name = one.name.as_str()),
            (all, []) => tr!("unsubscribe-done-many", count = all.len()),
            ([], [one]) if failed.is_empty() => {
                tr!("unsubscribe-needs-page", name = one.name.as_str())
            }
            ([], [one]) => tr!("unsubscribe-failed", name = one.name.as_str()),
            ([], rest) => tr!("unsubscribe-none-many", count = rest.len()),
            (all, rest) => tr!("unsubscribe-some", count = all.len(), left = rest.len()),
        };
        if links.is_empty() {
            self.show_snackbar(text, None, cx);
        } else {
            let label = if links.len() == 1 {
                tr!("unsubscribe-open-page")
            } else {
                tr!("unsubscribe-open-pages")
            };
            self.show_snackbar_action(text, label, Command::OpenLinks(links), cx);
        }
        cx.notify();
    }
}
