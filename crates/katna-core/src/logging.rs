// SPDX-License-Identifier: GPL-3.0-or-later

//! Logging setup with `tracing`.
//!
//! Programs call [`init`] once at startup with the filter from the
//! configuration. `$KATNA_LOG`, when set, overrides it, so a user can turn on
//! debug output without editing the settings file. Output goes to stderr,
//! which systemd sends to the journal. The last [`RECENT`] lines are also
//! kept in memory for crash reports ([`recent_lines`]).

use std::collections::VecDeque;
use std::io::{self, Write};
use std::sync::Mutex;

use tracing_subscriber::EnvFilter;

use crate::error::{Error, Result};

/// Environment variable that overrides the configured log filter.
pub const LOG_ENV: &str = "KATNA_LOG";

/// Log lines kept in memory for crash reports.
pub const RECENT: usize = 50;

static RECENT_LINES: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());

/// The last [`RECENT`] lines logged by this process, oldest first, without
/// terminal colors.
pub fn recent_lines() -> Vec<String> {
    // A panic while logging may have poisoned the lock; the lines are
    // still good.
    let lines = RECENT_LINES.lock().unwrap_or_else(|err| err.into_inner());
    lines.iter().cloned().collect()
}

/// Writes one log event to stderr and keeps its lines in [`RECENT_LINES`].
#[derive(Default)]
struct Tee {
    event: Vec<u8>,
}

impl Write for Tee {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.event.extend_from_slice(buf);
        io::stderr().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        io::stderr().flush()
    }
}

impl Drop for Tee {
    fn drop(&mut self) {
        let text = without_colors(&String::from_utf8_lossy(&self.event));
        // A line logged while another is being kept is left out rather than
        // waited for. Tests wait, so parallel tests logging never drop the
        // lines one of them checks.
        #[cfg(not(test))]
        let Ok(mut lines) = RECENT_LINES.try_lock() else {
            return;
        };
        #[cfg(test)]
        let mut lines = RECENT_LINES.lock().unwrap_or_else(|err| err.into_inner());
        for line in text.lines().filter(|line| !line.trim().is_empty()) {
            if lines.len() == RECENT {
                lines.pop_front();
            }
            lines.push_back(line.to_string());
        }
    }
}

/// `text` without ANSI escape sequences (`ESC [ … letter`).
fn without_colors(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c == '\u{1b}' {
            if chars.next() == Some('[') {
                for c in chars.by_ref() {
                    if c.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// Installs the global `tracing` subscriber.
///
/// `config_filter` is a `tracing` filter such as `info` or
/// `warn,katna_sync=debug` (normally [`crate::config::Logging::filter`]).
/// Fails if a filter is invalid or a subscriber is already installed.
pub fn init(config_filter: &str) -> Result<()> {
    let filter = build_filter(config_filter, std::env::var(LOG_ENV).ok().as_deref())?;
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(Tee::default)
        .try_init()
        .map_err(|err| Error::Logging(err.to_string()))
}

/// Libraries whose warnings are about the user's system or a server rather
/// than Katna, such as one line per broken font file, or one per slightly
/// malformed IMAP response Gmail sends ("Rectified missing `text`"). The
/// config filter hides them unless it names them; `$KATNA_LOG` shows them
/// as asked.
const QUIET: &[&str] = &["fontdb", "imap_codec"];

/// Chooses the filter: `env_filter` if set and not empty, else `config_filter`
/// with [`QUIET`] libraries kept to errors.
fn build_filter(config_filter: &str, env_filter: Option<&str>) -> Result<EnvFilter> {
    let (source, directives) = match env_filter.map(str::trim) {
        Some(env) if !env.is_empty() => (LOG_ENV, env.to_owned()),
        _ => {
            let mut directives = config_filter.trim().to_owned();
            for target in QUIET {
                if !directives.contains(target) {
                    directives.push_str(&format!(",{target}=error"));
                }
            }
            ("logging.filter", directives)
        }
    };
    EnvFilter::builder()
        .parse(&directives)
        .map_err(|err| Error::Logging(format!("{source} = {directives:?}: {err}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_config_filter_without_env() {
        let filter = build_filter("warn,katna_sync=debug", None).unwrap();
        assert_eq!(
            filter.to_string(),
            "katna_sync=debug,imap_codec=error,fontdb=error,warn"
        );
    }

    #[test]
    fn keeps_font_complaints_quiet_unless_asked() {
        let filter = build_filter("info,fontdb=debug", None).unwrap();
        assert_eq!(filter.to_string(), "imap_codec=error,fontdb=debug,info");
        let filter = build_filter("info", Some("debug")).unwrap();
        assert_eq!(filter.to_string(), "debug");
    }

    #[test]
    fn env_overrides_config() {
        let filter = build_filter("warn", Some("trace")).unwrap();
        assert_eq!(filter.to_string(), "trace");
        let filter = build_filter("warn", Some("  ")).unwrap();
        assert_eq!(filter.to_string(), "imap_codec=error,fontdb=error,warn");
    }

    #[test]
    fn rejects_invalid_filters() {
        let err = build_filter("info", Some("katna=nonsense")).unwrap_err();
        assert!(err.to_string().contains(LOG_ENV), "{err}");
        assert!(build_filter("[", None).is_err());
    }

    #[test]
    fn keeps_recent_lines_without_colors() {
        assert_eq!(
            without_colors("\u{1b}[2mtime\u{1b}[0m \u{1b}[33mWARN\u{1b}[0m x"),
            "time WARN x"
        );
        for i in 0..RECENT + 5 {
            let mut tee = Tee::default();
            writeln!(tee, "\u{1b}[32mrecent line {i}\u{1b}[0m").unwrap();
        }
        let lines = recent_lines();
        assert_eq!(lines.len(), RECENT);
        // Tests running alongside may log between these lines.
        let ours: Vec<&String> = lines
            .iter()
            .filter(|l| l.starts_with("recent line "))
            .collect();
        assert_eq!(
            ours.last().unwrap().as_str(),
            format!("recent line {}", RECENT + 4)
        );
        assert!(ours.iter().all(|l| !l.contains('\u{1b}')));
    }

    #[test]
    fn init_twice_fails_cleanly() {
        // The first call may fail if another test installed a subscriber
        // first; the second call must always fail without panicking.
        let _ = init("info");
        assert!(matches!(init("info"), Err(Error::Logging(_))));
    }
}
