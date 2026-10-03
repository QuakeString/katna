// SPDX-License-Identifier: GPL-3.0-or-later

//! The mail providers on the first step of adding an account, as tiles:
//! what each is called, how it signs in, and where it explains the app
//! password it may ask for. Google and Microsoft show their own marks,
//! which their sign-in branding guidelines let apps use on a sign-in
//! button; the others show their own marks too, from the open icon sets
//! `assets.rs` names, only to stand for that provider.

use gpui::{AnyElement, div, prelude::*, rgba};
use katna_core::OAuthProvider;
use katna_i18n::tr;
use katna_ui::px;

use crate::theme::{Theme, mix};
use crate::widgets::icon;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MailProvider {
    Google,
    Microsoft,
    Yahoo,
    ICloud,
    Zoho,
    Fastmail,
    Gmx,
    Yandex,
    /// Any other provider, by IMAP or POP3.
    Other,
}

/// What a provider needs before Katna can read its mail with a password.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PasswordHelp {
    /// An app password, made in the account's security settings.
    AppPassword,
    /// IMAP and POP3 access turned on in its web mail's settings.
    TurnOnImap,
}

impl MailProvider {
    /// In the order of the tiles; `Other` spans the last row.
    pub(super) const ALL: [Self; 9] = [
        Self::Google,
        Self::Microsoft,
        Self::Yahoo,
        Self::ICloud,
        Self::Zoho,
        Self::Fastmail,
        Self::Gmx,
        Self::Yandex,
        Self::Other,
    ];

    /// The tiles this build shows: Microsoft's only once it can sign in
    /// there, as its mail takes nothing else.
    pub(super) fn shown() -> impl Iterator<Item = Self> {
        Self::ALL
            .into_iter()
            .filter(|p| *p != Self::Microsoft || OAuthProvider::Microsoft.available())
    }

    /// The provider's own name, the same in every language.
    pub(super) fn name(self) -> String {
        match self {
            Self::Google => "Google".into(),
            Self::Microsoft => "Microsoft".into(),
            Self::Yahoo => "Yahoo".into(),
            Self::ICloud => "iCloud".into(),
            Self::Zoho => "Zoho".into(),
            Self::Fastmail => "Fastmail".into(),
            Self::Gmx => "GMX".into(),
            Self::Yandex => "Yandex".into(),
            Self::Other => tr!("add-account-provider-other"),
        }
    }

    /// The line under the name on its tile.
    pub(super) fn detail(self) -> String {
        match self {
            Self::Google => tr!("add-account-provider-google-detail"),
            Self::Microsoft => tr!("add-account-provider-microsoft-detail"),
            Self::Other => tr!("add-account-provider-other-detail"),
            other => tr!("add-account-provider-mail", provider = other.name()),
        }
    }

    /// The name of its mail, for "Sign in to …".
    pub(super) fn mail_name(self) -> String {
        match self {
            Self::Google => "Gmail".into(),
            Self::Microsoft => "Outlook".into(),
            Self::ICloud => "iCloud Mail".into(),
            Self::Gmx => "GMX Mail".into(),
            Self::Other => String::new(),
            other => format!("{} Mail", other.name()),
        }
    }

    /// Its sign-in in the browser, when this build has it.
    pub(super) fn sign_in(self) -> Option<OAuthProvider> {
        let provider = match self {
            Self::Google => OAuthProvider::Google,
            Self::Microsoft => OAuthProvider::Microsoft,
            _ => return None,
        };
        provider.available().then_some(provider)
    }

    /// What the provider asks for first, and the page that shows how.
    pub(super) fn password_help(self) -> Option<(PasswordHelp, &'static str)> {
        let help = match self {
            Self::Google => (
                PasswordHelp::AppPassword,
                "https://support.google.com/accounts/answer/185833",
            ),
            Self::Yahoo => (
                PasswordHelp::AppPassword,
                "https://help.yahoo.com/kb/SLN15241.html",
            ),
            Self::ICloud => (
                PasswordHelp::AppPassword,
                "https://support.apple.com/102654",
            ),
            Self::Zoho => (
                PasswordHelp::AppPassword,
                "https://www.zoho.com/mail/help/imap-access.html",
            ),
            Self::Fastmail => (
                PasswordHelp::AppPassword,
                "https://www.fastmail.help/hc/en-us/articles/360058752854",
            ),
            Self::Gmx => (
                PasswordHelp::TurnOnImap,
                "https://support.gmx.com/pop-imap/toggle.html",
            ),
            Self::Yandex => (
                PasswordHelp::AppPassword,
                "https://yandex.com/support/id/en/authorization/app-passwords",
            ),
            Self::Microsoft | Self::Other => return None,
        };
        Some(help)
    }

