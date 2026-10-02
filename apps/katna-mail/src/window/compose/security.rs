// SPDX-License-Identifier: GPL-3.0-or-later

//! Signing and encrypting in the compose window: two toggles by the
//! recipients,
//! and GnuPG run on the finished message before it goes to the outbox, so
//! the outbox and Sent hold only what was sent.

use gpui::{AnyElement, Context, div, prelude::*, rgba};
use katna_crypto::{Gnupg, Protect, Recipients, Security, Standard};
use katna_i18n::tr;
use katna_ui::px;

use crate::theme::{Theme, fade};
use crate::widgets::{icon_button_colored, tip};
use crate::window::MailWindow;

/// What the sender asked for: signing and encryption, and (kept here so
/// they travel with the message through Undo and drafts) open and click
/// tracking and a read receipt (`compose/tracking.rs`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(in crate::window) struct Sealing {
    pub sign: bool,
    pub encrypt: bool,
    /// S/MIME rather than OpenPGP.
    pub smime: bool,
    /// Open and click tracking, through Katna Server.
    pub track: bool,
    /// A `Disposition-Notification-To` header (RFC 8098).
    pub receipt: bool,
    /// Delivery receipts from the mail server (SMTP DSN, RFC 3461).
    pub delivery: bool,
}

impl Sealing {
    /// The default for a new message: tracked, with read and delivery
    /// receipts asked for. Tracking only happens when signed in to a Katna
    /// account and the message can be tracked; delivery receipts only where
    /// the mail server sends them (`compose/tracking.rs`).
    pub fn new_message() -> Self {
        Self {
            track: true,
            receipt: true,
            delivery: true,
            ..Self::default()
        }
    }

    /// The default for an answer to (or forward of) a message: encrypted
    /// mail is answered encrypted and signed, in the same standard.
    pub fn answering(security: Option<&Security>) -> Self {
        match security {
            Some(security) if security.decrypted() => Self {
                sign: true,
                encrypt: true,
                smime: security.standard == Standard::Smime,
                ..Self::new_message()
            },
            _ => Self::new_message(),
        }
    }

    pub(super) fn any(&self) -> bool {
        self.sign || self.encrypt
    }
}

/// Signs and/or encrypts `raw` as `sealing` asks, from `sender` to the
/// given addresses. Blocks on GnuPG (and pinentry): run it off the UI
/// thread.
pub(in crate::window) fn seal(
    raw: Vec<u8>,
    sealing: Sealing,
    sender: String,
    visible: Vec<String>,
    hidden: Vec<String>,
) -> Result<Vec<u8>, String> {
    if !sealing.any() {
        return Ok(raw);
    }
    let gnupg = Gnupg::new();
    let preferred = if sealing.smime {
        Standard::Smime
    } else {
        Standard::OpenPgp
    };
    let standard = katna_crypto::sending_standard(&gnupg, &sender, preferred);
    let how = Protect {
        standard,
        sign: sealing.sign,
        encrypt: sealing.encrypt,
    };
    let recipients = Recipients {
        sender,
        visible,
        hidden,
    };
    katna_crypto::protect(&raw, how, &recipients, &gnupg).map_err(|err| err.to_string())
}

/// A sealed message from the outbox made readable again, with the sealing
/// it went out with, so a cancelled scheduled send reopens as written.
/// The sender is always among the recipients of an encrypted message.
/// `None` when it cannot be decrypted. Blocks on GnuPG like [`seal`].
pub(in crate::window) fn unseal(raw: Vec<u8>) -> Option<(Vec<u8>, Sealing)> {
    if katna_crypto::protection(&raw).is_none() {
        return Some((raw, Sealing::default()));
    }
    let opened = katna_crypto::open(&raw, &Gnupg::new())?;
    let security = &opened.security;
    if security.encrypted() && !security.decrypted() {
        return None;
    }
    let sealing = Sealing {
        sign: !security.signatures.is_empty(),
        encrypt: security.encrypted(),
        smime: security.standard == Standard::Smime,
        ..Sealing::default()
    };
    Some((opened.raw, sealing))
}

impl MailWindow {
    /// Encrypt and Sign, a faint line, then Track, Read receipt and
    /// Delivery receipt, together on one soft tray. A toggle that is on
    /// takes the accent; nothing else is coloured.
    pub(super) fn render_security_group(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        div()
            .flex_none()
            .p(px(2.0))
            .flex()
            .flex_row()
            .items_center()
            .rounded_full()
            .bg(rgba(th.tray()))
            .children(self.render_sealing(th, cx))
            .child(
                div()
                    .flex_none()
                    .mx(px(2.0))
                    .w(px(1.0))
                    .h(px(14.0))
                    .bg(rgba(fade(th.divider, super::FAINT_LINE))),
            )
            .children(self.render_tracking(th, cx))
            .into_any_element()
    }

    /// The Encrypt and Sign toggles, in the security group.
    pub(super) fn render_sealing(&self, th: &Theme, cx: &mut Context<Self>) -> [AnyElement; 2] {
        let sealing = self.compose.as_ref().map(|c| c.sealing).unwrap_or_default();
        let toggle = |id: &'static str, name: &'static str, on: bool, label: String| {
            icon_button_colored(id, name, 18.0, if on { th.accent } else { th.text_dim }, th)
                .size(px(28.0))
                .tooltip(tip(label, th))
        };
        [
            toggle(
                "compose-encrypt",
                "lock",
                sealing.encrypt,
                if sealing.encrypt {
                    tr!("compose-encrypted")
                } else {
                    tr!("compose-encrypt")
                },
            )
            .on_click(cx.listener(|this, _, _, cx| {
                if let Some(c) = &mut this.compose {
                    c.sealing.encrypt = !c.sealing.encrypt;
                    // Encrypted mail is signed too, unless the user turns
                    // that off afterwards.
                    c.sealing.sign |= c.sealing.encrypt;
                }
                cx.notify();
            }))
            .into_any_element(),
            toggle(
                "compose-sign",
                "shield-check",
                sealing.sign,
                if sealing.sign {
                    tr!("compose-signed")
                } else {
                    tr!("compose-sign")
                },
            )
            .on_click(cx.listener(|this, _, _, cx| {
                if let Some(c) = &mut this.compose {
                    c.sealing.sign = !c.sealing.sign;
                }
                cx.notify();
            }))
            .into_any_element(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use katna_crypto::Decryption;

    use super::*;

    #[test]
    fn answers_to_encrypted_mail_are_encrypted() {
        let security = Security {
            standard: Standard::Smime,
            whole: true,
            decryption: Some(Decryption::Decrypted),
            signatures: Vec::new(),
        };
        assert_eq!(
            Sealing::answering(Some(&security)),
            Sealing {
                sign: true,
                encrypt: true,
                smime: true,
                ..Sealing::new_message()
            }
        );
        let signed_only = Security {
            decryption: None,
            ..security
        };
        assert_eq!(
            Sealing::answering(Some(&signed_only)),
            Sealing::new_message()
        );
        assert_eq!(Sealing::answering(None), Sealing::new_message());
    }

    #[test]
    fn nothing_asked_leaves_the_message_alone() {
        let raw = b"From: a@example.org\r\n\r\nHi\r\n".to_vec();
        let sealed = seal(
            raw.clone(),
            Sealing::default(),
            "a@example.org".into(),
            vec![],
            vec![],
        );
        assert_eq!(sealed, Ok(raw));
    }
}
