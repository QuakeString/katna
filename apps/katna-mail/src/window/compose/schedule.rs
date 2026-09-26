// SPDX-License-Identifier: GPL-3.0-or-later

//! The times of schedule send: the suggested ones, the calendar of the
//! date and time picker, and how the chosen time is written. The
//! background service holds the message until then, so it goes out with
//! the app closed.

use jiff::civil::{Date, DateTime, Time, Weekday};
use jiff::tz::TimeZone;
use jiff::{Timestamp, ToSpan, Zoned};

/// A suggested time: its name and when.
pub(super) struct Preset {
    pub label: &'static str,
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
            label: "This morning",
            at,
        }));
    } else if now.hour() < 12 {
        presets.extend(at(today, 13).map(|at| Preset {
            label: "This afternoon",
            at,
        }));
    }
    presets.extend(at(tomorrow, 8).map(|at| Preset {
        label: "Tomorrow morning",
        at,
    }));
    presets.extend(at(tomorrow, 13).map(|at| Preset {
        label: "Tomorrow afternoon",
        at,
    }));
    let monday = today.nth_weekday(1, Weekday::Monday).ok();
    if let Some(monday) = monday.filter(|m| *m != tomorrow) {
        presets.extend(at(monday, 8).map(|at| Preset {
            label: "Monday morning",
            at,
        }));
    }
    presets.truncate(3);
    presets
}

/// When a preset or scheduled message goes out, as the menu shows it:
/// "Sep 27, 8:00 AM".
pub(super) fn short(at: &Zoned) -> String {
    format!("{}, {}", at.strftime("%b %-d"), clock(at.time()))
}

/// `at` in the user's zone for the snackbar: "Sun, Sep 27, 8:00 AM".
pub(super) fn describe(at: Timestamp, tz: &TimeZone) -> String {
    let at = at.to_zoned(tz.clone());
    format!("{}, {}", at.strftime("%a, %b %-d"), clock(at.time()))
}

/// "8:00 AM".
pub(super) fn clock(time: Time) -> String {
    let (hour, half) = match time.hour() {
        0 => (12, "AM"),
        h @ 1..=11 => (h, "AM"),
        12 => (12, "PM"),
        h => (h - 12, "PM"),
    };
    format!("{hour}:{:02} {half}", time.minute())
}

/// The `Date` header of a message sent at `at`, in the user's zone.
pub(super) fn rfc2822(at: Timestamp, tz: &TimeZone) -> String {
    at.to_zoned(tz.clone())
        .strftime("%a, %d %b %Y %H:%M:%S %z")
        .to_string()
}

/// Reads a time of day as people type it: "8:00 AM", "8am", "13:30",
/// "1.30 pm", "9".
pub(super) fn parse_time(text: &str) -> Option<Time> {
    let text = text.trim().to_ascii_lowercase().replace(' ', "");
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

/// The 42 days of a month calendar with `month` in it, from the Sunday on
/// or before its first day.
pub(super) fn month_grid(month: Date) -> Vec<Date> {
    let first = month.first_of_month();
    let back = first.weekday().to_sunday_zero_offset();
    let start = first.checked_sub(i64::from(back).days()).unwrap_or(first);
    (0..42)
        .filter_map(|i| start.checked_add(i64::from(i).days()).ok())
        .collect()
}

/// `date` at `time` in `tz`, if that moment exists.
pub(super) fn moment(date: Date, time: Time, tz: &TimeZone) -> Option<Timestamp> {
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
        let labels: Vec<_> = p.iter().map(|p| p.label).collect();
        assert_eq!(
            labels,
            ["Tomorrow morning", "Tomorrow afternoon", "Monday morning"]
        );
        assert_eq!(short(&p[2].at), "Sep 28, 8:00 AM");
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
        assert_eq!(describe(at, &tz), "Sun, Sep 27, 8:00 AM");
        assert_eq!(clock(Time::constant(0, 5, 0, 0)), "12:05 AM");
    }

    #[test]
    fn month_grids_start_on_sunday() {
        let grid = month_grid(date(2026, 9, 15));
        assert_eq!(grid.len(), 42);
        assert_eq!(grid[0], date(2026, 8, 30));
        assert_eq!(grid[2], date(2026, 9, 1));
    }
}
