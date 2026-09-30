// SPDX-License-Identifier: GPL-3.0-or-later

//! The times of schedule send: the suggested ones, the calendar of the
//! date and time picker, and how the chosen time is written. The
//! background service holds the message until then, so it goes out with
//! the app closed.

use jiff::civil::{Date, DateTime, Time, Weekday};
use jiff::tz::TimeZone;
use jiff::{Timestamp, ToSpan, Zoned};
use katna_i18n::{format, tr};

/// A suggested time: its name and when.
pub(super) struct Preset {
    pub label: String,
    pub at: Zoned,
}

/// The suggested times of the schedule menu, as `now` sees them: tomorrow
/// morning and afternoon, and Monday morning unless that is tomorrow.
pub(super) fn presets(now: &Zoned) -> Vec<Preset> {
    let at = |date: Date, hour: i8| -> Option<Zoned> {
        date.to_datetime(Time::constant(hour, 0, 0, 0))
            .to_zoned(now.time_zone().clone())
            .ok()
    };
    let today = now.date();
    let Ok(tomorrow) = today.tomorrow() else {
        return Vec::new();
    };
    let mut presets = Vec::new();
    if now.hour() < 7 {
        presets.extend(at(today, 8).map(|at| Preset {
            label: tr!("schedule-this-morning"),
            at,
        }));
    } else if now.hour() < 12 {
        presets.extend(at(today, 13).map(|at| Preset {
            label: tr!("schedule-this-afternoon"),
            at,
        }));
    }
    presets.extend(at(tomorrow, 8).map(|at| Preset {
        label: tr!("schedule-tomorrow-morning"),
        at,
    }));
    presets.extend(at(tomorrow, 13).map(|at| Preset {
        label: tr!("schedule-tomorrow-afternoon"),
        at,
    }));
    let monday = today.nth_weekday(1, Weekday::Monday).ok();
    if let Some(monday) = monday.filter(|m| *m != tomorrow) {
        presets.extend(at(monday, 8).map(|at| Preset {
            label: tr!("schedule-monday-morning"),
            at,
        }));
    }
    presets.truncate(3);
    presets
}

/// When a preset goes out, as the menu shows it: "Sep 27, 8:00 AM".
pub(super) fn short(at: &Zoned) -> String {
    format::day_month_time(at.datetime())
}

/// `at` in the user's zone for the snackbar and the list of scheduled
/// mail: "Sun, Sep 27, 2026, 8:00 AM".
pub(super) fn describe(at: Timestamp, tz: &TimeZone) -> String {
    format::long(at.to_zoned(tz.clone()).datetime())
}

/// A time of day as the language writes it: "8:00 AM", "08:00".
pub(in crate::window) fn clock(time: Time) -> String {
    format::time(DateTime::from_parts(jiff::civil::date(2026, 1, 1), time))
}

/// The `Date` header of a message sent at `at`, in the user's zone.
pub(super) fn rfc2822(at: Timestamp, tz: &TimeZone) -> String {
    at.to_zoned(tz.clone())
        .strftime("%a, %d %b %Y %H:%M:%S %z")
        .to_string()
}

/// The parts of a written time: the hour, the minutes and AM/PM.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TimePart {
    Hour,
    Minute,
    Half,
}

/// Where each part of a written time is, in bytes: the first run of
/// digits is the hour, the second the minutes, and letters are AM/PM
/// (before or after the digits, as the language writes it).
fn time_parts(text: &str) -> Vec<(TimePart, std::ops::Range<usize>)> {
    let mut parts: Vec<(TimePart, std::ops::Range<usize>)> = Vec::new();
    let mut digits = 0;
    for (ix, c) in text.char_indices() {
        let part = if c.is_numeric() {
            Some(TimePart::Minute)
        } else if c.is_alphabetic() {
            Some(TimePart::Half)
        } else {
            None
        };
        let end = ix + c.len_utf8();
        match (part, parts.last_mut()) {
            (Some(TimePart::Minute), Some((TimePart::Hour | TimePart::Minute, run)))
            | (Some(TimePart::Half), Some((TimePart::Half, run)))
                if run.end == ix =>
            {
                run.end = end;
            }
            (Some(TimePart::Minute), _) => {
                let kind = if digits == 0 {
                    TimePart::Hour
                } else {
                    TimePart::Minute
                };
                digits += 1;
                parts.push((kind, ix..end));
            }
            (Some(TimePart::Half), _) => parts.push((TimePart::Half, ix..end)),
            _ => {}
        }
    }
    parts
}

