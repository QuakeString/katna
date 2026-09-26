// SPDX-License-Identifier: GPL-3.0-or-later

//! The desktop's colors (`docs/ARCHITECTURE.md` §13.2).
//!
//! - **KDE**: the active color scheme from the `[Colors:*]` groups of
//!   `kdeglobals` (Breeze Light, KDE's built-in default, when there are
//!   none) and `AccentColor` from `[General]`.
//! - **GNOME**: the libadwaita palette with the accent color, and the colors
//!   a theme tool put in `~/.config/gtk-4.0/gtk.css` (`@define-color`).
//! - **Everywhere**: the accent color from the Settings portal
//!   (`org.freedesktop.appearance accent-color`), and on GNOME the
//!   `accent-color` GSettings key when the portal has none.
//!
//! Elsewhere there is no scheme, and the apps keep their own colors with the
//! accent color. Colors are `0xRRGGBBAA`. No GPUI types here.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

use futures_lite::{Stream, StreamExt};
use zbus::Connection;
use zbus::zvariant::{OwnedValue, Structure, Value};

/// A desktop color scheme, with every color opaque.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scheme {
    /// Where it came from, such as `BreezeDark` or `Adwaita`.
    pub name: String,
    /// Behind views: toolbars, sidebars, the window itself.
    pub window_bg: u32,
    pub window_fg: u32,
    /// Lists, text views and cards.
    pub view_bg: u32,
    pub view_fg: u32,
    /// Secondary text.
    pub inactive_fg: u32,
    /// The accent (selection) color and the text on it.
    pub accent: u32,
    pub accent_fg: u32,
    /// Errors and destructive actions.
    pub negative: u32,
}

impl Scheme {
    /// Whether the scheme is dark: its window is darker than mid-grey.
    pub fn dark(&self) -> bool {
        luminance(self.window_bg) < 0.18
    }
}

/// What the desktop says about colors.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SystemColors {
    /// The accent color, if the desktop has one.
    pub accent: Option<u32>,
    source: Source,
}

#[derive(Debug, Clone, PartialEq, Default)]
enum Source {
    #[default]
    None,
    /// KDE's scheme: one palette, light or dark.
    Kde(Scheme),
    /// libadwaita's light and dark palettes, with the custom colors of
    /// `gtk.css` for each.
    Gnome {
        light: Vec<(String, u32)>,
        dark: Vec<(String, u32)>,
    },
}

impl SystemColors {
    /// Only an accent color, as on desktops without a color scheme.
    pub fn accent_only(accent: Option<u32>) -> Self {
        Self {
            accent,
            source: Source::None,
        }
    }

    /// A KDE color scheme.
    pub fn kde(scheme: Scheme) -> Self {
        Self {
            accent: Some(scheme.accent),
            source: Source::Kde(scheme),
        }
    }

    /// GNOME's palettes with `accent` and the style sheet `gtk_css`.
    pub fn gnome(accent: Option<u32>, gtk_css: &str) -> Self {
        Self {
            accent,
            source: Source::Gnome {
                light: parse_gtk_css(gtk_css, false),
                dark: parse_gtk_css(gtk_css, true),
            },
        }
    }

    /// The desktop's scheme for a light or `dark` window, if it has one
    /// that is as dark as asked: a light scheme is no use when the app is
    /// dark, and the other way round.
    pub fn scheme_for(&self, dark: bool) -> Option<Scheme> {
        let scheme = match &self.source {
            Source::None => return None,
            Source::Kde(scheme) => scheme.clone(),
            Source::Gnome {
                light,
                dark: custom,
            } => adwaita(dark, self.accent, if dark { custom } else { light }),
        };
        (scheme.dark() == dark).then_some(scheme)
    }
}

/// Which desktop's colors to read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopKind {
    Kde,
    Gnome,
    Other,
}

