// SPDX-License-Identifier: GPL-3.0-or-later

//! The little CSS that mail uses in `style` attributes: colors, lengths and
//! a declaration list. Nothing here can load anything.

/// An sRGB color with alpha, as `0xRRGGBBAA`.
pub type Color = u32;

/// Parses a CSS color: `#rgb`, `#rgba`, `#rrggbb`, `#rrggbbaa`,
/// `rgb()`/`rgba()` and the named colors. `transparent` is fully
/// transparent; anything else unknown is `None`.
pub fn color(value: &str) -> Option<Color> {
    let value = value.trim().to_ascii_lowercase();
    if let Some(hex) = value.strip_prefix('#') {
        return hex_color(hex);
    }
    if let Some(args) = value
        .strip_prefix("rgba(")
        .or_else(|| value.strip_prefix("rgb("))
        .and_then(|rest| rest.strip_suffix(')'))
    {
        let parts: Vec<&str> = args
            .split([',', ' ', '/'])
            .map(str::trim)
            .filter(|p| !p.is_empty())
            .collect();
        if parts.len() < 3 {
            return None;
        }
        let channel = |p: &str| -> Option<u32> {
            let v = match p.strip_suffix('%') {
                Some(pct) => pct.parse::<f32>().ok()? * 2.55,
                None => p.parse::<f32>().ok()?,
            };
            Some(v.round().clamp(0.0, 255.0) as u32)
        };
        let alpha = match parts.get(3) {
            Some(a) => {
                let v = match a.strip_suffix('%') {
                    Some(pct) => pct.parse::<f32>().ok()? / 100.0,
                    None => a.parse::<f32>().ok()?,
                };
                (v.clamp(0.0, 1.0) * 255.0).round() as u32
            }
            None => 255,
        };
        return Some(
            (channel(parts[0])? << 24)
                | (channel(parts[1])? << 16)
                | (channel(parts[2])? << 8)
                | alpha,
        );
    }
    if value == "transparent" {
        return Some(0);
    }
    NAMED
        .binary_search_by_key(&value.as_str(), |(name, _)| name)
        .ok()
        .map(|ix| (NAMED[ix].1 << 8) | 0xff)
}

fn hex_color(hex: &str) -> Option<Color> {
    if !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let digit = |i: usize| u32::from_str_radix(&hex[i..=i], 16).ok();
    match hex.len() {
        3 | 4 => {
            let mut c = 0;
            for i in 0..3 {
                c = (c << 8) | (digit(i)? * 17);
            }
            let a = if hex.len() == 4 { digit(3)? * 17 } else { 255 };
            Some((c << 8) | a)
        }
        6 => Some((u32::from_str_radix(hex, 16).ok()? << 8) | 0xff),
        8 => u32::from_str_radix(hex, 16).ok(),
        _ => None,
    }
}

/// A length from CSS or an HTML attribute.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Length {
    Px(f32),
    /// A fraction of the containing box (1 is all of it).
    Percent(f32),
}

/// Parses a length: `12px`, `9pt`, `1.5em` (of `em` pixels), `50%`, or a
/// bare number (pixels, as in `width="600"`). `auto` and anything else
/// unknown is `None`.
pub fn length(value: &str, em: f32) -> Option<Length> {
    let value = value.trim().to_ascii_lowercase();
    let number = |s: &str| s.trim().parse::<f32>().ok().filter(|v| v.is_finite());
    if let Some(pct) = value.strip_suffix('%') {
        return Some(Length::Percent((number(pct)? / 100.0).max(0.0)));
    }
    let px = if let Some(v) = value.strip_suffix("px") {
        number(v)?
    } else if let Some(v) = value.strip_suffix("pt") {
        number(v)? * 4.0 / 3.0
    } else if let Some(v) = value.strip_suffix("rem") {
        number(v)? * 16.0
    } else if let Some(v) = value.strip_suffix("em") {
        number(v)? * em
    } else {
        number(&value)?
    };
    Some(Length::Px(px.max(0.0)))
}

/// A length in pixels; percentages are `None`.
pub fn px(value: &str, em: f32) -> Option<f32> {
    match length(value, em)? {
        Length::Px(px) => Some(px),
        Length::Percent(_) => None,
    }
}

/// A `font-size` in pixels, relative to the parent's `parent` pixels.
pub fn font_size(value: &str, parent: f32) -> Option<f32> {
    let value = value.trim().to_ascii_lowercase();
    let keyword = match value.as_str() {
        "xx-small" => Some(9.0),
        "x-small" => Some(10.0),
        "small" => Some(13.0),
        "medium" => Some(16.0),
        "large" => Some(18.0),
        "x-large" => Some(24.0),
        "xx-large" => Some(32.0),
        "smaller" => Some(parent / 1.2),
        "larger" => Some(parent * 1.2),
        _ => None,
    };
    keyword.or_else(|| match length(&value, parent)? {
        Length::Px(px) => Some(px),
        Length::Percent(p) => Some(parent * p),
    })
}

