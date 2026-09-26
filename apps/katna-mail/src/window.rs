// SPDX-License-Identifier: GPL-3.0-or-later

//! The main window (`docs/ARCHITECTURE.md` §13.5): a top bar with the
//! search box in the middle, the navigation rail with Compose and the
//! folders, and one card that shows either the message list or the open
//! message. Motion comes from springs (`katna_ui::motion`) and click
//! ripples; all of it honors the desktop's reduce-motion setting.

use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::rc::Rc;
use std::time::Duration;

use gpui::{
    Animation, AnimationExt, AnyElement, App, BoxShadow, Context, Div, Entity, FocusHandle,
    Focusable, FontWeight, HighlightStyle, Hsla, KeyBinding, Render, ScrollHandle, ScrollStrategy,
    SharedString, SpringAnimation, Stateful, StyledText, Subscription, Task,
    UniformListScrollHandle, Window, actions, div, ease_out_quint, linear_color_stop,
    linear_gradient, point, prelude::*, px, rgba, svg, uniform_list,
};
use jiff::tz::TimeZone;
use katna_chrome::{Bar, Environment, WindowChrome};
use katna_core::{Account, Paths};
use katna_render::MessageView;
use katna_search::SearchResults;
use katna_store::{FolderId, MessageId};
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::{InputEvent, Ripple, TextInput};

use crate::data::{self, Mail, OpenError, Row};
use crate::format;
use crate::sidebar::{self, Role, Tree};
use crate::theme::{Theme, avatar_color, fade, initial, mix};

actions!(
    katna_mail,
    [
        SelectNext,
        SelectPrevious,
        SelectFirst,
        SelectLast,
        PageDown,
        PageUp,
        OpenMessage,
        CloseMessage,
        ScrollDown,
        ScrollUp,
        ScrollPageDown,
        ScrollPageUp,
        FocusSearch,
        FocusList,
        ToggleNavigation,
        Compose,
        Reload,
        Quit,
    ]
);

const WINDOW_CONTEXT: &str = "MailWindow";
const LIST_CONTEXT: &str = "MessageList";
const READER_CONTEXT: &str = "MessageReader";
const SEARCH_CONTEXT: &str = "SearchBox";

const TOP_BAR_HEIGHT: f32 = 64.0;
const NAV_WIDTH: f32 = 256.0;
const RAIL_WIDTH: f32 = 72.0;
const NAV_ROW_HEIGHT: f32 = 32.0;
const ROW_HEIGHT: f32 = 40.0;
const TOOLBAR_HEIGHT: f32 = 48.0;
const SEARCH_WIDTH: f32 = 720.0;
const COMPOSE_WIDTH: f32 = 142.0;
const PAGE: usize = 10;
/// Wait this long after a keystroke before searching, so fast typing
/// searches once.
const SEARCH_DELAY: Duration = Duration::from_millis(60);
/// Hover on the folded rail this long before it opens over the list.
const PEEK_DELAY: Duration = Duration::from_millis(300);
const SNACKBAR_TIME: Duration = Duration::from_secs(4);
/// The reading view shows at most this many lines of a body.
const MAX_BODY_LINES: usize = 4000;
const LINE_SCROLL: f32 = 48.0;
const NOT_YET: &str = "Katna Mail cannot send mail yet.";

/// Binds the window's keys. Call once at startup.
pub fn bind_keys(cx: &mut App) {
    let list = Some(LIST_CONTEXT);
    let reader = Some(READER_CONTEXT);
    let window = Some(WINDOW_CONTEXT);
    cx.bind_keys([
        KeyBinding::new("down", SelectNext, list),
        KeyBinding::new("j", SelectNext, list),
        KeyBinding::new("up", SelectPrevious, list),
        KeyBinding::new("k", SelectPrevious, list),
        KeyBinding::new("home", SelectFirst, list),
        KeyBinding::new("end", SelectLast, list),
        KeyBinding::new("pagedown", PageDown, list),
        KeyBinding::new("pageup", PageUp, list),
        KeyBinding::new("enter", OpenMessage, list),
        KeyBinding::new("o", OpenMessage, list),
        KeyBinding::new("j", SelectNext, reader),
        KeyBinding::new("k", SelectPrevious, reader),
        KeyBinding::new("u", CloseMessage, reader),
        KeyBinding::new("escape", CloseMessage, reader),
        KeyBinding::new("backspace", CloseMessage, reader),
        KeyBinding::new("down", ScrollDown, reader),
        KeyBinding::new("up", ScrollUp, reader),
        KeyBinding::new("pagedown", ScrollPageDown, reader),
        KeyBinding::new("space", ScrollPageDown, reader),
        KeyBinding::new("pageup", ScrollPageUp, reader),
        KeyBinding::new("shift-space", ScrollPageUp, reader),
        KeyBinding::new("/", FocusSearch, list),
        KeyBinding::new("/", FocusSearch, reader),
        KeyBinding::new("c", Compose, list),
        KeyBinding::new("c", Compose, reader),
        KeyBinding::new("ctrl-f", FocusSearch, window),
        KeyBinding::new("down", FocusList, Some(SEARCH_CONTEXT)),
        KeyBinding::new("f5", Reload, window),
        KeyBinding::new("ctrl-r", Reload, window),
        KeyBinding::new("ctrl-q", Quit, None),
    ]);
    katna_ui::text_input::bind_keys(cx);
}

/// What the message list shows.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Listing {
    Folder(FolderId),
    Search { query: String, total: Option<usize> },
}

/// The open message.
struct Reader {
    id: MessageId,
    /// `None` when the message body is not stored (not downloaded yet).
    view: Option<MessageView>,
    /// The body, in blocks of consecutive quoted or unquoted lines.
    blocks: Vec<(bool, SharedString)>,
    cut: bool,
    flagged: bool,
}

/// A short note at the bottom of the window.
struct Snackbar {
    text: SharedString,
    shown: Spring,
    _hide: Task<()>,
}

pub struct MailWindow {
    chrome: WindowChrome,
    /// The desktop's UI font, or `None` to leave GPUI's default.
    font: Option<SharedString>,
    paths: Paths,
    mail: Result<Mail, OpenError>,
    accounts: Vec<Account>,
    tree: Tree,
    /// Unread mail per folder, counted in the background.
    unread: HashMap<FolderId, u64>,
    unread_task: Option<Task<()>>,
    expanded: HashSet<String>,
    nav_rows: Vec<sidebar::Row>,
    folder: Option<FolderId>,
    show_recipients: bool,
    listing: Option<Listing>,
    ids: Vec<MessageId>,
    /// The list cursor.
    selected: Option<usize>,
    /// Rows of the list on screen at the last layout.
    visible: Range<usize>,
    hovered: Option<usize>,
    reader: Option<Reader>,
    /// Whether the card shows the open message instead of the list.
    reading: bool,
    /// Changes whenever the card switches content, to replay its fade-in.
    card_seq: usize,
    search: Entity<TextInput>,
    search_error: Option<SharedString>,
    search_task: Option<Task<()>>,
    /// The navigation is open (not folded to the rail).
    nav_open: bool,
    /// The folded rail is opened over the list while the pointer is on it.
    nav_peek: bool,
    peek_task: Option<Task<()>>,
    /// 0 = rail, 1 = open: the drawn navigation.
    nav_spring: Spring,
    /// 0 = rail, 1 = open: the space the navigation takes from the card.
    reserve_spring: Spring,
    search_spring: Spring,
    snackbar: Option<Snackbar>,
    /// Navigation openness at this frame, for the folder rows.
    nav_t: f32,
    list_focus: FocusHandle,
    list_scroll: UniformListScrollHandle,
    nav_scroll: UniformListScrollHandle,
    reader_scroll: ScrollHandle,
    tz: TimeZone,
    _subscriptions: Vec<Subscription>,
}

