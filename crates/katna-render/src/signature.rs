// SPDX-License-Identifier: GPL-3.0-or-later

//! What a signature tells about its sender, for the person card: their
//! phone numbers, other addresses and pages, and their company's name,
//! group, website, offices and pages, with any other line worth keeping.
//! Pictures (`[image: logo]`), banners, "Follow us" lines, taglines,
//! "Print only when necessary" lines and legal footers are left out, and
//! so is what the card already shows (their name and title).

use crate::trim::{blank, delimiter, footer_line, mobile_signature, phone, postal, rule, sign_off};

/// Pages kept of each kind, at most.
const MOST_LINKS: usize = 8;

/// What the card already says about the person, so the signature need
/// not say it again.
#[derive(Debug, Default, Clone, Copy)]
pub struct Known<'a> {
    pub name: Option<&'a str>,
    pub email: &'a str,
    /// Their title and the like, as shown.
    pub shown: &'a [&'a str],
    /// The name of their company, when the card knows it.
    pub company: Option<&'a str>,
}

/// What a signature says.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Details {
    /// Their title, when it shares a line with their name or follows it:
    /// "Demo Rao | Head of Growth".
    pub title: Option<String>,
    /// Their phone numbers, in the order written.
    pub phones: Vec<Phone>,
    /// Addresses other than the one they wrote from.
    pub emails: Vec<String>,
    /// Their own pages: a LinkedIn profile, GitHub, a booking page.
    pub pages: Vec<Link>,
    pub company: CompanyDetails,
    /// Any other line: a department, a Skype name, office hours.
    pub other: Vec<String>,
}

/// What a signature says about the sender's company.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CompanyDetails {
    pub name: Option<String>,
    /// What its logo is called (`[image: Demo Labs]`): its name, likely.
    pub logo: Option<String>,
    /// "Part of the Demo Group".
    pub group: Option<String>,
    pub website: Option<String>,
    /// The company's pages: LinkedIn, Instagram, YouTube…
    pub pages: Vec<Link>,
    pub offices: Vec<Office>,
}

/// An office: "Registered Office" and its address lines.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Office {
    pub label: Option<String>,
    pub lines: Vec<String>,
}

/// A phone number and what kind it is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phone {
    pub kind: PhoneKind,
    pub number: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhoneKind {
    /// No label, or one the card does not tell apart.
    Phone,
    Mobile,
    Direct,
    Office,
    Fax,
    WhatsApp,
}

/// A page a signature links to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    pub site: Site,
    pub url: String,
}

/// The sites the card draws its own mark for; any other page is `Web`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Site {
    Web,
    LinkedIn,
    X,
    Facebook,
    Instagram,
    YouTube,
    GitHub,
    WhatsApp,
    Telegram,
}

const SITES: &[(&str, Site)] = &[
    ("linkedin.com", Site::LinkedIn),
    ("x.com", Site::X),
    ("twitter.com", Site::X),
    ("facebook.com", Site::Facebook),
    ("fb.com", Site::Facebook),
    ("instagram.com", Site::Instagram),
    ("youtube.com", Site::YouTube),
    ("youtu.be", Site::YouTube),
    ("github.com", Site::GitHub),
    ("wa.me", Site::WhatsApp),
    ("whatsapp.com", Site::WhatsApp),
    ("t.me", Site::Telegram),
    ("telegram.me", Site::Telegram),
];

/// Words that only name a site or invite people to follow one: what is
/// left of "Follow us: LinkedIn | Instagram" once the links are out.
const SITE_WORDS: &[&str] = &[
    "follow",
    "us",
    "on",
    "connect",
    "with",
    "find",
    "join",
    "linkedin",
    "twitter",
    "x",
    "facebook",
    "instagram",
    "youtube",
    "github",
    "whatsapp",
    "telegram",
    "website",
    "web",
    "site",
    "visit",
    "our",
    "blog",
    "and",
    "social",
    "media",
];