/// Reads the desktop's colors. `config_home` is normally `~/.config`;
/// `portal_accent` is the portal's accent color, read with
/// [`portal_accent`] (`None` if there is no portal or no accent).
pub fn read(desktop: DesktopKind, config_home: &Path, portal_accent: Option<u32>) -> SystemColors {
    match desktop {
        DesktopKind::Kde => {
            let contents =
                std::fs::read_to_string(config_home.join("kdeglobals")).unwrap_or_default();
            let (mut scheme, accent) = parse_kdeglobals(&contents);
            // A custom or wallpaper accent (`AccentColor`) replaces the
            // scheme's selection color, as in KDE apps.
            if let Some(accent) = accent.or(portal_accent) {
                scheme.accent = accent;
            }
            SystemColors::kde(scheme)
        }
        DesktopKind::Gnome => {
            let accent = portal_accent.or_else(gsettings_accent);
            let css = std::fs::read_to_string(gtk_css(config_home)).unwrap_or_default();
            SystemColors::gnome(accent, &css)
        }
        DesktopKind::Other => SystemColors::accent_only(portal_accent),
    }
}

/// `$XDG_CONFIG_HOME`, else `~/.config`.
pub fn config_home() -> Option<PathBuf> {
    std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
}

fn gtk_css(config_home: &Path) -> PathBuf {
    config_home.join("gtk-4.0").join("gtk.css")
}

/// The files the colors come from; when one of them changes, read again.
pub fn watched_files(desktop: DesktopKind, config_home: &Path) -> Vec<PathBuf> {
    match desktop {
        DesktopKind::Kde => vec![config_home.join("kdeglobals")],
        DesktopKind::Gnome => vec![gtk_css(config_home)],
        DesktopKind::Other => Vec::new(),
    }
}

/// The modification times of `files`, to notice a change by polling.
pub fn stamps(files: &[PathBuf]) -> Vec<Option<SystemTime>> {
    files
        .iter()
        .map(|f| std::fs::metadata(f).and_then(|m| m.modified()).ok())
        .collect()
}

// ---------------------------------------------------------------- KDE

/// KDE's built-in colors (Breeze Light), for keys a scheme leaves out.
const BREEZE_LIGHT: Scheme = Scheme {
    name: String::new(),
    window_bg: 0xeff0f1ff,
    window_fg: 0x232629ff,
    view_bg: 0xffffffff,
    view_fg: 0x232629ff,
    inactive_fg: 0x707d8aff,
    accent: 0x3daee9ff,
    accent_fg: 0xffffffff,
    negative: 0xda4453ff,
};

/// Reads the color scheme and the accent color from a `kdeglobals` file.
/// Missing keys take KDE's built-in values, as KDE apps do.
pub fn parse_kdeglobals(contents: &str) -> (Scheme, Option<u32>) {
    let mut scheme = Scheme {
        name: "BreezeLight".to_owned(),
        ..BREEZE_LIGHT
    };
    let mut accent = None;
    let mut group = String::new();
    for line in contents.lines() {
        let line = line.trim();
        if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            group = name.to_owned();
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let (key, value) = (key.trim(), value.trim());
        if group == "General" {
            match key {
                "ColorScheme" if !value.is_empty() => scheme.name = value.to_owned(),
                "AccentColor" => accent = parse_kde_color(value),
                _ => {}
            }
            continue;
        }
        let Some(color) = parse_kde_color(value) else {
            continue;
        };
        let slot = match (group.as_str(), key) {
            ("Colors:Window", "BackgroundNormal") => &mut scheme.window_bg,
            ("Colors:Window", "ForegroundNormal") => &mut scheme.window_fg,
            ("Colors:View", "BackgroundNormal") => &mut scheme.view_bg,
            ("Colors:View", "ForegroundNormal") => &mut scheme.view_fg,
            ("Colors:View", "ForegroundInactive") => &mut scheme.inactive_fg,
            ("Colors:View", "ForegroundNegative") => &mut scheme.negative,
            ("Colors:Selection", "BackgroundNormal") => &mut scheme.accent,
            ("Colors:Selection", "ForegroundNormal") => &mut scheme.accent_fg,
            _ => continue,
        };
        *slot = color;
    }
    (scheme, accent)
}

