// SPDX-License-Identifier: GPL-3.0-or-later

//! Contacts from a CSV file (`docs/ARCHITECTURE.md` §8.6), as Google
//! Contacts, Outlook and Thunderbird export them: each column is known by
//! its heading, in any of their spellings; others are left out.

use katna_core::contact::{Card, PostalAddress, Typed};
use katna_dav::vcard::Parsed;

/// The rows of `text`, each a list of fields: quoted fields may hold
/// commas, quotes (doubled) and line breaks.
fn rows(text: &str) -> Vec<Vec<String>> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' if quoted => {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    field.push('"');
                } else {
                    quoted = false;
                }
            }
            '"' if field.is_empty() => quoted = true,
            ',' if !quoted => row.push(std::mem::take(&mut field)),
            '\r' if !quoted => {}
            '\n' if !quoted => {
                row.push(std::mem::take(&mut field));
                rows.push(std::mem::take(&mut row));
            }
            c => field.push(c),
        }
    }
    if !field.is_empty() || !row.is_empty() {
        row.push(field);
        rows.push(row);
    }
    rows.retain(|r| r.iter().any(|f| !f.trim().is_empty()));
    rows
}

/// A label as a kind: `home`, `work`, `mobile`, `other`, or its own name.
fn kind(label: &str) -> String {
    let label = label.trim().trim_start_matches('*').trim().to_lowercase();
    match label.as_str() {
        "business" | "work" | "company" => "work".into(),
        "cell" | "mobile" | "mobile number" => "mobile".into(),
        _ => label,
    }
}

/// The values in a field: Google puts several in one, split by `:::`.
fn values(field: &str) -> impl Iterator<Item = String> + '_ {
    field
        .split(":::")
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(str::to_owned)
}

/// `value` as a card keeps a birthday (`YYYY-MM-DD` or `--MM-DD`), from
/// those forms or Outlook's month/day/year; `None` when it isn't one.
fn birthday(value: &str) -> Option<String> {
    let value = value.trim();
    let date = |y: i16, m: i8, d: i8| jiff::civil::Date::new(y, m, d).ok();
    if let Some(rest) = value.strip_prefix("--") {
        let (m, d) = rest.split_once('-')?;
        date(2000, m.parse().ok()?, d.parse().ok()?)?;
        return Some(format!("--{rest}"));
    }
    if let Ok(day) = value.parse::<jiff::civil::Date>() {
        return Some(day.to_string());
    }
    let mut parts = value.split('/');
    let (m, d, y) = (parts.next()?, parts.next()?, parts.next()?);
    let day = date(
        y.trim().parse().ok()?,
        m.trim().parse().ok()?,
        d.trim().parse().ok()?,
    )?;
    // Outlook writes 0/0/00 for none.
    (day.year() > 1800).then(|| day.to_string())
}

/// What a column holds.
enum Column {
    Given,
    Middle,
    Family,
    Prefix,
    Suffix,
    Full,
    Nickname,
    Organization,
    JobTitle,
    Department,
    Birthday,
    Notes,
    Labels,
    /// Numbered values with a label column beside them (Google), or a
    /// fixed kind (Outlook, Thunderbird).
    Email(Slot),
    Phone(Slot),
    Website(Slot),
    /// A part of an address.
    Address(Slot, Part),
}

/// Which of several values, and its kind when the heading says it.
#[derive(Clone, PartialEq)]
enum Slot {
    /// Google's `N`, whose kind is in the `… N - Label` column.
    Numbered(String),
    Kind(String),
}

#[derive(Clone, Copy)]
enum Part {
    Street,
    Extended,
    PoBox,
    City,
    Region,
    Postcode,
    Country,
    Formatted,
}

