// SPDX-License-Identifier: GPL-3.0-or-later

//! About Katna: the suite, the version of Katna Mail with its What's new
//! and changelog, the source on GitHub, a way to support the work, where
//! to follow the author, the free software Katna is built on, each with its
//! license, every library it uses (`docs/credits.json`), and some love
//! for Rust, KDE and Linux. Opened from Help in the menu bar, quick
//! settings and the version in the Settings header.

mod coffee;

use crate::widgets::Tip as _;
use std::sync::LazyLock;
use std::time::{Duration, Instant};

use gpui::{
    AnyElement, Context, FocusHandle, FontWeight, KeyDownEvent, MouseButton, ScrollHandle,
    SharedString, Window, div, prelude::*, rgba,
};
use katna_i18n::tr;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::px;
use katna_ui::tokens::space;
use katna_ui::unpx;

use super::select::{ABOUT_SLOT, Pieces, selectable};
use super::{MailWindow, PANEL_RADIUS, ShowAbout, ShowWhatsNew};
use crate::theme::{Theme, fade};
use crate::widgets::{
    FocusRing, elevation, filled_button, icon, icon_button_with, outlined_button, ring_style,
};
use crate::{format, whats_new};

const WIDTH: f32 = 520.0;

/// The tallest About and What's new get outside the phone layout: at most
/// 600 px and 70% of the window, their middle scrolling within.
pub(super) fn dialog_max_height(window: &Window) -> f32 {
    (unpx(window.viewport_size().height) * 0.7).min(600.0)
}

/// Where "Buy me a coffee" leads. `None` until the page exists; the
/// button then shows, disabled, with "Coming soon".
const SUPPORT_URL: Option<&str> = Some("https://buymeacoffee.com/quakestring");

/// Buy Me a Coffee's yellow, softened: the button is filled with it at
/// these opacities (light, dark theme) and outlined with it.
const COFFEE_YELLOW: u32 = 0xffdd0000;
const COFFEE_FILL: (u32, u32) = (0x66, 0x33);
const COFFEE_HOVER: (u32, u32) = (0x8c, 0x4d);
const COFFEE_BORDER: u32 = 0xb3;

/// The source of Katna.
const SOURCE_URL: &str = env!("CARGO_PKG_REPOSITORY");

/// GPUI and the Zed project, thanked in a box of their own.
const ZED_URL: &str = "https://zed.dev";
const GPUI_URL: &str = "https://github.com/zed-industries/zed/tree/main/crates/gpui";

/// KDE's donation page.
const KDE_DONATE_URL: &str = "https://kde.org/donate/";

