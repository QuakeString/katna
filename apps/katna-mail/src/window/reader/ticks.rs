// SPDX-License-Identifier: GPL-3.0-or-later

//! Ticks beside the recipients of the user's own mail in the reading view:
//! one grey tick once a delivery receipt says their mail server took it,
//! or, where the mail server sends no delivery receipts (Gmail), once it
//! went out half an hour ago and no bounce came back; two accent ticks
//! once they read it, by a read receipt or an open seen by open tracking;
//! a warning when it bounced (`docs/ARCHITECTURE.md` §16.1).

use std::collections::HashMap;

use gpui::{AnyElement, div, prelude::*};
use katna_i18n::tr;
use katna_ui::px;

use crate::format;
use crate::theme::Theme;
use crate::widgets::{icon, tip};
use crate::window::MailWindow;

/// How a recipient was seen to read a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ReadBy {
    Receipt,
    Tracking,
}

/// What is known about one recipient of the user's message.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct Tick {
    /// When it went out from here (Unix seconds).
    pub sent: Option<i64>,
    /// Unix seconds.
    pub delivered: Option<i64>,
    /// When it bounced (Unix seconds).
    pub failed: Option<i64>,
    /// Unix seconds.
    pub read: Option<(i64, ReadBy)>,
}

/// How long after sending a message with no bounce counts as delivered.
const NO_BOUNCE: i64 = 30 * 60;

/// The ticks for a message, by recipient address (lower case), from its
/// stored receipts, the read receipts in the conversation (with when they
/// came) and what open tracking saw. A read receipt wins over tracking.
pub(super) fn ticks(
    stored: &[katna_store::Receipt],
    read_receipts: &[(&crate::receipts::Receipt, Option<i64>)],
    activity: Option<&katna_store::MessageActivity>,
) -> HashMap<String, Tick> {
    let mut ticks: HashMap<String, Tick> = HashMap::new();
    for receipt in stored {
        let tick = ticks.entry(receipt.recipient.clone()).or_default();
        tick.sent = receipt.sent_at;
        tick.delivered = receipt.delivered_at;
        tick.failed = receipt.failed_at;
        tick.read = receipt.read_at.map(|at| (at, ReadBy::Receipt));
    }
    for (receipt, at) in read_receipts.iter().filter(|(r, _)| r.displayed) {
        let tick = ticks.entry(receipt.by.to_lowercase()).or_default();
        if tick.read.is_none() {
            tick.read = at.map(|at| (at, ReadBy::Receipt));
        }
    }
    for recipient in activity.iter().flat_map(|a| &a.recipients) {
        if !recipient.opened() {
            continue;
        }
        let tick = ticks.entry(recipient.email.to_lowercase()).or_default();
        if tick.read.is_none() {
            tick.read = recipient.last.map(|ms| (ms / 1000, ReadBy::Tracking));
        }
    }
    ticks.retain(|_, t| *t != Tick::default());
    ticks
}

impl MailWindow {
    /// The tick after a recipient's name, if any: `id` tells it apart.
    pub(super) fn render_tick(
        &self,
        tick: Option<&Tick>,
        id: (&'static str, usize),
        th: &Theme,
    ) -> Option<AnyElement> {
        let tick = tick?;
        let now = jiff::Timestamp::now().as_second();
        let when = |at: i64| {
            format::local(at, &self.tz)
                .zip(format::local(now, &self.tz))
                .map(|(at, now)| format::list_date(at, now))
                .unwrap_or_default()
        };
        let (name, color, text) = if let Some((at, by)) = tick.read {
            let id = match by {
                ReadBy::Receipt => tr!("reader-tick-read", when = when(at)),
                ReadBy::Tracking => tr!("reader-tick-opened", when = when(at)),
            };
            ("done-all", th.accent, id)
        } else if let Some(at) = tick.failed {
            (
                "warning",
                th.error,
                tr!("reader-tick-bounced", when = when(at)),
            )
        } else if let Some(at) = tick.delivered {
            (
                "check",
                th.text_faint,
                tr!("reader-tick-delivered", when = when(at)),
            )
        } else {
            let at = tick.sent.filter(|&at| now - at >= NO_BOUNCE)?;
            (
                "check",
                th.text_faint,
                tr!("reader-tick-no-bounce", when = when(at)),
            )
        };
        Some(
            div()
                .id(id)
                .flex_none()
                .ml(px(2.0))
                .child(icon(name, color, 16.0))
                .tooltip(tip(text, th))
                .into_any_element(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_read_receipt_wins_over_tracking() {
        let stored = [katna_store::Receipt {
            recipient: "bea@x.org".into(),
            delivered_at: Some(10),
            ..Default::default()
        }];
        let activity = katna_store::MessageActivity {
            recipients: vec![
                katna_store::RecipientActivity {
                    email: "Bea@x.org".into(),
                    name: None,
                    opens: 2,
                    maybe_opens: 0,
                    clicks: 0,
                    last: Some(30_000),
                },
                katna_store::RecipientActivity {
                    email: "carl@x.org".into(),
                    name: None,
                    opens: 0,
                    maybe_opens: 1,
                    clicks: 0,
                    last: None,
                },
            ],
            subject: String::new(),
            sent_at: None,
            links: 0,
        };
        let found = ticks(&stored, &[], Some(&activity));
        assert_eq!(
            found["bea@x.org"],
            Tick {
                delivered: Some(10),
                read: Some((30, ReadBy::Tracking)),
                ..Tick::default()
            }
        );
        assert!(
            !found.contains_key("carl@x.org"),
            "maybe opened is not read"
        );

        let receipt = crate::receipts::Receipt {
            by: "bea@x.org".into(),
            name: None,
            original: None,
            displayed: true,
        };
        let found = ticks(&stored, &[(&receipt, Some(99))], Some(&activity));
        assert_eq!(found["bea@x.org"].read, Some((99, ReadBy::Receipt)));
    }
}
