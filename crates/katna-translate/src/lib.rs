// SPDX-License-Identifier: GPL-3.0-or-later

//! Automatic translation (`docs/ARCHITECTURE.md` §16.4, plan 7.8).
//!
//! Katna Mail offers to translate a message that is not in the reading
//! language. Which language a message is in is found here, on this
//! computer ([`detect`]), so nothing is sent for mail the user can already
//! read. When the user asks (or chose to always translate that language),
//! `katna-daemon` sends the message's plain text, in [`pieces`], to
//! LibreTranslate behind Katna Server and keeps the result in the store.
//!
//! Languages are named by LibreTranslate's codes: ISO 639-1 (`es`), with
//! `zh` for simplified and `zt` for traditional Chinese.

use serde::{Deserialize, Serialize};

/// Longest piece of text sent in one request, in characters.
pub const MAX_PIECE: usize = 4000;

/// Most characters of one message translated; the rest is left out, as
/// the reading pane leaves out the end of very long mail.
pub const MAX_TEXT: usize = 40_000;

/// Characters looked at to find a message's language.
const DETECT_CHARS: usize = 2000;

/// Fewest letters a message needs before its language is guessed.
const MIN_LETTERS: usize = 20;

/// The language `text` is written in, as a LibreTranslate code, when that
/// is clear. Quoted lines (`>`), links and addresses are left out, so a
/// reply in one language to mail in another counts as the reply's.
pub fn detect(text: &str) -> Option<&'static str> {
    let own = own_words(text);
    if own.chars().filter(|c| c.is_alphabetic()).count() < MIN_LETTERS {
        return None;
    }
    let info = whatlang::detect(&own)?;
    if !info.is_reliable() {
        return None;
    }
    code_of(info.lang())
}

/// The text of `text` a language is judged from: its own lines, without
/// quotes, links or addresses, at most [`DETECT_CHARS`] characters.
fn own_words(text: &str) -> String {
    let mut own = String::new();
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('>') {
            continue;
        }
        for word in line.split_whitespace() {
            if word.contains("://") || word.contains('@') || word.starts_with("www.") {
                continue;
            }
            own.push_str(word);
            own.push(' ');
        }
        own.push('\n');
        if own.len() >= DETECT_CHARS * 4 {
            break;
        }
    }
    own.chars().take(DETECT_CHARS).collect()
}

/// LibreTranslate's code for a language whatlang finds.
fn code_of(lang: whatlang::Lang) -> Option<&'static str> {
    Some(match lang.code() {
        "afr" => "af",
        "amh" => "am",
        "ara" => "ar",
        "aze" => "az",
        "bel" => "be",
        "ben" => "bn",
        "bul" => "bg",
        "cat" => "ca",
        "ces" => "cs",
        "cmn" => "zh",
        "dan" => "da",
        "deu" => "de",
        "ell" => "el",
        "eng" => "en",
        "epo" => "eo",
        "est" => "et",
        "fin" => "fi",
        "fra" => "fr",
        "guj" => "gu",
        "heb" => "he",
        "hin" => "hi",
        "hrv" => "hr",
        "hun" => "hu",
        "hye" => "hy",
        "ind" => "id",
        "ita" => "it",
        "jav" => "jv",
        "jpn" => "ja",
        "kan" => "kn",
        "kat" => "ka",
        "khm" => "km",
        "kor" => "ko",
        "lat" => "la",
        "lav" => "lv",
        "lit" => "lt",
        "mal" => "ml",
        "mar" => "mr",
        "mkd" => "mk",
        "mya" => "my",
        "nep" => "ne",
        "nld" => "nl",
        "nob" => "nb",
        "ori" => "or",
        "pan" => "pa",
        "pes" => "fa",
        "pol" => "pl",
        "por" => "pt",
        "ron" => "ro",
        "rus" => "ru",
        "sin" => "si",
        "slk" => "sk",
        "slv" => "sl",
        "sna" => "sn",
        "spa" => "es",
        "srp" => "sr",
        "swe" => "sv",
        "tam" => "ta",
        "tel" => "te",
        "tgl" => "tl",
        "tha" => "th",
        "tuk" => "tk",
        "tur" => "tr",
        "ukr" => "uk",
        "urd" => "ur",
        "uzb" => "uz",
        "vie" => "vi",
        "yid" => "yi",
        "zul" => "zu",
        _ => return None,
    })
}

