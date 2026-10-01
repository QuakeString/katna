// SPDX-License-Identifier: GPL-3.0-or-later

//! Writing help from an AI service (`docs/ARCHITECTURE.md` §16.5): Katna
//! Mail sends the text the user selected to rephrase, or the paragraph
//! being written to finish its sentence, and the daemon asks the service
//! the settings name (`[ai]`): Katna AI on Katna Server with this
//! computer's Katna account, or the user's own service with its key from
//! the Secret Service. Nothing is kept or logged but that it happened.

use std::time::Duration;

use katna_ai::provider::{self, OTHER, ProviderError};
use katna_ai::wire::{AiAnswer, CompleteRequest, Plan, RephraseRequest, problem};
use katna_ai::{Prompt, Tone, prompt};
use katna_core::config::{Ai, AiSource};
use katna_sync::autoconfig::http;
use katna_sync::net::Tls;

use crate::katna_account::{self, Session};
use crate::secrets::Secrets;

/// How long a rephrase may take.
const REPHRASE_TIMEOUT: Duration = Duration::from_secs(45);
/// How long finishing a sentence may take; a late suggestion is no use.
const COMPLETE_TIMEOUT: Duration = Duration::from_secs(8);

/// [`Plan::kind`] when the user's own service answered.
pub const OWN: &str = "own";

/// Why no text came back; [`AiError::problem`] is what Katna Mail hears.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum AiError {
    #[error("writing help is off")]
    Off,
    #[error("sign in to a Katna account")]
    SignIn,
    #[error("the free month is over")]
    Pay,
    #[error("over a limit for now")]
    TooMany,
    #[error("no key saved")]
    NoKey,
    #[error("the service refused the key")]
    BadKey,
    #[error("{0}")]
    Failed(String),
}

impl AiError {
    /// The [`wire::problem`] name of this error.
    pub fn problem(&self) -> &'static str {
        match self {
            AiError::Off => problem::OFF,
            AiError::SignIn => problem::SIGN_IN,
            AiError::Pay => problem::PAY,
            AiError::TooMany => problem::TOO_MANY,
            AiError::NoKey => problem::NO_KEY,
            AiError::BadKey => problem::BAD_KEY,
            AiError::Failed(_) => problem::FAILED,
        }
    }
}

/// Rephrases `text` in tone `tone` (a [`Tone`] id); `instruction` is the
/// user's own for the custom tone.
pub async fn rephrase(
    settings: &Ai,
    secrets: &Secrets,
    text: &str,
    tone: &str,
    instruction: &str,
) -> Result<AiAnswer, AiError> {
    let tone = Tone::parse(tone).ok_or_else(|| AiError::Failed(format!("no tone {tone:?}")))?;
    let prompt = prompt::rephrase(text, tone, instruction)
        .ok_or_else(|| AiError::Failed("nothing to rephrase".into()))?;
    let answer = match settings.source {
        AiSource::Off => return Err(AiError::Off),
        AiSource::Katna => {
            let request = RephraseRequest {
                text: text.to_owned(),
                tone: tone.id().to_owned(),
                instruction: instruction.to_owned(),
            };
            katna("/api/v1/ai/rephrase", &request, secrets, REPHRASE_TIMEOUT).await?
        }
        AiSource::Own => own(settings, secrets, &prompt, REPHRASE_TIMEOUT).await?,
    };
    let text = prompt::clean_rephrase(text, &answer.text)
        .ok_or_else(|| AiError::Failed("the answer was empty".into()))?;
    tracing::info!(tone = tone.id(), "text rephrased");
    Ok(AiAnswer { text, ..answer })
}

