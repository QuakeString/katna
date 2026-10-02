// SPDX-License-Identifier: GPL-3.0-or-later

//! Lays out and paints one paragraph of a [`RichEditor`]: text in several
//! fonts and sizes on shared baselines, wrapped at word boundaries, with
//! its selection, cursor and list marker.
//!
//! GPUI shapes a line in one font size, so each stretch of text in one
//! style is shaped on its own and the stretches are placed side by side.

use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Range;
use std::rc::Rc;

use crate::scale::px;
use gpui::{
    App, AvailableSpace, BorderStyle, Bounds, ElementId, Entity, FontStyle, FontWeight,
    GlobalElementId, Hsla, LayoutId, Pixels, Point, ShapedLine, SharedString, StrikethroughStyle,
    Style, TextAlign, TextRun, UnderlineStyle, Window, fill, point, prelude::*, quad, relative,
    size,
};
use unicode_segmentation::UnicodeSegmentation;

use super::RichEditor;
use super::doc::{Align, CharStyle, Para, Path, floor_grapheme};

/// How the editor's text looks without formatting, and the colors of
/// what it draws.
#[derive(Clone)]
pub(crate) struct TextBase {
    pub font: gpui::Font,
    pub size: Pixels,
    pub line_height: Pixels,
    pub color: Hsla,
    pub link: Hsla,
    pub accent: Hsla,
    pub misspelled: Hsla,
    pub grammar: Hsla,
    /// The installed family for each typeface, found once.
    pub families: Rc<HashMap<super::doc::Font, SharedString>>,
    /// Draw everything unformatted (plain text mode).
    pub plain: bool,
}

impl TextBase {
    fn run(&self, style: &CharStyle, len: usize, deco: Deco) -> (TextRun, Pixels) {
        let plain = self.plain;
        let mut font = self.font.clone();
        if !plain {
            if let Some(family) = self.families.get(&style.font) {
                font.family = family.clone();
            }
            if style.bold {
                font.weight = FontWeight::BOLD;
            }
            if style.italic {
                font.style = FontStyle::Italic;
            }
        }
        let link = !plain && style.link.is_some();
        let color = if deco.ghost {
            self.color.opacity(0.45)
        } else if plain {
            self.color
        } else if let Some(c) = style.color {
            rgb(c)
        } else if link {
            self.link
        } else {
            self.color
        };
        let underline = if deco.marked {
            Some(UnderlineStyle {
                color: Some(color),
                thickness: px(1.0),
                wavy: false,
            })
        } else if deco.misspelled {
            Some(UnderlineStyle {
                color: Some(self.misspelled),
                thickness: px(1.0),
                wavy: true,
            })
        } else if deco.grammar {
            // Straight and thicker, so it reads apart from spelling.
            Some(UnderlineStyle {
                color: Some(self.grammar),
                thickness: px(2.0),
                wavy: false,
            })
        } else if !deco.ghost && !plain && (style.underline || link) {
            Some(UnderlineStyle {
                color: Some(color),
                thickness: px(1.0),
                wavy: false,
            })
        } else {
            None
        };
        let strikethrough = (!deco.ghost && !plain && style.strike).then_some(StrikethroughStyle {
            color: Some(color),
            thickness: px(1.0),
        });
        let size = if plain {
            self.size
        } else {
            self.size * style.size.scale()
        };
        (
            TextRun {
                len,
                font,
                color,
                background_color: None,
                underline,
                strikethrough,
            },
            size,
        )
    }
}

pub(crate) fn rgb(color: u32) -> Hsla {
    gpui::rgb(color).into()
}

/// Extra marks on text: the IME's composition, spelling and grammar
/// mistakes.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct Deco {
    pub marked: bool,
    pub misspelled: bool,
    pub grammar: bool,
    /// A writing suggestion, not in the text.
    pub ghost: bool,
}

/// Where a paragraph's text went on screen at the last paint.
pub(crate) struct ParaLayout {
    /// In window coordinates.
    pub bounds: Bounds<Pixels>,
    pub lines: Vec<LineBox>,
}

