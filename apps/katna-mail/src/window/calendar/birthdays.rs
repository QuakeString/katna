// SPDX-License-Identifier: GPL-3.0-or-later

//! Saved contacts' birthdays on the Calendar (`docs/ARCHITECTURE.md`
//! §8.6), as Google Calendar's Birthdays calendar shows them: a whole-day
//! event every year, in a calendar of their own that can be unticked. They
//! are made from the cards when the calendar is read, never stored, and a
//! click opens the person's contact page. A birthday a mail service's own
//! calendar already has (Google's) shows once.

use std::collections::HashSet;

use gpui::{Context, Window};
use jiff::civil::Date;
use katna_dav::Occurrence;
use katna_i18n::tr;
use katna_store::Store;
use katna_store::calendar::{
    Calendar, CalendarAccess, CalendarSource, EventData, EventKind, StoredEvent,
};

use super::super::MailWindow;
use super::super::apps::App;
use super::super::contacts_page::View;

/// The Birthdays calendar's id: no stored calendar has one below zero.
pub(in crate::window) const BIRTHDAYS: i64 = -1;

/// Google's green for birthdays.
const COLOR: &str = "#0b8043";

/// The day of `birthday` (`YYYY-MM-DD`, or `--MM-DD` without the year) in
/// its year, else in 2000, a leap year so the 29th of February is kept.
fn birth_day(birthday: &str) -> Option<Date> {
    let text = birthday.trim();
    if let Some(rest) = text.strip_prefix("--") {
        return format!("2000-{rest}").parse().ok();
    }
    text.parse().ok()
}

/// The Birthdays calendar, and a yearly whole-day event for each of
/// `cards` (`(id, name, birthday)`), unless `taken` has that person's
/// birthday on that day already: `(month, day, name in lower case)`.
fn birthday_rows(
    cards: Vec<(i64, String, String)>,
    taken: &HashSet<(i8, i8, String)>,
) -> Vec<StoredEvent> {
    let mut seen = HashSet::new();
    cards
        .into_iter()
        .filter_map(|(id, name, birthday)| {
            let day = birth_day(&birthday)?;
            let name = name.trim().to_owned();
            let first = name.split_whitespace().next().unwrap_or_default();
            let key = (day.month(), day.day(), first.to_lowercase());
            // One person across accounts, or a birthday Google has.
            if name.is_empty() || taken.contains(&key) || !seen.insert(key) {
                return None;
            }
            let start = day.to_zoned(jiff::tz::TimeZone::UTC).ok()?.timestamp();
            Some(StoredEvent {
                id,
                calendar_id: BIRTHDAYS,
                data: EventData {
                    remote_id: format!("birthday-{id}"),
                    uid: format!("birthday-{id}"),
                    title: tr!("calendar-birthday-of", name = name.clone()),
                    start: start.as_second(),
                    end: start.as_second() + 86_400,
                    all_day: true,
                    rrule: "FREQ=YEARLY".into(),
                    busy: false,
                    kind: EventKind::Birthday,
                    ..EventData::default()
                },
            })
        })
        .collect()
}

/// The birthdays in `occurrences` already, as `(month, day, first name)`:
/// Google's own birthday events are titled with the person's name.
fn birthdays_shown(
    occurrences: &[Occurrence],
    tz: &jiff::tz::TimeZone,
) -> HashSet<(i8, i8, String)> {
    occurrences
        .iter()
        .filter(|o| o.event.data.kind == EventKind::Birthday)
        .filter_map(|o| {
            let day = jiff::Timestamp::from_second(o.start)
                .ok()?
                .to_zoned(tz.clone())
                .date();
            let first = o.event.data.title.split_whitespace().next()?;
            let first = first
                .trim_end_matches("'s")
                .trim_end_matches('’')
                .to_lowercase();
            Some((day.month(), day.day(), first))
        })
        .collect()
}

