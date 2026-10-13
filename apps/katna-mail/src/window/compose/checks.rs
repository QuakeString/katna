// SPDX-License-Identifier: GPL-3.0-or-later

//! What Send asks before a message goes out, as in webmail: a message
//! whose words speak of an attachment but has none, and a message without
//! a subject. Every way of sending asks: Send, Send and archive and
//! schedule send.

use katna_ui::rich::{Block, Doc};

/// A question Send asks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::window) enum SendCheck {
    /// The text mentions an attachment, but nothing is attached.
    Attachment,
    /// The subject is empty.
    Subject,
}

/// The questions already answered "Send anyway" for this send.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(in crate::window) struct Passed {
    pub attachment: bool,
    pub subject: bool,
    /// The files in Google Drive are shared with the recipients (or they
    /// were told they would not be).
    pub shared: bool,
}

impl Passed {
    pub fn with(mut self, check: SendCheck) -> Self {
        match check {
            SendCheck::Attachment => self.attachment = true,
            SendCheck::Subject => self.subject = true,
        }
        self
    }

    pub fn with_shared(mut self) -> Self {
        self.shared = true;
        self
    }
}

/// The first question to ask before sending, if any. `attached` counts
/// the files attached.
pub(in crate::window) fn check(
    subject: &str,
    body: &Doc,
    attached: usize,
    passed: Passed,
) -> Option<SendCheck> {
    if !passed.attachment
        && attached == 0
        && (mentions_attachment(subject) || mentions_attachment(&own_words(body)))
    {
        return Some(SendCheck::Attachment);
    }
    if !passed.subject && subject.trim().is_empty() {
        return Some(SendCheck::Subject);
    }
    None
}

/// What the user wrote in `doc`: up to the quoted or forwarded message,
/// without the signature.
fn own_words(doc: &Doc) -> String {
    let mut out = String::new();
    for block in &doc.blocks {
        let Block::Para(p) = block else {
            continue;
        };
        let line = p.text.trim();
        if p.style.quote > 0
            || crate::quoting::is_reply_header(line)
            || crate::quoting::is_forward_header(line)
        {
            break;
        }
        if !p.style.signature {
            out.push_str(&p.text);
            out.push('\n');
        }
    }
    out
}

/// Whether `text` speaks of a file sent with it: "attached", "the
/// attachment", "please find enclosed", "PFA" and the like.
fn mentions_attachment(text: &str) -> bool {
    text.split(|c: char| !c.is_alphanumeric())
        .map(str::to_lowercase)
        .any(|word| word.starts_with("attach") || word.starts_with("enclos") || word == "pfa")
}

#[cfg(test)]
mod tests {
    use katna_ui::rich::html;

    use super::*;

    fn doc(text: &str) -> Doc {
        html::from_plain(text)
    }

    #[test]
    fn asks_when_the_text_speaks_of_an_attachment() {
        let passed = Passed::default();
        for text in [
            "Hi, the report is attached.",
            "Please see the attachment",
            "PFA the invoice",
            "Please find enclosed my CV",
            "I'm attaching the slides",
        ] {
            assert_eq!(
                check("Report", &doc(text), 0, passed),
                Some(SendCheck::Attachment),
                "{text}"
            );
        }
        // Not when something is attached, or it was answered already.
        let text = doc("The report is attached.");
        assert_eq!(check("Report", &text, 1, passed), None);
        assert_eq!(
            check("Report", &text, 0, passed.with(SendCheck::Attachment)),
            None
        );
        // The subject counts too.
        assert_eq!(
            check("Invoice attached", &doc("Hi"), 0, passed),
            Some(SendCheck::Attachment)
        );
        assert_eq!(check("Lunch", &doc("See you at noon"), 0, passed), None);
    }

    #[test]
    fn reads_only_the_users_own_words() {
        let mut body = doc("Sounds good.\n\nOn Mon, Bob wrote:\nsee attached");
        assert_eq!(check("Re: plan", &body, 0, Passed::default()), None);
        let fwd = doc("FYI\n\n---------- Forwarded message ---------\nThe file is attached.");
        assert_eq!(check("Fwd: plan", &fwd, 0, Passed::default()), None);
        // Nor the signature.
        body = doc("Thanks");
        body.blocks
            .extend(doc("Sent with attachments disabled").blocks);
        if let Some(Block::Para(p)) = body.blocks.last_mut() {
            p.style.signature = true;
        }
        assert_eq!(check("Hello", &body, 0, Passed::default()), None);
    }

    #[test]
    fn asks_about_an_empty_subject_after_the_attachment() {
        let passed = Passed::default();
        assert_eq!(check("  ", &doc("Hi"), 0, passed), Some(SendCheck::Subject));
        assert_eq!(
            check("", &doc("See attached"), 0, passed),
            Some(SendCheck::Attachment)
        );
        assert_eq!(
            check(
                "",
                &doc("See attached"),
                0,
                passed.with(SendCheck::Attachment)
            ),
            Some(SendCheck::Subject)
        );
        assert_eq!(
            check("", &doc("Hi"), 0, passed.with(SendCheck::Subject)),
            None
        );
    }
}
