// SPDX-License-Identifier: GPL-3.0-or-later

//! The Settings page, shown in place of the list as in webmail's "See all
//! settings". Its tabs: General (conversations, undo send, offline
//! mail, the tray),
//! Inbox (tabs per account), Accounts (the folder pane, remove one, or
//! delete all data), Appearance (reading pane, density, theme, pictures),
//! Shortcuts (every one, each can be changed by pressing the new keys),
//! Default apps (where each kind of attachment opens), Compose (signatures,
//! with defaults for new mail and replies), User feedback (crash reports
//! and feedback) and Experimental, with pages for
//! the tabs still to come. The top bar's search box finds settings while
//! the page is open (`settings_search.rs`). Changes apply at once and are
//! saved to `config.toml`.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use gpui::{
    AnyElement, App, Context, Div, Entity, FocusHandle, Focusable, FontWeight, Keystroke,
    ScrollHandle, SharedString, Stateful, Subscription, Task, Window, div, prelude::*, px, rgba,
};
use katna_core::config::{
    AccountTabs, Density, FileGroup, OpenIn, ReadingPane, TabStyle, Theme as ThemeChoice,
};
use katna_ui::motion::lerp;
use katna_ui::rich::RichEvent;
use katna_ui::{InputEvent, RichEditor, Ripple, TextInput};

use super::keymap::{self, Group, SHORTCUTS};
use super::settings::{Change, heading};
use super::tab_strip::TabStrip;
use super::{
    FocusNext, FocusPrevious, MailWindow, OpenSettings, ShowShortcuts, apps::App as RailApp,
};
use crate::tabs::{self, Provider};
use crate::theme::Theme;
use crate::widgets::{FocusRing, TabStops, icon, icon_button, outlined_button, tip};

/// A signature edit is saved this long after the last key.
const SAVE_DELAY: Duration = Duration::from_millis(600);
/// After a key without Ctrl or Alt, wait this long for a second one, as in
/// "g i".
const SEQUENCE_WAIT: Duration = Duration::from_millis(900);
const LABEL_WIDTH: f32 = 220.0;
/// About as many characters of a setting's line as fit on one line under
/// its name; a longer line goes behind an (i) button.
const ONE_LINE: usize = 40;
/// The least room the controls of a row take beside its name; with less,
/// they go below it.
const CONTROL_WIDTH: f32 = 300.0;
/// The same for the keys of a shortcut.
const KEYS_WIDTH: f32 = 160.0;

/// A part of the page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Section {
    General,
    Inbox,
    Accounts,
    Subscriptions,
    Appearance,
    Shortcuts,
    DefaultApps,
    /// Folders & rules: folders and labels, and mail rules.
    MailRules,
    /// Compose: signatures, and templates to come.
    Signatures,
    McpServer,
    Feedback,
    Experimental,
}

impl Section {
    pub(super) const ALL: [Self; 12] = [
        Self::General,
        Self::Inbox,
        Self::Accounts,
        Self::Subscriptions,
        Self::Appearance,
        Self::Shortcuts,
        Self::DefaultApps,
        Self::MailRules,
        Self::Signatures,
        Self::McpServer,
        Self::Feedback,
        Self::Experimental,
    ];

    pub(super) fn label(self) -> &'static str {
        match self {
            Self::General => "General",
            Self::Inbox => "Inbox",
            Self::Accounts => "Accounts",
            Self::Subscriptions => "Subscription",
            Self::Appearance => "Appearance",
            Self::Shortcuts => "Shortcuts",
            Self::DefaultApps => "Default apps",
            Self::MailRules => "Folders & rules",
            Self::Signatures => "Compose",
            Self::McpServer => "MCP server",
            Self::Feedback => "User feedback",
            Self::Experimental => "Experimental",
        }
    }
}

pub(super) struct SettingsPage {
    pub(super) section: Section,
    /// The signature being edited, with its editors.
    editing: Option<SignatureEditor>,
    save: Option<Task<()>>,
    recording: Option<Recording>,
    pub(super) scroll: ScrollHandle,
    /// The open section's tab. It takes the focus when the page opens, so
    /// Tab goes on from there.
    pub(super) focus: FocusHandle,
    /// The controls Tab stops at, which the page scrolls to.
    stops: TabStops,
    /// The row of section tabs, on one line that scrolls sideways.
    tabs: TabStrip,
    /// What the top bar's search box has, which shows matching settings
    /// in place of the open tab.
    pub(super) query: SharedString,
    /// The row a search has just led to.
    pub(super) flash: Option<super::settings_search::Flash>,
    /// The row whose (i) line is shown under its name.
    pub(super) info: Rc<RefCell<Option<SharedString>>>,
}

impl SettingsPage {
    pub(super) fn tab_stops(&self) -> &TabStops {
        &self.stops
    }

    /// A shortcut's new keys are being recorded.
    pub(super) fn recording(&self) -> bool {
        self.recording.is_some()
    }
}

struct SignatureEditor {
    id: u32,
    name: Entity<TextInput>,
    text: Entity<RichEditor>,
    _subscriptions: Vec<Subscription>,
}

/// Keys being pressed for a shortcut.
struct Recording {
    shortcut: &'static str,
    /// The key to replace, or `None` to add one.
    replace: Option<usize>,
    keys: Vec<String>,
    _intercept: Subscription,
    _wait: Option<Task<()>>,
}

impl MailWindow {
    pub(super) fn open_settings_page(
        &mut self,
        section: Section,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_app(RailApp::Mail, cx);
        self.settings_open = false;
        self.menu = None;
        let fresh = self.settings_page.is_none();
        let scroll = ScrollHandle::new();
        let page = self.settings_page.get_or_insert_with(|| SettingsPage {
            section,
            editing: None,
            save: None,
            recording: None,
            scroll: scroll.clone(),
            focus: cx.focus_handle().tab_stop(true),
            stops: TabStops::new(scroll),
            tabs: TabStrip::default(),
            query: SharedString::default(),
            flash: None,
            info: Rc::default(),
        });
        if fresh {
            window.focus(&page.focus, cx);
        }
        page.section = section;
        if let Some(ix) = Section::ALL.iter().position(|s| *s == section) {
            page.tabs.reveal(ix, fresh);
        }
        page.recording = None;
        page.flash = None;
        page.scroll.set_offset(gpui::point(px(0.0), px(0.0)));
        if section == Section::Signatures {
            let first = self.config.sending.signatures.first().map(|s| s.id);
            let editing = self
                .settings_page
                .as_ref()
                .and_then(|p| p.editing.as_ref())
                .map(|e| e.id)
                .filter(|id| self.config.sending.signature(Some(*id)).is_some())
                .or(first);
            self.edit_signature(editing, window, cx);
        }
        self.card_seq += 1;
        cx.notify();
    }

