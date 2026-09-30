// SPDX-License-Identifier: GPL-3.0-or-later

//! OAuth2 sign-in for Google, Microsoft and Zoho accounts
//! (`docs/ARCHITECTURE.md` §6.4): the installed-app flow with PKCE
//! (RFC 7636) and a loopback redirect (RFC 8252). The browser shows the
//! provider's own sign-in page; its answer comes back to a one-shot HTTP
//! listener on `127.0.0.1`, and the code is exchanged over our own HTTPS
//! client. The refresh token goes to the Secret Service; [`TokenSource`]
//! turns it into short-lived access tokens for SASL XOAUTH2.

use std::{
    collections::HashMap,
    fmt,
    sync::Mutex,
    time::{Duration, Instant},
};

use async_net::{TcpListener, TcpStream};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use futures_lite::{AsyncReadExt, AsyncWriteExt, FutureExt};
use katna_core::{LinkedSignIn, OAuthProvider, Security, Server};
use serde::Deserialize;

use crate::{
    Error, Result,
    autoconfig::http::{self, form_encode},
    net::Tls,
};

/// How long the token endpoint may take to answer.
const TOKEN_TIMEOUT: Duration = Duration::from_secs(30);

/// An access token is renewed this long before it runs out.
const EXPIRY_MARGIN: Duration = Duration::from_secs(120);

/// Google Drive, limited to the files Katna itself put there: for
/// attachments too large to send by mail.
pub const GOOGLE_DRIVE_FILE: &str = "https://www.googleapis.com/auth/drive.file";

/// OneDrive, through Microsoft Graph: for attachments too large to send
/// by mail. Graph is another resource than Outlook's IMAP and SMTP, so
/// its tokens come separately ([`TokenSource::access_token_for`]).
pub const MICROSOFT_FILES: &str = "https://graph.microsoft.com/Files.ReadWrite";

/// Google Calendar: the calendars and their events
/// ([`crate::calendar::google`]).
pub const GOOGLE_CALENDAR: &str = "https://www.googleapis.com/auth/calendar";

/// Outlook's calendars, through Microsoft Graph
/// ([`crate::calendar::graph`]); like [`MICROSOFT_FILES`], its tokens come
/// separately.
pub const MICROSOFT_CALENDARS: &str = "https://graph.microsoft.com/Calendars.ReadWrite";

/// Google Contacts, read and written through the People API.
pub const GOOGLE_CONTACTS: &str = "https://www.googleapis.com/auth/contacts";

/// Outlook contacts, through Microsoft Graph, like [`MICROSOFT_FILES`].
pub const MICROSOFT_CONTACTS: &str = "https://graph.microsoft.com/Contacts.ReadWrite";

/// Google's CardDAV server, the way to contacts when the People API is
/// not available ([`crate::carddav`]).
pub const GOOGLE_CARDDAV: &str = "https://www.googleapis.com/auth/carddav";

/// Google's "Other contacts": people the user mailed but never saved,
/// read-only.
pub const GOOGLE_OTHER_CONTACTS: &str = "https://www.googleapis.com/auth/contacts.other.readonly";

/// Google Tasks: the account's task lists, synced with Katna Tasks.
pub const GOOGLE_TASKS: &str = "https://www.googleapis.com/auth/tasks";

/// Google Meet, limited to the meeting spaces Katna itself made: for
/// "Start a video call" ([`crate::meet`]).
pub const GOOGLE_MEET: &str = "https://www.googleapis.com/auth/meetings.space.created";

/// Microsoft To Do, through Microsoft Graph: the account's task lists.
/// Like [`MICROSOFT_FILES`], its tokens come separately.
pub const MICROSOFT_TASKS: &str = "https://graph.microsoft.com/Tasks.ReadWrite";

/// Zoho Mail's tasks (Zoho ToDo's), through the Zoho Mail API.
pub const ZOHO_TASKS: &str = "ZohoMail.tasks.ALL";

/// The Zoho Mail accounts of who signed in: the `accountId` and `zuid`
/// some Zoho Mail task calls take.
pub const ZOHO_MAIL_ACCOUNTS: &str = "ZohoMail.accounts.READ";

/// Zoho Calendar: the calendars and their events.
pub const ZOHO_CALENDAR: &str = "ZohoCalendar.calendar.ALL,ZohoCalendar.event.ALL";

/// Who signed in to Zoho: its address and name, from
/// `/oauth/user/info` (Zoho gives no ID token).
pub const ZOHO_PROFILE: &str = "AaaServer.profile.READ";

/// The loopback port Zoho sends the browser back to. Zoho takes only the
/// redirect URIs registered in its API Console, port and all, so it
/// cannot be any free port as with Google and Microsoft.
pub const ZOHO_REDIRECT_PORT: u16 = 53710;

