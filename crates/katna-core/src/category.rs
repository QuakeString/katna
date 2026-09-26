// SPDX-License-Identifier: GPL-3.0-or-later

//! Inbox categories (Gmail-style tabs) and a header-based classifier.
//!
//! The classifier only looks at header facts, never at the body, so sync can
//! run it on the header fields it already downloads (level 1,
//! `docs/ARCHITECTURE.md` §6.2). Gmail accounts use Gmail's own categories
//! for the inbox instead (`X-GM-RAW` search in `katna-sync`).

use std::fmt;
use std::str::FromStr;

/// An inbox tab.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MailCategory {
    /// Mail from people, and everything that fits no other tab.
    #[default]
    Primary,
    /// Marketing and offers.
    Promotions,
    /// Social networks and media sites.
    Social,
    /// Automated notifications: receipts, bills, statements, alerts.
    Updates,
    /// Mailing lists and discussion groups.
    Forums,
}

impl MailCategory {
    /// Every category, in tab order.
    pub const ALL: [Self; 5] = [
        Self::Primary,
        Self::Promotions,
        Self::Social,
        Self::Updates,
        Self::Forums,
    ];

    /// Stable number stored in `message.category`. Never change these.
    pub const fn to_storage(self) -> i64 {
        match self {
            Self::Primary => 1,
            Self::Promotions => 2,
            Self::Social => 3,
            Self::Updates => 4,
            Self::Forums => 5,
        }
    }

    /// Reads a number stored by [`MailCategory::to_storage`].
    pub const fn from_storage(value: i64) -> Option<Self> {
        match value {
            1 => Some(Self::Primary),
            2 => Some(Self::Promotions),
            3 => Some(Self::Social),
            4 => Some(Self::Updates),
            5 => Some(Self::Forums),
            _ => None,
        }
    }

    /// Stable lower-case name (`primary`, `promotions`, …), also Gmail's
    /// `category:` search term.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Promotions => "promotions",
            Self::Social => "social",
            Self::Updates => "updates",
            Self::Forums => "forums",
        }
    }
}

impl fmt::Display for MailCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Error from parsing an unknown category name.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("unknown mail category {0:?}")]
pub struct UnknownCategory(pub String);

impl FromStr for MailCategory {
    type Err = UnknownCategory;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|c| c.as_str().eq_ignore_ascii_case(s))
            .ok_or_else(|| UnknownCategory(s.to_owned()))
    }
}

/// Header fields the classifier reads, besides `From`, `Subject`,
/// `In-Reply-To`, `References` and `List-Id`. Sync asks the server for these
/// (IMAP cannot fetch header fields by pattern).
pub const CLASSIFIER_HEADERS: &[&str] = &[
    "List-Unsubscribe",
    "List-Post",
    "Precedence",
    "Auto-Submitted",
    "X-Mailer",
    "Feedback-ID",
    "X-Campaign",
    "X-Campaign-Id",
    "X-CampaignID",
    "X-MC-User",
    "X-Mailchimp-Campaign",
    "X-SFMC-Stack",
    "X-MarketoID",
    "X-CSA-Complaints",
    "X-Report-Abuse",
    "X-SG-EID",
    "X-Mailgun-Sid",
    "X-SES-Outgoing",
    "X-Autoreply",
    "X-Autorespond",
];

/// Headers that marketing platforms add to campaigns.
const CAMPAIGN_HEADERS: &[&str] = &[
    "x-campaign",
    "x-campaign-id",
    "x-campaignid",
    "x-mc-user",
    "x-mailchimp-campaign",
    "x-sfmc-stack",
    "x-marketoid",
    "x-csa-complaints",
    "x-report-abuse",
];

/// Headers of bulk-mail services, used for marketing and notifications alike.
const BULK_SERVICE_HEADERS: &[&str] =
    &["x-sg-eid", "x-mailgun-sid", "x-ses-outgoing", "feedback-id"];

