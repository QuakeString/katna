// SPDX-License-Identifier: GPL-3.0-or-later

//! Frame geometry for client-side decorations: where the visible frame sits
//! inside the surface, which part of the shadow takes input, and which resize
//! edge the pointer is on.
//!
//! Plain `f32` logical pixels, no GPUI types, so it is unit-tested. The
//! numbers follow GTK 4 (`RESIZE_HANDLE_SIZE` 12 px, corner handles 24 px
//! along each edge), so resizing feels the same as in GNOME's own apps.

/// Width of the resize band outside the visible frame.
pub const RESIZE_HANDLE: f32 = 12.0;

/// How far a corner handle reaches along each edge.
pub const RESIZE_CORNER: f32 = 24.0;

/// Which sides of the window touch a screen edge or another tiled window.
/// Tiled sides have no shadow, no rounded corners and cannot be resized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Sides {
    pub top: bool,
    pub right: bool,
    pub bottom: bool,
    pub left: bool,
}

impl Sides {
    pub const NONE: Self = Self {
        top: false,
        right: false,
        bottom: false,
        left: false,
    };
    pub const ALL: Self = Self {
        top: true,
        right: true,
        bottom: true,
        left: true,
    };

    pub fn any(self) -> bool {
        self.top || self.right || self.bottom || self.left
    }

    pub fn all(self) -> bool {
        self.top && self.right && self.bottom && self.left
    }
}

/// A resize edge or corner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edge {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

/// An axis-aligned rectangle in surface coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
    }
}

/// Frame geometry of one window at one moment.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrameGeometry {
    /// Surface size, including the shadow margin.
    pub surface_width: f32,
    pub surface_height: f32,
    /// Shadow margin on every side that is not tiled.
    pub inset: f32,
    pub tiled: Sides,
}

impl FrameGeometry {
    fn margin(&self, tiled: bool) -> f32 {
        if tiled { 0.0 } else { self.inset }
    }

    /// The visible window (what the compositor gets as window geometry).
    pub fn frame(&self) -> Rect {
        let left = self.margin(self.tiled.left);
        let top = self.margin(self.tiled.top);
        let right = self.margin(self.tiled.right);
        let bottom = self.margin(self.tiled.bottom);
        Rect {
            x: left,
            y: top,
            width: (self.surface_width - left - right).max(0.0),
            height: (self.surface_height - top - bottom).max(0.0),
        }
    }

    /// The part of the surface that takes pointer input: the frame plus the
    /// resize band on non-tiled sides. Clicks on the rest of the shadow go to
    /// the window below, as in GTK.
    pub fn input_region(&self) -> Rect {
        let frame = self.frame();
        let grow = |tiled: bool| {
            if tiled {
                0.0
            } else {
                RESIZE_HANDLE.min(self.inset)
            }
        };
        let left = grow(self.tiled.left);
        let top = grow(self.tiled.top);
        let right = grow(self.tiled.right);
        let bottom = grow(self.tiled.bottom);
        Rect {
            x: frame.x - left,
            y: frame.y - top,
            width: frame.width + left + right,
            height: frame.height + top + bottom,
        }
    }