/// Zoho's data centres: where `location` (as Zoho's sign-in answer names
/// it) keeps its accounts, and the mail domains that live there. An
/// account's tokens come only from its own data centre.
pub const ZOHO_DATA_CENTRES: [(&str, &str, &[&str]); 8] = [
    (
        "us",
        "https://accounts.zoho.com",
        &["zoho.com", "zohomail.com"],
    ),
    (
        "eu",
        "https://accounts.zoho.eu",
        &["zoho.eu", "zohomail.eu"],
    ),
    (
        "in",
        "https://accounts.zoho.in",
        &["zoho.in", "zohomail.in"],
    ),
    (
        "au",
        "https://accounts.zoho.com.au",
        &["zoho.com.au", "zohomail.com.au"],
    ),
    (
        "jp",
        "https://accounts.zoho.jp",
        &["zoho.jp", "zohomail.jp"],
    ),
    (
        "ca",
        "https://accounts.zohocloud.ca",
        &["zohocloud.ca", "zohomail.ca"],
    ),
    (
        "sa",
        "https://accounts.zoho.sa",
        &["zoho.sa", "zohomail.sa"],
    ),
    (
        "uk",
        "https://accounts.zoho.uk",
        &["zoho.uk", "zohomail.uk"],
    ),
];

/// The Zoho data centre of a mail domain or server host
/// (`imap.zoho.in`, `ada@zohomail.eu`), if it is one of Zoho's.
pub fn zoho_accounts_server(host_or_address: &str) -> Option<&'static str> {
    let host = host_or_address
        .rsplit('@')
        .next()
        .unwrap_or_default()
        .trim_end_matches('.')
        .to_ascii_lowercase();
    // Longest domain first, so `zoho.com.au` is not taken for `zoho.com`.
    let mut best: Option<(&str, usize)> = None;
    for (_, server, domains) in ZOHO_DATA_CENTRES {
        for domain in domains {
            let under = host == *domain
                || host
                    .strip_suffix(domain)
                    .is_some_and(|rest| rest.ends_with('.'));
            if under && best.is_none_or(|(_, len)| domain.len() > len) {
                best = Some((server, domain.len()));
            }
        }
    }
    best.map(|(server, _)| server)
}

/// Whether a mail server host (`imappro.zoho.in`) or address is Zoho's.
pub fn is_zoho_host(host_or_address: &str) -> bool {
    zoho_accounts_server(host_or_address).is_some()
}

/// The Zoho Mail API of a linked Zoho sign-in's data centre:
/// `https://mail.zoho.in/api` for `https://accounts.zoho.in`.
/// `KATNA_ZOHO_API_URL` stands in for it under test.
pub fn zoho_mail_api(linked: &LinkedSignIn) -> String {
    if let Some(url) = http::test_url("KATNA_ZOHO_API_URL") {
        return url;
    }
    let server = linked.accounts_server.trim_end_matches('/');
    let server = if is_zoho_accounts_server(server) {
        server
    } else {
        "https://accounts.zoho.com"
    };
    format!("{}/api", server.replacen("://accounts.", "://mail.", 1))
}

/// Whether `server` is one of Zoho's sign-in servers: the code and the
/// client secret go only there, whatever the browser was sent back with.
fn is_zoho_accounts_server(server: &str) -> bool {
    let server = server.trim_end_matches('/');
    ZOHO_DATA_CENTRES
        .iter()
        .any(|(_, known, _)| *known == server)
}

/// Largest request the loopback listener reads.
const MAX_REQUEST: usize = 16 * 1024;

/// Where and how to sign in to one provider.
#[derive(Clone)]
pub struct Provider {
    pub kind: OAuthProvider,
    pub auth_url: String,
    pub token_url: String,
    pub client_id: String,
    /// Only Google's desktop apps have one, and it is not secret.
    pub client_secret: String,
    /// Space-separated scopes.
    pub scope: String,
    /// More scopes the user allows at sign-in, of another resource whose
    /// tokens come separately (Microsoft Graph); empty for Google.
    pub consent: String,
    /// Host name of the loopback redirect, as registered with the
    /// provider; `host:port` when the provider takes only one port.
    pub redirect_host: &'static str,
    pub tls: Tls,
}

impl fmt::Debug for Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Provider")
            .field("kind", &self.kind)
            .field("token_url", &self.token_url)
            .finish_non_exhaustive()
    }
}

