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

use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::rc::Rc;
use std::sync::Arc;

use gpui::{
    Animation, AnimationExt, AnyElement, Bounds, ClipboardItem, Context, FocusHandle, FontWeight,
    KeyDownEvent, ListAlignment, ListState, MouseButton, MouseDownEvent, MouseUpEvent, Pixels,
    Point, ScrollHandle, ScrollWheelEvent, SharedString, Task, Window, anchored, canvas, deferred,
    div, ease_out_quint, linear_color_stop, linear_gradient, list, point, prelude::*, rgba,
};
use jiff::civil::Date;
use katna_core::AccountId;
use katna_i18n::tr;
use katna_preview::Kind;
use katna_store::{LibraryFile, MessageId};
use katna_ui::{ScrollBar, px};

use super::MailWindow;
use super::account_roll::Notches;
use super::apps::App;
use super::attachments::{
    CARD_RADIUS, Item, Thumb, card_top, has_thumbnail, hover_panel, kind_badge, panel_button,
    row_file_index, thumbnail,
};
use super::compose::schedule;
use crate::data::RowFile;
use crate::format;
use crate::theme::{Theme, fade};
use crate::widgets::{icon, icon_button, placeholder, raised, tip};
use katna_ui::text_input::{InputEvent, TextInput};

mod drive;
pub(super) mod picker;

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
/// The time calendar: a day's width and a week's height, its padding and
/// the space between its two months.
const DAY: f32 = 36.0;
const ROW: f32 = 34.0;
const PAD: f32 = 16.0;
const BETWEEN: f32 = 28.0;
const CALENDAR_WIDTH: f32 = 14.0 * DAY + BETWEEN + 2.0 * PAD;
/// Pictures whose pixel size is read in one background run; the rest wait
/// for the next time the page opens.
const MEASURE_BATCH: usize = 200;
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

/// The days the files are from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Time {
    Any,
    /// Local days from the first to the last, both included.
    Days(Date, Date),
}

impl Time {
    /// The days from `one` to `other`, either way round.
    fn between(one: Date, other: Date) -> Self {
        Self::Days(one.min(other), one.max(other))
    }

    /// The chip's text: a quick pick's name when the days are one,
    /// else the days.
    fn label(self, today: Date) -> String {
        let Self::Days(first, last) = self else {
            return tr!("files-time-any");
        };
        if let Some(quick) = Quick::ALL
            .into_iter()
            .find(|q| q.days(today) == Some((first, last)))
        {
            return quick.label();
        }
        let day = |d: Date| {
            let at = d.to_datetime(jiff::civil::Time::midnight());
            if d.year() == today.year() {
                katna_i18n::format::day_month(at)
            } else {
                katna_i18n::format::day_month_year(at)
            }
        };
        if first.day() == 1 && last == first.last_of_month() {
            katna_i18n::format::month_year(first)
        } else if first == last {
            day(first)
        } else {
            tr!("files-time-between", first = day(first), last = day(last))
        }
    }

    /// Whether a file from the local day `day` is shown.
    fn keeps(self, day: Option<Date>) -> bool {
        match self {
            Self::Any => true,
            Self::Days(first, last) => day.is_some_and(|d| first <= d && d <= last),
        }
    }

    /// The same length of time `by` lengths later (earlier when `by` is
    /// negative): whole months move by months, so "This month" turns to
    /// the next month rather than 31 days on.
    fn shifted(self, by: i32) -> Self {
        let Self::Days(first, last) = self else {
            return self;
        };
        let moved = if first.day() == 1 && last == last.last_of_month() {
            let months = i32::from(last.year() - first.year()) * 12
                + i32::from(last.month() - first.month())
                + 1;
            first
                .checked_add(jiff::Span::new().months(by * months))
                .ok()
                .and_then(|first| {
                    let end = first
                        .checked_add(jiff::Span::new().months(months - 1))
                        .ok()?
                        .last_of_month();
                    Some((first, end))
                })
        } else {
            let days = first.until(last).map_or(0, |s| s.get_days()) + 1;
            let span = jiff::Span::new().days(i64::from(by) * i64::from(days));
            first
                .checked_add(span)
                .ok()
                .zip(last.checked_add(span).ok())
        };
        moved.map_or(self, |(first, last)| Self::Days(first, last))
    }
}

/// The quick picks over the time calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Quick {
    Today,
    Yesterday,
    ThisWeek,
    LastWeek,
    ThisMonth,
    LastMonth,
}

impl Quick {
    const ALL: [Self; 6] = [
        Self::Today,
        Self::Yesterday,
        Self::ThisWeek,
        Self::LastWeek,
        Self::ThisMonth,
        Self::LastMonth,
    ];

    fn label(self) -> String {
        tr!(match self {
            Self::Today => "files-time-today",
            Self::Yesterday => "files-time-yesterday",
            Self::ThisWeek => "files-time-this-week",
            Self::LastWeek => "files-time-last-week",
            Self::ThisMonth => "files-time-this-month",
            Self::LastMonth => "files-time-last-month",
        })
    }

