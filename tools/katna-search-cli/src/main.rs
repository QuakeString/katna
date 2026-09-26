// SPDX-License-Identifier: GPL-3.0-or-later

//! Command-line indexing and search. See `docs/IMPLEMENTATION_PLAN.md` tasks
//! 0.4 (import) and 0.8 (index, query).

use std::convert::Infallible;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use katna_import::{Added, IncomingMessage, MessageSink, Options, Stats};

const USAGE: &str = "\
usage: katna-search-cli import [--folder NAME] <maildir-or-mbox>

Reads a Maildir tree (Maildir, Maildir++ or the Enron corpus layout) or an
mbox file and reports what would be imported. Messages are not stored yet;
that follows once katna-store has its message API.

  --folder NAME   Folder for the messages of an mbox file (default: file name)";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("import") => import(&args[1..]),
        _ => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
    }
}

fn import(args: &[String]) -> ExitCode {
    let mut folder = None;
    let mut source = None;
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--folder" => folder = args.next().cloned(),
            "-h" | "--help" => {
                println!("{USAGE}");
                return ExitCode::SUCCESS;
            }
            _ if source.is_none() && !arg.starts_with('-') => source = Some(PathBuf::from(arg)),
            _ => {
                eprintln!("{USAGE}");
                return ExitCode::from(2);
            }
        }
    }
    let Some(source) = source else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };

    let started = Instant::now();
    let progress = |stats: &Stats| eprintln!("  {} messages", stats.imported);
    let mut sink = CountingSink;
    let options = Options {
        batch_size: 10_000,
        ..Options::default()
    };
    let result = if source.is_dir() {
        katna_import::import_maildir(&source, &mut sink, &options, progress)
    } else {
        let folder = folder.unwrap_or_else(|| {
            source
                .file_stem()
                .map(|stem| stem.to_string_lossy().into_owned())
                .unwrap_or_else(|| "Imported".to_owned())
        });
        katna_import::import_mbox(&source, &folder, &mut sink, &options, progress)
    };
    let stats = match result {
        Ok(stats) => stats,
        Err(err) => {
            eprintln!("error: {err}");
            return ExitCode::FAILURE;
        }
    };

    let seconds = started.elapsed().as_secs_f64();
    println!(
        "{} messages ({:.1} MiB) read in {seconds:.1} s ({:.0} messages/s), {} skipped",
        stats.imported,
        stats.bytes as f64 / (1024.0 * 1024.0),
        stats.imported as f64 / seconds.max(f64::EPSILON),
        stats.skipped.len(),
    );
    for skipped in stats.skipped.iter().take(20) {
        println!("  skipped {}: {}", skipped.source, skipped.reason);
    }
    ExitCode::SUCCESS
}

/// Stands in for katna-store until it can store messages.
struct CountingSink;

impl MessageSink for CountingSink {
    type Error = Infallible;

    fn add(&mut self, _message: IncomingMessage<'_>) -> Result<Added, Infallible> {
        Ok(Added::New)
    }

    fn commit(&mut self) -> Result<(), Infallible> {
        Ok(())
    }
}
