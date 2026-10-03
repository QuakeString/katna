// SPDX-License-Identifier: GPL-3.0-or-later

//! Settings in a window of its own, beside the mail window, as decided in
//! the Settings layout study (2026-10-03). On a phone the page fills the
//! main window instead (`SettingsPage::own_window`).
//!
//! The page stays in [`MailWindow::settings_page`]; the window only draws
//! it, as the popped-out message's window does. The dialogs, menus and
//! notes Settings opens (a color, a rule, removing an account) show in
//! whichever of the two windows was used last.

use gpui::{
    AnyElement, App, Context, Decorations, Entity, FocusHandle, Focusable, MouseButton,
    Subscription, Window, WindowBounds, WindowHandle, div, point, prelude::*, rgba, size,
};
use katna_chrome::{Bar, WindowChrome, window_options};
use katna_core::ids::MAIL_APP_ID;
use katna_i18n::tr;
use katna_ui::scale::desktop_px;
use katna_ui::tokens::{space, text};
use katna_ui::{InputEvent, TextInput, px, unpx};

use super::nav::{search_edge, search_fill};
use super::select::CopyText;
use super::{
    CheckForUpdates, FocusNext, FocusPrevious, FocusSearch, MailWindow, OpenSettings, Quit,
    ShowAbout, ShowShortcuts, ShowWhatsNew, TOP_BAR_HEIGHT, WINDOW_CONTEXT,
};
use crate::theme::Theme;
use crate::widgets::{icon, icon_button, tip};

/// The size the window opens at.
const WIDTH: f32 = 1040.0;
const HEIGHT: f32 = 760.0;
/// Narrower than this, the list of pages fills the window until one is
/// picked, as on a phone.
const NARROW: f32 = 640.0;
/// The search box at the top.
const SEARCH_WIDTH: f32 = 560.0;
const SEARCH_HEIGHT: f32 = 40.0;

/// The root view of Settings' window.
pub(in crate::window) struct SettingsWindow {
    mail: Entity<MailWindow>,
    chrome: WindowChrome,
    focus: FocusHandle,
    _subscriptions: Vec<Subscription>,
}

/// The window handle's type, for [`MailWindow::settings_window`].
pub(in crate::window) type Handle = WindowHandle<SettingsWindow>;

impl MailWindow {
    /// The Settings page fills this window, as on a phone.
    pub(in crate::window) fn settings_in_main(&self) -> bool {
        self.settings_page
            .as_ref()
            .is_some_and(|page| !page.own_window)
    }

    /// Settings is laid out as on a phone: a list of pages, then one page.
    pub(in crate::window) fn settings_phone(&self) -> bool {
        match &self.settings_page {
            Some(page) if page.own_window => page.narrow,
            _ => self.layout.shape.is_phone(),
        }
    }

    /// The box that searches settings: its window's, or the top bar's.
    pub(in crate::window) fn settings_search_box(&self) -> Entity<TextInput> {
        match &self.settings_page {
            Some(page) if page.own_window => page.search.clone(),
            _ => self.search.clone(),
        }
    }

    /// What Settings opens shows in its window.
    pub(in crate::window) fn overlays_in_settings(&self) -> bool {
        self.settings_overlays && self.settings_window.is_some()
    }

    /// A dialog is open, which stays in its window until it closes.
    fn dialog_up(&self) -> bool {
        self.add_account.is_some()
            || self.danger.is_some()
            || self.delete_ask.is_some()
            || self.new_label.is_some()
            || self.rule_editor.is_some()
            || self.scheme_editor.is_some()
            || self.whats_new_open()
            || self.update_dialog_open()
            || self.about_open()
    }

    /// One of the two windows came to the front: what opens from now on
    /// shows there. A menu still open in the other one closes rather than
    /// jump across.
    pub(in crate::window) fn settings_overlays_to(
        &mut self,
        in_settings: bool,
        cx: &mut Context<Self>,
    ) {
        if self.settings_overlays == in_settings || self.dialog_up() {
            return;
        }
        self.settings_overlays = in_settings;
        self.language_picker = None;
        self.close_context_menu(cx);
        if self
            .color_picker
            .as_ref()
            .is_some_and(|p| matches!(p.target(), super::scheme_color::Target::Account(_)))
        {
            self.color_picker = None;
        }
        cx.notify();
    }

