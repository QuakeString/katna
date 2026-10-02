// SPDX-License-Identifier: GPL-3.0-or-later

//! What a signature tells about its sender, for the person card: the few
//! lines worth reading, the pages it links to, and the whole of it,
//! tidied. Pictures (`[image: logo]`), bare links, banners, "Print only
//! when necessary" lines and legal footers are left out, and so is what
//! the card already shows (their name, title and phone).

use crate::trim::{blank, delimiter, footer_line, phone, rule, sign_off};

/// Lines a summary shows before the rest waits behind "Full signature".
const SHOWN_LINES: usize = 4;
/// Pages a summary links to, at most.
const MOST_LINKS: usize = 6;

/// What the card already says about the person, so the signature need
/// not say it again.
#[derive(Debug, Default, Clone, Copy)]
pub struct Known<'a> {
    pub name: Option<&'a str>,
    pub email: &'a str,
    /// Their title, company, phone and the like, as shown.
    pub shown: &'a [&'a str],
}

/// A signature as the card shows it.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Summary {
    /// The lines worth reading, at most a few.
    pub lines: Vec<String>,
    /// The pages it links to: their website, then social pages.
    pub links: Vec<Link>,
    /// The whole signature without pictures, bare links and footers.
    pub full: String,
    /// Whether `full` says more than `lines` and `links`.
    pub more: bool,
}

/// A page a signature links to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    pub site: Site,
    pub url: String,
}

/// The sites the card draws its own icon for; any other page is `Web`.
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
];

