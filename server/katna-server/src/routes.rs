// SPDX-License-Identifier: GPL-3.0-or-later

//! HTTP routes.
//!
//! Public (in tracked mail):
//! - `GET /o/<id>.png`: the open pixel.
//! - `GET /l/<id>/<n>`: link `n`, redirected to the target stored for it.
//!
//! Both always answer (the picture, the redirect), but record an event
//! only within limits per client network and per tracking ID, so a flood
//! of fetches neither fills the database nor floods the sender.
//!
//! For the daemon, with `Authorization: Bearer <install token>`:
//! - `POST /api/v1/installs` (no token): register, get an install token.
//! - `DELETE /api/v1/installs/me`: forget the install and all its data.
//! - `/api/v1/account/...`: Katna accounts ([`crate::accounts`]).
//!
//! The rest need the install to be signed in to a Katna account
//! ([`SignedIn`]):
//! - `POST /api/v1/tracks` `{"count": n, "links": [...]}`: `n` new IDs.
//! - `DELETE /api/v1/tracks/<id>`: forget one ID and its events.
//! - `GET /api/v1/events`: server-sent events after `Last-Event-ID` (or
//!   `?after=`).
//! - `GET /api/v1/languages`, `POST /api/v1/translate`, `POST
//!   /api/v1/detect`: LibreTranslate, passed through ([`crate::translate`]).
//! - `POST /api/v1/ai/rephrase`, `POST /api/v1/ai/complete`: Katna AI
//!   ([`crate::ai`]).

use std::collections::HashMap;
use std::convert::Infallible;
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::extract::{ConnectInfo, FromRequestParts, Path, Query, State};
use axum::http::header::{self, HeaderMap, HeaderValue};
use axum::http::request::Parts;
use axum::http::{Method, StatusCode};
use axum::response::sse::{KeepAlive, Sse};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tokio::sync::{Semaphore, broadcast};

use crate::accounts::{self, AccountLimits};
use crate::ai;
use crate::auth::SignedIn;
use crate::classify::{self, Kind, Source};
use crate::config::Config;
use crate::db::{Db, DbError, Event, OverLimit, TrackLimits, now_ms};
use crate::ids;
use crate::limits::WindowLimit;
use crate::mailer::{MailError, Mailer};
use crate::translate;

/// Most IDs one request may create (one per recipient).
pub const MAX_IDS_PER_REQUEST: u32 = 100;

/// Most links one message may have tracked.
pub const MAX_LINKS: usize = 500;

/// Longest link target accepted.
pub const MAX_LINK_LEN: usize = 4096;

/// Most bytes of link targets one request may carry.
pub const MAX_LINK_BYTES: usize = 256 * 1024;

/// Most bytes of link targets one account may store per 24 hours.
pub const DAILY_LINK_BYTES: u64 = 16 * 1024 * 1024;

/// Opens and clicks recorded per client network (an IPv4 address or an
/// IPv6 /64) per [`EVENT_WINDOW`]; more still get the picture or the
/// redirect but are not recorded.
pub const EVENTS_PER_IP: u32 = 300;

/// Opens and clicks recorded per tracking ID per [`EVENT_WINDOW`].
pub const EVENTS_PER_ID: u32 = 20;

/// The window of [`EVENTS_PER_IP`] and [`EVENTS_PER_ID`].
pub const EVENT_WINDOW: Duration = Duration::from_secs(3600);

/// Event streams one install may have open at once.
pub const STREAMS_PER_INSTALL: usize = 4;

/// Locks that keep each install's events in order (see
/// [`AppState::record`]).
const RECORD_LOCKS: usize = 64;

/// The lock that orders `install`'s events.
fn record_lock(install: &str) -> usize {
    use std::hash::{BuildHasher, BuildHasherDefault, DefaultHasher};
    // The same install always gets the same lock.
    BuildHasherDefault::<DefaultHasher>::default().hash_one(install) as usize % RECORD_LOCKS
}