/// The declarations of a `style` attribute, names lower-cased, values
/// trimmed and without `!important`. Later ones win, so callers take the
/// last match.
pub fn declarations(style: &str) -> Vec<(String, String)> {
    style
        .split(';')
        .filter_map(|decl| {
            let (name, value) = decl.split_once(':')?;
            let name = name.trim().to_ascii_lowercase();
            let value = value.trim();
            let value = value
                .strip_suffix("!important")
                .or_else(|| value.strip_suffix("! important"))
                .unwrap_or(value)
                .trim();
            (!name.is_empty() && !value.is_empty()).then(|| (name, value.to_owned()))
        })
        .collect()
}

/// The four sides of `padding` or `margin` shorthand, in pixels: top,
/// right, bottom, left. Percentages and `auto` count as 0.
pub fn sides(value: &str, em: f32) -> [f32; 4] {
    let v: Vec<f32> = value
        .split_whitespace()
        .map(|p| px(p, em).unwrap_or(0.0))
        .collect();
    match v.as_slice() {
        [a] => [*a; 4],
        [a, b] => [*a, *b, *a, *b],
        [a, b, c] => [*a, *b, *c, *b],
        [a, b, c, d, ..] => [*a, *b, *c, *d],
        [] => [0.0; 4],
    }
}

