// SPDX-License-Identifier: GPL-3.0-or-later

//! The occurrences of stored events in a range: what the Calendar page,
//! the agenda beside the inbox and the desktop's clock show. Series expand
//! through [`crate::recurrence`]; a changed occurrence takes the place of
//! the one it changed, and a cancelled one leaves a gap. Whole-day events
//! land on their days in the viewer's zone.

use std::collections::HashSet;
use std::sync::Arc;

use jiff::Timestamp;
use jiff::civil::Date;
use jiff::tz::TimeZone;
use katna_store::calendar::{EventStatus, StoredEvent};

use crate::recurrence::{Rule, starts};

/// One occurrence of an event.
#[derive(Debug, Clone)]
pub struct Occurrence {
    pub event: Arc<StoredEvent>,
    /// Unix seconds. For a whole-day event, local midnight of its first day.
    pub start: i64,
    /// Unix seconds, after the event.
    pub end: i64,
    /// The start it has in its series, for a repeating event: which
    /// occurrence this is, when it is changed or answered alone.
    pub series_start: Option<i64>,
}

impl Occurrence {
    pub fn all_day(&self) -> bool {
        self.event.data.all_day
    }
}

fn ts(seconds: i64) -> Timestamp {
    Timestamp::from_second(seconds).unwrap_or(Timestamp::UNIX_EPOCH)
}

/// The UTC day a whole-day event's stored time stands for.
fn utc_day(seconds: i64) -> Date {
    ts(seconds).to_zoned(TimeZone::UTC).date()
}

/// Local midnight of `day` in `tz`, in Unix seconds.
fn local_midnight(day: Date, tz: &TimeZone) -> i64 {
    day.to_zoned(tz.clone())
        .map_or(0, |z| z.timestamp().as_second())
}

/// A whole-day span from its stored UTC midnights to local ones.
fn local_days(start: i64, end: i64, tz: &TimeZone) -> (i64, i64) {
    let first = utc_day(start);
    let last = utc_day(end.max(start + 1));
    (local_midnight(first, tz), local_midnight(last, tz))
}

/// The occurrences of `rows` overlapping `from..to` (Unix seconds), with
/// whole-day events placed on their days in `tz`, sorted by start (whole
/// days first on a day). `rows` come from
/// `Store::event_rows_in_range` over the same range.
pub fn occurrences(rows: Vec<StoredEvent>, from: i64, to: i64, tz: &TimeZone) -> Vec<Occurrence> {
    // The occurrences a changed or cancelled one replaces.
    let replaced: HashSet<(i64, String, i64)> = rows
        .iter()
        .filter_map(|row| {
            row.data
                .recurrence_id
                .map(|rid| (row.calendar_id, row.data.uid.clone(), rid))
        })
        .collect();
    let mut out = Vec::new();
    for row in rows {
        let row = Arc::new(row);
        let data = &row.data;
        if !data.rrule.is_empty() && data.recurrence_id.is_none() {
            if data.status == EventStatus::Cancelled {
                continue;
            }
            let zone = if data.all_day {
                TimeZone::UTC
            } else {
                TimeZone::get(&data.time_zone).unwrap_or_else(|_| tz.clone())
            };
            let Some(rule) = Rule::parse(&data.rrule, &zone) else {
                push(&mut out, &row, data.start, data.end, None, from, to, tz);
                continue;
            };
            let length = data.end - data.start;
            // Whole-day series are expanded on UTC days, which can be up to
            // a day away from the viewer's.
            let (lo, hi) = (from - length - 86_400, to + 86_400);
            let exdates: Vec<Timestamp> = data.exdates.iter().map(|&s| ts(s)).collect();
            let rdates: Vec<Timestamp> = data.rdates.iter().map(|&s| ts(s)).collect();
            for start in starts(
                &rule,
                ts(data.start),
                &zone,
                ts(lo),
                ts(hi),
                &exdates,
                &rdates,
            ) {
                let start = start.as_second();
                if replaced.contains(&(row.calendar_id, data.uid.clone(), start)) {
                    continue;
                }
                push(
                    &mut out,
                    &row,
                    start,
                    start + length,
                    Some(start),
                    from,
                    to,
                    tz,
                );
            }
        } else if data.status != EventStatus::Cancelled {
            push(
                &mut out,
                &row,
                data.start,
                data.end,
                data.recurrence_id,
                from,
                to,
                tz,
            );
        }
    }
    out.sort_by(|a, b| {
        a.start
            .cmp(&b.start)
            .then(b.all_day().cmp(&a.all_day()))
            .then(b.end.cmp(&a.end))
            .then(a.event.id.cmp(&b.event.id))
    });
    out
}

