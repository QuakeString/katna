// SPDX-License-Identifier: GPL-3.0-or-later

//! Account model shared by the daemon, the store and the apps.
//!
//! Protocol settings (servers, ports, sync options) are added with the sync
//! engine in Phase 1. Secrets are never part of this model: they live in the
//! Secret Service (`docs/ARCHITECTURE.md` §5.1).

use std::fmt;
use std::str::FromStr;

/// Database ID of an account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccountId(pub i64);

impl fmt::Display for AccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// The protocol an account uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccountKind {
    Imap,
    Jmap,
    Pop3,
    CalDav,
    CardDav,
    /// Mail imported from local files (Maildir, mbox); never synced.
    Local,
}

impl AccountKind {
    /// Every kind, in a stable order.
    pub const ALL: [Self; 6] = [
        Self::Imap,
        Self::Jmap,
        Self::Pop3,
        Self::CalDav,
        Self::CardDav,
        Self::Local,
    ];

    /// Stable name used in the database and in configuration.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Imap => "imap",
            Self::Jmap => "jmap",
            Self::Pop3 => "pop3",
            Self::CalDav => "caldav",
            Self::CardDav => "carddav",
            Self::Local => "local",
        }
    }

    /// Whether the account carries mail.
    pub fn is_mail(self) -> bool {
        matches!(self, Self::Imap | Self::Jmap | Self::Pop3 | Self::Local)
    }
}

impl fmt::Display for AccountKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Returned when parsing an unknown account kind.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("unknown account kind {0:?}")]
pub struct UnknownAccountKind(pub String);

impl FromStr for AccountKind {
    type Err = UnknownAccountKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|kind| kind.as_str() == s)
            .ok_or_else(|| UnknownAccountKind(s.to_owned()))
    }
}

/// A configured account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub id: AccountId,
    pub kind: AccountKind,
    /// Name shown in the sidebar, for example "Work".
    pub display_name: String,
    /// Main address of a mail account, or the login of a DAV account.
    pub address: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_names_round_trip() {
        for kind in AccountKind::ALL {
            assert_eq!(kind.as_str().parse::<AccountKind>(), Ok(kind));
            assert_eq!(kind.to_string(), kind.as_str());
        }
    }

    #[test]
    fn unknown_kind_is_an_error() {
        assert_eq!(
            "IMAP".parse::<AccountKind>(),
            Err(UnknownAccountKind("IMAP".to_owned()))
        );
    }

    #[test]
    fn mail_kinds() {
        assert!(AccountKind::Imap.is_mail());
        assert!(AccountKind::Pop3.is_mail());
        assert!(AccountKind::Local.is_mail());
        assert!(!AccountKind::CalDav.is_mail());
    }
}
