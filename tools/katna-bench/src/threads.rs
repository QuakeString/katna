// SPDX-License-Identifier: GPL-3.0-or-later

//! `katna-bench threads`: times the threading backfill and the conversation
//! list reads on a store (`docs/ARCHITECTURE.md` §6.5).

use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use katna_core::Paths;
use katna_store::{MailCategory, Mode, Store, ThreadId};

use crate::{ms, usage_error};

pub fn run(args: &[String]) -> ExitCode {
    let mut data_dir = None;
    let mut reset = false;
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--data-dir" => match args.next() {
                Some(dir) => data_dir = Some(PathBuf::from(dir)),
                None => return usage_error(),
            },
            "--reset" => reset = true,
            _ => return usage_error(),
        }
    }
    let Some(data_dir) = data_dir else {
        return usage_error();
    };
    match measure(&Paths::with_root(&data_dir), reset) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}

fn measure(paths: &Paths, reset: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut store = Store::open(paths, Mode::ReadWrite)?;
    let messages = store.message_count()?;
    if reset {
        let started = Instant::now();
        store.reset_threads()?;
        println!(
            "reset threads of {messages} messages: {:.0} ms",
            ms(started)
        );
    }

    let left = store.unthreaded_count()?;
    let started = Instant::now();
    let changed = katna_import::backfill::run(&mut store, katna_import::backfill::BATCH, |_| true)?;
    let elapsed = ms(started);
    println!(
        "backfill: {changed} of {left} messages in {elapsed:.0} ms ({:.0} messages/s)",
        changed as f64 / (elapsed / 1000.0).max(1e-9)
    );
    drop(store);

    let store = Store::open(paths, Mode::ReadOnly)?;
    let Some(folder) = store
        .folder_summaries()?
        .into_iter()
        .max_by_key(|f| f.total)
    else {
        return Ok(());
    };
    println!(
        "largest folder: {} ({} messages)",
        folder.path, folder.total
    );

    let started = Instant::now();
    let entries = store.folder_threads(folder.id, None)?;
    println!(
        "folder_threads: {} conversations in {:.1} ms",
        entries.len(),
        ms(started)
    );
    let started = Instant::now();
    let primary = store.folder_threads(folder.id, Some(MailCategory::Primary))?;
    println!(
        "folder_threads(primary): {} in {:.1} ms",
        primary.len(),
        ms(started)
    );
    let started = Instant::now();
    let tabs = store.category_unread(folder.id)?;
    println!("category_unread: {tabs:?} in {:.1} ms", ms(started));

    let page: Vec<ThreadId> = entries.iter().filter_map(|e| e.thread).take(50).collect();
    let started = Instant::now();
    let summaries = store.thread_summaries(&page, folder.id)?;
    println!(
        "thread_summaries: {} in {:.1} ms",
        summaries.len(),
        ms(started)
    );
    let started = Instant::now();
    let mut total = 0;
    for &thread in &page {
        total += store.thread_messages(thread)?.len();
    }
    println!(
        "thread_messages: {} threads, {total} messages in {:.1} ms",
        page.len(),
        ms(started)
    );
    let biggest = summaries.iter().map(|s| s.message_count).max().unwrap_or(0);
    println!("largest conversation on the first page: {biggest} messages");
    Ok(())
}