    /// Opens Settings' window for the page just made.
    pub(in crate::window) fn open_settings_window(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(page) = &self.settings_page else {
            return;
        };
        let focus = page.focus.clone();
        let search = page.search.clone();
        let env = self.chrome.environment();
        let mail = cx.entity();
        let mut options = window_options(
            &env,
            MAIL_APP_ID,
            tr!("settings"),
            size(desktop_px(WIDTH), desktop_px(HEIGHT)),
            cx,
        );
        // Over the middle of the mail window. On Wayland the compositor
        // places it instead.
        if let Some(WindowBounds::Windowed(bounds)) = &mut options.window_bounds {
            let main = window.bounds();
            bounds.origin = point(
                main.origin.x + (main.size.width - bounds.size.width) / 2.0,
                main.origin.y + (main.size.height - bounds.size.height) / 2.0,
            );
        }
        // The mail window coming back to the front takes the dialogs back.
        self.settings_activation =
            Some(cx.observe_window_activation(window, |this, window, cx| {
                if window.is_window_active() {
                    this.settings_overlays_to(false, cx);
                }
            }));
        // Opened after this update, which holds the mail window.
        cx.defer(move |cx| {
            let closing = mail.clone();
            let opened = cx.open_window(options, |window, cx| {
                window.focus(&focus, cx);
                window.on_window_should_close(cx, move |_, cx| {
                    closing.update(cx, |this, cx| {
                        this.settings_window = None;
                        this.settings_window_gone(cx);
                    });
                    true
                });
                // Escape closes its popovers, as in the mail window.
                mail.update(cx, |this, cx| this.watch_escape(window, cx));
                let released = mail.clone();
                cx.new(|cx| {
                    // The close button of Katna's own title bar removes the
                    // window without asking first.
                    cx.on_release(move |_, cx| {
                        let released = released.clone();
                        cx.defer(move |cx| {
                            released.update(cx, |this, cx| this.settings_window_gone(cx));
                        });
                    })
                    .detach();
                    let subscriptions = vec![
                        cx.subscribe_in(
                            &search,
                            window,
                            |this: &mut SettingsWindow, _, event: &InputEvent, window, cx| {
                                this.mail.update(cx, |mail, cx| {
                                    mail.on_settings_search(event, window, cx)
                                });
                            },
                        ),
                        cx.observe_window_activation(window, |this, window, cx| {
                            if window.is_window_active() {
                                this.mail
                                    .update(cx, |mail, cx| mail.settings_overlays_to(true, cx));
                            }
                        }),
                    ];
                    SettingsWindow {
                        mail: mail.clone(),
                        chrome: WindowChrome::new(env, tr!("settings"), window, cx),
                        focus: cx.focus_handle(),
                        _subscriptions: subscriptions,
                    }
                })
            });
            mail.update(cx, |this, cx| {
                match opened {
                    Ok(handle) => this.settings_window = Some(handle),
                    // The page fills the mail window instead.
                    Err(err) => {
                        tracing::warn!("cannot open a Settings window: {err}");
                        this.settings_activation = None;
                        if let Some(page) = &mut this.settings_page {
                            page.own_window = false;
                        }
                        this.swap_app_search(false, cx);
                    }
                }
                cx.notify();
            });
        });
    }

    /// Brings Settings' window to the front with its page's keys, when
    /// asked for from the mail window.
    pub(in crate::window) fn raise_settings_window(&self, window: &Window, cx: &mut App) {
        let (Some(handle), Some(page)) = (self.settings_window, &self.settings_page) else {
            return;
        };
        if window.window_handle().window_id() == handle.window_id() {
            return;
        }
        let focus = page.focus.clone();
        handle
            .update(cx, |_, window, cx| {
                window.activate_window();
                window.focus(&focus, cx);
            })
            .ok();
    }

    /// Closes Settings' window, if open. The page itself is left as it is.
    pub(in crate::window) fn close_settings_window(&mut self, cx: &mut Context<Self>) {
        self.settings_activation = None;
        self.settings_overlays = false;
        if let Some(handle) = self.settings_window.take() {
            // Not from inside that window's own update.
            cx.defer(move |cx| {
                handle
                    .update(cx, |_, window, _| window.remove_window())
                    .ok();
            });
        }
        cx.notify();
    }

