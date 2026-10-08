// SPDX-License-Identifier: GPL-3.0-or-later

//! Laying out an element in a text direction: mail and its paragraphs
//! read their own way whatever the window's (`katna_core::bidi`).

use gpui::Styled;
pub use katna_core::bidi::Direction;

/// `element` laid out in `dir` (its text aligned to that direction's
/// start, its rows from that side); `None` leaves it as what holds it.
pub fn directed<E: Styled>(element: E, dir: Option<Direction>) -> E {
    match dir {
        Some(Direction::Ltr) => element.layout_ltr(),
        Some(Direction::Rtl) => element.layout_rtl(),
        None => element,
    }
}
