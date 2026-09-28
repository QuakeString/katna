// SPDX-License-Identifier: GPL-3.0-or-later

//! About Katna: the suite, the version of Katna Mail with its What's new
//! and changelog, the source on GitHub, a way to support the work, where
//! to follow the author, the free software Katna is built on, each with its
//! license, every library it uses (`docs/credits.json`), and some love
//! for Rust, KDE and Linux. Opened from Help in the menu bar, quick
//! settings and the version in the Settings header.

use std::sync::LazyLock;

use gpui::{
    AnyElement, Context, FocusHandle, FontWeight, KeyDownEvent, MouseButton, SharedString, Window,
    div, prelude::*, rgba,
};
use katna_i18n::tr;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::px;
use katna_ui::unpx;

use super::{MailWindow, PANEL_RADIUS, ShowAbout, ShowWhatsNew};
use crate::theme::{Theme, fade};
use crate::whats_new;
use crate::widgets::{elevation, filled_button, icon, outlined_button, tip};

const WIDTH: f32 = 520.0;

/// Where "Buy me a coffee" leads. `None` until the page exists; the
/// button then shows, disabled, with "Coming soon".
const SUPPORT_URL: Option<&str> = Some("https://buymeacoffee.com/quakestring");

/// Buy Me a Coffee's own button colours: black on yellow with a black
/// outline.
const COFFEE_YELLOW: u32 = 0xffdd00ff;
const COFFEE_INK: u32 = 0x000000ff;

/// The source of Katna.
const SOURCE_URL: &str = env!("CARGO_PKG_REPOSITORY");

/// GPUI and the Zed project, thanked in a box of their own.
const ZED_URL: &str = "https://zed.dev";
const GPUI_URL: &str = "https://github.com/zed-industries/zed/tree/main/crates/gpui";

/// KDE's donation page.
const KDE_DONATE_URL: &str = "https://kde.org/donate/";

/// Where to follow Katna's author: the site and the profile, or `None`
/// until there is one (the link is left out).
const FOLLOW: &[(&str, Option<&str>)] = &[
    ("GitHub", Some("https://github.com/QuakeString")),
    ("X", Some("https://x.com/QuakeString")),
    (
        "LinkedIn",
        Some("https://www.linkedin.com/in/md-mozammel-hossain-97a20446/"),
    ),
];

/// The heart of Katna, picked by hand: the name, the message id of what
/// it does in Katna, its license and its home. It matches the credits in README.md; every
/// library is in [`LIBRARIES`].
const CREDITS: &[(&str, &str, &str, &str)] = &[
    (
        "Pimalaya",
        "about-credit-pimalaya",
        "MIT or Apache-2.0",
        "https://github.com/pimalaya",
    ),
    (
        "imap-codec",
        "about-credit-imap-codec",
        "MIT or Apache-2.0",
        "https://github.com/duesee/imap-codec",
    ),
    (
        "Tantivy",
        "about-credit-tantivy",
        "MIT",
        "https://github.com/quickwit-oss/tantivy",
    ),
    (
        "SQLite and rusqlite",
        "about-credit-sqlite",
        "Public domain and MIT",
        "https://github.com/rusqlite/rusqlite",
    ),
    (
        "rustls",
        "about-credit-rustls",
        "Apache-2.0, ISC or MIT",
        "https://github.com/rustls/rustls",
    ),
    (
        "mail-parser",
        "about-credit-mail-parser",
        "Apache-2.0 or MIT",
        "https://github.com/stalwartlabs/mail-parser",
    ),
    (
        "html5ever",
        "about-credit-html5ever",
        "MIT or Apache-2.0",
        "https://github.com/servo/html5ever",
    ),
    (
        "zbus and ashpd",
        "about-credit-zbus",
        "MIT",
        "https://github.com/z-galaxy/zbus",
    ),
    (
        "oo7",
        "about-credit-oo7",
        "MIT",
        "https://github.com/linux-credentials/oo7",
    ),
    (
        "hayro and krilla",
        "about-credit-hayro",
        "Apache-2.0 or MIT",
        "https://github.com/LaurenzV/hayro",
    ),
    (
        "calamine",
        "about-credit-calamine",
        "MIT",
        "https://github.com/tafia/calamine",
    ),
    (
        "resvg",
        "about-credit-resvg",
        "Apache-2.0 or MIT",
        "https://github.com/linebender/resvg",
    ),
    (
        "Jiff",
        "about-credit-jiff",
        "Unlicense or MIT",
        "https://github.com/BurntSushi/jiff",
    ),
    (
        "Spellbook",
        "about-credit-spellbook",
        "MPL-2.0",
        "https://github.com/helix-editor/spellbook",
    ),
    (
        "smol",
        "about-credit-smol",
        "Apache-2.0 or MIT",
        "https://github.com/smol-rs/smol",
    ),
];

