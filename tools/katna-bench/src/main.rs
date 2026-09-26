// SPDX-License-Identifier: GPL-3.0-or-later

//! Benchmarks on public corpora. See `docs/IMPLEMENTATION_PLAN.md` task 0.8.
//!
//! - `search` times a set of queries against an imported and indexed store
//!   (the Enron corpus in the nightly job) and checks the p99 budget.
//! - `synth` fills a store with a synthetic corpus of Enron's shape, for
//!   machines that cannot download Enron.

mod synth;

use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use katna_core::Paths;
use katna_search::{Query, SearchIndex, SearchOptions};
use katna_store::{Mode, Store};

const USAGE: &str = "\
usage: katna-bench search --data-dir DIR [--queries FILE] [--runs N] [--limit N]
                          [--max-p99-ms MS]
       katna-bench synth --data-dir DIR [--messages N] [--seed N]

search: Runs each query once cold, then --runs times (default 20), against
the store and search index in DIR (import with `katna-search-cli import`,
index with `katna-search-cli index`). A run is what showing results takes:
parse the query, search, then read subjects and make highlighted snippets
for the first --limit results (default 20). Prints per-query and overall
latency; exits with status 1 if the overall p99 exceeds --max-p99-ms.

  --queries FILE   One query per line; # starts a comment. Default: a set
                   written for the Enron corpus.

synth: Imports --messages (default 500000) synthetic messages with Enron's
shape (senders, folders, sizes, attachments, word frequencies) into the
store in DIR. The same --seed gives the same corpus.";

/// Queries written for the Enron corpus; the synthetic corpus uses the same
/// names and words.
const DEFAULT_QUERIES: &str = "\
# Phase 0 goal query
from:kenneth.lay has:attachment budget
budget
enron
\"natural gas\"
california power OR electricity
energy trading contract
from:jeff.skilling
to:tim.belden -from:phillip.allen
subject:meeting after:2001-01-01
has:attachment larger:100K
(gas OR power) price -subject:re
in:inbox is:unread
";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("search") => search(&args[1..]),
        Some("synth") => synth::run(&args[1..]),
        Some("-h" | "--help") => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        _ => usage_error(),
    }
}

fn usage_error() -> ExitCode {
    eprintln!("{USAGE}");
    ExitCode::from(2)
}

struct SearchArgs {
    data_dir: PathBuf,
    queries: Option<PathBuf>,
    runs: usize,
    limit: usize,
    max_p99_ms: Option<f64>,
}

fn search(args: &[String]) -> ExitCode {
    let mut data_dir = None;
    let mut parsed = SearchArgs {
        data_dir: PathBuf::new(),
        queries: None,
        runs: 20,
        limit: 20,
        max_p99_ms: None,
    };
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        let value = args.next();
        let ok = match arg.as_str() {
            "--data-dir" => {
                data_dir = value.map(PathBuf::from);
                data_dir.is_some()
            }
            "--queries" => {
                parsed.queries = value.map(PathBuf::from);
                parsed.queries.is_some()
            }
            "--runs" => value
                .and_then(|v| v.parse().ok())
                .map(|v| parsed.runs = v)
                .is_some(),
            "--limit" => value
                .and_then(|v| v.parse().ok())
                .map(|v| parsed.limit = v)
                .is_some(),
            "--max-p99-ms" => value
                .and_then(|v| v.parse().ok())
                .map(|v| parsed.max_p99_ms = Some(v))
                .is_some(),
            _ => false,
        };
        if !ok {
            return usage_error();
        }
    }
    let Some(data_dir) = data_dir else {
        return usage_error();
    };
    parsed.data_dir = data_dir;
    match run_search(&parsed) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}

