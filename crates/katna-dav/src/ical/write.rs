// SPDX-License-Identifier: GPL-3.0-or-later

//! Writing iCalendar (RFC 5545): the store's events as a `VCALENDAR`, as a
//! CalDAV server takes it (a series and its changed occurrences in one
//! resource), and the `RRULE`/`EXDATE`/`RDATE` lines Google's API takes.
//! Text is escaped and lines folded at 75 octets; times are written in
//! their zone with a `VTIMEZONE` for it, whole days as `VALUE=DATE`.

use jiff::Timestamp;
use jiff::civil::DateTime;
use jiff::tz::{Offset, TimeZone};
use katna_store::calendar::{EventData, EventKind, EventStatus};

/// What Katna calls itself in `PRODID`.
const PRODID: &str = "-//Katna//Katna Calendar//EN";

/// Longest line, in octets, before it folds.
const LINE: usize = 75;

/// Escapes text for a `TEXT` value: `\`, `;`, `,` and line breaks.
pub fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' => out.push_str("\\\\"),
            ';' => out.push_str("\\;"),
            ',' => out.push_str("\\,"),
            '\r' => {
                if chars.peek() != Some(&'\n') {
                    out.push_str("\\n");
                }
            }
            '\n' => out.push_str("\\n"),
            c if c.is_control() && c != '\t' => {}
            c => out.push(c),
        }
    }
    out
}

/// A parameter value, quoted when it has `:`, `;` or `,`; quotes and
/// control characters, which a parameter can't hold, are dropped.
fn param(value: &str) -> String {
    let clean: String = value
        .chars()
        .filter(|c| *c != '"' && !c.is_control())
        .collect();
    if clean.contains([':', ';', ',']) {
        format!("\"{clean}\"")
    } else {
        clean
    }
}

/// Adds `line` to `out`, folded into lines of at most 75 octets (a line
/// break and a space before each piece after the first), ending in CRLF.
pub fn fold(line: &str, out: &mut String) {
    let mut room = LINE;
    let mut used = 0;
    for c in line.chars() {
        let len = c.len_utf8();
        if used + len > room {
            out.push_str("\r\n ");
            // The space counts.
            room = LINE - 1;
            used = 0;
        }
        out.push(c);
        used += len;
    }
    out.push_str("\r\n");
}

fn civil(dt: DateTime) -> String {
    format!(
        "{:04}{:02}{:02}T{:02}{:02}{:02}",
        dt.year(),
        dt.month(),
        dt.day(),
        dt.hour(),
        dt.minute(),
        dt.second()
    )
}

fn ts(seconds: i64) -> Timestamp {
    Timestamp::from_second(seconds).unwrap_or(Timestamp::UNIX_EPOCH)
}

/// The zone times of an event are written in: its own when it is a known
/// one other than UTC, else `None` (UTC).
fn event_zone(zone: &str) -> Option<(String, TimeZone)> {
    if zone.is_empty() || zone.eq_ignore_ascii_case("UTC") || zone == "Etc/UTC" {
        return None;
    }
    TimeZone::get(zone).ok().map(|tz| (zone.to_owned(), tz))
}

/// A date or date-time value: its parameters and the value. Whole days
/// are `VALUE=DATE` (the stored UTC midnight's day); times are local in
/// `zone` with its `TZID` when it is a known zone, else UTC.
pub fn time_value(seconds: i64, all_day: bool, zone: &str) -> (String, String) {
    if all_day {
        let day = ts(seconds).to_zoned(TimeZone::UTC).date();
        return (
            ";VALUE=DATE".to_owned(),
            format!("{:04}{:02}{:02}", day.year(), day.month(), day.day()),
        );
    }
    match event_zone(zone) {
        Some((name, tz)) => (
            format!(";TZID={}", param(&name)),
            civil(ts(seconds).to_zoned(tz).datetime()),
        ),
        None => (
            String::new(),
            format!("{}Z", civil(ts(seconds).to_zoned(TimeZone::UTC).datetime())),
        ),
    }
}

/// A list property (`EXDATE`, `RDATE`) for `times` of `event`, or `None`
/// when there are none.
fn time_list(name: &str, times: &[i64], event: &EventData) -> Option<String> {
    let (params, _) = time_value(times.first().copied()?, event.all_day, &event.time_zone);
    let values: Vec<String> = times
        .iter()
        .map(|&t| time_value(t, event.all_day, &event.time_zone).1)
        .collect();
    Some(format!("{name}{params}:{}", values.join(",")))
}

