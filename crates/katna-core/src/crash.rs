// SPDX-License-Identifier: GPL-3.0-or-later

//! Crash reports kept on this machine (`docs/ARCHITECTURE.md` §19.2).
//!
//! Every Katna program calls [`install`] at startup. A Rust panic then
//! leaves a plain-text report in [`Paths::crash_dir`]. Native crashes (a
//! segfault in a GPU driver or a C library) cannot be caught without
//! `unsafe`, so [`collect_core_dumps`] asks `systemd-coredump` afterwards
//! and turns each core dump of a Katna program into a report too.
//!
//! Reports never leave the machine here. Before one is written, the home
//! directory, user name, host name, machine ID and email addresses are
//! replaced by placeholders, URLs are cut to their scheme and host, and a
//! panic message's quoted text is left out ([`Scrubber`],
//! [`panic_message`]). The crash directory is private to the user (`0700`,
//! reports `0600`).

use std::backtrace::Backtrace;
use std::fmt::Write as _;
use std::fs;
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::paths::{Paths, create_private_dir};

/// The version this build reports: `$KATNA_VERSION` at build time (the
/// package's version, such as `0.0.0.r512.g4040a0e`), else the crate's.
pub const VERSION: &str = match option_env!("KATNA_VERSION") {
    Some(version) => version,
    None => env!("CARGO_PKG_VERSION"),
};

/// Reports kept; older ones are deleted when a new one is written.
pub const KEEP: usize = 20;

/// A panic loop writes at most this many reports per run.
const MAX_PER_RUN: u32 = 3;

/// Core dumps older than this are not looked at on the first check.
const FIRST_LOOK_BACK: Duration = Duration::from_secs(3 * 24 * 60 * 60);

/// The longest a `coredumpctl` call may take.
const COREDUMPCTL_TIMEOUT: Duration = Duration::from_secs(10);

/// Lines of a native crash's stack kept in its report.
const MAX_STACK_LINES: usize = 400;

/// Name of the file holding the newest report the user has seen.
const SEEN_FILE: &str = "seen";

/// Name of the file holding the time of the newest core dump looked at.
const LAST_CORE_FILE: &str = "last-core";

/// Name of the file listing the reports sent to the crash tracker.
const SENT_FILE: &str = "sent";

/// Reports older than this are not sent: they were written before the
/// user could have agreed, or tried long enough.
pub const SEND_WITHIN: Duration = Duration::from_secs(7 * 24 * 60 * 60);

struct Installed {
    app: &'static str,
    dir: PathBuf,
    config: PathBuf,
    scrubber: Scrubber,
}

static INSTALLED: OnceLock<Installed> = OnceLock::new();
static WRITTEN: AtomicU32 = AtomicU32::new(0);

/// Writes a report for every panic of this process, as `app` (the
/// program's name, such as `katna-mail`), into `paths.crash_dir()`.
///
/// Nothing is written while Settings > User feedback > "Save crash
/// reports on this computer" is off ([`enabled`], read at the time of the
/// panic). The panic is then passed on to the hook that was there
/// before, so the journal still gets its message. Call once, early in
/// `main`; later calls do nothing.
pub fn install(app: &'static str, paths: &Paths) {
    let installed = Installed {
        app,
        dir: paths.crash_dir(),
        config: paths.config_file(),
        scrubber: Scrubber::from_system(),
    };
    if INSTALLED.set(installed).is_err() {
        return;
    }
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        if let Some(installed) = INSTALLED.get()
            && WRITTEN.fetch_add(1, Ordering::Relaxed) < MAX_PER_RUN
            && enabled(&installed.config)
        {
            let text = panic_report(installed.app, info, &Backtrace::force_capture());
            let text = installed.scrubber.scrub(&text);
            if let Err(err) = write_report(
                &installed.dir,
                installed.app,
                SystemTime::now(),
                std::process::id(),
                &text,
            ) {
                eprintln!("{}: could not save a crash report: {err}", installed.app);
            }
        }
        previous(info);
    }));
}

/// Whether the settings file at `config` lets crash reports be saved.
/// A settings file that cannot be read counts as the default (yes).
pub fn enabled(config: &Path) -> bool {
    crate::Config::load(config).map_or(true, |config| config.feedback.save_crash_reports)
}

fn panic_report(app: &str, info: &std::panic::PanicHookInfo<'_>, backtrace: &Backtrace) -> String {
    let payload = info.payload();
    let message = payload
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
        .unwrap_or("(no message)");
    let location = info
        .location()
        .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
        .unwrap_or_else(|| "unknown".into());
    let thread = std::thread::current();
    let mut text = header(app, "panic", SystemTime::now());
    let _ = writeln!(text, "Thread: {}", thread.name().unwrap_or("unnamed"));
    let _ = writeln!(text, "Location: {location}");
    let _ = writeln!(text, "Message: {}", panic_message(message));
    let _ = writeln!(text, "\nBacktrace:\n{backtrace}");
    // Release binaries are stripped, so the backtrace above names few
    // functions; these offsets do, with the build's debug file.
    let _ = writeln!(text, "Frames:\n{}", raw_frames());
    let log = crate::logging::recent_lines();
    if !log.is_empty() {
        let _ = writeln!(text, "\nRecent log:");
        for line in log {
            let _ = writeln!(text, "{line}");
        }
    }
    text
}

/// Frames with more than this many are cut.
const MAX_FRAMES: usize = 100;

