// SPDX-License-Identifier: GPL-3.0-or-later

//! Command-line client for `katna-daemon`. See `docs/IMPLEMENTATION_PLAN.md` task 1.13.
//!
//! Commands go to the daemon over D-Bus; `folders` and `list` read the store
//! read-only, as the apps do.

use std::{
    io::{self, BufRead, IsTerminal},
    process::ExitCode,
};

use futures_lite::StreamExt;
use katna_core::{AccountId, Paths};
use katna_dbus::{AccountStatus, NewImapAccount, PimProxy, ServerSpec};
use katna_store::{Mode, ParticipantRole, Store};

const USAGE: &str = "\
usage: katnactl status
       katnactl add-imap ADDRESS [--name NAME] [--user LOGIN]
                [--imap HOST[:PORT]] [--security tls|starttls|plain]
                [--smtp HOST[:PORT]] [--smtp-security tls|starttls|plain]
                [--insecure]
       katnactl password ACCOUNT
       katnactl remove ACCOUNT
       katnactl sync [ACCOUNT]
       katnactl watch
       katnactl folders ACCOUNT
       katnactl list ACCOUNT [--folder PATH] [--limit N]
       katnactl show MESSAGE

Talks to katna-daemon, which syncs your accounts in the background.

status     Accounts and what their sync is doing.
add-imap   Adds an IMAP account. Asks for the password (or reads one line
           from standard input), and the daemon checks it before saving it
           in your keyring. For Gmail, Yahoo, iCloud and Fastmail the
           servers are known; use an app password there. Other providers
           need --imap. Ports default to 993/465 (tls) or 143/587 (starttls).
           --insecure accepts self-signed certificates (test servers only).
password   Changes an account's saved password.
remove     Deletes an account, its synced mail and its password.
sync       Syncs every folder now, of one account or of all.
watch      Prints the daemon's change signals until interrupted.
folders    The synced folders of an account.
list       The newest messages in a folder (default: INBOX, 20), with their
           message numbers.
show       Prints a message as it came from the server, downloading it
           first if it is not stored yet.";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        None | Some("-h" | "--help" | "help") => {
            println!("{USAGE}");
            return ExitCode::SUCCESS;
        }
        Some("--version") => {
            println!("katnactl {}", env!("CARGO_PKG_VERSION"));
            return ExitCode::SUCCESS;
        }
        Some(command) => run(command, &args[1..]),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(Failure::Usage(message)) => {
            eprintln!("katnactl: {message}\n\n{USAGE}");
            ExitCode::from(2)
        }
        Err(Failure::Error(message)) => {
            eprintln!("katnactl: {message}");
            ExitCode::FAILURE
        }
    }
}

enum Failure {
    Usage(String),
    Error(String),
}

type Result<T, E = Failure> = std::result::Result<T, E>;

fn usage(message: impl Into<String>) -> Failure {
    Failure::Usage(message.into())
}

fn error(message: impl std::fmt::Display) -> Failure {
    Failure::Error(message.to_string())
}

impl From<zbus::Error> for Failure {
    fn from(err: zbus::Error) -> Self {
        match err {
            zbus::Error::MethodError(name, Some(message), _) => match name.as_str() {
                "org.freedesktop.DBus.Error.ServiceUnknown" => error(format!(
                    "katna-daemon is not running and could not be started \
                     ({message}). Run `katna-daemon install-user-service` once, \
                     or start `katna-daemon` in another terminal."
                )),
                _ => error(message),
            },
            other => error(other),
        }
    }
}

fn run(command: &str, args: &[String]) -> Result<()> {
    match command {
        "status" => no_args(args).and_then(|()| with_daemon(status)),
        "add-imap" => add_imap(args),
        "password" => {
            let id = one_account(args)?;
            let password = read_password()?;
            with_daemon(|pim| async move {
                pim.set_password(id.0, &password).await?;
                println!("password saved; syncing account {id}");
                Ok(())
            })
        }
        "remove" => {
            let id = one_account(args)?;
            with_daemon(|pim| async move {
                if pim.remove_account(id.0).await? {
                    println!("removed account {id}");
                    Ok(())
                } else {
                    Err(error(format!("no account {id}")))
                }
            })
        }
        "sync" => {
            let id = match args {
                [] => 0,
                [id] => parse_account(id)?.0,
                _ => return Err(usage("sync takes at most one account")),
            };
            with_daemon(|pim| async move { Ok(pim.sync_now(id).await?) })
        }
        "watch" => no_args(args).and_then(|()| with_daemon(watch)),
        "folders" => folders(one_account(args)?),
        "list" => list(args),
        "show" => match args {
            [id] => show(
                id.parse()
                    .map_err(|_| usage(format!("{id:?} is not a message number")))?,
            ),
            _ => Err(usage("expected one message number (see `katnactl list`)")),
        },
        other => Err(usage(format!("unknown command {other:?}"))),
    }
}

