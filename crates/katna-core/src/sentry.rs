// SPDX-License-Identifier: GPL-3.0-or-later

//! Crash reports as Sentry envelopes (`docs/ARCHITECTURE.md` §19.2,
//! Part 2). Only used once the user has agreed to send crash reports; the
//! daemon posts what [`envelope`] builds.
//!
//! The event is read back from the report's own text, and the text itself
//! goes along as an attachment, so what is sent is exactly the report the
//! user can open in Settings > User feedback. No user, IP address, host
//! name or device ID is added.

use std::fmt::Write as _;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde_json::{Map, Value, json};

use crate::crash::{Report, VERSION, rfc3339};

/// Where and how to post envelopes, from a DSN such as
/// `https://<key>@o1.ingest.de.sentry.io/<project>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dsn {
    dsn: String,
    key: String,
    url: String,
}

impl Dsn {
    /// Reads a DSN; only `https` ones are accepted.
    pub fn parse(dsn: &str) -> Option<Self> {
        let dsn = dsn.trim();
        let rest = dsn.strip_prefix("https://")?;
        let (auth, rest) = rest.split_once('@')?;
        // Old DSNs carry `key:secret`; only the public key is used.
        let key = auth.split(':').next()?;
        let (host, path) = rest.split_once('/')?;
        let path = path.trim_end_matches('/');
        let (prefix, project) = match path.rsplit_once('/') {
            Some((prefix, project)) => (format!("/{prefix}"), project),
            None => (String::new(), path),
        };
        let fine = |part: &str| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || "-._:".contains(c))
        };
        if !fine(key) || !fine(host) || !fine(project) {
            return None;
        }
        Some(Self {
            dsn: dsn.to_string(),
            key: key.to_string(),
            url: format!("https://{host}{prefix}/api/{project}/envelope/"),
        })
    }

    /// The URL envelopes are posted to.
    pub fn envelope_url(&self) -> &str {
        &self.url
    }

    /// The `X-Sentry-Auth` header.
    pub fn auth_header(&self) -> String {
        format!(
            "Sentry sentry_version=7, sentry_key={}, sentry_client=katna/{VERSION}",
            self.key
        )
    }
}

/// The content type of an envelope.
pub const CONTENT_TYPE: &str = "application/x-sentry-envelope";

/// The envelope for `report`, whose text is `text`: an error event and
/// the text as an attachment. The event ID comes from the report, so a
/// report sent twice is kept once.
pub fn envelope(dsn: &Dsn, report: &Report, text: &str, now: SystemTime) -> Vec<u8> {
    let id = event_id(report, text);
    let event = event(&id, report, text);
    let header = json!({
        "event_id": id,
        "dsn": dsn.dsn,
        "sent_at": rfc3339(now),
        "sdk": { "name": "katna", "version": VERSION },
    });
    let event = event.to_string();
    let mut out = String::new();
    let _ = writeln!(out, "{header}");
    let _ = writeln!(
        out,
        "{}",
        json!({ "type": "event", "length": event.len(), "content_type": "application/json" })
    );
    let _ = writeln!(out, "{event}");
    let _ = writeln!(
        out,
        "{}",
        json!({
            "type": "attachment",
            "length": text.len(),
            "filename": report.name,
            "content_type": "text/plain",
        })
    );
    out.push_str(text);
    out.push('\n');
    out.into_bytes()
}

/// 32 hex digits from the report's name and text.
fn event_id(report: &Report, text: &str) -> String {
    let half = |salt: u8| {
        let mut hasher = DefaultHasher::new();
        salt.hash(&mut hasher);
        report.name.hash(&mut hasher);
        text.hash(&mut hasher);
        hasher.finish()
    };
    format!("{:016x}{:016x}", half(1), half(2))
}