/// The rest of the sentence at the end of `before`, the paragraph being
/// written; `answered` is the mail being answered, empty unless the user
/// allows sending it. An empty text when the service is unsure.
pub async fn complete(
    settings: &Ai,
    secrets: &Secrets,
    before: &str,
    answered: &str,
) -> Result<AiAnswer, AiError> {
    if !settings.autocomplete {
        return Err(AiError::Off);
    }
    let answered = if settings.autocomplete_answered {
        answered
    } else {
        ""
    };
    let Some(prompt) = prompt::complete(before, answered) else {
        return Ok(AiAnswer {
            text: String::new(),
            plan: Plan::default(),
        });
    };
    let asked = std::time::Instant::now();
    let answer = match settings.source {
        AiSource::Off => return Err(AiError::Off),
        AiSource::Katna => {
            let request = CompleteRequest {
                before: prompt::cut_start(before, prompt::MAX_BEFORE).to_owned(),
                answered: answered.to_owned(),
            };
            katna("/api/v1/ai/complete", &request, secrets, COMPLETE_TIMEOUT).await?
        }
        AiSource::Own => own(settings, secrets, &prompt, COMPLETE_TIMEOUT).await?,
    };
    let text = prompt::clean_completion(before, &answer.text).unwrap_or_default();
    tracing::info!(
        ms = asked.elapsed().as_millis() as u64,
        empty = text.is_empty(),
        "sentence finished"
    );
    Ok(AiAnswer { text, ..answer })
}

/// TLS for the AI services, set up once: loading the system's
/// certificates for every suggestion would slow each one down.
fn tls() -> Result<Tls, AiError> {
    static TLS: std::sync::OnceLock<Result<Tls, String>> = std::sync::OnceLock::new();
    TLS.get_or_init(|| Tls::system().map_err(|err| err.to_string()))
        .clone()
        .map_err(|err| AiError::Failed(format!("TLS: {err}")))
}

/// Asks Katna AI on Katna Server.
async fn katna(
    path: &str,
    request: &impl serde::Serialize,
    secrets: &Secrets,
    timeout: Duration,
) -> Result<AiAnswer, AiError> {
    let base = katna_account::server_url();
    if base.is_empty() {
        return Err(AiError::Off);
    }
    let token = Session::new(secrets)
        .map_err(|err| AiError::Failed(err.to_string()))?
        .token()
        .await
        .map_err(|err| AiError::Failed(err.to_string()))?
        .filter(|token| !token.is_empty())
        .ok_or(AiError::SignIn)?;
    let tls = tls()?;
    let body = serde_json::to_vec(request).map_err(|err| AiError::Failed(err.to_string()))?;
    let authorization = format!("Bearer {token}");
    let (status, answer) = http::request(
        "POST",
        &format!("{base}{path}"),
        &[("Authorization", &authorization)],
        Some(&body),
        &tls,
        timeout,
    )
    .await
    .map_err(|err| AiError::Failed(err.to_string()))?;
    match status {
        200 => serde_json::from_slice(&answer)
            .map_err(|err| AiError::Failed(format!("Katna AI answer: {err}"))),
        401 | 403 => Err(AiError::SignIn),
        402 => Err(AiError::Pay),
        429 => Err(AiError::TooMany),
        404 => Err(AiError::Failed("Katna AI is not on this server yet".into())),
        status => Err(AiError::Failed(format!("Katna AI: HTTP {status}"))),
    }
}

/// Asks the user's own service.
async fn own(
    settings: &Ai,
    secrets: &Secrets,
    prompt: &Prompt,
    timeout: Duration,
) -> Result<AiAnswer, AiError> {
    let (preset, base, model) = own_service(settings);
    if base.is_empty() || model.is_empty() {
        return Err(AiError::Failed("no service address or model".into()));
    }
    let key = secrets
        .ai_key()
        .await
        .map_err(|err| AiError::Failed(err.to_string()))?
        .unwrap_or_default();
    if key.is_empty() && preset.needs_key {
        return Err(AiError::NoKey);
    }
    let call = provider::call(preset.kind, &base, &model, &key, prompt);
    let tls = tls()?;
    let headers: Vec<(&str, &str)> = call
        .headers
        .iter()
        .map(|(name, value)| (*name, value.as_str()))
        .collect();
    let (status, body) =
        http::request("POST", &call.url, &headers, Some(&call.body), &tls, timeout)
            .await
            .map_err(|err| AiError::Failed(err.to_string()))?;
    let text = provider::answer(preset.kind, status, &body).map_err(|err| match err {
        ProviderError::Key => AiError::BadKey,
        ProviderError::TooMany => AiError::TooMany,
        ProviderError::Failed(err) => AiError::Failed(err),
    })?;
    Ok(AiAnswer {
        text,
        plan: Plan {
            kind: OWN.to_owned(),
            days_left: None,
        },
    })
}