impl MailWindow {
    pub fn new(
        env: Environment,
        paths: Paths,
        font: Option<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let started = std::time::Instant::now();
        let search = cx.new(|cx| TextInput::new("Search mail", cx));
        let subscriptions = vec![cx.subscribe_in(&search, window, Self::on_search_event)];
        let mut this = Self {
            chrome: WindowChrome::new(env, "Katna Mail", window, cx),
            font,
            mail: Mail::open(&paths),
            accounts: Vec::new(),
            paths,
            tree: Tree::default(),
            unread: HashMap::new(),
            unread_task: None,
            expanded: HashSet::new(),
            nav_rows: Vec::new(),
            folder: None,
            show_recipients: false,
            listing: None,
            ids: Vec::new(),
            selected: None,
            visible: 0..0,
            hovered: None,
            reader: None,
            reading: false,
            card_seq: 0,
            search,
            search_error: None,
            search_task: None,
            nav_open: true,
            nav_peek: false,
            peek_task: None,
            nav_spring: Spring::new(motion::SLIDE, 1.0),
            reserve_spring: Spring::new(motion::SLIDE, 1.0),
            search_spring: Spring::new(motion::SMOOTH, 0.0),
            snackbar: None,
            nav_t: 1.0,
            list_focus: cx.focus_handle(),
            list_scroll: UniformListScrollHandle::new(),
            nav_scroll: UniformListScrollHandle::new(),
            reader_scroll: ScrollHandle::new(),
            tz: TimeZone::try_system().unwrap_or(TimeZone::UTC),
            _subscriptions: subscriptions,
        };
        this.load_tree();
        if let Some((folder, ancestors)) = this.tree.default_folder() {
            this.expanded.extend(ancestors);
            this.rebuild_nav();
            this.open_folder(folder, cx);
        }
        this.count_unread(cx);
        if let Some(err) = this.mail.as_ref().ok().and_then(Mail::index_error) {
            tracing::info!("{err}");
        }
        window.focus(&this.list_focus, cx);
        tracing::info!(elapsed = ?started.elapsed(), messages = this.ids.len(), "mail loaded");
        this
    }

    /// Puts `query` in the search box and searches.
    pub fn search_for(&mut self, query: String, window: &mut Window, cx: &mut Context<Self>) {
        window.focus(&self.search.focus_handle(cx), cx);
        self.search
            .update(cx, |search, cx| search.set_text(query, cx));
    }

    fn load_tree(&mut self) {
        let Ok(mail) = &self.mail else {
            return;
        };
        self.accounts = mail.accounts();
        self.tree = Tree::build(&self.accounts, &mail.folders(), &self.unread);
        self.expanded = self.tree.initially_expanded();
        self.rebuild_nav();
    }

    /// Counts unread mail in the background, then shows the counts.
    fn count_unread(&mut self, cx: &mut Context<Self>) {
        if self.mail.is_err() {
            return;
        }
        let paths = self.paths.clone();
        self.unread_task = Some(cx.spawn(async move |this, cx| {
            let unread = cx
                .background_executor()
                .spawn(async move { data::unread_counts(&paths) })
                .await;
            this.update(cx, |this, cx| {
                this.unread = unread;
                if let Ok(mail) = &this.mail {
                    this.tree = Tree::build(&this.accounts, &mail.folders(), &this.unread);
                    this.rebuild_nav();
                }
                cx.notify();
            })
            .ok();
        }));
    }

    /// With one account the folders stand alone, without an account heading.
    fn rebuild_nav(&mut self) {
        let mut rows = self.tree.rows(&self.expanded);
        let accounts = rows
            .iter()
            .filter(|r| matches!(r, sidebar::Row::Account { .. }))
            .count();
        if accounts == 1 {
            rows.retain(|r| !matches!(r, sidebar::Row::Account { .. }));
        }
        self.nav_rows = rows;
    }

    fn open_folder(&mut self, folder: FolderId, cx: &mut Context<Self>) {
        let Ok(mail) = &mut self.mail else {
            return;
        };
        let role = self.tree.node(folder).map_or(Role::Other, |n| n.role);
        if role.shows_recipients() != self.show_recipients {
            self.show_recipients = role.shows_recipients();
            mail.clear_rows();
        }
        self.folder = Some(folder);
        self.ids = mail.folder_message_ids(folder);
        self.listing = Some(Listing::Folder(folder));
        self.list_scroll.scroll_to_item(0, ScrollStrategy::Top);
        self.selected = (!self.ids.is_empty()).then_some(0);
        self.show_list();
        cx.notify();
    }

    fn show_list(&mut self) {
        if self.reading || self.reader.is_some() {
            self.card_seq += 1;
        }
        self.reading = false;
        self.reader = None;
        self.hovered = None;
    }

    /// Moves the list cursor; while reading, opens that message instead.
    fn select(&mut self, ix: usize, cx: &mut Context<Self>) {
        if ix >= self.ids.len() {
            return;
        }
        self.selected = Some(ix);
        self.list_scroll.scroll_to_item(ix, ScrollStrategy::Nearest);
        if self.reading {
            self.load_reader(ix);
        }
        cx.notify();
    }

    fn open(&mut self, ix: usize, window: &mut Window, cx: &mut Context<Self>) {
        if ix >= self.ids.len() {
            return;
        }
        self.selected = Some(ix);
        self.reading = true;
        self.load_reader(ix);
        window.focus(&self.list_focus, cx);
        cx.notify();
    }

    fn load_reader(&mut self, ix: usize) {
        let id = self.ids[ix];
        if self.reader.as_ref().is_none_or(|r| r.id != id) {
            self.reader = self.mail.as_mut().ok().map(|mail| {
                let mut reader = read(mail, id);
                reader.flagged = mail
                    .rows(&[id], false)
                    .into_iter()
                    .flatten()
                    .any(|row| row.flagged);
                reader
            });
            self.reader_scroll.set_offset(point(px(0.0), px(0.0)));
        }
    }

    fn move_selection(&mut self, delta: isize, cx: &mut Context<Self>) {
        if self.ids.is_empty() {
            return;
        }
        let last = self.ids.len() - 1;
        let ix = match self.selected {
            None if delta > 0 => 0,
            None => last,
            Some(ix) => ix.saturating_add_signed(delta).min(last),
        };
        self.select(ix, cx);
    }

    fn select_next(&mut self, _: &SelectNext, _: &mut Window, cx: &mut Context<Self>) {
        self.move_selection(1, cx);
    }

    fn select_previous(&mut self, _: &SelectPrevious, _: &mut Window, cx: &mut Context<Self>) {
        self.move_selection(-1, cx);
    }

    fn select_first(&mut self, _: &SelectFirst, _: &mut Window, cx: &mut Context<Self>) {
        self.select(0, cx);
    }

    fn select_last(&mut self, _: &SelectLast, _: &mut Window, cx: &mut Context<Self>) {
        self.select(self.ids.len().saturating_sub(1), cx);
    }

    fn page_down(&mut self, _: &PageDown, _: &mut Window, cx: &mut Context<Self>) {
        self.move_selection(PAGE as isize, cx);
    }

    fn page_up(&mut self, _: &PageUp, _: &mut Window, cx: &mut Context<Self>) {
        self.move_selection(-(PAGE as isize), cx);
    }

