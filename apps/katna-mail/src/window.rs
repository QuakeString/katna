// SPDX-License-Identifier: GPL-3.0-or-later

//! The main window: sidebar, message list and reading pane
//! (`docs/ARCHITECTURE.md` §13.3), inside the window chrome.

use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::rc::Rc;
use std::time::Duration;

use gpui::{
    AnyElement, App, Context, Entity, FocusHandle, Focusable, FontWeight, Hsla, KeyBinding,
    MouseButton, Render, ScrollHandle, ScrollStrategy, SharedString, Subscription, Task,
    UniformListScrollHandle, Window, actions, div, prelude::*, px, rgba, svg, uniform_list,
};
use jiff::tz::TimeZone;
use katna_chrome::tokens::with_alpha;
use katna_chrome::{ChromeTokens, Environment, WindowChrome};
use katna_core::Paths;
use katna_render::MessageView;
use katna_search::SearchResults;
use katna_store::{FolderId, MessageId};
use katna_ui::{InputEvent, TextInput};

use crate::data::{self, Mail, OpenError, Row};
use crate::format;
use crate::sidebar::{self, Role, Tree};

actions!(
    katna_mail,
    [
        SelectNext,
        SelectPrevious,
        SelectFirst,
        SelectLast,
        PageDown,
        PageUp,
        FocusSearch,
        FocusList,
        Reload,
        Quit,
    ]
);

const WINDOW_CONTEXT: &str = "MailWindow";
const LIST_CONTEXT: &str = "MessageList";
const SEARCH_CONTEXT: &str = "SearchBox";

const SIDEBAR_WIDTH: f32 = 248.0;
const LIST_WIDTH: f32 = 400.0;
const SIDEBAR_ROW_HEIGHT: f32 = 32.0;
const MESSAGE_ROW_HEIGHT: f32 = 72.0;
const PAGE: usize = 10;
/// Wait this long after a keystroke before searching, so fast typing
/// searches once.
const SEARCH_DELAY: Duration = Duration::from_millis(60);
/// The reading pane shows at most this many lines of a body.
const MAX_BODY_LINES: usize = 4000;

