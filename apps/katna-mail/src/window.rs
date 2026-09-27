// SPDX-License-Identifier: GPL-3.0-or-later

//! The main window (`docs/ARCHITECTURE.md` §13.6): a top bar with the
//! search box in the middle, the navigation with Compose and the folders,
//! the list card and the open conversation, beside the list (three panes,
//! the default) or in its place (two panes). Motion comes from springs
//! (`katna_ui::motion`) and click ripples; all of it honors the desktop's
//! reduce-motion setting.
//!
//! The parts live in submodules: `nav` (top bar and navigation), `list`
//! (toolbar, tabs and rows), `reader` (the open conversation), `settings`
//! (quick settings), `search_panel` (search options), `compose`, `apps`
//! (the app rail), `add_account` (adding an account), `context_menu`
//! (the list's right-click menu), `onboarding` (the first start), `tour`
//! (a walk through the window), `whats_new` (after an update) and `layout` (phone, tablet and desktop
//! layouts, by the window's width).

mod account_view;
mod accounts;
mod add_account;
mod apps;
mod attachments;
mod colors;
mod compose;
mod context_menu;
mod crash_notice;
mod dark;
mod desktop;
mod detached;
mod download;
mod feedback_page;
mod keymap;
mod labels;
mod layout;
mod list;
mod look;
mod nav;
mod onboarding;
mod popovers;
mod print;
mod reader;
mod remote;
mod reply_row;
mod rich;
mod search_panel;
mod settings;
mod settings_page;
mod settings_search;
mod tour;
mod viewer;
mod whats_new;

use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::path::PathBuf;
use std::time::Duration;

use futures_lite::StreamExt;
use gpui::{
    AnyElement, App, Context, Entity, FocusHandle, Focusable, FontWeight, Hsla, ListAlignment,
    ListState, MouseButton, MouseMoveEvent, Render, ScrollHandle, SharedString, Subscription, Task,
    TextRun, UniformListScrollHandle, WeakEntity, Window, actions, div, prelude::*, px, rgba,
};
use jiff::tz::TimeZone;
use katna_chrome::{Bar, ChromeColors, Environment, WindowChrome};
use katna_core::config::{ReadingPane, Theme as ThemeChoice};
use katna_core::{Account, AccountId, Config, Paths};
use katna_dbus::zbus::Connection;
use katna_search::SearchResults;
use katna_store::{FolderId, MessageFlags, MessageId};
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::{InputEvent, TextInput};

use crate::daemon::{self, Command};
use crate::data::{self, Entry, EntryKey, Mail, OpenError};
use crate::sidebar::{self, Role, Tree};
use crate::tabs::{self, Provider, Tab};
use crate::theme::Theme;
use crate::widgets::{elevation, icon};

use apps::{App as RailApp, People};
use reader::Conversation;
use search_panel::SearchPanel;

pub use desktop::{MenuBar, menu_bar, refresh_menu_bar};
pub use look::look;

actions!(
    katna_mail,
    [
        SelectNext,
        SelectPrevious,
        FocusNext,
        FocusPrevious,
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
        Archive,
        Delete,
        ReportSpam,
        MarkRead,
        MarkUnread,
        ToggleStar,
        MarkImportant,
        MarkNotImportant,
        ToggleCheck,
        ToggleSettings,
        Reply,
        ReplyAll,
        Forward,
        MoveTo,
        SelectAll,
        SelectNone,
        Undo,
        GoToInbox,
        GoToStarred,
        GoToSent,
        GoToDrafts,
        GoToAllMail,
        OpenSettings,
        ShowShortcuts,
    ]
);

const WINDOW_CONTEXT: &str = "MailWindow";
const LIST_CONTEXT: &str = "MessageList";
const READER_CONTEXT: &str = "MessageReader";
const SEARCH_CONTEXT: &str = "SearchBox";

pub(super) const TOP_BAR_HEIGHT: f32 = 64.0;
/// How far frosted menus and popovers blur what is behind them, in pixels.
const FROST_BLUR: f32 = 20.0;
const NAV_WIDTH: f32 = 256.0;
/// How far the folder highlight pill (and the drawer's) stays off the
/// pane's left edge.
const NAV_ROW_INSET: f32 = 8.0;
/// The one gap between the top bar's elements: the menu button and
/// Compose, Compose and the search box (when the window is too narrow for
/// the box's usual place), the search box and Settings, Settings and the
/// account picture. The header bar itself spaces its items 6 px apart.
const TOP_BAR_GAP: f32 = 16.0;
const BAR_ITEM_GAP: f32 = 6.0;
/// The room Settings and the account picture take at the top bar's end,
/// up to the window buttons: both 40 px wide, with the gap between them,
/// and 8 px after the picture plus the bar's own spacing.
const TOP_END_WIDTH: f32 = 40.0 + TOP_BAR_GAP + 40.0 + 8.0 + BAR_ITEM_GAP;
/// The word on the top bar's Compose button, and its size.
const COMPOSE_LABEL: &str = "Compose";
const COMPOSE_TEXT_SIZE: f32 = 14.0;
/// Where Compose starts on the top bar: the bar's 6 px padding, the menu
/// button (48 px with a 6 px margin) and the gap after it.
const COMPOSE_LEFT: f32 = 6.0 + 6.0 + 48.0 + TOP_BAR_GAP;

/// Width of the top bar's Compose button: a 40 px square when folded to
/// its pencil (`label` 0), the pencil and the word (`text` px wide) when
/// `label` is 1.
fn compose_width(label: f32, text: f32) -> f32 {
    lerp(40.0, 16.0 + 24.0 + 12.0 + text + 24.0, label)
}

/// How wide the word on Compose is drawn in the window's font, so the
/// button fits it whatever font and size the desktop uses.
fn compose_text_width(font: Option<&SharedString>, window: &Window) -> f32 {
    let mut style = window.text_style().font();
    if let Some(family) = font {
        style.family = family.clone();
    }
    style.weight = FontWeight::MEDIUM;
    let run = TextRun {
        len: COMPOSE_LABEL.len(),
        font: style,
        color: Hsla::default(),
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    let line =
        window
            .text_system()
            .shape_line(COMPOSE_LABEL.into(), px(COMPOSE_TEXT_SIZE), &[run], None);
    f32::from(line.width).ceil() + 1.0
}
/// Corners of cards that float: menus aside, dialogs and panels.
const PANEL_RADIUS: f32 = 15.0;
const SEARCH_WIDTH: f32 = 720.0;
/// Quick settings panel, with its right margin.
const SETTINGS_WIDTH: f32 = 336.0;
/// Space between the list and the reading pane; also the handle to drag.
const SPLIT_GAP: f32 = 12.0;
/// Narrower lists show each line as three (sender, subject, snippet).
const STACKED_BELOW: f32 = 680.0;
const PAGE: usize = 10;
/// Wait this long after a keystroke before searching, so fast typing
/// searches once.
const SEARCH_DELAY: Duration = Duration::from_millis(60);
/// Rest on Mail in the app rail this long before the folded navigation
/// opens over the list.
const PEEK_DELAY: Duration = Duration::from_millis(300);
/// The opened navigation waits this long after the pointer leaves, so it can
/// cross from the rail to the panel.
const PEEK_LINGER: Duration = Duration::from_millis(250);
const SNACKBAR_TIME: Duration = Duration::from_secs(5);
/// Changes signalled by the daemon within this time are read together.
const CHANGE_DELAY: Duration = Duration::from_millis(120);
const LINE_SCROLL: f32 = 48.0;
/// How long a send failure stays on screen.
const FAILURE_TIME: Duration = Duration::from_secs(12);

/// What the message list shows.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Listing {
    Folder(FolderId),
    Search {
        query: String,
        total: Option<usize>,
        /// What was searched instead, when the query had a word that is not
        /// in the mail ("Showing results for …").
        corrected: Option<String>,
    },
}

/// An open popup menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Menu {
    /// Next to the list's checkbox: select all, none, read, ...
    Select,
    /// The list's "more" button.
    ListMore,
    /// "Move to" with the folders of the account.
    MoveTo,
    /// The open conversation's "more" button.
    ReaderMore,
}

/// A change the user asks for on some lines of the list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Act {
    Archive,
    Delete,
    Spam,
    MoveTo(FolderId),
    Read(bool),
    Star(bool),
    Important(bool),
    Pin(bool),
}

/// What the pointer rests on that opens the folded navigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Hover {
    /// Mail in the app rail.
    Rail,
    Panel,
}

/// A short note at the bottom of the window, maybe with an Undo button.
struct Snackbar {
    text: SharedString,
    undo: Option<Command>,
    shown: Spring,
    _hide: Task<()>,
}

/// Changes sent to the daemon but not read back from the store yet, so
/// the list shows them at once.
#[derive(Debug, Default, Clone, Copy)]
struct Pending {
    unread: Option<bool>,
    flagged: Option<bool>,
    important: Option<bool>,
    pinned: Option<bool>,
}

