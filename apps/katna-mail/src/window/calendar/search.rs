// SPDX-License-Identifier: GPL-3.0-or-later

//! The top bar's search box on the Calendar page, as in Google Calendar:
//! it finds events by title, place, notes and guests (every word), and
//! lists them by date, coming ones first, then past ones newest first. A
//! click opens the event on its day with its card; Esc goes back to the
//! results (`docs/ARCHITECTURE.md` §18).

use std::collections::HashMap;
use std::rc::Rc;

use gpui::{
    AnyElement, ClickEvent, Context, Entity, FontWeight, Task, Window, div, prelude::*, rgba,
};
use jiff::Zoned;
use jiff::tz::TimeZone;
use katna_core::Paths;
use katna_dav::Occurrence;
use katna_i18n::tr;
use katna_store::calendar::{EventData, EventStatus, StoredEvent};
use katna_store::{Mode, Store};
use katna_ui::px;
use katna_ui::text_input::{InputEvent, TextInput};

use super::super::MailWindow;
use super::{CalView, civil, schedule_day};
use crate::theme::Theme;

/// How far back and ahead the search looks, in days.
const REACH_DAYS: i64 = 2 * 366;

/// The search on the Calendar page.
#[derive(Default)]
pub(in crate::window) struct Search {
    /// What the box holds while the page shows.
    pub(in crate::window) query: String,
    /// The mail search's words, kept while the box searches events.
    mail_query: Option<String>,
    /// The results for the words they were read for.
    found: Option<(String, Rc<Found>)>,
    /// The results are put away while an event opened from them shows on
    /// its day, or the page is moved; Esc brings them back.
    pub(in crate::window) away: bool,
    task: Option<Task<()>>,
}

/// Events found: coming ones soonest first, past ones newest first. A
/// repeating event shows at most once in each.
#[derive(Default, Debug)]
pub(in crate::window) struct Found {
    coming: Vec<Occurrence>,
    past: Vec<Occurrence>,
}

impl Search {
    /// Whether the results take the main view's place.
    pub(in crate::window) fn showing(&self) -> bool {
        !self.query.trim().is_empty() && !self.away
    }
}

/// The words of `query`, lowercase.
fn words(query: &str) -> Vec<String> {
    query.split_whitespace().map(str::to_lowercase).collect()
}

/// Whether the event's title, place, notes, organizer or guests hold every
/// word.
fn matches(data: &EventData, words: &[String]) -> bool {
    let mut text = format!(
        "{}\n{}\n{}\n{}\n{}",
        data.title, data.location, data.description, data.organizer, data.organizer_name
    );
    for guest in &data.attendees {
        text.push('\n');
        text.push_str(&guest.name);
        text.push('\n');
        text.push_str(&guest.email);
    }
    let text = text.to_lowercase();
    words.iter().all(|w| text.contains(w.as_str()))
}

/// The events of `rows` matching `words` in `from..to`, split at `now`.
fn find(
    rows: Vec<StoredEvent>,
    words: &[String],
    now: i64,
    from: i64,
    to: i64,
    tz: &TimeZone,
) -> Found {
    // Only series with a match, with their changed occurrences.
    let uids: std::collections::HashSet<String> = rows
        .iter()
        .filter(|row| matches(&row.data, words))
        .map(|row| row.data.uid.clone())
        .collect();
    if uids.is_empty() {
        return Found::default();
    }
    let rows = rows
        .into_iter()
        .filter(|row| uids.contains(&row.data.uid))
        .collect();
    let mut coming: HashMap<String, Occurrence> = HashMap::new();
    let mut past: HashMap<String, Occurrence> = HashMap::new();
    for occurrence in katna_dav::occurrences(rows, from, to, tz) {
        let data = &occurrence.event.data;
        if data.status == EventStatus::Cancelled || !matches(data, words) {
            continue;
        }
        let uid = data.uid.clone();
        if occurrence.end > now {
            coming
                .entry(uid)
                .and_modify(|o| {
                    if occurrence.start < o.start {
                        *o = occurrence.clone();
                    }
                })
                .or_insert(occurrence);
        } else {
            past.entry(uid)
                .and_modify(|o| {
                    if occurrence.start > o.start {
                        *o = occurrence.clone();
                    }
                })
                .or_insert(occurrence);
        }
    }
    let mut coming: Vec<Occurrence> = coming.into_values().collect();
    coming.sort_by_key(|o| (o.start, o.event.id));
    let mut past: Vec<Occurrence> = past.into_values().collect();
    past.sort_by_key(|o| (std::cmp::Reverse(o.start), o.event.id));
    Found { coming, past }
}

