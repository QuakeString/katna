// SPDX-License-Identifier: GPL-3.0-or-later

//! Marks made on a PDF in the viewer: highlights, underlines, squiggles,
//! strike-throughs, pen strokes, sticky notes and text boxes, with undo
//! and redo. They are kept in
//! points from the page's top left as the page is drawn (rotation
//! applied, like [`crate::pdf::TextLine`]); [`crate::pdf::Document::with_marks`]
//! writes them into a copy of the file as standard PDF annotations.

/// What a mark is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    Highlight,
    Underline,
    Squiggly,
    StrikeOut,
    /// A pen stroke.
    Ink,
    /// A sticky note: an icon on the page with text that opens from it.
    Note,
    /// Text written on the page.
    FreeText,
}

impl Kind {
    /// Marks laid on text, made by selecting it.
    pub fn on_text(self) -> bool {
        matches!(
            self,
            Kind::Highlight | Kind::Underline | Kind::Squiggly | Kind::StrikeOut
        )
    }

    /// Marks placed with a click and then typed.
    pub fn typed(self) -> bool {
        matches!(self, Kind::Note | Kind::FreeText)
    }
}

/// A box around part of a line of text, in points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quad {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Quad {
    /// A wavy line along the bottom, for a squiggle.
    pub fn squiggle(&self) -> Vec<(f32, f32)> {
        let h = (self.bottom - self.top).max(1.0);
        let step = (h * 0.15).max(1.0);
        let (low, high) = (self.bottom - h * 0.02, self.bottom - h * 0.14);
        let mut points = Vec::new();
        let mut x = self.left;
        let mut up = false;
        while x < self.right {
            points.push((x, if up { high } else { low }));
            up = !up;
            x += step;
        }
        points.push((self.right, if up { high } else { low }));
        points
    }

    fn contains(&self, x: f32, y: f32, slop: f32) -> bool {
        x >= self.left - slop
            && x <= self.right + slop
            && y >= self.top - slop
            && y <= self.bottom + slop
    }
}

/// A red, green and blue colour, each 0 to 1.
pub type Color = [f32; 3];

/// Pen strokes are this wide, in points.
pub const PEN_WIDTH: f32 = 2.0;

/// A sticky note's icon is this many points square.
pub const NOTE_SIZE: f32 = 20.0;

/// Text boxes' text size and width, in points.
pub const TEXT_SIZE: f32 = 12.0;
pub const TEXT_WIDTH: f32 = 220.0;

/// Lines of a text box are this many times the text size apart.
pub const LINE_HEIGHT: f32 = 1.25;

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    /// The boxes of the marked text, one per line, and the text itself.
    Text { quads: Vec<Quad>, text: String },
    /// A stroke's points, in the order drawn.
    Ink(Vec<(f32, f32)>),
    /// A sticky note's icon at its top left, and its text.
    Note { at: (f32, f32), text: String },
    /// A text box: its top left, width, text size and text.
    Box {
        at: (f32, f32),
        width: f32,
        size: f32,
        text: String,
    },
}

impl Shape {
    /// The text of a note or a text box.
    pub fn typed_text(&self) -> Option<&str> {
        match self {
            Shape::Note { text, .. } | Shape::Box { text, .. } => Some(text),
            _ => None,
        }
    }

