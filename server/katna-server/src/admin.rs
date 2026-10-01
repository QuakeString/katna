// SPDX-License-Identifier: GPL-3.0-or-later

//! The admin page at `/admin`: Katna AI's service, model, limits and use
//! this month, for the Katna accounts listed in `KATNA_SERVER_ADMIN_EMAILS`
//! (without any, the page and its routes answer 404).
//!
//! Signing in takes the account's password and then a code mailed to it
//! (both within the account routes' limits); the session is a random token
//! in a `Secure`, `HttpOnly`, `SameSite=Strict` cookie, kept in memory for
//! 12 hours (a restart signs everyone out). Every call of the page also
//! carries an `X-Katna-Admin` header, which another site's page cannot add
//! without the server's leave.
//!
//! Keys stay in the environment: the page shows which services have one
//! and chooses among those, and never shows or takes a key. Its settings
//! are saved in the database over the environment's.
//!
//! - `GET /admin`, `/admin/app.js`, `/admin/app.css`: the page.
//! - `POST /admin/api/sign-in` `{email, password}`: mails a code (202).
//! - `POST /admin/api/code` `{email, code}`: sets the session cookie (204).
//! - `POST /admin/api/sign-out`.
//! - `GET /admin/api/state`: settings, services and use.
//! - `POST /admin/api/settings`: saves [`AiSettings`]; answers the state.
//! - `POST /admin/api/test`: asks each chosen service a short question.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use axum::extract::{FromRequestParts, State};
use axum::http::header::{self, HeaderMap, HeaderValue};
use axum::http::request::Parts;
use axum::http::{Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use katna_ai::{PRESETS, Tone, prompt};
use serde::{Deserialize, Serialize};

use crate::accounts::{check, limited, mail_code};
use crate::ai::{AiSettings, ask_one};
use crate::auth::{DUMMY_HASH, normalize_email, verify_password};
use crate::db::{AiStats, month_of, months_to, now_ms};
use crate::ids;
use crate::mailer::Purpose;
use crate::routes::{ApiError, AppState, ClientAddr};

/// The session cookie. `__Host-` makes browsers keep it only when set
/// over HTTPS for this host and the whole site.
const COOKIE: &str = "__Host-katna-admin";

/// How long a session lasts.
const SESSION: Duration = Duration::from_secs(12 * 3600);

/// Sessions kept at once; the oldest goes first.
const MAX_SESSIONS: usize = 32;

/// The header every call of the page carries.
const HEADER: &str = "x-katna-admin";

/// How long "Test" waits for each service.
const TEST_TIMEOUT: Duration = Duration::from_secs(20);

const PAGE: &str = include_str!("admin/index.html");
const SCRIPT: &str = include_str!("admin/app.js");
const STYLE: &str = include_str!("admin/app.css");

/// The page may load its own script and style and call its own routes,
/// nothing else.
const CSP: &str = "default-src 'none'; script-src 'self'; style-src 'self'; \
                   connect-src 'self'; img-src 'self'; base-uri 'none'; \
                   form-action 'none'; frame-ancestors 'none'";

/// Who is signed in, by the hash of their token.
#[derive(Default)]
pub struct Sessions {
    inner: Mutex<HashMap<Vec<u8>, Session>>,
}

struct Session {
    account: String,
    email: String,
    started: Instant,
}

impl Sessions {
    /// A new session for `account`; its token.
    fn start(&self, account: &str, email: &str) -> String {
        let token = ids::new_token();
        let mut sessions = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        sessions.retain(|_, session| session.started.elapsed() < SESSION);
        while sessions.len() >= MAX_SESSIONS {
            let oldest = sessions
                .iter()
                .min_by_key(|(_, session)| session.started)
                .map(|(hash, _)| hash.clone());
            match oldest {
                Some(hash) => sessions.remove(&hash),
                None => break,
            };
        }
        sessions.insert(
            ids::token_hash(&token),
            Session {
                account: account.to_owned(),
                email: email.to_owned(),
                started: Instant::now(),
            },
        );
        token
    }

    /// The account and address of a session's token, while it lasts.
    fn find(&self, token: &str) -> Option<(String, String)> {
        let sessions = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        sessions
            .get(&ids::token_hash(token))
            .filter(|session| session.started.elapsed() < SESSION)
            .map(|session| (session.account.clone(), session.email.clone()))
    }

    fn end(&self, token: &str) {
        self.inner
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .remove(&ids::token_hash(token));
    }
}

/// A request from someone signed in to the admin page.
pub struct Admin {
    pub account: String,
    pub email: String,
    token: String,
}

impl FromRequestParts<AppState> for Admin {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, ApiError> {
        page_on(state)?;
        if parts.method != Method::GET {
            from_page(&parts.headers)?;
        }
        let token = cookie(&parts.headers).ok_or(ApiError::AdminSignIn)?;
        let (account, email) = state
            .admin_sessions()
            .find(&token)
            .ok_or(ApiError::AdminSignIn)?;
        if !is_admin(state, &email) {
            return Err(ApiError::AdminSignIn);
        }
        Ok(Admin {
            account,
            email,
            token,
        })
    }
}

/// 404 unless admins are set.
fn page_on(state: &AppState) -> Result<(), ApiError> {
    if state.config().admin_emails.is_empty() {
        Err(ApiError::NotFound)
    } else {
        Ok(())
    }
}

/// The request came from the page itself.
fn from_page(headers: &HeaderMap) -> Result<(), ApiError> {
    match headers.get(HEADER) {
        Some(value) if value == "1" => Ok(()),
        _ => Err(ApiError::BadRequest("not from the admin page")),
    }
}

fn is_admin(state: &AppState, email: &str) -> bool {
    state
        .config()
        .admin_emails
        .iter()
        .any(|admin| admin == email)
}

/// The session token in the request's cookies.
fn cookie(headers: &HeaderMap) -> Option<String> {
    headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(|value| value.split(';'))
        .filter_map(|pair| pair.trim().split_once('='))
        .find(|(name, _)| *name == COOKIE)
        .map(|(_, token)| token.to_owned())
        .filter(|token| ids::is_valid_token(token))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/admin", get(page))
        .route("/admin/", get(page))
        .route("/admin/app.js", get(script))
        .route("/admin/app.css", get(style))
        .route("/admin/api/sign-in", post(sign_in))
        .route("/admin/api/code", post(code))
        .route("/admin/api/sign-out", post(sign_out))
        .route("/admin/api/state", get(show))
        .route("/admin/api/settings", post(save))
        .route("/admin/api/test", post(test))
}

/// `body` as `content_type`, not cached, under the page's rules.
fn asset(state: &AppState, content_type: &'static str, body: &'static str) -> Response {
    if page_on(state).is_err() {
        return ApiError::NotFound.into_response();
    }
    let mut response = body.into_response();
    let headers = response.headers_mut();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type));
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(CSP),
    );
    headers.insert(header::X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));
    headers.insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("no-referrer"),
    );
    response
}