/// What is left of a contact line once its address or number is out:
/// "W :", "E-mail:", "| M".
const LABELS: &[&str] = &[
    "w",
    "web",
    "website",
    "url",
    "e",
    "email",
    "e-mail",
    "mail",
    "m",
    "mob",
    "mobile",
    "t",
    "tel",
    "p",
    "ph",
    "phone",
    "cell",
    "f",
    "fax",
    "d",
    "o",
    "off",
    "office",
    "direct",
    "dir",
    "did",
    "wa",
    "whatsapp",
    "telephone",
    "board",
    "landline",
    "hp",
    "c",
];

/// Words that end a company's name.
const COMPANY_WORDS: &[&str] = &[
    "inc",
    "llc",
    "ltd",
    "limited",
    "gmbh",
    "corp",
    "corporation",
    "co",
    "company",
    "plc",
    "llp",
    "pvt",
    "private",
    "pte",
    "ag",
    "sa",
    "bv",
    "srl",
    "group",
    "industries",
    "services",
    "solutions",
    "technologies",
    "systems",
    "enterprises",
    "consultants",
];

impl Site {
    /// The site's name, for tooltips.
    pub fn name(self) -> &'static str {
        match self {
            Site::Web => "",
            Site::LinkedIn => "LinkedIn",
            Site::X => "X",
            Site::Facebook => "Facebook",
            Site::Instagram => "Instagram",
            Site::YouTube => "YouTube",
            Site::GitHub => "GitHub",
            Site::WhatsApp => "WhatsApp",
            Site::Telegram => "Telegram",
        }
    }

    /// The site `url` is on.
    pub fn of(url: &str) -> Site {
        let host = host(url);
        SITES
            .iter()
            .find(|(site, _)| host == *site || host.ends_with(&format!(".{site}")))
            .map_or(Site::Web, |(_, s)| *s)
    }
}

/// `url`'s host without `www.`: `example.com`.
pub fn host(url: &str) -> String {
    let rest = url
        .split_once("://")
        .map_or(url, |(_, rest)| rest)
        .split(['/', '?', '#'])
        .next()
        .unwrap_or_default()
        .to_lowercase();
    rest.strip_prefix("www.").unwrap_or(&rest).to_owned()
}

impl PhoneKind {
    /// The kind a label names: "M", "Mob" or "Cell" is a mobile, "D" or
    /// "DID" a direct line, "T", "Tel" or "Ph" a plain phone.
    fn of(label: &str) -> PhoneKind {
        let label: String = label
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<String>()
            .to_lowercase();
        match label.as_str() {
            "m" | "mob" | "mobile" | "cell" | "c" | "hp" | "handphone" | "mobileno" => {
                PhoneKind::Mobile
            }
            "d" | "dir" | "direct" | "did" | "directline" => PhoneKind::Direct,
            "o" | "off" | "office" | "board" | "landline" | "work" => PhoneKind::Office,
            "f" | "fax" => PhoneKind::Fax,
            "wa" | "whatsapp" => PhoneKind::WhatsApp,
            _ => PhoneKind::Phone,
        }
    }
}

