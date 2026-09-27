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
        Ok(config)
    }
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