/// The stack of the calling thread as `module + 0xoffset` lines (return
/// addresses, as `coredumpctl` and `addr2line` take them), then the
/// executable's build ID.
fn raw_frames() -> String {
    let mut ips = Vec::new();
    backtrace::trace(|frame| {
        ips.push(frame.ip() as usize);
        ips.len() < MAX_FRAMES
    });
    let maps = fs::read_to_string("/proc/self/maps").unwrap_or_default();
    let modules = modules(&maps);
    let mut text = String::new();
    for (i, ip) in ips.into_iter().enumerate() {
        match modules.iter().find(|m| m.start <= ip && ip < m.end) {
            Some(m) => {
                let _ = writeln!(text, "  #{i:<3} {} + {:#x}", m.name, ip - m.base);
            }
            None => {
                let _ = writeln!(text, "  #{i:<3} {ip:#x}");
            }
        }
    }
    if let Some(id) = elf_head("/proc/self/exe").and_then(|elf| build_id(&elf)) {
        let _ = writeln!(text, "Build ID: {id}");
    }
    text
}

/// One executable mapping of `/proc/self/maps`.
#[derive(Debug, PartialEq, Eq)]
struct Module {
    start: usize,
    end: usize,
    /// Where the file's first byte is mapped: offsets are counted from it.
    base: usize,
    /// The file name without its directory.
    name: String,
}

fn modules(maps: &str) -> Vec<Module> {
    // The lowest address each file is mapped at, from its offset-0 mapping.
    let mut bases: Vec<(&str, usize)> = Vec::new();
    let mut modules = Vec::new();
    for line in maps.lines() {
        let mut fields = line.split_whitespace();
        let (Some(range), Some(perms), Some(offset)) =
            (fields.next(), fields.next(), fields.next())
        else {
            continue;
        };
        let Some(path) = fields.nth(2) else { continue };
        if !path.starts_with('/') {
            continue;
        }
        let Some((start, end)) = range.split_once('-') else {
            continue;
        };
        let (Ok(start), Ok(end), Ok(offset)) = (
            usize::from_str_radix(start, 16),
            usize::from_str_radix(end, 16),
            usize::from_str_radix(offset, 16),
        ) else {
            continue;
        };
        if offset == 0 && !bases.iter().any(|(p, _)| *p == path) {
            bases.push((path, start));
        }
        if perms.contains('x') {
            let base = bases
                .iter()
                .find(|(p, _)| *p == path)
                .map_or(start - offset, |(_, base)| *base);
            let name = path.rsplit('/').next().unwrap_or(path).to_string();
            modules.push(Module {
                start,
                end,
                base,
                name,
            });
        }
    }
    modules
}

/// The start of a file, where linkers put the build ID note.
fn elf_head(path: &str) -> Option<Vec<u8>> {
    use std::io::Read;
    let mut head = Vec::new();
    fs::File::open(path)
        .ok()?
        .take(64 * 1024)
        .read_to_end(&mut head)
        .ok()?;
    Some(head)
}

/// The GNU build ID of a 64-bit little-endian ELF file, in hex.
fn build_id(elf: &[u8]) -> Option<String> {
    let u16_at = |at: usize| Some(u16::from_le_bytes(elf.get(at..at + 2)?.try_into().ok()?));
    let u32_at = |at: usize| Some(u32::from_le_bytes(elf.get(at..at + 4)?.try_into().ok()?));
    let u64_at = |at: usize| {
        let bytes = elf.get(at..at + 8)?.try_into().ok()?;
        usize::try_from(u64::from_le_bytes(bytes)).ok()
    };
    // "\x7fELF", 64-bit, little-endian.
    if elf.get(..6)? != b"\x7fELF\x02\x01" {
        return None;
    }
    let (phoff, phentsize, phnum) = (u64_at(0x20)?, usize::from(u16_at(0x36)?), u16_at(0x38)?);
    for i in 0..usize::from(phnum) {
        let ph = phoff + i * phentsize;
        // PT_NOTE
        if u32_at(ph)? != 4 {
            continue;
        }
        let (mut at, end) = (u64_at(ph + 8)?, u64_at(ph + 8)? + u64_at(ph + 32)?);
        while at + 12 <= end {
            let namesz = u32_at(at)? as usize;
            let descsz = u32_at(at + 4)? as usize;
            let kind = u32_at(at + 8)?;
            let name_at = at + 12;
            let desc_at = name_at + namesz.next_multiple_of(4);
            // NT_GNU_BUILD_ID
            if kind == 3 && elf.get(name_at..name_at + namesz)? == b"GNU\0" {
                let desc = elf.get(desc_at..desc_at + descsz)?;
                return Some(desc.iter().map(|b| format!("{b:02x}")).collect());
            }
            at = desc_at + descsz.next_multiple_of(4);
        }
    }
    None
}

/// The first lines of every report: what crashed, when, and on what.
fn header(app: &str, kind: &str, when: SystemTime) -> String {
    let mut text = String::from("Katna crash report\n\n");
    let _ = writeln!(text, "App: {app} {VERSION}");
    let _ = writeln!(text, "Kind: {kind}");
    let _ = writeln!(text, "When: {}", utc(when));
    let _ = writeln!(text, "System: {}", system());
    text
}

/// "Arch Linux, KDE on wayland", from `/etc/os-release` and the session.
fn system() -> String {
    let os = fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|text| {
            text.lines().find_map(|line| {
                let value = line.strip_prefix("PRETTY_NAME=")?;
                Some(value.trim_matches('"').to_string())
            })
        })
        .unwrap_or_else(|| "Linux".into());
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
    let session = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
    match (desktop.is_empty(), session.is_empty()) {
        (true, true) => os,
        (false, true) => format!("{os}, {desktop}"),
        (true, false) => format!("{os}, {session}"),
        (false, false) => format!("{os}, {desktop} on {session}"),
    }
}