    /// Tab: the next field or button, scrolled into view on this page.
    pub(super) fn focus_next(
        &mut self,
        _: &FocusNext,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus_next(cx);
        if let Some(page) = &self.settings_page {
            page.stops.reveal_focus();
        }
    }

    /// Shift+Tab: the one before.
    pub(super) fn focus_previous(
        &mut self,
        _: &FocusPrevious,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus_prev(cx);
        if let Some(page) = &self.settings_page {
            page.stops.reveal_focus();
        }
    }

    pub(super) fn close_settings_page(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.settings_page = None;
        self.card_seq += 1;
        window.focus(&self.list_focus, cx);
        cx.notify();
    }

    pub(super) fn open_settings(
        &mut self,
        _: &OpenSettings,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_settings_page(Section::General, window, cx);
    }

    pub(super) fn show_shortcuts(
        &mut self,
        _: &ShowShortcuts,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_settings_page(Section::Shortcuts, window, cx);
    }

    fn page_section(&mut self, section: Section, window: &mut Window, cx: &mut Context<Self>) {
        // A tab picked while searching shows that tab, not the results.
        if self
            .settings_page
            .as_ref()
            .is_some_and(|p| !p.query.is_empty())
        {
            self.search.update(cx, |search, cx| search.set_text("", cx));
        }
        if self
            .settings_page
            .as_ref()
            .is_some_and(|p| p.section != section)
        {
            self.open_settings_page(section, window, cx);
        }
        // The tab pressed becomes the open one, which keeps the focus.
        if let Some(page) = &self.settings_page {
            window.focus(&page.focus, cx);
        }
    }