/// The LibreTranslate code of a Katna language tag (`general.language`,
/// such as `en-IN`, `zh-Hant` or `pt-BR`).
pub fn code_for_tag(tag: &str) -> String {
    let mut parts = tag.split(['-', '_']);
    let base = parts.next().unwrap_or_default().to_ascii_lowercase();
    let rest: Vec<String> = parts.map(str::to_ascii_lowercase).collect();
    match base.as_str() {
        "zh" if rest
            .iter()
            .any(|p| matches!(p.as_str(), "hant" | "tw" | "hk" | "mo")) =>
        {
            "zt".into()
        }
        "fil" => "tl".into(),
        "no" | "nn" => "nb".into(),
        _ => base,
    }
}

/// Whether mail in language `mail` can be read by someone reading
/// `reading`: the same language, counting both Chinese scripts as one.
pub fn same_language(mail: &str, reading: &str) -> bool {
    let chinese = |code: &str| matches!(code, "zh" | "zt");
    mail == reading || (chinese(mail) && chinese(reading))
}

/// The name of language `code` in that language (`Español` for `es`),
/// or `None` for a code [`detect`] never gives.
pub fn native_name(code: &str) -> Option<&'static str> {
    whatlang::Lang::all()
        .iter()
        .copied()
        .find(|lang| code_of(*lang) == Some(code))
        .map(|lang| lang.name())
}

/// `code` as the same code with a `'static` lifetime, when [`detect`] can
/// give it.
pub fn known_code(code: &str) -> Option<&'static str> {
    whatlang::Lang::all()
        .iter()
        .find_map(|lang| code_of(*lang).filter(|known| *known == code))
}

/// `text` cut into pieces of at most `max` characters, at paragraph
/// breaks where it can, else at line breaks, sentence ends or spaces. At
/// most [`MAX_TEXT`] characters go in all. Joined back with nothing
/// between them, the pieces are that text.
pub fn pieces(text: &str, max: usize) -> Vec<&str> {
    let max = max.max(1);
    let end = text
        .char_indices()
        .nth(MAX_TEXT)
        .map_or(text.len(), |(at, _)| at);
    let mut rest = &text[..end];
    let mut pieces = Vec::new();
    while !rest.is_empty() {
        let Some((limit, _)) = rest.char_indices().nth(max) else {
            pieces.push(rest);
            break;
        };
        let window = &rest[..limit];
        let cut = ["\n\n", "\n", ". ", "。", " "]
            .iter()
            .find_map(|sep| {
                window
                    .rfind(sep)
                    .map(|at| at + sep.len())
                    .filter(|&at| at > limit / 2)
            })
            .unwrap_or(limit);
        pieces.push(&rest[..cut]);
        rest = &rest[cut..];
    }
    pieces
}

/// A request for `POST /api/v1/translate` on Katna Server, which passes
/// it to LibreTranslate as it is.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranslateRequest {
    /// The text.
    pub q: String,
    /// Its language, or `auto`.
    pub source: String,
    /// The language wanted.
    pub target: String,
    /// Always `text`: Katna sends plain text only.
    pub format: String,
}

impl TranslateRequest {
    /// Plain `text` from `source` into `target`.
    pub fn new(text: &str, source: &str, target: &str) -> Self {
        Self {
            q: text.to_owned(),
            source: source.to_owned(),
            target: target.to_owned(),
            format: "text".to_owned(),
        }
    }
}

/// LibreTranslate's answer to a [`TranslateRequest`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranslateResponse {
    /// The translated text.
    #[serde(rename = "translatedText")]
    pub translated_text: String,
}