/// Binds the window's keys. Call once at startup.
pub fn bind_keys(cx: &mut App) {
    let list = Some(LIST_CONTEXT);
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
        KeyBinding::new("/", FocusSearch, list),
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

/// The message in the reading pane.
struct Reader {
    id: MessageId,
    /// `None` when the message body is not stored (not downloaded yet).
    view: Option<MessageView>,
    /// The body, in blocks of consecutive quoted or unquoted lines.
    blocks: Vec<(bool, SharedString)>,
    cut: bool,
}

pub struct MailWindow {
    chrome: WindowChrome,
    /// The desktop's UI font, or `None` to leave GPUI's default.
    font: Option<SharedString>,
    paths: Paths,
    mail: Result<Mail, OpenError>,
    tree: Tree,
    /// Unread mail per folder, counted in the background.
    unread: HashMap<FolderId, u64>,
    unread_task: Option<Task<()>>,
    expanded: HashSet<String>,
    sidebar_rows: Vec<sidebar::Row>,
    folder: Option<FolderId>,
    show_recipients: bool,
    listing: Option<Listing>,
    ids: Vec<MessageId>,
    selected: Option<usize>,
    reader: Option<Reader>,
    search: Entity<TextInput>,
    search_error: Option<SharedString>,
    search_task: Option<Task<()>>,
    list_focus: FocusHandle,
    list_scroll: UniformListScrollHandle,
    sidebar_scroll: UniformListScrollHandle,
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
            paths,
            tree: Tree::default(),
            unread: HashMap::new(),
            unread_task: None,
            expanded: HashSet::new(),
            sidebar_rows: Vec::new(),
            folder: None,
            show_recipients: false,
            listing: None,
            ids: Vec::new(),
            selected: None,
            reader: None,
            search,
            search_error: None,
            search_task: None,
            list_focus: cx.focus_handle(),
            list_scroll: UniformListScrollHandle::new(),
            sidebar_scroll: UniformListScrollHandle::new(),
            reader_scroll: ScrollHandle::new(),
            tz: TimeZone::try_system().unwrap_or(TimeZone::UTC),
            _subscriptions: subscriptions,
        };
        this.load_tree();
        if let Some((folder, ancestors)) = this.tree.default_folder() {
            this.expanded.extend(ancestors);
            this.rebuild_sidebar();
            this.open_folder(folder, cx);
        }
        this.count_unread(cx);
        let index_error = this
            .mail
            .as_ref()
            .ok()
            .and_then(|m| m.index_error().map(str::to_owned));
        if let Some(err) = index_error {
            tracing::info!("search is off: {err}");
            this.search.update(cx, |search, _| {
                search.set_placeholder("Search is not available");
            });
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
        self.tree = Tree::build(&mail.accounts(), &mail.folders(), &self.unread);
        self.expanded = self.tree.initially_expanded();
        self.rebuild_sidebar();
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
                    this.tree = Tree::build(&mail.accounts(), &mail.folders(), &this.unread);
                    this.rebuild_sidebar();
                }
                cx.notify();
            })
            .ok();
        }));
    }

    fn rebuild_sidebar(&mut self) {
        self.sidebar_rows = self.tree.rows(&self.expanded);
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
        if self.ids.is_empty() {
            self.selected = None;
            self.reader = None;
        } else {
            self.select(0, cx);
        }
        cx.notify();
    }

    fn select(&mut self, ix: usize, cx: &mut Context<Self>) {
        let Some(&id) = self.ids.get(ix) else {
            return;
        };
        self.selected = Some(ix);
        self.list_scroll.scroll_to_item(ix, ScrollStrategy::Nearest);
        if self.reader.as_ref().is_none_or(|r| r.id != id) {
            self.reader = self.mail.as_ref().ok().map(|mail| read(mail, id));
            self.reader_scroll.set_offset(gpui::point(px(0.0), px(0.0)));
        }
        cx.notify();
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

    fn reload(&mut self, _: &Reload, _: &mut Window, cx: &mut Context<Self>) {
        if self.mail.is_err() {
            self.mail = Mail::open(&self.paths);
            self.load_tree();
            if let Some((folder, ancestors)) = self.tree.default_folder() {
                self.expanded.extend(ancestors);
                self.rebuild_sidebar();
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
        self.rebuild_sidebar();
        let selected_id = self.selected.and_then(|ix| self.ids.get(ix).copied());
        match self.listing.clone() {
            Some(Listing::Folder(folder)) => {
                if let Ok(mail) = &self.mail {
                    self.ids = mail.folder_message_ids(folder);
                }
                self.selected = selected_id.and_then(|id| self.ids.iter().position(|i| *i == id));
            }
            Some(Listing::Search { query, .. }) => self.start_search(query, cx),
            None => {}
        }
        self.count_unread(cx);
        cx.notify();
    }

    fn quit(&mut self, _: &Quit, _: &mut Window, cx: &mut Context<Self>) {
        cx.quit();
    }

    fn click_sidebar_row(&mut self, ix: usize, cx: &mut Context<Self>) {
        let Some(sidebar::Row::Folder { key, folder, .. }) = self.sidebar_rows.get(ix).cloned()
        else {
            return;
        };
        match folder {
            Some(folder) => {
                self.clear_search(cx);
                self.open_folder(folder, cx);
            }
            None => self.toggle(&key, cx),
        }
    }

    fn toggle(&mut self, key: &str, cx: &mut Context<Self>) {
        if !self.expanded.remove(key) {
            self.expanded.insert(key.to_owned());
        }
        self.rebuild_sidebar();
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
        let Some(index) = self.mail.as_ref().ok().and_then(Mail::index) else {
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
                self.ids = results.hits.iter().map(|hit| hit.message).collect();
                self.listing = Some(Listing::Search {
                    query,
                    total: results.total,
                });
                self.selected = None;
                self.list_scroll.scroll_to_item(0, ScrollStrategy::Top);
            }
            Err(err) => self.search_error = Some(err.into()),
        }
        cx.notify();
    }

    fn list_title(&self) -> (SharedString, SharedString) {
        match &self.listing {
            Some(Listing::Folder(folder)) => {
                let node = self.tree.node(*folder);
                let name = node.map_or("Folder".to_owned(), |n| n.name.clone());
                let count = self.ids.len() as u64;
                let unread = node.map_or(0, |n| n.unread);
                let detail = match (count, unread) {
                    (0, _) => "Empty".to_owned(),
                    (1, _) => "1 message".to_owned(),
                    (n, 0) => format!("{} messages", format::thousands(n)),
                    (n, u) => format!(
                        "{} messages, {} unread",
                        format::thousands(n),
                        format::thousands(u)
                    ),
                };
                (name.into(), detail.into())
            }
            Some(Listing::Search { query, total }) => {
                let shown = self.ids.len();
                let detail = match total {
                    Some(0) | None if shown == 0 => format!("No results for “{query}”"),
                    Some(total) if *total > shown => format!(
                        "First {} of {} results",
                        format::thousands(shown as u64),
                        format::thousands(*total as u64)
                    ),
                    _ if shown == 1 => "1 result".to_owned(),
                    _ => format!("{} results", format::thousands(shown as u64)),
                };
                ("Search".into(), detail.into())
            }
            None => ("Katna Mail".into(), SharedString::default()),
        }
    }

    fn render_search_box(&self, t: &ChromeTokens) -> AnyElement {
        let available = self.mail.as_ref().is_ok_and(|m| m.index().is_some());
        div()
            .key_context(SEARCH_CONTEXT)
            .w(px(300.0))
            .h(px(30.0))
            .px(px(8.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(6.0))
            .rounded(px(if t.window_radius > 8.0 { 8.0 } else { 4.0 }))
            .bg(rgba(if t.dark {
                with_alpha(t.fg, 0x14)
            } else {
                t.view_bg
            }))
            .border_1()
            .border_color(rgba(with_alpha(t.fg, 0x26)))
            .text_size(px(14.0))
            .line_height(px(20.0))
            .when(!available, |d| d.opacity(0.6))
            .child(icon("search", t.fg_dim, 14.0))
            .child(self.search.clone())
            .into_any_element()
    }

    fn render_sidebar(&self, t: &ChromeTokens, cx: &mut Context<Self>) -> AnyElement {
        div()
            .w(px(SIDEBAR_WIDTH))
            .flex_none()
            .h_full()
            .bg(rgba(t.sidebar_bg))
            .border_r_1()
            .border_color(rgba(t.header_shade))
            .child(
                uniform_list(
                    "sidebar",
                    self.sidebar_rows.len(),
                    cx.processor(|this, range: Range<usize>, window, cx| {
                        let t = this.chrome.tokens(window);
                        range
                            .map(|ix| this.render_sidebar_row(ix, &t, cx))
                            .collect::<Vec<_>>()
                    }),
                )
                .track_scroll(&self.sidebar_scroll)
                .size_full()
                .p(px(6.0)),
            )
            .into_any_element()
    }

    fn render_sidebar_row(
        &self,
        ix: usize,
        t: &ChromeTokens,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let row = div()
            .id(("sidebar-row", ix))
            .w_full()
            .h(px(SIDEBAR_ROW_HEIGHT))
            .px(px(8.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(6.0))
            .rounded(px(6.0))
            .text_size(px(14.0));
        match &self.sidebar_rows[ix] {
            sidebar::Row::Account { name, .. } => row
                .pt(px(8.0))
                .text_size(px(12.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgba(t.fg_dim))
                .child(div().flex_1().truncate().child(name.to_uppercase()))
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
                let selected = folder.is_some() && *folder == self.folder;
                let key = key.clone();
                let chevron = div()
                    .id(("sidebar-chevron", ix))
                    .w(px(16.0))
                    .flex_none()
                    .when(*has_children, |d| {
                        d.child(icon(
                            if *expanded {
                                "chevron-down"
                            } else {
                                "chevron-right"
                            },
                            t.fg_dim,
                            12.0,
                        ))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            cx.stop_propagation();
                            this.toggle(&key, cx);
                        }))
                    });
                row.pl(px(4.0 + 14.0 * *depth as f32))
                    .when(selected, |d| d.bg(rgba(with_alpha(t.accent, 0x33))))
                    .when(!selected, |d| {
                        d.hover(|s| s.bg(rgba(with_alpha(t.fg, 0x0f))))
                    })
                    .on_click(cx.listener(move |this, _, _, cx| this.click_sidebar_row(ix, cx)))
                    .child(chevron)
                    .child(icon(role_icon(*role), t.fg_dim, 16.0))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .truncate()
                            .when(*unread > 0, |d| d.font_weight(FontWeight::SEMIBOLD))
                            .child(label.clone()),
                    )
                    .when(*unread > 0, |d| {
                        d.child(
                            div()
                                .flex_none()
                                .text_size(px(12.0))
                                .text_color(rgba(t.fg_dim))
                                .child(format::thousands(*unread)),
                        )
                    })
                    .into_any_element()
            }
        }
    }

    fn render_list(&self, t: &ChromeTokens, cx: &mut Context<Self>) -> AnyElement {
        let (title, detail) = self.list_title();
        let header = div()
            .flex_none()
            .h(px(52.0))
            .px(px(16.0))
            .flex()
            .flex_col()
            .justify_center()
            .border_b_1()
            .border_color(rgba(t.header_shade))
            .child(
                div()
                    .text_size(px(15.0))
                    .font_weight(FontWeight::BOLD)
                    .truncate()
                    .child(title),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgba(t.fg_dim))
                    .truncate()
                    .child(self.search_error.clone().unwrap_or(detail)),
            );
        let body = if self.ids.is_empty() {
            let text = match &self.listing {
                Some(Listing::Search { .. }) => "No messages match this search.",
                Some(Listing::Folder(_)) => "No messages in this folder.",
                None => "",
            };
            placeholder(text, t)
        } else {
            uniform_list(
                "messages",
                self.ids.len(),
                cx.processor(|this, range: Range<usize>, window, cx| {
                    let t = this.chrome.tokens(window);
                    let rows = match &mut this.mail {
                        Ok(mail) => mail.rows(&this.ids[range.clone()], this.show_recipients),
                        Err(_) => vec![None; range.len()],
                    };
                    range
                        .zip(rows)
                        .map(|(ix, row)| this.render_message_row(ix, row, &t, cx))
                        .collect::<Vec<_>>()
                }),
            )
            .track_scroll(&self.list_scroll)
            .flex_1()
            .into_any_element()
        };
        div()
            .id("message-list")
            .key_context(LIST_CONTEXT)
            .track_focus(&self.list_focus)
            .w(px(LIST_WIDTH))
            .flex_none()
            .h_full()
            .flex()
            .flex_col()
            .bg(rgba(t.view_bg))
            .border_r_1()
            .border_color(rgba(t.header_shade))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::page_down))
            .on_action(cx.listener(Self::page_up))
            .child(header)
            .child(body)
            .into_any_element()
    }

    fn render_message_row(
        &self,
        ix: usize,
        row: Option<Rc<Row>>,
        t: &ChromeTokens,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let selected = self.selected == Some(ix);
        let base = div()
            .id(("message-row", ix))
            .h(px(MESSAGE_ROW_HEIGHT))
            .px(px(16.0))
            .flex()
            .flex_col()
            .justify_center()
            .gap(px(1.0))
            .border_b_1()
            .border_color(rgba(t.header_shade))
            .when(selected, |d| d.bg(rgba(with_alpha(t.accent, 0x2e))))
            .when(!selected, |d| {
                d.hover(|s| s.bg(rgba(with_alpha(t.fg, 0x0a))))
            })
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, window, cx| {
                    window.focus(&this.list_focus, cx);
                    this.select(ix, cx);
                }),
            );
        let Some(row) = row else {
            return base
                .text_color(rgba(t.fg_dim))
                .text_size(px(13.0))
                .child("This message was removed.")
                .into_any_element();
        };
        let date = row
            .date
            .and_then(|d| format::local(d, &self.tz))
            .zip(format::local(jiff::Timestamp::now().as_second(), &self.tz))
            .map(|(d, now)| format::list_date(d, now))
            .unwrap_or_default();
        let weight = if row.unread {
            FontWeight::BOLD
        } else {
            FontWeight::NORMAL
        };
        let top = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(6.0))
            .text_size(px(14.0))
            .child(
                div()
                    .size(px(8.0))
                    .flex_none()
                    .rounded_full()
                    .when(row.unread, |d| d.bg(rgba(t.accent))),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .truncate()
                    .font_weight(weight)
                    .child(row.correspondent.clone()),
            )
            .when(row.attachments, |d| {
                d.child(icon("attachment", t.fg_dim, 14.0))
            })
            .when(row.flagged, |d| d.child(icon("star", STAR, 14.0)))
            .child(
                div()
                    .flex_none()
                    .text_size(px(12.0))
                    .text_color(rgba(if row.unread { t.accent } else { t.fg_dim }))
                    .child(date),
            );
        base.child(top)
            .child(
                div()
                    .pl(px(14.0))
                    .text_size(px(13.0))
                    .truncate()
                    .when(row.unread, |d| d.font_weight(FontWeight::SEMIBOLD))
                    .child(row.subject.clone()),
            )
            .child(
                div()
                    .pl(px(14.0))
                    .text_size(px(13.0))
                    .text_color(rgba(t.fg_dim))
                    .truncate()
                    .child(row.snippet.clone()),
            )
            .into_any_element()
    }

    fn render_reader(&self, t: &ChromeTokens) -> AnyElement {
        let Some(reader) = &self.reader else {
            let text = if self.ids.is_empty() {
                ""
            } else {
                "Select a message to read it."
            };
            return div()
                .flex_1()
                .h_full()
                .bg(rgba(t.view_bg))
                .child(placeholder(text, t))
                .into_any_element();
        };
        let Some(view) = &reader.view else {
            return div()
                .flex_1()
                .h_full()
                .bg(rgba(t.view_bg))
                .child(placeholder("This message has not been downloaded yet.", t))
                .into_any_element();
        };
        let dim = rgba(t.fg_dim);
        let addresses = |label: &str, list: &[katna_render::Address]| -> Option<AnyElement> {
            if list.is_empty() {
                return None;
            }
            let names: Vec<String> = list
                .iter()
                .map(|a| match &a.name {
                    Some(name) => format!("{name} <{}>", a.email),
                    None => a.email.clone(),
                })
                .collect();
            Some(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(6.0))
                    .text_size(px(13.0))
                    .child(div().flex_none().text_color(dim).child(label.to_owned()))
                    .child(div().flex_1().min_w_0().child(names.join(", ")))
                    .into_any_element(),
            )
        };
        let sender = view.from.first();
        let date = view
            .date
            .and_then(|d| format::local(d, &self.tz))
            .map(format::long_date)
            .unwrap_or_default();
        let header = div()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .pb(px(16.0))
            .border_b_1()
            .border_color(rgba(t.header_shade))
            .child(
                div()
                    .text_size(px(20.0))
                    .line_height(px(26.0))
                    .font_weight(FontWeight::BOLD)
                    .child(if view.subject.is_empty() {
                        "(no subject)".to_owned()
                    } else {
                        view.subject.clone()
                    }),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .pt(px(4.0))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .flex_row()
                            .gap(px(6.0))
                            .text_size(px(14.0))
                            .child(div().flex_none().font_weight(FontWeight::SEMIBOLD).child(
                                sender.map_or("(unknown sender)".to_owned(), |a| {
                                    a.label().to_owned()
                                }),
                            ))
                            .when_some(sender.filter(|a| a.name.is_some()), |d, a| {
                                d.child(
                                    div()
                                        .min_w_0()
                                        .truncate()
                                        .text_color(dim)
                                        .child(format!("<{}>", a.email)),
                                )
                            }),
                    )
                    .child(
                        div()
                            .flex_none()
                            .text_size(px(12.0))
                            .text_color(dim)
                            .child(date),
                    ),
            )
            .children(addresses("To", &view.to))
            .children(addresses("Cc", &view.cc))
            .when(!view.attachments.is_empty(), |d| {
                d.child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .gap(px(6.0))
                        .pt(px(6.0))
                        .children(view.attachments.iter().map(|a| {
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap(px(6.0))
                                .px(px(8.0))
                                .py(px(4.0))
                                .rounded(px(6.0))
                                .bg(rgba(with_alpha(t.fg, 0x0f)))
                                .text_size(px(12.0))
                                .child(icon("attachment", t.fg_dim, 12.0))
                                .child(a.name.clone())
                                .child(div().text_color(dim).child(format::size(a.size)))
                        })),
                )
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
            .gap(px(0.0))
            .pt(px(16.0))
            .text_size(px(14.0))
            .line_height(px(21.0))
            .children(notes.into_iter().flatten().map(|note| {
                div()
                    .mb(px(12.0))
                    .px(px(10.0))
                    .py(px(6.0))
                    .rounded(px(6.0))
                    .bg(rgba(with_alpha(t.accent, 0x1f)))
                    .text_size(px(12.0))
                    .child(note)
            }))
            .children(reader.blocks.iter().map(|(quoted, text)| {
                div()
                    .when(*quoted, |d| {
                        d.pl(px(10.0))
                            .border_l_2()
                            .border_color(rgba(with_alpha(t.fg, 0x33)))
                            .text_color(dim)
                    })
                    .child(text.clone())
            }));
        div()
            .id("reader")
            .flex_1()
            .min_w_0()
            .h_full()
            .overflow_y_scroll()
            .track_scroll(&self.reader_scroll)
            .bg(rgba(t.view_bg))
            .child(
                div()
                    .max_w(px(820.0))
                    .px(px(28.0))
                    .py(px(24.0))
                    .child(header)
                    .child(body),
            )
            .into_any_element()
    }

    fn render_error(&self, error: &OpenError, t: &ChromeTokens) -> AnyElement {
        let (title, text) = match error {
            OpenError::NoStore { data_dir } => (
                "No mail yet".to_owned(),
                format!(
                    "Katna Mail shows the mail that the Katna background service keeps in \
                     {data_dir}. Nothing is there yet. Until the service can add accounts, \
                     you can import a Maildir or mbox with katna-search-cli import, then \
                     press F5."
                ),
            ),
            OpenError::Other(err) => ("The mail store could not be opened".to_owned(), err.clone()),
        };
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(12.0))
            .bg(rgba(t.view_bg))
            .child(icon("mail", t.fg_dim, 64.0))
            .child(
                div()
                    .text_size(px(22.0))
                    .font_weight(FontWeight::BOLD)
                    .child(title),
            )
            .child(
                div()
                    .max_w(px(460.0))
                    .text_size(px(14.0))
                    .line_height(px(21.0))
                    .text_color(rgba(t.fg_dim))
                    .text_center()
                    .child(text),
            )
            .into_any_element()
    }
}

