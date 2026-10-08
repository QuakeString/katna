// SPDX-License-Identifier: GPL-3.0-or-later

//! The command palette (Ctrl+Shift+P, as in VS Code, Zed and Kate): one
//! box under the top bar that finds any action or setting by name. Up and
//! Down move, Enter runs, Esc closes. With nothing typed it lists what was
//! run last, then the actions; each action shows its keys, so the palette
//! also teaches them.

use std::ops::Range;

use gpui::{
    AnyElement, Context, Entity, FocusHandle, Focusable, FontWeight, HighlightStyle, KeyDownEvent,
    MouseButton, ScrollHandle, SharedString, StyledText, Subscription, UnderlineStyle, Window, div,
    prelude::*, rgba,
};
use katna_i18n::tr;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::text_input::{Down, Up};
use katna_ui::tokens::{radius, space, text};
use katna_ui::{InputEvent, TextInput, px};

use super::keymap::{self, Group, SHORTCUTS, Scope, Shortcut};
use super::settings_page::Scope as Place;
use super::settings_search::{self, Found};
use super::{MailWindow, ShowPalette};
use crate::theme::{Theme, fade};
use crate::widgets::icon;

/// The card's width beside the list.
const WIDTH: f32 = 600.0;
/// How far below the top bar it opens.
const BELOW_BAR: f32 = 8.0;
const FIELD_HEIGHT: f32 = 56.0;
const ROW_HEIGHT: f32 = 40.0;
const FOOT_HEIGHT: f32 = 38.0;
/// How many things it remembers as Recent.
const RECENT: usize = 5;
/// The most results it lists.
const MOST: usize = 60;

pub(super) struct Palette {
    search: Entity<TextInput>,
    _search_changed: Subscription,
    scroll: ScrollHandle,
    /// The line Enter runs.
    highlight: usize,
    /// Where the keys were before it opened: they go back there, and an
    /// action runs from there.
    back: Option<FocusHandle>,
    closing: bool,
    shown: Spring,
}

/// What a line runs.
#[derive(Clone)]
enum Command {
    Shortcut(&'static Shortcut),
    Setting(Found),
}

/// A line of the palette.
#[derive(Clone)]
struct Item {
    command: Command,
    title: String,
    /// Before the title, dimmer: "Go to:".
    prefix: Option<String>,
    /// After the title, small: where a setting is.
    place: Option<String>,
    icon: &'static str,
    /// Bytes of the title that match what was typed.
    marks: Vec<Range<usize>>,
    /// Names it for Recent.
    key: String,
}

impl MailWindow {
    pub(super) fn show_palette(
        &mut self,
        _: &ShowPalette,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.palette_open() {
            self.close_palette(window, cx);
            return;
        }
        self.settings_open = false;
        self.menu = None;
        let back = window.focused(cx);
        let accent = rgba(self.theme(window).accent).into();
        let search = cx.new(|cx| {
            let mut input = TextInput::new(tr!("palette-search"), cx);
            input.set_accent(accent);
            input
        });
        let changed = cx.subscribe_in(
            &search,
            window,
            |this, _, event: &InputEvent, window, cx| match event {
                InputEvent::Changed => {
                    if let Some(palette) = &mut this.palette {
                        palette.highlight = 0;
                        palette.scroll.set_offset(gpui::point(px(0.0), px(0.0)));
                    }
                    cx.notify();
                }
                InputEvent::Submit => this.run_highlighted(window, cx),
                InputEvent::Cancel => this.close_palette(window, cx),
            },
        );
        window.focus(&search.read(cx).focus_handle(cx), cx);
        let mut shown = Spring::new(motion::SMOOTH, 0.0);
        shown.set(1.0);
        self.palette = Some(Palette {
            search,
            _search_changed: changed,
            scroll: ScrollHandle::new(),
            highlight: 0,
            back,
            closing: false,
            shown,
        });
        cx.notify();
    }

    /// The palette is open and not on its way out.
    pub(super) fn palette_open(&self) -> bool {
        self.palette.as_ref().is_some_and(|p| !p.closing)
    }

    pub(super) fn close_palette(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(palette) = &mut self.palette
            && !palette.closing
        {
            palette.closing = true;
            palette.shown.set(0.0);
            match (&palette.back, &self.settings_page) {
                (Some(back), _) => window.focus(back, cx),
                (None, Some(page)) => window.focus(&page.focus, cx),
                (None, None) => window.focus(&self.list_focus, cx),
            }
        }
        cx.notify();
    }

