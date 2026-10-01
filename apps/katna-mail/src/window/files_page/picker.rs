// SPDX-License-Identifier: GPL-3.0-or-later

//! The attach picker: the chat reply box's paperclip > From Files. It is
//! the Files page in a panel over the chat, with the page's files, search
//! and chips, the files of this conversation first. A click ticks a file;
//! the eye (or the viewer's Select) looks before picking. Attach adds the
//! ticked files, downloading their mail first if need be. The foot weighs
//! them against the 25 MB a mail carries: on an account with Google Drive
//! or OneDrive the rest goes there, biggest first, as in Compose; on
//! others Attach waits until they fit.
//!
//! The picker borrows the page's filters while it is open and gives them
//! back when it closes, so the page's chips, menus and calendar serve it.

use std::collections::HashSet;
use std::ops::Range;
use std::rc::Rc;

use gpui::{
    Animation, AnimationExt, AnyElement, Context, Entity, Focusable, FontWeight, ListAlignment,
    ListState, SharedString, Subscription, Window, div, ease_out_quint, list, prelude::*, rgba,
};
use katna_core::{AccountId, OAuthProvider};
use katna_i18n::tr;
use katna_store::MessageId;
use katna_ui::px;
use katna_ui::text_input::{InputEvent, TextInput};
use katna_ui::unpx;

use super::super::MailWindow;
use super::super::attachments::{CARD_RADIUS, Item, card_top, row_file_index};
use super::super::compose::attach::{MAX_TOTAL, cloud_bound, limit_text};
use super::{Direction, Found, Sort, Time, Types};
use crate::data::EntryKey;
use crate::format;
use crate::theme::{Theme, fade};
use crate::widgets::{filled_button, icon, icon_button, placeholder, raised, tip};

/// Cards are at least this wide; the rest of a row is shared out.
const CARD_MIN: f32 = 132.0;
const GAP: f32 = 10.0;
const THUMB: f32 = 84.0;
const FOOT: f32 = 44.0;
const HEADING: f32 = 30.0;
/// Space between the panel and the edges of the chat.
const INSET: f32 = 10.0;
const PAD: f32 = 14.0;
/// The kinds of file the chips offer; the rest are under All files.
const KINDS: [Types; 5] = [
    Types::All,
    Types::Pictures,
    Types::Pdfs,
    Types::Documents,
    Types::Sheets,
];
/// The meter turns amber this full.
const NEARLY: f32 = 0.8;

/// The Files page's filters, kept while the picker borrows them.
struct Saved {
    query: String,
    types: Types,
    account: Option<AccountId>,
    direction: Direction,
    person: Option<String>,
    time: Time,
    sort: Sort,
    grid: bool,
}

/// A line of the picker.
#[derive(Debug, Clone)]
enum Line {
    /// "In this conversation" (`true`) or "Recent".
    Heading(bool),
    /// A row of cards: places in `shown`.
    Cards(Range<usize>),
}

/// The open attach picker.
pub(in crate::window) struct Picker {
    /// The conversation whose reply box it attaches to.
    key: EntryKey,
    input: Entity<TextInput>,
    _input: Subscription,
    saved: Saved,
    /// The conversation's mail, whose files come first.
    in_chat: HashSet<MessageId>,
    /// The ticked files, in the order they were ticked.
    ticked: Vec<(MessageId, usize)>,
    /// The files shown, in order: places in the page's files.
    shown: Rc<Vec<usize>>,
    lines: Rc<Vec<Line>>,
    columns: usize,
    stale: bool,
    state: ListState,
    /// The viewer shows one of its files: the arrows step through them.
    pub(in crate::window) previewing: bool,
}

/// How the ticked files weigh against what a mail carries.
struct Weight {
    count: usize,
    /// Everything the mail would carry, ticked files included.
    total: u64,
    /// The part of it going through the cloud.
    cloud: u64,
    provider: Option<OAuthProvider>,
}

