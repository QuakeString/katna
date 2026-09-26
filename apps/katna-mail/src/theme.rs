// SPDX-License-Identifier: GPL-3.0-or-later

//! Colors of the mail window. The layout follows the familiar webmail look
//! (tinted page, white cards, pill-shaped navigation); the window frame
//! still follows the desktop (`katna-chrome`). No GPUI types here.

/// Colors as `0xRRGGBBAA`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Theme {
    pub dark: bool,
    /// Behind the cards: top bar and navigation.
    pub page: u32,
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
    pub star: u32,
    pub chip: u32,
    pub snackbar: u32,
    pub snackbar_text: u32,
    /// Shadow color; its alpha is the strongest shadow.
    pub shadow: u32,
}

impl Theme {
    pub fn new(dark: bool) -> Self {
        if dark { DARK } else { LIGHT }
    }
}

const LIGHT: Theme = Theme {
    dark: false,
    page: 0xf6f8fcff,
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
    star: 0xf4b400ff,
    chip: 0xe1e3e1ff,
    snackbar: 0x313033ff,
    snackbar_text: 0xf4eff4ff,
    shadow: 0x3c40434d,
};

const DARK: Theme = Theme {
    dark: true,
    page: 0x131416ff,
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
    star: 0xfdd663ff,
    chip: 0x3c3f43ff,
    snackbar: 0xe3e3e3ff,
    snackbar_text: 0x1f1f1fff,
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
    fn avatars() {
        assert_eq!(avatar_color("Kay@Enron.com"), avatar_color("kay@enron.com"));
        assert_eq!(initial("  kay mann"), "K");
        assert_eq!(initial("\"Ölaf\""), "Ö");
        assert_eq!(initial("--"), "?");
    }
}
