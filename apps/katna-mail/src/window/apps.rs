// SPDX-License-Identifier: GPL-3.0-or-later

//! The app rail at the far left: Mail, Calendar, Contacts, Tasks, Notes
//! and Files, with settings at the bottom; their names can be hidden in
//! quick settings. Each app is a page of the one window: the rail, Ctrl+1
//! to Ctrl+5 (Outlook's keys), the Go menu, the desktop file's actions and
//! `katna-mail --page NAME` (D-Bus `ActivateAction("open-page", [NAME])`)
//! all switch pages through [`MailWindow::show_page`].
//!
//! Adding a page: give it a module of its own under `window/` with a
//! `render_<name>_page` method (as `calendar.rs` has), call it from its arm
//! in [`MailWindow::render_app_page`], and load what it needs in
//! [`MailWindow::open_app`]'s arm. Pages without one show "coming soon".

use crate::widgets::Tip as _;
use katna_ui::WindowDrag;
use std::ops::Range;
use std::rc::Rc;

use gpui::{
    AnimationExt, AnyElement, Context, FontWeight, SpringAnimation, Window, div, prelude::*, rgba,
    uniform_list,
};
use katna_core::config::AppKind;
use katna_i18n::tr;
use katna_store::Person;
use katna_ui::Ripple;
use katna_ui::motion;
use katna_ui::px;

use super::{MailWindow, OpenSettings};
use crate::format;
use crate::theme::{Theme, fade};
use crate::widgets::{icon, icon_button_colored};

pub(super) const APP_RAIL_WIDTH: f32 = 72.0;
/// Room for an app's button in the rail, name and all, as it folds.
const RAIL_ITEM_ROOM: f32 = 64.0;

/// The apps of the rail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum App {
    Mail,
    Calendar,
    Contacts,
    Tasks,
    Notes,
    Files,
}

impl App {
    pub(super) const ALL: [Self; 6] = [
        Self::Mail,
        Self::Calendar,
        Self::Contacts,
        Self::Tasks,
        Self::Notes,
        Self::Files,
    ];

    /// The app's switch in Settings > Apps; Mail has none, it is always
    /// on.
    pub(super) fn kind(self) -> Option<AppKind> {
        match self {
            Self::Mail => None,
            Self::Calendar => Some(AppKind::Calendar),
            Self::Contacts => Some(AppKind::Contacts),
            Self::Tasks => Some(AppKind::Tasks),
            Self::Notes => Some(AppKind::Notes),
            Self::Files => Some(AppKind::Files),
        }
    }

    /// The rail's app for `kind`.
    pub(super) fn of(kind: AppKind) -> Self {
        match kind {
            AppKind::Calendar => Self::Calendar,
            AppKind::Contacts => Self::Contacts,
            AppKind::Tasks => Self::Tasks,
            AppKind::Notes => Self::Notes,
            AppKind::Files => Self::Files,
        }
    }

    /// The Go menu's action that shows the page.
    pub(super) fn action_name(self) -> &'static str {
        match self {
            Self::Mail => "katna_mail::ShowMail",
            Self::Calendar => "katna_mail::ShowCalendar",
            Self::Contacts => "katna_mail::ShowContacts",
            Self::Tasks => "katna_mail::ShowTasks",
            Self::Notes => "katna_mail::ShowNotes",
            Self::Files => "katna_mail::ShowFiles",
        }
    }

    pub(super) fn label(self) -> String {
        tr!(match self {
            Self::Mail => "rail-mail",
            Self::Calendar => "rail-calendar",
            Self::Contacts => "rail-contacts",
            Self::Tasks => "rail-tasks",
            Self::Notes => "rail-notes",
            Self::Files => "rail-files",
        })
    }

    /// The page's name for `--page` and `open-page`, and in the saved
    /// window state.
    pub(super) fn key(self) -> &'static str {
        match self {
            Self::Files => "files",
            _ => self.icon(),
        }
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
            Self::Files => "attachment",
        }
    }

    /// The big button at the top of the left bar: the page's own action,
    /// its icon and its word. Files has nothing to create, so it writes.
    pub(super) fn primary(self) -> (&'static str, String) {
        match self {
            Self::Calendar => ("event", tr!("calendar-menu-new-event")),
            Self::Contacts => ("person-add", tr!("contacts-create")),
            Self::Tasks => ("add", tr!("tasks-create")),
            Self::Notes => ("pen", tr!("notes-new-note")),
            Self::Mail | Self::Files => ("compose", tr!("compose")),
        }
    }

    /// What the app will do, for its "coming soon" page.
    fn promise(self) -> String {
        match self {
            Self::Mail | Self::Contacts | Self::Tasks | Self::Files => String::new(),
            Self::Calendar => tr!("app-calendar-promise"),
            Self::Notes => tr!("app-notes-promise"),
        }
    }
}

