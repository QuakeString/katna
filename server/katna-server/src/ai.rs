// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna AI (`docs/ARCHITECTURE.md` §16.5): writing help for computers
//! signed in to a confirmed Katna account, through the AI service the
//! server's owner chose (`KATNA_SERVER_AI_*`; Gemini 2.5 Flash-Lite unless
//! told otherwise) and a fallback tried when it fails.
//!
//! - `POST /api/v1/ai/rephrase` `{"text", "tone", "instruction"}`
//! - `POST /api/v1/ai/complete` `{"before", "answered"}`
//!
//! Both answer `{"text", "plan": {"kind", "days_left"}}`
//! ([`katna_ai::wire`]): the service's text as it came (the daemon
//! cleans it up), and where the account stands. An account gets
//! [`AiConfig::trial_days`] free from its first use, then needs paid time
//! (402). Each account may cost at most [`AiConfig::account_cap_micros`] a
//! month and all together [`AiConfig::budget_micros`] (429 beyond), and an
//! account may ask [`AiConfig::per_hour`] times an hour.
//!
//! The prompts are built here from the request with [`katna_ai::prompt`],
//! so a client cannot send the service anything else. Neither the text nor
//! the answer is logged or kept; only the number of requests and their
//! cost per account and month are.

use std::sync::{Arc, OnceLock};
use std::time::Duration;

use axum::body::Bytes;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use http_body_util::{BodyExt, Full};
use hyper::Uri;
use hyper::header;
use hyper_util::rt::TokioIo;
use katna_ai::wire::{AiAnswer, CompleteRequest, Plan, RephraseRequest, plan};
use katna_ai::{Prompt, ProviderError, Tone, prompt, provider};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_rustls::TlsConnector;
use tokio_rustls::rustls::{self, ClientConfig, RootCertStore};

use crate::auth::SignedIn;
use crate::config::{AiConfig, AiService};
use crate::db::{month_of, now_ms};
use crate::routes::{ApiError, AppState};

/// Longest request body taken. The prompts cut the text much shorter.
pub const MAX_REQUEST: usize = 64 * 1024;

/// Longest answer read from a service.
const MAX_ANSWER: usize = 1024 * 1024;

/// How long a service may take to rephrase, and to finish a sentence
/// (the suggestion is useless once the user has typed on).
const REPHRASE_TIMEOUT: Duration = Duration::from_secs(60);
const COMPLETE_TIMEOUT: Duration = Duration::from_secs(10);

/// Requests passed on at once, for everyone together.
pub const CONCURRENCY: usize = 64;

const DAY_MS: i64 = 86_400_000;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/ai/rephrase", post(rephrase))
        .route("/api/v1/ai/complete", post(complete))
}

async fn rephrase(
    State(state): State<AppState>,
    SignedIn { account, .. }: SignedIn,
    body: Bytes,
) -> Result<Json<AiAnswer>, ApiError> {
    if body.len() > MAX_REQUEST {
        return Err(ApiError::BadRequest("text too long"));
    }
    let request: RephraseRequest = serde_json::from_slice(&body)
        .map_err(|_| ApiError::BadRequest("expected text and tone"))?;
    let tone = Tone::parse(&request.tone).ok_or(ApiError::BadRequest("unknown tone"))?;
    let plan = admit(&state, &account).await?;
    let Some(prompt) = prompt::rephrase(&request.text, tone, &request.instruction) else {
        return Err(ApiError::BadRequest("nothing to rephrase"));
    };
    let text = ask(&state, &account, &prompt, REPHRASE_TIMEOUT).await?;
    Ok(Json(AiAnswer { text, plan }))
}

async fn complete(
    State(state): State<AppState>,
    SignedIn { account, .. }: SignedIn,
    body: Bytes,
) -> Result<Json<AiAnswer>, ApiError> {
    if body.len() > MAX_REQUEST {
        return Err(ApiError::BadRequest("text too long"));
    }
    let request: CompleteRequest =
        serde_json::from_slice(&body).map_err(|_| ApiError::BadRequest("expected before"))?;
    let plan = admit(&state, &account).await?;
    // Too little to go on is no suggestion, not an error.
    let text = match prompt::complete(&request.before, &request.answered) {
        Some(prompt) => ask(&state, &account, &prompt, COMPLETE_TIMEOUT).await?,
        None => String::new(),
    };
    Ok(Json(AiAnswer { text, plan }))
}

