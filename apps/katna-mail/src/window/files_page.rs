// SPDX-License-Identifier: GPL-3.0-or-later

//! The Files page (HEY's Files, the attachment library): every file that
//! came or went with the mail of every account, as the cards under a
//! message, newest first and grouped by month. The side column narrows
//! it to a kind of file, an account, or mail received or sent; the top
//! bar's search box looks through names, senders and subjects; chips pick
//! a person, a time and the order. Every card links back to its mail.
//!
//! It reads the attachment lists sync keeps (`katna_store::LibraryFile`),
//! so it works offline and needs no server. A click opens a file as the
//! list's chips do: in the viewer or the app Default apps names, after
//! downloading its mail if needed. Thumbnails are made in the background
//! for mail already downloaded, and only for the cards on show.

use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::rc::Rc;
use std::sync::Arc;

use gpui::{
    Animation, AnimationExt, AnyElement, ClipboardItem, Context, FontWeight, ListAlignment,
    ListState, MouseButton, MouseDownEvent, Pixels, Point, SharedString, Task, Window, anchored,
    deferred, div, ease_out_quint, list, prelude::*, rgba,
};
use katna_core::AccountId;
use katna_i18n::tr;
use katna_preview::Kind;
use katna_store::{LibraryFile, MessageId};
use katna_ui::{ScrollBar, px};

use super::MailWindow;
use super::apps::App;
use super::attachments::{
    CARD_RADIUS, Thumb, card_top, has_thumbnail, hover_panel, kind_badge, panel_button,
    row_file_index, thumbnail,
};
use crate::data::RowFile;
use crate::format;
use crate::theme::Theme;
use crate::widgets::{icon, icon_button, placeholder, raised, tip};
use katna_ui::text_input::{InputEvent, TextInput};

const NAV_WIDTH: f32 = 256.0;
/// The most files the page reads.
const LIMIT: usize = 20_000;
/// Cards are at least this wide; the rest of a row is shared out.
const CARD_MIN: f32 = 196.0;
const CARD_MIN_PHONE: f32 = 150.0;
const GAP: f32 = 14.0;
/// The tallest a card's thumbnail stands; narrower cards (two a row on
/// a phone) have shorter ones, so thumbnails keep their shape.
const THUMB_HEIGHT: f32 = 112.0;
const NAME_HEIGHT: f32 = 40.0;
/// A card's height under its thumbnail.
const CARD_FOOT: f32 = NAME_HEIGHT + 22.0 + 28.0 + 2.0;
const HEADING_HEIGHT: f32 = 44.0;
const ROW_HEIGHT: f32 = 52.0;
const MENU_WIDTH: f32 = 290.0;
/// Pictures smaller than this are signature logos and the like, not files.
const SMALL_PICTURE: u64 = 12 * 1024;
/// Thumbnails kept in memory; those drawn longest ago go first.
const THUMBS_KEPT: usize = 96;
/// Thumbnails made in one background run.
const THUMB_BATCH: usize = 12;
/// Senders offered by the "Anyone" chip.
const PEOPLE: usize = 12;

/// The kinds of file the side column picks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Types {
    All,
    Pictures,
    Pdfs,
    Documents,
    Sheets,
    Slides,
    Other,
}

impl Types {
    const ALL: [Self; 7] = [
        Self::All,
        Self::Pictures,
        Self::Pdfs,
        Self::Documents,
        Self::Sheets,
        Self::Slides,
        Self::Other,
    ];

    fn of(kind: Kind) -> Self {
        match kind {
            Kind::Picture(_) => Self::Pictures,
            Kind::Pdf => Self::Pdfs,
            Kind::Document | Kind::Text => Self::Documents,
            Kind::Sheet { .. } => Self::Sheets,
            Kind::Slides => Self::Slides,
            Kind::Other => Self::Other,
        }
    }

    fn label(self) -> String {
        tr!(match self {
            Self::All => "files-all",
            Self::Pictures => "files-pictures",
            Self::Pdfs => "files-pdfs",
            Self::Documents => "files-documents",
            Self::Sheets => "files-sheets",
            Self::Slides => "files-slides",
            Self::Other => "files-other",
        })
    }

    fn icon(self) -> &'static str {
        match self {
            Self::All => "attachment",
            Self::Pictures => "image",
            Self::Pdfs => "file",
            Self::Documents => "document",
            Self::Sheets => "sheet",
            Self::Slides => "slides",
            Self::Other => "folder",
        }
    }
}

/// Mail received or sent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Any,
    Received,
    Sent,
}

/// How far back the files go.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Time {
    Any,
    Week,
    Month,
    Year,
    Older,
}

impl Time {
    const ALL: [Self; 5] = [Self::Any, Self::Week, Self::Month, Self::Year, Self::Older];

    fn label(self) -> String {
        tr!(match self {
            Self::Any => "files-time-any",
            Self::Week => "files-time-week",
            Self::Month => "files-time-month",
            Self::Year => "files-time-year",
            Self::Older => "files-time-older",
        })
    }

    /// Whether a file dated `date` is shown, `now` being the time now.
    fn keeps(self, date: Option<i64>, now: i64) -> bool {
        const DAY: i64 = 24 * 60 * 60;
        let age = date.map(|d| now - d);
        match self {
            Self::Any => true,
            Self::Week => age.is_some_and(|a| a <= 7 * DAY),
            Self::Month => age.is_some_and(|a| a <= 31 * DAY),
            Self::Year => age.is_some_and(|a| a <= 366 * DAY),
            Self::Older => age.is_none_or(|a| a > 366 * DAY),
        }
    }
}

/// The order of the files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sort {
    Newest,
    Oldest,
    Largest,
    Name,
}

impl Sort {
    const ALL: [Self; 4] = [Self::Newest, Self::Oldest, Self::Largest, Self::Name];

    fn label(self) -> String {
        tr!(match self {
            Self::Newest => "files-sort-newest",
            Self::Oldest => "files-sort-oldest",
            Self::Largest => "files-sort-largest",
            Self::Name => "files-sort-name",
        })
    }

    /// Grouped under month headings.
    fn by_date(self) -> bool {
        matches!(self, Self::Newest | Self::Oldest)
    }
}

/// One file as the page shows it.
struct Found {
    file: LibraryFile,
    kind: Kind,
    types: Types,
    /// Its name, subject and sender in lower case, for the search box.
    hay: String,
    /// The heading it goes under when sorted by date.
    group: Group,
}

