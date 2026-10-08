// SPDX-License-Identifier: GPL-3.0-or-later

//! What needs the user while Katna Mail's window may be closed
//! (`docs/ARCHITECTURE.md` §15.1.4): a password the server refused, a
//! sign-in that ended, mail that wasn't sent. Each gets one desktop
//! notification with its fix, never again while it lasts, and a line in
//! the tray icon's tooltip. What Katna waits out by itself (offline, a
//! server not answering) gets neither.

use katna_core::OAuthProvider;
use katna_dbus::{AccountStatus, OutboxItem, app_action, send_state, state};
use katna_i18n::tr;

/// From this many accounts, the tooltip says how many instead of each.
const FOLD_FROM: usize = 3;

/// One thing that needs the user.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum NeedsYou {
    /// The provider signed Katna out: sign in again.
    SignIn {
        account: i64,
        address: String,
        provider: OAuthProvider,
    },
    /// The server refused the saved password: type the new one.
    Password { account: i64, address: String },
    /// A message the server refused for good; it waits in the Outbox.
    NotSent { outbox: i64, subject: String },
}

/// Which problem a [`NeedsYou`] is, to tell it once.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Key {
    Account(i64),
    Outbox(i64),
}

impl NeedsYou {
    pub(crate) fn key(&self) -> Key {
        match self {
            Self::SignIn { account, .. } | Self::Password { account, .. } => Key::Account(*account),
            Self::NotSent { outbox, .. } => Key::Outbox(*outbox),
        }
    }

    /// The notification's title, text and button.
    pub(crate) fn notification(&self) -> (String, String, String) {
        match self {
            Self::SignIn {
                address, provider, ..
            } => (
                tr!("notify-signed-out"),
                tr!(
                    "notify-signed-out-body",
                    provider = provider.name(),
                    address = address.as_str()
                ),
                tr!("notify-sign-in"),
            ),
            Self::Password { address, .. } => (
                tr!("notify-password-refused"),
                tr!("notify-password-refused-body", address = address.as_str()),
                tr!("notify-new-password"),
            ),
            Self::NotSent { subject, .. } => (
                if subject.trim().is_empty() {
                    tr!("notify-not-sent-no-subject")
                } else {
                    tr!("notify-not-sent", subject = subject.trim())
                },
                tr!("notify-not-sent-body"),
                tr!("notify-open-outbox"),
            ),
        }
    }

    /// The Katna Mail page its click opens (`app_action::OPEN_PAGE`).
    pub(crate) fn page(&self) -> String {
        match self {
            Self::SignIn { account, .. } | Self::Password { account, .. } => {
                app_action::fix_page(*account)
            }
            Self::NotSent { .. } => app_action::OUTBOX_PAGE.to_owned(),
        }
    }
}

/// Everything that needs the user now: accounts first, then mail.
pub(crate) fn find(accounts: &[AccountStatus], outbox: &[OutboxItem]) -> Vec<NeedsYou> {
    let accounts = accounts
        .iter()
        .filter(|a| a.state == state::AUTH_FAILED)
        .map(|a| match a.sign_in.parse::<OAuthProvider>() {
            Ok(provider) => NeedsYou::SignIn {
                account: a.id,
                address: a.address.clone(),
                provider,
            },
            Err(_) => NeedsYou::Password {
                account: a.id,
                address: a.address.clone(),
            },
        });
    let mail = outbox
        .iter()
        .filter(|item| item.state == send_state::FAILED)
        .map(|item| NeedsYou::NotSent {
            outbox: item.id,
            subject: item.subject.clone(),
        });
    accounts.chain(mail).collect()
}

/// The tray tooltip's lines under the unread count: one per account, or
/// how many from [`FOLD_FROM`], then how many messages weren't sent.
pub(crate) fn tray_lines(all: &[NeedsYou]) -> Vec<String> {
    let accounts: Vec<&NeedsYou> = all
        .iter()
        .filter(|n| matches!(n.key(), Key::Account(_)))
        .collect();
    let mut lines: Vec<String> = if accounts.len() >= FOLD_FROM {
        vec![tr!("tray-accounts-need-you", count = accounts.len() as i64)]
    } else {
        accounts
            .iter()
            .map(|n| match n {
                NeedsYou::SignIn { address, .. } => {
                    tr!("tray-signed-out", address = address.as_str())
                }
                NeedsYou::Password { address, .. } => {
                    tr!("tray-password-refused", address = address.as_str())
                }
                NeedsYou::NotSent { .. } => String::new(),
            })
            .collect()
    };
    let not_sent = all.len() - accounts.len();
    if not_sent > 0 {
        lines.push(tr!("tray-not-sent", count = not_sent as i64));
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    fn account(id: i64, state: &str, sign_in: &str) -> AccountStatus {
        AccountStatus {
            id,
            kind: "imap".into(),
            display_name: String::new(),
            address: format!("a{id}@example.org"),
            state: state.into(),
            detail: String::new(),
            last_sync: 0,
            sign_in: sign_in.into(),
        }
    }

    fn mail(id: i64, state: &str) -> OutboxItem {
        OutboxItem {
            id,
            account: 1,
            message: id,
            subject: format!("Plan {id}"),
            send_at: 0,
            state: state.into(),
            detail: String::new(),
        }
    }

    #[test]
    fn finds_only_what_waits_for_the_user() {
        let all = find(
            &[
                account(1, state::AUTH_FAILED, ""),
                account(2, state::AUTH_FAILED, "google"),
                account(3, state::OFFLINE, ""),
                account(4, state::ONLINE, "google"),
            ],
            &[mail(7, send_state::FAILED), mail(8, send_state::QUEUED)],
        );
        assert_eq!(
            all.iter().map(NeedsYou::key).collect::<Vec<_>>(),
            [Key::Account(1), Key::Account(2), Key::Outbox(7)]
        );
        assert!(matches!(all[0], NeedsYou::Password { .. }));
        assert!(matches!(
            all[1],
            NeedsYou::SignIn {
                provider: OAuthProvider::Google,
                ..
            }
        ));
        assert_eq!(all[0].page(), app_action::fix_page(1));
        assert_eq!(all[2].page(), app_action::OUTBOX_PAGE);
    }

    #[test]
    fn tooltip_folds_many_accounts() {
        let one = find(&[account(1, state::AUTH_FAILED, "")], &[]);
        let lines = tray_lines(&one);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("a1@example.org"));
        let many = find(
            &[
                account(1, state::AUTH_FAILED, ""),
                account(2, state::AUTH_FAILED, ""),
                account(3, state::AUTH_FAILED, "google"),
            ],
            &[mail(7, send_state::FAILED), mail(9, send_state::FAILED)],
        );
        let lines = tray_lines(&many);
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains('3') && lines[1].contains('2'));
        assert!(tray_lines(&[]).is_empty());
    }
}
