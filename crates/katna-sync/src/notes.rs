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
use katna_store::{Note, NotePicture, RemoteNote, Store};
use mail_parser::MimeHeaders as _;

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
        let pictures = store.note_pictures(note.id).map_err(map)?;
        connection
            .append_with_flags(&folder, build_note_with(note, &pictures, from, now), &seen)
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
    build_note_with(note, &[], from, now)
}

/// [`build_note`] with the note's pictures, as inline parts its HTML
/// names by `cid:` (`multipart/related`, as mail apps send pictures in a
/// message's text).
pub fn build_note_with(note: &Note, pictures: &[NotePicture], from: &str, now: i64) -> Vec<u8> {
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
    // Pictures under the title, as Keep shows them on top.
    let pictures: Vec<&NotePicture> = pictures
        .iter()
        .filter(|p| is_header_safe(&p.cid) && !p.cid.contains(['"', '<', '>']))
        .collect();
    for picture in &pictures {
        html.push_str(&format!("<div><img src=\"cid:{}\"></div>", picture.cid));
    }
    if note.html.is_empty() {
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
    } else {
        // Formatted: the app's HTML after the title line, which
        // `parse_note` takes off again. With no title of its own, the
        // first line is the title line, as with plain lines.
        let first_is_title = note.title.trim().is_empty()
            && note.body.split('\n').next().map(str::trim) == Some(title);
        if first_is_title {
            let inner = note
                .html
                .strip_prefix("<div dir=\"ltr\">")
                .and_then(|h| h.strip_suffix("</div>"))
                .unwrap_or(&note.html);
            html.push_str(after_first_block(inner));
        } else {
            html.push_str(&note.html);
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
    if let Some(at) = note.remind_at {
        header(&mut out, "X-Katna-Remind", &at.to_string());
    }
    header(&mut out, "X-Mailer", KATNA_MAILER);
    header(
        &mut out,
        "Message-ID",
        &format!("<{}@katna.notes>", note.uuid),
    );
    header(&mut out, "MIME-Version", "1.0");
    let base64 = |out: &mut String, data: &[u8]| {
        let encoded = STANDARD.encode(data);
        for chunk in encoded.as_bytes().chunks(76) {
            out.push_str(std::str::from_utf8(chunk).unwrap_or_default());
            out.push_str("\r\n");
        }
    };
    if pictures.is_empty() {
        header(&mut out, "Content-Type", "text/html; charset=utf-8");
        header(&mut out, "Content-Transfer-Encoding", "base64");
        out.push_str("\r\n");
        base64(&mut out, html.as_bytes());
        return out.into_bytes();
    }
    let boundary = format!("katna-note-{}", note.uuid);
    header(
        &mut out,
        "Content-Type",
        &format!("multipart/related; type=\"text/html\"; boundary=\"{boundary}\""),
    );
    out.push_str("\r\n");
    out.push_str(&format!("--{boundary}\r\n"));
    header(&mut out, "Content-Type", "text/html; charset=utf-8");
    header(&mut out, "Content-Transfer-Encoding", "base64");
    out.push_str("\r\n");
    base64(&mut out, html.as_bytes());
    for picture in pictures {
        let name = encode_word(&picture.name).replace('"', "'");
        let mime = if is_header_safe(&picture.mime) && picture.mime.contains('/') {
            picture.mime.as_str()
        } else {
            "application/octet-stream"
        };
        out.push_str(&format!("--{boundary}\r\n"));
        header(
            &mut out,
            "Content-Type",
            &format!("{mime}; name=\"{name}\""),
        );
        header(
            &mut out,
            "Content-Disposition",
            &format!("inline; filename=\"{name}\""),
        );
        header(&mut out, "Content-ID", &format!("<{}>", picture.cid));
        header(&mut out, "Content-Transfer-Encoding", "base64");
        out.push_str("\r\n");
        base64(&mut out, &picture.data);
    }
    out.push_str(&format!("--{boundary}--\r\n"));
    out.into_bytes()
}

/// Katna's `X-Mailer`, so its notes are not taken for another device's.
const KATNA_MAILER: &str = "Katna Notes";

/// The device a note was written on, from its `X-Mailer`.
fn device_of(mailer: &str) -> Option<String> {
    let lower = mailer.to_ascii_lowercase();
    ["iPhone", "iPad", "Mac", "Android", "Windows"]
        .into_iter()
        .find(|d| lower.contains(&d.to_ascii_lowercase()))
        .or_else(|| lower.contains("ios").then_some("iPhone"))
        .map(str::to_owned)
}

/// The `cid:` names of the pictures `html` shows, in order.
fn picture_cids(html: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut rest = html;
    while let Some(at) = rest.find("src=\"cid:") {
        let after = &rest[at + 9..];
        let Some(end) = after.find('"') else { break };
        let cid = after[..end].to_owned();
        if !cid.is_empty() && !out.contains(&cid) {
            out.push(cid);
        }
        rest = &after[end..];
    }
    out
}

/// `html` without its `cid:` pictures, and without the lines that held
/// only a picture.
fn strip_pictures(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut rest = html;
    while let Some(at) = rest.find("<img") {
        let Some(end) = rest[at..].find('>').map(|e| at + e + 1) else {
            break;
        };
        out.push_str(&rest[..at]);
        if !rest[at..end].contains("src=\"cid:") {
            out.push_str(&rest[at..end]);
        }
        rest = &rest[end..];
    }
    out.push_str(rest);
    out.replace("<div></div>", "").replace("<p></p>", "")
}

/// Apple Notes shows a picture as an `object` naming its part; the
/// editor reads `img`.
fn objects_to_images(html: &str) -> String {
    let lower = html.to_ascii_lowercase();
    let mut out = String::with_capacity(html.len());
    let mut at = 0;
    while let Some(start) = lower[at..].find("<object").map(|ix| at + ix) {
        let Some(open_end) = lower[start..].find('>').map(|ix| start + ix + 1) else {
            break;
        };
        let tag = &html[start..open_end];
        let Some(cid) = tag
            .find("data=\"cid:")
            .map(|ix| &tag[ix + 10..])
            .and_then(|rest| rest.find('"').map(|end| &rest[..end]))
        else {
            out.push_str(&html[at..open_end]);
            at = open_end;
            continue;
        };
        let close = lower[open_end..]
            .find("</object>")
            .map_or(open_end, |ix| open_end + ix + 9);
        out.push_str(&html[at..start]);
        out.push_str(&format!("<img src=\"cid:{cid}\">"));
        at = close;
    }
    out.push_str(&html[at..]);
    out
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
    // Pictures are kept apart from the text, shown on top as Keep does.
    let html = message.body_html(0).map(|h| objects_to_images(&h));
    let cids: Vec<String> = html.as_deref().map(picture_cids).unwrap_or_default();
    let html = html.map(|h| strip_pictures(&h));
    let text = match &html {
        Some(html) => html_lines(html),
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
    // Formatting kept as it is, without the title line.
    let html = html
        .map(|html| {
            let body = body_of(&html);
            let body = if first_is_title {
                after_first_block(body)
            } else {
                body
            };
            if is_formatted(body) {
                body.trim().to_owned()
            } else {
                String::new()
            }
        })
        .unwrap_or_default();
    let labels = raw_header("X-Katna-Labels")
        .and_then(|v| STANDARD.decode(v.as_bytes()).ok())
        .and_then(|json| serde_json::from_slice::<Vec<String>>(&json).ok())
        .unwrap_or_default();
    let yes = |name: &str| raw_header(name).is_some_and(|v| v.eq_ignore_ascii_case("yes"));
    // Pictures its text shows.
    let pictures: Vec<NotePicture> = message
        .attachments()
        .filter_map(|part| {
            let cid = part.content_id()?.trim_matches(['<', '>', ' ']).to_owned();
            if !cids.contains(&cid) {
                return None;
            }
            let mime = part.content_type().map_or_else(
                || "application/octet-stream".to_owned(),
                |ct| match ct.subtype() {
                    Some(sub) => format!("{}/{sub}", ct.ctype()).to_ascii_lowercase(),
                    None => ct.ctype().to_ascii_lowercase(),
                },
            );
            Some(NotePicture {
                name: part.attachment_name().unwrap_or(&cid).to_owned(),
                mime,
                width: 0,
                height: 0,
                data: part.contents().to_vec(),
                cid,
            })
        })
        .collect();
    let device = raw_header("X-Mailer")
        .filter(|m| m != KATNA_MAILER)
        .and_then(|m| device_of(&m));
    Some(RemoteNote {
        uuid,
        title,
        body: lines.join("\n"),
        html,
        color: raw_header("X-Katna-Color")
            .and_then(|c| c.parse().ok())
            .unwrap_or(0),
        pinned: yes("X-Katna-Pinned"),
        archived: yes("X-Katna-Archived"),
        labels,
        link: raw_header("X-Katna-Link"),
        updated_at: message.date().map_or(0, |d| d.to_timestamp()),
        remind_at: raw_header("X-Katna-Remind").and_then(|v| v.parse().ok()),
        pictures,
        device,
    })
}

/// What is inside `html`'s `body`, or all of it when it has none.
fn body_of(html: &str) -> &str {
    let start = html
        .find("<body")
        .and_then(|at| html[at..].find('>').map(|end| at + end + 1))
        .unwrap_or(0);
    let body = &html[start..];
    body.find("</body").map_or(body, |end| &body[..end])
}

/// `html` without its first element (the title line of a note).
fn after_first_block(html: &str) -> &str {
    let html = html.trim_start();
    let name: String = html
        .strip_prefix('<')
        .unwrap_or("")
        .chars()
        .take_while(char::is_ascii_alphanumeric)
        .collect::<String>()
        .to_ascii_lowercase();
    if name.is_empty() {
        return html;
    }
    let lower = html.to_ascii_lowercase();
    let (open, close) = (format!("<{name}"), format!("</{name}"));
    let mut depth = 0usize;
    let mut at = 0;
    while let Some(lt) = lower[at..].find('<').map(|ix| at + ix) {
        let rest = &lower[lt..];
        let boundary = |tag: &str| {
            rest.starts_with(tag)
                && rest[tag.len()..]
                    .chars()
                    .next()
                    .is_some_and(|c| !c.is_ascii_alphanumeric())
        };
        if boundary(&close) {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return rest.find('>').map_or("", |gt| &html[lt + gt + 1..]);
            }
        } else if boundary(&open) {
            depth += 1;
        }
        at = lt + 1;
    }
    ""
}

/// Whether a note's HTML has any formatting: bold, italic, underline,
/// struck, headings, lists, sized text, pictures, quotes, code, dividers
/// or links.
fn is_formatted(html: &str) -> bool {
    const TAGS: [&str; 23] = [
        "b",
        "strong",
        "i",
        "em",
        "u",
        "s",
        "strike",
        "h1",
        "h2",
        "h3",
        "h4",
        "h5",
        "h6",
        "ul",
        "ol",
        "font",
        "img",
        "blockquote",
        "pre",
        "code",
        "tt",
        "hr",
        "a",
    ];
    const STYLES: [&str; 4] = ["font-weight", "font-style", "text-decoration", "font-size"];
    let lower = html.to_ascii_lowercase();
    let mut rest = lower.as_str();
    while let Some(lt) = rest.find('<') {
        let tag = &rest[lt + 1..];
        let end = tag.find('>').unwrap_or(tag.len());
        let inside = &tag[..end];
        let name: String = inside
            .chars()
            .take_while(char::is_ascii_alphanumeric)
            .collect();
        if TAGS.contains(&name.as_str()) || STYLES.iter().any(|s| inside.contains(s)) {
            return true;
        }
        rest = &tag[end..];
    }
    false
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
    fn formatting_goes_there_and_back() {
        let mut n = note();
        n.body = "Plan\n☐ Book train".to_owned();
        n.html = "<div dir=\"ltr\"><div><b>Plan</b></div><div>☐ Book train</div></div>".to_owned();
        let back = parse_note(&build_note(&n, "me@example.org", 1_790_650_000)).unwrap();
        assert_eq!(back.body, n.body);
        assert_eq!(back.html, n.html);
        // With no title of its own, the first line is the title, as with
        // plain lines.
        n.title = String::new();
        n.html = "<div dir=\"ltr\"><div>Plan</div><div><i>☐ Book train</i></div></div>".to_owned();
        let back = parse_note(&build_note(&n, "me@example.org", 1_790_650_000)).unwrap();
        assert_eq!(
            (back.title.as_str(), back.body.as_str()),
            ("Plan", "☐ Book train")
        );
        assert_eq!(back.html, "<div><i>☐ Book train</i></div>");
    }

    #[test]
    fn an_apple_note_keeps_its_formatting_but_not_its_title() {
        let raw = b"Subject: Shopping\r\n\
X-Uniform-Type-Identifier: com.apple.mail-note\r\n\
X-Universally-Unique-Identifier: 12345678-AAAA-4BBB-8CCC-1234567890AB\r\n\
Content-Type: text/html; charset=utf-8\r\n\
\r\n\
<html><head></head><body><div><b>Shopping</b></div><div><i>Milk</i></div><div>Bread<br></div></body></html>\r\n";
        let note = parse_note(raw).unwrap();
        assert_eq!(note.body, "Milk\nBread");
        assert_eq!(note.html, "<div><i>Milk</i></div><div>Bread<br></div>");
        // Plain lines keep no HTML.
        let plain = parse_note(&build_note(&super::tests::note(), "me@example.org", 0)).unwrap();
        assert!(plain.html.is_empty());
    }

    #[test]
    fn other_mail_is_not_a_note() {
        let raw = b"Subject: hi\r\nMessage-ID: <a@b>\r\n\r\nhello\r\n";
        assert_eq!(parse_note(raw), None);
    }

    #[test]
    fn pictures_and_reminders_go_there_and_back() {
        let mut n = note();
        n.body = "Shelf".to_owned();
        n.html = "<div dir=\"ltr\"><p><b>Shelf</b></p></div>".to_owned();
        n.remind_at = Some(1_790_700_000);
        let picture = NotePicture {
            cid: "p1".to_owned(),
            name: "shelf.png".to_owned(),
            mime: "image/png".to_owned(),
            data: vec![137, 80, 78, 71, 0, 1, 2, 3],
            ..NotePicture::default()
        };
        let raw = build_note_with(&n, std::slice::from_ref(&picture), "me@example.org", 0);
        let text = String::from_utf8(raw.clone()).unwrap();
        assert!(text.contains("multipart/related"));
        assert!(text.contains("Content-ID: <p1>"));
        let back = parse_note(&raw).unwrap();
        assert_eq!(back.remind_at, Some(1_790_700_000));
        assert_eq!(back.device, None);
        assert_eq!(back.pictures.len(), 1);
        assert_eq!(back.pictures[0].cid, "p1");
        assert_eq!(back.pictures[0].mime, "image/png");
        assert_eq!(back.pictures[0].data, picture.data);
        // The text comes back as it was, the picture apart.
        assert!(!back.html.contains("cid:"));
        assert_eq!(back.body, n.body);
        assert!(back.html.contains("<b>Shelf</b>"));
    }

    #[test]
    fn an_apple_picture_reads_as_an_image() {
        assert_eq!(
            objects_to_images(
                "<div>a</div><object type=\"application/x-apple-msg-attachment\" data=\"cid:X1\"></object>b"
            ),
            "<div>a</div><img src=\"cid:X1\">b"
        );
        assert_eq!(
            strip_pictures("<div>a</div><div><img src=\"cid:X1\"></div><div>b</div>"),
            "<div>a</div><div>b</div>"
        );
        assert_eq!(
            picture_cids("<img src=\"cid:a\"><img src=\"cid:b\">"),
            ["a", "b"]
        );
        assert_eq!(device_of("iOS Notes"), Some("iPhone".to_owned()));
        assert_eq!(device_of("Apple Mail (2.3654)"), None);
    }
}
