// SPDX-License-Identifier: GPL-3.0-or-later

//! Colors of the mail window. The layout follows the familiar webmail look
//! (tinted page, white cards, pill-shaped navigation). The colors come from
//! the desktop's color scheme and accent color when it has them
//! (`katna_platform::colors`), else from Katna's own palettes below. No GPUI
//! types here.

use katna_platform::colors::{Scheme, SystemColors, contrast, luminance, over};

use crate::schemes::{self, SideScheme};

/// The accent color the settings pick (Settings > Appearance > Accent).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Accent {
    /// The color scheme's own.
    Scheme,
    /// The desktop's, else the scheme's own.
    System,
    /// `0xRRGGBBAA`.
    Color(u32),
}

impl Accent {
    /// Reads the `accent` setting: empty, `system` or `#rrggbb`. Anything
    /// else is the scheme's own.
    pub fn parse(setting: &str) -> Self {
        match setting {
            "system" => Self::System,
            hex => hex
                .strip_prefix('#')
                .filter(|digits| digits.len() == 6)
                .and_then(|digits| u32::from_str_radix(digits, 16).ok())
                .map_or(Self::Scheme, |rgb| Self::Color(rgb << 8 | 0xff)),
        }
    }

    /// The `accent` setting for this choice.
    pub fn setting(self) -> String {
        match self {
            Self::Scheme => String::new(),
            Self::System => "system".to_owned(),
            Self::Color(color) => format!("#{:06x}", color >> 8),
        }
    }
}

/// How far from the text colour toward the card dim text goes over
/// see-through cards: near enough that previews keep 4.5:1 at 70 % over a
/// bright wallpaper.
const FROSTED_DIM: f32 = 0.2;
/// The same for faint text: hints keep about 3:1.
const FROSTED_FAINT: f32 = 0.38;

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
    /// Lines between things: rows, toolbars, headers, sections. A quarter
    /// of [`Theme::outline`], so they stay quiet.
    pub divider: u32,
    /// The edge of a box: fields, chips, buttons, cards and quote bars.
    pub outline: u32,
    /// Laid over an element under the pointer.
    pub hover: u32,
    /// The ink of click ripples.
    pub ripple: u32,
    pub nav_selected: u32,
    pub nav_selected_text: u32,
    /// The open line of a side pane (folders, Contacts, Tasks, Notes,
    /// Files): a quiet grey, so the accent is left to Compose and the rail.
    pub row_selected: u32,
    pub row_selected_text: u32,
    /// A count pill on that open line: lighter than the line, so the
    /// count still reads as a pill on the grey.
    pub row_selected_pill: u32,
    pub compose: u32,
    pub compose_text: u32,
    pub search: u32,
    pub search_focused: u32,
    pub accent: u32,
    /// Text and icons on `accent`.
    pub on_accent: u32,
    pub star: u32,
    /// The Important marker when set.
    pub important: u32,
    /// Rows the user ticked.
    pub checked_row: u32,
    /// Menus and dropdowns. In dark colors a step lighter than
    /// [`Theme::raised`], as they sit highest.
    pub menu: u32,
    /// Things that float over the cards: floating buttons, dialogs,
    /// popovers. In light colors the card's white, lifted by its shadow; in
    /// dark colors, where a shadow barely shows, a step lighter than the
    /// cards.
    pub raised: u32,
    /// A faint light edge around raised things in dark colors
    /// (`widgets::elevation`); transparent in light colors.
    pub rim: u32,
    /// A crisp hairline ring around cards in light colors, where a white
    /// card on a near-white page has no edge of its own
    /// (`widgets::card_shadow`); transparent in dark colors, whose cards
    /// stand off the darker page by their fill.
    pub card_edge: u32,
    /// Floating panels (menus, popovers) are frosted glass: `menu`,
    /// translucent, over a blur of this many device pixels of what is
    /// behind. 0 keeps them opaque ([`Theme::frosted`]).
    pub frost: u32,
    /// How opaque the frost's tint is, in percent.
    pub frost_tint: u8,
    /// How opaque the cards are, in percent: 100, or less when a blurred
    /// window's blur shows through them ([`Theme::frosted_panes`]).
    pub pane_tint: u8,
    /// How opaque the open mail's card is while it shows a chat, in
    /// percent.
    pub chat_tint: u8,
    /// How opaque the search box is while it is open, in percent.
    pub search_tint: u8,
    pub switch_off: u32,
    /// Category tab colors: primary, promotions, social, updates, forums.
    pub tabs: [u32; 5],
    /// The folder pane's icons for special folders and views.
    pub folder_icons: FolderIcons,
    pub chip: u32,
    pub snackbar: u32,
    pub snackbar_text: u32,
    /// Error text and the frame of a field in error.
    pub error: u32,
    /// What needs the user before it gets better: an account signed out
    /// or refusing its password, a mail not sent. Amber, readable as
    /// text. Errors stay [`Theme::error`].
    pub warning: u32,
    /// Shadow color; its alpha is the strongest shadow.
    pub shadow: u32,
}

