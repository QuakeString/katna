// SPDX-License-Identifier: GPL-3.0-or-later

//! A PDF page's text as lines, with where each character is drawn, so the
//! viewer can select and copy it. hayro reads the page as it does for
//! drawing; every glyph with a known character becomes a letter, and
//! letters on one baseline, left to right, become a line.

use hayro::hayro_interpret::font::Glyph;
use hayro::hayro_interpret::hayro_cmap::BfString;
use hayro::hayro_interpret::{
    BlendMode, ClipPath, Context, Device, GlyphDrawMode, Image, InterpreterCache,
    InterpreterSettings, Paint, PathDrawMode, SoftMask, interpret_page,
};
use hayro::hayro_syntax::page::Page;
use hayro::vello_cpu::kurbo::{Affine, BezPath, Point, Rect};

/// A line of a page's text, in points from the page's top left (rotation
/// applied, as drawn).
#[derive(Debug, Clone, PartialEq)]
pub struct TextLine {
    pub text: String,
    /// Each character: where it starts in `text`, and its left and right
    /// edges.
    pub chars: Vec<(usize, f32, f32)>,
    pub top: f32,
    pub bottom: f32,
}

impl TextLine {
    pub fn left(&self) -> f32 {
        self.chars.first().map_or(0.0, |c| c.1)
    }

    pub fn right(&self) -> f32 {
        self.chars.last().map_or(0.0, |c| c.2)
    }
}

/// A page's lines of text, in the order the page draws them.
/// `transform` goes from the page's own coordinates to points as shown,
/// `(w, h)` the page's size as shown.
pub(crate) fn lines(page: &Page<'_>, transform: Affine, (w, h): (f32, f32)) -> Vec<TextLine> {
    let cache = InterpreterCache::new();
    let mut context = Context::new(
        transform,
        Rect::new(0.0, 0.0, f64::from(w), f64::from(h)),
        &cache,
        page.xref(),
        InterpreterSettings::default(),
    );
    let mut letters = Letters::default();
    interpret_page(page, &mut context, &mut letters);
    join(letters.0)
}

/// Where page `page` draws pictures (not masks drawn in one colour), as
/// rectangles in points as shown: left, top, right, bottom.
pub(crate) fn pictures(page: &Page<'_>, transform: Affine, (w, h): (f32, f32)) -> Vec<Rect> {
    let cache = InterpreterCache::new();
    let mut context = Context::new(
        transform,
        Rect::new(0.0, 0.0, f64::from(w), f64::from(h)),
        &cache,
        page.xref(),
        InterpreterSettings::default(),
    );
    let mut pictures = Pictures::default();
    interpret_page(page, &mut context, &mut pictures);
    pictures.0
}

#[derive(Default)]
struct Pictures(Vec<Rect>);

