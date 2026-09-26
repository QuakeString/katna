// SPDX-License-Identifier: GPL-3.0-or-later

//! The rich view of a raw message: its HTML body laid out as a
//! [`Document`], with the images it carries.

use std::sync::Arc;

use mail_parser::{MessageParser, MimeHeaders, PartType};

use crate::html::{
    Align, Block, Document, Image, ImageKind, ImageSource, Inline, Run, RunStyle, TextBlock,
    document, looks_like_html,
};

/// Lays out the HTML body of `raw`. `None` when the message has no HTML
/// (the plain-text view is then the whole story).
pub fn message_document(raw: &[u8]) -> Option<Document> {
    let message = MessageParser::default().parse(raw)?;
    let parts: Vec<_> = message.html_bodies().collect();
    let has_html = parts.iter().any(|p| matches!(p.body, PartType::Html(_)));
    let inline_image = |cid: &str| -> Option<Arc<[u8]>> {
        message
            .parts
            .iter()
            .find(|p| {
                p.content_id()
                    .is_some_and(|id| id.trim_matches(['<', '>', ' ']).eq_ignore_ascii_case(cid))
            })
            .map(|p| Arc::from(p.contents()))
    };
    if !has_html {
        // Some senders label an HTML document text/plain.
        let text = message.body_text(0)?;
        return looks_like_html(&text).then(|| document(&text, &inline_image));
    }
    let mut doc = Document::default();
    for part in parts {
        match &part.body {
            PartType::Html(html) => {
                let next = document(html, &inline_image);
                doc.background = doc.background.or(next.background);
                doc.styled |= next.styled;
                doc.remote_images += next.remote_images;
                doc.trackers += next.trackers;
                doc.truncated |= next.truncated;
                doc.inline_ids.extend(next.inline_ids);
                doc.blocks.extend(next.blocks);
            }
            PartType::Text(text) => {
                let text = text.trim_end();
                if !text.is_empty() {
                    doc.blocks.push(Block::Text(TextBlock {
                        inlines: vec![Inline::Text(Run {
                            text: text.replace("\r\n", "\n"),
                            style: RunStyle::default(),
                        })],
                        align: Align::Start,
                        preformatted: true,
                    }));
                }
            }
            PartType::Binary(bytes) | PartType::InlineBinary(bytes) => {
                // An image between the text parts of a multipart/mixed.
                if let Some(kind) = ImageKind::sniff(bytes) {
                    doc.blocks.push(Block::Text(TextBlock {
                        inlines: vec![Inline::Image(Image {
                            source: ImageSource::Data {
                                kind,
                                bytes: Arc::from(bytes.as_ref()),
                            },
                            alt: part.attachment_name().unwrap_or_default().to_owned(),
                            width: None,
                            height: None,
                            link: None,
                        })],
                        align: Align::Start,
                        preformatted: false,
                    }));
                }
            }
            PartType::Message(_) | PartType::Multipart(_) => {}
        }
    }
    Some(doc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_messages_have_no_document() {
        let raw = b"From: a@example.org\r\nSubject: x\r\n\r\nJust text.\r\n";
        assert!(message_document(raw).is_none());
    }

    #[test]
    fn alternative_uses_the_html_part() {
        let raw = b"From: a@example.org\r\nSubject: x\r\n\
Content-Type: multipart/alternative; boundary=\"b\"\r\n\r\n\
--b\r\nContent-Type: text/plain\r\n\r\nplain version\r\n\
--b\r\nContent-Type: text/html\r\n\r\n<p>rich <b>version</b></p>\r\n\
--b--\r\n";
        let doc = message_document(raw).unwrap();
        let Block::Box(p) = &doc.blocks[0] else {
            panic!("{doc:?}")
        };
        let Block::Text(t) = &p.children[0] else {
            panic!()
        };
        assert_eq!(t.text(), "rich version");
    }

    #[test]
    fn html_sent_as_plain_text_is_rendered() {
        let raw = b"From: a@example.org\r\nSubject: x\r\n\r\n\
<!DOCTYPE html><html><body><div>Hello</div></body></html>\r\n";
        let doc = message_document(raw).unwrap();
        assert!(matches!(&doc.blocks[0], Block::Text(t) if t.text() == "Hello"));
    }

    #[test]
    fn cid_images_come_from_the_message() {
        let raw = b"From: a@example.org\r\nSubject: x\r\n\
Content-Type: multipart/related; boundary=\"r\"\r\n\r\n\
--r\r\nContent-Type: text/html\r\n\r\n<img src=\"cid:logo@x\" alt=\"Logo\">\r\n\
--r\r\nContent-Type: image/png\r\nContent-ID: <logo@x>\r\n\
Content-Transfer-Encoding: base64\r\n\r\niVBORw0KGgo=\r\n\
--r--\r\n";
        let doc = message_document(raw).unwrap();
        let Block::Text(t) = &doc.blocks[0] else {
            panic!("{doc:?}")
        };
        assert!(matches!(
            &t.inlines[0],
            Inline::Image(Image {
                source: ImageSource::Data {
                    kind: ImageKind::Png,
                    ..
                },
                ..
            })
        ));
    }
}
