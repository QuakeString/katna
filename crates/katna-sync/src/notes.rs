// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna Notes in an account's Notes folder (`docs/ARCHITECTURE.md`
//! §13.11), in the format Apple Notes, Thunderbird's IMAP Notes add-on and
//! Fastmail keep there: one small HTML message per note, marked
//! `X-Uniform-Type-Identifier: com.apple.mail-note`, its title as the
//! subject and the first line of its text. Editing a note replaces its
//! message. Katna's own extras ride in `X-Katna-*` headers, which the
//! other apps ignore.

use std::collections::HashSet;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use katna_core::AccountId;
use katna_store::{Note, RemoteNote, Store};

use crate::backend::Flags;
use crate::connection::Connection;
use crate::outbox::rfc5322_date;
use crate::{Error, Result};

/// The folder notes live in (a label on Gmail).
pub const NOTES_FOLDER: &str = "Notes";
const NOTE_TYPE: &str = "com.apple.mail-note";

/// What a look at the Notes folder did.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NotesSynced {
    pub uploaded: usize,
    pub downloaded: usize,
    pub deleted_here: usize,
    pub deleted_there: usize,
}

impl NotesSynced {
    /// Whether the notes here changed.
    pub fn changed_here(&self) -> bool {
        self.downloaded > 0 || self.deleted_here > 0 || self.uploaded > 0
    }
}

/// Brings `account`'s notes and its Notes folder in step: deletes the
/// copies of notes deleted here, writes the notes changed here, then reads
/// the notes written elsewhere and forgets the ones deleted there. The
/// folder is made the first time a note of the account needs it.
pub async fn sync_notes(
    connection: &Connection,
    store: &mut Store,
    account: AccountId,
    from: &str,
) -> Result<NotesSynced> {
    let mut done = NotesSynced::default();
    let map = |err: katna_store::Error| Error::Protocol(format!("store: {err}"));
    let notes = store.account_notes(account.0).map_err(map)?;
    let gone = store.notes_gone(account.0).map_err(map)?;
    let to_upload: Vec<&Note> = notes
        .iter()
        .filter(|n| n.dirty && n.trashed_at.is_none())
        .collect();
    let folders = connection.list_folders().await?;
    let folder = folders
        .iter()
        .find(|f| f.name.eq_ignore_ascii_case(NOTES_FOLDER) && f.selectable)
        .map(|f| f.name.clone());
    let folder = match folder {
        Some(folder) => folder,
        None if to_upload.is_empty() => {
            // No notes here, none there.
            if !gone.is_empty() {
                store.clear_notes_gone(account.0, &gone).map_err(map)?;
            }
            return Ok(done);
        }
        None => {
            connection.create_folder(NOTES_FOLDER).await?;
            NOTES_FOLDER.to_owned()
        }
    };
    connection.select(&folder).await?;
    let present: HashSet<u32> = connection.uids().await?.into_iter().collect();

    // Copies of notes deleted here, and the old copies of notes changed
    // here, go first.
    let mut delete: Vec<u32> = gone
        .iter()
        .filter_map(|uid| u32::try_from(*uid).ok())
        .filter(|uid| present.contains(uid))
        .collect();
    delete.extend(
        to_upload
            .iter()
            .filter_map(|n| n.server_uid)
            .filter_map(|uid| u32::try_from(uid).ok())
            .filter(|uid| present.contains(uid)),
    );
    delete.sort_unstable();
    delete.dedup();
    if !delete.is_empty() {
        connection.expunge(&delete).await?;
        done.deleted_there = delete.len();
    }
    if !gone.is_empty() {
        store.clear_notes_gone(account.0, &gone).map_err(map)?;
    }
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| i64::try_from(d.as_secs()).unwrap_or(0));
    // Seen, so the Notes folder shows no unread count in mail apps.
    let seen = Flags {
        seen: true,
        ..Flags::default()
    };
    for note in &to_upload {
        connection
            .append_with_flags(&folder, build_note(note, from, now), &seen)
            .await?;
        store.note_uploaded(note.id, note.updated_at).map_err(map)?;
        done.uploaded += 1;
    }

    // What is there now.
    connection.select(&folder).await?;
    let uids = connection.uids().await?;
    let known: HashSet<i64> = store
        .account_notes(account.0)
        .map_err(map)?
        .iter()
        .filter_map(|n| n.server_uid)
        .collect();
    let new: Vec<u32> = uids
        .iter()
        .copied()
        .filter(|uid| !known.contains(&i64::from(*uid)))
        .collect();
    for chunk in new.chunks(50) {
        for (uid, raw) in connection.fetch_bodies(chunk).await? {
            let Some(remote) = parse_note(&raw) else {
                tracing::debug!(uid, "not a note; left alone");
                continue;
            };
            store
                .apply_remote_note(account.0, i64::from(uid), &remote)
                .map_err(map)?;
            done.downloaded += 1;
        }
    }
    // Our own uploads came back as "downloaded" just now.
    done.downloaded = done.downloaded.saturating_sub(done.uploaded);
    let all: Vec<i64> = uids.iter().map(|uid| i64::from(*uid)).collect();
    done.deleted_here = store.forget_remote_notes(account.0, &all).map_err(map)?;
    Ok(done)
}

