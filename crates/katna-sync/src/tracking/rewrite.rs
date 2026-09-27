// SPDX-License-Identifier: GPL-3.0-or-later

//! One recipient's tracked copy of a message (`docs/ARCHITECTURE.md`
//! §16.1): the HTML version gets the open pixel before the quoted text,
//! and its links (outside the quoted text) point through the tracking
//! server. Headers, the plain text version and attachments stay as they
//! are. Signed or encrypted mail is never changed.

use std::fmt::Write as _;

/// The link targets the tracked copies of `raw` will redirect to, numbered
/// in this order. `None` when `raw` cannot be tracked: no UTF-8 HTML
/// version, or signed or encrypted.
pub fn links(raw: &[u8]) -> Option<Vec<String>> {
    let mut found = Vec::new();
    let mut any = false;
    walk(raw, &mut |html| {
        any = true;
        for (_, _, target) in anchors(own_part(html)) {
            if !found.contains(&target) {
                found.push(target);
            }
        }
        None
    })?;
    any.then_some(found)
}

/// `raw` with the tracking pixel and links of tracking ID `id` on the
/// server at `base` (`https://server.example`). `links` is what [`links`]
/// returned for `raw`.
pub fn tracked_copy(raw: &[u8], base: &str, id: &str, links: &[String]) -> Option<Vec<u8>> {
    let base = base.trim_end_matches('/');
    walk(raw, &mut |html| Some(rewrite_html(html, base, id, links)))
}

fn rewrite_html(html: &str, base: &str, id: &str, links: &[String]) -> String {
    let own_end = own_part(html).len();
    let mut out = String::with_capacity(html.len() + 256);
    let mut from = 0;
    for (start, end, target) in anchors(&html[..own_end]) {
        let Some(n) = links.iter().position(|link| *link == target) else {
            continue;
        };
        out.push_str(&html[from..start]);
        let _ = write!(out, "\"{base}/l/{id}/{n}\"");
        from = end;
    }
    out.push_str(&html[from..own_end]);
    // The pixel goes before the quoted text, or at the end of the body.
    let pixel = format!(
        "<img alt=\"\" width=\"1\" height=\"1\" style=\"display:block;width:1px;height:1px;border:0\" src=\"{base}/o/{id}.png\">"
    );
    let rest = &html[own_end..];
    if rest.is_empty() {
        match find_ci(&out, "</body") {
            Some(at) => out.insert_str(at, &pixel),
            None => out.push_str(&pixel),
        }
    } else {
        out.push_str(&pixel);
        out.push_str(rest);
    }
    out
}

/// The sender's own part of an HTML body: everything before the first
/// quote of earlier mail.
fn own_part(html: &str) -> &str {
    match find_ci(html, "<blockquote") {
        Some(at) => &html[..at],
        None => html,
    }
}

/// The `href` values of `<a>` tags that go to `http` or `https`
/// addresses: `(start, end, target)`, where `start..end` is the attribute
/// value with its quotes and `target` has `&amp;` decoded.
fn anchors(html: &str) -> Vec<(usize, usize, String)> {
    let lower = html.to_ascii_lowercase();
    let bytes = lower.as_bytes();
    let mut found = Vec::new();
    let mut at = 0;
    while let Some(offset) = lower[at..].find("<a") {
        let tag_start = at + offset;
        at = tag_start + 2;
        if !bytes.get(at).is_some_and(|b| b.is_ascii_whitespace()) {
            continue;
        }
        let Some(tag_len) = lower[at..].find('>') else {
            break;
        };
        let tag_end = at + tag_len;
        if let Some((start, end)) = href(&lower, at, tag_end) {
            let quoted = &html[start..end];
            let value = quoted.trim_matches(|c| c == '"' || c == '\'');
            let target = value.replace("&amp;", "&");
            let scheme = target.get(..8).unwrap_or(&target).to_ascii_lowercase();
            if (scheme.starts_with("https://") || scheme.starts_with("http://"))
                && !target.chars().any(|c| c.is_whitespace() || c.is_control())
            {
                found.push((start, end, target));
            }
        }
        at = tag_end;
    }
    found
}

