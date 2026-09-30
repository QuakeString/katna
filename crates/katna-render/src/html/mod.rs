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

mod build;
mod css;
mod dom;

use std::sync::Arc;

pub use css::{Color, Length};
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
    pub radius: f32,
    /// As wide as its content, like an `inline-block` button, placed by
    /// `align` in its own line.
    pub inline: bool,
    pub align: Align,
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