/// A series' `RRULE`, `EXDATE` and `RDATE` lines, unfolded: what Google's
/// `recurrence` takes. Empty for an event that doesn't repeat.
pub fn recurrence_lines(event: &EventData) -> Vec<String> {
    if event.rrule.is_empty() || event.recurrence_id.is_some() {
        return Vec::new();
    }
    let mut lines = vec![format!("RRULE:{}", event.rrule.trim())];
    lines.extend(time_list("EXDATE", &event.exdates, event));
    lines.extend(time_list("RDATE", &event.rdates, event));
    lines
}

/// `+0530`, `-0800`, `+003456`.
fn offset_text(offset: Offset) -> String {
    let total = offset.seconds();
    let sign = if total < 0 { '-' } else { '+' };
    let total = total.unsigned_abs();
    let (h, m, s) = (total / 3600, total / 60 % 60, total % 60);
    if s == 0 {
        format!("{sign}{h:02}{m:02}")
    } else {
        format!("{sign}{h:02}{m:02}{s:02}")
    }
}

/// A `VTIMEZONE` for zone `name`, with its changes of offset from a year
/// before `around` to five years after (servers that know the zone by its
/// name, as most do, use their own rules).
fn write_timezone(name: &str, tz: &TimeZone, around: i64, out: &mut String) {
    const YEAR: i64 = 366 * 86_400;
    out.push_str("BEGIN:VTIMEZONE\r\n");
    fold(&format!("TZID:{name}"), out);
    let from = ts(around.saturating_sub(YEAR));
    let until = ts(around.saturating_add(5 * YEAR));
    let first = tz.to_offset(from);
    // The offset before the first change listed.
    out.push_str("BEGIN:STANDARD\r\nDTSTART:19700101T000000\r\n");
    fold(&format!("TZOFFSETFROM:{}", offset_text(first)), out);
    fold(&format!("TZOFFSETTO:{}", offset_text(first)), out);
    out.push_str("END:STANDARD\r\n");
    for change in tz.following(from).take_while(|t| t.timestamp() < until) {
        let before = tz.to_offset(ts(change.timestamp().as_second() - 1));
        let kind = if change.dst().is_dst() {
            "DAYLIGHT"
        } else {
            "STANDARD"
        };
        let local = change
            .timestamp()
            .to_zoned(TimeZone::fixed(before))
            .datetime();
        fold(&format!("BEGIN:{kind}"), out);
        fold(&format!("DTSTART:{}", civil(local)), out);
        fold(&format!("TZOFFSETFROM:{}", offset_text(before)), out);
        fold(&format!("TZOFFSETTO:{}", offset_text(change.offset())), out);
        fold(&format!("END:{kind}"), out);
    }
    out.push_str("END:VTIMEZONE\r\n");
}

fn part_stat(status: &str) -> Option<&'static str> {
    Some(match status {
        "accepted" => "ACCEPTED",
        "tentative" => "TENTATIVE",
        "declined" => "DECLINED",
        "needs_action" => "NEEDS-ACTION",
        _ => return None,
    })
}

