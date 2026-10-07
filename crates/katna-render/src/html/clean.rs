// SPDX-License-Identifier: GPL-3.0-or-later

//! Designed HTML (a signature pasted or imported) made safe to send:
//! only the elements and attributes mail readers keep, inline styles
//! without anything that loads or moves, links to the web, mail and
//! phones only, no tracking pixels. Pictures on the web are carried inside
//! when the app fetched them ([`clean`]'s `fetched`), so readers see them
//! without loading anything.

use std::collections::HashMap;
use std::fmt::Write as _;

use super::css;
use super::dom::{self, DOCUMENT, Data, Dom, NodeId};
use super::{ImageKind, build};

/// Elements kept as they are.
const KEPT: &[&str] = &[
    "a",
    "abbr",
    "b",
    "big",
    "blockquote",
    "br",
    "center",
    "code",
    "col",
    "colgroup",
    "dd",
    "del",
    "div",
    "dl",
    "dt",
    "em",
    "font",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "hr",
    "i",
    "img",
    "ins",
    "li",
    "ol",
    "p",
    "pre",
    "s",
    "small",
    "span",
    "strike",
    "strong",
    "sub",
    "sup",
    "table",
    "tbody",
    "td",
    "tfoot",
    "th",
    "thead",
    "tr",
    "u",
    "ul",
];
/// Elements left out with all they hold: what runs, loads, asks for input
/// or is not text.
const DROPPED: &[&str] = &[
    "applet", "audio", "base", "button", "canvas", "embed", "form", "frame", "frameset", "head",
    "iframe", "input", "link", "meta", "noscript", "object", "script", "select", "style",
    "template", "textarea", "title", "video",
];
/// Elements that end with no content.
const VOID: &[&str] = &["br", "col", "hr", "img"];
/// Attributes kept on any kept element.
const ATTRS: &[&str] = &[
    "align",
    "alt",
    "bgcolor",
    "border",
    "cellpadding",
    "cellspacing",
    "color",
    "colspan",
    "dir",
    "face",
    "height",
    "rowspan",
    "size",
    "span",
    "style",
    "title",
    "valign",
    "width",
];
/// CSS that loads something, places a box over others or runs code.
const UNSAFE_CSS: &[&str] = &[
    "url(",
    "expression(",
    "javascript:",
    "behavior",
    "-moz-binding",
];
const UNSAFE_PROPERTIES: &[&str] = &["position", "z-index", "behavior", "-moz-binding"];

/// Designed HTML as it is sent, and what was left out of it.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Cleaned {
    pub html: String,
    pub left_out: LeftOut,
}

/// What [`clean`] left out or could not carry, so the app can say so.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LeftOut {
    /// Scripts, frames, forms and the like.
    pub active: usize,
    /// Style sheets (`<style>`): only inline styles travel in mail.
    pub style_sheets: usize,
    /// Tracking pixels.
    pub trackers: usize,
    /// Links to anything but the web, mail or phones.
    pub links: usize,
    /// Pictures still on the web: readers load them from there.
    pub web_pictures: usize,
}

/// The web addresses of the pictures `html` shows, for the app to fetch
/// before [`clean`]. Tracking pixels are not among them.
pub fn web_pictures(html: &str) -> Vec<String> {
    let dom = dom::parse(html);
    let mut urls = Vec::new();
    for node in &dom.nodes {
        if node.tag() != "img" {
            continue;
        }
        let Some(url) = node.attr("src").and_then(build::remote_url) else {
            continue;
        };
        if !tracker(node) && !urls.contains(&url) {
            urls.push(url);
        }
    }
    urls
}

/// `html` made safe to send. `fetched` holds pictures fetched from the
/// web, by the address [`web_pictures`] gave; they go inside as `data:`
/// URIs.
pub fn clean(html: &str, fetched: &HashMap<String, Vec<u8>>) -> Cleaned {
    let dom = dom::parse(html);
    let mut cleaner = Cleaner {
        dom: &dom,
        out: String::with_capacity(html.len()),
        left_out: LeftOut::default(),
        fetched,
    };
    // `<style>` sits in the head, which is left out whole.
    cleaner.left_out.style_sheets = dom.nodes.iter().filter(|n| n.tag() == "style").count();
    let body = find(&dom, DOCUMENT, "html").and_then(|html| find(&dom, html, "body"));
    for &child in &dom.nodes[body.unwrap_or(DOCUMENT)].children {
        cleaner.node(child);
    }
    Cleaned {
        html: cleaner.out.trim().to_owned(),
        left_out: cleaner.left_out,
    }
}