/// Writes `text` as a new report and deletes the oldest beyond [`KEEP`].
/// Returns the report's path.
fn write_report(
    dir: &Path,
    app: &str,
    when: SystemTime,
    pid: u32,
    text: &str,
) -> io::Result<PathBuf> {
    create_private_dir(dir)?;
    let path = dir.join(format!("{}-{app}-{pid}.txt", file_time(when)));
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    std::os::unix::fs::OpenOptionsExt::mode(&mut options, 0o600);
    let mut file = options.open(&path)?;
    file.write_all(text.as_bytes())?;
    prune(dir);
    Ok(path)
}

fn prune(dir: &Path) {
    for old in reports(dir).into_iter().skip(KEEP) {
        let _ = fs::remove_file(old.path);
    }
}

/// A saved crash report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    /// File name, such as `20260927T031603Z-katna-mail-4242.txt`. Names
    /// sort by time.
    pub name: String,
    pub path: PathBuf,
    /// The program that crashed, such as `katna-daemon`.
    pub app: String,
}

impl Report {
    fn from_path(path: PathBuf) -> Option<Self> {
        let name = path.file_name()?.to_str()?.to_string();
        let stem = name.strip_suffix(".txt")?;
        // <time>-<app>-<pid>; the app's name has dashes of its own.
        let (time, rest) = stem.split_once('-')?;
        let (app, pid) = rest.rsplit_once('-')?;
        if time.len() != 16 || !time.ends_with('Z') || pid.parse::<u32>().is_err() {
            return None;
        }
        Some(Self {
            app: app.to_string(),
            name,
            path,
        })
    }

    /// When the crash happened, in seconds since the epoch, from the name.
    pub fn unix_time(&self) -> Option<i64> {
        let t = self.name.get(..16)?;
        let num = |range: std::ops::Range<usize>| t.get(range)?.parse::<i64>().ok();
        let (y, mo, d) = (num(0..4)?, num(4..6)?, num(6..8)?);
        let (h, mi, s) = (num(9..11)?, num(11..13)?, num(13..15)?);
        // Howard Hinnant's days-from-civil algorithm.
        let y = if mo <= 2 { y - 1 } else { y };
        let era = y.div_euclid(400);
        let yoe = y - era * 400;
        let doy = (153 * (mo + if mo > 2 { -3 } else { 9 }) + 2) / 5 + d - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        let days = era * 146_097 + doe - 719_468;
        Some(days * 86_400 + h * 3600 + mi * 60 + s)
    }

    /// The report's text.
    pub fn read(&self) -> io::Result<String> {
        fs::read_to_string(&self.path)
    }
}

/// Every report in `dir`, newest first.
pub fn reports(dir: &Path) -> Vec<Report> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut reports: Vec<Report> = entries
        .filter_map(|entry| Report::from_path(entry.ok()?.path()))
        .collect();
    reports.sort_by(|a, b| b.name.cmp(&a.name));
    reports
}

/// Reports newer than the last one passed to [`mark_seen`], newest first.
pub fn unseen(dir: &Path) -> Vec<Report> {
    let seen = fs::read_to_string(dir.join(SEEN_FILE)).unwrap_or_default();
    let seen = seen.trim();
    reports(dir)
        .into_iter()
        .take_while(|report| report.name.as_str() > seen)
        .collect()
}

/// Remembers that the user has seen `report` and every older one.
pub fn mark_seen(dir: &Path, report: &Report) -> io::Result<()> {
    create_private_dir(dir)?;
    fs::write(dir.join(SEEN_FILE), &report.name)
}

/// Deletes every saved report.
pub fn delete_all(dir: &Path) -> io::Result<()> {
    for report in reports(dir) {
        fs::remove_file(report.path)?;
    }
    let _ = fs::remove_file(dir.join(SENT_FILE));
    Ok(())
}

/// The names of the reports sent to the crash tracker (or given up on).
pub fn sent(dir: &Path) -> Vec<String> {
    fs::read_to_string(dir.join(SENT_FILE))
        .unwrap_or_default()
        .lines()
        .map(str::to_string)
        .collect()
}

/// Reports not sent yet and at most [`SEND_WITHIN`] old at `now`, oldest
/// first.
pub fn unsent(dir: &Path, now: SystemTime) -> Vec<Report> {
    let sent = sent(dir);
    let oldest = now
        .checked_sub(SEND_WITHIN)
        .unwrap_or(UNIX_EPOCH)
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() as i64);
    let mut unsent: Vec<Report> = reports(dir)
        .into_iter()
        .filter(|report| !sent.contains(&report.name))
        .filter(|report| report.unix_time().is_some_and(|t| t >= oldest))
        .collect();
    unsent.reverse();
    unsent
}

/// Remembers that `report` was sent, so it is not sent again. Names of
/// reports that are gone are dropped from the list.
pub fn mark_sent(dir: &Path, report: &Report) -> io::Result<()> {
    let present: Vec<String> = reports(dir).into_iter().map(|r| r.name).collect();
    let mut sent: Vec<String> = sent(dir)
        .into_iter()
        .filter(|name| present.contains(name) && *name != report.name)
        .collect();
    sent.push(report.name.clone());
    create_private_dir(dir)?;
    fs::write(dir.join(SENT_FILE), sent.join("\n") + "\n")
}

