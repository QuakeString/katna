// SPDX-License-Identifier: GPL-3.0-or-later

//! Inline PGP armor in plain text, for snippets and the search index.

use std::borrow::Cow;

/// `text` without its PGP armor: an encrypted block goes entirely (it is
/// noise to people and to search, and the plaintext must not be stored),
/// a clear-signed block keeps its text but loses the armor lines, the
/// `Hash:` header, dash escaping and the signature.
pub fn without_armor(text: &str) -> Cow<'_, str> {
    if !text.contains("-----BEGIN PGP ") {
        return Cow::Borrowed(text);
    }
    enum State {
        Text,
        /// Skipping until this line.
        Skip(&'static str),
        /// The `Hash:` lines after `BEGIN PGP SIGNED MESSAGE`.
        SignedHeader,
        Signed,
    }
    let mut out = String::with_capacity(text.len());
    let mut state = State::Text;
    for line in text.split_inclusive('\n') {
        let bare = line.trim_end();
        state = match state {
            State::Text | State::Signed if bare == "-----BEGIN PGP MESSAGE-----" => {
                State::Skip("-----END PGP MESSAGE-----")
            }
            State::Text | State::Signed if bare == "-----BEGIN PGP SIGNATURE-----" => {
                State::Skip("-----END PGP SIGNATURE-----")
            }
            State::Text if bare == "-----BEGIN PGP SIGNED MESSAGE-----" => State::SignedHeader,
            State::Skip(end) => {
                if bare == end {
                    State::Text
                } else {
                    State::Skip(end)
                }
            }
            State::SignedHeader => {
                if bare.is_empty() {
                    State::Signed
                } else {
                    State::SignedHeader
                }
            }
            State::Signed => {
                out.push_str(line.strip_prefix("- ").unwrap_or(line));
                State::Signed
            }
            State::Text => {
                out.push_str(line);
                State::Text
            }
        };
    }
    Cow::Owned(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_is_untouched() {
        assert!(matches!(without_armor("Hello"), Cow::Borrowed("Hello")));
    }

    #[test]
    fn encrypted_blocks_go() {
        let text = "Before\n-----BEGIN PGP MESSAGE-----\n\nhQIMA\n=abcd\n-----END PGP MESSAGE-----\nAfter\n";
        assert_eq!(without_armor(text), "Before\nAfter\n");
    }

    #[test]
    fn signed_text_stays() {
        let text = "-----BEGIN PGP SIGNED MESSAGE-----\r\nHash: SHA512\r\n\r\n\
Hello Bob,\r\n- -- \r\nAda\r\n-----BEGIN PGP SIGNATURE-----\r\n\r\niQEz\r\n\
-----END PGP SIGNATURE-----\r\n";
        assert_eq!(without_armor(text), "Hello Bob,\r\n-- \r\nAda\r\n");
    }
}