/// Shared state of the routes.
#[derive(Clone)]
pub struct AppState {
    db: Db,
    config: Arc<Config>,
    events: broadcast::Sender<Arc<Event>>,
    /// Events are numbered and sent under a lock, so they reach the
    /// stream in the order of their numbers and a resuming stream misses
    /// none. Each install's stream only carries its own events, so the
    /// locks are shared out by install: opens for different installs are
    /// recorded side by side.
    record: Arc<[tokio::sync::Mutex<()>; RECORD_LOCKS]>,
    registrations: Arc<WindowLimit<Option<IpAddr>>>,
    /// Translation requests per install and day.
    translations: Arc<WindowLimit<String>>,
    /// Translation requests passed on at once.
    translating: Arc<Semaphore>,
    /// Events recorded per client network.
    event_ips: Arc<WindowLimit<Option<IpAddr>>>,
    /// Events recorded per tracking ID.
    event_ids: Arc<WindowLimit<String>>,
    /// Open event streams per install.
    streams: Arc<Mutex<HashMap<String, usize>>>,
    /// Installs and accounts whose devices were signed out, deleted or had
    /// the password changed: their open event streams check at once
    /// whether they may go on.
    signed_out: broadcast::Sender<Arc<str>>,
    mailer: Mailer,
    account_limits: Arc<AccountLimits>,
    /// Katna AI requests per account and hour.
    ai_requests: Arc<WindowLimit<String>>,
    /// Katna AI requests passed on at once.
    ai_calls: Arc<Semaphore>,
}

impl AppState {
    /// State for `config` over `db`, with the mailer the settings ask for;
    /// an error when they ask for none.
    pub fn new(db: Db, config: Config) -> Result<Self, MailError> {
        let mailer = Mailer::from_config(&config)?;
        Ok(Self::with_mailer(db, config, mailer))
    }

    /// State for `config` over `db`, sending account mail with `mailer`.
    pub fn with_mailer(db: Db, config: Config, mailer: Mailer) -> Self {
        let (events, _) = broadcast::channel(1024);
        let (signed_out, _) = broadcast::channel(256);
        let registrations = WindowLimit::new(config.installs_per_hour, Duration::from_secs(3600));
        let translations =
            WindowLimit::new(config.translations_per_day, Duration::from_secs(86_400));
        let translating = Semaphore::new(config.translate_concurrency.max(1));
        let ai_requests = WindowLimit::new(config.ai.per_hour, Duration::from_secs(3600));
        Self {
            db,
            config: Arc::new(config),
            events,
            record: Arc::new(std::array::from_fn(|_| tokio::sync::Mutex::new(()))),
            registrations: Arc::new(registrations),
            translations: Arc::new(translations),
            translating: Arc::new(translating),
            event_ips: Arc::new(WindowLimit::new(EVENTS_PER_IP, EVENT_WINDOW)),
            event_ids: Arc::new(WindowLimit::new(EVENTS_PER_ID, EVENT_WINDOW)),
            streams: Arc::default(),
            signed_out,
            mailer,
            account_limits: Arc::default(),
            ai_requests: Arc::new(ai_requests),
            ai_calls: Arc::new(Semaphore::new(ai::CONCURRENCY)),
        }
    }

    /// Tells open event streams of `install_or_account` to check whether
    /// they may go on.
    pub(crate) fn signed_out(&self, install_or_account: &str) {
        let _ = self.signed_out.send(Arc::from(install_or_account));
    }

    /// Translation requests passed on at once.
    pub(crate) fn translating(&self) -> &Arc<Semaphore> {
        &self.translating
    }

    /// Sends account mail.
    pub fn mailer(&self) -> &Mailer {
        &self.mailer
    }

    pub(crate) fn account_limits(&self) -> &AccountLimits {
        &self.account_limits
    }

    /// The database.
    pub fn db(&self) -> &Db {
        &self.db
    }

    /// The settings.
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Translation requests per install and day.
    pub(crate) fn translations(&self) -> &WindowLimit<String> {
        &self.translations
    }

    /// Katna AI requests per account and hour.
    pub(crate) fn ai_requests(&self) -> &WindowLimit<String> {
        &self.ai_requests
    }

    /// Katna AI requests passed on at once.
    pub(crate) fn ai_calls(&self) -> &Arc<Semaphore> {
        &self.ai_calls
    }
}

