// SPDX-License-Identifier: GPL-3.0-or-later

//! Turning an app off (Settings > Apps): Calendar, Contacts, Tasks, Notes
//! or Files, never Mail. An app turned off is gone from everywhere Katna
//! shows it (the rail, its keys, the Go menu, Settings' list) and the
//! daemon stops syncing it; nothing on the accounts' servers changes.
//! Turning one off asks first, in a sheet that names where it leaves, and
//! the snackbar after offers Undo. Right-clicking an app in the rail
//! offers the same.

use gpui::{
    AnimationExt, AnyElement, Context, FontWeight, MouseButton, Pixels, Point, Window, deferred,
    div, prelude::*, rgba,
};
use katna_core::config::AppKind;
use katna_i18n::tr;
use katna_ui::anchored;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::tokens::{space, text};
use katna_ui::{WindowDrag, px};

use super::MenuKey;
use super::apps::App;
use super::settings_page::Section;
use super::{MailWindow, desktop};
use crate::daemon::Command;
use crate::theme::{Theme, fade};
use crate::widgets::{ButtonStyle, FocusRing, button, icon, menu, radio};

const SHEET_WIDTH: f32 = 420.0;

/// The question before turning an app off.
pub(super) struct AppOffAsk {
    app: AppKind,
    closing: bool,
    /// "Remove the copy on this computer" is picked; "Keep a copy" is the
    /// default.
    remove: bool,
    /// The sheet has had the keys given to it.
    focused: bool,
    shown: Spring,
}

/// The right-click menu of an app in the rail.
pub(super) struct RailMenu {
    app: App,
    at: Point<Pixels>,
}

/// The app's name, as the rail shows it.
pub(super) fn app_name(app: AppKind) -> String {
    App::of(app).label()
}

/// The places an app turned off leaves, as the sheet lists them.
fn leaves(app: AppKind) -> Vec<String> {
    let ids: &[&str] = match app {
        AppKind::Calendar => &[
            "app-off-leaves-calendar-rail",
            "app-off-leaves-calendar-agenda",
            "app-off-leaves-calendar-meeting",
            "app-off-leaves-calendar-reminders",
            "app-off-leaves-calendar-desktop",
        ],
        AppKind::Contacts => &[
            "app-off-leaves-contacts-rail",
            "app-off-leaves-contacts-card",
            "app-off-leaves-contacts-birthdays",
        ],
        AppKind::Tasks => &[
            "app-off-leaves-tasks-rail",
            "app-off-leaves-tasks-mail",
            "app-off-leaves-tasks-calendar",
            "app-off-leaves-tasks-tray",
            "app-off-leaves-tasks-reminders",
        ],
        AppKind::Notes => &[
            "app-off-leaves-notes-rail",
            "app-off-leaves-notes-mail",
            "app-off-leaves-notes-meetings",
            "app-off-leaves-notes-tray",
            "app-off-leaves-notes-reminders",
        ],
        AppKind::Files => &["app-off-leaves-files-rail", "app-off-leaves-files-compose"],
    };
    ids.iter().map(|id| tr!(*id)).collect()
}