/// The models the user's own service `provider` (at `address` for
/// [`OTHER`]) offers to the saved key.
pub async fn models(
    secrets: &Secrets,
    provider: &str,
    address: &str,
) -> Result<Vec<String>, AiError> {
    let preset = provider::preset(provider);
    let base = if preset.id == OTHER {
        address.trim().to_owned()
    } else {
        preset.base.to_owned()
    };
    if base.is_empty() {
        return Err(AiError::Failed("no service address".into()));
    }
    let key = secrets
        .ai_key()
        .await
        .map_err(|err| AiError::Failed(err.to_string()))?
        .unwrap_or_default();
    if key.is_empty() && preset.needs_key {
        return Err(AiError::NoKey);
    }
    let call = provider::models_call(preset.kind, &base, &key);
    let tls = tls()?;
    let headers: Vec<(&str, &str)> = call
        .headers
        .iter()
        .map(|(name, value)| (*name, value.as_str()))
        .collect();
    let (status, body) = http::request("GET", &call.url, &headers, None, &tls, MODELS_TIMEOUT)
        .await
        .map_err(|err| AiError::Failed(err.to_string()))?;
    provider::models(preset.kind, status, &body).map_err(|err| match err {
        ProviderError::Key => AiError::BadKey,
        ProviderError::TooMany => AiError::TooMany,
        ProviderError::Failed(err) => AiError::Failed(err),
    })
}

/// How long listing a service's models may take.
const MODELS_TIMEOUT: Duration = Duration::from_secs(15);

/// The user's own service: its preset, address and model.
fn own_service(settings: &Ai) -> (&'static provider::Preset, String, String) {
    let preset = provider::preset(&settings.provider);
    let base = if preset.id == OTHER {
        settings.address.trim().to_owned()
    } else {
        preset.base.to_owned()
    };
    let model = match settings.model.trim() {
        "" => preset.model.to_owned(),
        model => model.to_owned(),
    };
    (preset, base, model)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn own_service_fills_in_the_usual_model() {
        let mut settings = Ai {
            source: AiSource::Own,
            provider: "mistral".into(),
            ..Ai::default()
        };
        let (preset, base, model) = own_service(&settings);
        assert_eq!(preset.id, "mistral");
        assert_eq!(base, "https://api.mistral.ai/v1");
        assert_eq!(model, "mistral-small-latest");
        settings.provider = OTHER.into();
        settings.address = " http://localhost:11434/v1 ".into();
        settings.model = "llama3.2".into();
        let (preset, base, model) = own_service(&settings);
        assert!(!preset.needs_key);
        assert_eq!(base, "http://localhost:11434/v1");
        assert_eq!(model, "llama3.2");
    }

    #[test]
    fn off_and_missing_keys_send_nothing() {
        let secrets = Secrets::memory();
        let off = Ai {
            source: AiSource::Off,
            ..Ai::default()
        };
        let done =
            futures_lite::future::block_on(rephrase(&off, &secrets, "hi there", "clearer", ""));
        assert_eq!(done, Err(AiError::Off));
        let own = Ai {
            source: AiSource::Own,
            autocomplete: true,
            ..Ai::default()
        };
        let done =
            futures_lite::future::block_on(rephrase(&own, &secrets, "hi there", "clearer", ""));
        assert_eq!(done, Err(AiError::NoKey));
        // Autocomplete is off unless switched on.
        let done = futures_lite::future::block_on(complete(&off, &secrets, "I will send", ""));
        assert_eq!(done, Err(AiError::Off));
        let done = futures_lite::future::block_on(complete(&own, &secrets, "I will send", ""));
        assert_eq!(done, Err(AiError::NoKey));
    }
}