impl Weight {
    /// More than a mail carries, with no cloud to take the rest.
    fn over(&self) -> bool {
        self.provider.is_none() && self.total > MAX_TOTAL as u64
    }
}

impl MailWindow {
    /// Opens the picker over the open conversation's chat.
    pub(in crate::window) fn open_files_picker(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(reader) = &self.reader else {
            return;
        };
        let key = reader.key;
        let in_chat = reader.message_ids();
        if self.picker.is_some() {
            self.close_files_picker(cx);
        }
        self.load_library(cx);
        let library = &mut self.library;
        let saved = Saved {
            query: std::mem::take(&mut library.query),
            types: std::mem::replace(&mut library.types, Types::All),
            account: library.account.take(),
            direction: std::mem::replace(&mut library.direction, Direction::Any),
            person: library.person.take(),
            time: std::mem::replace(&mut library.time, Time::Any),
            sort: std::mem::replace(&mut library.sort, Sort::Newest),
            grid: std::mem::replace(&mut library.grid, true),
        };
        library.changed();
        let accent = rgba(self.theme(window).accent).into();
        let input = cx.new(|cx| {
            let mut input = TextInput::new(tr!("picker-search"), cx);
            input.set_accent(accent);
            input
        });
        let subscription = cx.subscribe_in(
            &input,
            window,
            |this, input, event: &InputEvent, _, cx| match event {
                InputEvent::Changed => {
                    this.library.query = input.read(cx).text().to_owned();
                    this.library.changed();
                    cx.notify();
                }
                InputEvent::Cancel => this.close_files_picker(cx),
                InputEvent::Submit => {}
            },
        );
        window.focus(&input.focus_handle(cx), cx);
        self.picker = Some(Picker {
            key,
            input,
            _input: subscription,
            saved,
            in_chat,
            ticked: Vec::new(),
            shown: Rc::default(),
            lines: Rc::default(),
            columns: 0,
            stale: true,
            state: ListState::new(0, ListAlignment::Top, px(600.0)),
            previewing: false,
        });
        cx.notify();
    }

    /// Closes the picker, giving the Files page its filters back.
    pub(in crate::window) fn close_files_picker(&mut self, cx: &mut Context<Self>) {
        let Some(picker) = self.picker.take() else {
            return;
        };
        let saved = picker.saved;
        let library = &mut self.library;
        library.query = saved.query;
        library.types = saved.types;
        library.account = saved.account;
        library.direction = saved.direction;
        library.person = saved.person;
        library.time = saved.time;
        library.sort = saved.sort;
        library.grid = saved.grid;
        library.changed();
        // The reply box takes the keys back.
        if let Some(body) = self.chat_reply_focus(cx) {
            cx.defer(move |cx| {
                if let Some(window) = cx.active_window() {
                    window
                        .update(cx, |_, window, cx| window.focus(&body, cx))
                        .ok();
                }
            });
        }
        cx.notify();
    }

    /// Esc closes the picker before anything else; whether it was open.
    pub(in crate::window) fn fold_files_picker(&mut self, cx: &mut Context<Self>) -> bool {
        if self.picker.is_none() {
            return false;
        }
        self.close_files_picker(cx);
        true
    }