fn read(paths: &Paths, query: &str, tz: &TimeZone) -> Result<Found, String> {
    let now = jiff::Timestamp::now().as_second();
    let reach = REACH_DAYS * 86_400;
    let (from, to) = (now - reach, now + reach);
    let store = Store::open(paths, Mode::ReadOnly).map_err(|err| err.to_string())?;
    let rows = store
        .event_rows_in_range(from, to)
        .map_err(|err| err.to_string())?;
    Ok(find(rows, &words(query), now, from, to, tz))
}

impl MailWindow {
    /// Turns the top bar's search box to events while the page shows, and
    /// back to mail after, each keeping its own words.
    pub(in crate::window) fn swap_calendar_search(
        &mut self,
        entering: bool,
        cx: &mut Context<Self>,
    ) {
        let (placeholder, text) = if entering {
            let search = &mut self.calendar.search;
            search.mail_query = Some(self.search.read(cx).text().to_owned());
            search.away = false;
            (tr!("calendar-search"), search.query.clone())
        } else {
            let text = self.calendar.search.mail_query.take().unwrap_or_default();
            (tr!("search-mail"), text)
        };
        self.search.update(cx, |search, cx| {
            search.set_placeholder(placeholder);
            search.set_text(text, cx);
        });
    }

    /// The top bar's search box changed while the page shows.
    pub(in crate::window) fn on_calendar_search(
        &mut self,
        search: &Entity<TextInput>,
        event: &InputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            InputEvent::Changed => {
                let query = search.read(cx).text().to_owned();
                self.calendar.search.query = query.clone();
                self.calendar.search.away = false;
                self.calendar.open = None;
                self.find_events(query, cx);
            }
            InputEvent::Cancel => {
                if self.calendar.search.query.is_empty() {
                    window.focus(&self.calendar.focus, cx);
                } else {
                    self.calendar.search.query.clear();
                    self.calendar.search.found = None;
                    search.update(cx, |search, cx| search.set_text("", cx));
                }
            }
            // Enter opens the first event found.
            InputEvent::Submit => {
                let first =
                    self.calendar.search.found.as_ref().and_then(|(_, found)| {
                        found.coming.first().or(found.past.first()).cloned()
                    });
                if let Some(occurrence) = first {
                    let size = window.viewport_size();
                    let at = gpui::point(size.width / 2.0, size.height / 3.0);
                    self.open_found(occurrence, at, cx);
                    window.focus(&self.calendar.focus, cx);
                }
            }
        }
        cx.notify();
    }

    fn find_events(&mut self, query: String, cx: &mut Context<Self>) {
        if query.trim().is_empty() {
            self.calendar.search.task = None;
            self.calendar.search.found = None;
            return;
        }
        let paths = self.paths.clone();
        let tz = self.tz.clone();
        self.calendar.search.task = Some(cx.spawn(async move |this, cx| {
            let found = cx
                .background_executor()
                .spawn({
                    let query = query.clone();
                    async move { read(&paths, &query, &tz) }
                })
                .await;
            this.update(cx, |this, cx| {
                match found {
                    Ok(found) => this.calendar.search.found = Some((query, Rc::new(found))),
                    Err(err) => tracing::warn!(%err, "searching the calendar failed"),
                }
                cx.notify();
            })
            .ok();
        }));
    }

    /// Shows a found event on its day with its card, the results put away.
    fn open_found(
        &mut self,
        occurrence: Occurrence,
        at: gpui::Point<gpui::Pixels>,
        cx: &mut Context<Self>,
    ) {
        let day = civil(occurrence.start, &self.tz).date();
        // Year and Schedule show no single day: the day opens in Day.
        let view =
            matches!(self.calendar.view, CalView::Year | CalView::Schedule).then_some(CalView::Day);
        self.open_calendar_day(day, view, cx);
        self.calendar.search.away = true;
        self.open_calendar_event(occurrence, at, cx);
    }

    /// The events found, by date, in the main view's place.
    pub(in crate::window) fn render_calendar_search(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let search = &self.calendar.search;
        let Some((_, found)) = &search.found else {
            return super::loading(th);
        };
        let hidden = |o: &&Occurrence| self.calendar.hidden.contains(&o.event.calendar_id);
        let coming: Vec<&Occurrence> = found.coming.iter().filter(|o| !hidden(o)).collect();
        let past: Vec<&Occurrence> = found.past.iter().filter(|o| !hidden(o)).collect();
        if coming.is_empty() && past.is_empty() {
            return crate::widgets::placeholder(&tr!("calendar-search-none"), th);
        }
        let today = Zoned::now().with_time_zone(self.tz.clone()).date();
        let mut rows: Vec<AnyElement> = self.found_days(&coming, today, th, cx);
        if !past.is_empty() {
            rows.push(
                div()
                    .pt(px(20.0))
                    .pb(px(4.0))
                    .pl(px(16.0))
                    .text_size(px(13.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgba(th.text_dim))
                    .child(tr!("calendar-search-past"))
                    .into_any_element(),
            );
            rows.extend(self.found_days(&past, today, th, cx));
        }
        div()
            .id("calendar-search")
            .size_full()
            .overflow_y_scroll()
            .children(rows)
            .into_any_element()
    }

    /// `list`'s events under their days, in the list's order.
    fn found_days(
        &self,
        list: &[&Occurrence],
        today: jiff::civil::Date,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        let mut days: Vec<(jiff::civil::Date, Vec<Occurrence>)> = Vec::new();
        for occurrence in list {
            let day = civil(occurrence.start, &self.tz).date();
            match days.last_mut() {
                Some((d, events)) if *d == day => events.push((*occurrence).clone()),
                _ => days.push((day, vec![(*occurrence).clone()])),
            }
        }
        days.into_iter()
            .map(|(day, events)| {
                let events = events.into_iter().map(|occurrence| {
                    let open = occurrence.clone();
                    self.schedule_event(&format!("found-{day}"), occurrence, th)
                        .on_click(cx.listener(move |this, event: &ClickEvent, _, cx| {
                            cx.stop_propagation();
                            this.open_found(open.clone(), event.position(), cx);
                        }))
                });
                schedule_day(day, day == today, day.year() != today.year(), th)
                    .child(div().flex_1().min_w_0().flex().flex_col().children(events))
                    .into_any_element()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use katna_store::calendar::{Attendee, EventKind};

    fn row(id: i64, uid: &str, title: &str, start: i64, rrule: &str) -> StoredEvent {
        StoredEvent {
            id,
            calendar_id: 1,
            data: EventData {
                remote_id: uid.into(),
                uid: uid.into(),
                etag: None,
                recurrence_id: None,
                status: EventStatus::Confirmed,
                title: title.into(),
                location: String::new(),
                description: String::new(),
                start,
                end: start + 3600,
                all_day: false,
                time_zone: String::new(),
                rrule: rrule.into(),
                exdates: Vec::new(),
                rdates: Vec::new(),
                range_end: None,
                busy: true,
                kind: EventKind::default(),
                color: String::new(),
                organizer: String::new(),
                organizer_name: String::new(),
                attendees: Vec::new(),
                self_status: String::new(),
                join_url: String::new(),
                reminders: Vec::new(),
                web_link: String::new(),
                updated_at: 0,
            },
        }
    }

    #[test]
    fn finds_by_every_word_coming_then_past() {
        let day = 86_400;
        let now = 100 * day;
        let mut guest = row(3, "c", "Lunch", now + 2 * day, "");
        guest.data.attendees.push(Attendee {
            email: "priya@acme.example".into(),
            name: "Priya Nair".into(),
            ..Default::default()
        });
        let rows = vec![
            row(1, "a", "Design review", now + 5 * day, ""),
            row(2, "b", "Weekly design sync", now - 20 * day, "FREQ=WEEKLY"),
            guest,
            row(4, "d", "Budget", now + day, ""),
        ];
        let found = find(
            rows.clone(),
            &words("DESIGN"),
            now,
            0,
            200 * day,
            &TimeZone::UTC,
        );
        let titles = |list: &[Occurrence]| -> Vec<String> {
            list.iter().map(|o| o.event.data.title.clone()).collect()
        };
        // The weekly sync shows once ahead and once behind.
        assert_eq!(
            titles(&found.coming),
            ["Weekly design sync", "Design review"]
        );
        assert_eq!(titles(&found.past), ["Weekly design sync"]);
        assert!(found.past[0].start < now && found.past[0].start > now - 7 * day);
        let found = find(
            rows.clone(),
            &words("priya lunch"),
            now,
            0,
            200 * day,
            &TimeZone::UTC,
        );
        assert_eq!(titles(&found.coming), ["Lunch"]);
        let found = find(
            rows,
            &words("design budget"),
            now,
            0,
            200 * day,
            &TimeZone::UTC,
        );
        assert!(found.coming.is_empty() && found.past.is_empty());
    }
}
