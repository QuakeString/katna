// SPDX-License-Identifier: GPL-3.0-or-later

//! Menus, popovers and tooltips placed at a point, kept inside the window's
//! content: GPUI's `anchored()`, with the same builder, but fitted to the
//! content rather than to the window's whole surface. With Katna's own
//! frame the surface also holds the frame's shadow and resize border, and
//! GPUI's version only moves a popup once it runs past the surface, so one
//! near an edge could sit over the shadow, cut off by the screen or the
//! next window.
//!
//! The frame says where its content is with [`set_content_insets`] each
//! frame; without a frame (the desktop's own) the content is the surface.

use std::collections::HashMap;

use gpui::{
    Anchor, AnyElement, App, Axis, Bounds, Display, Edges, Element, ElementId, Global,
    GlobalElementId, InspectorElementId, IntoElement, LayoutId, ParentElement, Pixels, Point,
    Position, Style, Window, WindowId, point,
};

/// How far in from each edge of each window's surface its content starts.
#[derive(Default)]
struct ContentInsets(HashMap<WindowId, Edges<Pixels>>);

impl Global for ContentInsets {}

/// Popups in `window` stay this far in from its surface's edges: where
/// its frame's shadow, resize border and line end. Call it each frame,
/// with zero edges when the window has no frame of its own.
pub fn set_content_insets(window: &Window, edges: Edges<Pixels>, cx: &mut App) {
    let id = window.window_handle().window_id();
    let insets = cx.default_global::<ContentInsets>();
    if edges == Edges::default() {
        insets.0.remove(&id);
    } else {
        insets.0.insert(id, edges);
    }
}

/// The part of `window` that popups may cover, in window coordinates.
pub fn content_bounds(window: &Window, cx: &App) -> Bounds<Pixels> {
    let insets = cx
        .try_global::<ContentInsets>()
        .and_then(|insets| insets.0.get(&window.window_handle().window_id()).copied())
        .unwrap_or_default();
    let size = window.viewport_size();
    Bounds::new(
        point(insets.left, insets.top),
        gpui::size(
            (size.width - insets.left - insets.right).max(Pixels::ZERO),
            (size.height - insets.top - insets.bottom).max(Pixels::ZERO),
        ),
    )
}

/// An element placed at a point that keeps inside the window's content,
/// as GPUI's `anchored()`. Its children should have no margin.
pub fn anchored() -> Anchored {
    Anchored {
        children: Vec::new(),
        anchor: Anchor::TopLeft,
        fit: Fit::SwitchAnchor,
        position: None,
        local: false,
        offset: None,
    }
}

pub struct Anchored {
    children: Vec<AnyElement>,
    anchor: Anchor,
    fit: Fit,
    position: Option<Point<Pixels>>,
    /// `position` counts from the parent rather than the window.
    local: bool,
    offset: Option<Point<Pixels>>,
}

#[derive(Clone, Copy, PartialEq)]
enum Fit {
    /// Turns to the other side of its point when it would run out.
    SwitchAnchor,
    /// Slides back inside, this far from the edges.
    Snap(Edges<Pixels>),
}

impl Anchored {
    /// Which corner of the element sits at its point.
    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    /// The point, in window coordinates (or the parent's, with
    /// [`Anchored::local`]); else where the element would be laid out.
    pub fn position(mut self, position: Point<Pixels>) -> Self {
        self.position = Some(position);
        self
    }

    /// Moves the element this far from its point.
    pub fn offset(mut self, offset: Point<Pixels>) -> Self {
        self.offset = Some(offset);
        self
    }

    /// [`Anchored::position`] counts from the parent.
    pub fn local(mut self) -> Self {
        self.local = true;
        self
    }

    /// Slides back inside the content when it would run out, rather than
    /// turning to the other side of its point.
    pub fn snap_to_window(self) -> Self {
        self.snap_to_window_with_margin(Edges::default())
    }

    /// As [`Anchored::snap_to_window`], keeping `edges` clear.
    pub fn snap_to_window_with_margin(mut self, edges: impl Into<Edges<Pixels>>) -> Self {
        self.fit = Fit::Snap(edges.into());
        self
    }
}

impl ParentElement for Anchored {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl IntoElement for Anchored {
    type Element = Self;

