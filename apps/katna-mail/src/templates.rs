// SPDX-License-Identifier: GPL-3.0-or-later

//! Mail templates: the fields a template fills in from the recipient and
//! the sender. No GPUI here.

use crate::outgoing::Mailbox;

/// The recipient's first name.
pub const FIRST_NAME: &str = "{first name}";
/// The recipient's whole name.
pub const NAME: &str = "{name}";
/// The sender's name.
pub const MY_NAME: &str = "{my name}";

/// The fields to fill: the recipient's when there is one (else they stay
/// for later), and the sender's name.
pub fn fields(recipient: Option<&Mailbox>, me: &Mailbox) -> Vec<(&'static str, String)> {
    let mut fields = Vec::with_capacity(3);
    if let Some(to) = recipient {
        let name = name_of(to);
        let first = name
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_owned();
        fields.push((FIRST_NAME, first));
        fields.push((NAME, name));
    }
    fields.push((MY_NAME, name_of(me)));
    fields
}

/// A person's name: the one their address carries, else a guess from the
/// address (`kay.mann@…` is Kay Mann).
pub fn name_of(mailbox: &Mailbox) -> String {
    let given = mailbox
        .name
        .as_deref()
        .map(|n| n.trim().trim_matches(|c| c == '"' || c == '\'').trim())
        .filter(|n| !n.is_empty() && !n.eq_ignore_ascii_case(&mailbox.email));
    if let Some(name) = given {
        // "Mann, Kay" is Kay Mann.
        return match name.split_once(',') {
            Some((last, first)) if !first.trim().is_empty() && !last.contains(' ') => {
                format!("{} {}", first.trim(), last.trim())
            }
            _ => name.to_owned(),
        };
    }
    let local = mailbox.email.split('@').next().unwrap_or_default();
    local
        .split(['.', '_', '-', '+'])
        .filter(|part| part.chars().any(char::is_alphabetic))
        .map(|part| {
            let part: String = part.chars().filter(|c| !c.is_ascii_digit()).collect();
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first
                    .to_uppercase()
                    .chain(chars.flat_map(char::to_lowercase))
                    .collect(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mailbox(name: Option<&str>, email: &str) -> Mailbox {
        Mailbox {
            name: name.map(str::to_owned),
            email: email.to_owned(),
        }
    }

    #[test]
    fn names_from_the_address_or_its_name() {
        assert_eq!(name_of(&mailbox(Some("Kay Mann"), "k@x.org")), "Kay Mann");
        assert_eq!(
            name_of(&mailbox(Some("\"Mann, Kay\""), "k@x.org")),
            "Kay Mann"
        );
        assert_eq!(name_of(&mailbox(None, "kay.mann@enron.com")), "Kay Mann");
        assert_eq!(name_of(&mailbox(Some("ada@x.org"), "ada@x.org")), "Ada");
        assert_eq!(name_of(&mailbox(None, "jdoe42@x.org")), "Jdoe");
    }

    #[test]
    fn fills_the_recipient_fields_only_with_a_recipient() {
        let me = mailbox(Some("Mozammel Hossain"), "mo@x.org");
        let to = mailbox(Some("Kay Mann"), "kay@x.org");
        assert_eq!(
            fields(Some(&to), &me),
            [
                (FIRST_NAME, "Kay".to_owned()),
                (NAME, "Kay Mann".to_owned()),
                (MY_NAME, "Mozammel Hossain".to_owned()),
            ]
        );
        assert_eq!(
            fields(None, &me),
            [(MY_NAME, "Mozammel Hossain".to_owned())]
        );
    }
}
