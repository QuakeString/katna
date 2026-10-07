// SPDX-License-Identifier: GPL-3.0-or-later

//! A new event in a small window of its own: the desktop clock's Add…
//! (`katna-mail --capture event:YYYY-MM-DD`). It is the Calendar's small
//! card for a new event, without the mail window around it; More options
//! opens the whole editor in the mail window (opened if need be), and the
//! window closes once the event is saved.

use std::rc::Rc;
use std::time::Duration;

use gpui::{App, Context, Decorations, Window, div, point, prelude::*, rgba, size};
use jiff::civil::Date;
use katna_chrome::window_options;
use katna_core::ids::MAIL_APP_ID;
use katna_dbus::app_action;
use katna_i18n::tr;
use katna_ui::px;
use katna_ui::tokens::space;
use katna_ui::scale::desktop_px;

use super::calendar::read_calendars;
use super::event_edit::next_hour;
use super::{MailWindow, RailApp, WINDOW_CONTEXT, capture};
use crate::instance::Request;

/// The window's size: the small card, with room under it for its pickers.
const WIDTH: f32 = 520.0;
const HEIGHT: f32 = 360.0;

/// How long More options waits for the mail window before this one closes
/// anyway.
const MAIN_WAIT: Duration = Duration::from_secs(10);

/// Opens the New event window on `day` (`YYYY-MM-DD`); an open one comes
/// forward instead.
pub(super) fn open(day: &str, cx: &mut App) {
    let Ok(day) = day.parse::<Date>() else {
        tracing::warn!(day, "not a day for a new event");
        return;
    };
    if let Some(handle) = capture::event_window(cx)
        && handle
            .update(cx, |_, window, _| window.activate_window())
            .is_ok()
    {
        return;
    }
    let Some((env, paths, font)) = capture::host(cx) else {
        return;
    };
    let mut options = window_options(
        &env,
        MAIL_APP_ID,
        tr!("calendar-event-window-title"),
        size(desktop_px(WIDTH), desktop_px(HEIGHT)),
        cx,
    );
    options.is_resizable = false;
    options.is_minimizable = false;
    options.window_min_size = None;
    let opened = cx.open_window(options, |window, cx| {
        cx.new(|cx| MailWindow::event_window(env, paths, font, day, window, cx))
    });
    match opened {
        Ok(handle) => {
            capture::set_event_window(Some(handle), cx);
            let _ = handle.update(cx, |_, window, _| window.activate_window());
        }
        Err(err) => tracing::warn!("cannot open the New event window: {err}"),
    }
}

impl MailWindow {
    fn event_window(
        env: katna_chrome::Environment,
        paths: katna_core::Paths,
        font: Option<gpui::SharedString>,
        day: Date,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut this = Self::build(env, paths, font, window, cx);
        this.event_only = true;
        this.app = RailApp::Calendar;
        this.calendar.calendars = Rc::new(read_calendars(&this.paths, &this.calendar_left_out()));
        this.watch_colors(cx);
        // The next hour today, 9 AM on another day.
        let start = next_hour(day, &this.tz);
        this.start_new_event(day, Some(start), false, point(px(0.0), px(0.0)), window, cx);
        this
    }

    /// More options: the whole editor in the mail window, with the day and
    /// the title typed here; this window closes once that one is open.
    pub(super) fn more_options_in_main(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some((day, title)) = self.draft_day_and_title(cx) else {
            return;
        };
        let page = app_action::new_event_page(&day.to_string(), &title);
        capture::ask_app(Request::Page(page), cx);
        // Gone at once from the screen; closed after the mail window
        // opened, as the app ends when its last window closes.
        window.minimize_window();
        let windows = cx.windows().len();
        cx.spawn_in(window, async move |_, cx| {
            let start = std::time::Instant::now();
            while start.elapsed() < MAIN_WAIT {
                if cx
                    .update(|_, cx| cx.windows().len() > windows)
                    .unwrap_or(true)
                {
                    break;
                }
                cx.background_executor()
                    .timer(Duration::from_millis(100))
                    .await;
            }
            let _ = cx.update(|window, _| window.remove_window());
        })
        .detach();
    }

    /// The window: the card, and what opens over it (pickers, the
    /// snackbar of a failed save).
    pub(super) fn render_event_window(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> gpui::AnyElement {
        // Saved, or closed: nothing is left to show.
        if self.calendar.draft.is_none() && !self.event_saving {
            capture::set_event_window(None, cx);
            window.remove_window();
            return div().into_any_element();
        }
        let th = self.theme(window);
        let reduce = cx.reduce_motion();
        let server_frame = matches!(window.window_decorations(), Decorations::Server);
        let card = self.calendar.draft.as_ref().map(|draft| {
            self.quick_card_body(draft, &th, cx)
                .size_full()
                .px(px(space::S3))
                .pt(px(space::S6))
                .pb(px(space::S5))
        });
        let pick = self.render_draft_pick(&th, cx);
        let snackbar = self.render_snackbar(&th, window, reduce, cx);
        let content = div()
            .key_context(WINDOW_CONTEXT)
            .relative()
            .size_full()
            .bg(rgba(th.surface))
            .text_color(rgba(th.text))
            .children(card)
            .children(pick)
            .children(snackbar)
            .into_any_element();
        let page = if server_frame {
            div().size_full().child(content)
        } else {
            self.chrome
                .render_bar(katna_chrome::Bar::default(), content, window, cx)
        };
        match &self.font {
            Some(font) => page.font_family(font.clone()).into_any_element(),
            None => page.into_any_element(),
        }
    }
}