    fn open_message(&mut self, _: &OpenMessage, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ix) = self.selected {
            self.open(ix, window, cx);
        }
    }

    fn close_message(&mut self, _: &CloseMessage, window: &mut Window, cx: &mut Context<Self>) {
        self.show_list();
        window.focus(&self.list_focus, cx);
        cx.notify();
    }

    fn scroll_reader(&mut self, dy: f32, cx: &mut Context<Self>) {
        let offset = self.reader_scroll.offset();
        let max = self.reader_scroll.max_offset();
        let y = (offset.y - px(dy)).clamp(-max.y, px(0.0));
        self.reader_scroll.set_offset(point(offset.x, y));
        cx.notify();
    }

    fn reader_page(&self) -> f32 {
        (f32::from(self.reader_scroll.bounds().size.height) - LINE_SCROLL).max(LINE_SCROLL)
    }

    fn scroll_down(&mut self, _: &ScrollDown, _: &mut Window, cx: &mut Context<Self>) {
        self.scroll_reader(LINE_SCROLL, cx);
    }

    fn scroll_up(&mut self, _: &ScrollUp, _: &mut Window, cx: &mut Context<Self>) {
        self.scroll_reader(-LINE_SCROLL, cx);
    }

    fn scroll_page_down(&mut self, _: &ScrollPageDown, _: &mut Window, cx: &mut Context<Self>) {
        self.scroll_reader(self.reader_page(), cx);
    }

    fn scroll_page_up(&mut self, _: &ScrollPageUp, _: &mut Window, cx: &mut Context<Self>) {
        self.scroll_reader(-self.reader_page(), cx);
    }

    fn focus_search(&mut self, _: &FocusSearch, window: &mut Window, cx: &mut Context<Self>) {
        window.focus(&self.search.focus_handle(cx), cx);
        self.search.update(cx, |search, cx| {
            let text = search.text().to_owned();
            search.set_text(text, cx);
        });
    }

    fn focus_list(&mut self, _: &FocusList, window: &mut Window, cx: &mut Context<Self>) {
        window.focus(&self.list_focus, cx);
        if self.selected.is_none() && !self.ids.is_empty() {
            self.select(0, cx);
        }
    }

    fn toggle_navigation(&mut self, _: &ToggleNavigation, _: &mut Window, cx: &mut Context<Self>) {
        self.nav_open = !self.nav_open;
        self.nav_peek = false;
        self.peek_task = None;
        cx.notify();
    }

    fn compose(&mut self, _: &Compose, _: &mut Window, cx: &mut Context<Self>) {
        self.show_snackbar(NOT_YET, cx);
    }

    fn show_snackbar(&mut self, text: &str, cx: &mut Context<Self>) {
        let mut shown = Spring::new(motion::SLIDE, 0.0);
        shown.set(1.0);
        let hide = cx.spawn(async move |this, cx| {
            cx.background_executor().timer(SNACKBAR_TIME).await;
            this.update(cx, |this, cx| {
                if let Some(snackbar) = &mut this.snackbar {
                    snackbar.shown.set(0.0);
                    cx.notify();
                }
            })
            .ok();
        });
        self.snackbar = Some(Snackbar {
            text: text.to_owned().into(),
            shown,
            _hide: hide,
        });
        cx.notify();
    }

    fn hover_navigation(&mut self, hovered: bool, cx: &mut Context<Self>) {
        if self.nav_open {
            return;
        }
        if !hovered {
            self.peek_task = None;
            if self.nav_peek {
                self.nav_peek = false;
                cx.notify();
            }
            return;
        }
        if self.nav_peek || self.peek_task.is_some() {
            return;
        }
        self.peek_task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(PEEK_DELAY).await;
            this.update(cx, |this, cx| {
                this.peek_task = None;
                if !this.nav_open {
                    this.nav_peek = true;
                    cx.notify();
                }
            })
            .ok();
        }));
    }

    fn reload(&mut self, _: &Reload, _: &mut Window, cx: &mut Context<Self>) {
        if self.mail.is_err() {
            self.mail = Mail::open(&self.paths);
            self.load_tree();
            if let Some((folder, ancestors)) = self.tree.default_folder() {
                self.expanded.extend(ancestors);
                self.rebuild_nav();
                self.open_folder(folder, cx);
            }
            self.count_unread(cx);
            cx.notify();
            return;
        }
        let expanded = std::mem::take(&mut self.expanded);
        if let Ok(mail) = &mut self.mail {
            mail.refresh();
        }
        self.load_tree();
        self.expanded = expanded;
        self.rebuild_nav();
        let selected_id = self.selected.and_then(|ix| self.ids.get(ix).copied());
        match self.listing.clone() {
            Some(Listing::Folder(folder)) => {
                if let Ok(mail) = &self.mail {
                    self.ids = mail.folder_message_ids(folder);
                }
                self.selected = selected_id.and_then(|id| self.ids.iter().position(|i| *i == id));
                if self.selected.is_none() {
                    self.show_list();
                }
            }
            Some(Listing::Search { query, .. }) => self.start_search(query, cx),
            None => {}
        }
        self.card_seq += 1;
        self.count_unread(cx);
        cx.notify();
    }

    fn quit(&mut self, _: &Quit, _: &mut Window, cx: &mut Context<Self>) {
        cx.quit();
    }

    fn click_nav_row(&mut self, ix: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(sidebar::Row::Folder { key, folder, .. }) = self.nav_rows.get(ix).cloned() else {
            return;
        };
        match folder {
            Some(folder) => {
                self.clear_search(cx);
                if self.folder == Some(folder) && !self.reading {
                    self.card_seq += 1;
                }
                self.open_folder(folder, cx);
                window.focus(&self.list_focus, cx);
            }
            None => self.toggle(&key, cx),
        }
    }

    fn toggle(&mut self, key: &str, cx: &mut Context<Self>) {
        if !self.expanded.remove(key) {
            self.expanded.insert(key.to_owned());
        }
        self.rebuild_nav();
        cx.notify();
    }

    fn clear_search(&mut self, cx: &mut Context<Self>) {
        self.search_task = None;
        self.search_error = None;
        self.search.update(cx, |search, cx| {
            if !search.text().is_empty() {
                search.set_text("", cx);
            }
        });
    }

    fn on_search_event(
        &mut self,
        search: &Entity<TextInput>,
        event: &InputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            InputEvent::Changed => {
                let text = search.read(cx).text().trim().to_owned();
                self.start_search(text, cx);
            }
            InputEvent::Submit => self.focus_list(&FocusList, window, cx),
            InputEvent::Cancel => {
                if search.read(cx).text().is_empty() {
                    window.focus(&self.list_focus, cx);
                } else {
                    self.clear_search(cx);
                }
            }
        }
    }

    fn start_search(&mut self, text: String, cx: &mut Context<Self>) {
        self.search_error = None;
        if text.is_empty() {
            self.search_task = None;
            if matches!(self.listing, Some(Listing::Search { .. }))
                && let Some(folder) = self.folder
            {
                self.open_folder(folder, cx);
            }
            return;
        }
        let Some(index) = self.mail.as_mut().ok().and_then(Mail::index) else {
            self.search_error = self
                .mail
                .as_ref()
                .ok()
                .and_then(|m| m.index_error().map(SharedString::from));
            cx.notify();
            return;
        };
        self.search_task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(SEARCH_DELAY).await;
            let now = jiff::Timestamp::now().as_second();
            let query = text.clone();
            let results = cx
                .background_executor()
                .spawn(async move { data::search(&index, &query, now) })
                .await;
            this.update(cx, |this, cx| this.show_results(text, results, cx))
                .ok();
        }));
    }

    fn show_results(
        &mut self,
        query: String,
        results: Result<SearchResults, String>,
        cx: &mut Context<Self>,
    ) {
        match results {
            Ok(results) => {
                // Search results mix folders; show senders.
                if self.show_recipients {
                    self.show_recipients = false;
                    if let Ok(mail) = &mut self.mail {
                        mail.clear_rows();
                    }
                }
                let first = !matches!(self.listing, Some(Listing::Search { .. }));
                self.ids = results.hits.iter().map(|hit| hit.message).collect();
                self.listing = Some(Listing::Search {
                    query,
                    total: results.total,
                });
                self.selected = (!self.ids.is_empty()).then_some(0);
                self.list_scroll.scroll_to_item(0, ScrollStrategy::Top);
                if first {
                    self.card_seq += 1;
                }
                self.show_list();
            }
            Err(err) => self.search_error = Some(err.into()),
        }
        cx.notify();
    }

    fn folder_name(&self) -> Option<String> {
        match &self.listing {
            Some(Listing::Folder(folder)) => self.tree.node(*folder).map(|n| n.name.clone()),
            _ => None,
        }
    }

    // Top bar

    fn render_top_start(&self, th: &Theme, wide: bool, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let menu = icon_button("menu-button", "menu", 24.0, th)
            .ml(px(2.0))
            .on_click(cx.listener(|this, _, window, cx| {
                this.toggle_navigation(&ToggleNavigation, window, cx)
            }))
            .into_any_element();
        let logo = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(10.0))
            .pl(px(4.0))
            .child(
                div()
                    .size(px(34.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(9.0))
                    .bg(linear_gradient(
                        135.0,
                        linear_color_stop(rgba(0x4f8df7ff), 0.0),
                        linear_color_stop(rgba(0x3949c9ff), 1.0),
                    ))
                    .child(icon("mail", 0xffffffff, 22.0)),
            )
            .when(wide, |d| {
                d.child(
                    div()
                        .text_size(px(21.0))
                        .text_color(rgba(th.text_dim))
                        .child("Katna Mail"),
                )
            })
            .into_any_element();
        vec![menu, logo]
    }

    fn render_search(&self, th: &Theme, width: f32, t: f32, cx: &mut Context<Self>) -> AnyElement {
        let available = self.mail.as_ref().is_ok_and(Mail::has_index);
        let has_text = !self.search.read(cx).text().is_empty();
        div()
            .id("search-box")
            .key_context(SEARCH_CONTEXT)
            .w(px(width))
            .h(px(48.0))
            .pl(px(4.0))
            .pr(px(8.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .rounded_full()
            .bg(rgba(mix(th.search, th.search_focused, t)))
            .shadow(elevation(th, 2.0 * t))
            .text_size(px(16.0))
            .line_height(px(24.0))
            .text_color(rgba(th.text))
            .when(!available, |d| d.opacity(0.6))
            // A drag here selects text rather than moving the window.
            .on_mouse_move(|_, _, cx| cx.stop_propagation())
            .child(
                icon_button("search-button", "search", 22.0, th).on_click(cx.listener(
                    |this, _, window, cx| {
                        let text = this.search.read(cx).text().trim().to_owned();
                        if text.is_empty() {
                            this.focus_search(&FocusSearch, window, cx);
                        } else {
                            this.start_search(text, cx);
                        }
                    },
                )),
            )
            .child(div().flex_1().min_w_0().child(self.search.clone()))
            .when(has_text, |d| {
                d.child(
                    icon_button("search-clear", "close", 22.0, th).on_click(cx.listener(
                        |this, _, window, cx| {
                            this.clear_search(cx);
                            this.focus_search(&FocusSearch, window, cx);
                        },
                    )),
                )
            })
            .into_any_element()
    }

    fn render_account(&self, th: &Theme) -> Vec<AnyElement> {
        let Some(account) = self.accounts.first() else {
            return Vec::new();
        };
        let name = if account.display_name.trim().is_empty() {
            &account.address
        } else {
            &account.display_name
        };
        vec![
            div()
                .id("account")
                .mr(px(8.0))
                .p(px(4.0))
                .rounded_full()
                .hover(|s| s.bg(rgba(th.hover)))
                .on_mouse_move(|_, _, cx| cx.stop_propagation())
                .child(avatar(name, &account.address, 32.0))
                .into_any_element(),
        ]
    }

    // Navigation

    fn render_navigation(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let t = self.nav_t;
        let reserve = self.reserve_spring.value();
        // How far the panel is open beyond the space it takes: it floats.
        let float = (t - reserve).clamp(0.0, 1.0);
        let compose = div()
            .id("compose")
            .relative()
            .overflow_hidden()
            .ml(px(8.0))
            .h(px(56.0))
            .w(px(lerp(56.0, COMPOSE_WIDTH, t)))
            .flex_none()
            .flex()
            .flex_row()
            .items_center()
            .rounded(px(16.0))
            .bg(rgba(th.compose))
            .text_color(rgba(th.compose_text))
            .hover(|s| s.shadow(elevation(th, 1.5)))
            .cursor_pointer()
            .on_click(cx.listener(|this, _, window, cx| this.compose(&Compose, window, cx)))
            .child(Ripple::new("compose-ripple", rgba(th.ripple)))
            .child(
                div()
                    .pl(px(16.0))
                    .child(icon("compose", th.compose_text, 24.0)),
            )
            .child(
                div()
                    .pl(px(12.0))
                    .text_size(px(14.0))
                    .font_weight(FontWeight::MEDIUM)
                    .opacity(t)
                    .child("Compose"),
            );
        let list = uniform_list(
            "navigation",
            self.nav_rows.len(),
            cx.processor(|this, range: Range<usize>, window, cx| {
                let th = Theme::new(this.chrome.tokens(window).dark);
                range
                    .map(|ix| this.render_nav_row(ix, &th, cx))
                    .collect::<Vec<_>>()
            }),
        )
        .track_scroll(&self.nav_scroll)
        .flex_1()
        .pb(px(16.0));
        let panel = div()
            .id("navigation-panel")
            .absolute()
            .top_0()
            .left_0()
            .bottom_0()
            .w(px(lerp(RAIL_WIDTH, NAV_WIDTH, t)))
            .flex()
            .flex_col()
            .gap(px(16.0))
            .pt(px(8.0))
            .overflow_hidden()
            .bg(rgba(if float > 0.0 { th.surface } else { th.page }))
            .when(float > 0.0, |d| {
                d.rounded_r(px(16.0)).shadow(elevation(th, 3.0 * float))
            })
            .on_hover(
                cx.listener(|this, hovered: &bool, _, cx| this.hover_navigation(*hovered, cx)),
            )
            .child(compose)
            .child(list);
        div()
            .relative()
            .flex_none()
            .h_full()
            .w(px(lerp(RAIL_WIDTH, NAV_WIDTH, reserve)))
            .child(panel)
            .into_any_element()
    }

    fn render_nav_row(&self, ix: usize, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let t = self.nav_t;
        match &self.nav_rows[ix] {
            sidebar::Row::Account { name, .. } => div()
                .id(("nav-row", ix))
                .h(px(NAV_ROW_HEIGHT))
                .flex()
                .items_end()
                .pb(px(4.0))
                .pl(px(26.0))
                .text_size(px(12.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(th.text_faint))
                .opacity(t)
                .child(div().truncate().child(name.clone()))
                .into_any_element(),
            sidebar::Row::Folder {
                key,
                depth,
                label,
                role,
                folder,
                unread,
                has_children,
                expanded,
            } => {
                let selected = folder.is_some_and(|f| self.listing == Some(Listing::Folder(f)));
                let key = key.clone();
                let indent = 12.0 * *depth as f32 * t;
                let text = if selected {
                    th.nav_selected_text
                } else {
                    th.text
                };
                let bold = selected || *unread > 0;
                let chevron = div()
                    .id(("nav-chevron", ix))
                    .absolute()
                    .left(px(lerp(-10.0, 4.0 + indent, t)))
                    .top(px(6.0))
                    .size(px(20.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .opacity(t)
                    .hover(|s| s.bg(rgba(th.hover)))
                    .child(icon(
                        if *expanded {
                            "chevron-down"
                        } else {
                            "chevron-right"
                        },
                        th.text_dim,
                        16.0,
                    ))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        cx.stop_propagation();
                        this.toggle(&key, cx);
                    }));
                let row = div()
                    .id(("nav-row", ix))
                    .relative()
                    .overflow_hidden()
                    .h(px(NAV_ROW_HEIGHT))
                    .ml(px(lerp(8.0, 0.0, t)))
                    .w(px(lerp(56.0, NAV_WIDTH - 16.0, t)))
                    .pl(px(lerp(18.0, 26.0, t) + indent))
                    .pr(px(12.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .rounded_l(px(lerp(16.0, 0.0, t)))
                    .rounded_r(px(16.0))
                    .text_size(px(14.0))
                    .text_color(rgba(text))
                    .when(bold, |d| d.font_weight(FontWeight::BOLD))
                    .when(!selected, |d| d.hover(|s| s.bg(rgba(th.hover))))
                    .cursor_pointer()
                    .on_click(
                        cx.listener(move |this, _, window, cx| this.click_nav_row(ix, window, cx)),
                    )
                    .child(Ripple::new(("nav-ripple", ix), rgba(th.ripple)))
                    .child(
                        div()
                            .relative()
                            .child(icon(role_icon(*role), text, 20.0))
                            .when(*unread > 0 && t < 1.0, |d| {
                                d.child(
                                    div()
                                        .absolute()
                                        .top(px(-2.0))
                                        .right(px(-3.0))
                                        .size(px(8.0))
                                        .rounded_full()
                                        .border_1()
                                        .border_color(rgba(th.page))
                                        .bg(rgba(th.accent))
                                        .opacity(1.0 - t),
                                )
                            }),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .pl(px(18.0))
                            .truncate()
                            .opacity(t)
                            .child(label.clone()),
                    )
                    .when(*unread > 0, |d| {
                        d.child(
                            div()
                                .flex_none()
                                .pl(px(8.0))
                                .text_size(px(12.0))
                                .opacity(t)
                                .child(format::thousands(*unread)),
                        )
                    })
                    .when(*has_children, |d| d.child(chevron));
                row.with_spring(
                    ("nav-selected", ix),
                    SpringAnimation::new(motion::SMOOTH).to(if selected { 1.0 } else { 0.0 }),
                    {
                        let bg = th.nav_selected;
                        move |row, s: f32| {
                            if s > 0.001 {
                                row.bg(rgba(fade(bg, s)))
                            } else {
                                row
                            }
                        }
                    },
                )
                .into_any_element()
            }
        }
    }

    // The card: list or open message

    fn render_list_toolbar(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let count = self.ids.len();
        let label: SharedString = match &self.listing {
            Some(Listing::Search { query, .. }) => format!("Results for “{query}”").into(),
            _ => self.folder_name().unwrap_or_default().into(),
        };
        let range = if count == 0 {
            String::new()
        } else {
            let start = self.visible.start.min(count - 1) + 1;
            let end = self.visible.end.clamp(start, count);
            let total = match &self.listing {
                Some(Listing::Search {
                    total: Some(total), ..
                }) if *total > count => format!("about {}", format::thousands(*total as u64)),
                _ => format::thousands(count as u64),
            };
            format!(
                "{}–{} of {total}",
                format::thousands(start as u64),
                format::thousands(end as u64)
            )
        };
        let at_top = self.visible.start == 0;
        let at_end = self.visible.end >= count;
        toolbar(th)
            .child(
                icon_button("refresh", "refresh", 20.0, th)
                    .on_click(cx.listener(|this, _, window, cx| this.reload(&Reload, window, cx))),
            )
            .child(
                div()
                    .pl(px(8.0))
                    .flex_1()
                    .min_w_0()
                    .truncate()
                    .text_size(px(14.0))
                    .text_color(rgba(th.text_dim))
                    .child(self.search_error.clone().unwrap_or(label)),
            )
            .child(
                div()
                    .flex_none()
                    .px(px(8.0))
                    .text_size(px(12.0))
                    .text_color(rgba(th.text_faint))
                    .child(range),
            )
            .child(
                icon_button("page-up", "chevron-left", 20.0, th)
                    .when(at_top, |d| d.opacity(0.4))
                    .on_click(cx.listener(|this, _, _, cx| {
                        let page = this.visible.len().max(1);
                        let ix = this.visible.start.saturating_sub(page);
                        this.list_scroll.scroll_to_item(ix, ScrollStrategy::Top);
                        cx.notify();
                    })),
            )
            .child(
                icon_button("page-down", "chevron-right", 20.0, th)
                    .when(at_end, |d| d.opacity(0.4))
                    .on_click(cx.listener(|this, _, _, cx| {
                        let ix = this.visible.end.min(this.ids.len().saturating_sub(1));
                        this.list_scroll.scroll_to_item(ix, ScrollStrategy::Top);
                        cx.notify();
                    })),
            )
            .into_any_element()
    }

    fn render_list(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        if self.ids.is_empty() {
            let text = match &self.listing {
                Some(Listing::Search { .. }) => "No messages matched your search.".to_owned(),
                Some(Listing::Folder(_)) => format!(
                    "No messages in {}.",
                    self.folder_name().unwrap_or_else(|| "this folder".into())
                ),
                None => String::new(),
            };
            return placeholder(&text, th);
        }
        uniform_list(
            "messages",
            self.ids.len(),
            cx.processor(|this, range: Range<usize>, window, cx| {
                if this.visible != range {
                    this.visible = range.clone();
                    cx.notify();
                }
                let th = Theme::new(this.chrome.tokens(window).dark);
                let rows = match &mut this.mail {
                    Ok(mail) => mail.rows(&this.ids[range.clone()], this.show_recipients),
                    Err(_) => vec![None; range.len()],
                };
                let wide = f32::from(window.viewport_size().width) > 1000.0;
                range
                    .zip(rows)
                    .map(|(ix, row)| this.render_row(ix, row, &th, wide, cx))
                    .collect::<Vec<_>>()
            }),
        )
        .track_scroll(&self.list_scroll)
        .size_full()
        .into_any_element()
    }

    fn render_row(
        &self,
        ix: usize,
        row: Option<Rc<Row>>,
        th: &Theme,
        wide: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let hovered = self.hovered == Some(ix);
        let under_hovered = ix > 0 && self.hovered == Some(ix - 1);
        let cursor = self.selected == Some(ix);
        let unread = row.as_ref().is_some_and(|r| r.unread);
        let base = div()
            .id(("row", ix))
            .relative()
            .w_full()
            .h(px(ROW_HEIGHT))
            .flex()
            .flex_row()
            .items_center()
            .bg(rgba(if unread { th.surface } else { th.read_row }))
            .border_b_1()
            .border_color(rgba(th.divider))
            .text_size(px(14.0))
            .cursor_pointer()
            .on_hover(cx.listener(move |this, hovered: &bool, _, cx| {
                if *hovered {
                    this.hovered = Some(ix);
                } else if this.hovered == Some(ix) {
                    this.hovered = None;
                }
                cx.notify();
            }))
            .on_click(cx.listener(move |this, _, window, cx| this.open(ix, window, cx)))
            .child(Ripple::new(("row-ripple", ix), rgba(th.ripple)))
            // The shadow of the lifted row above, which this row would
            // otherwise paint over.
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .right_0()
                    .h(px(5.0))
                    .with_spring(
                        ("row-drop", ix),
                        SpringAnimation::new(motion::QUICK).to(if under_hovered {
                            1.0
                        } else {
                            0.0
                        }),
                        {
                            let shadow = th.shadow;
                            move |el, s: f32| {
                                el.bg(linear_gradient(
                                    180.0,
                                    linear_color_stop(rgba(fade(shadow, 0.55 * s)), 0.0),
                                    linear_color_stop(rgba(fade(shadow, 0.0)), 1.0),
                                ))
                            }
                        },
                    ),
            )
            // The keyboard cursor: a bar that grows from the middle.
            .child(
                div()
                    .absolute()
                    .left_0()
                    .w(px(3.0))
                    .rounded_r(px(2.0))
                    .bg(rgba(th.accent))
                    .with_spring(
                        ("row-cursor", ix),
                        SpringAnimation::new(motion::SLIDE).to(if cursor { 1.0 } else { 0.0 }),
                        move |el, s: f32| {
                            let s = s.clamp(0.0, 1.0);
                            el.top(px(ROW_HEIGHT / 2.0 * (1.0 - s)))
                                .h(px(ROW_HEIGHT * s))
                        },
                    ),
            );
        let lifted = |base: Stateful<Div>| {
            let shadow = th.shadow;
            base.with_spring(
                ("row-lift", ix),
                SpringAnimation::new(motion::QUICK).to(if hovered { 1.0 } else { 0.0 }),
                move |el, s: f32| {
                    if s > 0.001 {
                        el.shadow(vec![
                            BoxShadow {
                                color: rgba(fade(shadow, 0.9 * s)).into(),
                                offset: point(px(0.0), px(1.0)),
                                blur_radius: px(2.0),
                                spread_radius: px(0.0),
                                inset: false,
                            },
                            BoxShadow {
                                color: rgba(fade(shadow, 0.45 * s)).into(),
                                offset: point(px(0.0), px(1.0)),
                                blur_radius: px(3.0),
                                spread_radius: px(1.0),
                                inset: false,
                            },
                        ])
                    } else {
                        el
                    }
                },
            )
            .into_any_element()
        };
        let Some(row) = row else {
            return lifted(
                base.pl(px(56.0))
                    .text_color(rgba(th.text_faint))
                    .child("This message was removed."),
            );
        };
        let now = jiff::Timestamp::now().as_second();
        let date = row
            .date
            .and_then(|d| format::local(d, &self.tz))
            .zip(format::local(now, &self.tz))
            .map(|(d, now)| format::list_date(d, now))
            .unwrap_or_default();
        let weight = if row.unread {
            FontWeight::BOLD
        } else {
            FontWeight::NORMAL
        };
        let mut line = row.subject.clone();
        let subject_end = line.len();
        if !row.snippet.is_empty() {
            line.push_str(" - ");
            line.push_str(&row.snippet);
        }
        let line_len = line.len();
        let text = StyledText::new(line).with_highlights(vec![
            (
                0..subject_end,
                HighlightStyle {
                    color: Some(rgba(th.text).into()),
                    font_weight: Some(weight),
                    ..Default::default()
                },
            ),
            (
                subject_end..line_len,
                HighlightStyle {
                    color: Some(rgba(th.text_faint).into()),
                    ..Default::default()
                },
            ),
        ]);
        lifted(
            base.child(div().w(px(52.0)).flex_none().flex().justify_center().child(
                if row.flagged {
                    icon("star-filled", th.star, 20.0)
                } else {
                    icon("star", th.text_faint, 20.0)
                },
            ))
            .child(
                div()
                    .w(px(if wide { 200.0 } else { 140.0 }))
                    .flex_none()
                    .pr(px(24.0))
                    .truncate()
                    .font_weight(weight)
                    .text_color(rgba(th.text))
                    .child(row.correspondent.clone()),
            )
            .child(div().flex_1().min_w_0().truncate().child(text))
            .when(row.attachments, |d| {
                d.child(
                    div()
                        .pl(px(8.0))
                        .child(icon("attachment", th.text_faint, 18.0)),
                )
            })
            .child(
                div()
                    .flex_none()
                    .min_w(px(80.0))
                    .pl(px(16.0))
                    .pr(px(16.0))
                    .flex()
                    .justify_end()
                    .text_size(px(12.0))
                    .font_weight(weight)
                    .text_color(rgba(if row.unread { th.text } else { th.text_faint }))
                    .child(date),
            ),
        )
    }

    fn render_reader_toolbar(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let count = self.ids.len();
        let ix = self.selected.unwrap_or(0);
        toolbar(th)
            .child(icon_button("back", "back", 20.0, th).on_click(
                cx.listener(|this, _, window, cx| this.close_message(&CloseMessage, window, cx)),
            ))
            .child(div().flex_1())
            .child(
                div()
                    .px(px(8.0))
                    .text_size(px(12.0))
                    .text_color(rgba(th.text_faint))
                    .child(format!(
                        "{} of {}",
                        format::thousands(ix as u64 + 1),
                        format::thousands(count as u64)
                    )),
            )
            .child(
                icon_button("newer", "chevron-left", 20.0, th)
                    .when(ix == 0, |d| d.opacity(0.4))
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.select_previous(&SelectPrevious, window, cx)
                    })),
            )
            .child(
                icon_button("older", "chevron-right", 20.0, th)
                    .when(ix + 1 >= count, |d| d.opacity(0.4))
                    .on_click(
                        cx.listener(|this, _, window, cx| {
                            this.select_next(&SelectNext, window, cx)
                        }),
                    ),
            )
            .into_any_element()
    }

    fn render_reader(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(reader) = &self.reader else {
            return placeholder("", th);
        };
        let Some(view) = &reader.view else {
            return placeholder("This message has not been downloaded yet.", th);
        };
        let dim = rgba(th.text_faint);
        let subject = if view.subject.is_empty() {
            "(no subject)".to_owned()
        } else {
            view.subject.clone()
        };
        let sender = view.from.first();
        let (sender_name, sender_email) = sender.map_or_else(
            || ("(unknown sender)".to_owned(), String::new()),
            |a| (a.label().to_owned(), a.email.clone()),
        );
        let names = |list: &[katna_render::Address]| {
            list.iter()
                .map(|a| a.label().to_owned())
                .collect::<Vec<_>>()
                .join(", ")
        };
        let mut recipients = String::new();
        if !view.to.is_empty() {
            recipients = format!("to {}", names(&view.to));
        }
        if !view.cc.is_empty() {
            if !recipients.is_empty() {
                recipients.push_str(", ");
            }
            recipients.push_str(&format!("cc {}", names(&view.cc)));
        }
        let now = jiff::Timestamp::now().as_second();
        let date = view
            .date
            .and_then(|d| {
                let long = format::long_date(format::local(d, &self.tz)?);
                Some(match format::ago(d, now) {
                    Some(ago) => format!("{long} ({ago})"),
                    None => long,
                })
            })
            .unwrap_or_default();
        let flagged = reader.flagged;

        let title = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(12.0))
            .pl(px(72.0))
            .pr(px(24.0))
            .pt(px(20.0))
            .pb(px(16.0))
            .child(
                div()
                    .text_size(px(22.0))
                    .line_height(px(28.0))
                    .text_color(rgba(th.text))
                    .child(subject),
            )
            .when_some(self.folder_name(), |d, folder| {
                d.child(
                    div()
                        .px(px(6.0))
                        .py(px(1.0))
                        .rounded(px(4.0))
                        .bg(rgba(th.chip))
                        .text_size(px(12.0))
                        .text_color(rgba(th.text_dim))
                        .child(folder),
                )
            });
        let header = div()
            .flex()
            .flex_row()
            .items_start()
            .gap(px(8.0))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_baseline()
                            .gap(px(6.0))
                            .child(
                                div()
                                    .flex_none()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgba(th.text))
                                    .child(sender_name.clone()),
                            )
                            .when(sender.is_some_and(|a| a.name.is_some()), |d| {
                                d.child(
                                    div()
                                        .min_w_0()
                                        .truncate()
                                        .text_size(px(12.0))
                                        .text_color(dim)
                                        .child(format!("<{sender_email}>")),
                                )
                            }),
                    )
                    .when(!recipients.is_empty(), |d| {
                        d.child(
                            div()
                                .text_size(px(12.0))
                                .text_color(dim)
                                .truncate()
                                .child(recipients),
                        )
                    }),
            )
            .child(
                div()
                    .flex_none()
                    .text_size(px(12.0))
                    .text_color(dim)
                    .child(date),
            )
            .child(if flagged {
                icon("star-filled", th.star, 20.0)
            } else {
                icon("star", th.text_faint, 20.0)
            });

        let notes = [
            view.from_html
                .then_some("This message is HTML; it is shown as plain text for now."),
            reader
                .cut
                .then_some("The message is too long to show in full."),
        ];
        let body = div()
            .flex()
            .flex_col()
            .pt(px(20.0))
            .text_size(px(14.0))
            .line_height(px(21.0))
            .text_color(rgba(th.text))
            .children(notes.into_iter().flatten().map(|note| {
                div()
                    .mb(px(12.0))
                    .px(px(12.0))
                    .py(px(8.0))
                    .rounded(px(8.0))
                    .bg(rgba(th.read_row))
                    .text_size(px(12.0))
                    .text_color(rgba(th.text_dim))
                    .child(note)
            }))
            .children(reader.blocks.iter().map(|(quoted, text)| {
                div()
                    .when(*quoted, |d| {
                        d.pl(px(12.0))
                            .border_l_2()
                            .border_color(rgba(th.divider))
                            .text_color(dim)
                    })
                    .child(text.clone())
            }));
        let attachments = (!view.attachments.is_empty()).then(|| {
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .gap(px(12.0))
                .pt(px(20.0))
                .mt(px(20.0))
                .border_t_1()
                .border_color(rgba(th.divider))
                .children(view.attachments.iter().map(|a| {
                    div()
                        .w(px(200.0))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(10.0))
                        .px(px(12.0))
                        .py(px(10.0))
                        .rounded(px(8.0))
                        .border_1()
                        .border_color(rgba(th.divider))
                        .child(icon("attachment", th.text_faint, 20.0))
                        .child(
                            div()
                                .min_w_0()
                                .flex()
                                .flex_col()
                                .child(
                                    div()
                                        .truncate()
                                        .text_size(px(13.0))
                                        .text_color(rgba(th.text))
                                        .child(a.name.clone()),
                                )
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(dim)
                                        .child(format::size(a.size)),
                                ),
                        )
                }))
        });
        let actions = div()
            .flex()
            .flex_row()
            .gap(px(12.0))
            .pt(px(28.0))
            .pb(px(32.0))
            .child(pill_button("reply", "reply", "Reply", th, cx))
            .child(pill_button("forward", "forward", "Forward", th, cx));

        let message = div()
            .flex()
            .flex_row()
            .pr(px(24.0))
            .child(
                div()
                    .w(px(72.0))
                    .flex_none()
                    .flex()
                    .justify_center()
                    .child(avatar(&sender_name, &sender_email, 40.0)),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .max_w(px(960.0))
                    .child(header)
                    .child(body)
                    .children(attachments)
                    .child(actions),
            );
        div()
            .id("reader")
            .size_full()
            .overflow_y_scroll()
            .track_scroll(&self.reader_scroll)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(title)
                    .child(message)
                    .with_animation(
                        ("open-message", reader.id.0 as usize),
                        Animation::new(Duration::from_millis(280)).with_easing(ease_out_quint()),
                        |el, t| el.opacity(t).mt(px(14.0 * (1.0 - t))),
                    ),
            )
            .into_any_element()
    }

    fn render_card(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let (toolbar, body) = if self.reading {
            (
                self.render_reader_toolbar(th, cx),
                self.render_reader(th, cx),
            )
        } else {
            (self.render_list_toolbar(th, cx), self.render_list(th, cx))
        };
        let card = div()
            .id("card")
            .key_context(if self.reading {
                READER_CONTEXT
            } else {
                LIST_CONTEXT
            })
            .track_focus(&self.list_focus)
            .size_full()
            .flex()
            .flex_col()
            .rounded(px(16.0))
            .overflow_hidden()
            .bg(rgba(th.surface))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::page_down))
            .on_action(cx.listener(Self::page_up))
            .on_action(cx.listener(Self::open_message))
            .on_action(cx.listener(Self::close_message))
            .on_action(cx.listener(Self::scroll_down))
            .on_action(cx.listener(Self::scroll_up))
            .on_action(cx.listener(Self::scroll_page_down))
            .on_action(cx.listener(Self::scroll_page_up))
            .child(toolbar)
            .child(div().flex_1().min_h_0().child(body).with_animation(
                ("card", self.card_seq),
                Animation::new(Duration::from_millis(220)).with_easing(ease_out_quint()),
                |el, t| el.opacity(t),
            ));
        // Padding, not margins: a margin would push the card past the window.
        div()
            .flex_1()
            .min_w_0()
            .h_full()
            .pr(px(16.0))
            .pb(px(16.0))
            .child(card)
            .into_any_element()
    }

    fn render_snackbar(&mut self, th: &Theme, window: &Window, reduce: bool) -> Option<AnyElement> {
        let snackbar = self.snackbar.as_mut()?;
        let s = snackbar.shown.tick(window, reduce);
        if snackbar.shown.target() == 0.0 && snackbar.shown.settled() {
            self.snackbar = None;
            return None;
        }
        let s = s.max(0.0);
        Some(
            div()
                .absolute()
                .left(px(24.0))
                .bottom(px(lerp(-12.0, 24.0, s)))
                .opacity(s.min(1.0))
                .min_w(px(288.0))
                .px(px(16.0))
                .py(px(14.0))
                .rounded(px(6.0))
                .bg(rgba(th.snackbar))
                .text_color(rgba(th.snackbar_text))
                .text_size(px(14.0))
                .shadow(elevation(th, 3.0))
                .child(snackbar.text.clone())
                .into_any_element(),
        )
    }

    fn render_error(&self, error: &OpenError, th: &Theme) -> AnyElement {
        let (title, text) = match error {
            OpenError::NoStore { data_dir } => (
                "No mail yet".to_owned(),
                format!(
                    "Katna Mail shows the mail that the Katna background service keeps in \
                     {data_dir}. Nothing is there yet. Add an account with katnactl, or \
                     import a Maildir or mbox with katna-search-cli import, then press F5."
                ),
            ),
            OpenError::Other(err) => ("The mail store could not be opened".to_owned(), err.clone()),
        };
        div()
            .size_full()
            .px(px(16.0))
            .pb(px(16.0))
            .child(
                div()
                    .size_full()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap(px(12.0))
                    .rounded(px(16.0))
                    .bg(rgba(th.surface))
                    .child(icon("mail", th.text_faint, 64.0))
                    .child(
                        div()
                            .text_size(px(22.0))
                            .text_color(rgba(th.text))
                            .child(title),
                    )
                    .child(
                        div()
                            .max_w(px(460.0))
                            .text_size(px(14.0))
                            .line_height(px(21.0))
                            .text_color(rgba(th.text_faint))
                            .text_center()
                            .child(text),
                    ),
            )
            .into_any_element()
    }
}