/// `note` as a message for the Notes folder of `from`'s account.
pub fn build_note(note: &Note, from: &str, now: i64) -> Vec<u8> {
    let title = if note.title.trim().is_empty() {
        note.body
            .lines()
            .find(|l| !l.trim().is_empty())
            .unwrap_or("")
            .trim()
    } else {
        note.title.trim()
    };
    let mut html = String::from("<html><head></head><body>");
    // Apple takes a note's first line as its title.
    html.push_str(&format!("<div>{}</div>", escape(title)));
    let mut lines: Vec<&str> = note.body.split('\n').collect();
    if note.title.trim().is_empty() && lines.first().is_some_and(|l| l.trim() == title) {
        lines.remove(0);
    }
    for line in lines {
        if line.is_empty() {
            html.push_str("<div><br></div>");
        } else {
            html.push_str(&format!("<div>{}</div>", escape(line)));
        }
    }
    html.push_str("</body></html>");
    let mut out = String::new();
    let header = |out: &mut String, name: &str, value: &str| {
        out.push_str(name);
        out.push_str(": ");
        out.push_str(value);
        out.push_str("\r\n");
    };
    header(&mut out, "Date", &rfc5322_date(now));
    header(&mut out, "From", from);
    header(&mut out, "Subject", &encode_word(title));
    header(&mut out, "X-Uniform-Type-Identifier", NOTE_TYPE);
    header(&mut out, "X-Universally-Unique-Identifier", &note.uuid);
    header(
        &mut out,
        "X-Mail-Created-Date",
        &rfc5322_date(note.created_at),
    );
    header(&mut out, "X-Katna-Title", &encode_word(&note.title));
    if note.color != 0 {
        header(&mut out, "X-Katna-Color", &note.color.to_string());
    }
    if note.pinned {
        header(&mut out, "X-Katna-Pinned", "yes");
    }
    if note.archived {
        header(&mut out, "X-Katna-Archived", "yes");
    }
    if !note.labels.is_empty() {
        let json = serde_json::to_string(&note.labels).unwrap_or_default();
        header(&mut out, "X-Katna-Labels", &STANDARD.encode(json));
    }
    if let Some(link) = note.link.as_deref().filter(|l| is_header_safe(l)) {
        header(&mut out, "X-Katna-Link", link);
    }
    header(
        &mut out,
        "Message-ID",
        &format!("<{}@katna.notes>", note.uuid),
    );
    header(&mut out, "MIME-Version", "1.0");
    header(&mut out, "Content-Type", "text/html; charset=utf-8");
    header(&mut out, "Content-Transfer-Encoding", "base64");
    out.push_str("\r\n");
    let encoded = STANDARD.encode(html.as_bytes());
    for chunk in encoded.as_bytes().chunks(76) {
        out.push_str(std::str::from_utf8(chunk).unwrap_or_default());
        out.push_str("\r\n");
    }
    out.into_bytes()
}