/// What `signature` (the lines under a mail, from its sign-off down)
/// says about its sender. `None` when it says nothing the card does not.
pub fn details(signature: &str, known: &Known) -> Option<Details> {
    let mut out = Details::default();
    let mut in_footer = false;
    // The office whose lines are being read, after its label.
    let mut office: Option<usize> = None;
    // The last line kept under `other`, when it is the line just read.
    let mut last_other: Option<usize> = None;
    // The line read last was their name.
    let mut after_name = false;
    let lines = signature
        .lines()
        .skip_while(|l| blank(l) || delimiter(l) || sign_off(l));
    for raw in lines {
        let previous_other = last_other.take();
        if blank(raw) {
            in_footer = false;
            office = None;
            after_name = false;
            continue;
        }
        if eco(raw) || mobile_signature(raw) {
            continue;
        }
        // A footer runs to the end of its paragraph, links and all.
        in_footer |= footer_line(raw);
        if in_footer || rule(raw) {
            continue;
        }
        let line = pictures(raw, &mut out.company);
        let (text, urls) = links(&line);
        for url in urls {
            add_link(&mut out, url, known.email);
        }
        let mut parts: Vec<String> = Vec::new();
        for part in text.split(['|', '\u{2022}', '\u{b7}']) {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            if let Some(phones) = phones(part) {
                out.phones.extend(phones);
                continue;
            }
            let mut rest = Vec::new();
            for word in part.split_whitespace() {
                let bare = word.trim_matches(|c: char| matches!(c, '(' | ')' | ',' | ';' | '"'));
                if email_like(bare) {
                    let bare = bare.trim_end_matches('.').to_lowercase();
                    if bare != known.email.to_lowercase() && !out.emails.contains(&bare) {
                        out.emails.push(bare);
                    }
                } else {
                    rest.push(word);
                }
            }
            let part = rest.join(" ");
            let part = part.trim_matches(|c: char| c.is_whitespace() || c == '-');
            let label = part.trim_end_matches([':', ' ']);
            if !label.is_empty() && !label_only(label) && !site_names(label) {
                parts.push(part.to_owned());
            }
        }
        // "Demo Rao | Head of Growth": their name goes, and what is
        // next to it is their title.
        let named = parts.len() > 1 && parts.iter().any(|p| is_name(p, known));
        if named {
            parts.retain(|p| !is_name(p, known) && !said(p, known));
            after_name = true;
            if out.title.is_none() && parts.len() == 1 && title_like(&parts[0]) {
                out.title = parts.pop();
            }
        }
        if parts.is_empty() {
            continue;
        }
        let text = parts.join(" \u{b7} ");
        let next_to_name = std::mem::take(&mut after_name);
        // "Registered Office:" starts an office.
        if (text.ends_with(':') && text.split_whitespace().count() <= 4) || office_label(&text) {
            out.company.offices.push(Office {
                label: Some(text.trim_end_matches(':').trim().to_owned()),
                lines: Vec::new(),
            });
            office = Some(out.company.offices.len() - 1);
            continue;
        }
        if let Some(ix) = office {
            out.company.offices[ix].lines.push(text);
            continue;
        }
        if postal(&text) && (text.contains(',') || text.split_whitespace().count() >= 2) {
            // The line above, a street, belongs with it.
            let mut lines = Vec::new();
            if let Some(ix) = previous_other
                && ix + 1 == out.other.len()
                && street(&out.other[ix])
            {
                lines.push(out.other.remove(ix));
            }
            lines.push(text);
            out.company.offices.push(Office { label: None, lines });
            continue;
        }
        if is_name(&text, known) {
            after_name = true;
            continue;
        }
        if known.company.is_some_and(|c| same_words(&text, c))
            || out.company.name.is_none() && company_name(&text)
        {
            out.company.name.get_or_insert(text);
            continue;
        }
        if group(&text) {
            out.company.group.get_or_insert(text);
            continue;
        }
        if said(&text, known) || tagline(&text) {
            continue;
        }
        if next_to_name && known.shown.is_empty() && out.title.is_none() && title_like(&text) {
            out.title = Some(text);
            continue;
        }
        out.other.push(text);
        last_other = Some(out.other.len() - 1);
    }
    out.company.offices.retain(|o| !o.lines.is_empty());
    out.pages.truncate(MOST_LINKS);
    out.company.pages.truncate(MOST_LINKS);
    let empty = out.title.is_none()
        && out.phones.is_empty()
        && out.emails.is_empty()
        && out.pages.is_empty()
        && out.other.is_empty()
        && out.company == CompanyDetails::default();
    (!empty).then_some(out)
}

/// Files `url` as the company's website, one of its pages or one of the
/// person's own.
fn add_link(out: &mut Details, url: String, email: &str) {
    let site = Site::of(&url);
    let same = |l: &Link| match site {
        Site::Web => host(&l.url) == host(&url),
        _ => l.site == site && l.url.trim_end_matches('/') == url.trim_end_matches('/'),
    };
    let website = out.company.website.as_deref().map(host);
    if out.pages.iter().any(same)
        || out.company.pages.iter().any(same)
        || site == Site::Web && website.as_deref() == Some(host(&url).as_str())
    {
        return;
    }
    let lower = url.to_lowercase();
    // A LinkedIn profile, GitHub or a booking page is theirs; a company
    // page, a channel or an account on the rest is the company's.
    let own = match site {
        Site::LinkedIn => lower.contains("/in/"),
        Site::GitHub | Site::WhatsApp | Site::Telegram => true,
        Site::Web => {
            if website.is_none() || own_site(&url, email) {
                out.company.website.get_or_insert(url);
                return;
            }
            true
        }
        _ => false,
    };
    let link = Link { site, url };
    if own {
        out.pages.push(link);
    } else {
        out.company.pages.push(link);
    }
}

