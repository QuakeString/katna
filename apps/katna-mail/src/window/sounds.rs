// SPDX-License-Identifier: GPL-3.0-or-later

//! The sounds Katna plays: the Sounds row of Settings > Notifications,
//! with a line per event (a sound to pick, a button that plays it and a
//! switch), the menu of sounds, and the sounds Katna Mail plays itself
//! (mail sent, mail not sent). The daemon plays the others with its
//! notifications.

use gpui::{AnyElement, Context, Pixels, Point, div, prelude::*, rgba};
use katna_core::config::SoundEvent;
use katna_i18n::tr;
use katna_platform::sound;
use katna_ui::px;

use super::MailWindow;
use super::context_menu::{Rows, menu_row_with};
use super::settings::Change;
use crate::theme::Theme;
use crate::widgets::{icon, icon_button, tip};

/// The id of an event's name and of the line under it.
fn event_text(event: SoundEvent) -> (&'static str, &'static str) {
    match event {
        SoundEvent::NewMail => ("sounds-new-mail", "sounds-new-mail-detail"),
        SoundEvent::Reminders => ("sounds-reminders", "sounds-reminders-detail"),
        SoundEvent::MailBack => ("sounds-mail-back", "sounds-mail-back-detail"),
        SoundEvent::Sent => ("sounds-sent", "sounds-sent-detail"),
        SoundEvent::NotSent => ("sounds-not-sent", "sounds-not-sent-detail"),
    }
}

/// The name people see of sound `id`.
fn sound_name(id: &str) -> String {
    tr!(match id {
        "message-new-email" | "Mail" => "sound-new-email",
        "message-new-instant" | "IM" => "sound-new-message",
        "message-sent-email" => "sound-sent",
        "alarm-clock-elapsed" | "Alarm" => "sound-alarm",
        "bell" => "sound-bell",
        "complete" => "sound-complete",
        "dialog-information" => "sound-information",
        "dialog-warning" => "sound-warning",
        "dialog-error" | "Error" => "sound-error",
        "Reminder" => "sound-reminder",
        _ => "sound-default",
    })
}

/// A stable number for event `event`, for element ids.
fn index(event: SoundEvent) -> usize {
    SoundEvent::ALL
        .iter()
        .position(|e| *e == event)
        .unwrap_or_default()
}

impl MailWindow {
    /// Plays `event`'s sound, if it is on, unless Do not disturb is on.
    pub(super) fn play_event_sound(&self, event: SoundEvent) {
        if let Some(chosen) = self.config.sounds.playing(event) {
            sound::play_unless_quiet(sound::resolve(event, chosen));
        }
    }

    /// The Sounds row's lines, one per event.
    pub(super) fn sound_lines(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .children(SoundEvent::ALL.map(|event| self.sound_line(event, th, cx)))
            .into_any_element()
    }

    /// A line of the Sounds row: what the sound is for, then the sound
    /// (a click opens the menu of sounds), a button that plays it and the
    /// switch. A click anywhere else turns the sound on or off.
    fn sound_line(&self, event: SoundEvent, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let (name, detail) = event_text(event);
        let setting = self.config.sounds.get(event);
        let on = setting.on;
        let current = sound::resolve(event, &setting.sound);
        let ix = index(event);
        let picker = self
            .page_control(div().id(("sound-pick", ix)), th, cx)
            .h(px(32.0))
            .w(px(152.0))
            .flex_none()
            .pl(px(14.0))
            .pr(px(8.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .rounded_full()
            .bg(rgba(th.chip))
            .text_size(px(13.0))
            .when(!on, |d| d.text_color(rgba(th.text_faint)))
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .on_click(cx.listener(move |this, event_: &gpui::ClickEvent, _, cx| {
                cx.stop_propagation();
                this.open_sound_menu(event, event_.position(), cx);
            }))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .truncate()
                    .child(sound_name(current)),
            )
            .child(icon("chevron-down", th.text_dim, 16.0));
        let play = icon_button(("sound-play", ix), "play", 16.0, th)
            .size(px(32.0))
            .tooltip(tip(tr!("sounds-play"), th))
            .on_click(cx.listener(move |_, _, _, cx| {
                cx.stop_propagation();
                sound::play(current);
            }));
        let controls = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .child(picker)
            .child(play)
            .into_any_element();
        self.switch_row_with(
            match event {
                SoundEvent::NewMail => "page-sound-new-mail",
                SoundEvent::Reminders => "page-sound-reminders",
                SoundEvent::MailBack => "page-sound-mail-back",
                SoundEvent::Sent => "page-sound-sent",
                SoundEvent::NotSent => "page-sound-not-sent",
            },
            tr!(name),
            tr!(detail),
            on,
            Change::Sound(event, !on),
            Some(controls),
            th,
            cx,
        )
    }

    /// Opens the menu of sounds for `event` where it was clicked.
    fn open_sound_menu(&mut self, event: SoundEvent, at: Point<Pixels>, cx: &mut Context<Self>) {
        self.open_sound_context_menu(event, at, cx);
    }

    /// The menu of sounds for `event`: each plays as it is picked, the
    /// chosen one ticked.
    pub(super) fn sound_menu_rows(
        &self,
        event: SoundEvent,
        rh: f32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Rows {
        let current = sound::resolve(event, &self.config.sounds.get(event).sound);
        let mut rows = Rows::new(rh);
        for (n, choice) in sound::choices().iter().enumerate() {
            let id = choice.id;
            let lead = if id == current {
                icon("check", th.accent, 20.0)
            } else {
                div().size(px(20.0)).into_any_element()
            };
            rows.item(
                menu_row_with(("sound-choice", n), lead, sound_name(id).into(), th, rh).on_click(
                    cx.listener(move |this, _, _, cx| {
                        this.close_context_menu(cx);
                        sound::play(id);
                        this.apply(Change::SoundChoice(event, id), cx);
                    }),
                ),
            );
        }
        rows
    }
}