impl<'a> Device<'a> for Pictures {
    fn set_soft_mask(&mut self, _: Option<SoftMask<'a>>) {}
    fn set_blend_mode(&mut self, _: BlendMode) {}
    fn draw_path(&mut self, _: &BezPath, _: Affine, _: &Paint<'a>, _: &PathDrawMode) {}
    fn push_clip_path(&mut self, _: &ClipPath) {}
    fn push_transparency_group(&mut self, _: f32, _: Option<SoftMask<'a>>, _: BlendMode) {}
    fn pop_clip_path(&mut self) {}
    fn pop_transparency_group(&mut self) {}
    fn draw_glyph(
        &mut self,
        _: &Glyph<'a>,
        _: Affine,
        _: Affine,
        _: &Paint<'a>,
        _: &GlyphDrawMode,
    ) {
    }

    fn draw_image(&mut self, image: Image<'a, '_>, transform: Affine) {
        if !matches!(image, Image::Raster(_)) {
            return;
        }
        // The picture fills (0, 0) to (width, height) under `transform`.
        let (iw, ih) = (f64::from(image.width()), f64::from(image.height()));
        let corners =
            [(0.0, 0.0), (iw, 0.0), (0.0, ih), (iw, ih)].map(|(x, y)| transform * Point::new(x, y));
        let rect = corners
            .iter()
            .skip(1)
            .fold(Rect::from_points(corners[0], corners[0]), |r, p| {
                r.union_pt(*p)
            });
        if rect.is_finite() && rect.area() > 0.0 {
            self.0.push(rect);
        }
    }
}

/// A glyph drawn with a known character.
#[derive(Debug, Clone, PartialEq)]
struct Letter {
    text: String,
    x0: f32,
    x1: f32,
    /// The baseline, from the page's top.
    base: f32,
    /// The font size as drawn.
    size: f32,
}

#[derive(Default)]
struct Letters(Vec<Letter>);

impl<'a> Device<'a> for Letters {
    fn set_soft_mask(&mut self, _: Option<SoftMask<'a>>) {}
    fn set_blend_mode(&mut self, _: BlendMode) {}
    fn draw_path(&mut self, _: &BezPath, _: Affine, _: &Paint<'a>, _: &PathDrawMode) {}
    fn push_clip_path(&mut self, _: &ClipPath) {}
    fn push_transparency_group(&mut self, _: f32, _: Option<SoftMask<'a>>, _: BlendMode) {}
    fn draw_image(&mut self, _: Image<'a, '_>, _: Affine) {}
    fn pop_clip_path(&mut self) {}
    fn pop_transparency_group(&mut self) {}

    fn draw_glyph(
        &mut self,
        glyph: &Glyph<'a>,
        transform: Affine,
        glyph_transform: Affine,
        _: &Paint<'a>,
        _: &GlyphDrawMode,
    ) {
        let text = match glyph.as_unicode() {
            Some(BfString::Char(c)) => c.to_string(),
            Some(BfString::String(s)) => s,
            None => return,
        };
        let text: String = text.chars().filter(|c| !c.is_control()).collect();
        if text.is_empty() {
            return;
        }
        // Glyph space is 1000 units to the em.
        let advance = match glyph {
            Glyph::Outline(g) => g.advance_width().filter(|w| *w > 0.0),
            Glyph::Type3(_) => None,
        }
        .unwrap_or(500.0);
        let m = transform * glyph_transform;
        let origin = m * Point::ZERO;
        let end = m * Point::new(f64::from(advance), 0.0);
        let up = m * Point::new(0.0, 1000.0) - origin;
        let size = up.hypot() as f32;
        // Only text along the page is laid out in lines; rotated text is
        // taken as it comes.
        let (x0, x1) = (origin.x.min(end.x) as f32, origin.x.max(end.x) as f32);
        let letter = Letter {
            text,
            x0,
            x1,
            base: origin.y as f32,
            size,
        };
        // Filled and stroked text is drawn twice.
        if self.0.last() != Some(&letter) && size > 0.0 && size.is_finite() {
            self.0.push(letter);
        }
    }
}

/// Letters into lines: a letter starts a new line when it leaves the
/// baseline or goes back to the left; a gap wider than a fifth of the
/// size between letters is a space.
fn join(letters: Vec<Letter>) -> Vec<TextLine> {
    let mut lines: Vec<TextLine> = Vec::new();
    let mut last: Option<Letter> = None;
    for letter in letters {
        let joins = last.as_ref().is_some_and(|l| {
            let size = l.size.max(letter.size);
            (l.base - letter.base).abs() < size * 0.35 && letter.x0 > l.x1 - size * 0.5
        });
        let size = letter.size;
        let (top, bottom) = (letter.base - size * 0.8, letter.base + size * 0.2);
        match lines.last_mut() {
            Some(line) if joins => {
                let previous = last.as_ref().expect("a joined letter follows one");
                let gap = letter.x0 - previous.x1;
                if gap > size.max(previous.size) * 0.2
                    && !line.text.ends_with(' ')
                    && !letter.text.starts_with(' ')
                {
                    line.chars.push((line.text.len(), previous.x1, letter.x0));
                    line.text.push(' ');
                }
                push(line, &letter);
                line.top = line.top.min(top);
                line.bottom = line.bottom.max(bottom);
            }
            _ => {
                let mut line = TextLine {
                    text: String::new(),
                    chars: Vec::new(),
                    top,
                    bottom,
                };
                push(&mut line, &letter);
                lines.push(line);
            }
        }
        last = Some(letter);
    }
    for line in &mut lines {
        let trimmed = line.text.trim_end().len();
        line.text.truncate(trimmed);
        line.chars.retain(|c| c.0 < trimmed);
    }
    lines.retain(|line| !line.text.trim().is_empty());
    lines
}

/// Adds `letter` to `line`; a ligature's characters share its width.
fn push(line: &mut TextLine, letter: &Letter) {
    let count = letter.text.chars().count().max(1) as f32;
    let step = (letter.x1 - letter.x0) / count;
    for (ix, (offset, _)) in letter.text.char_indices().enumerate() {
        let x = letter.x0 + step * ix as f32;
        line.chars.push((line.text.len() + offset, x, x + step));
    }
    line.text.push_str(&letter.text);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn letter(text: &str, x0: f32, base: f32) -> Letter {
        Letter {
            text: text.into(),
            x0,
            x1: x0 + 6.0,
            base,
            size: 12.0,
        }
    }

    #[test]
    fn letters_make_lines_and_spaces() {
        let lines = join(vec![
            letter("H", 72.0, 100.0),
            letter("i", 78.0, 100.0),
            // A gap: a space.
            letter("y", 90.0, 100.0),
            letter("o", 96.0, 100.0),
            // The next line.
            letter("A", 72.0, 116.0),
            letter("ﬁ", 78.0, 116.0),
        ]);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].text, "Hi yo");
        assert_eq!(lines[0].chars[2], (2, 84.0, 90.0));
        assert_eq!(lines[0].left(), 72.0);
        assert_eq!(lines[0].right(), 102.0);
        assert_eq!(lines[1].text, "Aﬁ");
        assert!(lines[0].bottom <= lines[1].top + 0.01);
        // The same glyph drawn twice (filled, then stroked) counts once:
        // done by the device, not here.
    }

    #[test]
    fn going_back_left_starts_a_line() {
        // A second column's line on the same baseline, drawn after.
        let lines = join(vec![letter("a", 300.0, 100.0), letter("b", 72.0, 100.0)]);
        assert_eq!(lines.len(), 2);
    }
}
