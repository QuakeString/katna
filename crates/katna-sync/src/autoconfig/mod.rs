// SPDX-License-Identifier: GPL-3.0-or-later

//! Finding an address's IMAP, POP3 and SMTP servers (plan task 1.2), in the order
//! Thunderbird uses (`docs/ARCHITECTURE.md` §6.4):
//!
//! 1. Built-in settings for a few big providers (no network).
//! 2. The provider's own configuration file: `https://autoconfig.DOMAIN/`
//!    and `https://DOMAIN/.well-known/autoconfig/`, then Thunderbird's
//!    ISPDB. All three are asked at once; the first in this order wins.
//! 3. DNS SRV records (RFC 6186 and RFC 8314).
//! 4. The ISPDB entry of the domain that receives the mail (MX), which
//!    finds hosted domains such as Google Workspace or Fastmail.
//! 5. Guessing `imap.DOMAIN`, `pop.DOMAIN`, `mail.DOMAIN` and `smtp.DOMAIN`
//!    on the usual ports, keeping those that greet like a mail server.
//!
//! Each step looks for IMAP and POP3 alike; an account takes IMAP when the
//! provider offers both, and POP3 only when that is all it has.
//!
//! Configuration files only come over HTTPS. Google and Microsoft servers
//! are marked for OAuth2 sign-in ([`Discovered::oauth`]); other servers that
//! only offer OAuth2 are skipped. Whatever is found, adding the account
//! still checks the login.

pub(crate) mod dns;
pub mod http;

use std::{future::Future, net::SocketAddr, pin::Pin, time::Duration};

use futures_lite::{FutureExt, future};
use katna_core::{OAuthProvider, Security, Server};

use crate::{
    Error, Result,
    net::{Conn, Tls},
    oauth::Provider,
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
    /// The IMAP server; `None` when the provider only offers POP3.
    pub imap: Option<Server>,
    /// The POP3 server, when the provider offers one. Always set when
    /// `imap` is `None`.
    pub pop3: Option<Server>,
    /// `None` when no SMTP server was found; the account can still read.
    pub smtp: Option<Server>,
    pub source: Source,
    /// The servers belong to a provider Katna signs in to with OAuth2.
    pub oauth: Option<OAuthProvider>,
    /// A password (an app password, often) also works. `false` when the
    /// provider only takes OAuth2.
    pub password: bool,
}

impl Discovered {
    fn new(servers: Servers, source: Source) -> Self {
        let Servers { imap, pop3, smtp } = servers;
        Self {
            oauth: imap
                .as_ref()
                .and_then(|imap| Provider::for_imap_host(&imap.host)),
            imap,
            pop3,
            smtp,
            source,
            password: true,
        }
    }

    /// The server mail is read from: IMAP when there is one, else POP3.
    pub fn incoming(&self) -> &Server {
        self.imap
            .as_ref()
            .or(self.pop3.as_ref())
            .expect("discovery finds IMAP or POP3")
    }

    /// The provider's own servers, for signing in with OAuth2 only.
    fn oauth_only(provider: OAuthProvider, address: &str, source: Source) -> Self {
        let (imap, smtp) = Provider::servers(provider, address);
        Self {
            imap: Some(imap),
            pop3: None,
            smtp: Some(smtp),
            source,
            oauth: Some(provider),
            password: false,
        }
    }
}

/// Which of an account's servers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Imap,
    Pop3,
    Smtp,
}

