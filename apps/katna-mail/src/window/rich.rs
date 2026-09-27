// SPDX-License-Identifier: GPL-3.0-or-later

//! Draws an HTML message ([`katna_render::html::Document`]) with GPUI
//! elements: boxes become flex boxes, table rows become rows of cells,
//! paragraphs become styled text with clickable links.

use std::collections::HashMap;
use std::sync::Arc;

use gpui::{
    AnyElement, App, FontStyle, FontWeight, HighlightStyle, InteractiveText, ObjectFit,
    SharedString, StrikethroughStyle, StyledText, UnderlineStyle, div, img, prelude::*, relative,
    rgba,
};
use katna_render::html::{
    Align, Block, BoxBlock, BoxKind, Document, Image, ImageKind, ImageSource, Inline, Length,
    TextBlock,
};
use katna_ui::px;

use super::dark::Dark;
use super::remote::Fetch;
use crate::theme::Theme;
use crate::widgets::icon;

/// Mail is written for browsers, where normal text is 16 CSS pixels; the
/// app's body text is 14.
const SCALE: f32 = 14.0 / 16.0;

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
    /// Remote images may be drawn (the user allowed them).
    remote: bool,
    mono: Option<SharedString>,
    next_id: usize,
    /// Set in a dark theme: the message's colors are remapped.
    dark: Option<Dark>,
    /// The background under what is being drawn, as drawn.
    bg: u32,
}

impl<'a> Painter<'a> {
    pub(super) fn new(
        th: &'a Theme,
        images: &'a HashMap<String, Fetch>,
        remote: bool,
        mono: Option<SharedString>,
        dark_mail: bool,
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
            remote,
            mono,
            next_id: 0,
            // Without `dark_mail`, mail keeps its colors, as in a light theme.
            dark: (th.dark && dark_mail).then(|| Dark::new(th.surface)),
            bg: th.surface,
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
                        d.border_1().border_color(rgba(self.th.divider))
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
                d.border_2()
            } else {
                d.border_4()
            }
            .border_color(rgba(self.fill(color)));
        }
        if s.radius > 0.0 {
            d = d.rounded(px(s.radius.min(48.0)));
        }
        if !cell {
            match s.width {
                // A preferred width that gives way in a narrow pane.
                Some(Length::Px(w)) => d = d.w_full().max_w(px(w)),
                Some(Length::Percent(p)) => d = d.w(relative(p.min(1.0))),
                None => {}
            }
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
        let d = div().min_w_0().flex().flex_col();
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
        let styled = StyledText::new(SharedString::from(text)).with_highlights(highlights);
        let body: AnyElement = if links.is_empty() {
            styled.into_any_element()
        } else {
            let (ranges, urls): (Vec<_>, Vec<_>) = links.into_iter().unzip();
            let urls = Arc::new(urls);
            InteractiveText::new(("rich-text", self.id()), styled)
                .on_click(ranges, move |ix, _, cx: &mut App| {
                    if let Some(url) = urls.get(ix) {
                        cx.open_url(url);
                    }
                })
                .into_any_element()
        };
        div()
            .min_w_0()
            .text_size(px(size))
            .line_height(relative(1.45))
            .when(all_mono, |d| match &self.mono {
                Some(mono) => d.font_family(mono.clone()),
                None => d,
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
        let source: Option<Arc<gpui::Image>> = match &image.source {
            ImageSource::Data { kind, bytes } => Some(Arc::new(gpui::Image::from_bytes(
                format(*kind),
                bytes.to_vec(),
            ))),
            ImageSource::Remote(url) if self.remote => match self.images.get(url) {
                Some(Fetch::Ready(image)) => Some(image.clone()),
                _ => None,
            },
            ImageSource::Remote(_) => None,
        };
        let el = match source {
            Some(source) => {
                let mut el = img(source).max_w_full().object_fit(ObjectFit::Contain);
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
                let link = link.clone();
                div()
                    .id(("rich-image", self.id()))
                    .max_w_full()
                    .cursor_pointer()
                    .on_click(move |_, _, cx| cx.open_url(&link))
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

pub(super) fn format(kind: ImageKind) -> gpui::ImageFormat {
    match kind {
        ImageKind::Png => gpui::ImageFormat::Png,
        ImageKind::Jpeg => gpui::ImageFormat::Jpeg,
        ImageKind::Gif => gpui::ImageFormat::Gif,
        ImageKind::Webp => gpui::ImageFormat::Webp,
        ImageKind::Bmp => gpui::ImageFormat::Bmp,
        ImageKind::Ico => gpui::ImageFormat::Ico,
        ImageKind::Svg => gpui::ImageFormat::Svg,
    }
}

/// Every remote image URL of `doc`.
pub(super) fn remote_urls(doc: &Document) -> Vec<String> {
    fn walk(blocks: &[Block], out: &mut Vec<String>) {
        for block in blocks {
            match block {
                Block::Box(b) => walk(&b.children, out),
                Block::Text(t) => {
                    for inline in &t.inlines {
                        if let Inline::Image(Image {
                            source: ImageSource::Remote(url),
                            ..
                        }) = inline
                            && !out.contains(url)
                        {
                            out.push(url.clone());
                        }
                    }
                }
                Block::Rule => {}
            }
        }
    }
    let mut out = Vec::new();
    walk(&doc.blocks, &mut out);
    out
}