/// Where to follow Katna's author: the site, its icon and the profile, or
/// `None` until there is one (the link is left out).
const FOLLOW: &[(&str, &str, Option<&str>)] = &[
    (
        "GitHub",
        "brand-github",
        Some("https://github.com/QuakeString"),
    ),
    ("x.com", "brand-x", Some("https://x.com/QuakeString")),
    (
        "LinkedIn",
        "brand-linkedin",
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
    (
        "Nord, Solarized, Dracula, Gruvbox, Catppuccin, Tokyo Night, One, Rosé Pine, Everforest, Kanagawa and Ayu",
        "about-credit-color-schemes",
        "MIT",
        "https://github.com/QuakeString/katna#built-with-love-on-the-shoulders-of-giants",
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
    pub(super) focus: FocusHandle,
    pub(super) closing: bool,
    shown: Spring,
    /// Every library is listed, not only the heart of Katna.
    all: bool,
    /// When the little play in "Buy me a coffee" began, while it plays.
    coffee: Option<Instant>,
    /// The page's scroll, which shrinks the header.
    scroll: ScrollHandle,
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
        self.ui_text.clear();
        self.about = Some(About {
            focus,
            closing: false,
            shown,
            all: false,
            coffee: None,
            scroll: ScrollHandle::new(),
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
        // Escape reaches `popovers` first. Enter on a focused button
        // presses that button instead.
        let on_dialog = self
            .about
            .as_ref()
            .is_some_and(|a| a.focus.is_focused(window));
        if event.keystroke.key == "escape" || (event.keystroke.key == "enter" && on_dialog) {
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
            .pl(px(space::S4))
            // The copy button at the end brings its own room.
            .pr(px(space::S2))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .rounded_full()
            .text_size(px(13.0))
            .text_color(rgba(th.text_dim))
            .cursor_pointer()
            .relative()
            .child(crate::widgets::hover_fade("hover-glow", None, th))
            .tip(tr!("about-tooltip"), th)
            .on_click(cx.listener(|this, _, window, cx| this.open_about(window, cx)))
            .group(VERSION_GROUP)
            .child(icon("info", th.text_dim, 18.0))
            .child(
                div()
                    .truncate()
                    .child(format!("Katna Mail {}", whats_new::VERSION)),
            )
            .child(self.copy_version_button("settings-version-copy", 24.0, th, cx))
            .into_any_element()
    }

    /// The small button beside the version wherever it shows (About,
    /// What's new, Settings): it shows while the pointer is on the
    /// version (put the version in a [`VERSION_GROUP`] group) or Tab
    /// reaches it, and copies [`version_details`], then shows a check for
    /// a moment.
    pub(super) fn copy_version_button(
        &self,
        id: &'static str,
        size: f32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let copied = self
            .version_copied
            .is_some_and(|at| at.elapsed() < COPIED_FOR);
        let (name, color) = if copied {
            ("check", th.accent)
        } else {
            ("copy", th.text_dim)
        };
        let ring = ring_style(th);
        icon_button_with(id, icon(name, color, 14.0), th)
            .size(px(size))
            .opacity(if copied { 1.0 } else { 0.0 })
            .group_hover(VERSION_GROUP, |s| s.opacity(1.0))
            .tab_index(0)
            .focus_visible(move |s| ring(s).opacity(1.0))
            .tip(
                if copied {
                    tr!("about-version-copied")
                } else {
                    tr!("about-copy-version")
                },
                th,
            )
            // A click here is not one on the pill around it.
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_click(cx.listener(|this, _, _, cx| {
                cx.stop_propagation();
                cx.write_to_clipboard(gpui::ClipboardItem::new_string(version_details()));
                let at = Instant::now();
                this.version_copied = Some(at);
                cx.notify();
                cx.spawn(async move |this, cx| {
                    cx.background_executor().timer(COPIED_FOR).await;
                    this.update(cx, |this, cx| {
                        if this.version_copied == Some(at) {
                            this.version_copied = None;
                            cx.notify();
                        }
                    })
                    .ok();
                })
                .detach();
            }))
            .into_any_element()
    }

    pub(super) fn render_about(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        // It floats: its surface is a step lighter in dark colors.
        let th = &th.lifted();
        let about = self.about.as_mut()?;
        let t = about.shown.tick(window, reduce);
        if about.closing && about.shown.settled() {
            self.about = None;
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        let scene = about.coffee.and_then(|at| {
            let ms = u64::try_from(at.elapsed().as_millis()).unwrap_or(u64::MAX);
            coffee::scene(ms)
        });
        if scene.is_some() {
            window.request_animation_frame();
        } else {
            about.coffee = None;
        }
        let about = self.about.as_ref()?;
        let phone = self.layout.shape.is_phone();
        let vw = self.room_width();
        let width = if phone { vw } else { WIDTH.min(vw - 48.0) };

        let offset = -unpx(about.scroll.offset().y);
        let radius = if phone { 0.0 } else { PANEL_RADIUS };
        // Its words can be selected and copied, top to bottom; buttons and
        // rows that open a page stay buttons.
        let mut pieces = self.ui_text.slot_pieces(ABOUT_SLOT, th);
        // As tall as the version's chip, which shrinks with the header.
        let shrunk = (offset.max(0.0) / (HEADER_TALL - HEADER_SHORT)).clamp(0.0, 1.0);
        let chip = lerp(18.0, 16.0, shrunk) + 2.0 * lerp(4.0, 1.0, shrunk);
        let copy = self.copy_version_button("about-version-copy", chip, th, cx);
        let header = header(th, width, offset.max(0.0), radius, &mut pieces, copy, cx);

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
                outlined_button("about-whats-new", tr!("about-whats-new"), th)
                    .focus_ring(th)
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.close_about(window, cx);
                        this.show_whats_new(window, cx);
                    })),
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

        // Buy Me a Coffee's yellow, softer, with the theme's text on it.
        let alpha = |(light, dark): (u32, u32)| COFFEE_YELLOW | if th.dark { dark } else { light };
        let (fill, hover) = (alpha(COFFEE_FILL), alpha(COFFEE_HOVER));
        let button = div()
            .id("about-coffee")
            .flex_1()
            .h(px(coffee::HEIGHT + 2.0))
            .px(px(20.0))
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .gap(px(8.0))
            .rounded(px(10.0))
            .border_1()
            .border_color(rgba(COFFEE_YELLOW | COFFEE_BORDER))
            .bg(rgba(fill))
            .text_color(rgba(th.text))
            .text_size(px(16.0))
            .font_weight(FontWeight::SEMIBOLD)
            .child(coffee::render(scene, th.text));
        let button = match SUPPORT_URL {
            Some(url) => button
                .cursor_pointer()
                .when(!reduce, |d| {
                    d.on_hover(cx.listener(|this, hovered: &bool, _, cx| {
                        if let Some(about) = &mut this.about
                            && *hovered
                            && about.coffee.is_none()
                        {
                            about.coffee = Some(Instant::now());
                            cx.notify();
                        }
                    }))
                })
                .hover(move |s| s.bg(rgba(hover)))
                .active(move |s| s.bg(rgba(hover)))
                .on_click(move |_, _, cx| cx.open_url(url)),
            None => button.opacity(0.55).tip(tr!("about-coming-soon"), th),
        };
        // One wide button across the content, like the box below it.
        let coffee = div()
            .flex_none()
            .mx(px(24.0))
            .mt(px(16.0))
            .flex()
            .child(button);

        let follow = FOLLOW
            .iter()
            .filter_map(|(site, logo, url)| Some((*site, *logo, (*url)?)))
            .enumerate()
            .map(|(ix, (site, logo, url))| {
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
                    .relative()
                    .child(crate::widgets::hover_fade("hover-glow", None, th))
                    .on_click(move |_, _, cx| cx.open_url(url))
                    .child(icon(logo, th.text, 14.0))
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
                .child(selectable(
                    pieces
                        .words(tr!("about-follow-me"))
                        .text_size(px(13.0))
                        .text_color(rgba(th.text_dim)),
                    Some(pieces.part()),
                    cx,
                ))
                .children(follow)
        });

        let personal = selectable(
            div()
                .flex_none()
                .px(px(24.0))
                .pt(px(20.0))
                .flex()
                .flex_col()
                .gap(px(space::S2))
                .child(
                    pieces
                        .words(tr!("about-personal-title"))
                        .text_size(px(15.0))
                        .font_weight(FontWeight::MEDIUM),
                )
                .child(
                    pieces
                        .words(tr!("about-personal-text"))
                        .text_size(px(14.0))
                        .line_height(px(21.0))
                        .text_color(rgba(th.text_dim)),
                ),
            Some(pieces.part()),
            cx,
        );

        let gpui = div()
            .flex_none()
            .mx(px(24.0))
            .mt(px(24.0))
            .p(px(16.0))
            .flex()
            .flex_col()
            .gap(px(space::S2))
            .rounded(px(12.0))
            .bg(rgba(fade(th.accent, if th.dark { 0.16 } else { 0.07 })))
            .child(selectable(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(space::S2))
                    .child(
                        pieces
                            .words(tr!("about-gpui-title"))
                            .text_size(px(15.0))
                            .font_weight(FontWeight::MEDIUM),
                    )
                    .child(
                        pieces
                            .words(tr!("about-gpui-text"))
                            .text_size(px(14.0))
                            .line_height(px(21.0))
                            .text_color(rgba(th.text_dim)),
                    ),
                Some(pieces.part()),
                cx,
            ))
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
                    .gap(px(space::S2))
                    .child(selectable(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(space::S2))
                            .child(
                                pieces
                                    .words(tr!("about-love-title"))
                                    .text_size(px(15.0))
                                    .font_weight(FontWeight::MEDIUM),
                            )
                            .child(
                                pieces
                                    .words(tr!("about-love-text"))
                                    .text_size(px(14.0))
                                    .line_height(px(21.0))
                                    .text_color(rgba(th.text_dim)),
                            )
                            .child(
                                pieces
                                    .words(tr!("about-kde-text"))
                                    .mt(px(6.0))
                                    .text_size(px(14.0))
                                    .line_height(px(21.0))
                                    .text_color(rgba(th.text_dim)),
                            ),
                        Some(pieces.part()),
                        cx,
                    ))
                    .child(div().mt(px(8.0)).flex().flex_row().child(link_button(
                        "about-donate-kde",
                        tr!("about-donate-kde"),
                        KDE_DONATE_URL.to_owned(),
                        th,
                    ))),
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
                    .relative()
                    .child(crate::widgets::hover_fade("hover-glow", Some(8.0), th))
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
                    .relative()
                    .child(crate::widgets::hover_fade("hover-glow", Some(8.0), th))
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
                        .relative()
                        .child(crate::widgets::hover_fade("hover-glow", Some(8.0), th))
                        .tip(lib.repository.clone(), th)
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
            .size_full()
            .overflow_y_scroll()
            .track_scroll(&about.scroll)
            .pb(px(12.0))
            .flex()
            .flex_col()
            // Room for the header, which floats over the top and shrinks.
            .child(div().flex_none().h(px(HEADER_TALL)))
            .child(links)
            .children(follow)
            .child(coffee)
            .child(personal)
            .child(gpui)
            .child(love)
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
            .child(selectable(
                div().flex_1().min_w_0().child(
                    pieces
                        .words(tr!("about-license"))
                        .text_size(px(12.0))
                        .line_height(px(17.0))
                        .text_color(rgba(th.text_dim)),
                ),
                Some(pieces.part()),
                cx,
            ))
            .child(
                filled_button("about-close", tr!("about-close"), th)
                    .focus_ring_filled(th)
                    .on_click(cx.listener(|this, _, window, cx| this.close_about(window, cx))),
            );

        let card = div()
            .id("about")
            .map(|d| self.ui_text_area(d, cx))
            .track_focus(&about.focus)
            .map(|d| super::popovers::keep_tab_inside(d, &about.focus))
            .on_key_down(cx.listener(Self::about_key))
            .occlude()
            .w(px(width))
            .when(phone, |d| d.h_full())
            .when(!phone, |d| d.max_h(px(dialog_max_height(window))).min_h_0())
            .flex()
            .flex_col()
            .overflow_hidden()
            .when(!phone, |d| {
                d.rounded(px(PANEL_RADIUS)).shadow(elevation(th, 3.0))
            })
            .map(|d| crate::widgets::frosted(d, th, th.surface, radius))
            .text_color(rgba(th.text))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .relative()
                    .flex()
                    .flex_col()
                    .child(body)
                    .child(header),
            )
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
                // No veil: the window stays as it is around the dialog.
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
        .focus_ring(th)
        .gap(px(8.0))
        .child(icon("open-external", th.accent, 16.0))
        .on_click(move |_, _, cx| cx.open_url(&url))
}