/// Parses a KDE color: `61,174,233`, `61,174,233,255` or `#3daee9`.
pub fn parse_kde_color(value: &str) -> Option<u32> {
    let value = value.trim();
    if value.starts_with('#') {
        return parse_hex(value);
    }
    let parts: Vec<u8> = value
        .split(',')
        .map(|p| p.trim().parse::<u8>())
        .collect::<Result<_, _>>()
        .ok()?;
    match parts[..] {
        [r, g, b] => Some(rgba(r, g, b, 255)),
        [r, g, b, a] => Some(rgba(r, g, b, a)),
        _ => None,
    }
}

// ---------------------------------------------------------------- GNOME

/// GNOME's named accent colors (`org.gnome.desktop.interface accent-color`,
/// GNOME 47), with libadwaita's values.
const GNOME_ACCENTS: [(&str, u32); 9] = [
    ("blue", 0x3584e4ff),
    ("teal", 0x2190a4ff),
    ("green", 0x3a944aff),
    ("yellow", 0xc88800ff),
    ("orange", 0xed5b00ff),
    ("red", 0xe62d42ff),
    ("pink", 0xd56199ff),
    ("purple", 0x9141acff),
    ("slate", 0x6f8396ff),
];

/// The color of a GNOME accent name, as printed by `gsettings get`
/// (`'teal'`).
pub fn gnome_accent(name: &str) -> Option<u32> {
    let name = name.trim().trim_matches('\'');
    GNOME_ACCENTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, c)| *c)
}

fn gsettings_accent() -> Option<u32> {
    let output = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "accent-color"])
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| gnome_accent(&String::from_utf8_lossy(&output.stdout)))
        .flatten()
}

/// The libadwaita palette (1.6) for `dark`, with `accent` (blue if `None`)
/// and the colors `custom` defines, by their libadwaita names.
pub fn adwaita(dark: bool, accent: Option<u32>, custom: &[(String, u32)]) -> Scheme {
    let get = |name: &str| custom.iter().rev().find(|(n, _)| n == name).map(|c| c.1);
    let (window_bg, fg, view_bg, negative) = if dark {
        (0x222226ff, 0xffffffff, 0x1d1d20ff, 0xff938cff)
    } else {
        (0xfafafbff, 0x000006cc, 0xffffffff, 0xc30000ff)
    };
    let window_bg = get("window_bg_color").unwrap_or(window_bg);
    let view_bg = get("view_bg_color").unwrap_or(view_bg);
    let window_fg = get("window_fg_color").unwrap_or(fg);
    let view_fg = get("view_fg_color").unwrap_or(window_fg);
    let accent = get("accent_bg_color")
        .or(accent)
        .unwrap_or(GNOME_ACCENTS[0].1);
    let accent_fg = get("accent_fg_color").unwrap_or(0xffffffff);
    let negative = get("error_color")
        .or_else(|| get("destructive_color"))
        .unwrap_or(negative);
    let window_bg = over(window_bg, 0x000000ff);
    let view_bg = over(view_bg, window_bg);
    let view_fg = over(view_fg, view_bg);
    Scheme {
        name: if custom.is_empty() {
            "Adwaita".to_owned()
        } else {
            "Adwaita (custom colors)".to_owned()
        },
        window_bg,
        window_fg: over(window_fg, window_bg),
        view_bg,
        view_fg,
        // libadwaita's dim labels are the text at 55%.
        inactive_fg: over((view_fg & 0xffff_ff00) | 0x8c, view_bg),
        accent: over(accent, view_bg),
        accent_fg: over(accent_fg, over(accent, view_bg)),
        negative: over(negative, view_bg),
    }
}

