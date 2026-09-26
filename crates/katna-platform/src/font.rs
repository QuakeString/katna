// SPDX-License-Identifier: GPL-3.0-or-later

//! The system UI font (part of plan task 3.3).
//!
//! GPUI maps its system font to IBM Plex Sans on Linux, which few systems
//! have; without it, text falls back to a face without bold. So the apps
//! ask the desktop: `font` in `kdeglobals` on KDE, the `font-name`
//! GSettings key elsewhere. The result is checked against the installed
//! families, with a per-desktop list of common fonts as the fallback.

use std::path::Path;
use std::process::Command;

/// A font setting of the desktop.
#[derive(Debug, Clone, PartialEq)]
pub struct UiFont {
    pub family: String,
    /// Size in points, if the setting has one.
    pub size_pt: Option<f32>,
}

/// Fallbacks, most likely first, when the desktop's font is unknown or
/// not installed.
const GNOME_FALLBACKS: &[&str] = &[
    "Adwaita Sans",
    "Cantarell",
    "Inter",
    "Noto Sans",
    "Ubuntu",
    "DejaVu Sans",
    "Liberation Sans",
];
const KDE_FALLBACKS: &[&str] = &[
    "Noto Sans",
    "Inter",
    "Adwaita Sans",
    "Cantarell",
    "DejaVu Sans",
    "Liberation Sans",
];

/// Parses a GSettings font value such as `'Cantarell 11'` (as printed by
/// `gsettings get`) or `Adwaita Sans 11`.
pub fn parse_gsettings_font(value: &str) -> Option<UiFont> {
    let value = value.trim().trim_matches('\'').trim();
    if value.is_empty() {
        return None;
    }
    // Pango font descriptions end with the size; styles such as `Bold`
    // may come before it, but UI font settings rarely have them.
    let (family, size_pt) = match value.rsplit_once(' ') {
        Some((family, size)) => match size.parse::<f32>() {
            Ok(size) => (family.trim(), Some(size)),
            Err(_) => (value, None),
        },
        None => (value, None),
    };
    (!family.is_empty()).then(|| UiFont {
        family: family.to_owned(),
        size_pt,
    })
}

/// Reads `font=` of the `[General]` group of a `kdeglobals` file, for
/// example `font=Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1`.
pub fn parse_kdeglobals_font(contents: &str) -> Option<UiFont> {
    let mut in_general = false;
    for line in contents.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_general = line == "[General]";
            continue;
        }
        if !in_general {
            continue;
        }
        let Some(value) = line.strip_prefix("font=") else {
            continue;
        };
        let mut fields = value.split(',');
        let family = fields.next()?.trim();
        if family.is_empty() {
            return None;
        }
        return Some(UiFont {
            family: family.to_owned(),
            size_pt: fields.next().and_then(|s| s.trim().parse().ok()),
        });
    }
    None
}

/// The desktop's UI font setting, if it can be read. `kde` selects
/// `kdeglobals` (under `config_home`, normally `~/.config`) instead of
/// GSettings.
pub fn desktop_ui_font(kde: bool, config_home: &Path) -> Option<UiFont> {
    if kde {
        let contents = std::fs::read_to_string(config_home.join("kdeglobals")).ok()?;
        return parse_kdeglobals_font(&contents);
    }
    let output = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "font-name"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    parse_gsettings_font(&String::from_utf8_lossy(&output.stdout))
}

/// The family to use: `preferred` if installed, else the first installed
/// fallback for the desktop. Matching ignores case.
pub fn pick_family(preferred: Option<&str>, installed: &[String], kde: bool) -> Option<String> {
    let fallbacks = if kde { KDE_FALLBACKS } else { GNOME_FALLBACKS };
    preferred
        .into_iter()
        .chain(fallbacks.iter().copied())
        .find_map(|want| {
            installed
                .iter()
                .find(|have| have.eq_ignore_ascii_case(want))
                .cloned()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gsettings_values() {
        assert_eq!(
            parse_gsettings_font("'Cantarell 11'\n"),
            Some(UiFont {
                family: "Cantarell".into(),
                size_pt: Some(11.0)
            })
        );
        assert_eq!(
            parse_gsettings_font("Adwaita Sans 10.5").unwrap().family,
            "Adwaita Sans"
        );
        assert_eq!(parse_gsettings_font("'Inter'").unwrap().size_pt, None);
        assert_eq!(parse_gsettings_font("''"), None);
    }

    #[test]
    fn kdeglobals() {
        let file = "[Colors:View]\nfont=Wrong,9\n\n[General]\nColorScheme=BreezeLight\n\
                    font=Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1\n";
        assert_eq!(
            parse_kdeglobals_font(file),
            Some(UiFont {
                family: "Noto Sans".into(),
                size_pt: Some(10.0)
            })
        );
        assert_eq!(parse_kdeglobals_font("[General]\nfont=\n"), None);
        assert_eq!(parse_kdeglobals_font("[KDE]\nfont=Noto Sans,10\n"), None);
    }

    #[test]
    fn picking() {
        let installed: Vec<String> = ["DejaVu Sans", "Noto Sans", "Cantarell"]
            .map(String::from)
            .to_vec();
        assert_eq!(
            pick_family(Some("cantarell"), &installed, true).as_deref(),
            Some("Cantarell")
        );
        assert_eq!(
            pick_family(Some("Adwaita Sans"), &installed, false).as_deref(),
            Some("Cantarell")
        );
        assert_eq!(
            pick_family(None, &installed, true).as_deref(),
            Some("Noto Sans")
        );
        assert_eq!(pick_family(None, &[], false), None);
    }
}