/// What `heading` names, or `None` for a column left out (and Google's
/// label columns, read beside their values).
fn column(heading: &str) -> Option<Column> {
    let h = heading.trim().to_lowercase();
    // Google: "E-mail 1 - Value", "Address 2 - City", "Organization 1 - Name".
    if let Some((what, part)) = h.split_once(" - ") {
        let (what, n) = what.rsplit_once(' ').unwrap_or((what, "1"));
        let slot = Slot::Numbered(n.to_owned());
        return match (what, part) {
            ("e-mail" | "email", "value") => Some(Column::Email(slot)),
            ("phone", "value") => Some(Column::Phone(slot)),
            ("website", "value") => Some(Column::Website(slot)),
            ("organization", "name") => Some(Column::Organization),
            ("organization", "title") => Some(Column::JobTitle),
            ("organization", "department") => Some(Column::Department),
            ("address", part) => {
                let part = match part {
                    "street" => Part::Street,
                    "extended address" => Part::Extended,
                    "po box" => Part::PoBox,
                    "city" => Part::City,
                    "region" => Part::Region,
                    "postal code" => Part::Postcode,
                    "country" => Part::Country,
                    "formatted" => Part::Formatted,
                    _ => return None,
                };
                Some(Column::Address(slot, part))
            }
            _ => None,
        };
    }
    let kind = |k: &str| Slot::Kind(k.to_owned());
    Some(match h.as_str() {
        "first name" | "given name" => Column::Given,
        "middle name" | "additional name" => Column::Middle,
        "last name" | "family name" | "surname" => Column::Family,
        "name prefix" => Column::Prefix,
        "name suffix" | "suffix" => Column::Suffix,
        "name" | "display name" | "full name" => Column::Full,
        "nickname" => Column::Nickname,
        "organization name" | "company" | "organization" => Column::Organization,
        "organization title" | "job title" => Column::JobTitle,
        "organization department" | "department" => Column::Department,
        "birthday" => Column::Birthday,
        "notes" | "note" => Column::Notes,
        "labels" | "group membership" | "categories" => Column::Labels,
        "e-mail address" | "primary email" | "email" | "e-mail" => Column::Email(kind("")),
        "e-mail 2 address" | "e-mail 3 address" | "secondary email" => Column::Email(kind("other")),
        "home phone" | "home phone 2" => Column::Phone(kind("home")),
        "business phone" | "business phone 2" | "work phone" | "company main phone" => {
            Column::Phone(kind("work"))
        }
        "mobile phone" | "mobile number" => Column::Phone(kind("mobile")),
        "other phone" | "primary phone" | "pager" | "pager number" => Column::Phone(kind("other")),
        "home fax" | "business fax" | "other fax" | "fax number" => Column::Phone(kind("fax")),
        "web page" | "web page 1" | "personal web page" => Column::Website(kind("")),
        "web page 2" => Column::Website(kind("work")),
        _ => {
            // Outlook's "Home Street", Thunderbird's "Work City".
            let (place, part) = h.split_once(' ')?;
            let place = match place {
                "home" => "home",
                "business" | "work" => "work",
                "other" => "other",
                _ => return None,
            };
            let part = match part {
                "street" | "address" => Part::Street,
                "street 2" | "street 3" | "address 2" => Part::Extended,
                "po box" => Part::PoBox,
                "city" => Part::City,
                "state" | "state/province" | "region" => Part::Region,
                "postal code" | "zipcode" | "zip code" => Part::Postcode,
                "country/region" | "country" => Part::Country,
                _ => return None,
            };
            Column::Address(kind(place), part)
        }
    })
}

