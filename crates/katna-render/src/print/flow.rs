// SPDX-License-Identifier: GPL-3.0-or-later

//! The conversation placed on a strip as wide as the page's text: each
//! message's headers, then its body as the reading pane draws it
//! (`katna-mail`'s `window/rich.rs`): boxes stacked, table rows as cells
//! side by side, paragraphs wrapped at spaces with their pictures inline.

use std::sync::Arc;

use krilla::Data;
use krilla::image::Image as Bitmap;

use super::{
    BOLD, BOLD_ITALIC, Faces, ITALIC, Item, MONO, Placed, PrintMessage, PrintOptions, REGULAR,
    RemoteImages, Strip,
};
use crate::html::{
    Align, Block, BoxBlock, BoxKind, Color, DEFAULT_SIZE, Document, Image, ImageKind, ImageSource,
    Inline, Length, TextBlock,
};

/// Points per CSS pixel: the mail's 16 px text prints at 10.5 pt.
const PX: f32 = TEXT_SIZE / DEFAULT_SIZE;

const SUBJECT_SIZE: f32 = 16.0;
const TEXT_SIZE: f32 = 10.5;
const SMALL_SIZE: f32 = 9.0;
/// The smallest text printed, in points.
const MIN_SIZE: f32 = 6.0;
/// Line height as a share of the font size, as in the reader.
const LEADING: f32 = 1.45;
/// Above and below the baseline, as shares of the font size.
const ASCENT: f32 = 0.8 + (LEADING - 1.0) / 2.0;
const DESCENT: f32 = 0.2 + (LEADING - 1.0) / 2.0;

/// The reader's ink for mail on a light page.
const TEXT: Color = 0x2222_22ff;
const HEADER: Color = 0x1f1f_1fff;
const DIM: Color = 0x5f63_68ff;
const LINK: Color = 0x1a0d_abff;
const RULE: Color = 0xdadc_e0ff;
const QUOTE: Color = 0xcccc_ccff;

/// A table row no taller than this share of a page stays on one page.
const KEEP_ROW: f32 = 0.3;

pub(super) fn lay_out(
    subject: &str,
    messages: &[PrintMessage],
    width: f32,
    page: f32,
    faces: &Faces,
    options: PrintOptions,
    pictures: bool,
) -> Strip {
    let shapers = faces
        .0
        .iter()
        .map(|face| rustybuzz::Face::from_slice(&face.data, face.index))
        .collect::<Option<Vec<_>>>()
        .unwrap_or_default();
    let mut flow = Flow {
        shapers,
        strip: Strip::default(),
        page,
        backgrounds: options.backgrounds,
        pictures,
        remote: None,
    };
    let mut y = flow.line(subject, BOLD, SUBJECT_SIZE, HEADER, width, 0.0);
    y += 6.0;
    for (ix, message) in messages.iter().enumerate() {
        y = flow.rule(0.0, y, width, 5.5);
        y = flow.line(&message.from, BOLD, TEXT_SIZE, HEADER, width, y);
        for line in [&message.date, &message.to, &message.cc] {
            if !line.is_empty() {
                y = flow.line(line, REGULAR, SMALL_SIZE, DIM, width, y);
            }
        }
        y += 8.0;
        match message.document.as_ref().filter(|_| !options.simple) {
            Some(doc) => {
                flow.remote = Some(&message.images);
                y = flow.document(doc, width, y);
                flow.remote = None;
            }
            None if options.simple && message.document.is_some() => {
                let doc = message.document.as_ref().expect("a document");
                for text in simple_text(&doc.blocks) {
                    y = flow.line(&text, REGULAR, TEXT_SIZE, TEXT, width, y);
                }
            }
            None => {
                for text in message.body.lines() {
                    y = flow.line(text, REGULAR, TEXT_SIZE, TEXT, width, y);
                }
            }
        }
        if !message.attachments.is_empty() {
            y += 8.0;
            let label = if message.attachments_label.is_empty() {
                match message.attachments.len() {
                    1 => "1 attachment".to_owned(),
                    n => format!("{n} attachments"),
                }
            } else {
                message.attachments_label.clone()
            };
            y = flow.line(&label, BOLD, SMALL_SIZE, DIM, width, y);
            for name in &message.attachments {
                y = flow.line(name, REGULAR, SMALL_SIZE, DIM, width, y);
            }
        }
        if ix + 1 < messages.len() {
            y += 10.0;
        }
    }
    flow.strip.height = y;
    flow.strip
}

