// SPDX-License-Identifier: GPL-3.0-or-later

//! The Settings page, shown in place of the list as in webmail's "See all
//! settings": General (reading pane, density, theme, conversations, undo
//! send), Inbox (tabs per account), Accounts (remove one, or delete all
//! data), Signatures (several, with defaults for new mail and replies) and
//! Keyboard shortcuts (every one, each can be changed by pressing the new
//! keys). Changes apply at once and are saved
//! to `config.toml`.

use std::time::Duration;

use gpui::{
    AnyElement, Context, Div, Entity, Focusable, FontWeight, Keystroke, ScrollHandle, SharedString,
    Stateful, Subscription, Task, Window, div, prelude::*, px, rgba,
};
use katna_core::config::{AccountTabs, Density, ReadingPane, TabStyle, Theme as ThemeChoice};
use katna_ui::{InputEvent, Ripple, TextArea, TextInput};

use super::keymap::{self, Group, SHORTCUTS};
use super::settings::{Change, heading};
use super::{MailWindow, OpenSettings, ShowShortcuts, apps::App as RailApp};
use crate::tabs::{self, Provider};
use crate::theme::Theme;
use crate::widgets::{icon, icon_button, outlined_button};

/// A signature edit is saved this long after the last key.
const SAVE_DELAY: Duration = Duration::from_millis(600);
/// After a key without Ctrl or Alt, wait this long for a second one, as in
/// "g i".
const SEQUENCE_WAIT: Duration = Duration::from_millis(900);
const LABEL_WIDTH: f32 = 220.0;

/// A part of the page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Section {
    General,
    Inbox,
    Accounts,
    Signatures,
    Shortcuts,
}

impl Section {
    const ALL: [Self; 5] = [
        Self::General,
        Self::Inbox,
        Self::Accounts,
        Self::Signatures,
        Self::Shortcuts,
    ];

    fn label(self) -> &'static str {
        match self {
            Self::General => "General",
            Self::Inbox => "Inbox",
            Self::Accounts => "Accounts",
            Self::Signatures => "Signatures",
            Self::Shortcuts => "Keyboard shortcuts",
        }
    }
}

pub(super) struct SettingsPage {
    section: Section,
    /// The signature being edited, with its editors.
    editing: Option<SignatureEditor>,
    save: Option<Task<()>>,
    recording: Option<Recording>,
    scroll: ScrollHandle,
}

