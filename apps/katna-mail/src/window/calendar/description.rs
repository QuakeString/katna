// SPDX-License-Identifier: GPL-3.0-or-later

//! An event's description as its card shows it: web addresses become
//! links, Outlook's `text<https://…>` shows as `text` linking there, and
//! the rules of underscores that Teams and Outlook draw around their
//! invitations are dropped, so it reads as short lines that wrap.

use std::ops::Range;

/// Longest description shown, in characters.
const MAX: usize = 2000;

/// A line of only these, five or more, is a drawn rule.
fn is_rule(line: &str) -> bool {
    let line = line.trim();
    line.chars().count() >= 5
        && line
            .chars()
            .all(|c| matches!(c, '_' | '-' | '=' | '*' | '~'))
}

/// Where a web address written in `text` from `start` ends.
fn url_end(text: &str, start: usize) -> usize {
    let rest = &text[start..];
    let end = rest
        .find(|c: char| c.is_whitespace() || matches!(c, '<' | '>' | '"' | '\'' | '`'))
        .unwrap_or(rest.len());
    let url = rest[..end].trim_end_matches(['.', ',', ';', ':', '!', '?', ')', ']', '}']);
    start + url.len()
}

/// Where the next `http://` or `https://` in `text` from `from` starts.
fn next_url(text: &str, from: usize) -> Option<usize> {
    let rest = &text[from..];
    let at = [rest.find("https://"), rest.find("http://")]
        .into_iter()
        .flatten()
        .min()?;
    Some(from + at)
}

/// `text` tidied, with the byte ranges of its links and where each goes.
pub(super) fn tidy(text: &str) -> (String, Vec<(Range<usize>, String)>) {
    let text = text.replace("\r\n", "\n").replace('\u{a0}', " ");
    let mut lines: Vec<&str> = Vec::new();
    for line in text.lines() {
        let line = line.trim_end();
        if is_rule(line) {
            continue;
        }
        // One empty line between paragraphs at most, none at the ends.
        if line.trim().is_empty() && lines.last().is_none_or(|l| l.trim().is_empty()) {
            continue;
        }
        lines.push(line);
    }
    while lines.last().is_some_and(|l| l.trim().is_empty()) {
        lines.pop();
    }
    let text = lines.join("\n");

    let mut out = String::with_capacity(text.len());
    let mut links = Vec::new();
    let mut at = 0;
    while let Some(start) = next_url(&text, at) {
        let end = url_end(&text, start);
        let url = &text[start..end];
        // `text<https://…>`: the words before it link there.
        let bracketed = text[..start].ends_with('<') && text[end..].starts_with('>');
        // `<https://…>` alone, after a space: the address itself.
        let labelled =
            bracketed && !text[..start - 1].ends_with(char::is_whitespace) && start - 1 > at;
        if bracketed && !labelled {
            out.push_str(&text[at..start - 1]);
            let from = out.len();
            out.push_str(url);
            links.push((from..out.len(), url.to_owned()));
            at = end + 1;
            continue;
        }
        if labelled {
            let before = &text[at..start - 1];
            let label_start = before.rfind(['\n', '|', ':']).map_or(0, |i| i + 1);
            let label = before[label_start..].trim();
            out.push_str(&before[..label_start]);
            if !before[..label_start].is_empty() && before[label_start..].starts_with(' ') {
                out.push(' ');
            }
            let from = out.len();
            out.push_str(if label.is_empty() { url } else { label });
            links.push((from..out.len(), url.to_owned()));
            at = end + 1;
            continue;
        }
        out.push_str(&text[at..start]);
        let from = out.len();
        out.push_str(url);
        links.push((from..out.len(), url.to_owned()));
        at = end;
    }
    out.push_str(&text[at..]);

    if out.chars().count() > MAX {
        let cut = out.char_indices().nth(MAX).map_or(out.len(), |(i, _)| i);
        out.truncate(cut);
        out.push('…');
        links.retain(|(range, _)| range.end <= cut);
    }
    (out, links)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn linked(text: &str) -> (String, Vec<(String, String)>) {
        let (out, links) = tidy(text);
        let links = links
            .into_iter()
            .map(|(range, url)| (out[range].to_owned(), url))
            .collect();
        (out, links)
    }

    #[test]
    fn a_teams_invitation_reads_cleanly() {
        let text = "\r\n________________________________________________________________________________\r\n\
                    Microsoft Teams meeting\r\n\
                    Join: https://teams.microsoft.com/meet/396982345961985?p=ZLXefigd\r\n\
                    Meeting ID: 396 982 345 961 985\r\n\
                    ________________________________\r\n\
                    Need help?<https://aka.ms/JoinTeamsMeeting?omkt=en-US> | System reference<https://teams.microsoft.com/l/meetup-join/19%3ameeting>\r\n\
                    For organizers: Meeting options<https://teams.microsoft.com/meetingOptions/?o=1>\r\n\
                    ________________________________________________________________________________\r\n";
        let (out, links) = linked(text);
        assert_eq!(
            out,
            "Microsoft Teams meeting\n\
             Join: https://teams.microsoft.com/meet/396982345961985?p=ZLXefigd\n\
             Meeting ID: 396 982 345 961 985\n\
             Need help? | System reference\n\
             For organizers: Meeting options"
        );
        assert_eq!(
            links,
            [
                (
                    "https://teams.microsoft.com/meet/396982345961985?p=ZLXefigd".to_owned(),
                    "https://teams.microsoft.com/meet/396982345961985?p=ZLXefigd".to_owned()
                ),
                (
                    "Need help?".to_owned(),
                    "https://aka.ms/JoinTeamsMeeting?omkt=en-US".to_owned()
                ),
                (
                    "System reference".to_owned(),
                    "https://teams.microsoft.com/l/meetup-join/19%3ameeting".to_owned()
                ),
                (
                    "Meeting options".to_owned(),
                    "https://teams.microsoft.com/meetingOptions/?o=1".to_owned()
                ),
            ]
        );
    }

    #[test]
    fn addresses_in_sentences_link_without_their_full_stop() {
        let (out, links) = linked("See http://example.org/a. Or <https://example.org/b>");
        assert_eq!(out, "See http://example.org/a. Or https://example.org/b");
        assert_eq!(links[0].1, "http://example.org/a");
        assert_eq!(
            links[1],
            (
                "https://example.org/b".into(),
                "https://example.org/b".into()
            )
        );
    }

    #[test]
    fn empty_lines_come_one_at_a_time() {
        assert_eq!(tidy("a\n\n\n\nb\n\n").0, "a\n\nb");
        assert_eq!(tidy("plain words").1, []);
    }
}
