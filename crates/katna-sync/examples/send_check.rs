// SPDX-License-Identifier: GPL-3.0-or-later

//! Sending and push check on a real account: runs the account worker on a
//! throwaway store, sends ONE message from the account to itself over SMTP,
//! and reports how long it took to show up in the inbox (IMAP IDLE, not a
//! periodic sync) and whether the server filed it in Sent by itself (Gmail
//! does). Exits non-zero if the message is not in the inbox within
//! [`INBOX_WITHIN`]. It sends nothing to anyone else and changes nothing
//! else on the account.
//!
//! ```sh
//! KATNA_IMAP=imap.gmail.com:993:tls KATNA_SMTP=smtp.gmail.com:465:tls \
//! KATNA_USER=you@gmail.com KATNA_PASSWORD='<app password>' \
//!   cargo run --release -p katna-sync --example send_check
//! ```
//!
//! Endpoints are `host:port:tls|starttls|plain`. `KATNA_INSECURE=1` accepts
//! any certificate, for the self-signed `dev/` servers only. Prints times,
//! folder names and the test subject only.

use std::{
    env,
    process::ExitCode,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use async_io::Timer;
use futures_lite::FutureExt;
use katna_core::{AccountId, AccountKind, Paths};
use katna_store::{FolderRole, Mode, Store};
use katna_sync::{
    Credentials, Endpoint, Error, MailSender, Result, Security,
    net::Tls,
    smtp::SmtpSender,
    worker::{self, Event, ImapConnector, WorkerConfig},
};

/// Longest wait for the test message in the inbox. IDLE should take
/// seconds; the next full sync would take 15 minutes.
const INBOX_WITHIN: Duration = Duration::from_secs(60);
/// Longest wait for the server's own copy in Sent.
const SENT_WITHIN: Duration = Duration::from_secs(150);

fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    match smol::block_on(run()) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("FAILED: {e}");
            ExitCode::FAILURE
        }
    }
}

fn var(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("set {name}"))
}

fn endpoint(name: &str) -> Endpoint {
    let spec = var(name);
    let mut parts = spec.split(':');
    let (Some(host), Some(port), security) = (parts.next(), parts.next(), parts.next()) else {
        panic!("{name} is host:port:tls|starttls|plain");
    };
    let security = match security.unwrap_or("tls") {
        "tls" => Security::Tls,
        "starttls" => Security::StartTls,
        "plain" => Security::Plain,
        other => panic!("unknown security {other:?}"),
    };
    Endpoint::new(host, port.parse().expect("port"), security)
}

/// The folder with `role` that holds a message with `subject`, if any.
fn holds(store: &Store, account: AccountId, role: FolderRole, subject: &str) -> Result<bool> {
    for folder in store.folders(account)? {
        if folder.role == Some(role)
            && store
                .messages_in_folder(folder.id)?
                .iter()
                .any(|m| m.subject == subject)
        {
            return Ok(true);
        }
    }
    Ok(false)
}

async fn run() -> Result<bool> {
    let user = var("KATNA_USER");
    let creds = Credentials::new(user.clone(), var("KATNA_PASSWORD"));
    let tls = match env::var("KATNA_INSECURE").as_deref() {
        Ok("1") => Tls::insecure_for_local_tests(),
        _ => Tls::system()?,
    };
    let tmp = tempfile::tempdir()?;
    let paths = Paths::with_root(tmp.path());
    let mut store = Store::open(&paths, Mode::ReadWrite)?;
    let account = store.add_account(AccountKind::Imap, "check", &user)?.id;

    let connector = ImapConnector {
        endpoint: endpoint("KATNA_IMAP"),
        credentials: creds.clone(),
        tls: tls.clone(),
    };
    let config = WorkerConfig {
        watch_interval: Some(Duration::from_secs(20)),
        ..WorkerConfig::default()
    };
    let (events_tx, events) = async_channel::unbounded();
    let (handle, control) = worker::control();
    let task = smol::spawn(worker::run(
        connector, store, account, config, events_tx, control,
    ));
    let reader = Store::open(&paths, Mode::ReadOnly)?;
    let next = |limit: Duration| {
        let events = events.clone();
        async move {
            events
                .recv()
                .or(async {
                    Timer::after(limit).await;
                    Err(async_channel::RecvError)
                })
                .await
                .ok()
        }
    };

    // First sync, then a moment for the worker to settle into IDLE.
    let started = Instant::now();
    loop {
        match next(Duration::from_secs(300)).await {
            Some(Event::Synced(_)) => break,
            Some(
                Event::Connected
                | Event::BodiesStored(_)
                | Event::ChangesSent(_)
                | Event::QuotaChanged,
            ) => {}
            // The worker tries again by itself.
            Some(Event::Disconnected { error, retry_in }) => {
                println!("not connected yet ({error}); retrying in {retry_in:?}");
            }
            Some(Event::AuthFailed(error)) => return Err(Error::Auth(error)),
            None => return Err(Error::Closed("no first sync within 5 minutes".into())),
        }
    }
    println!("first sync: {:?}", started.elapsed());
    Timer::after(Duration::from_secs(3)).await;

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let subject = format!("Katna send check {nanos}");
    let domain = user.rsplit_once('@').map_or("katna.invalid", |(_, d)| d);
    let message = format!(
        "From: <{user}>\r\nTo: <{user}>\r\nSubject: {subject}\r\n\
         Message-ID: <send-check-{nanos}@{domain}>\r\n\
         Content-Type: text/plain; charset=utf-8\r\n\r\n\
         Sent by Katna's send_check to this account itself. Safe to delete.\r\n"
    );
    let mut smtp = SmtpSender::connect(&endpoint("KATNA_SMTP"), &creds, tls).await?;
    let sent_at = Instant::now();
    smtp.send(&user, &[&user], message.into_bytes()).await?;
    smtp.quit().await?;
    println!("sent over SMTP in {:?}: {subject}", sent_at.elapsed());

    let mut in_inbox = None;
    let mut in_sent = None;
    while sent_at.elapsed() < SENT_WITHIN && (in_inbox.is_none() || in_sent.is_none()) {
        if in_inbox.is_none() && sent_at.elapsed() > INBOX_WITHIN {
            break;
        }
        match next(Duration::from_secs(5)).await {
            Some(Event::Disconnected { error, .. }) => println!("worker disconnected: {error}"),
            Some(Event::AuthFailed(error)) => {
                return Err(Error::Auth(error));
            }
            _ => {}
        }
        if in_inbox.is_none() && holds(&reader, account, FolderRole::Inbox, &subject)? {
            in_inbox = Some(sent_at.elapsed());
        }
        if in_sent.is_none() && holds(&reader, account, FolderRole::Sent, &subject)? {
            in_sent = Some(sent_at.elapsed());
        }
    }
    drop(handle);
    task.await;

    match in_sent {
        Some(after) => println!("in Sent after {after:?} (filed by the server)"),
        None => println!(
            "not in Sent within {SENT_WITHIN:?} (Katna's outbox files it on servers that don't)"
        ),
    }
    match in_inbox {
        Some(after) => {
            println!("in the inbox after {after:?} (pushed by IDLE)");
            Ok(true)
        }
        None => {
            println!("NOT in the inbox within {INBOX_WITHIN:?}");
            Ok(false)
        }
    }
}