struct Flow<'a> {
    shapers: Vec<rustybuzz::Face<'a>>,
    strip: Strip,
    /// A page's height for text.
    page: f32,
    backgrounds: bool,
    /// Pictures are drawn (else framed like remote ones not loaded).
    pictures: bool,
    /// The remote pictures of the message being placed.
    remote: Option<&'a RemoteImages>,
}

/// How a piece of text is drawn.
#[derive(Clone, PartialEq)]
struct Style {
    face: usize,
    size: f32,
    color: Color,
    background: Option<Color>,
    underline: bool,
    strike: bool,
}

/// Text in one style, measured.
#[derive(Clone)]
struct Frag {
    text: String,
    style: Style,
    width: f32,
}

/// What a paragraph is made of, before it is broken into lines.
enum Piece {
    Text(String, Style),
    Picture(Picture),
}

#[derive(Clone)]
struct Picture {
    w: f32,
    h: f32,
    bitmap: Option<Bitmap>,
    /// Its description, in a frame where there is no bitmap.
    alt: String,
}

/// A paragraph in the pieces lines are made of.
#[derive(Clone)]
enum Atom {
    /// Text that stays on one line: a word, in one or more styles.
    Word(Vec<Frag>),
    /// Spaces, where a line may break.
    Space(Frag),
    Picture(Picture),
    /// A line break (`<br>`).
    Break,
}

impl Atom {
    fn width(&self) -> f32 {
        match self {
            Self::Word(frags) => frags.iter().map(|f| f.width).sum(),
            Self::Space(frag) => frag.width,
            Self::Picture(p) => p.w,
            Self::Break => 0.0,
        }
    }

    /// Above and below the baseline.
    fn extent(&self) -> (f32, f32) {
        match self {
            Self::Word(frags) => frags.iter().fold((0.0, 0.0), |(a, d), f| {
                (a.max(f.style.size * ASCENT), d.max(f.style.size * DESCENT))
            }),
            Self::Picture(p) => (p.h, 0.0),
            Self::Space(_) | Self::Break => (0.0, 0.0),
        }
    }
}

/// Colors the text of a box inherits.
#[derive(Clone, Copy)]
struct Ink {
    text: Color,
}

fn alpha(color: Color) -> u32 {
    color & 0xff
}

/// Whether `color` is light enough to vanish on white paper.
fn light(color: Color) -> bool {
    let [r, g, b, _] = color.to_be_bytes();
    let luma = 0.299 * f32::from(r) + 0.587 * f32::from(g) + 0.114 * f32::from(b);
    luma > 190.0
}

impl<'a> Flow<'a> {
    /// How wide `text` is in `face` at `size`, shaped as krilla shapes it.
    fn measure(&self, face: usize, text: &str, size: f32) -> f32 {
        let Some(shaper) = self.shapers.get(face) else {
            return 0.0;
        };
        let mut buffer = rustybuzz::UnicodeBuffer::new();
        buffer.push_str(text);
        buffer.guess_segment_properties();
        let output = rustybuzz::shape(shaper, &[], buffer);
        let units: i32 = output.glyph_positions().iter().map(|p| p.x_advance).sum();
        units as f32 / f32::from(shaper.units_per_em() as u16) * size
    }

    fn push(&mut self, top: f32, bottom: f32, item: Item) -> usize {
        self.strip.items.push(Placed { top, bottom, item });
        self.strip.items.len() - 1
    }

    /// A rectangle; `ring` draws only its outline, that wide.
    #[allow(clippy::too_many_arguments)]
    fn shape(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radius: f32,
        color: Color,
        ring: Option<f32>,
    ) -> usize {
        self.push(
            y,
            y + h,
            Item::Shape {
                x,
                y,
                w,
                h,
                radius,
                color,
                ring,
            },
        )
    }

    /// Gives the shape at `index`, placed before what it is behind, its
    /// height.
    fn stretch(&mut self, index: usize, height: f32) {
        let placed = &mut self.strip.items[index];
        if let Item::Shape { h, y, .. } = &mut placed.item {
            *h = height.max(0.0);
            placed.bottom = *y + *h;
        }
    }

    /// Makes the shape at `index` taller by `by`.
    fn grow(&mut self, index: usize, by: f32) {
        let placed = &mut self.strip.items[index];
        if let Item::Shape { h, .. } = &mut placed.item {
            *h += by.max(0.0);
            placed.bottom += by.max(0.0);
        }
    }