impl Render for MailWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let th = Theme::new(self.chrome.tokens(window).dark);
        let reduce = cx.reduce_motion();
        let viewport = f32::from(window.viewport_size().width);
        let wide = viewport > 760.0;

        self.nav_spring.set(if self.nav_open || self.nav_peek {
            1.0
        } else {
            0.0
        });
        self.reserve_spring
            .set(if self.nav_open { 1.0 } else { 0.0 });
        let search_focused = self.search.focus_handle(cx).is_focused(window);
        self.search_spring
            .set(if search_focused { 1.0 } else { 0.0 });
        self.nav_t = self.nav_spring.tick(window, reduce);
        self.reserve_spring.tick(window, reduce);
        let search_t = self.search_spring.tick(window, reduce);

        let accent: Hsla = rgba(th.accent).into();
        self.search
            .update(cx, |search, _| search.set_accent(accent));

        let content = match &self.mail {
            Err(err) => self.render_error(err, &th),
            // Reversed so the navigation paints last, over the card, when
            // it opens from the rail.
            Ok(_) => div()
                .size_full()
                .flex()
                .flex_row_reverse()
                .child(self.render_card(&th, cx))
                .child(self.render_navigation(&th, cx))
                .into_any_element(),
        };
        let snackbar = self.render_snackbar(&th, window, reduce);
        let content = div()
            .key_context(WINDOW_CONTEXT)
            .relative()
            .size_full()
            .bg(rgba(th.page))
            .text_color(rgba(th.text))
            .on_action(cx.listener(Self::focus_search))
            .on_action(cx.listener(Self::focus_list))
            .on_action(cx.listener(Self::toggle_navigation))
            .on_action(cx.listener(Self::compose))
            .on_action(cx.listener(Self::reload))
            .on_action(cx.listener(Self::quit))
            .child(content)
            .children(snackbar)
            .into_any_element();

        let side = if wide { NAV_WIDTH } else { 110.0 };
        let search_width = (viewport - 2.0 * side).clamp(200.0, SEARCH_WIDTH);
        let bar = Bar {
            start: self.render_top_start(&th, wide, cx),
            center: self
                .mail
                .is_ok()
                .then(|| self.render_search(&th, search_width, search_t, cx)),
            end: self.render_account(&th),
            height: Some(TOP_BAR_HEIGHT),
            background: Some(th.page),
        };
        let frame = self.chrome.render_bar(bar, content, window, cx);
        match &self.font {
            Some(font) => frame.font_family(font.clone()),
            None => frame,
        }
    }
}

