// SPDX-License-Identifier: GPL-3.0-or-later

//! Icons built into the binary, served to GPUI's `svg()` element, and the
//! language picker's flags (`flags/`, from `flag-icons`, MIT) and Katna's
//! logo (`logo/`), served to `img()`.

use std::borrow::Cow;

use gpui::{AssetSource, SharedString};

macro_rules! icons {
    ($($name:literal),* $(,)?) => {
        const ICONS: &[(&str, &[u8])] = &[
            $((concat!("icons/", $name, ".svg"), include_bytes!(concat!("../icons/", $name, ".svg")))),*
        ];
    };
}

icons!(
    "add",
    "align-center",
    "align-left",
    "align-right",
    "all-mail",
    "archive",
    "attachment",
    "back",
    "bolt",
    "calendar",
    "check-circle",
    "check",
    "checkbox-checked",
    "checkbox-partial",
    "checkbox",
    "chevron-down",
    "chevron-left",
    "chevron-right",
    "chevron-up",
    "clear-format",
    "close-full",
    "close",
    "coffee",
    "compose",
    "contacts",
    "document",
    "download",
    "drafts",
    "drag-handle",
    "drop-down",
    "emoji",
    "event",
    "expand",
    "eye",
    "feeds",
    "file",
    "folder",
    "folders-pane-fill",
    "folders-pane",
    "font",
    "format-bold",
    "format-italic",
    "format-strike",
    "format-text",
    "format-underline",
    "forum",
    "forward",
    "heart",
    "highlight",
    "image",
    "important-filled",
    "important",
    "inbox",
    "indent-less",
    "indent-more",
    "info",
    "junk",
    "label",
    "language",
    "link",
    "list-bulleted",
    "list-numbered",
    "lock",
    "mail",
    "mark-read",
    "minimize",
    "more",
    "move-to",
    "notch",
    "notes",
    "open-external",
    "open-full",
    "people",
    "person-add",
    "phone",
    "pin-filled",
    "pin",
    "plain-text",
    "print",
    "quote",
    "read-receipt",
    "redo",
    "refresh",
    "reply-all",
    "reply",
    "restore",
    "schedule",
    "search",
    "send",
    "sent",
    "settings",
    "sheet",
    "shield-alert",
    "shield-check",
    "shield",
    "signature",
    "slides",
    "snooze",
    "sparkle",
    "spell-check",
    "star-filled",
    "star",
    "table",
    "tag",
    "tasks",
    "template",
    "text-color",
    "text-size",
    "tour",
    "trash",
    "tune",
    "undo",
    "unread",
    "warning",
    "window-restore",
    "work",
    "zoom-in",
    "zoom-out",
);

macro_rules! flags {
    ($($code:literal),* $(,)?) => {
        const FLAGS: &[(&str, &[u8])] = &[
            $((concat!("flags/", $code, ".svg"), include_bytes!(concat!("../flags/", $code, ".svg")))),*
        ];
    };
}

flags!(
    "bd", "br", "bt", "cn", "de", "es", "et", "fr", "gb", "id", "il", "in", "ir", "it", "jp", "ke",
    "kh", "kr", "la", "lk", "mm", "my", "ng", "nl", "np", "ph", "pk", "pl", "ru", "sa", "se", "th",
    "tr", "tw", "ua", "us", "vn", "za",
);