/// Turns the core dumps `systemd-coredump` has of the programs `apps`
/// (by process name, such as `katna-mail`) into reports in
/// `paths.crash_dir()`, and returns the new reports.
///
/// Only core dumps newer than the last call are looked at (three days
/// back on the first call). Without `coredumpctl`, or when it fails, or
/// while saving crash reports is off ([`enabled`]), nothing happens. It
/// can take a few seconds: call it off the UI thread.
pub fn collect_core_dumps(paths: &Paths, apps: &[&str]) -> Vec<PathBuf> {
    if !enabled(&paths.config_file()) {
        return Vec::new();
    }
    let dir = paths.crash_dir();
    let last_file = dir.join(LAST_CORE_FILE);
    let since = fs::read_to_string(&last_file)
        .ok()
        .and_then(|text| text.trim().parse::<u64>().ok())
        .unwrap_or_else(|| {
            SystemTime::now()
                .checked_sub(FIRST_LOOK_BACK)
                .unwrap_or(UNIX_EPOCH)
                .duration_since(UNIX_EPOCH)
                .map_or(0, |d| d.as_micros() as u64)
        });
    let scrubber = Scrubber::from_system();
    let mut newest = since;
    let mut written = Vec::new();
    for app in apps {
        // `--since` takes whole seconds; entries at `since` itself are
        // skipped below.
        let since_arg = format!("--since=@{}", since / 1_000_000);
        let match_arg = format!("COREDUMP_COMM={app}");
        let Some(list) = coredumpctl(&["--json=short", "list", &since_arg, &match_arg]) else {
            continue;
        };
        for dump in parse_list(&list) {
            if dump.time <= since {
                continue;
            }
            newest = newest.max(dump.time);
            let pid = dump.pid.to_string();
            let info = coredumpctl(&["info", &format!("COREDUMP_PID={pid}"), &match_arg])
                .unwrap_or_default();
            let when = UNIX_EPOCH + Duration::from_micros(dump.time);
            let text = scrubber.scrub(&native_report(app, &dump, &info, when));
            match write_report(&dir, app, when, dump.pid, &text) {
                Ok(path) => written.push(path),
                // Another Katna program wrote this one first.
                Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {}
                Err(err) => tracing::warn!(%err, "could not save a crash report"),
            }
        }
    }
    if newest > since {
        let _ = create_private_dir(&dir);
        let _ = fs::write(&last_file, newest.to_string());
    }
    written
}

/// Runs `coredumpctl` with `args`; its output, or `None` when it is
/// missing, fails, finds nothing or takes too long.
fn coredumpctl(args: &[&str]) -> Option<String> {
    let mut child = Command::new("coredumpctl")
        .arg("--no-pager")
        .args(args)
        .env("SYSTEMD_COLORS", "0")
        .env("SYSTEMD_PAGER", "")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    let stdout = child.stdout.take()?;
    let reader = std::thread::spawn(move || io::read_to_string(stdout).ok());
    let started = std::time::Instant::now();
    loop {
        match child.try_wait().ok()? {
            Some(status) if status.success() => break,
            Some(_) => return None,
            None if started.elapsed() > COREDUMPCTL_TIMEOUT => {
                let _ = child.kill();
                let _ = child.wait();
                return None;
            }
            None => std::thread::sleep(Duration::from_millis(50)),
        }
    }
    reader.join().ok().flatten()
}

/// One entry of `coredumpctl --json=short list`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
struct CoreDump {
    /// Microseconds since the epoch.
    time: u64,
    pid: u32,
    #[serde(default)]
    sig: Option<i32>,
    #[serde(default)]
    exe: Option<String>,
}

fn parse_list(json: &str) -> Vec<CoreDump> {
    serde_json::from_str(json).unwrap_or_default()
}

fn signal_name(sig: i32) -> Option<&'static str> {
    Some(match sig {
        4 => "SIGILL",
        5 => "SIGTRAP",
        6 => "SIGABRT",
        7 => "SIGBUS",
        8 => "SIGFPE",
        11 => "SIGSEGV",
        31 => "SIGSYS",
        _ => return None,
    })
}

/// The report of a native crash, from its `coredumpctl info` output.
fn native_report(app: &str, dump: &CoreDump, info: &str, when: SystemTime) -> String {
    let kind = match dump.sig {
        Some(sig) => match signal_name(sig) {
            Some(name) => format!("native crash, signal {sig} ({name})"),
            None => format!("native crash, signal {sig}"),
        },
        None => "native crash".into(),
    };
    let mut text = header(app, &kind, when);
    // The version above is this build's; the crashed one may be older.
    let _ = writeln!(text, "Reported by: a later run of Katna");
    if let Some(exe) = &dump.exe {
        let _ = writeln!(text, "Executable: {exe}");
    }
    let stack = stack_of(info);
    if stack.is_empty() {
        let _ = writeln!(text, "\nNo stack trace was saved with the core dump.");
    } else {
        let _ = writeln!(text, "\n{stack}");
    }
    text
}

/// The useful part of `coredumpctl info`: the package line and the
/// modules and stacks that follow `Message:`. Host, boot and machine IDs,
/// units and the command line are left out.
fn stack_of(info: &str) -> String {
    let mut out = Vec::new();
    let mut in_message = false;
    for line in info.lines() {
        let trimmed = line.trim_start();
        if in_message {
            // Another dump's fields start again at the left.
            if trimmed.starts_with("PID:") {
                break;
            }
            out.push(line.trim_end());
        } else if trimmed.starts_with("Package:") {
            out.push(trimmed);
        } else if let Some(rest) = trimmed.strip_prefix("Message:") {
            in_message = true;
            out.push(rest.trim());
        }
    }
    while out.last().is_some_and(|line| line.trim().is_empty()) {
        out.pop();
    }
    if out.len() > MAX_STACK_LINES {
        let more = out.len() - MAX_STACK_LINES;
        out.truncate(MAX_STACK_LINES);
        return format!("{}\n({more} more lines left out)", out.join("\n"));
    }
    out.join("\n")
}

/// `2026-09-27 03:16:03 UTC`
fn utc(when: SystemTime) -> String {
    let (y, mo, d, h, mi, s) = civil(when);
    format!("{y:04}-{mo:02}-{d:02} {h:02}:{mi:02}:{s:02} UTC")
}

/// `2026-09-27T03:16:03Z`
pub(crate) fn rfc3339(when: SystemTime) -> String {
    let (y, mo, d, h, mi, s) = civil(when);
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{mi:02}:{s:02}Z")
}