impl Focusable for MailWindow {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.list_focus.clone()
    }
}

fn icon(name: &str, color: u32, size: f32) -> AnyElement {
    svg()
        .path(SharedString::from(format!("icons/{name}.svg")))
        .size(px(size))
        .flex_none()
        .text_color(rgba(color))
        .into_any_element()
}

/// A round icon button with a centered ripple.
fn icon_button(id: &'static str, name: &str, size: f32, th: &Theme) -> Stateful<Div> {
    div()
        .id(id)
        .relative()
        .overflow_hidden()
        .size(px(40.0))
        .flex_none()
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .cursor_pointer()
        .hover(|s| s.bg(rgba(th.hover)))
        // Keep the header bar from starting a window move.
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .child(Ripple::new((id, 0_usize), rgba(th.ripple)).centered())
        .child(icon(name, th.text_dim, size))
}

/// An outlined button with an icon and a label.
fn pill_button(
    id: &'static str,
    name: &str,
    label: &'static str,
    th: &Theme,
    cx: &mut Context<MailWindow>,
) -> AnyElement {
    div()
        .id(id)
        .relative()
        .overflow_hidden()
        .h(px(36.0))
        .pl(px(16.0))
        .pr(px(22.0))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(8.0))
        .rounded_full()
        .border_1()
        .border_color(rgba(fade(th.text_faint, 0.7)))
        .text_size(px(14.0))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgba(th.text_dim))
        .cursor_pointer()
        .hover(|s| s.bg(rgba(th.hover)))
        .on_click(cx.listener(|this, _, _, cx| this.show_snackbar(NOT_YET, cx)))
        .child(Ripple::new((id, 0_usize), rgba(th.ripple)))
        .child(icon(name, th.text_dim, 20.0))
        .child(label)
        .into_any_element()
}