/// The span of the `href` value (with its quotes) inside a tag's
/// attributes `lower[from..to]`.
fn href(lower: &str, from: usize, to: usize) -> Option<(usize, usize)> {
    let bytes = lower.as_bytes();
    let mut at = from;
    while let Some(offset) = lower[at..to].find("href") {
        let name = at + offset;
        at = name + 4;
        if name > 0 && !bytes[name - 1].is_ascii_whitespace() {
            continue;
        }
        let mut i = at;
        while i < to && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if bytes.get(i) != Some(&b'=') {
            continue;
        }
        i += 1;
        while i < to && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        return match bytes.get(i) {
            Some(&quote @ (b'"' | b'\'')) => {
                let close = lower[i + 1..to].find(quote as char)?;
                Some((i, i + 1 + close + 1))
            }
            Some(_) => {
                let end = lower[i..to]
                    .find(|c: char| c.is_ascii_whitespace())
                    .map_or(to, |len| i + len);
                Some((i, end))
            }
            None => None,
        };
    }
    None
}

fn find_ci(haystack: &str, needle: &str) -> Option<usize> {
    haystack.to_ascii_lowercase().find(needle)
}

/// Calls `edit` with the text of every UTF-8 `text/html` part that is not
/// an attachment; a `Some` answer replaces the part's text. Returns the
/// whole message, or `None` for signed or encrypted mail or a broken
/// structure.
fn walk(raw: &[u8], edit: &mut dyn FnMut(&str) -> Option<String>) -> Option<Vec<u8>> {
    let (header, body, separator) = split_entity(raw)?;
    let fields = Fields::parse(header);
    let content_type = fields.get("content-type").unwrap_or("text/plain");
    let media = content_type
        .split(';')
        .next()
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    if media == "multipart/signed" || media == "multipart/encrypted" || media.contains("pkcs7") {
        return None;
    }
    if let Some(sub) = media.strip_prefix("multipart/") {
        let _ = sub;
        let boundary = param(content_type, "boundary")?;
        let body = walk_multipart(body, &boundary, edit)?;
        let mut out = raw[..header.len() + separator.len()].to_vec();
        out.extend_from_slice(&body);
        return Some(out);
    }
    let attachment = fields
        .get("content-disposition")
        .is_some_and(|d| d.trim().to_ascii_lowercase().starts_with("attachment"));
    let utf8 = param(content_type, "charset").is_none_or(|charset| {
        matches!(
            charset.to_ascii_lowercase().as_str(),
            "utf-8" | "us-ascii" | "utf8"
        )
    });
    if media != "text/html" || attachment || !utf8 {
        return Some(raw.to_vec());
    }
    let encoding = fields
        .get("content-transfer-encoding")
        .unwrap_or("7bit")
        .trim()
        .to_ascii_lowercase();
    let decoded = match encoding.as_str() {
        "quoted-printable" => decode_qp(body),
        "base64" => decode_base64(body)?,
        _ => body.to_vec(),
    };
    let text = String::from_utf8(decoded).ok()?;
    let Some(new) = edit(&text) else {
        return Some(raw.to_vec());
    };
    let mut out = Fields::without(header, "content-transfer-encoding");
    let newline = if separator.starts_with(b"\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    out.extend_from_slice(
        format!("Content-Transfer-Encoding: quoted-printable{newline}{newline}").as_bytes(),
    );
    out.extend_from_slice(encode_qp(&new).as_bytes());
    Some(out)
}