/// Up (`by` 1) or Down (-1) in a time field: changes the hour, the
/// minutes or AM/PM, whichever the cursor is in, wrapping round without
/// changing the other parts. Returns the new text and a cursor in the same
/// part, or `None` when the text is not a time.
pub(in crate::window) fn step_time(text: &str, cursor: usize, by: i32) -> Option<(String, usize)> {
    let time = parse_time(text)?;
    let parts = time_parts(text);
    // The part the cursor is in, or else the one it touches from the left.
    let (part, range) = parts
        .iter()
        .find(|(_, r)| r.start <= cursor && cursor < r.end)
        .or_else(|| parts.iter().find(|(_, r)| r.end == cursor))?
        .clone();
    let twelve = parts.iter().any(|(p, _)| *p == TimePart::Half);
    let (hour, minute) = (i32::from(time.hour()), i32::from(time.minute()));
    let (hour, minute) = match part {
        TimePart::Hour if twelve => {
            let shown = (hour + 11) % 12 + 1;
            let shown = (shown - 1 + by).rem_euclid(12) + 1;
            (shown % 12 + hour / 12 * 12, minute)
        }
        TimePart::Hour => ((hour + by).rem_euclid(24), minute),
        TimePart::Minute => (hour, (minute + by).rem_euclid(60)),
        TimePart::Half => ((hour + 12) % 24, minute),
    };
    let new = clock(Time::new(hour as i8, minute as i8, 0, 0).ok()?);
    let at_start = cursor == range.start;
    let place = time_parts(&new)
        .into_iter()
        .find(|(p, _)| *p == part)
        .map_or(new.len(), |(_, r)| if at_start { r.start } else { r.end });
    Some((new, place))
}

/// Up and Down for a time field: see [`step_time`].
pub(in crate::window) fn time_stepper() -> katna_ui::text_input::Stepper {
    std::sync::Arc::new(step_time)
}

/// Reads a time of day as people type it: as [`clock`] writes it, or
/// "8:00 AM", "8am", "13:30", "1.30 pm", "9", in any script's digits.
pub(in crate::window) fn parse_time(text: &str) -> Option<Time> {
    let squeeze = |t: &str| -> String {
        t.chars()
            .filter(|c| !c.is_whitespace())
            .map(ascii_digit)
            .collect::<String>()
            .to_lowercase()
    };
    let text = squeeze(text);
    // As the language writes times, with its own words for AM and PM.
    let written = (0..24 * 60).find_map(|minutes: i32| {
        let time = Time::new((minutes / 60) as i8, (minutes % 60) as i8, 0, 0).ok()?;
        (squeeze(&clock(time)) == text).then_some(time)
    });
    if written.is_some() {
        return written;
    }
    let (digits, half) = if let Some(t) = text.strip_suffix("am").or(text.strip_suffix('a')) {
        (t.to_owned(), Some(false))
    } else if let Some(t) = text.strip_suffix("pm").or(text.strip_suffix('p')) {
        (t.to_owned(), Some(true))
    } else {
        (text, None)
    };
    let (hour, minute) = match digits.split_once([':', '.']) {
        Some((h, m)) => (h.parse::<i8>().ok()?, m.parse::<i8>().ok()?),
        None if digits.len() > 2 => {
            let (h, m) = digits.split_at(digits.len() - 2);
            (h.parse().ok()?, m.parse().ok()?)
        }
        None => (digits.parse().ok()?, 0),
    };
    let hour = match half {
        Some(pm) if (1..=12).contains(&hour) => hour % 12 + if pm { 12 } else { 0 },
        Some(_) => return None,
        None => hour,
    };
    Time::new(hour, minute, 0, 0).ok()
}

/// `c` as an ASCII digit when it is a digit of another script (`৯` is 9).
fn ascii_digit(c: char) -> char {
    // The zeros of the scripts CLDR writes numbers in.
    const ZEROS: [u32; 20] = [
        0x0660, 0x06f0, 0x07c0, 0x0966, 0x09e6, 0x0a66, 0x0ae6, 0x0b66, 0x0be6, 0x0c66, 0x0ce6,
        0x0d66, 0x0de6, 0x0e50, 0x0ed0, 0x0f20, 0x1040, 0x17e0, 0x1810, 0xff10,
    ];
    let n = u32::from(c);
    ZEROS
        .iter()
        .find(|&&zero| (zero..zero + 10).contains(&n))
        .and_then(|zero| char::from_digit(n - zero, 10))
        .unwrap_or(c)
}

/// The 42 days of a month calendar with `month` in it, from the `first`
/// day of the week on or before its first day.
pub(in crate::window) fn month_grid(month: Date, first: Weekday) -> Vec<Date> {
    let start_of_month = month.first_of_month();
    let back = start_of_month.weekday().since(first);
    let start = start_of_month
        .checked_sub(i64::from(back).days())
        .unwrap_or(start_of_month);
    (0..42)
        .filter_map(|i| start.checked_add(i64::from(i).days()).ok())
        .collect()
}

