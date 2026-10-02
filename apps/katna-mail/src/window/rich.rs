// SPDX-License-Identifier: GPL-3.0-or-later

//! Draws an HTML message ([`katna_render::html::Document`]) with GPUI
//! elements: boxes become flex boxes, table rows become rows of cells,
//! paragraphs become styled text with clickable links.
//!
//! A link's text can say anything, so the address a link really opens
//! shows at the foot of the reading pane while the pointer is on it, as
//! in a browser ([`link_status`]): its host stands out, and a host in
//! another script shows as the punycode the network sees.

use std::collections::HashMap;
use std::sync::Arc;

use gpui::{
    AnyElement, App, FontStyle, FontWeight, HighlightStyle, InteractiveText, ObjectFit,
    SharedString, StrikethroughStyle, UnderlineStyle, WeakEntity, div, img, prelude::*, relative,
    rgba,
};
use katna_render::html::{
    Align, Block, BoxBlock, BoxKind, Document, Image, ImageKind, ImageSource, Inline, Length,
    TextBlock,
};
use katna_ui::px;

use super::MailWindow;
use super::dark::Dark;
use super::remote::{Fetch, MailImage, svg_key};
use super::select::Pieces;
use crate::theme::Theme;
use crate::widgets::{ScaledEdge, icon};

/// Mail is written for browsers, where normal text is 16 CSS pixels; the
/// app's body text is 14.
const SCALE: f32 = 14.0 / 16.0;

/// Most remote images one message loads: a message naming thousands would
/// have the daemon open thousands of connections at once.
pub(super) const MAX_REMOTE_IMAGES: usize = 200;
/// Most SVG pictures carried in one message that are drawn.
const MAX_DRAWN_SVGS: usize = 100;

/// Where the reading pane learns which link is under the pointer.
#[derive(Clone)]
pub(super) struct Links {
    pub window: WeakEntity<MailWindow>,
    /// The conversation ([`HoveredLink::conversation`]) and the place of
    /// the message in it.
    pub conversation: usize,
    pub part: usize,
}

/// The link under the pointer in the reading pane.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct HoveredLink {
    /// Which drawn link: conversation, message and element.
    owner: (usize, usize, usize),
    pub url: String,
}

impl HoveredLink {
    /// The conversation it is in, so it goes when another one opens.
    pub(super) fn conversation(&self) -> usize {
        self.owner.0
    }
}

impl Links {
    /// The pointer is on link `element` (to `url`), or left it (`None`).
    fn hover(&self, element: usize, url: Option<String>, cx: &mut App) {
        let owner = (self.conversation, self.part, element);
        self.window
            .update(cx, |this, cx| {
                let next = match url {
                    Some(url) => Some(HoveredLink { owner, url }),
                    // Another link took over already.
                    None if this.hovered_link.as_ref().is_some_and(|h| h.owner != owner) => {
                        return;
                    }
                    None => None,
                };
                if this.hovered_link != next {
                    this.hovered_link = next;
                    cx.notify();
                }
            })
            .ok();
    }
}

/// Text colors for a message page.
#[derive(Clone, Copy)]
struct Ink {
    text: u32,
    link: u32,
    faint: u32,
    rule: u32,
    quote: u32,
}

pub(super) struct Painter<'a> {
    ink: Ink,
    th: &'a Theme,
    /// Remote images fetched so far, by URL.
    images: &'a HashMap<String, Fetch>,
    /// SVG pictures of the message drawn so far ([`svg_key`]).
    drawn: &'a HashMap<usize, (Arc<[u8]>, Fetch)>,
    /// Where the address of the link under the pointer goes.
    links: Option<Links>,
    /// Remote images may be drawn (the user allowed them).
    remote: bool,
    mono: Option<SharedString>,
    next_id: usize,
    /// Set in a dark theme: the message's colors are remapped.
    dark: Option<Dark>,
    /// The background under what is being drawn, as drawn.
    bg: u32,
    /// Its text, selectable.
    pieces: Pieces,
}

