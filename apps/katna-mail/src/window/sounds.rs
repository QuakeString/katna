// SPDX-License-Identifier: GPL-3.0-or-later

//! The sounds Katna plays: the Sounds row of Settings > Notifications,
//! with the sets of sounds as tiles (a click picks one, its button plays
//! it), a line per event (a sound to pick, a button that plays it and a
//! switch), the menu of sounds with a file of the user's own, and the
//! sounds Katna Mail plays itself (mail sent, mail not sent). The daemon
//! plays the others with its notifications.

use std::path::{Path, PathBuf};

use gpui::{
    AnyElement, Context, FontWeight, PathPromptOptions, Pixels, Point, div, prelude::*, rgba,
};
use katna_core::config::SoundEvent;
use katna_i18n::tr;
use katna_platform::sound;
use katna_ui::px;

use super::MailWindow;
use super::context_menu::{Rows, menu_row, menu_row_with};
use super::settings::Change;
use crate::theme::Theme;
use crate::widgets::{icon, icon_button, tip};

/// The kinds of file a sound of the user's own can be.
const FILE_KINDS: [&str; 6] = ["wav", "ogg", "oga", "opus", "flac", "mp3"];
/// The largest file taken as a sound: a few seconds of sound.
const MOST_FILE_BYTES: u64 = 10 * 1024 * 1024;

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

/// A set's name, what it holds and its icon.
fn set_text(id: &str) -> (String, String, &'static str) {
    let (name, detail, icon) = match id {
        sound::SYSTEM => ("sound-set-system", "sound-set-system-detail", "volume"),
        "katna" => ("sound-set-katna", "sound-set-katna-detail", "bell"),
        "nature" => ("sound-set-nature", "sound-set-nature-detail", "water-drop"),
        "birds" => ("sound-set-birds", "sound-set-birds-detail", "bird"),
        "animals" => ("sound-set-animals", "sound-set-animals-detail", "paw"),
        "insects" => ("sound-set-insects", "sound-set-insects-detail", "bug"),
        "electronic" => (
            "sound-set-electronic",
            "sound-set-electronic-detail",
            "chip",
        ),
        _ => ("sound-set-morning", "sound-set-morning-detail", "sunrise"),
    };
    (tr!(name), tr!(detail), icon)
}

