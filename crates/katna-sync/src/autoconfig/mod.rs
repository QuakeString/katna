// SPDX-License-Identifier: GPL-3.0-or-later

//! Finding an address's IMAP and SMTP servers (plan task 1.2), in the order
//! Thunderbird uses (`docs/ARCHITECTURE.md` §6.4):
//!
//! 1. Built-in settings for a few big providers (no network).
//! 2. The provider's own configuration file: `https://autoconfig.DOMAIN/`
//!    and `https://DOMAIN/.well-known/autoconfig/`, then Thunderbird's
//!    ISPDB. All three are asked at once; the first in this order wins.
//! 3. DNS SRV records (RFC 6186 and RFC 8314).
//! 4. The ISPDB entry of the domain that receives the mail (MX), which
//!    finds hosted domains such as Google Workspace or Fastmail.
//! 5. Guessing `imap.DOMAIN`, `mail.DOMAIN` and `smtp.DOMAIN` on the usual
//!    ports, keeping those that greet like a mail server.
//!
//! Configuration files only come over HTTPS. Servers that only offer
//! OAuth2 are skipped until Katna supports it. Whatever is found, adding the
//! account still checks the login.

pub(crate) mod dns;
pub mod http;

use std::{future::Future, net::SocketAddr, pin::Pin, time::Duration};

use futures_lite::{FutureExt, future};
use katna_core::{Security, Server};

use crate::{
    Error, Result,
    net::{Conn, Tls},
};

/// Where settings were found.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    BuiltIn,
    /// The provider's own configuration file.
    Provider,
    Ispdb,
    DnsSrv,
    /// ISPDB, for the domain the MX records point to.
    Mx,
    Guess,
}

impl Source {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BuiltIn => "built-in",
            Self::Provider => "provider",
            Self::Ispdb => "ispdb",
            Self::DnsSrv => "dns-srv",
            Self::Mx => "mx",
            Self::Guess => "guess",
        }
    }
}

/// Settings found for an address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Discovered {
    pub imap: Server,
    /// `None` when no SMTP server was found; the account can still read.
    pub smtp: Option<Server>,
    pub source: Source,
}

/// Which of an account's two servers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Imap,
    Smtp,
}

/// Thunderbird's database of provider settings.
pub const ISPDB: &str = "https://autoconfig.thunderbird.net/v1.1/";

/// How to reach the network while discovering.
#[derive(Clone)]
pub struct Discovery {
    /// ISPDB base URL; the domain is appended.
    pub ispdb: String,
    /// DNS servers for SRV and MX lookups.
    pub resolvers: Vec<SocketAddr>,
    pub tls: Tls,
    /// Limit for each request or probe.
    pub timeout: Duration,
}

impl Discovery {
    /// The system's resolvers and trust store, and the public ISPDB.
    pub fn system() -> Result<Self> {
        Ok(Self {
            ispdb: ISPDB.to_owned(),
            resolvers: dns::system_resolvers(),
            tls: Tls::system()?,
            timeout: Duration::from_secs(8),
        })
    }

    /// Finds the servers of `address`.
    pub async fn discover(&self, address: &str) -> std::result::Result<Discovered, String> {
        discover_with(self, address).await
    }
}

/// What discovery needs from the network; tests replace it.
trait Network {
    fn ispdb(&self) -> &str;
    /// The body of an HTTPS `200` answer.
    fn fetch(&self, url: String) -> impl Future<Output = Option<Vec<u8>>>;
    fn srv(&self, name: String) -> impl Future<Output = Vec<dns::Srv>>;
    fn mx(&self, name: String) -> impl Future<Output = Vec<dns::Mx>>;
    /// Whether a mail server of that kind answers there.
    fn probe(&self, server: Server, protocol: Protocol) -> impl Future<Output = bool>;
}

impl Network for Discovery {
    fn ispdb(&self) -> &str {
        &self.ispdb
    }

    async fn fetch(&self, url: String) -> Option<Vec<u8>> {
        match http::get(&url, &self.tls, self.timeout).await {
            Ok(body) => body,
            Err(err) => {
                tracing::debug!(url, %err, "autoconfig fetch failed");
                None
            }
        }
    }

    async fn srv(&self, name: String) -> Vec<dns::Srv> {
        dns::srv(&self.resolvers, &name, self.timeout).await
    }