/// The servers one step found: IMAP or POP3 or both, and SMTP if any.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Servers {
    imap: Option<Server>,
    pop3: Option<Server>,
    smtp: Option<Server>,
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
                Protocol::Pop3 => line.starts_with(b"+OK"),
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
    let mut oauth_only = None;
    for (index, body) in bodies.into_iter().enumerate() {
        let Some(body) = body else { continue };
        let source = if index < 2 {
            Source::Provider
        } else {
            Source::Ispdb
        };
        match parse_config(&body, &user) {
            Ok(Some(servers)) => return Ok(Discovered::new(servers, source)),
            Ok(None) => {
                oauth_only.get_or_insert((oauth_provider(&body, &user), source));
            }
            Err(err) => tracing::debug!(%err, "unusable autoconfig file"),
        }
    }
    // The provider says how to sign in; guessing would only find servers
    // that refuse the password.
    match oauth_only {
        Some((Some(provider), source)) => {
            return Ok(Discovered::oauth_only(provider, address, source));
        }
        Some((None, _)) => return Err(oauth_error(&domain)),
        None => {}
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
                Ok(Some(servers)) => return Ok(Discovered::new(servers, Source::Mx)),
                Ok(None) => {
                    return match oauth_provider(&body, &user) {
                        Some(provider) => Ok(Discovered::oauth_only(provider, address, Source::Mx)),
                        None => Err(oauth_error(&domain)),
                    };
                }
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
    format!("{domain} only allows OAuth2 sign-in, which Katna cannot do for this provider yet")
}

/// The provider whose OAuth2 sign-in the IMAP servers of a configuration
/// file ask for, if Katna knows it.
fn oauth_provider(xml: &[u8], user: &User<'_>) -> Option<OAuthProvider> {
    let text = std::str::from_utf8(xml).ok()?;
    let doc = roxmltree::Document::parse(text).ok()?;
    doc.descendants()
        .filter(|node| {
            node.has_tag_name("incomingServer") && node.attribute("type") == Some("imap")
        })
        .filter_map(|node| {
            let host = node
                .children()
                .find(|c| c.has_tag_name("hostname"))?
                .text()?;
            Provider::for_imap_host(&user.expand(host.trim()))
        })
        .next()
}

/// Microsoft's own domains, whose servers only take OAuth2.
const MICROSOFT_DOMAINS: &[&str] = &["outlook.com", "hotmail.com", "live.com", "msn.com"];

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
    if MICROSOFT_DOMAINS.contains(&user.domain) {
        return Some(Discovered::oauth_only(
            OAuthProvider::Microsoft,
            user.address,
            Source::BuiltIn,
        ));
    }
    let (_, imap, smtp, smtp_security, local_login) = BUILT_IN
        .iter()
        .find(|(domains, ..)| domains.contains(&user.domain))?;
    let username = if *local_login {
        user.local
    } else {
        user.address
    };
    let servers = Servers {
        imap: Some(server(imap, 993, Security::Tls, username)),
        pop3: None,
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
    };
    Some(Discovered::new(servers, Source::BuiltIn))
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

