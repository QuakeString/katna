// SPDX-License-Identifier: GPL-3.0-or-later

//! Colors of the mail window. The layout follows the familiar webmail look
//! (tinted page, white cards, pill-shaped navigation). The colors come from
//! the desktop's color scheme and accent color when it has them
//! (`katna_platform::colors`), else from Katna's own palettes below. No GPUI
//! types here.

use katna_platform::colors::{Scheme, SystemColors, contrast, luminance, over};

/// Colors as `0xRRGGBBAA`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Theme {
    pub dark: bool,
    /// Behind the cards: top bar and navigation.
    pub page: u32,
    /// What the window paints behind everything: `page`, or nothing when
    /// the window frame paints a translucent `page` for the compositor's
    /// blur ([`Theme::translucent`]).
    pub backdrop: u32,
    /// The list and reading cards, and unread rows.
    pub surface: u32,
    /// Rows of read mail.
    pub read_row: u32,
    pub text: u32,
    pub text_dim: u32,
    pub text_faint: u32,
    pub divider: u32,
    /// Laid over an element under the pointer.
    pub hover: u32,
    /// The ink of click ripples.
    pub ripple: u32,
    pub nav_selected: u32,
    pub nav_selected_text: u32,
    pub compose: u32,
    pub compose_text: u32,
    pub search: u32,
    pub search_focused: u32,
    pub accent: u32,
    /// Text and icons on `accent`.
    pub on_accent: u32,
    pub star: u32,
    /// Rows the user ticked.
    pub checked_row: u32,
    /// Menus and dropdowns.
    pub menu: u32,
    pub switch_off: u32,
    /// Category tab colors: primary, promotions, social, updates, forums.
    pub tabs: [u32; 5],
    pub chip: u32,
    pub snackbar: u32,
    pub snackbar_text: u32,
    /// Error text and the frame of a field in error.
    pub error: u32,
    /// Shadow color; its alpha is the strongest shadow.
    pub shadow: u32,
}

impl Theme {
    /// Katna's own palette.
    pub fn new(dark: bool) -> Self {
        if dark { DARK } else { LIGHT }
    }

    /// For a blurred window: the frame paints the page's color, translucent
    /// (`katna_chrome::WindowChrome::blurred`); the window paints no
    /// backdrop over it. Cards and menus stay opaque, so text stays
    /// readable.
    pub fn translucent(self) -> Self {
        Self {
            backdrop: 0x00000000,
            ..self
        }
    }

    /// The desktop's color scheme when it is as dark as `dark` asks, else
    /// Katna's palette in the desktop's accent color (if it has one).
    pub fn system(dark: bool, colors: &SystemColors) -> Self {
        match colors.scheme_for(dark) {
            Some(scheme) => Self::from_scheme(&scheme),
            None => colors.accent.map_or_else(
                || Self::new(dark),
                |accent| Self::new(dark).with_accent(accent),
            ),
        }
    }

    /// Katna's palette with its blues replaced by tones of `accent`.
    pub fn with_accent(self, accent: u32) -> Self {
        let accent = readable(opaque(accent), self.surface, 3.0);
        let (selected, selected_text, compose, checked) = if self.dark {
            (
                tone(accent, 0.24),
                tone(accent, 0.88),
                tone(accent, 0.24),
                tone(accent, 0.24),
            )
        } else {
            (
                tone(accent, 0.91),
                tone(accent, 0.14),
                tone(accent, 0.86),
                tone(accent, 0.87),
            )
        };
        let mut tabs = self.tabs;
        tabs[0] = accent;
        Self {
            accent,
            on_accent: on(accent),
            nav_selected: selected,
            nav_selected_text: selected_text,
            compose,
            compose_text: selected_text,
            checked_row: checked,
            tabs,
            ..self
        }
    }

