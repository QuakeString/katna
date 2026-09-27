// SPDX-License-Identifier: GPL-3.0-or-later

//! About Katna: the suite, the version of Katna Mail with its What's new
//! and changelog, the source on GitHub, a way to support the work, where
//! to follow the author, the free software Katna is built on, each with its
//! license, and some love for Rust, KDE and Linux. Opened from Help in the menu bar, quick
//! settings and the version in the Settings header.

use gpui::{
    AnyElement, Context, FocusHandle, FontWeight, KeyDownEvent, MouseButton, SharedString, Window,
    div, linear_color_stop, linear_gradient, prelude::*, px, rgba,
};
use katna_ui::motion::{self, Spring, lerp};

use super::{MailWindow, PANEL_RADIUS, ShowAbout, ShowWhatsNew};
use crate::theme::{Theme, fade};
use crate::whats_new;
use crate::widgets::{elevation, filled_button, icon, outlined_button, tip};

const WIDTH: f32 = 520.0;

/// Where "Buy me a coffee" leads. `None` until the page exists; the
/// button then shows, disabled, with "Coming soon".
const SUPPORT_URL: Option<&str> = None;

/// The source of Katna.
const SOURCE_URL: &str = env!("CARGO_PKG_REPOSITORY");

/// Where to follow Katna's author: the site and the profile, or `None`
/// until there is one (the link is left out).
const FOLLOW: &[(&str, Option<&str>)] = &[
    ("GitHub", Some("https://github.com/QuakeString")),
    ("X", None),
    ("LinkedIn", None),
];

/// Free software Katna is built on: the name, what it does in Katna, its
/// license and its home. The README's credits follow this list.
const CREDITS: &[(&str, &str, &str, &str)] = &[
    (
        "Pimalaya",
        "IMAP, SMTP and sign-in (io-imap, io-smtp, io-sasl)",
        "MIT or Apache-2.0",
        "https://github.com/pimalaya",
    ),
    (
        "imap-codec",
        "Reading and writing IMAP",
        "MIT or Apache-2.0",
        "https://github.com/duesee/imap-codec",
    ),
    (
        "GPUI",
        "The user interface, from the makers of Zed",
        "Apache-2.0",
        "https://github.com/zed-industries/zed",
    ),
    (
        "wgpu",
        "Drawing on the graphics card",
        "MIT or Apache-2.0",
        "https://github.com/gfx-rs/wgpu",
    ),
    (
        "Tantivy",
        "Search",
        "MIT",
        "https://github.com/quickwit-oss/tantivy",
    ),
    (
        "SQLite and rusqlite",
        "The mail store",
        "Public domain and MIT",
        "https://github.com/rusqlite/rusqlite",
    ),
    (
        "mail-parser",
        "Reading mail, from Stalwart Labs",
        "Apache-2.0 or MIT",
        "https://github.com/stalwartlabs/mail-parser",
    ),
    (
        "html5ever",
        "HTML mail, from the Servo project",
        "MIT or Apache-2.0",
        "https://github.com/servo/html5ever",
    ),
    (
        "rustls",
        "Secure connections",
        "Apache-2.0, ISC or MIT",
        "https://github.com/rustls/rustls",
    ),
    (
        "zbus",
        "Talking to the desktop over D-Bus",
        "MIT",
        "https://github.com/z-galaxy/zbus",
    ),
    (
        "oo7",
        "Passwords in the desktop's keyring",
        "MIT",
        "https://github.com/linux-credentials/oo7",
    ),
    (
        "hayro",
        "The PDF viewer",
        "Apache-2.0 or MIT",
        "https://github.com/LaurenzV/hayro",
    ),
    (
        "calamine",
        "Spreadsheet previews",
        "MIT",
        "https://github.com/tafia/calamine",
    ),
    (
        "resvg",
        "SVG pictures",
        "Apache-2.0 or MIT",
        "https://github.com/linebender/resvg",
    ),
    (
        "Spellbook",
        "Spell check, from the Helix editor",
        "MPL-2.0",
        "https://github.com/helix-editor/spellbook",
    ),
    (
        "Jiff",
        "Dates and time zones",
        "Unlicense or MIT",
        "https://github.com/BurntSushi/jiff",
    ),
];

pub(super) struct About {
    focus: FocusHandle,
    closing: bool,
    shown: Spring,
}

impl MailWindow {
    pub(super) fn show_about(
        &mut self,
        _: &ShowAbout,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_about(window, cx);
    }

    pub(super) fn show_whats_new_action(
        &mut self,
        _: &ShowWhatsNew,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.show_whats_new(window, cx);
    }

    pub(super) fn open_about(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.settings_open = false;
        let focus = cx.focus_handle();
        window.focus(&focus, cx);
        let mut shown = Spring::new(motion::SMOOTH, 0.0);
        shown.set(1.0);
        self.about = Some(About {
            focus,
            closing: false,
            shown,
        });
        cx.notify();
    }