/// The people list of the Contacts page.
pub(super) enum People {
    Loading,
    Loaded(Rc<Vec<Person>>),
    Failed(String),
}

/// A page's side column (calendars, lists, labels): docked beside the
/// page on a desktop, or on a phone or tablet a drawer the menu button
/// opens over it, as Mail's folders do.
pub(super) struct PageSide {
    /// Where the column was: beside the page.
    pub(super) docked: Option<AnyElement>,
    /// Last child of the page, which must be `relative`.
    pub(super) drawer: Option<AnyElement>,
}

impl MailWindow {
    /// Places a page's side column, `width` wide. `closes` lets a click on
    /// it close the drawer, for columns that pick what the page shows.
    pub(super) fn page_side(
        &self,
        side: AnyElement,
        width: f32,
        closes: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> PageSide {
        if self.layout.shape.is_desktop() {
            // Folded by the menu button, it narrows away and fades as
            // Mail's folders do; its content keeps its width meanwhile.
            let t = self.page_side_t.clamp(0.0, 1.0);
            let docked = (t > 0.001).then(|| {
                div()
                    .flex_none()
                    .h_full()
                    .w(px(width * t))
                    .overflow_hidden()
                    .opacity(t)
                    .child(
                        div()
                            .h_full()
                            .w(px(width))
                            .flex()
                            .flex_col()
                            // The big button heads the column, as Compose
                            // heads Mail's folders.
                            .child(div().flex_none().h(px(self.side_button_room())))
                            .child(div().flex_1().min_h_0().child(side)),
                    )
                    .into_any_element()
            });
            return PageSide {
                docked,
                drawer: None,
            };
        }
        let t = self.layout.drawer_t();
        if t <= 0.001 {
            return PageSide {
                docked: None,
                drawer: None,
            };
        }
        let panel = div()
            .id("page-drawer")
            .occlude()
            .absolute()
            .top_0()
            .bottom_0()
            .left(px(-width * (1.0 - t)))
            .w(px(width))
            .overflow_hidden()
            .bg(rgba(th.surface))
            .shadow(crate::widgets::elevation(th, 3.0 * t))
            .when(closes, |d| {
                d.on_click(cx.listener(|this, _, _, cx| this.close_drawer(cx)))
            })
            .child(side);
        let drawer = div()
            .absolute()
            .top_0()
            .bottom_0()
            .left_0()
            .right_0()
            .children(self.render_scrim(self.layout.shape.width, cx))
            .child(panel)
            .into_any_element();
        PageSide {
            docked: None,
            drawer: Some(drawer),
        }
    }

    /// Whether the pages show their side column beside them on a desktop:
    /// one fold for Mail's folders and every page's column.
    pub(super) fn page_side_open(&self) -> bool {
        self.nav_open
    }

    /// The big button's action on the page on show.
    /// The icon and words of the big button at the top of the side
    /// column: the page's own action, or Upload while a drive is open.
    pub(super) fn primary_button(&self) -> (&'static str, String) {
        if self.drive_upload_here() {
            ("upload", tr!("files-drive-upload"))
        } else {
            self.app.primary()
        }
    }

    /// The big button's icon, turning from the last page's into this
    /// one's: the same on the rail's square, the pill and a phone's button.
    pub(super) fn primary_icon(&self, th: &crate::theme::Theme) -> gpui::AnyElement {
        crate::widgets::morph_icon(
            self.primary_icon_from,
            self.primary_icon,
            self.primary_icon_turn.value(),
            th.compose_text,
            24.0,
        )
    }

    /// The big button's word, rolling from the last page's into this
    /// one's in a box as wide as the button gives it this frame.
    pub(super) fn primary_label(&self) -> gpui::AnyElement {
        crate::widgets::morph_label(
            &self.primary_label_from,
            &self.primary_label,
            self.primary_icon_turn.value(),
            self.primary_label_width,
        )
    }

    pub(super) fn primary_action(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.drive_upload_here() {
            self.upload_into_drive(false, cx);
            return;
        }
        match self.app {
            App::Calendar => self.create_event_button(window, cx),
            App::Contacts => {
                self.contacts.open = None;
                self.start_contact_edit(None, window, cx);
            }
            App::Tasks => self.tasks_create(window, cx),
            App::Notes => self.new_note(window, cx),
            App::Mail | App::Files => self.compose(&super::Compose, window, cx),
        }
    }

    /// The room a side column `width` wide takes beside the page now: none
    /// on a phone or tablet, where it is a drawer, or while folded.
    pub(super) fn page_side_width(&self, width: f32) -> f32 {
        if self.layout.shape.is_desktop() {
            width * self.page_side_t.clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    /// Shows page `app`, leaving Settings as picking a folder does.
    pub(super) fn show_page(&mut self, app: App, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(kind) = app.kind().filter(|_| !self.app_on(app)) {
            self.say_app_off(kind, cx);
            return;
        }
        if self.settings_page.is_some() {
            self.close_settings_page(window, cx);
        }
        self.open_app(app, cx);
        self.focus_app_page(window, cx);
    }

    /// Gives the keys to the page on show, so keys such as Ctrl+Z reach it
    /// rather than the hidden mail list.
    pub(super) fn focus_app_page(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        match self.app {
            App::Mail => window.focus(&self.list_focus, cx),
            App::Calendar => window.focus(&self.calendar.focus, cx),
            App::Contacts => window.focus(&self.window_focus, cx),
            App::Tasks => {
                if let Some(focus) = &self.tasks.focus {
                    window.focus(focus, cx);
                }
            }
            App::Notes | App::Files => {}
        }
    }

    /// Hands the search box to the app on show (`entering`), or back to
    /// mail, as switching apps does, for the Settings page opening over it
    /// and closing.
    pub(super) fn swap_app_search(&mut self, entering: bool, cx: &mut Context<Self>) {
        match self.app {
            App::Calendar => self.swap_calendar_search(entering, cx),
            App::Tasks => self.swap_tasks_search(entering, cx),
            App::Files => self.swap_files_search(entering, cx),
            App::Contacts => self.swap_contacts_search(entering, cx),
            App::Notes => self.sync_notes_search(cx),
            App::Mail => {}
        }
    }

    /// Whether `app` is turned on in Settings > Apps.
    pub(super) fn app_on(&self, app: App) -> bool {
        app.kind().is_none_or(|kind| self.config.app_on(kind))
    }

    /// The apps turned on, in the rail's order.
    pub(super) fn apps(&self) -> impl Iterator<Item = App> + '_ {
        App::ALL.into_iter().filter(|app| self.app_on(*app))
    }

    /// Only Mail is on: there is nothing to switch to, so the rail and the
    /// phone's bottom bar go.
    pub(super) fn mail_only(&self) -> bool {
        self.config.apps.mail_only()
    }

    pub(super) fn open_app(&mut self, app: App, cx: &mut Context<Self>) {
        if self.app == app {
            return;
        }
        // A turned-off app opens from nowhere: its key, a launcher's
        // action, a reminder or a link lands here and says so instead.
        if let Some(kind) = app.kind()
            && !self.config.app_on(kind)
        {
            self.say_app_off(kind, cx);
            return;
        }
        let from = self.app;
        self.app = app;
        // An event or task picked up on the Calendar stays where it was.
        self.cancel_calendar_drags();
        // Each page shows its side column as it left it, without motion.
        let open = if self.page_side_open() { 1.0 } else { 0.0 };
        self.page_side_spring.snap(open);
        self.page_side_t = open;
        super::desktop::menu_page_changed(app != App::Mail, cx);
        // Notes and Tasks hand the search box back before Contacts takes
        // it, and take it after Contacts hands it back.
        if from == App::Notes {
            self.sync_notes_search(cx);
        }
        if from == App::Calendar {
            self.swap_calendar_search(false, cx);
        }
        if from == App::Tasks {
            self.swap_tasks_search(false, cx);
        }
        if from == App::Files {
            self.swap_files_search(false, cx);
        }
        if from == App::Contacts || app == App::Contacts {
            // The search box follows: contacts on this page, mail elsewhere.
            self.swap_contacts_search(app == App::Contacts, cx);
        }
        if app == App::Tasks {
            self.swap_tasks_search(true, cx);
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
            self.swap_calendar_search(true, cx);
            self.load_calendar(cx);
        }
        if app == App::Notes {
            self.sync_notes_search(cx);
        }
        if app == App::Tasks {
            self.open_tasks_page(cx);
        }
        if app == App::Files {
            self.swap_files_search(true, cx);
            self.load_library(cx);
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

    /// Whether a page shows a whole editor in place of itself and its
    /// side column (Calendar's event editor), with no room for the big
    /// button.
    pub(super) fn page_editor_open(&self) -> bool {
        self.app == App::Calendar && self.event_editor_open()
    }

    /// Room at the top of a page's side column for its big button.
    pub(super) fn side_button_room(&self) -> f32 {
        super::COMPOSE_NAV_ROOM * self.compose_shown.value().clamp(0.0, 1.0)
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
        // An app turned off in Settings > Apps folds away; one turned on
        // grows back in its place.
        let mail_only = self.mail_only();
        let items = App::ALL.into_iter().map(|app| {
            let on = self.app == app;
            let item = div()
                .id(("app", app as usize))
                .w(px(APP_RAIL_WIDTH))
                .pt(px(4.0))
                .pb(px(8.0))
                .flex()
                .flex_col()
                .items_center()
                .gap(px(4.0))
                .cursor_pointer()
                .keeps_press()
                .group("app")
                .when(app == App::Mail, |d| {
                    d.on_hover(cx.listener(|this, hovered: &bool, _, cx| {
                        this.hover_navigation(super::Hover::Mail, *hovered, cx)
                    }))
                })
                .on_click(cx.listener(move |this, _, window, cx| this.show_page(app, window, cx)))
                .on_mouse_down(
                    gpui::MouseButton::Right,
                    cx.listener(move |this, event: &gpui::MouseDownEvent, _, cx| {
                        this.open_rail_menu(app, event.position, cx);
                    }),
                )
                .child(app_face(app, on, false, th))
                .when(!labels, |d| d.tip(app.label(), th))
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
                            SpringAnimation::new(katna_ui::motion::scaled(motion::SLIDE))
                                .to(if labels { 1.0 } else { 0.0 }),
                            |el, s: f32| {
                                let s = s.clamp(0.0, 1.0);
                                el.h(px(16.0 * s)).opacity(s)
                            },
                        ),
                );
            div().overflow_hidden().child(item).with_spring(
                ("app-on", app as usize),
                SpringAnimation::new(katna_ui::motion::scaled(motion::SLIDE))
                    // With only Mail on there is nothing to switch to, so
                    // Mail's button goes too; Compose and Settings stay.
                    .to(if self.app_on(app) && !mail_only {
                        1.0
                    } else {
                        0.0
                    }),
                |el, s: f32| {
                    let s = s.clamp(0.0, 1.0);
                    // Taller than an app's button with its name.
                    el.max_h(px(RAIL_ITEM_ROOM * s)).opacity(s)
                },
            )
        });
        div()
            .id("app-rail")
            .window_drag()
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
                    // The tour skips the rail when there is nothing to switch to.
                    .when(!self.mail_only(), |d| {
                        d.child(self.tour_mark(super::tour::Spot::Apps))
                    })
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
                .tip(tr!("settings"), th)
                .on_click(cx.listener(|this, _, window, cx| {
                    if this.settings_page.is_some() {
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
            App::Calendar => {
                // After Ctrl+2 focus is still on the mail list, which is not
                // on show, so the page's keys and Ctrl+Z would go nowhere.
                let focused = window.focused(cx);
                if focused.is_none()
                    || self.list_focus.is_focused(window)
                    || self.window_focus.is_focused(window)
                {
                    window.focus(&self.calendar.focus, cx);
                }
                self.render_calendar_page(th, cx)
            }
            App::Notes => self.render_notes(th, window, cx),
            App::Tasks => self.render_tasks(th, cx),
            App::Files => self.render_files(th, window, cx),
            App::Mail => self.render_coming_soon(th),
        };
        // Edge to edge on a phone, as Mail's cards are.
        let shape = self.layout.shape;
        div()
            .flex_1()
            .min_w_0()
            .h_full()
            .pr(px(shape.card_margin()))
            .pb(px(shape.card_margin()))
            .child(
                div()
                    .size_full()
                    .rounded(px(shape.card_radius()))
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
            .child(crate::widgets::tag(tr!("app-coming-soon"), th).font_weight(FontWeight::MEDIUM))
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
                gpui::Animation::new(katna_ui::motion::time(std::time::Duration::from_millis(
                    260,
                )))
                .with_easing(gpui::ease_out_quint()),
                |el, t| el.opacity(t).mt(px(12.0 * (1.0 - t))),
            )
            .into_any_element()
    }

    /// The people the mail was exchanged with, most first: Frequent.
    pub(super) fn render_contacts(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let people = match &self.people {
            None | Some(People::Loading) => {
                return self.placeholder(tr!("app-contacts-loading"), th);
            }
            Some(People::Failed(err)) => return self.placeholder(err.clone(), th),
            Some(People::Loaded(people)) => people.clone(),
        };
        if people.is_empty() {
            return self.placeholder(tr!("app-contacts-empty"), th);
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
        .keeps_press()
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
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(6.0))
                        .child(
                            div()
                                .min_w_0()
                                .truncate()
                                .text_size(px(14.0))
                                .text_color(rgba(th.text))
                                .child(name.clone().unwrap_or_else(|| person.email.clone())),
                        )
                        .children(this.muted_mark(&person.email, 16.0, th)),
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

/// An app's button in the rail and in the phone's bottom bar: its icon on a
/// round button. A press sends a wave from the middle, and the selected pill
/// grows from the middle inside the button, which keeps its size, so the
/// wave is never cut short and nothing jumps under the pointer.
pub(super) fn app_face(app: App, on: bool, bottom: bool, th: &Theme) -> gpui::Div {
    let ix = app as usize;
    let (pill, glow, ripple) = if bottom {
        ("bottom-pill", "bottom-glow", "bottom-ripple")
    } else {
        ("app-pill", "app-glow", "app-ripple")
    };
    div()
        .relative()
        .overflow_hidden()
        .w(px(56.0))
        .h(px(32.0))
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .child(div().absolute().inset_0().flex().justify_center().child(
            div().h_full().rounded_full().with_spring(
                (pill, ix),
                SpringAnimation::new(katna_ui::motion::scaled(motion::SLIDE)).to(if on {
                    1.0
                } else {
                    0.0
                }),
                {
                    let bg = th.nav_selected;
                    move |el, s: f32| {
                        let s = s.clamp(0.0, 1.0);
                        el.bg(rgba(fade(bg, s))).w(px(32.0 + 24.0 * s))
                    }
                },
            ),
        ))
        .child(crate::widgets::hover_fade((glow, ix), None, th))
        .child(Ripple::new((ripple, ix), rgba(th.ripple)).centered())
        .child(icon(
            app.icon(),
            if on {
                th.nav_selected_text
            } else {
                th.text_dim
            },
            22.0,
        ))
}