    /// Settings' window went away, however it was closed: the page closes
    /// too, its changes saved.
    fn settings_window_gone(&mut self, cx: &mut Context<Self>) {
        let alive = self
            .settings_window
            .is_some_and(|handle| handle.update(cx, |_, _, _| ()).is_ok());
        if alive {
            return;
        }
        self.settings_window = None;
        self.settings_activation = None;
        self.settings_overlays = false;
        if self
            .settings_page
            .as_ref()
            .is_some_and(|page| page.own_window)
        {
            self.drop_settings_page(cx);
        }
    }

    /// Tab and Shift+Tab in Settings' window: the next field or button,
    /// scrolled into view.
    fn settings_window_tab(&mut self, forward: bool, window: &mut Window, cx: &mut Context<Self>) {
        if forward {
            window.focus_next(cx);
        } else {
            window.focus_prev(cx);
        }
        if let Some(page) = &self.settings_page {
            page.tab_stops().reveal_focus();
        }
    }

    /// What fills Settings' window under its search box: the page, and
    /// over it what was opened from it.
    fn render_settings_window(
        &mut self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let narrow = unpx(window.viewport_size().width) < NARROW;
        if let Some(page) = &mut self.settings_page
            && page.narrow != narrow
        {
            page.narrow = narrow;
            // Narrowed to a phone's width, it shows the list of pages.
            page.list = narrow;
        }
        // Keys that lost their place come back to the page.
        if window.focused(cx).is_none()
            && let Some(page) = &self.settings_page
        {
            window.focus(&page.focus, cx);
        }
        let reduce = cx.reduce_motion();
        let page = self.render_settings_page(th, window, cx);
        let in_settings = self.overlays_in_settings();
        let overlays = if in_settings {
            self.render_shared_overlays(th, window, reduce, cx)
        } else {
            Vec::new()
        };
        let snackbar = if in_settings {
            self.render_snackbar(th, window, reduce, cx)
        } else {
            None
        };
        div()
            .relative()
            .size_full()
            .text_color(rgba(th.text))
            .child(page)
            .children(overlays)
            .children(snackbar)
            .into_any_element()
    }

    /// The search box at the top of Settings' window.
    fn render_settings_window_search(
        &self,
        th: &Theme,
        width: f32,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let search = self.settings_page.as_ref()?.search.clone();
        let focused = search.focus_handle(cx).is_focused(window);
        let t = if focused { 1.0 } else { 0.0 };
        let has_text = !search.read(cx).text().is_empty();
        let focus_box = search.clone();
        Some(
            div()
                .id("settings-search")
                .w(px(width))
                .h(px(SEARCH_HEIGHT))
                .pl(px(space::S3))
                .pr(px(space::S1))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(space::S2))
                .rounded_full()
                .bg(rgba(search_fill(th, t)))
                .border_1()
                .border_color(rgba(search_edge(th, t)))
                .text_size(px(text::SUBTITLE))
                .line_height(px(24.0))
                .text_color(rgba(th.text))
                .cursor_text()
                // A drag here selects text rather than moving the window.
                .on_mouse_move(|_, _, cx| cx.stop_propagation())
                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                    window.focus(&focus_box.focus_handle(cx), cx);
                })
                .child(icon(
                    "search",
                    if focused { th.accent } else { th.text_dim },
                    20.0,
                ))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .overflow_hidden()
                        .child(search.clone()),
                )
                .when(has_text, |d| {
                    d.child(
                        icon_button("settings-search-clear", "close", 20.0, th)
                            .tooltip(tip(tr!("search-clear"), th))
                            .on_click(move |_, window, cx| {
                                search.update(cx, |search, cx| search.set_text("", cx));
                                window.focus(&search.focus_handle(cx), cx);
                            }),
                    )
                })
                .into_any_element(),
        )
    }
}

