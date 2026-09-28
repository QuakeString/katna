// SPDX-License-Identifier: GPL-3.0-or-later

//! Command-line client for `katna-daemon`. See `docs/IMPLEMENTATION_PLAN.md` task 1.13.
//!
//! Commands go to the daemon over D-Bus; `folders` and `list` read the store
//! read-only, as the apps do.

use std::{
    io::{self, BufRead, IsTerminal, Read},
    process::ExitCode,
};

use futures_lite::StreamExt;
use katna_core::{AccountId, Paths};
use katna_dbus::{AccountStatus, NewImapAccount, NewPop3Account, OutboxItem, PimProxy, ServerSpec};
use katna_store::{MessageId, Mode, ParticipantRole, Store};

const USAGE: &str = "\
usage: katnactl status
       katnactl add-imap ADDRESS [--name NAME] [--user LOGIN]
                [--imap HOST[:PORT]] [--security tls|starttls|plain]
                [--smtp HOST[:PORT]] [--smtp-security tls|starttls|plain]
                [--insecure]
       katnactl add-pop3 ADDRESS --pop3 HOST[:PORT] [--name NAME] [--user LOGIN]
                [--security tls|starttls|plain] [--smtp HOST[:PORT]]
                [--smtp-security tls|starttls|plain] [--insecure]
                [--remove | --keep-days N] [--keep-deleted]
       katnactl discover ADDRESS
       katnactl password ACCOUNT
       katnactl remove ACCOUNT
       katnactl sync [ACCOUNT]
       katnactl reset-cache
       katnactl watch
       katnactl folders ACCOUNT
       katnactl list ACCOUNT [--folder PATH] [--limit N]
       katnactl show MESSAGE
       katnactl flag MESSAGE... +NAME|-NAME...
       katnactl move PATH MESSAGE...
       katnactl delete MESSAGE...
       katnactl archive MESSAGE...
       katnactl send ACCOUNT [FILE] [--delay SECONDS]
       katnactl outbox
       katnactl undo ID
       katnactl discard ID
       katnactl crashes [show [NAME] | delete]

Talks to katna-daemon, which syncs your accounts in the background.

status     Accounts and what their sync is doing.
add-imap   Adds an IMAP account. Asks for the password (or reads one line
           from standard input), and the daemon checks it before saving it
           in your keyring. Without --imap the daemon finds the servers (as
           `discover` does). Gmail, Yahoo and iCloud need an app password.
           Ports default to 993/465 (tls) or 143/587 (starttls).
           --insecure accepts self-signed certificates (test servers only).
add-pop3   Adds a POP3 account: its mail is downloaded into local folders.
           Ports default to 995/465 (tls) or 110/587 (starttls). Without
           --smtp the daemon looks for the SMTP server. Mail stays on the
           server until you delete it in Katna; --remove deletes it from the
           server once downloaded, --keep-days N after N days, and
           --keep-deleted keeps it there after you delete it in Katna.
discover   Shows the servers the daemon finds for an address, and where
           it found them (provider settings, Thunderbird's database, DNS,
           or by trying the usual names).
password   Changes an account's saved password.
remove     Deletes an account, its synced mail and its password.
sync       Syncs every folder now, of one account or of all.
reset-cache
           Deletes downloaded mail that is still on the IMAP server, the
           search index and sender pictures, then syncs to download recent
           mail again. Accounts, settings and local-only mail stay.
watch      Prints the daemon's change signals until interrupted.
folders    The synced folders of an account.
list       The newest messages in a folder (default: INBOX, 20), with their
           message numbers.
show       Prints a message as it came from the server, downloading it
           first if it is not stored yet.
flag       Adds (+) or removes (-) flags: seen, answered, flagged, draft,
           forwarded. For example `katnactl flag 12 13 +seen -flagged`.
