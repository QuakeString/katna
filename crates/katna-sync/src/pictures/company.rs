// SPDX-License-Identifier: GPL-3.0-or-later

//! What a company says about itself, for the Company section of a
//! person's card in the chat view (`docs/ARCHITECTURE.md`, the chat view).
//!
//! Read from the company's own home page: its `<title>`, the `og:` and
//! description tags, and the schema.org `Organization` many sites publish
//! for search engines (address, founding year, social pages). For a
//! company Wikidata knows by the same website, Wikipedia's first lines add
//! a short summary; a company Wikidata lists under another website is
//! never matched, so the summary never belongs to another company. The
//! answer, or its absence, is kept a week beside the sender pictures.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    FREE_MAIL, Pictures, attributes, http, organizational_domain, read_fresh, valid_domain, within,
};

/// The most a company's text runs to.
const MAX_TEXT: usize = 320;
/// How much of a Wikidata or Wikipedia answer is read.
const MAX_API: usize = 256 * 1024;
/// Social and other pages a company links to, at most.
const MAX_LINKS: usize = 4;

/// A company, as its home page and Wikipedia describe it.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Company {
    pub name: String,
    /// What it does, from its own page.
    #[serde(default)]
    pub description: String,
    /// Its town and country, as it writes them.
    #[serde(default)]
    pub place: String,
    /// The year it started.
    #[serde(default)]
    pub founded: Option<i32>,
    /// Its home page.
    pub website: String,
    /// Its pages elsewhere (LinkedIn, Instagram…).
    #[serde(default)]
    pub links: Vec<String>,
    /// Wikipedia's first lines, when Wikidata gives it the same website.
    #[serde(default)]
    pub summary: String,
    /// When it was read (Unix seconds).
    #[serde(default)]
    pub checked: i64,
}

impl Pictures {
    /// The company of the person at `address`: the one at `website`, the
    /// site named in their signature, else the one their address belongs
    /// to unless that is a free-mail service. `None` when there is none
    /// or its page says nothing. The caller checks that the sender is
    /// authenticated.
    pub async fn company(&self, address: &str, website: &str) -> Option<Company> {
        let from_site = host_of(website).map(|h| organizational_domain(&h));
        let from_address = address
            .rsplit_once('@')
            .map(|(_, d)| organizational_domain(d.trim().trim_end_matches('.')));
        let org = [from_site, from_address]
            .into_iter()
            .flatten()
            .find(|org| valid_domain(org) && !FREE_MAIL.contains(&org.as_str()))?;
        let cached = self.cache.join(format!("{org}.company"));
        if let Some(bytes) = read_fresh(&cached) {
            return serde_json::from_slice(&bytes).ok();
        }
        let mut reached = false;
        let company = self.read_company(&org, &mut reached).await;
        if reached {
            let bytes = company
                .as_ref()
                .and_then(|c| serde_json::to_vec(c).ok())
                .unwrap_or_default();
            if let Err(err) =
                std::fs::create_dir_all(&self.cache).and_then(|()| std::fs::write(&cached, bytes))
            {
                tracing::debug!(%err, "cannot keep the company");
            }
        }
        company
    }

    async fn read_company(&self, org: &str, reached: &mut bool) -> Option<Company> {
        let mut found = None;
        for host in [org.to_owned(), format!("www.{org}")] {
            let Some(page) = self.fetch_head(&format!("https://{host}/"), reached).await else {
                continue;
            };
            found = from_page(&String::from_utf8_lossy(&page), &host);
            break;
        }
        let mut company = found?;
        company.checked = jiff::Timestamp::now().as_second();
        company.summary = self.wikipedia(&company.name, org).await.unwrap_or_default();
        Some(company)
    }

    /// Wikipedia's first lines on the company called `name`, when
    /// Wikidata lists `org` as its website.
    async fn wikipedia(&self, name: &str, org: &str) -> Option<String> {
        let search = format!(
            "https://www.wikidata.org/w/api.php?action=wbsearchentities&format=json&language=en&type=item&limit=3&search={}",
            encode(name)
        );
        let found: Value = self.api(&search).await?;
        let ids: Vec<&str> = found["search"]
            .as_array()?
            .iter()
            .filter_map(|e| e["id"].as_str())
            .collect();
        if ids.is_empty() {
            return None;
        }
        let entities = format!(
            "https://www.wikidata.org/w/api.php?action=wbgetentities&format=json&props=claims%7Csitelinks&sitefilter=enwiki&ids={}",
            ids.join("%7C")
        );
        let entities: Value = self.api(&entities).await?;
        let title = ids.iter().find_map(|id| {
            let entity = &entities["entities"][*id];
            let same_site = entity["claims"]["P856"].as_array()?.iter().any(|claim| {
                claim["mainsnak"]["datavalue"]["value"]
                    .as_str()
                    .and_then(host_of)
                    .is_some_and(|host| organizational_domain(&host) == org)
            });
            same_site.then(|| entity["sitelinks"]["enwiki"]["title"].as_str())?
        })?;
        let summary: Value = self
            .api(&format!(
                "https://en.wikipedia.org/api/rest_v1/page/summary/{}",
                encode(&title.replace(' ', "_"))
            ))
            .await?;
        let text = clean(summary["extract"].as_str()?);
        (!text.is_empty()).then(|| sentences(&text, MAX_TEXT))
    }