/// The people in CSV `text`, as vCard import reads them.
pub(super) fn parse(text: &str) -> Vec<Parsed> {
    let mut rows = rows(text).into_iter();
    let Some(head) = rows.next() else {
        return Vec::new();
    };
    let columns: Vec<Option<Column>> = head.iter().map(|h| column(h)).collect();
    // Google's "E-mail 1 - Label" (or "- Type") columns, by what they label.
    let label_of = |what: &[&str], n: &str| {
        head.iter().position(|h| {
            let h = h.trim().to_lowercase();
            what.iter()
                .any(|w| h == format!("{w} {n} - label") || h == format!("{w} {n} - type"))
        })
    };
    rows.filter_map(|row| {
        let field = |ix: usize| row.get(ix).map(|f| f.trim()).unwrap_or_default();
        let kind_at = |slot: &Slot, what: &[&str]| match slot {
            Slot::Kind(k) => k.clone(),
            Slot::Numbered(n) => label_of(what, n)
                .map(|ix| kind(field(ix)))
                .unwrap_or_default(),
        };
        let mut card = Card::default();
        let mut labels = Vec::new();
        let mut addresses: Vec<(Slot, PostalAddress, Vec<String>)> = Vec::new();
        for (ix, column) in columns.iter().enumerate() {
            let (Some(column), value) = (column, field(ix)) else {
                continue;
            };
            if value.is_empty() {
                continue;
            }
            let set = |to: &mut String| {
                if to.is_empty() {
                    value.clone_into(to);
                }
            };
            match column {
                Column::Given => set(&mut card.name.given),
                Column::Middle => set(&mut card.name.middle),
                Column::Family => set(&mut card.name.family),
                Column::Prefix => set(&mut card.name.prefix),
                Column::Suffix => set(&mut card.name.suffix),
                Column::Full => set(&mut card.name.full),
                Column::Nickname => set(&mut card.nickname),
                Column::Organization => set(&mut card.organization),
                Column::JobTitle => set(&mut card.title),
                Column::Department => set(&mut card.department),
                Column::Birthday => {
                    if card.birthday.is_empty() {
                        card.birthday = birthday(value).unwrap_or_default();
                    }
                }
                Column::Notes => set(&mut card.note),
                Column::Labels => {
                    for label in value.split(';').flat_map(values) {
                        // Google's own groups: "* myContacts", "* starred".
                        if !label.starts_with('*') && !labels.contains(&label) {
                            labels.push(label);
                        }
                    }
                }
                Column::Email(slot) => {
                    let kind = kind_at(slot, &["e-mail", "email"]);
                    for v in values(value) {
                        card.emails.push(Typed::new(v, kind.clone()));
                    }
                }
                Column::Phone(slot) => {
                    let kind = kind_at(slot, &["phone"]);
                    for v in values(value) {
                        card.phones.push(Typed::new(v, kind.clone()));
                    }
                }
                Column::Website(slot) => {
                    let kind = kind_at(slot, &["website"]);
                    for v in values(value) {
                        card.urls.push(Typed::new(v, kind.clone()));
                    }
                }
                Column::Address(slot, part) => {
                    let at = match addresses.iter().position(|(s, _, _)| s == slot) {
                        Some(at) => at,
                        None => {
                            let kind = kind_at(slot, &["address"]);
                            addresses.push((
                                slot.clone(),
                                PostalAddress {
                                    kind,
                                    ..PostalAddress::default()
                                },
                                Vec::new(),
                            ));
                            addresses.len() - 1
                        }
                    };
                    let (_, address, street) = &mut addresses[at];
                    match part {
                        Part::Street | Part::Extended | Part::PoBox => street.push(value.into()),
                        Part::City => set(&mut address.city),
                        Part::Region => set(&mut address.region),
                        Part::Postcode => set(&mut address.postcode),
                        Part::Country => set(&mut address.country),
                        Part::Formatted => {}
                    }
                }
            }
        }
        // Parts when there are any, else Google's address as one text.
        for (slot, mut address, street) in addresses {
            address.street = street.join("\n");
            if address.is_empty()
                && let Slot::Numbered(n) = &slot
                && let Some(ix) = head
                    .iter()
                    .position(|h| h.trim().to_lowercase() == format!("address {n} - formatted"))
            {
                address.street = field(ix).to_owned();
            }
            if !address.is_empty() {
                card.addresses.push(address);
            }
        }
        // A full name that only repeats the parts is left to them.
        if card.name.full == card.name.shown() && !card.name.given.is_empty() {
            card.name.full.clear();
        }
        (!card.is_empty()).then(|| Parsed {
            uid: String::new(),
            card,
            categories: labels,
            group: false,
            members: Vec::new(),
            photo: None,
        })
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fields_may_be_quoted() {
        assert_eq!(
            rows("\u{feff}a,\"b, c\",\"say \"\"hi\"\"\"\r\n\"two\nlines\",,x\r\n\r\n"),
            [
                vec!["a".to_owned(), "b, c".into(), "say \"hi\"".into()],
                vec!["two\nlines".to_owned(), String::new(), "x".into()],
            ]
        );
    }

    #[test]
    fn google_contacts_csv() {
        let text = "First Name,Middle Name,Last Name,Nickname,Organization Name,Organization Title,Birthday,Notes,Labels,E-mail 1 - Label,E-mail 1 - Value,E-mail 2 - Label,E-mail 2 - Value,Phone 1 - Label,Phone 1 - Value,Address 1 - Label,Address 1 - Formatted,Address 1 - Street,Address 1 - City,Address 1 - Postal Code,Address 1 - Country,Website 1 - Label,Website 1 - Value\n\
            Asha,,Rao,Ash,Acme,Engineer,--03-14,Met at the fair,Suppliers ::: * myContacts ::: * starred,* Work,asha@acme.in,Home,asha@home.in,Mobile,+91 98765 43210 ::: +91 1234 567890,Home,\"12 Park St\nKolkata\",12 Park St,Kolkata,700016,India,Profile,https://asha.in\n\
            ,,,,,,,,* myContacts,,,,,,,,,,,,,,\n";
        let people = parse(text);
        assert_eq!(people.len(), 1, "the empty row is left out");
        let p = &people[0];
        assert_eq!(p.card.name.shown(), "Asha Rao");
        assert_eq!(p.card.nickname, "Ash");
        assert_eq!(p.card.organization, "Acme");
        assert_eq!(p.card.title, "Engineer");
        assert_eq!(p.card.birthday, "--03-14");
        assert_eq!(p.card.note, "Met at the fair");
        assert_eq!(p.categories, ["Suppliers"]);
        assert_eq!(
            p.card.emails,
            [
                Typed::new("asha@acme.in", "work"),
                Typed::new("asha@home.in", "home")
            ]
        );
        assert_eq!(p.card.phones.len(), 2);
        assert_eq!(p.card.phones[1].kind, "mobile");
        assert_eq!(p.card.addresses[0].street, "12 Park St");
        assert_eq!(p.card.addresses[0].city, "Kolkata");
        assert_eq!(p.card.addresses[0].kind, "home");
        assert_eq!(p.card.urls, [Typed::new("https://asha.in", "profile")]);
    }

    #[test]
    fn outlook_csv() {
        let text = "First Name,Last Name,Company,Job Title,E-mail Address,E-mail 2 Address,Business Phone,Mobile Phone,Home Street,Home City,Home Country/Region,Birthday,Categories\n\
            Bilal,Khan,Globex,Manager,bilal@globex.com,bk@home.net,+1 555 0100,+1 555 0199,1 Main St,Springfield,USA,7/2/1985,Clients;Golf\n";
        let p = &parse(text)[0];
        assert_eq!(p.card.name.shown(), "Bilal Khan");
        assert_eq!(p.card.emails[1], Typed::new("bk@home.net", "other"));
        assert_eq!(p.card.phones[0], Typed::new("+1 555 0100", "work"));
        assert_eq!(p.card.phones[1].kind, "mobile");
        assert_eq!(p.card.addresses[0].kind, "home");
        assert_eq!(p.card.addresses[0].country, "USA");
        assert_eq!(p.card.birthday, "1985-07-02");
        assert_eq!(p.categories, ["Clients", "Golf"]);
    }

    #[test]
    fn birthdays_in_every_form() {
        assert_eq!(birthday("1990-03-14").as_deref(), Some("1990-03-14"));
        assert_eq!(birthday("--02-29").as_deref(), Some("--02-29"));
        assert_eq!(birthday("12/31/1999").as_deref(), Some("1999-12-31"));
        assert_eq!(birthday("0/0/00"), None);
        assert_eq!(birthday("soon"), None);
    }

    #[test]
    fn a_display_name_alone_is_kept() {
        let p = &parse("Display Name,Primary Email\nChen Wei,chen@x.cn\n")[0];
        assert_eq!(p.card.name.full, "Chen Wei");
        assert_eq!(p.card.emails[0].value, "chen@x.cn");
    }
}
