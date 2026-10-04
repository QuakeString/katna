// SPDX-License-Identifier: GPL-3.0-or-later

//! A task typed in one line, as quick capture and KRunner's `task:` take
//! it (`docs/ARCHITECTURE.md` §18.1): "Call the plumber fri 6pm #home"
//! gives the title "Call the plumber", Friday at 18:00 and the label
//! "home". The day, time and repeat come from the quick add parser shared
//! with Calendar's events ([`katna_core::quick_add`]); tasks have no place,
//! so "at" stays in the title.

use jiff::civil::Date;
use katna_core::quick_add::{self, Words};
use katna_store::tasks::TaskList;

/// What a typed task says.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TypedTask {
    /// What is left once the day, time, repeat and labels are taken out;
    /// the text as typed when that would leave nothing.
    pub title: String,
    /// `YYYY-MM-DD`; today when only a time or a repeat was typed.
    pub due: Option<String>,
    /// Minutes after midnight on the due day.
    pub due_time: Option<u32>,
    /// An RFC 5545 `RRULE` value.
    pub repeat: Option<String>,
    /// The words typed after `#`, without it, in the order typed.
    pub labels: Vec<String>,
}

/// Reads `text` as typed on `today`, with the words of `language` (a BCP
/// 47 tag).
pub fn parse(text: &str, today: Date, language: &str) -> TypedTask {
    let (rest, labels) = take_labels(text);
    let words = Words {
        at: &[],
        ..*Words::for_language(language)
    };
    let typed = quick_add::parse(&rest, today, &words);
    let title = typed.title.trim().to_owned();
    // The parser keeps a title made only of such words ("tomorrow") whole;
    // then nothing but the labels was understood.
    if !typed.found() || title.is_empty() || title == rest.trim() {
        let title = if rest.trim().is_empty() {
            text.trim()
        } else {
            rest.trim()
        };
        return TypedTask {
            title: title.to_owned(),
            labels,
            ..TypedTask::default()
        };
    }
    let due_time = typed
        .start
        .map(|t| u32::try_from(i32::from(t.hour()) * 60 + i32::from(t.minute())).unwrap_or(0));
    // A repeat without a day starts on its first day from today ("every
    // Monday" typed on a Tuesday: next Monday); a time alone is today.
    let first_repeat = typed.repeat.as_deref().and_then(|rule| {
        let yesterday = today.yesterday().ok()?;
        let (day, _) = crate::todo::next_due(&yesterday.to_string(), rule, yesterday)?;
        day.parse().ok()
    });
    let due = typed
        .day
        .or(first_repeat)
        .or_else(|| (due_time.is_some() || typed.repeat.is_some()).then_some(today))
        .map(|d| d.to_string());
    TypedTask {
        title,
        due,
        due_time,
        repeat: typed.repeat,
        labels,
    }
}

/// The list a task goes to when none is named, as
/// `Store::default_task_list` picks it: the first account's own default
/// list, else the first on this computer.
pub fn default_list(lists: &[TaskList]) -> Option<&TaskList> {
    lists
        .iter()
        .filter(|l| l.account.is_some() && l.is_default)
        .min_by_key(|l| l.account.map(|a| a.0))
        .or_else(|| lists.iter().find(|l| l.account.is_none()))
}

/// Takes the labels out of `text`: each word of `#` and then letters,
/// digits, `-` or `_`. Returns the rest, with single spaces, and the
/// labels, each once.
pub fn take_labels(text: &str) -> (String, Vec<String>) {
    let mut labels: Vec<String> = Vec::new();
    let mut rest = Vec::new();
    for word in text.split_whitespace() {
        match label(word) {
            Some(name) => {
                if !labels.iter().any(|l| l.eq_ignore_ascii_case(name)) {
                    labels.push(name.to_owned());
                }
            }
            None => rest.push(word),
        }
    }
    (rest.join(" "), labels)
}

/// The label a word names (`#home`: `home`), if it is one.
fn label(word: &str) -> Option<&str> {
    let name = word.strip_prefix('#')?;
    (!name.is_empty()
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_'))
    .then_some(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(text: &str) -> Date {
        text.parse().unwrap()
    }

    #[test]
    fn reads_the_day_time_and_label() {
        // A Saturday.
        let typed = parse("Call the plumber fri 6pm #home", day("2026-10-03"), "en");
        assert_eq!(
            typed,
            TypedTask {
                title: "Call the plumber".into(),
                due: Some("2026-10-09".into()),
                due_time: Some(18 * 60),
                repeat: None,
                labels: vec!["home".into()],
            }
        );
    }

    #[test]
    fn plain_text_stays_as_typed() {
        let typed = parse("Water the plants", day("2026-10-03"), "en");
        assert_eq!(typed.title, "Water the plants");
        assert_eq!(typed.due, None);
        // "at" is no place for a task.
        let typed = parse("Meet at the station", day("2026-10-03"), "en");
        assert_eq!(typed.title, "Meet at the station");
    }

    #[test]
    fn labels_are_words_after_a_hash() {
        assert_eq!(
            take_labels("#Home  buy #milk-2 #home C# ##x"),
            (
                "buy C# ##x".to_owned(),
                vec!["Home".to_owned(), "milk-2".to_owned()]
            )
        );
        let typed = parse("#work", day("2026-10-03"), "en");
        assert_eq!(typed.title, "#work");
        assert_eq!(typed.labels, ["work"]);
    }

    #[test]
    fn the_default_list_is_the_first_accounts_own() {
        use katna_core::AccountId;
        let list = |id, account: Option<i64>, is_default| TaskList {
            id,
            account: account.map(AccountId),
            title: format!("List {id}"),
            is_default,
        };
        let lists = [
            list(1, None, true),
            list(2, Some(3), true),
            list(3, Some(2), false),
            list(4, Some(2), true),
        ];
        assert_eq!(default_list(&lists).map(|l| l.id), Some(4));
        assert_eq!(default_list(&lists[..1]).map(|l| l.id), Some(1));
        assert_eq!(default_list(&[]).map(|l| l.id), None);
    }

    #[test]
    fn a_time_alone_is_today_and_a_repeat_starts_on_its_day() {
        let typed = parse("Standup 9:30", day("2026-10-03"), "en");
        assert_eq!(typed.due.as_deref(), Some("2026-10-03"));
        assert_eq!(typed.due_time, Some(9 * 60 + 30));
        let typed = parse("Bins every monday", day("2026-10-03"), "en");
        assert_eq!(typed.title, "Bins");
        assert_eq!(typed.due.as_deref(), Some("2026-10-05"));
        assert!(typed.repeat.is_some());
    }
}