struct SignatureEditor {
    id: u32,
    name: Entity<TextInput>,
    text: Entity<TextArea>,
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
        let page = self.settings_page.get_or_insert_with(|| SettingsPage {
            section,
            editing: None,
            save: None,
            recording: None,
            scroll: ScrollHandle::new(),
        });
        page.section = section;
        page.recording = None;
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
        if self
            .settings_page
            .as_ref()
            .is_some_and(|p| p.section != section)
        {
            self.open_settings_page(section, window, cx);
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
        let tabs = Section::ALL.map(|s| {
            let on = s == section;
            div()
                .id(("settings-section", s as usize))
                .relative()
                .overflow_hidden()
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
                .text_color(rgba(if on { th.accent } else { th.text_dim }))
                .border_b_2()
                .border_color(rgba(if on { th.accent } else { 0 }))
                .cursor_pointer()
                .hover(|d| d.bg(rgba(th.hover)))
                .on_click(cx.listener(move |this, _, window, cx| this.page_section(s, window, cx)))
                .child(Ripple::new(
                    ("settings-section-ripple", s as usize),
                    rgba(th.ripple),
                ))
                .child(s.label())
        });
        let body = match section {
            Section::General => self.general_section(th, cx),
            Section::Inbox => self.inbox_section(th, cx),
            Section::Accounts => self.accounts_section(th, cx),
            Section::Signatures => self.signatures_section(th, cx),
            Section::Shortcuts => self.shortcuts_section(th, cx),
        };
        let card = div()
            .id("settings-page")
            .size_full()
            .flex()
            .flex_col()
            .rounded(px(16.0))
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
                        icon_button("settings-page-back", "back", 20.0, th).on_click(
                            cx.listener(|this, _, window, cx| this.close_settings_page(window, cx)),
                        ),
                    )
                    .child(div().text_size(px(22.0)).child("Settings")),
            )
            .child(
                div()
                    .flex_none()
                    .px(px(16.0))
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .border_b_1()
                    .border_color(rgba(th.divider))
                    .children(tabs),
            )
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
                            .px(px(32.0))
                            .pt(px(8.0))
                            .pb(px(32.0))
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
            .pr(px(16.0))
            .pb(px(16.0))
            .child(card)
            .into_any_element()
    }

    // General

    fn general_section(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
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
            .child(row(
                "Reading pane",
                Some("Where an opened conversation shows."),
                panes,
                th,
            ))
            .child(row("Density", None, density, th))
            .child(row("Theme", None, theme, th))
            .child(row(
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
            .child(row(
                "Sending",
                Some("How long a sent message waits, so it can be taken back."),
                self.undo_send_choice(th, cx),
                th,
            ))
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
            .child(row(
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
                    row(
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
                    d.cursor_pointer()
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
        let text = cx.new(|cx| {
            let mut area = TextArea::new("Your name, and anything to add below it", cx);
            area.set_text(signature.text.clone(), 0, cx);
            area.set_accent(accent);
            area
        });
        let id = signature.id;
        let subscriptions = vec![
            cx.subscribe(&name, move |this, input, event: &InputEvent, cx| {
                if *event == InputEvent::Changed {
                    let value = input.read(cx).text().to_owned();
                    this.update_signature(id, |s| s.name = value, cx);
                }
            }),
            cx.subscribe(&text, move |this, area, event: &InputEvent, cx| {
                if *event == InputEvent::Changed {
                    let value = area.read(cx).text().to_owned();
                    this.update_signature(id, |s| s.text = value, cx);
                }
            }),
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
        if let Some(editor) = self.settings_page.as_ref().and_then(|p| p.editing.as_ref()) {
            window.focus(&editor.text.focus_handle(cx), cx);
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

    fn signatures_section(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let sending = &self.config.sending;
        let editing = self.settings_page.as_ref().and_then(|p| p.editing.as_ref());
        let list =
            sending.signatures.iter().map(|s| {
                let on = editing.is_some_and(|e| e.id == s.id);
                let id = s.id;
                div()
                    .id(("page-signature", id as usize))
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
                    .child(Ripple::new(
                        ("page-signature-ripple", id as usize),
                        rgba(th.ripple),
                    ))
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
            div()
                .flex_1()
                .min_w_0()
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
                .child(div().flex().flex_row().child(div().flex_1()).child(
                    outlined_button("page-signature-delete", "Delete", th).on_click(cx.listener(
                        move |this, _, window, cx| this.delete_signature(id, window, cx),
                    )),
                ))
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
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.set_default_signature(replies, id, cx)
                    }))
                }))
        };
        div()
            .flex()
            .flex_col()
            .child(row(
                "Signatures",
                Some("Added below your message, after a \u{201c}--\u{201d} line. Pick another one in the compose window."),
                div()
                    .flex()
                    .flex_row()
                    .gap(px(16.0))
                    .child(
                        div()
                            .flex_none()
                            .w(px(200.0))
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .children(list)
                            .child(
                                outlined_button("page-signature-new", "Create new", th)
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
                d.child(row(
                    "For new mail",
                    None,
                    defaults(false),
                    th,
                ))
                .child(row(
                    "For replies and forwards",
                    Some("In a conversation where you signed a message, a reply starts with that signature instead."),
                    defaults(true),
                    th,
                ))
            })
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
                            .min_h(px(44.0))
                            .py(px(4.0))
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(12.0))
                            .border_b_1()
                            .border_color(rgba(th.divider))
                            .child(
                                div()
                                    .w(px(LABEL_WIDTH))
                                    .flex_none()
                                    .text_size(px(14.0))
                                    .child(s.label),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .min_w_0()
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
                                    .size(px(32.0))
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.start_recording(name, None, cx)
                                    })),
                            )
                            .child(
                                icon_button(("key-reset", n), "restore", 18.0, th)
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
            .child(row(
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
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(13.0))
                            .text_color(rgba(th.text_faint))
                            .child("Click a key to change it, or + to add one, then press the new keys. Esc cancels."),
                    )
                    .when(changed, |d| {
                        d.child(
                            outlined_button("keys-reset-all", "Restore all defaults", th)
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
    ) -> Stateful<Div> {
        let id = id.into();
        div()
            .id(id.clone())
            .relative()
            .overflow_hidden()
            .h(px(36.0))
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
            .child(label)
    }
}

fn style_name(style: TabStyle) -> &'static str {
    match style {
        TabStyle::Auto | TabStyle::Gmail => "Primary, Promotions, Social, Updates, Forums",
        TabStyle::Focused => "Focused and Other",
        TabStyle::Zoho => "Inbox, Newsletters and Notifications",
        TabStyle::Off => "No tabs",
    }
}

/// A setting: its name (and a line on it) on the left, the controls on the
/// right.
pub(super) fn row(
    label: impl Into<SharedString>,
    detail: Option<&'static str>,
    content: impl IntoElement,
    th: &Theme,
) -> Div {
    div()
        .py(px(20.0))
        .flex()
        .flex_row()
        .gap(px(24.0))
        .border_b_1()
        .border_color(rgba(th.divider))
        .child(
            div()
                .w(px(LABEL_WIDTH))
                .flex_none()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(
                    div()
                        .text_size(px(14.0))
                        .font_weight(FontWeight::MEDIUM)
                        .child(label.into()),
                )
                .children(detail.map(|d| {
                    div()
                        .text_size(px(12.0))
                        .line_height(px(17.0))
                        .text_color(rgba(th.text_faint))
                        .child(d)
                })),
        )
        .child(div().flex_1().min_w_0().child(content))
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