fn no_args(args: &[String]) -> Result<()> {
    match args {
        [] => Ok(()),
        _ => Err(usage(format!("unexpected {:?}", args[0]))),
    }
}

fn one_account(args: &[String]) -> Result<AccountId> {
    match args {
        [id] => parse_account(id),
        _ => Err(usage("expected one account number (see `katnactl status`)")),
    }
}

fn parse_account(text: &str) -> Result<AccountId> {
    text.parse()
        .map(AccountId)
        .map_err(|_| usage(format!("{text:?} is not an account number")))
}

/// Runs `f` with a proxy to the daemon on the session bus.
fn with_daemon<F, Fut>(f: F) -> Result<()>
where
    F: FnOnce(PimProxy<'static>) -> Fut,
    Fut: Future<Output = Result<()>>,
{
    futures_lite::future::block_on(async {
        let connection = zbus::Connection::session()
            .await
            .map_err(|err| error(format!("session bus: {err}")))?;
        let pim = PimProxy::new(&connection).await?;
        f(pim).await
    })
}

async fn status(pim: PimProxy<'static>) -> Result<()> {
    let accounts = pim.accounts().await?;
    if accounts.is_empty() {
        println!("no accounts; add one with `katnactl add-imap ADDRESS`");
    }
    for account in &accounts {
        print_status(account);
    }
    Ok(())
}

fn print_status(account: &AccountStatus) {
    let synced = match account.last_sync {
        0 => String::new(),
        time => format!(", synced {}", format_time(time)),
    };
    println!(
        "{:>3}  {}  {} ({}): {}{synced}",
        account.id, account.address, account.display_name, account.kind, account.state
    );
    if !account.detail.is_empty() {
        println!("     {}", account.detail);
    }
}

async fn watch(pim: PimProxy<'static>) -> Result<()> {
    let accounts = pim.receive_accounts_changed().await?.map(|_| None);
    let status = pim
        .receive_sync_status_changed()
        .await?
        .map(|signal| signal.args().ok().map(|args| (args.account, false)));
    let mail = pim
        .receive_mail_changed()
        .await?
        .map(|signal| signal.args().ok().map(|args| (args.account, true)));
    let mut signals = accounts.or(status).or(mail);
    println!("watching; stop with Ctrl+C");
    while let Some(signal) = signals.next().await {
        match signal {
            None => println!("accounts changed"),
            Some((id, true)) => println!("account {id}: mail changed"),
            Some((id, false)) => {
                let accounts = pim.accounts().await?;
                if let Some(account) = accounts.iter().find(|a| a.id == id) {
                    print_status(account);
                }
            }
        }
    }
    Ok(())
}

/// Well-known providers: (domains, IMAP host, SMTP host, SMTP security).
/// Proper autoconfiguration is plan task 1.2.
const PROVIDERS: &[(&[&str], &str, &str, &str)] = &[
    (
        &["gmail.com", "googlemail.com"],
        "imap.gmail.com",
        "smtp.gmail.com",
        "tls",
    ),
    (
        &["yahoo.com", "ymail.com"],
        "imap.mail.yahoo.com",
        "smtp.mail.yahoo.com",
        "tls",
    ),
    (
        &["icloud.com", "me.com", "mac.com"],
        "imap.mail.me.com",
        "smtp.mail.me.com",
        "starttls",
    ),
    (
        &["fastmail.com", "fastmail.fm"],
        "imap.fastmail.com",
        "smtp.fastmail.com",
        "tls",
    ),
];

fn add_imap(args: &[String]) -> Result<()> {
    let mut address = None;
    let mut name = String::new();
    let mut user = String::new();
    let mut imap = None;
    let mut smtp = None;
    let mut security = "tls".to_owned();
    let mut smtp_security = None;
    let mut insecure = false;
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        let mut value = || {
            args.next()
                .cloned()
                .ok_or_else(|| usage(format!("{arg} needs a value")))
        };
        match arg.as_str() {
            "--name" => name = value()?,
            "--user" => user = value()?,
            "--imap" => imap = Some(value()?),
            "--smtp" => smtp = Some(value()?),
            "--security" => security = value()?,
            "--smtp-security" => smtp_security = Some(value()?),
            "--insecure" => insecure = true,
            flag if flag.starts_with('-') => return Err(usage(format!("unknown option {flag}"))),
            _ if address.is_none() => address = Some(arg.clone()),
            _ => return Err(usage(format!("unexpected {arg:?}"))),
        }
    }
    let address = address.ok_or_else(|| usage("add-imap needs an address"))?;
    let domain = address
        .rsplit_once('@')
        .map(|(_, domain)| domain.to_ascii_lowercase())
        .unwrap_or_default();
    let known = PROVIDERS
        .iter()
        .find(|(domains, ..)| domains.contains(&domain.as_str()));
    let (imap, smtp, smtp_security) = match (imap, known) {
        (Some(imap), _) => {
            let smtp_security = smtp_security.unwrap_or_else(|| security.clone());
            (imap, smtp, smtp_security)
        }
        (None, Some((_, imap_host, smtp_host, smtp_sec))) => (
            (*imap_host).to_owned(),
            smtp.or_else(|| Some((*smtp_host).to_owned())),
            smtp_security.unwrap_or_else(|| (*smtp_sec).to_owned()),
        ),
        (None, None) => {
            return Err(usage(format!(
                "the servers for {domain:?} are not known; give --imap HOST"
            )));
        }
    };
    let account = NewImapAccount {
        display_name: name,
        address: address.clone(),
        imap: server_spec(&imap, &security, &user, insecure, [993, 143])?,
        smtp: match smtp {
            Some(smtp) => server_spec(&smtp, &smtp_security, &user, insecure, [465, 587])?,
            None => ServerSpec::default(),
        },
    };
    if known.is_some_and(|(domains, ..)| domains[0] == "gmail.com") {
        eprintln!("Gmail needs an app password: Google Account > Security > App passwords.");
    }
    let password = read_password()?;
    println!("checking the login at {}…", account.imap.host);
    with_daemon(|pim| async move {
        let id = pim.add_imap_account(&account, &password).await?;
        println!("added account {id} ({address}); syncing in the background");
        println!("follow it with `katnactl watch` or `katnactl status`");
        Ok(())
    })
}