    async fn mx(&self, name: String) -> Vec<dns::Mx> {
        dns::mx(&self.resolvers, &name, self.timeout).await
    }

    async fn probe(&self, server: Server, protocol: Protocol) -> bool {
        let greeting = async {
            let mut conn = Conn::new(self.tls.clone());
            match server.security {
                Security::Tls => conn.connect_tls(&server.host, server.port).await?,
                _ => conn.connect_tcp(&server.host, server.port).await?,
            }
            let line = conn.read().await?.to_vec();
            let _ = conn.close().await;
            Ok::<_, Error>(line)
        };
        let greeting = greeting
            .or(async {
                async_io::Timer::after(self.timeout).await;
                Err(Error::Timeout(self.timeout))
            })
            .await;
        match greeting {
            Ok(line) => match protocol {
                Protocol::Imap => line.starts_with(b"* OK") || line.starts_with(b"* PREAUTH"),
                Protocol::Smtp => line.starts_with(b"220"),
            },
            Err(_) => false,
        }
    }
}

async fn discover_with(
    net: &impl Network,
    address: &str,
) -> std::result::Result<Discovered, String> {
    let address = address.trim();
    let (local, domain) = address
        .rsplit_once('@')
        .filter(|(local, domain)| !local.is_empty() && domain.contains('.'))
        .ok_or_else(|| format!("{address:?} is not an email address"))?;
    let domain = domain.to_ascii_lowercase();
    let user = User {
        address,
        local,
        domain: &domain,
    };
    if let Some(found) = built_in(&user) {
        return Ok(found);
    }

    let urls = [
        format!(
            "https://autoconfig.{domain}/mail/config-v1.1.xml?emailaddress={}",
            query_escape(address)
        ),
        format!("https://{domain}/.well-known/autoconfig/mail/config-v1.1.xml"),
        format!("{}{domain}", net.ispdb()),
    ];
    let bodies = join_all(urls.into_iter().map(|url| net.fetch(url)).collect()).await;
    let mut oauth_only = false;
    for (index, body) in bodies.into_iter().enumerate() {
        let Some(body) = body else { continue };
        match parse_config(&body, &user) {
            Ok(Some((imap, smtp))) => {
                let source = if index < 2 {
                    Source::Provider
                } else {
                    Source::Ispdb
                };
                return Ok(Discovered { imap, smtp, source });
            }
            Ok(None) => oauth_only = true,
            Err(err) => tracing::debug!(%err, "unusable autoconfig file"),
        }
    }
    // The provider says how to sign in; guessing would only find servers
    // that refuse the password.
    if oauth_only {
        return Err(oauth_error(&domain));
    }

    if let Some(found) = from_srv(net, &user).await {
        return Ok(found);
    }

    // Hosted mail: ask the ISPDB about the domain the MX points to.
    let mut tried = Vec::new();
    for mx in net.mx(domain.clone()).await.into_iter().take(3) {
        let base = base_domain(&mx.exchange);
        if base == domain || tried.contains(&base) {
            continue;
        }
        tried.push(base.clone());
        if let Some(body) = net.fetch(format!("{}{base}", net.ispdb())).await {
            match parse_config(&body, &user) {
                Ok(Some((imap, smtp))) => {
                    return Ok(Discovered {
                        imap,
                        smtp,
                        source: Source::Mx,
                    });
                }
                Ok(None) => return Err(oauth_error(&domain)),
                Err(err) => tracing::debug!(%err, "unusable ISPDB file"),
            }
        }
    }

    if let Some(found) = guess(net, &user).await {
        return Ok(found);
    }
    Err(format!(
        "no mail servers found for {domain}; give them by hand"
    ))
}

fn oauth_error(domain: &str) -> String {
    format!("{domain} only allows OAuth2 sign-in, which Katna does not support yet")
}

struct User<'a> {
    address: &'a str,
    local: &'a str,
    domain: &'a str,
}

impl User<'_> {
    /// Fills in Thunderbird's placeholders.
    fn expand(&self, text: &str) -> String {
        text.replace("%EMAILADDRESS%", self.address)
            .replace("%EMAILLOCALPART%", self.local)
            .replace("%EMAILDOMAIN%", self.domain)
    }
}

