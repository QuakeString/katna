// SPDX-License-Identifier: GPL-3.0-or-later

//! Automatic translation (`docs/ARCHITECTURE.md` §16.3): the plain text of
//! a message the user asked to translate goes to LibreTranslate behind
//! Katna Server, and the answer is kept in the store.
//!
//! Only text goes: the app sends the message's text (HTML already made
//! plain, quotes and signature kept), never its attachments, headers or
//! addresses. Its language is found here first, so mail already in the
//! reading language never leaves the computer.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use katna_store::Translation;
use katna_translate::{ServerLanguage, TranslateRequest, TranslateResponse};

/// How long the server's list of languages is trusted.
const LANGUAGES_FOR: Duration = Duration::from_secs(6 * 3600);

/// Why a translation could not be made.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TranslateError {
    /// Translation is turned off in this build (no server).
    #[error("translation is turned off")]
    Off,
    /// The text is already in the language asked for.
    #[error("the message is already in that language")]
    SameLanguage,
    /// The server cannot translate between these languages.
    #[error("the server cannot translate {0} into {1}")]
    Unsupported(String, String),
    /// Over the server's limit for now.
    #[error("too many translations for now; try again later")]
    TooMany,
    /// The server refused this install; signing in may be needed.
    #[error("the server did not accept this computer")]
    Refused,
    /// The server could not be reached or answered badly.
    #[error("{0}")]
    Server(String),
}

/// Katna Server as seen by translation: one authorized JSON request.
/// The daemon's is HTTPS with the install's token; tests use a fake.
pub trait Server {
    /// Sends `body` (none when empty) to `path` (such as
    /// `/api/v1/translate`) and returns the status and body of the answer.
    fn call(
        &self,
        method: &str,
        path: &str,
        body: &[u8],
    ) -> impl Future<Output = Result<(u16, Vec<u8>), String>>;
}

/// Each language the server knows, with those it translates it into.
type Known = HashMap<String, Vec<String>>;

/// The languages the server translates between, kept for a while.
#[derive(Default)]
pub struct Languages {
    known: Mutex<Option<(Instant, Known)>>,
}

impl Languages {
    /// The languages `server` can translate into `target`.
    pub async fn sources(
        &self,
        server: &impl Server,
        target: &str,
    ) -> Result<Vec<String>, TranslateError> {
        let known = self.all(server).await?;
        let mut sources: Vec<String> = known
            .iter()
            .filter(|(code, targets)| {
                code.as_str() != target && targets.iter().any(|t| t == target)
            })
            .map(|(code, _)| code.clone())
            .collect();
        sources.sort();
        Ok(sources)
    }

    async fn all(&self, server: &impl Server) -> Result<Known, TranslateError> {
        if let Some((at, known)) = &*self.known.lock().unwrap()
            && at.elapsed() < LANGUAGES_FOR
        {
            return Ok(known.clone());
        }
        let body = answer(server.call("GET", "/api/v1/languages", &[]).await)?;
        let list: Vec<ServerLanguage> = serde_json::from_slice(&body)
            .map_err(|err| TranslateError::Server(format!("languages: {err}")))?;
        let known: Known = list
            .into_iter()
            .map(|language| (language.code, language.targets))
            .collect();
        *self.known.lock().unwrap() = Some((Instant::now(), known.clone()));
        Ok(known)
    }

    /// Whether `server` translates `source` into `target`, when known.
    async fn translates(&self, server: &impl Server, source: &str, target: &str) -> bool {
        match self.all(server).await {
            Ok(known) => known
                .get(source)
                .is_some_and(|targets| targets.iter().any(|t| t == target)),
            // Asking will tell.
            Err(_) => true,
        }
    }
}

/// Translates `text` into `target`: the language found in it, and the
/// translation. Nothing is sent when `text` is already in `target`.
pub async fn translate(
    server: &impl Server,
    languages: &Languages,
    text: &str,
    target: &str,
) -> Result<Translation, TranslateError> {
    let source = katna_translate::detect(text);
    if let Some(source) = source {
        if katna_translate::same_language(source, target) {
            return Err(TranslateError::SameLanguage);
        }
        if !languages.translates(server, source, target).await {
            return Err(TranslateError::Unsupported(source.into(), target.into()));
        }
    }
    let source = source.unwrap_or("auto");
    let mut translated = String::with_capacity(text.len());
    for piece in katna_translate::pieces(text, katna_translate::MAX_PIECE) {
        // Blank lines and spaces around a piece are kept as they were:
        // translation trims them.
        let body = piece.trim();
        if body.is_empty() {
            translated.push_str(piece);
            continue;
        }
        let request = serde_json::to_vec(&TranslateRequest::new(body, source, target))
            .map_err(|err| TranslateError::Server(err.to_string()))?;
        let answer = answer(server.call("POST", "/api/v1/translate", &request).await)?;
        let answer: TranslateResponse = serde_json::from_slice(&answer)
            .map_err(|err| TranslateError::Server(format!("translate: {err}")))?;
        translated.push_str(&piece[..piece.len() - piece.trim_start().len()]);
        translated.push_str(&answer.translated_text);
        translated.push_str(&piece[piece.trim_end().len()..]);
    }
    Ok(Translation {
        source: source.to_owned(),
        text: translated,
    })
}

