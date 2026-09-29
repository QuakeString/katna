// SPDX-License-Identifier: GPL-3.0-or-later

//! Typed quick add, as in Fantastical: "Lunch with Anita Friday 1pm at
//! Cafe Mocha" fills the new event's day, times and place as it is typed,
//! and keeps "Lunch with Anita" as its title (`docs/ARCHITECTURE.md` §18).
//! English words only for now; a title with none of them stays as typed.

use jiff::ToSpan;
use jiff::civil::{Date, Time, Weekday};

/// What a typed title says.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct Typed {
    /// The title without the words that gave the rest.
    pub title: String,
    pub day: Option<Date>,
    pub start: Option<Time>,
    pub end: Option<Time>,
    /// How long, when "for 30 min" says it.
    pub minutes: Option<i64>,
    pub location: Option<String>,
}

impl Typed {
    /// Whether anything besides the title was found.
    pub(super) fn found(&self) -> bool {
        self.day.is_some() || self.start.is_some() || self.location.is_some()
    }
}

const WEEKDAYS: [(&str, Weekday); 7] = [
    ("mon", Weekday::Monday),
    ("tue", Weekday::Tuesday),
    ("wed", Weekday::Wednesday),
    ("thu", Weekday::Thursday),
    ("fri", Weekday::Friday),
    ("sat", Weekday::Saturday),
    ("sun", Weekday::Sunday),
];

const MONTHS: [&str; 12] = [
    "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
];

fn weekday(word: &str) -> Option<Weekday> {
    let full = [
        "monday",
        "tuesday",
        "wednesday",
        "thursday",
        "friday",
        "saturday",
        "sunday",
    ];
    WEEKDAYS.iter().zip(full).find_map(|((short, day), long)| {
        (word == *short || word == long || (word.len() >= 3 && long.starts_with(word)))
            .then_some(*day)
    })
}

fn month(word: &str) -> Option<i8> {
    if word.len() < 3 {
        return None;
    }
    let names = [
        "january",
        "february",
        "march",
        "april",
        "may",
        "june",
        "july",
        "august",
        "september",
        "october",
        "november",
        "december",
    ];
    names
        .iter()
        .position(|name| name.starts_with(word) || (word == "sept" && *name == "september"))
        .or_else(|| MONTHS.iter().position(|m| word == *m))
        .map(|i| i as i8 + 1)
}

/// "5", "5th", "21st".
fn day_number(word: &str) -> Option<i8> {
    let digits = word.trim_end_matches(|c: char| c.is_ascii_alphabetic());
    let suffix = &word[digits.len()..];
    if !matches!(suffix, "" | "st" | "nd" | "rd" | "th") {
        return None;
    }
    digits.parse().ok().filter(|d| (1..=31).contains(d))
}

/// A clock time: "1pm", "1:30pm", "13:00", "9am", "noon", "midnight".
/// With `sure`, a bare hour ("at 3") counts too. Returns the time and
/// whether it said am or pm.
fn clock(word: &str, sure: bool) -> Option<(Time, bool)> {
    match word {
        "noon" | "midday" => return Some((Time::constant(12, 0, 0, 0), true)),
        "midnight" => return Some((Time::midnight(), true)),
        _ => {}
    }
    let (body, half) = if let Some(body) = word.strip_suffix("am").or(word.strip_suffix('a')) {
        (body, Some(false))
    } else if let Some(body) = word.strip_suffix("pm").or(word.strip_suffix('p')) {
        (body, Some(true))
    } else {
        (word, None)
    };
    let (hour, minute) = match body.split_once(':').or_else(|| body.split_once('.')) {
        Some((h, m)) if m.len() == 2 => (h.parse::<i8>().ok()?, m.parse::<i8>().ok()?),
        Some(_) => return None,
        None if half.is_some() || sure => (body.parse::<i8>().ok()?, 0),
        None => return None,
    };
    if body.is_empty() || !(0..60).contains(&minute) {
        return None;
    }
    let hour = match half {
        Some(pm) if (1..=12).contains(&hour) => hour % 12 + if pm { 12 } else { 0 },
        Some(_) => return None,
        // "at 3" is in the afternoon; "at 9" and "13:00" as written.
        None if !body.contains([':', '.']) && (1..=7).contains(&hour) => hour + 12,
        None if (0..24).contains(&hour) => hour,
        None => return None,
    };
    Some((Time::new(hour, minute, 0, 0).ok()?, half.is_some()))
}

/// "30 min", "2 hours", "1.5h", "90m", as minutes. `next` is the word
/// after `word`, for a unit apart from its number; returns how many words
/// it took.
fn duration(word: &str, next: Option<&str>) -> Option<(i64, usize)> {
    let split = word
        .find(|c: char| c.is_ascii_alphabetic())
        .unwrap_or(word.len());
    let (number, unit, used) = if split < word.len() {
        (&word[..split], &word[split..], 1)
    } else {
        (word, next?, 2)
    };
    let number: f64 = number.parse().ok()?;
    let per = match unit {
        "m" | "min" | "mins" | "minute" | "minutes" => 1.0,
        "h" | "hr" | "hrs" | "hour" | "hours" => 60.0,
        _ => return None,
    };
    let minutes = (number * per).round() as i64;
    (minutes > 0 && minutes <= 24 * 60).then_some((minutes, used))
}