    /// The webmail look drawn in a desktop color scheme: the page in the
    /// window color, the cards in the view color, highlights in tints of
    /// the accent color.
    pub fn from_scheme(s: &Scheme) -> Self {
        let dark = s.dark();
        let base = Self::new(dark);
        let (page, surface, text) = (s.window_bg, s.view_bg, s.view_fg);
        let accent = readable(s.accent, surface, 3.0);
        // A few percent of the text color over the card.
        let ink = |alpha: f32| over(fade(text, alpha), surface);
        let mut tabs = base.tabs;
        tabs[0] = accent;
        Self {
            dark,
            page,
            backdrop: page,
            surface,
            read_row: mix(surface, page, 0.9),
            text,
            text_dim: mix(text, surface, 0.18),
            text_faint: readable(s.inactive_fg, surface, 3.0),
            divider: fade(text, 0.14),
            hover: fade(text, if dark { 0.08 } else { 0.07 }),
            ripple: fade(text, if dark { 0.16 } else { 0.14 }),
            nav_selected: mix(page, accent, if dark { 0.34 } else { 0.22 }),
            nav_selected_text: text,
            compose: mix(page, accent, if dark { 0.34 } else { 0.28 }),
            compose_text: text,
            search: over(fade(s.window_fg, if dark { 0.08 } else { 0.06 }), page),
            search_focused: if dark { ink(0.1) } else { surface },
            accent,
            on_accent: if contrast(s.accent_fg, accent) >= 3.0 {
                s.accent_fg
            } else {
                on(accent)
            },
            star: base.star,
            checked_row: mix(surface, accent, if dark { 0.3 } else { 0.2 }),
            menu: if dark { ink(0.06) } else { surface },
            switch_off: ink(0.18),
            tabs,
            chip: ink(0.1),
            snackbar: base.snackbar,
            snackbar_text: base.snackbar_text,
            error: readable(s.negative, surface, 3.0),
            shadow: base.shadow,
        }
    }
}

fn opaque(color: u32) -> u32 {
    color | 0xff
}

/// Black or white, whichever reads better on `color`.
fn on(color: u32) -> u32 {
    if contrast(color, 0x000000ff) > contrast(color, 0xffffffff) {
        0x000000ff
    } else {
        0xffffffff
    }
}

/// `color`, darkened or lightened away from `bg` until its contrast with
/// `bg` is at least `min`.
fn readable(color: u32, bg: u32, min: f32) -> u32 {
    let toward = if luminance(bg) > 0.18 {
        0x000000ff
    } else {
        0xffffffff
    };
    (0..=20)
        .map(|step| mix(color, toward, step as f32 * 0.05))
        .find(|c| contrast(*c, bg) >= min)
        .unwrap_or(toward)
}

/// `color` at HSL lightness `l`, keeping its hue and (for pale tints, a
/// little less of) its saturation.
fn tone(color: u32, l: f32) -> u32 {
    let [r, g, b] = [24, 16, 8].map(|s| ((color >> s) & 0xff) as f32 / 255.0);
    let (max, min) = (r.max(g).max(b), r.min(g).min(b));
    let (h, s) = if max == min {
        (0.0, 0.0)
    } else {
        let d = max - min;
        let lum = (max + min) / 2.0;
        let s = if lum > 0.5 {
            d / (2.0 - max - min)
        } else {
            d / (max + min)
        };
        let h = if max == r {
            (g - b) / d + if g < b { 6.0 } else { 0.0 }
        } else if max == g {
            (b - r) / d + 2.0
        } else {
            (r - g) / d + 4.0
        };
        (h / 6.0, s)
    };
    let s = s.min(0.9);
    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;
    let hue = |t: f32| {
        let t = t.rem_euclid(1.0);
        if t < 1.0 / 6.0 {
            p + (q - p) * 6.0 * t
        } else if t < 0.5 {
            q
        } else if t < 2.0 / 3.0 {
            p + (q - p) * (2.0 / 3.0 - t) * 6.0
        } else {
            p
        }
    };
    let byte = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u32;
    byte(hue(h + 1.0 / 3.0)) << 24 | byte(hue(h)) << 16 | byte(hue(h - 1.0 / 3.0)) << 8 | 0xff
}