    /// A thin line across `width`, with `gap` above and below.
    fn rule(&mut self, x: f32, y: f32, width: f32, gap: f32) -> f32 {
        let top = y + gap;
        self.shape(x, top, width, 0.75, 0.0, RULE, None);
        self.strip.keep.push((y, top + 0.75 + gap));
        top + 0.75 + gap
    }

    /// Plain text in one style, wrapped at spaces; its spaces are kept.
    fn line(
        &mut self,
        text: &str,
        face: usize,
        size: f32,
        color: Color,
        width: f32,
        y: f32,
    ) -> f32 {
        let style = Style {
            face,
            size,
            color,
            background: None,
            underline: false,
            strike: false,
        };
        let atoms = self.atoms(vec![Piece::Text(text.to_owned(), style)], true);
        self.paragraph(atoms, Align::Start, 0.0, y, width, size)
    }

    /// An HTML body, on its own page color when it asks for one.
    fn document(&mut self, doc: &Document, width: f32, y: f32) -> f32 {
        let page = doc
            .background
            .filter(|&c| self.backgrounds && alpha(c) > 0 && c | 0xff != 0xffff_ffff);
        let ink = Ink { text: TEXT };
        let Some(page) = page else {
            return self.blocks(&doc.blocks, 0.0, y, width, ink);
        };
        let inset = 6.0;
        let back = self.shape(0.0, y, width, 0.0, 5.0, page, None);
        let end = self.blocks(&doc.blocks, inset, y + inset, width - 2.0 * inset, ink);
        self.stretch(back, end + inset - y);
        end + inset
    }

    fn blocks(&mut self, blocks: &[Block], x: f32, mut y: f32, w: f32, ink: Ink) -> f32 {
        for block in blocks {
            y = self.block(block, x, y, w, ink);
        }
        y
    }

    fn block(&mut self, block: &Block, x: f32, y: f32, w: f32, ink: Ink) -> f32 {
        match block {
            Block::Rule => self.rule(x, y, w, 5.0),
            Block::Text(t) => {
                let strut = t
                    .inlines
                    .iter()
                    .find_map(|i| match i {
                        Inline::Text(run) => Some(size(run.style.size)),
                        Inline::Image(_) => None,
                    })
                    .unwrap_or(TEXT_SIZE);
                let pieces = self.pieces(t, w, ink);
                let atoms = self.atoms(pieces, t.preformatted);
                self.paragraph(atoms, t.align, x, y, w, strut)
            }
            Block::Box(b) => self.boxed(b, x, y, w, ink, false),
        }
    }

    /// A box; `cell` when it is a cell of a table row, `w` wide.
    fn boxed(&mut self, b: &BoxBlock, x: f32, y: f32, w: f32, ink: Ink, cell: bool) -> f32 {
        self.framed(b, x, y, w, ink, cell).0
    }