    /// The lines for what is typed, and how many of them are Recent.
    fn palette_items(&self, query: &str) -> (Vec<Item>, usize) {
        let shortcuts = SHORTCUTS
            .iter()
            .filter(|s| self.palette_has(s))
            .map(shortcut_item);
        let settings = settings_search::candidates(false)
            .into_iter()
            .filter(|(found, _)| {
                !settings_search::is_coming(found.section)
                    && found
                        .section
                        .scope()
                        .app()
                        .is_none_or(|app| self.config.app_on(app))
            })
            .map(|(found, words)| (setting_item(found), words));
        if query.trim().is_empty() {
            let all: Vec<Item> = shortcuts.collect();
            let mut settings: Vec<Item> = settings.map(|(item, _)| item).collect();
            let recent: Vec<Item> = self
                .config
                .shortcuts
                .recent
                .iter()
                .filter_map(|key| {
                    all.iter().find(|i| i.key == *key).cloned().or_else(|| {
                        let ix = settings.iter().position(|i| i.key == *key)?;
                        Some(settings.swap_remove(ix))
                    })
                })
                .take(RECENT)
                .collect();
            let count = recent.len();
            let rest: Vec<Item> = all
                .into_iter()
                .filter(|i| !recent.iter().any(|r| r.key == i.key))
                .collect();
            return (recent.into_iter().chain(rest).collect(), count);
        }
        // "Go to:" and the English name find an action too.
        let english = |item: &Item| match &item.command {
            Command::Shortcut(s) => match &item.prefix {
                Some(prefix) => format!(
                    "{prefix} {} {}",
                    katna_i18n::english("palette-go-to"),
                    s.english_title()
                ),
                None => s.english_title(),
            },
            Command::Setting(_) => String::new(),
        };
        let mut found: Vec<(u8, Item)> = shortcuts
            .map(|item| {
                let words = english(&item);
                (item, words)
            })
            .chain(settings)
            .filter_map(|(mut item, words)| {
                let extra = format!("{} {words}", item.place.as_deref().unwrap_or_default());
                let (rank, marks) = matches(query, &item.title, &extra)?;
                item.marks = marks;
                Some((rank, item))
            })
            .collect();
        // Stable: actions before settings of the same rank.
        found.sort_by_key(|(rank, _)| *rank);
        (found.into_iter().take(MOST).map(|(_, i)| i).collect(), 0)
    }

    /// Whether the palette lists `shortcut`: an action, not a key for
    /// moving about, and not one for an app turned off or AI that is off.
    fn palette_has(&self, shortcut: &Shortcut) -> bool {
        shortcut.group != Group::Moving
            && shortcut.name != "palette"
            && shortcut.app().is_none_or(|app| self.config.app_on(app))
            && (shortcut.name != "summarize" || self.summaries_on())
    }

    fn palette_step(&mut self, step: isize, cx: &mut Context<Self>) {
        let Some(palette) = &self.palette else {
            return;
        };
        let query = palette.search.read(cx).text().to_owned();
        let (items, recent) = self.palette_items(&query);
        let Some(palette) = &mut self.palette else {
            return;
        };
        if !items.is_empty() {
            let ix = (palette.highlight as isize + step).rem_euclid(items.len() as isize) as usize;
            palette.highlight = ix;
            let heading = recent > 0 || !query.trim().is_empty();
            palette
                .scroll
                .scroll_to_item(child_index(ix, recent, heading));
        }
        cx.notify();
    }

    fn run_highlighted(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(palette) = &self.palette else {
            return;
        };
        let query = palette.search.read(cx).text().to_owned();
        let highlight = palette.highlight;
        let (items, _) = self.palette_items(&query);
        if let Some(item) = items.into_iter().nth(highlight) {
            self.run_palette_item(item, window, cx);
        }
    }

    /// Closes the palette and runs `item` where the keys were.
    fn run_palette_item(&mut self, item: Item, window: &mut Window, cx: &mut Context<Self>) {
        let recent = &mut self.config.shortcuts.recent;
        recent.retain(|key| *key != item.key);
        recent.insert(0, item.key);
        recent.truncate(RECENT);
        self.save_config();
        self.close_palette(window, cx);
        match item.command {
            Command::Shortcut(shortcut) => {
                // Reply and the like work in the open conversation.
                if shortcut.scope == Scope::Reader && self.reader.is_some() {
                    window.focus(&self.reader_focus, cx);
                }
                window.dispatch_action(shortcut.action(), cx);
            }
            Command::Setting(found) => self.go_to_setting(found, window, cx),
        }
    }