/// One language of LibreTranslate's `GET /languages`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerLanguage {
    /// Its code.
    pub code: String,
    /// The codes it can be translated into.
    #[serde(default)]
    pub targets: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_clear_languages() {
        let spanish = "Hola Ana, gracias por tu mensaje. Nos vemos el martes en la oficina \
                       para hablar del nuevo proyecto y de las fechas de entrega.";
        assert_eq!(detect(spanish), Some("es"));
        let german = "Sehr geehrte Damen und Herren, anbei erhalten Sie die Rechnung für \
                      den vergangenen Monat. Bitte überweisen Sie den Betrag bis Freitag.";
        assert_eq!(detect(german), Some("de"));
        let english = "Hi Sam, thanks for the notes from yesterday's meeting. I will send \
                       the updated plan to the whole team before Friday.";
        assert_eq!(detect(english), Some("en"));
        let bengali = "আপনার চিঠির জন্য অনেক ধন্যবাদ। আমরা আগামী সপ্তাহে আবার দেখা করব এবং \
                       প্রকল্পের বিষয়ে কথা বলব।";
        assert_eq!(detect(bengali), Some("bn"));
    }

    #[test]
    fn short_or_unclear_text_has_no_language() {
        assert_eq!(detect(""), None);
        assert_eq!(detect("OK, thanks!"), None);
        assert_eq!(
            detect("https://example.com/a/b/c ana@example.com 12345"),
            None
        );
    }

    #[test]
    fn a_reply_counts_not_the_quote() {
        let reply = "Sounds good, I will call you tomorrow morning about the contract and \
                     the dates for the visit.\n\n> Hola, ¿podemos hablar mañana sobre el \
                     contrato y las fechas de la visita? Un saludo, Ana";
        assert_eq!(detect(reply), Some("en"));
    }

    #[test]
    fn tags_become_codes() {
        assert_eq!(code_for_tag("en-IN"), "en");
        assert_eq!(code_for_tag("pt-BR"), "pt");
        assert_eq!(code_for_tag("zh-Hans"), "zh");
        assert_eq!(code_for_tag("zh-Hant"), "zt");
        assert_eq!(code_for_tag("zh_TW"), "zt");
        assert_eq!(code_for_tag("fil"), "tl");
        assert_eq!(code_for_tag("bn"), "bn");
        assert!(same_language("zh", "zt"));
        assert!(!same_language("es", "en"));
    }

    #[test]
    fn names_in_their_own_language() {
        assert_eq!(native_name("es"), Some("Español"));
        assert_eq!(native_name("de"), Some("Deutsch"));
        assert_eq!(native_name("xx"), None);
        assert_eq!(known_code(&String::from("es")), Some("es"));
        assert_eq!(known_code("xx"), None);
    }

    #[test]
    fn pieces_join_back_and_break_between_paragraphs() {
        let text = "First paragraph here.\n\nSecond one, a little longer than that.\n\nThird.";
        let pieces = pieces(text, 30);
        assert!(pieces.iter().all(|p| p.chars().count() <= 30));
        assert_eq!(pieces.concat(), text);
        assert_eq!(pieces[0], "First paragraph here.\n\n");
        // One long word is cut where it must be.
        let word = "a".repeat(25);
        assert_eq!(
            super::pieces(&word, 10),
            vec!["a".repeat(10), "a".repeat(10), "a".repeat(5)]
        );
        // Characters, not bytes.
        let bengali = "আ".repeat(12);
        assert!(
            super::pieces(&bengali, 5)
                .iter()
                .all(|p| p.chars().count() <= 5)
        );
    }

    #[test]
    fn very_long_text_is_cut() {
        let text = "word ".repeat(MAX_TEXT);
        let total: usize = pieces(&text, MAX_PIECE)
            .iter()
            .map(|p| p.chars().count())
            .sum();
        assert_eq!(total, MAX_TEXT);
    }

    #[test]
    fn server_answers_parse() {
        let answer: TranslateResponse =
            serde_json::from_str(r#"{"translatedText":"Hello","detectedLanguage":{}}"#).unwrap();
        assert_eq!(answer.translated_text, "Hello");
        let request = serde_json::to_value(TranslateRequest::new("Hola", "es", "en")).unwrap();
        assert_eq!(request["format"], "text");
        let languages: Vec<ServerLanguage> =
            serde_json::from_str(r#"[{"code":"en","name":"English","targets":["es","de"]}]"#)
                .unwrap();
        assert_eq!(languages[0].targets, ["es", "de"]);
    }
}