/// `date` at `time` in `tz`, if that moment exists.
pub(in crate::window) fn moment(date: Date, time: Time, tz: &TimeZone) -> Option<Timestamp> {
    DateTime::from_parts(date, time)
        .to_zoned(tz.clone())
        .ok()
        .map(|z| z.timestamp())
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::date;

    #[test]
    fn up_and_down_change_the_part_under_the_cursor() {
        let at = |h, m| clock(Time::constant(h, m, 0, 0));
        let part = |text: &str, part| {
            time_parts(text)
                .into_iter()
                .find(|(p, _)| *p == part)
                .unwrap()
                .1
        };
        let step = |h, m, which, by| {
            let text = at(h, m);
            let range = part(&text, which);
            let (new, cursor) = step_time(&text, range.start, by).unwrap();
            assert_eq!(cursor, part(&new, which).start, "{new}");
            parse_time(&new).unwrap()
        };
        let t = |h, m| Time::constant(h, m, 0, 0);
        let twelve = time_parts(&at(8, 0))
            .iter()
            .any(|(p, _)| *p == TimePart::Half);
        assert_eq!(step(8, 0, TimePart::Hour, 1), t(9, 0));
        assert_eq!(step(8, 0, TimePart::Minute, -1), t(8, 59));
        assert_eq!(step(8, 59, TimePart::Minute, 1), t(8, 0));
        if twelve {
            assert_eq!(step(8, 0, TimePart::Half, 1), t(20, 0));
            assert_eq!(step(20, 0, TimePart::Half, -1), t(8, 0));
            // 12 goes round to 1 in the same half.
            assert_eq!(step(12, 30, TimePart::Hour, 1), t(13, 30));
            assert_eq!(step(0, 30, TimePart::Hour, -1), t(11, 30));
        } else {
            assert_eq!(step(23, 0, TimePart::Hour, 1), t(0, 0));
        }
        // The cursor at the end of the minutes, touching them from the left.
        let text = at(8, 0);
        let end = part(&text, TimePart::Minute).end;
        let (new, cursor) = step_time(&text, end, 1).unwrap();
        assert_eq!(parse_time(&new), Some(t(8, 1)));
        assert_eq!(cursor, part(&new, TimePart::Minute).end);
        assert_eq!(step_time("soon", 1, 1), None);
    }

    #[test]
    fn reads_times() {
        let t = |h, m| Some(Time::constant(h, m, 0, 0));
        assert_eq!(parse_time("8:00 AM"), t(8, 0));
        assert_eq!(parse_time("8am"), t(8, 0));
        assert_eq!(parse_time("12 am"), t(0, 0));
        assert_eq!(parse_time("12:30pm"), t(12, 30));
        assert_eq!(parse_time("1.30 pm"), t(13, 30));
        assert_eq!(parse_time("13:30"), t(13, 30));
        assert_eq!(parse_time("930"), t(9, 30));
        assert_eq!(parse_time("9"), t(9, 0));
        assert_eq!(parse_time("25:00"), None);
        assert_eq!(parse_time("13pm"), None);
        assert_eq!(parse_time("soon"), None);
        assert_eq!(parse_time(&clock(Time::constant(20, 15, 0, 0))), t(20, 15));
        assert_eq!(parse_time("৯:৩০"), t(9, 30));
    }

    #[test]
    fn suggests_times() {
        let tz = TimeZone::UTC;
        // Saturday evening: tomorrow (Sunday) and Monday.
        let now = date(2026, 9, 26)
            .at(19, 0, 0, 0)
            .to_zoned(tz.clone())
            .unwrap();
        let p = presets(&now);
        let labels: Vec<_> = p.iter().map(|p| p.label.as_str()).collect();
        assert_eq!(
            labels,
            ["Tomorrow morning", "Tomorrow afternoon", "Monday morning"]
        );
        assert_eq!(short(&p[2].at), "Sep 28, 8:00\u{202f}AM");
        // Sunday: Monday is tomorrow.
        let now = date(2026, 9, 27)
            .at(19, 0, 0, 0)
            .to_zoned(tz.clone())
            .unwrap();
        assert_eq!(presets(&now).len(), 2);
        // A weekday morning offers this afternoon.
        let now = date(2026, 9, 29).at(9, 0, 0, 0).to_zoned(tz).unwrap();
        assert_eq!(presets(&now)[0].label, "This afternoon");
    }

    #[test]
    fn writes_dates() {
        let at = date(2026, 9, 27)
            .at(8, 0, 0, 0)
            .to_zoned(TimeZone::fixed(jiff::tz::offset(5)))
            .unwrap()
            .timestamp();
        let tz = TimeZone::fixed(jiff::tz::offset(5));
        assert_eq!(rfc2822(at, &tz), "Sun, 27 Sep 2026 08:00:00 +0500");
        assert_eq!(describe(at, &tz), "Sun, Sep 27, 2026, 8:00\u{202f}AM");
        assert_eq!(clock(Time::constant(0, 5, 0, 0)), "12:05\u{202f}AM");
    }

    #[test]
    fn month_grids_start_on_the_first_day_of_the_week() {
        let grid = month_grid(date(2026, 9, 15), Weekday::Sunday);
        assert_eq!(grid.len(), 42);
        assert_eq!(grid[0], date(2026, 8, 30));
        assert_eq!(grid[2], date(2026, 9, 1));
        let grid = month_grid(date(2026, 9, 15), Weekday::Monday);
        assert_eq!(grid[0], date(2026, 8, 31));
        assert_eq!(grid[1], date(2026, 9, 1));
    }
}