/// The colors of the folder pane's icons, each saying what its folder
/// holds; the inbox takes the accent, and Trash, Archive and the user's
/// own folders keep the dimmed text color.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FolderIcons {
    pub unread: u32,
    pub starred: u32,
    pub important: u32,
    pub sent: u32,
    pub all_mail: u32,
    pub spam: u32,
    pub drafts: u32,
    pub scheduled: u32,
}

impl Theme {
    /// The chat view's bubble for the user's own mail: a light tint of the
    /// accent.
    pub fn bubble_own(&self) -> u32 {
        mix(
            self.surface,
            self.accent,
            if self.dark { 0.14 } else { 0.10 },
        )
    }

    /// The chat view's bubble for other people's mail: a quiet grey a step
    /// off the card.
    pub fn bubble_other(&self) -> u32 {
        if self.dark {
            mix(self.surface, 0xffff_ffff, 0.06)
        } else {
            mix(self.surface, 0x0000_00ff, 0.05)
        }
    }

    /// A soft tray behind a group of tools (Compose's security toggles and
    /// its row of writing tools): half a chip.
    pub fn tray(&self) -> u32 {
        fade(self.chip, 0.5)
    }

    /// A recipient chip under the pointer: the chip a step deeper.
    pub fn chip_hover(&self) -> u32 {
        mix(self.chip, self.text, 0.08)
    }

    /// A meter nearly at its limit, before it turns to [`Theme::error`]:
    /// amber.
    pub fn caution(&self) -> u32 {
        if self.dark { 0xfdd663ff } else { 0xe37400ff }
    }

    /// What goes to Google Drive or OneDrive rather than in the mail: the
    /// clouds' own blue, whatever the accent.
    pub fn cloud(&self) -> u32 {
        if self.dark { 0x8ab4f8ff } else { 0x1a73e8ff }
    }

    /// A line `t` of the outline's strength, never stronger than
    /// [`Theme::divider`].
    pub fn faint_line(&self, t: f32) -> u32 {
        fade(self.outline, t.min(LINE))
    }

    /// Katna's own palette.
    pub fn new(dark: bool) -> Self {
        if dark { DARK } else { LIGHT }
    }

    /// For a blurred window: the frame paints the page's color, translucent
    /// (`katna_chrome::WindowChrome::blurred`); the window paints no
    /// backdrop over it. Cards stay opaque unless they are frosted too
    /// ([`Theme::frosted_panes`]); menus are frosted ([`Theme::frosted`]).
    pub fn translucent(self) -> Self {
        Self {
            backdrop: 0x00000000,
            ..self
        }
    }

    /// Frosted floating panels, blurring `radius` device pixels of what is
    /// behind them under a tint `tint` percent opaque (Settings >
    /// Experimental > Blur).
    pub fn frosted(self, radius: f32, tint: u8) -> Self {
        Self {
            frost: radius.round().max(1.0) as u32,
            frost_tint: tint.min(100),
            ..self
        }
    }

    /// For a blurred window: the cards (`pane`), the open mail's card
    /// while it shows a chat (`chat`) and the open search box (`search`)
    /// let the blur through, each this many percent opaque over the blurred
    /// desktop, or stay solid (`None`). Bubbles, menus and fields keep
    /// their own fills. Over see-through cards dim and faint text move
    /// closer to the text colour, so previews stay readable over a bright
    /// wallpaper.
    pub fn frosted_panes(self, pane: Option<u8>, chat: Option<u8>, search: Option<u8>) -> Self {
        let pane_tint = pane.map_or(100, |p| p.min(100));
        let see_through = pane_tint < 100 || chat.is_some_and(|c| c < 100);
        // The stronger of a colour and one `t` of the way from the text
        // to the card.
        let lift = |color: u32, t: f32| {
            let lifted = mix(self.text, self.surface, t);
            if contrast(lifted, self.surface) > contrast(color, self.surface) {
                lifted
            } else {
                color
            }
        };
        Self {
            text_dim: if see_through {
                lift(self.text_dim, FROSTED_DIM)
            } else {
                self.text_dim
            },
            text_faint: if see_through {
                lift(self.text_faint, FROSTED_FAINT)
            } else {
                self.text_faint
            },
            pane_tint,
            chat_tint: chat.map_or(pane_tint, |c| c.min(100)),
            search_tint: search.map_or(100, |s| s.min(100)),
            ..self
        }
    }