/// The server's routes.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(about))
        .route("/healthz", get(health))
        .route("/o/{file}", get(open_pixel))
        .route("/l/{id}/{n}", get(follow_link))
        .route("/api/v1/installs", post(register))
        .route("/api/v1/installs/me", delete(unregister))
        .route("/api/v1/tracks", post(create_tracks))
        .route("/api/v1/tracks/{id}", delete(delete_track))
        .route("/api/v1/events", get(events))
        .route("/api/v1/languages", get(translate::languages))
        .route("/api/v1/translate", post(translate::translate))
        .route("/api/v1/detect", post(translate::detect))
        .merge(accounts::routes())
        .merge(ai::routes())
        .with_state(state)
}

/// An error answered to the client.
#[derive(Debug)]
pub enum ApiError {
    /// Missing or unknown install token.
    Unauthorized,
    /// The install is not signed in to a Katna account.
    SignInNeeded,
    /// The account's address is not confirmed yet.
    NotVerified,
    /// Wrong address or password.
    WrongPassword,
    /// Already there (an account for the address).
    Conflict(&'static str),
    /// Mail with a code could not be sent.
    MailFailed,
    /// A password could not be hashed.
    Hash,
    /// A malformed request.
    BadRequest(&'static str),
    /// A request a person can correct: a code for programs and a message.
    Invalid(&'static str, &'static str),
    /// Nothing there.
    NotFound,
    /// Over a limit.
    TooMany(&'static str),
    /// Too busy just now; try again shortly.
    Busy(&'static str),
    /// Katna AI's free month is over and no time is paid for.
    PaymentNeeded,
    /// A service the server passes requests to failed.
    Upstream(&'static str),
    /// The database failed.
    Internal(DbError),
}

impl From<DbError> for ApiError {
    fn from(error: DbError) -> Self {
        ApiError::Internal(error)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        // `code` is for programs; `error` for people reading logs.
        let (status, code, message) = match self {
            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "unknown_install",
                "unknown install token",
            ),
            ApiError::SignInNeeded => (
                StatusCode::FORBIDDEN,
                "sign_in",
                "sign in to a Katna account",
            ),
            ApiError::NotVerified => (
                StatusCode::FORBIDDEN,
                "not_verified",
                "confirm the Katna account's address first",
            ),
            ApiError::WrongPassword => (
                StatusCode::UNAUTHORIZED,
                "wrong_password",
                "wrong address or password",
            ),
            ApiError::Conflict(message) => (StatusCode::CONFLICT, "exists", message),
            ApiError::BadRequest(message) => (StatusCode::BAD_REQUEST, "bad_request", message),
            ApiError::Invalid(code, message) => (StatusCode::BAD_REQUEST, code, message),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "not_found", "not found"),
            ApiError::TooMany(message) => (StatusCode::TOO_MANY_REQUESTS, "too_many", message),
            ApiError::Busy(message) => (StatusCode::SERVICE_UNAVAILABLE, "busy", message),
            ApiError::PaymentNeeded => (
                StatusCode::PAYMENT_REQUIRED,
                "pay",
                "the free month of Katna AI is over",
            ),
            ApiError::Upstream(message) => (StatusCode::BAD_GATEWAY, "upstream", message),
            ApiError::MailFailed => (
                StatusCode::BAD_GATEWAY,
                "mail_failed",
                "the mail with the code could not be sent",
            ),
            ApiError::Hash => (StatusCode::INTERNAL_SERVER_ERROR, "server", "server error"),
            ApiError::Internal(error) => {
                tracing::error!(%error, "request failed");
                (StatusCode::INTERNAL_SERVER_ERROR, "server", "server error")
            }
        };
        (
            status,
            Json(serde_json::json!({ "error": message, "code": code })),
        )
            .into_response()
    }
}

/// The install a request's bearer token belongs to, signed in or not.
/// Only the install and account routes take this; a server feature takes
/// [`SignedIn`] instead.
pub struct Install(pub String);

impl FromRequestParts<AppState> for Install {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, ApiError> {
        Ok(Install(crate::auth::install_auth(parts, state).await?.id))
    }
}

/// The connection's address, when the server was started with it (tests
/// call the router without one).
pub struct ClientAddr(pub Option<SocketAddr>);

impl<S: Send + Sync> FromRequestParts<S> for ClientAddr {
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Infallible> {
        Ok(ClientAddr(
            parts
                .extensions
                .get::<ConnectInfo<SocketAddr>>()
                .map(|ConnectInfo(address)| *address),
        ))
    }
}

/// The client's address: the connection's, or the last `X-Forwarded-For`
/// entry (the one our own reverse proxy added) when that is trusted.
pub(crate) fn client_ip(
    state: &AppState,
    headers: &HeaderMap,
    connection: Option<SocketAddr>,
) -> Option<IpAddr> {
    if state.config.trust_forwarded
        && let Some(forwarded) = headers
            .get_all("x-forwarded-for")
            .iter()
            .filter_map(|value| value.to_str().ok())
            .flat_map(|value| value.split(','))
            .next_back()
    {
        return forwarded.trim().parse().ok();
    }
    connection.map(|address| address.ip())
}

/// The key an address is rate limited under: IPv6 addresses by their /64
/// network, which one home or server gets whole, so a new address from it
/// is not a fresh allowance.
pub(crate) fn limit_key(ip: Option<IpAddr>) -> Option<IpAddr> {
    ip.map(|ip| match ip {
        IpAddr::V6(v6) => match v6.to_ipv4_mapped() {
            Some(v4) => IpAddr::V4(v4),
            None => {
                let network = u128::from(v6) & !((1u128 << 64) - 1);
                IpAddr::V6(network.into())
            }
        },
        v4 => v4,
    })
}

const ABOUT: &str = r#"<!doctype html>
<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width">
<title>Katna tracking server</title>
<style>body{font:16px/1.5 system-ui,sans-serif;max-width:40em;margin:3em auto;padding:0 1em}</style>
</head><body>
<h1>Katna tracking server</h1>
<p>Someone sent you mail with <a href="https://katna.invenia.in">Katna</a> and chose to see
whether it was opened and whether its links were followed. Your mail program fetched a small
picture or followed a link through this server, and the sender was told when.</p>
<p>This server stores only a random number for each copy of the mail, the addresses of its
links, and the time of each open or click, labelled as a person, Apple's privacy proxy or a
security scanner. It never receives the subject, the recipients or the content of the mail,
and it does not keep IP addresses or browser details. Records are deleted after a few months.</p>
<p>To stop it, turn off remote pictures in your mail program.</p>
</body></html>"#;

async fn about() -> Html<&'static str> {
    Html(ABOUT)
}

async fn health(State(state): State<AppState>) -> Result<&'static str, ApiError> {
    state.db.ping().await?;
    Ok("ok")
}

/// A transparent 1×1 PNG.
pub const PIXEL: &[u8] = &[
    0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1f, 0x15, 0xc4,
    0x89, 0x00, 0x00, 0x00, 0x0b, 0x49, 0x44, 0x41, 0x54, 0x78, 0xda, 0x63, 0x60, 0x00, 0x02, 0x00,
    0x00, 0x05, 0x00, 0x01, 0xe9, 0xfa, 0xdc, 0xd8, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e, 0x44,
    0xae, 0x42, 0x60, 0x82,
];

fn no_cache(headers: &mut HeaderMap) {
    headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-store, no-cache, must-revalidate, private, max-age=0"),
    );
    headers.insert(header::PRAGMA, HeaderValue::from_static("no-cache"));
    headers.insert(header::EXPIRES, HeaderValue::from_static("0"));
    headers.insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("no-referrer"),
    );
}

