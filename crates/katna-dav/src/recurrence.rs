// SPDX-License-Identifier: GPL-3.0-or-later

//! Repeating events (RFC 5545 §3.3.10 `RRULE`): the starts a rule gives in
//! a range, in the event's own time zone, so a 9:00 meeting stays at 9:00
//! across daylight saving. `FREQ` of `DAILY`, `WEEKLY`, `MONTHLY` and
//! `YEARLY` with `INTERVAL`, `COUNT`, `UNTIL`, `BYDAY` (with ordinals),
//! `BYMONTHDAY`, `BYMONTH`, `BYSETPOS` and `WKST`: what Google, Outlook
//! and CalDAV servers write. Rules repeating more often than daily, and
//! parts nobody writes (`BYWEEKNO`, `BYYEARDAY`, `BYHOUR`…), are not
//! expanded: such an event shows once.

use jiff::civil::{Date, DateTime, Weekday};
use jiff::tz::TimeZone;
use jiff::{Timestamp, ToSpan};

/// How often a rule repeats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Freq {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

/// A parsed `RRULE` value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub freq: Freq,
    pub interval: u32,
    pub count: Option<u32>,
    /// The last start allowed, as an instant.
    pub until: Option<Timestamp>,
    /// Weekdays, with an ordinal (`2MO`, `-1FR`) or `0` for every one.
    pub by_day: Vec<(i8, Weekday)>,
    pub by_month_day: Vec<i8>,
    pub by_month: Vec<i8>,
    pub by_set_pos: Vec<i32>,
    pub week_start: Weekday,
}

/// Rules starting this long ago still expand; past this many periods the
/// expansion stops, so a broken rule can't hang the app.
const MAX_PERIODS: usize = 100_000;
/// At most this many starts from one rule in one range.
const MAX_STARTS: usize = 5_000;

fn weekday(text: &str) -> Option<Weekday> {
    Some(match text {
        "MO" => Weekday::Monday,
        "TU" => Weekday::Tuesday,
        "WE" => Weekday::Wednesday,
        "TH" => Weekday::Thursday,
        "FR" => Weekday::Friday,
        "SA" => Weekday::Saturday,
        "SU" => Weekday::Sunday,
        _ => return None,
    })
}

/// The `UNTIL` of a rule: a date (the whole day counts, in `tz`), a UTC
/// time (`…Z`) or a floating time in `tz`.
fn parse_until(text: &str, tz: &TimeZone) -> Option<Timestamp> {
    let digits = |s: &str| s.bytes().all(|b| b.is_ascii_digit());
    let date = |s: &str| -> Option<Date> {
        if s.len() != 8 || !digits(s) {
            return None;
        }
        Date::new(
            s[..4].parse().ok()?,
            s[4..6].parse().ok()?,
            s[6..8].parse().ok()?,
        )
        .ok()
    };
    if let Some((d, t)) = text.split_once('T') {
        let day = date(d)?;
        let utc = t.ends_with('Z');
        let t = t.trim_end_matches('Z');
        if t.len() != 6 || !digits(t) {
            return None;
        }
        let time = jiff::civil::Time::new(
            t[..2].parse().ok()?,
            t[2..4].parse().ok()?,
            t[4..6].parse().ok()?,
            0,
        )
        .ok()?;
        let dt = day.to_datetime(time);
        if utc {
            dt.to_zoned(TimeZone::UTC).ok().map(|z| z.timestamp())
        } else {
            dt.to_zoned(tz.clone()).ok().map(|z| z.timestamp())
        }
    } else {
        // A date: every start on that day is still in.
        let day = date(text)?;
        let end = day
            .tomorrow()
            .ok()?
            .to_datetime(jiff::civil::Time::midnight());
        end.to_zoned(tz.clone())
            .ok()
            .map(|z| z.timestamp() - 1.second())
    }
}