    /// Makes the picker's lines again for `columns` cards a row, when the
    /// files or the filters changed.
    fn rebuild_picker(&mut self, columns: usize) {
        let Some(picker) = &self.picker else {
            return;
        };
        if !self.library.stale && !picker.stale && picker.columns == columns {
            return;
        }
        self.library.rebuild(columns, THUMB + FOOT + GAP);
        let Some(files) = self.library.found().cloned() else {
            return;
        };
        let shown = self.library.shown.clone();
        let Some(picker) = &mut self.picker else {
            return;
        };
        let (mut first, rest): (Vec<usize>, Vec<usize>) = shown
            .iter()
            .partition(|&&ix| picker.in_chat.contains(&files[ix].file.message));
        let mut lines = Vec::new();
        let rows = |lines: &mut Vec<Line>, from: usize, to: usize| {
            let mut at = from;
            while at < to {
                let end = (at + columns.max(1)).min(to);
                lines.push(Line::Cards(at..end));
                at = end;
            }
        };
        if !first.is_empty() {
            lines.push(Line::Heading(true));
            rows(&mut lines, 0, first.len());
        }
        if !rest.is_empty() {
            lines.push(Line::Heading(false));
            rows(&mut lines, first.len(), first.len() + rest.len());
        }
        first.extend(rest);
        picker.shown = Rc::new(first);
        picker.lines = Rc::new(lines);
        picker.columns = columns;
        picker.stale = false;
        picker
            .state
            .reset_with_uniform_height(picker.lines.len(), px(THUMB + FOOT + GAP));
    }

    fn toggle_pick(&mut self, key: (MessageId, usize), cx: &mut Context<Self>) {
        let Some(picker) = &mut self.picker else {
            return;
        };
        match picker.ticked.iter().position(|k| *k == key) {
            Some(at) => {
                picker.ticked.remove(at);
            }
            None => picker.ticked.push(key),
        }
        cx.notify();
    }

    /// The viewer's Select: ticks or unticks the file it shows.
    pub(in crate::window) fn pick_previewed(&mut self, cx: &mut Context<Self>) {
        let Some(viewer) = self.files.viewer.clone() else {
            return;
        };
        let Some((place, _)) = viewer.read(cx).library else {
            return;
        };
        let Some(key) = self.picker_key_at(place) else {
            return;
        };
        self.toggle_pick(key, cx);
        let on = self.picker_ticked(key);
        viewer.update(cx, |viewer, cx| {
            viewer.pick = Some(on);
            cx.notify();
        });
    }

    fn picker_key_at(&self, place: usize) -> Option<(MessageId, usize)> {
        let picker = self.picker.as_ref()?;
        let ix = *picker.shown.get(place)?;
        Some(self.library.found()?.get(ix)?.key())
    }

    fn picker_ticked(&self, key: (MessageId, usize)) -> bool {
        self.picker
            .as_ref()
            .is_some_and(|p| p.ticked.contains(&key))
    }

    /// Opens the file at `place` in the viewer, over the picker, its
    /// arrows stepping through the picker's files. A file whose mail is
    /// not downloaded opens once it is.
    fn preview_pick(&mut self, place: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(picker) = &self.picker else {
            return;
        };
        let count = picker.shown.len();
        let Some(&ix) = picker.shown.get(place) else {
            return;
        };
        let Some(found) = self.library.found().and_then(|f| f.get(ix)) else {
            return;
        };
        let file = found.row_file();
        let key = found.key();
        let Some((raw, encrypted)) = self.attachment_raw(file.message) else {
            let done = self.download(file.message, cx);
            self.show_snackbar(tr!("files-downloading"), None, cx);
            cx.spawn_in(window, async move |this, cx| {
                done.recv().await.ok();
                this.update_in(cx, |this, window, cx| {
                    if this.attachment_raw(file.message).is_some() {
                        this.preview_pick(place, window, cx);
                    } else {
                        this.show_snackbar(tr!("files-download-failed"), None, cx);
                    }
                })
                .ok();
            })
            .detach();
            return;
        };
        let view = katna_render::message_view(&raw);
        let Some(index) = row_file_index(&view.attachments, &file) else {
            self.show_snackbar(tr!("attachment-open-message"), None, cx);
            return;
        };
        let items: Vec<Item> = view
            .attachments
            .iter()
            .enumerate()
            .map(|(ix, a)| Item::new(ix, a))
            .collect();
        self.show_viewer(raw, encrypted, items, index, None, window, cx);
        let on = self.picker_ticked(key);
        if let Some(picker) = &mut self.picker {
            picker.previewing = true;
        }
        if let Some(viewer) = &self.files.viewer {
            viewer.update(cx, |viewer, cx| {
                viewer.library = Some((place, count));
                viewer.pick = Some(on);
                cx.notify();
            });
        }
    }