fn pixel_response() -> Response {
    let mut response = (
        [(header::CONTENT_TYPE, HeaderValue::from_static("image/png"))],
        PIXEL,
    )
        .into_response();
    no_cache(response.headers_mut());
    response
}

/// Records one event and sends it to the install's stream. Failures are
/// logged, never shown: the recipient's picture or link must work anyway.
async fn record(
    state: &AppState,
    id: &str,
    install: &str,
    kind: Kind,
    link: Option<i32>,
    source: Source,
) {
    let _order = state.record[record_lock(install)].lock().await;
    match state
        .db
        .insert_event(install, id, kind, link, source, now_ms())
        .await
    {
        Ok(event) => {
            let _ = state.events.send(Arc::new(event));
        }
        Err(error) => tracing::warn!(%error, "could not record an event"),
    }
}

struct Seen<'a> {
    method: &'a Method,
    headers: &'a HeaderMap,
    connection: Option<SocketAddr>,
}

impl Seen<'_> {
    fn source(&self, state: &AppState, kind: Kind, created_at: i64) -> Source {
        let user_agent = self
            .headers
            .get(header::USER_AGENT)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("");
        let since_sent = Duration::from_millis(now_ms().saturating_sub(created_at).max(0) as u64);
        classify::classify(
            kind,
            classify::Request {
                ip: client_ip(state, self.headers, self.connection),
                user_agent,
                head: self.method == Method::HEAD,
                since_sent,
            },
        )
    }
}

