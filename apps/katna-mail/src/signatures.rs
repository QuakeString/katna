// SPDX-License-Identifier: GPL-3.0-or-later

//! Signatures in message bodies: a signature as editor content, and
//! finding which one a sent message was signed with, so a reply in that
//! conversation starts with the same one. No GPUI here.

pub mod import;
pub mod layout;

use katna_core::config::Signature;
use katna_ui::rich::{Doc, html};

/// The signature as editor content: its formatted version when it has
/// one, else its text.
pub fn doc(signature: &Signature) -> Doc {
    if signature.html.trim().is_empty() {
        html::from_plain(signature.text.trim_end())
    } else {
        let mut next = 0;
        html::from_html(&signature.html, &mut next)
    }
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
            ..Signature::default()
        }
    }

    #[test]
    fn signatures_as_content() {
        let plain = doc(&sig(1, "Kay\nEnron\n\n"));
        assert_eq!(html::to_plain(&plain), "Kay\nEnron\n");
        let mut rich = sig(2, "Kay");
        rich.html = "<div><b>Kay</b></div>".to_owned();
        let rich = doc(&rich);
        assert_eq!(html::to_plain(&rich), "Kay\n");
        assert!(rich.has_formatting());
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
