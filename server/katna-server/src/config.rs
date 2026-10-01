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
    /// Tracking IDs one Katna account may create per 24 hours
    /// (`KATNA_SERVER_DAILY_LIMIT`, default 5000).
    pub daily_limit: u32,
    /// New installs one address may register per hour
    /// (`KATNA_SERVER_INSTALLS_PER_HOUR`, default 10).
    pub installs_per_hour: u32,
    /// LibreTranslate on the internal network, such as
    /// `http://translate:5000` (`KATNA_SERVER_TRANSLATE_URL`); empty turns
    /// translation off.
    pub translate_url: String,
    /// Translation requests one account may make per 24 hours
    /// (`KATNA_SERVER_TRANSLATIONS_PER_DAY`, default 2000; a long message
    /// takes one per 4000 characters).
    pub translations_per_day: u32,
    /// Translation requests passed to LibreTranslate at once, for all
    /// accounts together (`KATNA_SERVER_TRANSLATE_CONCURRENCY`, default 8:
    /// twice its default threads); more are answered 503 at once.
    pub translate_concurrency: usize,
    /// Where the mail with Katna account codes goes out: made from
    /// `KATNA_SERVER_SMTP_HOST`, `_PORT` (465), `_USERNAME` and `_PASSWORD`,
    /// or given whole as `KATNA_SERVER_SMTP_URL` (for example
    /// `smtps://user:password@smtp.example.com` or
    /// `smtp://user:password@smtp.example.com:587?tls=required`). Without
    /// either the server does not start, unless [`Config::dev_mailer_log`]
    /// is set.
    pub smtp_url: Option<Secret>,
    /// Write account codes to the log instead of mailing them, when no SMTP
    /// relay is set (`KATNA_SERVER_DEV_MAILER=log`). Only for local testing:
    /// anyone who reads the log could then reset any account.
    pub dev_mailer_log: bool,
    /// Hosts that links may not go to (`KATNA_SERVER_BLOCKED_HOSTS`, split
    /// by commas or spaces, lowercased). Each blocks itself and its
    /// subdomains: tracking IDs with such a link are refused, and stored
    /// links to them are no longer followed.
    pub blocked_hosts: Vec<String>,
    /// The sender of that mail (`KATNA_SERVER_MAIL_FROM`; default the SMTP
    /// username when it is an address, else
    /// `Katna <no-reply@katna.invenia.in>`).
    pub mail_from: String,
    /// Katna AI: writing help for Katna accounts (`crate::ai`).
    pub ai: AiConfig,
    /// Katna accounts that may open the admin page at `/admin`
    /// (`KATNA_SERVER_ADMIN_EMAILS`, split by commas or spaces,
    /// lowercased); empty turns the page off.
    pub admin_emails: Vec<String>,
}

/// Katna AI's settings (`KATNA_SERVER_AI_*`).
#[derive(Clone, Debug)]
pub struct AiConfig {
    /// Whether Katna AI answers at all (the admin page's switch).
    pub on: bool,
    /// The services asked, in order: the first, then the fallback when the
    /// first fails. Empty turns Katna AI off.
    pub services: Vec<AiService>,
    /// Every service with a key in the settings, which the admin page may
    /// choose from: `KATNA_SERVER_AI_<PROVIDER>_KEY` (and
    /// `KATNA_SERVER_AI_OTHER_BASE`), and those of [`AiConfig::services`].
    pub keys: Vec<AiService>,
    /// Days free from an account's first use (`KATNA_SERVER_AI_TRIAL_DAYS`,
    /// default 30).
    pub trial_days: u32,
    /// What one account may cost per calendar month, in millionths of a
    /// US dollar (`KATNA_SERVER_AI_ACCOUNT_CAP_USD`, default 1.00).
    pub account_cap_micros: u64,
    /// What all accounts together may cost per calendar month
    /// (`KATNA_SERVER_AI_BUDGET_USD`, default 50; 0 turns Katna AI off).
    pub budget_micros: u64,
    /// The price of a million tokens read and written
    /// (`KATNA_SERVER_AI_PRICE_IN_USD`, default 0.10, and `_OUT_USD`, 0.40:
    /// Gemini 2.5 Flash-Lite's), for the caps.
    pub price_in_micros: u64,
    pub price_out_micros: u64,
    /// Requests one account may make per hour
    /// (`KATNA_SERVER_AI_PER_HOUR`, default 300).
    pub per_hour: u32,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            on: true,
            services: Vec::new(),
            keys: Vec::new(),
            trial_days: 30,
            account_cap_micros: 1_000_000,
            budget_micros: 50_000_000,
            price_in_micros: 100_000,
            price_out_micros: 400_000,
            per_hour: 300,
        }
    }
}

