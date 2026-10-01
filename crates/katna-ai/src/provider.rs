// SPDX-License-Identifier: GPL-3.0-or-later

//! The AI services a user can bring a key for, and the one HTTPS request
//! each takes. Most speak OpenAI's chat completions; Gemini and Claude
//! have their own. Nothing here sends anything: [`call`] says what to
//! send and [`answer`] reads what came back.

use serde_json::{Value, json};

use crate::prompt::Prompt;

/// The kind of API a service speaks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// OpenAI's `/chat/completions`, also spoken by Mistral, DeepSeek,
    /// OpenRouter, Ollama and LM Studio.
    OpenAi,
    /// Google's Gemini API (`generateContent`).
    Gemini,
    /// Anthropic's Claude API (`/v1/messages`).
    Anthropic,
}

/// A service in the "Your own key" list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Preset {
    /// What the settings file keeps.
    pub id: &'static str,
    /// Its name, a brand: not translated.
    pub name: &'static str,
    pub kind: Kind,
    /// Where its API is; empty for [`OTHER`], whose address the user gives.
    pub base: &'static str,
    /// The model used unless the user names another.
    pub model: &'static str,
    /// Whether it needs a key; a model on this computer does not.
    pub needs_key: bool,
}

/// The id of a service the user gives the address of: anything that
/// speaks OpenAI's API, such as Ollama (`http://localhost:11434/v1`) or LM
/// Studio (`http://localhost:1234/v1`).
pub const OTHER: &str = "other";

/// The services, in the order the settings list them.
pub const PRESETS: &[Preset] = &[
    Preset {
        id: "gemini",
        name: "Google Gemini",
        kind: Kind::Gemini,
        base: "https://generativelanguage.googleapis.com",
        model: "gemini-2.5-flash-lite",
        needs_key: true,
    },
    Preset {
        id: "openai",
        name: "OpenAI",
        kind: Kind::OpenAi,
        base: "https://api.openai.com/v1",
        model: "gpt-5-mini",
        needs_key: true,
    },
    Preset {
        id: "anthropic",
        name: "Claude",
        kind: Kind::Anthropic,
        base: "https://api.anthropic.com",
        model: "claude-haiku-4-5",
        needs_key: true,
    },
    Preset {
        id: "mistral",
        name: "Mistral",
        kind: Kind::OpenAi,
        base: "https://api.mistral.ai/v1",
        model: "mistral-small-latest",
        needs_key: true,
    },
    Preset {
        id: "deepseek",
        name: "DeepSeek",
        kind: Kind::OpenAi,
        base: "https://api.deepseek.com",
        model: "deepseek-chat",
        needs_key: true,
    },
    Preset {
        id: "openrouter",
        name: "OpenRouter",
        kind: Kind::OpenAi,
        base: "https://openrouter.ai/api/v1",
        model: "google/gemini-2.5-flash-lite",
        needs_key: true,
    },
    Preset {
        id: OTHER,
        name: "",
        kind: Kind::OpenAi,
        base: "",
        model: "",
        needs_key: false,
    },
];

/// The service named `id` in the settings; the first when unknown.
pub fn preset(id: &str) -> &'static Preset {
    PRESETS.iter().find(|p| p.id == id).unwrap_or(&PRESETS[0])
}

/// One HTTPS request to send.
#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub url: String,
    pub headers: Vec<(&'static str, String)>,
    pub body: Vec<u8>,
}

/// Why a service gave no answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderError {
    /// It refused the key (401 or 403).
    Key,
    /// Over its limits, or out of credit, for now (429).
    TooMany,
    /// Any other failure, as the service told it.
    Failed(String),
}

/// The request asking `prompt` of a service of `kind` at `base` (no
/// trailing `/`), with `model` and `key` (empty for none).
pub fn call(kind: Kind, base: &str, model: &str, key: &str, prompt: &Prompt) -> Call {
    let base = base.trim().trim_end_matches('/');
    let mut headers = Vec::new();
    let (url, body) = match kind {
        Kind::OpenAi => {
            if !key.is_empty() {
                headers.push(("Authorization", format!("Bearer {key}")));
            }
            let mut body = json!({
                "model": model,
                "messages": [
                    {"role": "system", "content": prompt.system},
                    {"role": "user", "content": prompt.user},
                ],
            });
            if base == "https://api.openai.com/v1" && reasons(model) {
                // Its reasoning models: as little thinking as allowed (it
                // counts against the limit), and no temperature.
                body["reasoning_effort"] = json!("minimal");
                body["max_completion_tokens"] = json!(prompt.max_tokens.max(512));
            } else {
                body["max_tokens"] = json!(prompt.max_tokens);
                body["temperature"] = json!(prompt.temperature);
            }
            (format!("{base}/chat/completions"), body)
        }
        Kind::Gemini => {
            headers.push(("x-goog-api-key", key.to_owned()));
            let mut config = json!({
                "maxOutputTokens": prompt.max_tokens,
                "temperature": prompt.temperature,
            });
            if model.contains("2.5") {
                config["thinkingConfig"] = json!({"thinkingBudget": 0});
            } else if model.starts_with("gemini-3") {
                config["thinkingConfig"] = json!({"thinkingLevel": "low"});
                config["maxOutputTokens"] = json!(prompt.max_tokens.max(512));
            }
            let body = json!({
                "systemInstruction": {"parts": [{"text": prompt.system}]},
                "contents": [{"role": "user", "parts": [{"text": prompt.user}]}],
                "generationConfig": config,
            });
            (
                format!("{base}/v1beta/models/{model}:generateContent"),
                body,
            )
        }
        Kind::Anthropic => {
            headers.push(("x-api-key", key.to_owned()));
            headers.push(("anthropic-version", "2023-06-01".to_owned()));
            let body = json!({
                "model": model,
                "max_tokens": prompt.max_tokens,
                "temperature": prompt.temperature,
                "system": prompt.system,
                "messages": [{"role": "user", "content": prompt.user}],
            });
            (format!("{base}/v1/messages"), body)
        }
    };
    Call {
        url,
        headers,
        body: serde_json::to_vec(&body).unwrap_or_default(),
    }
}