impl<'a> Painter<'a> {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        th: &'a Theme,
        images: &'a HashMap<String, Fetch>,
        drawn: &'a HashMap<usize, (Arc<[u8]>, Fetch)>,
        links: Option<Links>,
        remote: bool,
        mono: Option<SharedString>,
        dark_mail: bool,
        pieces: Pieces,
    ) -> Self {
        Self {
            ink: Ink {
                text: th.text,
                link: th.accent,
                faint: th.text_faint,
                rule: th.divider,
                quote: th.divider,
            },
            th,
            images,
            drawn,
            links,
            remote,
            mono,
            next_id: 0,
            // Without `dark_mail`, mail keeps its colors, as in a light theme.
            dark: (th.dark && dark_mail).then(|| Dark::new(th.surface)),
            bg: th.surface,
            pieces,
        }
    }

    /// A background color as drawn.
    fn fill(&self, color: u32) -> u32 {
        match &self.dark {
            Some(dark) => dark.background(color),
            None => color,
        }
    }

    /// A text color as drawn on `bg`.
    fn ink_on(&self, color: u32, bg: u32) -> u32 {
        match &self.dark {
            Some(dark) => dark.text(color, bg),
            None => color,
        }
    }

    /// Makes `color` (as drawn) the background of what follows, unless it
    /// is too faint to count; returns the one to restore.
    fn enter(&mut self, color: u32) -> u32 {
        let outer = self.bg;
        if color & 0xff >= 0x80 {
            self.bg = color;
        }
        outer
    }

    /// The whole message. In a light theme a message that sets its own
    /// colors is drawn on its own page, as its sender meant; in a dark theme
    /// its colors are remapped to dark ones ([`Dark`]).
    pub(super) fn document(mut self, doc: &Document) -> AnyElement {
        let page = if self.dark.is_some() {
            doc.background.map(|bg| self.fill(bg))
        } else if doc.styled || doc.background.is_some() {
            self.ink = Ink {
                text: 0x222222ff,
                link: 0x1a0dabff,
                faint: 0x5f6368ff,
                rule: 0xdadce0ff,
                quote: 0xccccccff,
            };
            Some(doc.background.unwrap_or(0xffffffff))
        } else {
            None
        };
        if let Some(page) = page {
            self.enter(page);
        }
        let children = self.blocks(&doc.blocks);
        div()
            .w_full()
            .flex()
            .flex_col()
            .text_color(rgba(self.ink.text))
            .when_some(page, |d, page| {
                d.bg(rgba(page))
                    .rounded(px(8.0))
                    .p(px(8.0))
                    .when(!self.th.dark, |d| {
                        d.border_1().border_color(rgba(self.th.outline))
                    })
            })
            .children(children)
            .into_any_element()
    }

    fn id(&mut self) -> usize {
        self.next_id += 1;
        self.next_id
    }

    fn blocks(&mut self, blocks: &[Block]) -> Vec<AnyElement> {
        blocks.iter().map(|b| self.block(b)).collect()
    }

    fn block(&mut self, block: &Block) -> AnyElement {
        match block {
            Block::Rule => div()
                .my(px(8.0))
                .h(px(1.0))
                .bg(rgba(self.ink.rule))
                .into_any_element(),
            Block::Text(text) => self.paragraph(text),
            Block::Box(b) => self.boxed(b, false),
        }
    }

    /// A box; `cell` when it is a cell of a table row.
    fn boxed(&mut self, b: &BoxBlock, cell: bool) -> AnyElement {
        let s = &b.style;
        let [top, right, bottom, left] = s.padding.map(|p| px(p.min(96.0)));
        let background = s.background.map(|bg| self.fill(bg));
        let outer = self.enter(background.unwrap_or(0));
        let children: Vec<AnyElement> = match &b.kind {
            BoxKind::Row => b.children.iter().map(|c| self.cell(c)).collect(),
            _ => self.blocks(&b.children),
        };
        self.bg = outer;
        let mut d = div()
            .min_w_0()
            .flex()
            .pt(top)
            .pr(right)
            .pb(bottom)
            .pl(left)
            .mt(px(s.margin[0].min(64.0) * SCALE))
            .mb(px(s.margin[1].min(64.0) * SCALE));
        d = match &b.kind {
            BoxKind::Row => d.flex_row().items_start(),
            _ => d.flex_col(),
        };
        if let Some(bg) = background {
            d = d.bg(rgba(bg));
        }
        if let Some((width, color)) = s.border {
            d = if width <= 1.5 {
                d.border_1()
            } else if width <= 3.0 {
                d.border_px(2.0)
            } else {
                d.border_px(4.0)
            }
            .border_color(rgba(self.fill(color)));
        } else if let Some((width, color)) = s.border_top.or(s.border_bottom) {
            // A divider along the top or bottom edge.
            let width: gpui::AbsoluteLength = px(width.clamp(1.0, 4.0).round()).into();
            let edges = &mut d.style().border_widths;
            if s.border_top.is_some() {
                edges.top = Some(width);
            }
            if s.border_bottom.is_some() {
                edges.bottom = Some(width);
            }
            d = d.border_color(rgba(self.fill(color)));
        }
        if s.radius > 0.0 {
            d = d.rounded(px(s.radius.min(48.0)));
        }
        if !cell {
            match s.width {
                // A preferred width that gives way in a narrow pane.
                Some(Length::Px(w)) => d = d.w_full().max_w(px(w)),
                Some(Length::Percent(p)) => d = d.w(relative(p.min(1.0))),
                // As wide as what holds it, as a block is in a browser.
                None if !s.center && !s.inline => d = d.w_full(),
                None => {}
            }
        } else {
            d = d.w_full();
        }
        if let Some(max) = s.max_width {
            d = d.max_w(px(max));
        }
        if s.center {
            d = d.mx_auto();
        }
        if s.inline {
            // A button: as wide as its text, placed in its own line.
            let line = div().w_full().flex().flex_row();
            let line = match s.align {
                Align::Start => line,
                Align::Center => line.justify_center(),
                Align::End => line.justify_end(),
            };
            return line
                .child(
                    d.flex_none()
                        .max_w_full()
                        .whitespace_nowrap()
                        .overflow_hidden()
                        .children(children),
                )
                .into_any_element();
        }
        match &b.kind {
            BoxKind::Quote => d
                .border_l_2()
                .border_color(rgba(self.ink.quote))
                .text_color(rgba(self.ink.faint))
                .children(children)
                .into_any_element(),
            BoxKind::ListItem(marker) => div()
                .flex()
                .flex_row()
                .min_w_0()
                .child(
                    div()
                        .flex_none()
                        .w(px(28.0))
                        .pr(px(6.0))
                        .flex()
                        .justify_end()
                        .child(SharedString::from(marker.clone())),
                )
                .child(d.flex_1().children(children))
                .into_any_element(),
            _ => d.children(children).into_any_element(),
        }
    }

    /// A cell of a table row: its width becomes its share of the row.
    fn cell(&mut self, block: &Block) -> AnyElement {
        let Block::Box(b) = block else {
            return self.block(block);
        };
        let el = self.boxed(b, true);
        // Never narrower than its longest word, as in a browser.
        let min = b.style.min_width;
        let d = div()
            .when(min > 0.0, |d| d.min_w(px(min.ceil())))
            .when(min <= 0.0, |d| d.min_w_0())
            .flex()
            .flex_col();
        match b.style.width {
            Some(Length::Px(w)) => d.flex_basis(px(w)).flex_shrink(1.0),
            Some(Length::Percent(p)) => d.flex_basis(relative(p.min(1.0))).flex_shrink(1.0),
            None => d.flex_1(),
        }
        .child(el)
        .into_any_element()
    }

    fn paragraph(&mut self, t: &TextBlock) -> AnyElement {
        // Runs of text between images become one styled text each.
        let mut pieces: Vec<AnyElement> = Vec::new();
        let mut run_start = 0;
        let mut has_image = false;
        for (ix, inline) in t.inlines.iter().enumerate() {
            if let Inline::Image(image) = inline {
                has_image = true;
                if run_start < ix {
                    pieces.push(self.text(&t.inlines[run_start..ix], t));
                }
                pieces.push(self.image(image));
                run_start = ix + 1;
            }
        }
        if run_start < t.inlines.len() {
            pieces.push(self.text(&t.inlines[run_start..], t));
        }
        if !has_image && pieces.len() == 1 {
            let only = pieces.pop().expect("one piece");
            return align(div().w_full(), t.align)
                .child(only)
                .into_any_element();
        }
        let row = div().w_full().flex().flex_row().flex_wrap().items_end();
        match t.align {
            Align::Start => row,
            Align::Center => row.justify_center(),
            Align::End => row.justify_end(),
        }
        .children(pieces)
        .into_any_element()
    }

    /// Consecutive text runs of a paragraph.
    fn text(&mut self, inlines: &[Inline], t: &TextBlock) -> AnyElement {
        let mut text = String::new();
        let mut highlights = Vec::new();
        let mut links: Vec<(std::ops::Range<usize>, String)> = Vec::new();
        let mut size: f32 = 0.0;
        let mut all_mono = true;
        for inline in inlines {
            let Inline::Text(run) = inline else {
                continue;
            };
            let start = text.len();
            text.push_str(&run.text);
            let range = start..text.len();
            let st = &run.style;
            size = size.max(st.size);
            all_mono &= st.monospace;
            let background = st.background.map(|c| self.fill(c));
            let under = background.filter(|c| c & 0xff >= 0x80).unwrap_or(self.bg);
            let color = match st.color {
                Some(c) => Some(self.ink_on(c, under)),
                None => st.link.as_ref().map(|_| self.ink.link),
            };
            let style = HighlightStyle {
                color: color.map(|c| rgba(c).into()),
                font_weight: st.bold.then_some(FontWeight::BOLD),
                font_style: st.italic.then_some(FontStyle::Italic),
                background_color: background.map(|c| rgba(c).into()),
                underline: st.underline.then(|| UnderlineStyle {
                    thickness: px(1.0),
                    color: None,
                    wavy: false,
                }),
                strikethrough: st.strike.then(|| StrikethroughStyle {
                    thickness: px(1.0),
                    color: None,
                }),
                fade_out: None,
            };
            if style != HighlightStyle::default() {
                highlights.push((range.clone(), style));
            }
            if let Some(link) = &st.link {
                match links.last_mut() {
                    Some((last, url)) if last.end == range.start && url == link => {
                        last.end = range.end;
                    }
                    _ => links.push((range, link.clone())),
                }
            }
        }
        if !t.preformatted {
            // Long lines of `&nbsp;` and such still wrap.
            text = text.replace('\u{a0}', " ");
        }
        let size = if size > 0.0 { size } else { 16.0 };
        // Scaled like the app's text, but small print stays readable.
        let size = (size * SCALE).max(size.min(11.0));
        let (styled, holder) = self.pieces.piece(SharedString::from(text), highlights);
        let holder = holder
            .min_w_0()
            .text_size(px(size))
            .line_height(relative(1.45))
            .when(all_mono, |d| match &self.mono {
                Some(mono) => d.font_family(mono.clone()),
                None => d,
            });
        if links.is_empty() {
            return holder.child(styled).into_any_element();
        }
        let id = self.id();
        let (ranges, urls): (Vec<_>, Vec<_>) = links.into_iter().unzip();
        let urls = Arc::new(urls);
        let clicked = urls.clone();
        let mut body = InteractiveText::new(("rich-text", id), styled).on_click(
            ranges.clone(),
            move |ix, _, cx: &mut App| {
                if let Some(url) = clicked.get(ix) {
                    cx.open_url(url);
                }
            },
        );
        let Some(hover) = self.links.clone() else {
            return holder.child(body).into_any_element();
        };
        let leave = hover.clone();
        body = body.on_hover(move |ix, _, _, cx| {
            let url = ix
                .and_then(|ix| ranges.iter().position(|r| r.contains(&ix)))
                .and_then(|link| urls.get(link))
                .cloned();
            hover.hover(id, url, cx);
        });
        // The text's own hover ends only when the pointer moves on it.
        holder
            .id(("rich-links", id))
            .on_hover(move |hovered, _, cx| {
                if !*hovered {
                    leave.hover(id, None, cx);
                }
            })
            .child(body)
            .into_any_element()
    }

    fn image(&mut self, image: &Image) -> AnyElement {
        let width = match image.width {
            Some(Length::Px(w)) => Some(w),
            _ => None,
        };
        let full = matches!(image.width, Some(Length::Percent(p)) if p >= 0.99);
        let source: Option<MailImage> = match &image.source {
            // SVG is drawn to a bitmap first (`remote::mail_image`).
            ImageSource::Data { kind, bytes } => match format(*kind) {
                Some(format) => Some(MailImage {
                    image: Arc::new(gpui::Image::from_bytes(format, bytes.to_vec())),
                    size: None,
                }),
                None => match self.drawn.get(&svg_key(bytes)) {
                    Some((_, Fetch::Ready(image))) => Some(image.clone()),
                    _ => None,
                },
            },
            ImageSource::Remote(url) if self.remote => match self.images.get(url) {
                Some(Fetch::Ready(image)) => Some(image.clone()),
                _ => None,
            },
            ImageSource::Remote(_) => None,
        };
        let el = match source {
            Some(source) => {
                let mut el = img(source.image)
                    .max_w_full()
                    .object_fit(ObjectFit::Contain);
                // An SVG drawn at twice its size shows at its own size,
                // unless the message gives one.
                if let Some((w, _)) = source
                    .size
                    .filter(|_| width.is_none() && image.height.is_none())
                {
                    el = el.w(px(w));
                }
                if let Some(w) = width {
                    el = el.w(px(w));
                }
                if let Some(h) = image.height {
                    el = el.h(px(h));
                }
                if full {
                    el = el.w_full();
                }
                el.into_any_element()
            }
            None => self.missing_image(image, width),
        };
        match &image.link {
            Some(link) => {
                let id = self.id();
                let hover = self.links.clone();
                let shown = link.clone();
                let link = link.clone();
                div()
                    .id(("rich-image", id))
                    .max_w_full()
                    .cursor_pointer()
                    .on_click(move |_, _, cx| cx.open_url(&link))
                    .when_some(hover, |d, hover| {
                        d.on_hover(move |hovered, _, cx| {
                            hover.hover(id, hovered.then(|| shown.clone()), cx);
                        })
                    })
                    .child(el)
                    .into_any_element()
            }
            None => el,
        }
    }

    /// Where a remote image goes while it is hidden or loading: a quiet
    /// frame of its size with its description.
    fn missing_image(&self, image: &Image, width: Option<f32>) -> AnyElement {
        let full = matches!(image.width, Some(Length::Percent(p)) if p >= 0.99);
        let big =
            (full || width.is_some_and(|w| w >= 48.0)) && image.height.is_some_and(|h| h >= 32.0);
        let label = (!image.alt.is_empty()).then(|| SharedString::from(image.alt.clone()));
        if !big && label.is_none() {
            return div().into_any_element();
        }
        div()
            .max_w_full()
            .when_some(width.filter(|_| big), |d, w| d.w(px(w)))
            .when(big && full, |d| d.w_full())
            .when_some(image.height.filter(|_| big), |d, h| d.h(px(h.min(400.0))))
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .gap(px(6.0))
            .p(px(4.0))
            .rounded(px(4.0))
            .border_1()
            .border_color(rgba(self.ink.rule))
            .overflow_hidden()
            .text_size(px(12.0))
            .text_color(rgba(self.ink.faint))
            .when(big, |d| d.child(icon("image", self.ink.faint, 18.0)))
            .when_some(label, |d, label| {
                d.child(div().min_w_0().truncate().child(label))
            })
            .into_any_element()
    }
}