/// Rewrites each part of a multipart body, keeping the delimiters, the
/// preamble and the epilogue byte for byte.
fn walk_multipart(
    body: &[u8],
    boundary: &str,
    edit: &mut dyn FnMut(&str) -> Option<String>,
) -> Option<Vec<u8>> {
    let delimiter = format!("--{boundary}");
    // Start of each delimiter line.
    let mut starts = Vec::new();
    let mut at = 0;
    while at < body.len() {
        let line_end = body[at..]
            .iter()
            .position(|&b| b == b'\n')
            .map_or(body.len(), |i| at + i + 1);
        let line = &body[at..line_end];
        if line.starts_with(delimiter.as_bytes()) {
            starts.push(at);
            if line[delimiter.len()..].starts_with(b"--") {
                break;
            }
        }
        at = line_end;
    }
    if starts.len() < 2 {
        return None;
    }
    let mut out = body[..starts[0]].to_vec();
    for pair in starts.windows(2) {
        let (start, next) = (pair[0], pair[1]);
        let line_end = body[start..]
            .iter()
            .position(|&b| b == b'\n')
            .map_or(body.len(), |i| start + i + 1);
        out.extend_from_slice(&body[start..line_end]);
        // The line break before the next delimiter belongs to it.
        let mut part_end = next;
        if body[..part_end].ends_with(b"\r\n") {
            part_end -= 2;
        } else if body[..part_end].ends_with(b"\n") {
            part_end -= 1;
        }
        let part = &body[line_end..part_end.max(line_end)];
        out.extend_from_slice(&walk(part, edit)?);
        out.extend_from_slice(&body[part_end.max(line_end)..next]);
    }
    out.extend_from_slice(&body[*starts.last()?..]);
    Some(out)
}

/// Header, body and the blank line between them.
fn split_entity(raw: &[u8]) -> Option<(&[u8], &[u8], &[u8])> {
    if let Some(at) = raw.windows(4).position(|w| w == b"\r\n\r\n") {
        return Some((&raw[..at + 2], &raw[at + 4..], &raw[at + 2..at + 4]));
    }
    if let Some(at) = raw.windows(2).position(|w| w == b"\n\n") {
        return Some((&raw[..at + 1], &raw[at + 2..], &raw[at + 1..at + 2]));
    }
    None
}

/// Unfolded header fields of one entity.
struct Fields(Vec<(String, String)>);

impl Fields {
    fn parse(header: &[u8]) -> Self {
        let text = String::from_utf8_lossy(header);
        let mut fields: Vec<(String, String)> = Vec::new();
        for line in text.lines() {
            if line.starts_with([' ', '\t']) {
                if let Some((_, value)) = fields.last_mut() {
                    value.push(' ');
                    value.push_str(line.trim());
                }
            } else if let Some((name, value)) = line.split_once(':') {
                fields.push((name.trim().to_ascii_lowercase(), value.trim().to_owned()));
            }
        }
        Self(fields)
    }

    fn get(&self, name: &str) -> Option<&str> {
        self.0
            .iter()
            .find(|(field, _)| field == name)
            .map(|(_, value)| value.as_str())
    }

    /// `header` without the field `name` (and its continuation lines).
    fn without(header: &[u8], name: &str) -> Vec<u8> {
        let mut out = Vec::with_capacity(header.len());
        let mut skipping = false;
        for line in header.split_inclusive(|&b| b == b'\n') {
            if !matches!(line.first(), Some(b' ' | b'\t')) {
                skipping = line.len() > name.len()
                    && line[..name.len()].eq_ignore_ascii_case(name.as_bytes())
                    && line[name.len()] == b':';
            }
            if !skipping {
                out.extend_from_slice(line);
            }
        }
        out
    }
}

/// A parameter of a structured header value, without quotes.
fn param(value: &str, name: &str) -> Option<String> {
    value.split(';').skip(1).find_map(|part| {
        let (key, value) = part.split_once('=')?;
        key.trim()
            .eq_ignore_ascii_case(name)
            .then(|| value.trim().trim_matches('"').to_owned())
    })
}

