// SPDX-License-Identifier: GPL-3.0-or-later

//! Inbox tabs per account. Each mail provider's webmail splits the inbox
//! its own way, so Katna does the same: Gmail's five categories for Gmail,
//! Focused and Other for Outlook, Newsletters and Notifications for Zoho.
//! Other accounts get Gmail's categories, sorted by header rules. The
//! Settings page can pick another set, turn single tabs off (their mail
//! then shows in the first tab) or turn tabs off. No GPUI here.

use katna_core::MailCategory;
use katna_core::config::{AccountTabs, TabStyle};

/// Who runs an account's mail server, as far as tabs care.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provider {
    Gmail,
    Microsoft,
    Zoho,
    Other,
}

impl Provider {
    /// Guesses the provider from the address and the IMAP host.
    pub fn detect(address: &str, imap_host: Option<&str>) -> Self {
        let domain = address
            .rsplit_once('@')
            .map_or("", |(_, d)| d)
            .to_ascii_lowercase();
        let host = imap_host.unwrap_or_default().to_ascii_lowercase();
        let is = |hosts: &[&str], domains: &[&str]| {
            hosts
                .iter()
                .any(|h| host == *h || host.ends_with(&format!(".{h}")))
                || domains.iter().any(|d| {
                    domain == *d
                        || domain
                            .strip_prefix(d.trim_end_matches('*'))
                            .is_some_and(|rest| d.ends_with('*') && !rest.is_empty())
                })
        };
        if is(
            &["gmail.com", "googlemail.com"],
            &["gmail.com", "googlemail.com"],
        ) {
            Self::Gmail
        } else if is(
            &["outlook.com", "office365.com", "hotmail.com"],
            &["outlook.*", "hotmail.*", "live.*", "msn.com"],
        ) {
            Self::Microsoft
        } else if is(
            &["zoho.com", "zoho.eu", "zoho.in", "zohomail.com"],
            &["zoho.*", "zohomail.*"],
        ) {
            Self::Zoho
        } else {
            Self::Other
        }
    }

    /// The tab set [`TabStyle::Auto`] picks.
    pub fn style(self) -> TabStyle {
        match self {
            Self::Gmail | Self::Other => TabStyle::Gmail,
            Self::Microsoft => TabStyle::Focused,
            Self::Zoho => TabStyle::Zoho,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Gmail => "Gmail",
            Self::Microsoft => "Outlook",
            Self::Zoho => "Zoho Mail",
            Self::Other => "sorted by Katna",
        }
    }
}

/// One inbox tab.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tab {
    /// Stable name, as in [`AccountTabs::hidden`].
    pub key: &'static str,
    pub label: &'static str,
    pub icon: &'static str,
    /// Index into the theme's tab colors.
    pub color: usize,
    /// The categories whose mail the tab lists.
    pub categories: Vec<MailCategory>,
}

impl Tab {
    fn new(
        key: &'static str,
        label: &'static str,
        icon: &'static str,
        color: usize,
        categories: &[MailCategory],
    ) -> Self {
        Self {
            key,
            label,
            icon,
            color,
            categories: categories.to_vec(),
        }
    }
}

/// Every tab of `style`, the first being the one that takes what the
/// others do not. Empty for [`TabStyle::Off`]; `Auto` must be resolved
/// first.
pub fn all_tabs(style: TabStyle) -> Vec<Tab> {
    use MailCategory::*;
    match style {
        TabStyle::Auto | TabStyle::Gmail => vec![
            Tab::new("primary", "Primary", "inbox", 0, &[Primary]),
            Tab::new("promotions", "Promotions", "tag", 1, &[Promotions]),
            Tab::new("social", "Social", "people", 2, &[Social]),
            Tab::new("updates", "Updates", "info", 3, &[Updates]),
            Tab::new("forums", "Forums", "forum", 4, &[Forums]),
        ],
        TabStyle::Focused => vec![
            Tab::new("focused", "Focused", "inbox", 0, &[Primary]),
            Tab::new(
                "other",
                "Other",
                "all-mail",
                3,
                &[Promotions, Social, Updates, Forums],
            ),
        ],
        TabStyle::Zoho => vec![
            Tab::new("inbox", "Inbox", "inbox", 0, &[Primary]),
            Tab::new(
                "newsletters",
                "Newsletters",
                "tag",
                1,
                &[Promotions, Forums],
            ),
            Tab::new(
                "notifications",
                "Notifications",
                "info",
                3,
                &[Updates, Social],
            ),
        ],
        TabStyle::Off => Vec::new(),
    }
}

