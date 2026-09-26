// SPDX-License-Identifier: GPL-3.0-or-later

//! Dates and sizes as the message list and reading pane show them.

use jiff::Timestamp;
use jiff::civil::DateTime;
use jiff::tz::TimeZone;

/// The local time of `unix` seconds, or `None` if it is out of range.
pub fn local(unix: i64, tz: &TimeZone) -> Option<DateTime> {
    Some(
        Timestamp::from_second(unix)
            .ok()?
            .to_zoned(tz.clone())
            .datetime(),
    )
}

/// Short date for the message list, relative to `now` (both local):
/// the time today, the weekday within the last six days, day and month
/// this year, otherwise the full date.
pub fn list_date(date: DateTime, now: DateTime) -> String {
    let days = (now.date() - date.date()).get_days();
    if date.date() == now.date() {
        date.strftime("%H:%M").to_string()
    } else if (1..7).contains(&days) {
        date.strftime("%a").to_string()
    } else if date.year() == now.year() && date <= now {
        date.strftime("%b %-d").to_string()
    } else {
        date.strftime("%Y-%m-%d").to_string()
    }
}

/// Full date for the reading pane.
pub fn long_date(date: DateTime) -> String {
    date.strftime("%a, %-d %b %Y, %H:%M").to_string()
}

/// How long ago `then` was, both in Unix seconds, as the reading pane
/// shows it next to the date: `2 hours ago`. `None` after a week, or when
/// `then` is in the future.
pub fn ago(then: i64, now: i64) -> Option<String> {
    let secs = now.checked_sub(then).filter(|s| *s >= 0)?;
    let plural = |n: i64, unit: &str| {
        if n == 1 {
            format!("1 {unit} ago")
        } else {
            format!("{n} {unit}s ago")
        }
    };
    Some(match secs {
        0..60 => "just now".to_owned(),
        60..3600 => plural(secs / 60, "minute"),
        3600..86_400 => plural(secs / 3600, "hour"),
        86_400..604_800 => plural(secs / 86_400, "day"),
        _ => return None,
    })
}

/// `n` with thousands separators: `1,234,567`.
pub fn thousands(n: u64) -> String {
    let digits = n.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}

/// A byte size such as `12 KB`, as file managers show it (powers of 1000).
pub fn size(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["KB", "MB", "GB", "TB"];
    if bytes < 1000 {
        return format!("{bytes} bytes");
    }
    let mut value = bytes as f64 / 1000.0;
    let mut unit = 0;
    while value >= 999.95 && unit + 1 < UNITS.len() {
        value /= 1000.0;
        unit += 1;
    }
    if value < 10.0 {
        format!("{value:.1} {}", UNITS[unit])
    } else {
        format!("{value:.0} {}", UNITS[unit])
    }
}

/// A reason from the daemon (lower case, no full stop) as a sentence.
pub fn sentence(reason: &str) -> String {
    let reason = reason.trim().trim_end_matches('.');
    let mut chars = reason.chars();
    match chars.next() {
        Some(first) => format!("{}{}.", first.to_uppercase(), chars.as_str()),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use jiff::civil::date;

    use super::*;

    #[test]
    fn how_long_ago() {
        assert_eq!(ago(100, 110).as_deref(), Some("just now"));
        assert_eq!(ago(0, 60).as_deref(), Some("1 minute ago"));
        assert_eq!(ago(0, 7300).as_deref(), Some("2 hours ago"));
        assert_eq!(ago(0, 3 * 86_400).as_deref(), Some("3 days ago"));
        assert_eq!(ago(0, 8 * 86_400), None);
        assert_eq!(ago(10, 0), None);
    }

    #[test]
    fn list_dates() {
        let now = date(2026, 9, 26).at(15, 30, 0, 0);
        let fmt = |d: DateTime| list_date(d, now);
        assert_eq!(fmt(date(2026, 9, 26).at(9, 5, 0, 0)), "09:05");
        assert_eq!(fmt(date(2026, 9, 25).at(23, 0, 0, 0)), "Fri");
        assert_eq!(fmt(date(2026, 9, 20).at(8, 0, 0, 0)), "Sun");
        assert_eq!(fmt(date(2026, 9, 19).at(8, 0, 0, 0)), "Sep 19");
        assert_eq!(fmt(date(2026, 1, 2).at(8, 0, 0, 0)), "Jan 2");
        assert_eq!(fmt(date(2001, 5, 14).at(8, 0, 0, 0)), "2001-05-14");
        // Dates in the future (wrong clocks) show in full.
        assert_eq!(fmt(date(2026, 12, 1).at(8, 0, 0, 0)), "2026-12-01");
    }

    #[test]
    fn long_dates() {
        assert_eq!(
            long_date(date(2001, 5, 14).at(16, 39, 0, 0)),
            "Mon, 14 May 2001, 16:39"
        );
    }

    #[test]
    fn local_time_in_a_zone() {
        let tz = TimeZone::fixed(jiff::tz::offset(2));
        assert_eq!(
            local(1_788_249_600, &tz),
            Some(date(2026, 9, 1).at(10, 0, 0, 0))
        );
        assert_eq!(local(i64::MAX, &tz), None);
    }

    #[test]
    fn thousands_separators() {
        assert_eq!(thousands(0), "0");
        assert_eq!(thousands(999), "999");
        assert_eq!(thousands(1_000), "1,000");
        assert_eq!(thousands(517_401), "517,401");
        assert_eq!(thousands(1_234_567), "1,234,567");
    }

    #[test]
    fn sizes() {
        assert_eq!(size(0), "0 bytes");
        assert_eq!(size(999), "999 bytes");
        assert_eq!(size(1_000), "1.0 KB");
        assert_eq!(size(12_345), "12 KB");
        assert_eq!(size(999_999), "1.0 MB");
        assert_eq!(size(5_300_000), "5.3 MB");
    }

    #[test]
    fn reasons_read_as_sentences() {
        assert_eq!(
            sentence("\u{201c}Work\u{201d} already exists"),
            "\u{201c}Work\u{201d} already exists."
        );
        assert_eq!(sentence("the name is empty."), "The name is empty.");
        assert_eq!(sentence(" "), "");
    }
}