impl Provider {
    /// The provider with the client ID from `katna_core::ids`; `None` when
    /// this build has none.
    pub fn new(kind: OAuthProvider, tls: Tls) -> Option<Self> {
        // A token server under test stands in for the provider's, also in
        // builds without a client ID.
        let test_tokens = match kind {
            OAuthProvider::Google => http::test_url("KATNA_GOOGLE_TOKEN_URL"),
            OAuthProvider::Microsoft => http::test_url("KATNA_MICROSOFT_TOKEN_URL"),
            OAuthProvider::Zoho => http::test_url("KATNA_ZOHO_TOKEN_URL"),
        };
        if !kind.available() && test_tokens.is_none() {
            return None;
        }
        Some(match kind {
            OAuthProvider::Google => Self {
                kind,
                auth_url: "https://accounts.google.com/o/oauth2/v2/auth".into(),
                token_url: test_tokens
                    .unwrap_or_else(|| "https://oauth2.googleapis.com/token".into()),
                client_id: if kind.available() {
                    kind.client_id().into()
                } else {
                    "katna-test".into()
                },
                client_secret: katna_core::ids::GOOGLE_OAUTH_CLIENT_SECRET.into(),
                // Full IMAP and SMTP, the files Katna puts in Drive for
                // large attachments, the calendars, the contacts (People
                // API, other contacts, CardDAV), the task lists, the
                // meetings Katna makes, and who signed in (address, name,
                // picture) in the ID token.
                scope: format!(
                    "https://mail.google.com/ {GOOGLE_DRIVE_FILE} {GOOGLE_CALENDAR} \
                     {GOOGLE_CONTACTS} {GOOGLE_OTHER_CONTACTS} {GOOGLE_CARDDAV} \
                     {GOOGLE_TASKS} {GOOGLE_MEET} openid email profile"
                ),
                consent: String::new(),
                redirect_host: "127.0.0.1",
                tls,
            },
            OAuthProvider::Microsoft => Self {
                kind,
                auth_url: "https://login.microsoftonline.com/common/oauth2/v2.0/authorize".into(),
                token_url: test_tokens.unwrap_or_else(|| {
                    "https://login.microsoftonline.com/common/oauth2/v2.0/token".into()
                }),
                client_id: if kind.available() {
                    kind.client_id().into()
                } else {
                    "katna-test".into()
                },
                client_secret: String::new(),
                scope: "https://outlook.office.com/IMAP.AccessAsUser.All \
                        https://outlook.office.com/SMTP.Send offline_access openid email profile"
                    .into(),
                // OneDrive, for large attachments, the calendars, the contacts
                // and To Do, allowed at the same sign-in.
                consent: format!(
                    "{MICROSOFT_FILES} {MICROSOFT_CALENDARS} {MICROSOFT_CONTACTS} {MICROSOFT_TASKS}"
                ),
                // Entra registers loopback redirects as `http://localhost`.
                redirect_host: "localhost",
                tls,
            },
            OAuthProvider::Zoho => Self {
                kind,
                // Zoho sends the user on to their own data centre and
                // names it in its answer ([`SignIn::finish`]).
                auth_url: "https://accounts.zoho.com/oauth/v2/auth".into(),
                token_url: test_tokens
                    .unwrap_or_else(|| "https://accounts.zoho.com/oauth/v2/token".into()),
                client_id: if kind.available() {
                    kind.client_id().into()
                } else {
                    "katna-test".into()
                },
                client_secret: katna_core::ids::ZOHO_OAUTH_CLIENT_SECRET.into(),
                // Zoho's scopes are separated by commas. Calendar comes at
                // the same sign-in as the tasks, so it needs no second one.
                scope: format!("{ZOHO_TASKS},{ZOHO_MAIL_ACCOUNTS},{ZOHO_CALENDAR},{ZOHO_PROFILE}"),
                consent: String::new(),
                // The one port registered with Zoho: [`ZOHO_REDIRECT_PORT`].
                redirect_host: "localhost:53710",
                tls,
            },
        })
    }

    /// The same provider at the sign-in server of another of its data
    /// centres (Zoho). A token server under test stays.
    pub fn at_accounts_server(mut self, server: &str) -> Self {
        let server = server.trim_end_matches('/');
        self.auth_url = format!("{server}/oauth/v2/auth");
        if !self.token_url.starts_with("http://") {
            self.token_url = format!("{server}/oauth/v2/token");
        }
        self
    }

    /// The sign-in server the tokens come from: the token URL without its
    /// path.
    pub fn accounts_server(&self) -> &str {
        self.token_url
            .strip_suffix("/oauth/v2/token")
            .unwrap_or(&self.token_url)
    }

    /// The IMAP and SMTP servers of the provider's accounts.
    pub fn servers(kind: OAuthProvider, username: &str) -> (Server, Server) {
        let server = |host: &str, port, security| Server {
            host: host.to_owned(),
            port,
            security,
            username: username.to_owned(),
            accept_invalid_certs: false,
        };
        match kind {
            OAuthProvider::Google => (
                server("imap.gmail.com", 993, Security::Tls),
                server("smtp.gmail.com", 465, Security::Tls),
            ),
            OAuthProvider::Microsoft => (
                server("outlook.office365.com", 993, Security::Tls),
                server("smtp.office365.com", 587, Security::StartTls),
            ),
            // Only a linked sign-in: Zoho Mail keeps its own servers and
            // password. These are its US data centre's.
            OAuthProvider::Zoho => (
                server("imap.zoho.com", 993, Security::Tls),
                server("smtp.zoho.com", 465, Security::Tls),
            ),
        }
    }