#[allow(clippy::too_many_arguments)]
fn push(
    out: &mut Vec<Occurrence>,
    event: &Arc<StoredEvent>,
    start: i64,
    end: i64,
    series_start: Option<i64>,
    from: i64,
    to: i64,
    tz: &TimeZone,
) {
    let (start, end) = if event.data.all_day {
        local_days(start, end, tz)
    } else {
        (start, end.max(start))
    };
    // An event of no length still shows at its time.
    let overlaps = start < to && (end > from || (end == start && start >= from));
    if overlaps {
        out.push(Occurrence {
            event: event.clone(),
            start,
            end,
            series_start,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use katna_store::calendar::EventData;

    fn kolkata() -> TimeZone {
        TimeZone::get("Asia/Kolkata").unwrap()
    }

    fn at(text: &str, tz: &TimeZone) -> i64 {
        text.parse::<jiff::civil::DateTime>()
            .unwrap()
            .to_zoned(tz.clone())
            .unwrap()
            .timestamp()
            .as_second()
    }

    fn row(id: i64, data: EventData) -> StoredEvent {
        StoredEvent {
            id,
            calendar_id: 1,
            data,
        }
    }

    fn standup(tz: &TimeZone) -> EventData {
        EventData {
            uid: "standup".into(),
            title: "Standup".into(),
            start: at("2026-09-28T09:00", tz),
            end: at("2026-09-28T09:30", tz),
            time_zone: "Asia/Kolkata".into(),
            rrule: "FREQ=DAILY;BYDAY=MO,TU,WE,TH,FR".into(),
            ..EventData::default()
        }
    }

    #[test]
    fn a_changed_occurrence_replaces_its_own_and_a_cancelled_one_leaves_a_gap() {
        let tz = kolkata();
        let mut moved = standup(&tz);
        moved.rrule.clear();
        moved.recurrence_id = Some(at("2026-09-29T09:00", &tz));
        moved.start = at("2026-09-29T11:00", &tz);
        moved.end = at("2026-09-29T11:30", &tz);
        let mut cancelled = standup(&tz);
        cancelled.rrule.clear();
        cancelled.recurrence_id = Some(at("2026-09-30T09:00", &tz));
        cancelled.status = EventStatus::Cancelled;
        let got = occurrences(
            vec![row(1, standup(&tz)), row(2, moved), row(3, cancelled)],
            at("2026-09-28T00:00", &tz),
            at("2026-10-01T00:00", &tz),
            &tz,
        );
        let starts: Vec<(i64, i64)> = got.iter().map(|o| (o.event.id, o.start)).collect();
        assert_eq!(
            starts,
            [
                (1, at("2026-09-28T09:00", &tz)),
                (2, at("2026-09-29T11:00", &tz))
            ]
        );
        assert_eq!(got[0].series_start, Some(at("2026-09-28T09:00", &tz)));
    }

    #[test]
    fn whole_days_land_on_local_days() {
        let tz = kolkata();
        let utc = TimeZone::UTC;
        let holiday = EventData {
            title: "Gandhi Jayanti".into(),
            all_day: true,
            start: at("2026-10-02T00:00", &utc),
            end: at("2026-10-03T00:00", &utc),
            ..EventData::default()
        };
        let got = occurrences(
            vec![row(1, holiday)],
            at("2026-10-02T00:00", &tz),
            at("2026-10-03T00:00", &tz),
            &tz,
        );
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].start, at("2026-10-02T00:00", &tz));
        assert_eq!(got[0].end, at("2026-10-03T00:00", &tz));
        // Not on the day before, though it starts then in UTC terms of Kolkata.
        assert!(
            occurrences(
                vec![row(
                    1,
                    EventData {
                        all_day: true,
                        start: at("2026-10-02T00:00", &utc),
                        end: at("2026-10-03T00:00", &utc),
                        ..EventData::default()
                    }
                )],
                at("2026-10-01T00:00", &tz),
                at("2026-10-02T00:00", &tz),
                &tz
            )
            .is_empty()
        );
    }

    #[test]
    fn yearly_whole_day_birthdays() {
        let tz = kolkata();
        let utc = TimeZone::UTC;
        let birthday = EventData {
            title: "Anita's birthday".into(),
            all_day: true,
            start: at("2020-03-14T00:00", &utc),
            end: at("2020-03-15T00:00", &utc),
            rrule: "FREQ=YEARLY".into(),
            ..EventData::default()
        };
        let got = occurrences(
            vec![row(1, birthday)],
            at("2026-03-01T00:00", &tz),
            at("2026-04-01T00:00", &tz),
            &tz,
        );
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].start, at("2026-03-14T00:00", &tz));
    }

    #[test]
    fn cancelled_single_events_do_not_show() {
        let tz = kolkata();
        let mut e = standup(&tz);
        e.rrule.clear();
        e.status = EventStatus::Cancelled;
        assert!(
            occurrences(
                vec![row(1, e)],
                at("2026-09-28T00:00", &tz),
                at("2026-09-29T00:00", &tz),
                &tz
            )
            .is_empty()
        );
    }
}