fn align(d: gpui::Div, align: Align) -> gpui::Div {
    match align {
        Align::Start => d,
        Align::Center => d.text_center(),
        Align::End => d.text_right(),
    }
}

/// The format GPUI decodes a picture of `kind` as; `None` for SVG, which
/// mail never hands GPUI as it is: GPUI would load the files it links to.
/// It is drawn to a bitmap by `katna_preview::svg` instead.
pub(super) fn format(kind: ImageKind) -> Option<gpui::ImageFormat> {
    Some(match kind {
        ImageKind::Png => gpui::ImageFormat::Png,
        ImageKind::Jpeg => gpui::ImageFormat::Jpeg,
        ImageKind::Gif => gpui::ImageFormat::Gif,
        ImageKind::Webp => gpui::ImageFormat::Webp,
        ImageKind::Bmp => gpui::ImageFormat::Bmp,
        ImageKind::Ico => gpui::ImageFormat::Ico,
        ImageKind::Svg => return None,
    })
}

/// The kind of a picture GPUI decoded, for printing it.
pub(super) fn kind(format: gpui::ImageFormat) -> Option<ImageKind> {
    Some(match format {
        gpui::ImageFormat::Png => ImageKind::Png,
        gpui::ImageFormat::Jpeg => ImageKind::Jpeg,
        gpui::ImageFormat::Gif => ImageKind::Gif,
        gpui::ImageFormat::Webp => ImageKind::Webp,
        gpui::ImageFormat::Bmp => ImageKind::Bmp,
        gpui::ImageFormat::Ico => ImageKind::Ico,
        gpui::ImageFormat::Svg => ImageKind::Svg,
        gpui::ImageFormat::Tiff | gpui::ImageFormat::Pnm => return None,
    })
}

