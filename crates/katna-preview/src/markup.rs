// SPDX-License-Identifier: GPL-3.0-or-later

//! Marks made on a PDF in the viewer: highlights, underlines, squiggles,
//! strike-throughs and pen strokes, with undo and redo. They are kept in
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
}

impl Kind {
    /// Marks laid on text, made by selecting it.
    pub fn on_text(self) -> bool {
        !matches!(self, Kind::Ink)
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

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    /// The boxes of the marked text, one per line, and the text itself.
    Text { quads: Vec<Quad>, text: String },
    /// A stroke's points, in the order drawn.
    Ink(Vec<(f32, f32)>),
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
    /// Whether the point `(x, y)` on `page` is on this mark, give or take
    /// `slop` points.
    pub fn hit(&self, page: usize, x: f32, y: f32, slop: f32) -> bool {
        if page != self.page {
            return false;
        }
        match &self.shape {
            Shape::Text { quads, .. } => quads.iter().any(|q| q.contains(x, y, slop)),
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