    /// Its first and last day, `today` being today.
    fn days(self, today: Date) -> Option<(Date, Date)> {
        let back = |d: Date, n: i64| d.checked_sub(jiff::Span::new().days(n)).ok();
        let week = back(
            today,
            i64::from(today.weekday().since(katna_i18n::format::first_weekday())),
        )?;
        Some(match self {
            Self::Today => (today, today),
            Self::Yesterday => {
                let day = back(today, 1)?;
                (day, day)
            }
            Self::ThisWeek => (week, week.checked_add(jiff::Span::new().days(6)).ok()?),
            Self::LastWeek => (back(week, 7)?, back(week, 1)?),
            Self::ThisMonth => (today.first_of_month(), today.last_of_month()),
            Self::LastMonth => {
                let end = back(today.first_of_month(), 1)?;
                (end.first_of_month(), end)
            }
        })
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
    /// Its mail's local day.
    day: Option<Date>,
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
    File {
        ix: usize,
        at: Point<Pixels>,
    },
    People(Point<Pixels>),
    /// The time calendar: its left month (the first of it), and while
    /// days are dragged across, the day the drag started on.
    Time {
        at: Point<Pixels>,
        month: Date,
        anchor: Option<Date>,
    },
    Sort(Point<Pixels>),
    Drive(drive::DriveMenu),
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
    /// The mouse wheel over the time chip, moving its days.
    wheel: Notches,
    /// Where the time chip was drawn, for its calendar to hang from.
    time_chip: Rc<Cell<Option<Bounds<Pixels>>>>,
    /// The phone's rows of chips, when they scroll sideways.
    kinds_scroll: ScrollHandle,
    filters_scroll: ScrollHandle,
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
    /// The file the arrow keys are on (its index in `files`), and what
    /// takes them.
    cursor: Option<usize>,
    focus: Option<FocusHandle>,
    thumbs: HashMap<(MessageId, usize), (Thumb, u64)>,
    asked: HashSet<(MessageId, usize)>,
    wanted: Vec<(RowFile, Kind)>,
    _thumbs: Option<Task<()>>,
    frame: u64,
    /// Pixel sizes of the pictures read so far, kept between runs.
    sizes: crate::data::PictureSizes,
    /// Downloaded pictures whose size is not read yet.
    unmeasured: Vec<RowFile>,
    _measure: Option<Task<()>>,
    /// The accounts' drives.
    pub(super) cloud: drive::Cloud,
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
            wheel: Notches::default(),
            time_chip: Rc::default(),
            kinds_scroll: ScrollHandle::new(),
            filters_scroll: ScrollHandle::new(),
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
            cursor: None,
            focus: None,
            thumbs: HashMap::new(),
            asked: HashSet::new(),
            wanted: Vec::new(),
            _thumbs: None,
            frame: 0,
            sizes: HashMap::new(),
            unmeasured: Vec::new(),
            _measure: None,
            cloud: drive::Cloud::default(),
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
    fn rebuild(&mut self, columns: usize, line: f32) {
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
                && self.time.keeps(found.day)
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
        if let Some(view) = self.cloud.view.as_mut() {
            view.stale_now();
        }
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

/// A chip over the files: a filter, `on` when it narrows them.
fn filter_chip(
    id: impl Into<gpui::ElementId>,
    label: String,
    on: bool,
    th: &Theme,
) -> gpui::Stateful<gpui::Div> {
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
}

/// The arrow at the end of a chip that opens a menu.
/// Space between the chips.
const CHIP_GAP: f32 = 8.0;
/// A kind chip's width beside its label: border, padding, icon and gap.
const KIND_CHIP_ROOM: f32 = 2.0 + 12.0 + 16.0 + 6.0 + 12.0;
/// A kind chip folded to its icon: as wide as it is tall.
const FOLDED_CHIP: f32 = 32.0;
/// A filter chip's width beside its label, its arrow's, and the time
/// chip's clear button's.
const FILTER_CHIP_ROOM: f32 = 2.0 + 12.0 + 6.0;
const CHIP_ARROW: f32 = 4.0 + 18.0;
const CHIP_CLEAR: f32 = 4.0 + 20.0;
/// How wide a row of chips fades out where more of it is scrolled away.
const CHIP_FADE: f32 = 28.0;

/// A row of chips on a phone or a tablet. When `wide` (wider than the
/// room it has) it scrolls sideways and fades out at an edge with more
/// chips past it.
fn chip_row(
    id: &'static str,
    scroll: &ScrollHandle,
    chips: gpui::Div,
    wide: bool,
    pad: f32,
    th: &Theme,
) -> gpui::Div {
    let x = scroll.offset().x;
    let max = scroll.max_offset().x;
    // Until it is first laid out it starts at the left with more past
    // its right edge.
    let (left, right) = if wide {
        (x < px(-0.5), max <= px(0.5) || x > -max + px(0.5))
    } else {
        (false, false)
    };
    let edge = |left: bool| {
        div()
            .absolute()
            .top_0()
            .bottom(px(10.0))
            .w(px(CHIP_FADE))
            .map(|d| if left { d.left_0() } else { d.right_0() })
            .bg(linear_gradient(
                if left { 90.0 } else { 270.0 },
                linear_color_stop(rgba(th.surface), 0.2),
                linear_color_stop(rgba(fade(th.surface, 0.0)), 1.0),
            ))
    };
    div()
        .relative()
        .flex_none()
        .child(
            div()
                .id(id)
                .track_scroll(scroll)
                .when(wide, |d| d.overflow_x_scroll())
                .px(px(pad))
                .pb(px(10.0))
                .flex()
                .flex_row()
                .child(chips),
        )
        .when(left, |d| d.child(edge(true)))
        .when(right, |d| d.child(edge(false)))
}

fn chip_arrow(on: bool, th: &Theme) -> AnyElement {
    icon(
        "drop-down",
        if on {
            th.nav_selected_text
        } else {
            th.text_dim
        },
        18.0,
    )
}

impl MailWindow {
    /// Reads the files, each time the page opens.
    pub(super) fn load_library(&mut self, cx: &mut Context<Self>) {
        self.load_drives();
        let paths = self.paths.clone();
        self.library._load = Some(cx.spawn(async move |this, cx| {
            let (files, sizes) = cx
                .background_executor()
                .spawn(async move {
                    (
                        crate::data::library(&paths, LIMIT),
                        crate::data::picture_sizes(&paths),
                    )
                })
                .await;
            this.update(cx, |this, cx| {
                this.library.sizes = sizes;
                this.set_library(files);
                this.measure_pictures(cx);
                cx.notify();
            })
            .ok();
        }));
    }

    /// The page has read its files, since it last opened.
    pub(super) fn library_loaded(&self) -> bool {
        self.library.files.is_some()
    }

    /// Reads the pixel sizes of downloaded pictures not read before, in the
    /// background, and keeps them for the next time the page opens: the
    /// cards showing now stay where they are.
    fn measure_pictures(&mut self, cx: &mut Context<Self>) {
        let mut batch = std::mem::take(&mut self.library.unmeasured);
        if batch.is_empty() {
            return;
        }
        batch.truncate(MEASURE_BATCH);
        let paths = self.paths.clone();
        let mut sizes = self.library.sizes.clone();
        self.library._measure = Some(cx.spawn(async move |this, cx| {
            let sizes = cx
                .background_executor()
                .spawn(async move {
                    // A few mails at a time, so memory stays small.
                    for chunk in batch.chunks(16) {
                        let mut ids: Vec<MessageId> = chunk.iter().map(|f| f.message).collect();
                        ids.sort_unstable();
                        ids.dedup();
                        let raws = crate::data::raw_messages(&paths, &ids);
                        for file in chunk {
                            let Some(raw) = raws.get(&file.message) else {
                                continue;
                            };
                            let view = katna_render::message_view(raw);
                            let size = row_file_index(&view.attachments, file)
                                .and_then(|index| katna_render::attachment_file(raw, index))
                                .and_then(|got| katna_preview::picture::dimensions(&got.bytes));
                            sizes.insert((file.message, file.order), size.unwrap_or((0, 0)));
                        }
                    }
                    crate::data::save_picture_sizes(&paths, &sizes);
                    sizes
                })
                .await;
            this.update(cx, |this, _| {
                this.library.sizes = sizes;
                this.library._measure = None;
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
        let rule = self.config.mail.files.clone();
        let library = &mut self.library;
        // Sizes of files no longer on the page are forgotten.
        let present: HashSet<(MessageId, usize)> =
            files.iter().map(|f| (f.message, f.order)).collect();
        library.sizes.retain(|key, _| present.contains(key));
        library.unmeasured.clear();
        let found: Vec<Found> = files
            .into_iter()
            .filter_map(|file| {
                let kind = katna_preview::kind(&file.mime, &file.name);
                // Signature logos and the like.
                if matches!(kind, Kind::Picture(_)) {
                    let key = (file.message, file.order);
                    let size = library.sizes.get(&key).copied();
                    if size.is_none() && file.downloaded {
                        library.unmeasured.push(RowFile {
                            message: file.message,
                            name: file.name.clone(),
                            mime: file.mime.clone(),
                            size: file.size,
                            nth: file.nth,
                            order: file.order,
                        });
                    }
                    let size = size.filter(|&(w, h)| w > 0 && h > 0);
                    if rule.leaves_out(file.size, size)
                        || rule.is_signature(&file.name, file.conversations)
                    {
                        return None;
                    }
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
                    day: file
                        .date
                        .and_then(|d| format::local(d, &self.tz))
                        .map(|d| d.date()),
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
        if self.library.cloud.view.is_some() {
            match event {
                InputEvent::Changed => {
                    let text = search.read(cx).text().to_owned();
                    self.drive_search(text, false, cx);
                }
                InputEvent::Submit => {
                    let text = search.read(cx).text().to_owned();
                    self.drive_search(text, true, cx);
                }
                InputEvent::Cancel => {
                    if search.read(cx).text().is_empty() {
                        window.blur(cx);
                    } else {
                        search.update(cx, |search, cx| search.set_text("", cx));
                        self.drive_search(String::new(), true, cx);
                    }
                }
            }
            cx.notify();
            return;
        }
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
    /// Moves the arrow keys' cursor onto the file the page shows at
    /// `place`, scrolling it into view.
    fn put_files_cursor(&mut self, place: usize, cx: &mut Context<Self>) {
        let Some(&ix) = self.library.shown.get(place) else {
            return;
        };
        self.library.cursor = Some(ix);
        let line = self.library.lines.iter().position(|line| match line {
            Line::Cards(range) => range.contains(&place),
            Line::Row(at) => *at == place,
            Line::Heading(_) => false,
        });
        if let Some(line) = line {
            // The heading over the first files shows with them.
            let line = if place == 0 { 0 } else { line };
            self.library.state.scroll_to_reveal_item(line);
        }
        cx.notify();
    }

    /// Gives the files the keyboard, the cursor on the first file when
    /// none has it yet: Down from the search box.
    pub(super) fn focus_files(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(focus) = &self.library.focus {
            window.focus(focus, cx);
        }
        if self.library.cursor.is_none() {
            self.put_files_cursor(0, cx);
        }
    }

    /// The arrow keys move between files (Up and Down by a row of cards),
    /// Home and End go to the first and last, Enter or Space opens one,
    /// and the Menu key or Shift+F10 opens its menu.
    fn on_files_key(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        let keys = &event.keystroke;
        let m = keys.modifiers;
        if m.control || m.alt || m.platform {
            return;
        }
        if keys.key == "escape" && self.library.menu.is_some() {
            self.library.menu = None;
            cx.notify();
            cx.stop_propagation();
            return;
        }
        if self.library.cloud.view.is_some() {
            if !m.shift && self.on_drive_key(keys.key.as_str(), window, cx) {
                cx.stop_propagation();
            }
            return;
        }
        let count = self.library.shown.len();
        if count == 0 {
            return;
        }
        let place = self
            .library
            .cursor
            .and_then(|ix| self.library.shown.iter().position(|&s| s == ix));
        let last = count - 1;
        let to = match (keys.key.as_str(), place) {
            ("enter" | "space", Some(place)) => {
                let file = self
                    .library
                    .found()
                    .map(|f| f[self.library.shown[place]].row_file());
                if let Some(file) = file {
                    self.open_row_file(&file, window, cx);
                }
                cx.stop_propagation();
                return;
            }
            ("menu", Some(_)) | ("f10", Some(_)) if keys.key == "menu" || m.shift => {
                if let Some(ix) = self.library.cursor {
                    let at = self.files_cursor_point(window);
                    self.library.menu = Some(Menu::File { ix, at });
                    cx.notify();
                }
                cx.stop_propagation();
                return;
            }
            ("left" | "right" | "up" | "down" | "home" | "end", None) => 0,
            ("left", Some(p)) => p.saturating_sub(1),
            ("right", Some(p)) => (p + 1).min(last),
            ("up", Some(p)) => self.files_row_step(p, -1).unwrap_or(p),
            ("down", Some(p)) => self.files_row_step(p, 1).unwrap_or(p),
            ("home", _) => 0,
            ("end", _) => last,
            ("escape", Some(_)) => {
                self.library.cursor = None;
                cx.notify();
                cx.stop_propagation();
                return;
            }
            _ => return,
        };
        self.put_files_cursor(to, cx);
        cx.stop_propagation();
    }

    /// The file in the row of cards (or list line) above or below the
    /// one at `place`, in the same column where it has one; headings are
    /// passed over.
    fn files_row_step(&self, place: usize, by: isize) -> Option<usize> {
        let lines = &self.library.lines;
        let range = |line: &Line| match line {
            Line::Cards(range) => Some(range.clone()),
            Line::Row(at) => Some(*at..*at + 1),
            Line::Heading(_) => None,
        };
        let at = lines
            .iter()
            .position(|l| range(l).is_some_and(|r| r.contains(&place)))?;
        let column = place - range(&lines[at])?.start;
        let mut ix = at as isize;
        loop {
            ix += by;
            let line = lines.get(usize::try_from(ix).ok()?)?;
            if let Some(next) = range(line) {
                return Some((next.start + column).min(next.end - 1));
            }
        }
    }

    /// The viewer shows attachment `index` of its mail (Shift and an
    /// arrow): its place in the list follows, so the arrows go on from
    /// there.
    pub(super) fn paged_library(&mut self, index: usize, cx: &mut Context<Self>) {
        // In the attach picker the place stays on the file it opened.
        if self.picker.as_ref().is_some_and(|p| p.previewing) {
            return;
        }
        let (Some(viewer), Some(message)) = (self.files.viewer.clone(), self.files.viewer_mail)
        else {
            return;
        };
        let (Some(files), Some((raw, _))) =
            (self.library.found().cloned(), self.attachment_raw(message))
        else {
            return;
        };
        let view = katna_render::message_view(&raw);
        let shown = self.library.shown.clone();
        let place = shown.iter().position(|&ix| {
            let file = files[ix].row_file();
            file.message == message && row_file_index(&view.attachments, &file) == Some(index)
        });
        // An attachment the page leaves out (a small picture) keeps the
        // last place.
        if let Some(place) = place {
            self.put_files_cursor(place, cx);
            viewer.update(cx, |viewer, cx| {
                viewer.library = Some((place, shown.len()));
                cx.notify();
            });
        }
    }

    /// Where a menu opened from the keyboard appears: the middle of the
    /// page, near the files.
    fn files_cursor_point(&self, window: &Window) -> Point<Pixels> {
        let bounds = self.library.state.viewport_bounds();
        if bounds.size.width > Pixels::ZERO {
            bounds.center()
        } else {
            let size = window.viewport_size();
            gpui::point(size.width / 2.0, size.height / 2.0)
        }
    }

    /// Where `file` is among the files the page shows, and how many
    /// there are.
    pub(super) fn library_place(&self, file: &RowFile) -> Option<(usize, usize)> {
        let files = self.library.found()?;
        let shown = &self.library.shown;
        let place = shown.iter().position(|&ix| {
            let found = &files[ix].file;
            found.message == file.message && found.order == file.order
        })?;
        Some((place, shown.len()))
    }

    /// Shows in the open viewer the file `by` places on from the one it
    /// shows, among the files the page shows (wrapping around). A file
    /// whose mail is not downloaded yet opens once it is.
    pub(super) fn step_library(&mut self, by: isize, window: &mut Window, cx: &mut Context<Self>) {
        if self.picker.as_ref().is_some_and(|p| p.previewing) {
            self.step_picker(by, window, cx);
            return;
        }
        let Some(viewer) = self.files.viewer.clone() else {
            return;
        };
        let Some((place, count)) = viewer.read(cx).library else {
            return;
        };
        let Some(files) = self.library.found().cloned() else {
            return;
        };
        let shown = self.library.shown.clone();
        if count == 0 || shown.is_empty() {
            return;
        }
        let count = shown.len() as isize;
        // A file its mail no longer lists is passed over.
        let mut found = None;
        for step in 1..=count {
            let place =
                (place as isize + by.signum() * step + by - by.signum()).rem_euclid(count) as usize;
            let file = files[shown[place]].row_file();
            let Some((raw, encrypted)) = self.attachment_raw(file.message) else {
                // Sealed (encrypted, not opened) or not downloaded: the
                // usual way, which downloads it first and opens it after.
                self.open_row_file(&file, window, cx);
                return;
            };
            let view = katna_render::message_view(&raw);
            if let Some(index) = row_file_index(&view.attachments, &file) {
                found = Some((place, file, raw, encrypted, view, index));
                break;
            }
        }
        let Some((place, file, raw, encrypted, view, index)) = found else {
            return;
        };
        let items: Vec<Item> = view
            .attachments
            .iter()
            .enumerate()
            .map(|(ix, a)| Item::new(ix, a))
            .collect();
        self.files.viewer_mail = Some(file.message);
        self.files.viewer_encrypted = encrypted;
        // Closing the viewer leaves the arrow keys on this file.
        self.put_files_cursor(place, cx);
        viewer.update(cx, |viewer, cx| {
            viewer.library = Some((place, shown.len()));
            viewer.show_from(raw, items, index, cx);
            cx.notify();
        });
    }

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
        // The attach picker over Compose borrows the files and the drive
        // while it is open: the page under it waits.
        let picking = self.picker.is_some();
        if !picking && (self.library.stale || columns != self.library.columns) {
            let line = card_height(card_width) + GAP;
            self.library.rebuild(columns, line);
        }
        let body = match &self.library.files {
            _ if picking => div().into_any_element(),
            _ if self.library.cloud.view.is_some() => {
                self.render_drive_body(card_width, self.library.columns, pad, th, window, cx)
            }
            None => placeholder(&tr!("files-loading"), th),
            Some(Err(err)) => placeholder(err, th),
            Some(Ok(files)) if files.is_empty() => placeholder(&tr!("files-empty"), th),
            Some(Ok(_)) => self.render_files_body(card_width, room, pad, th, window, cx),
        };
        let menu = if picking {
            None
        } else {
            self.render_files_menu(th, cx)
        };
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
        let count = |n: usize, on: bool| super::nav::count_pill(n as u64, on, th);
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
        let mut nav = div().flex_none().pb(px(16.0)).flex().flex_col();
        // While a drive shows, no mail line is the open one, and a click
        // on one goes back to the mail files.
        let mail = page.cloud.view.is_none();
        for (n, types) in Types::ALL.into_iter().enumerate() {
            let on = mail && page.types == types;
            nav = nav.child(
                super::nav::side_row(("files-type", n), types.icon(), types.label(), on, th)
                    .when(page.counts[n] > 0, |d| d.child(count(page.counts[n], on)))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.close_drive(cx);
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
                let on = mail && page.account == Some(id);
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
                        if this.library.cloud.view.is_some() {
                            this.close_drive(cx);
                            this.library.account = Some(id);
                        } else {
                            this.library.account = (this.library.account != Some(id)).then_some(id);
                        }
                        this.library.changed();
                        cx.notify();
                    })),
                );
            }
        }
        let mut nav = self.render_drives_nav(nav, heading, rule, th, cx);
        nav = nav.child(rule()).child(heading(tr!("files-shown")));
        for (n, direction, icon_name, label) in [
            (0_usize, Direction::Received, "inbox", tr!("files-received")),
            (1, Direction::Sent, "sent", tr!("files-sent")),
        ] {
            let on = mail && page.direction == direction;
            nav = nav.child(
                super::nav::side_row(("files-direction", n), icon_name, label, on, th).on_click(
                    cx.listener(move |this, _, _, cx| {
                        if this.library.cloud.view.is_some() {
                            this.close_drive(cx);
                            this.library.direction = Direction::Any;
                        }
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

    /// How much lower the baseline of a line of `big` text sits than one
    /// of `small` text when both start at the same top, in the font the
    /// app draws with: what puts the two on one baseline. Measured, as
    /// it differs from font to font by a pixel or two.
    fn baseline_drop(&self, window: &Window, big: Pixels, small: Pixels) -> Pixels {
        let style = window.text_style();
        let family = self.font.clone().unwrap_or(style.font_family.clone());
        let text = window.text_system();
        let id = text.resolve_font(&gpui::font(family));
        let rem = window.rem_size();
        // As GPUI lays a line out: the ascent and descent centred in the
        // line's height. Fonts give the descent either way up; the line
        // layout takes it as a positive length.
        let baseline = |size: Pixels| {
            let line = style.line_height.to_pixels(size.into(), rem);
            let (ascent, descent) = (text.ascent(id, size), text.descent(id, size).abs());
            (line - ascent - descent) / 2.0 + ascent
        };
        // In whole device pixels, as the glyphs land on them: lowering by
        // the exact difference can still round the two a pixel apart.
        let scale = window.scale_factor();
        let device = |size: Pixels| (baseline(size) * scale).round();
        (device(big) - device(small)) / scale
    }

    /// The bar over the files, then the files.
    fn render_files_body(
        &mut self,
        card_width: f32,
        room: f32,
        pad: f32,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let page = &self.library;
        let desktop = self.layout.shape.is_desktop();
        let chip = |id: &'static str, label: String, on: bool| filter_chip(id, label, on, th);
        let arrow = |on: bool| chip_arrow(on, th);
        let person_chip = self.files_person_chip(th, cx);
        let time_chip = self.files_time_chip(th, cx);
        let sort_chip = chip("files-sort", page.sort.label(), false)
            .child(arrow(false))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, e: &MouseDownEvent, _, cx| {
                    cx.stop_propagation();
                    this.library.menu = Some(Menu::Sort(e.position));
                    cx.notify();
                }),
            );
        let views = self.files_view_buttons(th, cx);
        let count = tr!(
            "files-count",
            count = page.shown.len(),
            size = format::size(page.shown_bytes)
        );
        let drop = self.baseline_drop(window, px(20.0), px(13.0));
        let title = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_row()
            .items_start()
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
                    // Lowered from the title's top onto its baseline.
                    .relative()
                    .top(drop)
                    .truncate()
                    .text_size(px(13.0))
                    .text_color(rgba(th.text_faint))
                    .child(count),
            );
        let chips = div()
            .flex_none()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(CHIP_GAP))
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
            // Kinds of file as chips, as the side column is a drawer, then
            // the filters. A chip is never cut off: a row too narrow for
            // them all puts the filters on a row of their own, then folds
            // the kinds to their icons (the picked one keeps its label),
            // and only then scrolls sideways, fading at its edges.
            let on_kind = page.types;
            let label_width = |text: &str, weight| {
                super::text_width(text, 13.0, weight, self.font.as_ref(), window)
            };
            let kind_width = |types: Types| {
                let weight = if types == on_kind {
                    FontWeight::BOLD
                } else {
                    FontWeight::NORMAL
                };
                KIND_CHIP_ROOM + label_width(&types.label(), weight)
            };
            let row_width = |widths: &[f32]| {
                widths.iter().sum::<f32>() + CHIP_GAP * widths.len().saturating_sub(1) as f32
            };
            let labelled: Vec<f32> = Types::ALL.into_iter().map(kind_width).collect();
            let folded: Vec<f32> = Types::ALL
                .into_iter()
                .map(|types| {
                    if types == on_kind {
                        kind_width(types)
                    } else {
                        FOLDED_CHIP
                    }
                })
                .collect();
            let (person, _) = self.files_person_label();
            let today = jiff::Timestamp::now().to_zoned(self.tz.clone()).date();
            let filters = row_width(&[
                FILTER_CHIP_ROOM + CHIP_ARROW + label_width(&person, FontWeight::NORMAL),
                FILTER_CHIP_ROOM
                    + if page.time == Time::Any {
                        CHIP_ARROW
                    } else {
                        CHIP_CLEAR
                    }
                    + label_width(&page.time.label(today), FontWeight::NORMAL),
                FILTER_CHIP_ROOM + CHIP_ARROW + label_width(&page.sort.label(), FontWeight::NORMAL),
            ]);
            let one_row = row_width(&labelled) + CHIP_GAP + filters <= room;
            let fold = !one_row && row_width(&labelled) > room;
            let kinds_width = row_width(if fold { &folded } else { &labelled });
            let kinds = Types::ALL
                .into_iter()
                .enumerate()
                .map(|(n, types)| self.files_kind_chip(n, types, fold, th, cx));
            let (beside, below) = if one_row {
                (Some(chips), None)
            } else {
                (None, Some(chips))
            };
            let kinds_row = div()
                .flex()
                .flex_row()
                .gap(px(CHIP_GAP))
                .children(kinds)
                .children(beside);
            let row = |id: &'static str, scroll: &ScrollHandle, child: gpui::Div, wide: bool| {
                chip_row(id, scroll, child, wide, pad, th)
            };
            let filters_row = below
                .map(|chips| row("files-filters", &page.filters_scroll, chips, filters > room));
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
                .child(row(
                    "files-chips",
                    &page.kinds_scroll,
                    kinds_row,
                    kinds_width > room,
                ))
                .children(filters_row)
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
        let focus = self
            .library
            .focus
            .get_or_insert_with(|| cx.focus_handle())
            .clone();
        // Opened from the rail, the keys still go to the hidden mail list
        // (or nowhere): the files take them.
        if window.focused(cx).is_none_or(|f| f == self.list_focus) {
            window.focus(&focus, cx);
        }
        div()
            .id("files-body")
            .track_focus(&focus)
            .on_key_down(cx.listener(Self::on_files_key))
            .size_full()
            .flex()
            .flex_col()
            .child(bar)
            .child(div().flex_1().min_h_0().child(content))
            .into_any_element()
    }

    /// The grid and list buttons, for the mail files and the drives.
    fn files_view_buttons(&self, th: &Theme, cx: &mut Context<Self>) -> gpui::Div {
        let grid_now = self.library.grid;
        let view_button = |id: &'static str, name: &'static str, grid: bool, label: String| {
            let on = grid_now == grid;
            icon_button(id, name, 20.0, th)
                .when(on, |d| d.bg(rgba(th.search)))
                .tooltip(tip(label, th))
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.library.grid = grid;
                    this.library.changed();
                    cx.notify();
                }))
        };
        div()
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
            ))
    }

    /// The person chip's label, and whether it narrows the files.
    fn files_person_label(&self) -> (String, bool) {
        let person = self.library.person.as_ref().map(|email| {
            self.library
                .senders
                .iter()
                .find(|s| s.email == *email)
                .map(|s| s.name.clone())
                .unwrap_or_else(|| email.clone())
        });
        match &person {
            Some(name) => (tr!("files-from-person", name = name.as_str()), true),
            None => (tr!("files-anyone"), false),
        }
    }

    /// The chip that narrows the files to one sender's: the page's and the
    /// attach picker's.
    fn files_person_chip(&self, th: &Theme, cx: &mut Context<Self>) -> gpui::Stateful<gpui::Div> {
        let (label, on) = self.files_person_label();
        filter_chip("files-people", label, on, th)
            .child(chip_arrow(on, th))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, e: &MouseDownEvent, _, cx| {
                    cx.stop_propagation();
                    this.library.menu = Some(Menu::People(e.position));
                    cx.notify();
                }),
            )
    }

    /// The chip that narrows the files to some days, with its calendar:
    /// the page's and the attach picker's.
    fn files_time_chip(&self, th: &Theme, cx: &mut Context<Self>) -> gpui::Stateful<gpui::Div> {
        let today = jiff::Timestamp::now().to_zoned(self.tz.clone()).date();
        let days = self.library.time != Time::Any;
        filter_chip("files-time", self.library.time.label(today), days, th)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, e: &MouseDownEvent, _, cx| {
                    cx.stop_propagation();
                    // The right month holds the last day picked, else today.
                    let last = match this.library.time {
                        Time::Days(_, last) => last,
                        Time::Any => today,
                    };
                    let month = last
                        .first_of_month()
                        .checked_sub(jiff::Span::new().months(1))
                        .unwrap_or(last.first_of_month());
                    // Under the chip, its right edge lined up with the
                    // chip's (the window's edge moves it if need be).
                    let at = this.library.time_chip.get().map_or(e.position, |b| {
                        point(b.right() - px(CALENDAR_WIDTH), b.bottom() + px(6.0))
                    });
                    this.library.menu = Some(Menu::Time {
                        at,
                        month,
                        anchor: None,
                    });
                    cx.notify();
                }),
            )
            .child({
                let chip = self.library.time_chip.clone();
                canvas(move |bounds, _, _| chip.set(Some(bounds)), |_, _, _, _| {})
                    .absolute()
                    .size_full()
            })
            // Over picked days the wheel moves them, keeping their length:
            // down to later days, up to earlier ones.
            .on_scroll_wheel(cx.listener(|this, e: &ScrollWheelEvent, _, cx| {
                if this.library.time == Time::Any {
                    return;
                }
                cx.stop_propagation();
                let step = this.library.wheel.turn(e.delta, std::time::Instant::now());
                if step != 0 {
                    this.library.time = this.library.time.shifted(step);
                    this.library.changed();
                    cx.notify();
                }
            }))
            .map(|d| {
                if days {
                    d.tooltip(tip(tr!("files-time-wheel"), th)).child(
                        div()
                            .id("files-time-clear")
                            .size(px(20.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded_full()
                            .hover(|s| s.bg(rgba(th.hover)))
                            .child(icon("close", th.nav_selected_text, 16.0))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _: &MouseDownEvent, _, cx| {
                                    cx.stop_propagation();
                                    this.library.time = Time::Any;
                                    this.library.changed();
                                    cx.notify();
                                }),
                            ),
                    )
                } else {
                    d.child(chip_arrow(false, th))
                }
            })
    }

    /// The chip for a kind of file, where the side column is not shown:
    /// a narrow page and the attach picker.
    fn files_kind_chip(
        &self,
        n: usize,
        types: Types,
        fold: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> gpui::Stateful<gpui::Div> {
        let on = self.library.types == types;
        // Folded to its icon, with its label as a tip; the picked kind
        // keeps its label.
        let folded = fold && !on;
        div()
            .id(("files-kind-chip", n))
            .flex_none()
            .h(px(32.0))
            .when(folded, |d| d.w(px(FOLDED_CHIP)).justify_center())
            .when(!folded, |d| d.px(px(12.0)))
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
            .map(|d| {
                if folded {
                    d.tooltip(tip(types.label(), th))
                } else {
                    d.child(div().whitespace_nowrap().child(types.label()))
                }
            })
            .on_click(cx.listener(move |this, _, _, cx| {
                this.library.types = types;
                this.library.changed();
                cx.notify();
            }))
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
                this.library.cursor = Some(ix);
                if let Some(focus) = &this.library.focus {
                    window.focus(focus, cx);
                }
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
                    .children(self.muted_mark(&found.file.from_email, 14.0, th))
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
        // The arrow keys' cursor: a ring over the card's edge.
        let card = card.when(self.library.cursor == Some(ix), |d| {
            d.child(
                div()
                    .absolute()
                    .inset_0()
                    .rounded(px(CARD_RADIUS))
                    .border_2()
                    .border_color(rgba(th.accent)),
            )
        });
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
            .when(self.library.cursor == Some(ix), |d| {
                d.bg(rgba(th.nav_selected))
            })
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
                        .child(div().min_w_0().truncate().child(sender.clone()))
                        .children(self.muted_mark(&found.file.from_email, 14.0, th)),
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
                        this.message_in_window(message, None, None, cx);
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
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(px(6.0))
                                    .child(div().min_w_0().truncate().child(sender.name.clone()))
                                    .children(self.muted_mark(&sender.email, 16.0, th)),
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
            Menu::Time { at, month, .. } => {
                let panel = self.render_time_calendar(month, th, cx);
                return Some(self.files_overlay(at, panel, cx));
            }
            Menu::Drive(ref menu) => self.drive_menu_items(menu, item, separator, cx)?,
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
            .into_any_element();
        Some(self.files_overlay(at, panel, cx))
    }

    /// `panel` over the page at `at`, closed by a click anywhere else.
    fn files_overlay(
        &self,
        at: Point<Pixels>,
        panel: AnyElement,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let close = || {
            cx.listener(|this: &mut Self, _: &MouseDownEvent, _, cx| {
                this.library.menu = None;
                cx.notify();
            })
        };
        let panel = div().child(panel).with_animation(
            "files-menu",
            Animation::new(std::time::Duration::from_millis(140)).with_easing(ease_out_quint()),
            |el, t| el.opacity(t).mt(px(-4.0 * (1.0 - t))),
        );
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
                        .on_mouse_down(MouseButton::Right, close())
                        // A drag across the calendar's days may end out here.
                        .on_mouse_up(MouseButton::Left, cx.listener(Self::end_day_drag)),
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
            .into_any_element()
    }

    /// The days picked so far, while the calendar is open.
    fn pick_days(&mut self, time: Time, cx: &mut Context<Self>) {
        if self.library.time != time {
            self.library.time = time;
            // Not `changed`, which would close the calendar.
            self.library.stale = true;
            if let Some(view) = self.library.cloud.view.as_mut() {
                view.stale_now();
            }
            cx.notify();
        }
    }

    /// The mouse let go after a press on a day: the pick is made.
    fn end_day_drag(&mut self, _: &MouseUpEvent, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(Menu::Time {
            anchor: Some(_), ..
        }) = self.library.menu
        {
            self.library.menu = None;
            cx.notify();
        }
    }

    /// The time chip's popover: quick picks over two months. A click on
    /// a day picks it, a drag picks the days it crosses, Shift+click
    /// stretches the days picked to the day clicked; the files behind
    /// follow while the mouse is down, and the popover closes on release.
    fn render_time_calendar(&self, month: Date, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let today = jiff::Timestamp::now().to_zoned(self.tz.clone()).date();
        let time = self.library.time;
        let picked = match time {
            Time::Days(first, last) => Some((first, last)),
            Time::Any => None,
        };
        let quick = Quick::ALL.into_iter().enumerate().filter_map(|(n, quick)| {
            let (first, last) = quick.days(today)?;
            let on = picked == Some((first, last));
            Some(
                div()
                    .id(("files-quick", n))
                    .h(px(28.0))
                    .px(px(8.0))
                    .flex()
                    .items_center()
                    .rounded(px(8.0))
                    .border_1()
                    .border_color(rgba(if on { th.nav_selected } else { th.divider }))
                    .when(on, |d| {
                        d.bg(rgba(th.nav_selected))
                            .text_color(rgba(th.nav_selected_text))
                    })
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .text_size(px(13.0))
                    .child(quick.label())
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.library.time = Time::Days(first, last);
                        this.library.changed();
                        cx.notify();
                    })),
            )
        });
        let turn = |id: &'static str, name: &'static str, by: i32, label: String| {
            icon_button(id, name, 18.0, th)
                .size(px(28.0))
                .tooltip(tip(label, th))
                .on_click(cx.listener(move |this, _, _, cx| {
                    if let Some(Menu::Time { month, .. }) = &mut this.library.menu
                        && let Ok(to) = month.checked_add(jiff::Span::new().months(by))
                    {
                        *month = to;
                    }
                    cx.notify();
                }))
        };
        let weekdays =
            || {
                div().flex().flex_row().children(
                    katna_i18n::format::weekdays_short()
                        .into_iter()
                        .map(|(_, name)| {
                            div()
                                .w(px(DAY))
                                .h(px(24.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(px(12.0))
                                .text_color(rgba(th.text_faint))
                                .child(name)
                        }),
                )
            };
        let shown = |shown: Date, side: usize| {
            let days = schedule::month_grid(shown, katna_i18n::format::first_weekday());
            let weeks = days.chunks(7).enumerate().map(|(week, days)| {
                div()
                    .h(px(ROW))
                    .flex()
                    .flex_row()
                    .children(days.iter().enumerate().map(|(n, &day)| {
                        let id = (side * 64 + week * 7 + n) as u64;
                        let cell = div().w(px(DAY)).h(px(ROW)).relative();
                        if day.month() != shown.month() {
                            return cell.into_any_element();
                        }
                        let end = picked.is_some_and(|(a, b)| day == a || day == b);
                        let inside = picked.is_some_and(|(a, b)| a <= day && day <= b);
                        let (start, stop) =
                            picked.map_or((false, false), |(a, b)| (day == a, day == b));
                        let (accent, hover) = (th.accent, th.hover);
                        cell.when(inside && !(start && stop), |d| {
                            d.child(
                                div()
                                    .absolute()
                                    .top(px(2.0))
                                    .bottom(px(2.0))
                                    .left(px(if start { DAY / 2.0 } else { 0.0 }))
                                    .right(px(if stop { DAY / 2.0 } else { 0.0 }))
                                    .bg(rgba(th.nav_selected)),
                            )
                        })
                        .child(
                            div()
                                .id(("files-day", id))
                                .absolute()
                                .top(px(2.0))
                                .left(px(3.0))
                                .size(px(30.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded_full()
                                .cursor_pointer()
                                .text_size(px(13.0))
                                .text_color(rgba(if end {
                                    th.on_accent
                                } else if day > today {
                                    th.text_faint
                                } else {
                                    th.text
                                }))
                                .when(end, |d| d.bg(rgba(th.accent)))
                                .when(day == today && !end, |d| {
                                    d.border_1().border_color(rgba(th.accent))
                                })
                                // Set once: GPUI panics on a second hover style.
                                .hover(move |s| s.bg(rgba(if end { accent } else { hover })))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, e: &MouseDownEvent, _, cx| {
                                        cx.stop_propagation();
                                        if e.modifiers.shift
                                            && let Time::Days(first, _) = this.library.time
                                        {
                                            // Stretches from the first day
                                            // picked; done on release.
                                            this.pick_days(Time::between(first, day), cx);
                                            if let Some(Menu::Time { anchor, .. }) =
                                                &mut this.library.menu
                                            {
                                                *anchor = Some(first);
                                            }
                                            return;
                                        }
                                        if let Some(Menu::Time { anchor, .. }) =
                                            &mut this.library.menu
                                        {
                                            *anchor = Some(day);
                                        }
                                        this.pick_days(Time::Days(day, day), cx);
                                    }),
                                )
                                .on_mouse_move(cx.listener(move |this, _, _, cx| {
                                    if let Some(Menu::Time {
                                        anchor: Some(from), ..
                                    }) = this.library.menu
                                    {
                                        this.pick_days(Time::between(from, day), cx);
                                    }
                                }))
                                .child(katna_i18n::format::number(day.day() as u64)),
                        )
                        .into_any_element()
                    }))
            });
            div()
                .w(px(7.0 * DAY))
                .flex()
                .flex_col()
                .child(
                    div()
                        .h(px(32.0))
                        .flex()
                        .flex_row()
                        .items_center()
                        .child(if side == 0 {
                            turn(
                                "files-month-back",
                                "chevron-left",
                                -1,
                                tr!("files-time-month-back"),
                            )
                            .into_any_element()
                        } else {
                            div().size(px(28.0)).into_any_element()
                        })
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .justify_center()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .child(katna_i18n::format::month_year(shown)),
                        )
                        .child(if side == 1 {
                            turn(
                                "files-month-on",
                                "chevron-right",
                                1,
                                tr!("files-time-month-on"),
                            )
                            .into_any_element()
                        } else {
                            div().size(px(28.0)).into_any_element()
                        }),
                )
                .child(weekdays())
                .children(weeks)
        };
        let next = month
            .checked_add(jiff::Span::new().months(1))
            .unwrap_or(month);
        let summary = match time {
            Time::Any => tr!("files-time-hint"),
            Time::Days(..) => tr!(
                "files-time-summary",
                days = time.label(today),
                count = self.library.shown.len()
            ),
        };
        div()
            .id("files-time-calendar")
            .w(px(CALENDAR_WIDTH))
            .p(px(PAD))
            .flex()
            .flex_col()
            .gap(px(12.0))
            .map(|d| raised(d, th, 12.0, 3.0))
            .text_color(rgba(th.text))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::end_day_drag))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(6.0))
                    .children(quick),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(BETWEEN))
                    .child(shown(month, 0))
                    .child(shown(next, 1)),
            )
            .child(div().h(px(1.0)).bg(rgba(th.divider)))
            .child(
                div()
                    .h(px(28.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .truncate()
                            .text_size(px(13.0))
                            .text_color(rgba(th.text_dim))
                            .child(summary),
                    )
                    .when(picked.is_some(), |d| {
                        d.child(
                            div()
                                .id("files-time-clear-all")
                                .h(px(28.0))
                                .px(px(10.0))
                                .flex()
                                .items_center()
                                .rounded(px(8.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(rgba(th.hover)))
                                .text_size(px(13.0))
                                .text_color(rgba(th.accent))
                                .child(tr!("files-time-clear"))
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.library.time = Time::Any;
                                    this.library.changed();
                                    cx.notify();
                                })),
                        )
                    }),
            )
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn times_keep_their_files() {
        use jiff::civil::date;
        let sep = Time::Days(date(2026, 9, 12), date(2026, 9, 28));
        assert!(sep.keeps(Some(date(2026, 9, 12))));
        assert!(sep.keeps(Some(date(2026, 9, 28))));
        assert!(!sep.keeps(Some(date(2026, 9, 29))));
        assert!(!sep.keeps(None));
        assert!(Time::Any.keeps(None));
        assert_eq!(Time::between(date(2026, 9, 28), date(2026, 9, 12)), sep);
    }

    #[test]
    fn the_wheel_moves_the_days_by_their_length() {
        use jiff::civil::date;
        let week = Time::Days(date(2026, 1, 5), date(2026, 1, 11));
        assert_eq!(
            week.shifted(1),
            Time::Days(date(2026, 1, 12), date(2026, 1, 18))
        );
        assert_eq!(
            week.shifted(-1),
            Time::Days(date(2025, 12, 29), date(2026, 1, 4))
        );
        // Whole months move by months.
        let feb = Time::Days(date(2026, 2, 1), date(2026, 2, 28));
        assert_eq!(
            feb.shifted(1),
            Time::Days(date(2026, 3, 1), date(2026, 3, 31))
        );
        let two = Time::Days(date(2026, 1, 1), date(2026, 2, 28));
        assert_eq!(
            two.shifted(1),
            Time::Days(date(2026, 3, 1), date(2026, 4, 30))
        );
        assert_eq!(Time::Any.shifted(1), Time::Any);
    }

    #[test]
    fn quick_picks_cover_their_days() {
        use jiff::civil::date;
        // A Thursday.
        let today = date(2026, 10, 1);
        assert_eq!(
            Quick::LastMonth.days(today),
            Some((date(2026, 9, 1), date(2026, 9, 30)))
        );
        assert_eq!(
            Quick::Yesterday.days(today),
            Some((date(2026, 9, 30), date(2026, 9, 30)))
        );
        let (first, last) = Quick::ThisWeek.days(today).unwrap();
        assert!(first <= today && today <= last);
        assert_eq!(first.until(last).unwrap().get_days(), 6);
        let (before, end) = Quick::LastWeek.days(today).unwrap();
        assert_eq!(end.tomorrow().unwrap(), first);
        assert_eq!(before.until(end).unwrap().get_days(), 6);
    }

    #[test]
    fn kinds_fall_into_the_side_column() {
        assert_eq!(Types::of(Kind::Pdf), Types::Pdfs);
        assert_eq!(Types::of(Kind::Text), Types::Documents);
        assert_eq!(Types::of(Kind::Other), Types::Other);
    }
}