    /// The provider an address belongs to, from its domain; `Other` when
    /// Katna has no tile for it.
    pub(super) fn for_address(address: &str) -> Self {
        let domain = address
            .rsplit_once('@')
            .map_or("", |(_, d)| d)
            .trim()
            .to_lowercase();
        match domain.as_str() {
            "gmail.com" | "googlemail.com" => Self::Google,
            "outlook.com" | "hotmail.com" | "live.com" | "msn.com" => Self::Microsoft,
            "yahoo.com" | "ymail.com" | "rocketmail.com" => Self::Yahoo,
            "icloud.com" | "me.com" | "mac.com" => Self::ICloud,
            "zoho.com" | "zohomail.com" | "zoho.in" | "zoho.eu" => Self::Zoho,
            "fastmail.com" | "fastmail.fm" => Self::Fastmail,
            "gmx.com" | "gmx.net" | "gmx.de" => Self::Gmx,
            "yandex.com" | "yandex.ru" | "ya.ru" => Self::Yandex,
            _ => Self::Other,
        }
    }

    /// Its mark, `size` px square.
    pub(super) fn glyph(self, size: f32, th: &Theme) -> AnyElement {
        match self {
            Self::Google => layered(
                size,
                &[
                    ("google-g-blue", 0x4285f4ff),
                    ("google-g-green", 0x34a853ff),
                    ("google-g-yellow", 0xfbbc05ff),
                    ("google-g-red", 0xea4335ff),
                ],
            ),
            Self::Microsoft => {
                // Four squares with a gap of a twentieth between them.
                let gap = size / 21.0;
                let square = (size - gap) / 2.0;
                let tile = |color: u32| div().size(px(square)).bg(rgba(color));
                div()
                    .size(px(size))
                    .flex_none()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(gap))
                    .child(tile(0xf25022ff))
                    .child(tile(0x7fba00ff))
                    .child(tile(0x00a4efff))
                    .child(tile(0xffb900ff))
                    .into_any_element()
            }
            Self::Other => div()
                .size(px(size))
                .flex_none()
                .flex()
                .items_center()
                .justify_center()
                .child(icon("mail", th.text_dim, size * 0.9))
                .into_any_element(),
            Self::Fastmail => {
                // Its dark envelope flap would vanish on a dark surface.
                let ink = if th.dark { 0xc9d1d9ff } else { 0x333e48ff };
                layered(
                    size,
                    &[
                        ("brand-fastmail-blue", 0x0067b9ff),
                        ("brand-fastmail-sky", 0x69b3e7ff),
                        ("brand-fastmail-yellow", 0xffc107ff),
                        ("brand-fastmail-ink", ink),
                    ],
                )
            }
            Self::Yandex => div()
                .size(px(size))
                .flex_none()
                .flex()
                .items_center()
                .justify_center()
                .rounded_full()
                .bg(rgba(0xfc3f1dff))
                .child(icon("brand-yandex", 0xffffffff, size * 0.6))
                .into_any_element(),
            other => {
                let (name, color, scale) = match other {
                    Self::Yahoo => ("brand-yahoo", 0x6001d2ff, 0.9),
                    Self::ICloud => ("brand-icloud", 0x3693f3ff, 1.0),
                    Self::Zoho => ("brand-zoho", 0xe42527ff, 1.0),
                    _ => ("brand-gmx", 0x1c449bff, 1.0),
                };
                // Lighter on a dark surface, so a dark mark still reads.
                let color = if th.dark {
                    mix(color, 0xffff_ffff, 0.35)
                } else {
                    color
                };
                div()
                    .size(px(size))
                    .flex_none()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(icon(name, color, size * scale))
                    .into_any_element()
            }
        }
    }
}

/// A mark in several colours: one single-colour icon per colour, stacked.
pub(super) fn layered(size: f32, parts: &[(&'static str, u32)]) -> AnyElement {
    div()
        .relative()
        .size(px(size))
        .flex_none()
        .children(parts.iter().map(|&(name, color)| {
            div()
                .absolute()
                .top_0()
                .left_0()
                .child(icon(name, color, size))
        }))
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn knows_providers_by_address() {
        assert_eq!(
            MailProvider::for_address("Kay@GMail.com"),
            MailProvider::Google
        );
        assert_eq!(MailProvider::for_address("kay@gmx.de"), MailProvider::Gmx);
        assert_eq!(
            MailProvider::for_address("kay@example.org"),
            MailProvider::Other
        );
        assert_eq!(MailProvider::for_address("nobody"), MailProvider::Other);
    }

    #[test]
    fn other_is_the_last_tile() {
        assert_eq!(MailProvider::ALL.last(), Some(&MailProvider::Other));
        assert!(MailProvider::Other.password_help().is_none());
    }
}