impl Rule {
    /// Parses an `RRULE` value (without `RRULE:`); `tz` reads a floating
    /// `UNTIL`. `None` for rules that don't repeat daily or slower.
    pub fn parse(text: &str, tz: &TimeZone) -> Option<Self> {
        let text = text.trim();
        let text = text.strip_prefix("RRULE:").unwrap_or(text);
        let mut rule = Rule {
            freq: Freq::Daily,
            interval: 1,
            count: None,
            until: None,
            by_day: Vec::new(),
            by_month_day: Vec::new(),
            by_month: Vec::new(),
            by_set_pos: Vec::new(),
            week_start: Weekday::Monday,
        };
        let mut freq = None;
        for part in text.split(';').filter(|p| !p.is_empty()) {
            let (key, value) = part.split_once('=')?;
            let list = || value.split(',').map(str::trim);
            match key.trim().to_ascii_uppercase().as_str() {
                "FREQ" => {
                    freq = Some(match value.trim().to_ascii_uppercase().as_str() {
                        "DAILY" => Freq::Daily,
                        "WEEKLY" => Freq::Weekly,
                        "MONTHLY" => Freq::Monthly,
                        "YEARLY" => Freq::Yearly,
                        _ => return None,
                    });
                }
                "INTERVAL" => rule.interval = value.trim().parse().ok().filter(|&n| n > 0)?,
                "COUNT" => rule.count = Some(value.trim().parse().ok()?),
                "UNTIL" => rule.until = Some(parse_until(value.trim(), tz)?),
                "BYDAY" => {
                    for day in list() {
                        let day = day.to_ascii_uppercase();
                        let split = day.len().checked_sub(2)?;
                        let (n, name) = day.split_at(split);
                        let n: i8 = if n.is_empty() {
                            0
                        } else {
                            n.trim_start_matches('+').parse().ok()?
                        };
                        rule.by_day.push((n, weekday(name)?));
                    }
                }
                "BYMONTHDAY" => {
                    for d in list() {
                        let d: i8 = d.parse().ok()?;
                        if d == 0 || !(-31..=31).contains(&d) {
                            return None;
                        }
                        rule.by_month_day.push(d);
                    }
                }
                "BYMONTH" => {
                    for m in list() {
                        let m: i8 = m.parse().ok()?;
                        if !(1..=12).contains(&m) {
                            return None;
                        }
                        rule.by_month.push(m);
                    }
                }
                "BYSETPOS" => {
                    for p in list() {
                        rule.by_set_pos.push(p.parse().ok()?);
                    }
                }
                "WKST" => rule.week_start = weekday(&value.trim().to_ascii_uppercase())?,
                // Finer than a day: not expanded.
                "BYHOUR" | "BYMINUTE" | "BYSECOND" | "BYWEEKNO" | "BYYEARDAY" => return None,
                _ => {}
            }
        }
        rule.freq = freq?;
        Some(rule)
    }