/// (domains, IMAP host, SMTP host, SMTP security, login is the local part).
const BUILT_IN: &[(&[&str], &str, &str, Security, bool)] = &[
    (
        &["gmail.com", "googlemail.com"],
        "imap.gmail.com",
        "smtp.gmail.com",
        Security::Tls,
        false,
    ),
    (
        &["yahoo.com", "ymail.com"],
        "imap.mail.yahoo.com",
        "smtp.mail.yahoo.com",
        Security::Tls,
        false,
    ),
    (
        &["icloud.com", "me.com", "mac.com"],
        "imap.mail.me.com",
        "smtp.mail.me.com",
        Security::StartTls,
        true,
    ),
    (
        &["fastmail.com", "fastmail.fm"],
        "imap.fastmail.com",
        "smtp.fastmail.com",
        Security::Tls,
        false,
    ),
];

fn built_in(user: &User<'_>) -> Option<Discovered> {
    let (_, imap, smtp, smtp_security, local_login) = BUILT_IN
        .iter()
        .find(|(domains, ..)| domains.contains(&user.domain))?;
    let username = if *local_login {
        user.local
    } else {
        user.address
    };
    Some(Discovered {
        imap: server(imap, 993, Security::Tls, username),
        smtp: Some(server(
            smtp,
            if *smtp_security == Security::Tls {
                465
            } else {
                587
            },
            *smtp_security,
            username,
        )),
        source: Source::BuiltIn,
    })
}

fn server(host: &str, port: u16, security: Security, username: &str) -> Server {
    Server {
        host: host.to_owned(),
        port,
        security,
        username: username.to_owned(),
        accept_invalid_certs: false,
    }
}

/// Reads a Thunderbird `config-v1.1.xml`. Returns the best IMAP and SMTP
/// servers that take a password, `None` if the IMAP servers only take
/// OAuth2, and an error if there is no IMAP server at all.
fn parse_config(
    xml: &[u8],
    user: &User<'_>,
) -> std::result::Result<Option<(Server, Option<Server>)>, String> {
    let text = std::str::from_utf8(xml).map_err(|_| "not UTF-8".to_owned())?;
    let doc = roxmltree::Document::parse(text).map_err(|err| err.to_string())?;
    let provider = doc
        .descendants()
        .find(|node| node.has_tag_name("emailProvider"))
        .ok_or("no emailProvider")?;
    let mut any_imap = false;
    let mut best: [Option<Server>; 2] = [None, None];
    for node in provider.children().filter(|n| n.is_element()) {
        let slot = match (node.tag_name().name(), node.attribute("type")) {
            ("incomingServer", Some("imap")) => 0,
            ("outgoingServer", Some("smtp")) => 1,
            _ => continue,
        };
        any_imap |= slot == 0;
        let field = |name: &str| {
            node.children()
                .find(|c| c.has_tag_name(name))
                .and_then(|c| c.text())
                .map(|text| user.expand(text.trim()))
        };
        let passwords = node
            .children()
            .filter(|c| c.has_tag_name("authentication"))
            .filter_map(|c| c.text())
            .any(|auth| auth.trim().starts_with("password-"));
        let host = field("hostname").unwrap_or_default().to_ascii_lowercase();
        let port = field("port").and_then(|p| p.parse::<u16>().ok());
        // Thunderbird writes SSL; some providers write TLS for the same.
        let security = match field("socketType")
            .map(|t| t.to_ascii_uppercase())
            .as_deref()
        {
            Some("SSL" | "TLS") => Security::Tls,
            Some("STARTTLS") => Security::StartTls,
            Some("PLAIN") => Security::Plain,
            _ => continue,
        };
        let (Some(port), true) = (port, passwords && valid_host(&host)) else {
            continue;
        };
        let username = field("username").unwrap_or_else(|| user.address.to_owned());
        let candidate = server(&host, port, security, &username);
        // TLS beats STARTTLS beats plain; the file's order breaks ties.
        let better = best[slot]
            .as_ref()
            .is_none_or(|current| rank(candidate.security) < rank(current.security));
        if better {
            best[slot] = Some(candidate);
        }
    }
    let [imap, smtp] = best;
    match imap {
        Some(imap) => Ok(Some((imap, smtp))),
        None if any_imap => Ok(None),
        None => Err("no IMAP server".into()),
    }
}

fn rank(security: Security) -> u8 {
    match security {
        Security::Tls => 0,
        Security::StartTls => 1,
        Security::Plain => 2,
    }
}