const LIGHT: Theme = Theme {
    dark: false,
    page: 0xf6f8fcff,
    backdrop: 0xf6f8fcff,
    surface: 0xffffffff,
    read_row: 0xf2f6fcff,
    text: 0x1f1f1fff,
    text_dim: 0x444746ff,
    text_faint: 0x5e6368ff,
    divider: 0x64798f24,
    hover: 0x1f1f1f12,
    ripple: 0x1f1f1f24,
    nav_selected: 0xd3e3fdff,
    nav_selected_text: 0x041e49ff,
    compose: 0xc2e7ffff,
    compose_text: 0x001d35ff,
    search: 0xe9eef6ff,
    search_focused: 0xffffffff,
    accent: 0x0b57d0ff,
    on_accent: 0xffffffff,
    star: 0xf4b400ff,
    checked_row: 0xc2dbffff,
    menu: 0xffffffff,
    switch_off: 0xe1e3e1ff,
    tabs: [0x0b57d0ff, 0x188038ff, 0x1a73e8ff, 0xe37400ff, 0x9334e6ff],
    chip: 0xe1e3e1ff,
    snackbar: 0x313033ff,
    snackbar_text: 0xf4eff4ff,
    error: 0xb3261eff,
    shadow: 0x3c40434d,
};

const DARK: Theme = Theme {
    dark: true,
    page: 0x131416ff,
    backdrop: 0x131416ff,
    surface: 0x1f2124ff,
    read_row: 0x191b1eff,
    text: 0xe3e3e3ff,
    text_dim: 0xc4c7c5ff,
    text_faint: 0x9aa0a6ff,
    divider: 0xffffff17,
    hover: 0xffffff14,
    ripple: 0xffffff29,
    nav_selected: 0x004a77ff,
    nav_selected_text: 0xc2e7ffff,
    compose: 0x004a77ff,
    compose_text: 0xc2e7ffff,
    search: 0x2a2d31ff,
    search_focused: 0x383b40ff,
    accent: 0xa8c7faff,
    on_accent: 0x062e6fff,
    star: 0xfdd663ff,
    checked_row: 0x004a77ff,
    menu: 0x2d2f33ff,
    switch_off: 0x44474eff,
    tabs: [0xa8c7faff, 0x81c995ff, 0x8ab4f8ff, 0xfcad70ff, 0xd7aefbff],
    chip: 0x3c3f43ff,
    snackbar: 0xe3e3e3ff,
    snackbar_text: 0x1f1f1fff,
    error: 0xf2b8b5ff,
    shadow: 0x00000099,
};

/// Mixes two `0xRRGGBBAA` colors: `t` = 0 gives `a`, 1 gives `b`.
pub fn mix(a: u32, b: u32, t: f32) -> u32 {
    let t = t.clamp(0.0, 1.0);
    (0..4).fold(0, |out, i| {
        let shift = 24 - 8 * i;
        let ca = ((a >> shift) & 0xff) as f32;
        let cb = ((b >> shift) & 0xff) as f32;
        out | (((ca + (cb - ca) * t).round() as u32) << shift)
    })
}

/// `color` with its alpha scaled by `t` (0..=1).
pub fn fade(color: u32, t: f32) -> u32 {
    let alpha = ((color & 0xff) as f32 * t.clamp(0.0, 1.0)).round() as u32;
    (color & 0xffff_ff00) | alpha
}

/// Background colors for letter avatars, readable with white text.
const AVATARS: [u32; 8] = [
    0x1a73e8ff, 0xd93025ff, 0x188038ff, 0xe37400ff, 0x9334e6ff, 0x007b83ff, 0xc5221fff, 0x5f6368ff,
];

/// The avatar color for an address: stable for the same address.
pub fn avatar_color(address: &str) -> u32 {
    // FNV-1a, so the color does not change between runs or versions.
    let hash = address
        .to_lowercase()
        .bytes()
        .fold(0xcbf2_9ce4_8422_2325_u64, |h, b| {
            (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3)
        });
    AVATARS[(hash % AVATARS.len() as u64) as usize]
}

