// SPDX-License-Identifier: GPL-3.0-or-later

//! Text files for the viewer.

/// At most this much of a text file is shown.
pub const MAX_BYTES: usize = 512 * 1024;
/// ...and at most this many lines.
pub const MAX_LINES: usize = 10_000;

/// The start of a text file as lines, invalid UTF-8 replaced (Latin-1 when
/// the file is not UTF-8 at all), tabs expanded. Returns whether the file
/// was cut.
pub fn lines(bytes: &[u8]) -> (Vec<String>, bool) {
    let (text, mut cut) = decode(bytes, MAX_BYTES);
    let mut lines: Vec<String> = text
        .lines()
        .take(MAX_LINES)
        .map(|line| {
            line.trim_end_matches('\r')
                .replace('\t', "    ")
                .chars()
                .filter(|c| !c.is_control())
                .collect()
        })
        .collect();
    if text.lines().nth(MAX_LINES).is_some() {
        cut = true;
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    (lines, cut)
}

/// The first `max` bytes of a text file as a string: UTF-8 (invalid bytes
/// replaced), or Latin-1 when it is not UTF-8 at all; a byte order mark is
/// dropped. Returns whether the file was cut.
pub fn decode(bytes: &[u8], max: usize) -> (String, bool) {
    let cut = bytes.len() > max;
    let bytes = &bytes[..bytes.len().min(max)];
    let text = match std::str::from_utf8(bytes) {
        Ok(text) => text.to_owned(),
        // Cut inside a character: drop the partial one.
        Err(err) if cut && err.error_len().is_none() => {
            String::from_utf8_lossy(&bytes[..err.valid_up_to()]).into_owned()
        }
        Err(_) => bytes.iter().map(|&b| char::from(b)).collect(),
    };
    let text = match text.strip_prefix('\u{feff}') {
        Some(rest) => rest.to_owned(),
        None => text,
    };
    (text, cut)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf8_latin1_and_limits() {
        assert_eq!(
            lines(b"a\tb\r\nc\n"),
            (vec!["a    b".to_owned(), "c".to_owned()], false)
        );
        assert_eq!(lines(b"caf\xe9").0, ["café"]);
        assert_eq!(lines("\u{feff}hi".as_bytes()).0, ["hi"]);
        let long = "x\n".repeat(MAX_LINES + 5);
        let (shown, cut) = lines(long.as_bytes());
        assert_eq!(shown.len(), MAX_LINES);
        assert!(cut);
        assert_eq!(lines(b"").0, [""]);
    }
}
