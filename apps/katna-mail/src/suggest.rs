// SPDX-License-Identifier: GPL-3.0-or-later

//! Writing suggestions: the rest of a phrase shown ahead of the cursor
//! while writing, from phrases the user wrote before and from the
//! conversation being answered. Learned on this computer from the user's
//! own sent mail, plus a few phrases common in mail; nothing leaves the
//! machine. English first: it only offers what
//! it has seen, so it stays quiet in other languages until it has seen
//! them. No GPUI here.

use std::collections::HashMap;
use std::rc::Rc;

use katna_core::Paths;
use katna_store::{Mode, Store};
use katna_ui::rich::Suggest;

use crate::sidebar::Role;

/// Sent messages read at most, newest first.
const MESSAGES: usize = 3000;
/// Words suggested at most.
const WORDS: usize = 5;
/// A next word is suggested only when it followed its context this often
/// and in at least this share of the times.
const MIN_COUNT: u32 = 2;
const MIN_SHARE: f32 = 0.6;
/// Built-in phrases count as if written this often.
const BUILT_IN_WEIGHT: u32 = 2;
/// The conversation read at most, in characters.
const THREAD_CHARS: usize = 20_000;
/// Words of the conversation this long complete as they are typed, as do
/// names.
const TERM_CHARS: usize = 7;

/// Phrases common in mail, so there is something to offer from the start.
const BUILT_IN: &[&str] = &[
    "Thank you for your email",
    "Thank you for your help",
    "Thank you for your time",
    "Thank you for getting back to me",
    "Thanks for getting back to me",
    "Thanks for letting me know",
    "Thanks for the update",
    "Thanks in advance",
    "I hope you are doing well",
    "I hope this email finds you well",
    "I hope you had a great weekend",
    "Please let me know if you have any questions",
    "Let me know if you have any questions",
    "Let me know what you think",
    "Please find attached the",
    "Please see attached",
    "I look forward to hearing from you",
    "Looking forward to hearing from you",
    "Looking forward to seeing you",
    "Have a great weekend",
    "Have a great day",
    "Have a nice day",
    "I will get back to you as soon as possible",
    "I will let you know",
    "Sorry for the late reply",
    "Sorry for the delay",
    "Just a quick reminder",
    "Just wanted to follow up on",
    "I wanted to follow up on",
    "Would you be available for a call",
    "Does that work for you",
    "Please let me know if that works for you",
    "Feel free to reach out if you have any questions",
    "As discussed in our meeting",
    "Could you please send me the",
    "I would like to schedule a meeting",
    "Happy to help",
    "Best regards",
    "Kind regards",
    "Many thanks",
];

/// How often each word followed each context of one or two words.
#[derive(Default)]
pub struct Phrases {
    /// Every word as last written inside a sentence (so it is not
    /// capitalized just for starting one); lowercase to its id.
    words: Vec<String>,
    ids: HashMap<String, u32>,
    /// The next words after a pair of words, and after one word.
    after_two: HashMap<(u32, u32), Vec<(u32, u32)>>,
    after_one: HashMap<u32, Vec<(u32, u32)>>,
}

impl Phrases {
    /// Learns from the user's sent mail in the store, newest first. Slow
    /// on a big store; call it off the UI thread.
    pub fn learn(paths: &Paths) -> Self {
        let mut phrases = Self::built_in();
        match Store::open(paths, Mode::ReadOnly) {
            Ok(store) => {
                for text in sent_texts(&store) {
                    phrases.add(&text, 1);
                }
            }
            Err(err) => tracing::info!("writing suggestions: {err}"),
        }
        phrases.prune();
        phrases
    }

    /// Only the phrases built in.
    pub fn built_in() -> Self {
        let mut phrases = Self::default();
        for phrase in BUILT_IN {
            phrases.add(phrase, BUILT_IN_WEIGHT);
        }
        phrases
    }

    fn id(&mut self, word: &str, inside: bool) -> u32 {
        let key = word.to_lowercase();
        if let Some(id) = self.ids.get(&key) {
            if inside && self.words[*id as usize] != word {
                self.words[*id as usize] = word.to_owned();
            }
            return *id;
        }
        let id = self.words.len() as u32;
        self.words.push(word.to_owned());
        self.ids.insert(key, id);
        id
    }

    /// Counts the phrases of `text`, sentence by sentence.
    fn add(&mut self, text: &str, weight: u32) {
        for sentence in sentences(text) {
            let ids: Vec<u32> = words(sentence)
                .enumerate()
                .map(|(ix, w)| self.id(w, ix > 0))
                .collect();
            for (ix, next) in ids.iter().enumerate().skip(1) {
                bump(
                    self.after_one.entry(ids[ix - 1]).or_default(),
                    *next,
                    weight,
                );
                if ix >= 2 {
                    bump(
                        self.after_two
                            .entry((ids[ix - 2], ids[ix - 1]))
                            .or_default(),
                        *next,
                        weight,
                    );
                }
            }
        }
    }

