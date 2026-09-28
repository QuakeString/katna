// SPDX-License-Identifier: GPL-3.0-or-later

//! The size of KDE's Breeze title bar buttons, so Katna's own frame matches
//! the windows around it.
//!
//! Breeze sizes its buttons from the font (twice KDecoration's grid unit,
//! about 24 px with the default Noto Sans 10) times the "Button size" of
//! System Settings > Colors & Themes > Window Decorations > Breeze
//! (`ButtonSize` in the `[Windeco]` group of `breezerc`).

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// Button width in logical pixels with the default font and button size.
const DEFAULT_SIZE: f32 = 24.0;
/// The UI font size, in points, that [`DEFAULT_SIZE`] goes with.
const DEFAULT_FONT_PT: f32 = 10.0;

/// The desktop's Breeze button width in logical pixels, read once.
pub fn button_size() -> f32 {
    static SIZE: OnceLock<f32> = OnceLock::new();
    *SIZE.get_or_init(|| match config_home() {
        Some(home) => button_size_in(&home),
        None => DEFAULT_SIZE,
    })
}

fn config_home() -> Option<PathBuf> {
    std::env::var_os("XDG_CONFIG_HOME")
        .filter(|dir| !dir.is_empty())
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| Path::new(&home).join(".config")))
}

/// The Breeze button width for the settings under `config_home`.
pub fn button_size_in(config_home: &Path) -> f32 {
    let read = |name: &str| std::fs::read_to_string(config_home.join(name)).unwrap_or_default();
    let font_pt = font_pt(&read("kdeglobals")).unwrap_or(DEFAULT_FONT_PT);
    let scale = button_scale(&read("breezerc"));
    (DEFAULT_SIZE * font_pt / DEFAULT_FONT_PT * scale).round()
}

/// Breeze's "Button size" as a factor of its default.
pub fn button_scale(breezerc: &str) -> f32 {
    match value(breezerc, "Windeco", "ButtonSize") {
        Some("ButtonTiny") => 0.5,
        Some("ButtonSmall") => 0.75,
        Some("ButtonLarge") => 1.25,
        Some("ButtonVeryLarge") => 1.75,
        _ => 1.0,
    }
}

/// The point size of `font=` in the `[General]` group of `kdeglobals`, for
/// example `font=Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1`.
fn font_pt(kdeglobals: &str) -> Option<f32> {
    let size: f32 = value(kdeglobals, "General", "font")?
        .split(',')
        .nth(1)?
        .trim()
        .parse()
        .ok()?;
    (6.0..=48.0).contains(&size).then_some(size)
}

fn value<'a>(contents: &'a str, group: &str, key: &str) -> Option<&'a str> {
    let mut in_group = false;
    for line in contents.lines() {
        let line = line.trim();
        if let Some(name) = line.strip_prefix('[') {
            in_group = name.strip_suffix(']') == Some(group);
            continue;
        }
        if in_group
            && let Some((k, v)) = line.split_once('=')
            && k.trim() == key
        {
            return Some(v.trim());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_button_size() {
        assert_eq!(button_scale(""), 1.0);
        assert_eq!(button_scale("[Windeco]\nButtonSize=ButtonLarge\n"), 1.25);
        assert_eq!(button_scale("[Windeco]\nButtonSize=ButtonTiny\n"), 0.5);
        assert_eq!(button_scale("[Common]\nButtonSize=ButtonLarge\n"), 1.0);
    }

    #[test]
    fn follows_the_font() {
        assert_eq!(
            font_pt("[General]\nfont=Noto Sans,11,-1,5,400,0\n"),
            Some(11.0)
        );
        assert_eq!(font_pt("[WM]\nfont=Noto Sans,11\n"), None);
        assert_eq!(font_pt("[General]\nfont=Noto Sans\n"), None);
    }

    #[test]
    fn defaults_without_settings() {
        assert_eq!(button_size_in(Path::new("/nonexistent")), DEFAULT_SIZE);
    }
}