/// `20260927T031603Z`, for file names.
fn file_time(when: SystemTime) -> String {
    let (y, mo, d, h, mi, s) = civil(when);
    format!("{y:04}{mo:02}{d:02}T{h:02}{mi:02}{s:02}Z")
}

/// Year, month, day, hour, minute, second in UTC.
fn civil(when: SystemTime) -> (i64, u32, u32, u32, u32, u32) {
    let secs = when.duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs()) as i64;
    let (days, rem) = (secs.div_euclid(86_400), secs.rem_euclid(86_400));
    // Howard Hinnant's days-to-civil algorithm.
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = yoe + era * 400 + i64::from(m <= 2);
    (
        y,
        m,
        d,
        (rem / 3600) as u32,
        (rem % 3600 / 60) as u32,
        (rem % 60) as u32,
    )
}

/// Replaces what could identify the user in a report's text.
#[derive(Debug, Clone, Default)]
pub struct Scrubber {
    home: Option<String>,
    user: Option<String>,
    host: Option<String>,
    machine_id: Option<String>,
}

impl Scrubber {
    /// Reads the home directory, user name, host name and machine ID of
    /// this system.
    pub fn from_system() -> Self {
        let read = |path: &str| {
            fs::read_to_string(path)
                .ok()
                .map(|text| text.trim().to_string())
        };
        Self::new(
            std::env::var("HOME").ok().as_deref(),
            std::env::var("USER").ok().as_deref(),
            read("/proc/sys/kernel/hostname").as_deref(),
            read("/etc/machine-id").as_deref(),
        )
    }

    pub fn new(
        home: Option<&str>,
        user: Option<&str>,
        host: Option<&str>,
        machine_id: Option<&str>,
    ) -> Self {
        // Too short or too common to replace without mangling the text.
        let usable = |value: Option<&str>, min: usize| {
            value
                .map(str::trim)
                .filter(|v| v.len() >= min && !matches!(*v, "root" | "localhost" | "user"))
                .map(str::to_string)
        };
        Self {
            home: usable(home.map(|h| h.trim_end_matches('/')), 2).filter(|h| h != "/"),
            user: usable(user, 3),
            host: usable(host, 3),
            machine_id: usable(machine_id, 8),
        }
    }

    /// `text` with the home directory as `~`, the machine ID, host name and
    /// user name as `<machine>`, `<host>` and `<user>`, URLs cut to their
    /// scheme and host (their paths and queries can carry tracking IDs and
    /// addresses), and email addresses as `<email>`. Percent-escapes are
    /// decoded first, so `ada%40example.org` is caught too.
    pub fn scrub(&self, text: &str) -> String {
        let mut text = emails(&urls(&percent_decode(text)));
        if let Some(home) = &self.home {
            text = text.replace(home.as_str(), "~");
        }
        if let Some(id) = &self.machine_id {
            text = text.replace(id.as_str(), "<machine>");
        }
        if let Some(host) = &self.host {
            text = whole_words(&text, host, "<host>");
        }
        if let Some(user) = &self.user {
            text = whole_words(&text, user, "<user>");
        }
        text
    }
}

/// Replaces `word` where it is not part of a longer word.
fn whole_words(text: &str, word: &str, with: &str) -> String {
    let is_word = |c: char| c.is_alphanumeric() || c == '_';
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find(word) {
        let before = rest[..at].chars().next_back();
        let after = rest[at + word.len()..].chars().next();
        out.push_str(&rest[..at]);
        if before.is_some_and(is_word) || after.is_some_and(is_word) {
            out.push_str(word);
        } else {
            out.push_str(with);
        }
        rest = &rest[at + word.len()..];
    }
    out.push_str(rest);
    out
}

/// Replaces everything shaped like `local@domain.tld` with `<email>`, in
/// any script: the local part is any run of characters that are not
/// spaces, `@` or punctuation that surrounds addresses (`<>()[]{}"',;:`),
/// the domain letters, digits, dots and dashes ending in a top-level
/// domain of two or more letters. Code such as `memcpy@@GLIBC_2.14` or
/// `v1.2@3.4` is left alone.
fn emails(text: &str) -> String {
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let local = |c: char| !c.is_whitespace() && !c.is_control() && !"@<>()[]{}\"',;:`".contains(c);
    let domain = |c: char| c.is_alphanumeric() || c == '.' || c == '-';
    let offset = |i: usize| chars.get(i).map_or(text.len(), |(at, _)| *at);
    let mut out = String::with_capacity(text.len());
    // In characters: how far `out` has copied, and where we look.
    let mut copied = 0;
    let mut i = 0;
    while i < chars.len() {
        if chars[i].1 != '@' {
            i += 1;
            continue;
        }
        let mut start = i;
        while start > copied && local(chars[start - 1].1) {
            start -= 1;
        }
        let mut end = i + 1;
        while end < chars.len() && domain(chars[end].1) {
            end += 1;
        }
        // A sentence's full stop is not part of the domain.
        while end > i + 1 && chars[end - 1].1 == '.' {
            end -= 1;
        }
        let host = &text[offset(i + 1)..offset(end)];
        let tld = host.rsplit('.').next().unwrap_or("");
        if start < i
            && host.contains('.')
            && tld.chars().count() >= 2
            && tld.chars().all(char::is_alphabetic)
        {
            out.push_str(&text[offset(copied)..offset(start)]);
            out.push_str("<email>");
            copied = end;
            i = end;
        } else {
            i += 1;
        }
    }
    out.push_str(&text[offset(copied)..]);
    out
}