/// The CSS named colors, sorted by name, as `0xRRGGBB`.
const NAMED: &[(&str, u32)] = &[
    ("aliceblue", 0xf0f8ff),
    ("antiquewhite", 0xfaebd7),
    ("aqua", 0x00ffff),
    ("aquamarine", 0x7fffd4),
    ("azure", 0xf0ffff),
    ("beige", 0xf5f5dc),
    ("bisque", 0xffe4c4),
    ("black", 0x000000),
    ("blanchedalmond", 0xffebcd),
    ("blue", 0x0000ff),
    ("blueviolet", 0x8a2be2),
    ("brown", 0xa52a2a),
    ("burlywood", 0xdeb887),
    ("cadetblue", 0x5f9ea0),
    ("chartreuse", 0x7fff00),
    ("chocolate", 0xd2691e),
    ("coral", 0xff7f50),
    ("cornflowerblue", 0x6495ed),
    ("cornsilk", 0xfff8dc),
    ("crimson", 0xdc143c),
    ("cyan", 0x00ffff),
    ("darkblue", 0x00008b),
    ("darkcyan", 0x008b8b),
    ("darkgoldenrod", 0xb8860b),
    ("darkgray", 0xa9a9a9),
    ("darkgreen", 0x006400),
    ("darkgrey", 0xa9a9a9),
    ("darkkhaki", 0xbdb76b),
    ("darkmagenta", 0x8b008b),
    ("darkolivegreen", 0x556b2f),
    ("darkorange", 0xff8c00),
    ("darkorchid", 0x9932cc),
    ("darkred", 0x8b0000),
    ("darksalmon", 0xe9967a),
    ("darkseagreen", 0x8fbc8f),
    ("darkslateblue", 0x483d8b),
    ("darkslategray", 0x2f4f4f),
    ("darkslategrey", 0x2f4f4f),
    ("darkturquoise", 0x00ced1),
    ("darkviolet", 0x9400d3),
    ("deeppink", 0xff1493),
    ("deepskyblue", 0x00bfff),
    ("dimgray", 0x696969),
    ("dimgrey", 0x696969),
    ("dodgerblue", 0x1e90ff),
    ("firebrick", 0xb22222),
    ("floralwhite", 0xfffaf0),
    ("forestgreen", 0x228b22),
    ("fuchsia", 0xff00ff),
    ("gainsboro", 0xdcdcdc),
    ("ghostwhite", 0xf8f8ff),
    ("gold", 0xffd700),
    ("goldenrod", 0xdaa520),
    ("gray", 0x808080),
    ("green", 0x008000),
    ("greenyellow", 0xadff2f),
    ("grey", 0x808080),
    ("honeydew", 0xf0fff0),
    ("hotpink", 0xff69b4),
    ("indianred", 0xcd5c5c),
    ("indigo", 0x4b0082),
    ("ivory", 0xfffff0),
    ("khaki", 0xf0e68c),
    ("lavender", 0xe6e6fa),
    ("lavenderblush", 0xfff0f5),
    ("lawngreen", 0x7cfc00),
    ("lemonchiffon", 0xfffacd),
    ("lightblue", 0xadd8e6),
    ("lightcoral", 0xf08080),
    ("lightcyan", 0xe0ffff),
    ("lightgoldenrodyellow", 0xfafad2),
    ("lightgray", 0xd3d3d3),
    ("lightgreen", 0x90ee90),
    ("lightgrey", 0xd3d3d3),
    ("lightpink", 0xffb6c1),
    ("lightsalmon", 0xffa07a),
    ("lightseagreen", 0x20b2aa),
    ("lightskyblue", 0x87cefa),
    ("lightslategray", 0x778899),
    ("lightslategrey", 0x778899),
    ("lightsteelblue", 0xb0c4de),
    ("lightyellow", 0xffffe0),
    ("lime", 0x00ff00),
    ("limegreen", 0x32cd32),
    ("linen", 0xfaf0e6),
    ("magenta", 0xff00ff),
    ("maroon", 0x800000),
    ("mediumaquamarine", 0x66cdaa),
    ("mediumblue", 0x0000cd),
    ("mediumorchid", 0xba55d3),
    ("mediumpurple", 0x9370db),
    ("mediumseagreen", 0x3cb371),
    ("mediumslateblue", 0x7b68ee),
    ("mediumspringgreen", 0x00fa9a),
    ("mediumturquoise", 0x48d1cc),
    ("mediumvioletred", 0xc71585),
    ("midnightblue", 0x191970),
    ("mintcream", 0xf5fffa),
    ("mistyrose", 0xffe4e1),
    ("moccasin", 0xffe4b5),
    ("navajowhite", 0xffdead),
    ("navy", 0x000080),
    ("oldlace", 0xfdf5e6),
    ("olive", 0x808000),
    ("olivedrab", 0x6b8e23),
    ("orange", 0xffa500),
    ("orangered", 0xff4500),
    ("orchid", 0xda70d6),
    ("palegoldenrod", 0xeee8aa),
    ("palegreen", 0x98fb98),
    ("paleturquoise", 0xafeeee),
    ("palevioletred", 0xdb7093),
    ("papayawhip", 0xffefd5),
    ("peachpuff", 0xffdab9),
    ("peru", 0xcd853f),
    ("pink", 0xffc0cb),
    ("plum", 0xdda0dd),
    ("powderblue", 0xb0e0e6),
    ("purple", 0x800080),
    ("rebeccapurple", 0x663399),
    ("red", 0xff0000),
    ("rosybrown", 0xbc8f8f),
    ("royalblue", 0x4169e1),
    ("saddlebrown", 0x8b4513),
    ("salmon", 0xfa8072),
    ("sandybrown", 0xf4a460),
    ("seagreen", 0x2e8b57),
    ("seashell", 0xfff5ee),
    ("sienna", 0xa0522d),
    ("silver", 0xc0c0c0),
    ("skyblue", 0x87ceeb),
    ("slateblue", 0x6a5acd),
    ("slategray", 0x708090),
    ("slategrey", 0x708090),
    ("snow", 0xfffafa),
    ("springgreen", 0x00ff7f),
    ("steelblue", 0x4682b4),
    ("tan", 0xd2b48c),
    ("teal", 0x008080),
    ("thistle", 0xd8bfd8),
    ("tomato", 0xff6347),
    ("turquoise", 0x40e0d0),
    ("violet", 0xee82ee),
    ("wheat", 0xf5deb3),
    ("white", 0xffffff),
    ("whitesmoke", 0xf5f5f5),
    ("yellow", 0xffff00),
    ("yellowgreen", 0x9acd32),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors() {
        assert_eq!(color("#fff"), Some(0xffffffff));
        assert_eq!(color("#1A73E8"), Some(0x1a73e8ff));
        assert_eq!(color("#1a73e880"), Some(0x1a73e880));
        assert_eq!(color("rgb(26, 115, 232)"), Some(0x1a73e8ff));
        assert_eq!(color("rgba(0,0,0,0.5)"), Some(0x00000080));
        assert_eq!(color("rgb(0 0 0 / 50%)"), Some(0x00000080));
        assert_eq!(color(" White "), Some(0xffffffff));
        assert_eq!(color("transparent"), Some(0));
        assert_eq!(color("#ggg"), None);
        assert_eq!(color("inherit"), None);
        assert!(NAMED.windows(2).all(|w| w[0].0 < w[1].0), "sorted");
    }

    #[test]
    fn lengths() {
        assert_eq!(length("600", 16.0), Some(Length::Px(600.0)));
        assert_eq!(length("12px", 16.0), Some(Length::Px(12.0)));
        assert_eq!(length("9pt", 16.0), Some(Length::Px(12.0)));
        assert_eq!(length("1.5em", 10.0), Some(Length::Px(15.0)));
        assert_eq!(length("50%", 16.0), Some(Length::Percent(0.5)));
        assert_eq!(length("auto", 16.0), None);
        assert_eq!(font_size("small", 16.0), Some(13.0));
        assert_eq!(font_size("150%", 10.0), Some(15.0));
        assert_eq!(sides("4px 8px", 16.0), [4.0, 8.0, 4.0, 8.0]);
        assert_eq!(sides("1px 2px 3px 4px", 16.0), [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn style_attribute() {
        assert_eq!(
            declarations("Color: red; display:none !important;;bad; font: x"),
            [
                ("color".to_owned(), "red".to_owned()),
                ("display".to_owned(), "none".to_owned()),
                ("font".to_owned(), "x".to_owned()),
            ]
        );
    }
}
