// SPDX-License-Identifier: GPL-3.0-or-later

//! Remind me on any mail: the mail stays where it is, and a task in Tasks,
//! made from it, notifies at the time (`katna-daemon`'s task alarms). It
//! shares the snooze menu (B snoozes, H reminds), shows as a chip on the
//! mail's line, lists under Reminders in the folder pane, and as a small
//! line at the end of a chat.

use gpui::{AnyElement, ClickEvent, Context, FontWeight, Pixels, Point, Window, div, prelude::*};
use gpui::{relative, rgba};
use jiff::civil::{Date, Time};
use jiff::{Timestamp, Zoned};
use katna_core::config::AppKind;
use katna_core::quick_add;
use katna_i18n::tr;
use katna_store::MessageId;
use katna_ui::px;
use katna_ui::tokens::{radius, space};

use super::compose::schedule;
use super::{Listing, MailWindow};
use crate::data::{Entry, EntryKey};
use crate::tasks::{TaskCommand, TaskEdit};
use crate::theme::Theme;
use crate::widgets::icon_tag;

/// The folder pane line of Reminders.
pub(super) const NAV_KEY: &str = "katna:reminders";

/// When to remind about something due on `due`: the morning before it,
/// else that morning, if still to come after `now`.
pub(super) fn remind_before(due: Date, now: &Zoned) -> Option<Zoned> {
    let before = due.yesterday().ok();
    [before, Some(due)].into_iter().flatten().find_map(|day| {
        let at = day
            .to_datetime(Time::constant(8, 0, 0, 0))
            .to_zoned(now.time_zone().clone())
            .ok()?;
        (at.timestamp() > now.timestamp()).then_some(at)
    })
}

impl MailWindow {
    /// Snooze (B): the menu of times for the lines the keys act on.
    pub(super) fn snooze_key(
        &mut self,
        _: &super::SnoozeMail,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let keys = self.target_keys();
        let at = self.times_point(window);
        self.open_mail_times(keys, false, at, cx);
    }

    /// Remind me (H): the same menu, with Remind me picked.
    pub(super) fn remind_key(
        &mut self,
        _: &super::RemindMail,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let keys = self.target_keys();
        let at = self.times_point(window);
        self.open_mail_times(keys, true, at, cx);
    }

    /// Where the menu of times opens from a key: a third of the way down
    /// the window, over the middle.
    fn times_point(&self, window: &Window) -> Point<Pixels> {
        let size = window.viewport_size();
        gpui::point(size.width * 0.5 - px(150.0), size.height * 0.2)
    }

    /// The time before the day the mail of `keys` (one line) says
    /// something is due by: its subject, and the open mail's text.
    pub(super) fn before_due(&self, keys: &[EntryKey]) -> Option<Zoned> {
        let [key] = keys else {
            return None;
        };
        let mail = self.mail.as_ref().ok()?;
        let (mut text, _) = mail.task_source(*key)?;
        if let Some(reader) = self.reader.as_ref().filter(|r| r.key == *key)
            && let Some(newest) = reader.newest()
            && let Some(body) = self.plain_text_of(newest)
        {
            text.push_str(".\n");
            text.push_str(&body);
        }
        let now = Timestamp::now().to_zoned(self.tz.clone());
        let language = katna_i18n::current().language.tag.clone();
        let words = quick_add::Words::for_language(&language);
        let due = quick_add::due_in(&text, now.date(), words)?;
        remind_before(due, &now)
    }

    /// Reminds about the mail of `keys` at `at`: a task made from each,
    /// titled with `note` (else the subject), due then and notifying then.
    /// A conversation with a reminder already has it moved instead.
    pub(super) fn remind_mails(
        &mut self,
        keys: Vec<EntryKey>,
        at: i64,
        note: String,
        cx: &mut Context<Self>,
    ) {
        if !self.needs_app(AppKind::Tasks, cx) {
            return;
        }
        let Ok(when) = Timestamp::from_second(at) else {
            return;
        };
        let Ok(mail) = self.mail.as_ref() else {
            return;
        };
        let zoned = when.to_zoned(self.tz.clone());
        let due = zoned.date().to_string();
        let minutes = u32::try_from(i32::from(zoned.hour()) * 60 + i32::from(zoned.minute()))
            .unwrap_or_default();
        let fields = TaskEdit {
            due: Some(due.clone()),
            due_time: Some(Some(minutes)),
            remind_at: Some(Some(at)),
            ..TaskEdit::default()
        };
        let reminders = self.tasks.mail_reminders();
        let mut commands = Vec::new();
        let mut undo = Vec::new();
        for key in keys {
            let Some((subject, header)) = mail.task_source(key) else {
                continue;
            };
            let messages = mail.entry_messages(key);
            let had = reminders.iter().find(|t| {
                mail.message_with_header(&t.mail)
                    .is_some_and(|m| messages.contains(&m))
            });
            match had {
                Some(task) => {
                    let mut edit = fields.clone();
                    if !note.is_empty() {
                        edit.title = Some(note.clone());
                    }
                    undo.push(TaskCommand::Edit(task.id, TaskEdit::all_of(task)));
                    commands.push(TaskCommand::Edit(task.id, edit));
                }
                None => {
                    let title = if !note.is_empty() {
                        note.clone()
                    } else if subject.is_empty() {
                        tr!("tasks-no-subject")
                    } else {
                        subject
                    };
                    let add = TaskCommand::Add {
                        list: 0,
                        parent: None,
                        title,
                        due: due.clone(),
                        mail: header.clone(),
                    };
                    commands.push(TaskCommand::AddThen(Box::new(add), fields.clone()));
                    undo.push(TaskCommand::RemoveFromMail(vec![header]));
                }
            }
        }
        let done = tr!(
            "toast-remind-set",
            date = super::snooze::describe(at, &self.tz)
        );
        self.send_tasks(commands, Some(done), undo, cx);
    }