    /// A note's or a text box's box: left, top, right, bottom.
    pub fn frame(&self) -> Option<Quad> {
        match self {
            Shape::Note { at, .. } => Some(Quad {
                left: at.0,
                top: at.1,
                right: at.0 + NOTE_SIZE,
                bottom: at.1 + NOTE_SIZE,
            }),
            Shape::Box {
                at,
                width,
                size,
                text,
            } => Some(Quad {
                left: at.0,
                top: at.1,
                right: at.0 + width,
                bottom: at.1 + box_height(text, *width, *size),
            }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mark {
    /// The page, from 0.
    pub page: usize,
    pub kind: Kind,
    pub color: Color,
    pub shape: Shape,
}

impl Mark {
    /// This mark on its page turned a quarter turn, clockwise or not;
    /// `(w, h)` is the page's size before the turn. Notes and text boxes
    /// keep their corner and stay upright.
    pub fn turned(&self, clockwise: bool, (w, h): (f32, f32)) -> Mark {
        let point = |(x, y): (f32, f32)| {
            if clockwise { (h - y, x) } else { (y, w - x) }
        };
        let quad = |q: &Quad| {
            let (a, b) = (point((q.left, q.top)), point((q.right, q.bottom)));
            Quad {
                left: a.0.min(b.0),
                top: a.1.min(b.1),
                right: a.0.max(b.0),
                bottom: a.1.max(b.1),
            }
        };
        let corner = |shape: &Shape| {
            shape.frame().map_or((0.0, 0.0), |q| {
                let q = quad(&q);
                (q.left, q.top)
            })
        };
        let shape = match &self.shape {
            Shape::Text { quads, text } => Shape::Text {
                quads: quads.iter().map(quad).collect(),
                text: text.clone(),
            },
            Shape::Ink(points) => Shape::Ink(points.iter().copied().map(point).collect()),
            Shape::Note { text, .. } => Shape::Note {
                at: corner(&self.shape),
                text: text.clone(),
            },
            Shape::Box {
                width, size, text, ..
            } => Shape::Box {
                at: corner(&self.shape),
                width: *width,
                size: *size,
                text: text.clone(),
            },
        };
        Mark {
            shape,
            ..self.clone()
        }
    }

    /// Whether the point `(x, y)` on `page` is on this mark, give or take
    /// `slop` points.
    pub fn hit(&self, page: usize, x: f32, y: f32, slop: f32) -> bool {
        if page != self.page {
            return false;
        }
        match &self.shape {
            Shape::Text { quads, .. } => quads.iter().any(|q| q.contains(x, y, slop)),
            Shape::Note { .. } | Shape::Box { .. } => {
                self.shape.frame().is_some_and(|q| q.contains(x, y, slop))
            }
            Shape::Ink(points) => {
                let near = PEN_WIDTH / 2.0 + slop;
                match points.as_slice() {
                    [] => false,
                    [(px, py)] => (px - x).hypot(py - y) <= near,
                    _ => points
                        .windows(2)
                        .any(|w| distance_to_segment((x, y), w[0], w[1]) <= near),
                }
            }
        }
    }
}

/// Helvetica's advance widths, in thousandths of the text size, for the
/// printable ASCII characters from the space on. PDF readers all have
/// Helvetica, so a text box is laid out and drawn with it.
const HELVETICA: [u16; 95] = [
    278, 278, 355, 556, 556, 889, 667, 191, 333, 333, 389, 584, 278, 333, 278, 278, 556, 556, 556,
    556, 556, 556, 556, 556, 556, 556, 278, 278, 584, 584, 584, 556, 1015, 667, 667, 722, 722, 667,
    611, 778, 722, 278, 500, 667, 556, 833, 722, 778, 667, 778, 722, 667, 611, 722, 667, 944, 667,
    667, 611, 278, 278, 278, 469, 556, 333, 556, 556, 500, 556, 556, 278, 556, 556, 222, 222, 500,
    222, 833, 556, 556, 556, 556, 333, 500, 278, 556, 500, 722, 500, 500, 500, 334, 260, 334, 584,
];

/// How wide `text` is in Helvetica at `size` points.
pub fn text_width(text: &str, size: f32) -> f32 {
    let thousandths: u32 = text
        .chars()
        .map(|c| match c as u32 {
            code @ 32..=126 => u32::from(HELVETICA[(code - 32) as usize]),
            _ => 556,
        })
        .sum();
    thousandths as f32 * size / 1000.0
}

/// `text` broken into lines no wider than `width` at `size` points: at
/// its own line breaks, then between words, then inside words too long
/// for a line.
pub fn wrap(text: &str, width: f32, size: f32) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        let mut line = String::new();
        for word in paragraph.split(' ') {
            let joined = if line.is_empty() {
                word.to_owned()
            } else {
                format!("{line} {word}")
            };
            if text_width(&joined, size) <= width || line.is_empty() && word.is_empty() {
                line = joined;
                continue;
            }
            if !line.is_empty() {
                lines.push(std::mem::take(&mut line));
            }
            for c in word.chars() {
                line.push(c);
                if text_width(&line, size) > width && line.chars().count() > 1 {
                    line.pop();
                    lines.push(std::mem::take(&mut line));
                    line.push(c);
                }
            }
        }
        lines.push(line);
    }
    lines
}

/// How tall a text box `width` wide is, in points.
pub fn box_height(text: &str, width: f32, size: f32) -> f32 {
    wrap(text, width, size).len().max(1) as f32 * size * LINE_HEIGHT
}

fn distance_to_segment(p: (f32, f32), a: (f32, f32), b: (f32, f32)) -> f32 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let length = dx * dx + dy * dy;
    let t = if length == 0.0 {
        0.0
    } else {
        (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / length).clamp(0.0, 1.0)
    };
    (p.0 - (a.0 + t * dx)).hypot(p.1 - (a.1 + t * dy))
}

/// A change that can be undone.
#[derive(Debug, Clone)]
enum Step {
    Added(usize, Mark),
    Removed(usize, Mark),
    /// A mark changed: before and after.
    Changed(usize, Mark, Mark),
}

/// The marks on one PDF, in the order made, with undo and redo.
#[derive(Debug, Default)]
pub struct Marks {
    list: Vec<Mark>,
    done: Vec<Step>,
    undone: Vec<Step>,
    /// Counts changes, undo and redo included.
    changes: u64,
    /// `changes` when the marks were last saved.
    saved: u64,
}

impl Marks {
    pub fn list(&self) -> &[Mark] {
        &self.list
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn add(&mut self, mark: Mark) {
        self.list.push(mark.clone());
        self.done.push(Step::Added(self.list.len() - 1, mark));
        self.undone.clear();
        self.changes += 1;
    }

    /// Removes the mark `ix` (of [`Marks::list`]).
    pub fn remove(&mut self, ix: usize) {
        if ix < self.list.len() {
            let mark = self.list.remove(ix);
            self.done.push(Step::Removed(ix, mark));
            self.undone.clear();
            self.changes += 1;
        }
    }

    /// Puts `mark` in the place of mark `ix` (a note's text changed).
    pub fn replace(&mut self, ix: usize, mark: Mark) {
        if let Some(old) = self.list.get_mut(ix)
            && *old != mark
        {
            let before = std::mem::replace(old, mark.clone());
            self.done.push(Step::Changed(ix, before, mark));
            self.undone.clear();
            self.changes += 1;
        }
    }

    /// Turns every mark, and every step of undo and redo, with its page
    /// a quarter turn; `size` gives a page's size before the turn.
    pub fn turn(&mut self, clockwise: bool, size: impl Fn(usize) -> (f32, f32)) {
        let turn = |mark: &mut Mark| *mark = mark.turned(clockwise, size(mark.page));
        self.list.iter_mut().for_each(turn);
        for step in self.done.iter_mut().chain(self.undone.iter_mut()) {
            match step {
                Step::Added(_, mark) | Step::Removed(_, mark) => turn(mark),
                Step::Changed(_, before, after) => {
                    turn(before);
                    turn(after);
                }
            }
        }
    }

    /// The topmost mark at `(x, y)` on `page`.
    pub fn at(&self, page: usize, x: f32, y: f32, slop: f32) -> Option<usize> {
        self.list.iter().rposition(|m| m.hit(page, x, y, slop))
    }

    pub fn can_undo(&self) -> bool {
        !self.done.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.undone.is_empty()
    }

    pub fn undo(&mut self) -> bool {
        let Some(step) = self.done.pop() else {
            return false;
        };
        self.apply(&step, true);
        self.undone.push(step);
        true
    }

    pub fn redo(&mut self) -> bool {
        let Some(step) = self.undone.pop() else {
            return false;
        };
        self.apply(&step, false);
        self.done.push(step);
        true
    }

    fn apply(&mut self, step: &Step, undo: bool) {
        match (step, undo) {
            (Step::Added(ix, _), true) | (Step::Removed(ix, _), false) => {
                if *ix < self.list.len() {
                    self.list.remove(*ix);
                }
            }
            (Step::Added(ix, mark), false) | (Step::Removed(ix, mark), true) => {
                self.list.insert((*ix).min(self.list.len()), mark.clone());
            }
            (Step::Changed(ix, before, after), undo) => {
                if let Some(mark) = self.list.get_mut(*ix) {
                    *mark = if undo { before } else { after }.clone();
                }
            }
        }
        self.changes += 1;
    }

    /// Whether there are changes since the last [`Marks::saved`] (or ever).
    pub fn unsaved(&self) -> bool {
        self.changes != self.saved && !(self.list.is_empty() && self.saved == 0)
    }

    /// The marks as they are now were saved.
    pub fn saved(&mut self) {
        self.saved = self.changes;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pen(page: usize, points: Vec<(f32, f32)>) -> Mark {
        Mark {
            page,
            kind: Kind::Ink,
            color: [1.0, 0.0, 0.0],
            shape: Shape::Ink(points),
        }
    }

    #[test]
    fn undo_and_redo_adding_and_removing() {
        let mut marks = Marks::default();
        marks.add(pen(0, vec![(0.0, 0.0), (10.0, 0.0)]));
        marks.add(pen(1, vec![(0.0, 0.0), (0.0, 10.0)]));
        marks.remove(0);
        assert_eq!(marks.list().len(), 1);
        assert!(marks.undo());
        assert_eq!(marks.list()[0].page, 0);
        assert!(marks.undo());
        assert_eq!(marks.list().len(), 1);
        assert!(marks.redo());
        assert!(marks.redo());
        assert_eq!(marks.list().len(), 1);
        assert_eq!(marks.list()[0].page, 1);
        assert!(!marks.redo());
    }

    #[test]
    fn turning_there_and_back_keeps_marks() {
        let mut marks = Marks::default();
        let stroke = Mark {
            page: 0,
            kind: Kind::Ink,
            color: [1.0, 0.0, 0.0],
            shape: Shape::Ink(vec![(10.0, 20.0), (30.0, 25.0)]),
        };
        let note = Mark {
            page: 0,
            kind: Kind::Note,
            color: [0.0, 1.0, 0.0],
            shape: Shape::Note {
                at: (100.0, 200.0),
                text: "Hi".into(),
            },
        };
        marks.add(stroke.clone());
        marks.add(note.clone());
        marks.turn(true, |_| (600.0, 800.0));
        // Clockwise, the page's left edge becomes its top.
        let Shape::Ink(points) = &marks.list()[0].shape else {
            panic!("a stroke");
        };
        assert_eq!(points[0], (780.0, 10.0));
        let Shape::Note { at, .. } = &marks.list()[1].shape else {
            panic!("a note");
        };
        assert_eq!(*at, (800.0 - 200.0 - NOTE_SIZE, 100.0));
        marks.turn(false, |_| (800.0, 600.0));
        assert_eq!(marks.list(), &[stroke.clone(), note]);
        // Undo takes back the turned mark, not the one before the turn.
        marks.turn(true, |_| (600.0, 800.0));
        marks.undo();
        marks.undo();
        marks.redo();
        assert_eq!(marks.list(), &[stroke.turned(true, (600.0, 800.0))]);
    }

    #[test]
    fn changing_a_note_undoes() {
        let note = |text: &str| Mark {
            page: 0,
            kind: Kind::Note,
            color: [1.0, 0.9, 0.3],
            shape: Shape::Note {
                at: (10.0, 10.0),
                text: text.into(),
            },
        };
        let mut marks = Marks::default();
        marks.add(note("first"));
        marks.replace(0, note("second"));
        assert_eq!(marks.list()[0].shape.typed_text(), Some("second"));
        assert!(marks.undo());
        assert_eq!(marks.list()[0].shape.typed_text(), Some("first"));
        assert!(marks.redo());
        assert_eq!(marks.list()[0].shape.typed_text(), Some("second"));
        assert_eq!(marks.at(0, 15.0, 15.0, 0.0), Some(0));
        assert_eq!(marks.at(0, 35.0, 15.0, 0.0), None);
    }

    #[test]
    fn text_wraps_between_words_and_inside_long_ones() {
        assert_eq!(wrap("", 100.0, 12.0), vec![String::new()]);
        assert_eq!(
            wrap("one two\nthree", 1000.0, 12.0),
            vec!["one two", "three"]
        );
        let lines = wrap("the quick brown fox jumps over the lazy dog", 80.0, 12.0);
        assert!(lines.len() > 1);
        assert!(lines.iter().all(|l| text_width(l, 12.0) <= 80.0));
        assert_eq!(
            lines.join(" "),
            "the quick brown fox jumps over the lazy dog"
        );
        let long = wrap("wwwwwwwwwwwwwwwwwwww", 50.0, 12.0);
        assert!(long.len() > 1 && long.iter().all(|l| text_width(l, 12.0) <= 50.0));
        assert_eq!(box_height("a\nb", 100.0, 10.0), 25.0);
    }

    #[test]
    fn a_new_mark_drops_redo() {
        let mut marks = Marks::default();
        marks.add(pen(0, vec![(0.0, 0.0)]));
        marks.undo();
        marks.add(pen(2, vec![(0.0, 0.0)]));
        assert!(!marks.can_redo());
    }

    #[test]
    fn unsaved_until_saved() {
        let mut marks = Marks::default();
        assert!(!marks.unsaved());
        marks.add(pen(0, vec![(0.0, 0.0)]));
        assert!(marks.unsaved());
        marks.saved();
        assert!(!marks.unsaved());
        marks.undo();
        assert!(marks.unsaved());
    }

    #[test]
    fn finds_the_topmost_mark_near_a_stroke() {
        let mut marks = Marks::default();
        marks.add(pen(0, vec![(0.0, 0.0), (100.0, 0.0)]));
        marks.add(Mark {
            page: 0,
            kind: Kind::Highlight,
            color: [1.0, 1.0, 0.0],
            shape: Shape::Text {
                quads: vec![Quad {
                    left: 40.0,
                    top: -5.0,
                    right: 60.0,
                    bottom: 5.0,
                }],
                text: "word".into(),
            },
        });
        assert_eq!(marks.at(0, 50.0, 0.5, 1.0), Some(1));
        assert_eq!(marks.at(0, 20.0, 1.5, 1.0), Some(0));
        assert_eq!(marks.at(0, 20.0, 9.0, 1.0), None);
        assert_eq!(marks.at(1, 50.0, 0.0, 1.0), None);
    }
}