    /// Forgets what was seen only once: it would never be suggested, and
    /// it is most of the memory.
    fn prune(&mut self) {
        let keep = |nexts: &mut Vec<(u32, u32)>| {
            let total: u32 = nexts.iter().map(|(_, n)| n).sum();
            nexts.retain(|(_, n)| *n >= MIN_COUNT);
            nexts.shrink_to_fit();
            total >= MIN_COUNT && !nexts.is_empty()
        };
        self.after_two.retain(|_, nexts| keep(nexts));
        self.after_one.retain(|_, nexts| keep(nexts));
        self.after_two.shrink_to_fit();
        self.after_one.shrink_to_fit();
    }

    /// Adds the words seen after `context` (lowercase words) to `out`, by
    /// lowercase word: how it is written and how often.
    fn count_next(&self, context: &[String], out: &mut HashMap<String, (String, u32)>) {
        let id = |w: &String| self.ids.get(w).copied();
        let nexts = match context {
            [.., a, b] => id(a).zip(id(b)).and_then(|ab| self.after_two.get(&ab)),
            [b] => id(b).and_then(|b| self.after_one.get(&b)),
            [] => None,
        };
        for (word, n) in nexts.into_iter().flatten() {
            let word = &self.words[*word as usize];
            out.entry(word.to_lowercase())
                .or_insert_with(|| (word.clone(), 0))
                .1 += n;
        }
    }
}

/// Suggestions for one message: from the phrases of the user's sent mail
/// and of the conversation being answered, whose names and terms also
/// complete as they are typed.
pub struct Suggester {
    sent: Rc<Phrases>,
    thread: Phrases,
    /// Names and longer words of the conversation, lowercase to as
    /// written.
    terms: HashMap<String, String>,
}

impl Suggester {
    /// `thread` is the text of the conversation being answered, if any.
    pub fn new(sent: Rc<Phrases>, thread: &str) -> Self {
        // A conversation is short: what it says once counts.
        let thread: String = thread.chars().take(THREAD_CHARS).collect();
        let mut phrases = Phrases::default();
        phrases.add(&thread, MIN_COUNT);
        let mut terms = HashMap::new();
        for sentence in sentences(&thread) {
            for (ix, word) in words(sentence).enumerate() {
                let name = ix > 0 && word.starts_with(char::is_uppercase);
                if name || word.chars().count() >= TERM_CHARS {
                    terms
                        .entry(word.to_lowercase())
                        .or_insert_with(|| word.to_owned());
                }
            }
        }
        Self {
            sent,
            thread: phrases,
            terms,
        }
    }

    /// The likely next word after `context`, if one stands out. `prefix`,
    /// lowercase, limits it to words that start with it.
    fn next(&self, context: &[String], prefix: &str) -> Option<String> {
        let mut nexts = HashMap::new();
        self.sent.count_next(context, &mut nexts);
        self.thread.count_next(context, &mut nexts);
        let matching = nexts.iter().filter(|(w, _)| {
            prefix.is_empty() || (w.starts_with(prefix) && w.len() > prefix.len())
        });
        let total: u32 = matching.clone().map(|(_, (_, n))| n).sum();
        let (_, (best, count)) = matching.max_by_key(|(_, (_, n))| *n)?;
        (*count >= MIN_COUNT && *count as f32 >= total as f32 * MIN_SHARE).then(|| best.clone())
    }

    /// The one name or term of the conversation that starts with `prefix`
    /// (lowercase), if there is one.
    fn term(&self, prefix: &str) -> Option<&str> {
        if prefix.chars().count() < 2 {
            return None;
        }
        let mut found = self
            .terms
            .iter()
            .filter(|(w, _)| w.starts_with(prefix) && w.len() >= prefix.len() + 3);
        let (_, term) = found.next()?;
        found.next().is_none().then_some(term.as_str())
    }
}

impl Suggest for Suggester {
    fn suggest(&self, before: &str) -> Option<String> {
        // The sentence being written, up to the cursor.
        let sentence = before
            .rsplit(['.', '!', '?', '\n'])
            .next()
            .unwrap_or_default();
        let typed: Vec<&str> = words(sentence).collect();
        let open = sentence.ends_with(is_word_char);
        let (done, partial) = match (open, typed.split_last()) {
            (true, Some((last, rest))) => (rest, *last),
            _ => (&typed[..], ""),
        };
        let mut context: Vec<String> = done
            .iter()
            .rev()
            .take(2)
            .rev()
            .map(|w| w.to_lowercase())
            .collect();
        // Nothing to go on at the start of a sentence.
        if context.is_empty() {
            return None;
        }
        let mut out = String::new();
        if !partial.is_empty() {
            let lower = partial.to_lowercase();
            // The rest of the word being typed, when one stands out, or a
            // name or term of the conversation; else the word may be
            // whole already, and the phrase goes on after it.
            let word = self
                .next(&context, &lower)
                .or_else(|| self.term(&lower).map(str::to_owned));
            match word {
                Some(word) => {
                    out.push_str(word.get(partial.len()..)?);
                    context.push(word.to_lowercase());
                }
                None => context.push(lower),
            }
        }
        let mut space = !before.ends_with(char::is_whitespace);
        for _ in 0..WORDS {
            let tail = &context[context.len().saturating_sub(2)..];
            let Some(word) = self.next(tail, "") else {
                break;
            };
            if space {
                out.push(' ');
            }
            space = true;
            context.push(word.to_lowercase());
            out.push_str(&word);
        }
        // A letter or two is not worth a Tab.
        (out.trim().chars().count() >= 3).then_some(out)
    }
}