/// Where `account` stands, or why it may not ask now.
async fn admit(state: &AppState, account: &str) -> Result<Plan, ApiError> {
    let ai = state.ai();
    if !ai.on || ai.services.is_empty() || ai.budget_micros == 0 {
        return Err(ApiError::Busy("Katna AI is not set up on this server"));
    }
    if !state.ai_requests().allow(account.to_owned()) {
        return Err(ApiError::TooMany("too many Katna AI requests this hour"));
    }
    let now = now_ms();
    let stored = state.db().ai_plan(account, now).await?;
    let plan =
        standing(&ai, stored.first_use, stored.paid_until, now).ok_or(ApiError::PaymentNeeded)?;
    let spent = state.db().ai_spent(account, month_of(now)).await?;
    if spent.account as u64 >= ai.account_cap_micros {
        return Err(ApiError::TooMany("this month's Katna AI limit is reached"));
    }
    if spent.everyone as u64 >= ai.budget_micros {
        tracing::warn!("Katna AI's monthly budget is used up");
        return Err(ApiError::TooMany("Katna AI is over its limit this month"));
    }
    Ok(plan)
}

/// Where an account that first used Katna AI at `first_use` and paid
/// until `paid_until` stands at `now`; `None` when it needs to pay.
fn standing(ai: &AiConfig, first_use: i64, paid_until: Option<i64>, now: i64) -> Option<Plan> {
    if paid_until.is_some_and(|until| until > now) {
        return Some(Plan {
            kind: plan::PAID.into(),
            days_left: None,
        });
    }
    let trial_end = first_use + i64::from(ai.trial_days) * DAY_MS;
    (now < trial_end).then(|| Plan {
        kind: plan::TRIAL.into(),
        days_left: Some(((trial_end - now + DAY_MS - 1) / DAY_MS) as u32),
    })
}

/// The answer to `prompt` from the first service that gives one, its cost
/// counted to `account`.
async fn ask(
    state: &AppState,
    account: &str,
    prompt: &Prompt,
    timeout: Duration,
) -> Result<String, ApiError> {
    let Ok(_turn) = state.ai_calls().clone().try_acquire_owned() else {
        return Err(ApiError::Busy("Katna AI is busy; try again shortly"));
    };
    let ai = state.ai();
    for service in &ai.services {
        match ask_one(service, prompt, timeout).await {
            Ok((text, body)) => {
                let cost = cost(&ai, service, prompt, &text, &body);
                state
                    .db()
                    .ai_record(account, month_of(now_ms()), cost as i64)
                    .await?;
                return Ok(text);
            }
            Err(problem) => {
                tracing::warn!(
                    provider = service.provider,
                    problem,
                    "AI service gave no answer"
                )
            }
        }
    }
    Err(ApiError::Upstream("Katna AI is not available just now"))
}

/// `service`'s answer to `prompt` and the body it came in, or what went
/// wrong in a few words (never the service's message, which may quote the
/// text).
pub(crate) async fn ask_one(
    service: &AiService,
    prompt: &Prompt,
    timeout: Duration,
) -> Result<(String, Bytes), &'static str> {
    let call = provider::call(
        service.kind(),
        &service.base,
        &service.model,
        &service.key.0,
        prompt,
    );
    let (status, body) = match tokio::time::timeout(timeout, send(&call)).await {
        Ok(Ok(answer)) => answer,
        Ok(Err(error)) => {
            tracing::debug!(provider = service.provider, %error, "AI service unreachable");
            return Err("could not be reached");
        }
        Err(_) => return Err("took too long"),
    };
    match provider::answer(service.kind(), status, &body) {
        Ok(text) => Ok((text, body)),
        Err(ProviderError::Key) => Err("refused the key"),
        Err(ProviderError::TooMany) => Err("is over its limits"),
        Err(ProviderError::Failed(_)) if status == 404 => Err("does not know the model"),
        Err(ProviderError::Failed(_)) => Err("failed"),
    }
}

/// Katna AI's settings as the admin page saves them, over those of the
/// environment ([`AiConfig`]).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiSettings {
    /// Whether Katna AI answers at all.
    pub on: bool,
    /// The service asked first, and the one asked when that fails.
    pub main: Option<Choice>,
    pub fallback: Option<Choice>,
    pub trial_days: u32,
    pub account_cap_micros: u64,
    pub budget_micros: u64,
    pub price_in_micros: u64,
    pub price_out_micros: u64,
    pub per_hour: u32,
}