/// Reads `@define-color name value;` lines from a GTK style sheet, as
/// theme tools write them to `~/.config/gtk-4.0/gtk.css`. Lines inside
/// `@media (prefers-color-scheme: dark)` apply only when `dark` (and
/// `light` only when light). Values are hex, `rgb()`, `rgba()`, `white`,
/// `black`, `transparent` or `@name` of an earlier color; others are left
/// out.
pub fn parse_gtk_css(css: &str, dark: bool) -> Vec<(String, u32)> {
    let css = strip_comments(css);
    let mut colors: Vec<(String, u32)> = Vec::new();
    // Whether each open block applies.
    let mut blocks: Vec<bool> = Vec::new();
    let mut rest = css.as_str();
    while !rest.is_empty() {
        let end = rest.find([';', '{', '}']).unwrap_or(rest.len());
        let (statement, sep) = (rest[..end].trim(), rest[end..].chars().next());
        rest = rest.get(end + 1..).unwrap_or("");
        match sep {
            Some('{') => {
                let applies = match statement.strip_prefix("@media") {
                    Some(query) if query.contains("prefers-color-scheme") => {
                        query.contains(if dark { "dark" } else { "light" })
                    }
                    // Rules for widgets: nothing to read inside.
                    _ => false,
                };
                blocks.push(applies);
            }
            Some('}') => {
                blocks.pop();
            }
            _ => {
                if blocks.iter().all(|b| *b)
                    && let Some(define) = statement.strip_prefix("@define-color")
                {
                    let define = define.trim();
                    let Some((name, value)) = define.split_once(char::is_whitespace) else {
                        continue;
                    };
                    let value = value.trim();
                    let color = match value.strip_prefix('@') {
                        Some(other) => colors.iter().rev().find(|(n, _)| n == other).map(|c| c.1),
                        None => parse_css_color(value),
                    };
                    if let Some(color) = color {
                        colors.push((name.to_owned(), color));
                    }
                }
            }
        }
    }
    colors
}

fn strip_comments(css: &str) -> String {
    let mut out = String::with_capacity(css.len());
    let mut rest = css;
    while let Some(start) = rest.find("/*") {
        out.push_str(&rest[..start]);
        rest = rest[start + 2..]
            .find("*/")
            .map_or("", |end| &rest[start + 2 + end + 2..]);
    }
    out.push_str(rest);
    out
}

/// Parses a CSS color: `#rgb`, `#rrggbb`, `#rrggbbaa`, `rgb(r, g, b)`,
/// `rgba(r, g, b, a)`, `white`, `black` or `transparent`.
pub fn parse_css_color(value: &str) -> Option<u32> {
    let value = value.trim();
    match value.to_ascii_lowercase().as_str() {
        "white" => return Some(0xffffffff),
        "black" => return Some(0x000000ff),
        "transparent" => return Some(0),
        _ => {}
    }
    if value.starts_with('#') {
        return parse_hex(value);
    }
    let args = value
        .strip_prefix("rgba(")
        .or_else(|| value.strip_prefix("rgb("))?
        .strip_suffix(')')?;
    let parts: Vec<&str> = args.split(',').map(str::trim).collect();
    let channel = |s: &str| -> Option<u8> {
        match s.strip_suffix('%') {
            Some(p) => Some((p.parse::<f32>().ok()? * 2.55).round().clamp(0.0, 255.0) as u8),
            None => Some(s.parse::<f32>().ok()?.round().clamp(0.0, 255.0) as u8),
        }
    };
    let alpha = |s: &str| -> Option<u8> {
        Some((s.parse::<f32>().ok()?.clamp(0.0, 1.0) * 255.0).round() as u8)
    };
    match parts[..] {
        [r, g, b] => Some(rgba(channel(r)?, channel(g)?, channel(b)?, 255)),
        [r, g, b, a] => Some(rgba(channel(r)?, channel(g)?, channel(b)?, alpha(a)?)),
        _ => None,
    }
}