/// Whether `url` is on the domain of `email`, or one under it.
fn own_site(url: &str, email: &str) -> bool {
    let Some((_, domain)) = email.rsplit_once('@') else {
        return false;
    };
    let host = host(url);
    let domain = domain.to_lowercase();
    host == domain || host.ends_with(&format!(".{domain}")) || {
        // lizindia.com for mail from lizindia.in.
        let stem = |h: &str| h.rsplit('.').nth(1).map(str::to_owned);
        stem(&host).is_some() && stem(&host) == stem(&domain)
    }
}

/// `line` without its pictures (`[image: logo]`, `[cid:…]`, a line that
/// is only `[Aluminium]`). A picture named "Part of the Demo Group" names
/// the company's group.
fn pictures(line: &str, company: &mut CompanyDetails) -> String {
    let mut text = line.trim().to_owned();
    if text.starts_with('[') && text.ends_with(']') && !text[1..].contains('[') {
        let alt = text[1..text.len() - 1].trim();
        match alt.strip_prefix("image:") {
            Some(alt) => picture(alt.trim(), company),
            // A line that is only `[Aluminium]`: a picture's name, or a
            // link's; nothing to show.
            None if group(alt) => {
                company.group.get_or_insert_with(|| alt.to_owned());
            }
            None => {}
        }
        return String::new();
    }
    loop {
        let lower = text.to_lowercase();
        let Some(start) = ["[image:", "[cid:"].iter().find_map(|o| lower.find(o)) else {
            break;
        };
        let end = text[start..]
            .find(']')
            .map_or(text.len(), |e| start + e + 1);
        let inside = &text[start + 1..end.max(start + 2) - 1];
        let alt = inside
            .split_once(':')
            .map_or("", |(_, a)| a.trim())
            .to_owned();
        picture(&alt, company);
        text.replace_range(start..end, " ");
    }
    text
}

/// What a picture's name says: the company's group, or the name of its
/// logo (the first picture with a name of words, not a banner).
fn picture(alt: &str, company: &mut CompanyDetails) {
    if alt.contains("://") {
        return;
    }
    if group(alt) {
        company.group.get_or_insert_with(|| alt.to_owned());
        return;
    }
    let lower = alt.to_lowercase();
    let wordy = alt.chars().filter(|c| c.is_alphabetic()).count() >= 3
        && alt.split_whitespace().count() <= 5
        && !["banner", "award", "icon", "badge", "certified", "follow"]
            .iter()
            .any(|w| lower.contains(w));
    if wordy && !SITE_WORDS.contains(&lower.as_str()) {
        company.logo.get_or_insert_with(|| alt.to_owned());
    }
}

/// A line that can be a title: a few words, no digits.
fn title_like(text: &str) -> bool {
    (1..=6).contains(&text.split_whitespace().count())
        && !text.chars().any(|c| c.is_ascii_digit())
        && !text.ends_with(['.', ':', '!'])
}

/// `line` without its links, and the links: `<https://…>` as text
/// versions of HTML mail write them, and bare `www.` and `http(s)://`
/// words.
fn links(line: &str) -> (String, Vec<String>) {
    let mut text = line.to_owned();
    let mut urls = Vec::new();
    let mut from = 0;
    while let Some(start) = text[from..].find('<').map(|s| from + s) {
        let Some(end) = text[start..].find('>').map(|e| start + e) else {
            break;
        };
        let inner = text[start + 1..end].trim().to_owned();
        let web_link = inner.contains("://") || inner.starts_with("www.");
        if web_link && !inner.starts_with("mailto:") {
            urls.push(web(&inner));
        }
        if web_link || inner.starts_with("mailto:") {
            text.replace_range(start..=end, " ");
        } else {
            from = end;
        }
    }
    let mut words = Vec::new();
    for word in text.split_whitespace() {
        let bare = word.trim_matches(|c: char| matches!(c, '(' | ')' | ',' | ';' | '"'));
        let lower = bare.to_lowercase();
        if lower.starts_with("http://")
            || lower.starts_with("https://")
            || lower.starts_with("www.")
        {
            urls.push(web(bare.trim_end_matches('.')));
        } else {
            words.push(word);
        }
    }
    let text = words.join(" ");
    // "Book a call:" was the link's label; the link shows on its own.
    let text = if !urls.is_empty() && text.trim_end().ends_with(':') {
        String::new()
    } else {
        text
    };
    (text, urls)
}

