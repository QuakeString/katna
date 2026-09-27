// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna account routes, all with the install's `Authorization: Bearer`
//! token (see [`crate::auth`]):
//!
//! - `POST /api/v1/account` `{email, password, device}`: create an account,
//!   sign this install in and mail a code to confirm the address.
//! - `POST /api/v1/account/verify` `{code}`: confirm the address.
//! - `POST /api/v1/account/verify/resend`: mail a new code.
//! - `POST /api/v1/account/sign-in` `{email, password, device}`.
//! - `POST /api/v1/account/sign-out`: this install.
//! - `GET /api/v1/account`: `{email, verified, created_at}`.
//! - `GET /api/v1/account/devices`: the devices signed in.
//! - `DELETE /api/v1/account/devices/<id>`: sign another device out.
//! - `POST /api/v1/account/password` `{current, new}`: change the password;
//!   signs the other devices out.
//! - `POST /api/v1/account/reset` `{email}`: mail a reset code.
//! - `POST /api/v1/account/reset/confirm` `{email, code, password, device}`:
//!   set a new password and sign this install in; signs the others out.
//! - `POST /api/v1/account/delete` `{password}`: delete the account, its
//!   devices and all their data.

use std::net::IpAddr;
use std::time::Duration;

use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use crate::auth::{
    self, CODE_LIFETIME_MS, DUMMY_HASH, Member, check_password, clean_code, clean_device_name,
    code_hash, hash_password, normalize_email, verify_password,
};
use crate::db::{CodeCheck, Device, now_ms};
use crate::ids;
use crate::limits::WindowLimit;
use crate::mailer::Purpose;
use crate::routes::{ApiError, AppState, ClientAddr, client_ip};

/// Limits on guessing passwords and on mailing codes.
pub struct AccountLimits {
    /// Sign-in attempts per address.
    per_email: WindowLimit<String>,
    /// Sign-in and sign-up attempts per client address.
    per_ip: WindowLimit<Option<IpAddr>>,
    /// Codes mailed per address.
    mails: WindowLimit<String>,
}

impl Default for AccountLimits {
    fn default() -> Self {
        let quarter = Duration::from_secs(15 * 60);
        Self {
            per_email: WindowLimit::new(10, quarter),
            per_ip: WindowLimit::new(60, quarter),
            mails: WindowLimit::new(5, Duration::from_secs(3600)),
        }
    }
}

/// The account routes, merged into the server's router.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/account", post(create).get(show))
        .route("/api/v1/account/verify", post(verify))
        .route("/api/v1/account/verify/resend", post(resend))
        .route("/api/v1/account/sign-in", post(sign_in))
        .route("/api/v1/account/sign-out", post(sign_out))
        .route("/api/v1/account/devices", get(devices))
        .route("/api/v1/account/devices/{id}", delete(sign_out_device))
        .route("/api/v1/account/password", post(change_password))
        .route("/api/v1/account/reset", post(reset))
        .route("/api/v1/account/reset/confirm", post(confirm_reset))
        .route("/api/v1/account/delete", post(delete_account))
}

/// Body of sign-up and sign-in.
#[derive(Deserialize)]
pub struct Credentials {
    /// The account's address.
    pub email: String,
    /// Its password.
    pub password: String,
    /// This device's name, shown in the device list.
    #[serde(default)]
    pub device: String,
}

/// An account as the app shows it.
#[derive(Serialize)]
pub struct AccountInfo {
    /// Address.
    pub email: String,
    /// Whether the address is confirmed.
    pub verified: bool,
    /// When it was created (ms since the Unix epoch).
    pub created_at: i64,
}

fn limited(state: &AppState, headers: &HeaderMap, addr: ClientAddr) -> Result<(), ApiError> {
    let ip = client_ip(state, headers, addr.0);
    if state.account_limits().per_ip.allow(ip) {
        Ok(())
    } else {
        Err(ApiError::TooMany("too many attempts; try again later"))
    }
}

/// Mails a new code for `purpose`, within the per-address limit.
async fn mail_code(
    state: &AppState,
    account: &str,
    email: &str,
    purpose: Purpose,
) -> Result<(), ApiError> {
    if !state.account_limits().mails.allow(email.to_owned()) {
        return Err(ApiError::TooMany("too many codes mailed; try again later"));
    }
    let code = auth::new_code();
    state
        .db()
        .put_code(
            account,
            purpose.as_str(),
            &code_hash(account, &code),
            now_ms() + CODE_LIFETIME_MS,
        )
        .await?;
    state
        .mailer()
        .send_code(email, purpose, &code)
        .await
        .map_err(|error| {
            tracing::warn!(%error, "could not mail a code");
            ApiError::MailFailed
        })
}

