// SPDX-License-Identifier: GPL-3.0-or-later

//! HTML mail as a small layout tree the app draws with its own widgets.
//!
//! There is no browser engine: the HTML is parsed (`html5ever`, so broken
//! markup recovers the way it does in a browser) and walked once, keeping
//! only an allow-list of what mail uses — text with bold, italic, underline,
//! colors and links; paragraphs, headings, lists, quotes, preformatted text,
//! rules; tables as rows of cells; boxes with backgrounds, padding, borders
//! and widths; images. Scripts, styles sheets, forms, frames, objects and
//! anything unknown never reach the tree, so nothing in a message can run
//! or load on its own. Link targets are limited to `http`, `https` and
//! `mailto`.
//!
//! Images from the message itself (`cid:` parts and `data:` URLs) are
//! kept as bytes. Remote images are only named ([`ImageSource::Remote`]);
//! whether and how they are fetched is the app's choice. Tracking pixels
//! (tiny or hidden remote images) are dropped and counted.
//!
//! [`trimmed`] lays out a message in pieces: what the sender wrote, and
//! the quote, signature and forward a chat-style view folds away.

mod build;
mod clean;
mod css;
mod cut;
mod dom;

use std::collections::HashSet;
use std::sync::Arc;

use crate::trim::{Forwarded, Trimmed};

pub use clean::{Cleaned, LeftOut, clean, web_pictures};
pub use css::{Color, Length};
pub use katna_core::bidi::Direction;
pub use katna_core::image::ImageKind;

/// A message body laid out as blocks.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Document {
    pub blocks: Vec<Block>,
    /// The page color the message asks for (`<body bgcolor>` and the like).
    pub background: Option<Color>,
    /// The message sets its own text or background colors, so it is meant
    /// for a light page whatever the app's theme.
    pub styled: bool,
    /// Remote images the message shows (not counting trackers).
    pub remote_images: usize,
    /// Tracking pixels that were dropped.
    pub trackers: usize,
    /// The message was too large and was cut.
    pub truncated: bool,
    /// The `Content-ID`s of the message's parts shown as images, so they
    /// need not be listed as attachments too.
    pub inline_ids: Vec<String>,
    /// The direction `<html dir>` or `<body dir>` gives the whole message;
    /// `None` reads left to right, as a browser does.
    pub dir: Option<Direction>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    Box(BoxBlock),
    Text(TextBlock),
    /// A horizontal rule.
    Rule,
}

