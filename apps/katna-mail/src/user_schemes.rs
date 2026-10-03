// SPDX-License-Identifier: GPL-3.0-or-later

//! Color schemes people make (Settings > Appearance > Colors > Yours):
//! one TOML file each in `<config>/colors/`, with the eight colors of a
//! light side, a dark side or both. They are listed beside the desktop's
//! as [`DesktopScheme`]s with ids `user:<file stem>`. A KDE `.colors` file
//! can be read in as one. No GPUI types here.
//!
//! ```toml
//! name = "My colors"
//!
//! [light]
//! page = "#f3f6f4"
//! cards = "#ffffff"
//! text = "#1b2420"
//! faint = "#5f6b66"
//! accent = "#00807f"
//! bar-text = "#1b2420"
//! on-accent = "#ffffff"
//! error = "#c5221f"
//! ```

use std::path::{Path, PathBuf};

use katna_platform::colors::{self, DesktopScheme, Scheme, contrast, luminance};
use serde::{Deserialize, Serialize};

/// The start of a scheme id that names a file of [`dir`].
pub const PREFIX: &str = "user:";

/// One of the eight colors of a side, in the order the editor lists them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Seed {
    Page,
    Cards,
    Text,
    Faint,
    Accent,
    BarText,
    OnAccent,
    Error,
}

impl Seed {
    pub const ALL: [Seed; 8] = [
        Seed::Page,
        Seed::Cards,
        Seed::Text,
        Seed::Faint,
        Seed::Accent,
        Seed::BarText,
        Seed::OnAccent,
        Seed::Error,
    ];

    /// The id of its name in `settings.ftl`.
    pub fn name(self) -> &'static str {
        match self {
            Seed::Page => "scheme-seed-page",
            Seed::Cards => "scheme-seed-cards",
            Seed::Text => "scheme-seed-text",
            Seed::Faint => "scheme-seed-faint",
            Seed::Accent => "scheme-seed-accent",
            Seed::BarText => "scheme-seed-bar-text",
            Seed::OnAccent => "scheme-seed-on-accent",
            Seed::Error => "scheme-seed-error",
        }
    }

    pub fn get(self, scheme: &Scheme) -> u32 {
        match self {
            Seed::Page => scheme.window_bg,
            Seed::Cards => scheme.view_bg,
            Seed::Text => scheme.view_fg,
            Seed::Faint => scheme.inactive_fg,
            Seed::Accent => scheme.accent,
            Seed::BarText => scheme.window_fg,
            Seed::OnAccent => scheme.accent_fg,
            Seed::Error => scheme.negative,
        }
    }

    pub fn set(self, scheme: &mut Scheme, color: u32) {
        let slot = match self {
            Seed::Page => &mut scheme.window_bg,
            Seed::Cards => &mut scheme.view_bg,
            Seed::Text => &mut scheme.view_fg,
            Seed::Faint => &mut scheme.inactive_fg,
            Seed::Accent => &mut scheme.accent,
            Seed::BarText => &mut scheme.window_fg,
            Seed::OnAccent => &mut scheme.accent_fg,
            Seed::Error => &mut scheme.negative,
        };
        *slot = color | 0xff;
    }
}

/// The folder of the schemes people made.
pub fn dir(config_dir: &Path) -> PathBuf {
    config_dir.join("colors")
}

