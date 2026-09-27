// SPDX-License-Identifier: GPL-3.0-or-later

//! Automatic translation (`docs/ARCHITECTURE.md` §16.4): LibreTranslate
//! runs in its own container on the compose file's internal network, and
//! these routes pass its API through for computers signed in to a
//! confirmed Katna account, with a daily limit per account:
//!
//! - `GET /api/v1/languages`: the languages it translates between.
//! - `POST /api/v1/translate` `{"q", "source", "target"}`: plain text only.
//! - `POST /api/v1/detect` `{"q"}`: the language of a text.
//!
//! Only those fields go on (no `api_key`, no HTML), and neither the text
//! nor the translation is logged or kept.

use std::time::Duration;

use axum::Json;
use axum::body::Bytes;
use axum::extract::State;
use axum::http::StatusCode;
use axum::http::header::{self, HeaderValue};
use axum::response::{IntoResponse, Response};
use http_body_util::{BodyExt, Full};
use hyper_util::rt::TokioIo;
use serde::{Deserialize, Serialize};

use crate::auth::SignedIn;
use crate::routes::AppState;

/// Longest request body passed on. The daemon sends at most 4000
/// characters a request.
pub const MAX_REQUEST: usize = 64 * 1024;

/// Longest answer read from LibreTranslate.
const MAX_ANSWER: usize = 1024 * 1024;

/// How long LibreTranslate may take; a long piece on a small server is slow.
const TIMEOUT: Duration = Duration::from_secs(90);

/// A translation request as the daemon sends it.
#[derive(Debug, Deserialize, Serialize)]
struct TranslateBody {
    q: String,
    source: String,
    target: String,
}

/// A detection request.
#[derive(Debug, Deserialize, Serialize)]
struct DetectBody {
    q: String,
}

/// Whether `code` looks like a language code (`en`, `pt-BR`, `auto`).
fn is_code(code: &str) -> bool {
    (2..=8).contains(&code.len()) && code.bytes().all(|b| b.is_ascii_alphabetic() || b == b'-')
}

fn refuse(status: StatusCode, message: &'static str) -> Response {
    (status, Json(serde_json::json!({ "error": message }))).into_response()
}

/// Counts one use by `account`, or the answer when there is none left.
fn over_limit(state: &AppState, account: &str) -> Option<Response> {
    if state.config().translate_url.is_empty() {
        return Some(refuse(
            StatusCode::SERVICE_UNAVAILABLE,
            "translation is not set up on this server",
        ));
    }
    (!state.translations().allow(account.to_owned()))
        .then(|| refuse(StatusCode::TOO_MANY_REQUESTS, "too many translations today"))
}

pub(crate) async fn languages(
    State(state): State<AppState>,
    SignedIn { account, .. }: SignedIn,
) -> Response {
    if let Some(refused) = over_limit(&state, &account) {
        return refused;
    }
    pass_on(&state, "GET", "/languages", None).await
}

pub(crate) async fn translate(
    State(state): State<AppState>,
    SignedIn { account, .. }: SignedIn,
    body: Bytes,
) -> Response {
    if body.len() > MAX_REQUEST {
        return refuse(StatusCode::PAYLOAD_TOO_LARGE, "text too long");
    }
    let Ok(request) = serde_json::from_slice::<TranslateBody>(&body) else {
        return refuse(StatusCode::BAD_REQUEST, "expected q, source and target");
    };
    if request.q.is_empty() || !is_code(&request.source) || !is_code(&request.target) {
        return refuse(StatusCode::BAD_REQUEST, "expected q, source and target");
    }
    if let Some(refused) = over_limit(&state, &account) {
        return refused;
    }
    let body = serde_json::json!({
        "q": request.q,
        "source": request.source,
        "target": request.target,
        "format": "text",
    });
    pass_on(&state, "POST", "/translate", Some(body)).await
}

pub(crate) async fn detect(
    State(state): State<AppState>,
    SignedIn { account, .. }: SignedIn,
    body: Bytes,
) -> Response {
    if body.len() > MAX_REQUEST {
        return refuse(StatusCode::PAYLOAD_TOO_LARGE, "text too long");
    }
    let Ok(request) = serde_json::from_slice::<DetectBody>(&body) else {
        return refuse(StatusCode::BAD_REQUEST, "expected q");
    };
    if let Some(refused) = over_limit(&state, &account) {
        return refused;
    }
    pass_on(
        &state,
        "POST",
        "/detect",
        Some(serde_json::json!({ "q": request.q })),
    )
    .await
}

/// Sends the request to LibreTranslate and hands back its answer.
async fn pass_on(
    state: &AppState,
    method: &str,
    path: &str,
    body: Option<serde_json::Value>,
) -> Response {
    let base = state.config().translate_url.trim_end_matches('/');
    let exchange = exchange(base, method, path, body);
    match tokio::time::timeout(TIMEOUT, exchange).await {
        Ok(Ok((status, body))) => {
            if !status.is_success() {
                // The status only: the answer may quote the text.
                tracing::warn!(%status, path, "LibreTranslate refused a request");
            }
            let mut response = (status, body).into_response();
            response.headers_mut().insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );
            response
        }
        Ok(Err(error)) => {
            tracing::error!(%error, "LibreTranslate could not be reached");
            refuse(StatusCode::BAD_GATEWAY, "translation is not available")
        }
        Err(_) => {
            tracing::warn!(path, "LibreTranslate took too long");
            refuse(StatusCode::GATEWAY_TIMEOUT, "translation took too long")
        }
    }
}

/// One HTTP/1.1 request to LibreTranslate at `base` (`http://host:port`,
/// on the internal network).
async fn exchange(
    base: &str,
    method: &str,
    path: &str,
    body: Option<serde_json::Value>,
) -> Result<(StatusCode, Bytes), String> {
    let authority = base
        .strip_prefix("http://")
        .ok_or("KATNA_SERVER_TRANSLATE_URL must start with http://")?;
    let address = if authority.contains(':') {
        authority.to_owned()
    } else {
        format!("{authority}:80")
    };
    let stream = tokio::net::TcpStream::connect(&address)
        .await
        .map_err(|e| format!("{address}: {e}"))?;
    let (mut sender, connection) = hyper::client::conn::http1::handshake(TokioIo::new(stream))
        .await
        .map_err(|e| e.to_string())?;
    tokio::spawn(connection);
    let body = body.map(|b| b.to_string()).unwrap_or_default();
    let request = hyper::Request::builder()
        .method(method)
        .uri(path)
        .header(header::HOST, authority)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::ACCEPT, "application/json")
        .body(Full::new(Bytes::from(body)))
        .map_err(|e| e.to_string())?;
    let response = sender
        .send_request(request)
        .await
        .map_err(|e| e.to_string())?;
    let status = response.status();
    let body = http_body_util::Limited::new(response.into_body(), MAX_ANSWER)
        .collect()
        .await
        .map_err(|e| e.to_string())?
        .to_bytes();
    Ok((status, body))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes() {
        assert!(is_code("en"));
        assert!(is_code("pt-BR"));
        assert!(is_code("auto"));
        assert!(!is_code("e"));
        assert!(!is_code("en; drop"));
        assert!(!is_code("toolonglanguage"));
    }

    #[test]
    fn only_the_known_fields_go_on() {
        let request: TranslateBody = serde_json::from_str(
            r#"{"q":"Hola","source":"es","target":"en","format":"html","api_key":"x"}"#,
        )
        .unwrap();
        let passed = serde_json::to_value(&request).unwrap();
        assert_eq!(
            passed,
            serde_json::json!({"q":"Hola","source":"es","target":"en"})
        );
    }
}
