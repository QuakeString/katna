// SPDX-License-Identifier: GPL-3.0-or-later

//! The attach picker: Compose's paperclip, and the chat reply box's
//! paperclip > From Files. It is the Files page in a panel, over the chat
//! or over the window for Compose, with the page's files, search and
//! chips, the files of this conversation first. Its side column (pills
//! on a narrow panel) adds the drives of the accounts, and This
//! computer… for the system's file chooser. A click ticks a file; the eye
//! (or the viewer's Select) looks before picking. Attach adds the ticked
//! files, downloading their mail first if need be. The foot weighs them
//! against the 25 MB a mail carries: on an account with Google Drive or
//! OneDrive the rest goes there, biggest first, as in Compose; on others
//! Attach waits until they fit. Drive files go as a copy while they fit,
//! and as a link from their own drive when they do not or are the
//! drive's own documents (Smart attach), shared with the recipients at
//! Send.
//!
//! The picker borrows the page's filters while it is open and gives them
//! back when it closes, so the page's chips, menus and calendar serve it.

use std::collections::HashSet;
use std::ops::Range;
use std::rc::Rc;

use gpui::{
    Animation, AnimationExt, AnyElement, Context, Entity, Focusable, FontWeight, ListAlignment,
    ListState, SharedString, Subscription, Window, anchored, div, ease_out_quint, list, point,
    prelude::*, rgba,
};
use katna_core::{AccountId, OAuthProvider};
use katna_dbus::CloudEntry;
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
use crate::widgets::{filled_button, icon, icon_button, raised, tip};

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
/// The side column of sources, on a panel at least `SIDE_FROM` wide.
const SIDE: f32 = 210.0;
const SIDE_FROM: f32 = 640.0;
/// The picker over Compose: at most this big, this far inside the window.
const MAX_WIDTH: f32 = 960.0;
const MAX_HEIGHT: f32 = 680.0;
const MARGIN: f32 = 24.0;

/// Where the picker's files come from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Source {
    /// The files of all mail, this conversation's first.
    Mail,
    /// Only this conversation's.
    Chat,
    /// The drive of an account.
    Drive(AccountId),
}

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
    /// The drive the page showed.
    drive: Option<super::drive::DriveView>,
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
    /// The conversation whose reply box it attaches to; `None` over the
    /// window, for Compose.
    key: Option<EntryKey>,
    source: Source,
    input: Entity<TextInput>,
    _input: Subscription,
    saved: Saved,
    /// The conversation's mail, whose files come first.
    in_chat: HashSet<MessageId>,
    /// The ticked files, in the order they were ticked.
    ticked: Vec<(MessageId, usize)>,
    /// The ticked drive files, with their accounts.
    drive_ticked: Vec<(AccountId, CloudEntry)>,
    /// The files shown, in order: places in the page's files.
    shown: Rc<Vec<usize>>,
    lines: Rc<Vec<Line>>,
    columns: usize,
    stale: bool,
    state: ListState,
    /// Wide enough for the side column; a drive's bar then takes one row.
    pub(super) wide: bool,
    /// The viewer shows one of its files: the arrows step through them.
    pub(in crate::window) previewing: bool,
}

impl Picker {
    /// The search box, which searches a drive too.
    pub(super) fn search_box(&self) -> Entity<TextInput> {
        self.input.clone()
    }
}

/// How the ticked files weigh against what a mail carries.
struct Weight {
    count: usize,
    /// Everything the mail would carry, ticked files included.
    total: u64,
    /// The part of it going through the cloud: uploaded or linked.
    cloud: u64,
    /// How many files go as links.
    links: usize,
    /// The links are OneDrive's.
    onedrive: bool,
    /// More than a mail carries, with no cloud to take the rest.
    over: bool,
    /// For each ticked drive file, whether it goes as a link.
    drive_links: Vec<bool>,
}

