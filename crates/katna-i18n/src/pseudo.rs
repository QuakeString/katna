// SPDX-License-Identifier: GPL-3.0-or-later

//! Pseudo-languages for testing: English with every letter accented and
//! each text about 40 % longer, in brackets. Text that stays plain was
//! never converted; a label cut short has no room for a longer language.
//! `qps-plocm` also mirrors the layout, to find right-to-left bugs without
//! reading Arabic.

use std::borrow::Cow;

use crate::languages::{Language, Status};

pub(crate) const PLOC: &str = "qps-ploc";
pub(crate) const PLOCM: &str = "qps-plocm";

pub(crate) fn languages() -> [Language; 2] {
    let make = |tag: &str, english: &str, rtl: bool| Language {
        tag: tag.to_owned(),
        name: english.to_owned(),
        english: english.to_owned(),
        flag: "un".to_owned(),
        translation: "en".to_owned(),
        formats: "en-US".to_owned(),
        rtl,
        status: Status::Source,
        reviewers: Vec::new(),
        hidden: true,
    };
    [
        make(PLOC, "Pseudo (accented)", false),
        make(PLOCM, "Pseudo (mirrored)", true),
    ]
}

/// Whether `tag` is one of the pseudo-languages.
pub(crate) fn is_pseudo(tag: &str) -> bool {
    tag == PLOC || tag == PLOCM
}

/// A text element of a message, accented and lengthened. Fluent calls it
/// for the text between variables only, so variables stay as they are.
pub(crate) fn transform(text: &str) -> Cow<'_, str> {
    if text.trim().is_empty() {
        return Cow::Borrowed(text);
    }
    let mut out = String::with_capacity(text.len() * 2);
    out.push('[');
    let mut letters = 0_usize;
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            letters += 1;
        }
        out.push(accent(c));
    }
    for _ in 0..letters.div_ceil(3) {
        out.push('·');
    }
    out.push(']');
    Cow::Owned(out)
}

fn accent(c: char) -> char {
    match c {
        'a' => 'á',
        'b' => 'ƀ',
        'c' => 'ç',
        'd' => 'ð',
        'e' => 'é',
        'f' => 'ƒ',
        'g' => 'ĝ',
        'h' => 'ĥ',
        'i' => 'í',
        'j' => 'ĵ',
        'k' => 'ķ',
        'l' => 'ĺ',
        'm' => 'ɱ',
        'n' => 'ñ',
        'o' => 'ö',
        'p' => 'þ',
        'q' => 'ǫ',
        'r' => 'ŕ',
        's' => 'š',
        't' => 'ţ',
        'u' => 'ü',
        'v' => 'ṽ',
        'w' => 'ŵ',
        'x' => 'ẋ',
        'y' => 'ý',
        'z' => 'ž',
        'A' => 'Å',
        'B' => 'Ɓ',
        'C' => 'Ç',
        'D' => 'Ð',
        'E' => 'É',
        'F' => 'Ƒ',
        'G' => 'Ĝ',
        'H' => 'Ĥ',
        'I' => 'Î',
        'J' => 'Ĵ',
        'K' => 'Ķ',
        'L' => 'Ĺ',
        'M' => 'Ṁ',
        'N' => 'Ñ',
        'O' => 'Ö',
        'P' => 'Þ',
        'Q' => 'Ǫ',
        'R' => 'Ŕ',
        'S' => 'Š',
        'T' => 'Ţ',
        'U' => 'Û',
        'V' => 'Ṽ',
        'W' => 'Ŵ',
        'X' => 'Ẋ',
        'Y' => 'Ý',
        'Z' => 'Ž',
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accents_and_lengthens() {
        assert_eq!(transform("Compose"), "[Çöɱþöšé···]");
        assert_eq!(transform(" "), " ");
    }
}