/// Whether an event from this client may be recorded (counts it).
fn client_may_record(
    state: &AppState,
    headers: &HeaderMap,
    connection: Option<SocketAddr>,
) -> bool {
    state
        .event_ips
        .allow(limit_key(client_ip(state, headers, connection)))
}

async fn open_pixel(
    State(state): State<AppState>,
    Path(file): Path<String>,
    method: Method,
    headers: HeaderMap,
    ClientAddr(connection): ClientAddr,
) -> Response {
    let Some(id) = file.strip_suffix(".png").filter(|id| ids::is_valid_id(id)) else {
        return ApiError::NotFound.into_response();
    };
    // Past its limit a client gets the picture without it being looked up.
    if !client_may_record(&state, &headers, connection) {
        return pixel_response();
    }
    // Unknown and expired IDs still get the picture, so nothing breaks in
    // the recipient's mail program.
    match state.db.track(id, None).await {
        Ok(Some(track)) if state.event_ids.allow(id.to_owned()) => {
            let seen = Seen {
                method: &method,
                headers: &headers,
                connection,
            };
            let source = seen.source(&state, Kind::Open, track.created_at);
            record(&state, id, &track.install, Kind::Open, None, source).await;
        }
        Ok(_) => {}
        Err(error) => tracing::warn!(%error, "could not look up a tracking ID"),
    }
    pixel_response()
}

async fn follow_link(
    State(state): State<AppState>,
    Path((id, n)): Path<(String, u32)>,
    method: Method,
    headers: HeaderMap,
    ClientAddr(connection): ClientAddr,
) -> Result<Response, ApiError> {
    if !ids::is_valid_id(&id) {
        return Err(ApiError::NotFound);
    }
    let track = state
        .db
        .track(&id, Some(n))
        .await?
        .ok_or(ApiError::NotFound)?;
    // Only a target stored for this ID by a signed-in account, and not to
    // a host blocked since.
    let target = track
        .link
        .as_deref()
        .filter(|target| !is_blocked(&state.config, target))
        .and_then(|target| HeaderValue::from_str(target).ok())
        .ok_or(ApiError::NotFound)?;
    if client_may_record(&state, &headers, connection) && state.event_ids.allow(id.clone()) {
        let seen = Seen {
            method: &method,
            headers: &headers,
            connection,
        };
        let source = seen.source(&state, Kind::Click, track.created_at);
        record(
            &state,
            &id,
            &track.install,
            Kind::Click,
            Some(n as i32),
            source,
        )
        .await;
    }
    let mut response = (StatusCode::FOUND, [(header::LOCATION, target)]).into_response();
    no_cache(response.headers_mut());
    Ok(response)
}

#[derive(Serialize)]
struct Registered {
    install: String,
    token: String,
}

async fn register(
    State(state): State<AppState>,
    headers: HeaderMap,
    ClientAddr(connection): ClientAddr,
) -> Result<(StatusCode, Json<Registered>), ApiError> {
    let ip = limit_key(client_ip(&state, &headers, connection));
    if !state.registrations.allow(ip) {
        return Err(ApiError::TooMany("too many new installs from this address"));
    }
    let install = ids::new_id();
    let token = ids::new_token();
    state
        .db
        .create_install(&install, &ids::token_hash(&token), now_ms())
        .await?;
    Ok((StatusCode::CREATED, Json(Registered { install, token })))
}

async fn unregister(
    State(state): State<AppState>,
    Install(install): Install,
) -> Result<StatusCode, ApiError> {
    state.db.delete_install(&install).await?;
    state.signed_out(&install);
    Ok(StatusCode::NO_CONTENT)
}