    pub(super) fn render_settings_page(
        &mut self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(page) = &self.settings_page else {
            return div().into_any_element();
        };
        let section = page.section;
        let scroll = page.scroll.clone();
        let focus = page.focus.clone();
        let strip = page.tabs.clone();
        let tabs = Section::ALL.map(|s| {
            let on = s == section;
            div()
                .id(("settings-section", s as usize))
                .when(on, |d| d.track_focus(&focus))
                .focus_ring(th)
                .relative()
                .overflow_hidden()
                .flex_none()
                .h(px(48.0))
                .px(px(16.0))
                .flex()
                .items_center()
                .text_size(px(14.0))
                .font_weight(if on {
                    FontWeight::SEMIBOLD
                } else {
                    FontWeight::NORMAL
                })
                // A tab still to come is fainter until picked.
                .text_color(rgba(if on {
                    th.accent
                } else if super::settings_search::is_coming(s) {
                    th.text_faint
                } else {
                    th.text_dim
                }))
                .border_b_2()
                .border_color(rgba(if on { th.accent } else { 0 }))
                .cursor_pointer()
                .hover(|d| d.bg(rgba(th.hover)))
                .on_click(cx.listener(move |this, _, window, cx| this.page_section(s, window, cx)))
                // The tab is square, so the wave fills all of it.
                .child(
                    Ripple::new(("settings-section-ripple", s as usize), rgba(th.ripple))
                        .rounded(0.0),
                )
                .child(s.label())
        });
        let query = page.query.clone();
        let body = match section {
            _ if !query.is_empty() => self.render_settings_results(&query, th, cx),
            Section::General => self.general_section(th, cx),
            Section::Inbox => self.inbox_section(th, cx),
            Section::Accounts => self.accounts_section(th, cx),
            Section::Appearance => self.appearance_section(th, cx),
            Section::Signatures => self.signatures_section(th, cx),
            Section::DefaultApps => self.default_apps_section(th, cx),
            Section::Shortcuts => self.shortcuts_section(th, cx),
            Section::Experimental => self.experimental_section(th, cx),
            Section::Feedback => self.feedback_section(th, cx),
            Section::Subscriptions | Section::MailRules | Section::McpServer => {
                self.coming_soon_section(section, th)
            }
        };
        // On a phone the page fills the window below the top bar, like the
        // list, and its sides come in closer.
        let shape = self.layout.shape;
        let margin = shape.card_margin();
        let side = lerp(32.0, 16.0, shape.phone);
        let card = div()
            .id("settings-page")
            .size_full()
            .flex()
            .flex_col()
            .rounded(px(shape.card_radius()))
            .bg(rgba(th.surface))
            .overflow_hidden()
            .child(
                div()
                    .flex_none()
                    .h(px(56.0))
                    .pl(px(8.0))
                    .pr(px(16.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        icon_button("settings-page-back", "back", 20.0, th)
                            .focus_ring(th)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.close_settings_page(window, cx)
                            })),
                    )
                    .child(div().text_size(px(22.0)).child("Settings"))
                    .child(div().flex_1())
                    .child(self.version_button(th, cx)),
            )
            // One line of tabs that scrolls sideways when they don't fit,
            // with arrows at the edges except on a phone, where it's swiped.
            .child(strip.render(
                "settings-page-tabs",
                tabs,
                !shape.is_phone(),
                lerp(16.0, 4.0, shape.phone),
                th,
            ))
            .child(
                div()
                    .id("settings-page-body")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .track_scroll(&scroll)
                    .child(
                        div()
                            .flex_none()
                            .px(px(side))
                            .pt(px(8.0))
                            .pb(px(side))
                            .max_w(px(1040.0))
                            .flex()
                            .flex_col()
                            .child(body),
                    ),
            );
        div()
            .flex_1()
            .min_w_0()
            .h_full()
            .pr(px(margin))
            .pb(px(margin))
            .child(card)
            .into_any_element()
    }

    // General

    fn general_section(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let view = &self.config.mail;
        div()
            .flex()
            .flex_col()
            .child(self.row(
                "Conversation view",
                None,
                self.switch_row(
                    "page-conversations",
                    "Group replies to the same mail",
                    "One line per conversation in the list",
                    view.conversations,
                    Change::Conversations(!view.conversations),
                    th,
                    cx,
                ),
                th,
            ))
            .child(self.row("Reading", None, self.reading_switches(th, cx), th))
            .child(self.row(
                "Sending",
                Some("How long a sent message waits, so it can be taken back."),
                self.undo_send_choice(th, cx),
                th,
            ))
            .child(self.row(
                "Offline mail",
                Some("Recent mail is downloaded whole, to read without a connection. Older mail downloads when you open it."),
                self.offline_choice(th, cx),
                th,
            ))
            .child(self.row(
                "Reset cache",
                Some("When mail looks wrong or out of date, or to free disk space. Nothing changes on your mail servers."),
                self.reset_cache_control(th, cx),
                th,
            ))
            .child(self.row(
                "Desktop",
                Some("Shown even while Katna Mail is closed."),
                self.desktop_switches(th, cx),
                th,
            ))
            .into_any_element()
    }

    // Appearance

    fn appearance_section(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let view = &self.config.mail;
        let panes = div()
            .max_w(px(420.0))
            .flex()
            .flex_row()
            .gap(px(12.0))
            .child(self.pane_choice(ReadingPane::Right, "Right of the list", th, cx))
            .child(self.pane_choice(ReadingPane::None, "No split", th, cx));
        let mut density = div().flex().flex_col().gap(px(2.0));
        for (choice, id, label) in [
            (Density::Default, "page-density-default", "Default"),
            (Density::Compact, "page-density-compact", "Compact"),
        ] {
            density = density.child(self.radio_row(
                id,
                label,
                view.density == choice,
                Change::Density(choice),
                th,
                cx,
            ));
        }
        let mut theme = div().flex().flex_col().gap(px(2.0));
        for (choice, id, label) in [
            (
                ThemeChoice::System,
                "page-theme-system",
                "Same as the desktop",
            ),
            (ThemeChoice::Light, "page-theme-light", "Light"),
            (ThemeChoice::Dark, "page-theme-dark", "Dark"),
        ] {
            theme = theme.child(self.radio_row(
                id,
                label,
                view.theme == choice,
                Change::Theme(choice),
                th,
                cx,
            ));
        }
        div()
            .flex()
            .flex_col()
            .child(self.row(
                "Reading pane",
                Some("Where an opened conversation shows."),
                panes,
                th,
            ))
            .child(self.row("Density", None, density, th))
            .child(self.row("Theme", None, theme, th))
            .child(self.row(
                "Desktop colors",
                None,
                self.switch_row(
                    "page-desktop-colors",
                    "Use the desktop's colors",
                    "The color scheme and accent color of the desktop",
                    view.desktop_colors,
                    Change::DesktopColors(!view.desktop_colors),
                    th,
                    cx,
                ),
                th,
            ))
            .child(self.row(
                "App names",
                None,
                self.switch_row(
                    "page-app-labels",
                    "Show app names",
                    "Names under the app icons at the far left",
                    view.app_labels,
                    Change::AppLabels(!view.app_labels),
                    th,
                    cx,
                ),
                th,
            ))
            .child(self.row(
                "Sender pictures",
                None,
                self.switch_row(
                    "page-sender-pictures",
                    "Show company logos",
                    "Looked up by the sender's domain, never by message, and kept for a week",
                    view.sender_pictures,
                    Change::SenderPictures(!view.sender_pictures),
                    th,
                    cx,
                ),
                th,
            ))
            .into_any_element()
    }

    /// How many days of mail the daemon keeps downloaded (`sync.offline_days`).
    fn offline_choice(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let now = self.config.sync.offline_days;
        let chips = OFFLINE_CHOICES.into_iter().map(|(days, label)| {
            self.page_control(
                chip(
                    ("offline-days", days as usize),
                    label.to_owned(),
                    days == now,
                    th,
                ),
                th,
                cx,
            )
            .on_click(cx.listener(move |this, _, _, cx| this.apply(Change::OfflineDays(days), cx)))
        });
        // A number set in the file by hand shows too.
        let other = (!OFFLINE_CHOICES.iter().any(|(days, _)| *days == now))
            .then(|| chip("offline-days-other", format!("{now} days"), true, th));
        div()
            .flex()
            .flex_col()
            .px(px(8.0))
            .gap(px(8.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(6.0))
                    .children(chips)
                    .children(other),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .line_height(px(17.0))
                    .text_color(rgba(th.text_faint))
                    .child(
                        "Choosing fewer days keeps mail already downloaded. \
                         Nothing changes on the server.",
                    ),
            )
            .into_any_element()
    }

    /// The tray icon and the taskbar count, which the daemon shows.
    /// How an opened conversation shows.
    fn reading_switches(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let view = &self.config.mail;
        div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .child(self.switch_row(
                "page-newest-first",
                "Newest message first",
                "A conversation starts with its latest reply",
                view.newest_first,
                Change::NewestFirst(!view.newest_first),
                th,
                cx,
            ))
            .child(self.switch_row(
                "page-full-headers",
                "Show full headers",
                "From, to, cc, date and subject open on every message",
                view.full_headers,
                Change::FullHeaders(!view.full_headers),
                th,
                cx,
            ))
            .child(self.switch_row(
                "page-full-names",
                "Full names of recipients",
                "\u{201c}to me, Ada Lovelace\u{201d} rather than \u{201c}to me, Ada\u{201d}",
                view.full_names,
                Change::FullNames(!view.full_names),
                th,
                cx,
            ))
            .into_any_element()
    }

    fn desktop_switches(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let general = &self.config.general;
        div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .child(self.switch_row(
                "page-tray",
                "Show Katna in the system tray",
                "With the unread count and a menu",
                general.show_in_tray,
                Change::Tray(!general.show_in_tray),
                th,
                cx,
            ))
            .child(self.switch_row(
                "page-unread-badge",
                "Unread count on the taskbar icon",
                "How many Inbox messages are unread",
                general.unread_badge,
                Change::UnreadBadge(!general.unread_badge),
                th,
                cx,
            ))
            .into_any_element()
    }

    // Default apps

    fn default_apps_section(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let open = self.config.mail.open;
        let groups = FileGroup::ALL.map(|group| {
            let (title, detail, ids) = match group {
                FileGroup::Pdf => (
                    "PDF files",
                    "Pages, with zoom.",
                    ["open-pdf-katna", "open-pdf-system", "open-pdf-ask"],
                ),
                FileGroup::Pictures => (
                    "Pictures",
                    "Photos (turned upright), PNG, GIF, WebP, BMP, TIFF and SVG.",
                    ["open-pic-katna", "open-pic-system", "open-pic-ask"],
                ),
                FileGroup::Text => (
                    "Text files",
                    "Plain text, logs, code and other text.",
                    ["open-text-katna", "open-text-system", "open-text-ask"],
                ),
                FileGroup::Spreadsheets => (
                    "Spreadsheets",
                    "Excel (xlsx, xls), OpenDocument (ods) and CSV.",
                    ["open-sheet-katna", "open-sheet-system", "open-sheet-ask"],
                ),
                FileGroup::Documents => (
                    "Documents",
                    "Word (docx) and OpenDocument text (odt).",
                    ["open-doc-katna", "open-doc-system", "open-doc-ask"],
                ),
            };
            let mut choices = div().flex().flex_col().gap(px(2.0));
            for (choice, id, label) in [
                (OpenIn::Katna, ids[0], "Katna Mail's viewer"),
                (OpenIn::System, ids[1], "The desktop's default app"),
                (OpenIn::Ask, ids[2], "Ask which app each time"),
            ] {
                choices = choices.child(self.radio_row(
                    id,
                    label,
                    open.get(group) == choice,
                    Change::OpenIn(group, choice),
                    th,
                    cx,
                ));
            }
            self.row(title, Some(detail), choices, th)
        });
        div()
            .flex()
            .flex_col()
            .child(
                div()
                    .pt(px(20.0))
                    .pb(px(4.0))
                    .text_size(px(13.0))
                    .text_color(rgba(th.text_dim))
                    .child(
                        "Where attachments open when you click them. The viewer can \
                         always open a file in another app too. The desktop's default \
                         apps are set in its own settings.",
                    ),
            )
            .children(groups)
            .into_any_element()
    }

    // Inbox

    fn inbox_section(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let on = self.config.mail.inbox_tabs;
        let accounts: Vec<_> = self
            .accounts
            .iter()
            .filter(|a| a.kind.is_mail())
            .cloned()
            .collect();
        div()
            .flex()
            .flex_col()
            .child(self.row(
                "Inbox tabs",
                Some("Sort the inbox into tabs, as your mail provider's website does."),
                self.switch_row(
                    "page-tabs",
                    "Show inbox tabs",
                    "Off shows one list for every account",
                    on,
                    Change::Tabs(!on),
                    th,
                    cx,
                ),
                th,
            ))
            .when(on, |d| {
                d.children(accounts.iter().enumerate().map(|(ix, account)| {
                    let provider = self.provider(account);
                    let setting = self.config.mail.tabs_of(&account.address);
                    let title = if account.display_name.trim().is_empty() {
                        account.address.clone()
                    } else {
                        format!("{} ({})", account.display_name.trim(), account.address)
                    };
                    self.row(
                        title,
                        None,
                        self.account_tabs_choice(ix, &account.address, &setting, provider, th, cx),
                        th,
                    )
                }))
            })
            .when(accounts.is_empty() && on, |d| {
                d.child(note("Add an account to choose its tabs.", th))
            })
            .into_any_element()
    }

    #[allow(clippy::too_many_arguments)]
    fn account_tabs_choice(
        &self,
        ix: usize,
        address: &str,
        setting: &AccountTabs,
        provider: Provider,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let styles = [
            (
                TabStyle::Auto,
                format!(
                    "Automatic: {} ({})",
                    style_name(provider.style()),
                    provider.name()
                ),
            ),
            (TabStyle::Gmail, style_name(TabStyle::Gmail).to_owned()),
            (TabStyle::Focused, style_name(TabStyle::Focused).to_owned()),
            (TabStyle::Zoho, style_name(TabStyle::Zoho).to_owned()),
            (TabStyle::Off, "No tabs".to_owned()),
        ];
        let options = styles.into_iter().enumerate().map(|(n, (style, label))| {
            let address = address.to_owned();
            self.choice_row(
                ("page-tab-style", ix * 10 + n),
                label,
                setting.style == style,
                th,
                cx,
            )
            .on_click(cx.listener(move |this, _, _, cx| {
                this.set_account_tabs(&address, |t| t.style = style, cx)
            }))
        });
        let style = tabs::resolve(setting, provider);
        let all = tabs::all_tabs(style);
        let checks = all.iter().enumerate().map(|(n, tab)| {
            let first = n == 0;
            let shown = first || !setting.hidden.iter().any(|h| h == tab.key);
            let key = tab.key;
            let address = address.to_owned();
            div()
                .id(("page-tab-check", ix * 10 + n))
                .h(px(32.0))
                .px(px(8.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(10.0))
                .rounded(px(8.0))
                .text_size(px(14.0))
                .when(!first, |d| {
                    d.map(|d| self.page_control(d, th, cx))
                        .cursor_pointer()
                        .hover(|s| s.bg(rgba(th.hover)))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.set_account_tabs(
                                &address,
                                |t| {
                                    if !t.hidden.iter().any(|h| h == key) {
                                        t.hidden.push(key.to_owned());
                                    } else {
                                        t.hidden.retain(|h| h != key);
                                    }
                                },
                                cx,
                            )
                        }))
                })
                .child(icon(
                    if shown {
                        "checkbox-checked"
                    } else {
                        "checkbox"
                    },
                    if first {
                        th.text_faint
                    } else if shown {
                        th.accent
                    } else {
                        th.text_dim
                    },
                    20.0,
                ))
                .child(icon(tab.icon, th.tabs[tab.color], 18.0))
                .child(tab.label)
        });
        div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .children(options)
            .when(!all.is_empty(), |d| {
                d.child(
                    div()
                        .pt(px(12.0))
                        .pb(px(4.0))
                        .px(px(8.0))
                        .text_size(px(12.0))
                        .text_color(rgba(th.text_faint))
                        .child(format!(
                            "Tabs shown. Mail of a tab you turn off stays in {}.",
                            all[0].label
                        )),
                )
                .children(checks)
            })
            .into_any_element()
    }

    fn set_account_tabs(
        &mut self,
        address: &str,
        change: impl FnOnce(&mut AccountTabs),
        cx: &mut Context<Self>,
    ) {
        let key = address.to_lowercase();
        let tabs = &mut self.config.mail.account_tabs;
        let mut setting = tabs.get(&key).cloned().unwrap_or_default();
        change(&mut setting);
        if setting == AccountTabs::default() {
            tabs.remove(&key);
        } else {
            tabs.insert(key, setting);
        }
        self.save_config();
        self.relist(cx);
    }

    // Signatures

    fn edit_signature(&mut self, id: Option<u32>, window: &mut Window, cx: &mut Context<Self>) {
        let Some(signature) = self.config.sending.signature(id).cloned() else {
            if let Some(page) = &mut self.settings_page {
                page.editing = None;
            }
            return;
        };
        let accent = rgba(self.theme(window).accent).into();
        let name = cx.new(|cx| {
            let mut input = TextInput::new("Name, such as Work", cx);
            input.set_text(signature.name.clone(), cx);
            input.set_accent(accent);
            input
        });
        let text = self.signature_editor(&signature, window, cx);
        let id = signature.id;
        let subscriptions = vec![
            cx.subscribe(&name, move |this, input, event: &InputEvent, cx| {
                if *event == InputEvent::Changed {
                    let value = input.read(cx).text().to_owned();
                    this.update_signature(id, |s| s.name = value, cx);
                }
            }),
            cx.subscribe(
                &text,
                move |this, editor, event: &RichEvent, cx| match event {
                    RichEvent::Changed => {
                        let (text, html) = super::compose::signature_content(editor.read(cx).doc());
                        this.update_signature(
                            id,
                            |s| {
                                s.text = text;
                                s.html = html;
                            },
                            cx,
                        );
                    }
                    RichEvent::Selection => cx.notify(),
                    _ => {}
                },
            ),
        ];
        if let Some(page) = &mut self.settings_page {
            page.editing = Some(SignatureEditor {
                id,
                name,
                text,
                _subscriptions: subscriptions,
            });
        }
        cx.notify();
    }

    fn update_signature(
        &mut self,
        id: u32,
        change: impl FnOnce(&mut katna_core::config::Signature),
        cx: &mut Context<Self>,
    ) {
        let Some(signature) = self
            .config
            .sending
            .signatures
            .iter_mut()
            .find(|s| s.id == id)
        else {
            return;
        };
        change(signature);
        self.save_soon(cx);
        cx.notify();
    }

    /// Saves the settings once typing pauses.
    fn save_soon(&mut self, cx: &mut Context<Self>) {
        let task = cx.spawn(async move |this, cx| {
            cx.background_executor().timer(SAVE_DELAY).await;
            this.update(cx, |this, _| this.save_config()).ok();
        });
        if let Some(page) = &mut self.settings_page {
            page.save = Some(task);
        } else {
            task.detach();
        }
    }

    fn new_signature(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let n = self.config.sending.signatures.len() + 1;
        let name = if n == 1 {
            "My signature".to_owned()
        } else {
            format!("Signature {n}")
        };
        let id = self.config.sending.add_signature(name, String::new());
        self.save_config();
        self.edit_signature(Some(id), window, cx);
        // Name it first; Tab then goes on to the signature itself.
        if let Some(editor) = self.settings_page.as_ref().and_then(|p| p.editing.as_ref()) {
            editor.name.update(cx, |name, cx| name.select_all_text(cx));
            window.focus(&editor.name.focus_handle(cx), cx);
        }
    }

    fn delete_signature(&mut self, id: u32, window: &mut Window, cx: &mut Context<Self>) {
        self.config.sending.remove_signature(id);
        self.save_config();
        let next = self.config.sending.signatures.first().map(|s| s.id);
        self.edit_signature(next, window, cx);
        self.show_snackbar("Signature deleted", None, cx);
    }

    fn set_default_signature(&mut self, replies: bool, id: Option<u32>, cx: &mut Context<Self>) {
        let sending = &mut self.config.sending;
        if replies {
            sending.reply_signature = id;
        } else {
            sending.new_mail_signature = id;
        }
        self.save_config();
        cx.notify();
    }

    fn set_send_from(&mut self, address: String, cx: &mut Context<Self>) {
        self.config.sending.send_from = address;
        self.save_config();
        cx.notify();
    }

    fn set_send_and_archive(&mut self, on: bool, cx: &mut Context<Self>) {
        self.config.sending.send_and_archive = on;
        self.save_config();
        cx.notify();
    }

    /// Which account new mail goes out from, and what Send does on a reply.
    fn sending_rows(&self, th: &Theme, cx: &mut Context<Self>) -> [Div; 2] {
        let sending = &self.config.sending;
        let chosen = &sending.send_from;
        // An address no longer set up counts as the open account.
        let known = self
            .accounts
            .iter()
            .any(|a| a.address.eq_ignore_ascii_case(chosen));
        let choices = std::iter::once((String::new(), "The account you are in".to_owned())).chain(
            self.accounts
                .iter()
                .map(|a| (a.address.clone(), a.address.clone())),
        );
        let from =
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .gap(px(6.0))
                .children(choices.enumerate().map(|(n, (address, label))| {
                    let on = if address.is_empty() {
                        !known
                    } else {
                        address.eq_ignore_ascii_case(chosen)
                    };
                    chip(("page-send-from", n), label, on, th)
                        .map(|d| self.page_control(d, th, cx))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.set_send_from(address.clone(), cx)
                        }))
                }));
        let archive = div().flex().flex_row().flex_wrap().gap(px(6.0)).children(
            [(false, "Send"), (true, "Send and archive")].map(|(on, label)| {
                chip(
                    ("page-send-archive", usize::from(on)),
                    label.to_owned(),
                    sending.send_and_archive == on,
                    th,
                )
                .map(|d| self.page_control(d, th, cx))
                .on_click(cx.listener(move |this, _, _, cx| this.set_send_and_archive(on, cx)))
            }),
        );
        [
            self.row(
                "Send new messages from",
                Some("Replies and forwards always go out from the account you are in."),
                from,
                th,
            ),
            self.row(
                "Send on replies",
                Some(
                    "What Send does on a reply or forward. The menu beside Send offers the other.",
                ),
                archive,
                th,
            ),
        ]
    }

    fn signatures_section(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let sending_rows = self.sending_rows(th, cx);
        let tools = self.render_signature_tools(th, cx);
        let sending = &self.config.sending;
        let editing = self.settings_page.as_ref().and_then(|p| p.editing.as_ref());
        let list =
            sending.signatures.iter().map(|s| {
                let on = editing.is_some_and(|e| e.id == s.id);
                let id = s.id;
                div()
                    .id(("page-signature", id as usize))
                    .map(|d| self.page_control(d, th, cx))
                    .relative()
                    .overflow_hidden()
                    .h(px(40.0))
                    .px(px(12.0))
                    .flex()
                    .items_center()
                    .rounded(px(8.0))
                    .text_size(px(14.0))
                    .bg(rgba(if on { th.nav_selected } else { 0 }))
                    .text_color(rgba(if on { th.nav_selected_text } else { th.text }))
                    .cursor_pointer()
                    .hover(|d| d.bg(rgba(if on { th.nav_selected } else { th.hover })))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.edit_signature(Some(id), window, cx)
                    }))
                    .child(
                        Ripple::new(("page-signature-ripple", id as usize), rgba(th.ripple))
                            .rounded(8.0),
                    )
                    .child(div().truncate().child(if s.name.trim().is_empty() {
                        "Untitled".to_owned()
                    } else {
                        s.name.clone()
                    }))
            });
        let editor = editing.map(|e| {
            let id = e.id;
            let name_focus = e.name.focus_handle(cx);
            let text_focus = e.text.focus_handle(cx);
            control_column(240.0)
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(
                    field_box("page-signature-name", th)
                        .h(px(40.0))
                        .flex()
                        .items_center()
                        .on_click(move |_, window, cx| window.focus(&name_focus, cx))
                        .child(div().flex_1().child(e.name.clone())),
                )
                .child(
                    field_box("page-signature-text", th)
                        .min_h(px(140.0))
                        .max_h(px(320.0))
                        .overflow_y_scroll()
                        .py(px(10.0))
                        .line_height(px(20.0))
                        .cursor_text()
                        .on_click(move |_, window, cx| window.focus(&text_focus, cx))
                        .child(e.text.clone()),
                )
                .children(tools)
                .child(
                    div().flex().flex_row().child(div().flex_1()).child(
                        outlined_button("page-signature-delete", "Delete", th)
                            .map(|d| self.page_control(d, th, cx))
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.delete_signature(id, window, cx)
                            })),
                    ),
                )
        });
        let defaults = |replies: bool| {
            let current = if replies {
                sending.reply_signature
            } else {
                sending.new_mail_signature
            };
            let choices = std::iter::once((None, "No signature".to_owned())).chain(
                sending
                    .signatures
                    .iter()
                    .map(|s| (Some(s.id), s.name.clone())),
            );
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .gap(px(6.0))
                .children(choices.enumerate().map(|(n, (id, label))| {
                    chip(
                        ("page-signature-default", usize::from(replies) * 1000 + n),
                        label,
                        current == id,
                        th,
                    )
                    .map(|d| self.page_control(d, th, cx))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.set_default_signature(replies, id, cx)
                    }))
                }))
        };
        div()
            .flex()
            .flex_col()
            .children(sending_rows)
            .child(self.row(
                "Signatures",
                Some("Added below your message, after a \u{201c}--\u{201d} line. Pick another one in the compose window."),
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(16.0))
                    .child(
                        label_column(200.0)
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .children(list)
                            .child(
                                outlined_button("page-signature-new", "Create new", th)
                                    .map(|d| self.page_control(d, th, cx))
                                    .mt(px(8.0))
                                    .justify_center()
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.new_signature(window, cx)
                                    })),
                            ),
                    )
                    .children(editor)
                    .when(sending.signatures.is_empty(), |d| {
                        d.child(note("No signatures yet.", th))
                    }),
                th,
            ))
            .when(!sending.signatures.is_empty(), |d| {
                d.child(self.row(
                    "For new mail",
                    None,
                    defaults(false),
                    th,
                ))
                .child(self.row(
                    "For replies and forwards",
                    Some("In a conversation where you signed a message, a reply starts with that signature instead."),
                    defaults(true),
                    th,
                ))
            })
            .child(self.row(
                "Templates",
                Some("Save mail you write often, and start new mail or a reply from it."),
                div()
                    .flex()
                    .child(super::settings_search::coming_pill(th)),
                th,
            ))
            .into_any_element()
    }

    // Shortcuts

    fn shortcuts_section(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let config = &self.config.shortcuts;
        let recording = self
            .settings_page
            .as_ref()
            .and_then(|p| p.recording.as_ref());
        let changed = !config.keys.is_empty();
        let groups = Group::ALL.map(|group| {
            let rows =
                SHORTCUTS
                    .iter()
                    .enumerate()
                    .filter(|(_, s)| s.group == group)
                    .map(|(n, s)| {
                        let keys = keymap::keys(s, config);
                        let custom = config.keys.contains_key(s.name);
                        let name = s.name;
                        let recording_here = recording.filter(|r| r.shortcut == name);
                        let chips = keys.iter().enumerate().map(|(k, keys)| {
                            let off = !config.single_keys && keymap::is_single_key(keys);
                            if recording_here.is_some_and(|r| r.replace == Some(k)) {
                                return recording_chip(recording_here, th).into_any_element();
                            }
                            key_chip(("key", n * 16 + k), keymap::label(keys), off, th)
                                .map(|d| self.page_control(d, th, cx))
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.start_recording(name, Some(k), cx)
                                }))
                                .child(
                                    div()
                                        .id(("key-remove", n * 16 + k))
                                        .ml(px(2.0))
                                        .rounded_full()
                                        .invisible()
                                        .group_hover("key-chip", |d| d.visible())
                                        .hover(|d| d.bg(rgba(th.hover)))
                                        .on_click(cx.listener(move |this, _, _, cx| {
                                            cx.stop_propagation();
                                            this.remove_key(name, k, cx)
                                        }))
                                        .child(icon("close", th.text_dim, 14.0)),
                                )
                                .into_any_element()
                        });
                        let adding = recording_here.is_some_and(|r| r.replace.is_none());
                        div()
                            .relative()
                            .children(self.flash_mark(s.label, th))
                            .min_h(px(44.0))
                            .py(px(4.0))
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .items_center()
                            .gap_x(px(12.0))
                            .gap_y(px(4.0))
                            .border_b_1()
                            .border_color(rgba(th.divider))
                            .child(label_column(LABEL_WIDTH).text_size(px(14.0)).child(s.label))
                            .child(
                                control_column(KEYS_WIDTH)
                                    .flex()
                                    .flex_row()
                                    .flex_wrap()
                                    .items_center()
                                    .gap(px(6.0))
                                    .children(chips)
                                    .when(adding, |d| d.child(recording_chip(recording_here, th)))
                                    .when(keys.is_empty() && !adding, |d| {
                                        d.child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(rgba(th.text_faint))
                                                .child("No key"),
                                        )
                                    }),
                            )
                            .child(
                                icon_button(("key-add", n), "add", 18.0, th)
                                    .map(|d| self.page_control(d, th, cx))
                                    .size(px(32.0))
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.start_recording(name, None, cx)
                                    })),
                            )
                            .child(
                                icon_button(("key-reset", n), "restore", 18.0, th)
                                    .map(|d| self.page_control(d, th, cx))
                                    .size(px(32.0))
                                    .when(!custom, |d| d.invisible())
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.set_keys(name, None, cx)
                                    })),
                            )
                    });
            div()
                .flex()
                .flex_col()
                .child(heading(group.label(), th))
                .children(rows)
        });
        let single = config.single_keys;
        div()
            .flex()
            .flex_col()
            .child(self.row(
                "Single-key shortcuts",
                Some("Keys without Ctrl or Alt, as in webmail: e archives, j and k move, / searches. They work in the list and the open conversation, never while typing."),
                div().child(self.shortcut_switch(single, th, cx)),
                th,
            ))
            .child(
                div()
                    .pt(px(16.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .flex_wrap()
                    .child(
                        control_column(240.0)
                            .text_size(px(13.0))
                            .text_color(rgba(th.text_faint))
                            .child("Click a key to change it, or + to add one, then press the new keys. Esc cancels."),
                    )
                    .when(changed, |d| {
                        d.child(
                            outlined_button("keys-reset-all", "Restore all defaults", th)
                                .map(|d| self.page_control(d, th, cx))
                                .on_click(cx.listener(|this, _, _, cx| this.reset_all_keys(cx))),
                        )
                    }),
            )
            .children(groups)
            .into_any_element()
    }

    fn shortcut_switch(&self, on: bool, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        self.switch_row(
            "page-single-keys",
            "Use single-key shortcuts",
            "Ctrl shortcuts always work",
            on,
            Change::SingleKeys(!on),
            th,
            cx,
        )
    }

    /// Waits for the keys of `shortcut`, replacing its key `replace` or
    /// adding one. Keys pressed meanwhile do nothing else.
    fn start_recording(
        &mut self,
        shortcut: &'static str,
        replace: Option<usize>,
        cx: &mut Context<Self>,
    ) {
        let this = cx.entity().downgrade();
        let intercept = cx.intercept_keystrokes(move |event, _, cx| {
            let stroke = event.keystroke.clone();
            if this
                .update(cx, |this, cx| this.key_pressed(&stroke, cx))
                .is_ok()
            {
                cx.stop_propagation();
            }
        });
        if let Some(page) = &mut self.settings_page {
            page.recording = Some(Recording {
                shortcut,
                replace,
                keys: Vec::new(),
                _intercept: intercept,
                _wait: None,
            });
        }
        cx.notify();
    }

    fn key_pressed(&mut self, stroke: &Keystroke, cx: &mut Context<Self>) {
        let Some(recording) = self
            .settings_page
            .as_mut()
            .and_then(|p| p.recording.as_mut())
        else {
            return;
        };
        let Some(keys) = keymap::from_stroke(stroke) else {
            return;
        };
        if keys == "escape" && recording.keys.is_empty() {
            self.stop_recording(cx);
            return;
        }
        recording.keys.push(keys);
        let single = recording.keys.len() == 1 && keymap::is_single_key(&recording.keys[0]);
        if single {
            // Maybe the first of two, as in "g i".
            recording._wait = Some(cx.spawn(async move |this, cx| {
                cx.background_executor().timer(SEQUENCE_WAIT).await;
                this.update(cx, |this, cx| this.finish_recording(cx)).ok();
            }));
            cx.notify();
        } else {
            self.finish_recording(cx);
        }
    }

    fn stop_recording(&mut self, cx: &mut Context<Self>) {
        if let Some(page) = &mut self.settings_page {
            page.recording = None;
        }
        cx.notify();
    }

    fn finish_recording(&mut self, cx: &mut Context<Self>) {
        let Some(recording) = self.settings_page.as_mut().and_then(|p| p.recording.take()) else {
            return;
        };
        let pressed = recording.keys.join(" ");
        let Some(shortcut) = keymap::find(recording.shortcut) else {
            return;
        };
        if pressed.is_empty() || !keymap::valid(&pressed) {
            cx.notify();
            return;
        }
        let config = &self.config.shortcuts;
        let mut keys: Vec<String> = keymap::keys(shortcut, config)
            .into_iter()
            .map(str::to_owned)
            .collect();
        match recording.replace {
            Some(ix) if ix < keys.len() => keys[ix] = pressed.clone(),
            _ => keys.push(pressed.clone()),
        }
        let mut seen = std::collections::HashSet::new();
        keys.retain(|k| seen.insert(k.clone()));
        let mut note = None;
        if let Some(other) = keymap::conflict(shortcut.name, &pressed, config) {
            let rest: Vec<String> = keymap::keys(other, config)
                .into_iter()
                .filter(|k| *k != pressed)
                .map(str::to_owned)
                .collect();
            self.store_keys(other.name, rest);
            note = Some(format!(
                "{} now does \u{201c}{}\u{201d} instead of \u{201c}{}\u{201d}.",
                keymap::label(&pressed),
                shortcut.label,
                other.label
            ));
        }
        if note.is_none() && !self.config.shortcuts.single_keys && keymap::is_single_key(&pressed) {
            note = Some("Single-key shortcuts are off, so this key works once they are on.".into());
        }
        self.store_keys(shortcut.name, keys);
        self.shortcuts_changed(cx);
        if let Some(note) = note {
            self.show_snackbar(note, None, cx);
        }
    }

    fn remove_key(&mut self, name: &'static str, ix: usize, cx: &mut Context<Self>) {
        let Some(shortcut) = keymap::find(name) else {
            return;
        };
        let mut keys: Vec<String> = keymap::keys(shortcut, &self.config.shortcuts)
            .into_iter()
            .map(str::to_owned)
            .collect();
        if ix < keys.len() {
            keys.remove(ix);
        }
        self.set_keys(name, Some(keys), cx);
    }

    /// Sets the keys of `name`; `None` goes back to the defaults.
    fn set_keys(&mut self, name: &'static str, keys: Option<Vec<String>>, cx: &mut Context<Self>) {
        match keys {
            Some(keys) => self.store_keys(name, keys),
            None => {
                self.config.shortcuts.keys.remove(name);
            }
        }
        self.shortcuts_changed(cx);
    }

    fn store_keys(&mut self, name: &str, keys: Vec<String>) {
        let defaults = keymap::find(name).map(|s| s.defaults).unwrap_or_default();
        if keys.iter().map(String::as_str).eq(defaults.iter().copied()) {
            self.config.shortcuts.keys.remove(name);
        } else {
            self.config.shortcuts.keys.insert(name.to_owned(), keys);
        }
    }

    fn reset_all_keys(&mut self, cx: &mut Context<Self>) {
        self.config.shortcuts.keys.clear();
        self.shortcuts_changed(cx);
        self.show_snackbar("Every shortcut has its default keys again.", None, cx);
    }

    pub(super) fn shortcuts_changed(&mut self, cx: &mut Context<Self>) {
        if let Some(page) = &mut self.settings_page {
            page.recording = None;
        }
        self.save_config();
        keymap::bind(&self.config.shortcuts, cx);
        super::refresh_menu_bar(cx);
        cx.notify();
    }

    /// A radio-like row whose click the caller sets.
    fn choice_row(
        &self,
        id: impl Into<gpui::ElementId>,
        label: String,
        on: bool,
        th: &Theme,
        cx: &App,
    ) -> Stateful<Div> {
        let id = id.into();
        self.page_control(div().id(id.clone()), th, cx)
            .relative()
            .overflow_hidden()
            .min_h(px(36.0))
            .py(px(6.0))
            .px(px(8.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(14.0))
            .rounded(px(8.0))
            .text_size(px(14.0))
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .child(super::settings::animated_radio(id, on, th))
            .child(div().flex_1().min_w_0().child(label))
    }
}

/// The offline mail choices: days (0 for all mail) and their names.
const OFFLINE_CHOICES: [(u32, &str); 5] = [
    (7, "7 days"),
    (30, "30 days"),
    (90, "90 days"),
    (365, "1 year"),
    (0, "All mail"),
];

fn style_name(style: TabStyle) -> &'static str {
    match style {
        TabStyle::Auto | TabStyle::Gmail => "Primary, Promotions, Social, Updates, Forums",
        TabStyle::Focused => "Focused and Other",
        TabStyle::Zoho => "Inbox, Newsletters and Notifications",
        TabStyle::Off => "No tabs",
    }
}