fn parse_hex(value: &str) -> Option<u32> {
    let hex = value.strip_prefix('#')?;
    if !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let n = u32::from_str_radix(hex, 16).ok()?;
    match hex.len() {
        3 => {
            let (r, g, b) = ((n >> 8) & 0xf, (n >> 4) & 0xf, n & 0xf);
            Some((r * 0x11) << 24 | (g * 0x11) << 16 | (b * 0x11) << 8 | 0xff)
        }
        6 => Some(n << 8 | 0xff),
        8 => Some(n),
        _ => None,
    }
}

// ---------------------------------------------------------------- portal

const PORTAL: &str = "org.freedesktop.portal.Desktop";
const PORTAL_PATH: &str = "/org/freedesktop/portal/desktop";
const SETTINGS: &str = "org.freedesktop.portal.Settings";

/// The accent color from the Settings portal, if the desktop sets one.
pub async fn portal_accent(connection: &Connection) -> Option<u32> {
    let reply = connection
        .call_method(
            Some(PORTAL),
            PORTAL_PATH,
            Some(SETTINGS),
            "ReadOne",
            &("org.freedesktop.appearance", "accent-color"),
        )
        .await
        .ok()?;
    let value: OwnedValue = reply.body().deserialize().ok()?;
    accent_from_value(&value)
}

/// The portal's `(ddd)` accent color: each channel 0..=1, anything else
/// meaning "not set".
fn accent_from_value(value: &Value<'_>) -> Option<u32> {
    let value = match value {
        Value::Value(inner) => inner,
        value => value,
    };
    let Value::Structure(structure) = value else {
        return None;
    };
    accent_from_structure(structure)
}

fn accent_from_structure(structure: &Structure<'_>) -> Option<u32> {
    let channels: Vec<f64> = structure
        .fields()
        .iter()
        .map(|f| match f {
            Value::F64(v) => Some(*v),
            _ => None,
        })
        .collect::<Option<_>>()?;
    let [r, g, b] = channels[..] else {
        return None;
    };
    let byte = |v: f64| (0.0..=1.0).contains(&v).then(|| (v * 255.0).round() as u8);
    Some(rgba(byte(r)?, byte(g)?, byte(b)?, 255))
}

/// Yields whenever a desktop setting changes (accent color, color scheme,
/// and on KDE any `kdeglobals` group), so the colors can be read again.
pub async fn setting_changes(connection: &Connection) -> zbus::Result<impl Stream<Item = ()>> {
    let proxy = zbus::Proxy::new(connection, PORTAL, PORTAL_PATH, SETTINGS).await?;
    let changes = proxy.receive_signal("SettingChanged").await?;
    Ok(changes.map(|_| ()))
}

// ---------------------------------------------------------------- colors

const fn rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    (r as u32) << 24 | (g as u32) << 16 | (b as u32) << 8 | a as u32
}

fn channels(c: u32) -> [f32; 4] {
    [24, 16, 8, 0].map(|s| ((c >> s) & 0xff) as f32 / 255.0)
}

/// `color` laid over the opaque `below`: the opaque color that shows.
pub fn over(color: u32, below: u32) -> u32 {
    let [r, g, b, a] = channels(color);
    let [br, bg, bb, _] = channels(below);
    let mix = |c: f32, d: f32| ((c * a + d * (1.0 - a)) * 255.0).round() as u8;
    rgba(mix(r, br), mix(g, bg), mix(b, bb), 255)
}

