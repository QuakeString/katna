// SPDX-License-Identifier: GPL-3.0-or-later

//! Maildir and mbox import (implementation plan task 0.4), used for the Enron
//! benchmark corpus and for users migrating from other mail clients.
//!
//! The importer reads messages ([`maildir`], [`mbox`]), extracts the fields
//! the store indexes ([`parse`]) and hands each message to a [`MessageSink`],
//! in batches. The sink is the store ([`StoreSink`]); tests use an in-memory one.

pub mod maildir;
pub mod mbox;
pub mod parse;
pub mod store;

use std::error::Error as StdError;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, BufReader};
use std::path::Path;

pub use parse::{ParsedMessage, Participant, Role, parse_message};
pub use store::StoreSink;

/// IMAP system flags of a message, as far as local formats record them.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Flags {
    /// `\Seen` — Maildir `S`, mbox `Status: R`.
    pub seen: bool,
    /// `\Answered` — Maildir `R`, mbox `X-Status: A`.
    pub answered: bool,
    /// `\Flagged` — Maildir `F`, mbox `X-Status: F`.
    pub flagged: bool,
    /// `\Draft` — Maildir `D`, mbox `X-Status: T`.
    pub draft: bool,
    /// `\Deleted` — Maildir `T` (trashed), mbox `X-Status: D`.
    pub deleted: bool,
    /// `$Forwarded` — Maildir `P` (passed).
    pub forwarded: bool,
}

impl Flags {
    /// Reads the flags from a Maildir file name (`<unique>:2,<letters>`; `!`
    /// is accepted instead of `:` as some tools use it on other file systems).
    pub fn from_maildir_name(name: &str) -> Self {
        let mut flags = Self::default();
        let info = name
            .rsplit_once(":2,")
            .or_else(|| name.rsplit_once("!2,"))
            .map(|(_, info)| info);
        for letter in info.unwrap_or_default().chars() {
            match letter {
                'S' => flags.seen = true,
                'R' => flags.answered = true,
                'F' => flags.flagged = true,
                'D' => flags.draft = true,
                'T' => flags.deleted = true,
                'P' => flags.forwarded = true,
                _ => {}
            }
        }
        flags
    }
}

/// A parsed message waiting to be written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncomingMessage {
    /// Folder path with `/` separators (`allen-p/inbox`, `INBOX`, `Sent`).
    pub folder: String,
    pub flags: Flags,
    /// The message exactly as read from disk.
    pub raw: Vec<u8>,
    pub parsed: ParsedMessage,
}

/// What happened to one message of a batch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Added {
    New,
    /// The same message was already stored in another folder and is now in
    /// this one too (Enron has many: `all_documents`, `discussion_threads`).
    Copy,
    /// The same message is already in this folder (re-running an import).
    Duplicate,
}

/// Where imported messages go: the store, or a test double.
pub trait MessageSink {
    type Error: StdError + Send + Sync + 'static;

    /// Writes a batch durably, all or nothing. Returns one result per message,
    /// in order.
    fn write(&mut self, batch: &[IncomingMessage]) -> Result<Vec<Added>, Self::Error>;
}

/// Import settings.
#[derive(Debug, Clone)]
pub struct Options {
    /// Messages per commit.
    pub batch_size: usize,
    /// Larger messages are skipped (bytes).
    pub max_message_size: u64,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            batch_size: 1000,
            max_message_size: 128 * 1024 * 1024,
        }
    }
}

/// A message that was not imported, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Skipped {
    /// File path, or `<mbox path>#<n>` for the n-th message (from 1) of an mbox.
    pub source: String,
    pub reason: String,
}

/// Result of an import.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Stats {
    /// New messages.
    pub imported: u64,
    /// Messages already stored in another folder (see [`Added::Copy`]).
    pub copies: u64,
    pub duplicates: u64,
    /// Raw bytes of imported messages.
    pub bytes: u64,
    pub skipped: Vec<Skipped>,
}

/// An import that stopped.
#[derive(Debug)]
pub enum Error {
    /// The source could not be opened.
    Source(io::Error),
    /// The store failed; the batch being written is lost, earlier ones are kept.
    Sink(Box<dyn StdError + Send + Sync>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Source(err) => write!(f, "cannot read import source: {err}"),
            Error::Sink(err) => write!(f, "cannot store message: {err}"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Source(err) => Some(err),
            Error::Sink(err) => Some(err.as_ref()),
        }
    }
}