fn valid_host(host: &str) -> bool {
    !host.is_empty()
        && host.len() <= 253
        && host
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'.' || b == b'-')
}

/// RFC 6186 and RFC 8314 records, implicit TLS first.
async fn from_srv(net: &impl Network, user: &User<'_>) -> Option<Discovered> {
    let domain = user.domain;
    let names = [
        format!("_imaps._tcp.{domain}"),
        format!("_imap._tcp.{domain}"),
        format!("_submissions._tcp.{domain}"),
        format!("_submission._tcp.{domain}"),
    ];
    let found = join_all(names.into_iter().map(|name| net.srv(name)).collect()).await;
    let pick = |records: &[dns::Srv], security| {
        records
            .iter()
            .find(|r| r.port != 0 && valid_host(&r.target))
            .map(|r| server(&r.target, r.port, security, user.address))
    };
    let imap = pick(&found[0], Security::Tls).or_else(|| pick(&found[1], Security::StartTls))?;
    let smtp = pick(&found[2], Security::Tls).or_else(|| pick(&found[3], Security::StartTls));
    Some(Discovered {
        imap,
        smtp,
        source: Source::DnsSrv,
    })
}

/// Probes the usual host names and ports at once; the first candidate in
/// this order that answers wins.
async fn guess(net: &impl Network, user: &User<'_>) -> Option<Discovered> {
    let domain = user.domain;
    let candidates = |prefixes: &[&str], ports: [(u16, Security); 2]| {
        let mut list = Vec::new();
        for prefix in prefixes {
            for (port, security) in ports {
                list.push(server(
                    &format!("{prefix}.{domain}"),
                    port,
                    security,
                    user.address,
                ));
            }
        }
        list
    };
    let imap = candidates(
        &["imap", "mail"],
        [(993, Security::Tls), (143, Security::StartTls)],
    );
    let smtp = candidates(
        &["smtp", "mail"],
        [(465, Security::Tls), (587, Security::StartTls)],
    );
    let probes = imap
        .iter()
        .map(|s| (s.clone(), Protocol::Imap))
        .chain(smtp.iter().map(|s| (s.clone(), Protocol::Smtp)))
        .map(|(s, protocol)| net.probe(s, protocol))
        .collect();
    let answered = join_all(probes).await;
    let (imap_ok, smtp_ok) = answered.split_at(imap.len());
    let first = |servers: &[Server], ok: &[bool]| {
        servers
            .iter()
            .zip(ok)
            .find(|(_, ok)| **ok)
            .map(|(s, _)| s.clone())
    };
    Some(Discovered {
        imap: first(&imap, imap_ok)?,
        smtp: first(&smtp, smtp_ok),
        source: Source::Guess,
    })
}

/// The registrable part of a host name, roughly: the last two labels, or
/// three under a short second-level label such as `co.uk`.
fn base_domain(host: &str) -> String {
    let labels: Vec<&str> = host.trim_end_matches('.').split('.').collect();
    let keep = match labels.as_slice() {
        [.., second, top] if top.len() == 2 && second.len() <= 3 && labels.len() >= 3 => 3,
        _ => 2,
    };
    labels[labels.len().saturating_sub(keep)..]
        .join(".")
        .to_ascii_lowercase()
}

fn query_escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for byte in text.bytes() {
        if byte.is_ascii_alphanumeric() || b"-._~".contains(&byte) {
            out.push(char::from(byte));
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
    }
    out
}

/// Runs the futures at once and returns their outputs in order.
async fn join_all<F: Future>(futures: Vec<F>) -> Vec<F::Output> {
    let mut futures: Vec<Pin<Box<F>>> = futures.into_iter().map(Box::pin).collect();
    let mut outputs: Vec<Option<F::Output>> = futures.iter().map(|_| None).collect();
    future::poll_fn(|cx| {
        let mut pending = false;
        for (future, output) in futures.iter_mut().zip(outputs.iter_mut()) {
            if output.is_none() {
                match future.as_mut().poll(cx) {
                    std::task::Poll::Ready(value) => *output = Some(value),
                    std::task::Poll::Pending => pending = true,
                }
            }
        }
        if pending {
            std::task::Poll::Pending
        } else {
            std::task::Poll::Ready(())
        }
    })
    .await;
    outputs.into_iter().flatten().collect()
}

#[cfg(test)]
mod tests;