async fn page(State(state): State<AppState>) -> Response {
    asset(&state, "text/html; charset=utf-8", PAGE)
}

async fn script(State(state): State<AppState>) -> Response {
    asset(&state, "text/javascript; charset=utf-8", SCRIPT)
}

async fn style(State(state): State<AppState>) -> Response {
    asset(&state, "text/css; charset=utf-8", STYLE)
}

#[derive(Deserialize)]
struct SignIn {
    email: String,
    password: String,
}

/// The password is right for an admin: mails the second step's code.
async fn sign_in(
    State(state): State<AppState>,
    headers: HeaderMap,
    addr: ClientAddr,
    Json(body): Json<SignIn>,
) -> Result<StatusCode, ApiError> {
    page_on(&state)?;
    from_page(&headers)?;
    limited(&state, &headers, addr)?;
    let email = normalize_email(&body.email).ok_or(ApiError::WrongPassword)?;
    if !state.account_limits().per_email.allow(email.clone()) {
        return Err(ApiError::TooMany("too many attempts; try again later"));
    }
    let account = state.db().account_by_email(&email).await?;
    let hash = account.as_ref().map_or_else(
        || DUMMY_HASH.clone(),
        |account| account.password_hash.clone(),
    );
    let right = verify_password(body.password, hash).await?;
    // Wrong password, not an admin, or not confirmed: the same answer.
    let account = account
        .filter(|account| right && account.verified_at.is_some() && is_admin(&state, &email))
        .ok_or(ApiError::WrongPassword)?;
    mail_code(&state, &account.id, &account.email, Purpose::Admin).await?;
    Ok(StatusCode::ACCEPTED)
}

#[derive(Deserialize)]
struct CodeBody {
    email: String,
    code: String,
}

/// The mailed code is right: signs in.
async fn code(
    State(state): State<AppState>,
    headers: HeaderMap,
    addr: ClientAddr,
    Json(body): Json<CodeBody>,
) -> Result<Response, ApiError> {
    page_on(&state)?;
    from_page(&headers)?;
    limited(&state, &headers, addr)?;
    let email = normalize_email(&body.email).ok_or(ApiError::WrongPassword)?;
    let account = state
        .db()
        .account_by_email(&email)
        .await?
        .filter(|_| is_admin(&state, &email))
        .ok_or(ApiError::Invalid("wrong_code", "wrong code"))?;
    check(&state, &account.id, Purpose::Admin, &body.code).await?;
    let token = state.admin_sessions().start(&account.id, &account.email);
    tracing::info!("signed in to the admin page");
    let cookie = format!(
        "{COOKIE}={token}; Path=/; Secure; HttpOnly; SameSite=Strict; Max-Age={}",
        SESSION.as_secs()
    );
    let mut response = StatusCode::NO_CONTENT.into_response();
    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie).map_err(|_| ApiError::Hash)?,
    );
    Ok(response)
}