    fn palette_key(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        // Escape reaches the field first; this is for when it does not.
        if event.keystroke.key == "escape" {
            self.close_palette(window, cx);
            cx.stop_propagation();
        }
    }

    pub(super) fn render_palette(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        // It floats: its surface is a step lighter in dark colors.
        let th = &th.lifted();
        let palette = self.palette.as_mut()?;
        let t = palette.shown.tick(window, reduce);
        if palette.closing && palette.shown.settled() {
            self.palette = None;
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        let palette = self.palette.as_ref()?;
        let query = palette.search.read(cx).text().to_owned();
        let (items, recent) = self.palette_items(&query);
        let highlight = palette.highlight.min(items.len().saturating_sub(1));
        let vw = self.room_width();
        let width = WIDTH.min(vw - 2.0 * space::S3);
        // Dialogs are laid out below the top bar.
        let top = BELOW_BAR;
        let room = self.room_height(window) - super::TOP_BAR_HEIGHT - top - space::S5;
        let list_max = (room - FIELD_HEIGHT - FOOT_HEIGHT).max(ROW_HEIGHT * 3.0);

        let field = div()
            .flex_none()
            .h(px(FIELD_HEIGHT))
            .pl(px(space::S5))
            .pr(px(space::S5))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(space::S4))
            .border_b_1()
            .border_color(rgba(th.divider))
            .child(icon("search", th.text_dim, 20.0))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .text_size(px(text::SUBTITLE))
                    .child(palette.search.clone()),
            )
            .child(caps(&["Esc".to_owned()], th));

        let mut children: Vec<AnyElement> = Vec::new();
        let head = |label: String| {
            div()
                .flex_none()
                .h(px(30.0))
                .px(px(space::S4))
                .pb(px(space::S2))
                .flex()
                .items_end()
                .text_size(px(text::CAPTION))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgba(th.text_faint))
                .child(label)
                .into_any_element()
        };
        if items.is_empty() {
            children.push(
                div()
                    .h(px(ROW_HEIGHT * 2.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(text::BODY))
                    .text_color(rgba(th.text_dim))
                    .child(tr!("palette-none", query = query.trim()))
                    .into_any_element(),
            );
        } else if query.trim().is_empty() {
            if recent > 0 {
                children.push(head(tr!("palette-recent")));
            }
        } else {
            children.push(head(tr!("palette-results", count = items.len())));
        }
        for (ix, item) in items.into_iter().enumerate() {
            if ix == recent && recent > 0 {
                children.push(
                    div()
                        .flex_none()
                        .mx(px(space::S3))
                        .my(px(space::S2))
                        .h(px(1.0))
                        .bg(rgba(th.divider))
                        .into_any_element(),
                );
            }
            children.push(self.palette_row(ix, item, ix == highlight, th, cx));
        }
        let list = div()
            .id("palette-list")
            .flex_1()
            .min_h_0()
            .max_h(px(list_max))
            .overflow_y_scroll()
            .track_scroll(&palette.scroll)
            .px(px(space::S3))
            .pb(px(space::S3))
            .flex()
            .flex_col()
            .role(gpui::Role::ListBox)
            .aria_label(tr!("palette-search"))
            .children(children);

        let foot = div()
            .flex_none()
            .h(px(FOOT_HEIGHT))
            .px(px(space::S5))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(space::S5))
            .border_t_1()
            .border_color(rgba(th.divider))
            .text_size(px(text::CAPTION))
            .text_color(rgba(th.text_faint))
            .child(hint(
                caps(&["\u{2191}".to_owned(), "\u{2193}".to_owned()], th),
                tr!("palette-move"),
            ))
            .child(hint(caps(&["Enter".to_owned()], th), tr!("palette-run")))
            .child(hint(caps(&["Esc".to_owned()], th), tr!("palette-close")))
            .child(div().flex_1())
            .children(
                keymap::hint("palette", &self.config.shortcuts).map(|keys| div().child(keys)),
            );