/// Which ticked files go through the cloud, with `used` bytes in the
/// mail already: of `mail` files (sizes) and drive files (sizes, and
/// whether each must be a link), with or without a `cloud` for mail
/// files. Returns which mail files are uploaded, which drive files are
/// linked, and whether the rest is still too much.
fn smart_attach(
    mail: &[u64],
    drive: &[(u64, bool)],
    used: u64,
    cloud: bool,
) -> (Vec<bool>, Vec<bool>, bool) {
    let limit = MAX_TOTAL as u64;
    let mut sizes = mail.to_vec();
    sizes.extend(
        drive
            .iter()
            .map(|&(size, link)| if link { 0 } else { size }),
    );
    let bound = cloud_bound(&sizes, used, limit);
    let (mail_bound, drive_bound) = bound.split_at(mail.len());
    let mut linked: Vec<bool> = drive
        .iter()
        .zip(drive_bound)
        .map(|(&(_, link), &bound)| link || bound)
        .collect();
    let mut uploaded = mail_bound.to_vec();
    if !cloud && uploaded.iter().any(|&b| b) {
        // Mail files cannot go up: drive files go as links instead, and
        // the mail files must fit by themselves.
        uploaded = vec![false; mail.len()];
        linked = vec![true; drive.len()];
    }
    let carried: u64 = used
        + mail
            .iter()
            .zip(&uploaded)
            .filter(|(_, up)| !**up)
            .map(|(size, _)| size)
            .sum::<u64>()
        + drive
            .iter()
            .zip(&linked)
            .filter(|(_, link)| !**link)
            .map(|((size, _), _)| size)
            .sum::<u64>();
    (uploaded, linked, carried > limit)
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
        let (key, in_chat) = (reader.key, reader.message_ids());
        self.open_picker(Some(key), in_chat, window, cx);
    }

    /// Opens the picker over the window, for Compose's paperclip.
    pub(in crate::window) fn open_compose_picker(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.compose_takes_files() {
            return;
        }
        self.open_picker(None, HashSet::new(), window, cx);
    }

    fn open_picker(
        &mut self,
        key: Option<EntryKey>,
        in_chat: HashSet<MessageId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
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
            drive: library.cloud.view.take(),
        };
        library.changed();
        let accent = rgba(self.theme(window).accent).into();
        let input = cx.new(|cx| {
            let mut input = TextInput::new(tr!("picker-search"), cx);
            input.set_accent(accent);
            input
        });
        let subscription =
            cx.subscribe_in(&input, window, |this, input, event: &InputEvent, _, cx| {
                let text = input.read(cx).text().to_owned();
                let drive = this.library.cloud.view.is_some();
                match event {
                    // A drive is searched on its side once typing rests.
                    InputEvent::Changed | InputEvent::Submit if drive => {
                        this.drive_search(text, *event == InputEvent::Submit, cx);
                    }
                    InputEvent::Changed => {
                        this.library.query = text;
                        this.library.changed();
                    }
                    InputEvent::Cancel => this.close_files_picker(cx),
                    InputEvent::Submit => {}
                }
                cx.notify();
            });
        window.focus(&input.focus_handle(cx), cx);
        self.picker = Some(Picker {
            key,
            source: Source::Mail,
            input,
            _input: subscription,
            saved,
            in_chat,
            ticked: Vec::new(),
            drive_ticked: Vec::new(),
            shown: Rc::default(),
            lines: Rc::default(),
            columns: 0,
            stale: true,
            state: ListState::new(0, ListAlignment::Top, px(600.0)),
            wide: true,
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
        library.cloud.view = saved.drive;
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
        let (mut first, mut rest): (Vec<usize>, Vec<usize>) = shown
            .iter()
            .partition(|&&ix| picker.in_chat.contains(&files[ix].file.message));
        if picker.source == Source::Chat {
            rest.clear();
        }
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

    /// Shows the files of `source`, with the search box emptied for it.
    fn pick_source(&mut self, source: Source, cx: &mut Context<Self>) {
        let Some(picker) = &mut self.picker else {
            return;
        };
        if picker.source == source {
            return;
        }
        picker.source = source;
        picker.stale = true;
        let input = picker.input.clone();
        self.library.menu = None;
        self.library.cloud.view = None;
        self.library.query.clear();
        self.library.changed();
        let placeholder = match source {
            Source::Drive(_) => tr!("picker-search-drive"),
            Source::Mail | Source::Chat => tr!("picker-search"),
        };
        input.update(cx, |input, cx| {
            input.set_placeholder(placeholder);
            input.set_text("", cx);
        });
        if let Source::Drive(account) = source {
            self.open_picker_drive(account, cx);
        }
        cx.notify();
    }

    /// This computer…: the system's file chooser, in place of the picker.
    fn pick_from_computer(&mut self, cx: &mut Context<Self>) {
        self.close_files_picker(cx);
        self.pick_files(false, cx);
    }

    /// Ticks or unticks drive file `entry` of `account`.
    pub(super) fn toggle_drive_pick(
        &mut self,
        account: AccountId,
        entry: &CloudEntry,
        cx: &mut Context<Self>,
    ) {
        let Some(picker) = &mut self.picker else {
            return;
        };
        match picker
            .drive_ticked
            .iter()
            .position(|(a, e)| *a == account && e.id == entry.id)
        {
            Some(at) => {
                picker.drive_ticked.remove(at);
            }
            None => picker.drive_ticked.push((account, entry.clone())),
        }
        cx.notify();
    }

    /// Whether drive file `id` of `account` is ticked.
    pub(super) fn drive_picked(&self, account: AccountId, id: &str) -> bool {
        self.picker.as_ref().is_some_and(|p| {
            p.drive_ticked
                .iter()
                .any(|(a, e)| *a == account && e.id == id)
        })
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
        let drive = picker.drive_ticked.clone();
        let links = self.picker_weight(cx).drive_links;
        self.close_files_picker(cx);
        let (links, copies): (Vec<_>, Vec<_>) =
            drive.into_iter().zip(links).partition(|(_, link)| *link);
        for ((account, entry), _) in links {
            self.link_drive_file(account, entry, cx);
        }
        let copies: Vec<_> = copies.into_iter().map(|(file, _)| file).collect();
        self.attach_drive_copies(copies, cx);
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

    /// Fetches drive files `files` and attaches them as copies.
    fn attach_drive_copies(&mut self, files: Vec<(AccountId, CloudEntry)>, cx: &mut Context<Self>) {
        if files.is_empty() {
            return;
        }
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        let getting = SharedString::from(tr!("picker-getting", count = files.len()));
        self.show_snackbar(getting.clone(), None, cx);
        cx.spawn(async move |this, cx| {
            let mut paths = Vec::new();
            let mut failed = 0;
            for (account, entry) in files {
                match crate::daemon::cloud_fetch(&connection, account.0, &entry).await {
                    Ok(path) => paths.push(std::path::PathBuf::from(path)),
                    Err(err) => {
                        tracing::warn!(%err, "cannot fetch a picked drive file");
                        failed += 1;
                    }
                }
            }
            this.update(cx, |this, cx| {
                // The note that they were coming goes once they are here.
                if let Some(snackbar) = &mut this.snackbar
                    && snackbar.text == getting
                {
                    snackbar.shown.set(0.0);
                }
                this.attach_paths(paths, cx);
                if failed > 0 {
                    this.show_snackbar(tr!("picker-some-failed", count = failed), None, cx);
                }
            })
            .ok();
        })
        .detach();
    }

    /// How the ticked files weigh, with what the mail carries already.
    fn picker_weight(&self, cx: &gpui::App) -> Weight {
        let (used, provider) = self.compose_room(cx);
        let mail: Vec<u64> = match (&self.picker, self.library.found()) {
            (Some(picker), Some(files)) => picker
                .ticked
                .iter()
                .filter_map(|key| files.iter().find(|f| f.key() == *key))
                .map(|f| f.file.size)
                .collect(),
            _ => Vec::new(),
        };
        let drive: Vec<(u64, bool)> = self
            .picker
            .iter()
            .flat_map(|p| &p.drive_ticked)
            .map(|(_, e)| (e.size, e.native || e.size > MAX_TOTAL as u64))
            .collect();
        let (uploaded, linked, over) = smart_attach(&mail, &drive, used, provider.is_some());
        let up: u64 = mail
            .iter()
            .zip(&uploaded)
            .filter(|(_, up)| **up)
            .map(|(size, _)| size)
            .sum();
        let link: u64 = drive
            .iter()
            .zip(&linked)
            .filter(|(_, link)| **link)
            .map(|((size, _), _)| size)
            .sum();
        let links = uploaded.iter().chain(&linked).filter(|b| **b).count();
        // Links come from each file's own drive; uploads go to the
        // account's.
        let linked_onedrive = self
            .picker
            .iter()
            .flat_map(|p| &p.drive_ticked)
            .zip(&linked)
            .any(|((account, _), link)| *link && self.drive_is_onedrive(*account));
        let onedrive = linked_onedrive
            || (provider == Some(OAuthProvider::Microsoft)
                && linked.iter().all(|link| !link)
                && uploaded.iter().any(|up| *up));
        Weight {
            count: mail.len() + drive.len(),
            total: used + mail.iter().sum::<u64>() + drive.iter().map(|(s, _)| s).sum::<u64>(),
            cloud: up + link,
            links,
            onedrive,
            over,
            drive_links: linked,
        }
    }

    // --- Drawing -------------------------------------------------------------

    /// The picker: over the open conversation's chat, or for Compose over
    /// the window, closing on a click beside it.
    pub(in crate::window) fn render_files_picker(
        &mut self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let host = self.picker.as_ref()?.key;
        let phone = self.layout.shape.is_phone();
        let viewport = window.viewport_size();
        let (view_width, view_height) = (unpx(viewport.width), unpx(viewport.height));
        let (left, top, width, height) = match host {
            Some(key) => {
                // Another conversation opened: the picker was this one's.
                if self.reader.as_ref().map(|r| r.key) != Some(key) || !self.chat_shown() {
                    self.close_files_picker(cx);
                    return None;
                }
                let bounds = self.reader_scroll.bounds();
                let inset = if phone { 0.0 } else { INSET };
                (
                    unpx(bounds.origin.x) + inset,
                    unpx(bounds.origin.y) + inset,
                    (unpx(bounds.size.width) - 2.0 * inset).max(320.0),
                    (unpx(bounds.size.height) - 2.0 * inset).max(320.0),
                )
            }
            None => {
                if !self.compose_writing() {
                    self.close_files_picker(cx);
                    return None;
                }
                if phone {
                    (0.0, 0.0, view_width, view_height)
                } else {
                    let width = (view_width - 2.0 * MARGIN).min(MAX_WIDTH);
                    let height = (view_height - 2.0 * MARGIN).min(MAX_HEIGHT);
                    (
                        (view_width - width) / 2.0,
                        (view_height - height) / 2.0,
                        width,
                        height,
                    )
                }
            }
        };
        let panel = self.render_picker_panel(width, phone, th, window, cx)?;
        let radius = if phone { 0.0 } else { 16.0 };
        let panel = raised(
            div()
                .id("files-picker")
                .w(px(width))
                .h(px(height))
                .occlude()
                .overflow_hidden()
                .bg(rgba(th.menu)),
            th,
            radius,
            3.0,
        )
        .child(panel)
        .with_animation(
            "files-picker-in",
            Animation::new(katna_ui::motion::time(std::time::Duration::from_millis(
                if cx.reduce_motion() { 1 } else { 220 },
            )))
            .with_easing(ease_out_quint()),
            |el, t| el.opacity(t).mt(px(8.0 * (1.0 - t))),
        );
        if host.is_some() {
            return Some(
                anchored()
                    .position(point(px(left), px(top)))
                    .child(panel)
                    .into_any_element(),
            );
        }
        // Over Compose a click beside the picker closes it.
        Some(
            anchored()
                .position(point(px(0.0), px(0.0)))
                .child(
                    div()
                        .relative()
                        .w(px(view_width))
                        .h(px(view_height))
                        .child(
                            div()
                                .id("files-picker-beside")
                                .absolute()
                                .inset_0()
                                .occlude()
                                .on_click(
                                    cx.listener(|this, _, _, cx| this.close_files_picker(cx)),
                                ),
                        )
                        .child(div().absolute().left(px(left)).top(px(top)).child(panel)),
                )
                .into_any_element(),
        )
    }

    /// The picker's insides, `width` wide.
    fn render_picker_panel(
        &mut self,
        width: f32,
        phone: bool,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<gpui::Div> {
        self.library.frame += 1;
        let side = width >= SIDE_FROM;
        let room = width - 2.0 * PAD - if side { SIDE } else { 0.0 };
        let columns = (((room + GAP) / (CARD_MIN + GAP)).floor() as usize).max(1);
        let card_width = (room - GAP * (columns - 1) as f32) / columns as f32;
        let picker = self.picker.as_mut()?;
        picker.wide = side;
        let source = picker.source;
        if !matches!(source, Source::Drive(_)) {
            self.rebuild_picker(columns);
        }
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
        // A phone's picker fills the chat edge to edge, its search on a row
        // of its own.
        let (search, search_row) = if phone {
            (
                None,
                Some(
                    div()
                        .flex_none()
                        .px(px(PAD))
                        .pb(px(10.0))
                        .flex()
                        .child(search),
                ),
            )
        } else {
            (Some(search), None)
        };
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
                    .when(phone, |d| d.flex_1())
                    .text_size(px(16.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgba(th.text))
                    .child(tr!("picker-title")),
            )
            .children(search)
            .child(
                icon_button("picker-close", "close", 20.0, th)
                    .tooltip(tip(tr!("picker-cancel"), th))
                    .on_click(cx.listener(|this, _, _, cx| this.close_files_picker(cx))),
            );
        let main = if matches!(source, Source::Drive(_)) {
            self.render_drive_body(card_width, columns, PAD, th, window, cx)
        } else {
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
                        .map(|(n, types)| self.files_kind_chip(n, types, false, th, cx)),
                )
                .child(self.files_person_chip(th, cx))
                .child(self.files_time_chip(th, cx));
            let picker = self.picker.as_ref()?;
            let content = match &self.library.files {
                None => self.placeholder(tr!("files-loading"), th),
                Some(Err(err)) => self.placeholder(err.clone(), th),
                Some(Ok(_)) if picker.shown.is_empty() => {
                    self.placeholder(tr!("files-none-match"), th)
                }
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
            div()
                .size_full()
                .flex()
                .flex_col()
                .child(chips)
                .child(div().flex_1().min_h_0().child(content))
                .into_any_element()
        };
        let (side_column, pills) = if side {
            (Some(self.render_picker_sources(th, cx)), None)
        } else {
            (None, Some(self.render_picker_pills(th, cx)))
        };
        let foot = self.render_picker_foot(th, cx);
        let menu = self.render_files_menu(th, cx);
        Some(
            div()
                .size_full()
                .flex()
                .flex_col()
                .child(head)
                .children(search_row)
                .children(pills)
                .child(
                    div()
                        .flex_1()
                        .min_h_0()
                        .flex()
                        .flex_row()
                        .children(side_column)
                        .child(div().flex_1().min_w_0().h_full().child(main)),
                )
                .child(foot)
                .children(menu),
        )
    }

    /// The sources the picker offers: the mail's files, this
    /// conversation's, each drive, and This computer….
    fn picker_sources(&self) -> Vec<(Source, String, Option<String>)> {
        let Some(picker) = &self.picker else {
            return Vec::new();
        };
        let mut sources = vec![(Source::Mail, tr!("picker-mail-files"), None)];
        if picker.key.is_some() {
            sources.push((Source::Chat, tr!("picker-this-chat"), None));
        }
        sources.extend(self.library.cloud.drives.iter().map(|(account, address)| {
            (
                Source::Drive(*account),
                self.drive_name(*account),
                Some(address.clone()),
            )
        }));
        sources
    }

    /// How many files a source has, for its count; none for a drive.
    fn source_count(&self, source: Source) -> Option<usize> {
        let picker = self.picker.as_ref()?;
        let files = self.library.files.as_ref()?.as_ref().ok()?;
        match source {
            Source::Mail => Some(files.len()),
            Source::Chat => Some(
                files
                    .iter()
                    .filter(|f| picker.in_chat.contains(&f.file.message))
                    .count(),
            ),
            Source::Drive(_) => None,
        }
    }

    /// The side column of sources, on a wide panel.
    fn render_picker_sources(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(picker) = &self.picker else {
            return div().into_any_element();
        };
        let current = picker.source;
        let mut column = div()
            .id("picker-sources")
            .flex_none()
            .w(px(SIDE))
            .h_full()
            .overflow_y_scroll()
            .pt(px(4.0))
            .pb(px(12.0))
            .flex()
            .flex_col()
            .gap(px(2.0))
            .border_r_1()
            .border_color(rgba(th.divider));
        let mut drives = false;
        for (n, (source, label, address)) in self.picker_sources().into_iter().enumerate() {
            let on = source == current;
            let color = if on {
                th.row_selected_text
            } else {
                th.text_dim
            };
            let mark = match source {
                Source::Mail => icon("attachment", color, 20.0),
                Source::Chat => icon("chat", color, 20.0),
                Source::Drive(account) => self.drive_mark_of(account, 20.0),
            };
            if matches!(source, Source::Drive(_)) && !std::mem::replace(&mut drives, true) {
                column = column.child(
                    div()
                        .flex_none()
                        .mt(px(10.0))
                        .mb(px(4.0))
                        .px(px(24.0))
                        .text_size(px(12.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgba(th.text_faint))
                        .child(tr!("files-drives")),
                );
            }
            let label = match address {
                Some(address) => div()
                    .flex()
                    .flex_col()
                    .child(div().truncate().child(label))
                    .child(
                        div()
                            .truncate()
                            .text_size(px(11.0))
                            .font_weight(FontWeight::NORMAL)
                            .text_color(rgba(th.text_faint))
                            .child(address),
                    ),
                None => div().truncate().child(label),
            };
            let count = self.source_count(source).filter(|&c| c > 0);
            column = column.child(
                super::super::nav::side_row_with(("picker-source", n), mark, label, on, th)
                    .when(matches!(source, Source::Drive(_)), |d| d.h(px(44.0)))
                    .when_some(count, |d, c| {
                        d.child(crate::widgets::count_pill(c as u64, on, th))
                    })
                    .on_click(cx.listener(move |this, _, _, cx| this.pick_source(source, cx))),
            );
        }
        column
            .child(
                div()
                    .flex_none()
                    .mt(px(10.0))
                    .mb(px(8.0))
                    .mx(px(24.0))
                    .h(px(1.0))
                    .bg(rgba(th.divider)),
            )
            .child(
                super::super::nav::side_row_with(
                    "picker-computer",
                    icon("home", th.text_dim, 20.0),
                    tr!("picker-this-computer"),
                    false,
                    th,
                )
                .on_click(cx.listener(|this, _, _, cx| this.pick_from_computer(cx))),
            )
            .into_any_element()
    }

    /// The sources as a row of chips that scrolls sideways, on a narrow
    /// panel.
    fn render_picker_pills(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let current = self.picker.as_ref().map(|p| p.source);
        let mut row = div()
            .id("picker-pills")
            .flex_none()
            .overflow_x_scroll()
            .px(px(PAD))
            .pb(px(10.0))
            .flex()
            .flex_row()
            .gap(px(8.0));
        for (n, (source, label, address)) in self.picker_sources().into_iter().enumerate() {
            // Two drives would read the same: the address tells them apart.
            let label = match address {
                Some(address) if self.library.cloud.drives.len() > 1 => address,
                _ => label,
            };
            row = row.child(
                super::filter_chip(("picker-pill", n), label, Some(source) == current, th)
                    .pr(px(12.0))
                    .on_click(cx.listener(move |this, _, _, cx| this.pick_source(source, cx))),
            );
        }
        row.child(
            super::filter_chip(
                "picker-pill-computer",
                tr!("picker-this-computer"),
                false,
                th,
            )
            .pr(px(12.0))
            .on_click(cx.listener(|this, _, _, cx| this.pick_from_computer(cx))),
        )
        .into_any_element()
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
        let tick = pick_tick(ticked, th);
        let eye = pick_eye(("picker-eye", place), group.clone(), th).on_click(cx.listener(
            move |this, _, window, cx| {
                cx.stop_propagation();
                this.preview_pick(place, window, cx);
            },
        ));
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
            .border_color(rgba(th.outline))
            .bg(rgba(th.surface))
            .cursor_pointer()
            .hover(|s| s.shadow(crate::widgets::elevation(th, 1.0)))
            .on_click(cx.listener(move |this, _, _, cx| this.toggle_pick(key, cx)))
            .child(
                div()
                    .flex_none()
                    .h(px(THUMB))
                    .w_full()
                    .overflow_hidden()
                    .child(card_top(thumb, found.kind, 36.0, None, th)),
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
            .when(ticked, |d| d.child(pick_ring(th)))
            .into_any_element()
    }

    /// The foot: how many are ticked and how they weigh, Cancel and Attach.
    fn render_picker_foot(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let weight = self.picker_weight(cx);
        let limit = MAX_TOTAL as u64;
        let mail = weight.total - weight.cloud;
        let over = weight.over;
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
        let phone = self.layout.shape.is_phone();
        let meter = div()
            .relative()
            .w(px(160.0))
            .when(phone, |d| d.w_full())
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
                } else if weight.links > 0 {
                    let links = weight.links as u64;
                    d.child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgba(th.text))
                            .child(tr!("picker-in-mail", size = format::size(mail))),
                    )
                    .child("·")
                    .child(icon("cloud", th.cloud(), 16.0))
                    .child(
                        div()
                            .min_w_0()
                            .truncate()
                            .text_color(rgba(th.cloud()))
                            .child(if weight.onedrive {
                                tr!("picker-onedrive-links", count = links)
                            } else {
                                tr!("picker-drive-links", count = links)
                            }),
                    )
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
        let weight = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .child(line)
            .child(meter);
        // A phone has no room for the size beside the buttons: it goes
        // above them.
        if phone {
            return div()
                .flex_none()
                .px(px(PAD))
                .pt(px(10.0))
                .pb(px(10.0))
                .flex()
                .flex_col()
                .gap(px(10.0))
                .border_t_1()
                .border_color(rgba(th.divider))
                .child(weight.flex_none())
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .justify_end()
                        .items_center()
                        .gap(px(12.0))
                        .child(cancel)
                        .child(attach),
                )
                .into_any_element();
        }
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
            .child(weight)
            .child(cancel)
            .child(attach)
            .into_any_element()
    }
}

/// The circle at the top left of a card in the picker, filled with a
/// check once ticked: mail files and drive files alike.
pub(super) fn pick_tick(ticked: bool, th: &Theme) -> gpui::Div {
    div()
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
        })
}

