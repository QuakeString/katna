// SPDX-License-Identifier: GPL-3.0-or-later

//! Read-only check of a real IMAP account: log in, list folders, fetch
//! envelopes and wait for new mail. It never appends, sends or changes
//! flags.
//!
//! ```sh
//! KATNA_IMAP=imap.gmail.com:993:tls KATNA_USER=you@gmail.com \
//! KATNA_PASSWORD='<app password>' KATNA_FOLDER=INBOX KATNA_WAIT=120 \
//!   cargo run --release -p katna-sync --example imap_check
//! ```
//!
//! `KATNA_IMAP` is `host:port:tls|starttls|plain`. `KATNA_INSECURE=1`
//! accepts any certificate, for the self-signed `dev/` servers only.
//! Send yourself a mail during the wait to see push working.

use std::{
    env,
    process::ExitCode,
    time::{Duration, Instant},
};

use katna_sync::{
    Credentials, Endpoint, MailBackend, Result, Security, connection, imap::ImapBackend, net::Tls,
};

fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    match smol::block_on(run()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("FAILED: {e}");
            ExitCode::FAILURE
        }
    }
}

fn var(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("set {name}"))
}

fn endpoint() -> Endpoint {
    let spec = var("KATNA_IMAP");
    let mut parts = spec.split(':');
    let (Some(host), Some(port), security) = (parts.next(), parts.next(), parts.next()) else {
        panic!("KATNA_IMAP is host:port:tls|starttls|plain");
    };
    let security = match security.unwrap_or("tls") {
        "tls" => Security::Tls,
        "starttls" => Security::StartTls,
        "plain" => Security::Plain,
        other => panic!("unknown security {other:?}"),
    };
    Endpoint::new(host, port.parse().expect("port"), security)
}

async fn run() -> Result<()> {
    let endpoint = endpoint();
    let creds = Credentials::new(var("KATNA_USER"), var("KATNA_PASSWORD"));
    let folder = env::var("KATNA_FOLDER").unwrap_or_else(|_| "INBOX".into());
    let wait = Duration::from_secs(
        env::var("KATNA_WAIT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(60),
    );
    let tls = match env::var("KATNA_INSECURE").as_deref() {
        Ok("1") => Tls::insecure_for_local_tests(),
        _ => Tls::system()?,
    };

    let started = Instant::now();
    let mut imap = ImapBackend::connect(&endpoint, &creds, tls).await?;
    println!("login: {:?}", started.elapsed());
    println!("capabilities: {}", imap.capabilities().join(" "));

    let started = Instant::now();
    let folders = imap.list_folders().await?;
    println!("{} folders in {:?}:", folders.len(), started.elapsed());
    for f in &folders {
        println!("  {:<30} {:?}", f.name, f.role);
    }

    let status = imap.select(&folder).await?;
    println!("{folder}: {status:?}");
    let started = Instant::now();
    let envelopes = imap.fetch_envelopes(1, None).await?;
    println!("{} envelopes in {:?}", envelopes.len(), started.elapsed());
    for e in envelopes.iter().rev().take(3) {
        println!("  uid {} {:?}", e.uid, e.subject);
    }

    let (conn, task) = connection::spawn(imap);
    smol::spawn(task).detach();
    println!("waiting up to {wait:?} for changes to {folder}…");
    let started = Instant::now();
    let changes = conn.wait_for_changes(wait).await?;
    println!("after {:?}: {changes:?}", started.elapsed());
    conn.logout().await
}