    async fn api(&self, url: &str) -> Option<Value> {
        let body = http::get_public(url, &self.tls, self.timeout, MAX_API)
            .await
            .inspect_err(|err| tracing::debug!(url, %err, "not read"))
            .ok()??;
        serde_json::from_slice(&body).ok()
    }
}

/// The company a home page on `host` describes, if it names itself.
fn from_page(html: &str, host: &str) -> Option<Company> {
    let lower = html.to_ascii_lowercase();
    let mut meta: Vec<(String, String)> = Vec::new();
    let mut at = 0;
    while let Some(start) = lower[at..].find("<meta").map(|i| at + i) {
        let end = lower[start..].find('>').map_or(lower.len(), |i| start + i);
        at = end;
        let attrs = attributes(&html[start + 5..end]);
        let get = |name: &str| attrs.iter().find(|(n, _)| n == name).map(|(_, v)| v);
        if let (Some(key), Some(content)) = (get("property").or(get("name")), get("content")) {
            meta.push((key.to_ascii_lowercase(), unescape(content)));
        }
    }
    let tag = |key: &str| {
        meta.iter()
            .find(|(k, v)| k == key && !v.trim().is_empty())
            .map(|(_, v)| clean(v))
    };
    let org = organization(html, &lower);
    let field = |key: &str| org.as_ref().and_then(|o| o[key].as_str()).map(clean);
    let title = lower.find("<title").and_then(|start| {
        let open = lower[start..].find('>')? + start + 1;
        let close = lower[open..].find("</title")? + open;
        Some(clean(&unescape(&html[open..close])))
    });
    let name = field("name")
        .or_else(|| tag("og:site_name"))
        .or_else(|| title.as_deref().map(site_name))
        .filter(|n| !n.is_empty() && n.chars().count() <= 80)?;
    let description = field("description")
        .or_else(|| tag("og:description"))
        .or_else(|| tag("description"))
        .map(|d| sentences(&d, MAX_TEXT))
        .unwrap_or_default();
    let place = org
        .as_ref()
        .map(|o| place(&o["address"]))
        .unwrap_or_default();
    let founded = org
        .as_ref()
        .and_then(|o| o["foundingDate"].as_str().or(o["foundingYear"].as_str()))
        .and_then(|d| d.get(..4)?.parse().ok())
        .filter(|y| (1500..=2100).contains(y));
    let mut links: Vec<String> = match org.as_ref().map(|o| &o["sameAs"]) {
        Some(Value::String(one)) => vec![one.clone()],
        Some(Value::Array(many)) => many
            .iter()
            .filter_map(|v| v.as_str().map(str::to_owned))
            .collect(),
        _ => Vec::new(),
    };
    links.retain(|url| url.starts_with("https://") && !within(url, &organizational_domain(host)));
    links.dedup();
    links.truncate(MAX_LINKS);
    Some(Company {
        name,
        description,
        place,
        founded,
        website: format!("https://{host}/"),
        links,
        ..Company::default()
    })
}

/// The schema.org organization in the page's JSON-LD, if any.
fn organization(html: &str, lower: &str) -> Option<Value> {
    let mut at = 0;
    while let Some(start) = lower[at..].find("<script").map(|i| at + i) {
        let open = lower[start..]
            .find('>')
            .map_or(lower.len(), |i| start + i + 1);
        let close = lower[open..]
            .find("</script")
            .map_or(lower.len(), |i| open + i);
        at = close;
        if !lower[start..open].contains("ld+json") {
            continue;
        }
        let Ok(json) = serde_json::from_str::<Value>(&html[open..close]) else {
            continue;
        };
        if let Some(org) = find_organization(&json) {
            return Some(org.clone());
        }
    }
    None
}

