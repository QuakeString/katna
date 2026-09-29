// SPDX-License-Identifier: GPL-3.0-or-later

//! A saved contact, as the address books of Google, Microsoft, CardDAV
//! servers and this computer hold it (`docs/ARCHITECTURE.md` §8.6).
//!
//! Every source is read into one [`Card`]; the store keeps it as JSON next
//! to the source's own form, which is what gets written back.

use serde::{Deserialize, Serialize};

/// One value with its kind, such as an email address marked "work".
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Typed {
    pub value: String,
    /// `home`, `work`, `mobile`, `other`, or the source's own label; empty
    /// when the source gives none.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub kind: String,
}

impl Typed {
    pub fn new(value: impl Into<String>, kind: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            kind: kind.into(),
        }
    }
}

/// A postal address.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct PostalAddress {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub kind: String,
    pub street: String,
    pub city: String,
    pub region: String,
    pub postcode: String,
    pub country: String,
}

impl PostalAddress {
    /// The address on as few lines as it needs, empty parts left out.
    pub fn lines(&self) -> Vec<String> {
        let mut lines: Vec<String> = self
            .street
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(str::to_owned)
            .collect();
        let town = [self.city.trim(), self.region.trim(), self.postcode.trim()]
            .into_iter()
            .filter(|p| !p.is_empty())
            .collect::<Vec<_>>()
            .join(", ");
        if !town.is_empty() {
            lines.push(town);
        }
        if !self.country.trim().is_empty() {
            lines.push(self.country.trim().to_owned());
        }
        lines
    }

    pub fn is_empty(&self) -> bool {
        self.lines().is_empty()
    }
}

/// The parts of a person's name.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Name {
    /// The name as the person wants it shown ("Dr. Asha Rao").
    pub full: String,
    pub prefix: String,
    pub given: String,
    pub middle: String,
    pub family: String,
    pub suffix: String,
}

impl Name {
    /// The full name, else the parts joined.
    pub fn shown(&self) -> String {
        let full = self.full.trim();
        if !full.is_empty() {
            return full.to_owned();
        }
        [
            self.prefix.trim(),
            self.given.trim(),
            self.middle.trim(),
            self.family.trim(),
            self.suffix.trim(),
        ]
        .into_iter()
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
    }
}

/// A contact.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Card {
    pub name: Name,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub nickname: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<Typed>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub phones: Vec<Typed>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<PostalAddress>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub organization: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub department: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub title: String,
    /// `YYYY-MM-DD`, or `--MM-DD` when the year is not known.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub birthday: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub urls: Vec<Typed>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub note: String,
    /// Where the picture can be fetched, when the source keeps it apart
    /// from the card (Google, Microsoft); a CardDAV picture is inline and
    /// goes straight to the store.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub photo_url: String,
}

impl Card {
    /// The name to show: the person's name, else the nickname, company or
    /// first email address. Empty only for an empty card.
    pub fn display_name(&self) -> String {
        let name = self.name.shown();
        if !name.is_empty() {
            return name;
        }
        [
            self.nickname.trim(),
            self.organization.trim(),
            self.emails.first().map_or("", |e| e.value.trim()),
            self.phones.first().map_or("", |p| p.value.trim()),
        ]
        .into_iter()
        .find(|s| !s.is_empty())
        .unwrap_or_default()
        .to_owned()
    }

    /// The key the A–Z list sorts by: the shown name in lower case,
    /// without leading punctuation.
    pub fn sort_key(&self) -> String {
        self.display_name()
            .trim_start_matches(|c: char| !c.is_alphanumeric())
            .to_lowercase()
    }

    /// "Title, Company", or whichever of them there is.
    pub fn job(&self) -> String {
        [self.title.trim(), self.organization.trim()]
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// The email addresses, lower case and trimmed, without repeats.
    pub fn email_keys(&self) -> Vec<String> {
        let mut keys: Vec<String> = Vec::new();
        for email in &self.emails {
            let key = email.value.trim().to_lowercase();
            if !key.is_empty() && !keys.contains(&key) {
                keys.push(key);
            }
        }
        keys
    }

    /// Whether the card holds nothing worth keeping.
    pub fn is_empty(&self) -> bool {
        self.display_name().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_name_falls_back_to_company_then_email() {
        let mut card = Card {
            emails: vec![Typed::new("Asha@Example.com ", "work")],
            ..Card::default()
        };
        assert_eq!(card.display_name(), "Asha@Example.com");
        card.organization = "Acme".into();
        assert_eq!(card.display_name(), "Acme");
        card.name.given = "Asha".into();
        card.name.family = "Rao".into();
        assert_eq!(card.display_name(), "Asha Rao");
        card.name.full = "Dr. Asha Rao".into();
        assert_eq!(card.display_name(), "Dr. Asha Rao");
        assert_eq!(card.sort_key(), "dr. asha rao");
        assert_eq!(card.email_keys(), ["asha@example.com"]);
    }

    #[test]
    fn job_and_address_lines_skip_empty_parts() {
        let card = Card {
            title: "Buyer".into(),
            organization: "Acme".into(),
            ..Card::default()
        };
        assert_eq!(card.job(), "Buyer, Acme");
        let address = PostalAddress {
            street: "12 MG Road\n".into(),
            city: "Pune".into(),
            postcode: "411001".into(),
            country: "India".into(),
            ..PostalAddress::default()
        };
        assert_eq!(address.lines(), ["12 MG Road", "Pune, 411001", "India"]);
        assert!(PostalAddress::default().is_empty());
    }

    #[test]
    fn round_trips_as_json_without_empty_fields() {
        let card = Card {
            name: Name {
                given: "Asha".into(),
                ..Name::default()
            },
            phones: vec![Typed::new("+91 98", "mobile")],
            ..Card::default()
        };
        let json = serde_json::to_string(&card).unwrap();
        assert!(!json.contains("emails"));
        assert_eq!(serde_json::from_str::<Card>(&json).unwrap(), card);
    }
}
