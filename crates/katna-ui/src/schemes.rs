// SPDX-License-Identifier: GPL-3.0-or-later

//! The color schemes built into Katna (Katna Mail's Settings > Appearance >
//! Colors). Each has a light and a dark side, so every scheme works with
//! every mode. Katna's own palette is not here: it is hand-tuned in
//! Katna Mail's `theme.rs` (`Theme::new`). Besides Katna's own (Katna,
//! Clear, Graphite), the palettes are those of popular editor and desktop
//! themes, all published under the MIT licence; the About page credits
//! them. No GPUI types here: Katna Setup draws its window in them too.

/// The id of the desktop's scheme.
pub const SYSTEM: &str = "system";
/// The id of Katna's own palette.
pub const KATNA: &str = "katna";

/// The colors a scheme is made from; Katna Mail's `Theme::from_scheme`
/// works out the rest. `0xRRGGBBAA`, opaque.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Side {
    /// Behind the cards: top bar and folder pane.
    pub page: u32,
    /// The list and reading cards.
    pub card: u32,
    pub text: u32,
    /// Dates, hints and other secondary text.
    pub faint: u32,
    pub accent: u32,
    pub error: u32,
}

impl Side {
    const fn light(page: u32, card: u32, text: u32, faint: u32, accent: u32) -> Self {
        Self {
            page,
            card,
            text,
            faint,
            accent,
            error: 0xc5221fff,
        }
    }

    const fn dark(page: u32, card: u32, text: u32, faint: u32, accent: u32) -> Self {
        Self {
            page,
            card,
            text,
            faint,
            accent,
            error: 0xf28b82ff,
        }
    }
}

/// A built-in scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuiltIn {
    /// Its id in the settings file.
    pub id: &'static str,
    /// The id of its name in `settings.ftl`.
    pub name: &'static str,
    pub light: Side,
    pub dark: Side,
}

impl BuiltIn {
    pub fn side(&self, dark: bool) -> &Side {
        if dark { &self.dark } else { &self.light }
    }
}