impl Found {
    fn row_file(&self) -> RowFile {
        RowFile {
            message: self.file.message,
            name: self.file.name.clone(),
            mime: self.file.mime.clone(),
            size: self.file.size,
            nth: self.file.nth,
            order: self.file.order,
        }
    }

    /// The name the sender signs with, else their address.
    fn sender(&self) -> String {
        if self.file.mine {
            return tr!("files-me");
        }
        self.file
            .from_name
            .clone()
            .unwrap_or_else(|| self.file.from_email.clone())
    }

    fn key(&self) -> (MessageId, usize) {
        (self.file.message, self.file.order)
    }
}

/// The heading of a file: the past seven days, a month, or no date.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Group {
    Week,
    Month(i16, i8),
    Undated,
}

/// A line of the page.
#[derive(Debug, Clone)]
enum Line {
    Heading(Group),
    /// A row of cards: places in `shown`.
    Cards(Range<usize>),
    /// A file in the list view: its place in `shown`.
    Row(usize),
}

/// A sender the "Anyone" chip offers.
struct Sender {
    email: String,
    name: String,
    files: usize,
}

/// The open menu, and where it was opened.
enum Menu {
    File { ix: usize, at: Point<Pixels> },
    People(Point<Pixels>),
    Time(Point<Pixels>),
    Sort(Point<Pixels>),
}

/// What to do with a file once its mail is downloaded.
#[derive(Debug, Clone, Copy)]
enum Act {
    Save,
    OpenWith,
    Forward,
}

/// The Files page's state.
pub(super) struct Library {
    files: Option<Result<Rc<Vec<Found>>, String>>,
    _load: Option<Task<()>>,
    senders: Rc<Vec<Sender>>,
    /// Files per account, over all of them.
    per_account: HashMap<AccountId, usize>,
    /// The mail search's text while the page has the search box.
    pub(super) mail_query: Option<String>,
    query: String,
    types: Types,
    account: Option<AccountId>,
    direction: Direction,
    /// Files from this address only.
    person: Option<String>,
    time: Time,
    sort: Sort,
    grid: bool,
    menu: Option<Menu>,
    /// The files that pass the filters, in order: places in `files`.
    shown: Rc<Vec<usize>>,
    /// Their total size.
    shown_bytes: u64,
    /// Files of each kind that pass the other filters.
    counts: [usize; 7],
    lines: Rc<Vec<Line>>,
    /// The filters or the layout changed: `shown` and `lines` are made
    /// again before the next frame.
    stale: bool,
    columns: usize,
    state: ListState,
    bar: ScrollBar,
    thumbs: HashMap<(MessageId, usize), (Thumb, u64)>,
    asked: HashSet<(MessageId, usize)>,
    wanted: Vec<(RowFile, Kind)>,
    _thumbs: Option<Task<()>>,
    frame: u64,
}

impl Default for Library {
    fn default() -> Self {
        Self {
            files: None,
            _load: None,
            senders: Rc::default(),
            per_account: HashMap::new(),
            mail_query: None,
            query: String::new(),
            types: Types::All,
            account: None,
            direction: Direction::Any,
            person: None,
            time: Time::Any,
            sort: Sort::Newest,
            grid: true,
            menu: None,
            shown: Rc::default(),
            shown_bytes: 0,
            counts: [0; 7],
            lines: Rc::default(),
            stale: true,
            columns: 0,
            state: ListState::new(0, ListAlignment::Top, px(600.0)),
            bar: ScrollBar::default(),
            thumbs: HashMap::new(),
            asked: HashSet::new(),
            wanted: Vec::new(),
            _thumbs: None,
            frame: 0,
        }
    }
}

impl Library {
    fn found(&self) -> Option<&Rc<Vec<Found>>> {
        self.files.as_ref().and_then(|f| f.as_ref().ok())
    }

    /// Makes `shown`, the counts and the lines again for `columns` cards
    /// a row, each line about `line` high until it is drawn (so the
    /// scrollbar's thumb is about the right size from the start).
    fn rebuild(&mut self, columns: usize, line: f32, now: i64) {
        self.stale = false;
        self.columns = columns;
        let Some(files) = self.found().cloned() else {
            return;
        };
        let words: Vec<String> = self
            .query
            .split_whitespace()
            .map(str::to_lowercase)
            .collect();
        let mut counts = [0; 7];
        let mut shown = Vec::new();
        for (ix, found) in files.iter().enumerate() {
            let file = &found.file;
            let passes = self.account.is_none_or(|a| a == file.account)
                && match self.direction {
                    Direction::Any => true,
                    Direction::Received => !file.mine,
                    Direction::Sent => file.mine,
                }
                && self.person.as_ref().is_none_or(|p| *p == file.from_email)
                && self.time.keeps(file.date, now)
                && words.iter().all(|w| found.hay.contains(w.as_str()));
            if !passes {
                continue;
            }
            counts[0] += 1;
            counts[Types::ALL
                .iter()
                .position(|t| *t == found.types)
                .unwrap_or(6)] += 1;
            if self.types == Types::All || self.types == found.types {
                shown.push(ix);
            }
        }
        match self.sort {
            Sort::Newest => {}
            Sort::Oldest => shown.reverse(),
            Sort::Largest => shown.sort_by_key(|&ix| std::cmp::Reverse(files[ix].file.size)),
            Sort::Name => shown.sort_by_cached_key(|&ix| files[ix].file.name.to_lowercase()),
        }
        self.shown_bytes = shown.iter().map(|&ix| files[ix].file.size).sum();
        let mut lines = Vec::new();
        let mut start = 0;
        while start < shown.len() {
            let group = files[shown[start]].group;
            let mut end = start;
            if self.sort.by_date() {
                lines.push(Line::Heading(group));
                while end < shown.len() && files[shown[end]].group == group {
                    end += 1;
                }
            } else {
                end = shown.len();
            }
            if self.grid {
                let mut at = start;
                while at < end {
                    let to = (at + columns.max(1)).min(end);
                    lines.push(Line::Cards(at..to));
                    at = to;
                }
            } else {
                lines.extend((start..end).map(Line::Row));
            }
            start = end;
        }
        self.counts = counts;
        self.shown = Rc::new(shown);
        self.lines = Rc::new(lines);
        let line = if self.grid { line } else { ROW_HEIGHT };
        self.state
            .reset_with_uniform_height(self.lines.len(), px(line));
    }

