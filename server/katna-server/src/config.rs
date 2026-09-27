// SPDX-License-Identifier: GPL-3.0-or-later

//! Settings, read from the environment (the container's compose file).

use std::net::SocketAddr;

/// Server settings.
#[derive(Clone, Debug)]
pub struct Config {
    /// Address the HTTP server listens on (`KATNA_SERVER_LISTEN`, default
    /// `0.0.0.0:8080`). TLS is the reverse proxy's job.
    pub listen: SocketAddr,
    /// PostgreSQL connection string (`DATABASE_URL`).
    pub database_url: String,
    /// Read the client address from the last `X-Forwarded-For` entry
    /// (`KATNA_SERVER_TRUST_FORWARDED=1`). Only behind a reverse proxy that
    /// sets it; otherwise anyone could pick their own address.
    pub trust_forwarded: bool,
    /// Days after which tracking IDs, their events and unused installs are
    /// deleted (`KATNA_SERVER_RETENTION_DAYS`, default 180).
    pub retention_days: u32,
    /// Tracking IDs one install may create per 24 hours
    /// (`KATNA_SERVER_DAILY_LIMIT`, default 5000).
    pub daily_limit: u32,
    /// New installs one address may register per hour
    /// (`KATNA_SERVER_INSTALLS_PER_HOUR`, default 10).
    pub installs_per_hour: u32,
    /// Where the mail with Katna account codes goes out: made from
    /// `KATNA_SERVER_SMTP_HOST`, `_PORT` (465), `_USERNAME` and `_PASSWORD`,
    /// or given whole as `KATNA_SERVER_SMTP_URL` (for example
    /// `smtps://user:password@smtp.example.com` or
    /// `smtp://user:password@smtp.example.com:587?tls=required`). Without
    /// either the codes are only written to the log, for local testing.
    pub smtp_url: Option<Secret>,
    /// The sender of that mail (`KATNA_SERVER_MAIL_FROM`; default the SMTP
    /// username when it is an address, else
    /// `Katna <no-reply@katna.invenia.in>`).
    pub mail_from: String,
}

/// A setting that holds a password, kept out of debug output.
#[derive(Clone)]
pub struct Secret(pub String);

impl std::fmt::Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<hidden>")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            listen: SocketAddr::from(([0, 0, 0, 0], 8080)),
            database_url: String::new(),
            trust_forwarded: false,
            retention_days: 180,
            daily_limit: 5000,
            installs_per_hour: 10,
            smtp_url: None,
            mail_from: "Katna <no-reply@katna.invenia.in>".into(),
        }
    }
}

/// A setting that could not be read.
#[derive(Debug, thiserror::Error)]
#[error("{name}: {problem}")]
pub struct ConfigError {
    name: &'static str,
    problem: String,
}

impl Config {
    /// Reads the settings from the environment.
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_lookup(|name| std::env::var(name).ok())
    }

    /// Reads the settings through `lookup` (the environment, or a map in
    /// tests).
    pub fn from_lookup(lookup: impl Fn(&str) -> Option<String>) -> Result<Self, ConfigError> {
        let mut config = Config::default();
        if let Some(value) = lookup("KATNA_SERVER_LISTEN") {
            config.listen = parse("KATNA_SERVER_LISTEN", &value)?;
        }
        config.database_url =
            lookup("DATABASE_URL")
                .filter(|url| !url.is_empty())
                .ok_or(ConfigError {
                    name: "DATABASE_URL",
                    problem: "not set".into(),
                })?;
        if let Some(value) = lookup("KATNA_SERVER_TRUST_FORWARDED") {
            config.trust_forwarded = matches!(value.as_str(), "1" | "true" | "yes");
        }
        if let Some(value) = lookup("KATNA_SERVER_RETENTION_DAYS") {
            config.retention_days = parse("KATNA_SERVER_RETENTION_DAYS", &value)?;
        }
        if let Some(value) = lookup("KATNA_SERVER_DAILY_LIMIT") {
            config.daily_limit = parse("KATNA_SERVER_DAILY_LIMIT", &value)?;
        }
        if let Some(value) = lookup("KATNA_SERVER_INSTALLS_PER_HOUR") {
            config.installs_per_hour = parse("KATNA_SERVER_INSTALLS_PER_HOUR", &value)?;
        }
        let set = |name: &str| {
            lookup(name)
                .map(|v| v.trim().to_owned())
                .filter(|v| !v.is_empty())
        };
        let username = set("KATNA_SERVER_SMTP_USERNAME");
        config.smtp_url = match (set("KATNA_SERVER_SMTP_URL"), set("KATNA_SERVER_SMTP_HOST")) {
            (Some(url), _) => Some(Secret(url)),
            (None, Some(host)) => {
                let port: u16 = match set("KATNA_SERVER_SMTP_PORT") {
                    Some(port) => parse("KATNA_SERVER_SMTP_PORT", &port)?,
                    None => 465,
                };
                // The password is taken as written: spaces are part of it.
                let password = lookup("KATNA_SERVER_SMTP_PASSWORD").unwrap_or_default();
                Some(Secret(smtp_url(
                    &host,
                    port,
                    username.as_deref(),
                    &password,
                )))
            }
            (None, None) => None,
        };
        if let Some(value) = set("KATNA_SERVER_MAIL_FROM") {
            config.mail_from = value;
        } else if let Some(address) = username.filter(|u| u.contains('@')) {
            config.mail_from = format!("Katna <{address}>");
        }
        Ok(config)
    }
}