/// The built-in schemes after Katna's own, in the order Settings lists
/// them.
pub const BUILT_IN: &[BuiltIn] = &[
    // Apple's system colors: grey page, white cards, Apple blue.
    BuiltIn {
        id: "clear",
        name: "scheme-clear",
        light: Side::light(0xf2f2f7ff, 0xffffffff, 0x1d1d1fff, 0x6e6e73ff, 0x007affff),
        dark: Side::dark(0x161618ff, 0x1c1c1eff, 0xf5f5f7ff, 0x98989dff, 0x0a84ffff),
    },
    BuiltIn {
        id: "graphite",
        name: "scheme-graphite",
        light: Side::light(0xf1f1f2ff, 0xffffffff, 0x202124ff, 0x6b6d72ff, 0x4a4f57ff),
        dark: Side::dark(0x161719ff, 0x202225ff, 0xe6e6e7ff, 0x9b9da2ff, 0xb5bac2ff),
    },
    // Nord's Snow Storm and Polar Night.
    BuiltIn {
        id: "nord",
        name: "scheme-nord",
        light: Side::light(0xe5e9f0ff, 0xeceff4ff, 0x2e3440ff, 0x4c566aff, 0x5e81acff),
        dark: Side::dark(0x2e3440ff, 0x3b4252ff, 0xeceff4ff, 0xa3abb9ff, 0x88c0d0ff),
    },
    BuiltIn {
        id: "solarized",
        name: "scheme-solarized",
        light: Side {
            error: 0xdc322fff,
            ..Side::light(0xeee8d5ff, 0xfdf6e3ff, 0x073642ff, 0x657b83ff, 0x268bd2ff)
        },
        dark: Side {
            error: 0xdc322fff,
            ..Side::dark(0x00212bff, 0x002b36ff, 0xeee8d5ff, 0x93a1a1ff, 0x2aa198ff)
        },
    },
    // Dracula, with Alucard as its light side.
    BuiltIn {
        id: "dracula",
        name: "scheme-dracula",
        light: Side::light(0xefeddcff, 0xfffbebff, 0x1f1f1fff, 0x635d97ff, 0x644ac9ff),
        dark: Side {
            error: 0xff5555ff,
            ..Side::dark(0x21222cff, 0x282a36ff, 0xf8f8f2ff, 0x9ea7ccff, 0xbd93f9ff)
        },
    },
    BuiltIn {
        id: "gruvbox",
        name: "scheme-gruvbox",
        light: Side {
            error: 0x9d0006ff,
            ..Side::light(0xf2e5bcff, 0xfbf1c7ff, 0x3c3836ff, 0x7c6f64ff, 0xaf3a03ff)
        },
        dark: Side {
            error: 0xfb4934ff,
            ..Side::dark(0x1d2021ff, 0x282828ff, 0xebdbb2ff, 0xa89984ff, 0xfabd2fff)
        },
    },
    // Catppuccin Latte and Mocha.
    BuiltIn {
        id: "catppuccin",
        name: "scheme-catppuccin",
        light: Side {
            error: 0xd20f39ff,
            ..Side::light(0xe6e9efff, 0xeff1f5ff, 0x4c4f69ff, 0x6c6f85ff, 0x8839efff)
        },
        dark: Side {
            error: 0xf38ba8ff,
            ..Side::dark(0x181825ff, 0x1e1e2eff, 0xcdd6f4ff, 0xa6adc8ff, 0xcba6f7ff)
        },
    },
    // Tokyo Night Day and Night.
    BuiltIn {
        id: "tokyo-night",
        name: "scheme-tokyo-night",
        light: Side::light(0xd0d5e3ff, 0xe1e2e7ff, 0x3760bfff, 0x6172b0ff, 0x2e7de9ff),
        dark: Side {
            error: 0xf7768eff,
            ..Side::dark(0x16161eff, 0x1a1b26ff, 0xc0caf5ff, 0x9aa5ceff, 0x7aa2f7ff)
        },
    },
    // Atom's One Light and One Dark.
    BuiltIn {
        id: "one",
        name: "scheme-one",
        light: Side {
            error: 0xe45649ff,
            ..Side::light(0xeaeaebff, 0xfafafaff, 0x383a42ff, 0x696c77ff, 0x4078f2ff)
        },
        dark: Side {
            error: 0xe06c75ff,
            ..Side::dark(0x21252bff, 0x282c34ff, 0xabb2bfff, 0x8b919dff, 0x61afefff)
        },
    },
    // Rosé Pine Dawn and Main.
    BuiltIn {
        id: "rose-pine",
        name: "scheme-rose-pine",
        light: Side {
            error: 0xb4637aff,
            ..Side::light(0xf2e9e1ff, 0xfffaf3ff, 0x575279ff, 0x797593ff, 0x907aa9ff)
        },
        dark: Side {
            error: 0xeb6f92ff,
            ..Side::dark(0x191724ff, 0x1f1d2eff, 0xe0def4ff, 0x908caaff, 0xebbcbaff)
        },
    },
    BuiltIn {
        id: "everforest",
        name: "scheme-everforest",
        light: Side {
            error: 0xf85552ff,
            ..Side::light(0xefebd4ff, 0xfdf6e3ff, 0x5c6a72ff, 0x829181ff, 0x8da101ff)
        },
        dark: Side {
            error: 0xe67e80ff,
            ..Side::dark(0x232a2eff, 0x2d353bff, 0xd3c6aaff, 0x9da9a0ff, 0xa7c080ff)
        },
    },
    // Kanagawa Lotus and Wave.
    BuiltIn {
        id: "kanagawa",
        name: "scheme-kanagawa",
        light: Side {
            error: 0xc84053ff,
            ..Side::light(0xe5ddb0ff, 0xf2ecbcff, 0x545464ff, 0x716e61ff, 0x4d699bff)
        },
        dark: Side {
            error: 0xe82424ff,
            ..Side::dark(0x16161dff, 0x1f1f28ff, 0xdcd7baff, 0xa09f93ff, 0x7e9cd8ff)
        },
    },
    BuiltIn {
        id: "ayu",
        name: "scheme-ayu",
        light: Side {
            error: 0xe65050ff,
            ..Side::light(0xf3f4f5ff, 0xfcfcfcff, 0x5c6166ff, 0x8a9199ff, 0xfa8d3eff)
        },
        dark: Side {
            error: 0xd95757ff,
            ..Side::dark(0x0b0e14ff, 0x0d1017ff, 0xbfbdb6ff, 0x7a828eff, 0xe6b450ff)
        },
    },
];

/// The built-in scheme with `id`.
pub fn built_in(id: &str) -> Option<&'static BuiltIn> {
    BUILT_IN.iter().find(|s| s.id == id)
}