/// A note from a message of the Notes folder, or `None` for one that is
/// not a note.
pub fn parse_note(raw: &[u8]) -> Option<RemoteNote> {
    let message = mail_parser::MessageParser::default().parse(raw)?;
    let raw_header = |name: &str| {
        message
            .header_raw(name)
            .map(|v| v.trim().to_owned())
            .filter(|v| !v.is_empty())
    };
    let kind = raw_header("X-Uniform-Type-Identifier");
    if kind.as_deref() != Some(NOTE_TYPE) {
        return None;
    }
    let uuid = raw_header("X-Universally-Unique-Identifier")
        .or_else(|| message.message_id().map(str::to_owned))?;
    let subject = message.subject().unwrap_or("").trim().to_owned();
    let text = match message.body_html(0) {
        Some(html) => html_lines(&html),
        None => message
            .body_text(0)
            .map(|t| t.replace("\r\n", "\n"))
            .unwrap_or_default(),
    };
    let mut lines: Vec<&str> = text.split('\n').collect();
    while lines.last().is_some_and(|l| l.trim().is_empty()) {
        lines.pop();
    }
    // The first line repeats the title.
    let katna_title = raw_header("X-Katna-Title").map(|t| decode_word(&t));
    let first_is_title = lines.first().is_some_and(|l| l.trim() == subject);
    if first_is_title {
        lines.remove(0);
    }
    let title = match katna_title {
        Some(title) => title,
        None if first_is_title => subject,
        None => String::new(),
    };
    let labels = raw_header("X-Katna-Labels")
        .and_then(|v| STANDARD.decode(v.as_bytes()).ok())
        .and_then(|json| serde_json::from_slice::<Vec<String>>(&json).ok())
        .unwrap_or_default();
    let yes = |name: &str| raw_header(name).is_some_and(|v| v.eq_ignore_ascii_case("yes"));
    Some(RemoteNote {
        uuid,
        title,
        body: lines.join("\n"),
        color: raw_header("X-Katna-Color")
            .and_then(|c| c.parse().ok())
            .unwrap_or(0),
        pinned: yes("X-Katna-Pinned"),
        archived: yes("X-Katna-Archived"),
        labels,
        link: raw_header("X-Katna-Link"),
        updated_at: message.date().map_or(0, |d| d.to_timestamp()),
    })
}

/// The lines of a note's HTML, as Apple Notes writes it: a `div` per
/// line, `<div><br></div>` for an empty one.
fn html_lines(html: &str) -> String {
    let body = html
        .find("<body")
        .and_then(|at| html[at..].find('>').map(|end| at + end + 1))
        .map_or(html, |start| &html[start..]);
    let mut out = String::new();
    let mut rest = body;
    while let Some(lt) = rest.find('<') {
        out.push_str(&unescape(&rest[..lt]));
        let Some(gt) = rest[lt..].find('>') else {
            rest = "";
            break;
        };
        let tag = rest[lt + 1..lt + gt].trim().to_ascii_lowercase();
        let name = tag
            .trim_start_matches('/')
            .split(|c: char| c.is_whitespace() || c == '/')
            .next()
            .unwrap_or("");
        let closing = tag.starts_with('/');
        match name {
            "br" => out.push('\n'),
            "div" | "p" | "li" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "tr" => {
                if !out.is_empty() && !out.ends_with('\n') {
                    out.push('\n');
                }
            }
            "body" | "html" if closing => {
                rest = "";
                break;
            }
            _ => {}
        }
        rest = &rest[lt + gt + 1..];
    }
    out.push_str(&unescape(rest));
    out.replace('\u{a0}', " ")
}