/// One visual line.
pub(crate) struct LineBox {
    pub range: Range<usize>,
    /// From the top of the paragraph.
    pub top: Pixels,
    pub height: Pixels,
    pub baseline: Pixels,
    /// Where the line starts (alignment).
    pub x0: Pixels,
    pub pieces: Vec<Piece>,
    /// Starts or ends at a soft wrap rather than a line break.
    pub soft_start: bool,
    pub soft_end: bool,
}

/// Text in one style on one line.
pub(crate) struct Piece {
    pub range: Range<usize>,
    /// From the line's start.
    pub x: Pixels,
    pub shaped: ShapedLine,
    pub background: Option<Hsla>,
}

impl ParaLayout {
    /// The visual line showing a cursor at `offset`. A wrap point belongs
    /// to the lower line unless `upstream`.
    pub fn line_for(&self, offset: usize, upstream: bool) -> usize {
        let ix = self
            .lines
            .partition_point(|line| line.range.start <= offset)
            .saturating_sub(1);
        if upstream && ix > 0 && self.lines[ix].soft_start && self.lines[ix].range.start == offset {
            ix - 1
        } else {
            ix
        }
    }

    /// The x of `offset` on line `ix`, from the paragraph's left edge.
    pub fn x_for(&self, ix: usize, offset: usize) -> Pixels {
        let Some(line) = self.lines.get(ix) else {
            return px(0.0);
        };
        let offset = offset.clamp(line.range.start, line.range.end);
        for piece in &line.pieces {
            if offset <= piece.range.end {
                let local = offset.saturating_sub(piece.range.start);
                return line.x0 + piece.x + piece.shaped.x_for_index(local);
            }
        }
        line.x0
            + line
                .pieces
                .last()
                .map_or(px(0.0), |p| p.x + p.shaped.width())
    }

    /// The cursor's top-left and height, from the paragraph's origin.
    pub fn caret(&self, offset: usize, upstream: bool) -> (Point<Pixels>, Pixels) {
        let ix = self.line_for(offset, upstream);
        let Some(line) = self.lines.get(ix) else {
            return (point(px(0.0), px(0.0)), px(16.0));
        };
        (point(self.x_for(ix, offset), line.top), line.height)
    }

    /// The offset closest to `x` (from the left edge) on line `ix`.
    pub fn offset_in_line(&self, text: &str, ix: usize, x: Pixels) -> (usize, bool) {
        let Some(line) = self.lines.get(ix) else {
            return (0, false);
        };
        let x = x - line.x0;
        let mut offset = line.range.start;
        for piece in &line.pieces {
            if x < piece.x {
                break;
            }
            let local = piece.shaped.closest_index_for_x(x - piece.x);
            offset = piece.range.start + local;
            if x <= piece.x + piece.shaped.width() {
                break;
            }
        }
        // Past the text is a writing suggestion's line.
        let offset = floor_grapheme(
            text,
            offset
                .clamp(line.range.start, line.range.end)
                .min(text.len()),
        )
        .max(line.range.start.min(text.len()));
        (offset, line.soft_end && offset == line.range.end)
    }

    /// The offset under window position `p`.
    pub fn hit(&self, text: &str, p: Point<Pixels>) -> (usize, bool) {
        let local = p - self.bounds.origin;
        if self.lines.is_empty() {
            return (0, false);
        }
        let ix = if local.y < px(0.0) {
            0
        } else {
            self.lines
                .iter()
                .position(|line| local.y < line.top + line.height)
                .unwrap_or(self.lines.len() - 1)
        };
        self.offset_in_line(text, ix, local.x)
    }

    /// Rectangles covering `range`, from the paragraph's origin. A covered
    /// line end shows as a short stub.
    pub fn selection_rects(&self, range: Range<usize>, whole_end: bool) -> Vec<Bounds<Pixels>> {
        let mut rects = Vec::new();
        for (ix, line) in self.lines.iter().enumerate() {
            let start = range.start.max(line.range.start);
            let end = range.end.min(line.range.end);
            let last_line = ix + 1 == self.lines.len();
            let past_end = (range.end > line.range.end && !line.soft_end)
                || (whole_end && last_line && range.end >= line.range.end);
            if start > end || (start == end && !past_end) {
                continue;
            }
            let left = self.x_for(ix, start);
            let mut right = self.x_for(ix, end);
            if past_end {
                right += line.height * 0.3;
            }
            rects.push(Bounds::from_corners(
                point(left, line.top),
                point(right, line.top + line.height),
            ));
        }
        rects
    }
}