    /// The messages a reminder waits on, soonest first.
    fn reminder_messages(&self) -> Vec<MessageId> {
        let Ok(mail) = &self.mail else {
            return Vec::new();
        };
        let mut ids: Vec<MessageId> = Vec::new();
        for task in self.tasks.mail_reminders() {
            if let Some(id) = mail.message_with_header(&task.mail)
                && !ids.contains(&id)
            {
                ids.push(id);
            }
        }
        ids
    }

    /// How many mails have a reminder, for the folder pane.
    pub(super) fn reminder_count(&self) -> usize {
        self.reminder_messages().len()
    }

    /// The lines of Reminders: mail with a reminder, soonest first.
    pub(super) fn reminder_entries(&self) -> Vec<Entry> {
        let ids = self.reminder_messages();
        match &self.mail {
            Ok(mail) => mail.hit_entries(&ids, self.config.mail.conversations, None),
            Err(_) => Vec::new(),
        }
    }

    /// Opens Reminders, as `open_folder` opens a folder.
    pub(super) fn open_reminders(&mut self, cx: &mut Context<Self>) {
        // Mail the user got: its lines show who it came from.
        if self.show_recipients {
            self.show_recipients = false;
            if let Ok(mail) = &mut self.mail {
                mail.clear_rows();
            }
        }
        self.entries = self.reminder_entries();
        self.tabs = Vec::new();
        self.tab = 0;
        self.folder = None;
        self.unified = None;
        self.listing = Some(Listing::Reminders);
        self.reset_list(false);
        self.selected = (!self.entries.is_empty()).then_some(0);
        self.checked.clear();
        self.check_anchor = None;
        self.checked_all = false;
        self.page_pick = None;
        self.picked = None;
        self.menu = None;
        self.show_list();
        cx.notify();
    }

    /// The reminder on the open conversation, in a chat: a small line at
    /// its end, like the day labels, with Edit and Done.
    pub(super) fn render_chat_reminder(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let reader = self.reader.as_ref()?;
        let mail = self.mail.as_ref().ok()?;
        let ids = reader.message_ids();
        let task = self.tasks.mail_reminders().into_iter().find(|t| {
            mail.message_with_header(&t.mail)
                .is_some_and(|m| ids.contains(&m))
        })?;
        let (id, key) = (task.id, reader.key);
        let when = Timestamp::from_second(task.remind_at?)
            .map(|at| schedule::short(&at.to_zoned(self.tz.clone())))
            .unwrap_or_default();
        let link = |id: &'static str, label: String| {
            div()
                .id(id)
                .flex_none()
                .px(px(space::S1))
                .rounded(px(radius::XS))
                .text_color(rgba(th.accent))
                .font_weight(FontWeight::MEDIUM)
                .cursor_pointer()
                .hover(|s| s.underline())
                .child(label)
        };
        Some(
            icon_tag(
                "bell",
                div().min_w_0().truncate().child(tr!(
                    "remind-chat-line",
                    date = when,
                    title = task.title.clone()
                )),
                th,
            )
            .flex_shrink(1.0)
            .min_w_0()
            .self_center()
            .max_w(relative(0.9))
            .my(px(space::S3))
            .child(
                link("remind-edit", tr!("follow-up-card-edit")).on_click(cx.listener(
                    move |this, event: &ClickEvent, _, cx| {
                        this.open_mail_times(vec![key], true, event.position(), cx);
                    },
                )),
            )
            .child(
                link("remind-done", tr!("remind-done")).on_click(cx.listener(
                    move |this, _, _, cx| {
                        this.send_tasks(
                            vec![TaskCommand::SetDone(id, true)],
                            Some(tr!("toast-remind-done")),
                            vec![TaskCommand::SetDone(id, false)],
                            cx,
                        );
                    },
                )),
            )
            .into_any_element(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::date;
    use jiff::tz::TimeZone;

    #[test]
    fn reminds_the_morning_before_something_is_due() {
        let tz = TimeZone::get("Asia/Kolkata").unwrap();
        let at = |d: Date, h: i8| d.at(h, 0, 0, 0).to_zoned(tz.clone()).unwrap();
        // Tuesday afternoon; due Friday: Thursday 8:00.
        let now = at(date(2026, 9, 29), 15);
        assert_eq!(
            remind_before(date(2026, 10, 2), &now),
            Some(at(date(2026, 10, 1), 8))
        );
        // Due tomorrow: this morning has gone, so tomorrow morning.
        assert_eq!(
            remind_before(date(2026, 9, 30), &now),
            Some(at(date(2026, 9, 30), 8))
        );
        // Due today, after 8:00: nothing left to offer.
        assert_eq!(remind_before(date(2026, 9, 29), &now), None);
    }
}
