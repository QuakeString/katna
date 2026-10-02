// SPDX-License-Identifier: GPL-3.0-or-later

//! The admin page at `/admin`: Katna AI's service, model, limits and use
//! this month, for the server's admins: the addresses in
//! `KATNA_SERVER_ADMIN_EMAILS` (without any, the page and its routes answer
//! 404). Admins are not Katna accounts: each one's password is set on the
//! server with `katna-server admin-password`, and Katna accounts can never
//! open the page.
//!
//! The first password can also be chosen on the page itself, after a code
//! mailed to the admin's address, so whoever finds the page first cannot
//! claim it. Signing in takes that password and then a code mailed to the address
//! (both within the account routes' limits); the session is a random token
//! in a `Secure`, `HttpOnly`, `SameSite=Strict` cookie, kept in memory for
//! 12 hours (a restart signs everyone out). Every call of the page also
//! carries an `X-Katna-Admin` header, which another site's page cannot add
//! without the server's leave.
//!
//! Keys come from the environment (`KATNA_SERVER_AI_<SERVICE>_KEY`) or
//! are saved on the page, in the database, which is then used over the
//! environment's. A key never comes back out: the page shows only where it
//! is from and its last four characters. Its settings are saved in the
//! database over the environment's too.
//!
//! - `GET /admin`, `/admin/app.js`, `/admin/app.css`: the page.
//! - `GET /admin/api/status`: `{setup}`, true while an admin has no
//!   password yet.
//! - `POST /admin/api/setup` `{email}`: for an admin without a password,
//!   mails a code to choose the first one (always 202).
//! - `POST /admin/api/setup/finish` `{email, code, password}`: sets that
//!   first password and signs in (204 + cookie).
//! - `POST /admin/api/sign-in` `{email, password}`: mails a code (202).
//! - `POST /admin/api/code` `{email, code}`: sets the session cookie (204).
//! - `POST /admin/api/sign-out`.
//! - `GET /admin/api/state`: settings, services and use.
//! - `POST /admin/api/settings`: saves [`AiSettings`]; answers the state.
//! - `POST /admin/api/test`: asks each chosen service a short question.
//! - `POST /admin/api/key` `{provider, key, base}`: saves a service's key
//!   (and the "other" service's address); answers the state.
//! - `POST /admin/api/key/remove` `{provider}`: forgets a saved key, so the
//!   environment's is used again, if any; answers the state.
//! - `POST /admin/api/models` `{provider}`: the models the service offers
//!   to its key, `{models}`, or `{problem}`.

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
use katna_ai::provider::{self, ProviderError};
use katna_ai::{PRESETS, Tone, prompt};
use serde::{Deserialize, Serialize};

use crate::accounts::limited;
use crate::ai::{AiSettings, ask_one, send};
use crate::auth::{
    self, CODE_LIFETIME_MS, DUMMY_HASH, clean_code, code_hash, normalize_email, verify_password,
};
use crate::db::{AiStats, CodeCheck, month_of, months_to, now_ms};
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

/// How long the list of a service's models may take.
const MODELS_TIMEOUT: Duration = Duration::from_secs(15);

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
    email: String,
    started: Instant,
}