/// Calls `f` with every image of `blocks`, in order, until it says stop.
fn each_image(blocks: &[Block], f: &mut impl FnMut(&ImageSource) -> bool) -> bool {
    for block in blocks {
        let go_on = match block {
            Block::Box(b) => each_image(&b.children, f),
            Block::Text(t) => t.inlines.iter().all(|inline| match inline {
                Inline::Image(image) => f(&image.source),
                _ => true,
            }),
            Block::Rule => true,
        };
        if !go_on {
            return false;
        }
    }
    true
}

/// The remote image URLs of `doc`, each once, at most
/// [`MAX_REMOTE_IMAGES`]: the rest stay hidden.
pub(super) fn remote_urls(doc: &Document) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    each_image(&doc.blocks, &mut |source| {
        if let ImageSource::Remote(url) = source
            && !out.contains(url)
        {
            out.push(url.clone());
        }
        out.len() < MAX_REMOTE_IMAGES
    });
    out
}

/// The links of `doc` (of text and of pictures), in order, at most
/// [`MAX_LINKS`]: where its call links are looked for.
pub(super) fn links(doc: &Document) -> Vec<&str> {
    fn walk<'a>(blocks: &'a [Block], out: &mut Vec<&'a str>) {
        for block in blocks {
            if out.len() >= MAX_LINKS {
                return;
            }
            match block {
                Block::Box(b) => walk(&b.children, out),
                Block::Text(t) => out.extend(t.inlines.iter().filter_map(|inline| match inline {
                    Inline::Text(run) => run.style.link.as_deref(),
                    Inline::Image(image) => image.link.as_deref(),
                })),
                Block::Rule => {}
            }
        }
    }
    let mut out = Vec::new();
    walk(&doc.blocks, &mut out);
    out.dedup();
    out.truncate(MAX_LINKS);
    out
}