fn unescape(text: &str) -> String {
    let text = text.replace(['\r', '\n'], "");
    if !text.contains('&') {
        return text;
    }
    let mut out = String::new();
    let mut rest = text.as_str();
    while let Some(amp) = rest.find('&') {
        out.push_str(&rest[..amp]);
        let after = &rest[amp..];
        let Some(semi) = after.find(';').filter(|s| *s <= 10) else {
            out.push('&');
            rest = &after[1..];
            continue;
        };
        let entity = &after[1..semi];
        let ch = match entity {
            "amp" => Some('&'),
            "lt" => Some('<'),
            "gt" => Some('>'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            "nbsp" => Some(' '),
            _ => entity
                .strip_prefix("#x")
                .and_then(|h| u32::from_str_radix(h, 16).ok())
                .or_else(|| entity.strip_prefix('#').and_then(|d| d.parse().ok()))
                .and_then(char::from_u32),
        };
        match ch {
            Some(ch) => {
                out.push(ch);
                rest = &after[semi + 1..];
            }
            None => {
                out.push('&');
                rest = &after[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn is_header_safe(value: &str) -> bool {
    value.len() < 900 && value.chars().all(|c| c.is_ascii_graphic())
}

/// `text` as a header value: as it is when plain ASCII, else an RFC 2047
/// encoded word.
fn encode_word(text: &str) -> String {
    if text.chars().all(|c| c.is_ascii() && !c.is_ascii_control()) && !text.contains("=?") {
        text.to_owned()
    } else {
        format!("=?utf-8?B?{}?=", STANDARD.encode(text.as_bytes()))
    }
}

/// The reverse of [`encode_word`], for headers mail-parser leaves raw.
fn decode_word(value: &str) -> String {
    value
        .strip_prefix("=?utf-8?B?")
        .and_then(|v| v.strip_suffix("?="))
        .and_then(|b64| STANDARD.decode(b64).ok())
        .and_then(|bytes| String::from_utf8(bytes).ok())
        .unwrap_or_else(|| value.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn note() -> Note {
        Note {
            id: 1,
            uuid: "0F1E2D3C-4B5A-4978-8899-AABBCCDDEEFF".to_owned(),
            title: "Goa trip".to_owned(),
            body: "☐ Book train\n\n☑ Leave <approved> & signed".to_owned(),
            color: 9,
            pinned: true,
            labels: vec!["Travel".to_owned(), "Family".to_owned()],
            link: Some("inv3@example.com".to_owned()),
            created_at: 1_790_000_000,
            ..Note::default()
        }
    }

    #[test]
    fn a_note_goes_there_and_back_unchanged() {
        let raw = build_note(&note(), "me@example.org", 1_790_650_000);
        let text = String::from_utf8(raw.clone()).unwrap();
        assert!(text.contains("X-Uniform-Type-Identifier: com.apple.mail-note\r\n"));
        assert!(text.contains("Subject: Goa trip\r\n"));
        let back = parse_note(&raw).unwrap();
        assert_eq!(back.uuid, note().uuid);
        assert_eq!(back.title, "Goa trip");
        assert_eq!(back.body, note().body);
        assert_eq!(back.color, 9);
        assert!(back.pinned && !back.archived);
        assert_eq!(back.labels, ["Travel", "Family"]);
        assert_eq!(back.link.as_deref(), Some("inv3@example.com"));
        assert_eq!(back.updated_at, 1_790_650_000);
    }

    #[test]
    fn a_title_in_another_script_is_encoded() {
        let mut n = note();
        n.title = "गोवा यात्रा".to_owned();
        let raw = build_note(&n, "me@example.org", 1_790_650_000);
        assert!(raw.is_ascii());
        assert_eq!(parse_note(&raw).unwrap().title, "गोवा यात्रा");
    }

    #[test]
    fn an_apple_note_reads_with_its_first_line_as_the_title() {
        let raw = b"Subject: Shopping\r\n\
X-Uniform-Type-Identifier: com.apple.mail-note\r\n\
X-Universally-Unique-Identifier: 12345678-AAAA-4BBB-8CCC-1234567890AB\r\n\
Date: Mon, 28 Sep 2026 10:00:00 +0000\r\n\
Content-Type: text/html; charset=utf-8\r\n\
Content-Transfer-Encoding: quoted-printable\r\n\
\r\n\
<html><head></head><body><div>Shopping</div><div>Milk</div><div>Bread</div></body></html>\r\n";
        let note = parse_note(raw).unwrap();
        assert_eq!(note.title, "Shopping");
        assert_eq!(note.body, "Milk\nBread");
        assert_eq!(note.uuid, "12345678-AAAA-4BBB-8CCC-1234567890AB");
    }

    #[test]
    fn other_mail_is_not_a_note() {
        let raw = b"Subject: hi\r\nMessage-ID: <a@b>\r\n\r\nhello\r\n";
        assert_eq!(parse_note(raw), None);
    }
}
