// SPDX-License-Identifier: GPL-3.0-or-later

//! HTTP routes.
//!
//! Public (in tracked mail):
//! - `GET /o/<id>.png`: the open pixel.
//! - `GET /l/<id>/<n>`: link `n`, redirected to the target stored for it.
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

use std::convert::Infallible;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
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
use tokio::sync::broadcast;

use crate::accounts::{self, AccountLimits};
use crate::auth::SignedIn;
use crate::classify::{self, Kind, Source};
use crate::config::Config;
use crate::db::{Db, DbError, Event, now_ms};
use crate::ids;
use crate::limits::WindowLimit;
use crate::mailer::Mailer;

/// Most IDs one request may create (one per recipient).
pub const MAX_IDS_PER_REQUEST: u32 = 100;

/// Most links one message may have tracked.
pub const MAX_LINKS: usize = 500;

/// Longest link target accepted.
pub const MAX_LINK_LEN: usize = 4096;

/// Shared state of the routes.
#[derive(Clone)]
pub struct AppState {
    db: Db,
    config: Arc<Config>,
    events: broadcast::Sender<Arc<Event>>,
    /// Events are numbered and sent under this lock, so they reach the
    /// stream in the order of their numbers and a resuming stream misses
    /// none. One server process; the load is small.
    record: Arc<tokio::sync::Mutex<()>>,
    registrations: Arc<WindowLimit<Option<IpAddr>>>,
    mailer: Mailer,
    account_limits: Arc<AccountLimits>,
}

impl AppState {
    /// State for `config` over `db`, with the mailer the settings ask for.
    pub fn new(db: Db, config: Config) -> Self {
        // `main` checks the mail settings before this, so the fallback is
        // for tests.
        let mailer = Mailer::from_config(&config).unwrap_or_else(|error| {
            tracing::error!(%error, "mail settings unusable; codes go to the log");
            Mailer::Log
        });
        Self::with_mailer(db, config, mailer)
    }

    /// State for `config` over `db`, sending account mail with `mailer`.
    pub fn with_mailer(db: Db, config: Config, mailer: Mailer) -> Self {
        let (events, _) = broadcast::channel(1024);
        let registrations = WindowLimit::new(config.installs_per_hour, Duration::from_secs(3600));
        Self {
            db,
            config: Arc::new(config),
            events,
            record: Arc::default(),
            registrations: Arc::new(registrations),
            mailer,
            account_limits: Arc::default(),
        }
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
        .merge(accounts::routes())
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
    let _order = state.record.lock().await;
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
    // Unknown and expired IDs still get the picture, so nothing breaks in
    // the recipient's mail program.
    match state.db.track(id).await {
        Ok(Some(track)) => {
            let seen = Seen {
                method: &method,
                headers: &headers,
                connection,
            };
            let source = seen.source(&state, Kind::Open, track.created_at);
            record(&state, id, &track.install, Kind::Open, None, source).await;
        }
        Ok(None) => {}
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
    let track = state.db.track(&id).await?.ok_or(ApiError::NotFound)?;
    // Only a target stored for this ID, so the server is never an open
    // redirect.
    let target = track
        .links
        .get(n as usize)
        .and_then(|target| HeaderValue::from_str(target).ok())
        .ok_or(ApiError::NotFound)?;
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
    let ip = client_ip(&state, &headers, connection);
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

async fn create_tracks(
    State(state): State<AppState>,
    SignedIn { install, .. }: SignedIn,
    Json(request): Json<NewTracks>,
) -> Result<(StatusCode, Json<Created>), ApiError> {
    if request.count == 0 || request.count > MAX_IDS_PER_REQUEST {
        return Err(ApiError::BadRequest("count must be 1 to 100"));
    }
    if request.links.len() > MAX_LINKS {
        return Err(ApiError::BadRequest("too many links"));
    }
    if !request.links.iter().all(|target| is_allowed_target(target)) {
        return Err(ApiError::BadRequest(
            "links must be http or https addresses",
        ));
    }
    let now = now_ms();
    let today = state.db.tracks_since(&install, now - 86_400_000).await?;
    if today + i64::from(request.count) > i64::from(state.config.daily_limit) {
        return Err(ApiError::TooMany("daily limit of tracked copies reached"));
    }
    let ids: Vec<String> = (0..request.count).map(|_| ids::new_id()).collect();
    state
        .db
        .create_tracks(&install, &ids, &request.links, now)
        .await?;
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

async fn events(
    State(state): State<AppState>,
    SignedIn { install, .. }: SignedIn,
    Query(query): Query<After>,
    headers: HeaderMap,
) -> Sse<impl futures_lite::Stream<Item = Result<axum::response::sse::Event, Infallible>>> {
    let after = headers
        .get("last-event-id")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.trim().parse().ok())
        .or(query.after)
        .unwrap_or(0);
    // Subscribe before reading what is stored, so nothing falls between.
    let live = state.events.subscribe();
    Sse::new(crate::stream::events(
        state.db.clone(),
        install,
        after,
        live,
    ))
    .keep_alive(KeepAlive::new().interval(Duration::from_secs(30)))
}