pub struct MailWindow {
    chrome: WindowChrome,
    /// The app of the rail on show.
    app: RailApp,
    people: Option<People>,
    people_task: Option<Task<()>>,
    /// The desktop's UI font, or `None` to leave GPUI's default.
    font: Option<SharedString>,
    paths: Paths,
    config: Config,
    config_path: PathBuf,
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
    /// The inbox tabs of the listed folder's account; none outside inboxes.
    tabs: Vec<Tab>,
    /// The open inbox tab, an index into `tabs`.
    tab: usize,
    category_unread: HashMap<katna_core::MailCategory, u64>,
    entries: Vec<Entry>,
    /// The list cursor.
    selected: Option<usize>,
    /// Ticked lines.
    checked: HashSet<EntryKey>,
    /// Every line of the list is ticked, not only the ones on screen.
    checked_all: bool,
    /// How many lines "Select all" ticked on screen, while those are still
    /// the ticked ones. The banner offering the whole list follows this
    /// rather than the lines on screen, which the banner itself changes.
    page_pick: Option<usize>,
    pending: HashMap<EntryKey, Pending>,
    /// Rows of the list on screen at the last layout.
    visible: Range<usize>,
    hovered: Option<usize>,
    reader: Option<Conversation>,
    /// A window of its own showing one conversation (double-click on a
    /// line), not the main mail window.
    detached: bool,
    /// For a conversation window, the mail window it came from: it shows
    /// the snackbar (and its Undo) when the conversation moves away and
    /// this window closes.
    main: Option<WeakEntity<Self>>,
    /// Remote images and sender pictures of the open conversation.
    remote: remote::Remote,
    /// Whether a conversation is open: in place of the list with two
    /// panes, beside it with three.
    reading: bool,
    /// Changes whenever the card switches content, to replay its fade-in.
    card_seq: usize,
    search: Entity<TextInput>,
    /// The mail search put aside while the box searches settings.
    mail_query: Option<String>,
    /// Counts the rows a settings search has lit up.
    flash_seq: usize,
    search_error: Option<SharedString>,
    /// Search this text as typed, not corrected ("Search instead for …").
    search_verbatim: Option<String>,
    search_task: Option<Task<()>>,
    search_panel: Option<SearchPanel>,
    search_panel_spring: Spring,
    menu: Option<Menu>,
    /// The right-click menu of the list.
    context_menu: Option<context_menu::ContextMenu>,
    /// The navigation is open (not folded to the rail).
    nav_open: bool,
    /// The folded navigation is opened over the list while the pointer is
    /// on it or on Mail in the app rail.
    nav_peek: bool,
    /// The pointer is on Mail in the app rail, and on the panel.
    peek_hover: (bool, bool),
    peek_task: Option<Task<()>>,
    /// 0 = folded, 1 = open: the drawn navigation.
    nav_spring: Spring,
    /// 0 = folded, 1 = open: the space the navigation takes from the card.
    reserve_spring: Spring,
    search_spring: Spring,
    /// 0 = closed, 1 = open: the reading pane beside the list.
    pane_spring: Spring,
    /// Where the divider drag started: pointer x and the pane's share.
    split_drag: Option<(f32, f32)>,
    /// Width available to the list and the reading pane, at the last frame.
    cards_width: f32,
    /// Reply, Reply all and Forward at the foot of a conversation.
    reply_row: reply_row::ReplyRow,
    /// The same once the layout's motion settles, so the lines change
    /// shape once rather than midway through it.
    cards_target: f32,
    settings_open: bool,
    settings_spring: Spring,
    /// The reading-pane choice of the quick settings under the pointer.
    pane_hover: Option<ReadingPane>,
    /// The tab indicator's position, in tabs.
    tab_spring: Spring,
    snackbar: Option<Snackbar>,
    /// After a crash: the report to view or copy.
    crash_notice: Option<crash_notice::CrashNotice>,
    /// Settings > User feedback's list of crash reports, as last read.
    saved_reports: Option<feedback_page::SavedReports>,
    compose: Option<compose::Compose>,
    /// Attachment thumbnails and the attachment viewer.
    files: attachments::Files,
    add_account: Option<add_account::AddAccount>,
    /// The first-start pages, until the first account is in and set up.
    onboarding: Option<onboarding::Onboarding>,
    /// The What's new dialog, after an update or from quick settings.
    whats_new: Option<whats_new::WhatsNew>,
    tour: Option<tour::Tour>,
    tour_marks: tour::Marks,
    /// Where the parts the tour shows were in the last frame.
    tour_seen: HashMap<tour::Spot, gpui::Bounds<gpui::Pixels>>,
    /// An account still waits for its first sync, so an empty folder may
    /// only be not fetched yet.
    first_sync: bool,
    _first_sync_check: Option<Task<()>>,
    /// The account card above the rail's account picture.
    account_menu: bool,
    /// The message last handed to the outbox, for Undo.
    unsent: Option<compose::Unsent>,
    /// The spelling dictionary and scheduled mail of compose.
    writing: compose::Writing,
    /// The Settings page, when open in place of the list.
    settings_page: Option<settings_page::SettingsPage>,
    /// The question before removing an account or deleting all data.
    danger: Option<accounts::Danger>,
    new_label: Option<labels::NewLabel>,
    /// Bodies being downloaded because their message or an attachment
    /// chip of it was opened.
    downloads: HashMap<MessageId, download::Download>,
    /// The attachment chip waiting for its message to download.
    chip_download: Option<download::ChipDownload>,
    /// Navigation openness at this frame, for the folder rows.
    nav_t: f32,
    daemon: Option<Connection>,
    _listen: Option<Task<()>>,
    _watch_sending: Option<Task<()>>,
    desktop_colors: colors::DesktopColors,
    /// Phone, tablet or desktop, by the window's width.
    layout: layout::Layout,
    list_focus: FocusHandle,
    /// The message list: lines differ in height (attachment chips).
    list_state: ListState,
    /// The line whose "+N" attachments button has its list open.
    files_menu: Option<EntryKey>,
    /// Layout and line height the list's lines were measured for.
    list_shape: (bool, u32),
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
        let config_existed = paths.config_file().exists();
        let mut this = Self::build(env, paths, font, window, cx);
        keymap::bind(&this.config.shortcuts, cx);
        this.load_tree();
        if let Some((folder, ancestors)) = this.default_folder() {
            this.expanded.extend(ancestors);
            this.rebuild_nav();
            this.open_folder(folder, cx);
        }
        this.count_unread(cx);
        this.listen(cx);
        this.watch_colors(cx);
        if let Some(err) = this.mail.as_ref().ok().and_then(Mail::index_error) {
            tracing::info!("{err}");
        }
        window.focus(&this.list_focus, cx);
        if this.needs_account() {
            this.onboarding = Some(onboarding::Onboarding::new());
        }
        this.welcome_or_whats_new(config_existed, window, cx);
        this.check_crashes(cx);
        tracing::info!(elapsed = ?started.elapsed(), lines = this.entries.len(), "mail loaded");
        this
    }

    /// The window's state before any mail is listed.
    fn build(
        env: Environment,
        paths: Paths,
        font: Option<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let search = cx.new(|cx| TextInput::new("Search mail", cx));
        let subscriptions = vec![cx.subscribe_in(&search, window, Self::on_search_event)];
        let config_path = paths.config_file();
        let config = Config::load(&config_path).unwrap_or_else(|err| {
            tracing::warn!("{err}; using the default settings");
            Config::default()
        });
        let desktop_colors = colors::DesktopColors::new(&env.desktop);
        let mut this = Self {
            chrome: WindowChrome::new(env, "Katna Mail", window, cx),
            app: RailApp::Mail,
            people: None,
            people_task: None,
            font,
            mail: Mail::open(&paths),
            remote: remote::Remote::load(&paths),
            accounts: Vec::new(),
            paths,
            config,
            config_path,
            tree: Tree::default(),
            unread: HashMap::new(),
            unread_task: None,
            expanded: HashSet::new(),
            nav_rows: Vec::new(),
            folder: None,
            show_recipients: false,
            listing: None,
            tabs: Vec::new(),
            tab: 0,
            category_unread: HashMap::new(),
            entries: Vec::new(),
            selected: None,
            checked: HashSet::new(),
            checked_all: false,
            page_pick: None,
            pending: HashMap::new(),
            visible: 0..0,
            hovered: None,
            reader: None,
            detached: false,
            main: None,
            reading: false,
            card_seq: 0,
            search,
            mail_query: None,
            flash_seq: 0,
            search_error: None,
            search_verbatim: None,
            search_task: None,
            search_panel: None,
            search_panel_spring: Spring::new(motion::SMOOTH, 0.0),
            menu: None,
            context_menu: None,
            nav_open: true,
            nav_peek: false,
            peek_hover: (false, false),
            peek_task: None,
            nav_spring: Spring::new(motion::SLIDE, 1.0),
            reserve_spring: Spring::new(motion::SLIDE, 1.0),
            search_spring: Spring::new(motion::SMOOTH, 0.0),
            pane_spring: Spring::new(motion::SLIDE, 0.0),
            split_drag: None,
            cards_width: 0.0,
            reply_row: reply_row::ReplyRow::new(),
            cards_target: 0.0,
            settings_open: false,
            pane_hover: None,
            settings_spring: Spring::new(motion::SLIDE, 0.0),
            tab_spring: Spring::new(motion::SLIDE, 0.0),
            snackbar: None,
            crash_notice: None,
            saved_reports: None,
            compose: None,
            files: attachments::Files::default(),
            add_account: None,
            onboarding: None,
            whats_new: None,
            tour: None,
            tour_marks: Default::default(),
            tour_seen: HashMap::new(),
            first_sync: false,
            _first_sync_check: None,
            account_menu: false,
            unsent: None,
            writing: compose::Writing::default(),
            settings_page: None,
            danger: None,
            new_label: None,
            downloads: HashMap::new(),
            chip_download: None,
            nav_t: 1.0,
            daemon: None,
            _listen: None,
            _watch_sending: None,
            desktop_colors,
            layout: layout::Layout::new(),
            list_focus: cx.focus_handle(),
            list_state: ListState::new(0, ListAlignment::Top, px(400.0)),
            files_menu: None,
            list_shape: (false, 0),
            nav_scroll: UniformListScrollHandle::new(),
            reader_scroll: ScrollHandle::new(),
            tz: TimeZone::try_system().unwrap_or(TimeZone::UTC),
            _subscriptions: subscriptions,
        };
        this.watch_escape(window, cx);
        let weak = cx.entity().downgrade();
        // The toolbar's "1–50 of N" follows the scrolling.
        this.list_state.set_scroll_handler(move |_, _, cx| {
            weak.update(cx, |_, cx| cx.notify()).ok();
        });
        this
    }

    /// Searches `query` as typed, after a correction the user didn't want.
    fn search_verbatim(&mut self, query: String, cx: &mut Context<Self>) {
        self.search_verbatim = Some(query.clone());
        self.start_search(query, cx);
    }

    /// Puts `query` in the search box and searches.
    pub fn search_for(&mut self, query: String, window: &mut Window, cx: &mut Context<Self>) {
        window.focus(&self.search.focus_handle(cx), cx);
        self.search
            .update(cx, |search, cx| search.set_text(query, cx));
    }

    /// Opens the first line of the list, for screenshots and tests.
    pub fn open_first(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.open(0, window, cx);
    }

    /// The colors: the desktop's light or dark, unless the settings pick
    /// one, in the desktop's color scheme or accent color if it has them.
    fn theme(&self, window: &Window) -> Theme {
        self.theme_for(&self.chrome, window)
    }

    /// [`Self::theme`] for a window framed by `chrome`.
    fn theme_for(&self, chrome: &WindowChrome, window: &Window) -> Theme {
        let choice = match self.config.mail.theme {
            ThemeChoice::System => None,
            ThemeChoice::Light => Some(false),
            ThemeChoice::Dark => Some(true),
        };
        let dark = choice.unwrap_or_else(|| WindowChrome::desktop_dark(window));
        let system = &self.desktop_colors.colors;
        let desktop_scheme = self.config.mail.desktop_colors && system.scheme_for(dark).is_some();
        let th = if self.config.mail.desktop_colors {
            Theme::system(dark, system)
        } else {
            Theme::new(dark)
        };
        // The window frame follows the same choices.
        chrome.set_dark(choice);
        chrome.set_colors(desktop_scheme.then_some(ChromeColors {
            window_bg: th.page,
            view_bg: th.surface,
            fg: th.text,
            accent: th.accent,
        }));
        chrome.set_backdrop(Some(th.page));
        // Menus and popovers are frosted with the blurred background, in
        // every window, blurred or not.
        let th = if self.config.experimental.blur
            && katna_chrome::Look::blur_available()
            && katna_ui::frost::supported()
        {
            th.frosted(FROST_BLUR * window.scale_factor())
        } else {
            th
        };
        if chrome.blurred() {
            th.translucent()
        } else {
            th
        }
    }

    /// No account yet: the store is not made, or has no account.
    fn needs_account(&self) -> bool {
        match &self.mail {
            Err(OpenError::NoStore { .. }) => true,
            Err(OpenError::Other(_)) => false,
            Ok(_) => self.accounts.is_empty(),
        }
    }

    /// Asks the daemon whether an account waits for its first sync.
    fn check_first_sync(&mut self, cx: &mut Context<Self>) {
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        self._first_sync_check = Some(cx.spawn(async move |this, cx| {
            let pending = daemon::first_sync_pending(&connection).await;
            this.update(cx, |this, cx| {
                let pending = pending.unwrap_or(false);
                if this.first_sync != pending {
                    this.first_sync = pending;
                    cx.notify();
                }
            })
            .ok();
        }));
    }

    /// Whether an open conversation sits beside the list: the three-pane
    /// setting, in a window wide enough for it.
    fn split(&self) -> bool {
        let shape = &self.layout.shape;
        self.config.mail.reading_pane == ReadingPane::Right && shape.size.splits(shape.width)
    }

    fn save_config(&mut self) {
        if let Err(err) = self.config.save(&self.config_path) {
            tracing::warn!("saving settings: {err}");
        }
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

    /// Connects to the daemon and reloads whenever it says mail changed.
    fn listen(&mut self, cx: &mut Context<Self>) {
        self._listen = Some(cx.spawn(async move |this, cx| {
            let connection = match cx.background_executor().spawn(daemon::connect()).await {
                Ok(connection) => connection,
                Err(err) => {
                    tracing::info!("{err}");
                    return;
                }
            };
            this.update(cx, |this, cx| {
                this.daemon = Some(connection.clone());
                // The main window reports sending and the first sync.
                if !this.detached {
                    this.watch_sending(connection.clone(), cx);
                    this.watch_scheduled(connection.clone(), cx);
                    this.check_first_sync(cx);
                }
            })
            .ok();
            let mut changes = match daemon::mail_changes(&connection).await {
                Ok(changes) => changes,
                Err(err) => {
                    tracing::info!("not following mail changes: {err}");
                    return;
                }
            };
            while changes.next().await.is_some() {
                cx.background_executor().timer(CHANGE_DELAY).await;
                let refreshed = this.update(cx, |this, cx| {
                    this.refresh(false, cx);
                    if !this.detached {
                        this.check_first_sync(cx);
                    }
                });
                if refreshed.is_err() {
                    break;
                }
            }
        }));
    }

    /// Says when the server refuses a message for good.
    fn watch_sending(&mut self, connection: Connection, cx: &mut Context<Self>) {
        self._watch_sending = Some(cx.spawn(async move |this, cx| {
            let mut failures = match daemon::send_failures(&connection).await {
                Ok(failures) => Box::pin(failures),
                Err(err) => {
                    tracing::info!("not following the outbox: {err}");
                    return;
                }
            };
            while let Some((subject, detail)) = failures.next().await {
                let subject = if subject.trim().is_empty() {
                    "(no subject)".to_owned()
                } else {
                    subject
                };
                let text = format!("\u{201c}{subject}\u{201d} could not be sent: {detail}");
                if this
                    .update(cx, |this, cx| {
                        this.show_snackbar_for(text, None, FAILURE_TIME, cx)
                    })
                    .is_err()
                {
                    break;
                }
            }
        }));
    }

    /// With one account the folders stand alone, without an account heading.
    fn rebuild_nav(&mut self) {
        let mut rows = self.tree.rows(&self.expanded, self.shown_account());
        let accounts = rows
            .iter()
            .filter(|r| matches!(r, sidebar::Row::Account { .. }))
            .count();
        if accounts == 1 {
            rows.retain(|r| !matches!(r, sidebar::Row::Account { .. }));
        }
        // Scheduled mail shows under the first Sent folder while there is
        // some.
        let scheduled = self.writing.scheduled_count();
        if scheduled > 0 {
            let at = rows
                .iter()
                .position(|r| {
                    matches!(
                        r,
                        sidebar::Row::Folder {
                            role: Role::Sent,
                            ..
                        }
                    )
                })
                .map_or(rows.len(), |ix| ix + 1);
            rows.insert(
                at,
                sidebar::Row::Folder {
                    key: compose::SCHEDULED_NAV_KEY.to_owned(),
                    depth: 0,
                    label: "Scheduled".to_owned(),
                    role: Role::Other,
                    folder: None,
                    unread: scheduled as u64,
                    has_children: false,
                    expanded: false,
                },
            );
        }
        self.nav_rows = rows;
    }

    /// The folder the list shows; `None` for search results.
    fn listed_folder(&self) -> Option<FolderId> {
        match &self.listing {
            Some(Listing::Folder(folder)) => Some(*folder),
            _ => None,
        }
    }

    fn folder_role(&self) -> Role {
        match &self.listing {
            Some(Listing::Folder(folder)) => {
                self.tree.node(*folder).map_or(Role::Other, |n| n.role)
            }
            _ => Role::Other,
        }
    }

    /// Whether the list shows inbox tabs now.
    fn shows_tabs(&self) -> bool {
        !self.tabs.is_empty() && self.folder_role() == Role::Inbox
    }

    /// The inbox tabs of `account`: as its settings say, else its
    /// provider's.
    fn account_tabs(&self, account: AccountId) -> Vec<Tab> {
        if !self.config.mail.inbox_tabs {
            return Vec::new();
        }
        let Some(account) = self.accounts.iter().find(|a| a.id == account) else {
            return Vec::new();
        };
        let setting = self.config.mail.tabs_of(&account.address);
        tabs::tabs(&setting, self.provider(account))
    }

    fn provider(&self, account: &Account) -> Provider {
        let host = self
            .mail
            .as_ref()
            .ok()
            .and_then(|mail| mail.incoming_host(account.id));
        Provider::detect(&account.address, host.as_deref())
    }

    fn list_entries(&self, folder: FolderId) -> Vec<Entry> {
        let Ok(mail) = &self.mail else {
            return Vec::new();
        };
        let role = self.tree.node(folder).map_or(Role::Other, |n| n.role);
        let categories = self
            .tabs
            .get(self.tab)
            .filter(|_| role == Role::Inbox)
            .map(|tab| tab.categories.as_slice());
        mail.entries(folder, categories, self.config.mail.conversations)
    }

    fn open_folder(&mut self, folder: FolderId, cx: &mut Context<Self>) {
        self.follow_folder_account(folder);
        let Ok(mail) = &mut self.mail else {
            return;
        };
        let role = self.tree.node(folder).map_or(Role::Other, |n| n.role);
        if role.shows_recipients() != self.show_recipients {
            self.show_recipients = role.shows_recipients();
            mail.clear_rows();
        }
        if self.folder != Some(folder) {
            self.tab = 0;
        }
        self.tabs = match self.tree.account_of(folder) {
            Some(account) if role == Role::Inbox => self.account_tabs(account),
            _ => Vec::new(),
        };
        self.tab = self.tab.min(self.tabs.len().saturating_sub(1));
        self.folder = Some(folder);
        self.listing = Some(Listing::Folder(folder));
        self.entries = self.list_entries(folder);
        self.category_unread = match &self.mail {
            Ok(mail) if role == Role::Inbox => mail.category_unread(folder),
            _ => HashMap::new(),
        };
        self.reset_list(false);
        self.selected = (!self.entries.is_empty()).then_some(0);
        self.checked.clear();
        self.checked_all = false;
        self.page_pick = None;
        self.menu = None;
        self.show_list();
        cx.notify();
    }

    fn open_tab(&mut self, tab: usize, cx: &mut Context<Self>) {
        if self.tab == tab {
            return;
        }
        self.tab = tab;
        // No fade: the tab's lines replace the last ones in the same frame,
        // as the indicator slides over.
        if let Some(folder) = self.folder {
            self.open_folder(folder, cx);
        }
    }

    fn show_list(&mut self) {
        if self.reading && !self.split() {
            self.card_seq += 1;
        }
        self.reading = false;
        // A conversation beside the list or over it slides away first.
        if !self.split() && !self.slides() {
            self.reader = None;
        }
        self.hovered = None;
    }

    /// Moves the list cursor; while reading, opens that line instead.
    fn select(&mut self, ix: usize, cx: &mut Context<Self>) {
        if ix >= self.entries.len() {
            return;
        }
        self.selected = Some(ix);
        self.list_state.scroll_to_reveal_item(ix);
        if self.reading {
            self.load_reader(ix, cx);
        }
        cx.notify();
    }

    fn open(&mut self, ix: usize, window: &mut Window, cx: &mut Context<Self>) {
        if ix >= self.entries.len() {
            return;
        }
        if !self.reading && !self.split() {
            self.card_seq += 1;
        }
        self.selected = Some(ix);
        self.reading = true;
        self.menu = None;
        self.load_reader(ix, cx);
        window.focus(&self.list_focus, cx);
        cx.notify();
    }

    fn load_reader(&mut self, ix: usize, cx: &mut Context<Self>) {
        let entry = self.entries[ix];
        if self.reader.as_ref().is_some_and(|r| r.key == entry.key) {
            return;
        }
        let Ok(mail) = &mut self.mail else {
            return;
        };
        let conversation = Conversation::load(mail, entry.key);
        self.reader_scroll.set_offset(gpui::point(px(0.0), px(0.0)));
        // Opening marks the conversation read, as webmail does.
        let unread = conversation.unread_messages();
        self.reader = Some(conversation);
        if !unread.is_empty() {
            self.pending.entry(entry.key).or_default().unread = Some(false);
            self.send(Command::MarkRead(unread, true), None, None, true, cx);
        }
    }

    fn move_selection(&mut self, delta: isize, cx: &mut Context<Self>) {
        if self.entries.is_empty() {
            return;
        }
        let last = self.entries.len() - 1;
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
        self.select(self.entries.len().saturating_sub(1), cx);
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
        if self.detached {
            window.remove_window();
            return;
        }
        self.show_list();
        window.focus(&self.list_focus, cx);
        cx.notify();
    }

    fn scroll_reader(&mut self, dy: f32, cx: &mut Context<Self>) {
        let offset = self.reader_scroll.offset();
        let max = self.reader_scroll.max_offset();
        let y = (offset.y - px(dy)).clamp(-max.y, px(0.0));
        self.reader_scroll.set_offset(gpui::point(offset.x, y));
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
        if self.selected.is_none() && !self.entries.is_empty() {
            self.select(0, cx);
        }
    }

    fn toggle_navigation(&mut self, _: &ToggleNavigation, _: &mut Window, cx: &mut Context<Self>) {
        // Phones and tablets open the folders as a drawer over the list.
        if !self.layout.shape.is_desktop() {
            self.layout.drawer = !self.layout.drawer;
            self.nav_peek = false;
            self.peek_task = None;
            cx.notify();
            return;
        }
        self.nav_open = !self.nav_open;
        self.nav_peek = false;
        self.peek_hover = (false, false);
        self.peek_task = None;
        cx.notify();
    }

    fn toggle_settings(&mut self, _: &ToggleSettings, window: &mut Window, cx: &mut Context<Self>) {
        self.settings_open = !self.settings_open;
        self.search_panel = None;
        let _ = window;
        cx.notify();
    }

    fn compose(&mut self, _: &Compose, window: &mut Window, cx: &mut Context<Self>) {
        self.open_compose(compose::Kind::New, None, window, cx);
    }

    fn reply(&mut self, _: &Reply, window: &mut Window, cx: &mut Context<Self>) {
        self.reply_with(compose::Kind::Reply, window, cx);
    }

    fn reply_all(&mut self, _: &ReplyAll, window: &mut Window, cx: &mut Context<Self>) {
        self.reply_with(compose::Kind::ReplyAll, window, cx);
    }

    fn forward(&mut self, _: &Forward, window: &mut Window, cx: &mut Context<Self>) {
        self.reply_with(compose::Kind::Forward, window, cx);
    }

    /// Reply, reply all or forward from the open conversation.
    fn reply_with(&mut self, kind: compose::Kind, window: &mut Window, cx: &mut Context<Self>) {
        if self.reading && self.reader.is_some() {
            self.open_compose(kind, None, window, cx);
        }
    }

    fn move_to(&mut self, _: &MoveTo, _: &mut Window, cx: &mut Context<Self>) {
        if self.checked.is_empty()
            && !self.reading
            && let Some(entry) = self.selected.and_then(|ix| self.entries.get(ix))
        {
            // The list shows "Move to" for ticked lines.
            self.checked.insert(entry.key);
        }
        if !self.checked.is_empty() || self.reading {
            self.menu = Some(Menu::MoveTo);
            cx.notify();
        }
    }

    fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.checked = self.entries.iter().map(|e| e.key).collect();
        self.checked_all = true;
        cx.notify();
    }

    fn select_none(&mut self, _: &SelectNone, _: &mut Window, cx: &mut Context<Self>) {
        self.checked.clear();
        self.checked_all = false;
        cx.notify();
    }

    fn undo_action(&mut self, _: &Undo, window: &mut Window, cx: &mut Context<Self>) {
        self.undo(window, cx);
    }

    fn go_to(&mut self, role: Role, window: &mut Window, cx: &mut Context<Self>) {
        let Some(folder) = self
            .account()
            .and_then(|account| self.tree.role_folder(account, role))
        else {
            self.show_snackbar("This account has no such folder.", None, cx);
            return;
        };
        self.settings_page = None;
        self.open_app(RailApp::Mail, cx);
        self.clear_search(cx);
        self.card_seq += 1;
        self.open_folder(folder, cx);
        window.focus(&self.list_focus, cx);
    }

    fn go_to_inbox(&mut self, _: &GoToInbox, window: &mut Window, cx: &mut Context<Self>) {
        self.go_to(Role::Inbox, window, cx);
    }

    fn go_to_starred(&mut self, _: &GoToStarred, window: &mut Window, cx: &mut Context<Self>) {
        self.go_to(Role::Flagged, window, cx);
    }

    fn go_to_sent(&mut self, _: &GoToSent, window: &mut Window, cx: &mut Context<Self>) {
        self.go_to(Role::Sent, window, cx);
    }

    fn go_to_drafts(&mut self, _: &GoToDrafts, window: &mut Window, cx: &mut Context<Self>) {
        self.go_to(Role::Drafts, window, cx);
    }

    fn go_to_all_mail(&mut self, _: &GoToAllMail, window: &mut Window, cx: &mut Context<Self>) {
        self.go_to(Role::All, window, cx);
    }

    fn show_snackbar(
        &mut self,
        text: impl Into<SharedString>,
        undo: Option<Command>,
        cx: &mut Context<Self>,
    ) {
        self.show_snackbar_for(text, undo, SNACKBAR_TIME, cx);
    }

    fn show_snackbar_for(
        &mut self,
        text: impl Into<SharedString>,
        undo: Option<Command>,
        time: Duration,
        cx: &mut Context<Self>,
    ) {
        let mut shown = Spring::new(motion::SLIDE, 0.0);
        shown.set(1.0);
        let hide = cx.spawn(async move |this, cx| {
            cx.background_executor().timer(time).await;
            this.update(cx, |this, cx| {
                if let Some(snackbar) = &mut this.snackbar {
                    snackbar.shown.set(0.0);
                    cx.notify();
                }
            })
            .ok();
        });
        self.snackbar = Some(Snackbar {
            text: text.into(),
            undo,
            shown,
            _hide: hide,
        });
        cx.notify();
    }

    fn hide_snackbar(&mut self, cx: &mut Context<Self>) {
        if let Some(snackbar) = &mut self.snackbar {
            snackbar.shown.set(0.0);
            cx.notify();
        }
    }

    /// The pointer came to or left Mail in the rail or the navigation
    /// panel: while the navigation is folded, it opens after a moment of
    /// rest and closes a moment after the pointer is gone from both.
    fn hover_navigation(&mut self, what: Hover, hovered: bool, cx: &mut Context<Self>) {
        match what {
            Hover::Rail => self.peek_hover.0 = hovered,
            Hover::Panel => self.peek_hover.1 = hovered,
        }
        if self.nav_docked() || self.layout.drawer || self.app != RailApp::Mail {
            return;
        }
        let on = self.peek_hover.0 || self.peek_hover.1;
        if on == self.nav_peek {
            self.peek_task = None;
            return;
        }
        // Only the rail opens the panel; the panel only keeps it open.
        if on && !self.peek_hover.0 {
            return;
        }
        let delay = if on { PEEK_DELAY } else { PEEK_LINGER };
        self.peek_task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(delay).await;
            this.update(cx, |this, cx| {
                this.peek_task = None;
                if !this.nav_docked() {
                    this.nav_peek = on;
                    cx.notify();
                }
            })
            .ok();
        }));
    }

    fn reload(&mut self, _: &Reload, _: &mut Window, cx: &mut Context<Self>) {
        if self.mail.is_err() {
            self.reopen(cx);
            return;
        }
        self.send(Command::SyncNow, None, None, true, cx);
        self.refresh(true, cx);
    }

    /// Tries the store again after it could not be opened.
    fn reopen(&mut self, cx: &mut Context<Self>) {
        self.mail = Mail::open(&self.paths);
        self.load_tree();
        self.open_default_folder(cx);
        self.count_unread(cx);
        cx.notify();
    }

    /// Opens the first inbox when nothing is listed, as when the first
    /// account's folders arrive.
    fn open_default_folder(&mut self, cx: &mut Context<Self>) {
        if self.listing.is_some() {
            return;
        }
        if let Some((folder, ancestors)) = self.default_folder() {
            self.expanded.extend(ancestors);
            self.rebuild_nav();
            self.open_folder(folder, cx);
        }
    }

    /// After an account was added: its folders follow with the first sync.
    fn account_added(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        if self.mail.is_err() {
            self.reopen(cx);
        } else {
            self.refresh(true, cx);
        }
        self.first_sync = true;
        self.check_first_sync(cx);
        self.onboarding_account_added(cx);
    }

    /// Reads the store again, keeping the cursor and the open conversation.
    fn refresh(&mut self, animate: bool, cx: &mut Context<Self>) {
        if self.mail.is_err() {
            // The daemon may have made the store since.
            self.reopen(cx);
            return;
        }
        let expanded = std::mem::take(&mut self.expanded);
        if let Ok(mail) = &mut self.mail {
            mail.refresh();
        }
        self.pending.clear();
        if self.detached {
            self.expanded = expanded;
            if let (Some(reader), Ok(mail)) = (&mut self.reader, &mut self.mail) {
                reader.refresh(mail);
            }
            cx.notify();
            return;
        }
        self.load_tree();
        self.expanded = expanded;
        self.rebuild_nav();
        let selected_key = self
            .selected
            .and_then(|ix| self.entries.get(ix))
            .map(|e| e.key);
        match self.listing.clone() {
            Some(Listing::Folder(folder)) => {
                self.entries = self.list_entries(folder);
                self.reset_list(true);
                if let Ok(mail) = &self.mail
                    && self.folder_role() == Role::Inbox
                {
                    self.category_unread = mail.category_unread(folder);
                }
                self.selected =
                    selected_key.and_then(|key| self.entries.iter().position(|e| e.key == key));
                let keys: HashSet<EntryKey> = self.entries.iter().map(|e| e.key).collect();
                self.checked.retain(|key| keys.contains(key));
                if self.selected.is_none() && self.reading {
                    self.show_list();
                    self.reader = None;
                }
            }
            Some(Listing::Search { query, .. }) => self.start_search(query, cx),
            None => self.open_default_folder(cx),
        }
        if let (Some(reader), Ok(mail)) = (&mut self.reader, &mut self.mail) {
            reader.refresh(mail);
        }
        if animate {
            self.card_seq += 1;
        }
        self.count_unread(cx);
        cx.notify();
    }

    fn quit(&mut self, _: &Quit, _: &mut Window, cx: &mut Context<Self>) {
        cx.quit();
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
        if self.settings_page.is_some() {
            self.on_settings_search(event, window, cx);
            return;
        }
        match event {
            InputEvent::Changed => {
                let text = search.read(cx).text().trim().to_owned();
                if !text.is_empty() {
                    self.open_app(RailApp::Mail, cx);
                }
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
        let correct = self.search_verbatim.as_deref() != Some(text.as_str());
        self.search_task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(SEARCH_DELAY).await;
            let now = jiff::Timestamp::now().as_second();
            let query = text.clone();
            let results = cx
                .background_executor()
                .spawn(async move { data::search(&index, &query, now, correct) })
                .await;
            this.update(cx, |this, cx| this.show_results(text, results, cx))
                .ok();
        }));
    }

    fn show_results(
        &mut self,
        query: String,
        results: Result<(SearchResults, Option<String>), String>,
        cx: &mut Context<Self>,
    ) {
        match results {
            Ok((results, corrected)) => {
                // Search results mix folders; show senders.
                if self.show_recipients {
                    self.show_recipients = false;
                    if let Ok(mail) = &mut self.mail {
                        mail.clear_rows();
                    }
                }
                let first = !matches!(self.listing, Some(Listing::Search { .. }));
                // The same search again, after the mail changed: keep the
                // cursor, the ticks and the open conversation.
                let again = matches!(
                    &self.listing,
                    Some(Listing::Search { query: old, .. }) if *old == query
                );
                let selected_key = self
                    .selected
                    .and_then(|ix| self.entries.get(ix))
                    .map(|e| e.key);
                let hits: Vec<MessageId> = results.hits.iter().map(|hit| hit.message).collect();
                let only = self.shown_account();
                self.entries = match &self.mail {
                    Ok(mail) => mail.hit_entries(&hits, self.config.mail.conversations, only),
                    Err(_) => Vec::new(),
                };
                self.listing = Some(Listing::Search {
                    query,
                    total: results.total,
                    corrected,
                });
                if again {
                    self.selected =
                        selected_key.and_then(|key| self.entries.iter().position(|e| e.key == key));
                    let keys: HashSet<EntryKey> = self.entries.iter().map(|e| e.key).collect();
                    self.checked.retain(|key| keys.contains(key));
                    self.reset_list(true);
                    if self.selected.is_none() && self.reading {
                        self.show_list();
                        self.reader = None;
                    }
                    cx.notify();
                    return;
                }
                self.selected = (!self.entries.is_empty()).then_some(0);
                self.checked.clear();
                self.checked_all = false;
                self.page_pick = None;
                self.reset_list(false);
                if first {
                    self.card_seq += 1;
                }
                self.show_list();
                self.reader = None;
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

    /// The account of the list: the folder's, or the first one.
    fn account(&self) -> Option<AccountId> {
        self.folder
            .and_then(|f| self.tree.account_of(f))
            .or_else(|| self.shown_account())
            .or_else(|| self.accounts.first().map(|a| a.id))
    }

    // Changes

    /// The lines an action applies to: the ticked ones, else the open
    /// conversation, else the cursor's line.
    fn target_keys(&self) -> Vec<EntryKey> {
        if !self.checked.is_empty() {
            return self
                .entries
                .iter()
                .filter(|e| self.checked.contains(&e.key))
                .map(|e| e.key)
                .collect();
        }
        if self.reading
            && let Some(reader) = &self.reader
        {
            return vec![reader.key];
        }
        self.selected
            .and_then(|ix| self.entries.get(ix))
            .map(|e| vec![e.key])
            .unwrap_or_default()
    }

    fn act_on_targets(&mut self, act: Act, cx: &mut Context<Self>) {
        let keys = self.target_keys();
        self.act(act, keys, cx);
    }

    /// Applies `act` to the lines `keys`: at once in the window, then in
    /// the store and on the server through the daemon.
    fn act(&mut self, act: Act, keys: Vec<EntryKey>, cx: &mut Context<Self>) {
        self.menu = None;
        if keys.is_empty() {
            return;
        }
        let Ok(mail) = &self.mail else {
            return;
        };
        let what = match (keys.len(), self.config.mail.conversations) {
            (1, true) => "Conversation".to_owned(),
            (1, false) => "Message".to_owned(),
            (n, true) => format!("{n} conversations"),
            (n, false) => format!("{n} messages"),
        };
        let folder = self
            .folder
            .filter(|_| matches!(self.listing, Some(Listing::Folder(_))));
        let messages_in = |key: EntryKey| match folder {
            Some(folder) => mail.entry_messages_in(key, folder),
            None => mail.entry_messages(key),
        };
        // Flag changes touch every stored copy of a message (the line is
        // starred or unread when any copy is), and undo restores exactly
        // the copies that changed.
        let copies_of = |keys: &[EntryKey]| {
            let ids: Vec<MessageId> = keys.iter().flat_map(|k| mail.entry_messages(*k)).collect();
            mail.with_copies(&ids)
        };
        let for_good = matches!(act, Act::Delete) && {
            let trash = self.account().and_then(|a| mail.trash_folder(a));
            trash.is_none() || (folder.is_some() && folder == trash)
        };
        let (command, undo) = match act {
            Act::Read(read) => {
                let ids = data::flag_changes(&copies_of(&keys), MessageFlags::SEEN, read);
                for key in &keys {
                    self.pending.entry(*key).or_default().unread = Some(!read);
                }
                (Command::MarkRead(ids, read), None)
            }
            Act::Star(on) => {
                // Starring marks the newest message; unstarring clears all.
                let copies = if on {
                    let latest: Vec<MessageId> = self
                        .entries
                        .iter()
                        .filter(|e| keys.contains(&e.key))
                        .map(|e| e.latest)
                        .collect();
                    mail.with_copies(&latest)
                } else {
                    copies_of(&keys)
                };
                let ids = data::flag_changes(&copies, MessageFlags::FLAGGED, on);
                for key in &keys {
                    self.pending.entry(*key).or_default().flagged = Some(on);
                }
                let undo = Command::Star(ids.clone(), !on);
                (Command::Star(ids, on), Some(undo))
            }
            Act::Important(on) => {
                let ids = data::flag_changes(&copies_of(&keys), MessageFlags::IMPORTANT, on);
                for key in &keys {
                    self.pending.entry(*key).or_default().important = Some(on);
                }
                let undo = Command::Important(ids.clone(), !on);
                (Command::Important(ids, on), Some(undo))
            }
            Act::Pin(on) => {
                // The whole conversation, wherever its messages are.
                let ids: Vec<MessageId> = copies_of(&keys).into_iter().map(|(id, _)| id).collect();
                for key in &keys {
                    self.pending.entry(*key).or_default().pinned = Some(on);
                }
                let undo = Command::Pin(ids.clone(), !on);
                (Command::Pin(ids, on), Some(undo))
            }
            Act::Archive | Act::Delete | Act::Spam | Act::MoveTo(_) => {
                let ids: Vec<MessageId> = keys.iter().flat_map(|k| messages_in(*k)).collect();
                let target = match act {
                    Act::Spam => {
                        let junk = self
                            .account()
                            .and_then(|a| self.tree.role_folder(a, Role::Junk));
                        match junk {
                            Some(junk) => Some(junk),
                            None => {
                                self.show_snackbar("This account has no spam folder.", None, cx);
                                return;
                            }
                        }
                    }
                    Act::MoveTo(target) => Some(target),
                    _ => None,
                };
                let command = match (act, target) {
                    (Act::Archive, _) => Command::Archive(ids.clone()),
                    (Act::Delete, _) => Command::Delete(ids.clone()),
                    (_, Some(target)) => Command::Move(ids.clone(), target),
                    _ => return,
                };
                // Deleting on an account without a Trash folder, or in
                // Trash itself, is for good (as the daemon does it), so
                // there is nothing to undo.
                let undo = folder
                    .filter(|_| !for_good)
                    .map(|folder| Command::Move(ids, folder));
                self.remove_lines(&keys);
                (command, undo)
            }
        };
        let done = match act {
            Act::Spam => Some(format!("{what} reported as spam.")),
            Act::Delete if for_good => Some(format!("{what} deleted forever.")),
            _ => command.done_text(&what),
        };
        // Moved out of a conversation window, which now closes: the mail
        // window says so and offers Undo.
        if self.detached
            && self.reader.is_none()
            && let Some(main) = self.main.as_ref().and_then(WeakEntity::upgrade)
        {
            main.update(cx, |main, cx| main.send(command, done, undo, false, cx));
            cx.notify();
            return;
        }
        self.send(command, done, undo, false, cx);
        cx.notify();
    }

    /// Takes lines out of the list, keeping the cursor on the next one.
    fn remove_lines(&mut self, keys: &[EntryKey]) {
        let cursor = self.selected.unwrap_or(0);
        let removed_before = self.entries[..cursor.min(self.entries.len())]
            .iter()
            .filter(|e| keys.contains(&e.key))
            .count();
        for ix in (0..self.entries.len()).rev() {
            if keys.contains(&self.entries[ix].key) {
                self.list_state.splice(ix..ix + 1, 0);
            }
        }
        self.entries.retain(|e| !keys.contains(&e.key));
        self.selected = if self.entries.is_empty() {
            None
        } else {
            Some((cursor - removed_before).min(self.entries.len() - 1))
        };
        for key in keys {
            self.checked.remove(key);
        }
        self.checked_all = false;
        self.page_pick = None;
        if self.reader.as_ref().is_some_and(|r| keys.contains(&r.key)) {
            self.show_list();
            self.reader = None;
        }
        self.hovered = None;
    }

    /// Sends `command` to the daemon. Shows `done` when it is applied, and
    /// the error if it fails, unless `quiet`.
    fn send(
        &mut self,
        command: Command,
        done: Option<String>,
        undo: Option<Command>,
        quiet: bool,
        cx: &mut Context<Self>,
    ) {
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::send(&connection, &command).await
                })
                .await;
            this.update(cx, |this, cx| match result {
                Ok(()) => {
                    if let Some(done) = done {
                        this.show_snackbar(done, undo, cx);
                    }
                }
                Err(err) => {
                    tracing::info!("{err}");
                    if !quiet {
                        this.show_snackbar(err, None, cx);
                        // Show the store as it is again.
                        this.refresh(false, cx);
                    }
                }
            })
            .ok();
        })
        .detach();
    }

    fn undo(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(undo) = self.snackbar.as_mut().and_then(|s| s.undo.take()) else {
            return;
        };
        self.hide_snackbar(cx);
        if let Command::UndoSend(_) = undo {
            // Taken back from the outbox: the message opens again.
            let connection = self.daemon.clone();
            cx.spawn_in(window, async move |this, cx| {
                let result = cx
                    .background_executor()
                    .spawn(async move {
                        let connection = match connection {
                            Some(connection) => connection,
                            None => daemon::connect().await?,
                        };
                        daemon::send(&connection, &undo).await
                    })
                    .await;
                this.update_in(cx, |this, window, cx| match result {
                    Ok(()) => {
                        this.reopen_unsent(window, cx);
                        this.show_snackbar("Sending undone.", None, cx);
                    }
                    Err(err) => this.show_snackbar(err, None, cx),
                })
                .ok();
            })
            .detach();
            return;
        }
        self.send(undo, Some("Action undone.".to_owned()), None, false, cx);
    }

    fn archive(&mut self, _: &Archive, _: &mut Window, cx: &mut Context<Self>) {
        self.act_on_targets(Act::Archive, cx);
    }

    fn delete(&mut self, _: &Delete, _: &mut Window, cx: &mut Context<Self>) {
        self.act_on_targets(Act::Delete, cx);
    }

    fn report_spam(&mut self, _: &ReportSpam, _: &mut Window, cx: &mut Context<Self>) {
        self.act_on_targets(Act::Spam, cx);
    }

    fn mark_read(&mut self, _: &MarkRead, _: &mut Window, cx: &mut Context<Self>) {
        self.act_on_targets(Act::Read(true), cx);
    }

    fn mark_unread(&mut self, _: &MarkUnread, window: &mut Window, cx: &mut Context<Self>) {
        let reading = self.reading && self.checked.is_empty();
        self.act_on_targets(Act::Read(false), cx);
        // As in webmail, marking the open conversation unread closes it.
        if reading {
            self.close_message(&CloseMessage, window, cx);
            self.reader = None;
        }
    }

    fn toggle_star(&mut self, _: &ToggleStar, _: &mut Window, cx: &mut Context<Self>) {
        let keys = self.target_keys();
        let on = !keys.iter().all(|k| self.is_flagged(*k));
        self.act(Act::Star(on), keys, cx);
    }

    fn mark_important(&mut self, _: &MarkImportant, _: &mut Window, cx: &mut Context<Self>) {
        self.act_on_targets(Act::Important(true), cx);
    }

    fn mark_not_important(&mut self, _: &MarkNotImportant, _: &mut Window, cx: &mut Context<Self>) {
        self.act_on_targets(Act::Important(false), cx);
    }

    fn toggle_check(&mut self, _: &ToggleCheck, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(entry) = self.selected.and_then(|ix| self.entries.get(ix)) {
            let key = entry.key;
            if !self.checked.remove(&key) {
                self.checked.insert(key);
            }
            self.checked_all = false;
            self.page_pick = None;
            cx.notify();
        }
    }

    /// Whether line `key` is starred, counting changes not read back yet.
    fn is_flagged(&mut self, key: EntryKey) -> bool {
        if let Some(flagged) = self.pending.get(&key).and_then(|p| p.flagged) {
            return flagged;
        }
        let Some(entry) = self.entries.iter().find(|e| e.key == key).copied() else {
            return false;
        };
        let folder = self.listed_folder();
        match &mut self.mail {
            Ok(mail) => mail
                .rows(&[entry], folder, self.show_recipients)
                .into_iter()
                .flatten()
                .any(|r| r.flagged),
            Err(_) => false,
        }
    }

    fn on_split_drag(&mut self, event: &MouseMoveEvent, cx: &mut Context<Self>) {
        let Some((start_x, start_share)) = self.split_drag else {
            return;
        };
        if event.pressed_button != Some(MouseButton::Left) {
            self.split_drag = None;
            self.save_config();
            return;
        }
        let width = (self.cards_width - SPLIT_GAP).max(1.0);
        let dx = f32::from(event.position.x) - start_x;
        let share = (start_share - dx / width).clamp(0.25, 0.75);
        self.config.mail.reading_pane_share = share;
        cx.notify();
    }

    fn render_snackbar(
        &mut self,
        th: &Theme,
        window: &Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let snackbar = self.snackbar.as_mut()?;
        let s = snackbar.shown.tick(window, reduce);
        if snackbar.shown.target() == 0.0 && snackbar.shown.settled() {
            self.snackbar = None;
            return None;
        }
        let s = s.max(0.0);
        let has_undo = snackbar.undo.is_some();
        // On a phone the note spans the window above the bottom bar.
        let shape = self.layout.shape;
        let edge = lerp(24.0, 8.0, shape.phone);
        Some(
            div()
                .absolute()
                .left(px(edge))
                .when(shape.is_phone(), |d| d.right(px(edge)))
                .bottom(px(shape.bottom_bar() + lerp(-12.0, edge, s)))
                .opacity(s.min(1.0))
                .min_w(px(288.0))
                .max_w(px(560.0))
                .pl(px(16.0))
                .pr(px(if has_undo { 8.0 } else { 16.0 }))
                .py(px(if has_undo { 6.0 } else { 14.0 }))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(24.0))
                .rounded(px(6.0))
                .bg(rgba(th.snackbar))
                .text_color(rgba(th.snackbar_text))
                .text_size(px(14.0))
                .shadow(elevation(th, 3.0))
                .child(div().flex_1().min_w_0().child(snackbar.text.clone()))
                .when(has_undo, |d| {
                    d.child(
                        div()
                            .id("undo")
                            .px(px(12.0))
                            .py(px(8.0))
                            .rounded(px(4.0))
                            .text_color(rgba(if th.dark { th.nav_selected } else { 0xa8c7faff }))
                            .font_weight(gpui::FontWeight::MEDIUM)
                            .cursor_pointer()
                            .hover(|s| s.bg(rgba(0xffffff1f)))
                            .on_click(cx.listener(|this, _, window, cx| this.undo(window, cx)))
                            .child("Undo"),
                    )
                })
                .into_any_element(),
        )
    }

    fn render_error(&self, error: &OpenError, th: &Theme) -> AnyElement {
        let err = match error {
            // The daemon makes the store when the first account is added;
            // until then the first-start pages show.
            OpenError::NoStore { .. } => return div().into_any_element(),
            OpenError::Other(err) => err.clone(),
        };
        let card = page_card(th)
            .child(icon("mail", th.text_faint, 64.0))
            .child(
                div()
                    .text_size(px(22.0))
                    .text_color(rgba(th.text))
                    .child("The mail store could not be opened"),
            )
            .child(
                div()
                    .max_w(px(460.0))
                    .text_size(px(14.0))
                    .line_height(px(21.0))
                    .text_color(rgba(th.text_faint))
                    .text_center()
                    .child(err),
            );
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(card)
            .into_any_element()
    }

    /// The list and, with three panes, the reading pane beside it.
    fn render_cards(&mut self, th: &Theme, available: f32, cx: &mut Context<Self>) -> AnyElement {
        let split = self.split();
        let pane_t = self.pane_spring.value().max(0.0);
        let share = self.config.mail.reading_pane_share;
        let pane_width = ((available - SPLIT_GAP) * share).max(0.0);
        let list = self.render_list_card(th, cx);
        let row = div()
            .id("cards")
            .size_full()
            .flex()
            .flex_row()
            .on_mouse_move(
                cx.listener(|this, event: &MouseMoveEvent, _, cx| this.on_split_drag(event, cx)),
            )
            .child(div().flex_1().min_w_0().h_full().child(list));
        let row = if split && pane_t > 0.001 {
            let handle = div()
                .id("split-handle")
                .flex_none()
                .w(px(SPLIT_GAP * pane_t.min(1.0)))
                .h_full()
                .flex()
                .items_center()
                .justify_center()
                .cursor_col_resize()
                .group("split")
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, event: &gpui::MouseDownEvent, _, cx| {
                        this.split_drag = Some((
                            f32::from(event.position.x),
                            this.config.mail.reading_pane_share,
                        ));
                        cx.stop_propagation();
                    }),
                )
                .child(
                    div()
                        .w(px(4.0))
                        .h(px(40.0))
                        .rounded_full()
                        .bg(rgba(th.divider))
                        .group_hover("split", |s| s.bg(rgba(th.text_faint))),
                );
            // Clipped only from the side it slides in from, and a little
            // wider than the card, so the card's shadow is never cut.
            let room = crate::widgets::CARD_SHADOW_ROOM;
            let pane = div()
                .flex_none()
                .h_full()
                .w(px(pane_width * pane_t + 2.0 * room))
                .mx(px(-room))
                .px(px(room))
                .overflow_x_hidden()
                .child(
                    div()
                        .w(px(pane_width))
                        .h_full()
                        .ml(px(24.0 * (1.0 - pane_t.min(1.0))))
                        .opacity(pane_t.min(1.0))
                        .child(self.render_reader_card(th, cx)),
                );
            row.child(handle).child(pane)
        } else {
            row
        };
        // Padding, not margins: a margin would push the card past the window.
        let margin = self.layout.shape.card_margin();
        div()
            .flex_1()
            .min_w_0()
            .h_full()
            .pr(px(margin))
            .pb(px(margin))
            .child(row)
            .into_any_element()
    }
}

