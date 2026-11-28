// SPDX-License-Identifier: GPL-3.0-or-later

//! How a message can be unsubscribed from ([`katna_core::unsubscribe`]):
//! its `List-Unsubscribe` headers, else an unsubscribe link in its body.

use katna_core::unsubscribe::{Ways, list_headers, offers_unsubscribe};

use crate::html::{Block, Document, Inline};

/// The ways to unsubscribe from `raw`, whose HTML body is laid out as
/// `doc` and whose text is `text`.
pub fn unsubscribe_ways(raw: &[u8], doc: Option<&Document>, text: &str) -> Ways {
    let (header, post) = list_headers(raw);
    let body_link = match doc {
        Some(doc) => html_link(&doc.blocks),
        None => text_link(text),
    };
    Ways::read(header.as_deref(), post.as_deref(), body_link.as_deref())
}

/// The last link whose words offer to unsubscribe: footers come last.
fn html_link(blocks: &[Block]) -> Option<String> {
    fn walk(blocks: &[Block], found: &mut Option<String>) {
        for block in blocks {
            match block {
                Block::Box(b) => walk(&b.children, found),
                Block::Text(t) => {
                    // A link's words can be split into runs by styling.
                    let mut words = String::new();
                    let mut link: Option<&str> = None;
                    for inline in &t.inlines {
                        let Inline::Text(run) = inline else {
                            continue;
                        };
                        let this = run.style.link.as_deref();
                        if this != link {
                            if let Some(l) = link.filter(|_| offers_unsubscribe(&words)) {
                                *found = Some(l.to_owned());
                            }
                            words.clear();
                            link = this;
                        }
                        words.push_str(&run.text);
                    }
                    if let Some(l) = link.filter(|_| offers_unsubscribe(&words)) {
                        *found = Some(l.to_owned());
                    }
                }
                Block::Rule => {}
            }
        }
    }
    let mut found = None;
    walk(blocks, &mut found);
    found
}

/// In plain text: the last web address on a line that offers to
/// unsubscribe ("Unsubscribe: https://…").
fn text_link(text: &str) -> Option<String> {
    text.lines()
        .filter(|line| line.to_lowercase().contains("unsubscribe"))
        .filter_map(|line| {
            let start = line.find("https://").or_else(|| line.find("http://"))?;
            let url: String = line[start..]
                .chars()
                .take_while(|c| !c.is_whitespace() && !matches!(c, '>' | ')' | '"'))
                .collect();
            Some(url)
        })
        .next_back()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headers_come_first() {
        let raw = b"From: News <n@shop.example>\r\n\
List-Unsubscribe: <mailto:u@shop.example>,\r\n <https://shop.example/u?id=7>\r\n\
List-Unsubscribe-Post: List-Unsubscribe=One-Click\r\n\
Content-Type: text/html\r\n\r\n<a href=\"https://shop.example/body\">Unsubscribe</a>";
        let doc = crate::message_document(raw);
        let ways = unsubscribe_ways(raw, doc.as_ref(), "");
        assert_eq!(
            ways.one_click.as_deref(),
            Some("https://shop.example/u?id=7")
        );
        assert_eq!(ways.page.as_deref(), Some("https://shop.example/u?id=7"));
        assert!(ways.mail.is_some());
    }

    #[test]
    fn a_body_link_when_there_is_no_header() {
        let raw = b"From: News <n@shop.example>\r\nContent-Type: text/html\r\n\r\n\
<p><a href=\"https://shop.example/a\">Read more</a></p>\
<p style=\"text-align:center\"><a href=\"https://shop.example/u\"><b>Un</b>subscribe</a></p>";
        let doc = crate::message_document(raw);
        let ways = unsubscribe_ways(raw, doc.as_ref(), "");
        assert!(!ways.quiet());
        assert_eq!(ways.page.as_deref(), Some("https://shop.example/u"));
    }

    #[test]
    fn plain_text_lines() {
        assert_eq!(
            text_link("Hi\n\nTo unsubscribe, visit <https://list.example/u?x=1>.\n").as_deref(),
            Some("https://list.example/u?x=1")
        );
        assert_eq!(text_link("Visit https://list.example/"), None);
    }
}