/// Lays out `para` in `width`. `decos` marks ranges (sorted, apart).
pub(crate) fn layout(
    window: &Window,
    para: &Para,
    base: &TextBase,
    width: Pixels,
    decos: &[(Range<usize>, Deco)],
) -> Vec<LineBox> {
    let text = para.text.as_str();
    let ratio = base.line_height / base.size;
    let text_system = window.text_system();
    let mut lines = Vec::new();
    let mut top = px(0.0);
    let mut hard_start = 0;
    for hard in text.split('\n') {
        let hard_range = hard_start..hard_start + hard.len();
        hard_start = hard_range.end + 1;
        // Stretches of one style and decoration.
        let segments = segments(para, decos, hard_range.clone());
        // x at each byte of the unwrapped line.
        let mut shaped = Vec::with_capacity(segments.len());
        let mut x = px(0.0);
        for (range, style, deco) in &segments {
            let (run, font_size) = base.run(style, range.len(), *deco);
            let line = text_system.shape_line(
                SharedString::from(text[range.clone()].to_owned()),
                font_size,
                &[run],
                None,
            );
            let start_x = x;
            x += line.width();
            shaped.push((range.clone(), start_x, line));
        }
        let x_at = |offset: usize| -> Pixels {
            for (range, start_x, line) in &shaped {
                if offset <= range.end {
                    return *start_x + line.x_for_index(offset.saturating_sub(range.start));
                }
            }
            x
        };
        let breaks = wrap(text, hard_range.clone(), width, &x_at);
        let mut start = hard_range.start;
        let count = breaks.len() + 1;
        for (n, end) in breaks
            .into_iter()
            .chain(std::iter::once(hard_range.end))
            .enumerate()
        {
            let range = start..end;
            let mut pieces = Vec::new();
            let (mut ascent, mut descent, mut height) = (px(0.0), px(0.0), px(0.0));
            let mut px_x = px(0.0);
            for (seg, style, deco) in &segments {
                let piece = seg.start.max(range.start)..seg.end.min(range.end);
                if piece.is_empty() {
                    continue;
                }
                let (run, font_size) = base.run(style, piece.len(), *deco);
                let line = text_system.shape_line(
                    SharedString::from(text[piece.clone()].to_owned()),
                    font_size,
                    &[run],
                    None,
                );
                ascent = ascent.max(line.ascent);
                descent = descent.max(line.descent);
                height = height.max(font_size * ratio);
                let width = line.width();
                let background = (!base.plain).then_some(style.background).flatten().map(rgb);
                pieces.push(Piece {
                    range: piece,
                    x: px_x,
                    shaped: line,
                    background,
                });
                px_x += width;
            }
            if pieces.is_empty() {
                let style = para.style_at(range.start);
                let (run, font_size) = base.run(&style, 1, Deco::default());
                let probe = text_system.shape_line(" ".into(), font_size, &[run], None);
                ascent = probe.ascent;
                descent = probe.descent;
                height = font_size * ratio;
            }
            let height = height.max(ascent + descent);
            let baseline = (height - (ascent + descent)) / 2.0 + ascent;
            // Spaces at the end of a wrapped line hang past the edge.
            let visible = text[range.clone()].trim_end().len() + range.start;
            let visible_width = pieces
                .iter()
                .find(|p| visible <= p.range.end)
                .map_or(px_x, |p| {
                    p.x + p.shaped.x_for_index(visible.saturating_sub(p.range.start))
                });
            let x0 = match para.style.align {
                Align::Left => px(0.0),
                Align::Center => ((width - visible_width) / 2.0).max(px(0.0)),
                Align::Right => (width - visible_width).max(px(0.0)),
            };
            lines.push(LineBox {
                range: range.clone(),
                top,
                height,
                baseline,
                x0,
                pieces,
                soft_start: n > 0,
                soft_end: n + 1 < count,
            });
            top += height;
            start = end;
        }
    }
    lines
}