/// `url` as a link opens it: `www.x.com` as `https://www.x.com`.
fn web(url: &str) -> String {
    if url.contains("://") {
        url.to_owned()
    } else {
        format!("https://{url}")
    }
}

/// The phone numbers in `part` ("M : +91 90000 00000 / 90000 00001"),
/// with the kind its label names, when it is only a label and numbers.
fn phones(part: &str) -> Option<Vec<Phone>> {
    if !phone(part) {
        return None;
    }
    let start = part.find(|c: char| c.is_ascii_digit() || c == '+' || c == '(')?;
    let label = &part[..start];
    if label.split_whitespace().count() > 3 {
        return None;
    }
    let kind = PhoneKind::of(label);
    let numbers: Vec<Phone> = part[start..]
        .split(['/', ',', ';'])
        .map(|n| n.trim().trim_end_matches('.').trim())
        .filter(|n| phone(n))
        .map(|n| Phone {
            kind,
            // "+ 91 90000 00000" as "+91 90000 00000".
            number: n
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
                .replace("+ ", "+"),
        })
        .collect();
    (!numbers.is_empty()).then_some(numbers)
}

fn email_like(word: &str) -> bool {
    word.split_once('@')
        .is_some_and(|(user, host)| !user.is_empty() && host.contains('.'))
}

/// The lowercase words of `text`.
fn words(text: &str) -> impl Iterator<Item = String> + '_ {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(str::to_lowercase)
}

fn label_only(text: &str) -> bool {
    let words: Vec<String> = words(text).collect();
    words.is_empty() || words.iter().all(|w| LABELS.contains(&w.as_str()))
}

fn site_names(text: &str) -> bool {
    words(text).all(|w| SITE_WORDS.contains(&w.as_str()))
}

/// "Corporate Office", "Head Office - Pune": a line naming an office.
fn office_label(text: &str) -> bool {
    let lower = text.to_lowercase();
    lower.split_whitespace().count() <= 4
        && [
            "office",
            "address",
            "headquarters",
            "hq",
            "branch",
            "works",
            "factory",
            "plant",
        ]
        .iter()
        .any(|w| words(&lower).any(|x| x == *w))
        && !lower.chars().any(|c| c.is_ascii_digit())
}

/// A line that could be a street: it has a number or a comma.
fn street(text: &str) -> bool {
    text.contains(',') || text.chars().any(|c| c.is_ascii_digit())
}

fn company_name(text: &str) -> bool {
    text.split_whitespace().count() <= 8
        && words(text).any(|w| COMPANY_WORDS.contains(&w.as_str()))
        && !text.ends_with(':')
}

/// "Part of the Demo Group", "A Demo Group company", "Member of Demo".
fn group(text: &str) -> bool {
    let lower = text.to_lowercase();
    lower.contains("part of")
        || lower.contains("member of")
        || lower.contains("a subsidiary")
        || (lower.contains("group") && lower.contains("company"))
}

/// Whether `text` is their name, alone or with what the card shows.
fn is_name(text: &str, known: &Known) -> bool {
    let Some(name) = known.name else {
        return false;
    };
    let name: Vec<String> = words(name).collect();
    let line: Vec<String> = words(text).collect();
    !line.is_empty() && line.iter().all(|w| name.contains(w))
}

fn same_words(a: &str, b: &str) -> bool {
    words(a).eq(words(b))
}

/// "Print only when necessary", "Please consider the environment before
/// printing this e-mail", "Switch off as you go".
fn eco(line: &str) -> bool {
    let lower = line.to_lowercase();
    (lower.contains("print")
        && ["necessary", "environment", "before you", "think", "need to"]
            .iter()
            .any(|w| lower.contains(w)))
        || [
            "recycle",
            "switch off as you go",
            "go green",
            "save paper",
            "save trees",
        ]
        .iter()
        .any(|w| lower.contains(w))
}