    /// [`Self::boxed`], with the shapes as tall as the box (its background,
    /// border and quote bar), for a table row to stretch.
    fn framed(
        &mut self,
        b: &BoxBlock,
        x: f32,
        y: f32,
        w: f32,
        ink: Ink,
        cell: bool,
    ) -> (f32, Vec<usize>) {
        let s = &b.style;
        let y = y + s.margin[0].clamp(0.0, 64.0) * PX;
        let [pt, pr, pb, mut pl] = s.padding.map(|p| p.clamp(0.0, 96.0) * PX);
        let border = s
            .border
            .filter(|&(width, color)| width > 0.0 && alpha(color) > 0)
            .map(|(width, color)| ((width.min(4.0) * PX).max(0.5), color));
        let edge = border.map_or(0.0, |(width, _)| width);
        let radius = s.radius.min(48.0) * PX;
        let mut bw = w;
        if !cell {
            match s.width {
                Some(Length::Px(v)) => bw = (v * PX).min(w),
                Some(Length::Percent(p)) => bw = w * p.clamp(0.0, 1.0),
                None => {}
            }
        }
        if let Some(max) = s.max_width {
            bw = bw.min(max * PX);
        }
        let marker = match &b.kind {
            BoxKind::ListItem(marker) => Some(marker),
            _ => None,
        };
        let marker_w = if marker.is_some() { 28.0 * PX } else { 0.0 };
        if matches!(b.kind, BoxKind::Quote) {
            pl = pl.max(8.0);
        }
        if s.inline {
            // A button: as wide as its text.
            let natural = self.natural(&b.children, ink) + pl + pr + 2.0 * edge + marker_w;
            bw = bw.min(natural);
        }
        let bw = bw.max(1.0);
        let bx = if s.center || (s.inline && s.align == Align::Center) {
            x + (w - bw) / 2.0
        } else if s.inline && s.align == Align::End {
            x + w - bw
        } else {
            x
        };
        let back = s
            .background
            .filter(|&c| self.backgrounds && alpha(c) > 0)
            .map(|c| self.shape(bx, y, bw, 0.0, radius, c, None));
        let inner = Ink {
            text: if matches!(b.kind, BoxKind::Quote) {
                DIM
            } else {
                ink.text
            },
        };
        let cx = bx + edge + pl + marker_w;
        let cw = (bw - 2.0 * edge - pl - pr - marker_w).max(1.0);
        let cy = y + edge + pt;
        if let Some(marker) = marker {
            let size = TEXT_SIZE;
            let width = self.measure(REGULAR, marker, size);
            let right = cx - 6.0 * PX;
            self.push(
                cy,
                cy + size * LEADING,
                Item::Text {
                    x: (right - width).max(bx),
                    baseline: cy + size * ASCENT,
                    face: REGULAR,
                    size,
                    color: inner.text,
                    text: marker.clone(),
                },
            );
        }
        let end = match &b.kind {
            BoxKind::Row => self.row(&b.children, cx, cy, cw, inner),
            _ => self.blocks(&b.children, cx, cy, cw, inner),
        };
        let bottom = end + pb + edge;
        let mut frame = Vec::new();
        if let Some(back) = back {
            self.stretch(back, bottom - y);
            frame.push(back);
        }
        if let Some((width, color)) = border {
            frame.push(self.shape(bx, y, bw, bottom - y, radius, color, Some(width)));
        }
        if matches!(b.kind, BoxKind::Quote) {
            frame.push(self.shape(
                bx + edge,
                y + edge,
                1.5,
                bottom - y - 2.0 * edge,
                0.0,
                QUOTE,
                None,
            ));
        }
        if s.inline || (matches!(b.kind, BoxKind::Row) && bottom - y <= KEEP_ROW * self.page) {
            self.strip.keep.push((y, bottom));
        }
        (bottom + s.margin[1].clamp(0.0, 64.0) * PX, frame)
    }

    /// A table row: cells side by side, each as wide as it asks when the
    /// row has room, the rest sharing what is left.
    fn row(&mut self, cells: &[Block], x: f32, y: f32, w: f32, ink: Ink) -> f32 {
        let bases: Vec<Option<f32>> = cells
            .iter()
            .map(|cell| match cell {
                Block::Box(b) => match b.style.width {
                    Some(Length::Px(v)) => Some(v * PX),
                    Some(Length::Percent(p)) => Some(w * p.clamp(0.0, 1.0)),
                    None => None,
                },
                _ => None,
            })
            .collect();
        let fixed: f32 = bases.iter().flatten().sum();
        let open = bases.iter().filter(|b| b.is_none()).count();
        // Cells that ask for more than the row has give way, leaving the
        // others some room.
        let room = if open > 0 { w * 0.7 } else { w };
        let scale = if fixed > room && fixed > 0.0 {
            room / fixed
        } else {
            1.0
        };
        let share = if open > 0 {
            ((w - fixed * scale) / open as f32).max(0.0)
        } else {
            0.0
        };
        let mut cx = x;
        let mut bottom = y;
        let mut frames = Vec::new();
        for (cell, basis) in cells.iter().zip(bases) {
            let cw = basis.map_or(share, |b| b * scale).max(1.0);
            let (end, frame) = match cell {
                Block::Box(b) => self.framed(b, cx, y, cw, ink, true),
                other => (self.block(other, cx, y, cw, ink), Vec::new()),
            };
            bottom = bottom.max(end);
            frames.push((end, frame));
            cx += cw;
        }
        // As in a table, every cell is as tall as the row.
        for (end, frame) in frames {
            for index in frame {
                self.grow(index, bottom - end);
            }
        }
        bottom
    }