fn bump(nexts: &mut Vec<(u32, u32)>, word: u32, weight: u32) {
    match nexts.iter_mut().find(|(w, _)| *w == word) {
        Some((_, n)) => *n += weight,
        None => nexts.push((word, weight)),
    }
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '\'' || c == '\u{2019}'
}

fn words(text: &str) -> impl Iterator<Item = &str> {
    text.split(|c: char| !is_word_char(c))
        .filter(|w| !w.is_empty())
}

fn sentences(text: &str) -> impl Iterator<Item = &str> {
    text.split(['.', '!', '?', '\n', ';', ':'])
}

/// What the user wrote in their newest sent messages: each body without
/// the quoted mail and the signature.
fn sent_texts(store: &Store) -> Vec<String> {
    let folders = store.folder_summaries().unwrap_or_default();
    let mut ids = Vec::new();
    for folder in folders
        .iter()
        .filter(|f| Role::detect(f.role.as_deref(), &f.path) == Role::Sent)
    {
        ids.extend(store.folder_message_ids(folder.id).unwrap_or_default());
    }
    ids.sort_by_key(|id| std::cmp::Reverse(id.0));
    ids.dedup();
    ids.truncate(MESSAGES);
    let messages = store.messages_by_id(&ids).unwrap_or_default();
    messages
        .into_iter()
        .filter_map(|m| store.blobs().get(&m.blob_hash?).ok().flatten())
        .map(|raw| own_text(&katna_render::message_view(&raw).body))
        .collect()
}

/// A body up to the quoted mail or the signature.
fn own_text(body: &str) -> String {
    let mut out = String::new();
    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed == "--"
            || line.starts_with("-- ")
            || trimmed.starts_with('>')
            || trimmed.starts_with("-----Original Message")
            || (trimmed.starts_with("On ") && trimmed.ends_with("wrote:"))
            || trimmed.starts_with("From: ")
        {
            break;
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn learned(texts: &[&str]) -> Suggester {
        let mut phrases = Phrases::built_in();
        for text in texts {
            phrases.add(text, 1);
        }
        phrases.prune();
        Suggester::new(Rc::new(phrases), "")
    }

    #[test]
    fn finishes_a_common_phrase() {
        let phrases = learned(&[]);
        assert_eq!(
            phrases
                .suggest("Hi Kay,\nPlease let me know if you have")
                .as_deref(),
            Some(" any questions")
        );
        assert_eq!(
            phrases.suggest("I look forward to hea").as_deref(),
            Some("ring from you")
        );
    }

    #[test]
    fn learns_the_users_own_phrases() {
        let sent = "The quarterly numbers are in the shared folder.";
        let phrases = learned(&[sent, sent, sent]);
        assert_eq!(
            phrases.suggest("Hello. The quarterly ").as_deref(),
            Some("numbers are in the shared")
        );
        // Written as inside a sentence, not as at its start.
        let phrases = learned(&[
            "Numbers first.",
            "The numbers are in.",
            "The numbers are in.",
        ]);
        assert_eq!(phrases.suggest("the num").as_deref(), Some("bers are in"));
    }

    #[test]
    fn stays_quiet_when_unsure() {
        let phrases = learned(&["We meet on Monday.", "We meet on Friday."]);
        assert_eq!(phrases.suggest("We meet on "), None);
        // Nothing to go on at the start of a sentence.
        assert_eq!(phrases.suggest("Thank"), None);
        assert_eq!(phrases.suggest("Wir treffen uns "), None);
    }

    #[test]
    fn follows_the_conversation() {
        let thread = "Hi Kay,\nCan we move the quarterly review to Thursday at 3pm? \
                      Roberta will bring the forecast.\nThanks, Bob";
        let suggester = Suggester::new(Rc::new(Phrases::built_in()), thread);
        // Its phrases, though seen once.
        assert_eq!(
            suggester.suggest("Sure, Thursday at ").as_deref(),
            Some("3pm")
        );
        // Its names and longer words.
        assert_eq!(suggester.suggest("Thanks Rob").as_deref(), Some("erta"));
        assert_eq!(
            suggester.suggest("I will read the fore").as_deref(),
            Some("cast")
        );
        // And still the common phrases.
        assert_eq!(
            suggester.suggest("Let me know if you have").as_deref(),
            Some(" any questions")
        );
    }

    #[test]
    fn keeps_only_the_users_text() {
        let body = "Sounds good.\n\n-- \nKay\n\nOn Mon, Bob wrote:\n> hi\n";
        assert_eq!(own_text(body), "Sounds good.\n\n");
    }
}