    /// The days of the period that `anchor` starts, as the rule picks
    /// them, sorted. `first` is the series' first day.
    fn days(&self, anchor: Date, first: Date) -> Vec<Date> {
        let mut days = match self.freq {
            Freq::Daily => vec![anchor],
            Freq::Weekly => {
                let days: Vec<Date> = (0..7)
                    .filter_map(|i| anchor.checked_add(i.days()).ok())
                    .collect();
                if self.by_day.is_empty() {
                    days.into_iter()
                        .filter(|d| d.weekday() == first.weekday())
                        .collect()
                } else {
                    days.into_iter()
                        .filter(|d| self.by_day.iter().any(|&(_, w)| w == d.weekday()))
                        .collect()
                }
            }
            Freq::Monthly => self.month_days(anchor, first),
            Freq::Yearly => {
                if self.by_month.is_empty()
                    && self.by_month_day.is_empty()
                    && self.by_day.iter().any(|&(n, _)| n != 0)
                {
                    // `BYDAY=20MO` in a year: the nth of the year.
                    let year = year_days(anchor.year());
                    pick_weekdays(&year, &self.by_day)
                } else {
                    let months: Vec<i8> = if self.by_month.is_empty() {
                        vec![first.month()]
                    } else {
                        self.by_month.clone()
                    };
                    let mut days = Vec::new();
                    for month in months {
                        if let Ok(start) = Date::new(anchor.year(), month, 1) {
                            if self.by_day.is_empty() && self.by_month_day.is_empty() {
                                if let Ok(day) = Date::new(anchor.year(), month, first.day()) {
                                    days.push(day);
                                }
                            } else {
                                days.extend(self.month_days(start, first));
                            }
                        }
                    }
                    days
                }
            }
        };
        if !self.by_month.is_empty() && self.freq != Freq::Yearly {
            days.retain(|d| self.by_month.contains(&d.month()));
        }
        if self.freq == Freq::Daily {
            if !self.by_month_day.is_empty() {
                days.retain(|d| month_day_matches(*d, &self.by_month_day));
            }
            if !self.by_day.is_empty() {
                days.retain(|d| self.by_day.iter().any(|&(_, w)| w == d.weekday()));
            }
        }
        days.sort();
        days.dedup();
        if !self.by_set_pos.is_empty() {
            let len = days.len() as i32;
            let mut picked: Vec<Date> = self
                .by_set_pos
                .iter()
                .filter_map(|&p| {
                    let ix = if p > 0 { p - 1 } else { len + p };
                    (0..len).contains(&ix).then(|| days[ix as usize])
                })
                .collect();
            picked.sort();
            picked.dedup();
            days = picked;
        }
        days
    }

    /// The days of `month_start`'s month the rule picks.
    fn month_days(&self, month_start: Date, first: Date) -> Vec<Date> {
        let all: Vec<Date> = (0..month_start.days_in_month())
            .filter_map(|i| month_start.checked_add(i64::from(i).days()).ok())
            .collect();
        match (self.by_day.is_empty(), self.by_month_day.is_empty()) {
            (true, true) => all.into_iter().filter(|d| d.day() == first.day()).collect(),
            (true, false) => all
                .into_iter()
                .filter(|d| month_day_matches(*d, &self.by_month_day))
                .collect(),
            (false, true) => pick_weekdays(&all, &self.by_day),
            (false, false) => pick_weekdays(&all, &self.by_day)
                .into_iter()
                .filter(|d| month_day_matches(*d, &self.by_month_day))
                .collect(),
        }
    }

    /// The first day of the period after `anchor`'s, `steps` periods on.
    fn next_anchor(&self, anchor: Date, steps: i64) -> Option<Date> {
        let n = i64::from(self.interval) * steps;
        match self.freq {
            Freq::Daily => anchor.checked_add(n.days()).ok(),
            Freq::Weekly => anchor.checked_add((7 * n).days()).ok(),
            Freq::Monthly => anchor.checked_add(n.months()).ok(),
            Freq::Yearly => anchor.checked_add(n.years()).ok(),
        }
    }

    /// The first day of the period `first` is in.
    fn first_anchor(&self, first: Date) -> Date {
        match self.freq {
            Freq::Daily => first,
            Freq::Weekly => {
                let back = (first.weekday().to_monday_zero_offset()
                    - self.week_start.to_monday_zero_offset())
                .rem_euclid(7);
                first.checked_sub(i64::from(back).days()).unwrap_or(first)
            }
            Freq::Monthly => first.first_of_month(),
            Freq::Yearly => first.first_of_year(),
        }
    }
}

fn year_days(year: i16) -> Vec<Date> {
    let Ok(start) = Date::new(year, 1, 1) else {
        return Vec::new();
    };
    (0..start.days_in_year())
        .filter_map(|i| start.checked_add(i64::from(i).days()).ok())
        .collect()
}

