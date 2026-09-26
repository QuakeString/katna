// SPDX-License-Identifier: GPL-3.0-or-later

//! Logging setup with `tracing`.
//!
//! Programs call [`init`] once at startup with the filter from the
//! configuration. `$KATNA_LOG`, when set, overrides it, so a user can turn on
//! debug output without editing the settings file. Output goes to stderr,
//! which systemd sends to the journal.

use tracing_subscriber::EnvFilter;

use crate::error::{Error, Result};

/// Environment variable that overrides the configured log filter.
pub const LOG_ENV: &str = "KATNA_LOG";

/// Installs the global `tracing` subscriber.
///
/// `config_filter` is a `tracing` filter such as `info` or
/// `warn,katna_sync=debug` (normally [`crate::config::Logging::filter`]).
/// Fails if a filter is invalid or a subscriber is already installed.
pub fn init(config_filter: &str) -> Result<()> {
    let filter = build_filter(config_filter, std::env::var(LOG_ENV).ok().as_deref())?;
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .try_init()
        .map_err(|err| Error::Logging(err.to_string()))
}

/// Chooses the filter: `env_filter` if set and not empty, else `config_filter`.
fn build_filter(config_filter: &str, env_filter: Option<&str>) -> Result<EnvFilter> {
    let (source, directives) = match env_filter.map(str::trim) {
        Some(env) if !env.is_empty() => (LOG_ENV, env),
        _ => ("logging.filter", config_filter),
    };
    EnvFilter::builder()
        .parse(directives)
        .map_err(|err| Error::Logging(format!("{source} = {directives:?}: {err}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_config_filter_without_env() {
        let filter = build_filter("warn,katna_sync=debug", None).unwrap();
        assert_eq!(filter.to_string(), "katna_sync=debug,warn");
    }

    #[test]
    fn env_overrides_config() {
        let filter = build_filter("warn", Some("trace")).unwrap();
        assert_eq!(filter.to_string(), "trace");
        let filter = build_filter("warn", Some("  ")).unwrap();
        assert_eq!(filter.to_string(), "warn");
    }

    #[test]
    fn rejects_invalid_filters() {
        let err = build_filter("info", Some("katna=nonsense")).unwrap_err();
        assert!(err.to_string().contains(LOG_ENV), "{err}");
        assert!(build_filter("[", None).is_err());
    }

    #[test]
    fn init_twice_fails_cleanly() {
        // The first call may fail if another test installed a subscriber
        // first; the second call must always fail without panicking.
        let _ = init("info");
        assert!(matches!(init("info"), Err(Error::Logging(_))));
    }
}
