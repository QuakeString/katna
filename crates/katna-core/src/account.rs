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
    /// Incoming mail (POP3 accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pop3: Option<Server>,
    /// What a POP3 account leaves on the server.
    #[serde(skip_serializing_if = "Pop3Keep::is_default")]
    pub pop3_keep: Pop3Keep,
    /// Signs in through this provider with OAuth2 instead of a password;
    /// the Secret Service then keeps the refresh token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth: Option<OAuthProvider>,
    /// A sign-in beside the account's own, for services its mail login
    /// does not reach (Zoho's tasks and calendars for a Zoho Mail account
    /// that logs in with a password). The Secret Service keeps its refresh
    /// token apart from the password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked: Option<LinkedSignIn>,
}

/// A sign-in linked to an account ([`AccountSettings::linked`]).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkedSignIn {
    pub provider: OAuthProvider,
    /// The provider's sign-in server of the user's data centre, which
    /// refreshes the tokens (Zoho: `https://accounts.zoho.in`).
    pub accounts_server: String,
    /// Where the provider's APIs answer for this user (Zoho:
    /// `https://www.zohoapis.in`); empty when it did not say.
    #[serde(default)]
    pub api_domain: String,
}

/// A provider Katna signs in to with OAuth2 (`docs/ARCHITECTURE.md` §6.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OAuthProvider {
    /// Gmail and Google Workspace.
    Google,
    /// Outlook.com, Hotmail and Microsoft 365.
    Microsoft,
    /// Zoho's tasks and calendars. Only a linked sign-in
    /// ([`AccountSettings::linked`]): Zoho lets only its "self client" apps
    /// log in to IMAP with OAuth2, so Zoho Mail keeps its password.
    Zoho,
}

impl OAuthProvider {
    pub const ALL: [Self; 3] = [Self::Google, Self::Microsoft, Self::Zoho];

    /// The providers an account can log in to mail with.
    pub const MAIL: [Self; 2] = [Self::Google, Self::Microsoft];

    /// Stable name used in settings and on D-Bus.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Google => "google",
            Self::Microsoft => "microsoft",
            Self::Zoho => "zoho",
        }
    }

    /// The provider's own name, for "Sign in with …".
    pub fn name(self) -> &'static str {
        match self {
            Self::Google => "Google",
            Self::Microsoft => "Microsoft",
            Self::Zoho => "Zoho",
        }
    }

    /// The client ID from [`crate::ids`]; empty when this build cannot
    /// sign in to the provider.
    pub fn client_id(self) -> &'static str {
        match self {
            Self::Google => crate::ids::GOOGLE_OAUTH_CLIENT_ID,
            Self::Microsoft => crate::ids::MICROSOFT_OAUTH_CLIENT_ID,
            Self::Zoho => crate::ids::ZOHO_OAUTH_CLIENT_ID,
        }
    }

    /// Whether this build can sign in to the provider.
    pub fn available(self) -> bool {
        !self.client_id().is_empty()
    }
}

impl fmt::Display for OAuthProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for OAuthProvider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|provider| provider.as_str() == s)
            .ok_or_else(|| format!("unknown sign-in provider {s:?} (google, microsoft or zoho)"))
    }
}

/// What a POP3 account leaves on the server once mail is downloaded.
/// The default is Thunderbird's: keep it until it is deleted in Katna.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Pop3Keep {
    /// Leave downloaded mail on the server. When `false`, mail is removed
    /// from the server as soon as it is stored.
    pub leave_on_server: bool,
    /// With `leave_on_server`: remove mail from the server this many days
    /// after downloading it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<u32>,
    /// With `leave_on_server`: remove mail from the server once it is
    /// deleted for good in Katna.
    pub delete_with_local: bool,
}

impl Default for Pop3Keep {
    fn default() -> Self {
        Self {
            leave_on_server: true,
            days: None,
            delete_with_local: true,
        }
    }
}

impl Pop3Keep {
    fn is_default(&self) -> bool {
        *self == Self::default()
    }
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
            ..AccountSettings::default()
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
    fn pop3_keep_defaults_to_until_deleted() {
        let keep: Pop3Keep = serde_json::from_str(r#"{"days":14}"#).unwrap();
        assert_eq!(
            keep,
            Pop3Keep {
                days: Some(14),
                ..Pop3Keep::default()
            }
        );
        let settings = AccountSettings {
            pop3_keep: Pop3Keep {
                leave_on_server: false,
                ..Pop3Keep::default()
            },
            ..AccountSettings::default()
        };
        assert_eq!(
            serde_json::to_string(&settings).unwrap(),
            r#"{"pop3_keep":{"leave_on_server":false,"delete_with_local":true}}"#
        );
    }

    #[test]
    fn oauth_provider_is_stored_by_name() {
        let settings = AccountSettings {
            oauth: Some(OAuthProvider::Microsoft),
            ..AccountSettings::default()
        };
        let json = serde_json::to_string(&settings).unwrap();
        assert_eq!(json, r#"{"oauth":"microsoft"}"#);
        assert_eq!(
            serde_json::from_str::<AccountSettings>(&json).unwrap(),
            settings
        );
        for provider in OAuthProvider::ALL {
            assert_eq!(provider.as_str().parse(), Ok(provider));
        }
        assert!("yahoo".parse::<OAuthProvider>().is_err());
    }

    #[test]
    fn mail_kinds() {
        assert!(AccountKind::Imap.is_mail());
        assert!(AccountKind::Pop3.is_mail());
        assert!(AccountKind::Local.is_mail());
        assert!(!AccountKind::CalDav.is_mail());
    }
}
