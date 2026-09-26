// SPDX-License-Identifier: GPL-3.0-or-later

//! Account model shared by the daemon, the store and the apps.
//!
//! Server settings are [`AccountSettings`], stored as JSON with the account.
//! Secrets are never part of this model: they live in the Secret Service
//! (`docs/ARCHITECTURE.md` §5.1).

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

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

/// How an account reaches its servers.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct AccountSettings {
    /// Incoming mail (IMAP accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imap: Option<Server>,
    /// Outgoing mail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp: Option<Server>,
}

/// One server of an account.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
    pub security: Security,
    /// Login name; often the address.
    pub username: String,
    /// Accept any certificate. Only for local test servers with
    /// self-signed certificates.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub accept_invalid_certs: bool,
}

/// Transport security of a [`Server`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Security {
    /// TLS from the first byte (IMAPS 993, SMTPS 465).
    Tls,
    /// Plain text, upgraded with STARTTLS (143, 587).
    StartTls,
    /// No encryption. Only for local test servers.
    Plain,
}

impl Security {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Tls => "tls",
            Self::StartTls => "starttls",
            Self::Plain => "plain",
        }
    }
}

impl fmt::Display for Security {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Security {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        [Self::Tls, Self::StartTls, Self::Plain]
            .into_iter()
            .find(|security| security.as_str() == s)
            .ok_or_else(|| format!("unknown security {s:?} (tls, starttls or plain)"))
    }
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
    fn settings_json_is_compact_and_round_trips() {
        let settings = AccountSettings {
            imap: Some(Server {
                host: "imap.example.org".into(),
                port: 993,
                security: Security::Tls,
                username: "alice".into(),
                accept_invalid_certs: false,
            }),
            smtp: None,
        };
        let json = serde_json::to_string(&settings).unwrap();
        assert_eq!(
            json,
            r#"{"imap":{"host":"imap.example.org","port":993,"security":"tls","username":"alice"}}"#
        );
        assert_eq!(
            serde_json::from_str::<AccountSettings>(&json).unwrap(),
            settings
        );
        assert_eq!(
            serde_json::from_str::<AccountSettings>("{}").unwrap(),
            AccountSettings::default()
        );
        assert_eq!("starttls".parse(), Ok(Security::StartTls));
    }

    #[test]
    fn mail_kinds() {
        assert!(AccountKind::Imap.is_mail());
        assert!(AccountKind::Pop3.is_mail());
        assert!(AccountKind::Local.is_mail());
        assert!(!AccountKind::CalDav.is_mail());
    }
}