/// Imports every message below `root` (Maildir, Maildir++ or a plain tree
/// such as the Enron corpus; see [`maildir`]).
///
/// Unreadable or unparsable messages are recorded in [`Stats::skipped`] and
/// the import goes on. `progress` is called after each commit.
pub fn import_maildir<S: MessageSink>(
    root: &Path,
    sink: &mut S,
    options: &Options,
    progress: impl FnMut(&Stats),
) -> Result<Stats, Error> {
    let mut run = Run::new(sink, options, progress);
    for entry in maildir::Walker::new(root).map_err(Error::Source)? {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                run.skip(String::new(), err.to_string());
                continue;
            }
        };
        let source = entry.path.display().to_string();
        match fs::metadata(&entry.path) {
            Ok(meta) if meta.len() > options.max_message_size => {
                run.skip(
                    source,
                    format!("larger than {} bytes", options.max_message_size),
                );
                continue;
            }
            Ok(_) => {}
            Err(err) => {
                run.skip(source, err.to_string());
                continue;
            }
        }
        match fs::read(&entry.path) {
            Ok(raw) => run.add(source, entry.folder, entry.flags, raw)?,
            Err(err) => run.skip(source, err.to_string()),
        }
    }
    run.finish()
}

/// Imports every message of the mbox file at `path` into `folder`.
pub fn import_mbox<S: MessageSink>(
    path: &Path,
    folder: &str,
    sink: &mut S,
    options: &Options,
    progress: impl FnMut(&Stats),
) -> Result<Stats, Error> {
    let file = File::open(path).map_err(Error::Source)?;
    let mut run = Run::new(sink, options, progress);
    for (index, entry) in mbox::Reader::new(BufReader::new(file)).enumerate() {
        let source = format!("{}#{}", path.display(), index + 1);
        match entry {
            Ok(entry) if entry.raw.len() as u64 > options.max_message_size => {
                run.skip(
                    source,
                    format!("larger than {} bytes", options.max_message_size),
                );
            }
            Ok(entry) => run.add(source, folder.to_owned(), entry.flags, entry.raw)?,
            // A read error mid-file leaves the reader in an unknown position.
            Err(err) => return Err(Error::Source(err)),
        }
    }
    run.finish()
}

/// Shared bookkeeping of one import: batching, counting, progress.
struct Run<'a, S, P> {
    sink: &'a mut S,
    batch_size: usize,
    batch: Vec<IncomingMessage>,
    stats: Stats,
    progress: P,
}

impl<'a, S: MessageSink, P: FnMut(&Stats)> Run<'a, S, P> {
    fn new(sink: &'a mut S, options: &Options, progress: P) -> Self {
        let batch_size = options.batch_size.max(1);
        Self {
            sink,
            batch_size,
            batch: Vec::with_capacity(batch_size),
            stats: Stats::default(),
            progress,
        }
    }

    fn skip(&mut self, source: String, reason: String) {
        self.stats.skipped.push(Skipped { source, reason });
    }