    fn changed(&mut self) {
        self.stale = true;
        self.menu = None;
    }
}

/// The height of a card's thumbnail, for a card `width` wide.
fn thumb_height(width: f32) -> f32 {
    (width / 2.05).clamp(72.0, THUMB_HEIGHT)
}

/// The height of a card `width` wide.
fn card_height(width: f32) -> f32 {
    thumb_height(width) + CARD_FOOT
}

/// The heading a file dated `date` goes under.
fn group_of(date: Option<i64>, now: i64, tz: &jiff::tz::TimeZone) -> Group {
    let Some(date) = date else {
        return Group::Undated;
    };
    if now - date <= 7 * 24 * 60 * 60 && date <= now + 60 {
        return Group::Week;
    }
    match format::local(date, tz) {
        Some(local) => Group::Month(local.year(), local.month()),
        None => Group::Undated,
    }
}

fn group_label(group: Group, this_year: i16) -> String {
    match group {
        Group::Week => tr!("files-this-week"),
        Group::Month(year, month) if year == this_year => katna_i18n::format::month_name(month),
        Group::Month(year, month) => jiff::civil::Date::new(year, month, 1)
            .map(katna_i18n::format::month_year)
            .unwrap_or_default(),
        Group::Undated => tr!("files-undated"),
    }
}

impl MailWindow {
    /// Reads the files, each time the page opens.
    pub(super) fn load_library(&mut self, cx: &mut Context<Self>) {
        let paths = self.paths.clone();
        self.library._load = Some(cx.spawn(async move |this, cx| {
            let files = cx
                .background_executor()
                .spawn(async move { crate::data::library(&paths, LIMIT) })
                .await;
            this.update(cx, |this, cx| {
                this.set_library(files);
                cx.notify();
            })
            .ok();
        }));
    }

    fn set_library(&mut self, files: Result<Vec<LibraryFile>, String>) {
        let files = match files {
            Ok(files) => files,
            Err(err) => {
                self.library.files = Some(Err(err));
                return;
            }
        };
        let now = jiff::Timestamp::now().as_second();
        let mut senders: HashMap<String, Sender> = HashMap::new();
        let mut per_account = HashMap::new();
        let found: Vec<Found> = files
            .into_iter()
            .filter_map(|file| {
                let kind = katna_preview::kind(&file.mime, &file.name);
                // Signature logos and the like.
                if matches!(kind, Kind::Picture(_)) && file.size < SMALL_PICTURE {
                    return None;
                }
                let hay = format!(
                    "{}\n{}\n{}\n{}",
                    file.name,
                    file.subject,
                    file.from_name.as_deref().unwrap_or_default(),
                    file.from_email
                )
                .to_lowercase();
                *per_account.entry(file.account).or_default() += 1;
                if !file.mine && !file.from_email.is_empty() {
                    let sender = senders
                        .entry(file.from_email.clone())
                        .or_insert_with(|| Sender {
                            email: file.from_email.clone(),
                            name: file
                                .from_name
                                .clone()
                                .unwrap_or_else(|| file.from_email.clone()),
                            files: 0,
                        });
                    sender.files += 1;
                }
                Some(Found {
                    group: group_of(file.date, now, &self.tz),
                    kind,
                    types: Types::of(kind),
                    hay,
                    file,
                })
            })
            .collect();
        let mut senders: Vec<Sender> = senders.into_values().collect();
        senders.sort_by(|a, b| b.files.cmp(&a.files).then_with(|| a.name.cmp(&b.name)));
        senders.truncate(PEOPLE);
        self.library.senders = Rc::new(senders);
        self.library.per_account = per_account;
        self.library.files = Some(Ok(Rc::new(found)));
        self.library.stale = true;
    }

    /// The top bar's search box moves to the page, or back to mail.
    pub(super) fn swap_files_search(&mut self, entering: bool, cx: &mut Context<Self>) {
        let (placeholder, text) = if entering {
            self.library.mail_query = Some(self.search.read(cx).text().to_owned());
            (tr!("files-search"), self.library.query.clone())
        } else {
            self.library.menu = None;
            let text = self.library.mail_query.take().unwrap_or_default();
            (tr!("search-mail"), text)
        };
        self.search.update(cx, |search, cx| {
            search.set_placeholder(placeholder);
            search.set_text(text, cx);
        });
    }

