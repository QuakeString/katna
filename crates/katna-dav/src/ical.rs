// SPDX-License-Identifier: GPL-3.0-or-later

//! iCalendar (RFC 5545) as CalDAV servers and Google's `recurrence` lines
//! write it: content lines (unfolded, with their parameters), date-times
//! with `TZID` or `VALUE=DATE`, text unescaping, and `VEVENT`s turned into
//! the store's [`EventData`].
//!
//! The pieces below [`parse_events`] ([`components`], [`Property`],
//! [`date_time`], [`date_list`], [`zone`], [`duration`]) are meant for any
//! component, so `VTODO` can be read with them too.
//!
//! Time zones are looked up by name: IANA names, names ending in one (as
//! Thunderbird writes `/mozilla.org/…/Europe/Berlin`) and the Windows
//! names Exchange and Outlook use. `VTIMEZONE` rules themselves are not
//! read; an unknown zone falls back to the caller's.

use jiff::Timestamp;
use jiff::civil::{Date, DateTime, Time};
use jiff::tz::TimeZone;
use katna_store::calendar::{Attendee, EventData, EventKind, EventStatus};

use crate::recurrence::{Rule, last_start};

mod write;

pub use write::{escape, fold, recurrence_lines, time_value, write_calendar, write_event};

/// One content line: `NAME;PARAM=value:VALUE`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Property {
    /// Upper case.
    pub name: String,
    /// Names upper case; values without their quotes.
    pub params: Vec<(String, String)>,
    /// As written (still escaped for text).
    pub value: String,
}

impl Property {
    /// The value of parameter `name` (any case).
    pub fn param(&self, name: &str) -> Option<&str> {
        self.params
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }

    /// The value read as text: escapes undone.
    pub fn text(&self) -> String {
        unescape(&self.value)
    }
}

/// A component (`VCALENDAR`, `VEVENT`, `VTODO`, `VALARM`…) with its
/// properties and the components inside it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Component {
    /// Upper case.
    pub name: String,
    pub properties: Vec<Property>,
    pub children: Vec<Component>,
}

impl Component {
    /// The first property `name` (upper case).
    pub fn property(&self, name: &str) -> Option<&Property> {
        self.properties.iter().find(|p| p.name == name)
    }

    /// Every property `name` (upper case).
    pub fn all<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Property> + 'a {
        self.properties.iter().filter(move |p| p.name == name)
    }

    /// The text of the first property `name`, or empty.
    pub fn text(&self, name: &str) -> String {
        self.property(name).map(Property::text).unwrap_or_default()
    }

    /// The components `name` at any depth below this one.
    pub fn find<'a>(&'a self, name: &str, out: &mut Vec<&'a Component>) {
        for child in &self.children {
            if child.name == name {
                out.push(child);
            }
            child.find(name, out);
        }
    }
}

/// Joins folded lines (a line break followed by a space or tab goes on the
/// line before), taking `\r\n` or `\n`.
pub fn unfold(text: &str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    for raw in text.split('\n') {
        let line = raw.strip_suffix('\r').unwrap_or(raw);
        if let Some(rest) = line.strip_prefix([' ', '\t'])
            && let Some(last) = lines.last_mut()
        {
            last.push_str(rest);
            continue;
        }
        if !line.is_empty() {
            lines.push(line.to_owned());
        }
    }
    lines
}

/// Reads one unfolded content line; `None` if it has no `:`.
pub fn property(line: &str) -> Option<Property> {
    // The name ends at the first `;` or `:`.
    let name_end = line.find([';', ':'])?;
    let name = line[..name_end].trim().to_ascii_uppercase();
    let mut params = Vec::new();
    let mut rest = &line[name_end..];
    while let Some(after) = rest.strip_prefix(';') {
        let eq = after.find('=')?;
        let pname = after[..eq].trim().to_ascii_uppercase();
        let mut value = String::new();
        let mut quoted = false;
        let mut end = after.len() - eq - 1;
        for (i, c) in after[eq + 1..].char_indices() {
            match c {
                '"' => quoted = !quoted,
                ';' | ':' if !quoted => {
                    end = i;
                    break;
                }
                _ => value.push(c),
            }
        }
        params.push((pname, value));
        rest = &after[eq + 1 + end..];
    }
    let value = rest.strip_prefix(':')?;
    Some(Property {
        name,
        params,
        value: value.to_owned(),
    })
}