/// Parses `HOST[:PORT]`; `ports` are the defaults for tls and starttls.
fn server_spec(
    host_port: &str,
    security: &str,
    user: &str,
    insecure: bool,
    ports: [u16; 2],
) -> Result<ServerSpec> {
    let (host, port) = match host_port.rsplit_once(':') {
        Some((host, port)) => (
            host,
            port.parse()
                .map_err(|_| usage(format!("bad port in {host_port:?}")))?,
        ),
        None => (
            host_port,
            if security == "tls" {
                ports[0]
            } else {
                ports[1]
            },
        ),
    };
    Ok(ServerSpec {
        host: host.to_owned(),
        port,
        security: security.to_owned(),
        username: user.to_owned(),
        accept_invalid_certs: insecure,
    })
}

fn read_password() -> Result<String> {
    let password = if io::stdin().is_terminal() {
        rpassword::prompt_password("Password: ").map_err(|err| error(format!("password: {err}")))?
    } else {
        let mut line = String::new();
        io::stdin()
            .lock()
            .read_line(&mut line)
            .map_err(|err| error(format!("password: {err}")))?;
        line.trim_end_matches(['\r', '\n']).to_owned()
    };
    if password.is_empty() {
        return Err(error("the password is empty"));
    }
    Ok(password)
}

fn open_store() -> Result<Store> {
    let paths = Paths::from_env().map_err(error)?;
    Store::open(&paths, Mode::ReadOnly).map_err(|err| {
        error(format!(
            "{err} (the daemon creates the store when it first runs)"
        ))
    })
}

fn folders(account: AccountId) -> Result<()> {
    let store = open_store()?;
    let folders = store.folders(account).map_err(error)?;
    if folders.is_empty() {
        println!("no folders synced for account {account}");
    }
    for folder in folders {
        let count = store.folder_uids(folder.id).map_err(error)?.len();
        let role = folder
            .role
            .map(|role| format!(" [{}]", role.as_str()))
            .unwrap_or_default();
        println!("{:>7}  {}{role}", count, folder.path);
    }
    Ok(())
}