fn toolbar(th: &Theme) -> Div {
    div()
        .flex_none()
        .h(px(TOOLBAR_HEIGHT))
        .px(px(8.0))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(4.0))
        .border_b_1()
        .border_color(rgba(th.divider))
}

/// A letter avatar for `name`, colored by `address`.
fn avatar(name: &str, address: &str, size: f32) -> AnyElement {
    let key = if address.is_empty() { name } else { address };
    div()
        .size(px(size))
        .flex_none()
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .bg(rgba(avatar_color(key)))
        .text_color(rgba(0xffffffff))
        .text_size(px(size * 0.45))
        .font_weight(FontWeight::MEDIUM)
        .child(initial(name))
        .into_any_element()
}

/// Material-style elevation: `level` 0 is flat, 3 floats well above.
fn elevation(th: &Theme, level: f32) -> Vec<BoxShadow> {
    if level <= 0.001 {
        return Vec::new();
    }
    let t = (level / 3.0).min(1.0);
    vec![
        BoxShadow {
            color: rgba(fade(th.shadow, 0.9 * t)).into(),
            offset: point(px(0.0), px(1.0)),
            blur_radius: px(2.0 * level.min(2.0)),
            spread_radius: px(0.0),
            inset: false,
        },
        BoxShadow {
            color: rgba(fade(th.shadow, 0.5 * t)).into(),
            offset: point(px(0.0), px(level)),
            blur_radius: px(3.0 * level),
            spread_radius: px(level / 2.0),
            inset: false,
        },
    ]
}

