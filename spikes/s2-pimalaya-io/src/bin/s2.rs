// SPDX-License-Identifier: GPL-3.0-or-later

//! Runs the S2 success checks against one server and prints a report.
//!
//! ```sh
//! cargo run --release -- stalwart     # local dev server (dev/compose.yaml)
//! cargo run --release -- dovecot      # local dev server
//! S2_IMAP=imap.gmail.com:993:tls S2_SMTP=smtp.gmail.com:465:tls \
//!   S2_USER=me@gmail.com S2_PASSWORD=<app password> \
//!   S2_FOLDER='[Gmail]/All Mail' cargo run --release -- custom
//! ```
//!
//! The `custom` profile is read-only unless `S2_WRITE=1`: it lists folders,
//! fetches envelopes from `S2_FOLDER` and waits on INBOX with IDLE
//! (send yourself a mail while it waits). It never appends or sends.
//!
//! `S2_EXECUTOR=smol` runs everything as a task on smol's thread pool
//! instead of a plain `block_on`, to show the futures are `Send` and not
//! tied to one executor.

use std::{
    env,
    process::ExitCode,
    sync::Mutex,
    time::{Duration, Instant},
};

use async_io::Timer;
use futures_lite::future;
use s2_pimalaya_io::{
    Credentials, Endpoint, Error, MailBackend, MailSender, Result, Security, imap::ImapBackend,
    net::Tls, smtp::SmtpSender,
};

const BULK_FOLDER: &str = "S2 Bulk";
const BULK_COUNT: u32 = 1000;

struct Profile {
    name: String,
    imap: Endpoint,
    smtp: Option<Endpoint>,
    creds: Credentials,
    tls: Tls,
    folder: Option<String>,
    write: bool,
    /// Whether mail sent to ourselves lands in our INBOX (Stalwart does
    /// local delivery; the Dovecot setup relays everything to Mailpit).
    local_delivery: bool,
}

fn main() -> ExitCode {
    env_logger::init();
    let profile = match profile() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };
    let result = if env::var("S2_EXECUTOR").as_deref() == Ok("smol") {
        println!("executor: smol thread pool");
        smol::block_on(smol::spawn(run(profile)))
    } else {
        println!("executor: futures-lite block_on");
        future::block_on(run(profile))
    };
    match result {
        Ok(()) => {
            println!("\nall checks passed");
            ExitCode::SUCCESS
        }
        Err(e) => {
            println!("\nFAILED: {e}");
            ExitCode::FAILURE
        }
    }
}

fn profile() -> std::result::Result<Profile, String> {
    let which = env::args().nth(1).unwrap_or_default();
    let alice = Credentials {
        user: "alice@katna.test".into(),
        password: "katna-dev".into(),
    };
    let local = |port, security| Endpoint {
        host: "127.0.0.1".into(),
        port,
        security,
    };
    match which.as_str() {
        "stalwart" => Ok(Profile {
            name: which,
            imap: local(10993, Security::Tls),
            smtp: Some(local(10465, Security::Tls)),
            creds: alice.clone(),
            tls: Tls::insecure_for_local_tests(),
            folder: None,
            write: true,
            local_delivery: true,
        }),
        "dovecot" => Ok(Profile {
            name: which,
            imap: local(20143, Security::StartTls),
            smtp: Some(local(20587, Security::StartTls)),
            creds: alice,
            tls: Tls::insecure_for_local_tests(),
            folder: None,
            write: true,
            local_delivery: false,
        }),
        "custom" => {
            let var = |k: &str| env::var(k).map_err(|_| format!("{k} is not set"));
            Ok(Profile {
                name: which,
                imap: endpoint(&var("S2_IMAP")?)?,
                smtp: env::var("S2_SMTP").ok().map(|s| endpoint(&s)).transpose()?,
                creds: Credentials {
                    user: var("S2_USER")?,
                    password: var("S2_PASSWORD")?,
                },
                tls: Tls::system().map_err(|e| e.to_string())?,
                folder: env::var("S2_FOLDER").ok(),
                write: env::var("S2_WRITE").as_deref() == Ok("1"),
                local_delivery: true,
            })
        }
        _ => Err("usage: s2 stalwart|dovecot|custom (see the source for variables)".into()),
    }
}

fn endpoint(spec: &str) -> std::result::Result<Endpoint, String> {
    let mut parts = spec.split(':');
    let (Some(host), Some(port), security) = (parts.next(), parts.next(), parts.next()) else {
        return Err(format!(
            "expected host:port[:tls|starttls|plain], got {spec}"
        ));
    };
    Ok(Endpoint {
        host: host.into(),
        port: port.parse().map_err(|_| format!("bad port in {spec}"))?,
        security: match security.unwrap_or("tls") {
            "tls" => Security::Tls,
            "starttls" => Security::StartTls,
            "plain" => Security::Plain,
            other => return Err(format!("bad security {other}")),
        },
    })
}

