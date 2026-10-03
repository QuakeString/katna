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
//! - **Windows**: the accent color (`DWM\AccentColor` in the registry), a
//!   Contrast theme's colors while one is on, and the Contrast themes to
//!   pick from.
//!
//! Elsewhere there is no scheme, and the apps keep their own colors with the
//! accent color. Besides the scheme in use, the desktop's other schemes are
//! listed ([`DesktopScheme`]): every installed KDE scheme, with its light or
//! dark partner, and on Windows its own colors and Contrast themes. Colors
//! are `0xRRGGBBAA`. No GPUI types here.

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
    /// The desktop's schemes to pick from besides the one in use.
    pub schemes: Vec<DesktopScheme>,
}

/// One of the desktop's color schemes, with a light side, a dark side or
/// both (a KDE scheme and its partner, such as Breeze Light and Breeze
/// Dark).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopScheme {
    /// Stable across runs: `kde:` and the file name without `Light` or
    /// `Dark` (`kde:Breeze`), or `windows:` and a name.
    pub id: String,
    /// The name the desktop shows.
    pub name: String,
    pub light: Option<Scheme>,
    pub dark: Option<Scheme>,
}

impl DesktopScheme {
    /// The side for a light or `dark` window; a scheme with one side
    /// gives that side whatever is asked.
    pub fn side(&self, dark: bool) -> Option<&Scheme> {
        let (wanted, other) = if dark {
            (&self.dark, &self.light)
        } else {
            (&self.light, &self.dark)
        };
        wanted.as_ref().or(other.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
enum Source {
    #[default]
    None,
    /// KDE's scheme, light or dark, and its partner of the other kind if
    /// one is installed.
    Kde(Scheme, Option<Scheme>),
    /// A scheme used whether the window is light or dark: Windows'
    /// Contrast theme while one is on.
    Only(Scheme),
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
            ..Self::default()
        }
    }

    /// A KDE color scheme.
    pub fn kde(scheme: Scheme) -> Self {
        Self::kde_with_partner(scheme, None)
    }

    /// A KDE color scheme and its partner of the other kind (light for a
    /// dark scheme, dark for a light one).
    pub fn kde_with_partner(scheme: Scheme, partner: Option<Scheme>) -> Self {
        Self {
            accent: Some(scheme.accent),
            source: Source::Kde(scheme, partner),
            schemes: Vec::new(),
        }
    }

    /// A scheme used for light and dark windows alike, with `accent`.
    pub fn only(scheme: Scheme, accent: Option<u32>) -> Self {
        Self {
            accent,
            source: Source::Only(scheme),
            schemes: Vec::new(),
        }
    }

    /// The same, with `schemes` to pick from.
    pub fn with_schemes(self, schemes: Vec<DesktopScheme>) -> Self {
        Self { schemes, ..self }
    }

    /// The desktop's scheme with `id`.
    pub fn scheme(&self, id: &str) -> Option<&DesktopScheme> {
        self.schemes.iter().find(|s| s.id == id)
    }

    /// Whether the desktop's colors decide light or dark themselves:
    /// only Windows' Contrast themes, which all apps follow. `Some(dark)`
    /// then. A KDE scheme without a partner doesn't: the app's own light
    /// or dark mode wins, and [`Self::scheme_for`] has nothing for the
    /// other side.
    pub fn forced_dark(&self) -> Option<bool> {
        match &self.source {
            Source::Only(scheme) => Some(scheme.dark()),
            _ => None,
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
            schemes: Vec::new(),
        }
    }

    /// The desktop's scheme for a light or `dark` window, if it has one
    /// that is as dark as asked: a light scheme is no use when the app is
    /// dark, and the other way round.
    pub fn scheme_for(&self, dark: bool) -> Option<Scheme> {
        let scheme = match &self.source {
            Source::None => return None,
            Source::Only(scheme) => return Some(scheme.clone()),
            Source::Kde(scheme, partner) => match partner {
                Some(partner) if scheme.dark() != dark => partner.clone(),
                _ => scheme.clone(),
            },
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
    Windows,
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
            let schemes = kde_schemes(&kde_scheme_dirs(config_home));
            let mut partner = kde_partner(&scheme.name, &schemes);
            // A custom or wallpaper accent (`AccentColor`) replaces the
            // scheme's selection color, as in KDE apps.
            if let Some(accent) = accent.or(portal_accent) {
                scheme.accent = accent;
                if let Some(partner) = &mut partner {
                    partner.accent = accent;
                }
            }
            SystemColors::kde_with_partner(scheme, partner).with_schemes(schemes)
        }
        DesktopKind::Windows => windows::read(),
        DesktopKind::Gnome => {
            let accent = portal_accent.or_else(gsettings_accent);
            let css = std::fs::read_to_string(gtk_css(config_home)).unwrap_or_default();
            SystemColors::gnome(accent, &css)
        }
        DesktopKind::Other => SystemColors::accent_only(portal_accent),
    }
}

/// The color of the panel's own icons and text (`0xRRGGBB`), for a
/// one-color tray icon drawn as pixels (`icon::Style::Mono`). Panels can't
/// be asked, so it is inferred: Plasma's panel follows the color scheme's
/// window text, GNOME's top bar and most other panels are dark, and
/// Windows says whether its taskbar is light.
pub fn panel_text() -> u32 {
    #[cfg(windows)]
    {
        use winreg::RegKey;
        use winreg::enums::HKEY_CURRENT_USER;
        let light = RegKey::predef(HKEY_CURRENT_USER)
            .open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize")
            .and_then(|key| key.get_value::<u32, _>("SystemUsesLightTheme"))
            .is_ok_and(|light| light != 0);
        if light { 0x1c1c1c } else { 0xffffff }
    }
    #[cfg(not(windows))]
    {
        let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
        let kdeglobals =
            config_home().and_then(|home| std::fs::read_to_string(home.join("kdeglobals")).ok());
        panel_text_on(&desktop, kdeglobals.as_deref())
    }
}

/// [`panel_text`] on Linux, for `XDG_CURRENT_DESKTOP` and the contents of
/// `kdeglobals`.
pub fn panel_text_on(desktop: &str, kdeglobals: Option<&str>) -> u32 {
    if desktop.split(':').any(|d| d.eq_ignore_ascii_case("KDE")) {
        let (scheme, _) = parse_kdeglobals(kdeglobals.unwrap_or_default());
        return scheme.window_fg >> 8;
    }
    0xffffff
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
        DesktopKind::Windows | DesktopKind::Other => Vec::new(),
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

/// The folders holding KDE's color schemes, the user's first:
/// `$XDG_DATA_HOME/color-schemes` (`~/.local/share`) and each of
/// `$XDG_DATA_DIRS` (`/usr/local/share:/usr/share`).
pub fn kde_scheme_dirs(config_home: &Path) -> Vec<PathBuf> {
    let data_home = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| config_home.parent().map(|home| home.join(".local/share")));
    let data_dirs = std::env::var("XDG_DATA_DIRS")
        .ok()
        .filter(|dirs| !dirs.is_empty())
        .unwrap_or_else(|| "/usr/local/share:/usr/share".to_owned());
    data_home
        .into_iter()
        .chain(
            data_dirs
                .split(':')
                .filter(|d| !d.is_empty())
                .map(PathBuf::from),
        )
        .map(|dir| dir.join("color-schemes"))
        .collect()
}

/// Every KDE color scheme in `dirs` (`*.colors`; a file in an earlier
/// folder hides one of the same name in a later one), light and dark
/// partners paired, sorted by name.
pub fn kde_schemes(dirs: &[PathBuf]) -> Vec<DesktopScheme> {
    let mut files: Vec<(String, String)> = Vec::new();
    for dir in dirs {
        let Ok(entries) = std::fs::read_dir(dir) else {
            continue;
        };
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.extension().is_none_or(|e| e != "colors") {
                continue;
            }
            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            if files.iter().any(|(s, _)| s == stem) {
                continue;
            }
            if let Ok(contents) = std::fs::read_to_string(&path) {
                files.push((stem.to_owned(), contents));
            }
        }
    }
    pair_kde_schemes(&files)
}