    /// The cards' fill: the mail list, the open mail, the person card and
    /// the pages.
    pub fn pane(&self) -> u32 {
        fade(self.surface, f32::from(self.pane_tint) / 100.0)
    }

    /// The open mail's card while it shows a chat.
    pub fn chat_pane(&self) -> u32 {
        fade(self.surface, f32::from(self.chat_tint) / 100.0)
    }

    /// `fill` laid on a card ([`Theme::pane`]), such as a row's: as it is
    /// on a solid card. On one the blur shows through, the card's own
    /// colour adds nothing and any other becomes the faintest tint that
    /// gives that colour over the solid card, so a read row shows the
    /// card's frost as an unread one does, only a shade darker.
    pub fn on_pane(&self, fill: u32) -> u32 {
        if self.pane_tint >= 100 {
            fill
        } else {
            tint_over(self.surface | 0xff, fill)
        }
    }

    /// The Settings page's list of pages beside the open page: a soft grey
    /// on the card, so it reads as its own menu. The selected row and the
    /// hover lay their own grey on top.
    pub fn side_menu(&self) -> u32 {
        self.on_pane(fade(self.text, 0.05))
    }

    /// For what is drawn on a raised surface (a dialog, the Compose
    /// window): its cards are [`Theme::raised`], and the fills measured
    /// from the card (fields, chips, switches) are lifted with it. The
    /// same theme in light colors, whose raised surfaces are the cards'.
    pub fn lifted(self) -> Self {
        if !self.dark || self.raised == self.surface {
            return self;
        }
        let ink = |alpha: f32| over(fade(self.text, alpha), self.raised);
        Self {
            surface: self.raised,
            read_row: self.raised,
            pane_tint: 100,
            chat_tint: 100,
            search_focused: ink(0.1),
            chip: ink(0.1),
            switch_off: ink(0.18),
            ..self
        }
    }

    /// The colors for `dark` in the color scheme with the id `colors`
    /// ([`schemes`], or one of the desktop's) and `accent`. `system` is
    /// the desktop's scheme in use ([`Theme::system`]); Katna's own
    /// palette, and schemes no longer there, are [`Theme::new`]. A scheme
    /// with one side draws that side: see [`Theme::forced_dark`].
    pub fn pick(dark: bool, colors: &str, accent: Accent, system: &SystemColors) -> Self {
        let accent = match accent {
            Accent::Scheme => None,
            Accent::System => system.accent,
            Accent::Color(color) => Some(color),
        };
        let with_accent = |mut scheme: Scheme| {
            if let Some(accent) = accent {
                scheme.accent = opaque(accent);
            }
            Self::from_scheme(&scheme)
        };
        if let Some(built_in) = schemes::built_in(colors) {
            return with_accent(built_in.side(dark).scheme(built_in.id));
        }
        if let Some(side) = system.scheme(colors).and_then(|s| s.side(dark)) {
            return with_accent(side.clone());
        }
        if colors == schemes::SYSTEM {
            return match (system.scheme_for(dark), accent) {
                (Some(scheme), Some(_)) => with_accent(scheme),
                (_, None) => Self::system(dark, system),
                (None, Some(accent)) => Self::new(dark).with_accent(accent),
            };
        }
        match accent {
            Some(accent) => Self::new(dark).with_accent(accent),
            None => Self::new(dark),
        }
    }

    /// Light or dark as the scheme `colors` decides when it has one side
    /// only (a Contrast theme, a KDE scheme without a partner), whatever
    /// the mode asks; `None` when the mode decides.
    pub fn forced_dark(colors: &str, system: &SystemColors) -> Option<bool> {
        if colors == schemes::SYSTEM {
            return system.forced_dark();
        }
        let scheme = system.scheme(colors)?;
        match (&scheme.light, &scheme.dark) {
            (Some(_), None) => Some(false),
            (None, Some(_)) => Some(true),
            _ => None,
        }
    }