fn find(dom: &Dom, parent: NodeId, tag: &str) -> Option<NodeId> {
    dom.nodes[parent]
        .children
        .iter()
        .copied()
        .find(|&c| dom.nodes[c].tag() == tag)
}

/// A picture that only reports the mail was opened: tiny, or at a
/// tracker's address.
fn tracker(node: &dom::Node) -> bool {
    let style = css::declarations(node.attr("style").unwrap_or_default());
    let get = |key: &str| {
        style
            .iter()
            .rev()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
            .or(node.attr(key))
    };
    let tiny = |key| {
        get(key)
            .and_then(|v| css::px(v, 16.0))
            .is_some_and(|v| v <= 2.0)
    };
    let src = node.attr("src").unwrap_or_default().to_ascii_lowercase();
    let remote = build::remote_url(&src).is_some();
    remote && (tiny("width") || tiny("height") || build::tracker_path(&src))
}

struct Cleaner<'a> {
    dom: &'a Dom,
    out: String,
    left_out: LeftOut,
    fetched: &'a HashMap<String, Vec<u8>>,
}

impl Cleaner<'_> {
    fn node(&mut self, id: NodeId) {
        let node = &self.dom.nodes[id];
        let attrs = match &node.data {
            Data::Text(text) => {
                escape(&mut self.out, text, false);
                return;
            }
            Data::Element { attrs } => attrs,
            _ => return,
        };
        let tag = node.tag();
        if tag.is_empty() || DROPPED.contains(&tag) {
            // SVG, MathML and the like have no tag here: not mail.
            if tag != "head" && tag != "style" && tag != "title" && tag != "meta" {
                self.left_out.active += 1;
            }
            return;
        }
        if !KEPT.contains(&tag) {
            // Unknown wrappers (`<o:p>`, `<section>`, …) keep what they hold.
            for &child in &node.children {
                self.node(child);
            }
            return;
        }
        let mut src = None;
        if tag == "img" {
            src = self.image_src(node);
            if src.is_none() {
                return;
            }
        }
        let _ = write!(self.out, "<{tag}");
        for (key, value) in attrs {
            let key = key.to_ascii_lowercase();
            let value = match key.as_str() {
                "href" if tag == "a" => match link(value) {
                    Some(href) => href,
                    None => {
                        self.left_out.links += 1;
                        continue;
                    }
                },
                "src" if tag == "img" => match src.take() {
                    Some(src) => src,
                    None => continue,
                },
                "style" => style(value),
                k if ATTRS.contains(&k) => value.clone(),
                _ => continue,
            };
            if value.is_empty() && key != "alt" {
                continue;
            }
            let _ = write!(self.out, " {key}=\"");
            escape(&mut self.out, &value, true);
            self.out.push('"');
        }
        self.out.push('>');
        if VOID.contains(&tag) {
            return;
        }
        for &child in &node.children {
            self.node(child);
        }
        let _ = write!(self.out, "</{tag}>");
    }

    /// Where a picture comes from as sent, or `None` to leave it out.
    fn image_src(&mut self, node: &dom::Node) -> Option<String> {
        let src = node.attr("src").unwrap_or_default().trim();
        let lower = src.to_ascii_lowercase();
        // `cid:katna-N`: a picture of the signature being edited.
        if lower.starts_with("data:image/") || lower.starts_with("cid:katna-") {
            return Some(src.to_owned());
        }
        let url = build::remote_url(src)?;
        if tracker(node) {
            self.left_out.trackers += 1;
            return None;
        }
        match self.fetched.get(&url) {
            Some(bytes) => {
                let mime = match ImageKind::sniff(bytes)? {
                    ImageKind::Png => "image/png",
                    ImageKind::Jpeg => "image/jpeg",
                    ImageKind::Gif => "image/gif",
                    ImageKind::Webp => "image/webp",
                    ImageKind::Bmp => "image/bmp",
                    ImageKind::Ico => "image/x-icon",
                    ImageKind::Svg => "image/svg+xml",
                };
                Some(format!("data:{mime};base64,{}", base64(bytes)))
            }
            None => {
                self.left_out.web_pictures += 1;
                Some(url)
            }
        }
    }
}