    /// The provider whose OAuth2 servers `imap_host` belongs to.
    pub fn for_imap_host(imap_host: &str) -> Option<OAuthProvider> {
        let host = imap_host.trim_end_matches('.').to_ascii_lowercase();
        let under = |domain: &str| {
            host == domain || host.strip_suffix(domain).is_some_and(|h| h.ends_with('.'))
        };
        if under("gmail.com") || under("googlemail.com") {
            Some(OAuthProvider::Google)
        } else if under("office365.com") || under("outlook.com") || under("hotmail.com") {
            Some(OAuthProvider::Microsoft)
        } else {
            None
        }
    }
}

/// Who signed in, from the ID token.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Identity {
    pub email: String,
    pub name: String,
    /// A URL of the account's picture (Google only), or empty.
    pub picture: String,
}

/// What the token endpoint handed out.
#[derive(Clone)]
pub struct Grant {
    pub access_token: String,
    /// Absent when refreshing and the provider keeps the old one.
    pub refresh_token: Option<String>,
    pub expires_in: Duration,
    /// Absent when refreshing.
    pub identity: Option<Identity>,
    /// The scopes granted, when the provider says (Google does).
    pub scope: Option<String>,
    /// The sign-in server of the user's data centre, which refreshes the
    /// tokens (Zoho; `None` for the others).
    pub accounts_server: Option<String>,
    /// Where the provider's APIs answer for this user (Zoho).
    pub api_domain: Option<String>,
}

impl fmt::Debug for Grant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Grant")
            .field("expires_in", &self.expires_in)
            .field("identity", &self.identity)
            .finish_non_exhaustive()
    }
}

#[derive(Deserialize)]
struct TokenAnswer {
    access_token: String,
    #[serde(default)]
    refresh_token: Option<String>,
    #[serde(default)]
    expires_in: Option<u64>,
    #[serde(default)]
    id_token: Option<String>,
    #[serde(default)]
    scope: Option<String>,
    #[serde(default)]
    api_domain: Option<String>,
}

#[derive(Deserialize)]
struct ErrorAnswer {
    error: String,
    #[serde(default)]
    error_description: String,
}

#[derive(Deserialize, Default)]
struct Claims {
    #[serde(default)]
    email: String,
    #[serde(default)]
    preferred_username: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    picture: String,
}

/// Reads who signed in from an ID token. It came straight from the token
/// endpoint over TLS, so its signature needs no checking (OpenID Connect
/// Core §3.1.3.7).
fn identity(id_token: &str) -> Option<Identity> {
    let payload = id_token.split('.').nth(1)?;
    let json = URL_SAFE_NO_PAD.decode(payload.trim_end_matches('=')).ok()?;
    let claims: Claims = serde_json::from_slice(&json).ok()?;
    // Microsoft's personal accounts may only name the address as the
    // user name.
    let email = if claims.email.contains('@') {
        claims.email
    } else if claims.preferred_username.contains('@') {
        claims.preferred_username
    } else {
        return None;
    };
    Some(Identity {
        email,
        name: claims.name,
        picture: if claims.picture.starts_with("https://") {
            claims.picture
        } else {
            String::new()
        },
    })
}