    /// How wide `blocks` are without wrapping: their widest line.
    fn natural(&mut self, blocks: &[Block], ink: Ink) -> f32 {
        let mut widest: f32 = 0.0;
        for block in blocks {
            let width = match block {
                Block::Rule => 0.0,
                Block::Text(t) => {
                    let pieces = self.pieces(t, f32::MAX / 4.0, ink);
                    let atoms = self.atoms(pieces, t.preformatted);
                    let mut best: f32 = 0.0;
                    let mut line = 0.0;
                    for atom in &atoms {
                        if matches!(atom, Atom::Break) {
                            best = best.max(line);
                            line = 0.0;
                        } else {
                            line += atom.width();
                        }
                    }
                    best.max(line)
                }
                Block::Box(b) => {
                    let s = &b.style;
                    let pad = (s.padding[1] + s.padding[3]).clamp(0.0, 192.0) * PX
                        + 2.0 * s.border.map_or(0.0, |(w, _)| (w.min(4.0) * PX).max(0.5));
                    let inner = match &b.kind {
                        BoxKind::Row => b
                            .children
                            .iter()
                            .map(|c| self.natural(std::slice::from_ref(c), ink))
                            .sum(),
                        _ => self.natural(&b.children, ink),
                    };
                    match s.width {
                        Some(Length::Px(v)) => v * PX,
                        _ => inner + pad,
                    }
                }
            };
            widest = widest.max(width);
        }
        widest
    }

    /// A paragraph's text and pictures, styled for print.
    fn pieces(&self, t: &TextBlock, w: f32, ink: Ink) -> Vec<Piece> {
        t.inlines
            .iter()
            .filter_map(|inline| match inline {
                Inline::Text(run) => {
                    let st = &run.style;
                    let face = if st.monospace {
                        MONO
                    } else {
                        match (st.bold, st.italic) {
                            (true, true) => BOLD_ITALIC,
                            (true, false) => BOLD,
                            (false, true) => ITALIC,
                            (false, false) => REGULAR,
                        }
                    };
                    let background = st.background.filter(|&c| self.backgrounds && alpha(c) > 0);
                    let mut color = match st.color {
                        Some(c) if alpha(c) > 0 => c,
                        _ if st.link.is_some() => LINK,
                        _ => ink.text,
                    };
                    // Without its background, light text would vanish.
                    if !self.backgrounds && light(color) {
                        color = ink.text;
                    }
                    Some(Piece::Text(
                        run.text.clone(),
                        Style {
                            face,
                            size: size(st.size),
                            color,
                            background,
                            underline: st.underline,
                            strike: st.strike,
                        },
                    ))
                }
                Inline::Image(image) => self.picture(image, w).map(Piece::Picture),
            })
            .collect()
    }

    /// A picture sized as the reader sizes it, no wider than `w` nor
    /// taller than a page. One not loaded (or not printable) is a frame
    /// with its description, or nothing when it is small and has none.
    fn picture(&self, image: &Image, w: f32) -> Option<Picture> {
        let bitmap = if self.pictures {
            match &image.source {
                ImageSource::Data { kind, bytes } => decode(*kind, bytes),
                ImageSource::Remote(url) => self
                    .remote
                    .and_then(|remote| remote.get(url))
                    .and_then(|(kind, bytes)| decode(*kind, bytes)),
            }
        } else {
            None
        };
        let asked_w = match image.width {
            Some(Length::Px(v)) => Some(v * PX),
            Some(Length::Percent(p)) => Some(w * p.clamp(0.0, 1.0)),
            None => None,
        };
        let asked_h = image.height.map(|h| h * PX);
        let Some(bitmap) = bitmap else {
            let full = matches!(image.width, Some(Length::Percent(p)) if p >= 0.99);
            let px_w = match image.width {
                Some(Length::Px(v)) => Some(v),
                _ => None,
            };
            let big = (full || px_w.is_some_and(|v| v >= 48.0))
                && image.height.is_some_and(|h| h >= 32.0);
            if big {
                return Some(Picture {
                    w: asked_w.unwrap_or(w).min(w),
                    h: (image.height.unwrap_or(32.0).min(400.0) * PX).min(self.page * 0.9),
                    bitmap: None,
                    alt: image.alt.clone(),
                });
            }
            if image.alt.is_empty() {
                return None;
            }
            let label = self.measure(REGULAR, &image.alt, ALT_SIZE);
            return Some(Picture {
                w: (label + 8.0).min(w),
                h: ALT_SIZE * LEADING + 4.0,
                bitmap: None,
                alt: image.alt.clone(),
            });
        };
        let (iw, ih) = bitmap.size();
        let (iw, ih) = (iw.max(1) as f32, ih.max(1) as f32);
        let (mut pw, mut ph) = match (asked_w, asked_h) {
            (Some(aw), Some(ah)) => {
                // Contained in the box it asks for.
                let s = (aw / iw).min(ah / ih);
                (iw * s, ih * s)
            }
            (Some(aw), None) => (aw, aw * ih / iw),
            (None, Some(ah)) => (ah * iw / ih, ah),
            (None, None) => (iw * PX, ih * PX),
        };
        let fit = (w / pw).min(self.page * 0.9 / ph).min(1.0);
        pw *= fit;
        ph *= fit;
        Some(Picture {
            w: pw.max(0.5),
            h: ph.max(0.5),
            bitmap: Some(bitmap),
            alt: String::new(),
        })
    }