/// One AI service Katna AI asks.
#[derive(Clone, Debug)]
pub struct AiService {
    /// A `katna_ai::provider::PRESETS` id.
    pub provider: &'static str,
    /// Where its API is: the preset's, or `_BASE` (an `http://` address
    /// only on the internal network, such as Ollama beside the server).
    pub base: String,
    pub model: String,
    pub key: Secret,
}

impl AiConfig {
    /// `model` of the service `provider`, when its key is set.
    pub fn service(&self, provider: &str, model: &str) -> Option<AiService> {
        let model = model.trim();
        if model.is_empty() || model.len() > 200 || model.chars().any(char::is_control) {
            return None;
        }
        self.keys
            .iter()
            .find(|service| service.provider == provider)
            .map(|service| AiService {
                model: model.to_owned(),
                ..service.clone()
            })
    }
}

impl AiService {
    /// The kind of API it speaks.
    pub fn kind(&self) -> katna_ai::Kind {
        katna_ai::provider::preset(self.provider).kind
    }
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
            translate_url: String::new(),
            translations_per_day: 2000,
            translate_concurrency: 8,
            smtp_url: None,
            dev_mailer_log: false,
            blocked_hosts: Vec::new(),
            mail_from: "Katna <no-reply@katna.invenia.in>".into(),
            ai: AiConfig::default(),
            admin_emails: Vec::new(),
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
        if let Some(value) = lookup("KATNA_SERVER_TRANSLATE_URL") {
            let value = value.trim();
            if !value.is_empty() && !value.starts_with("http://") {
                return Err(ConfigError {
                    name: "KATNA_SERVER_TRANSLATE_URL",
                    problem: format!("{value:?}: an http:// address on the internal network"),
                });
            }
            config.translate_url = value.to_owned();
        }
        if let Some(value) = lookup("KATNA_SERVER_TRANSLATIONS_PER_DAY") {
            config.translations_per_day = parse("KATNA_SERVER_TRANSLATIONS_PER_DAY", &value)?;
        }
        if let Some(value) = lookup("KATNA_SERVER_TRANSLATE_CONCURRENCY") {
            config.translate_concurrency = parse("KATNA_SERVER_TRANSLATE_CONCURRENCY", &value)?;
        }
        if let Some(value) = lookup("KATNA_SERVER_BLOCKED_HOSTS") {
            config.blocked_hosts = value
                .split(|c: char| c == ',' || c.is_whitespace())
                .map(|host| host.trim().trim_matches('.').to_lowercase())
                .filter(|host| !host.is_empty())
                .collect();
        }
        if let Some(value) = lookup("KATNA_SERVER_DEV_MAILER") {
            config.dev_mailer_log = match value.trim() {
                "" => false,
                "log" => true,
                other => {
                    return Err(ConfigError {
                        name: "KATNA_SERVER_DEV_MAILER",
                        problem: format!("{other:?}: only \"log\" (or empty)"),
                    });
                }
            };
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
        config.ai = ai_config(&set)?;
        if let Some(value) = set("KATNA_SERVER_ADMIN_EMAILS") {
            config.admin_emails = value
                .split(|c: char| c == ',' || c.is_whitespace())
                .filter_map(crate::auth::normalize_email)
                .collect();
        }
        if let Some(value) = set("KATNA_SERVER_MAIL_FROM") {
            config.mail_from = value;
        } else if let Some(address) = username.filter(|u| u.contains('@')) {
            config.mail_from = format!("Katna <{address}>");
        }
        Ok(config)
    }
}