/// The flags are 4:3 with only a view box; GPUI draws an SVG picture at
/// its own size, so each is given 48 × 36, twice the size it is shown.
fn flag(data: &[u8]) -> Vec<u8> {
    let text = String::from_utf8_lossy(data);
    text.replacen("<svg ", r#"<svg width="48" height="36" "#, 1)
        .into_bytes()
}

/// Katna's logo in full, and its small form for under 48 px.
const LOGO: &[u8] = include_bytes!("../../../packaging/icons/src/katna.svg");
const LOGO_SMALL: &[u8] = include_bytes!("../../../packaging/icons/src/katna-small.svg");

/// Where [`Assets`] serves the logo for a `size` px square (GPUI pixels).
pub fn logo_path(size: f32) -> String {
    let size = size.round().clamp(1.0, 1024.0) as u32;
    let form = if size < 48 { "small" } else { "full" };
    format!("logo/{form}-{size}.svg")
}

/// The logo for `logo/{full,small}-<size>.svg`. GPUI draws an SVG picture
/// at twice its own size, so giving it the size it is shown at renders it
/// sharp on double-density screens without sampling it down much.
fn logo(path: &str) -> Option<Vec<u8>> {
    let rest = path.strip_prefix("logo/")?.strip_suffix(".svg")?;
    let (form, size) = rest.split_once('-')?;
    let size: u32 = size.parse().ok().filter(|s| (1..=1024).contains(s))?;
    let data = match form {
        "full" => LOGO,
        "small" => LOGO_SMALL,
        _ => return None,
    };
    let text = String::from_utf8_lossy(data);
    Some(
        text.replacen(
            r#"width="128" height="128""#,
            &format!(r#"width="{size}" height="{size}""#),
            1,
        )
        .into_bytes(),
    )
}

pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        if path.starts_with("flags/") {
            return Ok(FLAGS
                .iter()
                .find(|(name, _)| *name == path)
                .map(|(_, data)| Cow::Owned(flag(data))));
        }
        if path.starts_with("logo/") {
            return Ok(logo(path).map(Cow::Owned));
        }
        Ok(ICONS
            .iter()
            .find(|(name, _)| *name == path)
            .map(|(_, data)| Cow::Borrowed(*data)))
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(ICONS
            .iter()
            .filter(|(name, _)| name.starts_with(path))
            .map(|(name, _)| SharedString::from(*name))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_icon_file_is_built_in() {
        let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/icons");
        let mut files: Vec<String> = std::fs::read_dir(dir)
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        files.sort();
        let built_in: Vec<String> = ICONS
            .iter()
            .map(|(name, _)| name.trim_start_matches("icons/").to_owned())
            .collect();
        assert_eq!(files, built_in);
        assert!(Assets.load("icons/star.svg").unwrap().is_some());
        assert!(Assets.load("icons/nope.svg").unwrap().is_none());
    }

    #[test]
    fn every_language_has_its_flag() {
        for language in katna_i18n::picker() {
            let path = format!("flags/{}.svg", language.flag);
            let data = Assets.load(&path).unwrap();
            assert!(data.is_some(), "{path}");
            assert!(String::from_utf8_lossy(&data.unwrap()).contains(r#"width="48""#));
        }
    }

    #[test]
    fn logo_is_served_at_its_size_in_the_form_for_it() {
        assert_eq!(logo_path(32.0), "logo/small-32.svg");
        assert_eq!(logo_path(64.0), "logo/full-64.svg");
        let small = Assets.load(&logo_path(40.0)).unwrap().unwrap();
        let small = String::from_utf8_lossy(&small);
        assert!(small.contains(r#"width="40" height="40""#));
        assert!(small.contains("small form"));
        let full = Assets.load("logo/full-96.svg").unwrap().unwrap();
        assert!(String::from_utf8_lossy(&full).contains(r#"width="96" height="96""#));
        for bad in ["logo/full-0.svg", "logo/big-64.svg", "logo/full.svg"] {
            assert!(Assets.load(bad).unwrap().is_none(), "{bad}");
        }
    }

    #[test]
    fn installed_icon_has_no_filters_for_qt() {
        // KDE draws the scalable icon with Qt SVG, which skips blur filters.
        let icon = include_str!("../../../packaging/icons/in.invenia.katna.Mail.svg");
        assert!(!icon.contains("filter"));
        assert!(!include_str!("../../../packaging/icons/src/katna-small.svg").contains("filter"));
    }
}