fn find_organization(value: &Value) -> Option<&Value> {
    match value {
        Value::Array(items) => items.iter().find_map(find_organization),
        Value::Object(map) => {
            let kinds: Vec<&str> = match &map.get("@type") {
                Some(Value::String(one)) => vec![one.as_str()],
                Some(Value::Array(many)) => many.iter().filter_map(Value::as_str).collect(),
                _ => Vec::new(),
            };
            let is_org = kinds.iter().any(|k| {
                matches!(
                    *k,
                    "Organization"
                        | "Corporation"
                        | "LocalBusiness"
                        | "ProfessionalService"
                        | "NGO"
                        | "EducationalOrganization"
                        | "OnlineBusiness"
                        | "OnlineStore"
                        | "Store"
                )
            });
            if is_org && map.get("name").is_some_and(Value::is_string) {
                return Some(value);
            }
            map.get("@graph").and_then(find_organization)
        }
        _ => None,
    }
}

/// "Mumbai, India" from a schema.org address (text or `PostalAddress`).
fn place(address: &Value) -> String {
    let one = |a: &Value| match a {
        Value::String(text) => clean(text),
        Value::Object(_) => {
            let country = match &a["addressCountry"] {
                Value::String(c) => Some(c.clone()),
                other => other["name"].as_str().map(str::to_owned),
            };
            [a["addressLocality"].as_str().map(str::to_owned), country]
                .into_iter()
                .flatten()
                .map(|s| clean(&s))
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join(", ")
        }
        _ => String::new(),
    };
    match address {
        Value::Array(many) => many.first().map(one).unwrap_or_default(),
        other => one(other),
    }
}

/// "Acme" from a page title such as "Acme | Tools for builders".
fn site_name(title: &str) -> String {
    title
        .split(['|', '\u{2013}', '\u{2014}', '\u{b7}', '\u{2022}', ':'])
        .next()
        .unwrap_or_default()
        .split(" - ")
        .next()
        .unwrap_or_default()
        .trim()
        .to_owned()
}

/// The host of a web address, with or without its `https://`.
fn host_of(url: &str) -> Option<String> {
    let url = url.trim();
    let rest = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);
    let host = rest
        .split(['/', '?', '#', ':'])
        .next()?
        .trim_end_matches('.')
        .to_ascii_lowercase();
    valid_domain(&host).then_some(host)
}

/// Whole sentences of `text` up to `max` characters, or its start.
fn sentences(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        return text.to_owned();
    }
    let cut: String = text.chars().take(max).collect();
    match cut.rfind(". ") {
        Some(end) if end > max / 3 => cut[..=end].to_owned(),
        _ => format!("{}…", cut.trim_end()),
    }
}

/// `text` on one line, its spaces single.
fn clean(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The usual HTML entities in an attribute or title.
fn unescape(text: &str) -> String {
    text.replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&#039;", "'")
        .replace("&apos;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&nbsp;", " ")
}

/// `text` for a URL's query or path.
fn encode(text: &str) -> String {
    let mut out = String::new();
    for b in text.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_page_with_its_organization() {
        let html = r#"<html><head><title>Demo Studio | Brand design</title>
<meta property="og:site_name" content="Demo Studio">
<meta name="description" content="Brand and product design for cafés &amp; shops.">
<script type="application/ld+json">{"@context":"https://schema.org","@graph":[{"@type":"WebSite","name":"x"},{"@type":["Organization"],"name":"Demo Studio Pvt Ltd","foundingDate":"2015-04-01","address":{"@type":"PostalAddress","addressLocality":"Mumbai","addressCountry":"IN"},"sameAs":["https://www.linkedin.com/company/demo","https://demostudio.example/about"]}]}</script>
</head>"#;
        let company = from_page(html, "demostudio.example").unwrap();
        assert_eq!(company.name, "Demo Studio Pvt Ltd");
        assert_eq!(
            company.description,
            "Brand and product design for cafés & shops."
        );
        assert_eq!(company.place, "Mumbai, IN");
        assert_eq!(company.founded, Some(2015));
        assert_eq!(company.links, ["https://www.linkedin.com/company/demo"]);
        assert_eq!(company.website, "https://demostudio.example/");
    }

    #[test]
    fn a_page_with_only_a_title() {
        let html = "<head><title>Acme – Tools for builders</title></head>";
        let company = from_page(html, "www.acme.example").unwrap();
        assert_eq!(company.name, "Acme");
        assert!(company.description.is_empty());
        assert_eq!(from_page("<head></head>", "x.example"), None);
    }

    #[test]
    fn hosts_and_text() {
        assert_eq!(
            host_of("www.kgservices.in").as_deref(),
            Some("www.kgservices.in")
        );
        assert_eq!(
            host_of("http://Example.org/about").as_deref(),
            Some("example.org")
        );
        assert_eq!(host_of("not a site"), None);
        assert_eq!(encode("K. G. Services"), "K.%20G.%20Services");
        let long = "One sentence here. ".repeat(30);
        assert!(sentences(&long, 60).ends_with('.'));
    }
}
