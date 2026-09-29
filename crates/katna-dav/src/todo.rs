// SPDX-License-Identifier: GPL-3.0-or-later

//! To-dos (`VTODO`, RFC 5545 §3.6.2) as CalDAV servers keep tasks: read
//! into a [`Todo`], and written back over the server's own text so that
//! what Katna doesn't show (categories, attachments, a client's own
//! fields, several alarms) stays as it was.

use jiff::{Timestamp, civil::Date, tz::TimeZone};

use crate::ical::{self, Component, Property};

/// What Katna keeps of a to-do.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Todo {
    pub uid: String,
    pub title: String,
    pub notes: String,
    /// `YYYY-MM-DD`, or empty.
    pub due: String,
    /// A time on the due day: minutes after local midnight.
    pub due_time: Option<u32>,
    /// When it was ticked off (Unix seconds).
    pub done_at: Option<i64>,
    /// The `UID` of the to-do it is a step of.
    pub parent: Option<String>,
    /// When to remind (Unix seconds).
    pub remind_at: Option<i64>,
    /// An `RRULE` value, or empty.
    pub repeat: String,
    /// High priority (1 to 4).
    pub starred: bool,
}

/// The first to-do of `text` that isn't one changed instance of a
/// repeating one; times without a zone are read in `zone`.
pub fn parse(text: &str, zone: &TimeZone) -> Option<Todo> {
    let mut todos = Vec::new();
    for component in ical::components(text) {
        if component.name == "VTODO" {
            todos.push(component.clone());
        }
        let mut found = Vec::new();
        component.find("VTODO", &mut found);
        todos.extend(found.into_iter().cloned());
    }
    let todo = todos
        .iter()
        .find(|t| t.property("RECURRENCE-ID").is_none())
        .or(todos.first())?;
    Some(read(todo, zone))
}

fn read(todo: &Component, zone: &TimeZone) -> Todo {
    let due_at = todo.property("DUE").and_then(|p| ical::date_time(p, zone));
    let (due, due_time) = match due_at {
        Some((at, true)) => (day_of(at, &TimeZone::UTC), None),
        Some((at, false)) => {
            let local = Timestamp::from_second(at)
                .map(|t| t.to_zoned(zone.clone()))
                .ok();
            let time = local.as_ref().map(|z| {
                u32::try_from(i32::from(z.hour()) * 60 + i32::from(z.minute())).unwrap_or(0)
            });
            (day_of(at, zone), time)
        }
        None => (String::new(), None),
    };
    let status = todo.text("STATUS").to_ascii_uppercase();
    let completed = todo
        .property("COMPLETED")
        .and_then(|p| ical::date_time(p, zone))
        .map(|(at, _)| at);
    let done_at = (status == "COMPLETED" || completed.is_some()).then(|| completed.unwrap_or(0));
    let parent = todo
        .all("RELATED-TO")
        .find(|p| {
            p.param("RELTYPE")
                .is_none_or(|t| t.eq_ignore_ascii_case("PARENT"))
        })
        .map(Property::text)
        .filter(|uid| !uid.is_empty());
    let priority: u32 = todo.text("PRIORITY").trim().parse().unwrap_or(0);
    Todo {
        uid: todo.text("UID"),
        title: todo.text("SUMMARY"),
        notes: todo.text("DESCRIPTION"),
        due,
        due_time,
        done_at,
        parent,
        remind_at: reminder(todo, zone),
        repeat: todo
            .property("RRULE")
            .map(|p| p.value.trim().to_owned())
            .unwrap_or_default(),
        starred: (1..=4).contains(&priority),
    }
}

fn day_of(at: i64, zone: &TimeZone) -> String {
    Timestamp::from_second(at)
        .map(|t| t.to_zoned(zone.clone()).date().to_string())
        .unwrap_or_default()
}