/// The body of a `200` answer, or why there is none.
fn answer(result: Result<(u16, Vec<u8>), String>) -> Result<Vec<u8>, TranslateError> {
    match result {
        Ok((200, body)) => Ok(body),
        Ok((401 | 403, _)) => Err(TranslateError::Refused),
        Ok((429, _)) => Err(TranslateError::TooMany),
        Ok((status, _)) => Err(TranslateError::Server(format!(
            "the server answered {status}"
        ))),
        Err(err) => Err(TranslateError::Server(err)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    /// LibreTranslate behind Katna Server, faked: it "translates" by
    /// putting the target's code in front of each piece.
    #[derive(Default)]
    struct Fake {
        calls: RefCell<Vec<(String, String)>>,
        status: u16,
    }

    impl Fake {
        fn ok() -> Self {
            Self {
                status: 200,
                ..Self::default()
            }
        }
    }

    impl Server for Fake {
        async fn call(
            &self,
            method: &str,
            path: &str,
            body: &[u8],
        ) -> Result<(u16, Vec<u8>), String> {
            let body = String::from_utf8(body.to_vec()).unwrap();
            self.calls
                .borrow_mut()
                .push((format!("{method} {path}"), body.clone()));
            if self.status != 200 {
                return Ok((self.status, b"{\"error\":\"no\"}".to_vec()));
            }
            Ok(match path {
                "/api/v1/languages" => (
                    200,
                    br#"[{"code":"en","targets":["es","de"]},
                         {"code":"es","targets":["en"]},
                         {"code":"de","targets":["en"]}]"#
                        .to_vec(),
                ),
                "/api/v1/translate" => {
                    let request: TranslateRequest = serde_json::from_str(&body).unwrap();
                    assert_eq!(request.format, "text");
                    let answer = TranslateResponse {
                        translated_text: format!("[{}] {}", request.target, request.q),
                    };
                    (200, serde_json::to_vec(&answer).unwrap())
                }
                _ => (404, Vec::new()),
            })
        }
    }

    const SPANISH: &str = "Hola Ana, gracias por tu mensaje. Nos vemos el martes en la \
                           oficina para hablar del nuevo proyecto.";

    #[test]
    fn translates_only_the_text() {
        smol::block_on(async {
            let server = Fake::ok();
            let languages = Languages::default();
            let text = format!("\n{SPANISH}\n\n> Cita\n");
            let done = translate(&server, &languages, &text, "en").await.unwrap();
            assert_eq!(done.source, "es");
            assert_eq!(done.text, format!("\n[en] {SPANISH}\n\n> Cita\n"));
            let calls = server.calls.borrow();
            assert_eq!(calls[0].0, "GET /api/v1/languages");
            assert_eq!(calls[1].0, "POST /api/v1/translate");
            assert!(calls[1].1.contains(r#""source":"es""#));
            assert!(calls[1].1.contains(r#""target":"en""#));
        });
    }

    #[test]
    fn nothing_goes_for_mail_in_the_reading_language() {
        smol::block_on(async {
            let server = Fake::ok();
            let english = "Hi Sam, thanks for the notes from the meeting yesterday. I will \
                           send the plan to the whole team before Friday.";
            let got = translate(&server, &Languages::default(), english, "en").await;
            assert_eq!(got, Err(TranslateError::SameLanguage));
            assert!(server.calls.borrow().is_empty());
        });
    }

    #[test]
    fn languages_the_server_lacks_are_not_sent() {
        smol::block_on(async {
            let server = Fake::ok();
            let languages = Languages::default();
            let got = translate(&server, &languages, SPANISH, "de").await;
            assert_eq!(
                got,
                Err(TranslateError::Unsupported("es".into(), "de".into()))
            );
            assert_eq!(server.calls.borrow().len(), 1);
            assert_eq!(
                languages.sources(&server, "en").await.unwrap(),
                ["de", "es"]
            );
            // The list was kept.
            assert_eq!(server.calls.borrow().len(), 1);
        });
    }

    #[test]
    fn long_mail_goes_in_pieces() {
        smol::block_on(async {
            let server = Fake::ok();
            let paragraph = format!("{SPANISH}\n\n");
            let text = paragraph.repeat(katna_translate::MAX_PIECE / paragraph.len() * 2 + 2);
            let done = translate(&server, &Languages::default(), &text, "en")
                .await
                .unwrap();
            let posts = server
                .calls
                .borrow()
                .iter()
                .filter(|(call, _)| call.starts_with("POST"))
                .count();
            assert!(posts >= 3, "{posts} requests");
            assert_eq!(done.text.matches("[en] ").count(), posts);
        });
    }

    #[test]
    fn refusals_say_why() {
        smol::block_on(async {
            for (status, error) in [
                (401, TranslateError::Refused),
                (429, TranslateError::TooMany),
                (
                    502,
                    TranslateError::Server("the server answered 502".into()),
                ),
            ] {
                let server = Fake {
                    status,
                    ..Fake::default()
                };
                let got = translate(&server, &Languages::default(), SPANISH, "en").await;
                assert_eq!(got, Err(error));
            }
        });
    }
}
