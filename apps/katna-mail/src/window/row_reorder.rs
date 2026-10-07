// SPDX-License-Identifier: GPL-3.0-or-later

//! Reordering a list by dragging a whole row, shared by Settings >
//! Accounts and Settings > Folders & rules: the row being dragged lifts
//! and follows the pointer, the rows it passes slide out of its way, and
//! after a drop or a Move up or down every row glides from where it was
//! to its place.

use std::cell::RefCell;
use std::rc::Rc;

use gpui::{Bounds, Div, Pixels, Stateful, Window, canvas, prelude::*, rgba};
use katna_ui::motion::{self, Spring};
use katna_ui::px;
use katna_ui::unpx;

use crate::theme::Theme;
use crate::widgets::elevation;

/// The state of one list's reordering; `K` names a row (an account, a
/// rule) so a row keeps gliding while the order changes under it.
pub(super) struct Reorder<K> {
    /// Each row's bounds as last painted, offset included.
    bounds: Rc<RefCell<Vec<Option<Bounds<Pixels>>>>>,
    /// Where the pointer went down on a row's handle.
    grab: Option<(usize, f32)>,
    moving: Option<Moving>,
    /// Rows gliding to their place after the order changed.
    gliding: Vec<(K, Spring)>,
    /// The row whose Move up or down has the keyboard focus.
    pub(super) focused: Option<usize>,
}

impl<K> Default for Reorder<K> {
    fn default() -> Self {
        Self {
            bounds: Rc::default(),
            grab: None,
            moving: None,
            gliding: Vec::new(),
            focused: None,
        }
    }
}

/// A drag under way.
struct Moving {
    from: usize,
    /// Where the row would go if dropped now.
    to: usize,
    grab_y: f32,
    pointer_y: f32,
    /// The other rows moving aside.
    slides: Vec<Spring>,
    /// Each row's offset as drawn this frame.
    drawn: Vec<f32>,
}

impl<K: Copy + PartialEq> Reorder<K> {
    /// How far row `ix` is drawn from its place.
    pub(super) fn offset(&self, ix: usize, key: K) -> f32 {
        match &self.moving {
            Some(moving) => moving.drawn.get(ix).copied().unwrap_or(0.0),
            None => self
                .gliding
                .iter()
                .find(|(g, _)| *g == key)
                .map_or(0.0, |(_, spring)| spring.value()),
        }
    }

    /// Whether row `ix` is drawn above the others: lifted or landing.
    pub(super) fn raised(&self, ix: usize, key: K) -> bool {
        match &self.moving {
            Some(moving) => moving.from == ix,
            None => self.gliding.first().is_some_and(|(g, _)| *g == key),
        }
    }

    /// Whether a row is being dragged.
    pub(super) fn dragging(&self) -> bool {
        self.moving.is_some()
    }

    /// Whether row `ix` is the one being dragged.
    pub(super) fn lifted(&self, ix: usize) -> bool {
        self.moving.as_ref().is_some_and(|m| m.from == ix)
    }

    /// Where the dragged row came from and would go if dropped now.
    pub(super) fn drop_move(&self) -> Option<(usize, usize)> {
        self.moving.as_ref().map(|m| (m.from, m.to))
    }

    /// The pointer went down on row `ix`'s handle at `y`.
    pub(super) fn grab(&mut self, ix: usize, y: f32) {
        self.grab = Some((ix, y));
    }

    /// Every row's place and height as last painted, or `None` before
    /// they all are.
    fn places(&self, count: usize) -> Option<Vec<(f32, f32)>> {
        let bounds = self.bounds.borrow();
        (0..count)
            .map(|ix| {
                let b = (*bounds.get(ix)?)?;
                Some((unpx(b.top()), unpx(b.size.height)))
            })
            .collect()
    }

    /// Row `from` of `old` went to `to` (or the drag ended without a
    /// move): each row glides from where it is drawn to its new place,
    /// the moved one above the rest. `gap` is the space between rows.
    pub(super) fn moved(&mut self, old: &[K], from: usize, to: usize, gap: f32) {
        self.grab = None;
        if from < old.len() && to < old.len() {
            let mut new = old.to_vec();
            let moved = new.remove(from);
            new.insert(to, moved);
            if let Some(places) = self.places(old.len()) {
                self.glide(old, &new, &places, moved, gap);
            }
        }
        self.moving = None;
    }