/// The first alarm's time: an absolute trigger, or one before or after
/// the start (or the due time, `RELATED=END`).
fn reminder(todo: &Component, zone: &TimeZone) -> Option<i64> {
    let alarm = todo.children.iter().find(|c| c.name == "VALARM")?;
    let trigger = alarm.property("TRIGGER")?;
    let absolute = trigger
        .param("VALUE")
        .is_some_and(|v| v.eq_ignore_ascii_case("DATE-TIME"));
    if absolute {
        return ical::date_time(trigger, zone).map(|(at, _)| at);
    }
    let offset = ical::duration(&trigger.value)?;
    let from_end = trigger
        .param("RELATED")
        .is_some_and(|r| r.eq_ignore_ascii_case("END"));
    let base = if from_end { "DUE" } else { "DTSTART" };
    let base = todo
        .property(base)
        .or_else(|| todo.property("DUE"))
        .and_then(|p| ical::date_time(p, zone))?;
    // A whole day is its local midnight here.
    let start = if base.1 {
        let day = Timestamp::from_second(base.0)
            .ok()?
            .to_zoned(TimeZone::UTC)
            .date();
        day.to_zoned(zone.clone()).ok()?.timestamp().as_second()
    } else {
        base.0
    };
    Some(start + offset)
}

/// Properties of the to-do [`write`] sets; others in the old text stay.
const OWNED: [&str; 11] = [
    "UID",
    "SUMMARY",
    "DESCRIPTION",
    "DUE",
    "STATUS",
    "COMPLETED",
    "PERCENT-COMPLETE",
    "RRULE",
    "DTSTAMP",
    "LAST-MODIFIED",
    "RELATED-TO",
];

/// `todo` as an iCalendar object, `now` being the time of writing. With
/// `old`, the server's text of it, everything Katna doesn't set is kept,
/// and the alarms are only replaced when the reminder changed.
pub fn write(todo: &Todo, old: Option<&str>, now: i64, zone: &TimeZone) -> String {
    let before = old.and_then(|old| parse(old, zone));
    let mut own = vec![format!("UID:{}", escape(&todo.uid))];
    own.push(format!("DTSTAMP:{}", utc(now)));
    own.push(format!("LAST-MODIFIED:{}", utc(now)));
    own.push(format!("SUMMARY:{}", escape(&todo.title)));
    if !todo.notes.is_empty() {
        own.push(format!("DESCRIPTION:{}", escape(&todo.notes)));
    }
    let due = due_value(todo, zone);
    if let Some(due) = &due {
        own.push(format!("DUE{due}"));
    }
    match todo.done_at {
        Some(at) => {
            own.push("STATUS:COMPLETED".into());
            own.push(format!("COMPLETED:{}", utc(if at > 0 { at } else { now })));
            own.push("PERCENT-COMPLETE:100".into());
        }
        None => own.push("STATUS:NEEDS-ACTION".into()),
    }
    // A repeat counts from a due day, as in To Do.
    if !todo.repeat.is_empty() && due.is_some() {
        own.push(format!("RRULE:{}", todo.repeat.replace(['\r', '\n'], "")));
    }
    if let Some(parent) = &todo.parent {
        own.push(format!("RELATED-TO;RELTYPE=PARENT:{}", escape(parent)));
    }
    let alarms_changed = before
        .as_ref()
        .is_none_or(|b| b.remind_at != todo.remind_at);
    let starred_changed = before.as_ref().is_none_or(|b| b.starred != todo.starred);
    if starred_changed && todo.starred {
        own.push("PRIORITY:1".into());
    }
    let mut new_alarm = Vec::new();
    if alarms_changed && let Some(at) = todo.remind_at {
        new_alarm = vec![
            "BEGIN:VALARM".to_owned(),
            "ACTION:DISPLAY".to_owned(),
            format!("DESCRIPTION:{}", escape(&todo.title)),
            format!("TRIGGER;VALUE=DATE-TIME:{}", utc(at)),
            "END:VALARM".to_owned(),
        ];
    }

    let Some(old) = old else {
        let mut lines = vec![
            "BEGIN:VCALENDAR".to_owned(),
            "VERSION:2.0".to_owned(),
            "PRODID:-//Katna//Katna Tasks//EN".to_owned(),
            "BEGIN:VTODO".to_owned(),
        ];
        lines.extend(own);
        lines.extend(new_alarm);
        lines.push("END:VTODO".into());
        lines.push("END:VCALENDAR".into());
        return fold_all(&lines);
    };

    // The old text, line by line, with the master to-do's own lines
    // replaced. Changed instances of a repeating one stay as they are.
    let lines = ical::unfold(old);
    let mut out = Vec::with_capacity(lines.len() + own.len());
    let mut depth_in_todo = 0usize;
    let mut in_master = false;
    let mut todo_lines: Vec<String> = Vec::new();
    for line in lines {
        let upper = line.to_ascii_uppercase();
        if depth_in_todo == 0 {
            if upper == "BEGIN:VTODO" {
                depth_in_todo = 1;
                todo_lines = vec![line];
            } else {
                out.push(line);
            }
            continue;
        }
        if upper == "END:VTODO" {
            depth_in_todo = 0;
            let master = !todo_lines
                .iter()
                .any(|l| l.to_ascii_uppercase().starts_with("RECURRENCE-ID"));
            if master && !in_master {
                in_master = true;
                out.extend(rewrite(
                    &todo_lines,
                    &own,
                    &new_alarm,
                    alarms_changed,
                    starred_changed,
                    due.is_some(),
                ));
            } else {
                out.append(&mut todo_lines);
            }
            out.push(line);
            continue;
        }
        todo_lines.push(line);
    }
    fold_all(&out)
}

