// SPDX-License-Identifier: GPL-3.0-or-later

//! Read-only check of attachment lists on a real account: fetches the
//! headers and `BODYSTRUCTURE` of the newest messages of one folder, as
//! sync does, and counts how many came with a readable attachment list.
//! It never appends, sends or changes flags.
//!
//! ```sh
//! KATNA_IMAP=imap.gmail.com:993:tls KATNA_USER=you@gmail.com \
//! KATNA_PASSWORD='<app password>' KATNA_FOLDER=INBOX KATNA_COUNT=200 \
//!   cargo run --release -p katna-sync --example attachment_check
//! ```
//!
//! `KATNA_IMAP` is `host:port:tls|starttls|plain`. `KATNA_INSECURE=1`
//! accepts any certificate, for the self-signed `dev/` servers only.
//! Prints counts and MIME types only, never names, addresses or text.

use std::{collections::BTreeMap, env, process::ExitCode};

use katna_sync::{
    Credentials, Endpoint, MailBackend, Result, Security, imap::ImapBackend, net::Tls,
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
    let creds = Credentials::new(var("KATNA_USER"), var("KATNA_PASSWORD"));
    let path = env::var("KATNA_FOLDER").unwrap_or_else(|_| "INBOX".into());
    let count: u32 = env::var("KATNA_COUNT")
        .ok()
        .and_then(|c| c.parse().ok())
        .unwrap_or(200);
    let tls = match env::var("KATNA_INSECURE").as_deref() {
        Ok("1") => Tls::insecure_for_local_tests(),
        _ => Tls::system()?,
    };
    let mut imap = ImapBackend::connect(&endpoint(), &creds, tls).await?;
    let status = imap.select(&path).await?;
    let next = status.uid_next.unwrap_or(1);
    let first = next.saturating_sub(count).max(1);
    let messages = imap.fetch_headers(first, None).await?;
    imap.logout().await?;

    let (mut none, mut empty, mut listed, mut named, mut unnamed) = (0, 0, 0, 0, 0);
    let mut mixed_without = 0;
    let mut types: BTreeMap<String, usize> = BTreeMap::new();
    for message in &messages {
        let mixed = String::from_utf8_lossy(&message.header)
            .to_ascii_lowercase()
            .lines()
            .any(|l| l.starts_with("content-type:") && l.contains("multipart/mixed"));
        match &message.attachments {
            None => {
                none += 1;
                if mixed {
                    mixed_without += 1;
                }
            }
            Some(parts) if parts.is_empty() => {
                empty += 1;
                if mixed {
                    mixed_without += 1;
                }
            }
            Some(parts) => {
                listed += 1;
                for part in parts {
                    *types.entry(part.mime.clone()).or_default() += 1;
                    if part
                        .filename
                        .as_deref()
                        .is_some_and(|n| !n.trim().is_empty())
                    {
                        named += 1;
                    } else {
                        unnamed += 1;
                    }
                }
            }
        }
    }
    println!("{path}: {} messages from UID {first}", messages.len());
    println!("  no BODYSTRUCTURE read: {none}");
    println!("  read, no attachments: {empty}");
    println!("  read, with attachments: {listed} ({named} named, {unnamed} without a name)");
    println!("  multipart/mixed but no attachment listed: {mixed_without}");
    for (mime, n) in types {
        println!("    {mime}: {n}");
    }
    Ok(())
}