/// The components of an iCalendar text, usually one `VCALENDAR`.
/// Properties outside any component are dropped.
pub fn components(text: &str) -> Vec<Component> {
    let mut stack: Vec<Component> = Vec::new();
    let mut top = Vec::new();
    for line in unfold(text) {
        let Some(prop) = property(&line) else {
            continue;
        };
        match prop.name.as_str() {
            "BEGIN" => stack.push(Component {
                name: prop.value.trim().to_ascii_uppercase(),
                ..Component::default()
            }),
            "END" => {
                let name = prop.value.trim().to_ascii_uppercase();
                // Close up to the matching BEGIN; a stray END is ignored.
                if !stack.iter().any(|c| c.name == name) {
                    continue;
                }
                while let Some(done) = stack.pop() {
                    let matched = done.name == name;
                    match stack.last_mut() {
                        Some(parent) => parent.children.push(done),
                        None => top.push(done),
                    }
                    if matched {
                        break;
                    }
                }
            }
            _ => {
                if let Some(current) = stack.last_mut() {
                    current.properties.push(prop);
                }
            }
        }
    }
    // Unclosed components still count.
    while let Some(done) = stack.pop() {
        match stack.last_mut() {
            Some(parent) => parent.children.push(done),
            None => top.push(done),
        }
    }
    top
}

/// Undoes text escapes: `\n`, `\N`, `\,`, `\;`, `\\`.
pub fn unescape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n' | 'N') => out.push('\n'),
            Some(other) => out.push(other),
            None => out.push('\\'),
        }
    }
    out
}

