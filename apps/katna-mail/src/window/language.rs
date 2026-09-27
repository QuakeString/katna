// SPDX-License-Identifier: GPL-3.0-or-later

//! The language picker (`docs/ARCHITECTURE.md` §13.10): the flag button
//! left of Settings in the top bar, the popover it opens (search, System
//! default, then every language with its flag, own name and English name),
//! and the phone drawer's Language row, which opens the same list over the
//! window. Settings > General opens it too.

use std::cell::Cell;
use std::time::Duration;

use gpui::{
    Animation, AnimationExt, AnyElement, Context, Entity, Focusable, FontWeight, KeyDownEvent,
    MouseButton, MouseDownEvent, Pixels, Point, ScrollHandle, SharedString, Subscription, Window,
    deferred, div, img, prelude::*, px, rgba,
};
use katna_i18n::{Language, Status, tr};
use katna_ui::{InputEvent, TextInput};

use super::settings::Change;
use super::{BAR_ITEM_GAP, MailWindow, TOP_BAR_GAP};
use crate::theme::Theme;
use crate::widgets::{icon, raised, tip};

const MENU_WIDTH: f32 = 340.0;
const ROW_HEIGHT: f32 = 56.0;
/// The popover's tallest, leaving room above and below it in the window.
const MENU_MAX_HEIGHT: f32 = 560.0;

/// How to correct a translation, linked from a machine-drafted language.
const TRANSLATION_GUIDE: &str = "https://github.com/QuakeString/katna/blob/main/i18n/README.md";

/// The open picker.
pub(super) struct LanguagePicker {
    search: Entity<TextInput>,
    /// What the search box holds.
    query: String,
    /// The row the arrow keys are on.
    highlight: usize,
    /// Where it was opened from, when not the top bar's button (Settings):
    /// it opens under that point.
    at: Option<Point<Pixels>>,
    scroll: ScrollHandle,
    /// Scroll the current language into view once the list has its size:
    /// on the first frame it has none, and GPUI scrolls by it anyway.
    reveal: Cell<bool>,
    _subscription: Subscription,
}

/// A row of the list.
#[derive(Clone, Copy)]
enum Entry {
    System,
    Language(&'static Language),
}

impl Entry {
    fn tag(self) -> &'static str {
        match self {
            Self::System => "",
            Self::Language(language) => &language.tag,
        }
    }
}

/// A language's flag, 24 × 18 with rounded corners.
pub(super) fn flag(code: &str, th: &Theme) -> AnyElement {
    div()
        .flex_none()
        .w(px(24.0))
        .h(px(18.0))
        .rounded(px(3.0))
        .overflow_hidden()
        .border_1()
        .border_color(rgba(th.divider))
        .child(
            img(SharedString::from(format!("flags/{code}.svg")))
                .w(px(24.0))
                .h(px(18.0)),
        )
        .into_any_element()
}

impl MailWindow {
    /// The rows the search box lets through.
    fn language_entries(&self) -> Vec<Entry> {
        let query = self
            .language_picker
            .as_ref()
            .map(|p| p.query.clone())
            .unwrap_or_default();
        let system = tr!("language-system-default");
        let mut entries = Vec::new();
        let folded = katna_i18n::fold(query.trim());
        if folded.is_empty() || katna_i18n::fold(&system).contains(&folded) {
            entries.push(Entry::System);
        }
        entries.extend(
            katna_i18n::picker()
                .filter(|l| l.matches(&query))
                .map(Entry::Language),
        );
        entries
    }

    pub(super) fn language_picker_open(&self) -> bool {
        self.language_picker.is_some()
    }

    /// Opens the picker (under `at`, else under the top bar's button), or
    /// closes it.
    pub(super) fn toggle_language_picker(
        &mut self,
        at: Option<Point<Pixels>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.language_picker.take().is_some() {
            cx.notify();
            return;
        }
        self.account_menu = false;
        let search = cx.new(|cx| TextInput::new(tr!("language-search"), cx));
        let subscription = cx.subscribe_in(
            &search,
            window,
            |this, input, event: &InputEvent, window, cx| match event {
                InputEvent::Changed => {
                    let query = input.read(cx).text().to_owned();
                    if let Some(picker) = &mut this.language_picker {
                        picker.query = query;
                        picker.highlight = 0;
                        picker.scroll.scroll_to_item(0);
                    }
                    cx.notify();
                }
                InputEvent::Submit => {
                    let entries = this.language_entries();
                    let highlight = this.language_picker.as_ref().map_or(0, |p| p.highlight);
                    if let Some(entry) = entries.get(highlight).copied() {
                        this.pick_language(entry.tag(), window, cx);
                    }
                }
                InputEvent::Cancel => {
                    this.language_picker = None;
                    cx.notify();
                }
            },
        );
        // The current choice is highlighted, and in view.
        let current = self.config.general.language.clone();
        let highlight = std::iter::once("")
            .chain(katna_i18n::picker().map(|l| l.tag.as_str()))
            .position(|tag| tag == current)
            .unwrap_or(0);
        let scroll = ScrollHandle::new();
        let accent: gpui::Hsla = rgba(self.theme(window).accent).into();
        search.update(cx, |input, _| input.set_accent(accent));
        window.focus(&search.focus_handle(cx), cx);
        self.language_picker = Some(LanguagePicker {
            search,
            query: String::new(),
            highlight,
            at,
            scroll,
            reveal: Cell::new(true),
            _subscription: subscription,
        });
        cx.notify();
    }

