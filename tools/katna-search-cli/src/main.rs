// SPDX-License-Identifier: GPL-3.0-or-later

//! Command-line indexing and search. See `docs/IMPLEMENTATION_PLAN.md` tasks
//! 0.4 (import) and 0.8 (index, query).

use std::convert::Infallible;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

use katna_core::Paths;
use katna_import::{Added, IncomingMessage, MessageSink, Options, Stats, StoreSink};
use katna_store::{Mode, Store};

const USAGE: &str = "\
usage: katna-search-cli import --data-dir DIR [--account NAME] [--folder NAME] <source>
       katna-search-cli import --dry-run [--folder NAME] <source>

Imports a Maildir tree (Maildir, Maildir++ or the Enron corpus layout) or an
mbox file into a Katna store. Importing the same source again only adds what
is new.

  --data-dir DIR   Store to write (DIR/data/mail.db, ...). Required, so
                   this development tool never writes the store of a running
                   katna-daemon by accident.
  --account NAME   Local account to import into; created if missing
                   (default: the source's file name)
  --folder NAME    Folder for the messages of an mbox file (default: its
                   file name)
  --dry-run        Read and parse only; report what would be imported";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("import") => import(&args[1..]),
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

#[derive(Default)]
struct ImportArgs {
    source: Option<PathBuf>,
    data_dir: Option<PathBuf>,
    account: Option<String>,
    folder: Option<String>,
    dry_run: bool,
}

fn import(args: &[String]) -> ExitCode {
    let mut parsed = ImportArgs::default();
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        let slot = match arg.as_str() {
            "--data-dir" => {
                parsed.data_dir = args.next().map(PathBuf::from);
                continue;
            }
            "--account" => &mut parsed.account,
            "--folder" => &mut parsed.folder,
            "--dry-run" => {
                parsed.dry_run = true;
                continue;
            }
            "-h" | "--help" => {
                println!("{USAGE}");
                return ExitCode::SUCCESS;
            }
            _ if parsed.source.is_none() && !arg.starts_with('-') => {
                parsed.source = Some(PathBuf::from(arg));
                continue;
            }
            _ => return usage_error(),
        };
        *slot = args.next().cloned();
    }
    let Some(source) = parsed.source.take() else {
        return usage_error();
    };
    if parsed.data_dir.is_none() == !parsed.dry_run {
        return usage_error();
    }

    let started = Instant::now();
    let result = match parsed.data_dir.take() {
        None => run(&source, &parsed, &mut CountingSink),
        Some(dir) => {
            let mut store = match Store::open(&Paths::with_root(&dir), Mode::ReadWrite) {
                Ok(store) => store,
                Err(err) => {
                    eprintln!("error: {err}");
                    return ExitCode::FAILURE;
                }
            };
            let name = parsed.account.clone().unwrap_or_else(|| file_name(&source));
            let account = match StoreSink::local_account(&mut store, &name) {
                Ok(account) => account,
                Err(err) => {
                    eprintln!("error: {err}");
                    return ExitCode::FAILURE;
                }
            };
            eprintln!("Importing into account {name:?} in {}", dir.display());
            run(
                &source,
                &parsed,
                &mut StoreSink::new(&mut store, account.id),
            )
        }
    };
    let stats = match result {
        Ok(stats) => stats,
        Err(err) => {
            eprintln!("error: {err}");
            return ExitCode::FAILURE;
        }
    };

    let seconds = started.elapsed().as_secs_f64();
    let verb = if parsed.dry_run { "read" } else { "imported" };
    println!(
        "{} messages ({:.1} MiB) {verb} in {seconds:.1} s ({:.0} messages/s); \
         {} copies in other folders, {} already present, {} skipped",
        stats.imported,
        stats.bytes as f64 / (1024.0 * 1024.0),
        stats.imported as f64 / seconds.max(f64::EPSILON),
        stats.copies,
        stats.duplicates,
        stats.skipped.len(),
    );
    for skipped in stats.skipped.iter().take(20) {
        println!("  skipped {}: {}", skipped.source, skipped.reason);
    }
    ExitCode::SUCCESS
}

fn run(
    source: &Path,
    args: &ImportArgs,
    sink: &mut impl MessageSink,
) -> Result<Stats, katna_import::Error> {
    let options = Options::default();
    let progress = |stats: &Stats| {
        let done = stats.imported + stats.copies + stats.duplicates;
        if done % 10_000 < options.batch_size as u64 {
            eprintln!("  {done} messages");
        }
    };
    if source.is_dir() {
        katna_import::import_maildir(source, sink, &options, progress)
    } else {
        let folder = args.folder.clone().unwrap_or_else(|| file_name(source));
        katna_import::import_mbox(source, &folder, sink, &options, progress)
    }
}

fn file_name(path: &Path) -> String {
    path.file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())
        .unwrap_or_else(|| "Imported".to_owned())
}

/// Counts messages without storing them (`--dry-run`).
struct CountingSink;

impl MessageSink for CountingSink {
    type Error = Infallible;

    fn write(&mut self, batch: &[IncomingMessage]) -> Result<Vec<Added>, Infallible> {
        Ok(vec![Added::New; batch.len()])
    }
}