        let card = div()
            .id("palette")
            .on_action(cx.listener(|this, _: &Up, _, cx| this.palette_step(-1, cx)))
            .on_action(cx.listener(|this, _: &Down, _, cx| this.palette_step(1, cx)))
            .on_key_down(cx.listener(Self::palette_key))
            .role(gpui::Role::Dialog)
            .aria_label(tr!("shortcut-palette"))
            .occlude()
            .w(px(width))
            .max_h(px(room))
            .flex()
            .flex_col()
            .map(|d| crate::widgets::dialog(d, th, th.surface))
            .text_color(rgba(th.text))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(field)
            .child(list)
            .child(foot);
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .flex_col()
                .items_center()
                .pt(px(top))
                // No veil: the window stays as it is around the palette.
                .child(
                    div()
                        .id("palette-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .on_click(
                            cx.listener(|this, _, window, cx| this.close_palette(window, cx)),
                        ),
                )
                .child(
                    div()
                        .opacity(t)
                        .mt(px(lerp(-space::S3, 0.0, t)))
                        .child(card),
                )
                .into_any_element(),
        )
    }

    fn palette_row(
        &self,
        ix: usize,
        item: Item,
        lit: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let ink = if lit { th.nav_selected_text } else { th.text };
        let dim = if lit {
            fade(th.nav_selected_text, 0.72)
        } else {
            th.text_faint
        };
        let mark = HighlightStyle {
            font_weight: Some(FontWeight::BOLD),
            color: Some(rgba(if lit { th.nav_selected_text } else { th.accent }).into()),
            underline: lit.then(|| UnderlineStyle {
                thickness: px(1.5),
                color: Some(rgba(th.nav_selected_text).into()),
                wavy: false,
            }),
            ..HighlightStyle::default()
        };
        let title = StyledText::new(SharedString::from(item.title.clone())).with_highlights(
            item.marks
                .iter()
                .filter(|m| m.end <= item.title.len())
                .map(|m| (m.clone(), mark)),
        );
        let keys = match &item.command {
            Command::Shortcut(s) => self.palette_caps(s, th),
            Command::Setting(_) => None,
        };
        let (prefix, place, glyph) = (item.prefix.clone(), item.place.clone(), item.icon);
        let spoken = match &item.prefix {
            Some(prefix) => format!("{prefix} {}", item.title),
            None => item.title.clone(),
        };
        div()
            .id(("palette-row", ix))
            .flex_none()
            .h(px(ROW_HEIGHT))
            .pl(px(space::S4))
            .pr(px(space::S3))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(space::S4))
            .rounded(px(radius::SM))
            .cursor_pointer()
            .when(lit, |d| d.bg(rgba(th.nav_selected)))
            .when(!lit, |d| d.hover(|s| s.bg(rgba(th.hover))))
            .role(gpui::Role::ListBoxOption)
            .aria_label(spoken)
            .aria_selected(lit)
            .on_click(cx.listener(move |this, _, window, cx| {
                cx.stop_propagation();
                this.run_palette_item(item.clone(), window, cx);
            }))
            .child(icon(glyph, if lit { ink } else { th.text_dim }, 18.0))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_row()
                    .items_baseline()
                    .gap(px(space::S2))
                    .text_size(px(text::BODY))
                    .text_color(rgba(ink))
                    .whitespace_nowrap()
                    .children(
                        prefix.map(|prefix| div().flex_none().text_color(rgba(dim)).child(prefix)),
                    )
                    .child(div().flex_none().child(title))
                    .children(place.map(|place| {
                        div()
                            .ml(px(space::S2))
                            .min_w_0()
                            .truncate()
                            .text_size(px(text::CAPTION))
                            .text_color(rgba(dim))
                            .child(place)
                    })),
            )
            .when(lit, |d| {
                d.child(
                    div()
                        .flex_none()
                        .mr(px(space::S2))
                        .text_size(px(text::CAPTION))
                        .text_color(rgba(dim))
                        .child(tr!("palette-enter")),
                )
            })
            .children(keys)
            .into_any_element()
    }

    /// The key caps of a shortcut's keys that work now, or `None` with none.
    fn palette_caps(&self, shortcut: &Shortcut, th: &Theme) -> Option<AnyElement> {
        let config = &self.config.shortcuts;
        let keys = keymap::keys(shortcut, config);
        let keys = keys
            .iter()
            .find(|k| config.single_keys || !keymap::is_single_key(k))?;
        let mut row = div()
            .flex_none()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(space::S2));
        for (ix, stroke) in keys.split_whitespace().enumerate() {
            if ix > 0 {
                row = row.child(
                    div()
                        .px(px(space::S1))
                        .text_size(px(text::MICRO))
                        .text_color(rgba(th.text_faint))
                        .child(tr!("palette-then")),
                );
            }
            row = row.child(caps(&keymap::stroke_parts(stroke), th));
        }
        Some(row.into_any_element())
    }
}