/// Pairs KDE scheme files, given as (file name without `.colors`,
/// contents): a light and a dark scheme whose names differ only by
/// `Light` and `Dark` (`BreezeLight`, `BreezeDark`) become one scheme;
/// the rest stand alone.
pub fn pair_kde_schemes(files: &[(String, String)]) -> Vec<DesktopScheme> {
    let parsed: Vec<(String, String, Scheme)> = files
        .iter()
        .map(|(stem, contents)| {
            let (mut scheme, _) = parse_kdeglobals(contents);
            scheme.name = stem.clone();
            let name = kde_scheme_name(contents).unwrap_or_else(|| stem.clone());
            (stem.clone(), name, scheme)
        })
        .collect();
    let mut out: Vec<DesktopScheme> = Vec::new();
    let mut used = vec![false; parsed.len()];
    for (i, (stem, name, scheme)) in parsed.iter().enumerate() {
        if used[i] {
            continue;
        }
        used[i] = true;
        let base = kde_base(stem);
        let partner = (base != *stem)
            .then(|| {
                parsed.iter().enumerate().position(|(j, (other, _, s))| {
                    !used[j] && kde_base(other) == base && s.dark() != scheme.dark()
                })
            })
            .flatten();
        match partner {
            Some(j) => {
                used[j] = true;
                let (light, dark) = if scheme.dark() {
                    (&parsed[j], &parsed[i])
                } else {
                    (&parsed[i], &parsed[j])
                };
                out.push(DesktopScheme {
                    id: format!("kde:{base}"),
                    name: kde_pair_name(&light.1, &base),
                    light: Some(light.2.clone()),
                    dark: Some(dark.2.clone()),
                });
            }
            None => {
                let dark = scheme.dark();
                out.push(DesktopScheme {
                    id: format!("kde:{stem}"),
                    name: name.clone(),
                    light: (!dark).then(|| scheme.clone()),
                    dark: dark.then(|| scheme.clone()),
                });
            }
        }
    }
    out.sort_by_key(|s| s.name.to_lowercase());
    out
}