/// A master to-do's lines (from `BEGIN:VTODO`, without `END:VTODO`) with
/// Katna's `own` lines instead of the old ones.
fn rewrite(
    lines: &[String],
    own: &[String],
    new_alarm: &[String],
    alarms_changed: bool,
    starred_changed: bool,
    has_due: bool,
) -> Vec<String> {
    let mut out = Vec::with_capacity(lines.len() + own.len());
    let mut depth = 0usize;
    let mut dropping = false;
    for (index, line) in lines.iter().enumerate() {
        let upper = line.to_ascii_uppercase();
        if index == 0 {
            out.push(line.clone());
            continue;
        }
        if upper.starts_with("BEGIN:") {
            depth += 1;
            if depth == 1 {
                dropping = alarms_changed && upper == "BEGIN:VALARM";
            }
        }
        let inside = depth > 0;
        if upper.starts_with("END:") {
            depth = depth.saturating_sub(1);
            if dropping {
                dropping = depth > 0;
                continue;
            }
        }
        if dropping {
            continue;
        }
        if !inside {
            let name = ical::property(line).map(|p| p.name).unwrap_or_default();
            let parent_link = name == "RELATED-TO"
                && ical::property(line)
                    .and_then(|p| p.param("RELTYPE").map(str::to_ascii_uppercase))
                    .is_some_and(|t| t != "PARENT");
            if OWNED.contains(&name.as_str()) && !parent_link {
                continue;
            }
            if name == "PRIORITY" && starred_changed {
                continue;
            }
            // A start with no due day to count a repeat from goes too.
            if name == "DTSTART" && !has_due {
                continue;
            }
        }
        out.push(line.clone());
    }
    out.extend(own.iter().cloned());
    out.extend(new_alarm.iter().cloned());
    out
}

/// `;VALUE=DATE:YYYYMMDD`, or `:YYYYMMDDTHHMMSSZ` with a time.
fn due_value(todo: &Todo, zone: &TimeZone) -> Option<String> {
    let day: Date = todo.due.parse().ok()?;
    match todo.due_time {
        None => Some(format!(";VALUE=DATE:{}", day.strftime("%Y%m%d"))),
        Some(minutes) => {
            let hour = i8::try_from(minutes / 60).ok()?;
            let minute = i8::try_from(minutes % 60).ok()?;
            let at = day
                .at(hour, minute, 0, 0)
                .to_zoned(zone.clone())
                .ok()?
                .timestamp()
                .as_second();
            Some(format!(":{}", utc(at)))
        }
    }
}

