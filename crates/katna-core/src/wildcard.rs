// SPDX-License-Identifier: GPL-3.0-or-later

//! File-name patterns typed in a search box: `*` stands for any run of
//! characters and `?` for any one, so `*.pdf` finds every PDF and
//! `invoice*2026*` every invoice of 2026. Case never matters.

/// Whether `word` is a pattern rather than plain words.
pub fn is_pattern(word: &str) -> bool {
    word.contains(['*', '?'])
}

/// Whether the whole of `name` fits `pattern`.
pub fn matches(pattern: &str, name: &str) -> bool {
    let pattern: Vec<char> = pattern.to_lowercase().chars().collect();
    let name: Vec<char> = name.to_lowercase().chars().collect();
    let (mut p, mut n) = (0, 0);
    // Where the last `*` was, and where in the name it last stopped.
    let mut star: Option<(usize, usize)> = None;
    while n < name.len() {
        match pattern.get(p) {
            Some('*') => {
                star = Some((p, n));
                p += 1;
            }
            Some(&c) if c == '?' || c == name[n] => {
                p += 1;
                n += 1;
            }
            _ => match star {
                // The `*` takes one more character, and the rest tries again.
                Some((sp, sn)) => {
                    p = sp + 1;
                    n = sn + 1;
                    star = Some((sp, sn + 1));
                }
                None => return false,
            },
        }
    }
    pattern[p..].iter().all(|&c| c == '*')
}

/// The plain pieces of `pattern` between its wildcards, without the dots
/// and dashes at their ends, for a search that cannot take wildcards.
pub fn pieces(pattern: &str) -> Vec<&str> {
    pattern
        .split(['*', '?'])
        .map(|piece| {
            piece.trim_matches(|c: char| c == '.' || c == '-' || c == '_' || c.is_whitespace())
        })
        .filter(|piece| !piece.is_empty())
        .collect()
}

/// The extension `pattern` asks for when it is `*.ext`.
pub fn extension(pattern: &str) -> Option<&str> {
    let ext = pattern.strip_prefix("*.")?;
    (!ext.is_empty() && !is_pattern(ext) && !ext.contains('.')).then_some(ext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stars_and_marks() {
        assert!(matches("*.pdf", "Tax return 2025.PDF"));
        assert!(!matches("*.pdf", "notes.pdf.txt"));
        assert!(matches("invoice*2026*", "Invoice-March-2026.xlsx"));
        assert!(!matches("invoice*2026*", "My invoice 2026.pdf"));
        assert!(matches("scan_??.jpg", "scan_07.jpg"));
        assert!(!matches("scan_??.jpg", "scan_7.jpg"));
        assert!(matches("*", ""));
        assert!(matches("a*b*c", "aXXbYYbc"));
        assert!(!matches("a*b*c", "aXXbYYbd"));
    }

    #[test]
    fn plain_pieces() {
        assert!(is_pattern("*.pdf"));
        assert!(!is_pattern("budget"));
        assert_eq!(pieces("invoice*2026*"), ["invoice", "2026"]);
        assert_eq!(pieces("*.pdf"), ["pdf"]);
        assert!(pieces("*").is_empty());
        assert_eq!(extension("*.pdf"), Some("pdf"));
        assert_eq!(extension("*.tar.gz"), None);
        assert_eq!(extension("a*.pdf"), None);
    }
}