    /// About is open and not on its way out.
    pub(super) fn about_open(&self) -> bool {
        self.about.as_ref().is_some_and(|a| !a.closing)
    }

    pub(super) fn close_about(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(about) = &mut self.about
            && !about.closing
        {
            about.closing = true;
            about.shown.set(0.0);
            window.focus(&self.list_focus, cx);
        }
        cx.notify();
    }

    fn about_key(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        // Escape reaches `popovers` first.
        if matches!(event.keystroke.key.as_str(), "escape" | "enter") {
            self.close_about(window, cx);
            cx.stop_propagation();
        }
    }

    /// The version of Katna Mail at the end of the Settings header; it
    /// opens About.
    pub(super) fn version_button(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        div()
            .id("settings-version")
            .h(px(32.0))
            .px(px(12.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .rounded_full()
            .text_size(px(13.0))
            .text_color(rgba(th.text_dim))
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .tooltip(tip("About Katna", th))
            .on_click(cx.listener(|this, _, window, cx| this.open_about(window, cx)))
            .child(icon("info", th.text_dim, 18.0))
            .child(
                div()
                    .truncate()
                    .child(format!("Katna Mail {}", whats_new::VERSION)),
            )
            .into_any_element()
    }

    pub(super) fn render_about(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let about = self.about.as_mut()?;
        let t = about.shown.tick(window, reduce);
        if about.closing && about.shown.settled() {
            self.about = None;
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        let about = self.about.as_ref()?;
        let phone = self.layout.shape.is_phone();
        let vw = f32::from(window.viewport_size().width);
        let width = if phone { vw } else { WIDTH.min(vw - 48.0) };

        let header = div()
            .flex_none()
            .px(px(24.0))
            .pt(px(28.0))
            .flex()
            .flex_col()
            .items_center()
            .gap(px(6.0))
            .child(logo())
            .child(
                div()
                    .mt(px(8.0))
                    .text_size(px(24.0))
                    .line_height(px(32.0))
                    .child("Katna"),
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(rgba(th.text_dim))
                    .child("Mail and calendar for the Linux desktop"),
            )
            .child(
                div()
                    .mt(px(6.0))
                    .px(px(12.0))
                    .py(px(4.0))
                    .rounded_full()
                    .bg(rgba(th.chip))
                    .text_size(px(13.0))
                    .child(format!("Katna Mail {}", whats_new::VERSION)),
            );

        let changelog = whats_new::changelog_url(None);
        let links = div()
            .flex_none()
            .px(px(24.0))
            .pt(px(20.0))
            .flex()
            .flex_row()
            .flex_wrap()
            .justify_center()
            .gap(px(8.0))
            .child(
                outlined_button("about-whats-new", "What\u{2019}s new", th).on_click(cx.listener(
                    |this, _, window, cx| {
                        this.close_about(window, cx);
                        this.show_whats_new(window, cx);
                    },
                )),
            )
            .child(link_button("about-changelog", "Changelog", changelog, th))
            .child(link_button(
                "about-source",
                "Source code",
                SOURCE_URL.to_owned(),
                th,
            ));

        let coffee = div()
            .id("about-coffee")
            .flex_none()
            .mx(px(24.0))
            .mt(px(12.0))
            .h(px(40.0))
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .gap(px(8.0))
            .rounded_full()
            .bg(rgba(fade(th.star, 0.22)))
            .text_size(px(14.0))
            .font_weight(FontWeight::MEDIUM)
            .child(icon("coffee", th.text, 20.0))
            .child("Buy me a coffee");
        let coffee = match SUPPORT_URL {
            Some(url) => coffee
                .cursor_pointer()
                .hover(|s| s.bg(rgba(fade(th.star, 0.34))))
                .on_click(move |_, _, cx| cx.open_url(url)),
            None => coffee.opacity(0.55).tooltip(tip("Coming soon", th)),
        };

        let follow = FOLLOW
            .iter()
            .filter_map(|(site, url)| Some((*site, (*url)?)))
            .enumerate()
            .map(|(ix, (site, url))| {
                div()
                    .id(("about-follow", ix))
                    .h(px(32.0))
                    .px(px(14.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(6.0))
                    .rounded_full()
                    .bg(rgba(th.chip))
                    .text_size(px(13.0))
                    .font_weight(FontWeight::MEDIUM)
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .on_click(move |_, _, cx| cx.open_url(url))
                    .child(site)
                    .child(icon("open-external", th.text_dim, 14.0))
            })
            .collect::<Vec<_>>();
        let follow = (!follow.is_empty()).then(|| {
            div()
                .flex_none()
                .px(px(24.0))
                .pt(px(16.0))
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .justify_center()
                .gap(px(8.0))
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(rgba(th.text_dim))
                        .child("Follow the author"),
                )
                .children(follow)
        });

        let love = div()
            .flex_none()
            .mx(px(24.0))
            .mt(px(24.0))
            .p(px(16.0))
            .flex()
            .flex_row()
            .items_start()
            .gap(px(12.0))
            .rounded(px(12.0))
            .bg(rgba(fade(th.error, if th.dark { 0.14 } else { 0.07 })))
            .child(div().mt(px(1.0)).child(icon("heart", th.error, 22.0)))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .child(
                        div()
                            .text_size(px(15.0))
                            .font_weight(FontWeight::MEDIUM)
                            .child("Made with love for Rust, KDE and Linux"),
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .line_height(px(21.0))
                            .text_color(rgba(th.text_dim))
                            .child(
                                "Katna is written in Rust from top to bottom, and it \
                                 is made for the Linux desktop, with a special love \
                                 for KDE Plasma. Thank you to everyone who builds \
                                 them, and the libraries below.",
                            ),
                    ),
            );

        let credits = CREDITS
            .iter()
            .enumerate()
            .map(|(ix, (name, what, license, url))| {
                let url = *url;
                div()
                    .id(("about-credit", ix))
                    .flex_none()
                    .px(px(12.0))
                    .py(px(8.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .rounded(px(8.0))
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .on_click(move |_, _, cx| cx.open_url(url))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .line_height(px(20.0))
                                    .font_weight(FontWeight::MEDIUM)
                                    .child(*name),
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .line_height(px(18.0))
                                    .text_color(rgba(th.text_dim))
                                    .child(*what),
                            ),
                    )
                    .child(
                        div()
                            .flex_none()
                            .text_size(px(12.0))
                            .text_color(rgba(th.text_dim))
                            .child(*license),
                    )
            });
        let built_on = div()
            .flex_none()
            .px(px(12.0))
            .pt(px(24.0))
            .flex()
            .flex_col()
            .child(
                div()
                    .px(px(12.0))
                    .pb(px(4.0))
                    .text_size(px(12.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgba(th.text_dim))
                    .child("BUILT ON FREE SOFTWARE"),
            )
            .children(credits);

        let body = div()
            .id("about-body")
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .pb(px(12.0))
            .flex()
            .flex_col()
            .child(header)
            .child(links)
            .child(coffee)
            .children(follow)
            .child(love)
            .child(built_on);

        let footer = div()
            .flex_none()
            .px(px(24.0))
            .pt(px(12.0))
            .pb(px(16.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .border_t_1()
            .border_color(rgba(th.divider))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .text_size(px(12.0))
                    .line_height(px(17.0))
                    .text_color(rgba(th.text_dim))
                    .child("Katna is free software under the GNU GPL, version 3 or later."),
            )
            .child(
                filled_button("about-close", "Close", th)
                    .on_click(cx.listener(|this, _, window, cx| this.close_about(window, cx))),
            );

        let card = div()
            .id("about")
            .track_focus(&about.focus)
            .on_key_down(cx.listener(Self::about_key))
            .occlude()
            .w(px(width))
            .when(phone, |d| d.h_full())
            .when(!phone, |d| d.max_h_full().min_h_0())
            .flex()
            .flex_col()
            .overflow_hidden()
            .when(!phone, |d| {
                d.rounded(px(PANEL_RADIUS)).shadow(elevation(th, 3.0))
            })
            .bg(rgba(th.surface))
            .text_color(rgba(th.text))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(body)
            .child(footer);
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .when(!phone, |d| d.p(px(24.0)))
                .bg(rgba(fade(0x0000_0066, t)))
                .child(
                    div()
                        .id("about-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .on_click(cx.listener(|this, _, window, cx| this.close_about(window, cx))),
                )
                .child(
                    div()
                        .max_h_full()
                        .when(phone, |d| d.h_full())
                        .flex()
                        .flex_col()
                        .opacity(t)
                        .mt(px(lerp(24.0, 0.0, t)))
                        .child(card),
                )
                .into_any_element(),
        )
    }
}

/// An outlined button that opens `url` in the browser.
fn link_button(
    id: &'static str,
    label: &'static str,
    url: String,
    th: &Theme,
) -> gpui::Stateful<gpui::Div> {
    outlined_button(id, SharedString::from(label), th)
        .gap(px(8.0))
        .child(icon("open-external", th.accent, 16.0))
        .on_click(move |_, _, cx| cx.open_url(&url))
}

/// The Katna mark, larger than the one on the account pages.
fn logo() -> AnyElement {
    div()
        .size(px(64.0))
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(18.0))
        .bg(linear_gradient(
            135.0,
            linear_color_stop(rgba(0x4f8df7ff), 0.0),
            linear_color_stop(rgba(0x3949c9ff), 1.0),
        ))
        .child(icon("mail", 0xffffffff, 40.0))
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn credits_link_to_their_projects() {
        for (name, what, license, url) in CREDITS {
            assert!(!name.is_empty() && !what.is_empty() && !license.is_empty());
            assert!(url.starts_with("https://"), "{name}");
        }
        assert!(SOURCE_URL.starts_with("https://github.com/"));
        for (site, url) in FOLLOW {
            assert!(url.is_none_or(|url| url.starts_with("https://")), "{site}");
        }
    }
}
