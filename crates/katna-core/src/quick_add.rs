// SPDX-License-Identifier: GPL-3.0-or-later

//! Typed quick add, as in Fantastical and Todoist, for events and tasks
//! alike: "Lunch with Anita Friday 1pm at Cafe Mocha" or "Standup every
//! weekday 9:30 for 15 min" gives a day, times, a length, a repeat and a
//! place, and keeps the rest ("Lunch with Anita") as the title
//! (`docs/ARCHITECTURE.md` §18).
//!
//! The words come from a [`Words`] table per language. English is the
//! only one so far; another language adds a table to [`Words::for_language`]
//! (languages that write times as "13 h" or put the day after the month
//! need no change to the parser). A title with none of the words stays as
//! typed.

use jiff::ToSpan;
use jiff::civil::{Date, Time, Weekday};

/// What a typed title says.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Typed {
    /// The title without the words that gave the rest.
    pub title: String,
    pub day: Option<Date>,
    pub start: Option<Time>,
    pub end: Option<Time>,
    /// How long, when "for 30 min" says it.
    pub minutes: Option<i64>,
    /// An RFC 5545 RRULE value ("FREQ=WEEKLY;BYDAY=MO"), without "RRULE:".
    pub repeat: Option<String>,
    pub location: Option<String>,
    /// Labels, from words after a label mark ("#home" with
    /// [`Words::label_marks`] `["#"]`), each once, without the mark.
    pub labels: Vec<String>,
}

impl Typed {
    /// Whether anything besides the title was found.
    pub fn found(&self) -> bool {
        self.day.is_some()
            || self.start.is_some()
            || self.minutes.is_some()
            || self.repeat.is_some()
            || self.location.is_some()
            || !self.labels.is_empty()
    }
}

/// The words of one language, all lower case.
pub struct Words {
    pub today: &'static [&'static str],
    pub tonight: &'static [&'static str],
    pub tomorrow: &'static [&'static str],
    /// Before a weekday: the one after this week's ("next Friday").
    pub next: &'static [&'static str],
    /// Monday first, full names; three letters or more of one count.
    pub weekdays: [&'static str; 7],
    /// January first, full names; three letters or more of one count.
    pub months: [&'static str; 12],
    /// Suffixes of day numbers ("5th").
    pub ordinals: &'static [&'static str],
    /// Between a day number and a month ("5th of October").
    pub of: &'static [&'static str],
    /// Before a day or time, taken with it ("on Friday", "at 3pm").
    pub leads: &'static [&'static str],
    /// Before a place ("at Cafe Mocha").
    pub at: &'static [&'static str],
    /// Between two times ("1pm to 3pm").
    pub until: &'static [&'static str],
    pub noon: &'static [&'static str],
    pub midnight: &'static [&'static str],
    pub am: &'static [&'static str],
    pub pm: &'static [&'static str],
    /// Before a length ("for 30 min").
    pub for_: &'static [&'static str],
    pub minutes: &'static [&'static str],
    pub hours: &'static [&'static str],
    /// Before a repeat ("every Monday").
    pub every: &'static [&'static str],
    pub daily: &'static [&'static str],
    pub weekly: &'static [&'static str],
    pub monthly: &'static [&'static str],
    pub yearly: &'static [&'static str],
    /// After "every": "day", "week", …, and their plurals ("every 2 weeks").
    pub day_units: &'static [&'static str],
    pub week_units: &'static [&'static str],
    pub month_units: &'static [&'static str],
    pub year_units: &'static [&'static str],
    /// "every weekday".
    pub weekday_word: &'static [&'static str],
    /// Between weekdays ("every Mon and Thu").
    pub and: &'static [&'static str],
    /// What starts a label ("#home"): none for events, `#` for tasks. A
    /// label is letters, digits, `-` and `_` after the mark.
    pub label_marks: &'static [&'static str],
}

impl Words {
    pub const ENGLISH: Self = Self {
        today: &["today"],
        tonight: &["tonight"],
        tomorrow: &["tomorrow", "tmrw", "tmr"],
        next: &["next"],
        weekdays: [
            "monday",
            "tuesday",
            "wednesday",
            "thursday",
            "friday",
            "saturday",
            "sunday",
        ],
        months: [
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
        ],
        ordinals: &["st", "nd", "rd", "th"],
        of: &["of"],
        leads: &["on", "at", "@", "from", "by"],
        at: &["at", "@"],
        until: &["-", "–", "to", "until", "till"],
        noon: &["noon", "midday"],
        midnight: &["midnight"],
        am: &["am", "a"],
        pm: &["pm", "p"],
        for_: &["for"],
        minutes: &["m", "min", "mins", "minute", "minutes"],
        hours: &["h", "hr", "hrs", "hour", "hours"],
        every: &["every", "each"],
        daily: &["daily"],
        weekly: &["weekly"],
        monthly: &["monthly"],
        yearly: &["yearly", "annually"],
        day_units: &["day", "days"],
        week_units: &["week", "weeks"],
        month_units: &["month", "months"],
        year_units: &["year", "years"],
        weekday_word: &["weekday", "weekdays", "workday", "workdays"],
        label_marks: &[],
        and: &["and", "&", "+"],
    };