/// Returns whether the budget (if any) was met.
fn run_search(args: &SearchArgs) -> Result<bool, Box<dyn std::error::Error>> {
    let text = match &args.queries {
        Some(path) => std::fs::read_to_string(path)?,
        None => DEFAULT_QUERIES.to_owned(),
    };
    let queries: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect();
    let paths = Paths::with_root(&args.data_dir);
    let store = Store::open(&paths, Mode::ReadOnly)?;
    let index = SearchIndex::open_read_only(&paths.index_dir())?;
    println!(
        "{} messages in the store, {} in the index; {} queries, {} runs each, first {} results",
        store.message_count()?,
        index.num_docs(),
        queries.len(),
        args.runs,
        args.limit,
    );
    println!(
        "{:<42} {:>8} {:>9} {:>9} {:>9} {:>9} {:>9}",
        "query", "matches", "cold ms", "search", "p50 ms", "p99 ms", "max ms"
    );

    let mut all = Vec::new();
    for input in &queries {
        let options = SearchOptions {
            limit: args.limit,
            ..SearchOptions::default()
        };
        let run = || -> Result<(f64, f64), Box<dyn std::error::Error>> {
            let started = Instant::now();
            let query = Query::parse(input)?;
            let results = index.search(&query, &options)?;
            let search_ms = ms(started);
            let ids: Vec<_> = results.hits.iter().map(|hit| hit.message).collect();
            let messages = store.messages_by_id(&ids)?;
            let snippets = index.snippets(&store, &query, &ids, 120)?;
            std::hint::black_box((messages, snippets));
            Ok((search_ms, ms(started)))
        };
        let (_, cold) = run()?;
        let mut times = Vec::with_capacity(args.runs);
        let mut search_times = Vec::with_capacity(args.runs);
        for _ in 0..args.runs {
            let (search_ms, total_ms) = run()?;
            search_times.push(search_ms);
            times.push(total_ms);
        }
        let matches = index
            .search(
                &Query::parse(input)?,
                &SearchOptions {
                    limit: 1,
                    count: true,
                    ..SearchOptions::default()
                },
            )?
            .total
            .unwrap_or_default();
        times.sort_by(f64::total_cmp);
        search_times.sort_by(f64::total_cmp);
        println!(
            "{:<42} {matches:>8} {cold:>9.1} {:>9.1} {:>9.1} {:>9.1} {:>9.1}",
            truncate(input, 42),
            percentile(&search_times, 50.0),
            percentile(&times, 50.0),
            percentile(&times, 99.0),
            times.last().copied().unwrap_or_default(),
        );
        all.extend(times);
    }
    all.sort_by(f64::total_cmp);
    let p99 = percentile(&all, 99.0);
    println!(
        "overall: p50 {:.1} ms, p95 {:.1} ms, p99 {p99:.1} ms, max {:.1} ms over {} runs",
        percentile(&all, 50.0),
        percentile(&all, 95.0),
        all.last().copied().unwrap_or_default(),
        all.len(),
    );
    Ok(match args.max_p99_ms {
        Some(budget) if p99 > budget => {
            println!("FAIL: p99 {p99:.1} ms is over the budget of {budget} ms");
            false
        }
        Some(budget) => {
            println!("ok: p99 {p99:.1} ms is within the budget of {budget} ms");
            true
        }
        None => true,
    })
}

fn ms(since: Instant) -> f64 {
    since.elapsed().as_secs_f64() * 1000.0
}

/// Nearest-rank percentile of sorted `values`.
fn percentile(values: &[f64], p: f64) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let rank = ((p / 100.0) * values.len() as f64).ceil() as usize;
    values[rank.clamp(1, values.len()) - 1]
}

fn truncate(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        return text.to_owned();
    }
    let mut out: String = text.chars().take(max - 1).collect();
    out.push('…');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percentiles() {
        let values: Vec<f64> = (1..=100).map(f64::from).collect();
        assert_eq!(percentile(&values, 50.0), 50.0);
        assert_eq!(percentile(&values, 99.0), 99.0);
        assert_eq!(percentile(&values, 100.0), 100.0);
        assert_eq!(percentile(&[3.0], 99.0), 3.0);
        assert_eq!(percentile(&[], 50.0), 0.0);
    }

    #[test]
    fn default_queries_parse() {
        for line in DEFAULT_QUERIES.lines().filter(|l| !l.starts_with('#')) {
            Query::parse(line).unwrap();
        }
    }
}
