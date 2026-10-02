// SPDX-License-Identifier: GPL-3.0-or-later

//! Moving the window from empty space, as from its title bar.
//!
//! A [`WindowDrag::window_drag`] area (the rail, the folders, a toolbar)
//! starts a window move when it is pressed and the pointer moves a few
//! pixels. Anything clickable in it or over it calls
//! [`WindowDrag::keeps_press`], so a press there stays its own: a button
//! pressed and dragged off never moves the window.
//!
//! GPUI hands a mouse press to every element under the pointer, the one on
//! top first, so the clickable element marks the press before the area
//! sees it.

use std::cell::Cell;

use gpui::{InteractiveElement, MouseButton, MouseDownEvent, MouseMoveEvent, Pixels, Point};

/// How far the pointer moves, pressed, before the window does: more than
/// GPUI's own drags take (2 px), so a mail dragged to a folder stays a mail
/// drag, and a shaky click stays a click.
const THRESHOLD: f32 = 4.0;

#[derive(Clone, Copy, Default)]
enum Press {
    #[default]
    None,
    /// A clickable element took the press at this point.
    Kept(Point<Pixels>),
    /// An area was pressed here; the window moves once the pointer does.
    Armed(Point<Pixels>),
}

thread_local! {
    static PRESS: Cell<Press> = const { Cell::new(Press::None) };
}

fn down(event: &MouseDownEvent) {
    let press = match PRESS.get() {
        Press::Kept(at) if at == event.position => Press::None,
        _ => Press::Armed(event.position),
    };
    PRESS.set(press);
}

fn moved(event: &MouseMoveEvent, window: &mut gpui::Window) {
    let Press::Armed(at) = PRESS.get() else {
        return;
    };
    if event.pressed_button != Some(MouseButton::Left) {
        PRESS.set(Press::None);
        return;
    }
    let d = event.position - at;
    if crate::unpx(d.x).hypot(crate::unpx(d.y)) > THRESHOLD {
        PRESS.set(Press::None);
        window.start_window_move();
    }
}

pub trait WindowDrag: InteractiveElement + Sized {
    /// Pressing this element's empty space and moving moves the window.
    fn window_drag(self) -> Self {
        self.on_mouse_down(MouseButton::Left, |event, _, _| down(event))
            .on_mouse_up(MouseButton::Left, |_, _, _| PRESS.set(Press::None))
            .on_mouse_move(|event, window, _| moved(event, window))
    }

    /// A press on this element is its own, never a window move.
    fn keeps_press(self) -> Self {
        self.on_mouse_down(MouseButton::Left, |event, _, _| {
            PRESS.set(Press::Kept(event.position))
        })
    }
}

impl<E: InteractiveElement> WindowDrag for E {}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, point};

    fn press(at: Point<Pixels>) -> MouseDownEvent {
        MouseDownEvent {
            button: MouseButton::Left,
            position: at,
            modifiers: Modifiers::default(),
            click_count: 1,
            first_mouse: false,
        }
    }

    #[test]
    fn a_kept_press_never_arms() {
        let at = point(crate::px(10.0), crate::px(20.0));
        PRESS.set(Press::Kept(at));
        down(&press(at));
        assert!(matches!(PRESS.get(), Press::None));
        // A press left over from elsewhere does not keep the next one.
        PRESS.set(Press::Kept(at));
        let other = point(crate::px(30.0), crate::px(20.0));
        down(&press(other));
        assert!(matches!(PRESS.get(), Press::Armed(p) if p == other));
    }
}
