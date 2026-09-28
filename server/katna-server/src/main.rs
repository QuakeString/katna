// SPDX-License-Identifier: GPL-3.0-or-later

//! `katna-server`: see the library documentation and
//! `server/katna-server/README.md`.

use std::net::SocketAddr;
use std::process::ExitCode;
use std::time::Duration;

use katna_server::db::{Db, now_ms};
use katna_server::{AppState, Config, router};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            tracing::error!("{error}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;
    // Without an SMTP relay this is an error, unless KATNA_SERVER_DEV_MAILER
    // asks for the log.
    let mailer = katna_server::mailer::Mailer::from_config(&config)?;
    if !mailer.sends_mail() {
        tracing::warn!(
            "KATNA_SERVER_DEV_MAILER=log: account codes go to this log only; \
             never use this where others can sign up"
        );
    }
    let db = Db::connect(&config.database_url)?;
    // PostgreSQL may still be starting next to us.
    let mut attempt = 0;
    loop {
        match db.migrate().await {
            Ok(()) => break,
            Err(error) if attempt < 30 => {
                attempt += 1;
                tracing::warn!(%error, "database not ready, trying again");
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
            Err(error) => return Err(error.into()),
        }
    }
    tokio::spawn(purge(db.clone(), config.retention_days));

    let listener = tokio::net::TcpListener::bind(config.listen).await?;
    tracing::info!(address = %config.listen, "listening");
    let app = router(AppState::with_mailer(db, config, mailer))
        .into_make_service_with_connect_info::<SocketAddr>();
    // Event streams never end on their own, so shutdown waits for them only
    // briefly.
    let server = axum::serve(listener, app).with_graceful_shutdown(shutdown_signal());
    tokio::select! {
        result = server => result?,
        () = async {
            shutdown_signal().await;
            tokio::time::sleep(Duration::from_secs(5)).await;
        } => {}
    }
    Ok(())
}

/// Deletes old tracking IDs, events, unused installs and unconfirmed
/// accounts every hour.
async fn purge(db: Db, retention_days: u32) {
    let retention = i64::from(retention_days) * 86_400_000;
    loop {
        // Accounts whose address was never confirmed go after a week.
        match db.purge_accounts(now_ms() - 7 * 86_400_000, now_ms()).await {
            Ok(accounts) if accounts > 0 => {
                tracing::info!(accounts, "deleted unconfirmed accounts")
            }
            Ok(_) => {}
            Err(error) => tracing::warn!(%error, "could not delete unconfirmed accounts"),
        }
        match db.purge(now_ms() - retention).await {
            Ok((tracks, installs)) if tracks + installs > 0 => {
                tracing::info!(tracks, installs, "deleted old records");
            }
            Ok(_) => {}
            Err(error) => tracing::warn!(%error, "could not delete old records"),
        }
        tokio::time::sleep(Duration::from_secs(3600)).await;
    }
}

async fn shutdown_signal() {
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut signal) => {
                signal.recv().await;
            }
            Err(_) => std::future::pending().await,
        }
    };
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {}
        () = terminate => {}
    }
}