/// Writes one `VEVENT` of `event`; `stamp` is its `DTSTAMP` (Unix
/// seconds).
pub fn write_event(event: &EventData, stamp: i64, out: &mut String) {
    let mut line = |text: String| fold(&text, out);
    line("BEGIN:VEVENT".into());
    line(format!("UID:{}", escape(&event.uid)));
    line(format!("DTSTAMP:{}", time_value(stamp, false, "").1));
    if let Some(original) = event.recurrence_id {
        let (params, value) = time_value(original, event.all_day, &event.time_zone);
        line(format!("RECURRENCE-ID{params}:{value}"));
    }
    let (params, value) = time_value(event.start, event.all_day, &event.time_zone);
    line(format!("DTSTART{params}:{value}"));
    let (params, value) = time_value(event.end, event.all_day, &event.time_zone);
    line(format!("DTEND{params}:{value}"));
    if !event.rrule.is_empty() && event.recurrence_id.is_none() {
        for rule in recurrence_lines(event) {
            line(rule);
        }
    }
    line(format!("SUMMARY:{}", escape(&event.title)));
    if !event.location.is_empty() {
        line(format!("LOCATION:{}", escape(&event.location)));
    }
    if !event.description.is_empty() {
        line(format!("DESCRIPTION:{}", escape(&event.description)));
    }
    line(format!(
        "STATUS:{}",
        match event.status {
            EventStatus::Confirmed => "CONFIRMED",
            EventStatus::Tentative => "TENTATIVE",
            EventStatus::Cancelled => "CANCELLED",
        }
    ));
    line(format!(
        "TRANSP:{}",
        if event.busy { "OPAQUE" } else { "TRANSPARENT" }
    ));
    if event.kind == EventKind::OutOfOffice {
        line("X-MICROSOFT-CDO-BUSYSTATUS:OOF".into());
    }
    if !event.color.is_empty() {
        line(format!("COLOR:{}", escape(&event.color)));
    }
    if !event.organizer.is_empty() {
        let cn = if event.organizer_name.is_empty() {
            String::new()
        } else {
            format!(";CN={}", param(&event.organizer_name))
        };
        line(format!("ORGANIZER{cn}:mailto:{}", event.organizer));
    }
    for attendee in &event.attendees {
        let mut params = String::new();
        if !attendee.name.is_empty() {
            params.push_str(&format!(";CN={}", param(&attendee.name)));
        }
        if let Some(stat) = part_stat(&attendee.status) {
            params.push_str(&format!(";PARTSTAT={stat}"));
        }
        if attendee.optional {
            params.push_str(";ROLE=OPT-PARTICIPANT");
        }
        line(format!("ATTENDEE{params}:mailto:{}", attendee.email));
    }
    if !event.join_url.is_empty() {
        line(format!("CONFERENCE;VALUE=URI:{}", event.join_url));
    }
    if !event.web_link.is_empty() {
        line(format!("URL:{}", event.web_link));
    }
    if event.updated_at > 0 {
        line(format!(
            "LAST-MODIFIED:{}",
            time_value(event.updated_at, false, "").1
        ));
    }
    for minutes in &event.reminders {
        line("BEGIN:VALARM".into());
        line("ACTION:DISPLAY".into());
        line(format!("DESCRIPTION:{}", escape(&event.title)));
        line(format!("TRIGGER:-PT{}M", minutes.max(&0)));
        line("END:VALARM".into());
    }
    line("END:VEVENT".into());
}