/// Where a line of `text[range]` breaks to fit `width`, given the x of each
/// offset: before words that would pass the edge, and inside a word longer
/// than a whole line.
fn wrap(
    text: &str,
    range: Range<usize>,
    width: Pixels,
    x_at: &dyn Fn(usize) -> Pixels,
) -> Vec<usize> {
    let mut breaks = Vec::new();
    if width <= px(0.0) {
        return breaks;
    }
    let mut line_start = range.start;
    let mut x0 = x_at(line_start);
    for (ix, word) in text[range.clone()].split_word_bound_indices() {
        let (a, b) = (range.start + ix, range.start + ix + word.len());
        if word.chars().all(char::is_whitespace) {
            continue;
        }
        if x_at(b) - x0 <= width {
            continue;
        }
        if a > line_start {
            breaks.push(a);
            line_start = a;
            x0 = x_at(a);
            if x_at(b) - x0 <= width {
                continue;
            }
        }
        // A word wider than the line breaks between its graphemes. The
        // offsets count from where the word's first line starts, which stays
        // put while `line_start` moves on at each break.
        let from = line_start;
        let mut last = line_start;
        for (gx, g) in text[from..b].grapheme_indices(true) {
            let end = from + gx + g.len();
            if x_at(end) - x0 > width && last > line_start {
                breaks.push(last);
                line_start = last;
                x0 = x_at(last);
            }
            last = end;
        }
    }
    breaks
}

/// The stretches of `range` in one style and decoration.
fn segments(
    para: &Para,
    decos: &[(Range<usize>, Deco)],
    range: Range<usize>,
) -> Vec<(Range<usize>, CharStyle, Deco)> {
    let mut cuts = vec![range.start, range.end];
    for (span, _) in para.spans() {
        cuts.extend([span.start, span.end]);
    }
    for (span, _) in decos {
        cuts.extend([span.start, span.end]);
    }
    cuts.retain(|c| range.contains(c) || *c == range.end);
    cuts.sort_unstable();
    cuts.dedup();
    cuts.windows(2)
        .filter(|w| w[0] < w[1])
        .map(|w| {
            let piece = w[0]..w[1];
            let style = para
                .spans()
                .find(|(span, _)| span.start <= piece.start && piece.start < span.end)
                .map(|(_, s)| s.clone())
                .unwrap_or_default();
            let deco = decos
                .iter()
                .find(|(span, _)| span.start <= piece.start && piece.start < span.end)
                .map(|(_, d)| *d)
                .unwrap_or_default();
            (piece, style, deco)
        })
        .collect()
}

/// The lines of a paragraph at a width, kept between frames.
type LayoutCache = Rc<RefCell<Option<(Pixels, Vec<LineBox>)>>>;

/// Draws the paragraph at `path` of `editor`.
pub(crate) struct ParaElement {
    pub editor: Entity<RichEditor>,
    pub path: Path,
    /// Shown when the whole editor is empty.
    pub placeholder: Option<SharedString>,
}

pub(crate) struct Measured {
    para: Para,
    base: TextBase,
    decos: Vec<(Range<usize>, Deco)>,
    /// The layout made while measuring, by width.
    cache: LayoutCache,
}

pub(crate) struct Prepainted {
    lines: Option<Vec<LineBox>>,
}

