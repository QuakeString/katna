// SPDX-License-Identifier: GPL-3.0-or-later

//! Read-only check of conversations and inbox tabs on a real account: syncs
//! one folder into a throwaway store and compares the store's conversations
//! with the server's. On Gmail every conversation must match one
//! `X-GM-THRID`. It never appends, sends or changes flags.
//!
//! ```sh
//! KATNA_IMAP=imap.gmail.com:993:tls KATNA_USER=you@gmail.com \
//! KATNA_PASSWORD='<app password>' KATNA_FOLDER=INBOX \
//!   cargo run --release -p katna-sync --example thread_check
//! ```
//!
//! `KATNA_IMAP` is `host:port:tls|starttls|plain`. `KATNA_INSECURE=1`
//! accepts any certificate, for the self-signed `dev/` servers only.
//! Prints counts and subjects only, never addresses or message text.

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    env,
    process::ExitCode,
};

use katna_core::{AccountKind, MailCategory, Paths};
use katna_store::{Mode, Store};
use katna_sync::{
    Credentials, Endpoint, MailBackend, Result, Security, engine, imap::ImapBackend, net::Tls,
};

fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    match smol::block_on(run()) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => {
            eprintln!("MISMATCH: see above");
            ExitCode::FAILURE
        }
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

async fn run() -> Result<bool> {
    let creds = Credentials::new(var("KATNA_USER"), var("KATNA_PASSWORD"));
    let path = env::var("KATNA_FOLDER").unwrap_or_else(|_| "INBOX".into());
    let tls = match env::var("KATNA_INSECURE").as_deref() {
        Ok("1") => Tls::insecure_for_local_tests(),
        _ => Tls::system()?,
    };
    let tmp = tempfile::tempdir()?;
    let mut store = Store::open(&Paths::with_root(tmp.path()), Mode::ReadWrite)?;
    let account = store.add_account(AccountKind::Imap, "check", "check")?.id;

    let mut imap = ImapBackend::connect(&endpoint(), &creds, tls).await?;
    let gmail = imap.capabilities().iter().any(|c| c == "X-GM-EXT-1");
    let folders = engine::sync_folders(&mut imap, &mut store, account).await?;
    let (_, folder) = folders
        .iter()
        .find(|(f, _)| f.name == path)
        .unwrap_or_else(|| panic!("no folder {path}"));
    let report = engine::sync_folder(&mut imap, &mut store, account, *folder, &path).await?;
    println!("{path}: {} messages synced (Gmail: {gmail})", report.added);

    // The server's view: UID to X-GM-THRID.
    imap.select(&path).await?;
    let server: HashMap<u32, Option<u64>> = imap
        .fetch_headers(1, None)
        .await?
        .into_iter()
        .map(|h| (h.uid, h.gm_thread_id))
        .collect();
    imap.logout().await?;

    let entries = store.folder_threads(*folder, None)?;
    let threads: Vec<_> = entries.iter().filter_map(|e| e.thread).collect();
    let summaries = store.thread_summaries(&threads, *folder)?;
    let longest = summaries.iter().map(|s| s.message_count).max().unwrap_or(0);
    println!(
        "conversations: {} ({} without a thread yet), {} with more than one message, longest {longest}",
        entries.len(),
        entries.len() - threads.len(),
        summaries.iter().filter(|s| s.message_count > 1).count(),
    );
    for category in MailCategory::ALL {
        let tab = store.folder_threads(*folder, Some(category))?.len();
        println!("  tab {category:?}: {tab} conversations");
    }
    for (category, unread) in store.category_unread(*folder)? {
        if unread > 0 {
            println!("  unread in {category:?}: {unread}");
        }
    }

    if !gmail {
        return Ok(true);
    }
    // Every store thread must hold exactly one X-GM-THRID, and every
    // X-GM-THRID exactly one store thread.
    let mut by_thread: BTreeMap<i64, HashSet<u64>> = BTreeMap::new();
    let mut by_gm: BTreeMap<u64, HashSet<i64>> = BTreeMap::new();
    let mut missing = 0;
    for message in store.messages_in_folder(*folder)? {
        let Some((_, _, uid)) = store.remote_location(message.id)? else {
            continue;
        };
        let (Some(thread), Some(Some(gm))) = (message.thread_id, server.get(&uid)) else {
            missing += 1;
            continue;
        };
        by_thread.entry(thread.0).or_default().insert(*gm);
        by_gm.entry(*gm).or_default().insert(thread.0);
    }
    let merged = by_thread.values().filter(|gms| gms.len() > 1).count();
    let split = by_gm.values().filter(|threads| threads.len() > 1).count();
    println!(
        "X-GM-THRID: {} on the server, {} store threads; {merged} threads merge several, \
         {split} are split, {missing} messages without either",
        by_gm.len(),
        by_thread.len(),
    );
    Ok(merged == 0 && split == 0 && missing == 0)
}