    /// Whether [`Theme::pick`] draws a color scheme rather than Katna's
    /// palette, so the window frame takes its colors too.
    pub fn picks_scheme(dark: bool, colors: &str, system: &SystemColors) -> bool {
        schemes::built_in(colors).is_some()
            || system.scheme(colors).is_some()
            || (colors == schemes::SYSTEM && system.scheme_for(dark).is_some())
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
            divider: fade(text, 0.14 * LINE),
            outline: fade(text, 0.14),
            hover: fade(text, if dark { 0.08 } else { 0.07 }),
            ripple: fade(text, if dark { 0.16 } else { 0.14 }),
            nav_selected: mix(page, accent, if dark { 0.34 } else { 0.22 }),
            nav_selected_text: text,
            row_selected: fade(text, if dark { 0.12 } else { 0.10 }),
            row_selected_text: text,
            row_selected_pill: if dark {
                fade(text, 0.10)
            } else {
                fade(surface, 0.7)
            },
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
            important: base.important,
            checked_row: mix(surface, accent, if dark { 0.3 } else { 0.2 }),
            menu: if dark { ink(MENU_LIFT) } else { surface },
            raised: if dark { ink(RAISED_LIFT) } else { surface },
            rim: if dark { fade(text, RIM) } else { 0x00000000 },
            card_edge: if dark { 0x00000000 } else { base.card_edge },
            frost: 0,
            frost_tint: 100,
            pane_tint: 100,
            chat_tint: 100,
            search_tint: 100,
            switch_off: ink(0.18),
            tabs,
            folder_icons: base.folder_icons,
            chip: ink(0.1),
            snackbar: base.snackbar,
            snackbar_text: base.snackbar_text,
            error: readable(s.negative, surface, 3.0),
            warning: s
                .neutral
                .map_or(base.warning, |c| readable(c, surface, 4.5)),
            shadow: base.shadow,
        }
    }
}

/// How much of the text color lifts raised surfaces over the cards in
/// dark colors ([`Theme::raised`]), menus a step more ([`Theme::menu`]),
/// and how strong their light edge is ([`Theme::rim`]).
const RAISED_LIFT: f32 = 0.10;
const MENU_LIFT: f32 = 0.13;
const RIM: f32 = 0.13;

fn opaque(color: u32) -> u32 {
    color | 0xff
}