/// The partner of the KDE scheme in use, named `name` (its file name, as
/// `kdeglobals` says): the other side of its pair, if installed.
pub fn kde_partner(name: &str, schemes: &[DesktopScheme]) -> Option<Scheme> {
    let pair = schemes.iter().find(|s| {
        s.light.as_ref().is_some_and(|l| l.name == name)
            || s.dark.as_ref().is_some_and(|d| d.name == name)
    })?;
    let (light, dark) = (pair.light.as_ref()?, pair.dark.as_ref()?);
    Some(if light.name == name { dark } else { light }.clone())
}

/// A scheme file's name without `Light` or `Dark`.
fn kde_base(stem: &str) -> String {
    stem.replace("Light", "").replace("Dark", "")
}

/// The name of a pair: the light scheme's name without " Light" (Breeze
/// Light gives Breeze), else the file name without it.
fn kde_pair_name(light_name: &str, base: &str) -> String {
    let name = light_name.replace(" Light", "").replace("Light", "");
    let name = name.trim();
    if name.is_empty() {
        base.to_owned()
    } else {
        name.to_owned()
    }
}

/// `Name=` in a scheme file's `[General]` group.
fn kde_scheme_name(contents: &str) -> Option<String> {
    let mut general = false;
    for line in contents.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            general = line == "[General]";
            continue;
        }
        if general
            && let Some(name) = line.strip_prefix("Name=")
            && !name.trim().is_empty()
        {
            return Some(name.trim().to_owned());
        }
    }
    None
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

// ---------------------------------------------------------------- Windows

/// Windows' colors: its accent color, a Contrast theme while one is on,
/// and its own colors and Contrast themes to pick from.
pub mod windows {
    use super::{DesktopScheme, Scheme, SystemColors, rgba};