    fn add(
        &mut self,
        source: String,
        folder: String,
        flags: Flags,
        raw: Vec<u8>,
    ) -> Result<(), Error> {
        let Some(parsed) = parse_message(&raw) else {
            self.skip(source, "not an email message".into());
            return Ok(());
        };
        self.batch.push(IncomingMessage {
            folder,
            flags,
            raw,
            parsed,
        });
        if self.batch.len() >= self.batch_size {
            self.flush()?;
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Error> {
        let results = self
            .sink
            .write(&self.batch)
            .map_err(|err| Error::Sink(Box::new(err)))?;
        for (message, added) in self.batch.iter().zip(results) {
            match added {
                Added::New => {
                    self.stats.imported += 1;
                    self.stats.bytes += message.raw.len() as u64;
                }
                Added::Copy => self.stats.copies += 1,
                Added::Duplicate => self.stats.duplicates += 1,
            }
        }
        self.batch.clear();
        (self.progress)(&self.stats);
        Ok(())
    }

    fn finish(mut self) -> Result<Stats, Error> {
        if !self.batch.is_empty() {
            self.flush()?;
        }
        Ok(self.stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::convert::Infallible;

    /// Keeps messages in memory; duplicates are the same bytes in the same folder.
    #[derive(Default)]
    struct MemorySink {
        committed: Vec<(String, Flags, Option<String>)>,
        seen: HashSet<(String, Vec<u8>)>,
        batches: usize,
    }

    impl MessageSink for MemorySink {
        type Error = Infallible;

        fn write(&mut self, batch: &[IncomingMessage]) -> Result<Vec<Added>, Infallible> {
            self.batches += 1;
            Ok(batch
                .iter()
                .map(|message| {
                    if !self
                        .seen
                        .insert((message.folder.clone(), message.raw.clone()))
                    {
                        return Added::Duplicate;
                    }
                    self.committed.push((
                        message.folder.clone(),
                        message.flags,
                        message.parsed.subject.clone(),
                    ));
                    Added::New
                })
                .collect())
        }
    }

    fn write(root: &Path, rel: &str, contents: &[u8]) {
        let path = root.join(rel);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, contents).unwrap();
    }

    #[test]
    fn maildir_flags() {
        let cases = [
            (
                "1700000000.M1.host:2,FRS",
                (true, true, true, false, false, false),
            ),
            (
                "1700000000.M1.host!2,DPT",
                (false, false, false, true, true, true),
            ),
            (
                "1700000000.M1.host:2,",
                (false, false, false, false, false, false),
            ),
            (
                "1700000000.M1.host",
                (false, false, false, false, false, false),
            ),
        ];
        for (name, (seen, answered, flagged, draft, deleted, forwarded)) in cases {
            let expected = Flags {
                seen,
                answered,
                flagged,
                draft,
                deleted,
                forwarded,
            };
            assert_eq!(Flags::from_maildir_name(name), expected, "{name}");
        }
    }

    #[test]
    fn imports_a_tree_in_batches_and_skips_junk() {
        let dir = tempfile::tempdir().unwrap();
        for n in 1..=5 {
            let body = format!("Subject: message {n}\r\n\r\nbody\r\n");
            write(dir.path(), &format!("lay-k/inbox/{n}."), body.as_bytes());
        }
        write(dir.path(), "lay-k/inbox/6.", b"");
        write(dir.path(), "lay-k/inbox/7.", &[b'x'; 64]);

        let mut sink = MemorySink::default();
        let mut progress = Vec::new();
        let options = Options {
            batch_size: 2,
            max_message_size: 63,
        };
        let stats = import_maildir(dir.path(), &mut sink, &options, |stats| {
            progress.push(stats.imported)
        })
        .unwrap();

        assert_eq!(stats.imported, 5);
        assert_eq!(stats.duplicates, 0);
        let reasons: Vec<_> = stats
            .skipped
            .iter()
            .map(|s| {
                (
                    Path::new(&s.source).file_name().unwrap().to_owned(),
                    s.reason.as_str(),
                )
            })
            .collect();
        assert_eq!(
            reasons,
            [
                ("6.".into(), "not an email message"),
                ("7.".into(), "larger than 63 bytes"),
            ]
        );
        assert_eq!(progress, [2, 4, 5]);
        assert_eq!(sink.batches, 3);
        assert_eq!(sink.committed.len(), 5);
        assert_eq!(sink.committed[0].0, "lay-k/inbox");
        assert_eq!(sink.committed[4].2.as_deref(), Some("message 5"));

        // A second run finds only duplicates.
        let again = import_maildir(dir.path(), &mut sink, &options, |_| {}).unwrap();
        assert_eq!((again.imported, again.duplicates), (0, 5));
    }

    #[test]
    fn imports_an_mbox() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Archive.mbox");
        fs::write(
            &path,
            b"From a@example.org Mon Jan  1 10:00:00 2024\nStatus: RO\nSubject: one\n\nx\n\n\
              From b@example.org Mon Jan  1 11:00:00 2024\nSubject: two\n\ny\n",
        )
        .unwrap();

        let mut sink = MemorySink::default();
        let stats = import_mbox(&path, "Archive", &mut sink, &Options::default(), |_| {}).unwrap();
        assert_eq!(stats.imported, 2);
        assert_eq!(sink.committed[0].0, "Archive");
        assert!(sink.committed[0].1.seen);
        assert!(!sink.committed[1].1.seen);
    }

    #[test]
    fn missing_source_is_an_error() {
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("missing");
        let mut sink = MemorySink::default();
        let options = Options::default();
        assert!(matches!(
            import_maildir(&missing, &mut sink, &options, |_| {}),
            Err(Error::Source(_))
        ));
        assert!(matches!(
            import_mbox(&missing, "x", &mut sink, &options, |_| {}),
            Err(Error::Source(_))
        ));
    }
}