    fn into_element(self) -> Self {
        self
    }
}

impl Element for Anchored {
    type RequestLayoutState = Vec<LayoutId>;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let children = self
            .children
            .iter_mut()
            .map(|child| child.request_layout(window, cx))
            .collect::<Vec<_>>();
        let style = Style {
            position: Position::Absolute,
            display: Display::Flex,
            ..Style::default()
        };
        (
            window.request_layout(style, children.iter().copied(), cx),
            children,
        )
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        children: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(size) = children
            .iter()
            .map(|id| window.layout_bounds(*id))
            .reduce(|a, b| a.union(&b))
            .map(|b| b.size)
        else {
            return;
        };
        let offset = self.offset.unwrap_or_default();
        let at = match (self.position, self.local) {
            (Some(at), true) => bounds.origin + at,
            (Some(at), false) => at,
            (None, _) => bounds.origin,
        };
        let content = content_bounds(window, cx);
        let place = |anchor: Anchor| Bounds::from_anchor_and_size(anchor, at + offset, size);
        let mut placed = place(self.anchor);
        let edges = match self.fit {
            Fit::SwitchAnchor => {
                let mut anchor = self.anchor;
                let out_x =
                    |b: &Bounds<Pixels>| b.left() < content.left() || b.right() > content.right();
                let out_y =
                    |b: &Bounds<Pixels>| b.top() < content.top() || b.bottom() > content.bottom();
                if out_x(&placed) {
                    let other = anchor.other_side_along(Axis::Horizontal);
                    if !out_x(&place(other)) {
                        anchor = other;
                        placed = place(anchor);
                    }
                }
                if out_y(&placed) {
                    let other = anchor.other_side_along(Axis::Vertical);
                    if !out_y(&place(other)) {
                        placed = place(other);
                    }
                }
                Edges::default()
            }
            Fit::Snap(edges) => edges,
        };
        // Either way, whatever still runs out slides back in, as GPUI's.
        let room = Bounds::from_corners(
            content.origin + point(edges.left, edges.top),
            content.bottom_right() - point(edges.right, edges.bottom),
        );
        placed.origin = snapped(placed, room);
        let shift = placed.origin - bounds.origin;
        window.with_element_offset(point(shift.x.round(), shift.y.round()), |window| {
            for child in &mut self.children {
                child.prepaint(window, cx);
            }
        });
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        for child in &mut self.children {
            child.paint(window, cx);
        }
    }
}

/// Where `placed` goes to be inside `room`: slid back from an edge it runs
/// past, and to the start (left, top) when it is wider or taller than it.
fn snapped(placed: Bounds<Pixels>, room: Bounds<Pixels>) -> Point<Pixels> {
    let mut at = placed.origin;
    if placed.right() > room.right() {
        at.x -= placed.right() - room.right();
    }
    if at.x < room.left() {
        at.x = room.left();
    }
    if placed.bottom() > room.bottom() {
        at.y -= placed.bottom() - room.bottom();
    }
    if at.y < room.top() {
        at.y = room.top();
    }
    at
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::size;

    fn px(v: f32) -> Pixels {
        Pixels::from(v)
    }

    fn b(x: f32, y: f32, w: f32, h: f32) -> Bounds<Pixels> {
        Bounds::new(point(px(x), px(y)), size(px(w), px(h)))
    }

    #[test]
    fn a_popup_slides_back_inside_the_content_not_the_surface() {
        // A 400 px surface with a 24 px shadow each side: the content is
        // 24..376. A 200 px menu at 190 ran 14 px over the shadow.
        let room = b(24.0, 24.0, 352.0, 600.0);
        assert_eq!(
            snapped(b(190.0, 100.0, 200.0, 50.0), room),
            point(px(176.0), px(100.0))
        );
        assert_eq!(
            snapped(b(10.0, 10.0, 200.0, 50.0), room),
            point(px(24.0), px(24.0))
        );
        // Inside already: left be.
        assert_eq!(
            snapped(b(40.0, 40.0, 100.0, 50.0), room),
            point(px(40.0), px(40.0))
        );
        // Wider than the room: from its start.
        assert_eq!(
            snapped(b(100.0, 40.0, 500.0, 50.0), room),
            point(px(24.0), px(40.0))
        );
    }
}
