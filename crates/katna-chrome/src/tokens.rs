// SPDX-License-Identifier: GPL-3.0-or-later

//! Theme tokens for the window chrome (`docs/ARCHITECTURE.md` §13.2).
//!
//! Colors are `0xRRGGBBAA`. Values are approximations of libadwaita 1.5 and
//! Breeze (Plasma 6), not copies of their stylesheets; the portal accent
//! color and `kdeglobals` palette replace them in `katna-platform` later.

use crate::desktop::Preset;

/// One box shadow layer, CSS order: x, y, blur, spread, color.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shadow {
    pub x: f32,
    pub y: f32,
    pub blur: f32,
    pub spread: f32,
    pub color: u32,
}

/// How window buttons look.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonStyle {
    /// Adwaita: small circles with a faint fill.
    Circle,
    /// Breeze: no fill until hovered; close turns red on hover.
    Flat,
}

/// Chrome tokens for one preset in one color scheme.
#[derive(Debug, Clone, PartialEq)]
pub struct ChromeTokens {
    pub preset: Preset,
    pub dark: bool,
    /// Corner radius of a floating window.
    pub window_radius: f32,
    /// Shadow margin around a floating window (surface inset).
    pub shadow_inset: f32,
    pub shadow_focused: [Shadow; 3],
    pub shadow_unfocused: [Shadow; 3],
    /// 1 px outline around the window, drawn over the shadow.
    pub outline: u32,
    pub window_bg: u32,
    /// Background of content views: lists and the reading pane.
    pub view_bg: u32,
    /// Background of the sidebar.
    pub sidebar_bg: u32,
    pub fg: u32,
    pub fg_dim: u32,
    pub header_height: f32,
    pub header_bg: u32,
    pub header_bg_unfocused: u32,
    /// Line between the header bar and the content.
    pub header_shade: u32,
    pub title_weight: u16,
    pub title_size: f32,
    pub button_style: ButtonStyle,
    pub button_size: f32,
    pub button_icon_size: f32,
    pub button_gap: f32,
    pub button_bg: u32,
    pub button_bg_hover: u32,
    pub button_bg_active: u32,
    pub close_bg_hover: u32,
    pub close_fg_hover: u32,
    pub accent: u32,
}

impl ChromeTokens {
    pub fn new(preset: Preset, dark: bool) -> Self {
        match preset {
            Preset::AdwaitaLike => Self::adwaita(dark),
            Preset::BreezeLike => Self::breeze(dark),
        }
    }

    fn adwaita(dark: bool) -> Self {
        // libadwaita window.csd: a layered shadow when focused, a tight one
        // in the backdrop.
        let shadow_focused = [
            Shadow {
                x: 0.0,
                y: 2.0,
                blur: 8.0,
                spread: 2.0,
                color: 0x00000021,
            },
            Shadow {
                x: 0.0,
                y: 3.0,
                blur: 20.0,
                spread: 10.0,
                color: 0x00000017,
            },
            Shadow {
                x: 0.0,
                y: 6.0,
                blur: 32.0,
                spread: 16.0,
                color: 0x00000008,
            },
        ];
        let shadow_unfocused = [
            Shadow {
                x: 0.0,
                y: 2.0,
                blur: 6.0,
                spread: 2.0,
                color: 0x00000012,
            },
            Shadow {
                x: 0.0,
                y: 1.0,
                blur: 3.0,
                spread: 0.0,
                color: 0x00000010,
            },
            Shadow {
                x: 0.0,
                y: 0.0,
                blur: 0.0,
                spread: 0.0,
                color: 0x00000000,
            },
        ];
        let (window_bg, fg, header_bg, header_shade, outline) = if dark {
            (0x242424ff, 0xffffffff, 0x303030ff, 0x0000005c, 0xffffff12)
        } else {
            (0xfafafaff, 0x000000cc, 0xebebebff, 0x00000012, 0x00000024)
        };
        Self {
            preset: Preset::AdwaitaLike,
            dark,
            window_radius: 12.0,
            // Covers the largest layer (6 + 32 + 16) on the sides; the
            // bottom edge of the farthest layer is clipped by 6 px.
            shadow_inset: 48.0,
            shadow_focused,
            shadow_unfocused,
            outline,
            window_bg,
            view_bg: if dark { 0x1e1e1eff } else { 0xffffffff },
            sidebar_bg: if dark { 0x2e2e32ff } else { 0xebebedff },
            fg,
            fg_dim: with_alpha(fg, 0x80),
            header_height: 47.0,
            header_bg,
            header_bg_unfocused: if dark { 0x242424ff } else { 0xfafafaff },
            header_shade,
            title_weight: 700,
            title_size: 14.7,
            button_style: ButtonStyle::Circle,
            button_size: 24.0,
            button_icon_size: 16.0,
            button_gap: 12.0,
            button_bg: with_alpha(fg, 0x1a),
            button_bg_hover: with_alpha(fg, 0x26),
            button_bg_active: with_alpha(fg, 0x40),
            close_bg_hover: with_alpha(fg, 0x26),
            close_fg_hover: fg,
            accent: if dark { 0x78aeedff } else { 0x3584e4ff },
        }
    }

