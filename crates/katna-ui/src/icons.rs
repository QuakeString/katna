// SPDX-License-Identifier: GPL-3.0-or-later

//! Icons that point along the line of text, and so turn around in a
//! right-to-left layout (Arabic, Hebrew, Persian, Urdu): back and forward,
//! reply, send, chevrons, undo and redo, indents, the side pane. The rest keep their
//! look whatever the direction: a clock, a play button, a check mark,
//! logos, up and down arrows.

use gpui::{SharedString, Svg, svg};

/// The icons (file names under `icons/`, without `.svg`) drawn mirrored
/// in a right-to-left layout.
const DIRECTIONAL: &[&str] = &[
    "back",
    "chevron-left",
    "chevron-right",
    "folders-pane",
    "folders-pane-fill",
    "forward",
    "indent-less",
    "indent-more",
    "list-bulleted",
    "move-to",
    "redo",
    "reply",
    "reply-all",
    "send",
    "sent",
    "subtask",
    "undo",
];

/// Whether the icon `name` turns around in a right-to-left layout.
pub fn mirrors(name: &str) -> bool {
    DIRECTIONAL.binary_search(&name).is_ok()
}

/// An SVG of the icon `name` (`icons/<name>.svg`), mirrored in a
/// right-to-left layout when it points along the line.
pub fn icon_svg(name: &str) -> Svg {
    let svg = svg().path(SharedString::from(format!("icons/{name}.svg")));
    if mirrors(name) { svg.mirror_rtl() } else { svg }
}

/// As [`icon_svg`], from a path such as `icons/back.svg`.
pub fn path_svg(path: impl Into<SharedString>) -> Svg {
    let path = path.into();
    let name = path
        .strip_prefix("icons/")
        .and_then(|name| name.strip_suffix(".svg"));
    let mirrored = name.is_some_and(mirrors);
    let svg = svg().path(path);
    if mirrored { svg.mirror_rtl() } else { svg }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_list_is_sorted_for_its_search() {
        assert!(DIRECTIONAL.windows(2).all(|w| w[0] < w[1]));
    }

    #[test]
    fn arrows_along_the_line_mirror_and_the_rest_stay() {
        for name in ["back", "reply", "reply-all", "forward", "send", "chevron-right"] {
            assert!(mirrors(name), "{name}");
        }
        for name in ["schedule", "play", "check", "arrow-up", "chevron-down", "star"] {
            assert!(!mirrors(name), "{name}");
        }
    }
}