/// Whether an OpenAI model thinks before answering (GPT-5 and o-series).
fn reasons(model: &str) -> bool {
    model.starts_with("gpt-5") || model.starts_with('o')
}

/// The text of a service's answer with `status` and `body`.
pub fn answer(kind: Kind, status: u16, body: &[u8]) -> Result<String, ProviderError> {
    match status {
        200 => {}
        401 | 403 => return Err(ProviderError::Key),
        429 => return Err(ProviderError::TooMany),
        status => {
            let message = serde_json::from_slice::<Value>(body)
                .ok()
                .and_then(|v| error_message(&v))
                .unwrap_or_default();
            return Err(ProviderError::Failed(format!("HTTP {status} {message}")));
        }
    }
    let value: Value = serde_json::from_slice(body)
        .map_err(|err| ProviderError::Failed(format!("answer: {err}")))?;
    let text = match kind {
        Kind::OpenAi => value["choices"][0]["message"]["content"]
            .as_str()
            .map(str::to_owned),
        Kind::Gemini => value["candidates"][0]["content"]["parts"]
            .as_array()
            .map(|parts| {
                parts
                    .iter()
                    // Thought summaries are not the answer.
                    .filter(|p| !p["thought"].as_bool().unwrap_or(false))
                    .filter_map(|p| p["text"].as_str())
                    .collect::<String>()
            }),
        Kind::Anthropic => value["content"].as_array().map(|blocks| {
            blocks
                .iter()
                .filter(|b| b["type"] == "text")
                .filter_map(|b| b["text"].as_str())
                .collect::<String>()
        }),
    };
    // A model that answers nothing is unsure; that is an answer too.
    Ok(text.unwrap_or_default())
}

fn error_message(value: &Value) -> Option<String> {
    let error = &value["error"];
    let message = error["message"].as_str().or_else(|| error.as_str())?;
    Some(message.chars().take(200).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prompt() -> Prompt {
        Prompt {
            system: "sys".into(),
            user: "hi".into(),
            max_tokens: 40,
            temperature: 0.2,
        }
    }

    fn body(call: &Call) -> Value {
        serde_json::from_slice(&call.body).unwrap()
    }

    #[test]
    fn presets_are_unique_and_found() {
        for p in PRESETS {
            assert_eq!(preset(p.id), p);
        }
        assert_eq!(preset("nope").id, "gemini");
    }

    #[test]
    fn openai_compatible_requests() {
        let c = call(
            Kind::OpenAi,
            "https://api.mistral.ai/v1/",
            "m",
            "k",
            &prompt(),
        );
        assert_eq!(c.url, "https://api.mistral.ai/v1/chat/completions");
        assert_eq!(c.headers, vec![("Authorization", "Bearer k".to_owned())]);
        let b = body(&c);
        assert_eq!(b["max_tokens"], 40);
        assert_eq!(b["messages"][1]["content"], "hi");
        // A local model needs no key.
        let c = call(
            Kind::OpenAi,
            "http://localhost:11434/v1",
            "llama",
            "",
            &prompt(),
        );
        assert!(c.headers.is_empty());
        // OpenAI's own reasoning models think as little as they may.
        let b = body(&call(
            Kind::OpenAi,
            "https://api.openai.com/v1",
            "gpt-5-mini",
            "k",
            &prompt(),
        ));
        assert_eq!(b["reasoning_effort"], "minimal");
        assert!(b.get("temperature").is_none());
    }

    #[test]
    fn gemini_and_claude_requests() {
        let c = call(
            Kind::Gemini,
            "https://generativelanguage.googleapis.com",
            "gemini-2.5-flash-lite",
            "k",
            &prompt(),
        );
        assert!(
            c.url
                .ends_with("/v1beta/models/gemini-2.5-flash-lite:generateContent")
        );
        assert_eq!(
            body(&c)["generationConfig"]["thinkingConfig"]["thinkingBudget"],
            0
        );
        let c = call(
            Kind::Anthropic,
            "https://api.anthropic.com",
            "claude-haiku-4-5",
            "k",
            &prompt(),
        );
        assert_eq!(c.url, "https://api.anthropic.com/v1/messages");
        assert_eq!(body(&c)["system"], "sys");
    }

    #[test]
    fn answers_are_read() {
        let openai = br#"{"choices":[{"message":{"content":"Hello"}}]}"#;
        assert_eq!(answer(Kind::OpenAi, 200, openai).unwrap(), "Hello");
        let gemini = br#"{"candidates":[{"content":{"parts":[{"text":"a","thought":true},{"text":"Hi"}]}}]}"#;
        assert_eq!(answer(Kind::Gemini, 200, gemini).unwrap(), "Hi");
        let claude = br#"{"content":[{"type":"text","text":"Yo"}]}"#;
        assert_eq!(answer(Kind::Anthropic, 200, claude).unwrap(), "Yo");
        assert_eq!(answer(Kind::OpenAi, 401, b"{}"), Err(ProviderError::Key));
        assert_eq!(answer(Kind::Gemini, 429, b""), Err(ProviderError::TooMany));
        let failed = answer(Kind::OpenAi, 400, br#"{"error":{"message":"bad model"}}"#);
        assert_eq!(
            failed,
            Err(ProviderError::Failed("HTTP 400 bad model".into()))
        );
    }
}