move       Moves messages to the folder PATH of their account.
delete     Moves messages to the trash, or deletes them if already there.
archive    Moves messages to the archive folder.
send       Sends a message (RFC 5322 text, from FILE or standard input)
           from ACCOUNT to its To, Cc and Bcc addresses, after --delay
           seconds (default 0). A copy goes to the Sent folder.
outbox     Messages waiting to be sent, failed or undone.
undo       Takes a message back while it waits for its delay.
discard    Forgets a failed or undone message.
crashes    Crash reports saved on this computer, newest first (a native
           crash is picked up from systemd-coredump first). `show` prints
           the newest report or the one named; `delete` deletes them all.
           Nothing is sent anywhere.

Changes show at once and reach the server when the account is online.
Mail waits in the outbox while offline.";

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
        Some(command) => {
            if let Ok(paths) = Paths::from_env() {
                katna_core::crash::install("katnactl", &paths);
            }
            run(command, &args[1..])
        }
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
        "add-pop3" => add_pop3(args),
        "discover" => match args {
            [address] => {
                let address = address.clone();
                with_daemon(|pim| async move {
                    let (account, source) = pim.discover_account(&address).await?;
                    print_discovered(&account, &source);
                    Ok(())
                })
            }
            _ => Err(usage("discover needs one address")),
        },
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
        "reset-cache" => no_args(args).and_then(|()| {
            with_daemon(|pim| async move {
                let (messages, bytes) = pim.reset_cache().await?;
                println!(
                    "Deleted {messages} downloaded messages ({:.1} MB); downloading recent mail again.",
                    bytes as f64 / 1_000_000.0
                );
                Ok(())
            })
        }),
        "watch" => no_args(args).and_then(|()| with_daemon(watch)),
        "crashes" => crashes(args),
        "folders" => folders(one_account(args)?),
        "list" => list(args),
        "show" => match args {
            [id] => show(
                id.parse()
                    .map_err(|_| usage(format!("{id:?} is not a message number")))?,
            ),
            _ => Err(usage("expected one message number (see `katnactl list`)")),
        },
        "flag" => flag(args),
        "move" => match args {
            [path, messages @ ..] if !messages.is_empty() => {
                let messages = message_ids(messages)?;
                let folder = folder_by_path(messages[0], path)?;
                with_daemon(|pim| async move { Ok(pim.move_messages(&messages, folder).await?) })
            }
            _ => Err(usage("move needs a folder and at least one message")),
        },
        "delete" => {
            let messages = message_ids(args)?;
            with_daemon(|pim| async move { Ok(pim.delete_messages(&messages).await?) })
        }
        "archive" => {
            let messages = message_ids(args)?;
            with_daemon(|pim| async move { Ok(pim.archive_messages(&messages).await?) })
        }
        "send" => send(args),
        "outbox" => no_args(args).and_then(|()| with_daemon(outbox)),
        "undo" => {
            let id = outbox_id(args)?;
            with_daemon(|pim| async move {
                if pim.undo_send(id).await? {
                    println!("undone; the message stays in the outbox as cancelled");
                    Ok(())
                } else {
                    Err(error(format!("{id} is not waiting to be sent")))
                }
            })
        }
        "discard" => {
            let id = outbox_id(args)?;
            with_daemon(|pim| async move {
                if pim.discard_send(id).await? {
                    Ok(())
                } else {
                    Err(error(format!("{id} is not a failed or undone message")))
                }
            })
        }
        other => Err(usage(format!("unknown command {other:?}"))),
    }
}