/// The style an account uses: its setting, or the provider's.
pub fn resolve(setting: &AccountTabs, provider: Provider) -> TabStyle {
    match setting.style {
        TabStyle::Auto => provider.style(),
        style => style,
    }
}

/// The tabs an account's inbox shows. A single tab is no tabs; a tab
/// turned off gives its categories to the first.
pub fn tabs(setting: &AccountTabs, provider: Provider) -> Vec<Tab> {
    let mut all = all_tabs(resolve(setting, provider));
    if all.is_empty() {
        return all;
    }
    let mut first = all.remove(0);
    let mut shown = Vec::new();
    for tab in all {
        if setting.hidden.iter().any(|h| h == tab.key) {
            first.categories.extend(tab.categories);
        } else {
            shown.push(tab);
        }
    }
    if shown.is_empty() {
        return Vec::new();
    }
    shown.insert(0, first);
    shown
}

#[cfg(test)]
mod tests {
    use super::*;
    use MailCategory::*;

    #[test]
    fn providers() {
        assert_eq!(Provider::detect("kay@gmail.com", None), Provider::Gmail);
        assert_eq!(
            Provider::detect("kay@company.example", Some("imap.gmail.com")),
            Provider::Gmail
        );
        assert_eq!(
            Provider::detect("kay@company.example", Some("outlook.office365.com")),
            Provider::Microsoft
        );
        assert_eq!(
            Provider::detect("kay@hotmail.co.uk", None),
            Provider::Microsoft
        );
        assert_eq!(
            Provider::detect("kay@livemail.example", None),
            Provider::Other
        );
        assert_eq!(
            Provider::detect("kay@hotmail.de", None),
            Provider::Microsoft
        );
        assert_eq!(
            Provider::detect("kay@outlook.com", None),
            Provider::Microsoft
        );
        assert_eq!(
            Provider::detect("kay@firm.in", Some("imappro.zoho.in")),
            Provider::Zoho
        );
        assert_eq!(Provider::detect("kay@zohomail.eu", None), Provider::Zoho);
        assert_eq!(
            Provider::detect("kay@example.org", Some("mail.example.org")),
            Provider::Other
        );
        assert_eq!(
            Provider::detect("kay@example.org", Some("notgmail.com")),
            Provider::Other
        );
    }

    fn keys(tabs: &[Tab]) -> Vec<&str> {
        tabs.iter().map(|t| t.key).collect()
    }

    #[test]
    fn each_provider_has_its_tabs() {
        let auto = AccountTabs::default();
        assert_eq!(
            keys(&tabs(&auto, Provider::Gmail)),
            ["primary", "promotions", "social", "updates", "forums"]
        );
        assert_eq!(
            keys(&tabs(&auto, Provider::Microsoft)),
            ["focused", "other"]
        );
        assert_eq!(
            keys(&tabs(&auto, Provider::Zoho)),
            ["inbox", "newsletters", "notifications"]
        );
        let off = AccountTabs {
            style: TabStyle::Off,
            ..AccountTabs::default()
        };
        assert!(tabs(&off, Provider::Gmail).is_empty());
        let chosen = AccountTabs {
            style: TabStyle::Focused,
            ..AccountTabs::default()
        };
        assert_eq!(keys(&tabs(&chosen, Provider::Zoho)), ["focused", "other"]);
    }

    #[test]
    fn every_category_is_in_exactly_one_tab() {
        for style in [TabStyle::Gmail, TabStyle::Focused, TabStyle::Zoho] {
            let tabs = all_tabs(style);
            for category in MailCategory::ALL {
                let n = tabs
                    .iter()
                    .filter(|t| t.categories.contains(&category))
                    .count();
                assert_eq!(n, 1, "{style:?} {category:?}");
            }
        }
    }

    #[test]
    fn hidden_tabs_go_to_the_first() {
        let setting = AccountTabs {
            style: TabStyle::Auto,
            hidden: vec!["updates".into(), "forums".into()],
        };
        let tabs = tabs(&setting, Provider::Gmail);
        assert_eq!(keys(&tabs), ["primary", "promotions", "social"]);
        assert_eq!(tabs[0].categories, [Primary, Updates, Forums]);
        // Only one tab left: no tabs.
        let setting = AccountTabs {
            style: TabStyle::Focused,
            hidden: vec!["other".into()],
        };
        assert!(super::tabs(&setting, Provider::Gmail).is_empty());
    }
}