/// Adds the Birthdays calendar and, when it is shown, its events in
/// `from..to` to what was read; nothing when no one saved has a birthday,
/// or when Contacts is off (`shown` is `None`).
pub(in crate::window) fn add_birthdays(
    store: &Store,
    shown: Option<bool>,
    from: i64,
    to: i64,
    tz: &jiff::tz::TimeZone,
    calendars: &mut Vec<Calendar>,
    occurrences: &mut Vec<Occurrence>,
) {
    let Some(shown) = shown else {
        return;
    };
    // An older store without contacts has no birthdays.
    let cards = store.contact_birthdays().unwrap_or_default();
    if cards.is_empty() {
        return;
    }
    calendars.push(Calendar {
        id: BIRTHDAYS,
        account: None,
        source: CalendarSource::Local,
        remote_id: String::new(),
        name: tr!("calendar-birthdays"),
        color: COLOR.into(),
        access: CalendarAccess::Reader,
        is_primary: false,
        hidden: !shown,
        time_zone: String::new(),
        sync_token: None,
        position: i64::MAX,
    });
    if !shown {
        return;
    }
    let rows = birthday_rows(cards, &birthdays_shown(occurrences, tz));
    occurrences.extend(katna_dav::occurrences(rows, from, to, tz));
    // Whole days first on a day, as `katna_dav` sorts them.
    occurrences.sort_by_key(|o| (o.start, !o.all_day()));
}

impl MailWindow {
    /// Shows or hides the Birthdays calendar, remembered in the settings.
    pub(in crate::window) fn toggle_birthdays(&mut self, cx: &mut Context<Self>) {
        let contacts = &mut self.config.contacts;
        contacts.hide_birthdays = !contacts.hide_birthdays;
        self.save_config();
        self.load_calendar(cx);
        cx.notify();
    }

    /// Opens the contact page of the person whose birthday `card` is.
    pub(super) fn open_birthday_contact(
        &mut self,
        card: i64,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.calendar.open = None;
        self.show_page(App::Contacts, window, cx);
        self.set_contacts_view(View::Contacts, cx);
        let person = match &self.contacts.book {
            Some(Ok(book)) => book.people.iter().find(|p| p.ids.contains(&card)).cloned(),
            _ => None,
        };
        match person {
            Some(person) => self.open_saved_contact(person, cx),
            // Opens once the contacts are read.
            None => self.contacts.open_after_load = Some(card),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn birthdays_with_and_without_the_year() {
        assert_eq!(
            birth_day("1990-03-14"),
            Some(jiff::civil::date(1990, 3, 14))
        );
        assert_eq!(birth_day("--02-29"), Some(jiff::civil::date(2000, 2, 29)));
        assert_eq!(birth_day("soon"), None);
    }

    #[test]
    fn one_birthday_per_person_and_none_google_has() {
        let cards = vec![
            (1, "Asha Rao".into(), "1990-03-14".into()),
            (2, "Asha Rao".into(), "--03-14".into()),
            (3, "Bilal".into(), "--07-02".into()),
            (4, "Chen".into(), "--09-30".into()),
            (5, String::new(), "--01-01".into()),
        ];
        let taken = HashSet::from([(9, 30, "chen".to_owned())]);
        let rows = birthday_rows(cards, &taken);
        let ids: Vec<i64> = rows.iter().map(|r| r.id).collect();
        assert_eq!(ids, [1, 3]);
        assert!(rows.iter().all(|r| r.calendar_id == BIRTHDAYS
            && r.data.all_day
            && r.data.rrule == "FREQ=YEARLY"));

        let tz = jiff::tz::TimeZone::UTC;
        let (from, to) = (
            jiff::civil::date(2026, 3, 1).to_zoned(tz.clone()).unwrap(),
            jiff::civil::date(2026, 4, 1).to_zoned(tz.clone()).unwrap(),
        );
        let on = katna_dav::occurrences(
            rows,
            from.timestamp().as_second(),
            to.timestamp().as_second(),
            &tz,
        );
        assert_eq!(on.len(), 1);
        assert_eq!(
            on[0].start,
            jiff::civil::date(2026, 3, 14)
                .to_zoned(tz)
                .unwrap()
                .timestamp()
                .as_second()
        );
    }
}