    /// Splits pieces into words, spaces and breaks. Outside `pre`, runs
    /// of white space are one space and `&nbsp;` wraps like a space.
    fn atoms(&self, pieces: Vec<Piece>, pre: bool) -> Vec<Atom> {
        let mut out: Vec<Atom> = Vec::new();
        let mut word: Vec<Frag> = Vec::new();
        // At the start, and after a space: a space here is collapsed.
        let mut after_space = true;
        for piece in pieces {
            let (text, style) = match piece {
                Piece::Picture(picture) => {
                    self.end_word(&mut out, &mut word);
                    out.push(Atom::Picture(picture));
                    after_space = false;
                    continue;
                }
                Piece::Text(text, style) => (text, style),
            };
            let mut current = String::new();
            for ch in text.chars() {
                let ch = match ch {
                    '\t' if !pre => ' ',
                    '\u{a0}' if !pre => ' ',
                    '\r' => continue,
                    c => c,
                };
                if ch == '\n' {
                    self.add_frag(&mut word, &mut current, &style);
                    self.end_word(&mut out, &mut word);
                    out.push(Atom::Break);
                    after_space = true;
                    continue;
                }
                if ch == ' ' || ch == '\t' {
                    self.add_frag(&mut word, &mut current, &style);
                    self.end_word(&mut out, &mut word);
                    if !pre && after_space {
                        continue;
                    }
                    let space = if ch == '\t' { "    " } else { " " };
                    match out.last_mut() {
                        Some(Atom::Space(frag)) if frag.style == style => {
                            frag.text.push_str(space);
                            frag.width = self.measure(style.face, &frag.text, style.size);
                        }
                        _ => out.push(Atom::Space(Frag {
                            text: space.to_owned(),
                            width: self.measure(style.face, space, style.size),
                            style: style.clone(),
                        })),
                    }
                    after_space = true;
                    continue;
                }
                if ch.is_control() {
                    continue;
                }
                current.push(ch);
                after_space = false;
            }
            self.add_frag(&mut word, &mut current, &style);
        }
        self.end_word(&mut out, &mut word);
        out
    }

    fn add_frag(&self, word: &mut Vec<Frag>, text: &mut String, style: &Style) {
        if text.is_empty() {
            return;
        }
        let text = std::mem::take(text);
        word.push(Frag {
            width: self.measure(style.face, &text, style.size),
            text,
            style: style.clone(),
        });
    }

    fn end_word(&self, out: &mut Vec<Atom>, word: &mut Vec<Frag>) {
        if !word.is_empty() {
            out.push(Atom::Word(std::mem::take(word)));
        }
    }

    /// Breaks a word wider than `w` where it runs out.
    fn split_word(&self, frags: Vec<Frag>, w: f32) -> Vec<Atom> {
        let mut out = Vec::new();
        let mut line: Vec<Frag> = Vec::new();
        let mut line_w = 0.0;
        for frag in frags {
            let mut text = String::new();
            let mut text_w = 0.0;
            for ch in frag.text.chars() {
                let mut buf = [0; 4];
                let cw = self.measure(frag.style.face, ch.encode_utf8(&mut buf), frag.style.size);
                if line_w + text_w + cw > w && (line_w + text_w) > 0.0 {
                    if !text.is_empty() {
                        line.push(Frag {
                            text: std::mem::take(&mut text),
                            style: frag.style.clone(),
                            width: text_w,
                        });
                    }
                    out.push(Atom::Word(std::mem::take(&mut line)));
                    line_w = 0.0;
                    text_w = 0.0;
                }
                text.push(ch);
                text_w += cw;
            }
            if !text.is_empty() {
                line_w += text_w;
                line.push(Frag {
                    text,
                    style: frag.style,
                    width: text_w,
                });
            }
        }
        if !line.is_empty() {
            out.push(Atom::Word(line));
        }
        out
    }

