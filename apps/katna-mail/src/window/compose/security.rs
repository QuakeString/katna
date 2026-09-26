// SPDX-License-Identifier: GPL-3.0-or-later

//! Signing and encrypting in the compose window: two toggles by the
//! recipients,
//! and GnuPG run on the finished message before it goes to the outbox, so
//! the outbox and Sent hold only what was sent.

use gpui::{AnyElement, Context, prelude::*, px, rgba};
use katna_crypto::{Gnupg, Protect, Recipients, Security, Standard};

use crate::theme::{Theme, fade};
use crate::widgets::{icon_button_colored, tip};
use crate::window::MailWindow;

/// What the sender asked for.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(in crate::window) struct Sealing {
    pub sign: bool,
    pub encrypt: bool,
    /// S/MIME rather than OpenPGP.
    pub smime: bool,
}

impl Sealing {
    /// The default for an answer to (or forward of) a message: encrypted
    /// mail is answered encrypted and signed, in the same standard.
    pub fn answering(security: Option<&Security>) -> Self {
        match security {
            Some(security) if security.decrypted() => Self {
                sign: true,
                encrypt: true,
                smime: security.standard == Standard::Smime,
            },
            _ => Self::default(),
        }
    }

    fn any(&self) -> bool {
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

impl MailWindow {
    /// The Encrypt and Sign toggles, at the end of the recipients row.
    pub(super) fn render_sealing(&self, th: &Theme, cx: &mut Context<Self>) -> [AnyElement; 2] {
        let sealing = self.compose.as_ref().map(|c| c.sealing).unwrap_or_default();
        let toggle = |id: &'static str, name: &'static str, on: bool, label: &'static str| {
            icon_button_colored(id, name, 18.0, if on { th.accent } else { th.text_dim }, th)
                .size(px(28.0))
                .when(on, |d| d.bg(rgba(fade(th.accent, 0.12))))
                .tooltip(tip(label, th))
        };
        [
            toggle(
                "compose-encrypt",
                "lock",
                sealing.encrypt,
                if sealing.encrypt {
                    "Encrypted: only the recipients can read it"
                } else {
                    "Encrypt"
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
                    "Signed: recipients can check it is from you"
                } else {
                    "Sign"
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
                smime: true
            }
        );
        let signed_only = Security {
            decryption: None,
            ..security
        };
        assert_eq!(Sealing::answering(Some(&signed_only)), Sealing::default());
        assert_eq!(Sealing::answering(None), Sealing::default());
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