    /// Windows' own light and dark colors, as its Settings app draws them.
    pub fn default_scheme(accent: Option<u32>) -> DesktopScheme {
        let side = |page, card, text, faint, own: u32, dark| Scheme {
            name: "Windows".to_owned(),
            window_bg: page,
            window_fg: text,
            view_bg: card,
            view_fg: text,
            inactive_fg: faint,
            accent: accent.unwrap_or(own),
            accent_fg: if dark { 0x000000ff } else { 0xffffffff },
            negative: if dark { 0xff99a4ff } else { 0xc42b1cff },
        };
        DesktopScheme {
            id: "windows:default".to_owned(),
            name: "Windows".to_owned(),
            light: Some(side(
                0xf3f3f3ff, 0xffffffff, 0x1b1b1bff, 0x5f5f5fff, 0x005fb8ff, false,
            )),
            dark: Some(side(
                0x202020ff, 0x2b2b2bff, 0xffffffff, 0x9e9e9eff, 0x60cdffff, true,
            )),
        }
    }

    /// The accent color in `DWM\AccentColor`: `0xAABBGGRR`.
    pub fn accent_from_dword(value: u32) -> u32 {
        let [r, g, b] = [value & 0xff, (value >> 8) & 0xff, (value >> 16) & 0xff];
        r << 24 | g << 16 | b << 8 | 0xff
    }

    /// A color in Windows' `R G B` form (`Control Panel\Colors` and theme
    /// files).
    pub fn parse_rgb(value: &str) -> Option<u32> {
        let parts: Vec<u8> = value
            .split_whitespace()
            .map(|p| p.parse().ok())
            .collect::<Option<_>>()?;
        match parts[..] {
            [r, g, b] => Some(rgba(r, g, b, 255)),
            _ => None,
        }
    }

    /// A Contrast theme's colors, looked up by name as in `Control
    /// Panel\Colors` (`Window`, `WindowText`, `Hilight`, ...).
    pub fn contrast_scheme(name: &str, color: impl Fn(&str) -> Option<u32>) -> Option<Scheme> {
        let window = color("Window")?;
        let text = color("WindowText")?;
        Some(Scheme {
            name: name.to_owned(),
            window_bg: window,
            window_fg: text,
            view_bg: window,
            view_fg: text,
            inactive_fg: color("GrayText").unwrap_or(text),
            accent: color("Hilight").unwrap_or(text),
            accent_fg: color("HilightText").unwrap_or(window),
            negative: color("HotTrackingColor").unwrap_or(text),
        })
    }

    /// A Contrast theme from a `.theme` file's `[Control Panel\Colors]`
    /// group, named by its colors (the files name themselves through a
    /// resource string), else `stem`.
    pub fn parse_theme_file(stem: &str, contents: &str) -> Option<DesktopScheme> {
        let mut colors = Vec::new();
        let mut in_colors = false;
        for line in contents.lines() {
            let line = line.trim();
            if line.starts_with('[') {
                in_colors = line.eq_ignore_ascii_case("[Control Panel\\Colors]");
                continue;
            }
            if in_colors && let Some((key, value)) = line.split_once('=') {
                colors.push((key.trim().to_owned(), value.trim().to_owned()));
            }
        }
        let color = |key: &str| {
            colors
                .iter()
                .find(|(k, _)| k.eq_ignore_ascii_case(key))
                .and_then(|(_, v)| parse_rgb(v))
        };
        let name = contrast_name(color("Window")?).unwrap_or(stem).to_owned();
        let scheme = contrast_scheme(&name, color)?;
        let dark = scheme.dark();
        Some(DesktopScheme {
            id: format!("windows:{}", stem.to_lowercase()),
            name,
            light: (!dark).then(|| scheme.clone()),
            dark: dark.then_some(scheme),
        })
    }

