// SPDX-License-Identifier: GPL-3.0-or-later

//! The app rail at the far left: Mail, Calendar, Contacts, Tasks, Notes
//! and Feeds, with settings at the bottom; their names can be hidden in
//! quick settings. Each app is a page of the one window: the rail, Ctrl+1
//! to Ctrl+5 (Outlook's keys), the Go menu, the desktop file's actions and
//! `katna-mail --page NAME` (D-Bus `ActivateAction("open-page", [NAME])`)
//! all switch pages through [`MailWindow::show_page`].
//!
//! Adding a page: give it a module of its own under `window/` with a
//! `render_<name>_page` method (as `calendar.rs` has), call it from its arm
//! in [`MailWindow::render_app_page`], and load what it needs in
//! [`MailWindow::open_app`]'s arm. Pages without one show "coming soon".

use std::ops::Range;
use std::rc::Rc;

use gpui::{
    AnimationExt, AnyElement, Context, FontWeight, SpringAnimation, Window, div, prelude::*, rgba,
    uniform_list,
};
use katna_i18n::tr;
use katna_store::Person;
use katna_ui::Ripple;
use katna_ui::motion;
use katna_ui::px;

use super::{MailWindow, OpenSettings};
use crate::format;
use crate::theme::{Theme, fade};
use crate::widgets::{icon, icon_button_colored, placeholder, tip};

pub(super) const APP_RAIL_WIDTH: f32 = 72.0;

/// The apps of the rail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum App {
    Mail,
    Calendar,
    Contacts,
    Tasks,
    Notes,
    Feeds,
}

impl App {
    pub(super) const ALL: [Self; 6] = [
        Self::Mail,
        Self::Calendar,
        Self::Contacts,
        Self::Tasks,
        Self::Notes,
        Self::Feeds,
    ];

    pub(super) fn label(self) -> String {
        tr!(match self {
            Self::Mail => "rail-mail",
            Self::Calendar => "rail-calendar",
            Self::Contacts => "rail-contacts",
            Self::Tasks => "rail-tasks",
            Self::Notes => "rail-notes",
            Self::Feeds => "rail-feeds",
        })
    }

    /// The page's name for `--page` and `open-page`, and in the saved
    /// window state.
    pub(super) fn key(self) -> &'static str {
        self.icon()
    }

    /// The page named `key`.
    pub(super) fn from_key(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|app| app.key() == key)
    }

    pub(super) fn icon(self) -> &'static str {
        match self {
            Self::Mail => "mail",
            Self::Calendar => "calendar",
            Self::Contacts => "contacts",
            Self::Tasks => "tasks",
            Self::Notes => "notes",
            Self::Feeds => "feeds",
        }
    }

    /// What the app will do, for its "coming soon" page.
    fn promise(self) -> String {
        match self {
            Self::Mail | Self::Contacts | Self::Tasks => String::new(),
            Self::Calendar => tr!("app-calendar-promise"),
            Self::Notes => tr!("app-notes-promise"),
            Self::Feeds => tr!("app-feeds-promise"),
        }
    }
}

/// The people list of the Contacts page.
pub(super) enum People {
    Loading,
    Loaded(Rc<Vec<Person>>),
    Failed(String),
}

impl MailWindow {
    /// Shows page `app`, leaving Settings as picking a folder does.
    pub(super) fn show_page(&mut self, app: App, window: &mut Window, cx: &mut Context<Self>) {
        if self.settings_page.is_some() {
            self.close_settings_page(window, cx);
        }
        self.open_app(app, cx);
        if app == App::Calendar {
            window.focus(&self.calendar.focus, cx);
        }
        // Its keys and Ctrl+Z reach the page, not the hidden mail list.
        if app == App::Tasks
            && let Some(focus) = &self.tasks.focus
        {
            window.focus(focus, cx);
        }
    }

    pub(super) fn open_app(&mut self, app: App, cx: &mut Context<Self>) {
        if self.app == app {
            return;
        }
        let from = self.app;
        self.app = app;
        // Notes hands the search box back before Contacts takes it, and
        // takes it after Contacts hands it back.
        if from == App::Notes {
            self.sync_notes_search(cx);
        }
        if from == App::Contacts || app == App::Contacts {
            // The search box follows: contacts on this page, mail elsewhere.
            self.swap_contacts_search(app == App::Contacts, cx);
        }
        // The name at the top left rolls from the old app's to the new.
        self.title_from = from;
        self.title_roll.snap(0.0);
        self.title_roll.set(1.0);
        self.menu = None;
        self.search_panel = None;
        if app == App::Contacts && !matches!(self.contacts.book, Some(Ok(_))) {
            self.load_contacts(cx);
        }
        if app == App::Calendar {
            self.load_calendar(cx);
        }
        if app == App::Notes {
            self.sync_notes_search(cx);
        }
        if app == App::Tasks {
            self.open_tasks_page(cx);
        }
        cx.notify();
    }