fn decode_qp(body: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(body.len());
    let mut i = 0;
    while i < body.len() {
        match body[i] {
            b'=' if body.get(i + 1) == Some(&b'\r') && body.get(i + 2) == Some(&b'\n') => i += 3,
            b'=' if body.get(i + 1) == Some(&b'\n') => i += 2,
            b'=' => {
                let hex = body
                    .get(i + 1..i + 3)
                    .and_then(|h| std::str::from_utf8(h).ok());
                match hex.and_then(|h| u8::from_str_radix(h, 16).ok()) {
                    Some(byte) => {
                        out.push(byte);
                        i += 3;
                    }
                    None => {
                        out.push(b'=');
                        i += 1;
                    }
                }
            }
            byte => {
                out.push(byte);
                i += 1;
            }
        }
    }
    out
}

fn decode_base64(body: &[u8]) -> Option<Vec<u8>> {
    fn value(c: u8) -> Option<u32> {
        Some(match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            _ => return None,
        } as u32)
    }
    let digits: Vec<u8> = body
        .iter()
        .copied()
        .filter(|c| !c.is_ascii_whitespace() && *c != b'=')
        .collect();
    let mut out = Vec::with_capacity(digits.len() * 3 / 4);
    for chunk in digits.chunks(4) {
        let mut acc = 0u32;
        for (i, &c) in chunk.iter().enumerate() {
            acc |= value(c)? << (18 - 6 * i);
        }
        let bytes = acc.to_be_bytes();
        out.extend_from_slice(&bytes[1..chunk.len().max(1)]);
    }
    Some(out)
}