/// The name people see of sound `id`.
fn sound_name(id: &str) -> String {
    if let Some(path) = sound::file_of(id) {
        return path
            .file_stem()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
    }
    tr!(match id {
        sound::CHIME => "sound-katna-chime",
        "katna.twin-bells" => "sound-katna-twin-bells",
        "katna.soft-bell" => "sound-katna-soft-bell",
        "katna.rising" => "sound-katna-rising",
        "katna.falling" => "sound-katna-falling",
        "nature.water-drop" => "sound-nature-water-drop",
        "nature.wind-chimes" => "sound-nature-wind-chimes",
        "nature.ripple" => "sound-nature-ripple",
        "nature.breeze" => "sound-nature-breeze",
        "nature.pebble" => "sound-nature-pebble",
        "birds.robin" => "sound-birds-robin",
        "birds.wren-trill" => "sound-birds-wren-trill",
        "birds.finch" => "sound-birds-finch",
        "birds.swallow" => "sound-birds-swallow",
        "birds.cuckoo" => "sound-birds-cuckoo",
        "animals.tree-frog" => "sound-animals-tree-frog",
        "animals.owl" => "sound-animals-owl",
        "animals.cat" => "sound-animals-cat",
        "animals.dolphin" => "sound-animals-dolphin",
        "animals.bullfrog" => "sound-animals-bullfrog",
        "insects.cricket" => "sound-insects-cricket",
        "insects.cicada" => "sound-insects-cicada",
        "insects.katydid" => "sound-insects-katydid",
        "insects.bee" => "sound-insects-bee",
        "insects.fly" => "sound-insects-fly",
        "electronic.blip" => "sound-electronic-blip",
        "electronic.beacon" => "sound-electronic-beacon",
        "electronic.arcade" => "sound-electronic-arcade",
        "electronic.whoosh" => "sound-electronic-whoosh",
        "electronic.buzz" => "sound-electronic-buzz",
        "morning.kalimba" => "sound-morning-kalimba",
        "morning.sunrise" => "sound-morning-sunrise",
        "morning.marimba" => "sound-morning-marimba",
        "morning.glockenspiel" => "sound-morning-glockenspiel",
        "morning.low-marimba" => "sound-morning-low-marimba",
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

/// A short stable name of `event`, for the file of the user's own.
fn file_key(event: SoundEvent) -> &'static str {
    match event {
        SoundEvent::NewMail => "new-mail",
        SoundEvent::Reminders => "reminders",
        SoundEvent::MailBack => "mail-back",
        SoundEvent::Sent => "sent",
        SoundEvent::NotSent => "not-sent",
    }
}

impl MailWindow {
    /// Plays `event`'s sound, if it is on, unless Do not disturb is on.
    pub(super) fn play_event_sound(&self, event: SoundEvent) {
        let sounds = &self.config.sounds;
        if let Some(chosen) = sounds.playing(event) {
            sound::play_unless_quiet(sound::resolve(event, &sounds.set, chosen));
        }
    }

    /// The sound `event` plays, on or off.
    fn event_sound(&self, event: SoundEvent) -> String {
        let sounds = &self.config.sounds;
        sound::resolve(event, &sounds.set, &sounds.get(event).sound)
    }

    /// Where Katna keeps its copies of the user's own sound files.
    fn sound_files_dir(&self) -> PathBuf {
        self.paths.data_dir().join("sounds")
    }

    /// Names `id` as `event`'s sound (empty for its set's), letting go of
    /// Katna's copy of a file it named before.
    pub(super) fn set_event_sound(&mut self, event: SoundEvent, id: String) {
        let old = std::mem::replace(&mut self.config.sounds.get_mut(event).sound, id);
        if let Some(path) = sound::file_of(&old)
            && path.starts_with(self.sound_files_dir())
            && self.config.sounds.get(event).sound != old
        {
            let _ = std::fs::remove_file(path);
            // Its folder of its own, now empty.
            if let Some(folder) = path.parent() {
                let _ = std::fs::remove_dir(folder);
            }
        }
    }

    /// The Sounds row: the sets as tiles, then a line per event.
    pub(super) fn sound_lines(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        div()
            .flex()
            .flex_col()
            .gap(px(12.0))
            .child(self.sound_sets(th, cx))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .children(SoundEvent::ALL.map(|event| self.sound_line(event, th, cx))),
            )
            .into_any_element()
    }

    /// The sets of sounds as tiles: an icon, the set's name and what it
    /// holds, and a button that plays its new-mail sound. The picked one
    /// is ringed in the accent, with a check.
    fn sound_sets(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let picked = sound::set(&self.config.sounds.set).id;
        // Two columns: the Settings column is narrow beside its labels,
        // and a set's name and what it holds stay whole.
        div()
            .grid()
            .grid_cols(2)
            .gap(px(8.0))
            .children(sound::SETS.iter().enumerate().map(|(ix, set)| {
                let id = set.id;
                let on = id == picked;
                let (name, detail, glyph) = set_text(id);
                let first = set.sounds[0];
                let play = icon_button(("sound-set-play", ix), "play", 16.0, th)
                    .size(px(30.0))
                    .flex_none()
                    .tooltip(tip(tr!("sounds-play"), th))
                    .on_click(cx.listener(move |_, _, _, cx| {
                        cx.stop_propagation();
                        sound::play(first);
                    }));
                self.page_control(div().id(("sound-set", ix)), th, cx)
                    .relative()
                    .min_w_0()
                    .h(px(56.0))
                    .pl(px(10.0))
                    .pr(px(6.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(10.0))
                    .rounded(px(12.0))
                    .border_1()
                    .border_color(rgba(if on { th.accent } else { th.divider }))
                    .when(on, |d| d.border_2().pl(px(9.0)).pr(px(5.0)))
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        sound::play(first);
                        this.apply(Change::SoundSet(id), cx);
                    }))
                    .child(
                        div()
                            .flex_none()
                            .size(px(34.0))
                            .rounded(px(10.0))
                            .bg(rgba(th.chip))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(icon(glyph, th.text_dim, 20.0)),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .when(on, |d| d.font_weight(FontWeight::MEDIUM))
                                    .truncate()
                                    .child(name),
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(rgba(th.text_faint))
                                    .truncate()
                                    .child(detail),
                            ),
                    )
                    .child(play)
                    .when(on, |d| {
                        d.child(
                            div()
                                .absolute()
                                .top(px(-7.0))
                                .right(px(-7.0))
                                .size(px(18.0))
                                .rounded_full()
                                .bg(rgba(th.accent))
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(icon("check", th.on_accent, 12.0)),
                        )
                    })
            }))
            .into_any_element()
    }

    /// A line of the Sounds row: what the sound is for, then the sound
    /// (a click opens the menu of sounds), a button that plays it and the
    /// switch. A click anywhere else turns the sound on or off.
    fn sound_line(&self, event: SoundEvent, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let (name, detail) = event_text(event);
        let on = self.config.sounds.get(event).on;
        let current = self.event_sound(event);
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
                    .child(sound_name(&current)),
            )
            .child(icon("chevron-down", th.text_dim, 16.0));
        let play = icon_button(("sound-play", ix), "play", 16.0, th)
            .size(px(32.0))
            .tooltip(tip(tr!("sounds-play"), th))
            .on_click(cx.listener(move |_, _, _, cx| {
                cx.stop_propagation();
                sound::play(&current);
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

    /// The menu of sounds for `event`: the set's own first, then each
    /// set's sounds under its name, then a file of the user's own. Each
    /// plays as it is picked, the chosen one ticked.
    pub(super) fn sound_menu_rows(
        &self,
        event: SoundEvent,
        rh: f32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Rows {
        let chosen = self.config.sounds.get(event).sound.clone();
        let current = self.event_sound(event);
        // A sound of the event's own, rather than its set's.
        let own = !chosen.is_empty() && current == chosen;
        let set = sound::set(&self.config.sounds.set);
        let tick = |on: bool| {
            if on {
                icon("check", th.accent, 20.0)
            } else {
                div().size(px(20.0)).into_any_element()
            }
        };
        let mut rows = Rows::new(rh);
        let set_sound = sound::usual(event, set.id);
        rows.item(
            menu_row_with(
                "sound-choice-set",
                tick(!own),
                tr!("sounds-set-sound").into(),
                th,
                rh,
            )
            .on_click(cx.listener(move |this, _, _, cx| {
                this.close_context_menu(cx);
                sound::play(set_sound);
                this.apply(Change::SoundChoice(event, ""), cx);
            })),
        );
        let mut n: usize = 0;
        for set in sound::SETS {
            rows.rule(th);
            let (name, _, _) = set_text(set.id);
            rows.line(
                div()
                    .h(px(24.0))
                    .pl(px(16.0))
                    .flex()
                    .items_center()
                    .text_size(px(12.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgba(th.text_faint))
                    .child(name)
                    .into_any_element(),
                24.0,
                false,
            );
            let ids: Vec<&'static str> = if set.id == sound::SYSTEM {
                sound::system_choices().iter().map(|c| c.id).collect()
            } else {
                set.sounds.to_vec()
            };
            for id in ids {
                n += 1;
                rows.item(
                    menu_row_with(
                        ("sound-choice", n),
                        tick(own && id == current),
                        sound_name(id).into(),
                        th,
                        rh,
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.close_context_menu(cx);
                        sound::play(id);
                        this.apply(Change::SoundChoice(event, id), cx);
                    })),
                );
            }
        }
        rows.rule(th);
        if own && sound::file_of(&current).is_some() {
            rows.item(menu_row_with(
                "sound-choice-file",
                tick(true),
                sound_name(&current).into(),
                th,
                rh,
            ));
        }
        rows.item(
            menu_row(
                "sound-choose-file",
                "folder",
                tr!("sounds-choose-file").into(),
                th,
                rh,
            )
            .on_click(cx.listener(move |this, _, _, cx| {
                this.close_context_menu(cx);
                this.pick_sound_file(event, cx);
            })),
        );
        rows
    }

    /// Asks for a sound file for `event`, keeps a copy of it in Katna's
    /// data folder (the original may move) and names it as the event's
    /// sound, playing it.
    fn pick_sound_file(&mut self, event: SoundEvent, cx: &mut Context<Self>) {
        let chosen = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some(tr!("sounds-choose-file-title").into()),
        });
        let dir = self.sound_files_dir();
        cx.spawn(async move |this, cx| {
            let Ok(Ok(Some(paths))) = chosen.await else {
                return;
            };
            let Some(path) = paths.into_iter().next() else {
                return;
            };
            let copied = cx
                .background_executor()
                .spawn(async move { copy_sound(&path, &dir, event) })
                .await;
            this.update(cx, |this, cx| match copied {
                Ok(copy) => {
                    let id = sound::file_sound(&copy);
                    sound::play(&id);
                    this.set_event_sound(event, id);
                    this.save_config();
                    this.send(crate::daemon::Command::ReloadConfig, None, None, true, cx);
                    cx.notify();
                }
                Err(text) => this.show_snackbar(text, None, cx),
            })
            .ok();
        })
        .detach();
    }
}

/// Copies the sound file `path` into `dir` for `event`, under a new name
/// so a sound playing from the old copy is not cut off. The copy's path,
/// or what to tell the user.
fn copy_sound(path: &Path, dir: &Path, event: SoundEvent) -> Result<PathBuf, String> {
    let kind = path
        .extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_default();
    if !FILE_KINDS.contains(&kind.as_str()) {
        return Err(tr!("sounds-file-kind"));
    }
    let size =
        std::fs::metadata(path).map_err(|e| tr!("sounds-file-failed", error = e.to_string()))?;
    if size.len() > MOST_FILE_BYTES {
        return Err(tr!("sounds-file-too-big"));
    }
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    let stem = path
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    // The name shows in Settings: the file's own, after the event and a
    // stamp that keep it apart.
    let copy = dir
        .join(format!("{}-{stamp}", file_key(event)))
        .join(format!("{stem}.{kind}"));
    std::fs::create_dir_all(copy.parent().unwrap_or(dir))
        .and_then(|()| std::fs::copy(path, &copy))
        .map_err(|e| tr!("sounds-file-failed", error = e.to_string()))?;
    Ok(copy)
}
