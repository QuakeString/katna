// SPDX-License-Identifier: GPL-3.0-or-later

//! Share free times: the gaps between busy events in the next working
//! days, written into a new message, since Katna has no booking pages
//! (`docs/ARCHITECTURE.md` §18).

use gpui::{Context, Window};
use jiff::ToSpan;
use jiff::civil::{Date, Time, Weekday};
use jiff::tz::TimeZone;
use jiff::{Zoned, civil::DateTime};
use katna_dav::Occurrence;
use katna_i18n::{format, tr};
use katna_store::calendar::EventStatus;

use super::{MailWindow, civil, midnight, read};

/// Working days shared.
const DAYS: usize = 5;
/// Working hours.
const DAY_START: Time = Time::constant(9, 0, 0, 0);
const DAY_END: Time = Time::constant(17, 0, 0, 0);
/// A gap shorter than this isn't offered.
const SHORTEST: i64 = 30 * 60;

/// The free times of the next working days from `now`: each day with its
/// gaps (Unix seconds) between `busy` spans, within working hours.
fn free_times(busy: &[(i64, i64)], now: &Zoned, tz: &TimeZone) -> Vec<(Date, Vec<(i64, i64)>)> {
    let mut days = Vec::new();
    let mut day = now.date();
    while days.len() < DAYS {
        if !matches!(day.weekday(), Weekday::Saturday | Weekday::Sunday) {
            let at = |time: Time| {
                day.to_datetime(time)
                    .to_zoned(tz.clone())
                    .map(|z| z.timestamp().as_second())
                    .ok()
            };
            if let (Some(open), Some(close)) = (at(DAY_START), at(DAY_END)) {
                // Today from the next half hour.
                let now = now.timestamp().as_second();
                let mut from = open.max((now + 30 * 60 - 1) / (30 * 60) * (30 * 60));
                let mut gaps = Vec::new();
                let mut spans: Vec<(i64, i64)> = busy
                    .iter()
                    .copied()
                    .filter(|&(s, e)| s < close && e > open)
                    .collect();
                spans.sort_unstable();
                for (start, end) in spans {
                    if start - from >= SHORTEST {
                        gaps.push((from, start));
                    }
                    from = from.max(end);
                }
                if close - from >= SHORTEST {
                    gaps.push((from, close));
                }
                days.push((day, gaps));
            }
        }
        match day.tomorrow() {
            Ok(next) => day = next,
            Err(_) => break,
        }
    }
    days.retain(|(_, gaps)| !gaps.is_empty());
    days
}

/// The spans the user is busy in: events of shown calendars marked busy,
/// not cancelled and not declined, and whole-day ones only when busy.
fn busy_spans(occurrences: &[Occurrence], hidden: impl Fn(i64) -> bool) -> Vec<(i64, i64)> {
    occurrences
        .iter()
        .filter(|o| {
            let data = &o.event.data;
            !hidden(o.event.calendar_id)
                && data.busy
                && data.status != EventStatus::Cancelled
                && data.self_status != "declined"
        })
        .map(|o| (o.start, o.end))
        .collect()
}

impl MailWindow {
    /// Opens a new message listing the free times of the next working
    /// days.
    pub(super) fn share_free_times(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.menu = None;
        let tz = self.tz.clone();
        let now = Zoned::now().with_time_zone(tz.clone());
        let from = midnight(now.date(), &tz);
        let to = midnight(now.date().checked_add(14.days()).unwrap_or(now.date()), &tz);
        let occurrences = match read(&self.paths, from, to, &tz, false, &self.calendar_left_out()) {
            Ok((_, occurrences)) => occurrences,
            Err(err) => {
                tracing::warn!(%err, "reading the calendar for free times failed");
                return;
            }
        };
        let busy = busy_spans(&occurrences, |id| self.calendar_hidden(id));
        let days = free_times(&busy, &now, &tz);
        let time = |at: i64| format::time(civil(at, &tz));
        let mut lines = vec![tr!("calendar-free-intro", zone = super::gmt(&now))];
        lines.push(String::new());
        for (day, gaps) in &days {
            let date: DateTime = day.to_datetime(Time::midnight());
            let times = gaps
                .iter()
                .map(|&(start, end)| {
                    tr!("calendar-free-range", start = time(start), end = time(end))
                })
                .collect::<Vec<_>>()
                .join(", ");
            lines.push(tr!(
                "calendar-free-day",
                weekday = format::weekday(date),
                date = format::day_month(date),
                times = times
            ));
        }
        if days.is_empty() {
            lines = vec![tr!("calendar-free-none")];
        }
        self.open_mailto(
            crate::mailto::Mailto {
                subject: tr!("calendar-free-subject"),
                body: lines.join("\n"),
                ..Default::default()
            },
            window,
            cx,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::date;

    #[test]
    fn gaps_between_busy_times_in_working_hours() {
        let tz = TimeZone::UTC;
        let at = |m: i8, d: i8, h: i8, min: i8| {
            date(2026, m, d)
                .at(h, min, 0, 0)
                .to_zoned(TimeZone::UTC)
                .unwrap()
                .timestamp()
                .as_second()
        };
        // Tuesday 29 September 2026, 10:10.
        let now = date(2026, 9, 29)
            .at(10, 10, 0, 0)
            .to_zoned(tz.clone())
            .unwrap();
        let busy = [
            (at(9, 29, 11, 0), at(9, 29, 12, 0)),
            // Leaves 20 minutes: not offered.
            (at(9, 29, 12, 20), at(9, 29, 16, 50)),
            (at(9, 30, 9, 0), at(9, 30, 17, 0)),
            (at(10, 1, 13, 0), at(10, 1, 14, 0)),
        ];
        let days = free_times(&busy, &now, &tz);
        // Five working days; Wednesday is full.
        let listed: Vec<Date> = days.iter().map(|(d, _)| *d).collect();
        assert_eq!(
            listed,
            [
                date(2026, 9, 29),
                date(2026, 10, 1),
                date(2026, 10, 2),
                date(2026, 10, 5)
            ]
        );
        // Today from the next half hour.
        assert_eq!(days[0].1, [(at(9, 29, 10, 30), at(9, 29, 11, 0))]);
        assert_eq!(
            days[1].1,
            [
                (at(10, 1, 9, 0), at(10, 1, 13, 0)),
                (at(10, 1, 14, 0), at(10, 1, 17, 0))
            ]
        );
    }
}