/// The next `day` from `today`: today itself when it is that day, or
/// strictly after it for "next".
fn upcoming(today: Date, day: Weekday, strictly: bool) -> Date {
    let ahead =
        (day.to_monday_zero_offset() - today.weekday().to_monday_zero_offset()).rem_euclid(7);
    let ahead = if ahead == 0 && strictly { 7 } else { ahead };
    today.checked_add(i64::from(ahead).days()).unwrap_or(today)
}

/// `month` `day` next: this year, or next year once it has passed.
fn next_date(today: Date, month: i8, day: i8) -> Option<Date> {
    let this = Date::new(today.year(), month, day).ok()?;
    if this >= today {
        Some(this)
    } else {
        Date::new(today.year() + 1, month, day).ok()
    }
}

/// Reads `text` typed as a new event's title on `today`.
pub(super) fn parse(text: &str, today: Date) -> Typed {
    let words: Vec<&str> = text.split_whitespace().collect();
    let lower: Vec<String> = words
        .iter()
        .map(|w| {
            w.trim_matches(|c: char| matches!(c, ',' | ';' | '!' | '?'))
                .to_lowercase()
        })
        .collect();
    let mut used = vec![false; words.len()];
    let mut typed = Typed::default();
    let mut i = 0;
    // The word before `i`, to be taken with it: "on Friday", "at 3pm".
    let lead = |i: usize, words: &[&str]| -> Vec<usize> {
        match i.checked_sub(1).map(|p| lower[p].as_str()) {
            Some("on" | "at" | "@" | "from" | "by") if words.len() > 1 => vec![i - 1],
            _ => Vec::new(),
        }
    };
    while i < words.len() {
        let word = lower[i].as_str();
        let next = lower.get(i + 1).map(String::as_str);
        let take = |from: usize, to: usize, used: &mut [bool]| {
            for u in used.iter_mut().take(to).skip(from) {
                *u = true;
            }
        };
        // Days.
        if typed.day.is_none() {
            let day = match word {
                "today" | "tonight" => Some((today, 1)),
                "tomorrow" | "tmrw" | "tmr" => Some((today.tomorrow().unwrap_or(today), 1)),
                "next" => next
                    .and_then(weekday)
                    .map(|d| (upcoming(today, d, true), 2)),
                _ => weekday(word)
                    .filter(|_| word.len() >= 3)
                    .map(|d| (upcoming(today, d, false), 1))
                    .or_else(|| {
                        // "Oct 5", "October 5th", "5 Oct", "5th of October".
                        let after = next?;
                        if let (Some(m), Some(d)) = (month(word), day_number(after)) {
                            return next_date(today, m, d).map(|date| (date, 2));
                        }
                        let d = day_number(word)?;
                        if let Some(m) = month(after) {
                            return next_date(today, m, d).map(|date| (date, 2));
                        }
                        let m =
                            (after == "of").then(|| lower.get(i + 2).and_then(|w| month(w)))??;
                        next_date(today, m, d).map(|date| (date, 3))
                    }),
            };
            if let Some((date, n)) = day {
                typed.day = Some(date);
                for p in lead(i, &words) {
                    used[p] = true;
                }
                take(i, i + n, &mut used);
                if word == "tonight" && typed.start.is_none() {
                    typed.start = Some(Time::constant(19, 0, 0, 0));
                }
                i += n;
                continue;
            }
        }
        // Times, and a range: "1-2pm", "1pm to 3pm", "from 10 until 11".
        if typed.start.is_none() {
            let sure = i > 0 && matches!(lower[i - 1].as_str(), "at" | "@" | "from");
            // "1pm-3pm" in one word.
            let (first, second) = match word.split_once(['-', '–']) {
                Some((a, b)) if !a.is_empty() && !b.is_empty() => (a, Some((b, 1))),
                _ => match (next, lower.get(i + 2)) {
                    (Some("-" | "–" | "to" | "until" | "till"), Some(b)) => {
                        (word, Some((b.as_str(), 3)))
                    }
                    _ => (word, None),
                },
            };
            let end = second.and_then(|(b, n)| clock(b, true).map(|t| (t, n)));
            let start = clock(first, sure || end.is_some());
            if let Some((start, said)) = start {
                let (start, n) = match end {
                    Some(((end, end_said), n)) => {
                        // "1-2pm": the start takes the end's afternoon.
                        let mut start = start;
                        if !said && end_said && end.hour() >= 12 && start.hour() + 12 <= end.hour()
                        {
                            start =
                                Time::new(start.hour() + 12, start.minute(), 0, 0).unwrap_or(start);
                        }
                        if start.hour() >= 12 && !said && end.hour() < start.hour() {
                            start =
                                Time::new(start.hour() - 12, start.minute(), 0, 0).unwrap_or(start);
                        }
                        typed.end = Some(end);
                        (start, n)
                    }
                    None => (start, 1),
                };
                typed.start = Some(start);
                for p in lead(i, &words) {
                    used[p] = true;
                }
                take(i, i + n, &mut used);
                i += n;
                continue;
            }
        }
        // "for 30 min".
        if word == "for"
            && typed.minutes.is_none()
            && let Some((minutes, n)) =
                next.and_then(|w| duration(w, lower.get(i + 2).map(String::as_str)))
        {
            typed.minutes = Some(minutes);
            take(i, i + 1 + n, &mut used);
            i += 1 + n;
            continue;
        }
        i += 1;
    }
    // The place: after the last "at" or "@" that isn't a time, to the end
    // but for the words already taken.
    if let Some(at) = (0..words.len()).rev().find(|&p| {
        !used[p]
            && matches!(lower[p].as_str(), "at" | "@")
            && p + 1 < words.len()
            && !used[p + 1]
            && p > 0
    }) {
        let place: Vec<&str> = (at + 1..words.len())
            .filter(|&p| !used[p])
            .map(|p| words[p])
            .collect();
        if !place.is_empty() {
            typed.location = Some(place.join(" "));
            for u in used.iter_mut().skip(at) {
                *u = true;
            }
        }
    }
    let mut title: Vec<&str> = (0..words.len())
        .filter(|&p| !used[p])
        .map(|p| words[p])
        .collect();
    while title.last().is_some_and(|w| {
        matches!(
            w.to_lowercase().as_str(),
            "on" | "at" | "from" | "for" | "@" | "-"
        )
    }) {
        title.pop();
    }
    typed.title = title.join(" ");
    if typed.title.is_empty() {
        typed.title = text.trim().to_owned();
    }
    typed
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::date;

    /// A Tuesday.
    fn today() -> Date {
        date(2026, 9, 29)
    }

    fn t(h: i8, m: i8) -> Option<Time> {
        Some(Time::constant(h, m, 0, 0))
    }

    #[test]
    fn a_sentence_fills_day_time_and_place() {
        let typed = parse("Lunch with Anita Friday 1pm at Cafe Mocha", today());
        assert_eq!(typed.title, "Lunch with Anita");
        assert_eq!(typed.day, Some(date(2026, 10, 2)));
        assert_eq!(typed.start, t(13, 0));
        assert_eq!(typed.location.as_deref(), Some("Cafe Mocha"));
    }

    #[test]
    fn days() {
        let day = |text: &str| parse(text, today()).day;
        assert_eq!(day("Call tomorrow"), Some(date(2026, 9, 30)));
        assert_eq!(day("Standup today"), Some(today()));
        assert_eq!(day("Gym on tue"), Some(today()));
        assert_eq!(day("Gym next tuesday"), Some(date(2026, 10, 6)));
        assert_eq!(day("Dentist Oct 5"), Some(date(2026, 10, 5)));
        assert_eq!(day("Dentist 5th of October"), Some(date(2026, 10, 5)));
        assert_eq!(day("Party 12 September"), Some(date(2027, 9, 12)));
        assert_eq!(day("March on"), None);
    }

    #[test]
    fn times_and_ranges() {
        let times = |text: &str| {
            let typed = parse(text, today());
            (typed.title, typed.start, typed.end)
        };
        assert_eq!(times("Review 10:30"), ("Review".into(), t(10, 30), None));
        assert_eq!(times("Review at 3"), ("Review".into(), t(15, 0), None));
        assert_eq!(times("Sync 1-2pm"), ("Sync".into(), t(13, 0), t(14, 0)));
        assert_eq!(
            times("Sync 11am to 1pm"),
            ("Sync".into(), t(11, 0), t(13, 0))
        );
        assert_eq!(
            times("Sync 11 - 12:30pm"),
            ("Sync".into(), t(11, 0), t(12, 30))
        );
        assert_eq!(times("Lunch at noon"), ("Lunch".into(), t(12, 0), None));
        // Bare numbers are left in the title.
        assert_eq!(times("Buy 3 books"), ("Buy 3 books".into(), None, None));
    }

    #[test]
    fn durations_and_plain_titles() {
        let typed = parse("Call bank tomorrow 4pm for 30 min", today());
        assert_eq!(typed.title, "Call bank");
        assert_eq!(typed.minutes, Some(30));
        let typed = parse("Workshop at the library", today());
        assert_eq!(typed.title, "Workshop");
        assert_eq!(typed.location.as_deref(), Some("the library"));
        let typed = parse("Read a book", today());
        assert_eq!(typed.title, "Read a book");
        assert!(!typed.found());
    }
}