/// What a report says, read from its text (`crash::panic_report` and
/// `crash::native_report` write it).
#[derive(Debug, Default, PartialEq, Eq)]
struct Parsed<'a> {
    version: &'a str,
    kind: &'a str,
    system: &'a str,
    thread: &'a str,
    location: &'a str,
    message: &'a str,
    /// Rust's backtrace: function, file, line.
    backtrace: Vec<(&'a str, Option<&'a str>, Option<u32>)>,
    /// `module + 0xoffset` frames of a panic.
    frames: Vec<(&'a str, u64)>,
    build_id: &'a str,
    /// Modules and their build IDs, from `coredumpctl`.
    modules: Vec<(&'a str, &'a str)>,
    /// The crashed thread's stack from `coredumpctl`: function (if known),
    /// module, offset.
    stack: Vec<(Option<&'a str>, &'a str, u64)>,
}

fn parse(text: &str) -> Parsed<'_> {
    #[derive(PartialEq)]
    enum Part {
        Head,
        Backtrace,
        Frames,
        Stack,
        Rest,
    }
    let mut parsed = Parsed::default();
    let mut part = Part::Head;
    for line in text.lines() {
        let trimmed = line.trim();
        let field = |name: &str| trimmed.strip_prefix(name).map(str::trim);
        if trimmed == "Backtrace:" {
            part = Part::Backtrace;
            continue;
        }
        if trimmed == "Frames:" {
            part = Part::Frames;
            continue;
        }
        if trimmed == "Recent log:" {
            part = Part::Rest;
            continue;
        }
        if trimmed.starts_with("Stack trace of thread") {
            // Only the first thread: the one that crashed.
            part = if parsed.stack.is_empty() {
                Part::Stack
            } else {
                Part::Rest
            };
            continue;
        }
        if let Some(module) = trimmed.strip_prefix("Module ") {
            let name = module.split_whitespace().next().unwrap_or_default();
            let name = name.rsplit('/').next().unwrap_or(name);
            if let Some((_, id)) = module.split_once("build-id ") {
                let id = id.split_whitespace().next().unwrap_or_default();
                parsed.modules.push((name, id.trim_end_matches('.')));
            }
            continue;
        }
        match part {
            Part::Head => {
                if let Some(app) = field("App:") {
                    parsed.version = app.split_once(' ').map_or("", |(_, v)| v);
                } else if let Some(kind) = field("Kind:") {
                    parsed.kind = kind;
                } else if let Some(system) = field("System:") {
                    parsed.system = system;
                } else if let Some(thread) = field("Thread:") {
                    parsed.thread = thread;
                } else if let Some(location) = field("Location:") {
                    parsed.location = location;
                } else if let Some(message) = field("Message:") {
                    parsed.message = message;
                }
            }
            Part::Backtrace => {
                if let Some(at) = trimmed.strip_prefix("at ") {
                    if let Some(last) = parsed.backtrace.last_mut() {
                        let mut parts = at.rsplitn(3, ':');
                        let (_column, line, file) = (parts.next(), parts.next(), parts.next());
                        last.1 = file.or(Some(at));
                        last.2 = line.and_then(|l| l.parse().ok());
                    }
                } else if let Some((number, function)) = trimmed.split_once(": ")
                    && number.parse::<u32>().is_ok()
                {
                    parsed.backtrace.push((function, None, None));
                }
            }
            Part::Frames => {
                if let Some(id) = field("Build ID:") {
                    parsed.build_id = id;
                } else if let Some((module, offset)) = trimmed
                    .strip_prefix('#')
                    .and_then(|rest| rest.split_once(char::is_whitespace))
                    .and_then(|(_, rest)| module_offset(rest.trim()))
                {
                    parsed.frames.push((module, offset));
                }
            }
            Part::Stack => {
                if trimmed.is_empty() {
                    part = Part::Rest;
                    continue;
                }
                // `#0  0x00007f… function (module + 0xoffset)`
                let Some(rest) = trimmed.strip_prefix('#') else {
                    continue;
                };
                let mut words = rest.split_whitespace();
                let (_number, _address) = (words.next(), words.next());
                let rest: Vec<&str> = words.collect();
                let Some(open) = rest.iter().position(|w| w.starts_with('(')) else {
                    continue;
                };
                let function = (open > 0 && rest[0] != "n/a").then_some(rest[0]);
                let module = rest[open].trim_start_matches('(');
                let offset = rest
                    .get(open + 2)
                    .and_then(|o| o.trim_end_matches(')').strip_prefix("0x"))
                    .and_then(|o| u64::from_str_radix(o, 16).ok());
                if let (Some(offset), Some(&"+")) = (offset, rest.get(open + 1)) {
                    parsed.stack.push((function, module, offset));
                }
            }
            Part::Rest => {}
        }
    }
    parsed
}

