// SPDX-License-Identifier: GPL-3.0-or-later

//! Signatures in message bodies: the block a signature adds, swapping one
//! for another in a draft, and finding which one a sent message was signed
//! with, so a reply in that conversation starts with the same one. No GPUI
//! here.

use katna_core::config::Signature;

/// The signature line: dash, dash, space (RFC 3676 §4.3).
const SEPARATOR: &str = "-- ";

/// What a signature adds below the message, with the blank line before it.
pub fn block(text: &str) -> String {
    let text = text.trim_end();
    if text.is_empty() {
        String::new()
    } else {
        format!("\n{SEPARATOR}\n{text}\n")
    }
}

/// `body` with the signature `old` replaced by `new`. When `old` is not
/// found as written (it was edited), `new` goes where a signature goes:
/// after the text the user wrote, before any quoted message.
pub fn swap(body: &str, old: Option<&str>, new: Option<&str>) -> String {
    let new_block = new.map(block).unwrap_or_default();
    if let Some(old) = old.map(block).filter(|b| !b.is_empty())
        && let Some(at) = body.find(&old)
    {
        return format!("{}{new_block}{}", &body[..at], &body[at + old.len()..]);
    }
    if new_block.is_empty() {
        return body.to_owned();
    }
    let at = quote_start(body).unwrap_or(body.len());
    let (head, tail) = body.split_at(at);
    let head = head.trim_end_matches('\n');
    let sep = if tail.is_empty() { "" } else { "\n" };
    format!("{head}\n{new_block}{sep}{tail}")
}

/// Where the quoted message of a reply or forward starts: its "On …
/// wrote:" or "Forwarded message" line, or the first quoted line.
fn quote_start(body: &str) -> Option<usize> {
    let mut at = 0;
    for line in body.split_inclusive('\n') {
        let trimmed = line.trim();
        if trimmed.starts_with('>')
            || (trimmed.starts_with("On ") && trimmed.ends_with("wrote:"))
            || trimmed.contains("Forwarded message")
        {
            return Some(at);
        }
        at += line.len();
    }
    None
}

/// The signature `body` was signed with: the text after its first
/// unquoted `-- ` line, up to the quoted message, compared with each
/// signature line by line, ignoring spaces at the ends of lines.
pub fn used_in(body: &str, signatures: &[Signature]) -> Option<u32> {
    let mut lines = body.lines();
    lines.by_ref().find(|l| l.trim_end() == "--")?;
    let signed: Vec<&str> = lines
        .take_while(|l| {
            let t = l.trim();
            !t.starts_with('>') && !(t.starts_with("On ") && t.ends_with("wrote:"))
        })
        .collect();
    let signed = normalize(signed.into_iter());
    signatures
        .iter()
        .filter(|s| !s.text.trim().is_empty())
        .find(|s| normalize(s.text.lines()) == signed)
        .map(|s| s.id)
}

fn normalize<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<&'a str> {
    let mut lines: Vec<&str> = lines.map(str::trim).collect();
    while lines.last().is_some_and(|l| l.is_empty()) {
        lines.pop();
    }
    let blank = lines.iter().take_while(|l| l.is_empty()).count();
    lines.drain(..blank);
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sig(id: u32, text: &str) -> Signature {
        Signature {
            id,
            name: format!("S{id}"),
            text: text.to_owned(),
        }
    }

    #[test]
    fn blocks() {
        assert_eq!(block("Kay\nEnron\n\n"), "\n-- \nKay\nEnron\n");
        assert_eq!(block("  \n"), "");
    }

    #[test]
    fn swaps_in_new_mail() {
        let body = format!("\n{}", block("Kay"));
        assert_eq!(
            swap(&body, Some("Kay"), Some("Kay Mann\nEnron")),
            "\n\n-- \nKay Mann\nEnron\n"
        );
        assert_eq!(swap(&body, Some("Kay"), None), "\n");
        assert_eq!(swap("\n", None, Some("Kay")), "\n\n-- \nKay\n");
        assert_eq!(swap("Hello\n", None, None), "Hello\n");
    }

    #[test]
    fn a_new_signature_goes_above_the_quote() {
        let body = "Thanks!\n\nOn Monday, Ada wrote:\n> Hi\n";
        assert_eq!(
            swap(body, None, Some("Kay")),
            "Thanks!\n\n-- \nKay\n\nOn Monday, Ada wrote:\n> Hi\n"
        );
        // The old one was edited: it stays, and the new one is added.
        let body = format!("Hi\n{}", block("Kay"));
        let edited = body.replace("Kay", "Kay M.");
        assert!(swap(&edited, Some("Kay"), Some("Bob")).contains("-- \nBob\n"));
    }

    #[test]
    fn finds_the_signature_of_a_sent_message() {
        let signatures = [sig(1, "Kay"), sig(2, "Kay Mann  \nEnron Corp")];
        let sent = "Numbers attached.\n\n-- \nKay Mann\nEnron Corp\n\n\
                    On Monday, Ada wrote:\n> -- \n> Kay\n";
        assert_eq!(used_in(sent, &signatures), Some(2));
        assert_eq!(used_in("Hi\n-- \nKay\n", &signatures), Some(1));
        assert_eq!(used_in("Hi\n--\nKay\n", &signatures), Some(1));
        assert_eq!(used_in("Hi\n-- \nSomeone else\n", &signatures), None);
        assert_eq!(used_in("No signature\n> -- \n> Kay\n", &signatures), None);
    }
}