    /// Windows 11's Contrast themes by their background color.
    fn contrast_name(window: u32) -> Option<&'static str> {
        match window {
            0x202020ff => Some("Aquatic"),
            0xfffaefff => Some("Desert"),
            0x2d3236ff => Some("Dusk"),
            0x000000ff => Some("Night sky"),
            _ => None,
        }
    }

    /// Reads Windows' colors from the registry and its theme files.
    #[cfg(windows)]
    pub fn read() -> SystemColors {
        use winreg::RegKey;
        use winreg::enums::HKEY_CURRENT_USER;

        let user = RegKey::predef(HKEY_CURRENT_USER);
        let accent = user
            .open_subkey("Software\\Microsoft\\Windows\\DWM")
            .and_then(|key| key.get_value::<u32, _>("AccentColor"))
            .ok()
            .map(accent_from_dword);
        let contrast_on = user
            .open_subkey("Control Panel\\Accessibility\\HighContrast")
            .and_then(|key| key.get_value::<String, _>("Flags"))
            .ok()
            .and_then(|flags| flags.trim().parse::<u32>().ok())
            .is_some_and(|flags| flags & 1 != 0);
        let mut schemes = vec![default_scheme(accent)];
        schemes.extend(contrast_themes());
        let colors = if contrast_on {
            let key = user.open_subkey("Control Panel\\Colors").ok();
            let color = |name: &str| {
                key.as_ref()?
                    .get_value::<String, _>(name)
                    .ok()
                    .and_then(|v| parse_rgb(&v))
            };
            contrast_scheme("Contrast", color).map(|scheme| SystemColors::only(scheme, accent))
        } else {
            None
        };
        colors
            .unwrap_or_else(|| SystemColors::accent_only(accent))
            .with_schemes(schemes)
    }

    #[cfg(not(windows))]
    pub fn read() -> SystemColors {
        SystemColors::default()
    }

    /// The Contrast themes in `%WINDIR%\Resources\Ease of Access Themes`.
    #[cfg(windows)]
    fn contrast_themes() -> Vec<DesktopScheme> {
        let windir = std::env::var_os("WINDIR").unwrap_or_else(|| "C:\\Windows".into());
        let dir = std::path::Path::new(&windir).join("Resources\\Ease of Access Themes");
        let Ok(entries) = std::fs::read_dir(dir) else {
            return Vec::new();
        };
        let mut themes: Vec<DesktopScheme> = entries
            .filter_map(Result::ok)
            .filter_map(|entry| {
                let path = entry.path();
                let stem = path.file_stem()?.to_str()?.to_owned();
                // Theme files are UTF-16 with a byte order mark, or ANSI.
                let bytes = std::fs::read(&path).ok()?;
                let text = match bytes.as_slice() {
                    [0xff, 0xfe, rest @ ..] => String::from_utf16_lossy(
                        &rest
                            .as_chunks::<2>()
                            .0
                            .iter()
                            .map(|&c| u16::from_le_bytes(c))
                            .collect::<Vec<_>>(),
                    ),
                    _ => String::from_utf8_lossy(&bytes).into_owned(),
                };
                parse_theme_file(&stem, &text)
            })
            .collect();
        themes.sort_by(|a, b| a.name.cmp(&b.name));
        themes
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
    #[test]
    fn panel_text_follows_plasma_and_is_light_elsewhere() {
        let dark = "[Colors:Window]\nForegroundNormal=252,252,252\n";
        assert_eq!(super::panel_text_on("KDE", Some(dark)), 0xfcfcfc);
        // Breeze Light when kdeglobals says nothing.
        assert_eq!(super::panel_text_on("KDE", None), 0x232629);
        assert_eq!(super::panel_text_on("ubuntu:GNOME", Some(dark)), 0xffffff);
    }

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

#[cfg(test)]
mod scheme_list_tests {
    use super::*;

    const BREEZE_LIGHT_FILE: &str = "[General]\nName=Breeze Light\n\
        [Colors:Window]\nBackgroundNormal=239,240,241\nForegroundNormal=35,38,41\n\
        [Colors:View]\nBackgroundNormal=255,255,255\nForegroundNormal=35,38,41\n";
    const BREEZE_DARK_FILE: &str = "[General]\nName=Breeze Dark\n\
        [Colors:Window]\nBackgroundNormal=32,35,38\nForegroundNormal=252,252,252\n\
        [Colors:View]\nBackgroundNormal=20,22,24\nForegroundNormal=252,252,252\n";
    const CLASSIC_FILE: &str = "[General]\nName=Breeze Classic\n\
        [Colors:Window]\nBackgroundNormal=239,240,241\n";

    fn files() -> Vec<(String, String)> {
        vec![
            ("BreezeDark".to_owned(), BREEZE_DARK_FILE.to_owned()),
            ("BreezeClassic".to_owned(), CLASSIC_FILE.to_owned()),
            ("BreezeLight".to_owned(), BREEZE_LIGHT_FILE.to_owned()),
        ]
    }

    #[test]
    fn pairs_light_and_dark_kde_schemes() {
        let schemes = pair_kde_schemes(&files());
        assert_eq!(schemes.len(), 2);
        let breeze = &schemes[0];
        assert_eq!(
            (breeze.id.as_str(), breeze.name.as_str()),
            ("kde:Breeze", "Breeze")
        );
        assert_eq!(breeze.light.as_ref().unwrap().view_bg, 0xffffffff);
        assert_eq!(breeze.dark.as_ref().unwrap().view_bg, 0x141618ff);
        // A scheme with one side gives it whatever is asked.
        let classic = &schemes[1];
        assert_eq!(classic.id, "kde:BreezeClassic");
        assert!(classic.dark.is_none());
        assert_eq!(classic.side(true), classic.light.as_ref());

        // The partner of the scheme in use.
        let partner = kde_partner("BreezeDark", &schemes).unwrap();
        assert_eq!(partner.name, "BreezeLight");
        assert!(kde_partner("BreezeClassic", &schemes).is_none());
    }

    #[test]
    fn kde_scheme_with_partner_gives_both_sides() {
        let schemes = pair_kde_schemes(&files());
        let (dark, _) = parse_kdeglobals(&format!(
            "[General]\nColorScheme=BreezeDark\n{BREEZE_DARK_FILE}"
        ));
        let partner = kde_partner(&dark.name, &schemes);
        let colors = SystemColors::kde_with_partner(dark.clone(), partner);
        assert_eq!(colors.scheme_for(true).unwrap().view_bg, 0x141618ff);
        assert_eq!(colors.scheme_for(false).unwrap().view_bg, 0xffffffff);
        assert_eq!(colors.forced_dark(), None);
        // Without a partner, the mode still decides; the light side has
        // no scheme.
        let alone = SystemColors::kde(dark);
        assert_eq!(alone.forced_dark(), None);
        assert!(alone.scheme_for(false).is_none());
    }

    #[test]
    fn reads_windows_colors() {
        assert_eq!(windows::accent_from_dword(0xffd47800), 0x0078d4ff);
        assert_eq!(windows::parse_rgb("255 250 239"), Some(0xfffaefff));
        assert_eq!(windows::parse_rgb("255 250"), None);
        let desert = windows::parse_theme_file(
            "hcwhite",
            "[Theme]\nDisplayName=@themeui.dll,-1\n[Control Panel\\Colors]\n\
             Window=255 250 239\nWindowText=61 61 61\nHilight=144 57 9\nHilightText=255 245 227\n",
        )
        .unwrap();
        assert_eq!(
            (desert.id.as_str(), desert.name.as_str()),
            ("windows:hcwhite", "Desert")
        );
        assert!(desert.dark.is_none());
        assert_eq!(desert.light.as_ref().unwrap().accent, 0x903909ff);
        let only = SystemColors::only(desert.light.clone().unwrap(), None);
        assert_eq!(only.forced_dark(), Some(false));
        assert!(only.scheme_for(true).is_some());
        let default = windows::default_scheme(Some(0x0078d4ff));
        assert_eq!(default.light.as_ref().unwrap().accent, 0x0078d4ff);
        assert!(default.dark.as_ref().unwrap().dark());
    }
}