/// `katna-mail + 0x1f2e3`
fn module_offset(text: &str) -> Option<(&str, u64)> {
    let (module, offset) = text.split_once(" + ")?;
    let offset = offset.trim().strip_prefix("0x")?;
    Some((module.trim(), u64::from_str_radix(offset, 16).ok()?))
}

/// The Sentry debug ID of an ELF build ID: its first 16 bytes as a GUID,
/// the first three fields byte-swapped (as `sentry-cli` computes it).
fn debug_id(build_id: &str) -> Option<String> {
    let bytes: Vec<u8> = (0..build_id.len() / 2)
        .map(|i| u8::from_str_radix(build_id.get(i * 2..i * 2 + 2)?, 16).ok())
        .collect::<Option<_>>()?;
    if bytes.is_empty() {
        return None;
    }
    let mut guid = [0u8; 16];
    for (to, from) in guid.iter_mut().zip(&bytes) {
        *to = *from;
    }
    guid[..4].reverse();
    guid[4..6].reverse();
    guid[6..8].reverse();
    let hex: String = guid.iter().map(|b| format!("{b:02x}")).collect();
    Some(format!(
        "{}-{}-{}-{}-{}",
        &hex[..8],
        &hex[8..12],
        &hex[12..16],
        &hex[16..20],
        &hex[20..]
    ))
}

/// An ELF image for `debug_meta`, addressed relative to itself.
fn image(file: &str, build_id: &str) -> Option<Value> {
    Some(json!({
        "type": "elf",
        "code_file": file,
        "code_id": build_id,
        "debug_id": debug_id(build_id)?,
        "image_addr": "0x0",
    }))
}

fn in_app(function: &str) -> bool {
    function.contains("katna_")
}