    fn glide(&mut self, old: &[K], new: &[K], places: &[(f32, f32)], raised: K, gap: f32) {
        let (Some(&first), Some(&(painted, _))) = (old.first(), places.first()) else {
            return;
        };
        // Where the first row goes with no offset.
        let mut top = painted - self.offset(0, first);
        let mut gliding = Vec::new();
        for key in new {
            let Some(old_ix) = old.iter().position(|o| o == key) else {
                continue;
            };
            let (was, height) = places[old_ix];
            if (was - top).abs() > 0.5 {
                let mut spring = Spring::new(motion::SLIDE, was - top);
                spring.set(0.0);
                gliding.push((*key, spring));
            }
            top += height + gap;
        }
        if let Some(ix) = gliding.iter().position(|(g, _)| *g == raised) {
            let lifted = gliding.remove(ix);
            gliding.insert(0, lifted);
        }
        self.gliding = gliding;
    }

    /// The pointer moved to `y` while dragging row `from` of `keys`.
    pub(super) fn dragged(&mut self, keys: &[K], from: usize, y: f32, gap: f32) {
        let count = keys.len();
        if from >= count {
            return;
        }
        if self.moving.is_none() {
            let grab_y = match self.grab.take() {
                Some((ix, grab_y)) if ix == from => grab_y,
                _ => y,
            };
            // Rows still gliding carry on from where they are.
            let drawn: Vec<f32> = keys
                .iter()
                .enumerate()
                .map(|(ix, key)| self.offset(ix, *key))
                .collect();
            let mut slides: Vec<Spring> = drawn
                .iter()
                .map(|&at| Spring::new(motion::SLIDE, at))
                .collect();
            for slide in &mut slides {
                slide.set(0.0);
            }
            self.gliding.clear();
            self.moving = Some(Moving {
                from,
                to: from,
                grab_y,
                pointer_y: y,
                slides,
                drawn,
            });
        }
        let places = self.places(count);
        let Some(moving) = self.moving.as_mut() else {
            return;
        };
        moving.pointer_y = y;
        if let Some(places) = places {
            let natural: Vec<(f32, f32)> = places
                .iter()
                .zip(&moving.drawn)
                .map(|(&(top, height), drawn)| (top - drawn, height))
                .collect();
            let (top, height) = natural[moving.from];
            let center = top + height / 2.0 + (y - moving.grab_y);
            let to = natural
                .iter()
                .enumerate()
                .filter(|&(ix, &(top, height))| ix != moving.from && top + height / 2.0 < center)
                .count();
            if to != moving.to {
                moving.to = to;
                let step = height + gap;
                for (ix, slide) in moving.slides.iter_mut().enumerate() {
                    slide.set(if ix > moving.from && ix <= to {
                        -step
                    } else if ix < moving.from && ix >= to {
                        step
                    } else {
                        0.0
                    });
                }
            }
        }
    }

    /// Advances the sliding rows of a list of `count` for this frame.
    pub(super) fn tick(&mut self, count: usize, window: &Window, reduce: bool) {
        self.bounds.borrow_mut().resize(count, None);
        let places = self.places(count);
        if let Some(moving) = self.moving.as_mut() {
            for (ix, slide) in moving.slides.iter_mut().enumerate() {
                if ix != moving.from {
                    moving.drawn[ix] = slide.tick(window, reduce);
                }
            }
            let mut lifted = moving.pointer_y - moving.grab_y;
            // The lifted row stays between the first row's top and the
            // last row's bottom.
            if let Some(places) = places
                && let Some(last) = places.last()
            {
                let at = |ix: usize| places[ix].0 - moving.drawn[ix];
                let top = at(0);
                let bottom = at(count - 1) + last.1;
                let (own, height) = (at(moving.from), places[moving.from].1);
                lifted = lifted.clamp(top - own, (bottom - height - own).max(top - own));
            }
            moving.drawn[moving.from] = lifted;
        }
        self.gliding.retain_mut(|(_, spring)| {
            spring.tick(window, reduce);
            !spring.settled()
        });
    }

    /// Row `ix` (`key`) drawn where it is: offset while it slides, raised
    /// with a shadow while it is lifted or landing, its place noted for
    /// the next frame.
    pub(super) fn row(&self, row: Stateful<Div>, ix: usize, key: K, th: &Theme) -> Stateful<Div> {
        let offset = self.offset(ix, key);
        let raised = self.raised(ix, key);
        // The shadow of a lifted row, fading as it lands.
        let lift = if self.lifted(ix) {
            3.0
        } else if raised {
            (offset.abs() / 8.0).min(3.0)
        } else {
            0.0
        };
        let bounds = self.bounds.clone();
        row.relative()
            .top(px(offset))
            .when(raised, |d| {
                d.bg(rgba(th.surface)).shadow(elevation(th, lift))
            })
            .child(
                canvas(
                    move |b, _, _| {
                        let mut all = bounds.borrow_mut();
                        if all.len() <= ix {
                            all.resize(ix + 1, None);
                        }
                        all[ix] = Some(b);
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .top_0()
                .left_0()
                .size_full(),
            )
    }
}