/// The Go menu's actions of the apps turned off, which the menu bar
/// leaves out.
#[derive(Default)]
pub struct OffApps(pub Vec<&'static str>);

impl gpui::Global for OffApps {}

impl MailWindow {
    /// Tells the menu bar which apps are off, and sends it again when that
    /// changed.
    pub(super) fn share_off_apps(&self, cx: &mut gpui::App) {
        let off: Vec<&'static str> = App::ALL
            .into_iter()
            .filter(|app| !self.app_on(*app))
            .map(App::action_name)
            .collect();
        if cx.try_global::<OffApps>().is_some_and(|g| g.0 == off) {
            return;
        }
        cx.set_global(OffApps(off));
        desktop::refresh_menu_bar(cx);
    }

    /// Says that `app` is off, with a button that turns it on: for its key,
    /// a launcher's action or a link that leads to it.
    pub(super) fn say_app_off(&mut self, app: AppKind, cx: &mut Context<Self>) {
        self.show_snackbar_action(
            tr!("app-off-note", app = app_name(app)),
            tr!("app-off-turn-on"),
            Command::TurnAppOn(app),
            cx,
        );
    }

    /// Whether `app` is on; when it is off, says so, with its Turn on
    /// button. For the ways into an app from Mail (a key, a menu item).
    pub(super) fn needs_app(&mut self, app: AppKind, cx: &mut Context<Self>) -> bool {
        let on = self.config.app_on(app);
        if !on {
            self.say_app_off(app, cx);
        }
        on
    }

    /// Turns `app` on or off, saves it, and tells the daemon. Off, the
    /// window leaves the app if it was on show; on, the app reads its
    /// items again.
    pub(super) fn set_app_on(&mut self, app: AppKind, on: bool, cx: &mut Context<Self>) {
        if self.config.app_on(app) == on {
            return;
        }
        self.config.apps.set(app, on);
        self.save_config();
        // The daemon stops or starts its sync, reminders and tray items.
        self.send(Command::ReloadConfig, None, None, true, cx);
        self.share_off_apps(cx);
        if app == AppKind::Contacts {
            // Compose suggests saved contacts only while Contacts is on.
            self.load_address_book(cx);
        }
        if on {
            match app {
                AppKind::Calendar => self.load_calendar(cx),
                AppKind::Contacts => self.load_contacts(cx),
                AppKind::Tasks => self.load_tasks(cx),
                AppKind::Notes => self.load_notes(cx),
                AppKind::Files => self.load_library(cx),
            }
        } else if self.app == App::of(app) {
            self.open_app(App::Mail, cx);
        }
        cx.notify();
    }

    /// Asks before turning `app` off.
    pub(super) fn ask_app_off(&mut self, app: AppKind, cx: &mut Context<Self>) {
        self.rail_menu = None;
        let mut shown = Spring::new(motion::SMOOTH, 0.0);
        shown.set(1.0);
        self.app_off_ask = Some(AppOffAsk {
            app,
            closing: false,
            remove: false,
            focused: false,
            shown,
        });
        cx.notify();
    }

    /// Closes the question without turning anything off. Returns whether
    /// it was open.
    pub(super) fn close_app_off_ask(&mut self, cx: &mut Context<Self>) -> bool {
        let Some(ask) = self.app_off_ask.as_mut().filter(|a| !a.closing) else {
            return false;
        };
        ask.closing = true;
        ask.shown.set(0.0);
        cx.notify();
        true
    }

    fn confirm_app_off(&mut self, cx: &mut Context<Self>) {
        let Some(ask) = self.app_off_ask.as_mut().filter(|a| !a.closing) else {
            return;
        };
        ask.closing = true;
        ask.shown.set(0.0);
        let (app, remove) = (ask.app, ask.remove);
        self.set_app_on(app, false, cx);
        if remove {
            // Only what came from the accounts goes; Undo turns the app on
            // again and it downloads it afresh.
            self.send(Command::ForgetApp(app), None, None, true, cx);
        }
        self.show_snackbar(
            tr!("app-off-done", app = app_name(app)),
            Some(Command::TurnAppOn(app)),
            cx,
        );
    }

    pub(super) fn render_app_off_ask(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        // It floats: its surface is a step lighter in dark colors.
        let th = &th.lifted();
        let ask = self.app_off_ask.as_mut()?;
        let t = ask.shown.tick(window, reduce);
        if ask.closing && ask.shown.settled() {
            self.app_off_ask = None;
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        // The keys go to the question: Enter turns off, Esc cancels.
        if !ask.focused {
            ask.focused = true;
            window.focus(&self.dialog_focus, cx);
        }
        let app = ask.app;
        let picked_remove = ask.remove;
        let name = app_name(app);
        let lines = leaves(app).into_iter().map(|line| {
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(space::S4))
                .child(
                    div()
                        .size(px(space::S2))
                        .flex_none()
                        .rounded_full()
                        .bg(rgba(th.text_faint)),
                )
                .child(div().flex_1().min_w_0().child(line))
        });
        let body = div()
            .flex()
            .flex_col()
            .px(px(space::S6))
            .pt(px(space::S6))
            .pb(px(space::S5))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(space::S4))
                    .child(
                        div()
                            .size(px(40.0))
                            .flex_none()
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(katna_ui::tokens::radius::MD))
                            .bg(rgba(th.nav_selected))
                            .child(icon(App::of(app).icon(), th.nav_selected_text, 22.0)),
                    )
                    .child(
                        self.copyable(tr!("app-off-title", app = name.clone()), th)
                            .text_size(px(text::TITLE))
                            .line_height(px(28.0)),
                    ),
            )
            .child(
                div()
                    .mt(px(space::S5))
                    .text_size(px(text::BODY))
                    .text_color(rgba(th.text_dim))
                    .child(tr!("app-off-body", app = name)),
            )
            .child(
                div()
                    .mt(px(space::S3))
                    .flex()
                    .flex_col()
                    .gap(px(space::S2))
                    .text_size(px(text::BODY))
                    .line_height(px(20.0))
                    .text_color(rgba(th.text_dim))
                    .children(lines),
            )
            .child(
                div().mt(px(space::S4)).flex().flex_col().children(
                    [
                        (false, tr!("app-off-keep"), tr!("app-off-keep-detail")),
                        (true, tr!("app-off-remove"), tr!("app-off-remove-detail")),
                    ]
                    .into_iter()
                    .map(|(remove, title, detail)| {
                        let on = picked_remove == remove;
                        div()
                            .id(("app-off-copy", remove as usize))
                            .py(px(space::S2))
                            .flex()
                            .flex_row()
                            .items_start()
                            .gap(px(space::S4))
                            .cursor_pointer()
                            .child(radio(if on { 1.0 } else { 0.0 }, th))
                            .child(
                                div()
                                    .flex_1()
                                    .min_w_0()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .text_size(px(text::BODY))
                                            .line_height(px(20.0))
                                            .text_color(rgba(th.text))
                                            .child(title),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(text::SMALL))
                                            .line_height(px(18.0))
                                            .text_color(rgba(th.text_faint))
                                            .child(detail),
                                    ),
                            )
                            .on_click(cx.listener(move |this, _, _, cx| {
                                if let Some(ask) = &mut this.app_off_ask {
                                    ask.remove = remove;
                                }
                                cx.notify();
                            }))
                    }),
                ),
            )
            .child(
                div()
                    .mt(px(space::S6))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(space::S3))
                    .child(div().flex_1())
                    .child(
                        button("app-off-cancel", ButtonStyle::Text, th)
                            .focus_ring(th)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.close_app_off_ask(cx);
                            }))
                            .child(tr!("app-off-cancel")),
                    )
                    .child(
                        button("app-off-confirm", ButtonStyle::Filled, th)
                            .focus_ring_filled(th)
                            .on_click(cx.listener(|this, _, _, cx| this.confirm_app_off(cx)))
                            .child(tr!("app-off-confirm")),
                    ),
            );
        let vw = self.room_width();
        let focus = self.dialog_focus.clone();
        let card = div()
            .id("app-off-ask")
            .track_focus(&focus)
            .map(|d| super::popovers::keep_tab_inside(d, &focus))
            .on_key_down(
                cx.listener(move |this, event: &gpui::KeyDownEvent, window, cx| {
                    let stroke = &event.keystroke;
                    if stroke.key == "enter"
                        && !stroke.modifiers.modified()
                        && focus.is_focused(window)
                    {
                        cx.stop_propagation();
                        this.confirm_app_off(cx);
                    }
                }),
            )
            .occlude()
            .w(px(SHEET_WIDTH.min(vw - 32.0)))
            .flex()
            .flex_col()
            .map(|d| crate::widgets::dialog(d, th, th.surface))
            .text_color(rgba(th.text))
            .font_weight(FontWeight::NORMAL)
            .child(body);
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(rgba(fade(0x0000_0066, t)))
                .child(
                    div()
                        .id("app-off-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.close_app_off_ask(cx);
                        })),
                )
                .child(div().opacity(t).mt(px(lerp(24.0, 0.0, t))).child(card))
                .into_any_element(),
        )
    }

    /// Turns `app` on, or asks before turning it off: Settings' switches.
    pub(super) fn flip_app(&mut self, app: AppKind, on: bool, cx: &mut Context<Self>) {
        if on {
            self.set_app_on(app, true, cx);
        } else {
            self.ask_app_off(app, cx);
        }
    }

    /// Settings > Apps: a switch for each app; Mail's stays on.
    pub(super) fn apps_section(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let rows = App::ALL.into_iter().map(|app| {
            let kind = app.kind();
            let on = self.app_on(app);
            let detail = if on {
                tr!(match app {
                    App::Mail => "settings-apps-mail",
                    App::Calendar => "settings-apps-calendar",
                    App::Contacts => "settings-apps-contacts",
                    App::Tasks => "settings-apps-tasks",
                    App::Notes => "settings-apps-notes",
                    App::Files => "settings-apps-files",
                })
            } else {
                tr!("settings-apps-off")
            };
            let tile =
                div()
                    .size(px(32.0))
                    .flex_none()
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(katna_ui::tokens::radius::SM))
                    .bg(rgba(th.nav_selected))
                    .child(icon(app.icon(), th.nav_selected_text, 18.0))
                    .with_spring(
                        ("settings-app-tile", app as usize),
                        gpui::SpringAnimation::new(katna_ui::motion::scaled(motion::SLIDE))
                            .to(if on { 1.0 } else { 0.0 }),
                        // Off, the app's mark dims with its name.
                        |el, s: f32| el.opacity(lerp(0.45, 1.0, s.clamp(0.0, 1.0))),
                    );
            let words = div()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_col()
                .child(
                    div()
                        .text_size(px(text::BODY))
                        .text_color(rgba(if on { th.text } else { th.text_dim }))
                        .child(app.label()),
                )
                .child(
                    div()
                        .text_size(px(text::CAPTION))
                        .text_color(rgba(th.text_faint))
                        .child(detail),
                );
            let switch = div().with_spring(
                ("settings-app-switch", app as usize),
                gpui::SpringAnimation::new(katna_ui::motion::scaled(motion::SLIDE)).to(if on {
                    1.0
                } else {
                    0.0
                }),
                {
                    let th = *th;
                    move |el, s: f32| el.child(crate::widgets::switch(s.clamp(0.0, 1.0), &th))
                },
            );
            let row = crate::widgets::row(("settings-app", app as usize), false, th)
                .gap(px(space::S4))
                .child(tile)
                .child(words);
            match kind {
                // Mail is the one app that can't be turned off.
                None => row
                    .cursor_default()
                    .tooltip(crate::widgets::tip(tr!("settings-apps-mail-always"), th))
                    .child(icon("lock", th.text_faint, 16.0))
                    .child(
                        div()
                            .opacity(katna_ui::tokens::state::DISABLED)
                            .child(switch),
                    )
                    .into_any_element(),
                Some(kind) => self
                    .page_control(row, th, cx)
                    .on_click(cx.listener(move |this, _, _, cx| this.flip_app(kind, !on, cx)))
                    .child(switch)
                    .into_any_element(),
            }
        });
        let detail = tr!("settings-apps-detail");
        self.row(
            tr!("settings-apps"),
            Some(&detail),
            div().flex().flex_col().children(rows),
            th,
        )
        .into_any_element()
    }

    /// An app's own Settings page, `rows`, headed by its switch.
    pub(super) fn with_app_switch(
        &self,
        app: AppKind,
        rows: AnyElement,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let name = app_name(app);
        let on = self.config.app_on(app);
        let switch = self.switch_row(
            ("settings-app-on", app as usize),
            tr!("settings-app-on", app = name.clone()),
            if on {
                tr!("settings-app-on-detail", app = name)
            } else {
                tr!("settings-apps-off")
            },
            on,
            super::settings::Change::AppOn(app, !on),
            th,
            cx,
        );
        div()
            .flex()
            .flex_col()
            .child(switch)
            // Off, the app's other settings wait until it is on again.
            .when(on, |d| d.child(rows))
            .into_any_element()
    }

    /// Opens the rail's menu for `app` where it was right-clicked.
    pub(super) fn open_rail_menu(&mut self, app: App, at: Point<Pixels>, cx: &mut Context<Self>) {
        self.rail_menu = Some(RailMenu { app, at });
        cx.notify();
    }

    pub(super) fn close_rail_menu(&mut self, cx: &mut Context<Self>) -> bool {
        if self.rail_menu.take().is_some() {
            cx.notify();
            return true;
        }
        false
    }

    pub(super) fn render_rail_menu(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let RailMenu { app, at } = *self.rail_menu.as_ref()?;
        let item = |id: &'static str, glyph: &str, label: String| {
            div()
                .id(id)
                .h(px(36.0))
                .pl(px(space::S5))
                .pr(px(space::S6))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(space::S5))
                .cursor_pointer()
                .keeps_press()
                .hover(|s| s.bg(rgba(th.hover)))
                .menu_key(th)
                .child(icon(glyph, th.text_dim, 20.0))
                .child(div().flex_1().min_w_0().truncate().child(label))
        };
        let section = match app {
            App::Mail => Section::Reading,
            App::Calendar => Section::Calendar,
            App::Contacts => Section::Contacts,
            App::Tasks => Section::Tasks,
            App::Notes => Section::Notes,
            App::Files => Section::Files,
        };
        let list = menu(th)
            .w(px(240.0))
            .child(
                item(
                    "rail-menu-open",
                    app.icon(),
                    tr!("rail-menu-open", app = app.label()),
                )
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.rail_menu = None;
                    this.show_page(app, window, cx);
                })),
            )
            .child(
                item(
                    "rail-menu-settings",
                    "settings",
                    tr!("rail-menu-settings", app = app.label()),
                )
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.rail_menu = None;
                    this.open_settings_page(section, window, cx);
                })),
            )
            .when_some(app.kind(), |d, kind| {
                d.child(div().my(px(space::S2)).h(px(1.0)).bg(rgba(th.divider)))
                    .child(
                        item(
                            "rail-menu-off",
                            "power",
                            tr!("rail-menu-turn-off", app = app.label()),
                        )
                        .on_click(cx.listener(move |this, _, _, cx| this.ask_app_off(kind, cx))),
                    )
            });
        let close = cx.listener(|this: &mut Self, _: &gpui::MouseDownEvent, _, cx| {
            this.close_rail_menu(cx);
        });
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .child(
                    deferred(
                        div()
                            .id("rail-menu-scrim")
                            .absolute()
                            .top(px(-2000.0))
                            .left(px(-4000.0))
                            .w(px(8000.0))
                            .h(px(6000.0))
                            .occlude()
                            .on_mouse_down(MouseButton::Left, close)
                            .on_mouse_down(
                                MouseButton::Right,
                                cx.listener(|this, event: &gpui::MouseDownEvent, window, cx| {
                                    this.close_rail_menu(cx);
                                    super::popovers::pass_right_press(event, window);
                                }),
                            ),
                    )
                    .with_priority(3),
                )
                .child(
                    deferred(
                        anchored()
                            .position(at)
                            .snap_to_window_with_margin(px(8.0))
                            .child(div().occlude().child(list)),
                    )
                    .with_priority(4),
                )
                .into_any_element(),
        )
    }
}
