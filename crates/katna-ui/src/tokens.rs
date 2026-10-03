// SPDX-License-Identifier: GPL-3.0-or-later

//! Design tokens: the named sizes every Katna window is built from
//! (`docs/DESIGN.md`). Lengths are design pixels, for [`crate::px`].
//! Colours are roles in each app's theme; what lives here are the
//! opacities those roles are built with, so a hover or a selected row is
//! the same strength everywhere.
//!
//! New code takes its radii, spacing, text sizes, state opacities and
//! durations from here. `ci/check-tokens.sh` counts the raw numbers still
//! typed in the GPUI crates and only lets that count go down.

/// Corner radii. Nested shapes take [`radius::inner`], so a hover inside a
/// card follows the card's corner.
pub mod radius {
    /// Checkboxes, tags, inline code.
    pub const XS: f32 = 4.0;
    /// Fields, menus and their rows, thumbnails.
    pub const SM: f32 = 8.0;
    /// Tiles and cards inside a card.
    pub const MD: f32 = 12.0;
    /// Cards, dialogs, popovers, sheets, Compose.
    pub const LG: f32 = 16.0;
    /// Pills and avatars: round at any height.
    pub const FULL: f32 = 9999.0;

    /// The radius of a shape `padding` inside one rounded by `outer`, so
    /// their corners run parallel; never below [`XS`] unless the outer is.
    pub fn inner(outer: f32, padding: f32) -> f32 {
        (outer - padding).max(XS.min(outer)).max(0.0)
    }
}

/// Spacing on a 4 px grid, with 2 px for optical nudges. Odd values are
/// only for 1-2 px optical alignment, with a comment saying why.
pub mod space {
    /// Optical nudges only.
    pub const S1: f32 = 2.0;
    /// Icon to its label.
    pub const S2: f32 = 4.0;
    /// Between controls.
    pub const S3: f32 = 8.0;
    /// Row padding.
    pub const S4: f32 = 12.0;
    /// Card padding.
    pub const S5: f32 = 16.0;
    /// Dialog padding, between sections.
    pub const S6: f32 = 24.0;
    /// Between blocks of a page.
    pub const S7: f32 = 32.0;
    /// Around empty states.
    pub const S8: f32 = 48.0;
}

/// Text sizes, each with the line height it is set in. The weights are
/// GPUI's `NORMAL`, `MEDIUM`, `SEMIBOLD` and `BOLD`.
pub mod text {
    /// Badges, counts, timestamps in chips.
    pub const MICRO: f32 = 11.0;
    /// Hints, secondary labels, dates in the list.
    pub const CAPTION: f32 = 12.0;
    /// Previews, menu shortcuts, dense rows.
    pub const SMALL: f32 = 13.0;
    /// Rows, menus, buttons, mail text.
    pub const BODY: f32 = 14.0;
    /// Pane headers, card titles.
    pub const SUBTITLE: f32 = 16.0;
    /// The open mail's subject, dialog titles.
    pub const TITLE: f32 = 20.0;
    /// Page titles, onboarding.
    pub const DISPLAY: f32 = 24.0;

    /// The line height for text of `size`.
    pub fn line_height(size: f32) -> f32 {
        match size {
            s if s <= CAPTION => 16.0,
            s if s <= SMALL => 18.0,
            s if s <= BODY => 20.0,
            s if s <= SUBTITLE => 24.0,
            s if s <= TITLE => 28.0,
            _ => 32.0,
        }
    }
}

/// How strongly a state shows: the text colour at this opacity, laid over
/// the element.
pub mod state {
    /// Under the pointer.
    pub const HOVER: f32 = 0.07;
    /// Pressed, and the ink of click ripples.
    pub const PRESSED: f32 = 0.14;
    /// The open line of a pane, in light colors.
    pub const SELECTED: f32 = 0.10;
    /// The open line of a pane, in dark colors.
    pub const SELECTED_DARK: f32 = 0.12;
    /// Being dragged.
    pub const DRAGGED: f32 = 0.16;
    /// A control that cannot be used now: its whole opacity.
    pub const DISABLED: f32 = 0.38;
    /// A separating line, against the edge of a box (`line.faint` against
    /// `line.edge`).
    pub const FAINT_LINE: f32 = 0.25;
}

/// How far a thing floats: the surface, shadow and edge go together.
pub mod elevation {
    /// The page: flat.
    pub const PAGE: f32 = 0.0;
    /// Cards: the list, the open mail, the contact card, the agenda.
    pub const CARD: f32 = 1.0;
    /// Floating buttons, something being dragged.
    pub const FLOAT: f32 = 2.0;
    /// Menus and dialogs.
    pub const MENU: f32 = 3.0;
    /// Popovers that point at what they belong to, the tour.
    pub const POPOVER: f32 = 4.0;
}

/// Durations of timed fades. Movement uses the springs in
/// [`crate::motion`].
pub mod duration {
    use std::time::Duration;

    /// Hover in and out.
    pub const FAST: Duration = Duration::from_millis(140);
    /// Fades and folds.
    pub const BASE: Duration = Duration::from_millis(220);
    /// Page swaps.
    pub const SLOW: Duration = Duration::from_millis(400);
    /// Slow reveals.
    pub const LINGER: Duration = Duration::from_millis(900);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scales_grow() {
        let radii = [radius::XS, radius::SM, radius::MD, radius::LG, radius::FULL];
        assert!(radii.windows(2).all(|w| w[0] < w[1]));
        let spaces = [
            space::S1,
            space::S2,
            space::S3,
            space::S4,
            space::S5,
            space::S6,
            space::S7,
            space::S8,
        ];
        assert!(spaces.windows(2).all(|w| w[0] < w[1]));
        let sizes = [
            text::MICRO,
            text::CAPTION,
            text::SMALL,
            text::BODY,
            text::SUBTITLE,
            text::TITLE,
            text::DISPLAY,
        ];
        assert!(sizes.windows(2).all(|w| w[0] < w[1]));
        assert!(sizes.iter().all(|&s| text::line_height(s) > s));
    }

    #[test]
    fn inner_radius_follows_the_outer() {
        assert_eq!(radius::inner(radius::LG, 4.0), radius::MD);
        assert_eq!(radius::inner(radius::MD, 12.0), radius::XS);
        assert_eq!(radius::inner(2.0, 8.0), 2.0);
    }
}