impl IntoElement for ParaElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for ParaElement {
    type RequestLayoutState = Measured;
    type PrepaintState = Prepainted;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let editor = self.editor.read(cx);
        let base = editor.text_base(window);
        let mut para = editor.doc.para(self.path).cloned().unwrap_or_default();
        let mut decos = editor.decorations(self.path, &para);
        // A writing suggestion lays out as text after the cursor, so it
        // wraps like the text it would become.
        if editor.focus_handle.is_focused(window)
            && let Some((offset, ghost, style)) = editor.ghost_in(self.path)
            && offset == para.len()
        {
            para.insert(offset, ghost, &style);
            decos.push((
                offset..para.len(),
                Deco {
                    ghost: true,
                    ..Deco::default()
                },
            ));
        }
        let mut base = base;
        if para.is_empty()
            && let Some(placeholder) = &self.placeholder
        {
            para = Para::plain(placeholder.to_string()).with_style(para.style);
            base.color = base.color.opacity(0.5);
        }
        let cache = Rc::new(RefCell::new(None));
        let measure = Measured {
            para: para.clone(),
            base: base.clone(),
            decos: decos.clone(),
            cache: cache.clone(),
        };
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        let layout_id =
            window.request_measured_layout(style, move |known, available, window, _cx| {
                let width = known.width.or(match available.width {
                    AvailableSpace::Definite(width) => Some(width),
                    _ => None,
                });
                let wrap_width = width.unwrap_or(px(f32::MAX / 4.0));
                let lines = layout(
                    window,
                    &measure.para,
                    &measure.base,
                    wrap_width,
                    &measure.decos,
                );
                let height = lines.last().map_or(base.line_height, |l| l.top + l.height);
                let natural = lines
                    .iter()
                    .map(|l| l.pieces.last().map_or(px(0.0), |p| p.x + p.shaped.width()))
                    .fold(px(0.0), Pixels::max);
                *measure.cache.borrow_mut() = Some((wrap_width, lines));
                let width = match (width, available.width) {
                    (Some(width), _) => width,
                    (None, AvailableSpace::MinContent) => px(0.0),
                    (None, _) => natural,
                };
                size(width, height)
            });
        (
            layout_id,
            Measured {
                para,
                base,
                decos,
                cache,
            },
        )
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        measured: &mut Self::RequestLayoutState,
        window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
        let cached = measured.cache.borrow_mut().take();
        let lines = match cached {
            Some((width, lines)) if width == bounds.size.width => lines,
            _ => layout(
                window,
                &measured.para,
                &measured.base,
                bounds.size.width,
                &measured.decos,
            ),
        };
        Prepainted { lines: Some(lines) }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        measured: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(lines) = prepaint.lines.take() else {
            return;
        };
        let layout = ParaLayout { bounds, lines };
        let dragging = cx.has_active_drag();
        let editor = self.editor.read(cx);
        let placeholder =
            editor.doc.para(self.path).is_some_and(Para::is_empty) && self.placeholder.is_some();
        let focused = editor.focus_handle.is_focused(window);
        editor.drawn_focused.set(focused);
        let selection = editor.selection_in(self.path);
        let cursor = editor.cursor_in(self.path);
        // While a picture is dragged over the text, the caret shows where
        // it will land instead of where the typing is.
        let caret = match editor.drop_caret {
            Some((at, upstream)) if dragging => {
                (at.path == self.path).then_some((at.offset, upstream))
            }
            _ => cursor.filter(|_| focused),
        };
        let marker = editor.marker(self.path).map(|marker| {
            let style = editor
                .doc
                .para(self.path)
                .map(|p| p.style_at(0))
                .unwrap_or_default();
            (marker, style)
        });
        let base = &measured.base;
        let origin = bounds.origin;
        // Highlights, then the selection, then the text.
        for line in &layout.lines {
            for piece in &line.pieces {
                if let Some(color) = piece.background {
                    window.paint_quad(fill(
                        Bounds::new(
                            origin + point(line.x0 + piece.x, line.top),
                            size(piece.shaped.width(), line.height),
                        ),
                        color,
                    ));
                }
            }
        }
        // Text that just went in for the selection, fading.
        if let Some((range, left)) = editor.flash_in(self.path) {
            let color = editor.palette_now().inserted;
            let color = color.opacity(left);
            for rect in layout.selection_rects(range, false) {
                window.paint_quad(fill(rect + origin, color));
            }
            window.request_animation_frame();
        }
        if !placeholder && let Some((range, whole_end)) = selection {
            let color = if focused {
                base.accent.opacity(0.3)
            } else {
                base.accent.opacity(0.15)
            };
            for rect in layout.selection_rects(range, whole_end) {
                window.paint_quad(fill(rect + origin, color));
            }
        }
        // A longer suggestion ends in a small "✦ Tab" key, when it fits.
        let tab_key = (focused && editor.ghost_is_long())
            .then(|| editor.ghost_in(self.path))
            .flatten()
            .map(|(offset, ghost, _)| (offset + ghost.len(), editor.palette_now().accent));
        for line in &layout.lines {
            for piece in &line.pieces {
                let ascent = piece.shaped.ascent;
                let descent = piece.shaped.descent;
                let at = origin + point(line.x0 + piece.x, line.top + line.baseline - ascent);
                // A failed paint only loses this frame's text.
                let _ = piece
                    .shaped
                    .paint(at, ascent + descent, TextAlign::Left, None, window, cx);
            }
        }
        if let Some((end, accent)) = tab_key {
            paint_tab_key(&layout, end, base, accent, window, cx);
        }
        // The list marker sits in the margin, on the first line's baseline.
        if let Some((marker, style)) = marker
            && let Some(first) = layout.lines.first()
        {
            let (run, font_size) = base.run(
                &CharStyle {
                    link: None,
                    underline: false,
                    strike: false,
                    background: None,
                    ..style
                },
                marker.len(),
                Deco::default(),
            );
            let shaped = window
                .text_system()
                .shape_line(marker.into(), font_size, &[run], None);
            let x = origin.x + first.x0 - shaped.width() - px(6.0);
            let at = point(x, origin.y + first.top + first.baseline - shaped.ascent);
            let _ = shaped.paint(
                at,
                shaped.ascent + shaped.descent,
                TextAlign::Left,
                None,
                window,
                cx,
            );
        }
        if let Some((offset, upstream)) = caret {
            let offset = if placeholder { 0 } else { offset };
            let (at, height) = layout.caret(offset, upstream);
            window.paint_quad(fill(
                Bounds::new(origin + at, size(px(2.0), height)),
                base.accent,
            ));
        }
        let path = self.path;
        self.editor.update(cx, |editor, _| {
            editor.layouts.insert(path, layout);
        });
    }
}