/// Reads a Thunderbird `config-v1.1.xml`. Returns the best IMAP, POP3 and
/// SMTP servers that take a password, `None` if the IMAP and POP3 servers
/// only take OAuth2, and an error if there is neither at all.
fn parse_config(xml: &[u8], user: &User<'_>) -> std::result::Result<Option<Servers>, String> {
    let text = std::str::from_utf8(xml).map_err(|_| "not UTF-8".to_owned())?;
    let doc = roxmltree::Document::parse(text).map_err(|err| err.to_string())?;
    let provider = doc
        .descendants()
        .find(|node| node.has_tag_name("emailProvider"))
        .ok_or("no emailProvider")?;
    let mut any_incoming = false;
    // IMAP, SMTP, POP3.
    let mut best: [Option<Server>; 3] = [None, None, None];
    // Whether a cleartext server was offered, for each slot.
    let mut plain = [false, false, false];
    for node in provider.children().filter(|n| n.is_element()) {
        let slot = match (node.tag_name().name(), node.attribute("type")) {
            ("incomingServer", Some("imap")) => 0,
            ("outgoingServer", Some("smtp")) => 1,
            ("incomingServer", Some("pop3")) => 2,
            _ => continue,
        };
        any_incoming |= slot != 1;
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
        plain[slot] |= security == Security::Plain;
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
    // A cleartext server is only ever taken when the file offers nothing
    // else (TLS and STARTTLS always win above); the password then crosses
    // the network readable by anyone on the way.
    for (server, offered) in best.iter().zip(plain) {
        if let Some(server) = server.as_ref().filter(|s| s.security == Security::Plain) {
            tracing::warn!(
                host = server.host,
                "the provider only offers a cleartext connection; the password will not be encrypted"
            );
        } else if offered {
            tracing::debug!("cleartext server skipped for an encrypted one");
        }
    }
    let [imap, smtp, pop3] = best;
    match (imap, pop3) {
        (None, None) if any_incoming => Ok(None),
        (None, None) => Err("no IMAP or POP3 server".into()),
        (imap, pop3) => Ok(Some(Servers { imap, pop3, smtp })),
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
        format!("_pop3s._tcp.{domain}"),
        format!("_pop3._tcp.{domain}"),
    ];
    let found = join_all(names.into_iter().map(|name| net.srv(name)).collect()).await;
    let pick = |records: &[dns::Srv], security| {
        records
            .iter()
            .filter(|r| r.port != 0 && valid_host(&r.target))
            .find(|r| {
                let trusted = srv_target_trusted(&r.target, user);
                if !trusted {
                    tracing::warn!(
                        domain,
                        target = r.target,
                        "SRV record names a server outside the domain; ignored"
                    );
                }
                trusted
            })
            .map(|r| server(&r.target, r.port, security, user.address))
    };
    let imap = pick(&found[0], Security::Tls).or_else(|| pick(&found[1], Security::StartTls));
    let smtp = pick(&found[2], Security::Tls).or_else(|| pick(&found[3], Security::StartTls));
    let pop3 = pick(&found[4], Security::Tls).or_else(|| pick(&found[5], Security::StartTls));
    if imap.is_none() && pop3.is_none() {
        return None;
    }
    Some(Discovered::new(
        Servers { imap, pop3, smtp },
        Source::DnsSrv,
    ))
}

/// Whether an SRV record of `user`'s domain may name `target`. DNS
/// answers are not authenticated, so a record pointing outside the domain
/// could come from a network in the middle, and the login would then be
/// checked against the attacker's own certificate (RFC 6186 §6): only the
/// domain itself, hosts under it, and the big providers Katna knows are
/// taken.
fn srv_target_trusted(target: &str, user: &User<'_>) -> bool {
    let target = target.trim_end_matches('.').to_ascii_lowercase();
    let domain = user.domain;
    if target == domain
        || target
            .strip_suffix(domain)
            .is_some_and(|rest| rest.ends_with('.'))
    {
        return true;
    }
    let built_in = BUILT_IN
        .iter()
        .any(|(_, imap, smtp, ..)| target == *imap || target == *smtp);
    let oauth = [OAuthProvider::Google, OAuthProvider::Microsoft]
        .into_iter()
        .any(|provider| {
            let (imap, smtp) = Provider::servers(provider, user.address);
            target == imap.host || target == smtp.host
        });
    built_in || oauth
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
    let pop3 = candidates(
        &["pop", "pop3", "mail"],
        [(995, Security::Tls), (110, Security::StartTls)],
    );
    let probes = imap
        .iter()
        .map(|s| (s.clone(), Protocol::Imap))
        .chain(smtp.iter().map(|s| (s.clone(), Protocol::Smtp)))
        .chain(pop3.iter().map(|s| (s.clone(), Protocol::Pop3)))
        .map(|(s, protocol)| net.probe(s, protocol))
        .collect();
    let answered = join_all(probes).await;
    let (imap_ok, rest) = answered.split_at(imap.len());
    let (smtp_ok, pop3_ok) = rest.split_at(smtp.len());
    let first = |servers: &[Server], ok: &[bool]| {
        servers
            .iter()
            .zip(ok)
            .find(|(_, ok)| **ok)
            .map(|(s, _)| s.clone())
    };
    let servers = Servers {
        imap: first(&imap, imap_ok),
        pop3: first(&pop3, pop3_ok),
        smtp: first(&smtp, smtp_ok),
    };
    if servers.imap.is_none() && servers.pop3.is_none() {
        return None;
    }
    Some(Discovered::new(servers, Source::Guess))
}

/// The registrable part of a host name, roughly
/// ([`crate::pictures::organizational_domain`]).
fn base_domain(host: &str) -> String {
    crate::pictures::organizational_domain(host)
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