    /// The resize edge under the pointer, if any.
    /// The surface around the frame, as rectangles: where the shadow of a
    /// translucent window may paint without showing through the window.
    /// Corners rounded by `radius` (those with no tiled side) are followed
    /// one pixel row at a time, as the compositor's blur region is.
    pub fn outside(&self, radius: f32) -> Vec<Rect> {
        let f = self.frame();
        let (w, h) = (self.surface_width, self.surface_height);
        let mut rects = vec![
            Rect {
                x: 0.0,
                y: 0.0,
                width: w,
                height: f.y,
            },
            Rect {
                x: 0.0,
                y: f.bottom(),
                width: w,
                height: h - f.bottom(),
            },
            Rect {
                x: 0.0,
                y: f.y,
                width: f.x,
                height: f.height,
            },
            Rect {
                x: f.right(),
                y: f.y,
                width: w - f.right(),
                height: f.height,
            },
        ];
        let t = self.tiled;
        let radius = radius
            .min(f.width / 2.0)
            .min(f.height / 2.0)
            .max(0.0)
            .round();
        let rows = radius as u32;
        for row in 0..rows {
            let cut = corner_cut(radius, row);
            if cut <= 0.0 {
                continue;
            }
            let (top, bottom) = (f.y + row as f32, f.bottom() - 1.0 - row as f32);
            let (left, right) = (f.x, f.right() - cut);
            for (round, x, y) in [
                (!t.top && !t.left, left, top),
                (!t.top && !t.right, right, top),
                (!t.bottom && !t.left, left, bottom),
                (!t.bottom && !t.right, right, bottom),
            ] {
                if round {
                    rects.push(Rect {
                        x,
                        y,
                        width: cut,
                        height: 1.0,
                    });
                }
            }
        }
        rects.retain(|r| r.width > 0.0 && r.height > 0.0);
        rects
    }

    pub fn resize_edge(&self, x: f32, y: f32) -> Option<Edge> {
        let frame = self.frame();
        if frame.contains(x, y) || !self.input_region().contains(x, y) {
            return None;
        }

        // Outside the frame on at least one side. A corner handle reaches
        // RESIZE_CORNER along each edge.
        let left = x < frame.x + RESIZE_CORNER && !self.tiled.left;
        let right = !left && x >= frame.right() - RESIZE_CORNER && !self.tiled.right;
        let top = y < frame.y + RESIZE_CORNER && !self.tiled.top;
        let bottom = !top && y >= frame.bottom() - RESIZE_CORNER && !self.tiled.bottom;

        match (top, right, bottom, left) {
            (true, false, false, true) => Some(Edge::TopLeft),
            (true, true, false, false) => Some(Edge::TopRight),
            (false, true, true, false) => Some(Edge::BottomRight),
            (false, false, true, true) => Some(Edge::BottomLeft),
            (true, false, false, false) => Some(Edge::Top),
            (false, true, false, false) => Some(Edge::Right),
            (false, false, true, false) => Some(Edge::Bottom),
            (false, false, false, true) => Some(Edge::Left),
            _ => None,
        }
    }
}

/// How far row `row` of a corner with `radius` (row 0 is the outer edge)
/// lies outside the circle, in whole pixels.
pub fn corner_cut(radius: f32, row: u32) -> f32 {
    let dy = radius - (row as f32 + 0.5);
    (radius - (radius * radius - dy * dy).max(0.0).sqrt()).round()
}

#[cfg(test)]
mod tests {
    use super::*;

    const FLOATING: FrameGeometry = FrameGeometry {
        surface_width: 896.0,
        surface_height: 696.0,
        inset: 48.0,
        tiled: Sides::NONE,
    };

    #[test]
    fn frame_is_inset_on_untiled_sides() {
        assert_eq!(
            FLOATING.frame(),
            Rect {
                x: 48.0,
                y: 48.0,
                width: 800.0,
                height: 600.0
            }
        );
        let left_half = FrameGeometry {
            tiled: Sides {
                top: true,
                bottom: true,
                left: true,
                right: false,
            },
            ..FLOATING
        };
        let frame = left_half.frame();
        assert_eq!((frame.x, frame.y), (0.0, 0.0));
        assert_eq!((frame.width, frame.height), (848.0, 696.0));
    }

    #[test]
    fn input_region_is_frame_plus_handle() {
        let region = FLOATING.input_region();
        assert_eq!(region.x, 36.0);
        assert_eq!(region.width, 824.0);
        // Shadow beyond the handle passes clicks through.
        assert_eq!(FLOATING.resize_edge(10.0, 300.0), None);
    }