async fn run(p: Profile) -> Result<()> {
    println!("== {} ({}:{})", p.name, p.imap.host, p.imap.port);

    // 1. Log in.
    let t = Instant::now();
    let mut imap = ImapBackend::connect(&p.imap, &p.creds, p.tls.clone()).await?;
    println!("login: ok in {:?} ({:?})", t.elapsed(), p.imap.security);
    println!("  capabilities: {}", imap.capabilities().join(" "));

    // 2. List folders.
    let t = Instant::now();
    let folders = imap.list_folders().await?;
    println!("list folders: {} in {:?}", folders.len(), t.elapsed());
    for f in &folders {
        println!(
            "  {:<24} role={:?} selectable={}",
            f.name, f.role, f.selectable
        );
    }

    // 3. Fetch 1,000 envelopes.
    let folder = match &p.folder {
        Some(f) => f.clone(),
        None => {
            if !folders.iter().any(|f| f.name == BULK_FOLDER) {
                imap.create_folder(BULK_FOLDER).await?;
            }
            BULK_FOLDER.to_owned()
        }
    };
    let status = imap.select(&folder).await?;
    if p.write && status.exists < BULK_COUNT {
        let t = Instant::now();
        let missing = BULK_COUNT - status.exists;
        for i in 0..missing {
            imap.append(&folder, bulk_message(status.exists + i))
                .await?;
        }
        println!("append: {missing} messages in {:?}", t.elapsed());
    }
    let status = imap.select(&folder).await?;
    println!(
        "select {folder:?}: exists={} uidvalidity={:?} uidnext={:?} highestmodseq={:?}",
        status.exists, status.uid_validity, status.uid_next, status.highest_modseq
    );
    let t = Instant::now();
    let mut envelopes = imap.fetch_envelopes(1, None).await?;
    let elapsed = t.elapsed();
    // Newest 1,000, as the message list would show them.
    let skip = envelopes.len().saturating_sub(BULK_COUNT as usize);
    envelopes.drain(..skip);
    println!(
        "fetch envelopes: {} of {} in {elapsed:?}",
        envelopes.len(),
        status.exists
    );
    if let Some(e) = envelopes.last() {
        println!(
            "  last: uid={} size={} from={:?} subject={:?} seen={}",
            e.uid, e.size, e.from, e.subject, e.flags.seen
        );
    }
    if envelopes.len() < BULK_COUNT as usize {
        return Err(Error::Protocol(format!(
            "only {} envelopes, need {BULK_COUNT}",
            envelopes.len()
        )));
    }

    // 4. IDLE with nothing happening: our timer ends it cleanly.
    imap.select("INBOX").await?;
    let t = Instant::now();
    let changes = imap.wait_for_changes(Duration::from_secs(3)).await?;
    println!("idle timeout: {changes:?} after {:?}", t.elapsed());
    let changes = imap.poll_changes().await?;
    println!("  connection still usable (NOOP): {changes:?}");

    // 5. IDLE woken by a message from another connection.
    if p.write {
        let delivered = Mutex::new(None);
        let (changes, appended) =
            future::zip(imap.wait_for_changes(Duration::from_secs(30)), async {
                Timer::after(Duration::from_millis(500)).await;
                let mut own = ImapBackend::connect(&p.imap, &p.creds, p.tls.clone()).await?;
                own.append("INBOX", test_message("idle via append")).await?;
                *delivered.lock().unwrap() = Some(Instant::now());
                own.logout().await
            })
            .await;
        appended?;
        let changes = changes?;
        let latency = delivered.lock().unwrap().map(|d| d.elapsed());
        println!(
            "idle + append from another connection: {changes:?}, woke {latency:?} after append"
        );
        if changes.is_empty() {
            return Err(Error::Protocol(
                "IDLE did not report the appended message".into(),
            ));
        }
    } else {
        println!("idle: waiting up to 120 s on INBOX, send yourself a message now");
        let t = Instant::now();
        let changes = imap.wait_for_changes(Duration::from_secs(120)).await?;
        println!("  {changes:?} after {:?}", t.elapsed());
    }

    // 6. Send with SMTP; on servers with local delivery, IDLE sees it arrive.
    if let (Some(smtp), true) = (&p.smtp, p.write) {
        let t = Instant::now();
        let mut sender = SmtpSender::connect(smtp, &p.creds, p.tls.clone()).await?;
        println!("smtp login: ok in {:?} ({:?})", t.elapsed(), smtp.security);
        println!("  capabilities: {}", sender.capabilities().join(" | "));
        let me = p.creds.user.as_str();
        if p.local_delivery {
            imap.select("INBOX").await?;
            let sent = Mutex::new(None);
            let (changes, send) =
                future::zip(imap.wait_for_changes(Duration::from_secs(30)), async {
                    Timer::after(Duration::from_millis(500)).await;
                    sender
                        .send(me, &[me], test_message("idle via smtp"))
                        .await?;
                    *sent.lock().unwrap() = Some(Instant::now());
                    Ok::<_, Error>(())
                })
                .await;
            send?;
            let changes = changes?;
            let latency = sent.lock().unwrap().map(|s| s.elapsed());
            println!("smtp send + idle: {changes:?}, woke {latency:?} after send");
            if changes.is_empty() {
                return Err(Error::Protocol(
                    "IDLE did not report the sent message".into(),
                ));
            }
        } else {
            let t = Instant::now();
            sender
                .send(me, &["someone@example.org"], test_message("relayed"))
                .await?;
            println!("smtp send (relayed to Mailpit): ok in {:?}", t.elapsed());
        }
        sender.quit().await?;
    }

    imap.logout().await?;
    println!("logout: ok");
    Ok(())
}

fn bulk_message(n: u32) -> Vec<u8> {
    format!(
        "From: Bob <bob@katna.test>\r\n\
         To: Alice <alice@katna.test>\r\n\
         Subject: S2 bulk message {n}\r\n\
         Date: Sat, 26 Sep 2026 04:00:00 +0000\r\n\
         Message-ID: <s2-bulk-{n}@katna.test>\r\n\
         \r\n\
         Body of bulk message {n}.\r\n"
    )
    .into_bytes()
}

fn test_message(subject: &str) -> Vec<u8> {
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!(
        "From: Alice <alice@katna.test>\r\n\
         To: Alice <alice@katna.test>\r\n\
         Subject: S2 {subject}\r\n\
         Date: Sat, 26 Sep 2026 04:00:00 +0000\r\n\
         Message-ID: <s2-{id}@katna.test>\r\n\
         \r\n\
         Sent by the S2 spike.\r\n"
    )
    .into_bytes()
}
