// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna background service. See `docs/ARCHITECTURE.md` §9.

// No console window on Windows.
#![cfg_attr(all(windows, not(test)), windows_subsystem = "windows")]

use std::process::ExitCode;

use futures_lite::{FutureExt, StreamExt};
use katna_core::health::Health;
use katna_core::{Config, Paths};
use katna_daemon::{Ended, Instance, install, secrets::Secrets, update};
use katna_sync::worker::WorkerConfig;

const USAGE: &str = "\
usage: katna-daemon
       katna-daemon install-user-service

Without arguments, runs the Katna background service on the session bus.
Normally systemd or D-Bus activation starts it; `katnactl` talks to it.

install-user-service: Writes a systemd user unit and a D-Bus activation
file for this binary under ~/.config and ~/.local/share. Then:
  systemctl --user daemon-reload
  systemctl --user enable --now katna-daemon.service

The log filter comes from KATNA_LOG or [logging] filter in config.toml.";

/// The daemon's translations, embedded by `build.rs`.
const TRANSLATIONS: katna_i18n::Sources = include!(concat!(env!("OUT_DIR"), "/translations.rs"));

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.iter().map(String::as_str).collect::<Vec<_>>()[..] {
        [] => run(),
        ["install-user-service"] => install_user_service(),
        ["--version"] => {
            println!("katna-daemon {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        ["-h" | "--help"] => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
    }
}

fn run() -> ExitCode {
    let paths = match Paths::from_env() {
        Ok(paths) => paths,
        Err(err) => return fail(err),
    };
    katna_core::crash::install("katna-daemon", &paths);
    let config = match Config::load(&paths.config_file()) {
        Ok(config) => config,
        Err(err) => return fail(err),
    };
    if let Err(err) = katna_core::logging::init(&config.logging.filter) {
        return fail(err);
    }
    // The language of notifications and the tray (§13.10); Katna Mail's
    // setting, applied again when it changes (`Daemon::reload_config`).
    katna_i18n::init(TRANSLATIONS, Some(paths.data_dir().join("i18n")));
    katna_i18n::apply(&config.general.language);
    smol::block_on(async {
        // Windows has only Ctrl+C; the tray, Setup and logout stop it there.
        #[cfg(unix)]
        let stop_signals = [async_signal::Signal::Term, async_signal::Signal::Int];
        #[cfg(windows)]
        let stop_signals = [async_signal::Signal::Int];
        let mut signals = match async_signal::Signals::new(stop_signals) {
            Ok(signals) => signals,
            Err(err) => return fail(format!("signal handlers: {err}")),
        };
        let connection = match katna_dbus::session().await {
            Ok(connection) => connection,
            Err(err) => return fail(format!("session bus: {err}")),
        };
        let secrets = match Secrets::keyring().await {
            Ok(secrets) => secrets,
            Err(err) => return fail(err),
        };
        let bus = connection.clone();
        let health_file = paths.health_file();
        let started_at = unix_now();
        let mut health = Health::load(&health_file);
        if health.begin_start(started_at) {
            tracing::warn!(
                starts = health.starts.len() - 1,
                "the last starts never became healthy; safe mode"
            );
        }
        save_health(&health, &health_file);
        let instance =
            match Instance::start(paths, secrets, WorkerConfig::default(), connection).await {
                Ok(instance) => instance,
                // Not a failure: the service runs. Exiting cleanly keeps
                // systemd from starting this one again every few seconds.
                Err(err @ katna_daemon::StartError::AlreadyRunning) => {
                    eprintln!("katna-daemon: {err}");
                    // Nor a failed start: the running copy owns the file.
                    let mut health = Health::load(&health_file);
                    health.starts.retain(|&at| at != started_at);
                    save_health(&health, &health_file);
                    return ExitCode::SUCCESS;
                }
                Err(err) => return fail(err),
            };
        if health.needs_check(katna_core::crash::VERSION) {
            let checks = instance.self_check();
            health.checked(katna_core::crash::VERSION, unix_now(), checks);
            if health.healthy {
                tracing::info!("self-check passed");
            } else {
                tracing::warn!(checks = ?health.checks, "self-check failed");
            }
        }
        if health.healthy {
            health.reached_healthy();
        }
        save_health(&health, &health_file);
        match zbus::Connection::system().await {
            Ok(system) => instance.watch_system(system),
            Err(err) => tracing::warn!(%err, "no system bus; not watching suspend and network"),
        }
        tracing::info!("katna-daemon running");
        // Set when a package update replaced this binary.
        let updated = std::cell::OnceCell::new();
        let stop = async {
            let signal = signals.next().await;
            tracing::info!(?signal, "stopping");
        }
        .or(async {
            let binary = update::replaced().await;
            if update::restart_by_systemd(&bus).await {
                // Its SIGTERM stops this one above.
                std::future::pending::<()>().await;
            }
            let _ = updated.set(binary);
        });
        let ended = instance.serve(stop).await;
        if let (Ended::Stopped, Some(binary)) = (ended, updated.get()) {
            tracing::info!("starting the updated katna-daemon");
            // Only returns on failure; systemd then starts it again.
            return fail(format!(
                "starting {}: {}",
                binary.display(),
                update::restart(binary)
            ));
        }
        ExitCode::SUCCESS
    })
}

/// Writes the health file; a failure is logged, never fatal.
fn save_health(health: &Health, path: &std::path::Path) {
    if let Err(err) = health.save(path) {
        tracing::warn!(%err, "could not write the health file");
    }
}

/// The current time in Unix seconds.
fn unix_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| i64::try_from(d.as_secs()).unwrap_or(i64::MAX))
}

fn install_user_service() -> ExitCode {
    let exe = match std::env::current_exe() {
        Ok(exe) => exe,
        Err(err) => return fail(format!("finding this binary: {err}")),
    };
    let paths = match Paths::from_env() {
        Ok(paths) => paths,
        Err(err) => return fail(err),
    };
    // Paths::config_dir() is ~/.config/katna; the unit goes in ~/.config.
    let (Some(config_home), Some(data_home)) =
        (paths.config_dir().parent(), paths.data_dir().parent())
    else {
        return fail("could not find the XDG directories");
    };
    match install::install(&exe, config_home, data_home) {
        Ok(written) => {
            for path in written {
                println!("wrote {}", path.display());
            }
            println!(
                "now run: systemctl --user daemon-reload && systemctl --user enable --now {}",
                install::UNIT
            );
            ExitCode::SUCCESS
        }
        Err(err) => fail(format!("writing the service files: {err}")),
    }
}

fn fail(err: impl std::fmt::Display) -> ExitCode {
    eprintln!("katna-daemon: {err}");
    ExitCode::FAILURE
}