fn list(args: &[String]) -> Result<()> {
    let mut account = None;
    let mut folder = "INBOX".to_owned();
    let mut limit = 20usize;
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        let mut value = || {
            args.next()
                .cloned()
                .ok_or_else(|| usage(format!("{arg} needs a value")))
        };
        match arg.as_str() {
            "--folder" => folder = value()?,
            "--limit" => {
                limit = value()?
                    .parse()
                    .map_err(|_| usage("--limit needs a number"))?;
            }
            _ if account.is_none() => account = Some(parse_account(arg)?),
            _ => return Err(usage(format!("unexpected {arg:?}"))),
        }
    }
    let account = account.ok_or_else(|| usage("list needs an account number"))?;
    let store = open_store()?;
    let Some(stored) = store
        .folders(account)
        .map_err(error)?
        .into_iter()
        .find(|f| f.path == folder)
    else {
        return Err(error(format!("account {account} has no folder {folder:?}")));
    };
    let mut messages = store.messages_in_folder(stored.id).map_err(error)?;
    messages.sort_by_key(|m| std::cmp::Reverse(m.date));
    for message in messages.iter().take(limit) {
        let from = message
            .first(ParticipantRole::From)
            .map(|p| {
                p.display_name
                    .clone()
                    .unwrap_or_else(|| p.email_norm.clone())
            })
            .unwrap_or_default();
        let date = message.date.map(format_time).unwrap_or_default();
        let unread = if message.flags.contains(katna_store::MessageFlags::SEEN) {
            ' '
        } else {
            '*'
        };
        println!(
            "{:>7} {unread} {date:16}  {:24}  {}",
            message.id.0,
            truncate(&from, 24),
            message.subject
        );
    }
    println!(
        "({} of {} messages)",
        messages.len().min(limit),
        messages.len()
    );
    Ok(())
}

fn show(message: i64) -> Result<()> {
    let stored = || -> Result<Option<Vec<u8>>> {
        let store = open_store()?;
        let found = store
            .messages_by_id(&[katna_store::MessageId(message)])
            .map_err(error)?
            .into_iter()
            .next()
            .ok_or_else(|| error(format!("no message {message}")))?;
        match found.blob_hash {
            Some(hash) => store.blobs().get(&hash).map_err(error),
            None => Ok(None),
        }
    };
    let raw = match stored()? {
        Some(raw) => raw,
        None => {
            with_daemon(|pim| async move { Ok(pim.fetch_body(message).await?) })?;
            stored()?.ok_or_else(|| error(format!("message {message} was not stored")))?
        }
    };
    use std::io::Write;
    io::stdout()
        .write_all(&raw)
        .map_err(|err| error(format!("writing: {err}")))
}

fn truncate(text: &str, width: usize) -> String {
    if text.chars().count() <= width {
        return text.to_owned();
    }
    let mut short: String = text.chars().take(width - 1).collect();
    short.push('…');
    short
}

/// `YYYY-MM-DD HH:MM` in UTC.
fn format_time(unix: i64) -> String {
    let days = unix.div_euclid(86_400);
    let seconds = unix.rem_euclid(86_400);
    // Howard Hinnant's civil_from_days.
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!(
        "{year:04}-{month:02}-{day:02} {:02}:{:02}",
        seconds / 3600,
        seconds % 3600 / 60
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_utc_times() {
        assert_eq!(format_time(0), "1970-01-01 00:00");
        assert_eq!(format_time(1_790_416_800), "2026-09-26 10:00");
        assert_eq!(format_time(951_782_400), "2000-02-29 00:00");
    }

    #[test]
    fn server_ports_follow_security() {
        let tls = server_spec("imap.example.org", "tls", "", false, [993, 143])
            .ok()
            .unwrap();
        assert_eq!((tls.host.as_str(), tls.port), ("imap.example.org", 993));
        let starttls = server_spec("mail.example.org", "starttls", "", false, [465, 587])
            .ok()
            .unwrap();
        assert_eq!(starttls.port, 587);
        let explicit = server_spec("localhost:10993", "tls", "bob", true, [993, 143])
            .ok()
            .unwrap();
        assert_eq!((explicit.port, explicit.username.as_str()), (10993, "bob"));
        assert!(explicit.accept_invalid_certs);
    }

    #[test]
    fn truncates_by_characters() {
        assert_eq!(truncate("short", 24), "short");
        assert_eq!(truncate("Ünïcödé names are long", 8), "Ünïcödé…");
    }
}