/// The Sentry event for a report.
fn event(id: &str, report: &Report, text: &str) -> Value {
    let parsed = parse(text);
    let native = parsed.kind.starts_with("native");
    let mut images: Vec<Value> = Vec::new();
    let image_of = |module: &str, build_id: &str, images: &mut Vec<Value>| -> Option<usize> {
        let at = images.iter().position(|i| i["code_file"] == module);
        at.or_else(|| {
            images.push(image(module, build_id)?);
            Some(images.len() - 1)
        })
    };
    let relative = |function: Option<&str>, module: &str, offset: u64, image: Option<usize>| {
        let mut frame = Map::new();
        frame.insert("package".into(), module.into());
        if let Some(function) = function {
            frame.insert("function".into(), function.into());
            frame.insert("in_app".into(), in_app(function).into());
        }
        match image {
            Some(image) => {
                frame.insert("instruction_addr".into(), format!("{offset:#x}").into());
                frame.insert("addr_mode".into(), format!("rel:{image}").into());
            }
            None if function.is_none() => {
                frame.insert("function".into(), format!("{module} + {offset:#x}").into());
            }
            None => {}
        }
        Value::Object(frame)
    };

    let mut frames: Vec<Value> = if native {
        parsed
            .stack
            .iter()
            .map(|&(function, module, offset)| {
                let build_id = parsed.modules.iter().find(|(m, _)| *m == module);
                let image = build_id.and_then(|(_, id)| image_of(module, id, &mut images));
                relative(function, module, offset, image)
            })
            .collect()
    } else if parsed
        .backtrace
        .iter()
        .any(|(function, _, _)| in_app(function))
    {
        // Names from the binary's own symbols; the frames of the panic
        // machinery and of the hook are left out.
        let start = parsed
            .backtrace
            .iter()
            .rposition(|(f, _, _)| {
                f.contains("panic_fmt")
                    || f.contains("rust_begin_unwind")
                    || f.contains("begin_panic")
            })
            .map_or(0, |i| i + 1);
        parsed.backtrace[start..]
            .iter()
            .map(|&(function, file, line)| {
                let mut frame = Map::new();
                frame.insert("function".into(), function.into());
                frame.insert("in_app".into(), in_app(function).into());
                if let Some(file) = file {
                    frame.insert("filename".into(), file.into());
                }
                if let Some(line) = line {
                    frame.insert("lineno".into(), line.into());
                }
                Value::Object(frame)
            })
            .collect()
    } else {
        // A stripped build: addresses in the executable, for the debug
        // files CI uploads.
        let exe = std::path::Path::new(&report.app)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&report.app);
        parsed
            .frames
            .iter()
            .map(|&(module, offset)| {
                let image = (module == exe && !parsed.build_id.is_empty())
                    .then(|| image_of(module, parsed.build_id, &mut images))
                    .flatten();
                relative(None, module, offset, image)
            })
            .collect()
    };
    // Sentry lists the outermost call first.
    frames.reverse();

    let (kind, value) = if native {
        let signal = parsed
            .kind
            .split_once('(')
            .map(|(_, s)| s.trim_end_matches(')'))
            .unwrap_or("native crash");
        (
            signal.to_string(),
            format!("{} crashed: {}", report.app, parsed.kind),
        )
    } else {
        ("panic".to_string(), parsed.message.to_string())
    };
    let mut exception = json!({
        "type": kind,
        "value": value,
        "mechanism": { "type": if native { "coredump" } else { "panic" }, "handled": false },
    });
    if !frames.is_empty() {
        exception["stacktrace"] = json!({ "frames": frames });
    }
    let timestamp = report
        .unix_time()
        .and_then(|t| u64::try_from(t).ok())
        .map_or_else(SystemTime::now, |t| UNIX_EPOCH + Duration::from_secs(t));
    let version = if parsed.version.is_empty() {
        VERSION
    } else {
        parsed.version
    };
    let mut event = json!({
        "event_id": id,
        "timestamp": rfc3339(timestamp),
        "platform": "native",
        "level": "fatal",
        "logger": report.app,
        "release": format!("katna@{version}"),
        "exception": { "values": [exception] },
        "tags": {
            "app": report.app,
            "kind": if native { "native" } else { "panic" },
        },
        "contexts": {
            "os": { "type": "os", "name": "Linux", "raw_description": parsed.system },
            "app": { "type": "app", "app_name": report.app, "app_version": version },
        },
        "extra": {
            "report": report.name,
            "kind": parsed.kind,
            "thread": parsed.thread,
            "location": parsed.location,
        },
    });
    if !images.is_empty() {
        event["debug_meta"] = json!({ "images": images });
    }
    event
}

#[cfg(test)]
mod tests {
    use super::*;

    const PANIC: &str = "Katna crash report

App: katna-mail 0.0.0.r600.gabc1234
Kind: panic
When: 2026-09-27 03:16:03 UTC
System: Arch Linux, KDE on wayland
Thread: main
Location: apps/katna-mail/src/window/list.rs:120:9
Message: called `Option::unwrap()` on a `None` value

Backtrace:
   0: std::backtrace::Backtrace::force_capture
   1: katna_core::crash::install::{{closure}}
   2: std::panicking::rust_panic_with_hook
   3: __rustc::rust_begin_unwind
   4: core::panicking::panic_fmt
   5: core::option::unwrap_failed
   6: katna_mail::window::list::render_row
             at ./apps/katna-mail/src/window/list.rs:120:9
   7: main

Frames:
  #0   katna-mail + 0x1c2d3e
  #1   libc.so.6 + 0x2724a
Build ID: 0123456789abcdef0011223344556677deadbeef

Recent log:
2026-09-27T03:16:02Z  INFO mail loaded
";