/// `X-Mailer` values of marketing platforms (lower case, substring match).
const CAMPAIGN_MAILERS: &[&str] = &[
    "mailchimp",
    "campaign monitor",
    "sendinblue",
    "brevo",
    "hubspot",
    "klaviyo",
    "mailjet",
    "activecampaign",
    "convertkit",
    "constant contact",
    "emarsys",
    "acoustic",
    "exacttarget",
    "marketo",
    "mailerlite",
    "getresponse",
    "aweber",
    "omnisend",
];

/// Domains of social networks; subdomains match too.
const SOCIAL_DOMAINS: &[&str] = &[
    "facebookmail.com",
    "facebook.com",
    "instagram.com",
    "linkedin.com",
    "twitter.com",
    "x.com",
    "pinterest.com",
    "tiktok.com",
    "snapchat.com",
    "tumblr.com",
    "reddit.com",
    "redditmail.com",
    "quora.com",
    "meetup.com",
    "nextdoor.com",
    "youtube.com",
    "discord.com",
    "discordapp.com",
    "whatsapp.com",
    "threads.net",
    "bsky.app",
    "mastodon.social",
    "vk.com",
    "xing.com",
    "strava.com",
    "goodreads.com",
    "flickr.com",
];

/// Hosts of discussion lists; matched against the `List-Id` and the sender.
const FORUM_HOSTS: &[&str] = &[
    "googlegroups.com",
    "groups.io",
    "freelists.org",
    "lists.sourceforge.net",
    "mailman",
];

/// Local parts of automated senders (lower case, substring match).
const AUTOMATED_LOCAL_PARTS: &[&str] = &[
    "noreply",
    "no-reply",
    "no_reply",
    "donotreply",
    "do-not-reply",
    "do_not_reply",
    "notification",
    "notify",
    "alert",
    "mailer-daemon",
    "postmaster",
    "receipt",
    "billing",
    "invoice",
    "statement",
    "account",
    "security",
    "auto-confirm",
    "shipment",
    "shipping",
    "order",
];

/// Local parts of marketing senders (whole local part, or a prefix).
const MARKETING_LOCAL_PARTS: &[&str] = &[
    "newsletter",
    "news",
    "marketing",
    "offers",
    "offer",
    "deals",
    "promo",
    "promotions",
    "sales",
    "specials",
    "shop",
    "store",
];

/// Subject words of marketing mail (lower case, substring match).
const MARKETING_SUBJECT: &[&str] = &[
    "% off",
    "sale",
    "deal",
    "offer",
    "discount",
    "coupon",
    "promo",
    "free shipping",
    "newsletter",
    "limited time",
    "last chance",
    "exclusive",
    "new arrivals",
    "black friday",
    "cyber monday",
    "save up to",
    "don't miss",
];

/// Subject words of transactional mail (lower case, substring match).
const TRANSACTIONAL_SUBJECT: &[&str] = &[
    "receipt",
    "invoice",
    "your order",
    "order confirm",
    "shipped",
    "delivered",
    "delivery",
    "your account",
    "password",
    "verify",
    "verification",
    "security alert",
    "sign-in",
    "sign in",
    "login",
    "statement",
    "payment",
    "your bill",
    "reminder",
    "notification",
    "booking",
    "reservation",
    "itinerary",
    "confirmation",
    "renewal",
    "subscription",
    "code",
];

/// What the classifier knows about a message. Fill it with
/// [`MailFacts::observe`] for each header, or set the fields directly.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MailFacts {
    /// Sender address, lower case.
    pub from: Option<String>,
    /// Decoded subject.
    pub subject: Option<String>,
    pub list_id: Option<String>,
    pub list_post: Option<String>,
    pub list_unsubscribe: bool,
    /// `Precedence`, lower case.
    pub precedence: Option<String>,
    /// `Auto-Submitted`, lower case.
    pub auto_submitted: Option<String>,
    /// It has `In-Reply-To` or `References`.
    pub is_reply: bool,
    /// A marketing platform's campaign header or mailer.
    pub campaign: bool,
    /// A bulk-mail service's header (marketing or transactional).
    pub bulk_service: bool,
    /// `X-Autoreply` and friends: an automatic answer.
    pub auto_reply: bool,
}

