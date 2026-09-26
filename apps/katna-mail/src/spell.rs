// SPDX-License-Identifier: GPL-3.0-or-later

//! Spell checking while writing, with the Hunspell dictionaries installed
//! on the system (`hunspell-en_us` and friends) and the user's own words.
//! No GPUI here.

use std::cell::RefCell;
use std::path::{Path, PathBuf};

use katna_ui::rich::SpellCheck;

/// Where distributions put Hunspell dictionaries.
const DIRS: [&str; 4] = [
    "/usr/share/hunspell",
    "/usr/share/myspell/dicts",
    "/usr/share/myspell",
    "/usr/local/share/hunspell",
];

/// A loaded dictionary and the words the user added.
pub struct Speller {
    dictionary: RefCell<spellbook::Dictionary>,
    /// The user's word list, one per line.
    personal: PathBuf,
}

impl Speller {
    /// Adds `word` to the user's list, so it is not marked again.
    pub fn add_word(&self, word: &str) -> std::io::Result<()> {
        use std::io::Write as _;
        let _ = self.dictionary.borrow_mut().add(word);
        if let Some(dir) = self.personal.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.personal)?;
        writeln!(file, "{word}")
    }
}

impl SpellCheck for Speller {
    fn check(&self, word: &str) -> bool {
        let word = word.trim_matches(|c: char| !c.is_alphanumeric());
        // Words with digits or all capitals are names and codes.
        word.is_empty()
            || word.chars().any(|c| c.is_ascii_digit())
            || (word.chars().count() > 1 && word.chars().all(|c| !c.is_lowercase()))
            || self.dictionary.borrow().check(word)
    }

    fn suggest(&self, word: &str) -> Vec<String> {
        let mut out = Vec::new();
        self.dictionary.borrow().suggest(word, &mut out);
        out.truncate(5);
        out
    }
}

/// The dictionary language: the setting, else the desktop's (`LANG`),
/// as `en_US`.
pub fn language(setting: &str) -> String {
    if !setting.trim().is_empty() {
        return setting.trim().to_owned();
    }
    let env = ["LC_ALL", "LC_MESSAGES", "LANG"]
        .iter()
        .filter_map(|k| std::env::var(k).ok())
        .find(|v| !v.is_empty() && v != "C" && v != "POSIX")
        .unwrap_or_else(|| "en_US".to_owned());
    env.split(['.', '@']).next().unwrap_or("en_US").to_owned()
}

/// The `.aff` and `.dic` files for `language`, or for the same language in
/// another country (`en_GB` for `en_IN`), or English.
pub fn find(language: &str) -> Option<(PathBuf, PathBuf)> {
    let home = std::env::var_os("HOME").map(PathBuf::from);
    let mut dirs: Vec<PathBuf> = DIRS.iter().map(PathBuf::from).collect();
    if let Some(home) = home {
        dirs.insert(0, home.join(".local/share/hunspell"));
    }
    let base = language.split('_').next().unwrap_or(language);
    let exact = |dir: &Path, name: &str| {
        let aff = dir.join(format!("{name}.aff"));
        let dic = dir.join(format!("{name}.dic"));
        (aff.is_file() && dic.is_file()).then_some((aff, dic))
    };
    for dir in &dirs {
        if let Some(found) = exact(dir, language) {
            return Some(found);
        }
    }
    // The same language anywhere else.
    for dir in &dirs {
        let Ok(entries) = std::fs::read_dir(dir) else {
            continue;
        };
        let mut names: Vec<String> = entries
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                e.file_name()
                    .to_str()?
                    .strip_suffix(".dic")
                    .map(str::to_owned)
            })
            .filter(|n| n == base || n.starts_with(&format!("{base}_")))
            .collect();
        names.sort();
        if let Some(found) = names.iter().find_map(|n| exact(dir, n)) {
            return Some(found);
        }
    }
    (base != "en").then(|| find("en_US")).flatten()
}

/// Loads the dictionary for `language` and the user's words from
/// `personal`. Slow (a large dictionary takes a moment): call it off the
/// main thread.
pub fn load(language: &str, personal: PathBuf) -> Result<Speller, String> {
    let (aff, dic) = find(language).ok_or_else(|| {
        format!("No spelling dictionary for {language} is installed (for example hunspell-en_us).")
    })?;
    let read = |p: &Path| std::fs::read(p).map_err(|e| format!("{}: {e}", p.display()));
    // Some dictionaries are in legacy encodings; keep what reads.
    let aff = String::from_utf8_lossy(&read(&aff)?).into_owned();
    let dic = String::from_utf8_lossy(&read(&dic)?).into_owned();
    let mut dictionary =
        spellbook::Dictionary::new(&aff, &dic).map_err(|e| format!("Dictionary: {e}"))?;
    if let Ok(words) = std::fs::read_to_string(&personal) {
        for word in words.lines().map(str::trim).filter(|w| !w.is_empty()) {
            let _ = dictionary.add(word);
        }
    }
    Ok(Speller {
        dictionary: RefCell::new(dictionary),
        personal,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn languages() {
        assert_eq!(language("de_DE"), "de_DE");
    }

    #[test]
    fn checks_with_an_installed_dictionary() {
        // Only where a dictionary is installed (CI and desktops usually).
        if find("en_US").is_none() {
            return;
        }
        let dir = tempfile::tempdir().unwrap();
        let personal = dir.path().join("dictionary");
        let speller = load("en_US", personal.clone()).unwrap();
        assert!(speller.check("hello"));
        assert!(speller.check("NASA"));
        assert!(!speller.check("helo"));
        assert!(speller.suggest("helo").iter().any(|s| s == "hello"));
        speller.add_word("Katna").unwrap();
        assert!(speller.check("Katna"));
        let again = load("en_US", personal).unwrap();
        assert!(again.check("Katna"));
    }
}