/// Every scheme in `dir`, by name; files that can't be read are left out.
pub fn load_all(dir: &Path) -> Vec<DesktopScheme> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut schemes: Vec<DesktopScheme> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|e| e == "toml"))
        .filter_map(|path| {
            let stem = path.file_stem()?.to_str()?.to_owned();
            let text = std::fs::read_to_string(&path).ok()?;
            match parse(&stem, &text) {
                Ok(scheme) => Some(scheme),
                Err(err) => {
                    tracing::warn!(path = %path.display(), %err, "not a color scheme");
                    None
                }
            }
        })
        .collect();
    schemes.sort_by_key(|s| s.name.to_lowercase());
    schemes
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct File {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    light: Option<SideFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dark: Option<SideFile>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct SideFile {
    page: String,
    cards: String,
    text: String,
    faint: String,
    accent: String,
    bar_text: String,
    on_accent: String,
    error: String,
}

/// The scheme in the file `<stem>.toml` that holds `text`.
pub fn parse(stem: &str, text: &str) -> Result<DesktopScheme, String> {
    let file: File = toml::from_str(text).map_err(|err| err.message().to_owned())?;
    if file.light.is_none() && file.dark.is_none() {
        return Err("no [light] or [dark] colors".to_owned());
    }
    let side = |side: Option<SideFile>| -> Result<Option<Scheme>, String> {
        let Some(side) = side else {
            return Ok(None);
        };
        let color = |value: &str| {
            colors::parse_css_color(value).ok_or_else(|| format!("not a color: {value}"))
        };
        Ok(Some(Scheme {
            name: file.name.clone(),
            window_bg: color(&side.page)?,
            window_fg: color(&side.bar_text)?,
            view_bg: color(&side.cards)?,
            view_fg: color(&side.text)?,
            inactive_fg: color(&side.faint)?,
            accent: color(&side.accent)?,
            accent_fg: color(&side.on_accent)?,
            negative: color(&side.error)?,
        }))
    };
    let (light, dark) = (side(file.light)?, side(file.dark)?);
    Ok(DesktopScheme {
        id: format!("{PREFIX}{stem}"),
        name: file.name.trim().to_owned(),
        light,
        dark,
    })
}

/// `scheme` as its file's contents.
pub fn to_toml(scheme: &DesktopScheme) -> String {
    let hex = |color: u32| format!("#{:06x}", color >> 8);
    let side = |side: &Scheme| SideFile {
        page: hex(side.window_bg),
        cards: hex(side.view_bg),
        text: hex(side.view_fg),
        faint: hex(side.inactive_fg),
        accent: hex(side.accent),
        bar_text: hex(side.window_fg),
        on_accent: hex(side.accent_fg),
        error: hex(side.negative),
    };
    let file = File {
        name: scheme.name.clone(),
        light: scheme.light.as_ref().map(side),
        dark: scheme.dark.as_ref().map(side),
    };
    let text = toml::to_string(&file).unwrap_or_default();
    format!("# Katna Mail color scheme\n{text}")
}

/// The file of the scheme with `id`, if it is one of [`dir`]'s.
pub fn path(dir: &Path, id: &str) -> Option<PathBuf> {
    let stem = id.strip_prefix(PREFIX)?;
    (!stem.is_empty() && !stem.contains(['/', '\\', '.'])).then(|| dir.join(format!("{stem}.toml")))
}

/// A file stem for a new scheme called `name` that no file in `taken` has.
pub fn new_stem(name: &str, taken: &[DesktopScheme]) -> String {
    let mut base: String = name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    if base.is_empty() {
        base = "colors".to_owned();
    }
    let used = |stem: &str| taken.iter().any(|s| s.id == format!("{PREFIX}{stem}"));
    if !used(&base) {
        return base;
    }
    (2..)
        .map(|n| format!("{base}-{n}"))
        .find(|stem| !used(stem))
        .expect("a free name")
}

/// Writes `scheme` to its file in `dir`.
pub fn save(dir: &Path, scheme: &DesktopScheme) -> std::io::Result<()> {
    let path = path(dir, &scheme.id)
        .ok_or_else(|| std::io::Error::other(format!("not a scheme of yours: {}", scheme.id)))?;
    std::fs::create_dir_all(dir)?;
    let partial = path.with_extension("toml.partial");
    std::fs::write(&partial, to_toml(scheme))?;
    std::fs::rename(partial, path)
}

/// A KDE `.colors` file (the format of `kdeglobals`) as a scheme with one
/// side, light or dark as its colors are.
pub fn from_kde(stem: &str, contents: &str) -> DesktopScheme {
    let (mut scheme, _) = colors::parse_kdeglobals(contents);
    let name = contents
        .lines()
        .skip_while(|l| l.trim() != "[General]")
        .find_map(|l| l.trim().strip_prefix("Name="))
        .map(|name| name.trim().to_owned())
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| stem.to_owned());
    scheme.name = name.clone();
    let dark = scheme.dark();
    DesktopScheme {
        id: format!("{PREFIX}{stem}"),
        name,
        light: (!dark).then(|| scheme.clone()),
        dark: dark.then_some(scheme),
    }
}

/// A dark side worked out from a light one: the same hues, with the
/// backgrounds deep, the text light and the accent brightened.
pub fn dark_from_light(light: &Scheme) -> Scheme {
    let at = |color: u32, lightness: f32, saturation: f32| {
        let (h, s, _) = hsl(color);
        hsl_color(h, s * saturation, lightness)
    };
    let (_, _, accent_l) = hsl(light.accent);
    let accent = at(light.accent, accent_l.max(0.68), 1.0);
    let accent_fg = if luminance(accent) > 0.35 {
        0x1a1a1aff
    } else {
        0xffffffff
    };
    let text = at(light.view_fg, 0.92, 0.3);
    Scheme {
        name: light.name.clone(),
        window_bg: at(light.window_bg, 0.08, 0.5),
        window_fg: at(light.window_fg, 0.92, 0.3),
        view_bg: at(light.view_bg, 0.12, 0.5),
        view_fg: text,
        inactive_fg: at(light.inactive_fg, 0.66, 0.4),
        accent,
        accent_fg,
        negative: at(light.negative, 0.72, 0.9),
    }
}

