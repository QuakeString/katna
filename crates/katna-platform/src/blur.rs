// SPDX-License-Identifier: GPL-3.0-or-later

//! KDE's blur strength (System Settings > Desktop Effects > Blur), which
//! Katna's frosted menus and dialogs follow unless set by hand. It lives
//! in `kwinrc`; KWin takes no strength from the windows it blurs.

use std::path::{Path, PathBuf};

/// KWin's own value when `kwinrc` leaves it out.
const KWIN_DEFAULT: u8 = 15;
/// KWin's slider runs from 1 (light) to 15 (strong).
const KWIN_MAX: u8 = 15;

/// The file KWin keeps its settings in.
pub fn kwinrc(config_home: &Path) -> PathBuf {
    config_home.join("kwinrc")
}

/// KDE's blur strength, 1 to 15, or `None` when the Blur effect is off.
pub fn read_kde(config_home: &Path) -> Option<u8> {
    match std::fs::read_to_string(kwinrc(config_home)) {
        Ok(contents) => parse_kwinrc(&contents),
        // No file yet: KWin's defaults, with Blur on.
        Err(_) => Some(KWIN_DEFAULT),
    }
}

/// `BlurStrength` from `[Effect-blur]`, or `None` when `[Plugins]` turns
/// the effect off.
pub fn parse_kwinrc(contents: &str) -> Option<u8> {
    let mut group = "";
    let mut strength = KWIN_DEFAULT;
    let mut enabled = true;
    for line in contents.lines().map(str::trim) {
        if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            group = name;
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        match (group, key.trim()) {
            ("Plugins", "blurEnabled") => enabled = value.trim() != "false",
            ("Effect-blur", "BlurStrength") => {
                if let Ok(value) = value.trim().parse::<u8>() {
                    strength = value.clamp(1, KWIN_MAX);
                }
            }
            _ => {}
        }
    }
    enabled.then_some(strength)
}

/// The frost's blur, in pixels, for KDE's `strength`: from `light` at 1
/// to `full` at KWin's strongest, which is also its default.
pub fn kde_radius(strength: u8, light: f32, full: f32) -> f32 {
    let t = f32::from(strength.clamp(1, KWIN_MAX) - 1) / f32::from(KWIN_MAX - 1);
    light + t * (full - light)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_strength_and_the_switch() {
        assert_eq!(parse_kwinrc(""), Some(15));
        assert_eq!(parse_kwinrc("[Effect-blur]\nBlurStrength=6\n"), Some(6));
        assert_eq!(parse_kwinrc("[Effect-blur]\nBlurStrength=40\n"), Some(15));
        // The key only counts in its own group.
        assert_eq!(parse_kwinrc("[Other]\nBlurStrength=3\n"), Some(15));
        assert_eq!(
            parse_kwinrc("[Plugins]\nblurEnabled=false\n[Effect-blur]\nBlurStrength=6\n"),
            None
        );
        assert_eq!(parse_kwinrc("[Plugins]\nblurEnabled=true\n"), Some(15));
    }

    #[test]
    fn strength_maps_onto_the_range() {
        assert_eq!(kde_radius(1, 4.0, 24.0), 4.0);
        assert_eq!(kde_radius(15, 4.0, 24.0), 24.0);
        assert_eq!(kde_radius(8, 4.0, 24.0), 14.0);
    }
}