/// A link's address if it goes to the web, mail or a phone.
fn link(href: &str) -> Option<String> {
    let href = href.trim();
    let lower = href.to_ascii_lowercase();
    ["https://", "http://", "mailto:", "tel:"]
        .iter()
        .any(|p| lower.starts_with(p))
        .then(|| href.to_owned())
}

/// Inline style without what loads, runs or places boxes over others.
fn style(value: &str) -> String {
    css::declarations(value)
        .into_iter()
        .filter(|(name, value)| {
            let lower = value.to_ascii_lowercase();
            !UNSAFE_PROPERTIES.contains(&name.as_str())
                && !UNSAFE_CSS
                    .iter()
                    .any(|bad| lower.contains(bad) || name.contains(bad))
        })
        .map(|(name, value)| format!("{name}:{value}"))
        .collect::<Vec<_>>()
        .join(";")
}

fn escape(out: &mut String, text: &str, attribute: bool) {
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' if attribute => out.push_str("&quot;"),
            '\u{a0}' => out.push_str("&nbsp;"),
            c => out.push(c),
        }
    }
}

fn base64(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let n = chunk.iter().fold(0u32, |n, &b| n << 8 | u32::from(b)) << (8 * (3 - chunk.len()));
        for i in 0..4 {
            if i <= chunk.len() {
                out.push(TABLE[(n >> (18 - 6 * i) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_layout_and_inline_style() {
        let html = "<html><head><style>td{color:red}</style></head><body>\
            <table cellpadding=\"0\" style=\"border-collapse:collapse\"><tr>\
            <td style=\"background:#0b7;padding:8px;position:absolute\"><b>Kay</b></td>\
            <td class=\"x\" onclick=\"steal()\"><a href=\"https://enron.example\">Site</a> \
            <a href=\"javascript:alert(1)\">Bad</a></td></tr></table>\
            <script>alert(1)</script></body></html>";
        let cleaned = clean(html, &HashMap::new());
        assert_eq!(
            cleaned.html,
            "<table cellpadding=\"0\" style=\"border-collapse:collapse\"><tbody><tr>\
             <td style=\"background:#0b7;padding:8px\"><b>Kay</b></td>\
             <td><a href=\"https://enron.example\">Site</a> <a>Bad</a></td></tr></tbody></table>"
        );
        assert_eq!(
            cleaned.left_out,
            LeftOut {
                active: 1,
                style_sheets: 1,
                links: 1,
                ..LeftOut::default()
            }
        );
    }

    #[test]
    fn carries_fetched_pictures_inside() {
        let html = "<img src=\"http://cdn.example/logo.png\" width=\"80\" alt=\"Logo\">\
            <img src=\"https://t.example/open.gif\" width=\"1\" height=\"1\">\
            <img src=\"https://cdn.example/other.png\">";
        assert_eq!(
            web_pictures(html),
            [
                "https://cdn.example/logo.png",
                "https://cdn.example/other.png"
            ]
        );
        let png = b"\x89PNG\r\n\x1a\n".to_vec();
        let fetched = HashMap::from([("https://cdn.example/logo.png".to_owned(), png)]);
        let cleaned = clean(html, &fetched);
        assert_eq!(
            cleaned.html,
            "<img src=\"data:image/png;base64,iVBORw0KGgo=\" width=\"80\" alt=\"Logo\">\
             <img src=\"https://cdn.example/other.png\">"
        );
        assert_eq!(cleaned.left_out.trackers, 1);
        assert_eq!(cleaned.left_out.web_pictures, 1);
    }
}
