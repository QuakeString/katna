// SPDX-License-Identifier: GPL-3.0-or-later

//! Reads messages from an mbox file (mboxo/mboxrd, as written by Thunderbird,
//! mutt and Google Takeout).

use std::io::{self, BufRead};

use mail_parser::mailbox::mbox::MessageIterator;

use crate::Flags;

/// One message read by [`Reader`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// The raw message, with `>From ` quoting removed.
    pub raw: Vec<u8>,
    /// Flags from the `Status` and `X-Status` headers.
    pub flags: Flags,
}

/// Iterator over the messages of an mbox file.
pub struct Reader<R> {
    inner: MessageIterator<R>,
}

impl<R: BufRead> Reader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            inner: MessageIterator::new(reader),
        }
    }
}

impl<R: BufRead> Iterator for Reader<R> {
    type Item = io::Result<Entry>;

    fn next(&mut self) -> Option<Self::Item> {
        let message = match self.inner.next()? {
            Ok(message) => message,
            Err(err) => return Some(Err(err)),
        };
        let mut raw = message.unwrap_contents();
        // The blank line before the next "From " line belongs to the mbox
        // format, not to the message.
        if raw.ends_with(b"\r\n\r\n") {
            raw.truncate(raw.len() - 2);
        } else if raw.ends_with(b"\n\n") {
            raw.truncate(raw.len() - 1);
        }
        let flags = flags_from_headers(&raw);
        Some(Ok(Entry { raw, flags }))
    }
}

/// Reads `Status: RO` and `X-Status: AFTD` from the header section.
fn flags_from_headers(raw: &[u8]) -> Flags {
    let mut flags = Flags::default();
    for line in raw.split(|&b| b == b'\n') {
        let line = line.strip_suffix(b"\r").unwrap_or(line);
        if line.is_empty() {
            break;
        }
        let Some(colon) = line.iter().position(|&b| b == b':') else {
            continue;
        };
        let (name, value) = (&line[..colon], &line[colon + 1..]);
        if name.eq_ignore_ascii_case(b"Status") {
            flags.seen |= value.contains(&b'R');
        } else if name.eq_ignore_ascii_case(b"X-Status") {
            flags.answered |= value.contains(&b'A');
            flags.flagged |= value.contains(&b'F');
            flags.draft |= value.contains(&b'T');
            flags.deleted |= value.contains(&b'D');
        }
    }
    flags
}

#[cfg(test)]
mod tests {
    use super::*;

    const MBOX: &[u8] = b"From alice@example.org Mon Jan  1 10:00:00 2024
Status: RO
X-Status: AF
Subject: one

first
>From the start
>>From quoted

From bob@example.org Tue Jan  2 10:00:00 2024
Subject: two

second
";

    #[test]
    fn splits_messages_and_reads_flags() {
        let entries: Vec<_> = Reader::new(MBOX).map(Result::unwrap).collect();
        assert_eq!(entries.len(), 2);
        assert_eq!(
            entries[0].raw,
            b"Status: RO\nX-Status: AF\nSubject: one\n\nfirst\nFrom the start\n>From quoted\n"
        );
        assert_eq!(
            entries[0].flags,
            Flags {
                seen: true,
                answered: true,
                flagged: true,
                ..Flags::default()
            }
        );
        assert_eq!(entries[1].raw, b"Subject: two\n\nsecond\n");
        assert_eq!(entries[1].flags, Flags::default());
    }

    #[test]
    fn empty_file() {
        assert_eq!(Reader::new(&b""[..]).count(), 0);
    }
}