/// Posts `form` to the token endpoint.
async fn token_request<'a>(
    provider: &'a Provider,
    form: &mut Vec<(&'a str, &'a str)>,
) -> Result<Grant> {
    form.push(("client_id", &provider.client_id));
    if !provider.client_secret.is_empty() {
        form.push(("client_secret", &provider.client_secret));
    }
    let (status, body) =
        http::post_form(&provider.token_url, form, &provider.tls, TOKEN_TIMEOUT).await?;
    // Zoho answers a refusal with 200 and an error, so only an answer
    // with a token is one.
    let answer = (status == 200)
        .then(|| serde_json::from_slice::<TokenAnswer>(&body).ok())
        .flatten();
    if let Some(answer) = answer {
        return Ok(Grant {
            access_token: answer.access_token,
            refresh_token: answer.refresh_token.filter(|t| !t.is_empty()),
            // The providers give an hour; assume less if they say nothing.
            expires_in: Duration::from_secs(answer.expires_in.unwrap_or(600)),
            identity: answer.id_token.as_deref().and_then(identity),
            scope: answer.scope,
            accounts_server: None,
            api_domain: answer.api_domain.filter(|d| d.starts_with("https://")),
        });
    }
    let name = provider.kind.name();
    match serde_json::from_slice::<ErrorAnswer>(&body) {
        // The grant was revoked, expired or never valid: only signing in
        // again helps (RFC 6749 §5.2).
        Ok(answer)
            if matches!(
                answer.error.as_str(),
                "invalid_grant"
                    | "invalid_client"
                    | "unauthorized_client"
                    | "invalid_scope"
                    // Zoho's word for a revoked or unknown refresh token.
                    | "invalid_code"
            ) =>
        {
            Err(Error::Auth(format!(
                "{name} asks to sign in again ({}{}{})",
                answer.error,
                if answer.error_description.is_empty() {
                    ""
                } else {
                    ": "
                },
                answer.error_description.lines().next().unwrap_or_default()
            )))
        }
        Ok(answer) => Err(Error::Protocol(format!(
            "{name} token endpoint: {status} {}",
            answer.error
        ))),
        Err(_) if status == 200 => Err(Error::Protocol(format!(
            "{name} token endpoint: no token in its answer"
        ))),
        Err(_) => Err(Error::Protocol(format!(
            "{name} token endpoint answered {status}"
        ))),
    }
}

/// Trades a refresh token for a new access token.
pub async fn refresh(provider: &Provider, refresh_token: &str) -> Result<Grant> {
    let mut form = vec![
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token),
    ];
    // Google refuses scopes the grant lacks, as accounts signed in before
    // Drive was asked for do; without any it grants what the sign-in did.
    // Zoho takes none.
    if provider.kind == OAuthProvider::Microsoft {
        form.push(("scope", &provider.scope));
    }
    token_request(provider, &mut form).await
}

/// Trades a refresh token for an access token of another resource, one
/// of [`Provider::consent`]'s. A grant that never allowed it is
/// [`Error::Auth`].
async fn refresh_for(provider: &Provider, refresh_token: &str, scope: &str) -> Result<Grant> {
    let scope = format!("{scope} offline_access");
    token_request(
        provider,
        &mut vec![
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
            ("scope", &scope),
        ],
    )
    .await
}

/// `n` random bytes, base64url.
fn random(n: usize) -> Result<String> {
    use ring::rand::SecureRandom;
    let mut bytes = vec![0; n];
    ring::rand::SystemRandom::new()
        .fill(&mut bytes)
        .map_err(|_| Error::Protocol("no random numbers from the system".into()))?;
    Ok(URL_SAFE_NO_PAD.encode(bytes))
}

/// The S256 challenge of a PKCE verifier.
fn challenge(verifier: &str) -> String {
    URL_SAFE_NO_PAD.encode(ring::digest::digest(
        &ring::digest::SHA256,
        verifier.as_bytes(),
    ))
}

/// The two short pages the browser shows once the provider sends it back.
pub struct Pages {
    pub signed_in: String,
    pub failed: String,
}

/// A sign-in waiting for the browser: open [`SignIn::url`], then
/// [`SignIn::finish`].
pub struct SignIn {
    listeners: Vec<TcpListener>,
    redirect_uri: String,
    state: String,
    verifier: String,
    url: String,
}

impl SignIn {
    /// Starts listening on a free loopback port and builds the URL of the
    /// provider's sign-in page. `login_hint` (an address, or empty) fills
    /// in the account there.
    pub async fn start(provider: &Provider, login_hint: &str) -> Result<Self> {
        // A provider that takes one registered port names it.
        let (host, fixed) = match provider.redirect_host.rsplit_once(':') {
            Some((host, port)) => (host, port.parse().unwrap_or(0)),
            None => (provider.redirect_host, 0),
        };
        let v4 = TcpListener::bind(("127.0.0.1", fixed))
            .await
            .map_err(|err| {
                if fixed == 0 {
                    Error::from(err)
                } else {
                    Error::Protocol(format!(
                        "port {fixed}, where {} sends the browser back, is taken: {err}",
                        provider.kind.name()
                    ))
                }
            })?;
        let port = v4.local_addr()?.port();
        let mut listeners = vec![v4];
        // Browsers may try `localhost` on IPv6 first.
        if host == "localhost"
            && let Ok(v6) = TcpListener::bind(("::1", port)).await
        {
            listeners.push(v6);
        }
        let redirect_uri = format!("http://{host}:{port}/");
        let state = random(16)?;
        let verifier = random(48)?;
        let scope = if provider.consent.is_empty() {
            provider.scope.clone()
        } else {
            format!("{} {}", provider.scope, provider.consent)
        };
        let mut query = vec![
            ("response_type", "code"),
            ("client_id", provider.client_id.as_str()),
            ("redirect_uri", redirect_uri.as_str()),
            ("scope", scope.as_str()),
            ("state", state.as_str()),
            ("code_challenge_method", "S256"),
        ];
        let challenge = challenge(&verifier);
        query.push(("code_challenge", &challenge));
        match provider.kind {
            // A refresh token every time, also when the user signed in
            // before.
            OAuthProvider::Google | OAuthProvider::Zoho => {
                query.push(("access_type", "offline"));
                query.push(("prompt", "consent"));
            }
            OAuthProvider::Microsoft => query.push(("prompt", "select_account")),
        }
        if !login_hint.trim().is_empty() {
            query.push(("login_hint", login_hint.trim()));
        }
        let url = format!("{}?{}", provider.auth_url, form_encode(&query));
        Ok(Self {
            listeners,
            redirect_uri,
            state,
            verifier,
            url,
        })
    }