/// A setting: its name (and a line on it) on the left, the controls on the
/// right. Where the two don't fit side by side, as on a phone, the name
/// goes above the controls and both take the whole width. A line on it that
/// would take more than one line under the name goes behind an (i) button
/// beside the name instead: its tooltip on hover, or shown under the name
/// while `open` (a click, Enter or a tap). `flash` goes under the row when a
/// search has just led here.
pub(super) fn setting_row(
    label: SharedString,
    detail: Option<&'static str>,
    content: impl IntoElement,
    info: &Rc<RefCell<Option<SharedString>>>,
    flash: Option<AnyElement>,
    th: &Theme,
) -> Div {
    let long = detail.filter(|d| d.chars().count() > ONE_LINE);
    let open = long.is_some() && info.borrow().as_ref() == Some(&label);
    let button = long.map(|text| {
        let info = info.clone();
        let name = label.clone();
        div()
            .id(SharedString::from(format!("setting-info-{label}")))
            .focus_ring(th)
            .flex_none()
            .size(px(24.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .cursor_pointer()
            .hover(|d| d.bg(rgba(th.hover)))
            .tooltip(tip(text, th))
            .on_click(move |_, window, _| {
                let mut shown = info.borrow_mut();
                *shown = if shown.as_ref() == Some(&name) {
                    None
                } else {
                    Some(name.clone())
                };
                window.refresh();
            })
            .child(icon(
                "info",
                if open { th.accent } else { th.text_faint },
                16.0,
            ))
    });
    let shown = if long.is_some() {
        detail.filter(|_| open)
    } else {
        detail
    };
    div()
        .relative()
        .children(flash)
        .py(px(20.0))
        .flex()
        .flex_row()
        .flex_wrap()
        .gap_x(px(24.0))
        .gap_y(px(12.0))
        .border_b_1()
        .border_color(rgba(th.divider))
        .child(
            label_column(LABEL_WIDTH)
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(
                    div()
                        .min_h(px(20.0))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(4.0))
                        .child(
                            div()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::MEDIUM)
                                .child(label),
                        )
                        .children(button),
                )
                .children(shown.map(|d| {
                    div()
                        .text_size(px(12.0))
                        .line_height(px(17.0))
                        .text_color(rgba(th.text_faint))
                        .child(d)
                })),
        )
        .child(control_column(CONTROL_WIDTH).child(content))
}