    pub(super) fn load_people(&mut self, cx: &mut Context<Self>) {
        self.people = Some(People::Loading);
        let paths = self.paths.clone();
        self.people_task = Some(cx.spawn(async move |this, cx| {
            let people = cx
                .background_executor()
                .spawn(async move { crate::data::people(&paths) })
                .await;
            this.update(cx, |this, cx| {
                this.people = Some(match people {
                    Ok(people) => People::Loaded(Rc::new(people)),
                    Err(err) => People::Failed(err),
                });
                cx.notify();
            })
            .ok();
        }));
    }

    /// Room at the top of the rail for Compose while it is there.
    pub(super) fn rail_compose_room(&self) -> f32 {
        (super::COMPOSE_TOP + super::COMPOSE_HEIGHT + 12.0 - 4.0)
            * (1.0 - self.compose_dock.value().clamp(0.0, 1.0))
            * self.compose_shown.value().clamp(0.0, 1.0)
    }

    /// Where the middle of Mail's icon is, down from the rail's top.
    pub(super) fn rail_mail_middle(&self) -> f32 {
        // The rail's and the line's top padding, then half the icon's pill.
        4.0 + self.rail_compose_room() + 4.0 + 16.0
    }

    pub(super) fn render_app_rail(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let labels = self.config.mail.app_labels;
        // Room at the top for Compose while it is in the rail: the apps
        // move down as it slides in from the folders.
        let compose_room = self.rail_compose_room();
        let items = App::ALL.into_iter().map(|app| {
            let on = self.app == app;
            div()
                .id(("app", app as usize))
                .w(px(APP_RAIL_WIDTH))
                .pt(px(4.0))
                .pb(px(8.0))
                .flex()
                .flex_col()
                .items_center()
                .gap(px(4.0))
                .cursor_pointer()
                .group("app")
                .when(app == App::Mail, |d| {
                    d.on_hover(cx.listener(|this, hovered: &bool, _, cx| {
                        this.hover_navigation(super::Hover::Mail, *hovered, cx)
                    }))
                })
                .on_click(cx.listener(move |this, _, window, cx| this.show_page(app, window, cx)))
                .child(
                    div()
                        .relative()
                        .overflow_hidden()
                        .w(px(56.0))
                        .h(px(32.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_full()
                        .group_hover("app", |s| s.bg(rgba(th.hover)))
                        .child(
                            Ripple::new(("app-ripple", app as usize), rgba(th.ripple)).centered(),
                        )
                        .child(icon(
                            app.icon(),
                            if on {
                                th.nav_selected_text
                            } else {
                                th.text_dim
                            },
                            22.0,
                        ))
                        .with_spring(
                            ("app-pill", app as usize),
                            SpringAnimation::new(motion::SLIDE).to(if on { 1.0 } else { 0.0 }),
                            {
                                let bg = th.nav_selected;
                                move |el, s: f32| {
                                    let s = s.clamp(0.0, 1.0);
                                    if s > 0.001 {
                                        // The pill grows out from the middle.
                                        el.bg(rgba(fade(bg, s))).w(px(32.0 + 24.0 * s))
                                    } else {
                                        el
                                    }
                                }
                            },
                        ),
                )
                .when(!labels, |d| d.tooltip(tip(app.label(), th)))
                // The name folds away when the settings hide it.
                .child(
                    div()
                        .overflow_hidden()
                        .text_size(px(12.0))
                        .line_height(px(16.0))
                        .font_weight(if on {
                            FontWeight::BOLD
                        } else {
                            FontWeight::MEDIUM
                        })
                        .text_color(rgba(if on { th.text } else { th.text_dim }))
                        .child(app.label())
                        .with_spring(
                            ("app-label", app as usize),
                            SpringAnimation::new(motion::SLIDE).to(if labels { 1.0 } else { 0.0 }),
                            |el, s: f32| {
                                let s = s.clamp(0.0, 1.0);
                                el.h(px(16.0 * s)).opacity(s)
                            },
                        ),
                )
        });
        div()
            .id("app-rail")
            .relative()
            .flex_none()
            .w(px(APP_RAIL_WIDTH))
            .h_full()
            .pt(px(4.0))
            .pb(px(16.0))
            .flex()
            .flex_col()
            .items_center()
            .child(div().flex_none().h(px(compose_room)))
            .child(
                div()
                    .relative()
                    .flex()
                    .flex_col()
                    .items_center()
                    .child(self.tour_mark(super::tour::Spot::Apps))
                    .children(items),
            )
            .child(div().flex_1())
            .child(
                icon_button_colored(
                    "rail-settings",
                    "settings",
                    22.0,
                    if self.settings_page.is_some() {
                        th.accent
                    } else {
                        th.text_dim
                    },
                    th,
                )
                .tooltip(tip(tr!("settings"), th))
                .on_click(cx.listener(|this, _, window, cx| {
                    if this.settings_page.is_some() && this.app == App::Mail {
                        this.close_settings_page(window, cx);
                    } else {
                        this.open_settings(&OpenSettings, window, cx);
                    }
                })),
            )
            .into_any_element()
    }

    /// The page of an app other than Mail.
    pub(super) fn render_app_page(
        &mut self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let body = match self.app {
            App::Contacts => self.render_contacts_page(th, window, cx),
            App::Calendar => self.render_calendar_page(th, cx),
            App::Notes => self.render_notes(th, window, cx),
            App::Tasks => self.render_tasks(th, cx),
            App::Mail | App::Feeds => self.render_coming_soon(th),
        };
        div()
            .flex_1()
            .min_w_0()
            .h_full()
            .pr(px(16.0))
            .pb(px(16.0))
            .child(
                div()
                    .size_full()
                    .rounded(px(super::PANEL_RADIUS))
                    .overflow_hidden()
                    .bg(rgba(th.surface))
                    .child(body),
            )
            .into_any_element()
    }

    /// The page of an app not built yet: what it will do.
    fn render_coming_soon(&self, th: &Theme) -> AnyElement {
        let app = self.app;
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(12.0))
            .child(
                div()
                    .size(px(96.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .bg(rgba(th.nav_selected))
                    .child(icon(app.icon(), th.nav_selected_text, 48.0)),
            )
            .child(
                div()
                    .pt(px(8.0))
                    .text_size(px(22.0))
                    .text_color(rgba(th.text))
                    .child(tr!("app-page-title", app = app.label())),
            )
            .child(
                div()
                    .px(px(10.0))
                    .py(px(2.0))
                    .rounded_full()
                    .bg(rgba(th.chip))
                    .text_size(px(12.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgba(th.text_dim))
                    .child(tr!("app-coming-soon")),
            )
            .child(
                div()
                    .max_w(px(420.0))
                    .text_center()
                    .text_size(px(14.0))
                    .line_height(px(21.0))
                    .text_color(rgba(th.text_faint))
                    .child(app.promise()),
            )
            .with_animation(
                ("app-page", self.app as usize),
                gpui::Animation::new(std::time::Duration::from_millis(260))
                    .with_easing(gpui::ease_out_quint()),
                |el, t| el.opacity(t).mt(px(12.0 * (1.0 - t))),
            )
            .into_any_element()
    }

    /// The people the mail was exchanged with, most first: Frequent.
    pub(super) fn render_contacts(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let people = match &self.people {
            None | Some(People::Loading) => {
                return placeholder(&tr!("app-contacts-loading"), th);
            }
            Some(People::Failed(err)) => return placeholder(err, th),
            Some(People::Loaded(people)) => people.clone(),
        };
        if people.is_empty() {
            return placeholder(&tr!("app-contacts-empty"), th);
        }
        let header = div()
            .flex_none()
            .h(px(64.0))
            .px(px(24.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .border_b_1()
            .border_color(rgba(th.divider))
            .child(div().text_size(px(20.0)).child(tr!("contacts-frequent")))
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(rgba(th.text_faint))
                    .child(if people.len() >= crate::data::PEOPLE_LIMIT as usize {
                        tr!("app-contacts-top", count = people.len())
                    } else {
                        tr!("app-contacts-count", count = people.len())
                    }),
            );
        let count = people.len();
        let list = uniform_list(
            "people",
            count,
            cx.processor(move |this, range: Range<usize>, window, cx| {
                let th = this.theme(window);
                let rows = range
                    .map(|ix| render_person(ix, &people[ix], &th, this, cx))
                    .collect::<Vec<_>>();
                this.fetch_pictures(cx);
                rows
            }),
        )
        .size_full();
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(header)
            .child(div().flex_1().min_h_0().child(list))
            .into_any_element()
    }
}

fn render_person(
    ix: usize,
    person: &Person,
    th: &Theme,
    this: &MailWindow,
    cx: &mut Context<MailWindow>,
) -> AnyElement {
    let name = person
        .name
        .clone()
        .filter(|name| !name.trim().is_empty() && *name != person.email);
    let now = jiff::Timestamp::now().as_second();
    let last = person
        .last
        .and_then(|d| format::local(d, &this.tz))
        .zip(format::local(now, &this.tz))
        .map(|(d, now)| tr!("app-contacts-last", date = format::list_date(d, now)))
        .unwrap_or_default();
    let email = person.email.clone();
    div()
        .id(("person", ix))
        .w_full()
        .h(px(60.0))
        .px(px(24.0))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(16.0))
        .border_b_1()
        .border_color(rgba(th.divider))
        .cursor_pointer()
        .hover(|s| s.bg(rgba(th.hover)))
        .on_click(cx.listener(move |this, _, window, cx| {
            this.open_app(App::Mail, cx);
            this.search_for(format!("from:{email}"), window, cx);
        }))
        .child(this.person_avatar(
            name.as_deref().unwrap_or(&person.email),
            &person.email,
            36.0,
        ))
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
                        .child(name.clone().unwrap_or_else(|| person.email.clone())),
                )
                .when(name.is_some(), |d| {
                    d.child(
                        div()
                            .truncate()
                            .text_size(px(12.0))
                            .text_color(rgba(th.text_faint))
                            .child(person.email.clone()),
                    )
                }),
        )
        .child(
            div()
                .flex_none()
                .flex()
                .flex_col()
                .items_end()
                .text_size(px(12.0))
                .text_color(rgba(th.text_faint))
                .child(tr!("app-contacts-messages", count = person.messages))
                .child(last),
        )
        .into_any_element()
}