/// The group a version and its [`MailWindow::copy_version_button`] are in.
pub(super) const VERSION_GROUP: &str = "katna-version";

/// How long the copy button shows its check.
const COPIED_FOR: Duration = Duration::from_millis(1500);

/// The version details a bug report wants: Katna Mail's version, when
/// this build was made and the system it runs on.
pub(super) fn version_details() -> String {
    let mut lines = vec![format!("Katna Mail {}", whats_new::VERSION)];
    if let Some(date) =
        whats_new::built().and_then(|unix| format::local(unix, &jiff::tz::TimeZone::system()))
    {
        lines.push(tr!("about-version-built", date = format::long_date(date)));
    }
    lines.push(tr!(
        "about-version-system",
        system = katna_core::crash::system()
    ));
    lines.join("\n")
}

/// The header's height, open and shrunk.
const HEADER_TALL: f32 = 250.0;
const HEADER_SHORT: f32 = 88.0;

/// The wordmark's height, open and shrunk.
const LOGO_TALL: f32 = 112.0;
const LOGO_SHORT: f32 = 60.0;

/// The header over the top of the page: the wordmark with Katna, the
/// tagline and the version centred under it. As the page scrolls `offset`
/// px, it shrinks with the scroll, the wordmark moving to the left with
/// the three lines beside it, then stays while the rest scrolls under its
/// frost.
fn header(
    th: &Theme,
    width: f32,
    offset: f32,
    radius: f32,
    pieces: &mut Pieces,
    copy: AnyElement,
    cx: &mut Context<MailWindow>,
) -> AnyElement {
    let collapse = HEADER_TALL - HEADER_SHORT;
    let t = (offset / collapse).clamp(0.0, 1.0);
    // Frosted as the page nears the point where it slides under.
    let stuck = ((offset - collapse) / 12.0 + 1.0).clamp(0.0, 1.0);
    let (w, h) = crate::assets::WORDMARK_SIZE;
    let aspect = w as f32 / h as f32;
    // The wordmark gets to its place first, eased, so the lines rising
    // after it pass below and then beside it, never over it.
    let x = (t / 0.6).min(1.0);
    let e = x * x * (3.0 - 2.0 * x);
    let logo = lerp(LOGO_TALL, LOGO_SHORT, e);
    let logo_left = lerp((width - LOGO_TALL * aspect) / 2.0, 24.0, e);
    let logo_top = lerp(28.0, 14.0, e);
    let text_left = lerp(24.0, 24.0 + LOGO_SHORT * aspect + 16.0, t);
    // Each line's height and the gaps, open and shrunk: 96 and 61 px.
    let (name, name_line) = (lerp(24.0, 17.0, t), lerp(32.0, 22.0, t));
    let (tag, tag_line) = (lerp(14.0, 13.0, t), lerp(20.0, 18.0, t));
    let (chip, chip_line) = (lerp(13.0, 12.0, t), lerp(18.0, 16.0, t));
    let (chip_x, chip_y) = (lerp(12.0, 8.0, t), lerp(4.0, 1.0, t));
    let chip_h = chip_line + 2.0 * chip_y;
    let (gap_tag, gap_chip) = (lerp(6.0, 0.0, t), lerp(12.0, 3.0, t));
    let text_top = lerp(28.0 + LOGO_TALL + 14.0, (HEADER_SHORT - 61.0) / 2.0, t);

    // Centred, then at the left: the space before a line gives way.
    let line = |child: gpui::Div| {
        div()
            .flex()
            .flex_row()
            .child(div().flex_grow(1.0 - t))
            .child(child.min_w_0().truncate())
            .child(div().flex_grow(1.0))
    };
    let glass = crate::widgets::frosted_top(
        div().absolute().top_0().left_0().size_full().opacity(stuck),
        th,
        th.surface,
        radius,
    )
    .child(
        div()
            .absolute()
            .left_0()
            .right_0()
            .bottom_0()
            .h(px(1.0))
            .bg(rgba(th.divider)),
    );
    div()
        .id("about-header")
        .absolute()
        .top_0()
        .left_0()
        .right_0()
        .h(px(HEADER_TALL - offset.min(collapse)))
        // The wheel still scrolls the page from the header.
        .block_mouse_except_scroll()
        .child(glass)
        .child(
            div()
                .absolute()
                .top(px(logo_top))
                .left(px(logo_left))
                .child(crate::widgets::katna_wordmark_from(logo, LOGO_TALL, th)),
        )
        .child(selectable(
            div()
                .absolute()
                .top(px(text_top))
                .left(px(text_left))
                .right(px(24.0))
                .flex()
                .flex_col()
                .child(line(
                    pieces
                        .words("Katna")
                        .text_size(px(name))
                        .line_height(px(name_line)),
                ))
                .child(line(
                    pieces
                        .words(tr!("about-tagline"))
                        .mt(px(gap_tag))
                        .text_size(px(tag))
                        .line_height(px(tag_line))
                        .text_color(rgba(th.text_dim)),
                ))
                // The copy button sits beside the chip, shown on hover.
                .child(line(
                    div()
                        .group(VERSION_GROUP)
                        .mt(px(gap_chip))
                        .flex()
                        .flex_row()
                        .items_center()
                        // Room for the button on the left too, so the chip
                        // stays centred; none once the lines are at the left.
                        .child(div().flex_none().w(px((chip_h + space::S1) * (1.0 - t))))
                        .child(
                            pieces
                                .words(format!("Katna Mail {}", whats_new::VERSION))
                                .min_w_0()
                                .truncate()
                                .px(px(chip_x))
                                .py(px(chip_y))
                                .rounded_full()
                                .bg(rgba(th.chip))
                                .text_size(px(chip))
                                .line_height(px(chip_line)),
                        )
                        .child(div().flex_none().ml(px(space::S1)).child(copy)),
                )),
            Some(pieces.part()),
            cx,
        ))
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
        assert!(KDE_DONATE_URL.starts_with("https://kde.org/"));
        assert!(ZED_URL.starts_with("https://") && GPUI_URL.starts_with("https://github.com/"));
        for (site, logo, url) in FOLLOW {
            assert!(logo.starts_with("brand-"), "{site}");
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