/// The name column of a row that wraps: `width` wide beside the controls,
/// the whole row once the controls wrap below it.
fn label_column(width: f32) -> Div {
    div().flex_basis(px(width)).flex_grow(1.0).min_w_0()
}

/// The controls' column of a row that wraps. It takes nearly all the room
/// left beside the name, and wraps below the name when it would get less
/// than `width`.
fn control_column(width: f32) -> Div {
    div().flex_basis(px(width)).flex_grow(1000.0).min_w_0()
}

fn note(text: &'static str, th: &Theme) -> Div {
    div()
        .py(px(12.0))
        .text_size(px(14.0))
        .text_color(rgba(th.text_faint))
        .child(text)
}

fn field_box(id: &'static str, th: &Theme) -> Stateful<Div> {
    div()
        .id(id)
        .px(px(12.0))
        .rounded(px(8.0))
        .border_1()
        .border_color(rgba(th.divider))
        .text_size(px(14.0))
}

fn chip(id: impl Into<gpui::ElementId>, label: String, on: bool, th: &Theme) -> Stateful<Div> {
    div()
        .id(id)
        .px(px(12.0))
        .h(px(30.0))
        .flex()
        .items_center()
        .rounded(px(8.0))
        .border_1()
        .border_color(rgba(if on { th.nav_selected } else { th.divider }))
        .bg(rgba(if on { th.nav_selected } else { th.surface }))
        .text_color(rgba(if on {
            th.nav_selected_text
        } else {
            th.text_dim
        }))
        .text_size(px(13.0))
        .cursor_pointer()
        .hover(|s| s.bg(rgba(th.hover)))
        .child(if label.trim().is_empty() {
            "Untitled".to_owned()
        } else {
            label
        })
}