impl Sessions {
    /// A new session for the admin `email`; its token.
    fn start(&self, email: &str) -> String {
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
                email: email.to_owned(),
                started: Instant::now(),
            },
        );
        token
    }

    /// The admin of a session's token, while it lasts.
    fn find(&self, token: &str) -> Option<String> {
        let sessions = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        sessions
            .get(&ids::token_hash(token))
            .filter(|session| session.started.elapsed() < SESSION)
            .map(|session| session.email.clone())
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
        let email = state
            .admin_sessions()
            .find(&token)
            .ok_or(ApiError::AdminSignIn)?;
        if !is_admin(state, &email) {
            return Err(ApiError::AdminSignIn);
        }
        Ok(Admin { email, token })
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
        .route("/admin/api/status", get(status))
        .route("/admin/api/setup", post(setup))
        .route("/admin/api/setup/finish", post(finish_setup))
        .route("/admin/api/sign-in", post(sign_in))
        .route("/admin/api/code", post(code))
        .route("/admin/api/sign-out", post(sign_out))
        .route("/admin/api/state", get(show))
        .route("/admin/api/settings", post(save))
        .route("/admin/api/test", post(test))
        .route("/admin/api/key", post(save_key))
        .route("/admin/api/key/remove", post(remove_key))
        .route("/admin/api/models", post(models))
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

/// What a code mailed to the admin `email` is stored as.
fn admin_code_hash(email: &str, code: &str) -> Vec<u8> {
    code_hash(&format!("admin:{email}"), &clean_code(code))
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
    if !state
        .account_limits()
        .per_email
        .allow(format!("admin:{email}"))
    {
        return Err(ApiError::TooMany("too many attempts; try again later"));
    }
    let hash = if is_admin(&state, &email) {
        state.db().admin_password(&email).await?
    } else {
        None
    };
    let known = hash.is_some();
    let right = verify_password(body.password, hash.unwrap_or_else(|| DUMMY_HASH.clone())).await?;
    // Wrong password, not an admin, or no password set: the same answer.
    if !(right && known) {
        return Err(ApiError::WrongPassword);
    }
    if !state.account_limits().mails.allow(email.clone()) {
        return Err(ApiError::TooMany("too many codes mailed; try again later"));
    }
    let code = auth::new_code();
    state
        .db()
        .put_admin_code(
            &email,
            &admin_code_hash(&email, &code),
            now_ms() + CODE_LIFETIME_MS,
        )
        .await?;
    state
        .mailer()
        .send_code(&email, Purpose::Admin, &code)
        .await
        .map_err(|error| {
            tracing::warn!(%error, "could not mail an admin code");
            ApiError::MailFailed
        })?;
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
    let email = normalize_email(&body.email)
        .filter(|email| is_admin(&state, email))
        .ok_or(ApiError::Invalid("wrong_code", "wrong code"))?;
    check_admin(
        state
            .db()
            .check_admin_code(&email, &admin_code_hash(&email, &body.code), now_ms())
            .await?,
    )?;
    tracing::info!("signed in to the admin page");
    signed_in(&state, &email)
}

/// The answer to a typed code.
fn check_admin(check: CodeCheck) -> Result<(), ApiError> {
    match check {
        CodeCheck::Right => Ok(()),
        CodeCheck::Wrong => Err(ApiError::Invalid("wrong_code", "wrong code")),
        CodeCheck::Gone => Err(ApiError::Invalid(
            "code_expired",
            "the code has expired; ask for a new one",
        )),
        CodeCheck::Locked => Err(ApiError::TooMany(
            "too many wrong codes; try again tomorrow",
        )),
    }
}

/// A new session for the admin `email`, in the answer's cookie.
fn signed_in(state: &AppState, email: &str) -> Result<Response, ApiError> {
    let token = state.admin_sessions().start(email);
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

/// What a code mailed to choose the first password is stored as.
fn setup_code_hash(email: &str, code: &str) -> Vec<u8> {
    code_hash(&format!("admin-setup:{email}"), &clean_code(code))
}

#[derive(Serialize)]
struct Status {
    setup: bool,
}

/// Whether the page should offer to choose a first password.
async fn status(State(state): State<AppState>) -> Result<Json<Status>, ApiError> {
    page_on(&state)?;
    let setup = state
        .db()
        .admins_to_set_up(&state.config().admin_emails)
        .await?;
    Ok(Json(Status { setup }))
}

#[derive(Deserialize)]
struct SetupBody {
    email: String,
}

/// Mails a code to choose the first password, to an admin without one.
/// The same answer whatever the address, so the page does not tell who
/// the admins are.
async fn setup(
    State(state): State<AppState>,
    headers: HeaderMap,
    addr: ClientAddr,
    Json(body): Json<SetupBody>,
) -> Result<StatusCode, ApiError> {
    page_on(&state)?;
    from_page(&headers)?;
    limited(&state, &headers, addr)?;
    let Some(email) = normalize_email(&body.email).filter(|email| is_admin(&state, email)) else {
        return Ok(StatusCode::ACCEPTED);
    };
    if !state.account_limits().mails.allow(email.clone()) {
        return Err(ApiError::TooMany("too many codes mailed; try again later"));
    }
    let code = auth::new_code();
    let kept = state
        .db()
        .put_admin_setup_code(
            &email,
            &setup_code_hash(&email, &code),
            now_ms() + CODE_LIFETIME_MS,
            now_ms(),
        )
        .await?;
    if kept {
        state
            .mailer()
            .send_code(&email, Purpose::AdminSetup, &code)
            .await
            .map_err(|error| {
                tracing::warn!(%error, "could not mail an admin setup code");
                ApiError::MailFailed
            })?;
    }
    Ok(StatusCode::ACCEPTED)
}

#[derive(Deserialize)]
struct FinishSetup {
    email: String,
    code: String,
    password: String,
}

/// The mailed code is right: keeps the first password and signs in.
async fn finish_setup(
    State(state): State<AppState>,
    headers: HeaderMap,
    addr: ClientAddr,
    Json(body): Json<FinishSetup>,
) -> Result<Response, ApiError> {
    page_on(&state)?;
    from_page(&headers)?;
    limited(&state, &headers, addr)?;
    auth::check_password(&body.password)?;
    let email = normalize_email(&body.email)
        .filter(|email| is_admin(&state, email))
        .ok_or(ApiError::Invalid("wrong_code", "wrong code"))?;
    check_admin(
        state
            .db()
            .check_admin_code(&email, &setup_code_hash(&email, &body.code), now_ms())
            .await?,
    )?;
    let hash = auth::hash_password(body.password).await?;
    if !state
        .db()
        .set_first_admin_password(&email, &hash, now_ms())
        .await?
    {
        return Err(ApiError::Invalid(
            "code_expired",
            "the password is already set",
        ));
    }
    tracing::info!("admin page password chosen");
    signed_in(&state, &email)
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
    /// Whether it has a key (or, for the "other" service, an address).
    has_key: bool,
    /// Where that is from: `page` (saved here) or `env`.
    key_from: Option<&'static str>,
    /// The key's last four characters, when it is long enough to hide the
    /// rest.
    key_end: Option<String>,
    /// The "other" service's address.
    base: Option<String>,
    /// Whether it works without a key.
    key_optional: bool,
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
    let saved = state.db().ai_keys().await?;
    Ok(PageState {
        email,
        settings: AiSettings::of(&ai),
        services: PRESETS
            .iter()
            .map(|preset| {
                let known = ai.keys.iter().find(|known| known.provider == preset.id);
                let from_page = saved.iter().any(|key| key.provider == preset.id);
                let key = known.map_or("", |known| known.key.0.as_str());
                Service {
                    id: preset.id,
                    // The app translates the last one's name; the page is in
                    // English.
                    name: if preset.name.is_empty() {
                        "Other (OpenAI-like)"
                    } else {
                        preset.name
                    },
                    model: preset.model,
                    has_key: known.is_some(),
                    key_from: known.map(|_| if from_page { "page" } else { "env" }),
                    key_end: (key.chars().count() >= 16).then(|| {
                        let end: Vec<char> = key.chars().rev().take(4).collect();
                        end.into_iter().rev().collect()
                    }),
                    base: (preset.id == katna_ai::provider::OTHER)
                        .then(|| known.map(|known| known.base.clone()))
                        .flatten(),
                    key_optional: !preset.needs_key,
                }
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
        .apply(&state.ai_base().await?)
        .map_err(|problem| ApiError::Invalid("bad_settings", problem))?;
    let json = serde_json::to_string(&settings).map_err(|_| ApiError::Hash)?;
    state
        .db()
        .set_ai_settings(&json, &admin.email, now_ms())
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

/// A key typed on the page.
#[derive(Deserialize)]
struct NewKey {
    provider: String,
    #[serde(default)]
    key: String,
    /// The "other" service's address.
    #[serde(default)]
    base: String,
}

/// A service named on the page.
#[derive(Deserialize)]
struct Named {
    provider: String,
}

fn known_preset(id: &str) -> Result<&'static katna_ai::provider::Preset, ApiError> {
    PRESETS
        .iter()
        .find(|preset| preset.id == id)
        .ok_or(ApiError::Invalid("bad_provider", "no such service"))
}

async fn save_key(
    State(state): State<AppState>,
    admin: Admin,
    Json(new): Json<NewKey>,
) -> Result<Json<PageState>, ApiError> {
    let preset = known_preset(&new.provider)?;
    let key = new.key.trim();
    if key.len() > 1000 || key.chars().any(|c| c.is_whitespace() || c.is_control()) {
        return Err(ApiError::Invalid("bad_key", "that is not a key"));
    }
    if key.is_empty() && preset.needs_key {
        return Err(ApiError::Invalid("bad_key", "type the key"));
    }
    let base = new.base.trim().trim_end_matches('/');
    let base = if preset.id == katna_ai::provider::OTHER {
        if !(base.starts_with("https://") || base.starts_with("http://"))
            || base.len() > 300
            || base.chars().any(|c| c.is_whitespace() || c.is_control())
        {
            return Err(ApiError::Invalid(
                "bad_address",
                "the address starts with https://",
            ));
        }
        Some(base)
    } else {
        None
    };
    state
        .db()
        .set_ai_key(preset.id, key, base, &admin.email, now_ms())
        .await?;
    state.load_ai_settings().await?;
    tracing::info!(
        provider = preset.id,
        "a Katna AI key was saved on the admin page"
    );
    Ok(Json(page_state(&state, admin.email).await?))
}

async fn remove_key(
    State(state): State<AppState>,
    admin: Admin,
    Json(named): Json<Named>,
) -> Result<Json<PageState>, ApiError> {
    let preset = known_preset(&named.provider)?;
    if state.db().remove_ai_key(preset.id).await? {
        state.load_ai_settings().await?;
        tracing::info!(
            provider = preset.id,
            "a Katna AI key was removed on the admin page"
        );
    }
    Ok(Json(page_state(&state, admin.email).await?))
}

/// The models of a service, or why there are none.
#[derive(Serialize)]
struct Models {
    models: Vec<String>,
    problem: Option<&'static str>,
}

async fn models(
    State(state): State<AppState>,
    _admin: Admin,
    Json(named): Json<Named>,
) -> Result<Json<Models>, ApiError> {
    let preset = known_preset(&named.provider)?;
    let ai = state.ai();
    let Some(service) = ai.keys.iter().find(|known| known.provider == preset.id) else {
        return Ok(Json(Models {
            models: Vec::new(),
            problem: Some("has no key"),
        }));
    };
    let call = provider::models_call(service.kind(), &service.base, &service.key.0);
    let answer = match tokio::time::timeout(MODELS_TIMEOUT, send(&call)).await {
        Ok(Ok((status, body))) => match provider::models(service.kind(), status, &body) {
            Ok(models) => Ok(models),
            Err(ProviderError::Key) => Err("refused the key"),
            Err(ProviderError::TooMany) => Err("is over its limits"),
            Err(ProviderError::Failed(_)) => Err("did not list its models"),
        },
        Ok(Err(_)) => Err("could not be reached"),
        Err(_) => Err("took too long"),
    };
    Ok(Json(match answer {
        Ok(models) => Models {
            models,
            problem: None,
        },
        Err(problem) => Models {
            models: Vec::new(),
            problem: Some(problem),
        },
    }))
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
        let token = sessions.start("a@b.io");
        assert_eq!(sessions.find(&token), Some("a@b.io".into()));
        sessions.end(&token);
        assert_eq!(sessions.find(&token), None);
        // Only the newest are kept.
        let first = sessions.start("a@b.io");
        for _ in 0..MAX_SESSIONS {
            sessions.start("a@b.io");
        }
        assert_eq!(sessions.find(&first), None);
    }
}
