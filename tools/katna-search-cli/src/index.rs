// SPDX-License-Identifier: GPL-3.0-or-later

//! `index` and `query` (implementation plan task 0.8).

use std::io::{IsTerminal, Write};
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use katna_core::Paths;
use katna_search::{IndexOptions, Query, SearchIndex, SearchOptions, Snippet, Sort};
use katna_store::{Mode, ParticipantRole, Store};

use crate::usage_error;

/// Width of the snippet line.
const SNIPPET_CHARS: usize = 120;

pub fn index(args: &[String]) -> ExitCode {
    let mut data_dir = None;
    let mut rebuild = false;
    let mut options = IndexOptions::default();
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--data-dir" => data_dir = args.next().map(PathBuf::from),
            "--rebuild" => rebuild = true,
            "--threads" => match args.next().and_then(|n| n.parse().ok()) {
                Some(n) => options.parse_threads = n,
                None => return usage_error(),
            },
            _ => return usage_error(),
        }
    }
    let Some(data_dir) = data_dir else {
        return usage_error();
    };
    match run_index(&Paths::with_root(&data_dir), rebuild, &options) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run_index(
    paths: &Paths,
    rebuild: bool,
    options: &IndexOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read-only: indexing never changes the store.
    let store = Store::open(paths, Mode::ReadOnly)?;
    let dir = paths.index_dir();
    if rebuild {
        SearchIndex::delete(&dir)?;
    }
    let index = SearchIndex::open(&dir)?;
    let total = store.message_count()?;
    let before = index.num_docs();
    eprintln!(
        "Indexing {total} messages into {} ({before} indexed so far)",
        dir.display()
    );
    let started = Instant::now();
    let stats = index.update(&store, options, |done| {
        if done > 0 {
            let seconds = started.elapsed().as_secs_f64();
            eprintln!(
                "  {done} messages ({:.0} messages/s)",
                done as f64 / seconds.max(f64::EPSILON)
            );
        }
    })?;
    let seconds = started.elapsed().as_secs_f64();
    println!(
        "{} messages indexed, {} removed in {seconds:.1} s ({:.0} messages/s); \
         {} without stored text; index has {} messages, {:.1} MiB",
        stats.indexed,
        stats.removed,
        stats.indexed as f64 / seconds.max(f64::EPSILON),
        stats.without_text,
        index.num_docs(),
        dir_size(&dir) as f64 / (1024.0 * 1024.0),
    );
    Ok(())
}

fn dir_size(dir: &std::path::Path) -> u64 {
    std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(|entry| entry.ok()?.metadata().ok())
                .filter(|meta| meta.is_file())
                .map(|meta| meta.len())
                .sum()
        })
        .unwrap_or_default()
}

pub fn query(args: &[String]) -> ExitCode {
    let mut data_dir = None;
    let mut options = SearchOptions::default();
    let mut words = Vec::new();
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--data-dir" => data_dir = args.next().map(PathBuf::from),
            "--limit" => match args.next().and_then(|n| n.parse().ok()) {
                Some(n) => options.limit = n,
                None => return usage_error(),
            },
            "--sort" => match args.next().map(String::as_str) {
                Some("auto") => options.sort = Sort::Auto,
                Some("relevance") => options.sort = Sort::Relevance,
                Some("newest") => options.sort = Sort::Newest,
                Some("oldest") => options.sort = Sort::Oldest,
                _ => return usage_error(),
            },
            "--count" => options.count = true,
            "--" => words.extend(args.by_ref().cloned()),
            _ if arg.starts_with("--") => return usage_error(),
            _ => words.push(arg.clone()),
        }
    }
    let Some(data_dir) = data_dir else {
        return usage_error();
    };
    match run_query(&Paths::with_root(&data_dir), &words.join(" "), &options) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run_query(
    paths: &Paths,
    input: &str,
    options: &SearchOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let opened = Instant::now();
    let store = Store::open(paths, Mode::ReadOnly)?;
    let index = SearchIndex::open_read_only(&paths.index_dir())?;
    let open_ms = ms(opened);

    let started = Instant::now();
    let query = Query::parse(input)?;
    let results = index.search(&query, options)?;
    let search_ms = ms(started);

    let shown = Instant::now();
    let ids: Vec<_> = results.hits.iter().map(|hit| hit.message).collect();
    let messages = store.messages_by_id(&ids)?;
    let snippets = index.snippets(&store, &query, &ids, SNIPPET_CHARS)?;
    let display_ms = ms(shown);

    let color = std::io::stdout().is_terminal();
    let mut out = std::io::stdout().lock();
    for (id, snippet) in ids.iter().zip(&snippets) {
        let Some(message) = messages.iter().find(|m| m.id == *id) else {
            continue;
        };
        let from = message
            .first(ParticipantRole::From)
            .map(|p| {
                p.display_name
                    .clone()
                    .unwrap_or_else(|| p.email_norm.clone())
            })
            .unwrap_or_default();
        let subject = if message.subject.is_empty() {
            "(no subject)"
        } else {
            &message.subject
        };
        writeln!(
            out,
            "{}  {:<28.28}  {subject}  [{id}]",
            message
                .date
                .map(format_date)
                .unwrap_or_else(|| "????-??-??".into()),
            from,
        )?;
        if !snippet.text.is_empty() {
            writeln!(out, "            {}", highlight(snippet, color))?;
        }
    }
    let total = match results.total {
        Some(total) => format!(" of {total}"),
        None => String::new(),
    };
    eprintln!(
        "{}{total} results in {:.1} ms (search {search_ms:.1} ms, subjects and snippets \
         {display_ms:.1} ms; opening the store and index {open_ms:.1} ms)",
        results.hits.len(),
        search_ms + display_ms,
    );
    Ok(())
}

fn ms(since: Instant) -> f64 {
    since.elapsed().as_secs_f64() * 1000.0
}

/// Marks the highlighted ranges in bold on a terminal, else with `*…*`.
fn highlight(snippet: &Snippet, color: bool) -> String {
    let (on, off) = if color {
        ("\x1b[1m", "\x1b[0m")
    } else {
        ("*", "*")
    };
    let mut out = String::new();
    let mut at = 0;
    for range in &snippet.highlights {
        if range.start < at || range.end > snippet.text.len() {
            continue;
        }
        out.push_str(&snippet.text[at..range.start]);
        out.push_str(on);
        out.push_str(&snippet.text[range.clone()]);
        out.push_str(off);
        at = range.end;
    }
    out.push_str(&snippet.text[at..]);
    out.replace(['\r', '\n'], " ")
}

/// Unix seconds → `YYYY-MM-DD` (UTC).
fn format_date(time: i64) -> String {
    let days = time.div_euclid(86_400);
    // Howard Hinnant's civil_from_days.
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_dates() {
        assert_eq!(format_date(0), "1970-01-01");
        assert_eq!(format_date(989_883_540), "2001-05-14");
        assert_eq!(format_date(951_782_400), "2000-02-29");
        assert_eq!(format_date(-86_400), "1969-12-31");
    }

    #[test]
    fn highlights() {
        let snippet = Snippet {
            text: "the budget\nis here".into(),
            highlights: vec![4..10, 14..18],
        };
        assert_eq!(highlight(&snippet, false), "the *budget* is *here*");
        assert_eq!(
            highlight(&snippet, true),
            "the \x1b[1mbudget\x1b[0m is \x1b[1mhere\x1b[0m"
        );
    }
}
