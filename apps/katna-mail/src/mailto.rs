// SPDX-License-Identifier: GPL-3.0-or-later

//! `mailto:` links (RFC 6068), which Katna Mail opens as a new message when
//! it is the desktop's mail app. No GPUI here.

/// What a `mailto:` link fills in.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Mailto {
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub subject: String,
    pub body: String,
}

impl Mailto {
    /// Reads `uri`, or `None` when it is not a `mailto:` link.
    pub fn parse(uri: &str) -> Option<Self> {
        let rest = uri
            .get(..7)
            .filter(|scheme| scheme.eq_ignore_ascii_case("mailto:"))
            .map(|_| &uri[7..])?;
        let (to, query) = rest.split_once('?').unwrap_or((rest, ""));
        let mut mail = Self::default();
        mail.to.extend(addresses(to));
        for pair in query.split('&').filter(|p| !p.is_empty()) {
            let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
            match decode(key).to_ascii_lowercase().as_str() {
                "to" => mail.to.extend(addresses(value)),
                "cc" => mail.cc.extend(addresses(value)),
                "bcc" => mail.bcc.extend(addresses(value)),
                "subject" => mail.subject = decode(value),
                // Line breaks come as CRLF (%0D%0A).
                "body" => mail.body = decode(value).replace("\r\n", "\n"),
                _ => {}
            }
        }
        Some(mail)
    }
}

/// The comma-separated addresses of `field`, decoded.
fn addresses(field: &str) -> impl Iterator<Item = String> + '_ {
    decode(field)
        .split(',')
        .map(str::trim)
        .filter(|a| !a.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>()
        .into_iter()
}

/// Percent-decodes `text` as UTF-8. A `+` stays a plus: in `mailto:` links
/// spaces are `%20`.
fn decode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        let hex = |b: u8| (b as char).to_digit(16);
        if bytes[i] == b'%'
            && let (Some(&h), Some(&l)) = (bytes.get(i + 1), bytes.get(i + 2))
            && let (Some(h), Some(l)) = (hex(h), hex(l))
        {
            out.push((h * 16 + l) as u8);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_every_field() {
        let mail = Mailto::parse(
            "mailto:a@example.com,b@example.com?cc=c@example.com&bcc=d%40example.com\
             &subject=Lunch%20on%20Monday%3F&body=Hi%2C%0D%0Aare%20you%20free%3F",
        )
        .unwrap();
        assert_eq!(mail.to, ["a@example.com", "b@example.com"]);
        assert_eq!(mail.cc, ["c@example.com"]);
        assert_eq!(mail.bcc, ["d@example.com"]);
        assert_eq!(mail.subject, "Lunch on Monday?");
        assert_eq!(mail.body, "Hi,\nare you free?");
    }

    #[test]
    fn keeps_plus_signs_and_unicode() {
        let mail = Mailto::parse("MAILTO:kay+news@example.com?Subject=Caf%C3%A9+1").unwrap();
        assert_eq!(mail.to, ["kay+news@example.com"]);
        assert_eq!(mail.subject, "Café+1");
    }

    #[test]
    fn to_can_be_in_the_query_only() {
        let mail = Mailto::parse("mailto:?to=a@example.com&body=x").unwrap();
        assert_eq!(mail.to, ["a@example.com"]);
        assert_eq!(mail.body, "x");
        assert_eq!(Mailto::parse("mailto:").unwrap(), Mailto::default());
    }

    #[test]
    fn other_links_are_not_mail() {
        assert_eq!(Mailto::parse("https://example.com"), None);
        assert_eq!(Mailto::parse("mail"), None);
    }
}