/// The SMTP URL for `host` and `port`: TLS from the start on 465, else
/// STARTTLS, required. The username and password are percent-encoded.
fn smtp_url(host: &str, port: u16, username: Option<&str>, password: &str) -> String {
    let login = match username {
        Some(user) => format!("{}:{}@", encode(user), encode(password)),
        None => String::new(),
    };
    if port == 465 {
        format!("smtps://{login}{host}:{port}")
    } else {
        format!("smtp://{login}{host}:{port}?tls=required")
    }
}

/// Percent-encodes all but letters, digits and `-._~`.
fn encode(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for byte in text.bytes() {
        if byte.is_ascii_alphanumeric() || b"-._~".contains(&byte) {
            out.push(char::from(byte));
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
    }
    out
}

fn parse<T: std::str::FromStr>(name: &'static str, value: &str) -> Result<T, ConfigError>
where
    T::Err: std::fmt::Display,
{
    value.trim().parse().map_err(|error: T::Err| ConfigError {
        name,
        problem: format!("{value:?}: {error}"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn reads_settings() {
        let env: HashMap<&str, &str> = HashMap::from([
            ("DATABASE_URL", "postgres://katna@db/katna"),
            ("KATNA_SERVER_LISTEN", "127.0.0.1:9000"),
            ("KATNA_SERVER_TRUST_FORWARDED", "1"),
            ("KATNA_SERVER_RETENTION_DAYS", "30"),
        ]);
        let config = Config::from_lookup(|name| env.get(name).map(|v| v.to_string())).unwrap();
        assert_eq!(config.listen.port(), 9000);
        assert!(config.trust_forwarded);
        assert_eq!(config.retention_days, 30);
        assert_eq!(config.daily_limit, 5000);
        assert!(config.smtp_url.is_none());
    }

    #[test]
    fn hides_the_smtp_password() {
        let config = Config::from_lookup(|name| match name {
            "DATABASE_URL" => Some("postgres://x".into()),
            "KATNA_SERVER_SMTP_URL" => Some("smtps://me:hunter2@smtp.example.com".into()),
            _ => None,
        })
        .unwrap();
        assert!(!format!("{config:?}").contains("hunter2"));
    }

    #[test]
    fn builds_the_smtp_url_from_parts() {
        let env: HashMap<&str, &str> = HashMap::from([
            ("DATABASE_URL", "postgres://x"),
            ("KATNA_SERVER_SMTP_HOST", "smtppro.zoho.in"),
            ("KATNA_SERVER_SMTP_USERNAME", "no-reply@invenia.in"),
            ("KATNA_SERVER_SMTP_PASSWORD", "p@ss/word 1"),
        ]);
        let config = Config::from_lookup(|name| env.get(name).map(|v| v.to_string())).unwrap();
        assert_eq!(
            config.smtp_url.unwrap().0,
            "smtps://no-reply%40invenia.in:p%40ss%2Fword%201@smtppro.zoho.in:465"
        );
        assert_eq!(config.mail_from, "Katna <no-reply@invenia.in>");

        let env: HashMap<&str, &str> = HashMap::from([
            ("DATABASE_URL", "postgres://x"),
            ("KATNA_SERVER_SMTP_HOST", "smtp.example.com"),
            ("KATNA_SERVER_SMTP_PORT", "587"),
            ("KATNA_SERVER_SMTP_USERNAME", "katna"),
            ("KATNA_SERVER_SMTP_PASSWORD", "secret"),
            ("KATNA_SERVER_MAIL_FROM", "Katna <codes@example.com>"),
        ]);
        let config = Config::from_lookup(|name| env.get(name).map(|v| v.to_string())).unwrap();
        assert_eq!(
            config.smtp_url.unwrap().0,
            "smtp://katna:secret@smtp.example.com:587?tls=required"
        );
        assert_eq!(config.mail_from, "Katna <codes@example.com>");
    }

    #[test]
    fn needs_a_database() {
        assert!(Config::from_lookup(|_| None).is_err());
        let bad = Config::from_lookup(|name| match name {
            "DATABASE_URL" => Some("postgres://x".into()),
            "KATNA_SERVER_RETENTION_DAYS" => Some("soon".into()),
            _ => None,
        });
        assert!(bad.is_err());
    }
}