/// Katna AI's settings, through `set` (a setting given and not empty).
fn ai_config(set: &dyn Fn(&str) -> Option<String>) -> Result<AiConfig, ConfigError> {
    let mut ai = AiConfig::default();
    for prefix in ["KATNA_SERVER_AI", "KATNA_SERVER_AI_FALLBACK"] {
        let name = |what: &str| format!("{prefix}_{what}");
        let key = set(&name("KEY")).unwrap_or_default();
        let base = set(&name("BASE"));
        if key.is_empty() && base.is_none() {
            continue;
        }
        let provider = set(&name("PROVIDER")).unwrap_or_else(|| "gemini".into());
        let Some(preset) = katna_ai::PRESETS.iter().find(|p| p.id == provider) else {
            return Err(ConfigError {
                name: if prefix.ends_with("FALLBACK") {
                    "KATNA_SERVER_AI_FALLBACK_PROVIDER"
                } else {
                    "KATNA_SERVER_AI_PROVIDER"
                },
                problem: format!(
                    "{provider:?}: not one of gemini, openai, anthropic, mistral, deepseek, openrouter, other"
                ),
            });
        };
        let base = base.unwrap_or_else(|| preset.base.to_owned());
        if !base.starts_with("https://") && !base.starts_with("http://") {
            return Err(ConfigError {
                name: "KATNA_SERVER_AI_BASE",
                problem: format!("{base:?}: an https:// address (http:// only inside)"),
            });
        }
        let model = set(&name("MODEL")).unwrap_or_else(|| preset.model.to_owned());
        if model.is_empty() {
            return Err(ConfigError {
                name: "KATNA_SERVER_AI_MODEL",
                problem: "needed for this provider".into(),
            });
        }
        ai.services.push(AiService {
            provider: preset.id,
            base,
            model,
            key: Secret(key),
        });
    }
    // Keys for the admin page to choose from: one per service.
    for preset in katna_ai::PRESETS {
        let upper = preset.id.to_uppercase();
        let key = set(&format!("KATNA_SERVER_AI_{upper}_KEY")).unwrap_or_default();
        let base = if preset.id == katna_ai::provider::OTHER {
            match set("KATNA_SERVER_AI_OTHER_BASE") {
                Some(base) if base.starts_with("https://") || base.starts_with("http://") => base,
                Some(base) => {
                    return Err(ConfigError {
                        name: "KATNA_SERVER_AI_OTHER_BASE",
                        problem: format!("{base:?}: an https:// address (http:// only inside)"),
                    });
                }
                None => continue,
            }
        } else if key.is_empty() {
            continue;
        } else {
            preset.base.to_owned()
        };
        ai.keys.push(AiService {
            provider: preset.id,
            base,
            model: preset.model.to_owned(),
            key: Secret(key),
        });
    }
    for service in &ai.services {
        if !ai
            .keys
            .iter()
            .any(|known| known.provider == service.provider)
        {
            ai.keys.push(service.clone());
        }
    }
    let usd = |name: &'static str, default: u64| -> Result<u64, ConfigError> {
        match set(name) {
            None => Ok(default),
            Some(value) => {
                let dollars: f64 = parse(name, &value)?;
                if !(0.0..1e9).contains(&dollars) {
                    return Err(ConfigError {
                        name,
                        problem: format!("{value:?}: dollars, such as 1.50"),
                    });
                }
                Ok((dollars * 1e6).round() as u64)
            }
        }
    };
    ai.account_cap_micros = usd("KATNA_SERVER_AI_ACCOUNT_CAP_USD", ai.account_cap_micros)?;
    ai.budget_micros = usd("KATNA_SERVER_AI_BUDGET_USD", ai.budget_micros)?;
    ai.price_in_micros = usd("KATNA_SERVER_AI_PRICE_IN_USD", ai.price_in_micros)?;
    ai.price_out_micros = usd("KATNA_SERVER_AI_PRICE_OUT_USD", ai.price_out_micros)?;
    if let Some(value) = set("KATNA_SERVER_AI_TRIAL_DAYS") {
        ai.trial_days = parse("KATNA_SERVER_AI_TRIAL_DAYS", &value)?;
    }
    if let Some(value) = set("KATNA_SERVER_AI_PER_HOUR") {
        ai.per_hour = parse("KATNA_SERVER_AI_PER_HOUR", &value)?;
    }
    Ok(ai)
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
            ("KATNA_SERVER_TRANSLATE_URL", "http://translate:5000"),
        ]);
        let config = Config::from_lookup(|name| env.get(name).map(|v| v.to_string())).unwrap();
        assert_eq!(config.listen.port(), 9000);
        assert!(config.trust_forwarded);
        assert_eq!(config.retention_days, 30);
        assert_eq!(config.daily_limit, 5000);
        assert_eq!(config.translate_url, "http://translate:5000");
        assert_eq!(config.translations_per_day, 2000);
        assert!(config.smtp_url.is_none());
        assert!(!config.dev_mailer_log);
        assert!(config.blocked_hosts.is_empty());
    }

    #[test]
    fn reads_blocked_hosts_and_the_dev_mailer() {
        let env: HashMap<&str, &str> = HashMap::from([
            ("DATABASE_URL", "postgres://x"),
            (
                "KATNA_SERVER_BLOCKED_HOSTS",
                " Evil.example, .bad.test  phish.io.",
            ),
            ("KATNA_SERVER_DEV_MAILER", "log"),
            ("KATNA_SERVER_TRANSLATE_CONCURRENCY", "2"),
        ]);
        let config = Config::from_lookup(|name| env.get(name).map(|v| v.to_string())).unwrap();
        assert_eq!(
            config.blocked_hosts,
            ["evil.example", "bad.test", "phish.io"]
        );
        assert!(config.dev_mailer_log);
        assert_eq!(config.translate_concurrency, 2);
        let bad = Config::from_lookup(|name| match name {
            "DATABASE_URL" => Some("postgres://x".into()),
            "KATNA_SERVER_DEV_MAILER" => Some("yes".into()),
            _ => None,
        });
        assert!(bad.is_err());
    }

    #[test]
    fn reads_katna_ai() {
        let env: HashMap<&str, &str> = HashMap::from([
            ("DATABASE_URL", "postgres://x"),
            ("KATNA_SERVER_AI_KEY", "g-secret"),
            ("KATNA_SERVER_AI_FALLBACK_PROVIDER", "mistral"),
            ("KATNA_SERVER_AI_FALLBACK_KEY", "m-secret"),
            ("KATNA_SERVER_AI_BUDGET_USD", "12.5"),
            ("KATNA_SERVER_AI_OPENROUTER_KEY", "o-secret"),
            ("KATNA_SERVER_ADMIN_EMAILS", "Mz@Invenia.in, nope"),
        ]);
        let config = Config::from_lookup(|name| env.get(name).map(|v| v.to_string())).unwrap();
        let ai = &config.ai;
        assert_eq!(ai.services.len(), 2);
        assert_eq!(ai.services[0].provider, "gemini");
        assert_eq!(ai.services[0].model, "gemini-2.5-flash-lite");
        assert_eq!(ai.services[1].base, "https://api.mistral.ai/v1");
        assert_eq!(ai.budget_micros, 12_500_000);
        assert_eq!(ai.trial_days, 30);
        assert_eq!(config.admin_emails, ["mz@invenia.in"]);
        let providers: Vec<_> = ai.keys.iter().map(|s| s.provider).collect();
        assert_eq!(providers, ["openrouter", "gemini", "mistral"]);
        assert!(!format!("{config:?}").contains("secret"));
        // The two services' keys can be chosen on the admin page.
        let chosen = ai.service("mistral", "mistral-large-latest").unwrap();
        assert_eq!(chosen.key.0, "m-secret");
        assert_eq!(chosen.model, "mistral-large-latest");
        assert!(ai.service("openai", "gpt-5-mini").is_none());
        // No key: Katna AI is off.
        let off = Config::from_lookup(|name| (name == "DATABASE_URL").then(|| "x".into())).unwrap();
        assert!(off.ai.services.is_empty());
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