    /// The provider's sign-in page, for the browser.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Waits for the browser to come back, shows it one of `pages` and
    /// trades the code for tokens. Waits as long as it takes; drop the
    /// future to give up.
    pub async fn finish(self, provider: &Provider, pages: &Pages) -> Result<Grant> {
        let (code, server) = loop {
            let mut stream = accept(&self.listeners).await?;
            let Some(query) = read_request(&mut stream).await else {
                respond(&mut stream, "404 Not Found", "").await;
                continue;
            };
            let param = |name: &str| {
                query
                    .split('&')
                    .filter_map(|pair| pair.split_once('='))
                    .find(|(key, _)| *key == name)
                    .map(|(_, value)| unescape(value))
            };
            // Anything else knocking on the port is not the provider's
            // answer to this sign-in.
            if param("state").as_deref() != Some(self.state.as_str()) {
                respond(&mut stream, "400 Bad Request", "").await;
                continue;
            }
            if let Some(code) = param("code").filter(|c| !c.is_empty()) {
                // Zoho names the data centre that keeps the account; the
                // code is good only there.
                let server = param("accounts-server");
                if let Some(server) = &server
                    && !is_zoho_accounts_server(server)
                {
                    respond(&mut stream, "200 OK", &pages.failed).await;
                    return Err(Error::Protocol(format!(
                        "{} sent the browser back from an unknown server {server}",
                        provider.kind.name()
                    )));
                }
                respond(&mut stream, "200 OK", &pages.signed_in).await;
                break (code, server);
            }
            respond(&mut stream, "200 OK", &pages.failed).await;
            let error = param("error").unwrap_or_else(|| "no code".into());
            return Err(if error == "access_denied" {
                Error::Auth(format!(
                    "{} sign-in was cancelled or access was not allowed",
                    provider.kind.name()
                ))
            } else {
                Error::Protocol(format!(
                    "{} sign-in failed: {error} {}",
                    provider.kind.name(),
                    param("error_description").unwrap_or_default()
                ))
            });
        };
        let mut form = vec![
            ("grant_type", "authorization_code"),
            ("code", code.as_str()),
            ("redirect_uri", self.redirect_uri.as_str()),
            ("code_verifier", self.verifier.as_str()),
        ];
        // Microsoft hands out tokens for one resource at a time: these are
        // for IMAP and SMTP, whatever else the user allowed.
        if !provider.consent.is_empty() {
            form.push(("scope", provider.scope.as_str()));
        }
        let at_server = match (&server, provider.kind) {
            (Some(server), OAuthProvider::Zoho) => {
                Some(provider.clone().at_accounts_server(server))
            }
            _ => None,
        };
        let provider = at_server.as_ref().unwrap_or(provider);
        let mut grant = token_request(provider, &mut form).await?;
        if provider.kind == OAuthProvider::Zoho {
            grant.accounts_server = Some(match &server {
                Some(server) => server.trim_end_matches('/').to_owned(),
                None => provider.accounts_server().to_owned(),
            });
            if grant.identity.is_none() {
                grant.identity = zoho_identity(provider, &grant.access_token).await;
            }
        }
        if grant.refresh_token.is_none() {
            return Err(Error::Protocol(format!(
                "{} gave no refresh token",
                provider.kind.name()
            )));
        }
        if grant.identity.is_none() {
            return Err(Error::Protocol(format!(
                "{} did not say which account signed in",
                provider.kind.name()
            )));
        }
        Ok(grant)
    }
}

#[derive(Deserialize)]
struct ZohoUser {
    #[serde(default, rename = "Email")]
    email: String,
    #[serde(default, rename = "Display_Name")]
    display_name: String,
}