async fn create(
    State(state): State<AppState>,
    headers: HeaderMap,
    addr: ClientAddr,
    install: crate::routes::Install,
    Json(body): Json<Credentials>,
) -> Result<(StatusCode, Json<AccountInfo>), ApiError> {
    limited(&state, &headers, addr)?;
    let email = normalize_email(&body.email)
        .ok_or(ApiError::Invalid("bad_email", "not an email address"))?;
    check_password(&body.password)?;
    let hash = hash_password(body.password).await?;
    let account = ids::new_id();
    let now = now_ms();
    if !state
        .db()
        .create_account(&account, &email, &hash, now)
        .await?
    {
        return Err(ApiError::Conflict(
            "this address already has a Katna account",
        ));
    }
    state
        .db()
        .sign_in(&install.0, &account, &clean_device_name(&body.device), now)
        .await?;
    // The account exists either way; a failed mail is sent again with
    // "resend".
    if let Err(error) = mail_code(&state, &account, &email, Purpose::Verify).await {
        tracing::warn!(?error, "sign-up code not mailed");
    }
    Ok((
        StatusCode::CREATED,
        Json(AccountInfo {
            email,
            verified: false,
            created_at: now,
        }),
    ))
}

/// Body of `verify`.
#[derive(Deserialize)]
pub struct Code {
    /// The emailed code.
    pub code: String,
}

async fn check(
    state: &AppState,
    account: &str,
    purpose: Purpose,
    code: &str,
) -> Result<(), ApiError> {
    let code = clean_code(code);
    match state
        .db()
        .check_code(
            account,
            purpose.as_str(),
            &code_hash(account, &code),
            now_ms(),
        )
        .await?
    {
        CodeCheck::Right => Ok(()),
        CodeCheck::Wrong => Err(ApiError::Invalid("wrong_code", "wrong code")),
        CodeCheck::Gone => Err(ApiError::Invalid(
            "code_expired",
            "the code has expired; ask for a new one",
        )),
    }
}

