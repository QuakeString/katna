// SPDX-License-Identifier: GPL-3.0-or-later

//! Encrypted and signed mail in the reading view: GnuPG opens the message
//! off the UI thread (it may wait for a passphrase in pinentry), and a
//! banner above the body says what protected it and whether that held.
//! Decrypted text stays in memory only; it is never stored or indexed.

use gpui::{AnyElement, Context, FontWeight, div, prelude::*, px, rgba};
use katna_crypto::{
    Decryption, Failure, Gnupg, Opened, Protection, Security, Signature, SignatureState, Standard,
    Validity,
};
use katna_render::MessageView;
use katna_store::MessageId;

use super::{Body, Part, shown};
use crate::theme::{Theme, fade};
use crate::widgets::icon;
use crate::window::MailWindow;

/// What is known about an encrypted or signed message.
pub(super) enum Secured {
    /// GnuPG is working on it.
    Opening(Protection),
    Opened(Security),
}

/// A protected message before GnuPG has looked at it: its headers, and no
/// body yet (the body is ciphertext, or not checked).
pub(super) fn sealed(raw: Vec<u8>, protection: Protection) -> Body {
    Body {
        view: Some(headers_only(&raw)),
        blocks: Vec::new(),
        cut: false,
        doc: None,
        remote: Vec::new(),
        security: Some(Secured::Opening(protection)),
        sealed: Some(raw),
        opened: None,
    }
}

fn headers_only(raw: &[u8]) -> MessageView {
    MessageView {
        body: String::new(),
        truncated: false,
        from_html: false,
        attachments: Vec::new(),
        ..katna_render::message_view(raw)
    }
}

/// The body to show once GnuPG is done with `raw`.
fn opened_body(raw: &[u8], opened: Option<Opened>) -> Body {
    match opened {
        Some(Opened { security, .. })
            if matches!(security.decryption, Some(Decryption::Failed(_))) =>
        {
            Body {
                view: Some(headers_only(raw)),
                blocks: Vec::new(),
                cut: false,
                doc: None,
                remote: Vec::new(),
                security: Some(Secured::Opened(security)),
                sealed: None,
                opened: None,
            }
        }
        Some(Opened { raw, security }) => {
            let body = shown(&raw, Some(Secured::Opened(security)));
            Body {
                opened: Some(std::sync::Arc::new(raw)),
                ..body
            }
        }
        None => shown(raw, None),
    }
}

impl super::Conversation {
    /// What protected message `id` (by default the newest loaded one),
    /// once it is opened.
    pub(in crate::window) fn security(&self, id: Option<MessageId>) -> Option<&Security> {
        fn security(part: &Part) -> Option<&Security> {
            match part.body.as_ref()?.security.as_ref()? {
                Secured::Opened(security) => Some(security),
                Secured::Opening(_) => None,
            }
        }
        match id {
            Some(id) => self.parts.iter().find(|p| p.id == id).and_then(security),
            None => self
                .parts
                .iter()
                .rev()
                .find(|p| p.body.as_ref().is_some_and(|b| b.view.is_some()))
                .and_then(security),
        }
    }
}

impl MailWindow {
    /// Hands every protected message of the open conversation that is
    /// waiting to GnuPG.
    pub(super) fn open_sealed(&mut self, cx: &mut Context<Self>) {
        let Some(reader) = &mut self.reader else {
            return;
        };
        for part in &mut reader.parts {
            let Some(raw) = part.body.as_mut().and_then(|b| b.sealed.take()) else {
                continue;
            };
            let id = part.id;
            cx.spawn(async move |this, cx| {
                let body = cx
                    .background_executor()
                    .spawn(async move {
                        let opened = katna_crypto::open(&raw, &Gnupg::new());
                        opened_body(&raw, opened)
                    })
                    .await;
                this.update(cx, |this, cx| {
                    let Some(reader) = this.reader.as_mut() else {
                        return;
                    };
                    let Some(ix) = reader.parts.iter().position(|p| p.id == id) else {
                        return;
                    };
                    let part = &mut reader.parts[ix];
                    // Still waiting: the conversation may have been
                    // reloaded meanwhile.
                    if !part
                        .body
                        .as_ref()
                        .is_some_and(|b| matches!(b.security, Some(Secured::Opening(_))))
                    {
                        return;
                    }
                    // An encrypted subject ("..." outside) names the
                    // conversation once its first message is decrypted.
                    if ix == 0
                        && let Some(Secured::Opened(security)) = &body.security
                        && security.decrypted()
                        && let Some(subject) = body.view.as_ref().map(|v| v.subject.trim())
                        && !subject.is_empty()
                    {
                        reader.subject = subject.to_owned();
                    }
                    reader.parts[ix].body = Some(body);
                    cx.notify();
                })
                .ok();
            })
            .detach();
        }
    }

