// SPDX-License-Identifier: GPL-3.0-or-later

//! Meeting notes, as Google Calendar's "Take meeting notes": a note about
//! an event, opened from its card on the Calendar page, titled with the
//! event and its day and started with who comes. The note keeps
//! `event:<start>:<uid>` as its link, so the event's card lists it and
//! its Event chip opens the Calendar on that day.

use gpui::{AnyElement, Context, FontWeight, Window, div, prelude::*, rgba};
use katna_dav::Occurrence;
use katna_i18n::{format, tr};
use katna_ui::px;

use super::{MailWindow, UNTICKED};
use crate::theme::Theme;
use crate::widgets::icon;

/// What starts a meeting note's link.
const EVENT: &str = "event:";

/// The link of a note about `occurrence`: its start, so each occurrence
/// of a repeating event has its own notes, and its event's UID.
pub(super) fn event_link(occurrence: &Occurrence) -> String {
    format!("{EVENT}{}:{}", occurrence.start, occurrence.event.data.uid)
}

/// When the event a note link is about starts, if it is about one.
pub(super) fn event_start(link: &str) -> Option<i64> {
    link.strip_prefix(EVENT)?.split_once(':')?.0.parse().ok()
}

impl MailWindow {
    /// Take meeting notes (an event's card): a new note about the event,
    /// or its first one when it has notes already.
    fn take_meeting_notes(
        &mut self,
        occurrence: &Occurrence,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.notes_page(cx);
        let data = &occurrence.event.data;
        let day = super::super::calendar::civil(occurrence.start, &self.tz);
        let title = tr!(
            "notes-meeting-title",
            title = if data.title.is_empty() {
                tr!("calendar-no-title")
            } else {
                data.title.clone()
            },
            date = format::day_month(day)
        );
        let names: Vec<String> = data
            .attendees
            .iter()
            .map(|a| {
                if a.name.is_empty() {
                    a.email.clone()
                } else {
                    a.name.clone()
                }
            })
            .collect();
        // Google's template: who comes, then notes, then action items.
        let mut body = String::new();
        if !names.is_empty() {
            body.push_str(&tr!("notes-meeting-attendees", names = names.join(", ")));
            body.push_str("\n\n");
        }
        body.push_str(&tr!("notes-meeting-notes"));
        body.push_str("\n\n\n");
        body.push_str(&tr!("notes-meeting-actions"));
        body.push('\n');
        body.push_str(UNTICKED);
        self.open_note(
            None,
            false,
            Some((title, event_link(occurrence))),
            window,
            cx,
        );
        if let Some(editor) = self.notes.as_ref().and_then(|p| p.editor.as_ref()) {
            // Typed text goes under "Notes"; left as it is, the note is
            // not kept.
            let doc = super::format::doc_of(&body, "");
            let line = body[..body.find("\n\n\n").map_or(body.len(), |ix| ix + 1)]
                .matches('\n')
                .count();
            let at = doc
                .paths()
                .get(line)
                .map_or(doc.end(), |path| katna_ui::rich::Pos::new(*path, 0));
            editor.body.update(cx, |area, cx| area.set_doc(doc, at, cx));
        }
        if let Some(editor) = self.notes.as_mut().and_then(|p| p.editor.as_mut()) {
            editor.changed = false;
        }
        cx.notify();
    }

    /// Opens the Calendar on the day of the event a note is about.
    pub(super) fn open_note_event(&mut self, start: i64, cx: &mut Context<Self>) {
        self.close_note(cx);
        let day = super::super::calendar::civil(start, &self.tz).date();
        self.open_app(super::super::apps::App::Calendar, cx);
        self.open_calendar_on(day, cx);
    }

    /// An event card's meeting notes: the notes about it, and Take meeting
    /// notes.
    pub(in crate::window) fn render_event_notes(
        &self,
        occurrence: &Occurrence,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let link = event_link(occurrence);
        let tiles: Vec<AnyElement> = self
            .notes
            .as_ref()
            .and_then(|p| p.notes.as_ref())
            .and_then(|n| n.as_ref().ok())
            .map(|notes| {
                notes
                    .iter()
                    .filter(|n| n.trashed_at.is_none() && n.link.as_deref() == Some(&link))
                    .map(|n| self.linked_note(n, th, cx).into_any_element())
                    .collect()
            })
            .unwrap_or_default();
        let occurrence = occurrence.clone();
        div()
            .flex()
            .flex_row()
            .items_start()
            .gap(px(16.0))
            .child(
                div()
                    .flex_none()
                    .pt(px(8.0))
                    .child(icon("notes", th.text_dim, 20.0)),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .children(tiles)
                    .child(
                        div().flex().flex_row().child(
                            div()
                                .id("event-take-notes")
                                .h(px(36.0))
                                .px(px(12.0))
                                .ml(px(-12.0))
                                .flex()
                                .items_center()
                                .rounded(px(8.0))
                                .text_size(px(14.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgba(th.accent))
                                .cursor_pointer()
                                .hover(|s| s.bg(rgba(th.hover)))
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    this.take_meeting_notes(&occurrence, window, cx)
                                }))
                                .child(tr!("notes-meeting-take")),
                        ),
                    ),
            )
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_event_link_gives_back_its_start() {
        assert_eq!(
            event_start("event:1759132800:abc:def@x"),
            Some(1_759_132_800)
        );
        assert_eq!(event_start("<id@example.org>"), None);
        assert_eq!(event_start("event:soon:abc"), None);
    }
}
