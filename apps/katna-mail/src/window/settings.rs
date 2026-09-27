// SPDX-License-Identifier: GPL-3.0-or-later

//! Quick settings: a panel that slides in from the right with the reading
//! pane (three or two panes), density, theme, app names, inbox tabs, undo
//! send, the signature, conversation view, the tour, What's new and About.
//! Changes apply at once and are saved to `config.toml`.

use std::time::Duration;

use gpui::{
    Animation, AnimationExt, AnyElement, App, Context, Div, FontWeight, SharedString,
    SpringAnimation, Stateful, div, prelude::*, rgba,
};
use katna_core::config::{
    AccountsShown, Density, FileGroup, MarkRead, OpenIn, ReadingPane, Theme as ThemeChoice,
    UNDO_SEND_CHOICES, WindowFrame,
};
use katna_ui::Ripple;
use katna_ui::motion;
use katna_ui::px;

use super::{MailWindow, SETTINGS_WIDTH};
use crate::theme::{Theme, mix};
use crate::widgets::FocusRing;
use crate::widgets::{
    CARD_SHADOW_ROOM, card_outline, card_shadow, icon, icon_button, radio, switch, tip,
};

/// One loop of the reading-pane demo.
const PANE_DEMO: Duration = Duration::from_millis(2600);

/// What a quick setting changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Change {
    UndoSend(u32),
    Pane(ReadingPane),
    Density(Density),
    Theme(ThemeChoice),
    DesktopColors(bool),
    Tabs(bool),
    Conversations(bool),
    AppLabels(bool),
    SenderPictures(bool),
    NewestFirst(bool),
    FullHeaders(bool),
    FullNames(bool),
    SingleKeys(bool),
    OpenIn(FileGroup, OpenIn),
    AccountsShown(AccountsShown),
    /// The tray icon, shown by the daemon.
    Tray(bool),
    /// The unread count on the taskbar icon, shown by the daemon.
    UnreadBadge(bool),
    /// Katna's own window frame, or the desktop's.
    WindowFrame(WindowFrame),
    /// The blurred, translucent window background.
    Blur(bool),
    /// Days of mail the daemon downloads ahead of time; 0 for all mail.
    OfflineDays(u32),
    /// Crash reports written on this computer (Settings > User feedback).
    SaveCrashReports(bool),
    /// Crash reports sent to Katna's crash tracker: "Help improve Katna".
    SendCrashReports(bool),
    /// The interface scale, in percent.
    Scale(u16),
    /// Katna Mail opens at login (an autostart entry).
    OpenAtLogin(bool),
    MarkRead(MarkRead),
    RemoteImages(bool),
    ReplyAll(bool),
    ImportantMarkers(bool),
    LimitWidth(bool),
    DarkMail(bool),
    AttachmentPreviews(bool),
    OpenSavedFolder(bool),
    /// New-mail notifications, shown by the daemon.
    NewMailNotices(bool),
    /// Their sound.
    NotificationSound(bool),
    PlainText(bool),
    SpellCheck(bool),
    /// The interface's language, a tag; empty follows the desktop.
    Language(&'static str),
    /// Grammar mistakes underlined while writing (English only).
    GrammarCheck(bool),
}