/// Body of `POST /api/v1/tracks`.
#[derive(Deserialize)]
pub struct NewTracks {
    /// How many IDs: one per recipient.
    pub count: u32,
    /// Link targets, numbered from 0 in this order.
    #[serde(default)]
    pub links: Vec<String>,
}

#[derive(Serialize)]
struct Created {
    ids: Vec<String>,
}

/// Returns whether `target` may be redirected to: an absolute `http` or
/// `https` address without spaces or control characters.
pub fn is_allowed_target(target: &str) -> bool {
    let lower = target.get(..8).unwrap_or(target).to_ascii_lowercase();
    (lower.starts_with("https://") || lower.starts_with("http://"))
        && target.len() <= MAX_LINK_LEN
        && target.len() > "http://".len() + 1
        && !target.chars().any(|c| c.is_whitespace() || c.is_control())
}

/// The host of an `http` or `https` address as a browser reads it:
/// lowercased, without user, port or trailing dot, `%xx` decoded.
pub fn host_of(target: &str) -> Option<String> {
    let rest = target.split_once("://")?.1;
    // Browsers end the authority at a backslash as well as at `/`, `?` and
    // `#`.
    let authority = rest.split(['/', '\\', '?', '#']).next()?;
    let host_port = authority.rsplit('@').next()?;
    let host = if let Some(bracketed) = host_port.strip_prefix('[') {
        bracketed.split(']').next()?
    } else {
        host_port.split(':').next()?
    };
    let host = percent_decode(host)?.to_lowercase();
    let host = host.trim_end_matches('.');
    (!host.is_empty()).then(|| host.to_owned())
}

/// `%xx` decoded; `None` when that does not give UTF-8.
fn percent_decode(text: &str) -> Option<String> {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && let Some(hex) = text.get(i + 1..i + 3)
            && hex.bytes().all(|b| b.is_ascii_hexdigit())
            && let Ok(byte) = u8::from_str_radix(hex, 16)
        {
            out.push(byte);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).ok()
}

/// Whether `target`'s host is one of `KATNA_SERVER_BLOCKED_HOSTS` or under
/// one. An address whose host cannot be read counts as blocked when any
/// host is.
pub fn is_blocked(config: &Config, target: &str) -> bool {
    if config.blocked_hosts.is_empty() {
        return false;
    }
    let Some(host) = host_of(target) else {
        return true;
    };
    config.blocked_hosts.iter().any(|blocked| {
        host == *blocked
            || host
                .strip_suffix(blocked.as_str())
                .is_some_and(|sub| sub.ends_with('.'))
    })
}

async fn create_tracks(
    State(state): State<AppState>,
    SignedIn { install, account }: SignedIn,
    Json(request): Json<NewTracks>,
) -> Result<(StatusCode, Json<Created>), ApiError> {
    if request.count == 0 || request.count > MAX_IDS_PER_REQUEST {
        return Err(ApiError::BadRequest("count must be 1 to 100"));
    }
    if request.links.len() > MAX_LINKS
        || request.links.iter().map(String::len).sum::<usize>() > MAX_LINK_BYTES
    {
        return Err(ApiError::BadRequest("too many links"));
    }
    if !request.links.iter().all(|target| is_allowed_target(target)) {
        return Err(ApiError::BadRequest(
            "links must be http or https addresses",
        ));
    }
    if request
        .links
        .iter()
        .any(|target| is_blocked(&state.config, target))
    {
        return Err(ApiError::BadRequest(
            "a link goes to a host this server does not redirect to",
        ));
    }
    let ids: Vec<String> = (0..request.count).map(|_| ids::new_id()).collect();
    let limits = TrackLimits {
        ids: state.config.daily_limit,
        link_bytes: DAILY_LINK_BYTES,
    };
    let created = state
        .db
        .create_tracks(&account, &install, &ids, &request.links, now_ms(), limits)
        .await?;
    match created {
        Ok(()) => {}
        Err(OverLimit::Ids) => {
            return Err(ApiError::TooMany("daily limit of tracked copies reached"));
        }
        Err(OverLimit::LinkBytes) => {
            return Err(ApiError::TooMany("daily limit of tracked links reached"));
        }
    }
    Ok((StatusCode::CREATED, Json(Created { ids })))
}