    const STRIPPED: &str = "Katna crash report

App: katna-daemon 0.0.0.r600.gabc1234
Kind: panic
System: Arch Linux
Thread: sync
Location: crates/katna-sync/src/worker.rs:88:5
Message: boom

Backtrace:
   0: <unknown>
   1: <unknown>

Frames:
  #0   katna-daemon + 0x1000
  #1   katna-daemon + 0x2000
  #2   libc.so.6 + 0x2724a
Build ID: 0123456789abcdef0011223344556677deadbeef
";

    const NATIVE: &str = "Katna crash report

App: katna-mail 0.0.0.r601.gdef5678
Kind: native crash, signal 11 (SIGSEGV)
When: 2026-09-27 03:20:00 UTC
System: Arch Linux, KDE on wayland
Reported by: a later run of Katna
Executable: /usr/bin/katna-mail

Package: katna/0.0.0
Process 4242 (katna-mail) of user 1000 dumped core.

Module libc.so.6 with build-id 1111111111111111111111111111111111111111
Module katna-mail without build-id.
Stack trace of thread 4242:
#0  0x00007f0000001000 __pthread_kill_implementation (libc.so.6 + 0x9ea7c)
#1  0x0000555500001000 n/a (katna-mail + 0x1234)

Stack trace of thread 4243:
#0  0x00007f0000002000 poll (libc.so.6 + 0x10a0)
";

    fn report(name: &str) -> Report {
        let app = name.split('-').skip(1).collect::<Vec<_>>();
        Report {
            name: name.into(),
            path: format!("/tmp/{name}").into(),
            app: app[..app.len() - 1].join("-"),
        }
    }

    #[test]
    fn reads_dsns() {
        let dsn = Dsn::parse(crate::ids::SENTRY_DSN).unwrap();
        assert_eq!(
            dsn.envelope_url(),
            "https://o4512156164096000.ingest.de.sentry.io/api/4512156171698256/envelope/"
        );
        assert!(
            dsn.auth_header()
                .contains("sentry_key=1ebb96bdfbca71ddd5a26968b39d5e47")
        );
        let hosted = Dsn::parse("https://key:secret@crash.example.org/glitch/7/").unwrap();
        assert_eq!(
            hosted.envelope_url(),
            "https://crash.example.org/glitch/api/7/envelope/"
        );
        assert!(Dsn::parse("").is_none());
        assert!(Dsn::parse("http://key@host/1").is_none());
        assert!(Dsn::parse("https://host/1").is_none());
        assert!(Dsn::parse("https://key@host/").is_none());
    }

    #[test]
    fn debug_ids_from_build_ids() {
        assert_eq!(
            debug_id("0123456789abcdef0011223344556677deadbeef").unwrap(),
            "67452301-ab89-efcd-0011-223344556677"
        );
        assert_eq!(
            debug_id("01020304").unwrap(),
            "04030201-0000-0000-0000-000000000000"
        );
        assert!(debug_id("").is_none());
    }

    #[test]
    fn panics_become_events_with_named_frames() {
        let report = report("20260927T031603Z-katna-mail-4242.txt");
        let event = event("id", &report, PANIC);
        let exception = &event["exception"]["values"][0];
        assert_eq!(exception["type"], "panic");
        assert_eq!(
            exception["value"],
            "called `Option::unwrap()` on a `None` value"
        );
        assert_eq!(exception["mechanism"]["handled"], false);
        let frames = exception["stacktrace"]["frames"].as_array().unwrap();
        let names: Vec<&str> = frames
            .iter()
            .map(|f| f["function"].as_str().unwrap())
            .collect();
        assert_eq!(
            names,
            [
                "main",
                "katna_mail::window::list::render_row",
                "core::option::unwrap_failed"
            ]
        );
        assert_eq!(frames[1]["in_app"], true);
        assert_eq!(
            frames[1]["filename"],
            "./apps/katna-mail/src/window/list.rs"
        );
        assert_eq!(frames[1]["lineno"], 120);
        assert_eq!(event["release"], "katna@0.0.0.r600.gabc1234");
        assert_eq!(event["timestamp"], "2026-09-27T03:16:03Z");
        assert_eq!(
            event["contexts"]["os"]["raw_description"],
            "Arch Linux, KDE on wayland"
        );
        assert_eq!(
            event["extra"]["location"],
            "apps/katna-mail/src/window/list.rs:120:9"
        );
        assert!(event.get("user").is_none());
        assert!(event.get("server_name").is_none());
        assert!(event.get("debug_meta").is_none());
    }