/// A ticked card is ringed in the accent, over its edge.
pub(super) fn pick_ring(th: &Theme) -> gpui::Div {
    div()
        .absolute()
        .inset_0()
        .rounded(px(CARD_RADIUS))
        .border_2()
        .border_color(rgba(th.accent))
}

/// The eye at the top right of a card in the picker, shown while the
/// pointer is over card `group`: looks before picking.
pub(super) fn pick_eye(
    id: impl Into<gpui::ElementId>,
    group: SharedString,
    th: &Theme,
) -> gpui::Stateful<gpui::Div> {
    div()
        .id(id)
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
        .group_hover(group, |s| s.opacity(1.0))
        .hover(|s| s.bg(rgba(th.surface)))
        .tooltip(tip(tr!("picker-preview"), th))
        .child(icon("eye", th.text_dim, 18.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    const MB: u64 = 1_000_000;

    #[test]
    fn drive_files_go_as_a_copy_while_they_fit() {
        let (up, linked, over) = smart_attach(&[2 * MB], &[(3 * MB, false)], 0, true);
        assert_eq!((up, linked, over), (vec![false], vec![false], false));
    }

    #[test]
    fn big_files_and_documents_go_as_links() {
        let drive = [(61 * MB, true), (0, true), (3 * MB, false)];
        let (_, linked, over) = smart_attach(&[], &drive, 0, false);
        assert_eq!(linked, vec![true, true, false]);
        assert!(!over);
    }

    #[test]
    fn without_a_cloud_drive_files_make_room_for_mail_files() {
        let (up, linked, over) = smart_attach(&[20 * MB], &[(20 * MB, false)], 0, false);
        assert_eq!((up, linked, over), (vec![false], vec![true], false));
        let (_, _, over) = smart_attach(&[30 * MB], &[], 0, false);
        assert!(over);
    }
}