/// A service and model.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Choice {
    pub provider: String,
    pub model: String,
}

impl AiSettings {
    /// The settings `ai` stands for.
    pub fn of(ai: &AiConfig) -> Self {
        let choice = |service: Option<&AiService>| {
            service.map(|service| Choice {
                provider: service.provider.to_owned(),
                model: service.model.clone(),
            })
        };
        Self {
            on: ai.on,
            main: choice(ai.services.first()),
            fallback: choice(ai.services.get(1)),
            trial_days: ai.trial_days,
            account_cap_micros: ai.account_cap_micros,
            budget_micros: ai.budget_micros,
            price_in_micros: ai.price_in_micros,
            price_out_micros: ai.price_out_micros,
            per_hour: ai.per_hour,
        }
    }

    /// These settings over `env`'s keys, or why they cannot be used.
    pub fn apply(&self, env: &AiConfig) -> Result<AiConfig, &'static str> {
        if self.trial_days > 3650
            || self.per_hour > 100_000
            || [
                self.account_cap_micros,
                self.budget_micros,
                self.price_in_micros,
                self.price_out_micros,
            ]
            .iter()
            .any(|micros| *micros > 1_000_000_000_000)
        {
            return Err("a number is out of range");
        }
        let mut services = Vec::new();
        for choice in [&self.main, &self.fallback].into_iter().flatten() {
            let service = env
                .service(&choice.provider, &choice.model)
                .ok_or("no key for that service, or no model")?;
            services.push(service);
        }
        if self.on && services.is_empty() {
            return Err("choose a service");
        }
        Ok(AiConfig {
            on: self.on,
            services,
            keys: env.keys.clone(),
            trial_days: self.trial_days,
            account_cap_micros: self.account_cap_micros,
            budget_micros: self.budget_micros,
            price_in_micros: self.price_in_micros,
            price_out_micros: self.price_out_micros,
            per_hour: self.per_hour,
        })
    }
}

/// What one answer cost, in millionths of a US dollar: from the tokens the
/// service counted, or about four characters a token when it did not say.
fn cost(ai: &AiConfig, service: &AiService, prompt: &Prompt, text: &str, body: &[u8]) -> u64 {
    let (input, output) = provider::usage(service.kind(), body).unwrap_or_else(|| {
        let chars = prompt.system.chars().count() + prompt.user.chars().count();
        (
            chars.div_ceil(4) as u64,
            text.chars().count().div_ceil(4) as u64,
        )
    });
    (input * ai.price_in_micros + output * ai.price_out_micros).div_ceil(1_000_000)
}

/// Sends `call` (HTTPS, or HTTP for a service on the internal network) and
/// gives back the status and body.
async fn send(call: &provider::Call) -> Result<(u16, Bytes), String> {
    let uri: Uri = call.url.parse().map_err(|_| "bad service address")?;
    let host = uri
        .host()
        .ok_or("no host in the service address")?
        .to_owned();
    let https = match uri.scheme_str() {
        Some("https") => true,
        Some("http") => false,
        _ => return Err("the service address must be https://".into()),
    };
    let port = uri.port_u16().unwrap_or(if https { 443 } else { 80 });
    let stream = tokio::net::TcpStream::connect((host.as_str(), port))
        .await
        .map_err(|e| format!("{host}:{port}: {e}"))?;
    if https {
        let name = rustls::pki_types::ServerName::try_from(host.clone())
            .map_err(|_| format!("{host}: not a host name"))?;
        let stream = TlsConnector::from(tls())
            .connect(name, stream)
            .await
            .map_err(|e| format!("{host}: TLS: {e}"))?;
        exchange(stream, &uri, call).await
    } else {
        exchange(stream, &uri, call).await
    }
}

