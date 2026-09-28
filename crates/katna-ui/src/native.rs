// SPDX-License-Identifier: GPL-3.0-or-later

//! What Katna adds to GPUI's platform backend, in one place for every
//! system. On Linux this is Katna's patched copy of GPUI's backend
//! (`vendor/gpui-pre-linux/KATNA.md`): rich clipboard and drops, window
//! placement, compositor blur, the KDE global menu and raising a window
//! with another app's activation token. On Windows GPUI's
//! own backend is used and these fall back to what it does by itself.

#[cfg(not(windows))]
pub use gpui_linux::{
    DroppedContent, Placement, clipboard_html, compositor_blur, dropped_content, html_item,
    placement_session, read_rich, restore_placement, set_activation_token,
    set_client_corner_radius, set_kde_appmenu,
};

#[cfg(windows)]
pub use fallback::*;

/// Puts `item` in the primary selection, which the middle button pastes.
/// Windows has no primary selection.
pub fn write_to_primary(cx: &mut gpui::App, item: gpui::ClipboardItem) {
    #[cfg(not(windows))]
    cx.write_to_primary(item);
    #[cfg(windows)]
    let _ = (cx, item);
}

#[cfg(windows)]
mod fallback {
    use gpui::{ClipboardItem, ExternalPaths, Image};

    /// Content dropped from another app rather than as files.
    #[derive(Clone, Debug, Default)]
    pub struct DroppedContent {
        pub text: Option<String>,
        pub html: Option<String>,
        pub image: Option<Image>,
    }

    /// Where a window was, to put it back there.
    #[derive(Clone, Debug)]
    pub struct Placement {
        pub session: Option<String>,
        pub name: String,
        pub restore: bool,
    }

    /// GPUI's Windows backend reads the clipboard as it is.
    pub fn read_rich<R>(read: impl FnOnce() -> R) -> R {
        read()
    }

    /// The HTML of a clipboard item: GPUI's Windows backend has only text.
    pub fn clipboard_html(_item: &ClipboardItem) -> Option<&str> {
        None
    }

    /// A clipboard item with `plain` text; the HTML is dropped.
    pub fn html_item(plain: String, _html: String) -> ClipboardItem {
        ClipboardItem::new_string(plain)
    }

    /// Windows drops arrive as files only.
    pub fn dropped_content(_paths: &ExternalPaths) -> Option<DroppedContent> {
        None
    }

    /// Windows puts windows back itself.
    pub fn restore_placement(_placement: Placement) {}

    pub fn placement_session() -> Option<String> {
        None
    }

    pub fn compositor_blur() -> bool {
        false
    }

    pub fn set_client_corner_radius(_radius: f32) {}

    /// Windows raises windows without a token.
    pub fn set_activation_token(_token: impl Into<String>) {}

    /// The KDE global menu exists only on Linux.
    pub fn set_kde_appmenu(_service: impl Into<String>, _object_path: impl Into<String>) {}
}