fn send(args: &[String]) -> Result<()> {
    let (mut account, mut file, mut delay) = (None, None, 0u32);
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        if arg == "--delay" {
            let value = args.next().ok_or_else(|| usage("--delay needs seconds"))?;
            delay = value
                .parse()
                .map_err(|_| usage(format!("{value:?} is not a number of seconds")))?;
        } else if account.is_none() {
            account = Some(parse_account(arg)?);
        } else if file.is_none() {
            file = Some(arg.clone());
        } else {
            return Err(usage(format!("unexpected {arg:?}")));
        }
    }
    let account = account.ok_or_else(|| usage("send needs an account number"))?;
    let mut message = Vec::new();
    match &file {
        Some(path) => {
            message = std::fs::read(path).map_err(|err| error(format!("{path}: {err}")))?;
        }
        None => {
            io::stdin()
                .read_to_end(&mut message)
                .map_err(|err| error(format!("standard input: {err}")))?;
        }
    }
    let message = crlf(&message);
    with_daemon(|pim| async move {
        let id = pim.queue_send(account.0, &message, delay).await?;
        if delay > 0 {
            println!("queued as {id}; `katnactl undo {id}` takes it back for {delay} s");
        } else {
            println!("queued as {id}");
        }
        Ok(())
    })
}

/// Line ends as SMTP needs them: CRLF, even if the file has LF.
fn crlf(text: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(text.len() + text.len() / 40);
    for (at, &byte) in text.iter().enumerate() {
        if byte == b'\n' && (at == 0 || text[at - 1] != b'\r') {
            out.push(b'\r');
        }
        out.push(byte);
    }
    out
}

fn outbox_id(args: &[String]) -> Result<i64> {
    match args {
        [id] => id
            .parse()
            .map_err(|_| usage(format!("{id:?} is not an outbox number"))),
        _ => Err(usage("expected one outbox number (see `katnactl outbox`)")),
    }
}

async fn outbox(pim: PimProxy<'static>) -> Result<()> {
    let items = pim.outbox().await?;
    if items.is_empty() {
        println!("the outbox is empty");
    }
    for item in &items {
        print_outbox_item(item);
    }
    Ok(())
}

fn print_outbox_item(item: &OutboxItem) {
    let detail = match item.detail.as_str() {
        "" => String::new(),
        detail => format!(" ({detail})"),
    };
    println!(
        "{:>5}  account {}  {:<9}  {}  {}{detail}",
        item.id,
        item.account,
        item.state,
        format_time(item.send_at),
        truncate(&item.subject, 40),
    );
}

/// Message numbers, at least one.
fn message_ids(args: &[String]) -> Result<Vec<i64>> {
    if args.is_empty() {
        return Err(usage("expected message numbers (see `katnactl list`)"));
    }
    args.iter()
        .map(|arg| {
            arg.parse()
                .map_err(|_| usage(format!("{arg:?} is not a message number")))
        })
        .collect()
}

fn flag(args: &[String]) -> Result<()> {
    let (mut add, mut remove, mut messages) = (Vec::new(), Vec::new(), Vec::new());
    for arg in args {
        if let Some(name) = arg.strip_prefix('+') {
            add.push(name.to_owned());
        } else if let Some(name) = arg.strip_prefix('-') {
            remove.push(name.to_owned());
        } else {
            messages.push(arg.clone());
        }
    }
    if add.is_empty() && remove.is_empty() {
        return Err(usage("flag needs +NAME or -NAME"));
    }
    let messages = message_ids(&messages)?;
    with_daemon(|pim| async move {
        let add: Vec<&str> = add.iter().map(String::as_str).collect();
        let remove: Vec<&str> = remove.iter().map(String::as_str).collect();
        Ok(pim.set_flags(&messages, &add, &remove).await?)
    })
}

/// The folder `path` in the account of `message`.
fn folder_by_path(message: i64, path: &str) -> Result<i64> {
    let store = open_store()?;
    let account = store
        .messages_by_id(&[MessageId(message)])
        .map_err(error)?
        .first()
        .map(|m| m.account)
        .ok_or_else(|| error(format!("no message {message}")))?;
    store
        .folders(account)
        .map_err(error)?
        .into_iter()
        .find(|folder| folder.path == path)
        .map(|folder| folder.id.0)
        .ok_or_else(|| error(format!("no folder {path:?} in account {account}")))
}

/// Programs whose crashes are looked for in `systemd-coredump`.
const CRASH_APPS: [&str; 3] = ["katna-mail", "katna-daemon", "katnactl"];