    /// The top bar's search box changed while the page shows.
    pub(super) fn on_files_search(
        &mut self,
        search: &gpui::Entity<TextInput>,
        event: &InputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            InputEvent::Changed => {
                self.library.query = search.read(cx).text().to_owned();
                self.library.changed();
            }
            InputEvent::Cancel => {
                if self.library.query.is_empty() {
                    window.blur(cx);
                } else {
                    self.library.query.clear();
                    self.library.changed();
                    search.update(cx, |search, cx| search.set_text("", cx));
                }
            }
            InputEvent::Submit => {}
        }
        cx.notify();
    }

    /// Shows the mail file `id` came with, in Mail.
    pub(super) fn show_file_mail(
        &mut self,
        id: MessageId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_app(App::Mail, cx);
        self.show_message(id, window, cx);
    }

    /// Does `act` with `file`, downloading its mail first if needed.
    fn file_act(&mut self, file: RowFile, act: Act, window: &mut Window, cx: &mut Context<Self>) {
        self.library.menu = None;
        let Some((raw, encrypted)) = self.attachment_raw(file.message) else {
            let done = self.download(file.message, cx);
            self.show_snackbar(tr!("files-downloading"), None, cx);
            cx.spawn_in(window, async move |this, cx| {
                done.recv().await.ok();
                this.update_in(cx, |this, window, cx| {
                    if this.attachment_raw(file.message).is_some() {
                        this.file_act(file, act, window, cx);
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
            // Protected mail: its files show once it is opened.
            self.show_snackbar(tr!("attachment-open-message"), None, cx);
            return;
        };
        if let Act::Save = act {
            self.save_from_message(file.message, index, &file.name, cx);
            return;
        }
        cx.spawn_in(window, async move |this, cx| {
            let got = cx
                .background_executor()
                .spawn(async move { katna_render::attachment_file(&raw, index) })
                .await;
            this.update_in(cx, |this, window, cx| {
                let Some(got) = got else {
                    let name = file.name.as_str();
                    this.show_snackbar(tr!("attachment-read-failed", name = name), None, cx);
                    return;
                };
                match act {
                    Act::OpenWith => this.open_attachment_with(Arc::new(got), true, encrypted, cx),
                    Act::Forward => this.new_mail_with_file(&got, window, cx),
                    Act::Save => {}
                }
            })
            .ok();
        })
        .detach();
    }

    /// Makes the thumbnails the last frame asked for, a few at a time in
    /// the background, and lets go of those not drawn for longest.
    fn request_library_thumbs(&mut self, cx: &mut Context<Self>) {
        if self.library._thumbs.is_some() || self.library.wanted.is_empty() {
            return;
        }
        let batch: Vec<(RowFile, Kind)> = self.library.wanted.drain(..).take(THUMB_BATCH).collect();
        self.library.wanted.clear();
        for (file, _) in &batch {
            self.library.asked.insert((file.message, file.order));
        }
        let paths = self.paths.clone();
        self.library._thumbs = Some(cx.spawn(async move |this, cx| {
            let made = cx
                .background_executor()
                .spawn(async move {
                    let mut ids: Vec<MessageId> = batch.iter().map(|(f, _)| f.message).collect();
                    ids.sort_unstable();
                    ids.dedup();
                    let raws = crate::data::raw_messages(&paths, &ids);
                    let views: HashMap<MessageId, katna_render::MessageView> = raws
                        .iter()
                        .map(|(id, raw)| (*id, katna_render::message_view(raw)))
                        .collect();
                    batch
                        .into_iter()
                        .filter_map(|(file, kind)| {
                            let raw = raws.get(&file.message)?;
                            let view = views.get(&file.message)?;
                            let index = row_file_index(&view.attachments, &file)?;
                            Some(((file.message, file.order), thumbnail(raw, index, kind)?))
                        })
                        .collect::<Vec<_>>()
                })
                .await;
            this.update(cx, |this, cx| {
                let frame = this.library.frame;
                let library = &mut this.library;
                library._thumbs = None;
                for (key, thumb) in made {
                    library.thumbs.insert(key, (thumb, frame));
                }
                if library.thumbs.len() > THUMBS_KEPT {
                    let mut drawn: Vec<((MessageId, usize), u64)> =
                        library.thumbs.iter().map(|(k, (_, f))| (*k, *f)).collect();
                    drawn.sort_by_key(|(_, f)| *f);
                    let extra = library.thumbs.len() - THUMBS_KEPT;
                    for (key, _) in drawn.into_iter().take(extra) {
                        if let Some((thumb, _)) = library.thumbs.remove(&key) {
                            this.files.released.extend(thumb.bitmaps());
                        }
                        library.asked.remove(&key);
                    }
                }
                cx.notify();
            })
            .ok();
        }));
    }

    // --- Drawing -------------------------------------------------------------

    pub(super) fn render_files(
        &mut self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        self.library.frame += 1;
        let shape = self.layout.shape;
        let pad = if shape.is_phone() { 12.0 } else { 24.0 };
        let room = shape.width
            - shape.rail()
            - shape.card_margin()
            - self.page_side_width(NAV_WIDTH)
            - 2.0 * pad;
        let min = if shape.is_phone() {
            CARD_MIN_PHONE
        } else {
            CARD_MIN
        };
        let columns = (((room + GAP) / (min + GAP)).floor() as usize).max(1);
        let card_width = ((room - GAP * (columns - 1) as f32) / columns as f32).max(min);
        if self.library.stale || columns != self.library.columns {
            let line = card_height(card_width) + GAP;
            self.library
                .rebuild(columns, line, jiff::Timestamp::now().as_second());
        }
        let body = match &self.library.files {
            None => placeholder(&tr!("files-loading"), th),
            Some(Err(err)) => placeholder(err, th),
            Some(Ok(files)) if files.is_empty() => placeholder(&tr!("files-empty"), th),
            Some(Ok(_)) => self.render_files_body(card_width, pad, th, window, cx),
        };
        let menu = self.render_files_menu(th, cx);
        let side = self.render_files_nav(th, cx);
        let side = self.page_side(side, NAV_WIDTH, true, th, cx);
        div()
            .id("files-page")
            .relative()
            .size_full()
            .flex()
            .flex_row()
            .children(side.docked)
            .child(div().flex_1().min_w_0().h_full().child(body))
            .children(side.drawer)
            .children(menu)
            .with_animation(
                "files-page-in",
                Animation::new(std::time::Duration::from_millis(220)).with_easing(ease_out_quint()),
                |el, t| el.opacity(t),
            )
            .into_any_element()
    }

    fn render_files_nav(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let page = &self.library;
        // The line's text color: see `side_row`.
        let count = |n: usize, on: bool| {
            let n = n as u64;
            super::nav::count_pill(n, if on { th.row_selected_text } else { th.text })
        };
        let rule = || {
            div()
                .flex_none()
                .mt(px(12.0))
                .mx(px(24.0))
                .mb(px(4.0))
                .h(px(1.0))
                .bg(rgba(th.divider))
        };
        let heading = |text: String| {
            div()
                .flex_none()
                .mt(px(8.0))
                .mb(px(4.0))
                .px(px(24.0))
                .text_size(px(12.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(th.text_faint))
                .truncate()
                .child(text)
        };
        let mut nav = div().flex_none().pt(px(8.0)).pb(px(16.0)).flex().flex_col();
        for (n, types) in Types::ALL.into_iter().enumerate() {
            let on = page.types == types;
            nav = nav.child(
                super::nav::side_row(("files-type", n), types.icon(), types.label(), on, th)
                    .when(page.counts[n] > 0, |d| d.child(count(page.counts[n], on)))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.library.types = types;
                        this.library.changed();
                        cx.notify();
                    })),
            );
        }
        let accounts: Vec<_> = self
            .accounts
            .iter()
            .filter(|a| page.per_account.contains_key(&a.id))
            .collect();
        if accounts.len() > 1 {
            nav = nav.child(rule()).child(heading(tr!("files-accounts")));
            for account in accounts {
                let id = account.id;
                let on = page.account == Some(id);
                nav = nav.child(
                    super::nav::side_row(
                        ("files-account", id.0 as usize),
                        "mail",
                        account.address.clone(),
                        on,
                        th,
                    )
                    .child(count(page.per_account.get(&id).copied().unwrap_or(0), on))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        // A second click shows every account again.
                        this.library.account = (this.library.account != Some(id)).then_some(id);
                        this.library.changed();
                        cx.notify();
                    })),
                );
            }
        }
        nav = nav.child(rule()).child(heading(tr!("files-shown")));
        for (n, direction, icon_name, label) in [
            (0_usize, Direction::Received, "inbox", tr!("files-received")),
            (1, Direction::Sent, "sent", tr!("files-sent")),
        ] {
            let on = page.direction == direction;
            nav = nav.child(
                super::nav::side_row(("files-direction", n), icon_name, label, on, th).on_click(
                    cx.listener(move |this, _, _, cx| {
                        this.library.direction = if this.library.direction == direction {
                            Direction::Any
                        } else {
                            direction
                        };
                        this.library.changed();
                        cx.notify();
                    }),
                ),
            );
        }
        div()
            .id("files-nav")
            .flex_none()
            .w(px(NAV_WIDTH))
            .h_full()
            .overflow_y_scroll()
            .child(nav)
            .into_any_element()
    }

    /// The bar over the files, then the files.
    fn render_files_body(
        &mut self,
        card_width: f32,
        pad: f32,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let page = &self.library;
        let desktop = self.layout.shape.is_desktop();
        let chip = |id: &'static str, label: String, on: bool| {
            div()
                .id(id)
                .flex_none()
                .h(px(32.0))
                .pl(px(12.0))
                .pr(px(6.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(4.0))
                .rounded(px(8.0))
                .border_1()
                .border_color(rgba(if on { th.nav_selected } else { th.divider }))
                .when(on, |d| d.bg(rgba(th.nav_selected)))
                .cursor_pointer()
                .hover(|s| s.bg(rgba(th.hover)))
                .text_size(px(13.0))
                .text_color(rgba(if on {
                    th.nav_selected_text
                } else {
                    th.text_dim
                }))
                .child(div().whitespace_nowrap().child(label))
                .child(icon(
                    "drop-down",
                    if on {
                        th.nav_selected_text
                    } else {
                        th.text_dim
                    },
                    18.0,
                ))
        };
        let person = page.person.as_ref().map(|email| {
            page.senders
                .iter()
                .find(|s| s.email == *email)
                .map(|s| s.name.clone())
                .unwrap_or_else(|| email.clone())
        });
        let person_chip = chip(
            "files-people",
            match &person {
                Some(name) => tr!("files-from-person", name = name.as_str()),
                None => tr!("files-anyone"),
            },
            person.is_some(),
        )
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(|this, e: &MouseDownEvent, _, cx| {
                cx.stop_propagation();
                this.library.menu = Some(Menu::People(e.position));
                cx.notify();
            }),
        );
        let time_chip = chip("files-time", page.time.label(), page.time != Time::Any)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, e: &MouseDownEvent, _, cx| {
                    cx.stop_propagation();
                    this.library.menu = Some(Menu::Time(e.position));
                    cx.notify();
                }),
            );
        let sort_chip = chip("files-sort", page.sort.label(), false).on_mouse_down(
            MouseButton::Left,
            cx.listener(|this, e: &MouseDownEvent, _, cx| {
                cx.stop_propagation();
                this.library.menu = Some(Menu::Sort(e.position));
                cx.notify();
            }),
        );
        let view_button = |id: &'static str, name: &'static str, grid: bool, label: String| {
            let on = page.grid == grid;
            icon_button(id, name, 20.0, th)
                .when(on, |d| d.bg(rgba(th.search)))
                .tooltip(tip(label, th))
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.library.grid = grid;
                    this.library.changed();
                    cx.notify();
                }))
        };
        let views = div()
            .flex_none()
            .flex()
            .flex_row()
            .gap(px(4.0))
            .child(view_button("files-grid", "table", true, tr!("files-grid")))
            .child(view_button(
                "files-list",
                "list-bulleted",
                false,
                tr!("files-list"),
            ));
        let count = tr!(
            "files-count",
            count = page.shown.len(),
            size = format::size(page.shown_bytes)
        );
        let title = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_row()
            .items_baseline()
            .gap(px(10.0))
            .child(
                div()
                    .flex_none()
                    .text_size(px(20.0))
                    .text_color(rgba(th.text))
                    .child(page.types.label()),
            )
            .child(
                div()
                    .min_w_0()
                    // GPUI lines the two up by their boxes' first lines
                    // rather than by the letters, which leaves the smaller
                    // text 3 px low; this puts it on the title's baseline.
                    .relative()
                    .top(px(-3.0))
                    .truncate()
                    .text_size(px(13.0))
                    .text_color(rgba(th.text_faint))
                    .child(count),
            );
        let chips = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .child(person_chip)
            .child(time_chip)
            .child(sort_chip);
        // The rule under the bar lines up with the title and the cards
        // rather than running to the card's edges.
        let rule = div()
            .flex_none()
            .mx(px(pad))
            .h(px(1.0))
            .bg(rgba(th.divider));
        let bar = if desktop {
            div()
                .flex_none()
                .flex()
                .flex_col()
                .child(
                    div()
                        .h(px(56.0))
                        .px(px(pad))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(8.0))
                        .child(title)
                        .child(chips)
                        .child(views),
                )
                .child(rule)
        } else {
            // Kinds of file as chips, as the side column is a drawer.
            let kinds = Types::ALL.into_iter().enumerate().map(|(n, types)| {
                let on = page.types == types;
                div()
                    .id(("files-kind-chip", n))
                    .flex_none()
                    .h(px(32.0))
                    .px(px(12.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(6.0))
                    .rounded(px(8.0))
                    .border_1()
                    .border_color(rgba(if on { th.nav_selected } else { th.divider }))
                    .when(on, |d| d.bg(rgba(th.nav_selected)))
                    .cursor_pointer()
                    .text_size(px(13.0))
                    .text_color(rgba(if on {
                        th.nav_selected_text
                    } else {
                        th.text_dim
                    }))
                    .when(on, |d| d.font_weight(FontWeight::BOLD))
                    .child(icon(
                        if on { "check" } else { types.icon() },
                        if on {
                            th.nav_selected_text
                        } else {
                            th.text_dim
                        },
                        16.0,
                    ))
                    .child(div().whitespace_nowrap().child(types.label()))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.library.types = types;
                        this.library.changed();
                        cx.notify();
                    }))
            });
            div()
                .flex_none()
                .flex()
                .flex_col()
                .child(
                    div()
                        .h(px(52.0))
                        .px(px(pad))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(8.0))
                        .child(title)
                        .child(views),
                )
                .child(
                    div()
                        .id("files-chips")
                        .overflow_x_scroll()
                        .px(px(pad))
                        .pb(px(10.0))
                        .flex()
                        .flex_row()
                        .gap(px(8.0))
                        .children(kinds)
                        .child(chips),
                )
                .child(rule)
        };
        let content = if page.shown.is_empty() {
            placeholder(&tr!("files-none-match"), th)
        } else {
            let files = list(
                page.state.clone(),
                cx.processor(move |this, ix: usize, window, cx| {
                    let th = this.theme(window);
                    let row = this.render_files_line(ix, card_width, pad, &th, cx);
                    this.request_library_thumbs(cx);
                    row
                }),
            )
            .size_full();
            let thumb = th.text_dim & 0xffff_ff00 | 0x99;
            page.bar
                .clone()
                .wrap("files-scroll", &page.state, files, thumb, window, cx)
                .into_any_element()
        };
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(bar)
            .child(div().flex_1().min_h_0().child(content))
            .into_any_element()
    }

    fn render_files_line(
        &mut self,
        ix: usize,
        card_width: f32,
        pad: f32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(line) = self.library.lines.get(ix).cloned() else {
            return div().into_any_element();
        };
        let Some(files) = self.library.found().cloned() else {
            return div().into_any_element();
        };
        let shown = self.library.shown.clone();
        match line {
            Line::Heading(group) => {
                let this_year = format::local(jiff::Timestamp::now().as_second(), &self.tz)
                    .map_or(0, |d| d.year());
                div()
                    .h(px(HEADING_HEIGHT))
                    .px(px(pad))
                    .pt(px(18.0))
                    .text_size(px(14.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgba(th.text))
                    .child(group_label(group, this_year))
                    .into_any_element()
            }
            Line::Cards(range) => {
                let cards: Vec<AnyElement> = range
                    .map(|at| {
                        let found = &files[shown[at]];
                        self.render_file_card(shown[at], found, card_width, th, cx)
                    })
                    .collect();
                // Every row has a slot per column, so the cards share the
                // row's whole width exactly and a short row's cards keep
                // their size.
                let empty = self.library.columns.saturating_sub(cards.len());
                div()
                    .w_full()
                    .px(px(pad))
                    .pb(px(GAP))
                    .flex()
                    .flex_row()
                    .gap(px(GAP))
                    .children(cards)
                    .children((0..empty).map(|_| div().flex_1().flex_basis(px(0.0)).min_w_0()))
                    .into_any_element()
            }
            Line::Row(at) => {
                let found = &files[shown[at]];
                self.render_file_row(shown[at], found, pad, th, cx)
            }
        }
    }

    /// The thumbnail of `found`, asking for it when it is not made yet.
    fn library_thumb(&mut self, found: &Found) -> Option<Thumb> {
        if !self.config.mail.attachment_previews || !has_thumbnail(found.kind) {
            return None;
        }
        let key = found.key();
        let frame = self.library.frame;
        if let Some((thumb, drawn)) = self.library.thumbs.get_mut(&key) {
            *drawn = frame;
            return Some(thumb.clone());
        }
        if found.file.downloaded
            && !self.library.asked.contains(&key)
            && !self
                .library
                .wanted
                .iter()
                .any(|(f, _)| (f.message, f.order) == key)
        {
            self.library.wanted.push((found.row_file(), found.kind));
        }
        None
    }

    /// Handlers every file shares: open on click, the menu on right-click.
    fn file_handlers(
        &self,
        el: gpui::Stateful<gpui::Div>,
        ix: usize,
        file: RowFile,
        cx: &mut Context<Self>,
    ) -> gpui::Stateful<gpui::Div> {
        el.cursor_pointer()
            .on_click(cx.listener(move |this, _, window, cx| {
                this.library.menu = None;
                this.open_row_file(&file, window, cx);
            }))
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(move |this, e: &MouseDownEvent, _, cx| {
                    cx.stop_propagation();
                    this.library.menu = Some(Menu::File { ix, at: e.position });
                    cx.notify();
                }),
            )
    }

    fn render_file_card(
        &mut self,
        ix: usize,
        found: &Found,
        width: f32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let thumb = self.library_thumb(found);
        let thumb_height = thumb_height(width);
        let file = found.row_file();
        let message = file.message;
        let group = SharedString::from(format!("files-card-{ix}"));
        let frost = thumb.as_ref().and_then(Thumb::frosted);
        let show = panel_button(("files-card-mail", ix), "mail", tr!("files-show-mail"), th)
            .on_click(cx.listener(move |this, _, window, cx| {
                cx.stop_propagation();
                this.show_file_mail(message, window, cx);
            }));
        let save_file = file.clone();
        let save = panel_button(
            ("files-card-save", ix),
            "download",
            tr!("attachment-save"),
            th,
        )
        .on_click(cx.listener(move |this, _, window, cx| {
            cx.stop_propagation();
            this.file_act(save_file.clone(), Act::Save, window, cx);
        }));
        let panel = hover_panel(
            group.clone(),
            found.file.name.clone(),
            found.file.size,
            frost,
            vec![show.into_any_element(), save.into_any_element()],
            th,
        );
        let fill = self.chip_fill(&file, false, th);
        let date = found
            .file
            .date
            .and_then(|d| format::local(d, &self.tz))
            .zip(format::local(jiff::Timestamp::now().as_second(), &self.tz))
            .map(|(d, now)| format::list_date(d, now))
            .unwrap_or_default();
        let sender = found.sender();
        let avatar = self.person_avatar(&sender, &found.file.from_email, 18.0);
        let subject = if found.file.subject.trim().is_empty() {
            tr!("files-no-subject")
        } else {
            found.file.subject.clone()
        };
        // The thumbnail and name, which the hover panel covers; the lines
        // under them stay, so the subject can be clicked.
        let head = div()
            .relative()
            .flex_none()
            .h(px(thumb_height + NAME_HEIGHT))
            .flex()
            .flex_col()
            .child(
                div()
                    .h(px(thumb_height))
                    .w_full()
                    .child(card_top(thumb, found.kind, 44.0, th)),
            )
            .child(
                div()
                    .relative()
                    .overflow_hidden()
                    .h(px(NAME_HEIGHT))
                    .px(px(10.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .border_t_1()
                    .border_color(rgba(th.divider))
                    .children(fill)
                    .child(kind_badge(found.kind, 18.0))
                    .child(
                        div()
                            .min_w_0()
                            .truncate()
                            .text_size(px(13.0))
                            .text_color(rgba(th.text))
                            .child(found.file.name.clone()),
                    ),
            )
            .child(panel);
        let card = div()
            .id(("files-card", ix))
            .group(group)
            .relative()
            .flex_1()
            .flex_basis(px(0.0))
            .min_w_0()
            .h(px(thumb_height + CARD_FOOT))
            .flex()
            .flex_col()
            .overflow_hidden()
            .rounded(px(CARD_RADIUS))
            .border_1()
            .border_color(rgba(th.divider))
            .bg(rgba(th.surface))
            .hover(|s| s.shadow(crate::widgets::elevation(th, 1.0)))
            .child(head)
            .child(
                div()
                    .h(px(22.0))
                    .px(px(10.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(6.0))
                    .text_size(px(12.0))
                    .text_color(rgba(th.text_dim))
                    .child(avatar)
                    .child(div().min_w_0().truncate().child(sender))
                    .child(div().flex_none().text_color(rgba(th.text_faint)).child("·"))
                    .child(
                        div()
                            .flex_none()
                            .text_color(rgba(th.text_faint))
                            .child(date),
                    ),
            )
            .child(
                div()
                    .id(("files-card-subject", ix))
                    .h(px(28.0))
                    .px(px(10.0))
                    .pb(px(6.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(6.0))
                    .text_size(px(12.0))
                    .text_color(rgba(th.accent))
                    .hover(|s| s.underline())
                    .tooltip(tip(tr!("files-show-mail"), th))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        cx.stop_propagation();
                        this.show_file_mail(message, window, cx);
                    }))
                    .child(icon("mail", th.accent, 14.0))
                    .child(div().min_w_0().truncate().child(subject)),
            );
        self.file_handlers(card, ix, file, cx).into_any_element()
    }

    fn render_file_row(
        &mut self,
        ix: usize,
        found: &Found,
        pad: f32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let file = found.row_file();
        let message = file.message;
        let phone = self.layout.shape.is_phone();
        let fill = self.chip_fill(&file, false, th);
        let date = found
            .file
            .date
            .and_then(|d| format::local(d, &self.tz))
            .zip(format::local(jiff::Timestamp::now().as_second(), &self.tz))
            .map(|(d, now)| format::list_date(d, now))
            .unwrap_or_default();
        let sender = found.sender();
        let subject = if found.file.subject.trim().is_empty() {
            tr!("files-no-subject")
        } else {
            found.file.subject.clone()
        };
        let row = div()
            .id(("files-row", ix))
            .w_full()
            .relative()
            .overflow_hidden()
            .h(px(ROW_HEIGHT))
            .px(px(pad))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(14.0))
            .border_b_1()
            .border_color(rgba(th.divider))
            .hover(|s| s.bg(rgba(th.hover)))
            .children(fill)
            .child(kind_badge(found.kind, 24.0))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .truncate()
                            .text_size(px(14.0))
                            .text_color(rgba(th.text))
                            .child(found.file.name.clone()),
                    )
                    .child(
                        div()
                            .truncate()
                            .text_size(px(12.0))
                            .text_color(rgba(th.text_faint))
                            .child(if phone {
                                format!("{sender} · {}", format::size(found.file.size))
                            } else {
                                format::size(found.file.size)
                            }),
                    ),
            )
            .when(!phone, |d| {
                d.child(
                    div()
                        .w(px(180.0))
                        .flex_none()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(8.0))
                        .text_size(px(13.0))
                        .text_color(rgba(th.text_dim))
                        .child(self.person_avatar(&sender, &found.file.from_email, 20.0))
                        .child(div().min_w_0().truncate().child(sender.clone())),
                )
                .child(
                    div()
                        .id(("files-row-subject", ix))
                        .flex_1()
                        .min_w_0()
                        .truncate()
                        .text_size(px(13.0))
                        .text_color(rgba(th.accent))
                        .hover(|s| s.underline())
                        .tooltip(tip(tr!("files-show-mail"), th))
                        .on_click(cx.listener(move |this, _, window, cx| {
                            cx.stop_propagation();
                            this.show_file_mail(message, window, cx);
                        }))
                        .child(subject),
                )
            })
            .child(
                div()
                    .flex_none()
                    .w(px(if phone { 56.0 } else { 80.0 }))
                    .text_right()
                    .text_size(px(12.0))
                    .text_color(rgba(th.text_faint))
                    .child(date),
            )
            .child(
                icon_button(("files-row-mail", ix), "mail", 20.0, th)
                    .tooltip(tip(tr!("files-show-mail"), th))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        cx.stop_propagation();
                        this.show_file_mail(message, window, cx);
                    })),
            );
        self.file_handlers(row, ix, file, cx).into_any_element()
    }

    fn render_files_menu(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        let menu = self.library.menu.as_ref()?;
        let item = |id: SharedString, name: &'static str, label: String, on: bool| {
            div()
                .id(id)
                .h(px(36.0))
                .pl(px(16.0))
                .pr(px(16.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(14.0))
                .cursor_pointer()
                .hover(|s| s.bg(rgba(th.hover)))
                .child(icon(name, th.text_dim, 18.0))
                .child(div().flex_1().min_w_0().truncate().child(label))
                .when(on, |d| d.child(icon("check", th.accent, 18.0)))
        };
        let separator = || {
            div()
                .my(px(6.0))
                .h(px(1.0))
                .bg(rgba(th.divider))
                .into_any_element()
        };
        let (at, items): (Point<Pixels>, Vec<AnyElement>) = match *menu {
            Menu::File { ix, at } => {
                let found = self.library.found()?.get(ix)?;
                let file = found.row_file();
                let message = file.message;
                let email = found.file.from_email.clone();
                let sender = found.sender();
                let name = found.file.name.clone();
                let mut items = Vec::new();
                let f = file.clone();
                items.push(
                    item("files-menu-open".into(), "eye", tr!("files-open"), false)
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.library.menu = None;
                            this.open_row_file(&f, window, cx);
                        }))
                        .into_any_element(),
                );
                let f = file.clone();
                items.push(
                    item(
                        "files-menu-open-with".into(),
                        "open-external",
                        tr!("files-open-with"),
                        false,
                    )
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.file_act(f.clone(), Act::OpenWith, window, cx)
                    }))
                    .into_any_element(),
                );
                let f = file.clone();
                items.push(
                    item(
                        "files-menu-save".into(),
                        "download",
                        tr!("files-save"),
                        false,
                    )
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.file_act(f.clone(), Act::Save, window, cx)
                    }))
                    .into_any_element(),
                );
                items.push(separator());
                items.push(
                    item(
                        "files-menu-show-mail".into(),
                        "mail",
                        tr!("files-show-mail"),
                        false,
                    )
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.library.menu = None;
                        this.show_file_mail(message, window, cx);
                    }))
                    .into_any_element(),
                );
                items.push(
                    item(
                        "files-menu-window".into(),
                        "open-full",
                        tr!("files-mail-window"),
                        false,
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.library.menu = None;
                        this.message_in_window(message, false, cx);
                    }))
                    .into_any_element(),
                );
                let f = file.clone();
                items.push(
                    item(
                        "files-menu-forward".into(),
                        "forward",
                        tr!("files-forward"),
                        false,
                    )
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.file_act(f.clone(), Act::Forward, window, cx)
                    }))
                    .into_any_element(),
                );
                items.push(separator());
                if !email.is_empty() && !found.file.mine {
                    items.push(
                        item(
                            "files-menu-person".into(),
                            "people",
                            tr!("files-from-them", name = sender.as_str()),
                            false,
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.library.person = Some(email.clone());
                            this.library.changed();
                            cx.notify();
                        }))
                        .into_any_element(),
                    );
                }
                items.push(
                    item(
                        "files-menu-copy".into(),
                        "copy",
                        tr!("files-copy-name"),
                        false,
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.library.menu = None;
                        cx.write_to_clipboard(ClipboardItem::new_string(name.clone()));
                        this.show_snackbar(tr!("files-name-copied"), None, cx);
                    }))
                    .into_any_element(),
                );
                (at, items)
            }
            Menu::People(at) => {
                let mut items = vec![
                    item(
                        "files-person-any".into(),
                        "people",
                        tr!("files-anyone"),
                        self.library.person.is_none(),
                    )
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.library.person = None;
                        this.library.changed();
                        cx.notify();
                    }))
                    .into_any_element(),
                ];
                if !self.library.senders.is_empty() {
                    items.push(separator());
                }
                for (n, sender) in self.library.senders.iter().enumerate() {
                    let email = sender.email.clone();
                    let on = self.library.person.as_deref() == Some(email.as_str());
                    items.push(
                        div()
                            .id(("files-person", n))
                            .h(px(40.0))
                            .px(px(16.0))
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(12.0))
                            .cursor_pointer()
                            .hover(|s| s.bg(rgba(th.hover)))
                            .child(self.person_avatar(&sender.name, &sender.email, 24.0))
                            .child(
                                div()
                                    .flex_1()
                                    .min_w_0()
                                    .truncate()
                                    .child(sender.name.clone()),
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(rgba(th.text_faint))
                                    .child(katna_i18n::format::number(sender.files as u64)),
                            )
                            .when(on, |d| d.child(icon("check", th.accent, 18.0)))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.library.person = Some(email.clone());
                                this.library.changed();
                                cx.notify();
                            }))
                            .into_any_element(),
                    );
                }
                (at, items)
            }
            Menu::Time(at) => {
                let items = Time::ALL
                    .into_iter()
                    .enumerate()
                    .map(|(n, time)| {
                        item(
                            format!("files-time-{n}").into(),
                            "schedule",
                            time.label(),
                            self.library.time == time,
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.library.time = time;
                            this.library.changed();
                            cx.notify();
                        }))
                        .into_any_element()
                    })
                    .collect();
                (at, items)
            }
            Menu::Sort(at) => {
                let items = Sort::ALL
                    .into_iter()
                    .enumerate()
                    .map(|(n, sort)| {
                        item(
                            format!("files-sort-{n}").into(),
                            "list-bulleted",
                            sort.label(),
                            self.library.sort == sort,
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.library.sort = sort;
                            this.library.changed();
                            cx.notify();
                        }))
                        .into_any_element()
                    })
                    .collect();
                (at, items)
            }
        };
        let panel = div()
            .w(px(MENU_WIDTH))
            .py(px(8.0))
            .flex()
            .flex_col()
            .map(|d| raised(d, th, 8.0, 3.0))
            .text_size(px(14.0))
            .text_color(rgba(th.text))
            .children(items)
            .with_animation(
                "files-menu",
                Animation::new(std::time::Duration::from_millis(140)).with_easing(ease_out_quint()),
                |el, t| el.opacity(t).mt(px(-4.0 * (1.0 - t))),
            );
        let close = || {
            cx.listener(|this: &mut Self, _: &MouseDownEvent, _, cx| {
                this.library.menu = None;
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
                            .id("files-menu-scrim")
                            .absolute()
                            .top(px(-2000.0))
                            .left(px(-4000.0))
                            .w(px(8000.0))
                            .h(px(6000.0))
                            .occlude()
                            .on_mouse_down(MouseButton::Left, close())
                            .on_mouse_down(MouseButton::Right, close()),
                    )
                    .with_priority(3),
                )
                .child(
                    deferred(
                        anchored()
                            .position(at)
                            .snap_to_window_with_margin(px(8.0))
                            .child(div().occlude().child(panel)),
                    )
                    .with_priority(4),
                )
                .into_any_element(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn times_keep_their_files() {
        let now = 1_800_000_000;
        let day = 24 * 60 * 60;
        assert!(Time::Week.keeps(Some(now - 6 * day), now));
        assert!(!Time::Week.keeps(Some(now - 8 * day), now));
        assert!(Time::Older.keeps(None, now));
        assert!(!Time::Year.keeps(None, now));
        assert!(Time::Any.keeps(None, now));
    }

    #[test]
    fn kinds_fall_into_the_side_column() {
        assert_eq!(Types::of(Kind::Pdf), Types::Pdfs);
        assert_eq!(Types::of(Kind::Text), Types::Documents);
        assert_eq!(Types::of(Kind::Other), Types::Other);
    }
}