impl MailWindow {
    pub(super) fn render_settings(&self, th: &Theme, t: f32, cx: &mut Context<Self>) -> AnyElement {
        let view = &self.config.mail;
        // On a phone the panel is a page of its own, over the whole window
        // below the top bar.
        let phone = self.layout.shape.is_phone();
        let inner = SETTINGS_WIDTH - 16.0;
        let panel = div()
            .id("settings")
            .map(|d| if phone { d.w_full() } else { d.w(px(inner)) })
            .h_full()
            .flex()
            .flex_col()
            .relative()
            .when(!phone, |d| {
                d.rounded(px(super::PANEL_RADIUS)).shadow(card_shadow(
                    th,
                    t.min(1.0) * self.layout.shape.card_outline(),
                ))
            })
            .bg(rgba(th.surface))
            .child(
                div()
                    .flex_none()
                    .h(px(56.0))
                    .pl(px(20.0))
                    .pr(px(8.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::MEDIUM)
                            .child("Quick settings"),
                    )
                    .child(
                        icon_button("settings-close", "close", 20.0, th)
                            .tooltip(tip("Close", th))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.toggle_settings(&super::ToggleSettings, window, cx)
                            })),
                    ),
            )
            .child(
                div()
                    .id("settings-body")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    // One block that keeps its height, so the rows are
                    // scrolled rather than squeezed.
                    .child(
                        div()
                            .flex_none()
                            .px(px(20.0))
                            .pb(px(20.0))
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            .child(
                                div().pt(px(4.0)).pb(px(8.0)).flex().child(
                                    crate::widgets::outlined_button(
                                        "see-all-settings",
                                        "See all settings",
                                        th,
                                    )
                                    .flex_1()
                                    .justify_center()
                                    .on_click(cx.listener(
                                        |this, _, window, cx| {
                                            this.open_settings_page(
                                                super::settings_page::Section::General,
                                                window,
                                                cx,
                                            )
                                        },
                                    )),
                                ),
                            )
                            .child(heading("Reading pane", th))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .gap(px(12.0))
                                    .child(self.pane_choice(
                                        ReadingPane::Right,
                                        "Right of the list",
                                        th,
                                        cx,
                                    ))
                                    .child(self.pane_choice(ReadingPane::None, "No split", th, cx)),
                            )
                            .child(divider(th))
                            .child(heading("Density", th))
                            .child(self.radio_row(
                                "density-default",
                                "Default",
                                view.density == Density::Default,
                                Change::Density(Density::Default),
                                th,
                                cx,
                            ))
                            .child(self.radio_row(
                                "density-compact",
                                "Compact",
                                view.density == Density::Compact,
                                Change::Density(Density::Compact),
                                th,
                                cx,
                            ))
                            .child(divider(th))
                            .child(heading("Theme", th))
                            .children(
                                [
                                    (ThemeChoice::System, "theme-system", "Same as the desktop"),
                                    (ThemeChoice::Light, "theme-light", "Light"),
                                    (ThemeChoice::Dark, "theme-dark", "Dark"),
                                ]
                                .map(|(choice, id, label)| {
                                    self.radio_row(
                                        id,
                                        label,
                                        view.theme == choice,
                                        Change::Theme(choice),
                                        th,
                                        cx,
                                    )
                                }),
                            )
                            .child(self.switch_row(
                                "desktop-colors",
                                "Desktop colors",
                                "The color scheme and accent color of the desktop",
                                view.desktop_colors,
                                Change::DesktopColors(!view.desktop_colors),
                                th,
                                cx,
                            ))
                            .child(self.switch_row(
                                "app-labels",
                                "App names",
                                "Names under the app icons at the far left",
                                view.app_labels,
                                Change::AppLabels(!view.app_labels),
                                th,
                                cx,
                            ))
                            .child(divider(th))
                            .child(heading("Inbox", th))
                            .child(self.switch_row(
                                "tabs",
                                "Inbox tabs",
                                "The tabs of each account's mail provider",
                                view.inbox_tabs,
                                Change::Tabs(!view.inbox_tabs),
                                th,
                                cx,
                            ))
                            .child(self.link_row(
                                "quick-tabs",
                                "Choose tabs",
                                "Per account, in Settings".into(),
                                super::settings_page::Section::Inbox,
                                th,
                                cx,
                            ))
                            .child(divider(th))
                            .child(heading("Sending", th))
                            .child(self.undo_send_choice(th, cx))
                            .child(self.link_row(
                                "quick-signatures",
                                "Signatures",
                                self.signature_summary(),
                                super::settings_page::Section::Signatures,
                                th,
                                cx,
                            ))
                            .child(divider(th))
                            .child(heading("Email threading", th))
                            .child(self.switch_row(
                                "conversations",
                                "Conversation view",
                                "Group replies to the same mail",
                                view.conversations,
                                Change::Conversations(!view.conversations),
                                th,
                                cx,
                            ))
                            .child(divider(th))
                            .child(heading("Help", th))
                            .child(help_row("take-tour", "tour", "Take the tour", th).on_click(
                                cx.listener(|this, _, window, cx| {
                                    this.start_tour(false, window, cx)
                                }),
                            ))
                            .child(
                                help_row("whats-new", "sparkle", "What\u{2019}s new", th).on_click(
                                    cx.listener(|this, _, window, cx| {
                                        this.show_whats_new(window, cx)
                                    }),
                                ),
                            )
                            .child(help_row("about", "info", "About Katna", th).on_click(
                                cx.listener(|this, _, window, cx| this.open_about(window, cx)),
                            )),
                    ),
            )
            .children(card_outline(
                th,
                super::PANEL_RADIUS,
                self.layout.shape.card_outline(),
            ));
        // The panel keeps its width and slides out from under the edge. The
        // page of a phone fades in as it comes in from the right.
        let t = t.clamp(0.0, 1.0);
        if phone {
            return div()
                .id("settings-phone")
                .occlude()
                .size_full()
                .ml(px(24.0 * (1.0 - t)))
                .opacity(t)
                .child(panel)
                .into_any_element();
        }
        // The clip reaches a little past the panel's left and top edges, so
        // its shadow is never cut.
        let room = CARD_SHADOW_ROOM;
        div()
            .flex_none()
            .h_full()
            .w(px(SETTINGS_WIDTH * t + room))
            .ml(px(-room))
            .mt(px(-room))
            .pl(px(room))
            .pt(px(room))
            .pb(px(16.0 - room))
            .overflow_hidden()
            .child(
                div()
                    .h_full()
                    .pr(px(16.0))
                    .ml(px(24.0 * (1.0 - t)))
                    .opacity(t)
                    .child(panel),
            )
            .into_any_element()
    }

    /// Undo send: how long a sent message waits before it goes out.
    pub(super) fn undo_send_choice(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let now = self.config.sending.undo_send_seconds;
        let chips = UNDO_SEND_CHOICES.into_iter().map(|seconds| {
            let on = seconds == now;
            self.page_control(div().id(("undo-send", seconds as usize)), th, cx)
                .px(px(10.0))
                .h(px(28.0))
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
                .on_click(
                    cx.listener(move |this, _, _, cx| this.apply(Change::UndoSend(seconds), cx)),
                )
                .child(if seconds == 0 {
                    "Off".to_owned()
                } else {
                    format!("{seconds} s")
                })
        });
        div()
            .px(px(8.0))
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(div().text_size(px(14.0)).child("Undo send"))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(6.0))
                    .children(chips),
            )
            .into_any_element()
    }

    pub(super) fn apply(&mut self, change: Change, cx: &mut Context<Self>) {
        let sending = &mut self.config.sending;
        let view = &mut self.config.mail;
        let mut relist = false;
        match change {
            Change::Pane(pane) => {
                if view.reading_pane == pane {
                    return;
                }
                view.reading_pane = pane;
                // With a conversation open the cards change places and fade
                // in; with only the list showing nothing moves, so the list
                // stays as it is.
                if self.reading {
                    self.card_seq += 1;
                } else {
                    self.reader = None;
                }
            }
            Change::UndoSend(seconds) => sending.undo_send_seconds = seconds,
            Change::Density(density) => view.density = density,
            Change::Scale(percent) => {
                if view.scale == percent {
                    return;
                }
                view.scale = percent;
                katna_ui::scale::set_scale(f32::from(percent) / 100.0);
                // Every row is a new height, and every window a new size.
                self.list_state.remeasure();
                cx.refresh_windows();
            }
            Change::Theme(theme) => view.theme = theme,
            Change::DesktopColors(on) => view.desktop_colors = on,
            Change::AppLabels(on) => view.app_labels = on,
            Change::SenderPictures(on) => view.sender_pictures = on,
            Change::NewestFirst(on) => view.newest_first = on,
            Change::FullHeaders(on) => view.full_headers = on,
            Change::FullNames(on) => view.full_names = on,
            Change::OpenIn(group, open) => view.open.set(group, open),
            Change::Tabs(on) => {
                view.inbox_tabs = on;
                relist = true;
            }
            Change::Conversations(on) => {
                view.conversations = on;
                relist = true;
            }
            Change::AccountsShown(shown) => {
                view.accounts_shown = shown;
                // The account on screen stays: it becomes the one shown.
                if let Some(account) = self.account() {
                    self.set_shown_account(account);
                }
                self.rebuild_nav();
            }
            Change::WindowFrame(frame) => {
                self.config.experimental.window_frame = frame;
                cx.set_global(super::look(&self.config));
            }
            Change::Blur(on) => {
                self.config.experimental.blur = on;
                cx.set_global(super::look(&self.config));
            }
            Change::SaveCrashReports(on) => self.config.feedback.save_crash_reports = on,
            Change::MarkRead(when) => view.mark_read = when,
            Change::RemoteImages(on) => {
                view.remote_images = on;
                self.remote.always = on;
                self.fetch_remote(cx);
            }
            Change::ReplyAll(on) => view.reply_all = on,
            Change::ImportantMarkers(on) => {
                view.important_markers = on;
                self.list_state.remeasure();
            }
            Change::LimitWidth(on) => view.limit_width = on,
            Change::DarkMail(on) => view.dark_mail = on,
            Change::AttachmentPreviews(on) => {
                view.attachment_previews = on;
                self.request_thumbnails(cx);
            }
            Change::OpenSavedFolder(on) => view.open_saved_folder = on,
            Change::PlainText(on) => sending.plain_text = on,
            Change::SpellCheck(on) => sending.spell_check = on,
            Change::OpenAtLogin(on) => {
                if let Err(err) = crate::autostart::set(on) {
                    tracing::warn!(%err, "cannot change opening at login");
                    self.show_snackbar(
                        format!("Could not change opening at login: {err}"),
                        None,
                        cx,
                    );
                }
                if let Some(page) = self.settings_page.as_mut() {
                    page.open_at_login = crate::autostart::is_on();
                }
                cx.notify();
                return;
            }
            Change::NewMailNotices(on) | Change::NotificationSound(on) => {
                let notifications = &mut self.config.notifications;
                if matches!(change, Change::NewMailNotices(_)) {
                    notifications.new_mail = on;
                } else {
                    notifications.sound = on;
                }
                self.save_config();
                self.send(crate::daemon::Command::ReloadConfig, None, None, true, cx);
                cx.notify();
                return;
            }
            Change::SingleKeys(on) => {
                self.config.shortcuts.single_keys = on;
                self.shortcuts_changed(cx);
                return;
            }
            Change::OfflineDays(days) => {
                if self.config.sync.offline_days == days {
                    return;
                }
                self.config.sync.offline_days = days;
                self.save_config();
                self.send(crate::daemon::Command::ReloadConfig, None, None, true, cx);
                cx.notify();
                return;
            }
            Change::SendCrashReports(on) => {
                self.config.feedback.send_crash_reports = Some(on);
                self.save_config();
                self.send(crate::daemon::Command::ReloadConfig, None, None, true, cx);
                cx.notify();
                return;
            }
            Change::Language(tag) => {
                if self.config.general.language == tag {
                    return;
                }
                self.config.general.language = tag.to_owned();
                katna_i18n::apply(&self.config.general.language);
                // Text set once rather than at every frame.
                let placeholder = if self.settings_page.is_some() {
                    katna_i18n::tr!("search-settings")
                } else {
                    katna_i18n::tr!("search-mail")
                };
                self.search
                    .update(cx, |search, _| search.set_placeholder(placeholder));
                self.save_config();
                // The daemon's notifications, tray and dock menu follow.
                self.send(crate::daemon::Command::ReloadConfig, None, None, true, cx);
                cx.notify();
                return;
            }
            Change::GrammarCheck(on) => {
                self.config.sending.grammar_check = on;
                self.save_config();
                self.grammar_changed(cx);
                cx.notify();
                return;
            }
            Change::Tray(on) | Change::UnreadBadge(on) => {
                let general = &mut self.config.general;
                if matches!(change, Change::Tray(_)) {
                    general.show_in_tray = on;
                } else {
                    general.unread_badge = on;
                }
                self.save_config();
                self.send(crate::daemon::Command::ReloadConfig, None, None, true, cx);
                cx.notify();
                return;
            }
        }
        self.save_config();
        if relist {
            self.relist(cx);
        }
        cx.notify();
    }

    /// Lists the open folder again after a setting changed what it shows.
    pub(super) fn relist(&mut self, cx: &mut Context<Self>) {
        self.reader = None;
        self.reading = false;
        if let Some(folder) = self.folder {
            self.card_seq += 1;
            self.open_folder(folder, cx);
        }
        cx.notify();
    }

    /// A reading-pane option: a small drawing of the layout and its name.
    pub(super) fn pane_choice(
        &self,
        pane: ReadingPane,
        label: &'static str,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let on = self.config.mail.reading_pane == pane;
        // At rest the drawing shows the layout; under the pointer it plays
        // opening a mail in it, over and over.
        let hovered = self.pane_hover == Some(pane);
        let rest = if pane == ReadingPane::Right { 1.0 } else { 0.0 };
        let picture = if hovered && !cx.reduce_motion() {
            let th = *th;
            div()
                .with_animation(
                    ("pane-demo", pane as usize),
                    Animation::new(PANE_DEMO).repeat(),
                    move |el, t| el.child(pane_picture(pane, demo_open(t), &th)),
                )
                .into_any_element()
        } else {
            pane_picture(pane, rest, th).into_any_element()
        };
        self.page_control(
            div().id(match pane {
                ReadingPane::Right => "pane-right",
                ReadingPane::None => "pane-none",
            }),
            th,
            cx,
        )
        .relative()
        .overflow_hidden()
        .flex_1()
        .p(px(6.0))
        .flex()
        .flex_col()
        .gap(px(8.0))
        .rounded(px(12.0))
        .border_2()
        .cursor_pointer()
        .hover(|s| s.bg(rgba(th.hover)))
        .on_hover(cx.listener(move |this, hovered: &bool, _, cx| {
            let now = hovered.then_some(pane);
            if *hovered || this.pane_hover == Some(pane) {
                this.pane_hover = now;
                cx.notify();
            }
        }))
        .on_click(cx.listener(move |this, _, _, cx| this.apply(Change::Pane(pane), cx)))
        .child(Ripple::new(("pane-ripple", pane as usize), rgba(th.ripple)).rounded(12.0))
        .child(picture)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .px(px(2.0))
                .pb(px(2.0))
                .text_size(px(13.0))
                .child(animated_radio(("pane-radio", pane as usize), on, th))
                .child(label),
        )
        .with_spring(
            ("pane-border", pane as usize),
            SpringAnimation::new(motion::SMOOTH).to(if on { 1.0 } else { 0.0 }),
            {
                let (off, accent) = (th.divider, th.accent);
                move |el, s: f32| el.border_color(rgba(mix(off, accent, s.clamp(0.0, 1.0))))
            },
        )
        .into_any_element()
    }

    /// On the Settings page a control is a Tab stop. The quick settings
    /// panel leaves the focus in the list, so its keys keep working.
    pub(super) fn page_control(
        &self,
        control: Stateful<Div>,
        th: &Theme,
        cx: &App,
    ) -> Stateful<Div> {
        match &self.settings_page {
            Some(page) => control.focus_ring_in(page.tab_stops(), th, cx),
            None => control,
        }
    }

    pub(super) fn radio_row(
        &self,
        id: &'static str,
        label: &'static str,
        on: bool,
        change: Change,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        self.page_control(div().id(id), th, cx)
            .relative()
            .overflow_hidden()
            // A long label wraps onto a second line in a narrow window.
            .min_h(px(40.0))
            .py(px(8.0))
            .px(px(8.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(14.0))
            .rounded(px(8.0))
            .text_size(px(14.0))
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .on_click(cx.listener(move |this, _, _, cx| this.apply(change, cx)))
            .child(Ripple::new((id, 1_usize), rgba(th.ripple)).rounded(8.0))
            .child(animated_radio((id, 2_usize), on, th))
            .child(div().flex_1().min_w_0().child(label))
            .into_any_element()
    }

    /// A row that opens a section of the Settings page.
    fn link_row(
        &self,
        id: &'static str,
        label: &'static str,
        detail: SharedString,
        section: super::settings_page::Section,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        self.page_control(div().id(id), th, cx)
            .relative()
            .overflow_hidden()
            .py(px(8.0))
            .px(px(8.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .rounded(px(8.0))
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .on_click(
                cx.listener(move |this, _, window, cx| {
                    this.open_settings_page(section, window, cx)
                }),
            )
            .child(Ripple::new((id, 1_usize), rgba(th.ripple)).rounded(8.0))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .child(div().text_size(px(14.0)).child(label))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgba(th.text_faint))
                            .truncate()
                            .child(detail),
                    ),
            )
            .child(crate::widgets::icon("chevron-right", th.text_dim, 20.0))
            .into_any_element()
    }

    /// "Work, by default" or "None yet".
    fn signature_summary(&self) -> SharedString {
        let sending = &self.config.sending;
        match (
            sending.signatures.len(),
            sending.signature(sending.new_mail_signature),
        ) {
            (0, _) => "None yet".into(),
            (n, Some(default)) => {
                let name = if default.name.trim().is_empty() {
                    "Untitled"
                } else {
                    default.name.as_str()
                };
                if n == 1 {
                    format!("{name}, used by default").into()
                } else {
                    format!("{n} signatures; {name} by default").into()
                }
            }
            (n, None) => format!("{n}, none by default").into(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn switch_row(
        &self,
        id: &'static str,
        label: &'static str,
        detail: &'static str,
        on: bool,
        change: Change,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        self.page_control(div().id(id), th, cx)
            .relative()
            .overflow_hidden()
            .py(px(8.0))
            .px(px(8.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .rounded(px(8.0))
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .on_click(cx.listener(move |this, _, _, cx| this.apply(change, cx)))
            .child(Ripple::new((id, 1_usize), rgba(th.ripple)).rounded(8.0))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .child(div().text_size(px(14.0)).child(label))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgba(th.text_faint))
                            .child(detail),
                    ),
            )
            .child(div().with_spring(
                (id, 3_usize),
                SpringAnimation::new(motion::SLIDE).to(if on { 1.0 } else { 0.0 }),
                {
                    let th = *th;
                    move |el, s: f32| el.child(switch(s.clamp(0.0, 1.0), &th))
                },
            ))
            .into_any_element()
    }
}

pub(super) fn animated_radio(id: impl Into<gpui::ElementId>, on: bool, th: &Theme) -> AnyElement {
    let th = *th;
    div()
        .with_spring(
            id,
            SpringAnimation::new(motion::SLIDE).to(if on { 1.0 } else { 0.0 }),
            move |el, s: f32| el.child(radio(s.clamp(0.0, 1.0), &th)),
        )
        .into_any_element()
}

/// How far the demo of a reading-pane choice has opened its mail at `t`
/// through the loop: closed, opening, open for a while, closing.
fn demo_open(t: f32) -> f32 {
    let ease = |x: f32| {
        let x = x.clamp(0.0, 1.0);
        x * x * (3.0 - 2.0 * x)
    };
    ease((t - 0.15) / 0.25) - ease((t - 0.75) / 0.18)
}

/// A small drawing of a layout: the navigation, the list and the open
/// mail, which is `open` (0 to 1) of the way in: beside the list for
/// `Right`, in its place for `None`.
fn pane_picture(pane: ReadingPane, open: f32, th: &Theme) -> Div {
    let open = open.clamp(0.0, 1.0);
    let lines = |first: u32| {
        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .p(px(5.0))
            .children((0..4).map(move |i| {
                div()
                    .h(px(4.0))
                    .rounded_full()
                    .bg(rgba(if i == 0 { first } else { th.divider }))
            }))
    };
    // The mail being opened is marked in the list.
    let first = mix(th.divider, th.accent, open);
    let message = || {
        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .p(px(5.0))
            .child(
                div()
                    .h(px(6.0))
                    .w(px(28.0))
                    .rounded_full()
                    .bg(rgba(th.text_faint)),
            )
            .children((0..2).map(|_| div().h(px(3.0)).rounded_full().bg(rgba(th.divider))))
    };
    let card = || div().h_full().rounded(px(3.0)).bg(rgba(th.surface));
    let picture = div()
        .h(px(62.0))
        .p(px(5.0))
        .flex()
        .flex_row()
        .rounded(px(8.0))
        .bg(rgba(th.page))
        .child(
            div()
                .w(px(14.0))
                .mr(px(4.0))
                .h_full()
                .rounded(px(3.0))
                .bg(rgba(th.nav_selected)),
        );
    match pane {
        ReadingPane::Right => picture
            .child(card().flex_1().min_w_0().child(lines(first)))
            .child(
                card()
                    .flex_none()
                    .w(px(46.0 * open))
                    .ml(px(4.0 * open))
                    .overflow_hidden()
                    .opacity(open)
                    .child(message()),
            ),
        ReadingPane::None => picture.child(
            card()
                .relative()
                .flex_1()
                .min_w_0()
                .child(lines(first).opacity(1.0 - open))
                .child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .opacity(open)
                        .child(message()),
                ),
        ),
    }
}

pub(super) fn heading(text: &'static str, th: &Theme) -> Div {
    div()
        .pt(px(12.0))
        .pb(px(8.0))
        .text_size(px(12.0))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgba(th.text_dim))
        .child(text.to_uppercase())
}

pub(super) fn divider(th: &Theme) -> Div {
    div().mt(px(12.0)).h(px(1.0)).bg(rgba(th.divider))
}

/// A line under Help: an icon and what it opens.
fn help_row(
    id: &'static str,
    name: &str,
    label: &'static str,
    th: &Theme,
) -> gpui::Stateful<gpui::Div> {
    div()
        .id(id)
        .relative()
        .overflow_hidden()
        .h(px(40.0))
        .px(px(8.0))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(14.0))
        .rounded(px(8.0))
        .text_size(px(14.0))
        .cursor_pointer()
        .hover(|s| s.bg(rgba(th.hover)))
        .child(Ripple::new((id, 0usize), rgba(th.ripple)).rounded(8.0))
        .child(icon(name, th.text_dim, 20.0))
        .child(label)
}