    /// The viewer's arrows over the picker: the file `by` places on.
    pub(super) fn step_picker(&mut self, by: isize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(viewer) = self.files.viewer.clone() else {
            return;
        };
        let (Some((place, _)), Some(picker)) = (viewer.read(cx).library, &self.picker) else {
            return;
        };
        let count = picker.shown.len() as isize;
        if count == 0 {
            return;
        }
        let place = (place as isize + by).rem_euclid(count) as usize;
        self.preview_pick(place, window, cx);
    }

    /// Attaches the ticked files and closes the picker. Mail not
    /// downloaded yet is downloaded first.
    fn attach_picks(&mut self, cx: &mut Context<Self>) {
        let Some(picker) = &self.picker else {
            return;
        };
        let Some(files) = self.library.found().cloned() else {
            return;
        };
        let picks: Vec<_> = picker
            .ticked
            .iter()
            .filter_map(|key| files.iter().find(|f| f.key() == *key))
            .map(Found::row_file)
            .collect();
        self.close_files_picker(cx);
        let mut missing: Vec<MessageId> = picks
            .iter()
            .map(|f| f.message)
            .filter(|&id| self.attachment_raw(id).is_none())
            .collect();
        missing.sort_unstable();
        missing.dedup();
        let waits: Vec<_> = missing.iter().map(|&id| self.download(id, cx)).collect();
        if !waits.is_empty() {
            self.show_snackbar(tr!("files-downloading"), None, cx);
        }
        cx.spawn(async move |this, cx| {
            for wait in waits {
                wait.recv().await.ok();
            }
            let Ok(raws) = this.update(cx, |this, _| {
                picks
                    .iter()
                    .map(|f| (f.clone(), this.attachment_raw(f.message)))
                    .collect::<Vec<_>>()
            }) else {
                return;
            };
            let (got, failed) = cx
                .background_executor()
                .spawn(async move {
                    let mut got = Vec::new();
                    let mut failed = 0;
                    for (file, raw) in raws {
                        let read = raw.and_then(|(raw, encrypted)| {
                            let view = katna_render::message_view(&raw);
                            let index = row_file_index(&view.attachments, &file)?;
                            Some((katna_render::attachment_file(&raw, index)?, encrypted))
                        });
                        match read {
                            Some(read) => got.push(read),
                            None => failed += 1,
                        }
                    }
                    (got, failed)
                })
                .await;
            this.update(cx, |this, cx| this.attach_picked(got, failed, cx))
                .ok();
        })
        .detach();
    }

    /// How the ticked files weigh, with what the mail carries already.
    fn picker_weight(&self, cx: &gpui::App) -> Weight {
        let (used, provider) = self.compose_room(cx);
        let sizes: Vec<u64> = match (&self.picker, self.library.found()) {
            (Some(picker), Some(files)) => picker
                .ticked
                .iter()
                .filter_map(|key| files.iter().find(|f| f.key() == *key))
                .map(|f| f.file.size)
                .collect(),
            _ => Vec::new(),
        };
        let total = used + sizes.iter().sum::<u64>();
        let cloud = if provider.is_some() {
            cloud_bound(&sizes, used, MAX_TOTAL as u64)
                .into_iter()
                .zip(&sizes)
                .filter(|(bound, _)| *bound)
                .map(|(_, size)| size)
                .sum()
        } else {
            0
        };
        Weight {
            count: sizes.len(),
            total,
            cloud,
            provider,
        }
    }

    // --- Drawing -------------------------------------------------------------