    /// Reads message `id` again and hands it to GnuPG, after the
    /// passphrase prompt was closed or something else went wrong.
    fn retry_sealed(&mut self, id: MessageId, cx: &mut Context<Self>) {
        let (Some(reader), Ok(mail)) = (&mut self.reader, &self.mail) else {
            return;
        };
        if let Some(part) = reader.parts.iter_mut().find(|p| p.id == id) {
            part.body = Some(super::read(mail, id));
        }
        cx.notify();
    }

    /// The lines above a protected message's body, if it is protected.
    pub(super) fn security_banner(
        &self,
        part: &Part,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let secured = part.body.as_ref()?.security.as_ref()?;
        let green = if th.dark { 0x81c995ff } else { 0x188038ff };
        let mut lines: Vec<Line> = Vec::new();
        let mut retry = false;
        match secured {
            Secured::Opening(Protection::Encrypted(_)) => lines.push(Line {
                icon: "lock",
                color: th.text_faint,
                text: "Decrypting…".into(),
            }),
            Secured::Opening(Protection::Signed(_)) => lines.push(Line {
                icon: "shield",
                color: th.text_faint,
                text: "Checking the signature…".into(),
            }),
            Secured::Opened(security) => {
                if let Some(decryption) = &security.decryption {
                    let (color, text) = decryption_line(decryption, security.standard, th);
                    retry = matches!(
                        decryption,
                        Decryption::Failed(Failure::Cancelled | Failure::Other(_))
                    );
                    lines.push(Line {
                        icon: "lock",
                        color,
                        text,
                    });
                }
                for signature in &security.signatures {
                    lines.push(signature_line(signature, security.standard, green, th));
                }
                if !security.whole {
                    let what = if security.encrypted() {
                        "encrypted"
                    } else {
                        "signed"
                    };
                    lines.push(Line {
                        icon: "info",
                        color: th.error,
                        text: format!(
                            "Only part of this message is {what}. The rest was added \
                             outside the protection and could come from anyone."
                        ),
                    });
                }
            }
        }
        let id = part.id;
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
                .children(lines.into_iter().map(|line| {
                    div()
                        .flex()
                        .flex_row()
                        .items_start()
                        .gap(px(8.0))
                        .child(icon(line.icon, line.color, 18.0))
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .text_color(rgba(if line.color == th.text_faint {
                                    th.text_dim
                                } else {
                                    line.color
                                }))
                                .child(line.text),
                        )
                }))
                .when(retry, |d| {
                    d.child(
                        div()
                            .id(("security-retry", id.0 as usize))
                            .pl(px(26.0))
                            .cursor_pointer()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(th.accent))
                            .hover(|s| s.text_color(rgba(fade(th.accent, 0.8))))
                            .on_click(cx.listener(move |this, _, _, cx| this.retry_sealed(id, cx)))
                            .child("Try again"),
                    )
                })
                .into_any_element(),
        )
    }
}

struct Line {
    icon: &'static str,
    color: u32,
    text: String,
}

fn tool(standard: Standard) -> &'static str {
    match standard {
        Standard::OpenPgp => "GnuPG (gpg)",
        Standard::Smime => "GnuPG's S/MIME tool (gpgsm)",
    }
}

fn decryption_line(decryption: &Decryption, standard: Standard, th: &Theme) -> (u32, String) {
    let failure = match decryption {
        Decryption::Decrypted => {
            let text = match standard {
                Standard::OpenPgp => "Encrypted message",
                Standard::Smime => "Encrypted message (S/MIME)",
            };
            return (th.text_faint, text.to_owned());
        }
        Decryption::Failed(failure) => failure,
    };
    let text = match failure {
        Failure::NoSecretKey => {
            "Can't decrypt this message: it was encrypted for a key you don't have.".to_owned()
        }
        Failure::Cancelled => "Decrypting was cancelled.".to_owned(),
        Failure::Damaged => {
            "Can't decrypt this message: the encrypted data is damaged or was changed.".to_owned()
        }
        Failure::Unavailable => format!(
            "Can't decrypt this message: install {} to read encrypted mail.",
            tool(standard)
        ),
        Failure::Other(message) => format!("Can't decrypt this message: {message}"),
    };
    (th.error, text)
}