/// A `VCALENDAR` of `events` (a series first, then its changed
/// occurrences; or one event), with a `VTIMEZONE` for each zone they use.
/// `stamp` is the `DTSTAMP` (Unix seconds).
pub fn write_calendar(events: &[EventData], stamp: i64) -> String {
    let mut out = String::new();
    out.push_str("BEGIN:VCALENDAR\r\nVERSION:2.0\r\n");
    fold(&format!("PRODID:{PRODID}"), &mut out);
    out.push_str("CALSCALE:GREGORIAN\r\n");
    let mut zones: Vec<String> = Vec::new();
    for event in events.iter().filter(|e| !e.all_day) {
        if let Some((name, tz)) = event_zone(&event.time_zone)
            && !zones.contains(&name)
        {
            write_timezone(&name, &tz, event.start, &mut out);
            zones.push(name);
        }
    }
    for event in events {
        write_event(event, stamp, &mut out);
    }
    out.push_str("END:VCALENDAR\r\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ical::{components, parse_events, unfold};
    use katna_store::calendar::Attendee;

    fn at(text: &str, zone: &str) -> i64 {
        text.parse::<DateTime>()
            .unwrap()
            .to_zoned(TimeZone::get(zone).unwrap())
            .unwrap()
            .timestamp()
            .as_second()
    }

    #[test]
    fn text_escapes_and_reads_back() {
        let text = "One, two; three\nfour\\five\r\nsix";
        let escaped = escape(text);
        assert_eq!(escaped, "One\\, two\\; three\\nfour\\\\five\\nsix");
        assert_eq!(
            crate::ical::unescape(&escaped),
            "One, two; three\nfour\\five\nsix"
        );
    }

    #[test]
    fn long_lines_fold_at_75_octets_between_characters() {
        let long = format!("SUMMARY:{}", "ü".repeat(100));
        let mut out = String::new();
        fold(&long, &mut out);
        for piece in out.split("\r\n").filter(|l| !l.is_empty()) {
            assert!(piece.len() <= 75, "{} octets: {piece}", piece.len());
        }
        assert_eq!(unfold(&out), [long]);
    }

    #[test]
    fn a_series_with_its_changed_occurrence_reads_back_as_written() {
        let start = at("2026-10-05T09:00", "Europe/Berlin");
        let mut series = EventData {
            uid: "abc@katna".into(),
            title: "Planning, weekly; with \"quotes\"".into(),
            location: "Room 4".into(),
            description: "First line\nSecond line with a very long text that goes well past \
                          seventy-five octets so that it folds"
                .into(),
            start,
            end: start + 3600,
            time_zone: "Europe/Berlin".into(),
            rrule: "FREQ=WEEKLY;BYDAY=MO;UNTIL=20261231T225959Z".into(),
            exdates: vec![start + 7 * 86_400],
            busy: false,
            color: "#33b679".into(),
            organizer: "me@example.com".into(),
            organizer_name: "Me, Myself".into(),
            attendees: vec![
                Attendee {
                    email: "me@example.com".into(),
                    name: "Me, Myself".into(),
                    status: "accepted".into(),
                    is_self: true,
                    organizer: true,
                    ..Attendee::default()
                },
                Attendee {
                    email: "anita@example.com".into(),
                    name: "Anita".into(),
                    status: "needs_action".into(),
                    optional: true,
                    ..Attendee::default()
                },
            ],
            self_status: "accepted".into(),
            join_url: "https://meet.example.com/x".into(),
            reminders: vec![10, 30],
            updated_at: at("2026-09-01T12:00", "UTC"),
            ..EventData::default()
        };
        series.range_end = crate::ical::range_end(&series);
        // After the clocks go back, still 9:00 in Berlin.
        let original = at("2026-11-02T09:00", "Europe/Berlin");
        let moved = EventData {
            uid: series.uid.clone(),
            recurrence_id: Some(original),
            title: "Planning (moved)".into(),
            start: original + 1800,
            end: original + 5400,
            time_zone: "Europe/Berlin".into(),
            busy: true,
            range_end: Some(original + 5400),
            ..EventData::default()
        };
        let text = write_calendar(&[series.clone(), moved.clone()], 0);
        assert!(text.contains("BEGIN:VTIMEZONE\r\nTZID:Europe/Berlin\r\n"));
        assert!(text.contains("TZOFFSETTO:+0100"));
        assert!(text.contains("DTSTART;TZID=Europe/Berlin:20261005T090000\r\n"));
        let back = parse_events(&text, &TimeZone::UTC, "me@example.com");
        assert_eq!(back, [series, moved]);
    }

    #[test]
    fn whole_days_and_utc_times() {
        let day = at("2026-12-24T00:00", "UTC");
        let holiday = EventData {
            uid: "x@katna".into(),
            title: "Holiday".into(),
            start: day,
            end: day + 2 * 86_400,
            all_day: true,
            rrule: "FREQ=YEARLY".into(),
            exdates: vec![day + 365 * 86_400],
            busy: true,
            ..EventData::default()
        };
        let text = write_calendar(std::slice::from_ref(&holiday), 0);
        assert!(!text.contains("VTIMEZONE"));
        assert!(text.contains("DTSTART;VALUE=DATE:20261224\r\n"));
        assert!(text.contains("EXDATE;VALUE=DATE:20271224\r\n"));
        let back = &parse_events(&text, &TimeZone::UTC, "")[0];
        assert_eq!(back.start, holiday.start);
        assert_eq!(back.end, holiday.end);
        assert!(back.all_day);
        assert_eq!(back.exdates, holiday.exdates);

        let utc = EventData {
            uid: "y".into(),
            start: 1_800_000_000,
            end: 1_800_003_600,
            busy: true,
            ..EventData::default()
        };
        let text = write_calendar(std::slice::from_ref(&utc), 1_800_000_000);
        assert!(text.contains("DTSTART:20270115T080000Z\r\n"), "{text}");
        let top = components(&text);
        let event = &top[0].children[0];
        assert_eq!(event.text("DTSTAMP"), "20270115T080000Z");
        assert_eq!(parse_events(&text, &TimeZone::UTC, "")[0].start, utc.start);
    }

    #[test]
    fn recurrence_lines_for_google() {
        let start = at("2026-10-05T09:00", "Asia/Kolkata");
        let series = EventData {
            start,
            end: start + 900,
            time_zone: "Asia/Kolkata".into(),
            rrule: "FREQ=DAILY".into(),
            exdates: vec![start + 86_400, start + 2 * 86_400],
            ..EventData::default()
        };
        assert_eq!(
            recurrence_lines(&series),
            [
                "RRULE:FREQ=DAILY",
                "EXDATE;TZID=Asia/Kolkata:20261006T090000,20261007T090000"
            ]
        );
        assert!(
            recurrence_lines(&EventData {
                rrule: String::new(),
                ..series
            })
            .is_empty()
        );
    }
}