impl MailFacts {
    /// Facts from raw `(name, value)` header pairs.
    pub fn from_headers<'a>(headers: impl IntoIterator<Item = (&'a str, &'a str)>) -> Self {
        let mut facts = Self::default();
        for (name, value) in headers {
            facts.observe(name, value);
        }
        facts
    }

    /// Takes note of one header. Unknown headers are ignored.
    pub fn observe(&mut self, name: &str, value: &str) {
        let name = name.trim().to_ascii_lowercase();
        let value = value.trim();
        match name.as_str() {
            "from" if self.from.is_none() => self.from = address(value),
            "subject" if self.subject.is_none() => self.subject = Some(value.to_owned()),
            "list-id" => self.list_id = Some(value.to_ascii_lowercase()),
            "list-post" => self.list_post = Some(value.to_ascii_lowercase()),
            "list-unsubscribe" => self.list_unsubscribe = true,
            "precedence" => self.precedence = Some(value.to_ascii_lowercase()),
            "auto-submitted" => self.auto_submitted = Some(value.to_ascii_lowercase()),
            "in-reply-to" | "references" if !value.is_empty() => self.is_reply = true,
            "x-mailer" => {
                let mailer = value.to_ascii_lowercase();
                if CAMPAIGN_MAILERS.iter().any(|m| mailer.contains(m)) {
                    self.campaign = true;
                }
            }
            "x-autoreply" | "x-autorespond" => self.auto_reply = true,
            name if CAMPAIGN_HEADERS.contains(&name) || name.starts_with("x-mailchimp-") => {
                self.campaign = true;
            }
            name if BULK_SERVICE_HEADERS.contains(&name) => self.bulk_service = true,
            _ => {}
        }
    }
}

/// Picks the inbox tab for a message.
///
/// In order: social networks → Social; discussion lists (a postable
/// `List-Id`, Google Groups and similar) → Forums; bulk mail (an unsubscribe
/// link, a bulk-mail service or `Precedence: bulk`) → Promotions when it
/// looks like marketing, otherwise Updates; other automated mail
/// (`Auto-Submitted`, no-reply senders) → Updates; everything else, and
/// replies from people, → Primary.
pub fn classify(facts: &MailFacts) -> MailCategory {
    let from = facts.from.as_deref().unwrap_or_default();
    let (local, domain) = from.rsplit_once('@').unwrap_or((from, ""));
    let subject = facts.subject.as_deref().unwrap_or_default().to_lowercase();

    if SOCIAL_DOMAINS.iter().any(|d| domain_matches(domain, d)) {
        return MailCategory::Social;
    }

    let list_id = facts.list_id.as_deref().unwrap_or_default();
    let postable = facts
        .list_post
        .as_deref()
        .is_some_and(|post| post.contains("mailto:") || post.contains("http"));
    let forum_host = FORUM_HOSTS
        .iter()
        .any(|host| list_id.contains(host) || domain.contains(host));
    let precedence = facts.precedence.as_deref().unwrap_or_default();
    if forum_host
        || (!list_id.is_empty() && postable)
        || (!list_id.is_empty() && precedence == "list" && !facts.campaign)
    {
        return MailCategory::Forums;
    }

    let auto_submitted = facts
        .auto_submitted
        .as_deref()
        .is_some_and(|value| value != "no");
    let automated_sender = AUTOMATED_LOCAL_PARTS.iter().any(|p| local.contains(p));
    let marketing_sender = MARKETING_LOCAL_PARTS.iter().any(|p| {
        local == *p || local.starts_with(&format!("{p}.")) || local.starts_with(&format!("{p}-"))
    });
    let marketing_subject = MARKETING_SUBJECT.iter().any(|w| subject.contains(w));
    let transactional_subject = TRANSACTIONAL_SUBJECT.iter().any(|w| subject.contains(w));
    let bulk_precedence = precedence == "bulk" || precedence == "junk";
    let bulk = facts.list_unsubscribe || facts.bulk_service || facts.campaign || bulk_precedence;

    if bulk {
        let promo = 3 * u8::from(facts.campaign)
            + 2 * u8::from(marketing_subject)
            + 2 * u8::from(marketing_sender)
            + u8::from(bulk_precedence && !auto_submitted)
            + u8::from(facts.bulk_service && facts.list_unsubscribe);
        let update = 2 * u8::from(auto_submitted)
            + 2 * u8::from(transactional_subject)
            + 2 * u8::from(automated_sender);
        return if promo > update {
            MailCategory::Promotions
        } else {
            MailCategory::Updates
        };
    }
    if facts.is_reply && !auto_submitted && !facts.auto_reply {
        return MailCategory::Primary;
    }
    if auto_submitted || automated_sender {
        return MailCategory::Updates;
    }
    MailCategory::Primary
}

