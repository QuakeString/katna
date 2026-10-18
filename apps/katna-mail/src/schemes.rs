// SPDX-License-Identifier: GPL-3.0-or-later

//! The color schemes built into Katna (`katna_ui::schemes`).

pub use katna_ui::schemes::*;
pub use katna_ui::theme::SideScheme;

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