/// A library Katna uses directly, from `docs/credits.json`, which
/// `ci/gen-credits.sh` writes from `cargo metadata` (CREDITS.md too).
struct Library {
    name: String,
    version: String,
    authors: String,
    license: String,
    repository: String,
}

static LIBRARIES: LazyLock<Vec<Library>> =
    LazyLock::new(|| libraries(include_str!("../../../../docs/credits.json")));

fn libraries(json: &str) -> Vec<Library> {
    let json: serde_json::Value = serde_json::from_str(json).unwrap_or_default();
    let text = |v: &serde_json::Value, key| v[key].as_str().unwrap_or_default().to_owned();
    json["crates"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|c| Library {
            name: text(c, "name"),
            version: text(c, "version"),
            authors: c["authors"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|a| a.as_str())
                .collect::<Vec<_>>()
                .join(", "),
            license: text(c, "license"),
            repository: text(c, "repository"),
        })
        .collect()
}

pub(super) struct About {
    focus: FocusHandle,
    closing: bool,
    shown: Spring,
    /// Every library is listed, not only the heart of Katna.
    all: bool,
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
            all: false,
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
            .tooltip(tip(tr!("about-tooltip"), th))
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
        let updates = self.update_card(th, cx);
        let about = self.about.as_ref()?;
        let phone = self.layout.shape.is_phone();
        let vw = unpx(window.viewport_size().width);
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
                    .child(tr!("about-tagline")),
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
                outlined_button("about-whats-new", tr!("about-whats-new"), th).on_click(
                    cx.listener(|this, _, window, cx| {
                        this.close_about(window, cx);
                        this.show_whats_new(window, cx);
                    }),
                ),
            )
            .child(link_button(
                "about-changelog",
                tr!("about-changelog"),
                changelog,
                th,
            ))
            .child(link_button(
                "about-source",
                tr!("about-source"),
                SOURCE_URL.to_owned(),
                th,
            ));

        // Drawn like Buy Me a Coffee's own button, in both themes.
        let button = div()
            .id("about-coffee")
            .flex_1()
            .h(px(44.0))
            .px(px(20.0))
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .gap(px(8.0))
            .rounded(px(10.0))
            .border_1()
            .border_color(rgba(COFFEE_INK))
            .bg(rgba(COFFEE_YELLOW))
            .text_color(rgba(COFFEE_INK))
            .text_size(px(16.0))
            .font_weight(FontWeight::SEMIBOLD)
            .child(icon("coffee", COFFEE_INK, 22.0))
            .child(tr!("about-coffee"));
        let button = match SUPPORT_URL {
            Some(url) => button
                .cursor_pointer()
                .hover(|s| s.bg(rgba(0xffe433ff)))
                .active(|s| s.bg(rgba(0xf2d200ff)))
                .on_click(move |_, _, cx| cx.open_url(url)),
            None => button
                .opacity(0.55)
                .tooltip(tip(tr!("about-coming-soon"), th)),
        };
        // One wide button across the content, like the box below it.
        let coffee = div()
            .flex_none()
            .mx(px(24.0))
            .mt(px(12.0))
            .flex()
            .child(button);

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
                        .child(tr!("about-follow")),
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
                            .child(tr!("about-love-title")),
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .line_height(px(21.0))
                            .text_color(rgba(th.text_dim))
                            .child(tr!("about-love-text")),
                    )
                    .child(
                        div()
                            .mt(px(6.0))
                            .text_size(px(14.0))
                            .line_height(px(21.0))
                            .text_color(rgba(th.text_dim))
                            .child(tr!("about-kde-text")),
                    )
                    .child(div().mt(px(8.0)).flex().flex_row().child(link_button(
                        "about-donate-kde",
                        tr!("about-donate-kde"),
                        KDE_DONATE_URL.to_owned(),
                        th,
                    ))),
            );

        let gpui = div()
            .flex_none()
            .mx(px(24.0))
            .mt(px(24.0))
            .p(px(16.0))
            .flex()
            .flex_col()
            .gap(px(4.0))
            .rounded(px(12.0))
            .bg(rgba(fade(th.accent, if th.dark { 0.16 } else { 0.07 })))
            .child(
                div()
                    .text_size(px(15.0))
                    .font_weight(FontWeight::MEDIUM)
                    .child(tr!("about-gpui-title")),
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .line_height(px(21.0))
                    .text_color(rgba(th.text_dim))
                    .child(tr!("about-gpui-text")),
            )
            .child(
                div()
                    .mt(px(8.0))
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(8.0))
                    .child(link_button("about-zed", "zed.dev", ZED_URL.to_owned(), th))
                    .child(link_button(
                        "about-gpui",
                        tr!("about-gpui-github"),
                        GPUI_URL.to_owned(),
                        th,
                    )),
            );

        let personal = div()
            .flex_none()
            .px(px(24.0))
            .pt(px(20.0))
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(
                div()
                    .text_size(px(15.0))
                    .font_weight(FontWeight::MEDIUM)
                    .child(tr!("about-personal-title")),
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .line_height(px(21.0))
                    .text_color(rgba(th.text_dim))
                    .child(tr!("about-personal-text")),
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
                                    .child(tr!(what)),
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
                    .child(tr!("about-built-on")),
            )
            .children(credits);

        let all = about.all;
        let libraries = div()
            .flex_none()
            .px(px(12.0))
            .pt(px(8.0))
            .flex()
            .flex_col()
            .child(
                div()
                    .id("about-all-libraries")
                    .px(px(12.0))
                    .py(px(10.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .rounded(px(8.0))
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Some(about) = &mut this.about {
                            about.all = !about.all;
                        }
                        cx.notify();
                    }))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(th.accent))
                            .child(tr!("about-all-libraries", count = LIBRARIES.len())),
                    )
                    .child(icon(
                        if all { "chevron-down" } else { "chevron-right" },
                        th.accent,
                        20.0,
                    )),
            )
            .when(all, |d| {
                d.children(LIBRARIES.iter().enumerate().map(|(ix, lib)| {
                    let url = lib.repository.clone();
                    div()
                        .id(("about-library", ix))
                        .flex_none()
                        .px(px(12.0))
                        .py(px(6.0))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(12.0))
                        .rounded(px(8.0))
                        .cursor_pointer()
                        .hover(|s| s.bg(rgba(th.hover)))
                        .tooltip(tip(lib.repository.clone(), th))
                        .on_click(move |_, _, cx| cx.open_url(&url))
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
                                        .child(format!("{} {}", lib.name, lib.version)),
                                )
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .line_height(px(18.0))
                                        .text_color(rgba(th.text_dim))
                                        .child(tr!(
                                            "about-library-authors",
                                            authors = lib.authors.as_str()
                                        )),
                                ),
                        )
                        .child(
                            div()
                                .flex_none()
                                .max_w(px(160.0))
                                .text_size(px(12.0))
                                .text_color(rgba(th.text_dim))
                                .child(lib.license.clone()),
                        )
                }))
            });

        let body = div()
            .id("about-body")
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .pb(px(12.0))
            .flex()
            .flex_col()
            .child(header)
            .children(updates)
            .child(links)
            .child(coffee)
            .children(follow)
            .child(love)
            .child(personal)
            .child(gpui)
            .child(built_on)
            .child(libraries);

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
                    .child(tr!("about-license")),
            )
            .child(
                filled_button("about-close", tr!("about-close"), th)
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
    label: impl Into<SharedString>,
    url: String,
    th: &Theme,
) -> gpui::Stateful<gpui::Div> {
    outlined_button(id, label, th)
        .gap(px(8.0))
        .child(icon("open-external", th.accent, 16.0))
        .on_click(move |_, _, cx| cx.open_url(&url))
}

/// Katna Mail's wordmark.
fn logo() -> AnyElement {
    crate::widgets::katna_wordmark(112.0)
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
        assert!(KDE_DONATE_URL.starts_with("https://kde.org/"));
        assert!(ZED_URL.starts_with("https://") && GPUI_URL.starts_with("https://github.com/"));
        for (site, url) in FOLLOW {
            assert!(url.is_none_or(|url| url.starts_with("https://")), "{site}");
        }
    }

    #[test]
    fn every_library_has_a_home() {
        assert!(LIBRARIES.len() > 20);
        for lib in LIBRARIES.iter() {
            assert!(
                !lib.name.is_empty() && !lib.version.is_empty(),
                "{}",
                lib.name
            );
            assert!(
                !lib.authors.is_empty() && !lib.license.is_empty(),
                "{}",
                lib.name
            );
            assert!(lib.repository.starts_with("https://"), "{}", lib.name);
        }
        assert!(LIBRARIES.iter().any(|lib| lib.name == "gpui-pre"));
    }
}