/// At most this many links of a mail are looked at for call links.
const MAX_LINKS: usize = 500;

/// The SVG pictures `doc` carries, each once, at most [`MAX_DRAWN_SVGS`].
pub(super) fn carried_svgs(doc: &Document) -> Vec<Arc<[u8]>> {
    let mut out: Vec<Arc<[u8]>> = Vec::new();
    each_image(&doc.blocks, &mut |source| {
        if let ImageSource::Data {
            kind: ImageKind::Svg,
            bytes,
        } = source
            && !out.iter().any(|b| Arc::ptr_eq(b, bytes))
        {
            out.push(bytes.clone());
        }
        out.len() < MAX_DRAWN_SVGS
    });
    out
}

/// A link's address as the foot of the reading pane shows it: what comes
/// before the host, the host (IDN as punycode; a very long one keeps its
/// end, where the name that owns it is), and the rest. A name and
/// password before the host (`https://bank.example@evil.example/`) are
/// left out: they only disguise it.
pub(super) fn link_parts(link: &str) -> (String, String, String) {
    let Ok(url) = url::Url::parse(link) else {
        return (String::new(), String::new(), link.to_owned());
    };
    let Some(host) = url.host_str() else {
        // mailto: and such: no host to single out.
        let (scheme, rest) = link.split_once(':').unwrap_or(("", link));
        return (format!("{scheme}:"), rest.to_owned(), String::new());
    };
    const MAX_HOST: usize = 64;
    let mut host = host.to_owned();
    if host.len() > MAX_HOST {
        let cut = host.len() - MAX_HOST;
        host = format!("…{}", &host[cut..]);
    }
    if let Some(port) = url.port() {
        host.push_str(&format!(":{port}"));
    }
    let mut rest = url.path().to_owned();
    if rest == "/" && url.query().is_none() && url.fragment().is_none() {
        rest.clear();
    }
    if let Some(query) = url.query() {
        rest.push('?');
        rest.push_str(query);
    }
    if let Some(fragment) = url.fragment() {
        rest.push('#');
        rest.push_str(fragment);
    }
    (format!("{}://", url.scheme()), host, rest)
}