/// Quoted-printable (RFC 2045) with CRLF line ends.
fn encode_qp(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + text.len() / 8);
    for line in text.lines() {
        let bytes = line.as_bytes();
        let mut width = 0;
        for (ix, &byte) in bytes.iter().enumerate() {
            let last = ix + 1 == bytes.len();
            let literal = (byte == b' ' || byte == b'\t') && !last
                || (33..=126).contains(&byte) && byte != b'=';
            let piece = if literal {
                (byte as char).to_string()
            } else {
                format!("={byte:02X}")
            };
            if width + piece.len() > 75 {
                out.push_str("=\r\n");
                width = 0;
            }
            out.push_str(&piece);
            width += piece.len();
        }
        out.push_str("\r\n");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: &str = "https://server.katna.test";
    const ID: &str = "0123456789abcdef0123456789abcdef";

    fn message(html: &str) -> Vec<u8> {
        format!(
            "From: a@x.org\r\nTo: b@y.org, c@z.org\r\nSubject: Proposal v2\r\nMIME-Version: 1.0\r\n\
             Content-Type: multipart/alternative;\r\n boundary=\"katna-1\"\r\n\r\n\
             --katna-1\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Transfer-Encoding: 7bit\r\n\r\n\
             See https://example.com/a\r\n\
             --katna-1\r\nContent-Type: text/html; charset=utf-8\r\nContent-Transfer-Encoding: quoted-printable\r\n\r\n\
             {}\
             --katna-1--\r\n",
            encode_qp(html)
        )
        .into_bytes()
    }

    fn html_of(raw: &[u8]) -> String {
        let mut found = String::new();
        walk(raw, &mut |html| {
            found = html.to_owned();
            None
        })
        .unwrap();
        found
    }

    #[test]
    fn links_and_pixel_outside_the_quote() {
        let html = "<div>Hi, see <a href=\"https://example.com/p?a=1&amp;b=2\">this</a> and \
                    <A class=x HREF='http://example.org/'>that</A> and <a href=\"mailto:x@y.org\">me</a>.</div>\
                    <div>On Monday, Ada wrote:</div><blockquote><a href=\"https://old.example/\">old</a></blockquote>";
        let raw = message(html);
        let found = links(&raw).unwrap();
        assert_eq!(
            found,
            ["https://example.com/p?a=1&b=2", "http://example.org/"]
        );

        let copy = tracked_copy(&raw, BASE, ID, &found).unwrap();
        let text = String::from_utf8(copy.clone()).unwrap();
        // Headers and the plain version are unchanged.
        assert!(
            text.starts_with("From: a@x.org\r\nTo: b@y.org, c@z.org\r\nSubject: Proposal v2\r\n")
        );
        assert!(text.contains("See https://example.com/a\r\n"));
        let html = html_of(&copy);
        assert!(
            html.contains(&format!("<a href=\"{BASE}/l/{ID}/0\">this</a>")),
            "{html}"
        );
        assert!(
            html.contains(&format!("HREF=\"{BASE}/l/{ID}/1\">that")),
            "{html}"
        );
        assert!(html.contains("mailto:x@y.org"));
        // Quoted links stay; the pixel comes before the quote.
        assert!(html.contains("https://old.example/"));
        let pixel = html.find(&format!("{BASE}/o/{ID}.png")).unwrap();
        assert!(pixel < html.find("<blockquote").unwrap());
        assert!(text.ends_with("--katna-1--\r\n"));
    }

    #[test]
    fn pixel_before_the_end_of_the_body() {
        let raw = message("<html><body><p>Hello</p></body></html>");
        let copy = tracked_copy(&raw, BASE, ID, &[]).unwrap();
        let html = html_of(&copy);
        assert!(
            html.trim_end()
                .ends_with(&format!("src=\"{BASE}/o/{ID}.png\"></body></html>")),
            "{html}"
        );
    }

    #[test]
    fn nothing_to_track() {
        let plain = b"From: a@x.org\r\nTo: b@y.org\r\nContent-Type: text/plain\r\n\r\nHi\r\n";
        assert_eq!(links(plain), None);
        let signed = b"From: a@x.org\r\nContent-Type: multipart/signed; boundary=b\r\n\r\n--b\r\n\r\nx\r\n--b--\r\n";
        assert_eq!(links(signed), None);
        assert_eq!(tracked_copy(signed, BASE, ID, &[]), None);
    }

    #[test]
    fn nested_parts_and_base64() {
        let html_b64 = "PHA+PGEgaHJlZj0iaHR0cHM6Ly9leGFtcGxlLmNvbS8iPng8L2E+PC9wPg=="; // <p><a href="https://example.com/">x</a></p>
        let raw = format!(
            "From: a@x.org\r\nContent-Type: multipart/mixed; boundary=\"m\"\r\n\r\npreamble\r\n\
             --m\r\nContent-Type: multipart/related; boundary=\"r\"\r\n\r\n\
             --r\r\nContent-Type: text/html; charset=\"UTF-8\"\r\nContent-Transfer-Encoding: base64\r\n\r\n{html_b64}\r\n\
             --r\r\nContent-Type: image/png\r\nContent-Transfer-Encoding: base64\r\n\r\niVBORw0KGgo=\r\n--r--\r\n\
             \r\n--m\r\nContent-Type: text/html; name=\"a.html\"\r\nContent-Disposition: attachment\r\n\r\n\
             <a href=\"https://attached.example/\">a</a>\r\n--m--\r\nepilogue\r\n"
        );
        let found = links(raw.as_bytes()).unwrap();
        assert_eq!(found, ["https://example.com/"]);
        let copy =
            String::from_utf8(tracked_copy(raw.as_bytes(), BASE, ID, &found).unwrap()).unwrap();
        assert!(copy.contains("preamble\r\n--m\r\n"));
        assert!(copy.contains("iVBORw0KGgo=\r\n--r--"));
        assert!(copy.contains("<a href=\"https://attached.example/\">a</a>"));
        assert!(copy.ends_with("--m--\r\nepilogue\r\n"));
        assert!(
            copy.contains(
                &encode_qp(&format!("<p><a href=\"{BASE}/l/{ID}/0\">x</a>"))
                    .trim_end_matches("\r\n")
                    .split("=\r\n")
                    .next()
                    .unwrap()
                    .to_owned()
            )
        );
    }

    #[test]
    fn quoted_printable_round_trip() {
        let text = "caf\u{e9} = <a href=\"x\">\ttab</a> ".repeat(8);
        assert_eq!(
            decode_qp(encode_qp(&text).as_bytes()),
            format!("{text}\r\n").into_bytes()
        );
        assert_eq!(decode_base64(b"aGVsbG8=").unwrap(), b"hello");
    }
}