impl Render for MailWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = self.chrome.tokens(window);
        let accent: Hsla = rgba(t.accent).into();
        self.search
            .update(cx, |search, _| search.set_accent(accent));
        let content = match &self.mail {
            Err(err) => self.render_error(err, &t),
            Ok(_) => div()
                .size_full()
                .flex()
                .flex_row()
                .child(self.render_sidebar(&t, cx))
                .child(self.render_list(&t, cx))
                .child(self.render_reader(&t))
                .into_any_element(),
        };
        let content = div()
            .key_context(WINDOW_CONTEXT)
            .size_full()
            .on_action(cx.listener(Self::focus_search))
            .on_action(cx.listener(Self::focus_list))
            .on_action(cx.listener(Self::reload))
            .on_action(cx.listener(Self::quit))
            .child(content)
            .into_any_element();
        let end = match &self.mail {
            Ok(_) => vec![self.render_search_box(&t)],
            Err(_) => Vec::new(),
        };
        let frame = self.chrome.render(Vec::new(), end, content, window, cx);
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

/// Color of the star of flagged messages.
const STAR: u32 = 0xf5c211ff;

fn icon(name: &str, color: u32, size: f32) -> AnyElement {
    svg()
        .path(SharedString::from(format!("icons/{name}.svg")))
        .size(px(size))
        .flex_none()
        .text_color(rgba(color))
        .into_any_element()
}

fn role_icon(role: Role) -> &'static str {
    match role {
        Role::Inbox => "inbox",
        Role::Flagged => "star",
        Role::Drafts => "drafts",
        Role::Sent => "sent",
        Role::Archive | Role::All => "archive",
        Role::Junk => "junk",
        Role::Trash => "trash",
        Role::Other => "folder",
    }
}

fn placeholder(text: &str, t: &ChromeTokens) -> AnyElement {
    div()
        .size_full()
        .flex()
        .items_center()
        .justify_center()
        .p(px(24.0))
        .text_size(px(14.0))
        .text_color(rgba(t.fg_dim))
        .child(text.to_owned())
        .into_any_element()
}

/// Loads message `id` for the reading pane.
fn read(mail: &Mail, id: MessageId) -> Reader {
    let Some(raw) = mail.raw(id) else {
        return Reader {
            id,
            view: None,
            blocks: Vec::new(),
            cut: false,
        };
    };
    let view = katna_render::message_view(&raw);
    let (blocks, cut) = body_blocks(&view.body, MAX_BODY_LINES);
    Reader {
        id,
        cut: cut || view.truncated,
        view: Some(view),
        blocks,
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