    /// Breaks `atoms` into lines `w` wide and places them from `y`.
    /// `strut` is the text size an empty line takes.
    fn paragraph(
        &mut self,
        atoms: Vec<Atom>,
        align: Align,
        x: f32,
        mut y: f32,
        w: f32,
        strut: f32,
    ) -> f32 {
        let mut lines: Vec<Vec<Atom>> = Vec::new();
        let mut line: Vec<Atom> = Vec::new();
        let mut line_w = 0.0;
        // Spaces waiting for what follows them on the line.
        let mut pending: Vec<Atom> = Vec::new();
        // The line starts the paragraph or follows a break, where `pre`
        // text keeps its leading spaces.
        let mut hard = true;
        let mut queue: std::collections::VecDeque<Atom> = atoms.into();
        while let Some(atom) = queue.pop_front() {
            match atom {
                Atom::Break => {
                    pending.clear();
                    lines.push(std::mem::take(&mut line));
                    line_w = 0.0;
                    hard = true;
                }
                Atom::Space(_) => {
                    if line.is_empty() && !hard {
                        continue;
                    }
                    pending.push(atom);
                }
                // A word wider than the line is broken where it runs out; a
                // single glyph wider still is left to overflow.
                Atom::Word(frags)
                    if frags.iter().map(|f| f.width).sum::<f32>() > w
                        && frags.iter().map(|f| f.text.chars().count()).sum::<usize>() > 1 =>
                {
                    for (ix, piece) in self.split_word(frags, w).into_iter().enumerate() {
                        queue.insert(ix, piece);
                    }
                }
                atom => {
                    let pending_w: f32 = pending.iter().map(Atom::width).sum();
                    let width = atom.width();
                    if !line.is_empty() && line_w + pending_w + width > w {
                        lines.push(std::mem::take(&mut line));
                        pending.clear();
                        line_w = 0.0;
                        hard = false;
                    } else {
                        line_w += pending_w;
                        line.append(&mut pending);
                    }
                    line_w += width;
                    line.push(atom);
                }
            }
        }
        if !line.is_empty() || lines.is_empty() {
            if hard && line.is_empty() {
                line.append(&mut pending);
            }
            lines.push(line);
        }
        for line in lines {
            let (mut up, mut down) = line.iter().fold((0.0f32, 0.0f32), |(a, d), atom| {
                let (aa, dd) = atom.extent();
                (a.max(aa), d.max(dd))
            });
            if up == 0.0 && down == 0.0 {
                up = strut * ASCENT;
                down = strut * DESCENT;
            }
            let width: f32 = line.iter().map(Atom::width).sum();
            let mut cx = x + match align {
                Align::Start => 0.0,
                Align::Center => ((w - width) / 2.0).max(0.0),
                Align::End => (w - width).max(0.0),
            };
            let top = y;
            let bottom = y + up + down;
            let baseline = y + up;
            for atom in line {
                let width = atom.width();
                match atom {
                    Atom::Word(frags) => {
                        for frag in frags {
                            self.frag(&frag, cx, baseline, top, bottom);
                            cx += frag.width;
                        }
                        continue;
                    }
                    Atom::Space(frag) => self.frag(&frag, cx, baseline, top, bottom),
                    Atom::Picture(picture) => {
                        self.draw_picture(&picture, cx, baseline, top, bottom)
                    }
                    Atom::Break => {}
                }
                cx += width;
            }
            self.strip.keep.push((top, bottom));
            y = bottom;
        }
        y
    }

    fn frag(&mut self, frag: &Frag, x: f32, baseline: f32, top: f32, bottom: f32) {
        let st = &frag.style;
        if let Some(bg) = st.background {
            self.shape(
                x,
                baseline - st.size * 0.95,
                frag.width,
                st.size * 1.25,
                0.0,
                bg,
                None,
            );
        }
        if !frag.text.trim().is_empty() {
            self.push(
                top,
                bottom,
                Item::Text {
                    x,
                    baseline,
                    face: st.face,
                    size: st.size,
                    color: st.color,
                    text: frag.text.clone(),
                },
            );
        }
        let thick = (st.size * 0.06).max(0.5);
        if st.underline {
            self.shape(
                x,
                baseline + st.size * 0.12,
                frag.width,
                thick,
                0.0,
                st.color,
                None,
            );
        }
        if st.strike {
            self.shape(
                x,
                baseline - st.size * 0.3,
                frag.width,
                thick,
                0.0,
                st.color,
                None,
            );
        }
    }