/// The keys pressed together, a cap each.
fn caps(keys: &[String], th: &Theme) -> AnyElement {
    div()
        .flex_none()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(space::S2))
        .children(keys.iter().map(|key| {
            div()
                .h(px(22.0))
                .min_w(px(22.0))
                .px(px(space::S2))
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(radius::XS))
                .border_1()
                .border_b_2()
                .border_color(rgba(th.outline))
                .bg(rgba(th.page))
                .text_size(px(text::CAPTION))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgba(th.text_dim))
                .child(key.clone())
        }))
        .into_any_element()
}

/// A key and what it does, for the foot.
fn hint(keys: AnyElement, what: String) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(space::S2))
        .child(keys)
        .child(what)
}

/// The list's child that holds line `ix`: a heading (Recent, or the
/// count of results) comes first when there is one, and a line under
/// Recent.
fn child_index(ix: usize, recent: usize, heading: bool) -> usize {
    ix + usize::from(heading) + usize::from(recent > 0 && ix >= recent)
}

fn shortcut_item(shortcut: &'static Shortcut) -> Item {
    let go_to = shortcut.group == Group::GoTo;
    Item {
        command: Command::Shortcut(shortcut),
        title: shortcut.title(),
        prefix: go_to.then(|| tr!("palette-go-to")),
        place: None,
        icon: shortcut_icon(shortcut.name),
        marks: Vec::new(),
        key: shortcut.name.to_owned(),
    }
}

fn setting_item(found: Found) -> Item {
    // An app's page says whose it is: "Settings › Mail › Reading".
    let scope = found.section.scope();
    let page = if scope == Place::Katna || scope.label() == found.section.label() {
        found.section.label()
    } else {
        format!("{} \u{203a} {}", scope.label(), found.section.label())
    };
    let tab = found.row.is_none();
    Item {
        title: found.title.to_string(),
        prefix: None,
        place: (!tab).then(|| tr!("palette-setting-place", page = page)),
        icon: if tab { "settings" } else { "tune" },
        marks: Vec::new(),
        key: format!("setting-{}", found.key),
        command: Command::Setting(found),
    }
}

fn shortcut_icon(name: &str) -> &'static str {
    match name {
        "compose" => "compose",
        "reply" => "reply",
        "reply_all" => "reply-all",
        "forward" => "forward",
        "archive" => "archive",
        "delete" => "trash",
        "spam" => "junk",
        "move_to" => "move-to",
        "mark_read" => "mark-read",
        "mark_unread" => "mark-unread",
        "star" | "go_starred" => "star",
        "add_to_tasks" | "page_tasks" => "tasks",
        "snooze" => "snooze",
        "remind" => "bell",
        "important" | "not_important" => "important",
        "mute" => "bell-off",
        "summarize" => "sparkle",
        "check" => "checkbox-checked",
        "select_all" => "done-all",
        "select_none" => "checkbox",
        "undo" => "undo",
        "go_inbox" => "inbox",
        "go_sent" => "sent",
        "go_drafts" => "drafts",
        "go_all" => "all-mail",
        "page_mail" => "mail",
        "page_calendar" => "calendar",
        "page_contacts" => "contacts",
        "page_notes" => "notes",
        "page_files" => "file",
        "search" => "search",
        "navigation" => "menu",
        "quick_settings" => "tune",
        "settings" => "settings",
        "shortcuts" => "info",
        "reload" => "refresh",
        "quit" => "power",
        _ => "bolt",
    }
}

/// Whether `title` (or else `extra`, more words it is found by) answers
/// `query`, how well (lower is better), and the bytes of `title` that
/// match. The whole query in the title, at a word's start, is best; then
/// anywhere in it; then every word of it in the title; then every word in
/// the title or `extra`, some in the title first; then its letters in
/// order in the title, as in VS Code ("mku" finds "Mark as unread").
fn matches(query: &str, title: &str, extra: &str) -> Option<(u8, Vec<Range<usize>>)> {
    let query = query.trim().to_lowercase();
    if query.is_empty() {
        return Some((0, Vec::new()));
    }
    let lower = Lower::new(title);
    if let Some(at) = lower.find(&query, 0) {
        let word_start = lower.text[..at]
            .chars()
            .next_back()
            .is_none_or(|c| !c.is_alphanumeric());
        let rank = if word_start { 0 } else { 1 };
        return Some((rank, vec![lower.bytes(at, query.len())]));
    }
    let words: Vec<&str> = query.split_whitespace().collect();
    let in_title: Vec<Option<usize>> = words.iter().map(|w| lower.find(w, 0)).collect();
    let marks = || {
        words
            .iter()
            .zip(&in_title)
            .filter_map(|(w, at)| at.map(|at| lower.bytes(at, w.len())))
            .collect()
    };
    if in_title.iter().all(Option::is_some) {
        return Some((2, marks()));
    }
    let extra = extra.to_lowercase();
    if words
        .iter()
        .zip(&in_title)
        .all(|(w, at)| at.is_some() || extra.contains(w))
    {
        // Better with some of it in the title.
        let rank = if in_title.iter().any(Option::is_some) {
            3
        } else {
            4
        };
        return Some((rank, marks()));
    }
    // The letters in order, each found after the last.
    let mut marks = Vec::new();
    let mut from = 0;
    for c in query.chars().filter(|c| !c.is_whitespace()) {
        let mut buf = [0; 4];
        let at = lower.find(c.encode_utf8(&mut buf), from)?;
        marks.push(lower.bytes(at, c.len_utf8()));
        from = at + c.len_utf8();
    }
    Some((5, marks))
}