impl Render for MailWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.chrome.sync_look(window, cx);
        if self.detached {
            let detached = self.render_detached(window, cx);
            self.fetch_pictures(cx);
            return detached;
        }
        self.tour_new_frame();
        let th = self.theme(window);
        self.release_images(window, cx);
        if let Some(viewer) = &self.files.viewer {
            viewer.update(cx, |viewer, _| viewer.th = th);
        }
        let reduce = cx.reduce_motion();
        self.update_layout(window, reduce, cx);
        let shape = self.layout.shape;
        let width = shape.width;

        self.nav_spring.set(
            if self.nav_docked() || self.nav_peek || self.layout.drawer {
                1.0
            } else {
                0.0
            },
        );
        self.reserve_spring
            .set(if self.nav_docked() { 1.0 } else { 0.0 });
        let search_focused = self.search.focus_handle(cx).is_focused(window);
        self.search_spring
            .set(if search_focused { 1.0 } else { 0.0 });
        let pane_open = self.split() && self.reading && self.reader.is_some();
        self.pane_spring.set(if pane_open { 1.0 } else { 0.0 });
        self.settings_spring
            .set(if self.settings_open { 1.0 } else { 0.0 });
        self.search_panel_spring
            .set(if self.search_panel.is_some() {
                1.0
            } else {
                0.0
            });
        self.tab_spring.set(self.tab as f32);
        self.nav_t = self.nav_spring.tick(window, reduce);
        let reserve = self.reserve_spring.tick(window, reduce);
        let search_t = self.search_spring.tick(window, reduce);
        let pane_t = self.pane_spring.tick(window, reduce);
        let settings_t = self.settings_spring.tick(window, reduce);
        self.search_panel_spring.tick(window, reduce);
        self.tab_spring.tick(window, reduce);
        // Forget the closed conversation once its pane has slid away.
        if !self.reading
            && self.reader.is_some()
            && if self.split() {
                pane_t <= 0.0 && self.pane_spring.settled()
            } else {
                self.slides() && self.page_closed()
            }
        {
            self.reader = None;
        }

        let accent: Hsla = rgba(th.accent).into();
        self.search
            .update(cx, |search, _| search.set_accent(accent));
        self.sync_search_box(cx);

        // Widths: the cards get what the navigation and settings leave. On a
        // phone the settings float over the cards instead.
        let nav_width = NAV_WIDTH * reserve.max(0.0);
        let settings_floats = shape.is_phone();
        let settings_width = if settings_floats {
            0.0
        } else {
            SETTINGS_WIDTH * settings_t.clamp(0.0, 1.0)
        };
        let available =
            (width - shape.rail() - nav_width - shape.card_margin() - settings_width).max(200.0);
        self.cards_width = available;
        let reader_width = if self.split() {
            ((available - SPLIT_GAP) * self.config.mail.reading_pane_share).max(0.0)
        } else {
            available
        };
        self.update_reply_row(reader_width, window, reduce);
        let (rail, margin) = if shape.is_phone() {
            (0.0, 0.0)
        } else {
            (apps::APP_RAIL_WIDTH, 16.0)
        };
        let nav = if self.nav_docked() { NAV_WIDTH } else { 0.0 };
        let settings = if self.settings_open && !settings_floats {
            SETTINGS_WIDTH
        } else {
            0.0
        };
        self.cards_target = (width - rail - nav - margin - settings).max(200.0);

        let settings = (settings_t > 0.001).then(|| self.render_settings(&th, settings_t, cx));
        let (docked_settings, floating_settings) = if settings_floats {
            (None, settings)
        } else {
            (settings, None)
        };
        if self.onboarding.is_none() && self.needs_account() {
            self.onboarding = Some(onboarding::Onboarding::new());
        }
        // The Settings page stays reachable, for example to delete data.
        let onboarding = self.onboarding() && self.settings_page.is_none();
        let content = match &self.mail {
            _ if onboarding => self.render_onboarding(&th, window, cx),
            Err(err) => self.render_error(err, &th),
            // Reversed so the navigation paints last, over the cards, when
            // it opens from the rail.
            Ok(_) if self.app == RailApp::Mail => div()
                .size_full()
                .flex()
                .flex_row_reverse()
                .children(docked_settings)
                .child(if self.settings_page.is_some() {
                    self.render_settings_page(&th, cx)
                } else {
                    self.render_cards(&th, available, cx)
                })
                .child(self.render_navigation(&th, cx))
                .child(self.render_rail_slot(&th, cx))
                .into_any_element(),
            Ok(_) => div()
                .size_full()
                .flex()
                .flex_row_reverse()
                .children(docked_settings)
                .child(self.render_app_page(&th, cx))
                .child(self.render_rail_slot(&th, cx))
                .into_any_element(),
        };
        // The apps move to a bar along the bottom on a phone.
        let content = div()
            .size_full()
            .flex()
            .flex_col()
            .child(div().flex_1().min_h_0().child(content))
            .children(if onboarding {
                None
            } else {
                self.render_bottom_bar(&th, cx)
            })
            .into_any_element();
        let floating_settings = floating_settings.map(|panel| {
            div()
                .absolute()
                .top_0()
                .left_0()
                .right_0()
                .bottom(px(shape.bottom_bar()))
                .overflow_hidden()
                .child(panel)
                .into_any_element()
        });

        // On a desktop the search box stays where the list starts with the
        // folders open, whether they are open or folded, and never moves
        // with them. On a tablet (Compose beside the menu button, the
        // folders in a drawer) it starts a clear gap after Compose, and it
        // never comes closer than that. It grows into a pill across the top
        // bar of a phone, under its menu button and account picture.
        let (room_start, room_end) = shape.room;
        let compose_text = compose_text_width(self.font.as_ref(), window);
        let after_compose = room_start
            + COMPOSE_LEFT
            + compose_width(shape.compose_label(), compose_text)
            + TOP_BAR_GAP;
        let list_left = if shape.is_desktop() {
            shape.rail() + NAV_WIDTH
        } else {
            0.0
        };
        let search_left = list_left.max(after_compose);
        let regular = (width - search_left - room_end - TOP_END_WIDTH - TOP_BAR_GAP)
            .clamp(200.0, SEARCH_WIDTH);
        let pill = (width - 12.0 - room_start - room_end).max(200.0);
        let search_width = lerp(regular, pill, shape.phone);
        let search_panel_width = lerp(regular, width - 16.0, shape.phone);
        let search_panel_left = lerp(search_left, 8.0, shape.phone);
        let search_panel = self
            .render_search_panel(&th, search_panel_left, search_panel_width, window, cx)
            .map(|panel| self.search_panel_layer(panel, cx));
        let fab = if onboarding {
            None
        } else {
            self.render_phone_fab(&th, cx)
        };
        let compose = self.render_compose(&th, window, reduce, cx);
        let scheduled = self.render_scheduled(&th, window, cx);
        let account_menu = self.render_account_menu(&th, cx);
        let add_account = self.render_add_account(&th, window, reduce, cx);
        let danger = self.render_danger(&th, window, reduce, cx);
        let new_label = self.render_new_label(&th, window, reduce, cx);
        let whats_new = self.render_whats_new(&th, window, reduce, cx);
        let context_menu = self.render_context_menu(&th, window, cx);
        let snackbar = self.render_snackbar(&th, window, reduce, cx);
        let crash_notice = if onboarding {
            None
        } else {
            self.render_crash_notice(&th, window, reduce, cx)
        };
        let tour = self.render_tour(&th, window, cx);
        let content = div()
            .key_context(WINDOW_CONTEXT)
            .relative()
            .size_full()
            .bg(rgba(th.backdrop))
            .text_color(rgba(th.text))
            .on_action(cx.listener(Self::focus_next))
            .on_action(cx.listener(Self::focus_previous))
            .on_action(cx.listener(Self::focus_search))
            .on_action(cx.listener(Self::focus_list))
            .on_action(cx.listener(Self::toggle_navigation))
            .on_action(cx.listener(Self::toggle_settings))
            .on_action(cx.listener(Self::compose))
            .on_action(cx.listener(Self::reload))
            .on_action(cx.listener(Self::quit))
            .on_action(cx.listener(Self::reply))
            .on_action(cx.listener(Self::reply_all))
            .on_action(cx.listener(Self::forward))
            .on_action(cx.listener(Self::move_to))
            .on_action(cx.listener(Self::select_all))
            .on_action(cx.listener(Self::select_none))
            .on_action(cx.listener(Self::undo_action))
            .on_action(cx.listener(Self::go_to_inbox))
            .on_action(cx.listener(Self::go_to_starred))
            .on_action(cx.listener(Self::go_to_sent))
            .on_action(cx.listener(Self::go_to_drafts))
            .on_action(cx.listener(Self::go_to_all_mail))
            .on_action(cx.listener(Self::open_settings))
            .on_action(cx.listener(Self::show_shortcuts))
            .child(content)
            .children(floating_settings)
            .children(fab)
            .children(self.files.viewer.clone())
            .children(search_panel)
            .children(compose)
            .children(scheduled)
            .children(account_menu)
            .children(add_account)
            .children(context_menu)
            .children(danger)
            .children(new_label)
            .children(crash_notice)
            .children(whats_new)
            .children(snackbar)
            .children(tour)
            .into_any_element();

        // The first-start pages keep the bar empty.
        let bar = Bar {
            start: if onboarding {
                Vec::new()
            } else {
                self.render_top_start(&th, compose_text, cx)
            },
            center: (self.mail.is_ok() && !onboarding).then(|| {
                div()
                    .w_full()
                    .pl(px(lerp(search_left, 6.0 + room_start, shape.phone)))
                    .child(self.render_search(&th, search_width, search_t, cx))
                    .into_any_element()
            }),
            end: if onboarding {
                Vec::new()
            } else {
                self.render_top_end(&th, cx)
            },
            height: Some(TOP_BAR_HEIGHT),
            background: Some(th.backdrop),
        };
        // Pictures of people asked for while drawing.
        self.fetch_pictures(cx);
        let frame = self.chrome.render_bar(bar, content, window, cx);
        // A phone's top bar slides up out of the window as the list moves
        // on; the content below takes its room.
        let hidden = shape.top_bar_hidden();
        let frame = if hidden > 0.01 {
            div().size_full().overflow_hidden().child(
                frame
                    .mt(px(-hidden))
                    .h(window.viewport_size().height + px(hidden)),
            )
        } else {
            frame
        };
        match &self.font {
            Some(font) => frame.font_family(font.clone()).into_any_element(),
            None => frame.into_any_element(),
        }
    }
}

impl Focusable for MailWindow {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.list_focus.clone()
    }
}

/// The card that fills the page for the welcome and error pages.
fn page_card(th: &Theme) -> gpui::Div {
    div()
        .flex_1()
        .min_h_0()
        .mx(px(16.0))
        .mb(px(16.0))
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(12.0))
        .rounded(px(PANEL_RADIUS))
        .bg(rgba(th.surface))
}