fn domain_matches(domain: &str, wanted: &str) -> bool {
    domain == wanted
        || domain
            .strip_suffix(wanted)
            .is_some_and(|rest| rest.ends_with('.'))
}

/// `Name <Ada@Example.org>` → `ada@example.org`.
fn address(value: &str) -> Option<String> {
    let inner = match (value.rfind('<'), value.rfind('>')) {
        (Some(start), Some(end)) if start < end => &value[start + 1..end],
        _ => value,
    };
    let inner = inner.trim().trim_matches('"');
    (inner.contains('@')).then(|| inner.to_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn classify_headers(headers: &str) -> MailCategory {
        let pairs = headers.lines().filter_map(|line| line.split_once(':'));
        classify(&MailFacts::from_headers(pairs))
    }

    #[test]
    fn storage_numbers_are_stable() {
        let numbers: Vec<i64> = MailCategory::ALL.iter().map(|c| c.to_storage()).collect();
        assert_eq!(numbers, [1, 2, 3, 4, 5]);
        for category in MailCategory::ALL {
            assert_eq!(
                MailCategory::from_storage(category.to_storage()),
                Some(category)
            );
            assert_eq!(category.as_str().parse::<MailCategory>().unwrap(), category);
        }
        assert_eq!(MailCategory::from_storage(0), None);
        assert!("inbox".parse::<MailCategory>().is_err());
    }

    #[test]
    fn a_person_is_primary() {
        assert_eq!(
            classify_headers(
                "From: Ada Lovelace <ada@example.org>\n\
                 To: bob@example.net\n\
                 Subject: Lunch on Friday?\n\
                 Message-ID: <1@example.org>"
            ),
            MailCategory::Primary
        );
        assert_eq!(
            classify_headers(
                "From: kenneth.lay@enron.com\n\
                 Subject: Re: Budget\n\
                 In-Reply-To: <2@enron.com>\n\
                 X-Mailer: Microsoft Outlook 16.0"
            ),
            MailCategory::Primary
        );
    }

    #[test]
    fn social_networks() {
        assert_eq!(
            classify_headers(
                "From: Facebook <notification@facebookmail.com>\n\
                 Subject: Ada commented on your post\n\
                 List-Unsubscribe: <https://www.facebook.com/o.php?k=1>\n\
                 Precedence: bulk"
            ),
            MailCategory::Social
        );
        assert_eq!(
            classify_headers(
                "From: LinkedIn <messages-noreply@linkedin.com>\n\
                 Subject: You appeared in 9 searches this week"
            ),
            MailCategory::Social
        );
        assert_eq!(
            classify_headers("From: Bob <bob@notlinkedin.com>\nSubject: hi"),
            MailCategory::Primary,
            "only the domain and its subdomains"
        );
    }

    #[test]
    fn mailing_lists_are_forums() {
        assert_eq!(
            classify_headers(
                "From: Ada <ada@example.org>\n\
                 Subject: [rust-users] Re: Borrow checker\n\
                 List-Id: Rust users <rust-users.lists.rust-lang.org>\n\
                 List-Post: <mailto:rust-users@lists.rust-lang.org>\n\
                 List-Unsubscribe: <mailto:rust-users-leave@lists.rust-lang.org>\n\
                 Precedence: list\n\
                 In-Reply-To: <3@example.org>"
            ),
            MailCategory::Forums
        );
        assert_eq!(
            classify_headers(
                "From: Bob <bob@example.com>\n\
                 Subject: Meetup next week\n\
                 List-Id: <hikers.googlegroups.com>\n\
                 List-Unsubscribe: <mailto:googlegroups-manage+1@googlegroups.com>"
            ),
            MailCategory::Forums
        );
    }

    #[test]
    fn newsletters_and_offers_are_promotions() {
        assert_eq!(
            classify_headers(
                "From: Shop <news@shop.example>\n\
                 Subject: 30% off everything this weekend\n\
                 List-Id: <123456.list-id.mcsv.net>\n\
                 List-Unsubscribe: <https://shop.us1.list-manage.com/unsubscribe>\n\
                 X-MC-User: abc123\n\
                 X-Campaign: mailchimpabc.def\n\
                 Precedence: bulk"
            ),
            MailCategory::Promotions
        );
        assert_eq!(
            classify_headers(
                "From: Deals <deals@store.example>\n\
                 Subject: Fresh picks for you\n\
                 List-Unsubscribe: <mailto:unsub@store.example>\n\
                 X-SG-EID: abc\n\
                 Feedback-ID: 1:campaign:sendgrid"
            ),
            MailCategory::Promotions
        );
        assert_eq!(
            classify_headers(
                "From: Weekly <team@startup.example>\n\
                 Subject: Our spring newsletter\n\
                 List-Unsubscribe: <https://startup.example/u>\n\
                 X-Mailer: Sendinblue"
            ),
            MailCategory::Promotions
        );
    }

    #[test]
    fn notifications_and_receipts_are_updates() {
        assert_eq!(
            classify_headers(
                "From: Shop <no-reply@shop.example>\n\
                 Subject: Your order #1234 has shipped\n\
                 List-Unsubscribe: <https://shop.example/prefs>\n\
                 X-SES-Outgoing: 2026.09.26-54.240.1.1\n\
                 Feedback-ID: 1.us-east-1.abc:AmazonSES"
            ),
            MailCategory::Updates
        );
        assert_eq!(
            classify_headers(
                "From: GitHub <notifications@github.com>\n\
                 Subject: [katna/katna] Fix threading (#42)\n\
                 In-Reply-To: <katna/katna/pull/42@github.com>\n\
                 List-Id: katna/katna <katna.katna.github.com>\n\
                 List-Unsubscribe: <mailto:unsub@reply.github.com>\n\
                 X-Auto-Response-Suppress: All"
            ),
            MailCategory::Updates
        );
        assert_eq!(
            classify_headers(
                "From: Bank <alerts@bank.example>\n\
                 Subject: Your statement is ready\n\
                 Auto-Submitted: auto-generated"
            ),
            MailCategory::Updates
        );
        assert_eq!(
            classify_headers(
                "From: Service <service@saas.example>\n\
                 Subject: Weekly usage report\n\
                 List-Unsubscribe: <https://saas.example/u>"
            ),
            MailCategory::Updates,
            "an unsubscribe link without marketing signs"
        );
        assert_eq!(
            classify_headers("From: MAILER-DAEMON@mx.example.org\nSubject: Undelivered Mail"),
            MailCategory::Updates
        );
    }

    #[test]
    fn auto_submitted_no_is_a_person() {
        assert_eq!(
            classify_headers("From: ada@example.org\nSubject: Hi\nAuto-Submitted: no"),
            MailCategory::Primary
        );
    }

    #[test]
    fn extracts_the_sender_address() {
        assert_eq!(
            address("\"Ada\" <Ada@Example.org>").as_deref(),
            Some("ada@example.org")
        );
        assert_eq!(
            address("bob@example.net").as_deref(),
            Some("bob@example.net")
        );
        assert_eq!(address("undisclosed-recipients:;"), None);
    }
}