/// A key as a keycap; `off` when single keys are turned off.
fn key_chip(id: impl Into<gpui::ElementId>, label: String, off: bool, th: &Theme) -> Stateful<Div> {
    div()
        .id(id)
        .group("key-chip")
        .h(px(28.0))
        .pl(px(10.0))
        .pr(px(6.0))
        .flex()
        .flex_row()
        .items_center()
        .rounded(px(6.0))
        .border_1()
        .border_color(rgba(th.divider))
        .bg(rgba(th.page))
        .text_size(px(13.0))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgba(if off { th.text_faint } else { th.text }))
        .when(off, |d| d.line_through())
        .cursor_pointer()
        .hover(|s| s.border_color(rgba(th.text_faint)))
        .child(label)
}

fn recording_chip(recording: Option<&Recording>, th: &Theme) -> Div {
    let text: SharedString = match recording.map(|r| r.keys.as_slice()) {
        Some([first, ..]) => format!("{} then\u{2026}", keymap::label(first)).into(),
        _ => "Press keys\u{2026}".into(),
    };
    div()
        .h(px(28.0))
        .px(px(10.0))
        .flex()
        .items_center()
        .rounded(px(6.0))
        .border_2()
        .border_color(rgba(th.accent))
        .text_size(px(13.0))
        .text_color(rgba(th.accent))
        .child(text)
}
