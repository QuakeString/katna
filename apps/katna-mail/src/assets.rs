// SPDX-License-Identifier: GPL-3.0-or-later

//! Icons built into the binary, served to GPUI's `svg()` element.

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
    "clear-format",
    "close-full",
    "close",
    "compose",
    "contacts",
    "document",
    "download",
    "drafts",
    "drop-down",
    "emoji",
    "event",
    "expand",
    "feeds",
    "file",
    "folder",
    "font",
    "format-bold",
    "format-italic",
    "format-strike",
    "format-text",
    "format-underline",
    "forum",
    "forward",
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
    "link",
    "list-bulleted",
    "list-numbered",
    "lock",
    "mail",
    "mark-read",
    "menu",
    "minimize",
    "more",
    "move-to",
    "notch",
    "notes",
    "open-external",
    "open-full",
    "people",
    "person-add",
    "plain-text",
    "print",
    "quote",
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
    "snooze",
    "spell-check",
    "star-filled",
    "star",
    "table",
    "tag",
    "tasks",
    "text-color",
    "text-size",
    "tour",
    "trash",
    "tune",
    "undo",
    "warning",
    "window-restore",
    "zoom-in",
    "zoom-out",
);

pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
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
}