/// `YYYYMMDDTHHMMSSZ`.
fn utc(at: i64) -> String {
    Timestamp::from_second(at)
        .unwrap_or(Timestamp::UNIX_EPOCH)
        .strftime("%Y%m%dT%H%M%SZ")
        .to_string()
}

/// Escapes a text value (RFC 5545 §3.3.11).
fn escape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for c in value.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            ',' => out.push_str("\\,"),
            ';' => out.push_str("\\;"),
            '\n' => out.push_str("\\n"),
            '\r' => {}
            c => out.push(c),
        }
    }
    out
}

/// Folds each line at 75 octets, never inside a character.
fn fold_all(lines: &[String]) -> String {
    let mut out = String::new();
    for line in lines {
        let mut width = 0;
        for c in line.chars() {
            let len = c.len_utf8();
            if width + len > 75 {
                out.push_str("\r\n ");
                width = 1;
            }
            out.push(c);
            width += len;
        }
        out.push_str("\r\n");
    }
    out
}

/// Where a task's due day starts when it has no time: its reminders count
/// from this time of day (minutes after midnight), 9 AM, as To Do's do.
pub const DAY_START: u32 = 9 * 60;

/// When a task is due, as an instant: its due day (`YYYY-MM-DD`) at its
/// time, or at [`DAY_START`] when it has none, in `zone`.
pub fn due_at(due: &str, time: Option<u32>, zone: &TimeZone) -> Option<i64> {
    let day: Date = due.parse().ok()?;
    let minutes = time.unwrap_or(DAY_START).min(24 * 60 - 1);
    let time = jiff::civil::Time::new(
        i8::try_from(minutes / 60).ok()?,
        i8::try_from(minutes % 60).ok()?,
        0,
        0,
    )
    .ok()?;
    day.to_datetime(time)
        .to_zoned(zone.clone())
        .ok()
        .map(|z| z.timestamp().as_second())
}

/// A reminder at `at` of a task due at `from` (day and time) when the
/// task is due at `to` instead: as long before (or after) as it was.
pub fn moved_reminder(
    at: Option<i64>,
    from: (&str, Option<u32>),
    to: (&str, Option<u32>),
    zone: &TimeZone,
) -> Option<i64> {
    let at = at?;
    match (due_at(from.0, from.1, zone), due_at(to.0, to.1, zone)) {
        (Some(old), Some(new)) => Some(at - old + new),
        _ => Some(at),
    }
}

/// Where a repeating task goes when it is ticked off: its next due day
/// after both its due day and `today` (a late tick doesn't bring it back
/// already late), and its rule with a `COUNT` lowered by the days it used
/// up. `None` when `rule` doesn't repeat daily or slower, or has ended:
/// the task is done then.
pub fn next_due(due: &str, rule: &str, today: Date) -> Option<(String, String)> {
    let utc = TimeZone::UTC;
    let parsed = crate::recurrence::Rule::parse(rule, &utc)?;
    let base: Date = due.parse().unwrap_or(today);
    let noon = |day: Date| {
        day.to_datetime(jiff::civil::Time::constant(12, 0, 0, 0))
            .to_zoned(utc.clone())
            .ok()
            .map(|z| z.timestamp())
    };
    let first = noon(base)?;
    let after = noon(base.max(today))?;
    // Five years is room for any yearly rule, even one on 29 February.
    let end = noon(
        base.max(today)
            .checked_add(jiff::Span::new().years(5))
            .ok()?,
    )?;
    let starts = crate::recurrence::starts(&parsed, first, &utc, first, end, &[], &[]);
    let (used, next) = starts.iter().enumerate().find(|(_, s)| **s > after)?;
    let next = next.to_zoned(utc).date().to_string();
    let rule = match parsed.count {
        Some(count) => {
            let left = count.checked_sub(u32::try_from(used).ok()?)?;
            if left == 0 {
                return None;
            }
            rule.trim()
                .trim_start_matches("RRULE:")
                .split(';')
                .map(|part| match part.split_once('=') {
                    Some((key, _)) if key.trim().eq_ignore_ascii_case("COUNT") => {
                        format!("COUNT={left}")
                    }
                    _ => part.to_owned(),
                })
                .collect::<Vec<_>>()
                .join(";")
        }
        None => rule.to_owned(),
    };
    Some((next, rule))
}

