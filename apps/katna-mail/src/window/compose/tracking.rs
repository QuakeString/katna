// SPDX-License-Identifier: GPL-3.0-or-later

//! Open and click tracking, read receipts and delivery receipts in the
//! compose window: three toggles by the recipients, all on for every new
//! message and reply (`docs/ARCHITECTURE.md` §16.1). Tracking needs a
//! Katna account, and mail goes out untracked without one. Tracked mail
//! goes to the daemon's `QueueTrackedSend`, which sends each recipient a
//! copy of their own. A read receipt is a `Disposition-Notification-To`
//! header (RFC 8098), which needs no server and which the recipient's app
//! may ask them about. A delivery receipt is asked of the mail server
//! (SMTP DSN, RFC 3461), which mails one back per recipient when it
//! delivers; not every server offers them (Gmail does not).

use gpui::{AnyElement, Context, prelude::*, rgba};
use katna_i18n::tr;
use katna_ui::px;

use crate::theme::{Theme, fade};
use crate::widgets::{icon_button_colored, tip};
use crate::window::MailWindow;
use crate::window::settings_page::Section;

/// `raw` asking for a read receipt to `address`.
pub(in crate::window) fn with_receipt(raw: Vec<u8>, address: &str) -> Vec<u8> {
    // Header order does not matter; the top keeps a signed body intact.
    let mut out = format!("Disposition-Notification-To: <{address}>\r\n").into_bytes();
    out.extend_from_slice(&raw);
    out
}

/// `raw` asking the daemon for delivery receipts
/// ([`katna_store::DELIVERY_RECEIPT_HEADER`], taken out before it is sent).
pub(in crate::window) fn with_delivery_receipt(raw: Vec<u8>) -> Vec<u8> {
    let mut out = format!("{}: yes\r\n", katna_store::DELIVERY_RECEIPT_HEADER).into_bytes();
    out.extend_from_slice(&raw);
    out
}

impl MailWindow {
    /// The Track, Read receipt and Delivery receipt toggles, after Encrypt
    /// and Sign.
    pub(super) fn render_tracking(&self, th: &Theme, cx: &mut Context<Self>) -> [AnyElement; 3] {
        let (sealing, plain) = self
            .compose
            .as_ref()
            .map(|c| (c.sealing, c.plain(cx)))
            .unwrap_or_default();
        // Tracking rewrites the HTML of each copy, which signed, encrypted
        // and plain-text mail don't allow.
        let trackable = !sealing.any() && !plain;
        // The server takes tracked mail only from a Katna account.
        let signed_in = self.katna_signed_in();
        let toggle = |id: &'static str, name: &'static str, on: bool, label: String| {
            icon_button_colored(id, name, 18.0, if on { th.accent } else { th.text_dim }, th)
                .size(px(28.0))
                .when(on, |d| d.bg(rgba(fade(th.accent, 0.12))))
                .tooltip(tip(label, th))
        };
        let track = sealing.track && trackable && signed_in;
        let offered = self.delivery_receipts_offered() != Some(false);
        let delivery = sealing.delivery && offered;
        [
            toggle(
                "compose-track",
                "eye",
                track,
                if !trackable {
                    tr!("compose-track-unavailable")
                } else if !signed_in {
                    tr!("compose-track-sign-in")
                } else if track {
                    tr!("compose-tracked")
                } else {
                    tr!("compose-track")
                },
            )
            .when(!trackable, |d| d.opacity(0.5))
            .on_click(cx.listener(move |this, _, window, cx| {
                if trackable && !signed_in {
                    this.open_settings_page(Section::Subscriptions, window, cx);
                    return;
                }
                if let Some(c) = &mut this.compose
                    && trackable
                {
                    c.sealing.track = !c.sealing.track;
                }
                cx.notify();
            }))
            .into_any_element(),
            toggle(
                "compose-receipt",
                "read-receipt",
                sealing.receipt,
                if sealing.receipt {
                    tr!("compose-receipt-on")
                } else {
                    tr!("compose-receipt")
                },
            )
            .on_click(cx.listener(|this, _, _, cx| {
                if let Some(c) = &mut this.compose {
                    c.sealing.receipt = !c.sealing.receipt;
                }
                cx.notify();
            }))
            .into_any_element(),
            toggle(
                "compose-delivery",
                "delivery-receipt",
                delivery,
                if !offered {
                    tr!("compose-delivery-unavailable")
                } else if delivery {
                    tr!("compose-delivery-on")
                } else {
                    tr!("compose-delivery")
                },
            )
            .when(!offered, |d| d.opacity(0.5))
            .on_click(cx.listener(move |this, _, _, cx| {
                if let Some(c) = &mut this.compose
                    && offered
                {
                    c.sealing.delivery = !c.sealing.delivery;
                }
                cx.notify();
            }))
            .into_any_element(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_receipt_is_asked_for_in_a_header() {
        let raw = b"From: a@example.org\r\nSubject: Hi\r\n\r\nHi\r\n".to_vec();
        let asked = with_receipt(raw, "a@example.org");
        let parsed = mail_parser::MessageParser::default().parse(&asked).unwrap();
        assert_eq!(
            parsed
                .header_raw("Disposition-Notification-To")
                .map(str::trim),
            Some("<a@example.org>")
        );
        assert_eq!(parsed.subject(), Some("Hi"));
    }
}