    #[test]
    fn edges_and_corners() {
        let g = FLOATING;
        assert_eq!(g.resize_edge(40.0, 300.0), Some(Edge::Left));
        assert_eq!(g.resize_edge(855.0, 300.0), Some(Edge::Right));
        assert_eq!(g.resize_edge(860.0, 300.0), None);
        assert_eq!(g.resize_edge(400.0, 40.0), Some(Edge::Top));
        assert_eq!(g.resize_edge(400.0, 650.0), Some(Edge::Bottom));
        assert_eq!(g.resize_edge(40.0, 40.0), Some(Edge::TopLeft));
        // The corner handle reaches 24 px along the edge.
        assert_eq!(g.resize_edge(40.0, 60.0), Some(Edge::TopLeft));
        assert_eq!(g.resize_edge(60.0, 40.0), Some(Edge::TopLeft));
        assert_eq!(g.resize_edge(40.0, 80.0), Some(Edge::Left));
        assert_eq!(g.resize_edge(850.0, 650.0), Some(Edge::BottomRight));
        assert_eq!(g.resize_edge(840.0, 650.0), Some(Edge::BottomRight));
        // Inside the frame: not a resize edge.
        assert_eq!(g.resize_edge(400.0, 300.0), None);
    }

    #[test]
    fn tiled_sides_do_not_resize() {
        let g = FrameGeometry {
            tiled: Sides {
                top: true,
                bottom: true,
                left: true,
                right: false,
            },
            ..FLOATING
        };
        // Only the right edge is free.
        assert_eq!(g.resize_edge(850.0, 300.0), Some(Edge::Right));
        assert_eq!(g.resize_edge(850.0, 5.0), Some(Edge::Right));
        assert_eq!(g.resize_edge(0.0, 300.0), None);
    }

    #[test]
    fn maximized_has_no_margin_and_no_edges() {
        let g = FrameGeometry {
            tiled: Sides::ALL,
            ..FLOATING
        };
        assert_eq!(g.frame().width, 896.0);
        assert_eq!(g.input_region(), g.frame());
        assert_eq!(g.resize_edge(0.0, 0.0), None);
    }

    #[test]
    fn outside_leaves_out_the_rounded_frame() {
        let g = FrameGeometry {
            surface_width: 200.0,
            surface_height: 100.0,
            inset: 20.0,
            tiled: Sides::NONE,
        };
        let pieces = g.outside(5.0);
        let f = g.frame();
        // No piece covers the middle of the frame or its straight edges.
        for p in &pieces {
            assert!(!p.contains(f.x + 10.0, f.y));
            assert!(!p.contains(f.x + f.width / 2.0, f.y + f.height / 2.0));
        }
        // The outer corner pixel is outside the round corner; the shadow
        // paints there.
        assert!(pieces.iter().any(|p| p.contains(f.x, f.y)));
        assert!(
            pieces
                .iter()
                .any(|p| p.contains(f.right() - 0.5, f.bottom() - 0.5))
        );
        // The margin is covered.
        assert!(pieces.iter().any(|p| p.contains(5.0, 50.0)));
        assert!(pieces.iter().any(|p| p.contains(100.0, 95.0)));
        // Tiled: no corner rows on that side, no margin there.
        let tiled = FrameGeometry {
            tiled: Sides {
                left: true,
                ..Sides::NONE
            },
            ..g
        };
        let pieces = tiled.outside(5.0);
        let f = tiled.frame();
        assert!(!pieces.iter().any(|p| p.contains(f.x, f.y)));
        assert!(pieces.iter().any(|p| p.contains(f.right() - 0.5, f.y)));
    }

    #[test]
    fn corner_cut_matches_a_circle() {
        assert_eq!(corner_cut(5.0, 0), 3.0);
        assert_eq!(corner_cut(5.0, 4), 0.0);
        assert_eq!(corner_cut(0.0, 0), 0.0);
    }
}