/// Windows time zone names (Exchange, Outlook, Microsoft Graph) and the
/// IANA zone of each, after CLDR's `windowsZones.xml` (territory `001`).
const WINDOWS_ZONES: &[(&str, &str)] = &[
    ("Dateline Standard Time", "Etc/GMT+12"),
    ("UTC-11", "Etc/GMT+11"),
    ("Hawaiian Standard Time", "Pacific/Honolulu"),
    ("Alaskan Standard Time", "America/Anchorage"),
    ("Pacific Standard Time (Mexico)", "America/Tijuana"),
    ("Pacific Standard Time", "America/Los_Angeles"),
    ("US Mountain Standard Time", "America/Phoenix"),
    ("Mountain Standard Time (Mexico)", "America/Mazatlan"),
    ("Mountain Standard Time", "America/Denver"),
    ("Central America Standard Time", "America/Guatemala"),
    ("Central Standard Time", "America/Chicago"),
    ("Central Standard Time (Mexico)", "America/Mexico_City"),
    ("Canada Central Standard Time", "America/Regina"),
    ("SA Pacific Standard Time", "America/Bogota"),
    ("Eastern Standard Time (Mexico)", "America/Cancun"),
    ("Eastern Standard Time", "America/New_York"),
    ("US Eastern Standard Time", "America/Indianapolis"),
    ("Venezuela Standard Time", "America/Caracas"),
    ("Paraguay Standard Time", "America/Asuncion"),
    ("Atlantic Standard Time", "America/Halifax"),
    ("Central Brazilian Standard Time", "America/Cuiaba"),
    ("SA Western Standard Time", "America/La_Paz"),
    ("Pacific SA Standard Time", "America/Santiago"),
    ("Newfoundland Standard Time", "America/St_Johns"),
    ("E. South America Standard Time", "America/Sao_Paulo"),
    ("SA Eastern Standard Time", "America/Cayenne"),
    ("Argentina Standard Time", "America/Buenos_Aires"),
    ("Greenland Standard Time", "America/Godthab"),
    ("Montevideo Standard Time", "America/Montevideo"),
    ("UTC-02", "Etc/GMT+2"),
    ("Azores Standard Time", "Atlantic/Azores"),
    ("Cape Verde Standard Time", "Atlantic/Cape_Verde"),
    ("UTC", "Etc/UTC"),
    ("GMT Standard Time", "Europe/London"),
    ("Greenwich Standard Time", "Atlantic/Reykjavik"),
    ("Morocco Standard Time", "Africa/Casablanca"),
    ("W. Europe Standard Time", "Europe/Berlin"),
    ("Central Europe Standard Time", "Europe/Budapest"),
    ("Romance Standard Time", "Europe/Paris"),
    ("Central European Standard Time", "Europe/Warsaw"),
    ("W. Central Africa Standard Time", "Africa/Lagos"),
    ("GTB Standard Time", "Europe/Bucharest"),
    ("Middle East Standard Time", "Asia/Beirut"),
    ("Egypt Standard Time", "Africa/Cairo"),
    ("E. Europe Standard Time", "Europe/Chisinau"),
    ("South Africa Standard Time", "Africa/Johannesburg"),
    ("FLE Standard Time", "Europe/Kiev"),
    ("Israel Standard Time", "Asia/Jerusalem"),
    ("Jordan Standard Time", "Asia/Amman"),
    ("Arabic Standard Time", "Asia/Baghdad"),
    ("Turkey Standard Time", "Europe/Istanbul"),
    ("Arab Standard Time", "Asia/Riyadh"),
    ("Russian Standard Time", "Europe/Moscow"),
    ("E. Africa Standard Time", "Africa/Nairobi"),
    ("Iran Standard Time", "Asia/Tehran"),
    ("Arabian Standard Time", "Asia/Dubai"),
    ("Azerbaijan Standard Time", "Asia/Baku"),
    ("Georgian Standard Time", "Asia/Tbilisi"),
    ("Caucasus Standard Time", "Asia/Yerevan"),
    ("Afghanistan Standard Time", "Asia/Kabul"),
    ("West Asia Standard Time", "Asia/Tashkent"),
    ("Ekaterinburg Standard Time", "Asia/Yekaterinburg"),
    ("Pakistan Standard Time", "Asia/Karachi"),
    ("India Standard Time", "Asia/Kolkata"),
    ("Sri Lanka Standard Time", "Asia/Colombo"),
    ("Nepal Standard Time", "Asia/Katmandu"),
    ("Central Asia Standard Time", "Asia/Almaty"),
    ("Bangladesh Standard Time", "Asia/Dhaka"),
    ("Myanmar Standard Time", "Asia/Rangoon"),
    ("SE Asia Standard Time", "Asia/Bangkok"),
    ("N. Central Asia Standard Time", "Asia/Novosibirsk"),
    ("China Standard Time", "Asia/Shanghai"),
    ("North Asia Standard Time", "Asia/Krasnoyarsk"),
    ("Singapore Standard Time", "Asia/Singapore"),
    ("W. Australia Standard Time", "Australia/Perth"),
    ("Taipei Standard Time", "Asia/Taipei"),
    ("Ulaanbaatar Standard Time", "Asia/Ulaanbaatar"),
    ("Tokyo Standard Time", "Asia/Tokyo"),
    ("Korea Standard Time", "Asia/Seoul"),
    ("Cen. Australia Standard Time", "Australia/Adelaide"),
    ("AUS Central Standard Time", "Australia/Darwin"),
    ("E. Australia Standard Time", "Australia/Brisbane"),
    ("AUS Eastern Standard Time", "Australia/Sydney"),
    ("West Pacific Standard Time", "Pacific/Port_Moresby"),
    ("Tasmania Standard Time", "Australia/Hobart"),
    ("Vladivostok Standard Time", "Asia/Vladivostok"),
    ("Central Pacific Standard Time", "Pacific/Guadalcanal"),
    ("New Zealand Standard Time", "Pacific/Auckland"),
    ("Fiji Standard Time", "Pacific/Fiji"),
    ("Tonga Standard Time", "Pacific/Tongatapu"),
    ("Samoa Standard Time", "Pacific/Apia"),
];

/// The time zone a `TZID` (or a Windows zone name) names: an IANA name,
/// a path ending in one, or a Windows name. `None` when unknown.
pub fn zone(tzid: &str) -> Option<TimeZone> {
    let name = zone_name(tzid)?;
    TimeZone::get(&name).ok()
}

/// The Windows name of IANA zone `iana` (what Exchange and Microsoft
/// Graph take), when it is one of those [`WINDOWS_ZONES`] lists.
pub fn windows_zone(iana: &str) -> Option<&'static str> {
    if matches!(iana, "UTC" | "Etc/UTC") {
        return Some("UTC");
    }
    WINDOWS_ZONES
        .iter()
        .find(|(_, name)| *name == iana)
        .map(|(windows, _)| *windows)
}