/// One HTTP/1.1 request over `io`.
async fn exchange<I>(io: I, uri: &Uri, call: &provider::Call) -> Result<(u16, Bytes), String>
where
    I: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    let (mut sender, connection) = hyper::client::conn::http1::handshake(TokioIo::new(io))
        .await
        .map_err(|e| e.to_string())?;
    tokio::spawn(connection);
    let path = uri.path_and_query().map_or("/", |p| p.as_str());
    let authority = uri.authority().map_or("", |a| a.as_str());
    let mut request = hyper::Request::builder()
        .method("POST")
        .uri(path)
        .header(header::HOST, authority)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::ACCEPT, "application/json")
        .header(header::USER_AGENT, "katna-server");
    for (name, value) in &call.headers {
        request = request.header(*name, value);
    }
    let request = request
        .body(Full::new(Bytes::from(call.body.clone())))
        .map_err(|e| e.to_string())?;
    let response = sender
        .send_request(request)
        .await
        .map_err(|e| e.to_string())?;
    let status = response.status().as_u16();
    let body = http_body_util::Limited::new(response.into_body(), MAX_ANSWER)
        .collect()
        .await
        .map_err(|e| e.to_string())?
        .to_bytes();
    Ok((status, body))
}

/// TLS with the Mozilla roots built in (the image has no system store).
fn tls() -> Arc<ClientConfig> {
    static CONFIG: OnceLock<Arc<ClientConfig>> = OnceLock::new();
    CONFIG
        .get_or_init(|| {
            let roots = RootCertStore {
                roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
            };
            let config = ClientConfig::builder_with_provider(Arc::new(
                rustls::crypto::ring::default_provider(),
            ))
            .with_safe_default_protocol_versions()
            .expect("ring supports the default TLS versions")
            .with_root_certificates(roots)
            .with_no_client_auth();
            Arc::new(config)
        })
        .clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Secret;

    #[test]
    fn free_month_then_pay() {
        let ai = AiConfig::default();
        let start = 1_790_812_800_000;
        let trial = standing(&ai, start, None, start).unwrap();
        assert_eq!(trial.kind, plan::TRIAL);
        assert_eq!(trial.days_left, Some(30));
        let last_day = standing(&ai, start, None, start + 29 * DAY_MS + 1).unwrap();
        assert_eq!(last_day.days_left, Some(1));
        assert_eq!(standing(&ai, start, None, start + 30 * DAY_MS), None);
        // Paid time counts, and runs out.
        let later = start + 40 * DAY_MS;
        let paid = standing(&ai, start, Some(later + 1), later).unwrap();
        assert_eq!(paid.kind, plan::PAID);
        assert_eq!(standing(&ai, start, Some(later), later), None);
    }

    #[test]
    fn settings_need_a_key() {
        let gemini = AiService {
            provider: "gemini",
            base: "https://generativelanguage.googleapis.com".into(),
            model: "gemini-2.5-flash-lite".into(),
            key: Secret("g".into()),
        };
        let env = AiConfig {
            services: vec![gemini.clone()],
            keys: vec![gemini],
            ..AiConfig::default()
        };
        let mut settings = AiSettings::of(&env);
        assert_eq!(settings.apply(&env).unwrap().services.len(), 1);
        settings.main = Some(Choice {
            provider: "gemini".into(),
            model: "gemini-3-flash".into(),
        });
        settings.budget_micros = 10_000_000;
        let applied = settings.apply(&env).unwrap();
        assert_eq!(applied.services[0].model, "gemini-3-flash");
        assert_eq!(applied.budget_micros, 10_000_000);
        settings.fallback = Some(Choice {
            provider: "openai".into(),
            model: "gpt-5-mini".into(),
        });
        assert!(settings.apply(&env).is_err());
        settings.fallback = None;
        settings.main = None;
        assert!(settings.apply(&env).is_err());
        settings.on = false;
        assert!(settings.apply(&env).is_ok());
    }

    #[test]
    fn costs() {
        let ai = AiConfig::default();
        let service = AiService {
            provider: "gemini",
            base: "https://generativelanguage.googleapis.com".into(),
            model: "gemini-2.5-flash-lite".into(),
            key: Secret(String::new()),
        };
        let prompt = Prompt {
            system: "s".repeat(400),
            user: "u".repeat(400),
            max_tokens: 64,
            temperature: 0.4,
            quick: false,
        };
        // Counted by the service: 1000 in at $0.10/M, 500 out at $0.40/M.
        let body = br#"{"usageMetadata":{"promptTokenCount":1000,"candidatesTokenCount":500}}"#;
        assert_eq!(cost(&ai, &service, &prompt, "", body), 300);
        // Not counted: 200 in and 100 out, by characters.
        assert_eq!(cost(&ai, &service, &prompt, &"x".repeat(400), b"{}"), 60);
    }
}