async fn delete_track(
    State(state): State<AppState>,
    SignedIn { install, .. }: SignedIn,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    if ids::is_valid_id(&id) && state.db.delete_track(&install, &id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::NotFound)
    }
}

/// Query of `GET /api/v1/events`.
#[derive(Deserialize)]
pub struct After {
    /// Resume after this event number (`Last-Event-ID` wins).
    pub after: Option<i64>,
}

/// One of an install's open event streams; gives its place back when the
/// stream ends.
pub(crate) struct StreamSlot {
    streams: Arc<Mutex<HashMap<String, usize>>>,
    install: String,
}

impl StreamSlot {
    /// A place for one more of `install`'s streams, if it has fewer than
    /// [`STREAMS_PER_INSTALL`] open.
    fn take(state: &AppState, install: &str) -> Option<Self> {
        let mut streams = state.streams.lock().unwrap_or_else(|e| e.into_inner());
        let open = streams.entry(install.to_owned()).or_insert(0);
        if *open >= STREAMS_PER_INSTALL {
            return None;
        }
        *open += 1;
        Some(Self {
            streams: Arc::clone(&state.streams),
            install: install.to_owned(),
        })
    }
}

impl Drop for StreamSlot {
    fn drop(&mut self) {
        let mut streams = self.streams.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(open) = streams.get_mut(&self.install) {
            *open = open.saturating_sub(1);
            if *open == 0 {
                streams.remove(&self.install);
            }
        }
    }
}

async fn events(
    State(state): State<AppState>,
    SignedIn { install, account }: SignedIn,
    Query(query): Query<After>,
    headers: HeaderMap,
) -> Result<
    Sse<impl futures_lite::Stream<Item = Result<axum::response::sse::Event, Infallible>>>,
    ApiError,
> {
    let slot = StreamSlot::take(&state, &install)
        .ok_or(ApiError::TooMany("too many event streams open"))?;
    let after = headers
        .get("last-event-id")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.trim().parse().ok())
        .or(query.after)
        .unwrap_or(0);
    // Subscribe before reading what is stored, so nothing falls between.
    let live = state.events.subscribe();
    let signed_out = state.signed_out.subscribe();
    Ok(Sse::new(crate::stream::events(
        crate::stream::Stream {
            db: state.db.clone(),
            install,
            account,
            _slot: slot,
        },
        after,
        live,
        signed_out,
    ))
    .keep_alive(KeepAlive::new().interval(Duration::from_secs(30))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_hosts_as_browsers_do() {
        for (target, host) in [
            ("https://Example.COM/x", "example.com"),
            ("http://example.com:8080?q", "example.com"),
            ("https://good.example@evil.example/", "evil.example"),
            ("https://evil.example\\@good.example/", "evil.example"),
            ("https://ev%69l.example./", "evil.example"),
            ("https://[2001:db8::1]:443/", "2001:db8::1"),
            ("https://evil.example#@good.example", "evil.example"),
        ] {
            assert_eq!(host_of(target).as_deref(), Some(host), "{target}");
        }
        assert_eq!(host_of("https:///path"), None);
    }

    #[test]
    fn blocks_hosts_and_their_subdomains() {
        let config = Config {
            blocked_hosts: vec!["evil.example".into()],
            ..Config::default()
        };
        assert!(is_blocked(&config, "https://evil.example/login"));
        assert!(is_blocked(&config, "https://www.EVIL.example/login"));
        assert!(is_blocked(&config, "https://x@evil.example:443/"));
        assert!(!is_blocked(&config, "https://notevil.example/"));
        assert!(!is_blocked(&config, "https://evil.example.org/"));
        assert!(!is_blocked(&Config::default(), "https://evil.example/"));
    }

    #[test]
    fn ipv6_is_limited_by_network() {
        let key = |ip: &str| super::limit_key(Some(ip.parse().unwrap()));
        assert_eq!(key("2001:db8:1:2:aaaa::1"), key("2001:db8:1:2:bbbb::9"));
        assert_ne!(key("2001:db8:1:2::1"), key("2001:db8:1:3::1"));
        assert_eq!(key("::ffff:192.0.2.7"), key("192.0.2.7"));
        assert_ne!(key("192.0.2.7"), key("192.0.2.8"));
    }
}