/// Who signed in to Zoho, which gives no ID token: its user info.
async fn zoho_identity(provider: &Provider, access_token: &str) -> Option<Identity> {
    let url = format!("{}/oauth/user/info", provider.accounts_server());
    let auth = format!("Zoho-oauthtoken {access_token}");
    let (status, body) = http::request(
        "GET",
        &url,
        &[("Authorization", &auth)],
        None,
        &provider.tls,
        TOKEN_TIMEOUT,
    )
    .await
    .inspect_err(|err| tracing::info!(%err, "no Zoho user info"))
    .ok()?;
    let user: ZohoUser = (status == 200)
        .then(|| serde_json::from_slice(&body).ok())
        .flatten()?;
    user.email.contains('@').then(|| Identity {
        email: user.email,
        name: user.display_name,
        picture: String::new(),
    })
}

async fn accept(listeners: &[TcpListener]) -> Result<TcpStream> {
    let first = listeners[0].accept();
    let (stream, _) = match listeners.get(1) {
        Some(second) => first.or(second.accept()).await?,
        None => first.await?,
    };
    Ok(stream)
}

/// Reads a `GET /?query` request and returns the query; `None` for any
/// other request.
async fn read_request(stream: &mut TcpStream) -> Option<String> {
    let mut request = Vec::new();
    let mut buf = [0; 2048];
    let read = async {
        while !request.windows(4).any(|w| w == b"\r\n\r\n") && request.len() < MAX_REQUEST {
            let n = stream.read(&mut buf).await.ok()?;
            if n == 0 {
                break;
            }
            request.extend_from_slice(&buf[..n]);
        }
        Some(())
    }
    .or(async {
        async_io::Timer::after(Duration::from_secs(10)).await;
        None
    });
    read.await?;
    let line = request.split(|b| *b == b'\r').next()?;
    let line = std::str::from_utf8(line).ok()?;
    let target = line.strip_prefix("GET ")?.split(' ').next()?;
    target.strip_prefix("/?").map(str::to_owned)
}

async fn respond(stream: &mut TcpStream, status: &str, text: &str) {
    let body = if text.is_empty() {
        String::new()
    } else {
        format!(
            "<!doctype html><meta charset=\"utf-8\"><title>Katna</title>\
             <body style=\"font:16px sans-serif;margin:4em auto;max-width:32em\">\
             <p>{}</p></body>",
            html_escape(text)
        )
    };
    let answer = format!(
        "HTTP/1.1 {status}\r\nContent-Type: text/html; charset=utf-8\r\n\
         Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    let _ = stream.write_all(answer.as_bytes()).await;
    let _ = stream.flush().await;
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Decodes `%XX` and `+` in a query value.
fn unescape(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => out.push(b' '),
            b'%' => match bytes
                .get(i + 1..i + 3)
                .and_then(|hex| std::str::from_utf8(hex).ok())
                .and_then(|hex| u8::from_str_radix(hex, 16).ok())
            {
                Some(byte) => {
                    out.push(byte);
                    i += 2;
                }
                None => out.push(b'%'),
            },
            byte => out.push(byte),
        }
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Called with a new refresh token when the provider replaces the old one
/// (Microsoft does on every refresh), to save it.
pub type OnRotate = Box<dyn Fn(String) + Send + Sync>;

/// Hands out access tokens for one account, refreshing them as needed.
/// Shared by the account's connections.
pub struct TokenSource {
    provider: Provider,
    refresh_token: Mutex<String>,
    access: Mutex<Option<(String, Instant)>>,
    /// Access tokens of other resources, by scope.
    others: Mutex<HashMap<String, (String, Instant)>>,
    /// The scopes granted, once the provider said.
    granted: Mutex<Option<String>>,
    /// Lets one refresh run at a time: holding its only message.
    gate: (async_channel::Sender<()>, async_channel::Receiver<()>),
    on_rotate: Option<OnRotate>,
}

/// The refresh permit of a [`TokenSource`], returned when dropped.
struct Permit<'a>(&'a async_channel::Sender<()>);

impl Drop for Permit<'_> {
    fn drop(&mut self) {
        let _ = self.0.try_send(());
    }
}

impl TokenSource {
    pub fn new(provider: Provider, refresh_token: String, on_rotate: Option<OnRotate>) -> Self {
        let gate = async_channel::bounded(1);
        let _ = gate.0.try_send(());
        Self {
            provider,
            refresh_token: Mutex::new(refresh_token),
            access: Mutex::new(None),
            others: Mutex::new(HashMap::new()),
            granted: Mutex::new(None),
            gate,
            on_rotate,
        }
    }

    /// Starts with an access token from a sign-in that just happened.
    pub fn with_access_token(self, token: String, expires_in: Duration) -> Self {
        *self.access.lock().unwrap() = Some((token, Instant::now() + expires_in));
        self
    }

    /// Starts with an access token for `scope`, another resource's.
    pub fn with_access_token_for(self, scope: &str, token: String, expires_in: Duration) -> Self {
        self.others
            .lock()
            .unwrap()
            .insert(scope.to_owned(), (token, Instant::now() + expires_in));
        self
    }