    #[test]
    fn stripped_panics_keep_addresses_for_debug_files() {
        let report = report("20260927T031603Z-katna-daemon-7.txt");
        let event = event("id", &report, STRIPPED);
        let frames = event["exception"]["values"][0]["stacktrace"]["frames"]
            .as_array()
            .unwrap()
            .clone();
        assert_eq!(frames.len(), 3);
        // Outermost first.
        assert_eq!(frames[0]["function"], "libc.so.6 + 0x2724a");
        assert_eq!(frames[2]["instruction_addr"], "0x1000");
        assert_eq!(frames[2]["addr_mode"], "rel:0");
        let image = &event["debug_meta"]["images"][0];
        assert_eq!(image["code_file"], "katna-daemon");
        assert_eq!(image["code_id"], "0123456789abcdef0011223344556677deadbeef");
        assert_eq!(image["debug_id"], "67452301-ab89-efcd-0011-223344556677");
    }

    #[test]
    fn native_crashes_become_events() {
        let report = report("20260927T032000Z-katna-mail-4242.txt");
        let event = event("id", &report, NATIVE);
        let exception = &event["exception"]["values"][0];
        assert_eq!(exception["type"], "SIGSEGV");
        assert_eq!(exception["mechanism"]["type"], "coredump");
        let frames = exception["stacktrace"]["frames"].as_array().unwrap();
        // Only the crashed thread, outermost first.
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0]["function"], "katna-mail + 0x1234");
        assert_eq!(frames[1]["function"], "__pthread_kill_implementation");
        assert_eq!(frames[1]["package"], "libc.so.6");
        assert_eq!(frames[1]["instruction_addr"], "0x9ea7c");
        assert_eq!(frames[1]["addr_mode"], "rel:0");
        assert_eq!(event["debug_meta"]["images"][0]["code_file"], "libc.so.6");
        assert_eq!(event["release"], "katna@0.0.0.r601.gdef5678");
    }

    #[test]
    fn envelopes_carry_the_report_as_it_is() {
        let dsn = Dsn::parse(crate::ids::SENTRY_DSN).unwrap();
        let report = report("20260927T031603Z-katna-mail-4242.txt");
        let now = UNIX_EPOCH + Duration::from_secs(1_790_479_000);
        let bytes = envelope(&dsn, &report, PANIC, now);
        let text = String::from_utf8(bytes.clone()).unwrap();
        let mut lines = text.splitn(4, '\n');
        let header: Value = serde_json::from_str(lines.next().unwrap()).unwrap();
        assert_eq!(header["dsn"], crate::ids::SENTRY_DSN);
        assert_eq!(header["sent_at"], "2026-09-27T03:16:40Z");
        let id = header["event_id"].as_str().unwrap();
        assert_eq!(id.len(), 32);
        let item: Value = serde_json::from_str(lines.next().unwrap()).unwrap();
        assert_eq!(item["type"], "event");
        let event_text = lines.next().unwrap();
        assert_eq!(item["length"], event_text.len());
        let event: Value = serde_json::from_str(event_text).unwrap();
        assert_eq!(event["event_id"], id);
        let rest = lines.next().unwrap();
        let (item, attachment) = rest.split_once('\n').unwrap();
        let item: Value = serde_json::from_str(item).unwrap();
        assert_eq!(item["type"], "attachment");
        assert_eq!(item["filename"], "20260927T031603Z-katna-mail-4242.txt");
        assert_eq!(attachment, format!("{PANIC}\n"));
        // The same report gets the same ID.
        assert_eq!(bytes, envelope(&dsn, &report, PANIC, now));
    }
}
