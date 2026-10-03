// SPDX-License-Identifier: GPL-3.0-or-later

//! The desktop's animation speed: how long its animations take compared
//! with normal, and whether they are off.
//!
//! - **KDE**: `AnimationDurationFactor` in the `[KDE]` group of
//!   `kdeglobals` (System Settings > Animation speed): 1 is normal, 0.5
//!   twice as fast, 2 twice as slow, 0 instant.
//! - **GNOME and others**: the `enable-animations` GSettings key; off
//!   means no animations.
//! - **Windows**: not read yet; normal speed.

use std::path::Path;
use std::process::Command;

/// What the desktop says about animations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DesktopMotion {
    /// How long animations take compared with normal (1); 0 when the
    /// desktop turned them off.
    pub duration_factor: f32,
}

impl Default for DesktopMotion {
    fn default() -> Self {
        Self {
            duration_factor: 1.0,
        }
    }
}

impl DesktopMotion {
    /// The desktop turned animations off.
    pub fn off(&self) -> bool {
        self.duration_factor <= 0.0
    }
}

/// Reads `AnimationDurationFactor=` of the `[KDE]` group of a `kdeglobals`
/// file.
pub fn parse_kdeglobals(contents: &str) -> Option<f32> {
    let mut in_kde = false;
    for line in contents.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_kde = line == "[KDE]";
            continue;
        }
        if in_kde && let Some(value) = line.strip_prefix("AnimationDurationFactor=") {
            return value
                .trim()
                .parse::<f32>()
                .ok()
                .filter(|f| f.is_finite() && *f >= 0.0);
        }
    }
    None
}

/// Reads a GSettings boolean as printed by `gsettings get`.
pub fn parse_gsettings_bool(value: &str) -> Option<bool> {
    match value.trim() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

/// The desktop's animation setting. `kde` selects `kdeglobals` (under
/// `config_home`, normally `~/.config`) instead of GSettings.
pub fn read(kde: bool, config_home: Option<&Path>) -> DesktopMotion {
    let factor = if kde {
        config_home
            .and_then(|home| std::fs::read_to_string(home.join("kdeglobals")).ok())
            .and_then(|contents| parse_kdeglobals(&contents))
    } else if cfg!(target_os = "windows") {
        None
    } else {
        gsettings_animations().map(|on| if on { 1.0 } else { 0.0 })
    };
    DesktopMotion {
        duration_factor: factor.unwrap_or(1.0),
    }
}

fn gsettings_animations() -> Option<bool> {
    let output = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "enable-animations"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    parse_gsettings_bool(&String::from_utf8_lossy(&output.stdout))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_kde_animation_speed() {
        let file = "[General]\nfont=Noto Sans,10\n\n[KDE]\nAnimationDurationFactor=0.5\nLookAndFeelPackage=x\n";
        assert_eq!(parse_kdeglobals(file), Some(0.5));
        assert_eq!(
            parse_kdeglobals("[KDE]\nAnimationDurationFactor=0\n"),
            Some(0.0)
        );
        assert_eq!(
            parse_kdeglobals("[General]\nAnimationDurationFactor=2\n"),
            None
        );
        assert_eq!(
            parse_kdeglobals("[KDE]\nAnimationDurationFactor=-1\n"),
            None
        );
    }

    #[test]
    fn reads_gnome_animations() {
        assert_eq!(parse_gsettings_bool("true\n"), Some(true));
        assert_eq!(parse_gsettings_bool("false"), Some(false));
        assert_eq!(parse_gsettings_bool("'x'"), None);
        assert!(
            DesktopMotion {
                duration_factor: 0.0
            }
            .off()
        );
    }
}