fn crashes(args: &[String]) -> Result<()> {
    let paths = Paths::from_env().map_err(error)?;
    let dir = paths.crash_dir();
    match args {
        [] => {
            katna_core::crash::collect_core_dumps(&paths, &CRASH_APPS);
            let reports = katna_core::crash::reports(&dir);
            if reports.is_empty() {
                println!("no crash reports in {}", dir.display());
            }
            for report in reports {
                println!("{}", report.name);
            }
            Ok(())
        }
        [show, name @ ..] if show == "show" && name.len() <= 1 => {
            katna_core::crash::collect_core_dumps(&paths, &CRASH_APPS);
            let reports = katna_core::crash::reports(&dir);
            let report = match name {
                [name] => reports.into_iter().find(|r| &r.name == name),
                _ => reports.into_iter().next(),
            };
            let report = report.ok_or_else(|| error("no such crash report"))?;
            print!("{}", report.read().map_err(error)?);
            Ok(())
        }
        [delete] if delete == "delete" => {
            katna_core::crash::delete_all(&dir).map_err(error)?;
            println!("deleted the crash reports");
            Ok(())
        }
        _ => Err(usage("crashes takes `show [NAME]` or `delete`")),
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
        let connection = katna_dbus::session()
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

enum Signal {
    Accounts,
    Status(i64),
    Mail(i64),
    Outbox(i64),
}

async fn watch(pim: PimProxy<'static>) -> Result<()> {
    let accounts = pim
        .receive_accounts_changed()
        .await?
        .map(|_| Some(Signal::Accounts));
    let status = pim
        .receive_sync_status_changed()
        .await?
        .map(|signal| signal.args().ok().map(|args| Signal::Status(args.account)));
    let mail = pim
        .receive_mail_changed()
        .await?
        .map(|signal| signal.args().ok().map(|args| Signal::Mail(args.account)));
    let outbox = pim
        .receive_outbox_changed()
        .await?
        .map(|signal| signal.args().ok().map(|args| Signal::Outbox(args.id)));
    let mut signals = accounts.or(status).or(mail).or(outbox);
    println!("watching; stop with Ctrl+C");
    while let Some(signal) = signals.next().await {
        match signal {
            None => {}
            Some(Signal::Accounts) => println!("accounts changed"),
            Some(Signal::Mail(id)) => println!("account {id}: mail changed"),
            Some(Signal::Status(id)) => {
                let accounts = pim.accounts().await?;
                if let Some(account) = accounts.iter().find(|a| a.id == id) {
                    print_status(account);
                }
            }
            Some(Signal::Outbox(id)) => {
                let items = pim.outbox().await?;
                match items.iter().find(|item| item.id == id) {
                    Some(item) => print_outbox_item(item),
                    None => println!("{id:>5}  sent and filed, or discarded"),
                }
            }
        }
    }
    Ok(())
}

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
    let account = match imap {
        Some(imap) => {
            let smtp_security = smtp_security.unwrap_or_else(|| security.clone());
            NewImapAccount {
                display_name: name,
                address: address.clone(),
                imap: server_spec(&imap, &security, &user, insecure, [993, 143])?,
                smtp: match smtp {
                    Some(smtp) => server_spec(&smtp, &smtp_security, &user, insecure, [465, 587])?,
                    None => ServerSpec::default(),
                },
            }
        }
        None => {
            let lookup = address.clone();
            let mut found = None;
            let slot = &mut found;
            with_daemon(|pim| async move {
                let (account, source) = pim.discover_account(&lookup).await?;
                print_discovered(&account, &source);
                *slot = Some(account);
                Ok(())
            })?;
            let mut account = found.ok_or_else(|| error("no servers found"))?;
            account.display_name = name;
            if let Some(smtp) = smtp {
                let smtp_security = smtp_security.unwrap_or_else(|| "tls".to_owned());
                account.smtp = server_spec(&smtp, &smtp_security, &user, insecure, [465, 587])?;
            }
            for server in [&mut account.imap, &mut account.smtp] {
                if !user.is_empty() && !server.host.is_empty() {
                    server.username = user.clone();
                }
                server.accept_invalid_certs = insecure;
            }
            account
        }
    };
    if account.imap.host == "imap.gmail.com" {
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

fn add_pop3(args: &[String]) -> Result<()> {
    let mut address = None;
    let mut name = String::new();
    let mut user = String::new();
    let mut pop3 = None;
    let mut smtp = None;
    let mut security = "tls".to_owned();
    let mut smtp_security = None;
    let mut insecure = false;
    let mut leave_on_server = true;
    let mut keep_days = 0;
    let mut delete_with_local = true;
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
            "--pop3" => pop3 = Some(value()?),
            "--smtp" => smtp = Some(value()?),
            "--security" => security = value()?,
            "--smtp-security" => smtp_security = Some(value()?),
            "--insecure" => insecure = true,
            "--remove" => leave_on_server = false,
            "--keep-days" => {
                keep_days = value()?
                    .parse()
                    .map_err(|_| usage("--keep-days needs a number of days"))?
            }
            "--keep-deleted" => delete_with_local = false,
            flag if flag.starts_with('-') => return Err(usage(format!("unknown option {flag}"))),
            _ if address.is_none() => address = Some(arg.clone()),
            _ => return Err(usage(format!("unexpected {arg:?}"))),
        }
    }
    let address = address.ok_or_else(|| usage("add-pop3 needs an address"))?;
    let pop3 = pop3.ok_or_else(|| usage("add-pop3 needs --pop3 HOST"))?;
    let smtp = match smtp {
        Some(smtp) => {
            let smtp_security = smtp_security.unwrap_or_else(|| security.clone());
            server_spec(&smtp, &smtp_security, &user, insecure, [465, 587])?
        }
        None => {
            let lookup = address.clone();
            let mut found = ServerSpec::default();
            let slot = &mut found;
            with_daemon(|pim| async move {
                if let Ok((account, _)) = pim.discover_account(&lookup).await {
                    *slot = account.smtp;
                }
                Ok(())
            })?;
            if found.host.is_empty() {
                eprintln!("no SMTP server found; this account can receive but not send");
            } else {
                println!("SMTP: {}:{} {}", found.host, found.port, found.security);
                if !user.is_empty() {
                    found.username = user.clone();
                }
                found.accept_invalid_certs = insecure;
            }
            found
        }
    };
    let account = NewPop3Account {
        display_name: name,
        address: address.clone(),
        pop3: server_spec(&pop3, &security, &user, insecure, [995, 110])?,
        smtp,
        leave_on_server,
        keep_days,
        delete_with_local,
    };
    let password = read_password()?;
    println!("checking the login at {}…", account.pop3.host);
    with_daemon(|pim| async move {
        let id = pim.add_pop3_account(&account, &password).await?;
        println!("added account {id} ({address}); downloading mail in the background");
        println!("follow it with `katnactl watch` or `katnactl status`");
        Ok(())
    })
}

fn print_discovered(account: &NewImapAccount, source: &str) {
    let show = |spec: &ServerSpec| match spec.host.as_str() {
        "" => "not found".to_owned(),
        host => format!(
            "{host}:{} {} as {}",
            spec.port, spec.security, spec.username
        ),
    };
    println!("found via {source}:");
    println!("  IMAP  {}", show(&account.imap));
    println!("  SMTP  {}", show(&account.smtp));
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
            .messages_by_id(&[MessageId(message)])
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
    fn line_ends_become_crlf() {
        assert_eq!(crlf(b"a\nb\r\nc\n"), b"a\r\nb\r\nc\r\n");
    }

    #[test]
    fn truncates_by_characters() {
        assert_eq!(truncate("short", 24), "short");
        assert_eq!(truncate("Ünïcödé names are long", 8), "Ünïcödé…");
    }
}