/// The address of the link under the pointer, at the foot of the reading
/// pane: quiet, with its host standing out.
pub(super) fn link_status(link: &str, th: &Theme) -> AnyElement {
    let (before, host, rest) = link_parts(link);
    div()
        .absolute()
        .left(px(8.0))
        .bottom(px(8.0))
        .max_w(relative(0.8))
        .flex()
        .flex_row()
        .items_center()
        .px(px(8.0))
        .py(px(3.0))
        .rounded(px(6.0))
        .bg(rgba(th.read_row))
        .border_1()
        .border_color(rgba(th.outline))
        .shadow_sm()
        .text_size(px(12.0))
        .line_height(px(18.0))
        .text_color(rgba(th.text_faint))
        .whitespace_nowrap()
        .overflow_hidden()
        .child(div().flex_none().child(before))
        .child(
            div()
                .flex_none()
                .text_color(rgba(th.text))
                .font_weight(FontWeight::MEDIUM)
                .child(host),
        )
        .child(div().min_w_0().truncate().child(rest))
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_parts_single_out_the_real_host() {
        let parts = |link: &str| {
            let (a, b, c) = link_parts(link);
            (a.as_str().to_owned(), b, c)
        };
        assert_eq!(
            parts("https://paypa1-secure.example/login?next=1#top"),
            (
                "https://".to_owned(),
                "paypa1-secure.example".to_owned(),
                "/login?next=1#top".to_owned()
            )
        );
        assert_eq!(
            parts("https://www.paypal.com@evil.example/account"),
            (
                "https://".to_owned(),
                "evil.example".to_owned(),
                "/account".to_owned()
            )
        );
        // Cyrillic "а" in "pаypal": shown as the punycode it really is.
        assert_eq!(parts("https://p\u{430}ypal.com/").1, "xn--pypal-4ve.com");
        assert_eq!(
            parts("http://Example.ORG:8080"),
            (
                "http://".to_owned(),
                "example.org:8080".to_owned(),
                String::new()
            )
        );
        assert_eq!(
            parts("mailto:ada@example.org"),
            (
                "mailto:".to_owned(),
                "ada@example.org".to_owned(),
                String::new()
            )
        );
        let long = format!("https://{}.evil.example/", "a.".repeat(60));
        let host = parts(&long).1;
        assert!(host.starts_with('…') && host.ends_with(".evil.example"));
    }

    #[test]
    fn remote_images_are_capped() {
        let html: String = (0..MAX_REMOTE_IMAGES + 50)
            .map(|i| format!(r#"<img src="https://img.example/{i}.png" width="40" height="40">"#))
            .chain(std::iter::once(
                r#"<img src="https://img.example/0.png" width="40" height="40">"#.to_owned(),
            ))
            .collect();
        let doc = katna_render::html::document(&html, &|_| None);
        let urls = remote_urls(&doc);
        assert_eq!(urls.len(), MAX_REMOTE_IMAGES);
        assert_eq!(urls[0], "https://img.example/0.png");
    }
}