/// A title in lower case, with where each of its bytes came from in the
/// title as written, since lower case can change a letter's length.
struct Lower {
    text: String,
    /// For each byte of `text`, the byte of the title it came from.
    from: Vec<usize>,
    /// For each byte of `text`, where the title's letter it came from ends.
    to: Vec<usize>,
}

impl Lower {
    fn new(title: &str) -> Self {
        let mut text = String::new();
        let mut from = Vec::new();
        let mut to = Vec::new();
        for (at, c) in title.char_indices() {
            for l in c.to_lowercase() {
                let start = text.len();
                text.push(l);
                for _ in start..text.len() {
                    from.push(at);
                    to.push(at + c.len_utf8());
                }
            }
        }
        Self { text, from, to }
    }

    /// Where `needle` is in the lower-case text, at or after byte `start`.
    fn find(&self, needle: &str, start: usize) -> Option<usize> {
        self.text.get(start..)?.find(needle).map(|at| at + start)
    }

    /// The title's bytes for `len` bytes of the lower-case text at `at`.
    fn bytes(&self, at: usize, len: usize) -> Range<usize> {
        self.from[at]..self.to[at + len - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rank(query: &str, title: &str) -> Option<u8> {
        matches(query, title, "").map(|(rank, _)| rank)
    }

    #[test]
    #[allow(clippy::single_range_in_vec_init)]
    fn matching_ranks_and_marks() {
        assert_eq!(matches("mark", "Mark as read", ""), Some((0, vec![0..4])));
        assert_eq!(rank("as read", "Mark as read"), Some(0));
        assert_eq!(rank("ark", "Mark as read"), Some(1));
        assert_eq!(
            matches("read mark", "Mark as read", ""),
            Some((2, vec![8..12, 0..4]))
        );
        assert_eq!(
            matches("dark", "Mode", "dark light theme"),
            Some((4, Vec::new()))
        );
        assert_eq!(matches("go sent", "Sent", "go to:"), Some((3, vec![0..4])));
        assert_eq!(
            matches("mku", "Mark as unread", ""),
            Some((5, vec![0..1, 3..4, 8..9]))
        );
        assert_eq!(rank("zz", "Mark as read"), None);
        assert_eq!(matches("  ", "Archive", ""), Some((0, Vec::new())));
        // Case, and a letter whose lower case is longer.
        assert_eq!(matches("ARCH", "Archive", ""), Some((0, vec![0..4])));
        assert_eq!(matches("i̇s", "İstanbul", ""), Some((0, vec![0..3])));
    }

    #[test]
    fn every_listed_shortcut_has_an_icon_and_a_title() {
        for s in SHORTCUTS.iter().filter(|s| s.group != Group::Moving) {
            let item = shortcut_item(s);
            assert!(!item.title.starts_with("shortcut-"), "{}", s.name);
            if s.name != "palette" {
                assert_ne!(item.icon, "bolt", "no icon for {}", s.name);
            }
        }
    }

    #[test]
    fn rows_follow_their_headings() {
        // Nothing typed, two Recent: heading, 2 lines, a line, the rest.
        assert_eq!(child_index(0, 2, true), 1);
        assert_eq!(child_index(1, 2, true), 2);
        assert_eq!(child_index(2, 2, true), 4);
        // Results: their heading, then the lines.
        assert_eq!(child_index(0, 0, true), 1);
        // Nothing typed and nothing recent: no heading.
        assert_eq!(child_index(3, 0, false), 3);
    }
}