/// The IANA name of the zone `tzid` names, as [`zone`] finds it.
pub fn zone_name(tzid: &str) -> Option<String> {
    let tzid = tzid.trim().trim_matches('"');
    if tzid.is_empty() {
        return None;
    }
    if TimeZone::get(tzid).is_ok() {
        return Some(tzid.to_owned());
    }
    if let Some((_, iana)) = WINDOWS_ZONES
        .iter()
        .find(|(windows, _)| windows.eq_ignore_ascii_case(tzid))
    {
        return Some((*iana).to_owned());
    }
    if matches!(tzid, "tzone://Microsoft/Utc" | "Z" | "GMT") {
        return Some("UTC".to_owned());
    }
    // `/mozilla.org/20050126_1/Europe/Berlin`, `/citadel.org/…/Europe/Rome`:
    // the last one or two parts.
    let parts: Vec<&str> = tzid.split('/').filter(|p| !p.is_empty()).collect();
    for n in [3, 2, 1] {
        if parts.len() >= n {
            let tail = parts[parts.len() - n..].join("/");
            if TimeZone::get(&tail).is_ok() {
                return Some(tail);
            }
        }
    }
    None
}

fn digits(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())
}

/// `YYYYMMDD`.
pub fn parse_date(text: &str) -> Option<Date> {
    let text = text.trim();
    if text.len() != 8 || !digits(text) {
        return None;
    }
    Date::new(
        text[..4].parse().ok()?,
        text[4..6].parse().ok()?,
        text[6..8].parse().ok()?,
    )
    .ok()
}

/// Unix seconds of UTC midnight of `day`: how whole days are stored.
pub fn utc_midnight(day: Date) -> i64 {
    day.to_zoned(TimeZone::UTC)
        .map_or(0, |z| z.timestamp().as_second())
}

/// A date or date-time value: `YYYYMMDD` (a whole day, returned as its
/// UTC midnight), `YYYYMMDDTHHMMSSZ` (UTC), or a local time in `tz`.
/// Returns Unix seconds and whether it was a whole day.
pub fn parse_date_time(text: &str, tz: &TimeZone) -> Option<(i64, bool)> {
    let text = text.trim();
    let Some((day, time)) = text.split_once('T') else {
        return parse_date(text).map(|d| (utc_midnight(d), true));
    };
    let day = parse_date(day)?;
    let utc = time.ends_with('Z') || time.ends_with('z');
    let time = time.trim_end_matches(['Z', 'z']);
    if time.len() < 4 || !digits(time) {
        return None;
    }
    let second = if time.len() >= 6 {
        time[4..6].parse().ok()?
    } else {
        0
    };
    // A leap second (`…60`) is read as the one before it.
    let time = Time::new(
        time[..2].parse().ok()?,
        time[2..4].parse().ok()?,
        second.min(59),
        0,
    )
    .ok()?;
    let local = DateTime::from_parts(day, time);
    let zone = if utc { TimeZone::UTC } else { tz.clone() };
    let zoned = local.to_zoned(zone).ok()?;
    Some((zoned.timestamp().as_second(), false))
}

/// A property's date or date-time, read in its `TZID` when it has a known
/// one, else in `fallback`. `VALUE=DATE` makes it a whole day.
pub fn date_time(prop: &Property, fallback: &TimeZone) -> Option<(i64, bool)> {
    let tz = prop.param("TZID").and_then(zone);
    let value = prop.value.split('/').next().unwrap_or_default();
    if prop
        .param("VALUE")
        .is_some_and(|v| v.eq_ignore_ascii_case("DATE"))
    {
        let day = value.split('T').next().unwrap_or_default();
        return parse_date(day).map(|d| (utc_midnight(d), true));
    }
    parse_date_time(value, tz.as_ref().unwrap_or(fallback))
}

/// Every date of a list property (`EXDATE`, `RDATE`): comma-separated,
/// periods (`start/end`) counted by their start.
pub fn date_list(prop: &Property, fallback: &TimeZone) -> Vec<i64> {
    prop.value
        .split(',')
        .filter_map(|value| {
            let single = Property {
                value: value.to_owned(),
                ..prop.clone()
            };
            date_time(&single, fallback).map(|(s, _)| s)
        })
        .collect()
}

/// An RFC 5545 duration (`PT15M`, `-P1D`, `P1W`, `P1DT2H`) in seconds.
pub fn duration(text: &str) -> Option<i64> {
    let text = text.trim();
    let (sign, text) = match text.as_bytes().first()? {
        b'-' => (-1, &text[1..]),
        b'+' => (1, &text[1..]),
        _ => (1, text),
    };
    let text = text.strip_prefix(['P', 'p'])?;
    let mut total = 0i64;
    let mut number = String::new();
    let mut in_time = false;
    let mut any = false;
    for c in text.chars() {
        match c.to_ascii_uppercase() {
            '0'..='9' => number.push(c),
            'T' => in_time = true,
            unit => {
                let n: i64 = number.parse().ok()?;
                number.clear();
                any = true;
                total += n * match (unit, in_time) {
                    ('W', false) => 7 * 86_400,
                    ('D', false) => 86_400,
                    ('H', true) => 3_600,
                    ('M', true) => 60,
                    ('S', true) => 1,
                    _ => return None,
                };
            }
        }
    }
    (any && number.is_empty()).then_some(sign * total)
}

