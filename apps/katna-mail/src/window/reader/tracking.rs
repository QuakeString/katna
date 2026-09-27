// SPDX-License-Identifier: GPL-3.0-or-later

//! Open and click tracking and read receipts in the reading view: above
//! a tracked message, a line per recipient saying whether they opened it
//! and followed a link; above a read receipt, who read what
//! (`docs/ARCHITECTURE.md` §16.1).

use gpui::{AnyElement, div, prelude::*, rgba};
use katna_i18n::tr;
use katna_store::RecipientActivity;
use katna_ui::px;

use super::Part;
use crate::format;
use crate::theme::Theme;
use crate::widgets::icon;
use crate::window::MailWindow;

impl MailWindow {
    /// The lines above a tracked message, a message a read receipt came
    /// back for, or a read receipt.
    pub(super) fn tracking_banner(&self, part: &Part, th: &Theme) -> Option<AnyElement> {
        let reader = self.reader.as_ref()?;
        let green = if th.dark { 0x81c995ff } else { 0x188038ff };
        let mut lines: Vec<(&'static str, u32, String)> = Vec::new();
        if let Some(Some(receipt)) = &part.receipt {
            let who = receipt.who();
            lines.push(if receipt.displayed {
                (
                    "read-receipt",
                    green,
                    tr!("tracking-receipt-displayed", who = who),
                )
            } else {
                (
                    "read-receipt",
                    th.text_faint,
                    tr!("tracking-receipt-other", who = who),
                )
            });
        }
        if let Some(activity) = &part.activity {
            for recipient in &activity.recipients {
                lines.push(self.recipient_line(recipient, th));
            }
        }
        for receipt in reader
            .receipts_for(part)
            .into_iter()
            .filter(|r| r.displayed)
        {
            lines.push((
                "read-receipt",
                green,
                tr!("tracking-receipt", who = receipt.who()),
            ));
        }
        if lines.is_empty() {
            return None;
        }
        Some(
            div()
                .mt(px(12.0))
                .px(px(12.0))
                .py(px(8.0))
                .flex()
                .flex_col()
                .gap(px(6.0))
                .rounded(px(8.0))
                .border_1()
                .border_color(rgba(th.divider))
                .text_size(px(13.0))
                .line_height(px(18.0))
                .children(lines.into_iter().map(|(name, color, text)| {
                    div()
                        .flex()
                        .flex_row()
                        .items_start()
                        .gap(px(8.0))
                        .child(icon(name, color, 18.0))
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .text_color(rgba(if color == th.text_faint {
                                    th.text_dim
                                } else {
                                    th.text
                                }))
                                .child(text),
                        )
                }))
                .into_any_element(),
        )
    }

    fn recipient_line(
        &self,
        recipient: &RecipientActivity,
        th: &Theme,
    ) -> (&'static str, u32, String) {
        let who = recipient
            .name
            .clone()
            .filter(|n| !n.trim().is_empty())
            .unwrap_or_else(|| recipient.email.clone());
        let now = jiff::Timestamp::now().as_second();
        let when = recipient
            .last
            .and_then(|at| format::local(at / 1000, &self.tz))
            .zip(format::local(now, &self.tz))
            .map(|(at, now)| format::list_date(at, now))
            .unwrap_or_default();
        if recipient.clicks > 0 {
            let text = tr!(
                "tracking-opened-clicked",
                who = who,
                count = recipient.clicks,
                when = when
            );
            ("eye", th.accent, text)
        } else if recipient.opens > 0 {
            let text = tr!(
                "tracking-opened",
                who = who,
                count = recipient.opens,
                when = when
            );
            ("eye", th.accent, text)
        } else if recipient.maybe_opens > 0 {
            (
                "eye",
                th.text_faint,
                tr!("tracking-maybe-opened", who = who),
            )
        } else {
            ("eye", th.text_faint, tr!("tracking-not-opened", who = who))
        }
    }
}