    /// The table for `language` (a BCP 47 tag, "en-IN"), English when
    /// there is none yet.
    pub fn for_language(language: &str) -> &'static Self {
        let _ = language;
        &Self::ENGLISH
    }

    fn weekday(&self, word: &str) -> Option<Weekday> {
        const DAYS: [Weekday; 7] = [
            Weekday::Monday,
            Weekday::Tuesday,
            Weekday::Wednesday,
            Weekday::Thursday,
            Weekday::Friday,
            Weekday::Saturday,
            Weekday::Sunday,
        ];
        let word = word
            .strip_suffix('s')
            .filter(|w| w.len() >= 3)
            .unwrap_or(word);
        (word.chars().count() >= 3)
            .then(|| self.weekdays.iter().position(|name| name.starts_with(word)))
            .flatten()
            .map(|i| DAYS[i])
    }

    fn month(&self, word: &str) -> Option<i8> {
        (word.chars().count() >= 3)
            .then(|| self.months.iter().position(|name| name.starts_with(word)))
            .flatten()
            .map(|i| i as i8 + 1)
    }

    /// "5", "5th", "21st".
    fn day_number(&self, word: &str) -> Option<i8> {
        let digits = word.trim_end_matches(|c: char| c.is_alphabetic());
        let suffix = &word[digits.len()..];
        if !(suffix.is_empty() || self.ordinals.contains(&suffix)) {
            return None;
        }
        digits.parse().ok().filter(|d| (1..=31).contains(d))
    }

    /// A clock time: "1pm", "1:30pm", "13:00", "9am", "noon". With
    /// `sure`, a bare hour ("at 3") counts too. Returns the time and
    /// whether it said am or pm.
    fn clock(&self, word: &str, sure: bool) -> Option<(Time, bool)> {
        if self.noon.contains(&word) {
            return Some((Time::constant(12, 0, 0, 0), true));
        }
        if self.midnight.contains(&word) {
            return Some((Time::midnight(), true));
        }
        let strip = |list: &[&str]| {
            list.iter()
                .filter_map(|suffix| word.strip_suffix(suffix))
                .find(|body| body.ends_with(|c: char| c.is_ascii_digit()))
        };
        let (body, half) = if let Some(body) = strip(self.am) {
            (body, Some(false))
        } else if let Some(body) = strip(self.pm) {
            (body, Some(true))
        } else {
            (word, None)
        };
        let (hour, minute) = match body.split_once([':', '.']) {
            Some((h, m)) if m.len() == 2 => (h.parse::<i8>().ok()?, m.parse::<i8>().ok()?),
            Some(_) => return None,
            None if half.is_some() || sure => (body.parse::<i8>().ok()?, 0),
            None => return None,
        };
        if !(0..60).contains(&minute) {
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
    /// after `word`, for a unit apart from its number; returns how many
    /// words it took.
    fn duration(&self, word: &str, next: Option<&str>) -> Option<(i64, usize)> {
        let split = word.find(|c: char| c.is_alphabetic()).unwrap_or(word.len());
        let (number, unit, used) = if split < word.len() {
            (&word[..split], &word[split..], 1)
        } else {
            (word, next?, 2)
        };
        let number: f64 = number.parse().ok()?;
        let per = if self.minutes.contains(&unit) {
            1.0
        } else if self.hours.contains(&unit) {
            60.0
        } else {
            return None;
        };
        let minutes = (number * per).round() as i64;
        (minutes > 0 && minutes <= 24 * 60).then_some((minutes, used))
    }
}

/// RFC 5545's two-letter weekday.
fn code(day: Weekday) -> &'static str {
    ["MO", "TU", "WE", "TH", "FR", "SA", "SU"][day.to_monday_zero_offset() as usize]
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

/// A repeat before its day is known.
enum Repeat {
    /// FREQ and INTERVAL.
    Every(&'static str, i64),
    /// Weekly on these days.
    On(Vec<Weekday>),
}

/// Reads `text` typed as a new event's or task's title on `today`, with
/// the language's `words`.
pub fn parse(text: &str, today: Date, words: &Words) -> Typed {
    let original: Vec<&str> = text.split_whitespace().collect();
    let lower: Vec<String> = original
        .iter()
        .map(|w| {
            w.trim_matches(|c: char| matches!(c, ',' | ';' | '!' | '?'))
                .to_lowercase()
        })
        .collect();
    let has = |list: &[&str], i: usize| lower.get(i).is_some_and(|w| list.contains(&w.as_str()));
    let count = original.len();
    let mut used = vec![false; count];
    let mut typed = Typed::default();
    let mut repeat: Option<Repeat> = None;
    let mut i = 0;
    // The word before `i`, taken with it: "on Friday", "at 3pm".
    let lead = |i: usize, used: &mut [bool]| {
        if i > 0 && has(words.leads, i - 1) {
            used[i - 1] = true;
        }
    };
    let take = |from: usize, to: usize, used: &mut [bool]| {
        for u in used.iter_mut().take(to.min(count)).skip(from) {
            *u = true;
        }
    };
    while i < count {
        let word = lower[i].as_str();
        let next = lower.get(i + 1).map(String::as_str);

        // Labels: "#home", as typed (its case kept).
        if let Some(label) = label_of(original[i], words.label_marks) {
            if !typed.labels.contains(&label) {
                typed.labels.push(label);
            }
            take(i, i + 1, &mut used);
            i += 1;
            continue;
        }

        // Repeats: "every day", "every 2 weeks", "every Mon and Thu",
        // "every weekday", "daily".
        if repeat.is_none() {
            let simple = |w: &str| {
                if words.daily.contains(&w) {
                    Some("DAILY")
                } else if words.weekly.contains(&w) {
                    Some("WEEKLY")
                } else if words.monthly.contains(&w) {
                    Some("MONTHLY")
                } else if words.yearly.contains(&w) {
                    Some("YEARLY")
                } else {
                    None
                }
            };
            if let Some(freq) = simple(word) {
                repeat = Some(Repeat::Every(freq, 1));
                take(i, i + 1, &mut used);
                i += 1;
                continue;
            }
            if words.every.contains(&word) {
                let mut j = i + 1;
                let interval = lower
                    .get(j)
                    .and_then(|w| w.parse::<i64>().ok())
                    .filter(|n| (1..=99).contains(n));
                if interval.is_some() {
                    j += 1;
                }
                let unit = lower.get(j).map(String::as_str).unwrap_or_default();
                let freq = if words.day_units.contains(&unit) {
                    Some("DAILY")
                } else if words.week_units.contains(&unit) {
                    Some("WEEKLY")
                } else if words.month_units.contains(&unit) {
                    Some("MONTHLY")
                } else if words.year_units.contains(&unit) {
                    Some("YEARLY")
                } else {
                    None
                };
                if let Some(freq) = freq {
                    repeat = Some(Repeat::Every(freq, interval.unwrap_or(1)));
                    take(i, j + 1, &mut used);
                    i = j + 1;
                    continue;
                }
                if interval.is_none() && words.weekday_word.contains(&unit) {
                    repeat = Some(Repeat::On(vec![
                        Weekday::Monday,
                        Weekday::Tuesday,
                        Weekday::Wednesday,
                        Weekday::Thursday,
                        Weekday::Friday,
                    ]));
                    take(i, j + 1, &mut used);
                    i = j + 1;
                    continue;
                }
                // "every Mon", "every Mon and Thu", "every Mon, Wed".
                let mut days = Vec::new();
                let mut end = j;
                while let Some(w) = lower.get(end) {
                    if let Some(day) = words.weekday(w) {
                        days.push(day);
                        end += 1;
                    } else if !days.is_empty()
                        && words.and.contains(&w.as_str())
                        && lower
                            .get(end + 1)
                            .is_some_and(|w| words.weekday(w).is_some())
                    {
                        end += 1;
                    } else {
                        break;
                    }
                }
                if interval.is_none() && !days.is_empty() {
                    repeat = Some(Repeat::On(days));
                    take(i, end, &mut used);
                    i = end;
                    continue;
                }
            }
        }

        // Days.
        if typed.day.is_none() {
            let day = if words.today.contains(&word) || words.tonight.contains(&word) {
                Some((today, 1))
            } else if words.tomorrow.contains(&word) {
                Some((today.tomorrow().unwrap_or(today), 1))
            } else if words.next.contains(&word) {
                next.and_then(|w| words.weekday(w))
                    .map(|d| (upcoming(today, d, true), 2))
            } else {
                words
                    .weekday(word)
                    .map(|d| (upcoming(today, d, false), 1))
                    .or_else(|| {
                        // "Oct 5", "October 5th", "5 Oct", "5th of October".
                        let after = next?;
                        if let (Some(m), Some(d)) = (words.month(word), words.day_number(after)) {
                            return next_date(today, m, d).map(|date| (date, 2));
                        }
                        let d = words.day_number(word)?;
                        if let Some(m) = words.month(after) {
                            return next_date(today, m, d).map(|date| (date, 2));
                        }
                        let m = words
                            .of
                            .contains(&after)
                            .then(|| lower.get(i + 2).and_then(|w| words.month(w)))??;
                        next_date(today, m, d).map(|date| (date, 3))
                    })
            };
            if let Some((date, n)) = day {
                typed.day = Some(date);
                lead(i, &mut used);
                take(i, i + n, &mut used);
                if words.tonight.contains(&word) && typed.start.is_none() {
                    typed.start = Some(Time::constant(19, 0, 0, 0));
                }
                i += n;
                continue;
            }
        }

        // Times, and a range: "1-2pm", "1pm to 3pm", "from 10 until 11".
        if typed.start.is_none() {
            let sure = i > 0 && (has(words.at, i - 1) || lower[i - 1] == "from");
            let (first, second) = match word.split_once(['-', '–']) {
                Some((a, b)) if !a.is_empty() && !b.is_empty() => (a, Some((b, 1))),
                _ => match lower.get(i + 2) {
                    Some(b) if has(words.until, i + 1) => (word, Some((b.as_str(), 3))),
                    _ => (word, None),
                },
            };
            let end = second.and_then(|(b, n)| words.clock(b, true).map(|t| (t, n)));
            if let Some((mut start, said)) = words.clock(first, sure || end.is_some()) {
                let mut n = 1;
                if let Some(((end, end_said), taken)) = end {
                    // "1-2pm": the start takes the end's afternoon.
                    if !said && end_said && end.hour() >= 12 && start.hour() + 12 <= end.hour() {
                        start = Time::new(start.hour() + 12, start.minute(), 0, 0).unwrap_or(start);
                    }
                    if start.hour() >= 12 && !said && end.hour() < start.hour() {
                        start = Time::new(start.hour() - 12, start.minute(), 0, 0).unwrap_or(start);
                    }
                    typed.end = Some(end);
                    n = taken;
                }
                typed.start = Some(start);
                lead(i, &mut used);
                take(i, i + n, &mut used);
                i += n;
                continue;
            }
        }

        // "for 30 min".
        if typed.minutes.is_none()
            && words.for_.contains(&word)
            && let Some((minutes, n)) =
                next.and_then(|w| words.duration(w, lower.get(i + 2).map(String::as_str)))
        {
            typed.minutes = Some(minutes);
            take(i, i + 1 + n, &mut used);
            i += 1 + n;
            continue;
        }
        i += 1;
    }

    // The place: after the last "at" that isn't a time, to the end but for
    // the words already taken.
    if let Some(at) = (1..count)
        .rev()
        .find(|&p| !used[p] && has(words.at, p) && p + 1 < count && !used[p + 1])
    {
        let place: Vec<&str> = (at + 1..count)
            .filter(|&p| !used[p])
            .map(|p| original[p])
            .collect();
        if !place.is_empty() {
            typed.location = Some(place.join(" "));
            take(at, count, &mut used);
        }
    }

    // A weekly repeat on days starts on the first of them.
    if let Some(Repeat::On(days)) = &repeat
        && typed.day.is_none()
    {
        typed.day = days.iter().map(|&d| upcoming(today, d, false)).min();
    }
    let first = typed.day.unwrap_or(today);
    typed.repeat = repeat.map(|repeat| match repeat {
        Repeat::Every("WEEKLY", 1) => format!("FREQ=WEEKLY;BYDAY={}", code(first.weekday())),
        Repeat::Every(freq, 1) => format!("FREQ={freq}"),
        Repeat::Every(freq, n) => format!("FREQ={freq};INTERVAL={n}"),
        Repeat::On(days) => {
            let mut days: Vec<Weekday> = days;
            days.sort_by_key(|d| d.to_monday_zero_offset());
            days.dedup();
            let codes: Vec<&str> = days.into_iter().map(code).collect();
            format!("FREQ=WEEKLY;BYDAY={}", codes.join(","))
        }
    });

    let mut title: Vec<&str> = (0..count)
        .filter(|&p| !used[p])
        .map(|p| original[p])
        .collect();
    while title.last().is_some_and(|w| {
        let w = w.to_lowercase();
        words.leads.contains(&w.as_str()) || words.for_.contains(&w.as_str()) || w == "-"
    }) {
        title.pop();
    }
    typed.title = title.join(" ");
    if typed.title.is_empty() {
        typed.title = text.trim().to_owned();
    }
    typed
}

/// The label `word` names after one of `marks` ("#home" is "home"), with
/// a comma or full stop after it left off; `None` when it names none.
fn label_of(word: &str, marks: &[&str]) -> Option<String> {
    let word = word.trim_end_matches(|c: char| matches!(c, ',' | ';' | '.' | '!' | '?'));
    let rest = marks.iter().find_map(|m| word.strip_prefix(m))?;
    let ok = !rest.is_empty()
        && rest.chars().any(char::is_alphabetic)
        && rest
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_');
    ok.then(|| rest.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::date;

    /// A Tuesday.
    fn today() -> Date {
        date(2026, 9, 29)
    }

    fn en(text: &str) -> Typed {
        parse(text, today(), &Words::ENGLISH)
    }

    fn t(h: i8, m: i8) -> Option<Time> {
        Some(Time::constant(h, m, 0, 0))
    }

    #[test]
    fn hash_words_are_labels_for_tasks_only() {
        let tasks = Words {
            label_marks: &["#"],
            ..Words::ENGLISH
        };
        let typed = parse("Call the plumber fri 6pm #home", today(), &tasks);
        assert_eq!(typed.title, "Call the plumber");
        assert_eq!(typed.labels, ["home"]);
        assert_eq!(typed.day, Some(date(2026, 10, 2)));
        assert_eq!(typed.start, t(18, 0));
        let typed = parse("#Work Pay #Bills, #bills #Work #42 #", today(), &tasks);
        assert_eq!(typed.title, "Pay #42 #");
        assert_eq!(typed.labels, ["Work", "Bills", "bills"]);
        assert!(parse("Pay rent #home", today(), &tasks).found());
        assert!(!parse("Issue #42", today(), &tasks).found());
        // Events keep the word in their title.
        let typed = en("Standup #team");
        assert_eq!(typed.title, "Standup #team");
        assert!(typed.labels.is_empty());
    }

    #[test]
    fn a_sentence_fills_day_time_and_place() {
        let typed = en("Lunch with Anita Friday 1pm at Cafe Mocha");
        assert_eq!(typed.title, "Lunch with Anita");
        assert_eq!(typed.day, Some(date(2026, 10, 2)));
        assert_eq!(typed.start, t(13, 0));
        assert_eq!(typed.location.as_deref(), Some("Cafe Mocha"));
    }

    #[test]
    fn days() {
        let day = |text: &str| en(text).day;
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
            let typed = en(text);
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
    fn repeats() {
        let rule = |text: &str| {
            let typed = en(text);
            (typed.title, typed.repeat, typed.day)
        };
        assert_eq!(
            rule("Standup every weekday 9:30"),
            (
                "Standup".into(),
                Some("FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR".into()),
                Some(today())
            )
        );
        assert_eq!(
            rule("Gym every Mon and Thu 7am"),
            (
                "Gym".into(),
                Some("FREQ=WEEKLY;BYDAY=MO,TH".into()),
                Some(date(2026, 10, 1))
            )
        );
        assert_eq!(
            rule("Water plants every 3 days"),
            (
                "Water plants".into(),
                Some("FREQ=DAILY;INTERVAL=3".into()),
                None
            )
        );
        assert_eq!(
            rule("Review weekly"),
            ("Review".into(), Some("FREQ=WEEKLY;BYDAY=TU".into()), None)
        );
        assert_eq!(
            rule("Rent monthly"),
            ("Rent".into(), Some("FREQ=MONTHLY".into()), None)
        );
    }

    #[test]
    fn durations_and_plain_titles() {
        let typed = en("Call bank tomorrow 4pm for 30 min");
        assert_eq!(typed.title, "Call bank");
        assert_eq!(typed.minutes, Some(30));
        let typed = en("Workshop at the library");
        assert_eq!(typed.title, "Workshop");
        assert_eq!(typed.location.as_deref(), Some("the library"));
        let typed = en("Read a book");
        assert_eq!(typed.title, "Read a book");
        assert!(!typed.found());
    }
}