fn signature_line(signature: &Signature, standard: Standard, green: u32, th: &Theme) -> Line {
    let signer = signature
        .signer
        .clone()
        .unwrap_or_else(|| "an unknown signer".to_owned());
    let (icon, color, text) = match signature.state {
        SignatureState::Good if signature.verified() => (
            "shield-check",
            green,
            format!("Signed by {signer} · verified"),
        ),
        SignatureState::Good if !signature.from_sender => (
            "shield-alert",
            th.error,
            format!("Signed by {signer}, who is not the sender"),
        ),
        SignatureState::Good => match signature.validity {
            Validity::Never => (
                "shield-alert",
                th.error,
                format!("Signed by {signer}, with a key you marked as not trusted"),
            ),
            _ => (
                "shield",
                th.text_faint,
                format!("Signed by {signer} · the key is not verified"),
            ),
        },
        SignatureState::Bad => (
            "shield-alert",
            th.error,
            "Bad signature: this message was changed after it was signed, or the signature \
             is forged."
                .to_owned(),
        ),
        SignatureState::Expired => (
            "shield",
            th.text_faint,
            format!("Signed by {signer} · the signature has expired"),
        ),
        SignatureState::KeyExpired => (
            "shield",
            th.text_faint,
            format!("Signed by {signer} · the key has expired since"),
        ),
        SignatureState::KeyRevoked => (
            "shield-alert",
            th.error,
            format!("Signed by {signer} with a key that has been revoked"),
        ),
        SignatureState::MissingKey => {
            let key = signature
                .key
                .as_deref()
                .map(|key| format!(" ({})", short_key(key)))
                .unwrap_or_default();
            (
                "shield",
                th.text_faint,
                format!("Signed with a key you don't have{key}, so it can't be checked"),
            )
        }
        SignatureState::Unavailable => (
            "shield",
            th.text_faint,
            format!("Signed; install {} to check the signature", tool(standard)),
        ),
        SignatureState::Error => (
            "shield-alert",
            th.error,
            "The signature could not be checked.".to_owned(),
        ),
    };
    Line { icon, color, text }
}

/// The last 16 hex digits of a fingerprint, in groups of four.
fn short_key(key: &str) -> String {
    let tail = &key[key.len().saturating_sub(16)..];
    tail.as_bytes()
        .chunks(4)
        .map(|c| String::from_utf8_lossy(c).into_owned())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_keys() {
        assert_eq!(
            short_key("846A21AE29D952F43D0E033A658CA70CA20C0FE0"),
            "658C A70C A20C 0FE0"
        );
        assert_eq!(short_key("ABCD"), "ABCD");
    }

    #[test]
    fn encrypted_mail_is_marked_for_remote_blocking() {
        let raw = b"From: Ada <ada@example.org>\r\n\
Content-Type: text/html\r\n\r\n<p>Hi<img src=\"https://example.org/t.png\"></p>\r\n";
        let security = |decryption| {
            Secured::Opened(katna_crypto::Security {
                standard: katna_crypto::Standard::OpenPgp,
                whole: true,
                decryption,
                signatures: Vec::new(),
            })
        };
        let opened = shown(raw, Some(security(Some(Decryption::Decrypted))));
        assert!(!opened.remote.is_empty() && opened.encrypted());
        assert!(!shown(raw, Some(security(None))).encrypted());
        assert!(!shown(raw, None).encrypted());
        let opening = sealed(
            raw.to_vec(),
            katna_crypto::Protection::Encrypted(katna_crypto::Standard::OpenPgp),
        );
        assert!(opening.encrypted());
    }

    #[test]
    fn failed_decryption_shows_headers_only() {
        let raw = b"From: Ada <ada@example.org>\r\nSubject: Secret\r\n\
Content-Type: multipart/encrypted; protocol=\"application/pgp-encrypted\"; boundary=\"e\"\r\n\r\n\
--e\r\nContent-Type: application/pgp-encrypted\r\n\r\nVersion: 1\r\n\
--e\r\nContent-Type: application/octet-stream\r\n\r\n\
-----BEGIN PGP MESSAGE-----\r\nxx\r\n-----END PGP MESSAGE-----\r\n--e--\r\n";
        let gnupg = Gnupg::new().with_programs("/nonexistent/gpg", "/nonexistent/gpgsm");
        let body = opened_body(raw, katna_crypto::open(raw, &gnupg));
        let view = body.view.expect("view");
        assert_eq!(view.subject, "Secret");
        assert!(view.body.is_empty() && view.attachments.is_empty());
        assert!(matches!(
            body.security,
            Some(Secured::Opened(Security {
                decryption: Some(Decryption::Failed(Failure::Unavailable)),
                ..
            }))
        ));
    }
}