/// An address from `mailto:…`.
fn address(value: &str) -> String {
    let value = value.trim();
    let bare = if value.len() >= 7 && value[..7].eq_ignore_ascii_case("mailto:") {
        &value[7..]
    } else {
        value
    };
    bare.to_owned()
}

/// A `PARTSTAT` as the store keeps it.
fn part_stat(value: Option<&str>) -> String {
    match value.map(str::to_ascii_uppercase).as_deref() {
        Some("ACCEPTED") => "accepted",
        Some("TENTATIVE") => "tentative",
        Some("DECLINED") => "declined",
        Some("NEEDS-ACTION") => "needs_action",
        _ => "",
    }
    .to_owned()
}

/// A link to a video call, if `value` is an `https` one.
fn https(value: &str) -> Option<String> {
    let value = value.trim();
    value
        .get(..8)
        .is_some_and(|s| s.eq_ignore_ascii_case("https://"))
        .then(|| value.to_owned())
}

/// Unix seconds of an iCalendar UTC stamp (`LAST-MODIFIED`, `DTSTAMP`).
fn stamp(prop: Option<&Property>) -> i64 {
    prop.and_then(|p| parse_date_time(&p.value, &TimeZone::UTC))
        .map_or(0, |(s, _)| s)
}

/// Reads one `VEVENT`. Times without a known zone are read in `fallback`;
/// `self_address` finds the user among the attendees. `None` without a
/// readable `DTSTART` (or, for a cancelled occurrence, `RECURRENCE-ID`).
pub fn event(component: &Component, fallback: &TimeZone, self_address: &str) -> Option<EventData> {
    let start_prop = component.property("DTSTART");
    let zone_of_start = start_prop.and_then(|p| p.param("TZID")).and_then(zone_name);
    let tz = zone_of_start
        .as_deref()
        .and_then(|name| TimeZone::get(name).ok())
        .unwrap_or_else(|| fallback.clone());
    let recurrence_id = component
        .property("RECURRENCE-ID")
        .and_then(|p| date_time(p, &tz))
        .map(|(s, _)| s);
    let (start, all_day) = match start_prop.and_then(|p| date_time(p, &tz)) {
        Some(found) => found,
        None => (recurrence_id?, false),
    };
    let end = if let Some(end) = component.property("DTEND").and_then(|p| date_time(p, &tz)) {
        end.0
    } else if let Some(length) = component
        .property("DURATION")
        .and_then(|p| duration(&p.value))
    {
        start + length
    } else if all_day {
        start + 86_400
    } else {
        start
    };
    let end = end.max(start);

    let mut data = EventData {
        uid: component.text("UID"),
        recurrence_id,
        title: component.text("SUMMARY"),
        location: component.text("LOCATION"),
        description: component.text("DESCRIPTION"),
        start,
        end,
        all_day,
        time_zone: if all_day {
            String::new()
        } else {
            zone_of_start
                .or_else(|| fallback.iana_name().map(str::to_owned))
                .unwrap_or_default()
        },
        busy: true,
        web_link: component
            .property("URL")
            .map(|p| p.value.trim().to_owned())
            .unwrap_or_default(),
        ..EventData::default()
    };
    data.status = match component.text("STATUS").to_ascii_uppercase().as_str() {
        "CANCELLED" => EventStatus::Cancelled,
        "TENTATIVE" => EventStatus::Tentative,
        _ => EventStatus::Confirmed,
    };
    if component.text("TRANSP").eq_ignore_ascii_case("TRANSPARENT") {
        data.busy = false;
    }
    match component
        .text("X-MICROSOFT-CDO-BUSYSTATUS")
        .to_ascii_uppercase()
        .as_str()
    {
        "FREE" => data.busy = false,
        "OOF" => data.kind = EventKind::OutOfOffice,
        _ => {}
    }
    let own = EventKind::parse(&component.text("X-KATNA-KIND").to_ascii_lowercase());
    if matches!(own, EventKind::Focus | EventKind::WorkingLocation) {
        data.kind = own;
    }
    if let Some(rule) = component.property("RRULE")
        && recurrence_id.is_none()
    {
        data.rrule = rule.value.trim().to_owned();
    }
    for prop in component.all("EXDATE") {
        data.exdates.extend(date_list(prop, &tz));
    }
    for prop in component.all("RDATE") {
        data.rdates.extend(date_list(prop, &tz));
    }
    if let Some(color) = component.property("COLOR")
        && color.value.starts_with('#')
        && color.value.len() == 7
    {
        data.color = color.value.to_ascii_lowercase();
    }
    if let Some(organizer) = component.property("ORGANIZER") {
        data.organizer = address(&organizer.value);
        data.organizer_name = organizer.param("CN").unwrap_or_default().to_owned();
    }
    let is_self =
        |email: &str| !self_address.is_empty() && email.eq_ignore_ascii_case(self_address);
    for prop in component.all("ATTENDEE") {
        let email = address(&prop.value);
        let attendee = Attendee {
            status: part_stat(prop.param("PARTSTAT")),
            name: prop.param("CN").unwrap_or_default().to_owned(),
            optional: prop
                .param("ROLE")
                .is_some_and(|r| r.eq_ignore_ascii_case("OPT-PARTICIPANT")),
            is_self: is_self(&email),
            organizer: email.eq_ignore_ascii_case(&data.organizer),
            email,
        };
        if attendee.is_self {
            data.self_status = attendee.status.clone();
        }
        data.attendees.push(attendee);
    }
    if data.self_status.is_empty() && !data.organizer.is_empty() && is_self(&data.organizer) {
        data.self_status = "accepted".to_owned();
    }
    for name in [
        "CONFERENCE",
        "X-GOOGLE-CONFERENCE",
        "X-MICROSOFT-SKYPETEAMSMEETINGURL",
    ] {
        if let Some(url) = component.property(name).and_then(|p| https(&p.value)) {
            data.join_url = url;
            break;
        }
    }
    let mut alarms = Vec::new();
    component.find("VALARM", &mut alarms);
    for alarm in alarms {
        if let Some(trigger) = alarm.property("TRIGGER")
            && !trigger
                .param("RELATED")
                .is_some_and(|r| r.eq_ignore_ascii_case("END"))
            && let Some(before) = duration(&trigger.value)
            && before <= 0
        {
            let minutes = -before / 60;
            if !data.reminders.contains(&minutes) {
                data.reminders.push(minutes);
            }
        }
    }
    data.updated_at = stamp(
        component
            .property("LAST-MODIFIED")
            .or_else(|| component.property("DTSTAMP")),
    );
    data.range_end = range_end(&data);
    Some(data)
}

