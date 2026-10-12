// SPDX-License-Identifier: GPL-3.0-or-later

//! Reminders on notes, as Keep has them: the bell sets a time from the
//! same menu and picker as snoozing mail, the card shows it as a chip,
//! and the daemon shows a notification with Open and Snooze at that time
//! (`katna-daemon`'s alarms), even with the app closed.

use crate::widgets::Tip as _;
use gpui::{AnyElement, Context, ElementId, div, prelude::*, rgba};
use jiff::Timestamp;
use jiff::tz::TimeZone;
use katna_i18n::{format, tr};

use super::{MailWindow, item_of};
use crate::daemon::Command;
use crate::theme::Theme;

/// When a reminder is, as its chip says it: "Today, 18:00",
/// "Tomorrow, 09:00", "Mon, 10:30", "27 Sept, 08:00".
pub(super) fn remind_label(at: i64, now: i64, tz: &TimeZone) -> String {
    let (Some(at), Some(now)) = (crate::format::local(at, tz), crate::format::local(now, tz))
    else {
        return String::new();
    };
    let time = format::time(at);
    match (at.date() - now.date()).get_days() {
        0 => tr!("notes-remind-today", time = time),
        1 => tr!("notes-remind-tomorrow", time = time),
        2..=6 => tr!(
            "notes-remind-weekday",
            day = format::weekday(at),
            time = time
        ),
        _ => format::day_month_time(at),
    }
}

impl MailWindow {
    /// Sets notes `ids` to remind at `at`, or not at all, with Undo.
    pub(in crate::window) fn remind_notes(
        &mut self,
        ids: Vec<i64>,
        at: Option<i64>,
        cx: &mut Context<Self>,
    ) {
        let notes: Vec<katna_store::Note> = self
            .notes
            .as_ref()
            .and_then(|p| p.notes.as_ref())
            .and_then(|n| n.as_ref().ok())
            .map(|notes| {
                notes
                    .iter()
                    .filter(|n| ids.contains(&n.id))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();
        // The open note, new or changed since the board read it.
        let open = self
            .notes
            .as_mut()
            .and_then(|p| p.editor.as_mut())
            .filter(|e| e.id == 0 || ids.contains(&e.id));
        if let Some(editor) = open {
            let was = editor.remind_at;
            editor.remind_at = at;
            let id = editor.id;
            self.note_typed(cx);
            if id == 0 || ids.len() == 1 {
                self.said_reminder(at, was.is_some(), None, cx);
                return;
            }
        }
        let open_id = self
            .notes
            .as_ref()
            .and_then(|p| p.editor.as_ref())
            .map(|e| e.id);
        let mut done = Vec::new();
        let mut undo = Vec::new();
        for note in notes.iter().filter(|n| Some(n.id) != open_id) {
            let mut item = item_of(note);
            undo.push(Command::SaveNote(Box::new(item.clone())));
            item.remind_at = at.unwrap_or(0);
            done.push(item);
        }
        let had = notes.iter().any(|n| n.remind_at.is_some());
        for item in done {
            self.change_note(item, cx);
        }
        let undo = (!undo.is_empty()).then_some(Command::Several(undo));
        self.said_reminder(at, had, undo, cx);
    }

    /// Says in the snackbar what the reminder now is, with `undo`.
    fn said_reminder(
        &mut self,
        at: Option<i64>,
        had: bool,
        undo: Option<Command>,
        cx: &mut Context<Self>,
    ) {
        let text = match at {
            Some(at) => tr!(
                "notes-reminder-set",
                when = super::super::snooze::describe(at, &self.tz)
            ),
            None if had => tr!("notes-reminder-off"),
            None => return,
        };
        self.show_snackbar(text, undo, cx);
    }

    /// A reminder's chip, on a card or the open note; a past one is struck
    /// through, as Keep shows it. Clicking it opens the reminder menu.
    pub(super) fn reminder_chip(
        &self,
        id: impl Into<ElementId>,
        note: i64,
        at: i64,
        tint: u32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let now = Timestamp::now().as_second();
        let past = at <= now;
        let label = remind_label(at, now, &self.tz);
        crate::widgets::icon_tag(
            "bell",
            div().when(past, |d| d.line_through()).child(label),
            th,
        )
        .id(id)
        .bg(rgba(tint))
        .cursor_pointer()
        .hover(|s| s.bg(rgba(super::chip_hover(tint, th))))
        .tip(super::super::snooze::describe(at, &self.tz), th)
        .on_click(cx.listener(move |this, event: &gpui::ClickEvent, _, cx| {
            cx.stop_propagation();
            this.open_remind_menu(vec![note], true, event.position(), cx);
        }))
        .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_reminder_says_how_soon() {
        let tz = TimeZone::UTC;
        // Saturday, 3 October 2026, 12:00 UTC.
        let now = 1_791_028_800;
        let time = |at: i64| format::time(crate::format::local(at, &tz).unwrap());
        let (evening, morning) = (now + 6 * 3600, now + 21 * 3600);
        assert_eq!(
            remind_label(evening, now, &tz),
            format!("Today, {}", time(evening))
        );
        assert_eq!(
            remind_label(morning, now, &tz),
            format!("Tomorrow, {}", time(morning))
        );
        assert!(remind_label(now + 2 * 86_400, now, &tz).starts_with("Mon"));
    }
}