    fn draw_picture(&mut self, p: &Picture, x: f32, baseline: f32, top: f32, bottom: f32) {
        let y = baseline - p.h;
        match &p.bitmap {
            Some(bitmap) => {
                self.strip.pictures = true;
                self.push(
                    y,
                    y + p.h,
                    Item::Picture {
                        x,
                        y,
                        w: p.w,
                        h: p.h,
                        image: bitmap.clone(),
                    },
                );
            }
            None => {
                self.shape(x, y, p.w, p.h, 3.0, RULE, Some(0.5));
                if !p.alt.is_empty() {
                    let text = self.fit(&p.alt, p.w - 8.0);
                    let width = self.measure(REGULAR, &text, ALT_SIZE);
                    self.push(
                        top,
                        bottom,
                        Item::Text {
                            x: x + (p.w - width) / 2.0,
                            baseline: y + p.h / 2.0 + ALT_SIZE * 0.3,
                            face: REGULAR,
                            size: ALT_SIZE,
                            color: DIM,
                            text,
                        },
                    );
                }
            }
        }
    }

    /// `text` cut to fit `w`, with an ellipsis where it was cut.
    fn fit(&self, text: &str, w: f32) -> String {
        if self.measure(REGULAR, text, ALT_SIZE) <= w {
            return text.to_owned();
        }
        let mut cut: String = text.to_owned();
        while !cut.is_empty() {
            cut.pop();
            let candidate = format!("{}…", cut.trim_end());
            if self.measure(REGULAR, &candidate, ALT_SIZE) <= w {
                return candidate;
            }
        }
        String::new()
    }
}

/// An HTML body as lines of text: a line per paragraph, a table row's
/// cells on one line, list items with their markers, quotes behind "> ",
/// and a blank line after each paragraph.
fn simple_text(blocks: &[Block]) -> Vec<String> {
    fn walk(blocks: &[Block], out: &mut Vec<String>) {
        for block in blocks {
            match block {
                Block::Text(t) => {
                    let text = t.text().replace('\u{a0}', " ");
                    out.extend(text.lines().map(|line| {
                        if t.preformatted {
                            line.trim_end().to_owned()
                        } else {
                            line.trim().to_owned()
                        }
                    }));
                }
                Block::Rule => blank(out),
                Block::Box(b) => match &b.kind {
                    BoxKind::Row => {
                        let cells: Vec<String> = b
                            .children
                            .iter()
                            .map(|cell| {
                                let mut lines = Vec::new();
                                walk(std::slice::from_ref(cell), &mut lines);
                                lines
                                    .iter()
                                    .map(|l| l.trim())
                                    .filter(|l| !l.is_empty())
                                    .collect::<Vec<_>>()
                                    .join(" ")
                            })
                            .filter(|cell| !cell.is_empty())
                            .collect();
                        if !cells.is_empty() {
                            out.push(cells.join("    "));
                        }
                    }
                    BoxKind::ListItem(marker) => {
                        let mut lines = Vec::new();
                        walk(&b.children, &mut lines);
                        let mut first = true;
                        for line in lines.into_iter().filter(|l| !l.is_empty()) {
                            let lead = if first {
                                format!("{marker} ")
                            } else {
                                "  ".into()
                            };
                            out.push(format!("{lead}{line}"));
                            first = false;
                        }
                    }
                    BoxKind::Quote => {
                        let mut lines = Vec::new();
                        walk(&b.children, &mut lines);
                        out.extend(lines.into_iter().map(|l| format!("> {l}")));
                        blank(out);
                    }
                    BoxKind::Stack => {
                        walk(&b.children, out);
                        if b.style.margin[1] > 0.0 {
                            blank(out);
                        }
                    }
                },
            }
        }
    }
    fn blank(out: &mut Vec<String>) {
        if out.last().is_some_and(|l| !l.is_empty()) {
            out.push(String::new());
        }
    }
    let mut out = Vec::new();
    walk(blocks, &mut out);
    while out.last().is_some_and(String::is_empty) {
        out.pop();
    }
    out
}

/// The size a picture's description is written in.
const ALT_SIZE: f32 = 12.0 * PX;

/// Text of `px` CSS pixels, in points.
fn size(px: f32) -> f32 {
    (px * PX).max(MIN_SIZE)
}

/// A picture krilla can put in a PDF. Other kinds (SVG, BMP, icons) print
/// as a frame.
fn decode(kind: ImageKind, bytes: &Arc<[u8]>) -> Option<Bitmap> {
    let data = || Data::from(bytes.to_vec());
    match kind {
        ImageKind::Png => Bitmap::from_png(data(), true).ok(),
        ImageKind::Jpeg => Bitmap::from_jpeg(data(), true).ok(),
        ImageKind::Gif => Bitmap::from_gif(data(), true).ok(),
        ImageKind::Webp => Bitmap::from_webp(data(), true).ok(),
        ImageKind::Bmp | ImageKind::Ico | ImageKind::Svg => None,
    }
}
