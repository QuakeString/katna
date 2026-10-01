// SPDX-License-Identifier: GPL-3.0-or-later

//! The mail Katna Server sends itself: the codes that confirm a Katna
//! account's address and reset its password. It goes out through the SMTP
//! relay set in `KATNA_SERVER_SMTP_HOST` and the lines after it. Without
//! one the server refuses to start, unless `KATNA_SERVER_DEV_MAILER=log`
//! asks for the codes to be written to the log (local testing only: anyone
//! who can read that log could reset any account).

use std::sync::{Arc, Mutex};
use std::time::Duration;

use lettre::message::{Mailbox, header::ContentType};
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

use crate::config::Config;

/// Why a code is sent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Purpose {
    /// Confirms the address of a new account.
    Verify,
    /// Lets someone who forgot the password set a new one.
    Reset,
    /// Second step of signing in to the server's admin page.
    Admin,
}

impl Purpose {
    /// The name stored in the database.
    pub fn as_str(self) -> &'static str {
        match self {
            Purpose::Verify => "verify",
            Purpose::Reset => "reset",
            Purpose::Admin => "admin",
        }
    }
}

/// Mail could not be sent.
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct MailError(String);

/// Sends account mail.
#[derive(Clone)]
pub enum Mailer {
    /// Through an SMTP relay.
    Smtp {
        /// The relay.
        transport: AsyncSmtpTransport<Tokio1Executor>,
        /// The sender.
        from: Mailbox,
    },
    /// Only into the log (no relay set, and `KATNA_SERVER_DEV_MAILER=log`).
    Log,
    /// Into memory, for tests to read.
    Memory(Arc<Mutex<Vec<SentCode>>>),
}

/// A code the [`Mailer::Memory`] mailer "sent".
#[derive(Clone, Debug)]
pub struct SentCode {
    /// The address.
    pub to: String,
    /// Why.
    pub purpose: Purpose,
    /// The code.
    pub code: String,
}

impl Mailer {
    /// The mailer the settings ask for. Without an SMTP relay that is an
    /// error, unless the settings ask for the log explicitly.
    pub fn from_config(config: &Config) -> Result<Self, MailError> {
        let Some(url) = &config.smtp_url else {
            if config.dev_mailer_log {
                return Ok(Mailer::Log);
            }
            return Err(MailError(
                "no SMTP relay set: set KATNA_SERVER_SMTP_HOST (or KATNA_SERVER_SMTP_URL), \
                 or KATNA_SERVER_DEV_MAILER=log to write account codes to the log \
                 (local testing only)"
                    .into(),
            ));
        };
        let transport = AsyncSmtpTransport::<Tokio1Executor>::from_url(&url.0)
            .map_err(|error| MailError(format!("SMTP settings: {error}")))?
            .timeout(Some(Duration::from_secs(20)))
            .build();
        let from = config
            .mail_from
            .parse()
            .map_err(|error| MailError(format!("KATNA_SERVER_MAIL_FROM: {error}")))?;
        Ok(Mailer::Smtp { transport, from })
    }

    /// Whether mail really goes out.
    pub fn sends_mail(&self) -> bool {
        matches!(self, Mailer::Smtp { .. })
    }

    /// Sends `code` to `to`.
    pub async fn send_code(&self, to: &str, purpose: Purpose, code: &str) -> Result<(), MailError> {
        let (transport, from) = match self {
            Mailer::Smtp { transport, from } => (transport, from),
            Mailer::Memory(sent) => {
                sent.lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .push(SentCode {
                        to: to.to_owned(),
                        purpose,
                        code: code.to_owned(),
                    });
                return Ok(());
            }
            Mailer::Log => {
                tracing::warn!(
                    to,
                    purpose = purpose.as_str(),
                    code,
                    "KATNA_SERVER_DEV_MAILER=log: code not mailed"
                );
                return Ok(());
            }
        };
        let (subject, body) = text(purpose, code);
        let to: Mailbox = to
            .parse()
            .map_err(|error| MailError(format!("address: {error}")))?;
        let message = Message::builder()
            .from(from.clone())
            .to(to)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)
            .map_err(|error| MailError(error.to_string()))?;
        transport
            .send(message)
            .await
            .map_err(|error| MailError(error.to_string()))?;
        Ok(())
    }
}

/// Subject and body of the mail. English only for now: the server does not
/// know the app's language yet.
fn text(purpose: Purpose, code: &str) -> (String, String) {
    match purpose {
        Purpose::Verify => (
            format!("{code} is your Katna account code"),
            format!(
                "Welcome to Katna.\n\n\
                 Type this code in Katna to confirm your address:\n\n    {code}\n\n\
                 It works for 30 minutes. If you did not create a Katna account, \
                 you can ignore this mail; the account is deleted after a week.\n"
            ),
        ),
        Purpose::Reset => (
            format!("{code} is your Katna password reset code"),
            format!(
                "Type this code in Katna to choose a new password:\n\n    {code}\n\n\
                 It works for 30 minutes. If you did not ask for it, you can ignore \
                 this mail; your password stays as it is.\n"
            ),
        ),
        Purpose::Admin => (
            format!("{code} is your Katna Server admin code"),
            format!(
                "Type this code on the Katna Server admin page to sign in:\n\n    {code}\n\n\
                 It works for 30 minutes. If you did not just sign in there, someone \
                 knows the admin password: set a new one on the server with \
                 `katna-server admin-password`.\n"
            ),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_code_is_in_subject_and_body() {
        for purpose in [Purpose::Verify, Purpose::Reset, Purpose::Admin] {
            let (subject, body) = text(purpose, "482913");
            assert!(subject.starts_with("482913 "));
            assert!(body.contains("    482913\n"));
        }
    }

    // The relay's connection pool needs a runtime.
    #[tokio::test]
    async fn reads_the_relay() {
        let config = Config {
            smtp_url: Some(crate::config::Secret(
                "smtp://user:pass@localhost:2525".into(),
            )),
            ..Config::default()
        };
        assert!(Mailer::from_config(&config).unwrap().sends_mail());
    }

    #[test]
    fn no_relay_is_an_error_unless_the_log_is_asked_for() {
        assert!(Mailer::from_config(&Config::default()).is_err());
        let config = Config {
            dev_mailer_log: true,
            ..Config::default()
        };
        assert!(!Mailer::from_config(&config).unwrap().sends_mail());
    }
}