/// A slogan: "Engineering the future!", a sentence of marketing.
fn tagline(text: &str) -> bool {
    text.ends_with('!') || (text.chars().count() > 60 && text.ends_with('.'))
}

/// Whether the card already says what `line` does: their title.
fn said(line: &str, known: &Known) -> bool {
    let mut seen: Vec<String> = known.shown.iter().flat_map(|s| words(s)).collect();
    seen.extend(known.name.into_iter().flat_map(words));
    seen.extend(known.company.into_iter().flat_map(words));
    // "Sr. Engineer - Projects" under "Sr. Engineer" and "Projects".
    let mut words = words(line).peekable();
    words.peek().is_some() && words.all(|w| seen.contains(&w))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tuhin() -> &'static str {
        "[image: LIZ DEMO]\n\nTuhinsubhra Demo\nSr. Engineer - Projects\n[Aluminium]\n\
         M +91 90000 49962\nD +91 22000 14276\nwww.lizdemo.example\n\
         [image: PART OF GROUP DEMO]\n\
         [image: in] <https://www.linkedin.com/company/lizdemo-in> [image: o]\n\
         <https://www.instagram.com/lizdemo_india/> [image: f]\n\
         <https://www.youtube.com/@lizdemo_india>\n\
         Print only when necessary | Switch off as you go | Recycle Always"
    }

    #[test]
    fn gmail_text_signature() {
        let known = Known {
            name: Some("Tuhinsubhra Demo"),
            email: "tuhin@lizdemo.example",
            shown: &["Sr. Engineer", "Projects"],
            company: None,
        };
        let d = details(tuhin(), &known).unwrap();
        assert_eq!(
            d.phones,
            [
                Phone {
                    kind: PhoneKind::Mobile,
                    number: "+91 90000 49962".into()
                },
                Phone {
                    kind: PhoneKind::Direct,
                    number: "+91 22000 14276".into()
                },
            ]
        );
        assert!(d.other.is_empty(), "{d:?}");
        assert!(d.pages.is_empty(), "{d:?}");
        assert_eq!(d.company.group.as_deref(), Some("PART OF GROUP DEMO"));
        assert_eq!(d.company.logo.as_deref(), Some("LIZ DEMO"));
        assert_eq!(
            d.company.website.as_deref(),
            Some("https://www.lizdemo.example")
        );
        let sites: Vec<Site> = d.company.pages.iter().map(|l| l.site).collect();
        assert_eq!(sites, [Site::LinkedIn, Site::Instagram, Site::YouTube]);
    }

    #[test]
    fn marketing_signature() {
        let sig = "Best regards,\nDemo Rao | Head of Growth\nDemo Labs Inc.\n\
                   +1 555 010 7788 | Skype: demo.rao\n\
                   [image: Demo Labs] <https://demolabs.example>\n\
                   [image: Best Workplace 2026 banner]\n\
                   Follow us: Facebook <https://facebook.com/demolabs> | \
                   Twitter <https://twitter.com/demolabs> | LinkedIn \
                   <https://www.linkedin.com/company/demolabs>\n\
                   My profile: https://www.linkedin.com/in/demorao\n\
                   Book a call: https://cal.demo.example/rao\n\
                   Building tools people love!\n\n\
                   Please consider the environment before printing this e-mail.\n\n\
                   CONFIDENTIALITY: This e-mail and any attachments are confidential \
                   and intended only for the addressee. Visit https://demolabs.example/legal";
        let known = Known {
            name: Some("Demo Rao"),
            email: "rao@demolabs.example",
            shown: &[],
            company: None,
        };
        let d = details(sig, &known).unwrap();
        assert_eq!(d.title.as_deref(), Some("Head of Growth"));
        assert_eq!(d.company.logo.as_deref(), Some("Demo Labs"));
        assert_eq!(d.company.name.as_deref(), Some("Demo Labs Inc."));
        assert_eq!(d.phones.len(), 1);
        assert_eq!(d.phones[0].kind, PhoneKind::Phone);
        assert_eq!(d.other, ["Skype: demo.rao"]);
        let own: Vec<Site> = d.pages.iter().map(|l| l.site).collect();
        assert_eq!(own, [Site::LinkedIn, Site::Web]);
        let theirs: Vec<Site> = d.company.pages.iter().map(|l| l.site).collect();
        assert_eq!(theirs, [Site::Facebook, Site::X, Site::LinkedIn]);
        assert_eq!(
            d.company.website.as_deref(),
            Some("https://demolabs.example")
        );
    }

    #[test]
    fn offices_and_two_numbers() {
        let sig = "Thanks & Regards\nDemo Basu\nK. G. Demo Services\n\nCorporate Office:\n\
                   Demo IT Park, Phase - I, Module no. 201, New Town\n\
                   Action Area 1, Kolkata 700000, West Bengal, India\nRegistered Office:\n\
                   1 Demo Road, Dum Dum\nKolkata 700001, West Bengal, India\n\
                   | M   :  + 91 90000 00000 / 90000 00001\n| E   : sales@kgs.demo.example\n\
                   | W   : www.kgs.demo.example <http://www.kgs.demo.example/>";
        let known = Known {
            name: Some("Demo Basu"),
            email: "sales@kgs.demo.example",
            shown: &[],
            company: Some("K. G. Demo Services"),
        };
        let d = details(sig, &known).unwrap();
        assert_eq!(d.company.name.as_deref(), Some("K. G. Demo Services"));
        assert_eq!(d.company.offices.len(), 2, "{d:?}");
        assert_eq!(
            d.company.offices[0].label.as_deref(),
            Some("Corporate Office")
        );
        assert_eq!(
            d.company.offices[1].lines,
            ["1 Demo Road, Dum Dum", "Kolkata 700001, West Bengal, India"]
        );
        let numbers: Vec<&str> = d.phones.iter().map(|p| p.number.as_str()).collect();
        assert_eq!(numbers, ["+91 90000 00000", "90000 00001"]);
        assert!(d.phones.iter().all(|p| p.kind == PhoneKind::Mobile));
        assert!(d.emails.is_empty());
        assert!(d.other.is_empty(), "{d:?}");
    }

    #[test]
    fn an_address_without_a_label() {
        let sig = "Regards,\nDemo Sharma\nSales Head\nDemo Supplies Pvt. Ltd.\n\
                   12 Example Street, Shivaji Nagar\nPune 411000, India\n\
                   Tel: +91 20 0000 0000 | Fax: +91 20 0000 0001\n\
                   accounts@demo-supplies.example\nOffice hours: Mon to Fri, 10 to 6";
        let known = Known {
            name: Some("Demo Sharma"),
            email: "sales@demo-supplies.example",
            shown: &["Sales Head"],
            company: None,
        };
        let d = details(sig, &known).unwrap();
        assert_eq!(d.company.name.as_deref(), Some("Demo Supplies Pvt. Ltd."));
        assert_eq!(
            d.company.offices,
            [Office {
                label: None,
                lines: vec![
                    "12 Example Street, Shivaji Nagar".into(),
                    "Pune 411000, India".into()
                ]
            }]
        );
        let kinds: Vec<PhoneKind> = d.phones.iter().map(|p| p.kind).collect();
        assert_eq!(kinds, [PhoneKind::Phone, PhoneKind::Fax]);
        assert_eq!(d.emails, ["accounts@demo-supplies.example"]);
        assert_eq!(d.other, ["Office hours: Mon to Fri, 10 to 6"]);
    }

    #[test]
    fn nothing_new() {
        let known = Known {
            name: Some("Omar Haddad"),
            email: "omar@demo.example",
            shown: &["Product Manager"],
            company: None,
        };
        assert_eq!(details("Omar Haddad\nProduct Manager", &known), None);
        assert_eq!(details("", &known), None);
    }

    #[test]
    fn hosts() {
        assert_eq!(host("https://www.Demo.example/a?b"), "demo.example");
        assert_eq!(Site::of("https://youtu.be/abc"), Site::YouTube);
        assert_eq!(Site::of("https://notlinkedin.com/x"), Site::Web);
        assert!(own_site("https://www.lizdemo.com", "a@lizdemo.in"));
    }
}