/// Black or white, whichever reads better on `color`.
pub fn on(color: u32) -> u32 {
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

/// How much of the outline's strength lines between things keep.
const LINE: f32 = 0.25;

const LIGHT: Theme = Theme {
    dark: false,
    page: 0xf6f8fcff,
    backdrop: 0xf6f8fcff,
    surface: 0xffffffff,
    read_row: 0xf2f6fcff,
    text: 0x1f1f1fff,
    text_dim: 0x444746ff,
    text_faint: 0x5e6368ff,
    divider: 0x64798f09,
    outline: 0x64798f24,
    hover: 0x1f1f1f12,
    ripple: 0x1f1f1f24,
    nav_selected: 0xd3e3fdff,
    nav_selected_text: 0x041e49ff,
    row_selected: 0x1f1f1f1a,
    row_selected_text: 0x1f1f1fff,
    row_selected_pill: 0xffffffb3,
    compose: 0xc2e7ffff,
    compose_text: 0x001d35ff,
    search: 0xe9eef6ff,
    search_focused: 0xffffffff,
    accent: 0x0b57d0ff,
    on_accent: 0xffffffff,
    star: 0xf4b400ff,
    important: 0x0b57d0ff,
    checked_row: 0xc2dbffff,
    menu: 0xffffffff,
    raised: 0xffffffff,
    rim: 0x00000000,
    // The shadow's ink at 10%.
    card_edge: 0x3c40431a,
    frost: 0,
    frost_tint: 100,
    pane_tint: 100,
    chat_tint: 100,
    search_tint: 100,
    switch_off: 0xe1e3e1ff,
    tabs: [0x0b57d0ff, 0x188038ff, 0x1a73e8ff, 0xe37400ff, 0x9334e6ff],
    folder_icons: FolderIcons {
        unread: 0x1a73e8ff,
        starred: 0xe8a600ff,
        important: 0xe37400ff,
        sent: 0x188038ff,
        all_mail: 0x5c6bc0ff,
        spam: 0xd93025ff,
        drafts: 0x9334e6ff,
        scheduled: 0x00897bff,
    },
    chip: 0xe1e3e1ff,
    snackbar: 0x313033ff,
    snackbar_text: 0xf4eff4ff,
    error: 0xb3261eff,
    warning: 0xa05a00ff,
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
    divider: 0xffffff06,
    outline: 0xffffff17,
    hover: 0xffffff14,
    ripple: 0xffffff29,
    nav_selected: 0x004a77ff,
    nav_selected_text: 0xc2e7ffff,
    row_selected: 0xe3e3e31f,
    row_selected_text: 0xe3e3e3ff,
    row_selected_pill: 0xe3e3e31a,
    compose: 0x004a77ff,
    compose_text: 0xc2e7ffff,
    search: 0x2a2d31ff,
    search_focused: 0x383b40ff,
    accent: 0xa8c7faff,
    on_accent: 0x062e6fff,
    star: 0xfdd663ff,
    // The same blue as Gmail's marker, which reads on dark too.
    important: 0x0b57d0ff,
    checked_row: 0x004a77ff,
    // `surface` lifted by RAISED_LIFT and MENU_LIFT of `text`; the rim is
    // RIM of `text`.
    menu: 0x383a3dff,
    raised: 0x333537ff,
    rim: 0xe3e3e321,
    card_edge: 0x00000000,
    frost: 0,
    frost_tint: 100,
    pane_tint: 100,
    chat_tint: 100,
    search_tint: 100,
    switch_off: 0x44474eff,
    tabs: [0xa8c7faff, 0x81c995ff, 0x8ab4f8ff, 0xfcad70ff, 0xd7aefbff],
    folder_icons: FolderIcons {
        unread: 0x8ab4f8ff,
        starred: 0xfdd663ff,
        important: 0xfcad70ff,
        sent: 0x81c995ff,
        all_mail: 0x9fa8daff,
        spam: 0xf28b82ff,
        drafts: 0xd7aefbff,
        scheduled: 0x80cbc4ff,
    },
    chip: 0x3c3f43ff,
    snackbar: 0xe3e3e3ff,
    snackbar_text: 0x1f1f1fff,
    error: 0xf2b8b5ff,
    warning: 0xfdd663ff,
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

/// The faintest colour that, laid over the opaque `base`, gives `fill`
/// laid over it: transparent when `fill` adds nothing.
pub fn tint_over(base: u32, fill: u32) -> u32 {
    let channel = |c: u32, i: u32| ((c >> (24 - 8 * i)) & 0xff) as f32;
    let fill_alpha = channel(fill, 3) / 255.0;
    // What `fill` makes of the base, channel by channel.
    let target: [f32; 3] = std::array::from_fn(|i| {
        let (b, f) = (channel(base, i as u32), channel(fill, i as u32));
        b + (f - b) * fill_alpha
    });
    // The least alpha that can reach it with a colour in range.
    let alpha = (0..3).fold(0.0_f32, |a, i| {
        let b = channel(base, i);
        let d = target[i as usize] - b;
        let room = if d < 0.0 { b } else { 255.0 - b };
        if room > 0.0 { a.max(d.abs() / room) } else { a }
    });
    if alpha < 1.0 / 255.0 {
        return 0;
    }
    (0..3).fold((alpha * 255.0).round() as u32, |out, i| {
        let b = channel(base, i);
        let c = (b + (target[i as usize] - b) / alpha)
            .round()
            .clamp(0.0, 255.0) as u32;
        out | (c << (24 - 8 * i))
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
    AVATARS[(address_hash(address) % AVATARS.len() as u64) as usize]
}

/// FNV-1a of the lower-case address, so colors do not change between
/// runs or versions.
fn address_hash(address: &str) -> u64 {
    address
        .to_lowercase()
        .bytes()
        .fold(0xcbf2_9ce4_8422_2325_u64, |h, b| {
            (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3)
        })
}

/// The colors an account can wear: its dot on lines of the unified inbox,
/// the ring round its picture and its letter avatar. None is an inbox
/// tab's color, so a dot never reads as a tab. By name (as kept in
/// `mail.account_colors`): its light-mode color, readable with white
/// text, and its dark-mode one.
pub const ACCOUNT_COLORS: [(&str, u32, u32); 8] = [
    ("red", 0xd93025ff, 0xf28b82ff),
    ("pink", 0xc2185bff, 0xf48fb1ff),
    ("magenta", 0xa0189bff, 0xe68ae0ff),
    ("brown", 0x8d6e63ff, 0xbcaaa4ff),
    ("olive", 0x827717ff, 0xc0ca33ff),
    ("teal", 0x007b83ff, 0x4fb8c0ff),
    ("indigo", 0x3949abff, 0x9fa8daff),
    ("slate", 0x5f6368ff, 0x9aa0a6ff),
];

/// The color an account wears until one is picked: its letter avatar's
/// old color where that is in [`ACCOUNT_COLORS`], so accounts look as
/// before; else one picked from the address.
pub fn default_account_color(address: &str) -> &'static str {
    let old = avatar_color(address);
    ACCOUNT_COLORS
        .iter()
        .find(|(_, light, _)| *light == old || (old == 0xc5221fff && *light == 0xd93025ff))
        .map_or_else(
            || ACCOUNT_COLORS[(address_hash(address) % ACCOUNT_COLORS.len() as u64) as usize].0,
            |(name, ..)| name,
        )
}

/// Colors of one's own that accounts get, in turn, once every one of
/// [`ACCOUNT_COLORS`] is taken: light-mode colors, none a tab's.
pub const MORE_ACCOUNT_COLORS: [u32; 6] = [
    0x8e2430ff, 0xa87b00ff, 0x00695cff, 0x5b7f1bff, 0x6d4c41ff, 0x455a64ff,
];

/// The dark-mode color of an account color of one's own `light`.
pub fn account_dark(light: u32) -> u32 {
    mix(light, 0xffffffff, 0.4)
}

/// Whether `color` is close to an inbox tab's color, in light or dark,
/// so its dot could read as a tab.
#[cfg(test)]
fn near_tab_color(color: u32) -> bool {
    let distance = |a: u32, b: u32| {
        [24, 16, 8]
            .iter()
            .map(|shift| {
                let d = ((a >> shift) & 0xff) as i32 - ((b >> shift) & 0xff) as i32;
                d * d
            })
            .sum::<i32>()
    };
    let dark = account_dark(color);
    LIGHT.tabs.iter().any(|&tab| distance(color, tab) < 40 * 40)
        || DARK.tabs.iter().any(|&tab| distance(dark, tab) < 40 * 40)
}

/// Gives each of `addresses` (lower case, in the folder pane's order)
/// without a color in `picked` one no other account wears: its old
/// letter color when free, else the next free one of [`ACCOUNT_COLORS`],
/// then of [`MORE_ACCOUNT_COLORS`] (as `#rrggbb`). Whether any was given.
pub fn settle_account_colors(
    addresses: &[String],
    picked: &mut std::collections::BTreeMap<String, String>,
) -> bool {
    let mut worn: Vec<String> = addresses
        .iter()
        .filter_map(|a| picked.get(a).cloned())
        .collect();
    let mut changed = false;
    for address in addresses {
        if picked.contains_key(address) {
            continue;
        }
        let first = default_account_color(address).to_owned();
        let more = MORE_ACCOUNT_COLORS
            .iter()
            .map(|c| format!("#{:06x}", c >> 8));
        let color = std::iter::once(first.clone())
            .chain(ACCOUNT_COLORS.iter().map(|(n, ..)| (*n).to_owned()))
            .chain(more)
            .find(|c| !worn.contains(c))
            .unwrap_or(first);
        worn.push(color.clone());
        picked.insert(address.clone(), color);
        changed = true;
    }
    changed
}

/// The account color called `name` (the default one for unknown
/// names): its light and dark colors.
pub fn account_color(name: &str) -> (u32, u32) {
    let (_, light, dark) = ACCOUNT_COLORS
        .iter()
        .find(|(n, ..)| *n == name)
        .unwrap_or(&ACCOUNT_COLORS[0]);
    (*light, *dark)
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
    use katna_platform::colors::DesktopScheme;

    #[test]
    fn rows_tint_a_frosted_card_lightly() {
        // Over the solid card the tint gives the row's own colour.
        let over = |base: u32, tint: u32| {
            let a = (tint & 0xff) as f32 / 255.0;
            mix(base, tint | 0xff, a) | 0xff
        };
        for (surface, row) in [(0xffffffff, 0xf2f6fcff), (0x1f2124ff, 0x191b1eff)] {
            let tint = tint_over(surface, row);
            assert!(tint & 0xff < 0x40, "{tint:08x} is faint");
            let got = over(surface, tint);
            for shift in [24, 16, 8] {
                let d = ((got >> shift) & 0xff).abs_diff((row >> shift) & 0xff);
                assert!(d <= 1, "{got:08x} vs {row:08x}");
            }
        }
        assert_eq!(tint_over(0xffffffff, 0xffffffff), 0);
        // On a frosted card the card's colour adds nothing; a read row is
        // as see-through as an unread one, only tinted.
        let th = LIGHT.frosted_panes(Some(75), None, None);
        assert_eq!(th.on_pane(th.surface), 0);
        assert!(th.on_pane(th.read_row) & 0xff < 0x40);
    }

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
            assert!(contrast(th.row_selected_text, over(th.row_selected, th.page)) >= 4.5);
        }
    }

    #[test]
    fn picks_scheme_mode_and_accent_apart() {
        let none = SystemColors::default();
        // Katna's own palette, as it always looked.
        assert_eq!(Theme::pick(false, "katna", Accent::Scheme, &none), LIGHT);
        assert_eq!(Theme::pick(true, "katna", Accent::Scheme, &none), DARK);
        assert_eq!(Theme::pick(true, "gone", Accent::Scheme, &none), DARK);
        // Every built-in scheme in either mode, with any accent, stays
        // readable.
        for scheme in schemes::BUILT_IN {
            for dark in [false, true] {
                for accent in [
                    Accent::Scheme,
                    Accent::Color(0xffd400ff),
                    Accent::Color(0x1a1a1aff),
                ] {
                    let th = Theme::pick(dark, scheme.id, accent, &none);
                    assert_eq!(th.dark, dark, "{}", scheme.id);
                    assert!(contrast(th.text, th.surface) >= 4.5, "{}", scheme.id);
                    assert!(contrast(th.text_faint, th.surface) >= 3.0, "{}", scheme.id);
                    assert!(contrast(th.accent, th.surface) >= 3.0, "{}", scheme.id);
                    assert!(contrast(th.on_accent, th.accent) >= 3.0, "{}", scheme.id);
                }
            }
        }
        // An accent of its own over a scheme.
        let nord = Theme::pick(false, "nord", Accent::Color(0xd6336cff), &none);
        assert_eq!(nord.page, 0xe5e9f0ff);
        assert_eq!(nord.accent, 0xd6336cff);
        // The desktop's accent, else the scheme's own.
        let red = SystemColors::accent_only(Some(0xe62d42ff));
        assert_eq!(
            Theme::pick(false, "clear", Accent::System, &red).accent,
            0xe62d42ff
        );
        assert_eq!(
            Theme::pick(false, "clear", Accent::System, &none).accent,
            0x007affff
        );
        // The desktop's scheme with an accent of one's own.
        let th = Theme::pick(false, "system", Accent::Color(0x2e9e4fff), &red);
        assert_eq!(th.page, LIGHT.page);
        assert_eq!(th.accent, 0x2e9e4fff);
        // The desktop's other schemes; one with one side decides light or
        // dark itself.
        let breeze = Scheme {
            name: "BreezeClassic".to_owned(),
            window_bg: 0xeff0f1ff,
            window_fg: 0x31363bff,
            view_bg: 0xfcfcfcff,
            view_fg: 0x31363bff,
            inactive_fg: 0x7f8c8dff,
            accent: 0x3daee9ff,
            accent_fg: 0xffffffff,
            negative: 0xda4453ff,
            neutral: None,
        };
        let desktop = SystemColors::default().with_schemes(vec![DesktopScheme {
            id: "kde:BreezeClassic".to_owned(),
            name: "Breeze Classic".to_owned(),
            light: Some(breeze.clone()),
            dark: None,
        }]);
        assert_eq!(
            Theme::forced_dark("kde:BreezeClassic", &desktop),
            Some(false)
        );
        assert_eq!(Theme::forced_dark("nord", &desktop), None);
        // System colours from a dark KDE scheme without a light partner:
        // Light mode is Katna's light palette in the scheme's accent.
        let mut alone = SystemColors::kde(Scheme {
            name: "Breath Dark".to_owned(),
            window_bg: 0x2a2e32ff,
            view_bg: 0x1b1e20ff,
            ..breeze.clone()
        });
        alone.accent = Some(0x1abc9cff);
        assert_eq!(Theme::forced_dark("system", &alone), None);
        let th = Theme::pick(false, "system", Accent::System, &alone);
        assert!(!th.dark);
        assert_eq!(th.surface, Theme::new(false).surface);
        assert!(!Theme::picks_scheme(false, "system", &alone));
        assert!(Theme::pick(true, "system", Accent::System, &alone).dark);
        let th = Theme::pick(true, "kde:BreezeClassic", Accent::Scheme, &desktop);
        assert_eq!((th.dark, th.surface), (false, 0xfcfcfcff));
        assert!(Theme::picks_scheme(false, "kde:BreezeClassic", &desktop));
        assert!(Theme::picks_scheme(true, "nord", &none));
        assert!(!Theme::picks_scheme(true, "system", &none));
        assert!(!Theme::picks_scheme(true, "katna", &none));
    }

    #[test]
    fn reads_the_accent_setting() {
        assert_eq!(Accent::parse(""), Accent::Scheme);
        assert_eq!(Accent::parse("system"), Accent::System);
        assert_eq!(Accent::parse("#E8590c"), Accent::Color(0xe8590cff));
        assert_eq!(Accent::parse("#e859"), Accent::Scheme);
        for accent in [Accent::Scheme, Accent::System, Accent::Color(0x00807fff)] {
            assert_eq!(Accent::parse(&accent.setting()), accent);
        }
    }

    #[test]
    fn raised_surfaces_stand_out_in_dark() {
        use katna_platform::colors::parse_kdeglobals;

        let (breeze_dark, _) = parse_kdeglobals(
            "[Colors:Window]\nBackgroundNormal=32,35,38\nForegroundNormal=252,252,252\n\
             [Colors:View]\nBackgroundNormal=20,22,24\nForegroundNormal=252,252,252\n",
        );
        let breeze = Theme::from_scheme(&breeze_dark);
        for th in [DARK, breeze] {
            // Lighter than the cards and the page, menus lighter again.
            assert!(luminance(th.raised) > luminance(th.surface) + 0.01);
            assert!(luminance(th.raised) > luminance(th.page));
            assert!(luminance(th.menu) > luminance(th.raised));
            assert_ne!(th.rim & 0xff, 0);
            // Dark cards stand off the page by their fill, with no ring.
            assert_eq!(th.card_edge, 0);
            let lifted = th.lifted();
            assert_eq!(lifted.surface, th.raised);
            assert!(luminance(lifted.search_focused) > luminance(lifted.surface));
        }
        // Katna's own dark palette follows the same rule as a scheme.
        let ink = |alpha: f32| over(fade(DARK.text, alpha), DARK.surface);
        assert_eq!(DARK.raised, ink(RAISED_LIFT));
        assert_eq!(DARK.menu, ink(MENU_LIFT));
        assert_eq!(DARK.rim, fade(DARK.text, RIM));
        // Light colors are left as they were: shadows show there.
        assert_eq!(LIGHT.raised, LIGHT.surface);
        assert_eq!(LIGHT.rim, 0);
        // A white card on the near-white page gets a hairline ring.
        assert_ne!(LIGHT.card_edge & 0xff, 0);
        assert_eq!(LIGHT.lifted(), LIGHT);
    }

    #[test]
    fn avatars() {
        assert_eq!(avatar_color("Kay@Enron.com"), avatar_color("kay@enron.com"));
        // No account color is a tab's, in either mode.
        for th in [&LIGHT, &DARK] {
            for (name, light, dark) in ACCOUNT_COLORS {
                let color = if th.dark { dark } else { light };
                assert!(!th.tabs.contains(&color), "{name}");
            }
        }
        for address in ["kay@enron.com", "ada@example.org", "me@gmail.com"] {
            let name = default_account_color(address);
            assert!(ACCOUNT_COLORS.iter().any(|(n, ..)| *n == name));
        }
        for color in MORE_ACCOUNT_COLORS {
            assert!(!near_tab_color(color), "{color:08x}");
        }
        for (name, light, _) in ACCOUNT_COLORS {
            assert!(!near_tab_color(light), "{name}");
        }
        assert!(near_tab_color(0x0b57d0ff) && near_tab_color(0x1a70e0ff));
        // Eight accounts wear eight colors; a picked one is kept.
        let addresses: Vec<String> = (0..8).map(|n| format!("a{n}@example.org")).collect();
        let mut picked =
            std::collections::BTreeMap::from([(addresses[3].clone(), "#123456".to_owned())]);
        assert!(settle_account_colors(&addresses, &mut picked));
        assert_eq!(picked[&addresses[3]], "#123456");
        let mut worn: Vec<&String> = picked.values().collect();
        worn.sort();
        worn.dedup();
        assert_eq!(worn.len(), 8);
        assert!(!settle_account_colors(&addresses, &mut picked));
        assert_eq!(initial("  kay mann"), "K");
        assert_eq!(initial("\"Ölaf\""), "Ö");
        assert_eq!(initial("--"), "?");
    }
}