impl Focusable for SettingsWindow {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl Render for SettingsWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Text without a size of its own follows Settings > Appearance > Scaling.
        window.set_rem_size(px(16.0));
        self.chrome.sync_look(window, cx);
        let chrome = &self.chrome;
        let width = unpx(window.viewport_size().width);
        let search_width = SEARCH_WIDTH.min(width - 2.0 * space::S6).max(160.0);
        let drawn = self.mail.update(cx, |mail, cx| {
            if !mail
                .settings_page
                .as_ref()
                .is_some_and(|page| page.own_window)
            {
                return None;
            }
            mail.ui_text.begin_window(window);
            let th = mail.theme_for(chrome, window);
            let content = mail.render_settings_window(&th, window, cx);
            // Its text can be selected and copied too.
            let menu = mail.render_ui_text_menu(&th, window, cx);
            let content = mail
                .ui_text_root(div().size_full().child(content), cx)
                .children(menu)
                .into_any_element();
            let search = mail.render_settings_window_search(&th, search_width, window, cx)?;
            Some((th, content, search, mail.font.clone()))
        });
        let Some((th, content, search, font)) = drawn else {
            // Closed from the mail window: nothing is left to show.
            window.remove_window();
            return div().into_any_element();
        };
        let server = matches!(window.window_decorations(), Decorations::Server);
        let frame = if server {
            // The desktop's title bar holds the window's buttons; the
            // search box gets a bar of its own.
            div()
                .size_full()
                .flex()
                .flex_col()
                .bg(rgba(th.backdrop))
                .text_color(rgba(th.text))
                .child(
                    div()
                        .flex_none()
                        .h(px(TOP_BAR_HEIGHT))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(search),
                )
                .child(div().flex_1().min_h_0().child(content))
        } else {
            let bar = Bar {
                center: Some(search),
                height: Some(TOP_BAR_HEIGHT),
                background: Some(th.backdrop),
                ..Bar::default()
            };
            let content = div()
                .size_full()
                .bg(rgba(th.backdrop))
                .child(content)
                .into_any_element();
            self.chrome.render_bar(bar, content, window, cx)
        };
        let frame = frame
            .key_context(WINDOW_CONTEXT)
            // A press here, before what it opens: that shows here too,
            // where a desktop doesn't say which window is in front.
            .capture_any_mouse_down(cx.listener(|this, _, _, cx| {
                this.mail
                    .update(cx, |mail, cx| mail.settings_overlays_to(true, cx));
            }))
            // Focus can sit outside the content (the search box, or
            // nowhere), so Ctrl+C reaches the selection from here too.
            .on_action(cx.listener(|this, _: &CopyText, _, cx| {
                if !this.mail.update(cx, |mail, cx| mail.copy_ui_text(cx)) {
                    cx.propagate();
                }
            }))
            .on_action(cx.listener(|this, _: &FocusNext, window, cx| {
                this.mail
                    .update(cx, |mail, cx| mail.settings_window_tab(true, window, cx));
            }))
            .on_action(cx.listener(|this, _: &FocusPrevious, window, cx| {
                this.mail
                    .update(cx, |mail, cx| mail.settings_window_tab(false, window, cx));
            }))
            .on_action(cx.listener(|this, _: &FocusSearch, window, cx| {
                let search = this.mail.read(cx).settings_search_box();
                window.focus(&search.focus_handle(cx), cx);
            }))
            .on_action(cx.listener(|this, action: &OpenSettings, window, cx| {
                this.mail
                    .update(cx, |mail, cx| mail.open_settings(action, window, cx));
            }))
            .on_action(cx.listener(|this, action: &ShowShortcuts, window, cx| {
                this.mail
                    .update(cx, |mail, cx| mail.show_shortcuts(action, window, cx));
            }))
            .on_action(cx.listener(|this, action: &ShowAbout, window, cx| {
                this.mail
                    .update(cx, |mail, cx| mail.show_about(action, window, cx));
            }))
            .on_action(cx.listener(|this, action: &ShowWhatsNew, window, cx| {
                this.mail.update(cx, |mail, cx| {
                    mail.show_whats_new_action(action, window, cx)
                });
            }))
            .on_action(cx.listener(|this, action: &CheckForUpdates, window, cx| {
                this.mail.update(cx, |mail, cx| {
                    mail.check_for_updates_action(action, window, cx)
                });
            }))
            .on_action(cx.listener(|this, action: &Quit, window, cx| {
                this.mail
                    .update(cx, |mail, cx| mail.quit(action, window, cx));
            }));
        match font {
            Some(font) => frame.font_family(font).into_any_element(),
            None => frame.into_any_element(),
        }
    }
}