/// A box of blocks: a `<div>`, a table or cell, a list item, a quote…
#[derive(Debug, Clone, PartialEq)]
pub struct BoxBlock {
    pub kind: BoxKind,
    pub style: BoxStyle,
    pub children: Vec<Block>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BoxKind {
    /// Children one under the other.
    Stack,
    /// A table row: children (cells) side by side.
    Row,
    /// A quoted part, drawn with a bar on the left.
    Quote,
    /// A list item with its marker (`•`, `3.`).
    ListItem(String),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BoxStyle {
    pub background: Option<Color>,
    /// Top, right, bottom, left, in pixels.
    pub padding: [f32; 4],
    /// Space above and below, in pixels.
    pub margin: [f32; 2],
    /// Width in its parent; a pixel width is a preferred width that gives
    /// way when there is less room.
    pub width: Option<Length>,
    pub max_width: Option<f32>,
    /// The narrowest a table cell can be, in pixels: as wide as its longest
    /// word or image, as a browser keeps it, so `width="1%"` columns do not
    /// break their text a letter per line. 0 when it has none.
    pub min_width: f32,
    /// The box sits in the middle of its parent (`align="center"` on a
    /// table, `margin: 0 auto`).
    pub center: bool,
    /// Border width in pixels and color.
    pub border: Option<(f32, Color)>,
    /// A line along the top or bottom edge only (`border-top`,
    /// `border-bottom`), as dividers are drawn; `border` wins over them.
    pub border_top: Option<(f32, Color)>,
    pub border_bottom: Option<(f32, Color)>,
    /// A line along the left or right edge (a signature's divider between
    /// its logo and its text).
    pub border_left: Option<(f32, Color)>,
    pub border_right: Option<(f32, Color)>,
    pub radius: f32,
    /// As wide as its content, like an `inline-block` button, placed by
    /// `align` in its own line.
    pub inline: bool,
    pub align: Align,
    /// The box's own `dir` attribute: it and what it holds are laid out
    /// that way (table cells from the right in right to left).
    pub dir: Option<Direction>,
}

/// Where lines of text go.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Align {
    #[default]
    Start,
    Center,
    End,
}

/// A paragraph: runs of text and inline images that wrap together.
#[derive(Debug, Clone, PartialEq)]
pub struct TextBlock {
    pub inlines: Vec<Inline>,
    pub align: Align,
    /// Keep line breaks and spaces as they are (`<pre>`).
    pub preformatted: bool,
    /// Which way the paragraph reads: the `dir` around it, else that of
    /// its first strong character; `None` when it has neither (digits
    /// only), so it reads as the box around it. `align` is relative to it.
    pub dir: Option<Direction>,
}

impl TextBlock {
    /// The text of the paragraph, images left out.
    pub fn text(&self) -> String {
        self.inlines
            .iter()
            .filter_map(|i| match i {
                Inline::Text(run) => Some(run.text.as_str()),
                Inline::Image(_) => None,
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Inline {
    Text(Run),
    Image(Image),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Run {
    pub text: String,
    pub style: RunStyle,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RunStyle {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strike: bool,
    pub monospace: bool,
    /// `None`: the page's text color (or the link color in a link).
    pub color: Option<Color>,
    pub background: Option<Color>,
    /// Font size in CSS pixels; 16 is normal text in a browser.
    pub size: f32,
    /// An `http`, `https` or `mailto` URL.
    pub link: Option<String>,
}

impl Default for RunStyle {
    fn default() -> Self {
        Self {
            bold: false,
            italic: false,
            underline: false,
            strike: false,
            monospace: false,
            color: None,
            background: None,
            size: DEFAULT_SIZE,
            link: None,
        }
    }
}

/// A browser's default font size, in CSS pixels.
pub const DEFAULT_SIZE: f32 = 16.0;

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub source: ImageSource,
    pub alt: String,
    pub width: Option<Length>,
    /// Height in pixels.
    pub height: Option<f32>,
    /// Where a click on the image goes.
    pub link: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImageSource {
    /// An image carried in the message.
    Data { kind: ImageKind, bytes: Arc<[u8]> },
    /// An image on the web, not loaded. Always `https`: `http` URLs are
    /// upgraded, since a plain-text fetch would show every network on the
    /// way what was read.
    Remote(String),
}

/// Lays out `html`. `inline_image` finds a part of the message by its
/// `Content-ID` (without angle brackets) for `cid:` images.
pub fn document(html: &str, inline_image: &dyn Fn(&str) -> Option<Arc<[u8]>>) -> Document {
    build::build(&dom::parse(html), inline_image)
}

/// Lays out `html` in pieces, as [`crate::trim::plain`] splits text.
/// `said` keeps the whole message's page color and `styled`; each piece
/// counts its own images and trackers.
pub fn trimmed(html: &str, inline_image: &dyn Fn(&str) -> Option<Arc<[u8]>>) -> Trimmed<Document> {
    let dom = dom::parse(html);
    let cuts = cut::find(&dom);
    let forward_roots = cuts.forward.as_ref().map_or(&[][..], |f| &f.roots[..]);
    let set = |parts: &[&[dom::NodeId]]| -> HashSet<dom::NodeId> {
        parts.iter().flat_map(|p| p.iter().copied()).collect()
    };
    let piece = |roots: &[dom::NodeId], skip: HashSet<dom::NodeId>| {
        let mut doc = build::build_from(&dom, roots, &skip, inline_image);
        trim_blank(&mut doc.blocks);
        doc
    };
    let some = |doc: Document| (!doc.blocks.is_empty()).then_some(doc);

    let mut said = piece(
        &[dom::DOCUMENT],
        set(&[&cuts.quoted, &cuts.signature, forward_roots]),
    );
    let quoted = (!cuts.quoted.is_empty())
        .then(|| piece(&cuts.quoted, set(&[&cuts.signature, forward_roots])))
        .and_then(some);
    let signature = (!cuts.signature.is_empty())
        .then(|| piece(&cuts.signature, set(&[&cuts.quoted, forward_roots])))
        .and_then(some);
    let forwarded = cuts.forward.as_ref().map(|f| Forwarded {
        from: f.from.clone(),
        date: f.date.clone(),
        subject: f.subject.clone(),
        body: piece(&f.body, set(&[&cuts.quoted, &cuts.signature])),
    });
    for doc in [&quoted, &signature]
        .into_iter()
        .flatten()
        .chain(forwarded.as_ref().map(|f| &f.body))
    {
        said.styled |= doc.styled;
        said.background = said.background.or(doc.background);
    }
    Trimmed {
        said,
        quoted,
        signature,
        forwarded,
    }
}

/// Drops blank lines at the start and end of `blocks`, and inside plain
/// boxes there, so a piece does not open or close on empty space.
fn trim_blank(blocks: &mut Vec<Block>) {
    fn blank(block: &mut Block, end: bool) -> bool {
        match block {
            Block::Text(t) => t.inlines.iter().all(|i| match i {
                Inline::Text(run) => run.text.trim().is_empty(),
                Inline::Image(_) => false,
            }),
            Block::Box(b)
                if b.kind == BoxKind::Stack
                    && b.style.background.is_none()
                    && b.style.border.is_none() =>
            {
                trim(&mut b.children, end);
                b.children.is_empty()
            }
            Block::Box(_) | Block::Rule => false,
        }
    }
    fn trim(blocks: &mut Vec<Block>, end: bool) {
        loop {
            let edge = if end {
                blocks.last_mut()
            } else {
                blocks.first_mut()
            };
            if !edge.is_some_and(|b| blank(b, end)) {
                break;
            }
            if end {
                blocks.pop();
            } else {
                blocks.remove(0);
            }
        }
    }
    trim(blocks, true);
    trim(blocks, false);
}

/// Whether text that came as `text/plain` is really an HTML document, as
/// some senders get wrong.
pub fn looks_like_html(text: &str) -> bool {
    let head: String = text
        .trim_start()
        .chars()
        .take(64)
        .collect::<String>()
        .to_ascii_lowercase();
    [
        "<!doctype html",
        "<html",
        "<head",
        "<body",
        "<meta ",
        "<table",
        "<div",
    ]
    .iter()
    .any(|p| head.starts_with(p))
        && text.trim_end().ends_with('>')
}