#[cfg(test)]
mod tests {
    use super::*;

    const APPLE: &str = "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nPRODID:-//Apple Inc.//iOS 18//EN\r\n\
BEGIN:VTODO\r\nUID:r1\r\nDTSTAMP:20260901T000000Z\r\nSUMMARY:Pay rent\\, today\r\n\
DESCRIPTION:Flat 4\\nby transfer\r\nDUE;VALUE=DATE:20261001\r\nPRIORITY:1\r\n\
CATEGORIES:Home\r\nX-APPLE-SORT-ORDER:12\r\nRRULE:FREQ=MONTHLY\r\nDTSTART;VALUE=DATE:20261001\r\n\
BEGIN:VALARM\r\nACTION:DISPLAY\r\nTRIGGER:-PT30M\r\nEND:VALARM\r\n\
END:VTODO\r\nEND:VCALENDAR\r\n";

    #[test]
    fn reads_what_katna_shows() {
        let zone = TimeZone::get("Asia/Kolkata").unwrap();
        let todo = parse(APPLE, &zone).unwrap();
        assert_eq!(todo.uid, "r1");
        assert_eq!(todo.title, "Pay rent, today");
        assert_eq!(todo.notes, "Flat 4\nby transfer");
        assert_eq!(todo.due, "2026-10-01");
        assert_eq!(todo.due_time, None);
        assert!(todo.starred);
        assert_eq!(todo.repeat, "FREQ=MONTHLY");
        assert_eq!(todo.done_at, None);
        // 30 minutes before local midnight of the start.
        let midnight = Date::new(2026, 10, 1)
            .unwrap()
            .to_zoned(zone.clone())
            .unwrap()
            .timestamp()
            .as_second();
        assert_eq!(todo.remind_at, Some(midnight - 30 * 60));
    }

    #[test]
    fn writes_over_the_old_text_and_keeps_the_rest() {
        let zone = TimeZone::UTC;
        let mut todo = parse(APPLE, &zone).unwrap();
        todo.title = "Pay rent".into();
        todo.done_at = Some(1_790_000_000);
        todo.starred = false;
        let text = write(&todo, Some(APPLE), 1_790_000_100, &zone);
        assert!(text.contains("CATEGORIES:Home\r\n"));
        assert!(text.contains("X-APPLE-SORT-ORDER:12\r\n"));
        assert!(text.contains("SUMMARY:Pay rent\r\n"));
        assert!(!text.contains("PRIORITY"));
        assert!(text.contains("STATUS:COMPLETED\r\n"));
        // The reminder didn't change: the alarm stays as it was.
        assert!(text.contains("TRIGGER:-PT30M\r\n"));
        assert_eq!(text.matches("BEGIN:VTODO").count(), 1);
        assert_eq!(text.matches("SUMMARY").count(), 1);
        let again = parse(&text, &zone).unwrap();
        assert_eq!(again.title, "Pay rent");
        assert_eq!(again.done_at, Some(1_790_000_000));
        assert!(!again.starred);
        assert_eq!(again.repeat, "FREQ=MONTHLY");
    }