/// What is left of a contact line once its address or number is out:
/// "W :", "E-mail:", "| M".
const LABELS: &[&str] = &[
    "w", "web", "website", "url", "e", "email", "e-mail", "mail", "m", "mob", "mobile", "t", "tel",
    "p", "ph", "phone", "cell", "f", "fax",
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

    fn of(url: &str) -> Site {
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

/// The card's view of `signature` (the lines under a mail, from its
/// sign-off down). `None` when it says nothing the card does not.
pub fn summary(signature: &str, known: &Known) -> Option<Summary> {
    let lines = signature
        .lines()
        .skip_while(|l| blank(l) || delimiter(l) || sign_off(l));
    let mut full: Vec<String> = Vec::new();
    let mut own: Vec<String> = Vec::new();
    let mut links: Vec<Link> = Vec::new();
    let mut in_footer = false;
    for raw in lines {
        if blank(raw) {
            in_footer = false;
            if full.last().is_some_and(|l| !l.is_empty()) {
                full.push(String::new());
            }
            continue;
        }
        if eco(raw) {
            continue;
        }
        // A footer runs to the end of its paragraph, links and all.
        in_footer |= footer_line(raw);
        if in_footer || rule(raw) {
            continue;
        }
        let (text, urls) = clean(raw, known.email);
        for url in urls {
            let site = Site::of(&url);
            let same = |l: &Link| match site {
                Site::Web => host(&l.url) == host(&url),
                _ => l.site == site,
            };
            if !links.iter().any(same) {
                links.push(Link { site, url });
            }
        }
        let Some(text) = text else {
            continue;
        };
        // "Demo Rao | Head of Growth": the parts the card shows go.
        let parts: Vec<&str> = text
            .split(['|', '\u{2022}', '\u{b7}'])
            .map(str::trim)
            .filter(|p| !p.is_empty() && !said(p, known))
            .collect();
        if !parts.is_empty() {
            own.push(parts.join(" \u{b7} "));
        }
        full.push(text);
    }
    while full.last().is_some_and(String::is_empty) {
        full.pop();
    }
    // Their website first, then the social pages, then any other page.
    links.sort_by_key(|l| match l.site {
        Site::Web if own_site(&l.url, known.email) => 0,
        Site::Web => 2,
        _ => 1,
    });
    links.truncate(MOST_LINKS);
    // Up to a second label ("Registered Office:" under "Corporate
    // Office:"), without the labels.
    let label = |l: &str| l.ends_with(':') && l.split_whitespace().count() <= 4;
    let mut shown: Vec<String> = Vec::new();
    let mut labels = 0;
    for line in &own {
        if label(line) {
            labels += 1;
            if labels > 1 && !shown.is_empty() {
                break;
            }
            continue;
        }
        if shown.len() == SHOWN_LINES {
            break;
        }
        shown.push(line.clone());
    }
    let more = own.iter().filter(|l| !label(l)).count() > shown.len();
    if shown.is_empty() && links.is_empty() {
        return None;
    }
    Some(Summary {
        more,
        lines: shown,
        links,
        full: full.join("\n"),
    })
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

/// `line` without its pictures, links and their own address: the text
/// left, if any is worth showing, and the links taken out.
fn clean(line: &str, email: &str) -> (Option<String>, Vec<String>) {
    let mut text = line.trim().to_owned();
    // A line that is only `[Aluminium]` is a picture's name.
    if text.starts_with('[') && text.ends_with(']') && !text.contains("://") {
        return (None, Vec::new());
    }
    for open in ["[image:", "[cid:", "[Image:", "[IMAGE:"] {
        while let Some(start) = text.find(open) {
            let end = text[start..]
                .find(']')
                .map_or(text.len(), |e| start + e + 1);
            text.replace_range(start..end, " ");
        }
    }
    let mut urls = Vec::new();
    // Links in angle brackets, as text versions of HTML mail write them.
    while let Some(start) = text.find('<') {
        let Some(end) = text[start..].find('>').map(|e| start + e) else {
            break;
        };
        let inner = text[start + 1..end].trim().to_owned();
        if inner.contains("://") || inner.starts_with("www.") {
            urls.push(web(&inner));
        }
        let mailto = inner.starts_with("mailto:");
        if inner.contains("://") || inner.starts_with("www.") || mailto {
            text.replace_range(start..=end, " ");
        } else {
            break;
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
        } else if lower.trim_end_matches('.') == email.to_lowercase() {
            // Their address is at the top of the card.
        } else {
            words.push(word);
        }
    }
    let text = words
        .join(" ")
        .trim_matches(|c: char| c.is_whitespace() || matches!(c, '|' | '\u{2022}' | '\u{b7}'))
        .to_owned();
    // "Book a call:" was the link's label; the link shows on its own.
    let label = !urls.is_empty() && text.ends_with(':');
    let kept = !text.is_empty() && !label && !label_only(&text) && !site_names(&text);
    (kept.then_some(text), urls)
}

/// `url` as a link opens it: `www.x.com` as `https://www.x.com`.
fn web(url: &str) -> String {
    if url.contains("://") {
        url.to_owned()
    } else {
        format!("https://{url}")
    }
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

/// Whether the card already says what `line` does: their name, title or
/// company, or only phone numbers it shows.
fn said(line: &str, known: &Known) -> bool {
    let mut seen: Vec<String> = known.shown.iter().flat_map(|s| words(s)).collect();
    seen.extend(known.name.into_iter().flat_map(words));
    // "Sr. Engineer - Projects" under "Sr. Engineer" and "Projects".
    let wordy = words(line).any(|w| !w.chars().all(|c| c.is_ascii_digit()));
    let all_seen = words(line)
        .filter(|w| !w.chars().all(|c| c.is_ascii_digit()) || !phone(line))
        .all(|w| seen.contains(&w) || LABELS.contains(&w.as_str()));
    if !phone(line) {
        return wordy && all_seen;
    }
    // A phone line: said when its numbers are all on the card.
    let mut digits: String = line.chars().filter(char::is_ascii_digit).collect();
    for shown in known.shown {
        let number: String = shown.chars().filter(char::is_ascii_digit).collect();
        if number.len() < 7 {
            continue;
        }
        // +91 90917 49962 is 9091749962 when written without the code.
        for form in [number.as_str(), &number[number.len().saturating_sub(10)..]] {
            digits = digits.replacen(form, "", 1);
        }
    }
    digits.len() < 7 && all_seen
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tuhin() -> &'static str {
        "[image: LIZ INDIA]\n\nTuhinsubhra Demo\nSr. Engineer - Projects\n[Aluminium]\n\
         M +91 90000 49962\nD +91 22000 14276\nwww.lizdemo.example\n\
         [image: PART OF GROUP LTT]\n\
         [image: in] <https://www.linkedin.com/company/lizdemo-in> [image: o]\n\
         <https://www.instagram.com/lizdemo_india/> [image: f]\n\
         <https://www.youtube.com/@lizdemo_india>\n\
         Print only when necessary | Switch off as you go | Recycle Always"
    }

    #[test]
    fn pictures_links_and_what_the_card_shows_go() {
        let known = Known {
            name: Some("Tuhinsubhra Demo"),
            email: "tuhin@lizdemo.example",
            shown: &["Sr. Engineer", "Projects", "+91 90000 49962"],
        };
        let s = summary(tuhin(), &known).unwrap();
        assert_eq!(s.lines, ["D +91 22000 14276"]);
        let sites: Vec<Site> = s.links.iter().map(|l| l.site).collect();
        assert_eq!(
            sites,
            [Site::Web, Site::LinkedIn, Site::Instagram, Site::YouTube]
        );
        assert_eq!(s.links[0].url, "https://www.lizdemo.example");
        assert_eq!(s.links[2].url, "https://www.instagram.com/lizdemo_india/");
        assert!(!s.more, "{s:?}");
        assert!(!s.full.contains("image"), "{}", s.full);
        assert!(!s.full.contains("Print"), "{}", s.full);
        assert!(
            s.full
                .starts_with("Tuhinsubhra Demo\nSr. Engineer - Projects")
        );
    }

    #[test]
    fn follow_us_lines_and_footers() {
        let sig = "Best regards,\nDemo Rao | Head of Growth\nDemo Labs\n\
                   +1 555 010 7788\n\
                   Follow us: Facebook <https://facebook.com/demolabs> | \
                   Twitter <https://twitter.com/demolabs> | LinkedIn \
                   <https://www.linkedin.com/company/demolabs>\n\
                   Book a call: https://cal.demo.example/rao\n\n\
                   CONFIDENTIALITY: This e-mail and any attachments are confidential \
                   and intended only for the addressee. Visit https://demolabs.example/legal";
        let known = Known {
            name: Some("Demo Rao"),
            email: "rao@demolabs.example",
            shown: &["Head of Growth", "Demo Labs", "+1 555 010 7788"],
        };
        let s = summary(sig, &known).unwrap();
        assert!(s.lines.is_empty(), "{s:?}");
        let known = Known {
            shown: &["+1 555 010 7788"],
            ..known
        };
        let s = summary(sig, &known).unwrap();
        assert_eq!(s.lines, ["Head of Growth", "Demo Labs"]);
        let sites: Vec<Site> = s.links.iter().map(|l| l.site).collect();
        assert_eq!(sites, [Site::Facebook, Site::X, Site::LinkedIn, Site::Web]);
        assert!(!s.full.contains("CONFIDENTIALITY"), "{}", s.full);
    }

    #[test]
    fn offices_fold_and_labels_go() {
        let sig = "Thanks & Regards\nDemo Basu\nK. G. Demo Services\n\nCorporate Office:\n\
                   Demo IT Park, Phase - I, Module no. 201, New Town\n\
                   Action Area 1, Kolkata 700000, West Bengal, India\nRegistered Office:\n\
                   1 Demo Road, Dum Dum\nKolkata 700001, West Bengal, India\n\
                   | M   :  + 91 90000 00000 / 90000 00001\n| E   : sales@kgs.demo.example\n\
                   | W   : www.kgs.demo.example <http://www.kgs.demo.example/>";
        let known = Known {
            name: Some("Demo Basu"),
            email: "sales@kgs.demo.example",
            shown: &["+91 90000 00000"],
        };
        let s = summary(sig, &known).unwrap();
        assert_eq!(
            s.lines,
            [
                "K. G. Demo Services",
                "Demo IT Park, Phase - I, Module no. 201, New Town",
                "Action Area 1, Kolkata 700000, West Bengal, India",
            ]
        );
        assert!(s.more);
        assert_eq!(s.links.len(), 1);
        assert_eq!(s.links[0].site, Site::Web);
        assert!(
            s.full.contains("M : + 91 90000 00000 / 90000 00001"),
            "{}",
            s.full
        );
        assert!(!s.full.contains("| E"), "{}", s.full);
    }

    #[test]
    fn nothing_new() {
        let known = Known {
            name: Some("Omar Haddad"),
            email: "omar@demo.example",
            shown: &["Product Manager"],
        };
        assert_eq!(summary("Omar Haddad\nProduct Manager", &known), None);
        assert_eq!(summary("", &known), None);
    }

    #[test]
    fn hosts() {
        assert_eq!(host("https://www.Demo.example/a?b"), "demo.example");
        assert_eq!(Site::of("https://youtu.be/abc"), Site::YouTube);
        assert_eq!(Site::of("https://notlinkedin.com/x"), Site::Web);
        assert!(own_site("https://www.lizdemo.com", "a@lizdemo.in"));
    }
}