/// `text` with `%XX` escapes decoded (invalid UTF-8 that results is
/// replaced); anything else is kept.
fn percent_decode(text: &str) -> String {
    if !text.contains('%') {
        return text.to_string();
    }
    let bytes = text.as_bytes();
    let hex = |b: u8| char::from(b).to_digit(16);
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && let (Some(high), Some(low)) = (
                bytes.get(i + 1).copied().and_then(hex),
                bytes.get(i + 2).copied().and_then(hex),
            )
        {
            out.push((high * 16 + low) as u8);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// `text` with every `scheme://…` URL cut to `scheme://host`: no user
/// name, port, path, query or fragment.
fn urls(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find("://") {
        let before = &rest[..at];
        let scheme_len = before
            .chars()
            .rev()
            .take_while(|c| c.is_ascii_alphanumeric() || "+.-".contains(*c))
            .count();
        let scheme = &before[before.len() - scheme_len..];
        // Not a URL: copy through the `://` and go on.
        if !scheme.starts_with(|c: char| c.is_ascii_alphabetic()) {
            out.push_str(&rest[..at + 3]);
            rest = &rest[at + 3..];
            continue;
        }
        let after = &rest[at + 3..];
        let end = after
            .find(|c: char| c.is_whitespace() || "\"'<>`".contains(c))
            .unwrap_or(after.len());
        let authority = after[..end].split(['/', '?', '#']).next().unwrap_or("");
        let host = authority.rsplit('@').next().unwrap_or("");
        let host = match host.rfind(':') {
            // A port, but not inside an IPv6 literal.
            Some(colon) if !host[colon..].contains(']') => &host[..colon],
            _ => host,
        };
        out.push_str(before);
        out.push_str("://");
        out.push_str(host);
        rest = &after[end..];
    }
    out.push_str(rest);
    out
}

/// Longest panic message kept, in characters.
const MAX_MESSAGE: usize = 200;

/// A panic message without the text it quotes: Rust's messages quote the
/// string or value involved in `` `…` `` or `"…"` (a slice of a subject
/// or a body, say), so those spans become `…`. Cut to [`MAX_MESSAGE`]
/// characters.
fn panic_message(message: &str) -> String {
    let mut out = String::with_capacity(message.len());
    let mut rest = message;
    while let Some(open) = rest.find(['`', '"']) {
        let quote = &rest[open..open + 1];
        let Some(close) = rest[open + 1..].find(quote) else {
            break;
        };
        out.push_str(&rest[..open]);
        out.push_str(quote);
        out.push('…');
        out.push_str(quote);
        rest = &rest[open + 1 + close + 1..];
    }
    out.push_str(rest);
    if out.chars().count() > MAX_MESSAGE {
        out = out.chars().take(MAX_MESSAGE).collect::<String>() + "…";
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(secs: u64) -> SystemTime {
        UNIX_EPOCH + Duration::from_secs(secs)
    }

    #[test]
    fn formats_utc_times() {
        assert_eq!(utc(at(0)), "1970-01-01 00:00:00 UTC");
        // 2026-09-27 03:16:03 UTC
        assert_eq!(utc(at(1_790_478_963)), "2026-09-27 03:16:03 UTC");
        assert_eq!(file_time(at(1_790_478_963)), "20260927T031603Z");
        assert_eq!(rfc3339(at(1_790_478_963)), "2026-09-27T03:16:03Z");
        // A leap day.
        assert_eq!(utc(at(1_709_164_800)), "2024-02-29 00:00:00 UTC");
    }

    #[test]
    fn scrubs_personal_details() {
        let scrubber = Scrubber::new(
            Some("/home/mz/"),
            Some("mz1"),
            Some("mzarch"),
            Some("0123456789abcdef0123456789abcdef"),
        );
        let text = "panicked at /home/mz/src/katna/x.rs on mzarch for mz1 \
                    (machine 0123456789abcdef0123456789abcdef), \
                    from Ada <ada.l+x@mail.example.org>. Also mz1x and ada@.";
        assert_eq!(
            scrubber.scrub(text),
            "panicked at ~/src/katna/x.rs on <host> for <user> \
             (machine <machine>), from Ada <<email>>. Also mz1x and ada@."
        );
    }

    #[test]
    fn leaves_code_alone() {
        let text = "  3: katna_mail::window::MailWindow::render\n  \
                    memcpy@@GLIBC_2.14 (libc.so.6 + 0x1234)\n  v1.2@3.4";
        assert_eq!(Scrubber::default().scrub(text), text);
        assert_eq!(emails("write to a@b.co."), "write to <email>.");
        assert_eq!(emails("x@y"), "x@y");
    }

    #[test]
    fn scrubs_urls_encoded_and_non_ascii_addresses() {
        let scrubber = Scrubber::default();
        assert_eq!(
            scrubber.scrub(
                "GET https://autoconfig.example.org/mail/config-v1.1.xml?emailaddress=ada%40example.org done"
            ),
            "GET https://autoconfig.example.org done"
        );
        assert_eq!(
            scrubber.scrub("image url=https://user:pw@t.example:8443/o/8f3a91.png?u=ada\n"),
            "image url=https://t.example\n"
        );
        assert_eq!(
            scrubber.scrub("from \"jö.ü@exämple.de\" and ада@пример.рф, and ada%40example.org"),
            "from \"<email>\" and <email>, and <email>"
        );
        assert_eq!(
            scrubber.scrub("imaps://[2001:db8::1]:993/INBOX and file:///tmp/x"),
            "imaps://[2001:db8::1] and file://"
        );
        // Not URLs, and escapes that decode to nothing valid.
        assert_eq!(
            scrubber.scrub("a :// b, 100%, 50%zz"),
            "a :// b, 100%, 50%zz"
        );
    }

    #[test]
    fn panic_messages_lose_what_they_quote() {
        assert_eq!(
            panic_message(
                "byte index 5 is not a char boundary; it is inside 'é' (bytes 4..6) of `Dear Ada, the invoice`"
            ),
            "byte index 5 is not a char boundary; it is inside 'é' (bytes 4..6) of `…`"
        );
        assert_eq!(
            panic_message(
                "called `Result::unwrap()` on an `Err` value: Parse(\"Subject: secret\")"
            ),
            "called `…` on an `…` value: Parse(\"…\")"
        );
        assert_eq!(panic_message("boom in a test"), "boom in a test");
        assert_eq!(panic_message("odd ` quote"), "odd ` quote");
        let long = "x".repeat(500);
        assert_eq!(panic_message(&long).chars().count(), MAX_MESSAGE + 1);
    }

    #[cfg(unix)]
    #[test]
    fn reports_are_private() {
        use std::os::unix::fs::PermissionsExt;
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("state").join("crashes");
        let path = write_report(&dir, "katna-mail", at(1_790_000_000), 7, "text").unwrap();
        let mode = |p: &Path| fs::metadata(p).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode(&dir), 0o700);
        assert_eq!(mode(dir.parent().unwrap()), 0o700);
        assert_eq!(mode(&path), 0o600);
    }

    #[test]
    fn short_or_common_names_are_not_replaced() {
        let scrubber = Scrubber::new(Some("/"), Some("ab"), Some("localhost"), Some("abc"));
        let text = "ab localhost abc /usr/bin";
        assert_eq!(scrubber.scrub(text), text);
    }

    #[test]
    fn writes_lists_and_prunes_reports() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("crashes");
        for i in 0..(KEEP as u64 + 3) {
            write_report(&dir, "katna-mail", at(1_790_000_000 + i), 7, "text").unwrap();
        }
        write_report(&dir, "katna-daemon", at(1_790_000_100), 8, "daemon").unwrap();
        // The same crash twice is refused.
        let again = write_report(&dir, "katna-daemon", at(1_790_000_100), 8, "x");
        assert_eq!(again.unwrap_err().kind(), io::ErrorKind::AlreadyExists);
        fs::write(dir.join("notes.txt"), "not a report").unwrap();

        let all = reports(&dir);
        assert_eq!(all.len(), KEEP);
        assert_eq!(all[0].app, "katna-daemon");
        assert_eq!(all[0].read().unwrap(), "daemon");
        assert_eq!(all[1].app, "katna-mail");
        assert!(all.windows(2).all(|w| w[0].name > w[1].name));

        assert_eq!(unseen(&dir).len(), KEEP);
        mark_seen(&dir, &all[1]).unwrap();
        assert_eq!(unseen(&dir), vec![all[0].clone()]);
        mark_seen(&dir, &all[0]).unwrap();
        assert!(unseen(&dir).is_empty());

        delete_all(&dir).unwrap();
        assert!(reports(&dir).is_empty());
        assert!(dir.join("notes.txt").exists());
    }

    #[test]
    fn remembers_what_was_sent() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("crashes");
        let now = at(1_790_478_963);
        let day = 24 * 60 * 60;
        write_report(&dir, "katna-mail", at(1_790_478_963 - 8 * day), 1, "old").unwrap();
        write_report(&dir, "katna-mail", at(1_790_478_963 - day), 2, "a").unwrap();
        write_report(&dir, "katna-daemon", at(1_790_478_963 - 60), 3, "b").unwrap();

        // Oldest first, and not the one from before the last week.
        let unsent = unsent(&dir, now);
        let apps: Vec<&str> = unsent.iter().map(|r| r.app.as_str()).collect();
        assert_eq!(apps, ["katna-mail", "katna-daemon"]);

        mark_sent(&dir, &unsent[0]).unwrap();
        mark_sent(&dir, &unsent[0]).unwrap();
        assert_eq!(super::unsent(&dir, now), vec![unsent[1].clone()]);
        assert_eq!(sent(&dir), vec![unsent[0].name.clone()]);

        // Names of deleted reports are dropped.
        fs::remove_file(&unsent[0].path).unwrap();
        mark_sent(&dir, &unsent[1]).unwrap();
        assert_eq!(sent(&dir), vec![unsent[1].name.clone()]);
        assert!(super::unsent(&dir, now).is_empty());

        delete_all(&dir).unwrap();
        assert!(sent(&dir).is_empty());
    }

    #[test]
    fn reads_report_names() {
        let report = Report::from_path("/x/20260927T031603Z-katna-daemon-42.txt".into()).unwrap();
        assert_eq!(report.app, "katna-daemon");
        assert_eq!(report.unix_time(), Some(1_790_478_963));
        let leap = Report::from_path("20240229T000000Z-katna-mail-1.txt".into()).unwrap();
        assert_eq!(leap.unix_time(), Some(1_709_164_800));
        for name in [
            "seen",
            "last-core",
            "x-katna-mail-1.txt",
            "20260927T031603Z-mail-x.txt",
        ] {
            assert!(Report::from_path(PathBuf::from(name)).is_none(), "{name}");
        }
    }

    #[test]
    fn parses_coredumpctl_output() {
        let list = r#"[{"time":1790478963123456,"pid":4242,"uid":1000,"gid":1000,
            "sig":11,"corefile":"present","exe":"/usr/bin/katna-mail","size":1234}]"#;
        let dumps = parse_list(list);
        assert_eq!(dumps.len(), 1);
        assert_eq!(dumps[0].pid, 4242);
        assert_eq!(dumps[0].sig, Some(11));
        assert!(parse_list("No coredumps found.").is_empty());

        let info = "           PID: 4242 (katna-mail)
           UID: 1000 (mz)
        Signal: 11 (SEGV)
     Timestamp: Sun 2026-09-27 03:16:03 IST (1min ago)
  Command Line: katna-mail --message 12
    Executable: /usr/bin/katna-mail
      Hostname: mzarch
       Package: katna-git/0.0.0.r512.g4040a0e-1
       Message: Process 4242 (katna-mail) of user 1000 dumped core.

                Module /usr/bin/katna-mail with build-id 0123abcd
                Stack trace of thread 4242:
                #0  0x000055d0c0ffee00 n/a (/usr/bin/katna-mail + 0x1234)
                #1  0x00007f0000000000 __libc_start_main (libc.so.6 + 0x27cd0)