    #[test]
    fn a_new_step_links_to_its_task() {
        let zone = TimeZone::UTC;
        let todo = Todo {
            uid: "s1".into(),
            title: "Buy stamps".into(),
            due: "2026-10-02".into(),
            due_time: Some(9 * 60 + 30),
            parent: Some("r1".into()),
            remind_at: Some(1_790_900_000),
            ..Todo::default()
        };
        let text = write(&todo, None, 1_790_000_000, &zone);
        assert!(text.contains("DUE:20261002T093000Z\r\n"));
        assert!(text.contains("RELATED-TO;RELTYPE=PARENT:r1\r\n"));
        let again = parse(&text, &zone).unwrap();
        assert_eq!(again.due, "2026-10-02");
        assert_eq!(again.due_time, Some(9 * 60 + 30));
        assert_eq!(again.parent.as_deref(), Some("r1"));
        assert_eq!(again.remind_at, Some(1_790_900_000));
    }
    #[test]
    fn repeating_tasks_move_to_their_next_day() {
        let day = |text: &str| text.parse::<Date>().unwrap();
        let next = |due: &str, rule: &str, today: &str| next_due(due, rule, day(today));
        // Ticked on time: the next one.
        assert_eq!(
            next("2026-09-29", "FREQ=DAILY", "2026-09-29"),
            Some(("2026-09-30".into(), "FREQ=DAILY".into()))
        );
        // Ticked late: the next one after today, not one already late.
        assert_eq!(
            next("2026-09-07", "FREQ=WEEKLY", "2026-09-23"),
            Some(("2026-09-28".into(), "FREQ=WEEKLY".into()))
        );
        // Ticked early: the one after its own day.
        assert_eq!(
            next("2026-10-31", "FREQ=MONTHLY", "2026-09-29"),
            Some(("2026-12-31".into(), "FREQ=MONTHLY".into()))
        );
        assert_eq!(
            next("2026-09-29", "FREQ=WEEKLY;BYDAY=MO,TH", "2026-09-29"),
            Some(("2026-10-01".into(), "FREQ=WEEKLY;BYDAY=MO,TH".into()))
        );
        // A count goes down by the days used, and ends.
        assert_eq!(
            next("2026-09-29", "FREQ=DAILY;COUNT=3", "2026-09-29"),
            Some(("2026-09-30".into(), "FREQ=DAILY;COUNT=2".into()))
        );
        assert_eq!(
            next("2026-09-27", "FREQ=DAILY;COUNT=5", "2026-09-29"),
            Some(("2026-09-30".into(), "FREQ=DAILY;COUNT=2".into()))
        );
        assert_eq!(next("2026-09-29", "FREQ=DAILY;COUNT=1", "2026-09-29"), None);
        assert_eq!(
            next("2026-09-29", "FREQ=DAILY;UNTIL=20260930", "2026-09-30"),
            None
        );
        // Not a rule Katna repeats: done.
        assert_eq!(next("2026-09-29", "FREQ=HOURLY", "2026-09-29"), None);
        assert_eq!(next("2026-09-29", "", "2026-09-29"), None);
        // No day yet: from today.
        assert_eq!(
            next("", "FREQ=DAILY", "2026-09-29"),
            Some(("2026-09-30".into(), "FREQ=DAILY".into()))
        );
    }

    #[test]
    fn tasks_are_due_at_their_time_or_in_the_morning() {
        let utc = TimeZone::UTC;
        assert_eq!(due_at("2026-09-29", Some(0), &utc), Some(1_790_640_000));
        assert_eq!(
            due_at("2026-09-29", None, &utc),
            Some(1_790_640_000 + 9 * 3600)
        );
        assert_eq!(due_at("", Some(60), &utc), None);
        // A reminder an hour before stays an hour before.
        let at = 1_790_640_000 + 3600;
        assert_eq!(
            moved_reminder(
                Some(at),
                ("2026-09-29", Some(120)),
                ("2026-09-30", Some(180)),
                &utc
            ),
            Some(at + 86_400 + 3600)
        );
        assert_eq!(
            moved_reminder(None, ("2026-09-29", None), ("", None), &utc),
            None
        );
    }
}