/// What reads badly on a side: text on the cards under 4.5:1, faint text
/// under 3:1, text on the accent under 3:1.
pub fn low_contrast(side: &Scheme) -> Vec<(Seed, f32)> {
    [
        (Seed::Text, contrast(side.view_fg, side.view_bg), 4.5),
        (Seed::Faint, contrast(side.inactive_fg, side.view_bg), 3.0),
        (Seed::BarText, contrast(side.window_fg, side.window_bg), 4.5),
        (Seed::OnAccent, contrast(side.accent_fg, side.accent), 3.0),
    ]
    .into_iter()
    .filter(|(_, ratio, least)| ratio < least)
    .map(|(seed, ratio, _)| (seed, ratio))
    .collect()
}

/// `0xRRGGBBAA` as hue (0–360), saturation and lightness (0–1).
fn hsl(color: u32) -> (f32, f32, f32) {
    let channel = |shift: u32| ((color >> shift) & 0xff) as f32 / 255.0;
    let (r, g, b) = (channel(24), channel(16), channel(8));
    let (max, min) = (r.max(g).max(b), r.min(g).min(b));
    let l = (max + min) / 2.0;
    if max == min {
        return (0.0, 0.0, l);
    }
    let d = max - min;
    let s = if l > 0.5 {
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
    (h * 60.0, s, l)
}

/// A color from hue (0–360), saturation and lightness (0–1).
pub fn hsl_color(h: f32, s: f32, l: f32) -> u32 {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (r, g, b) = match (h / 60.0) as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let byte = |v: f32| ((v + m).clamp(0.0, 1.0) * 255.0).round() as u32;
    (byte(r) << 24) | (byte(g) << 16) | (byte(b) << 8) | 0xff
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINE: &str = r##"
name = "My colors"

[light]
page = "#f3f6f4"
cards = "#ffffff"
text = "#1b2420"
faint = "#5f6b66"
accent = "#00807f"
bar-text = "#1b2420"
on-accent = "#ffffff"
error = "#c5221f"
"##;

    #[test]
    fn reads_and_writes_the_same_scheme() {
        let scheme = parse("mine", MINE).unwrap();
        assert_eq!(scheme.id, "user:mine");
        assert_eq!(scheme.name, "My colors");
        let light = scheme.light.as_ref().unwrap();
        assert_eq!(light.view_fg, 0x1b2420ff);
        assert_eq!(light.accent, 0x00807fff);
        assert!(scheme.dark.is_none());
        assert_eq!(parse("mine", &to_toml(&scheme)).unwrap(), scheme);
        assert!(parse("x", "name = \"x\"\n").is_err(), "no sides");
        assert!(parse("x", &MINE.replace("#00807f", "teal-ish")).is_err());
    }

    #[test]
    fn saves_loads_and_names_files() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = dir(tmp.path());
        let mut scheme = parse("mine", MINE).unwrap();
        save(&dir, &scheme).unwrap();
        scheme.id = format!("{PREFIX}{}", new_stem("My colors", &[scheme.clone()]));
        assert_eq!(scheme.id, "user:my-colors");
        save(&dir, &scheme).unwrap();
        let all = load_all(&dir);
        assert_eq!(all.len(), 2);
        assert_eq!(new_stem("Mine", &all), "mine-2");
        assert_eq!(new_stem("!!", &[]), "colors");
        assert!(path(&dir, "user:../x").is_none());
        assert!(path(&dir, "nord").is_none());
    }

    #[test]
    fn imports_a_kde_scheme_as_one_side() {
        let kde = "[General]\nName=Honey\n[Colors:View]\nBackgroundNormal=30,30,30\n\
                   ForegroundNormal=240,240,240\n[Colors:Window]\nBackgroundNormal=20,20,20\n";
        let scheme = from_kde("Honey", kde);
        assert_eq!(scheme.name, "Honey");
        assert!(scheme.light.is_none());
        assert_eq!(scheme.dark.unwrap().view_bg, 0x1e1e1eff);
    }

    #[test]
    fn a_dark_side_from_a_light_one_reads_well() {
        let light = parse("mine", MINE).unwrap().light.unwrap();
        let dark = dark_from_light(&light);
        assert!(dark.dark());
        assert!(low_contrast(&dark).is_empty(), "{:?}", low_contrast(&dark));
        assert!(low_contrast(&light).is_empty());
        let mut bad = light.clone();
        Seed::Text.set(&mut bad, 0xdddddd00);
        assert_eq!(low_contrast(&bad)[0].0, Seed::Text);
        assert_eq!(Seed::Text.get(&bad), 0xddddddff);
    }

    #[test]
    fn every_color_has_an_english_name() {
        for seed in Seed::ALL {
            assert_ne!(katna_i18n::tr!(seed.name()), seed.name());
        }
    }

    #[test]
    fn hsl_round_trips() {
        for color in [0x00807fff, 0xc5221fff, 0xf3f6f4ff, 0x000000ff, 0xffffffff] {
            let (h, s, l) = hsl(color);
            let back = hsl_color(h, s, l);
            for shift in [24, 16, 8] {
                let (a, b) = ((color >> shift) & 0xff, (back >> shift) & 0xff);
                assert!(a.abs_diff(b) <= 1, "{color:08x} -> {back:08x}");
            }
        }
    }
}