";
        let stack = stack_of(info);
        assert!(stack.starts_with("Package: katna-git/0.0.0.r512.g4040a0e-1\n"));
        assert!(stack.contains("Stack trace of thread 4242:"));
        assert!(stack.contains("#1  0x00007f0000000000 __libc_start_main"));
        assert!(!stack.contains("Hostname"));
        assert!(!stack.contains("--message"));

        let dump = &dumps[0];
        let report = native_report("katna-mail", dump, info, at(1_790_478_963));
        assert!(report.contains("Kind: native crash, signal 11 (SIGSEGV)"));
        assert!(report.contains("When: 2026-09-27 03:16:03 UTC"));
        assert!(report.contains("Executable: /usr/bin/katna-mail"));
        let empty = native_report("katna-mail", dump, "", at(0));
        assert!(empty.contains("No stack trace was saved"));
    }

    #[test]
    fn maps_addresses_to_modules() {
        let maps = "\
55d000000000-55d000001000 r--p 00000000 fd:01 42 /usr/bin/katna-mail
55d000001000-55d000009000 r-xp 00001000 fd:01 42 /usr/bin/katna-mail
7f0000000000-7f0000028000 r--p 00000000 fd:01 7 /usr/lib/libc.so.6
7f0000028000-7f00001a0000 r-xp 00028000 fd:01 7 /usr/lib/libc.so.6
7ffd00000000-7ffd00021000 rw-p 00000000 00:00 0 [stack]
";
        let modules = modules(maps);
        assert_eq!(modules.len(), 2);
        assert_eq!(modules[0].name, "katna-mail");
        assert_eq!(modules[0].base, 0x55d0_0000_0000);
        assert_eq!(modules[1].name, "libc.so.6");
        assert_eq!(modules[1].start, 0x7f00_0002_8000);
        assert_eq!(modules[1].base, 0x7f00_0000_0000);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn frames_name_their_modules() {
        let frames = raw_frames();
        assert!(frames.lines().count() > 3, "{frames}");
        assert!(frames.contains(" + 0x"), "{frames}");
    }

    #[test]
    fn reads_the_build_id() {
        // A minimal ELF header with one PT_NOTE holding a GNU build ID.
        let mut elf = vec![0u8; 0x40 + 0x38];
        elf[..6].copy_from_slice(b"\x7fELF\x02\x01");
        elf[0x20..0x28].copy_from_slice(&0x40u64.to_le_bytes());
        elf[0x36..0x38].copy_from_slice(&0x38u16.to_le_bytes());
        elf[0x38..0x3a].copy_from_slice(&1u16.to_le_bytes());
        let note_at = elf.len() as u64;
        let mut note = Vec::new();
        note.extend_from_slice(&4u32.to_le_bytes());
        note.extend_from_slice(&4u32.to_le_bytes());
        note.extend_from_slice(&3u32.to_le_bytes());
        note.extend_from_slice(b"GNU\0");
        note.extend_from_slice(&[0xde, 0xad, 0xbe, 0xef]);
        elf[0x40..0x44].copy_from_slice(&4u32.to_le_bytes());
        elf[0x48..0x50].copy_from_slice(&note_at.to_le_bytes());
        elf[0x60..0x68].copy_from_slice(&(note.len() as u64).to_le_bytes());
        elf.extend_from_slice(&note);
        assert_eq!(build_id(&elf).as_deref(), Some("deadbeef"));
        assert_eq!(build_id(b"not an elf"), None);
        // The test binary itself, when the linker gave it one.
        if let Some(id) = elf_head("/proc/self/exe").and_then(|e| build_id(&e)) {
            assert!(
                id.len() >= 16 && id.bytes().all(|b| b.is_ascii_hexdigit()),
                "{id}"
            );
        }
    }

    #[test]
    fn the_setting_turns_reports_off() {
        let tmp = tempfile::tempdir().unwrap();
        let file = tmp.path().join("config.toml");
        assert!(enabled(&file));
        fs::write(&file, "[feedback]\nsave_crash_reports = false\n").unwrap();
        assert!(!enabled(&file));
        let paths = Paths::with_root(tmp.path());
        fs::create_dir_all(paths.config_dir()).unwrap();
        fs::copy(&file, paths.config_file()).unwrap();
        assert!(collect_core_dumps(&paths, &["katna-mail"]).is_empty());
        assert!(!paths.crash_dir().exists());
    }

    #[test]
    fn panic_hook_writes_a_report() {
        // The hook is process-wide; this is the only test that installs it.
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        install("katna-test", &paths);
        let _ = std::thread::Builder::new()
            .name("crasher".into())
            .spawn(|| panic!("boom in {}", "a test"))
            .unwrap()
            .join();
        // Other tests may panic while the hook is installed.
        let text = reports(&paths.crash_dir())
            .into_iter()
            .map(|report| report.read().unwrap())
            .find(|text| text.contains("Thread: crasher"))
            .expect("a report of the panic");
        assert!(text.starts_with("Katna crash report\n"), "{text}");
        assert!(text.contains("Kind: panic"), "{text}");
        assert!(text.contains("Thread: crasher"), "{text}");
        assert!(text.contains("Message: boom in a test"), "{text}");
        assert!(text.contains("crash.rs"), "{text}");
        assert!(text.contains("Backtrace:"), "{text}");
        assert!(text.contains("Frames:\n  #0 "), "{text}");
    }
}