fn month_day_matches(day: Date, wanted: &[i8]) -> bool {
    let len = day.days_in_month();
    wanted.iter().any(|&w| {
        if w > 0 {
            day.day() == w
        } else {
            day.day() == len + 1 + w
        }
    })
}

/// The days of `days` (one month or year, in order) that `by_day` picks:
/// every such weekday, or its nth from the start or end.
fn pick_weekdays(days: &[Date], by_day: &[(i8, Weekday)]) -> Vec<Date> {
    let mut out = Vec::new();
    for &(n, weekday) in by_day {
        let matching: Vec<Date> = days
            .iter()
            .copied()
            .filter(|d| d.weekday() == weekday)
            .collect();
        if n == 0 {
            out.extend(matching);
        } else {
            let len = matching.len() as i32;
            let ix = if n > 0 {
                i32::from(n) - 1
            } else {
                len + i32::from(n)
            };
            if (0..len).contains(&ix) {
                out.push(matching[ix as usize]);
            }
        }
    }
    out.sort();
    out
}

/// The starts of a series whose first start is `first` (in zone `tz`,
/// where its wall-clock time stays the same), that fall in `from..to`.
/// The first start always counts, as RFC 5545 says. `exdates` are
/// skipped; `rdates` added.
pub fn starts(
    rule: &Rule,
    first: Timestamp,
    tz: &TimeZone,
    from: Timestamp,
    to: Timestamp,
    exdates: &[Timestamp],
    rdates: &[Timestamp],
) -> Vec<Timestamp> {
    let local = first.to_zoned(tz.clone());
    let first_day = local.date();
    let time = local.time();
    let at = |day: Date| -> Option<Timestamp> {
        DateTime::from_parts(day, time)
            .to_zoned(tz.clone())
            .ok()
            .map(|z| z.timestamp())
    };
    let mut out = Vec::new();
    let mut count = 0u32;
    let mut push = |start: Timestamp, out: &mut Vec<Timestamp>| -> bool {
        count += 1;
        if rule.count.is_some_and(|max| count > max) {
            return false;
        }
        if start >= from && start < to && !exdates.contains(&start) {
            out.push(start);
        }
        out.len() < MAX_STARTS
    };
    let _ = push(first, &mut out);
    let mut anchor = rule.first_anchor(first_day);
    'periods: for _ in 0..MAX_PERIODS {
        for day in rule.days(anchor, first_day) {
            if day <= first_day {
                continue;
            }
            let Some(start) = at(day) else { continue };
            if rule.until.is_some_and(|until| start > until) || start >= to {
                break 'periods;
            }
            if !push(start, &mut out) {
                break 'periods;
            }
        }
        match rule.next_anchor(anchor, 1) {
            Some(next) => anchor = next,
            None => break,
        }
    }
    for &extra in rdates {
        if extra >= from && extra < to && !out.contains(&extra) && !exdates.contains(&extra) {
            out.push(extra);
        }
    }
    out.sort();
    out
}