    /// Also knows the scopes that sign-in granted.
    pub fn with_scope(self, scope: Option<String>) -> Self {
        *self.granted.lock().unwrap() = scope;
        self
    }

    /// Whether the grant covers `scope`: `None` until the provider said.
    pub fn granted(&self, scope: &str) -> Option<bool> {
        self.granted
            .lock()
            .unwrap()
            .as_deref()
            .map(|granted| granted.split_whitespace().any(|s| s == scope))
    }

    /// Like [`Self::granted`], asking the provider when it has not said
    /// yet; `true` when it never does.
    pub async fn has_scope(&self, scope: &str) -> Result<bool> {
        if let Some(known) = self.granted(scope) {
            return Ok(known);
        }
        self.forget_access_token();
        self.access_token().await?;
        Ok(self.granted(scope).unwrap_or(true))
    }

    pub fn provider(&self) -> OAuthProvider {
        self.provider.kind
    }

    fn cached(&self) -> Option<String> {
        self.access
            .lock()
            .unwrap()
            .as_ref()
            .filter(|(_, until)| Instant::now() + EXPIRY_MARGIN < *until)
            .map(|(token, _)| token.clone())
    }

    /// A valid access token. A refused refresh token is [`Error::Auth`]:
    /// the user has to sign in again. Network trouble is not.
    pub async fn access_token(&self) -> Result<String> {
        if let Some(token) = self.cached() {
            return Ok(token);
        }
        let _ = self.gate.1.recv().await;
        // Hands the permit back however this ends, also when the future is
        // dropped mid-refresh (a stopped worker, a lost race): otherwise
        // every later call would wait forever.
        let _permit = Permit(&self.gate.0);
        async {
            // Another connection may have refreshed meanwhile.
            if let Some(token) = self.cached() {
                return Ok(token);
            }
            let refresh_token = self.refresh_token.lock().unwrap().clone();
            let grant = refresh(&self.provider, &refresh_token).await?;
            self.keep_refresh_token(&refresh_token, &grant);
            if grant.scope.is_some() {
                *self.granted.lock().unwrap() = grant.scope.clone();
            }
            *self.access.lock().unwrap() = Some((
                grant.access_token.clone(),
                Instant::now() + grant.expires_in,
            ));
            tracing::debug!(provider = %self.provider.kind, "access token refreshed");
            Ok(grant.access_token)
        }
        .await
    }

    /// Drops the access token after a server refused it. Returns whether a
    /// retry with a fresh one may help.
    pub fn forget_access_token(&self) -> bool {
        self.access.lock().unwrap().take().is_some()
    }

    /// Saves the refresh token `grant` replaced `old` with, if any.
    fn keep_refresh_token(&self, old: &str, grant: &Grant) {
        if let Some(new) = &grant.refresh_token
            && new != old
        {
            *self.refresh_token.lock().unwrap() = new.clone();
            if let Some(on_rotate) = &self.on_rotate {
                on_rotate(new.clone());
            }
        }
    }

    /// A valid access token for `scope`, one of another resource the
    /// sign-in allowed ([`Provider::consent`]). [`Error::Auth`] when it
    /// did not allow it; the account's mail goes on working.
    pub async fn access_token_for(&self, scope: &str) -> Result<String> {
        let cached = |this: &Self| {
            this.others
                .lock()
                .unwrap()
                .get(scope)
                .filter(|(_, until)| Instant::now() + EXPIRY_MARGIN < *until)
                .map(|(token, _)| token.clone())
        };
        if let Some(token) = cached(self) {
            return Ok(token);
        }
        let _ = self.gate.1.recv().await;
        let result = async {
            if let Some(token) = cached(self) {
                return Ok(token);
            }
            let refresh_token = self.refresh_token.lock().unwrap().clone();
            let grant = refresh_for(&self.provider, &refresh_token, scope).await?;
            self.keep_refresh_token(&refresh_token, &grant);
            self.others.lock().unwrap().insert(
                scope.to_owned(),
                (
                    grant.access_token.clone(),
                    Instant::now() + grant.expires_in,
                ),
            );
            Ok(grant.access_token)
        }
        .await;
        let _ = self.gate.0.try_send(());
        result
    }

    /// Starts with an access token for `scope`, for tests.
    #[cfg(test)]
    pub(crate) fn set_access_token_for_tests(&self, scope: &str, token: &str) {
        self.others.lock().unwrap().insert(
            scope.to_owned(),
            (token.to_owned(), Instant::now() + Duration::from_secs(3600)),
        );
    }

    /// Like [`Self::forget_access_token`], for [`Self::access_token_for`].
    pub fn forget_access_token_for(&self, scope: &str) -> bool {
        self.others.lock().unwrap().remove(scope).is_some()
    }
}

#[cfg(test)]
mod tests;