/// The key after a longer suggestion ending at `end`: "✦ Tab" in a
/// faint rounded outline, on the suggestion's last line, left out when
/// the line has no room for it.
fn paint_tab_key(
    layout: &ParaLayout,
    end: usize,
    base: &TextBase,
    accent: Hsla,
    window: &mut Window,
    cx: &mut App,
) {
    let (at, height) = layout.caret(end, false);
    let font_size = base.size * 0.78;
    let label = "\u{2726} Tab";
    let spark = "\u{2726} ".len();
    let run = |len: usize, color: Hsla| TextRun {
        len,
        font: base.font.clone(),
        color,
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    let runs = [
        run(spark, accent),
        run(label.len() - spark, base.color.opacity(0.55)),
    ];
    let shaped = window
        .text_system()
        .shape_line(label.into(), font_size, &runs, None);
    let (pad, gap) = (px(5.0), px(8.0));
    let key = size(shaped.width() + pad * 2.0, font_size * 1.45);
    let origin = layout.bounds.origin + point(at.x + gap, at.y + (height - key.height) / 2.0);
    if origin.x + key.width > layout.bounds.origin.x + layout.bounds.size.width {
        return;
    }
    window.paint_quad(quad(
        Bounds::new(origin, key),
        px(5.0),
        gpui::transparent_black(),
        px(1.0),
        base.color.opacity(0.18),
        BorderStyle::Solid,
    ));
    let text_at = point(
        origin.x + pad,
        origin.y + (key.height - (shaped.ascent + shaped.descent)) / 2.0,
    );
    let _ = shaped.paint(
        text_at,
        shaped.ascent + shaped.descent,
        TextAlign::Left,
        None,
        window,
        cx,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Breaks of `text` at `width` with every byte 1 px wide.
    fn breaks(text: &str, width: f32) -> Vec<usize> {
        wrap(text, 0..text.len(), px(width), &|offset| px(offset as f32))
    }

    #[test]
    fn breaks_a_long_word_many_times_inside_the_text() {
        // A word several lines long, as a quoted link or hash in a reply.
        let text = format!("see {} ok", "a".repeat(60));
        assert_eq!(breaks(&text, 10.0), vec![4, 14, 24, 34, 44, 54, 65]);
    }

    #[test]
    fn breaks_a_long_word_of_wide_characters_on_their_boundaries() {
        let text = "日本語".repeat(20);
        let breaks = breaks(&text, 7.0);
        assert!(breaks.len() > 3, "{breaks:?}");
        assert!(breaks.windows(2).all(|w| w[0] < w[1]), "{breaks:?}");
        assert!(
            breaks
                .iter()
                .all(|&b| b < text.len() && text.is_char_boundary(b))
        );
    }

    #[test]
    fn breaks_before_words_that_pass_the_edge() {
        assert_eq!(breaks("aaa bbb ccc", 8.0), vec![8]);
        assert_eq!(breaks("aaa bbb ccc", 5.0), vec![4, 8]);
        assert_eq!(breaks("aaa bbb", 100.0), Vec::<usize>::new());
    }
}