/// Where a series ends: the last start, if the rule ends at all (a
/// `COUNT` or an `UNTIL`); `None` when it runs forever or can't be read.
pub fn last_start(rule: &Rule, first: Timestamp, tz: &TimeZone) -> Option<Timestamp> {
    if rule.count.is_none() && rule.until.is_none() {
        return None;
    }
    let end = rule.until.map_or(Timestamp::MAX, |until| {
        until.checked_add(1.second()).unwrap_or(until)
    });
    starts(rule, first, tz, first, end, &[], &[])
        .last()
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kolkata() -> TimeZone {
        TimeZone::get("Asia/Kolkata").unwrap()
    }

    fn berlin() -> TimeZone {
        TimeZone::get("Europe/Berlin").unwrap()
    }

    fn at(text: &str, tz: &TimeZone) -> Timestamp {
        text.parse::<DateTime>()
            .unwrap()
            .to_zoned(tz.clone())
            .unwrap()
            .timestamp()
    }

    fn days(rule: &str, first: &str, from: &str, to: &str, tz: &TimeZone) -> Vec<String> {
        let rule = Rule::parse(rule, tz).expect("rule");
        starts(&rule, at(first, tz), tz, at(from, tz), at(to, tz), &[], &[])
            .into_iter()
            .map(|t| t.to_zoned(tz.clone()).datetime().to_string())
            .collect()
    }

    #[test]
    fn weekly_on_weekdays() {
        let tz = kolkata();
        assert_eq!(
            days(
                "FREQ=WEEKLY;BYDAY=MO,WE,FR",
                "2026-09-28T09:00",
                "2026-09-28T00:00",
                "2026-10-05T00:00",
                &tz
            ),
            [
                "2026-09-28T09:00:00",
                "2026-09-30T09:00:00",
                "2026-10-02T09:00:00"
            ]
        );
    }

    #[test]
    fn keeps_the_wall_clock_across_daylight_saving() {
        let tz = berlin();
        // Summer time ends on 25 October 2026.
        let got = days(
            "FREQ=WEEKLY",
            "2026-10-19T09:00",
            "2026-10-01T00:00",
            "2026-11-01T00:00",
            &tz,
        );
        assert_eq!(got, ["2026-10-19T09:00:00", "2026-10-26T09:00:00"]);
    }

    #[test]
    fn count_and_until_end_a_series() {
        let tz = kolkata();
        assert_eq!(
            days(
                "FREQ=DAILY;COUNT=3",
                "2026-09-28T09:00",
                "2026-01-01T00:00",
                "2027-01-01T00:00",
                &tz
            )
            .len(),
            3
        );
        assert_eq!(
            days(
                "FREQ=DAILY;UNTIL=20260930",
                "2026-09-28T09:00",
                "2026-01-01T00:00",
                "2027-01-01T00:00",
                &tz
            ),
            [
                "2026-09-28T09:00:00",
                "2026-09-29T09:00:00",
                "2026-09-30T09:00:00"
            ]
        );
        // 03:30 UTC is 09:00 in Kolkata: the last start is included.
        assert_eq!(
            days(
                "FREQ=DAILY;UNTIL=20260929T033000Z",
                "2026-09-28T09:00",
                "2026-01-01T00:00",
                "2027-01-01T00:00",
                &tz
            )
            .len(),
            2
        );
    }

    #[test]
    fn count_counts_from_the_first_start_even_before_the_range() {
        let tz = kolkata();
        assert_eq!(
            days(
                "FREQ=WEEKLY;COUNT=3",
                "2026-09-01T10:00",
                "2026-09-10T00:00",
                "2026-12-01T00:00",
                &tz
            ),
            ["2026-09-15T10:00:00"]
        );
    }

    #[test]
    fn monthly_by_ordinal_weekday_and_last_day() {
        let tz = kolkata();
        assert_eq!(
            days(
                "FREQ=MONTHLY;BYDAY=2TU",
                "2026-09-08T15:00",
                "2026-09-01T00:00",
                "2026-12-01T00:00",
                &tz
            ),
            [
                "2026-09-08T15:00:00",
                "2026-10-13T15:00:00",
                "2026-11-10T15:00:00"
            ]
        );
        assert_eq!(
            days(
                "FREQ=MONTHLY;BYDAY=-1FR",
                "2026-09-25T15:00",
                "2026-09-01T00:00",
                "2026-11-01T00:00",
                &tz
            ),
            ["2026-09-25T15:00:00", "2026-10-30T15:00:00"]
        );
        assert_eq!(
            days(
                "FREQ=MONTHLY;BYMONTHDAY=-1",
                "2026-01-31T08:00",
                "2026-01-01T00:00",
                "2026-04-01T00:00",
                &tz
            ),
            [
                "2026-01-31T08:00:00",
                "2026-02-28T08:00:00",
                "2026-03-31T08:00:00"
            ]
        );
        // Plain monthly on the 31st skips short months.
        assert_eq!(
            days(
                "FREQ=MONTHLY",
                "2026-01-31T08:00",
                "2026-01-01T00:00",
                "2026-04-01T00:00",
                &tz
            ),
            ["2026-01-31T08:00:00", "2026-03-31T08:00:00"]
        );
    }

    #[test]
    fn last_workday_of_the_month_by_set_position() {
        let tz = kolkata();
        assert_eq!(
            days(
                "FREQ=MONTHLY;BYDAY=MO,TU,WE,TH,FR;BYSETPOS=-1",
                "2026-09-30T17:00",
                "2026-09-01T00:00",
                "2026-11-01T00:00",
                &tz
            ),
            ["2026-09-30T17:00:00", "2026-10-30T17:00:00"]
        );
    }

    #[test]
    fn yearly_birthdays_and_every_other_week() {
        let tz = TimeZone::UTC;
        assert_eq!(
            days(
                "FREQ=YEARLY",
                "2024-02-29T00:00",
                "2024-01-01T00:00",
                "2029-01-01T00:00",
                &tz
            ),
            ["2024-02-29T00:00:00", "2028-02-29T00:00:00"]
        );
        assert_eq!(
            days(
                "FREQ=YEARLY;BYMONTH=11;BYDAY=4TH",
                "2026-11-26T00:00",
                "2026-01-01T00:00",
                "2028-01-01T00:00",
                &tz
            ),
            ["2026-11-26T00:00:00", "2027-11-25T00:00:00"]
        );
        assert_eq!(
            days(
                "FREQ=WEEKLY;INTERVAL=2;BYDAY=TU,TH;WKST=SU",
                "2026-09-01T10:00",
                "2026-09-01T00:00",
                "2026-09-30T00:00",
                &tz
            ),
            [
                "2026-09-01T10:00:00",
                "2026-09-03T10:00:00",
                "2026-09-15T10:00:00",
                "2026-09-17T10:00:00",
                "2026-09-29T10:00:00"
            ]
        );
    }

    #[test]
    fn skipped_and_extra_starts() {
        let tz = kolkata();
        let rule = Rule::parse("FREQ=DAILY;COUNT=4", &tz).unwrap();
        let first = at("2026-09-28T09:00", &tz);
        let skip = at("2026-09-29T09:00", &tz);
        let extra = at("2026-10-10T18:00", &tz);
        let got = starts(
            &rule,
            first,
            &tz,
            first,
            at("2026-12-01T00:00", &tz),
            &[skip],
            &[extra],
        );
        assert_eq!(got.len(), 4);
        assert!(!got.contains(&skip));
        assert_eq!(got.last(), Some(&extra));
    }

    #[test]
    fn last_start_of_ending_rules() {
        let tz = kolkata();
        let first = at("2026-09-28T09:00", &tz);
        let rule = Rule::parse("FREQ=WEEKLY;COUNT=3", &tz).unwrap();
        assert_eq!(
            last_start(&rule, first, &tz),
            Some(at("2026-10-12T09:00", &tz))
        );
        let forever = Rule::parse("FREQ=WEEKLY", &tz).unwrap();
        assert_eq!(last_start(&forever, first, &tz), None);
    }

    #[test]
    fn rules_it_does_not_expand() {
        let tz = kolkata();
        assert!(Rule::parse("FREQ=HOURLY", &tz).is_none());
        assert!(Rule::parse("FREQ=DAILY;BYHOUR=9,17", &tz).is_none());
        assert!(Rule::parse("INTERVAL=2", &tz).is_none());
        assert!(Rule::parse("FREQ=WEEKLY;BYDAY=XX", &tz).is_none());
        assert!(Rule::parse("RRULE:FREQ=WEEKLY;X-NAME=1", &tz).is_some());
    }
}