fn role_icon(role: Role) -> &'static str {
    match role {
        Role::Inbox => "inbox",
        Role::Flagged => "star",
        Role::Drafts => "drafts",
        Role::Sent => "sent",
        Role::Archive => "archive",
        Role::All => "all-mail",
        Role::Junk => "junk",
        Role::Trash => "trash",
        Role::Other => "label",
    }
}

fn placeholder(text: &str, th: &Theme) -> AnyElement {
    div()
        .size_full()
        .flex()
        .items_center()
        .justify_center()
        .p(px(24.0))
        .text_size(px(14.0))
        .text_color(rgba(th.text_faint))
        .child(text.to_owned())
        .into_any_element()
}

/// Loads message `id` for the reading view.
fn read(mail: &Mail, id: MessageId) -> Reader {
    let Some(raw) = mail.raw(id) else {
        return Reader {
            id,
            view: None,
            blocks: Vec::new(),
            cut: false,
            flagged: false,
        };
    };
    let view = katna_render::message_view(&raw);
    let (blocks, cut) = body_blocks(&view.body, MAX_BODY_LINES);
    Reader {
        id,
        cut: cut || view.truncated,
        view: Some(view),
        blocks,
        flagged: false,
    }
}

/// Splits a body into runs of quoted (`>`) and unquoted lines, at most
/// `max_lines` lines in all. Returns whether lines were left out.
fn body_blocks(body: &str, max_lines: usize) -> (Vec<(bool, SharedString)>, bool) {
    let mut blocks: Vec<(bool, String)> = Vec::new();
    let mut lines = body.lines();
    for line in lines.by_ref().take(max_lines) {
        let line = line.trim_end();
        let quoted = line.starts_with('>');
        match blocks.last_mut() {
            Some((q, text)) if *q == quoted => {
                text.push('\n');
                text.push_str(line);
            }
            _ => blocks.push((quoted, line.to_owned())),
        }
    }
    let cut = lines.next().is_some();
    (
        blocks
            .into_iter()
            .map(|(quoted, text)| (quoted, text.into()))
            .collect(),
        cut,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quoted_blocks() {
        let body = "Hi,\n\nsee below.\n> old line 1\n>> older\nthanks\r\n";
        let (blocks, cut) = body_blocks(body, 100);
        let blocks: Vec<(bool, &str)> = blocks.iter().map(|(q, t)| (*q, t.as_ref())).collect();
        assert_eq!(
            blocks,
            [
                (false, "Hi,\n\nsee below."),
                (true, "> old line 1\n>> older"),
                (false, "thanks"),
            ]
        );
        assert!(!cut);
        let (blocks, cut) = body_blocks(body, 2);
        assert_eq!(blocks.len(), 1);
        assert!(cut);
    }
}