async fn sign_out(State(state): State<AppState>, admin: Admin) -> Response {
    state.admin_sessions().end(&admin.token);
    let mut response = StatusCode::NO_CONTENT.into_response();
    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_static(
            "__Host-katna-admin=; Path=/; Secure; HttpOnly; SameSite=Strict; Max-Age=0",
        ),
    );
    response
}

/// A service on the page.
#[derive(Serialize)]
struct Service {
    id: &'static str,
    name: &'static str,
    /// Its usual model.
    model: &'static str,
    /// Whether its key is in the environment.
    has_key: bool,
}

/// What the page shows.
#[derive(Serialize)]
struct PageState {
    email: String,
    settings: AiSettings,
    services: Vec<Service>,
    /// This month (yyyymm).
    month: i32,
    stats: AiStats,
}

async fn page_state(state: &AppState, email: String) -> Result<PageState, ApiError> {
    let ai = state.ai();
    let env = &state.config().ai;
    let now = now_ms();
    let stats = state
        .db()
        .ai_stats(
            &months_to(now, 6),
            now,
            i64::from(ai.trial_days) * 86_400_000,
            ai.account_cap_micros as i64,
        )
        .await?;
    Ok(PageState {
        email,
        settings: AiSettings::of(&ai),
        services: PRESETS
            .iter()
            .map(|preset| Service {
                id: preset.id,
                // The app translates the last one's name; the page is in
                // English.
                name: if preset.name.is_empty() {
                    "Other (OpenAI-like)"
                } else {
                    preset.name
                },
                model: preset.model,
                has_key: env.keys.iter().any(|known| known.provider == preset.id),
            })
            .collect(),
        month: month_of(now),
        stats,
    })
}

async fn show(State(state): State<AppState>, admin: Admin) -> Result<Json<PageState>, ApiError> {
    Ok(Json(page_state(&state, admin.email).await?))
}

async fn save(
    State(state): State<AppState>,
    admin: Admin,
    Json(settings): Json<AiSettings>,
) -> Result<Json<PageState>, ApiError> {
    let ai = settings
        .apply(&state.config().ai)
        .map_err(|problem| ApiError::Invalid("bad_settings", problem))?;
    let json = serde_json::to_string(&settings).map_err(|_| ApiError::Hash)?;
    state
        .db()
        .set_ai_settings(&json, &admin.account, now_ms())
        .await?;
    state.set_ai(ai);
    tracing::info!(
        on = settings.on,
        "Katna AI settings changed on the admin page"
    );
    Ok(Json(page_state(&state, admin.email).await?))
}

/// How one service answered "Test".
#[derive(Serialize)]
struct Tested {
    provider: &'static str,
    model: String,
    /// How long it took, when it answered.
    ms: Option<u64>,
    /// What went wrong, when it did not.
    problem: Option<&'static str>,
}

async fn test(State(state): State<AppState>, _admin: Admin) -> Json<Vec<Tested>> {
    let question = prompt::rephrase(
        "Thanks for your mail, I will look at it and get back to you tomorrow.",
        Tone::Shorter,
        "",
    )
    .expect("the test text can be rephrased");
    let ai = state.ai();
    let mut results = Vec::new();
    for service in &ai.services {
        let started = Instant::now();
        let answer = ask_one(service, &question, TEST_TIMEOUT).await;
        results.push(Tested {
            provider: service.provider,
            model: service.model.clone(),
            ms: answer
                .as_ref()
                .ok()
                .map(|_| started.elapsed().as_millis() as u64),
            problem: answer.err(),
        });
    }
    Json(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_session_cookie() {
        let token = ids::new_token();
        let mut headers = HeaderMap::new();
        headers.insert(
            header::COOKIE,
            HeaderValue::from_str(&format!("other=1; {COOKIE}={token}; x=y")).unwrap(),
        );
        assert_eq!(cookie(&headers), Some(token));
        headers.insert(
            header::COOKIE,
            HeaderValue::from_static("__Host-katna-admin=not-a-token"),
        );
        assert_eq!(cookie(&headers), None);
    }

    #[test]
    fn sessions_end() {
        let sessions = Sessions::default();
        let token = sessions.start("a1", "a@b.io");
        assert_eq!(sessions.find(&token), Some(("a1".into(), "a@b.io".into())));
        sessions.end(&token);
        assert_eq!(sessions.find(&token), None);
        // Only the newest are kept.
        let first = sessions.start("a1", "a@b.io");
        for _ in 0..MAX_SESSIONS {
            sessions.start("a1", "a@b.io");
        }
        assert_eq!(sessions.find(&first), None);
    }
}