async fn verify(
    State(state): State<AppState>,
    member: Member,
    Json(body): Json<Code>,
) -> Result<StatusCode, ApiError> {
    if !member.verified {
        check(&state, &member.account, Purpose::Verify, &body.code).await?;
        state.db().set_verified(&member.account, now_ms()).await?;
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn resend(State(state): State<AppState>, member: Member) -> Result<StatusCode, ApiError> {
    if member.verified {
        return Ok(StatusCode::NO_CONTENT);
    }
    let account = state
        .db()
        .account(&member.account)
        .await?
        .ok_or(ApiError::SignInNeeded)?;
    mail_code(&state, &account.id, &account.email, Purpose::Verify).await?;
    Ok(StatusCode::ACCEPTED)
}

async fn sign_in(
    State(state): State<AppState>,
    headers: HeaderMap,
    addr: ClientAddr,
    install: crate::routes::Install,
    Json(body): Json<Credentials>,
) -> Result<Json<AccountInfo>, ApiError> {
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
    let right = verify_password(body.password, hash).await;
    let account = account.filter(|_| right).ok_or(ApiError::WrongPassword)?;
    state
        .db()
        .sign_in(
            &install.0,
            &account.id,
            &clean_device_name(&body.device),
            now_ms(),
        )
        .await?;
    Ok(Json(AccountInfo {
        email: account.email,
        verified: account.verified_at.is_some(),
        created_at: account.created_at,
    }))
}

async fn sign_out(State(state): State<AppState>, member: Member) -> Result<StatusCode, ApiError> {
    state
        .db()
        .sign_out(&member.account, &member.install)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn show(
    State(state): State<AppState>,
    member: Member,
) -> Result<Json<AccountInfo>, ApiError> {
    let account = state
        .db()
        .account(&member.account)
        .await?
        .ok_or(ApiError::SignInNeeded)?;
    Ok(Json(AccountInfo {
        email: account.email,
        verified: account.verified_at.is_some(),
        created_at: account.created_at,
    }))
}

/// One device as listed.
#[derive(Serialize)]
pub struct DeviceInfo {
    /// The device.
    #[serde(flatten)]
    pub device: Device,
    /// Whether it is the one asking.
    pub this: bool,
}

async fn devices(
    State(state): State<AppState>,
    member: Member,
) -> Result<Json<Vec<DeviceInfo>>, ApiError> {
    let devices = state.db().devices(&member.account).await?;
    Ok(Json(
        devices
            .into_iter()
            .map(|device| DeviceInfo {
                this: device.id == member.install,
                device,
            })
            .collect(),
    ))
}

async fn sign_out_device(
    State(state): State<AppState>,
    member: Member,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    if ids::is_valid_id(&id) && state.db().sign_out(&member.account, &id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::NotFound)
    }
}

/// Body of `password`.
#[derive(Deserialize)]
pub struct NewPassword {
    /// The password now.
    pub current: String,
    /// The one to use from now on.
    pub new: String,
}

/// Checks `password` against the member's account.
async fn confirm_password(
    state: &AppState,
    member: &Member,
    password: String,
) -> Result<(), ApiError> {
    if !state
        .account_limits()
        .per_email
        .allow(member.account.clone())
    {
        return Err(ApiError::TooMany("too many attempts; try again later"));
    }
    let account = state
        .db()
        .account(&member.account)
        .await?
        .ok_or(ApiError::SignInNeeded)?;
    if verify_password(password, account.password_hash).await {
        Ok(())
    } else {
        Err(ApiError::WrongPassword)
    }
}

async fn change_password(
    State(state): State<AppState>,
    member: Member,
    Json(body): Json<NewPassword>,
) -> Result<StatusCode, ApiError> {
    check_password(&body.new)?;
    confirm_password(&state, &member, body.current).await?;
    let hash = hash_password(body.new).await?;
    state.db().set_password(&member.account, &hash).await?;
    state
        .db()
        .sign_out_others(&member.account, &member.install)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Body of `reset`.
#[derive(Deserialize)]
pub struct Email {
    /// The account's address.
    pub email: String,
}

async fn reset(
    State(state): State<AppState>,
    headers: HeaderMap,
    addr: ClientAddr,
    _install: crate::routes::Install,
    Json(body): Json<Email>,
) -> Result<StatusCode, ApiError> {
    limited(&state, &headers, addr)?;
    let email = normalize_email(&body.email)
        .ok_or(ApiError::Invalid("bad_email", "not an email address"))?;
    // The same answer whether or not the address has an account.
    if let Some(account) = state.db().account_by_email(&email).await? {
        mail_code(&state, &account.id, &email, Purpose::Reset).await?;
    }
    Ok(StatusCode::ACCEPTED)
}

/// Body of `reset/confirm`.
#[derive(Deserialize)]
pub struct ConfirmReset {
    /// The account's address.
    pub email: String,
    /// The emailed code.
    pub code: String,
    /// The new password.
    pub password: String,
    /// This device's name.
    #[serde(default)]
    pub device: String,
}

async fn confirm_reset(
    State(state): State<AppState>,
    headers: HeaderMap,
    addr: ClientAddr,
    install: crate::routes::Install,
    Json(body): Json<ConfirmReset>,
) -> Result<Json<AccountInfo>, ApiError> {
    limited(&state, &headers, addr)?;
    check_password(&body.password)?;
    let email =
        normalize_email(&body.email).ok_or(ApiError::Invalid("wrong_code", "wrong code"))?;
    let account = state
        .db()
        .account_by_email(&email)
        .await?
        .ok_or(ApiError::Invalid("wrong_code", "wrong code"))?;
    check(&state, &account.id, Purpose::Reset, &body.code).await?;
    let hash = hash_password(body.password).await?;
    let now = now_ms();
    state.db().set_password(&account.id, &hash).await?;
    // The code arrived by mail, so the address works.
    state.db().set_verified(&account.id, now).await?;
    state
        .db()
        .sign_in(
            &install.0,
            &account.id,
            &clean_device_name(&body.device),
            now,
        )
        .await?;
    state.db().sign_out_others(&account.id, &install.0).await?;
    Ok(Json(AccountInfo {
        email: account.email,
        verified: true,
        created_at: account.created_at,
    }))
}

/// Body of `delete`.
#[derive(Deserialize)]
pub struct Password {
    /// The account's password.
    pub password: String,
}

async fn delete_account(
    State(state): State<AppState>,
    member: Member,
    Json(body): Json<Password>,
) -> Result<StatusCode, ApiError> {
    confirm_password(&state, &member, body.password).await?;
    state.db().delete_account(&member.account).await?;
    Ok(StatusCode::NO_CONTENT)
}