/// The letter on an avatar: the first letter or digit of the name.
pub fn initial(name: &str) -> String {
    name.chars()
        .find(|c| c.is_alphanumeric())
        .map_or_else(|| "?".to_owned(), |c| c.to_uppercase().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixes_colors() {
        assert_eq!(mix(0x000000ff, 0xffffffff, 0.0), 0x000000ff);
        assert_eq!(mix(0x000000ff, 0xffffffff, 1.0), 0xffffffff);
        assert_eq!(mix(0x00000000, 0x6400c8ff, 0.5), 0x32006480);
        assert_eq!(mix(0x10203040, 0x50607080, 2.0), 0x50607080);
    }

    #[test]
    fn fades_alpha() {
        assert_eq!(fade(0x11223380, 0.5), 0x11223340);
        assert_eq!(fade(0x112233ff, 0.0), 0x11223300);
    }

    #[test]
    fn tones_keep_the_hue() {
        // Katna's own blue, as a pale and a deep tone.
        let pale = tone(0x0b57d0ff, 0.91);
        assert!(luminance(pale) > 0.7, "{pale:08x}");
        assert!((pale >> 8 & 0xff) > (pale >> 24), "still blue: {pale:08x}");
        assert!(luminance(tone(0x0b57d0ff, 0.14)) < 0.05);
        // Grey stays grey.
        assert_eq!(tone(0x808080ff, 0.5), 0x808080ff);
    }

    #[test]
    fn readable_colors() {
        // A bright yellow on white is too faint; it is darkened.
        let yellow = readable(0xf6d32dff, 0xffffffff, 3.0);
        assert!(contrast(yellow, 0xffffffff) >= 3.0);
        assert_ne!(yellow, 0xf6d32dff);
        assert_eq!(readable(0x0b57d0ff, 0xffffffff, 3.0), 0x0b57d0ff);
        assert_eq!(on(0xffff00ff), 0x000000ff);
        assert_eq!(on(0x0b57d0ff), 0xffffffff);
    }

    #[test]
    fn system_colors() {
        use katna_platform::colors::{adwaita, parse_kdeglobals};

        // No scheme and no accent: Katna's palette.
        assert_eq!(Theme::system(false, &SystemColors::default()), LIGHT);
        // Only an accent: Katna's palette in that color.
        let accent_only = SystemColors::accent_only(Some(0xe62d42ff));
        let th = Theme::system(false, &accent_only);
        assert_eq!(th.page, LIGHT.page);
        assert_eq!(th.accent, 0xe62d42ff);
        assert_ne!(th.nav_selected, LIGHT.nav_selected);

        // A KDE scheme: its window and view colors.
        let (breeze_dark, _) = parse_kdeglobals(
            "[Colors:Window]\nBackgroundNormal=32,35,38\nForegroundNormal=252,252,252\n\
             [Colors:View]\nBackgroundNormal=20,22,24\nForegroundNormal=252,252,252\n",
        );
        let kde = SystemColors::kde(breeze_dark);
        let th = Theme::system(true, &kde);
        assert!(th.dark);
        assert_eq!(th.page, 0x202326ff);
        assert_eq!(th.surface, 0x141618ff);
        assert_eq!(th.text, 0xfcfcfcff);
        assert_eq!(th.accent, 0x3daee9ff);
        // Asked for light, a dark scheme is left out; its accent stays.
        let th = Theme::system(false, &kde);
        assert_eq!(th.page, LIGHT.page);
        assert_ne!(th.accent, LIGHT.accent);

        // Every scheme keeps text readable.
        for scheme in [
            adwaita(false, None, &[]),
            adwaita(true, Some(0xc88800ff), &[]),
        ] {
            let th = Theme::from_scheme(&scheme);
            assert!(contrast(th.text, th.surface) >= 4.5);
            assert!(contrast(th.text_faint, th.surface) >= 3.0);
            assert!(contrast(th.accent, th.surface) >= 3.0);
            assert!(contrast(th.on_accent, th.accent) >= 3.0);
            assert!(contrast(th.nav_selected_text, th.nav_selected) >= 4.5);
        }
    }

    #[test]
    fn avatars() {
        assert_eq!(avatar_color("Kay@Enron.com"), avatar_color("kay@enron.com"));
        assert_eq!(initial("  kay mann"), "K");
        assert_eq!(initial("\"Ölaf\""), "Ö");
        assert_eq!(initial("--"), "?");
    }
}