    fn pick_language(&mut self, tag: &'static str, _window: &mut Window, cx: &mut Context<Self>) {
        self.language_picker = None;
        self.apply(Change::Language(tag), cx);
    }

    fn move_language_highlight(&mut self, by: isize, cx: &mut Context<Self>) {
        let count = self.language_entries().len();
        let Some(picker) = &mut self.language_picker else {
            return;
        };
        if count == 0 {
            return;
        }
        let next = (picker.highlight as isize + by).clamp(0, count as isize - 1) as usize;
        picker.highlight = next;
        picker.scroll.scroll_to_item(next);
        cx.notify();
    }

    /// The top bar's language button: the flag of the language in use and
    /// a small chevron.
    pub(super) fn render_language_button(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let resolved = katna_i18n::current();
        let language = resolved.language;
        let open = self.language_picker.is_some();
        let tooltip = if resolved.system {
            tr!("language-tooltip-system", language = language.name.as_str())
        } else {
            tr!("language-tooltip", language = language.name.as_str())
        };
        div()
            .id("language-button")
            .flex_none()
            .h(px(40.0))
            .px(px(8.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .rounded_full()
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .when(open, |d| d.bg(rgba(th.hover)))
            .on_mouse_move(|_, _, cx| cx.stop_propagation())
            .when(!open, |d| d.tooltip(tip(tooltip, th)))
            .on_click(
                cx.listener(|this, _, window, cx| this.toggle_language_picker(None, window, cx)),
            )
            .child(flag(&language.flag, th))
            .child(icon(
                "chevron-down",
                if open { th.accent } else { th.text_dim },
                18.0,
            ))
            .into_any_element()
    }

    /// How far the language button's right edge is from the top bar's end:
    /// the account picture (with 8 px after it), Settings, and a gap after
    /// each (see `render_top_end`).
    fn language_button_end(&self) -> f32 {
        let (_, room_end) = self.layout.shape.room;
        room_end + BAR_ITEM_GAP + 8.0 + 40.0 + TOP_BAR_GAP + 40.0 + TOP_BAR_GAP
    }

    /// The popover, over a scrim that closes it on a press outside.
    pub(super) fn render_language_picker(
        &self,
        th: &Theme,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let picker = self.language_picker.as_ref()?;
        if picker.reveal.get() {
            if picker.scroll.bounds().size.height > px(0.0) {
                picker.scroll.scroll_to_item(picker.highlight);
                picker.reveal.set(false);
            }
            window.request_animation_frame();
        }
        let phone = self.layout.shape.is_phone();
        let current = self.config.general.language.as_str();
        let entries = self.language_entries();
        let viewport = window.viewport_size();
        let bottom_room = self.layout.shape.bottom_bar();
        let rows = entries.iter().enumerate().map(|(ix, entry)| {
            let entry = *entry;
            let on = entry.tag() == current;
            let lit = ix == picker.highlight;
            let (flag_el, name, english): (AnyElement, SharedString, SharedString) = match entry {
                Entry::System => {
                    let now = katna_i18n::system_language();
                    (
                        div()
                            .w(px(24.0))
                            .flex()
                            .justify_center()
                            .child(icon("language", th.text_dim, 22.0))
                            .into_any_element(),
                        tr!("language-system-default").into(),
                        tr!("language-system-now", language = now.name.as_str()).into(),
                    )
                }
                Entry::Language(language) => (
                    flag(&language.flag, th),
                    language.name.clone().into(),
                    language.english.clone().into(),
                ),
            };
            let tag = entry.tag();
            div()
                .id(("language-row", ix))
                .flex_none()
                .h(px(ROW_HEIGHT))
                .px(px(16.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(16.0))
                .rounded(px(8.0))
                .cursor_pointer()
                .when(on, |d| d.bg(rgba(th.nav_selected)))
                .when(lit && !on, |d| d.bg(rgba(th.hover)))
                .hover(move |s| s.bg(rgba(if on { th.nav_selected } else { th.hover })))
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.pick_language(tag, window, cx);
                }))
                .child(flag_el)
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .truncate()
                                .text_size(px(15.0))
                                .line_height(px(22.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgba(if on { th.nav_selected_text } else { th.text }))
                                .child(name),
                        )
                        .child(
                            div()
                                .truncate()
                                .text_size(px(12.0))
                                .line_height(px(16.0))
                                .text_color(rgba(th.text_faint))
                                .child(english),
                        ),
                )
                .when(on, |d| d.child(icon("check", th.nav_selected_text, 20.0)))
        });
        let empty = entries.is_empty().then(|| {
            div()
                .px(px(16.0))
                .py(px(24.0))
                .text_size(px(14.0))
                .text_color(rgba(th.text_dim))
                .child(tr!(
                    "language-no-match",
                    query = picker.query.trim().to_owned()
                ))
        });
        let machine = (katna_i18n::current().language.status == Status::Machine).then(|| {
            div()
                .id("language-machine")
                .flex_none()
                .cursor_pointer()
                .hover(|s| s.text_color(rgba(th.accent)))
                .on_click(cx.listener(|_, _, _, cx| cx.open_url(TRANSLATION_GUIDE)))
                .mx(px(8.0))
                .mt(px(4.0))
                .pt(px(10.0))
                .pb(px(4.0))
                .px(px(8.0))
                .border_t_1()
                .border_color(rgba(th.divider))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .text_size(px(12.0))
                .text_color(rgba(th.text_dim))
                .child(icon("info", th.text_dim, 16.0))
                .child(div().flex_1().child(tr!("language-machine")))
        });
        let search = div()
            .flex_none()
            .mx(px(8.0))
            .mb(px(8.0))
            .h(px(44.0))
            .px(px(12.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(10.0))
            .rounded(px(12.0))
            .border_1()
            .border_color(rgba(th.accent))
            .child(icon("search", th.text_dim, 20.0))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .text_size(px(15.0))
                    .child(picker.search.clone()),
            );
        let height = if phone {
            viewport.height.as_f32() - super::TOP_BAR_HEIGHT - bottom_room - 16.0
        } else {
            MENU_MAX_HEIGHT.min(viewport.height.as_f32() - super::TOP_BAR_HEIGHT - 24.0)
        };
        let height = if phone || picker.at.is_none() {
            height
        } else {
            height.min(440.0)
        };
        let card = div()
            .id("language-menu")
            .occlude()
            .absolute()
            .map(|d| match (phone, picker.at) {
                (true, _) => d.top(px(4.0)).left(px(8.0)).right(px(8.0)),
                (false, None) => d
                    .top(px(4.0))
                    .right(px((self.language_button_end() - 8.0).max(8.0)))
                    .w(px(MENU_WIDTH)),
                // Under the point it was opened from, inside the window.
                (false, Some(at)) => {
                    let width = viewport.width.as_f32();
                    let left = (at.x.as_f32() - MENU_WIDTH / 2.0)
                        .clamp(8.0, (width - MENU_WIDTH - 8.0).max(8.0));
                    let below = at.y.as_f32() - super::TOP_BAR_HEIGHT + 16.0;
                    let room = viewport.height.as_f32() - super::TOP_BAR_HEIGHT - 16.0;
                    let top = below.min(room - height).max(4.0);
                    d.top(px(top)).left(px(left)).w(px(MENU_WIDTH))
                }
            })
            .h(px(height.max(200.0)))
            .pt(px(8.0))
            .pb(px(8.0))
            .flex()
            .flex_col()
            .map(|d| raised(d, th, super::PANEL_RADIUS, 2.0))
            .text_color(rgba(th.text))
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                match event.keystroke.key.as_str() {
                    "down" => this.move_language_highlight(1, cx),
                    "up" => this.move_language_highlight(-1, cx),
                    "pagedown" => this.move_language_highlight(6, cx),
                    "pageup" => this.move_language_highlight(-6, cx),
                    _ => return,
                }
                cx.stop_propagation();
            }))
            .child(search)
            .child(
                div()
                    .id("language-list")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .track_scroll(&picker.scroll)
                    .px(px(8.0))
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .children(rows)
                    .children(empty),
            )
            .children(machine)
            .with_animation(
                "language-menu",
                Animation::new(Duration::from_millis(180)).with_easing(gpui::ease_out_quint()),
                |el, t| el.opacity(t).mt(px(-8.0 * (1.0 - t))),
            );
        let close = || {
            cx.listener(|this: &mut Self, _: &MouseDownEvent, _, cx| {
                this.language_picker = None;
                cx.notify();
            })
        };
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .child(
                    deferred(
                        div()
                            .id("language-menu-scrim")
                            .absolute()
                            .top(px(-2000.0))
                            .left(px(-4000.0))
                            .w(px(8000.0))
                            .h(px(6000.0))
                            .occlude()
                            .on_mouse_down(MouseButton::Left, close())
                            .on_mouse_down(MouseButton::Right, close()),
                    )
                    .with_priority(1),
                )
                .child(deferred(card).with_priority(2))
                .into_any_element(),
        )
    }
}