    fn breeze(dark: bool) -> Self {
        let none = Shadow {
            x: 0.0,
            y: 0.0,
            blur: 0.0,
            spread: 0.0,
            color: 0,
        };
        let (window_bg, fg, header_bg, header_bg_unfocused) = if dark {
            (0x202326ff, 0xfcfcfcff, 0x2a2e32ff, 0x202326ff)
        } else {
            (0xeff0f1ff, 0x232629ff, 0xdee0e2ff, 0xeff0f1ff)
        };
        Self {
            preset: Preset::BreezeLike,
            dark,
            window_radius: 5.0,
            shadow_inset: 32.0,
            shadow_focused: [
                Shadow {
                    x: 0.0,
                    y: 6.0,
                    blur: 24.0,
                    spread: 2.0,
                    color: 0x00000040,
                },
                none,
                none,
            ],
            shadow_unfocused: [
                Shadow {
                    x: 0.0,
                    y: 4.0,
                    blur: 12.0,
                    spread: 0.0,
                    color: 0x00000026,
                },
                none,
                none,
            ],
            outline: with_alpha(fg, 0x33),
            window_bg,
            view_bg: if dark { 0x1b1e20ff } else { 0xffffffff },
            sidebar_bg: window_bg,
            fg,
            fg_dim: with_alpha(fg, 0x99),
            header_height: 30.0,
            header_bg,
            header_bg_unfocused,
            header_shade: with_alpha(fg, 0x26),
            title_weight: 400,
            title_size: 13.3,
            button_style: ButtonStyle::Flat,
            button_size: 18.0,
            button_icon_size: 16.0,
            button_gap: 6.0,
            button_bg: 0x00000000,
            button_bg_hover: with_alpha(fg, 0x33),
            button_bg_active: with_alpha(fg, 0x55),
            close_bg_hover: 0xda4453ff,
            close_fg_hover: 0xffffffff,
            accent: 0x3daee9ff,
        }
    }
}

/// Replaces the alpha byte of `0xRRGGBBAA`.
pub const fn with_alpha(rgba: u32, alpha: u8) -> u32 {
    (rgba & 0xffffff00) | alpha as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_alpha_replaces_only_alpha() {
        assert_eq!(with_alpha(0x112233ff, 0x80), 0x11223380);
    }

    #[test]
    fn shadow_fits_in_inset() {
        for preset in [Preset::AdwaitaLike, Preset::BreezeLike] {
            for dark in [false, true] {
                let t = ChromeTokens::new(preset, dark);
                for s in t.shadow_focused.iter().chain(&t.shadow_unfocused) {
                    // Horizontal reach of a layer: spread + blur (+ x offset).
                    assert!(
                        s.x.abs() + s.spread + s.blur <= t.shadow_inset,
                        "{preset:?} shadow {s:?} exceeds inset {}",
                        t.shadow_inset
                    );
                }
                assert!(t.shadow_inset >= crate::geometry::RESIZE_HANDLE);
            }
        }
    }
}