    /// The picker, over the chat of conversation `key`'s feed.
    pub(in crate::window) fn render_files_picker(
        &mut self,
        key: EntryKey,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        // Another conversation opened: the picker was this one's.
        if self.picker.as_ref()?.key != key {
            self.close_files_picker(cx);
            return None;
        }
        self.library.frame += 1;
        let width = unpx(self.reader_scroll.bounds().size.width).max(320.0);
        let room = width - 2.0 * INSET - 2.0 * PAD;
        let columns = (((room + GAP) / (CARD_MIN + GAP)).floor() as usize).max(1);
        let card_width = (room - GAP * (columns - 1) as f32) / columns as f32;
        self.rebuild_picker(columns);
        let picker = self.picker.as_ref()?;
        let input = picker.input.clone();
        let focus = input.focus_handle(cx);
        let search = div()
            .id("picker-search")
            .flex_1()
            .min_w_0()
            .h(px(36.0))
            .px(px(12.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .rounded_full()
            .bg(rgba(th.search))
            .cursor_text()
            .on_click(move |_, window, cx| window.focus(&focus, cx))
            .child(icon("search", th.text_dim, 18.0))
            .child(div().flex_1().min_w_0().text_size(px(14.0)).child(input));
        let head = div()
            .flex_none()
            .h(px(56.0))
            .px(px(PAD))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .child(
                div()
                    .flex_none()
                    .text_size(px(16.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgba(th.text))
                    .child(tr!("picker-title")),
            )
            .child(search)
            .child(
                icon_button("picker-close", "close", 20.0, th)
                    .tooltip(tip(tr!("picker-cancel"), th))
                    .on_click(cx.listener(|this, _, _, cx| this.close_files_picker(cx))),
            );
        let chips = div()
            .id("picker-chips")
            .flex_none()
            .px(px(PAD))
            .pb(px(10.0))
            .flex()
            .flex_row()
            .flex_wrap()
            .gap(px(8.0))
            .children(
                KINDS
                    .into_iter()
                    .enumerate()
                    .map(|(n, types)| self.files_kind_chip(n, types, th, cx)),
            )
            .child(self.files_person_chip(th, cx))
            .child(self.files_time_chip(th, cx));
        let content = match &self.library.files {
            None => placeholder(&tr!("files-loading"), th),
            Some(Err(err)) => placeholder(err, th),
            Some(Ok(_)) if picker.shown.is_empty() => placeholder(&tr!("files-none-match"), th),
            Some(Ok(_)) => list(
                picker.state.clone(),
                cx.processor(move |this, ix: usize, window, cx| {
                    let th = this.theme(window);
                    let line = this.render_picker_line(ix, card_width, &th, cx);
                    this.request_library_thumbs(cx);
                    line
                }),
            )
            .size_full()
            .into_any_element(),
        };
        let foot = self.render_picker_foot(th, cx);
        let menu = self.render_files_menu(th, cx);
        let panel = raised(
            div()
                .id("files-picker")
                .absolute()
                .top(px(INSET))
                .left(px(INSET))
                .right(px(INSET))
                .bottom(px(INSET))
                .occlude()
                .overflow_hidden()
                .flex()
                .flex_col()
                .bg(rgba(th.menu)),
            th,
            16.0,
            3.0,
        )
        .child(head)
        .child(chips)
        .child(div().flex_1().min_h_0().child(content))
        .child(foot)
        .children(menu)
        .with_animation(
            "files-picker-in",
            Animation::new(std::time::Duration::from_millis(if cx.reduce_motion() {
                1
            } else {
                220
            }))
            .with_easing(ease_out_quint()),
            |el, t| el.opacity(t).mt(px(8.0 * (1.0 - t))),
        );
        Some(panel.into_any_element())
    }

    fn render_picker_line(
        &mut self,
        ix: usize,
        card_width: f32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(line) = self.picker.as_ref().and_then(|p| p.lines.get(ix).cloned()) else {
            return div().into_any_element();
        };
        match line {
            Line::Heading(in_chat) => div()
                .h(px(HEADING))
                .px(px(PAD))
                .pt(px(8.0))
                .flex()
                .items_center()
                .text_size(px(11.5))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgba(th.text_faint))
                .child(if in_chat {
                    tr!("picker-in-chat")
                } else {
                    tr!("picker-recent")
                })
                .into_any_element(),
            Line::Cards(range) => {
                let cards: Vec<AnyElement> = range
                    .map(|place| self.render_picker_card(place, card_width, th, cx))
                    .collect();
                div()
                    .px(px(PAD))
                    .pb(px(GAP))
                    .flex()
                    .flex_row()
                    .gap(px(GAP))
                    .children(cards)
                    .into_any_element()
            }
        }
    }

    fn render_picker_card(
        &mut self,
        place: usize,
        width: f32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let (Some(picker), Some(files)) = (&self.picker, self.library.found().cloned()) else {
            return div().into_any_element();
        };
        let Some(found) = picker.shown.get(place).and_then(|&ix| files.get(ix)) else {
            return div().into_any_element();
        };
        let key = found.key();
        let ticked = picker.ticked.contains(&key);
        let thumb = self.library_thumb(found);
        let date = found
            .file
            .date
            .and_then(|d| format::local(d, &self.tz))
            .zip(format::local(jiff::Timestamp::now().as_second(), &self.tz))
            .map(|(d, now)| format::list_date(d, now))
            .unwrap_or_default();
        let group = SharedString::from(format!("picker-card-{place}"));
        let tick = div()
            .absolute()
            .top(px(8.0))
            .left(px(8.0))
            .size(px(22.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .map(|d| {
                if ticked {
                    d.bg(rgba(th.accent))
                        .child(icon("check", th.on_accent, 16.0))
                } else {
                    // Light on any preview, dark or light theme alike.
                    d.bg(rgba(0xffff_ffd9))
                        .border_2()
                        .border_color(rgba(0x0000_0059))
                }
            });
        let eye = div()
            .id(("picker-eye", place))
            .absolute()
            .top(px(6.0))
            .right(px(6.0))
            .size(px(28.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .bg(rgba(fade(th.surface, 0.9)))
            .opacity(0.0)
            .group_hover(group.clone(), |s| s.opacity(1.0))
            .hover(|s| s.bg(rgba(th.surface)))
            .tooltip(tip(tr!("picker-preview"), th))
            .on_click(cx.listener(move |this, _, window, cx| {
                cx.stop_propagation();
                this.preview_pick(place, window, cx);
            }))
            .child(icon("eye", th.text_dim, 18.0));
        div()
            .id(("picker-card", place))
            .group(group)
            .relative()
            .flex_none()
            .w(px(width))
            .h(px(THUMB + FOOT))
            .flex()
            .flex_col()
            .overflow_hidden()
            .rounded(px(CARD_RADIUS))
            .border_1()
            .border_color(rgba(th.divider))
            .bg(rgba(th.surface))
            .cursor_pointer()
            .hover(|s| s.shadow(crate::widgets::elevation(th, 1.0)))
            .on_click(cx.listener(move |this, _, _, cx| this.toggle_pick(key, cx)))
            .child(
                div()
                    .flex_none()
                    .h(px(THUMB))
                    .w_full()
                    .child(card_top(thumb, found.kind, 36.0, th)),
            )
            .child(
                div()
                    .h(px(FOOT))
                    .px(px(10.0))
                    .flex()
                    .flex_col()
                    .justify_center()
                    .border_t_1()
                    .border_color(rgba(th.divider))
                    .child(
                        div()
                            .truncate()
                            .text_size(px(12.5))
                            .text_color(rgba(th.text))
                            .child(found.file.name.clone()),
                    )
                    .child(
                        div()
                            .truncate()
                            .text_size(px(11.0))
                            .text_color(rgba(th.text_faint))
                            .child(format!("{} · {date}", found.sender())),
                    ),
            )
            .child(tick)
            .child(eye)
            // A ticked card is ringed in the accent, over its edge.
            .when(ticked, |d| {
                d.child(
                    div()
                        .absolute()
                        .inset_0()
                        .rounded(px(CARD_RADIUS))
                        .border_2()
                        .border_color(rgba(th.accent)),
                )
            })
            .into_any_element()
    }

    /// The foot: how many are ticked and how they weigh, Cancel and Attach.
    fn render_picker_foot(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let weight = self.picker_weight(cx);
        let limit = MAX_TOTAL as u64;
        let mail = weight.total - weight.cloud;
        let over = weight.over();
        let fill = |part: u64, of: u64| (part as f32 / of.max(1) as f32).clamp(0.0, 1.0);
        let (mail_color, scale) = if weight.cloud > 0 {
            (th.accent, weight.total)
        } else if over {
            (th.error, weight.total)
        } else if fill(weight.total, limit) >= NEARLY {
            (th.caution(), limit)
        } else {
            (th.accent, limit)
        };
        let meter = div()
            .relative()
            .w(px(160.0))
            .h(px(4.0))
            .rounded_full()
            .bg(rgba(th.divider))
            .flex()
            .flex_row()
            .overflow_hidden()
            .child(
                div()
                    .h_full()
                    .w(gpui::relative(fill(mail, scale)))
                    .bg(rgba(mail_color)),
            )
            .when(weight.cloud > 0, |d| {
                d.child(
                    div()
                        .h_full()
                        .w(gpui::relative(fill(weight.cloud, scale)))
                        .bg(rgba(th.cloud())),
                )
            });
        let size = format::size(weight.total);
        let line = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(6.0))
            .text_size(px(12.5))
            .text_color(rgba(th.text_dim))
            .child(tr!("picker-selected", count = weight.count))
            .child("·")
            .map(|d| {
                if over {
                    d.text_color(rgba(th.error)).child(tr!(
                        "picker-over",
                        size = size,
                        limit = limit_text()
                    ))
                } else if weight.cloud > 0 {
                    let via = format::size(weight.cloud);
                    d.child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgba(th.text))
                            .child(size),
                    )
                    .child("·")
                    .child(icon("cloud", th.cloud(), 16.0))
                    .child(div().text_color(rgba(th.cloud())).child(
                        if weight.provider == Some(OAuthProvider::Microsoft) {
                            tr!("picker-via-onedrive", size = via)
                        } else {
                            tr!("picker-via-drive", size = via)
                        },
                    ))
                } else {
                    d.child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgba(th.text))
                            .child(size),
                    )
                    .child(tr!("picker-of-limit", limit = limit_text()))
                }
            });
        let ready = weight.count > 0 && !over;
        let attach = filled_button(
            "picker-attach",
            if weight.count > 0 {
                tr!("picker-attach-count", count = weight.count)
            } else {
                tr!("picker-attach")
            },
            th,
        )
        .when(!ready, |d| d.opacity(0.45).cursor_default())
        .when(ready, |d| {
            d.on_click(cx.listener(|this, _, _, cx| this.attach_picks(cx)))
        });
        let cancel = div()
            .id("picker-cancel")
            .h(px(36.0))
            .px(px(16.0))
            .flex()
            .items_center()
            .rounded_full()
            .cursor_pointer()
            .text_size(px(14.0))
            .font_weight(FontWeight::MEDIUM)
            .text_color(rgba(th.text_dim))
            .hover(|s| s.bg(rgba(th.hover)))
            .on_click(cx.listener(|this, _, _, cx| this.close_files_picker(cx)))
            .child(tr!("picker-cancel"));
        div()
            .flex_none()
            .h(px(60.0))
            .px(px(PAD))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .border_t_1()
            .border_color(rgba(th.divider))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .child(line)
                    .child(meter),
            )
            .child(cancel)
            .child(attach)
            .into_any_element()
    }
}
