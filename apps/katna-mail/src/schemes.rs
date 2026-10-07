// SPDX-License-Identifier: GPL-3.0-or-later

//! The color schemes built into Katna (`katna_ui::schemes`), as the
//! desktop schemes `Theme::from_scheme` takes.

use katna_platform::colors::Scheme;
pub use katna_ui::schemes::*;

/// A built-in side as a desktop scheme.
pub trait SideScheme {
    /// The side as a desktop scheme, for `Theme::from_scheme`.
    fn scheme(&self, name: &str) -> Scheme;
}

impl SideScheme for Side {
    fn scheme(&self, name: &str) -> Scheme {
        Scheme {
            name: name.to_owned(),
            window_bg: self.page,
            window_fg: self.text,
            view_bg: self.card,
            view_fg: self.text,
            inactive_fg: self.faint,
            accent: self.accent,
            accent_fg: 0xffffffff,
            negative: self.error,
            neutral: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use katna_platform::colors::contrast;

    #[test]
    fn every_side_is_readable_and_the_right_shade() {
        let mut ids: Vec<_> = BUILT_IN.iter().map(|s| s.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), BUILT_IN.len(), "ids are unique");
        assert!(BUILT_IN.len() >= 10);
        for scheme in BUILT_IN {
            for dark in [false, true] {
                let side = scheme.side(dark);
                assert_eq!(side.scheme("").dark(), dark, "{} {dark}", scheme.id);
                let ratio = contrast(side.text, side.card);
                assert!(ratio >= 4.5, "{} {dark}: text {ratio}", scheme.id);
                // The open folder or contact is a soft grey of the text,
                // never the accent.
                let th = crate::theme::Theme::from_scheme(&side.scheme(""));
                assert_eq!(th.row_selected >> 8, side.text >> 8, "{}", scheme.id);
            }
            assert_ne!(katna_i18n::tr!(scheme.name), scheme.name, "no English name");
        }
    }
}