/// Every `VEVENT` of an iCalendar text (a CalDAV resource: a series and
/// its changed occurrences, or one event), read as [`event`] does.
pub fn parse_events(text: &str, fallback: &TimeZone, self_address: &str) -> Vec<EventData> {
    let mut events = Vec::new();
    for top in components(text) {
        let mut found = Vec::new();
        if top.name == "VEVENT" {
            found.push(&top);
        }
        top.find("VEVENT", &mut found);
        events.extend(
            found
                .into_iter()
                .filter_map(|c| event(c, fallback, self_address)),
        );
    }
    events
}

/// Where an event's last occurrence ends ([`EventData::range_end`]): its
/// own end when it doesn't repeat (or is a changed occurrence), `None`
/// when its series never ends.
pub fn range_end(event: &EventData) -> Option<i64> {
    if event.rrule.is_empty() || event.recurrence_id.is_some() {
        return Some(event.end);
    }
    let zone = if event.all_day {
        TimeZone::UTC
    } else {
        TimeZone::get(&event.time_zone).unwrap_or_else(|_| TimeZone::system())
    };
    // A rule that can't be read shows the event once.
    let Some(rule) = Rule::parse(&event.rrule, &zone) else {
        return Some(event.end);
    };
    let length = event.end - event.start;
    let first = Timestamp::from_second(event.start).ok()?;
    let last = last_start(&rule, first, &zone)?.as_second();
    let last = event.rdates.iter().copied().fold(last, i64::max);
    Some(last + length)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(text: &str, zone: &str) -> i64 {
        let tz = TimeZone::get(zone).unwrap();
        text.parse::<DateTime>()
            .unwrap()
            .to_zoned(tz)
            .unwrap()
            .timestamp()
            .as_second()
    }

    #[test]
    fn content_lines_unfold_and_keep_quoted_parameters() {
        let text = "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nSUMMARY:Long \r\n title\r\n\
                    ATTENDEE;CN=\"Doe: Jane\";PARTSTAT=ACCEPTED:mailto:jane@example.com\r\n\
                    END:VEVENT\r\nEND:VCALENDAR\r\n";
        let top = components(text);
        assert_eq!(top.len(), 1);
        let event = &top[0].children[0];
        assert_eq!(event.name, "VEVENT");
        assert_eq!(event.text("SUMMARY"), "Long title");
        let attendee = event.property("ATTENDEE").unwrap();
        assert_eq!(attendee.param("cn"), Some("Doe: Jane"));
        assert_eq!(attendee.param("PARTSTAT"), Some("ACCEPTED"));
        assert_eq!(attendee.value, "mailto:jane@example.com");
    }

    #[test]
    fn text_unescapes() {
        assert_eq!(
            unescape(r"One\, two\; three\nfour\\five\Nsix"),
            "One, two; three\nfour\\five\nsix"
        );
    }

    #[test]
    fn durations() {
        assert_eq!(duration("PT15M"), Some(900));
        assert_eq!(duration("-PT1H30M"), Some(-5400));
        assert_eq!(duration("P1W"), Some(604_800));
        assert_eq!(duration("P1DT2H"), Some(93_600));
        assert_eq!(duration("-P0D"), Some(0));
        assert_eq!(duration("PT"), None);
        assert_eq!(duration("15M"), None);
    }

    #[test]
    fn zones_by_iana_windows_and_path_names() {
        assert_eq!(zone_name("Europe/Berlin").as_deref(), Some("Europe/Berlin"));
        assert_eq!(
            zone_name("India Standard Time").as_deref(),
            Some("Asia/Kolkata")
        );
        assert_eq!(
            zone_name("/mozilla.org/20050126_1/America/New_York").as_deref(),
            Some("America/New_York")
        );
        assert_eq!(zone_name("Nowhere Standard Time"), None);
        assert_eq!(windows_zone("Asia/Kolkata"), Some("India Standard Time"));
        assert_eq!(
            windows_zone("Europe/Berlin"),
            Some("W. Europe Standard Time")
        );
        assert_eq!(windows_zone("Mars/Olympus"), None);
    }

    #[test]
    fn a_series_with_tzid_exdates_and_a_changed_occurrence() {
        let text = "BEGIN:VCALENDAR\n\
            VERSION:2.0\n\
            BEGIN:VTIMEZONE\n\
            TZID:America/New_York\n\
            END:VTIMEZONE\n\
            BEGIN:VEVENT\n\
            UID:standup@example.com\n\
            DTSTAMP:20260901T120000Z\n\
            DTSTART;TZID=America/New_York:20260928T090000\n\
            DTEND;TZID=America/New_York:20260928T091500\n\
            RRULE:FREQ=DAILY;COUNT=10\n\
            EXDATE;TZID=America/New_York:20260929T090000,20260930T090000\n\
            EXDATE;TZID=America/New_York:20261001T090000\n\
            SUMMARY:Standup\\, daily\n\
            LOCATION:Room 1\n\
            DESCRIPTION:Line one\\nLine two\n\
            ORGANIZER;CN=Boss:mailto:boss@example.com\n\
            ATTENDEE;CN=Me;PARTSTAT=TENTATIVE:mailto:ME@example.com\n\
            ATTENDEE;ROLE=OPT-PARTICIPANT;PARTSTAT=NEEDS-ACTION:mailto:x@example.com\n\
            X-GOOGLE-CONFERENCE:https://meet.example.com/abc\n\
            BEGIN:VALARM\n\
            ACTION:DISPLAY\n\
            TRIGGER:-PT10M\n\
            END:VALARM\n\
            END:VEVENT\n\
            BEGIN:VEVENT\n\
            UID:standup@example.com\n\
            RECURRENCE-ID;TZID=America/New_York:20261002T090000\n\
            DTSTART;TZID=America/New_York:20261002T100000\n\
            DURATION:PT30M\n\
            SUMMARY:Standup (moved)\n\
            END:VEVENT\n\
            END:VCALENDAR\n";
        let events = parse_events(text, &TimeZone::UTC, "me@example.com");
        assert_eq!(events.len(), 2);
        let series = &events[0];
        let start = at("2026-09-28T09:00", "America/New_York");
        assert_eq!(series.start, start);
        assert_eq!(series.end, start + 900);
        assert!(!series.all_day);
        assert_eq!(series.time_zone, "America/New_York");
        assert_eq!(series.rrule, "FREQ=DAILY;COUNT=10");
        assert_eq!(
            series.exdates,
            [start + 86_400, start + 2 * 86_400, start + 3 * 86_400]
        );
        assert_eq!(series.title, "Standup, daily");
        assert_eq!(series.description, "Line one\nLine two");
        assert_eq!(series.organizer, "boss@example.com");
        assert_eq!(series.organizer_name, "Boss");
        assert_eq!(series.self_status, "tentative");
        assert!(series.attendees[0].is_self);
        assert!(series.attendees[1].optional);
        assert_eq!(series.attendees[1].status, "needs_action");
        assert_eq!(series.join_url, "https://meet.example.com/abc");
        assert_eq!(series.reminders, [10]);
        assert_eq!(series.updated_at, at("2026-09-01T12:00", "UTC"));
        // Ten days, the last on 7 October at 9:15 New York time.
        assert_eq!(
            series.range_end,
            Some(at("2026-10-07T09:15", "America/New_York"))
        );

        let moved = &events[1];
        assert_eq!(moved.uid, "standup@example.com");
        assert_eq!(
            moved.recurrence_id,
            Some(at("2026-10-02T09:00", "America/New_York"))
        );
        assert_eq!(moved.start, at("2026-10-02T10:00", "America/New_York"));
        assert_eq!(moved.end, moved.start + 1800);
        assert!(moved.rrule.is_empty());
        assert_eq!(moved.range_end, Some(moved.end));
    }

    #[test]
    fn whole_days_are_utc_midnights() {
        let text = "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nUID:b\r\n\
            DTSTART;VALUE=DATE:20261224\r\nDTEND;VALUE=DATE:20261226\r\n\
            RRULE:FREQ=YEARLY\r\nEXDATE;VALUE=DATE:20271224\r\n\
            TRANSP:TRANSPARENT\r\nSTATUS:TENTATIVE\r\nEND:VEVENT\r\nEND:VCALENDAR\r\n";
        let events = parse_events(text, &TimeZone::get("Asia/Kolkata").unwrap(), "");
        let event = &events[0];
        assert!(event.all_day);
        assert_eq!(event.start, at("2026-12-24T00:00", "UTC"));
        assert_eq!(event.end, at("2026-12-26T00:00", "UTC"));
        assert_eq!(event.exdates, [at("2027-12-24T00:00", "UTC")]);
        assert!(!event.busy);
        assert_eq!(event.status, EventStatus::Tentative);
        assert_eq!(event.range_end, None, "a yearly series never ends");
        assert!(event.time_zone.is_empty());
    }

    #[test]
    fn a_date_without_value_date_and_floating_and_utc_times() {
        let kolkata = TimeZone::get("Asia/Kolkata").unwrap();
        assert_eq!(
            parse_date_time("20260929", &kolkata),
            Some((at("2026-09-29T00:00", "UTC"), true))
        );
        assert_eq!(
            parse_date_time("20260929T100000", &kolkata),
            Some((at("2026-09-29T10:00", "Asia/Kolkata"), false))
        );
        assert_eq!(
            parse_date_time("20260929T100000Z", &kolkata),
            Some((at("2026-09-29T10:00", "UTC"), false))
        );
        assert_eq!(parse_date_time("2026-09-29", &kolkata), None);
        // RDATE periods count by their start.
        let rdate =
            property("RDATE;VALUE=PERIOD:20261001T100000Z/PT1H,20261002T100000Z/PT1H").unwrap();
        assert_eq!(
            date_list(&rdate, &kolkata),
            [at("2026-10-01T10:00", "UTC"), at("2026-10-02T10:00", "UTC")]
        );
    }

    #[test]
    fn a_cancelled_occurrence_without_dtstart_keeps_its_place() {
        let text =
            "BEGIN:VEVENT\nUID:x\nRECURRENCE-ID:20261005T080000Z\nSTATUS:CANCELLED\nEND:VEVENT\n";
        let events = parse_events(text, &TimeZone::UTC, "");
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].status, EventStatus::Cancelled);
        assert_eq!(events[0].recurrence_id, Some(events[0].start));
    }

    #[test]
    fn events_without_a_start_are_skipped() {
        let text =
            "BEGIN:VCALENDAR\nBEGIN:VEVENT\nUID:x\nSUMMARY:No time\nEND:VEVENT\nEND:VCALENDAR\n";
        assert!(parse_events(text, &TimeZone::UTC, "").is_empty());
    }
}
