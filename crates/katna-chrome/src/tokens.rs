// SPDX-License-Identifier: GPL-3.0-or-later

//! Theme tokens for the window chrome (`docs/ARCHITECTURE.md` §13.2).
//!
//! Colors are `0xRRGGBBAA`. Values are approximations of libadwaita 1.5 and
//! Breeze (Plasma 6), not copies of their stylesheets. Apps that read the
//! desktop's color scheme (`katna_platform::colors`) redraw them in it with
//! [`ChromeTokens::recolored`].

use crate::desktop::Preset;

/// One box shadow layer, CSS order: x, y, blur, spread, color. As in CSS,
/// `blur` is twice the Gaussian's standard deviation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shadow {
    pub x: f32,
    pub y: f32,
    pub blur: f32,
    pub spread: f32,
    pub color: u32,
}

impl Shadow {
    /// How far past the window's edge the shadow reaches before it has
    /// faded out: three standard deviations of its blur, plus its spread and
    /// offset.
    pub fn reach(&self) -> f32 {
        1.5 * self.blur + self.spread + self.x.abs().max(self.y.abs())
    }
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
                blur: 16.0,
                spread: 4.0,
                color: 0x00000017,
            },
            Shadow {
                x: 0.0,
                y: 4.0,
                blur: 24.0,
                spread: 4.0,
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
            // Every layer fades out inside it (`shadows_fade_inside_the_margin`).
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
                    y: 3.0,
                    blur: 16.0,
                    spread: 0.0,
                    color: 0x00000040,
                },
                none,
                none,
            ],
            shadow_unfocused: [
                Shadow {
                    x: 0.0,
                    y: 2.0,
                    blur: 10.0,
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

/// Desktop colors that replace a preset's own (`0xRRGGBBAA`, opaque).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChromeColors {
    pub window_bg: u32,
    pub view_bg: u32,
    pub fg: u32,
    pub accent: u32,
}

impl ChromeTokens {
    /// The tokens drawn in `colors`, the desktop's color scheme. Shapes,
    /// sizes and shadows stay the preset's.
    pub fn recolored(self, colors: &ChromeColors) -> Self {
        let fg = colors.fg;
        let (button_bg, button_bg_hover, button_bg_active, close_bg_hover, close_fg_hover) =
            match self.button_style {
                ButtonStyle::Circle => (
                    with_alpha(fg, 0x1a),
                    with_alpha(fg, 0x26),
                    with_alpha(fg, 0x40),
                    with_alpha(fg, 0x26),
                    fg,
                ),
                ButtonStyle::Flat => (
                    0x00000000,
                    with_alpha(fg, 0x33),
                    with_alpha(fg, 0x55),
                    self.close_bg_hover,
                    self.close_fg_hover,
                ),
            };
        Self {
            outline: with_alpha(fg, if self.dark { 0x12 } else { 0x24 }),
            window_bg: colors.window_bg,
            view_bg: colors.view_bg,
            sidebar_bg: colors.window_bg,
            fg,
            fg_dim: with_alpha(fg, 0x99),
            header_bg: colors.window_bg,
            header_bg_unfocused: colors.window_bg,
            header_shade: with_alpha(fg, 0x26),
            button_bg,
            button_bg_hover,
            button_bg_active,
            close_bg_hover,
            close_fg_hover,
            accent: colors.accent,
            ..self
        }
    }
}

/// How opaque the window background is when the compositor blurs what is
/// behind it: enough to keep text readable over any wallpaper, while the
/// blur shows through.
pub fn blur_alpha(dark: bool) -> u8 {
    if dark { 0xcc } else { 0xbf }
}

impl ChromeTokens {
    /// The tokens of a translucent, blurred window: the window and header
    /// backgrounds let [`blur_alpha`] of the blur through.
    pub fn translucent(self) -> Self {
        let alpha = blur_alpha(self.dark);
        Self {
            window_bg: with_alpha(self.window_bg, alpha),
            header_bg: with_alpha(self.header_bg, alpha),
            header_bg_unfocused: with_alpha(self.header_bg_unfocused, alpha),
            sidebar_bg: with_alpha(self.sidebar_bg, alpha),
            ..self
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

    /// A shadow cut off by the edge of the surface shows as a hard line
    /// around the window.
    #[test]
    fn shadows_fade_inside_the_margin() {
        for preset in [Preset::AdwaitaLike, Preset::BreezeLike] {
            for dark in [false, true] {
                let t = ChromeTokens::new(preset, dark);
                for s in t.shadow_focused.iter().chain(&t.shadow_unfocused) {
                    assert!(s.reach() <= t.shadow_inset, "{preset:?}: {s:?}");
                }
            }
        }
    }

    #[test]
    fn recolored_keeps_shapes() {
        let colors = ChromeColors {
            window_bg: 0x303446ff,
            view_bg: 0x292c3cff,
            fg: 0xc6d0f5ff,
            accent: 0x8caaeeff,
        };
        for preset in [Preset::AdwaitaLike, Preset::BreezeLike] {
            let t = ChromeTokens::new(preset, true);
            let r = t.clone().recolored(&colors);
            assert_eq!(r.window_bg, 0x303446ff);
            assert_eq!(r.header_bg, 0x303446ff);
            assert_eq!(r.accent, 0x8caaeeff);
            assert_eq!(r.window_radius, t.window_radius);
            assert_eq!(r.shadow_focused, t.shadow_focused);
        }
    }

    #[test]
    fn translucent_keeps_colors() {
        let t = ChromeTokens::new(Preset::BreezeLike, false);
        let r = t.clone().translucent();
        assert_eq!(r.window_bg >> 8, t.window_bg >> 8);
        assert_eq!(r.window_bg & 0xff, u32::from(blur_alpha(false)));
        assert_eq!(r.view_bg, t.view_bg);
    }

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