/// Relative luminance (WCAG) of the color, ignoring alpha.
pub fn luminance(color: u32) -> f32 {
    let lin = |c: f32| {
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    let [r, g, b, _] = channels(color);
    0.2126 * lin(r) + 0.7152 * lin(g) + 0.0722 * lin(b)
}

/// WCAG contrast ratio of two colors, 1 to 21.
pub fn contrast(a: u32, b: u32) -> f32 {
    let (la, lb) = (luminance(a), luminance(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

#[cfg(test)]
mod tests {
    use super::*;

    const BREEZE_DARK: &str = "\
[ColorEffects:Disabled]
Color=56,56,56

[Colors:Selection]
BackgroundNormal=61,174,233
ForegroundNormal=255,255,255

[Colors:View]
BackgroundAlternate=29,31,34
BackgroundNormal=20,22,24
ForegroundInactive=161,169,177
ForegroundNegative=218,68,83
ForegroundNormal=252,252,252

[Colors:Window]
BackgroundNormal=32,35,38
ForegroundNormal=252,252,252

[General]
ColorScheme=BreezeDark
AccentColor=233,100,61
font=Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1
";

    #[test]
    fn kdeglobals_scheme() {
        let (scheme, accent) = parse_kdeglobals(BREEZE_DARK);
        assert_eq!(scheme.name, "BreezeDark");
        assert_eq!(scheme.window_bg, 0x202326ff);
        assert_eq!(scheme.view_bg, 0x141618ff);
        assert_eq!(scheme.view_fg, 0xfcfcfcff);
        assert_eq!(scheme.inactive_fg, 0xa1a9b1ff);
        assert_eq!(scheme.accent, 0x3daee9ff);
        assert_eq!(scheme.negative, 0xda4453ff);
        assert!(scheme.dark());
        assert_eq!(accent, Some(0xe9643dff));
    }

    #[test]
    fn kdeglobals_defaults_to_breeze_light() {
        let (scheme, accent) = parse_kdeglobals("[General]\nfont=Noto Sans,10\n");
        assert_eq!(scheme.name, "BreezeLight");
        assert_eq!(scheme.window_bg, 0xeff0f1ff);
        assert!(!scheme.dark());
        assert_eq!(accent, None);
    }

    #[test]
    fn kde_colors() {
        assert_eq!(parse_kde_color("61,174,233"), Some(0x3daee9ff));
        assert_eq!(parse_kde_color(" 1, 2, 3, 4 "), Some(0x01020304));
        assert_eq!(parse_kde_color("#3daee9"), Some(0x3daee9ff));
        assert_eq!(parse_kde_color("61,174"), None);
        assert_eq!(parse_kde_color("300,0,0"), None);
        assert_eq!(parse_kde_color("true"), None);
    }

    #[test]
    fn css_colors() {
        assert_eq!(parse_css_color("#fff"), Some(0xffffffff));
        assert_eq!(parse_css_color("#1e1e2e"), Some(0x1e1e2eff));
        assert_eq!(parse_css_color("#1e1e2e80"), Some(0x1e1e2e80));
        assert_eq!(parse_css_color("rgb(30, 30, 46)"), Some(0x1e1e2eff));
        assert_eq!(parse_css_color("rgba(0,0,6,0.8)"), Some(0x000006cc));
        assert_eq!(parse_css_color("rgb(100%, 0%, 0%)"), Some(0xff0000ff));
        assert_eq!(parse_css_color("black"), Some(0x000000ff));
        assert_eq!(parse_css_color("mix(red, blue, 0.5)"), None);
        assert_eq!(parse_css_color("#12"), None);
    }

    #[test]
    fn gtk_css_defines() {
        let css = "/* Generated by a theme tool */\n\
            @define-color accent_bg_color #8caaee;\n\
            @define-color accent_color @accent_bg_color;\n\
            @define-color window_bg_color rgb(48, 52, 70); /* base */\n\
            @media (prefers-color-scheme: light) {\n\
              @define-color view_bg_color #ffffff;\n\
            }\n\
            @media (prefers-color-scheme: dark) {\n\
              @define-color view_bg_color #292c3c;\n\
            }\n\
            window.background { background: shade(@window_bg_color, 1.1); }\n\
            @define-color headerbar_bg_color mix(@window_bg_color, black, 0.2);\n";
        let dark = parse_gtk_css(css, true);
        let get = |colors: &[(String, u32)], name: &str| {
            colors.iter().rev().find(|(n, _)| n == name).map(|c| c.1)
        };
        assert_eq!(get(&dark, "accent_color"), Some(0x8caaeeff));
        assert_eq!(get(&dark, "window_bg_color"), Some(0x303446ff));
        assert_eq!(get(&dark, "view_bg_color"), Some(0x292c3cff));
        assert_eq!(get(&dark, "headerbar_bg_color"), None);
        let light = parse_gtk_css(css, false);
        assert_eq!(get(&light, "view_bg_color"), Some(0xffffffff));

        let scheme = adwaita(true, Some(0x3584e4ff), &dark);
        assert_eq!(scheme.window_bg, 0x303446ff);
        assert_eq!(scheme.view_bg, 0x292c3cff);
        assert_eq!(scheme.accent, 0x8caaeeff);
        assert_eq!(scheme.name, "Adwaita (custom colors)");
    }

    #[test]
    fn adwaita_palette() {
        let light = adwaita(false, gnome_accent("'teal'\n"), &[]);
        assert_eq!(light.name, "Adwaita");
        assert!(!light.dark());
        assert_eq!(light.accent, 0x2190a4ff);
        // 80% black over white.
        assert_eq!(light.view_fg, 0x333338ff);
        let dark = adwaita(true, None, &[]);
        assert!(dark.dark());
        assert_eq!(dark.accent, 0x3584e4ff);
        assert_eq!(gnome_accent("magenta"), None);
    }

    #[test]
    fn portal_values() {
        let value = Value::from(Structure::from((1.0_f64, 0.5_f64, 0.0_f64)));
        assert_eq!(accent_from_value(&value), Some(0xff8000ff));
        let nested = Value::Value(Box::new(value));
        assert_eq!(accent_from_value(&nested), Some(0xff8000ff));
        // Out of range means the desktop has no accent color.
        let unset = Value::from(Structure::from((-1.0_f64, -1.0_f64, -1.0_f64)));
        assert_eq!(accent_from_value(&unset), None);
        assert_eq!(accent_from_value(&Value::from(7_u32)), None);
    }

    #[test]
    fn scheme_for_matches_darkness() {
        let (breeze, _) = parse_kdeglobals("");
        let kde = SystemColors::kde(breeze);
        assert_eq!(kde.accent, Some(0x3daee9ff));
        assert!(kde.scheme_for(false).is_some());
        assert!(kde.scheme_for(true).is_none());

        let gnome = SystemColors::gnome(Some(0x9141acff), "");
        assert_eq!(gnome.scheme_for(false).unwrap().view_bg, 0xffffffff);
        assert_eq!(gnome.scheme_for(true).unwrap().view_bg, 0x1d1d20ff);
        assert_eq!(gnome.scheme_for(true).unwrap().accent, 0x9141acff);
        // A dark custom palette has no light version.
        let custom = SystemColors::gnome(None, "@define-color window_bg_color #303446;");
        assert!(custom.scheme_for(false).is_none());
        assert!(custom.scheme_for(true).is_some());

        assert!(SystemColors::accent_only(None).scheme_for(false).is_none());
    }

    #[test]
    fn color_math() {
        assert_eq!(over(0x00000080, 0xffffffff), 0x7f7f7fff);
        assert_eq!(over(0x123456ff, 0xffffffff), 0x123456ff);
        assert!((contrast(0x000000ff, 0xffffffff) - 21.0).abs() < 0.01);
        assert!((contrast(0x777777ff, 0x777777ff) - 1.0).abs() < 0.001);
    }

    #[test]
    fn comments_are_skipped() {
        assert_eq!(strip_comments("a/* x */b/* y"), "ab");
    }
}
