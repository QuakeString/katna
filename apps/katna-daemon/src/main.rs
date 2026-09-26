// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna background service. See `docs/ARCHITECTURE.md` §9.

use std::process::ExitCode;

use futures_lite::StreamExt;
use katna_core::{Config, Paths};
use katna_daemon::{Instance, install, secrets::Secrets};
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
    let config = match Config::load(&paths.config_file()) {
        Ok(config) => config,
        Err(err) => return fail(err),
    };
    if let Err(err) = katna_core::logging::init(&config.logging.filter) {
        return fail(err);
    }
    smol::block_on(async {
        let mut signals = match async_signal::Signals::new([
            async_signal::Signal::Term,
            async_signal::Signal::Int,
        ]) {
            Ok(signals) => signals,
            Err(err) => return fail(format!("signal handlers: {err}")),
        };
        let connection = match zbus::Connection::session().await {
            Ok(connection) => connection,
            Err(err) => return fail(format!("session bus: {err}")),
        };
        let secrets = match Secrets::keyring().await {
            Ok(secrets) => secrets,
            Err(err) => return fail(err),
        };
        let instance =
            match Instance::start(paths, secrets, WorkerConfig::default(), connection).await {
                Ok(instance) => instance,
                Err(err) => return fail(err),
            };
        match zbus::Connection::system().await {
            Ok(system) => instance.watch_system(system),
            Err(err) => tracing::warn!(%err, "no system bus; not watching suspend and network"),
        }
        tracing::info!("katna-daemon running");
        let signal = signals.next().await;
        tracing::info!(?signal, "stopping");
        instance.shutdown().await;
        ExitCode::SUCCESS
    })
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
