// SPDX-License-Identifier: GPL-3.0-or-later

//! The index on disk: building it from the store, keeping it up to date from
//! the change journal, and searching it (`docs/ARCHITECTURE.md` §7.3–7.5).

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::io;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

use katna_store::{DbKind, MessageFlags, MessageId, ObjectKind, Store, StoredMessage};
use serde::{Deserialize, Serialize};
use tantivy::collector::{Count, TopDocs};
use tantivy::{DocAddress, Index, IndexReader, IndexWriter, Order, ReloadPolicy, Searcher, Term};

use crate::compile::{Fuzziness, compile_with};
use crate::document::{self, MessageText};
use crate::error::{Error, Result};
use crate::highlight::Highlighter;
use crate::query::{Query, TextField};
use crate::schema::{self, Fields, SCHEMA_VERSION};

/// Messages read from the store per query while indexing.
const READ_BATCH: u32 = 500;
/// Journal entries handled per incremental step.
const JOURNAL_BATCH: u32 = 5_000;

/// Snippets are made from at most this much of the raw message.
const SNIPPET_SCAN_BYTES: usize = 64 * 1024;

/// Recency boost: a message this many days older than the newest one gets
/// half the boost.
const RECENCY_HALF_LIFE_DAYS: f32 = 60.0;
/// Weight of the recency boost: the newest message scores up to this factor
/// higher than a very old one with the same text score.
const RECENCY_WEIGHT: f32 = 0.5;
/// Starred messages score this factor higher.
const FLAGGED_BOOST: f32 = 1.2;

/// How to index.
#[derive(Debug, Clone)]
pub struct IndexOptions {
    /// Threads that parse messages.
    pub parse_threads: usize,
    /// Threads of tantivy's index writer.
    pub writer_threads: usize,
    /// Memory budget of the index writer, in bytes, for all its threads.
    pub memory_budget: usize,
    /// Commit (make searchable and crash-safe) after this many messages.
    pub commit_every: u64,
    /// When set, [`SearchIndex::update`] commits what it has done and
    /// returns early; the next update carries on from there.
    pub stop: Option<Arc<AtomicBool>>,
}

impl IndexOptions {
    fn stopped(&self) -> bool {
        self.stop
            .as_ref()
            .is_some_and(|stop| stop.load(Ordering::Relaxed))
    }
}

impl Default for IndexOptions {
    fn default() -> Self {
        let cpus = thread::available_parallelism().map_or(2, |n| n.get());
        Self {
            parse_threads: cpus.clamp(1, 8),
            writer_threads: cpus.clamp(1, 4),
            memory_budget: 512 << 20,
            commit_every: 100_000,
            stop: None,
        }
    }
}

/// What an update did.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct UpdateStats {
    /// Documents added (new or changed messages).
    pub indexed: u64,
    /// Messages removed from the index because they left the store.
    pub removed: u64,
    /// Messages indexed without text because their raw message is missing.
    pub without_text: u64,
}

/// Progress saved with every commit, as the commit payload, so it is
/// atomic with the documents it describes.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexState {
    pub schema_version: u32,
    /// The `mail.db` journal is indexed up to this sequence number. `None`
    /// until the first full scan has finished.
    pub change_seq: Option<i64>,
    /// First full scan in progress: messages up to this ID are indexed.
    pub scan_after: Option<i64>,
    /// Journal sequence number when the first full scan started; changes
    /// after it are applied once the scan finishes.
    pub scan_seq: Option<i64>,
}

/// How to order results.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Sort {
    /// [`Relevance`](Sort::Relevance) when the query has free text,
    /// otherwise [`Newest`](Sort::Newest).
    #[default]
    Auto,
    /// BM25 with recency and starred boosts.
    Relevance,
    Newest,
    Oldest,
}

/// How to search.
#[derive(Debug, Clone, Copy)]
pub struct SearchOptions {
    pub limit: usize,
    pub offset: usize,
    pub sort: Sort,
    /// Also count all matches (slower for broad queries).
    pub count: bool,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            limit: 20,
            offset: 0,
            sort: Sort::Auto,
            count: false,
        }
    }
}

/// One result.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hit {
    pub message: MessageId,
    /// Relevance score, or 0 when sorted by date.
    pub score: f32,
    pub date: Option<i64>,
}

/// Results of [`SearchIndex::search`].
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SearchResults {
    pub hits: Vec<Hit>,
    /// Number of all matches, when [`SearchOptions::count`] was set.
    pub total: Option<usize>,
    /// Nothing matched the words as typed, so these results match words a
    /// typo or two away ("showing results for similar words").
    pub fuzzy: bool,
}

/// A piece of a message's text with the matched words marked.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Snippet {
    pub text: String,
    /// Byte ranges of `text` to highlight.
    pub highlights: Vec<Range<usize>>,
}

/// The search index in one directory.
pub struct SearchIndex {
    index: Index,
    fields: Fields,
    reader: IndexReader,
    dir: PathBuf,
}

impl SearchIndex {
    /// Opens the index in `dir` for indexing and searching, creating it if
    /// needed. An index built with another [`SCHEMA_VERSION`] is deleted and
    /// created again empty, so the next [`update`](Self::update) rebuilds it.
    pub fn open(dir: &Path) -> Result<Self> {
        if dir.join("meta.json").is_file() {
            match Self::open_existing(dir, ReloadPolicy::Manual) {
                Err(Error::SchemaVersion { found, .. }) => {
                    tracing::warn!(
                        path = %dir.display(),
                        found,
                        expected = SCHEMA_VERSION,
                        "search index has another schema version; rebuilding it"
                    );
                    Self::delete(dir)?;
                }
                opened => return opened,
            }
        }
        fs::create_dir_all(dir).map_err(|source| io_error(dir, source))?;
        let index = Index::create_in_dir(dir, schema::build_schema())?;
        schema::register_tokenizers(index.tokenizers());
        let mut writer: IndexWriter = index.writer_with_num_threads(1, 15_000_000)?;
        commit(
            &mut writer,
            &IndexState {
                schema_version: SCHEMA_VERSION,
                ..IndexState::default()
            },
        )?;
        Self::open_existing(dir, ReloadPolicy::Manual)
    }

    /// Opens an existing index for searching only, as apps do while the
    /// daemon writes it. Searches see the daemon's commits by themselves,
    /// within about half a second; [`reload`](Self::reload) makes them
    /// visible at once.
    pub fn open_read_only(dir: &Path) -> Result<Self> {
        if !dir.join("meta.json").is_file() {
            return Err(Error::NotFound(dir.to_owned()));
        }
        Self::open_existing(dir, ReloadPolicy::OnCommitWithDelay)
    }

    fn open_existing(dir: &Path, reload: ReloadPolicy) -> Result<Self> {
        let index = Index::open_in_dir(dir)?;
        let found = read_state(&index)?.schema_version;
        if found != SCHEMA_VERSION {
            return Err(Error::SchemaVersion {
                path: dir.to_owned(),
                found,
                expected: SCHEMA_VERSION,
            });
        }
        schema::register_tokenizers(index.tokenizers());
        let fields = Fields::from_schema(&index.schema())?;
        let reader = index.reader_builder().reload_policy(reload).try_into()?;
        Ok(Self {
            index,
            fields,
            reader,
            dir: dir.to_owned(),
        })
    }

    /// Deletes the index in `dir`, so the next [`open`](Self::open) starts
    /// from scratch. Nothing is lost: the index is rebuilt from the store.
    pub fn delete(dir: &Path) -> Result<()> {
        match fs::remove_dir_all(dir) {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(source) => Err(io_error(dir, source)),
        }
    }

    /// Empties the index in place, so the next [`update`](Self::update)
    /// indexes every message again. Unlike [`delete`](Self::delete) it can
    /// run while apps have the index open: they see it empty, then filling.
    pub fn clear(&self) -> Result<()> {
        let mut writer: IndexWriter = self.index.writer_with_num_threads(1, 15_000_000)?;
        writer.delete_all_documents()?;
        commit(
            &mut writer,
            &IndexState {
                schema_version: SCHEMA_VERSION,
                ..IndexState::default()
            },
        )?;
        drop(writer);
        self.reload()
    }

    /// The directory of the index.
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Documents in the index as of the last [`reload`](Self::reload).
    pub fn num_docs(&self) -> u64 {
        self.reader.searcher().num_docs()
    }

    /// The indexing progress of the last commit.
    pub fn state(&self) -> Result<IndexState> {
        read_state(&self.index)
    }

    /// Makes the latest commit visible to searches.
    pub fn reload(&self) -> Result<()> {
        Ok(self.reader.reload()?)
    }

    /// Brings the index up to date with `store`: the first time every
    /// message, afterwards what the change journal lists. Commits every
    /// [`IndexOptions::commit_every`] messages, so a crash loses little and
    /// searches see the indexed part early. `progress` gets the number of
    /// messages indexed so far.
    pub fn update(
        &self,
        store: &Store,
        options: &IndexOptions,
        mut progress: impl FnMut(u64),
    ) -> Result<UpdateStats> {
        let mut writer: IndexWriter = self
            .index
            .writer_with_num_threads(options.writer_threads.max(1), options.memory_budget)?;
        let mut state = self.state()?;
        let mut stats = UpdateStats::default();

        if state.change_seq.is_none() {
            let scan_seq = match state.scan_seq {
                Some(seq) => seq,
                None => store.latest_change(DbKind::Mail)?,
            };
            let mut after = MessageId(state.scan_after.unwrap_or(0));
            loop {
                let mut read = 0;
                self.add_documents(&writer, store, options, &mut stats, |store| {
                    if read >= options.commit_every || options.stopped() {
                        return Ok(Vec::new());
                    }
                    let messages = store.messages_after(after, READ_BATCH)?;
                    if let Some(last) = messages.last() {
                        after = last.id;
                    }
                    read += messages.len() as u64;
                    Ok(messages)
                })?;
                let stopped = options.stopped();
                let finished = read < options.commit_every && !stopped;
                state = IndexState {
                    schema_version: SCHEMA_VERSION,
                    change_seq: finished.then_some(scan_seq),
                    scan_after: (!finished).then_some(after.0),
                    scan_seq: (!finished).then_some(scan_seq),
                };
                commit(&mut writer, &state)?;
                progress(stats.indexed);
                if stopped {
                    self.reload()?;
                    return Ok(stats);
                }
                if finished {
                    break;
                }
            }
        }

        let mut seq = state.change_seq.unwrap_or_default();
        while !options.stopped() {
            let changes = store.changes_since(DbKind::Mail, seq, JOURNAL_BATCH)?;
            let Some(last) = changes.last() else {
                break;
            };
            seq = last.seq;
            let mut seen = HashSet::new();
            let ids: Vec<MessageId> = changes
                .iter()
                .filter(|change| change.kind == ObjectKind::Message)
                .map(|change| MessageId(change.object_id))
                .filter(|id| seen.insert(*id))
                .collect();
            for id in &ids {
                writer.delete_term(Term::from_field_u64(self.fields.msg_id, id.0 as u64));
            }
            let before = stats.indexed;
            let mut pending = ids.chunks(READ_BATCH as usize);
            self.add_documents(&writer, store, options, &mut stats, |store| {
                Ok(match pending.next() {
                    Some(chunk) => store.messages_by_id(chunk)?,
                    None => Vec::new(),
                })
            })?;
            stats.removed += ids.len() as u64 - (stats.indexed - before);
            state = IndexState {
                schema_version: SCHEMA_VERSION,
                change_seq: Some(seq),
                scan_after: None,
                scan_seq: None,
            };
            commit(&mut writer, &state)?;
            progress(stats.indexed);
        }

        writer.wait_merging_threads()?;
        self.reload()?;
        Ok(stats)
    }

    /// Adds the messages that `next` returns until it returns none. The
    /// calling thread reads the store (its connection is not shared); parse
    /// threads turn raw messages into documents. Returns how many were added.
    fn add_documents(
        &self,
        writer: &IndexWriter,
        store: &Store,
        options: &IndexOptions,
        stats: &mut UpdateStats,
        mut next: impl FnMut(&Store) -> Result<Vec<StoredMessage>>,
    ) -> Result<u64> {
        type Batch = Vec<(StoredMessage, Option<Vec<u8>>)>;
        let threads = options.parse_threads.max(1);
        let (sender, receiver) = mpsc::sync_channel::<Batch>(threads * 2);
        let receiver = Mutex::new(receiver);
        let failure: Mutex<Option<Error>> = Mutex::new(None);
        let added = AtomicU64::new(0);
        let fields = &self.fields;

        let read = thread::scope(|scope| {
            for _ in 0..threads {
                scope.spawn(|| {
                    loop {
                        let batch = match receiver.lock() {
                            Ok(receiver) => receiver.recv(),
                            Err(_) => break,
                        };
                        let Ok(batch) = batch else {
                            break;
                        };
                        for (message, raw) in batch {
                            let text = raw.as_deref().map(document::message_text);
                            let doc = document::build(fields, &message, text.as_ref());
                            match writer.add_document(doc) {
                                Ok(_) => {
                                    added.fetch_add(1, Ordering::Relaxed);
                                }
                                Err(err) => {
                                    if let Ok(mut failure) = failure.lock() {
                                        failure.get_or_insert(err.into());
                                    }
                                }
                            }
                        }
                    }
                });
            }

            let mut without_text = 0;
            let result = (|| -> Result<()> {
                loop {
                    let messages = next(store)?;
                    if messages.is_empty() {
                        return Ok(());
                    }
                    let mut batch: Batch = Vec::with_capacity(messages.len());
                    for message in messages {
                        let raw = match &message.blob_hash {
                            Some(hash) => store.blobs().get(hash)?,
                            None => None,
                        };
                        if raw.is_none() {
                            without_text += 1;
                        }
                        batch.push((message, raw));
                    }
                    if sender.send(batch).is_err() || failure.lock().map_or(true, |f| f.is_some()) {
                        return Ok(());
                    }
                }
            })();
            drop(sender);
            result.map(|()| without_text)
        });
        stats.without_text += read?;
        if let Some(err) = failure.into_inner().ok().flatten() {
            return Err(err);
        }
        let added = added.into_inner();
        stats.indexed += added;
        Ok(added)
    }

    /// Searches the index as of the last [`reload`](Self::reload). Names
    /// also match with typos; if nothing matches, every word may (see
    /// [`SearchResults::fuzzy`]).
    pub fn search(&self, query: &Query, options: &SearchOptions) -> Result<SearchResults> {
        let results = self.search_with(query, options, Fuzziness::Names)?;
        if results.hits.is_empty() && options.offset == 0 && query.has_free_text() {
            // Nothing matched: try words a typo or two away everywhere.
            return self.search_with(query, options, Fuzziness::Everywhere);
        }
        Ok(results)
    }

    /// "Did you mean": `input` (search-box text) with each misspelled word
    /// replaced by the nearest word in the mail, or `None` if every word is
    /// in the mail as typed. With `as_you_type`, an unfinished last word
    /// counts as there if a word starts with it. Cheap when nothing is
    /// misspelled.
    pub fn suggest(&self, input: &str, as_you_type: bool) -> Result<Option<String>> {
        crate::suggest::suggest(&self.reader.searcher(), &self.fields, input, as_you_type)
    }

    fn search_with(
        &self,
        query: &Query,
        options: &SearchOptions,
        fuzziness: Fuzziness,
    ) -> Result<SearchResults> {
        let searcher = self.reader.searcher();
        let compiled = compile_with(&self.fields, query, fuzziness, Some(&searcher));
        let sort = match options.sort {
            Sort::Auto if query.has_free_text() => Sort::Relevance,
            Sort::Auto => Sort::Newest,
            sort => sort,
        };
        let top = TopDocs::with_limit(options.limit.max(1)).and_offset(options.offset);

        let (scored, total): (Vec<(f32, DocAddress)>, Option<usize>) = match sort {
            Sort::Relevance | Sort::Auto => {
                let anchor = newest_date(&searcher)?;
                let collector = top.tweak_score(move |segment: &tantivy::SegmentReader| {
                    let fast = segment.fast_fields();
                    let dates = fast.i64("date").ok();
                    let flags = fast.u64("flags").ok();
                    move |doc, score: f32| {
                        let date = dates.as_ref().and_then(|c| c.first(doc));
                        let flags = flags.as_ref().and_then(|c| c.first(doc)).unwrap_or(0);
                        score * boost(anchor, date, flags)
                    }
                });
                if options.count {
                    let (hits, count) = searcher.search(&compiled, &(collector, Count))?;
                    (hits, Some(count))
                } else {
                    (searcher.search(&compiled, &collector)?, None)
                }
            }
            Sort::Newest | Sort::Oldest => {
                let order = if sort == Sort::Newest {
                    Order::Desc
                } else {
                    Order::Asc
                };
                let collector = top.order_by_fast_field::<i64>("date", order);
                let (hits, count) = if options.count {
                    let (hits, count) = searcher.search(&compiled, &(collector, Count))?;
                    (hits, Some(count))
                } else {
                    (searcher.search(&compiled, &collector)?, None)
                };
                (
                    hits.into_iter()
                        .map(|(_, address)| (0.0, address))
                        .collect(),
                    count,
                )
            }
        };

        let mut hits = Vec::with_capacity(scored.len());
        for (score, address) in scored {
            let segment = searcher.segment_reader(address.segment_ord);
            let fast = segment.fast_fields();
            let Some(id) = fast.u64("msg_id")?.first(address.doc_id) else {
                continue;
            };
            hits.push(Hit {
                message: MessageId(id as i64),
                score,
                date: fast.i64("date")?.first(address.doc_id),
            });
        }
        Ok(SearchResults {
            hits,
            total,
            fuzzy: fuzziness == Fuzziness::Everywhere,
        })
    }

    /// Highlighted body snippets for `messages`, which should be results of
    /// `query`: the raw messages are read from `store`, so the index stores
    /// no text. A message without a stored body gets an empty snippet.
    pub fn snippets(
        &self,
        store: &Store,
        query: &Query,
        messages: &[MessageId],
        max_chars: usize,
    ) -> Result<Vec<Snippet>> {
        let highlighter = self.highlighter(query, max_chars)?;
        let stored = store.messages_by_id(messages)?;
        let mut out = Vec::with_capacity(messages.len());
        for id in messages {
            let raw = match stored
                .iter()
                .find(|m| m.id == *id)
                .and_then(|m| m.blob_hash)
            {
                Some(hash) => store.blobs().get(&hash)?,
                None => None,
            };
            let Some(raw) = raw else {
                out.push(Snippet::default());
                continue;
            };
            // The text part nearly always comes before attachments, and
            // parsing (decoding) big attachments would dominate the time.
            let raw = &raw[..raw.len().min(SNIPPET_SCAN_BYTES)];
            let MessageText { body, .. } = document::message_text(raw);
            match highlighter.snippet(&body) {
                Some(snippet) => out.push(Snippet {
                    text: snippet.fragment().to_owned(),
                    highlights: snippet.highlighted().to_vec(),
                }),
                // No match in the body: show its start.
                None => out.push(Snippet {
                    text: start_of(&body, max_chars),
                    highlights: Vec::new(),
                }),
            }
        }
        Ok(out)
    }

    /// A highlighter for the body words of `query`, leaving out excluded
    /// words (`-word`), which tantivy's own snippets would highlight.
    fn highlighter(&self, query: &Query, max_chars: usize) -> Result<Highlighter> {
        let searcher = self.reader.searcher();
        let mut words = Vec::new();
        body_words(query, &mut words);
        let mut stems = BTreeMap::new();
        for stem in words.iter().flat_map(|word| schema::stems(word)) {
            let term = Term::from_field_text(self.fields.body_stem, &stem);
            let doc_freq = searcher.doc_freq(&term)?;
            if doc_freq > 0 {
                stems.insert(stem, 1.0 / (1.0 + doc_freq as f32));
            }
        }
        Ok(Highlighter::new(stems, self.fields.body, max_chars))
    }

    /// The fields of the index schema.
    pub fn fields(&self) -> &Fields {
        &self.fields
    }
}

fn body_words(query: &Query, out: &mut Vec<String>) {
    match query {
        Query::Text {
            field: TextField::Any,
            text,
        } => out.extend(schema::tokens(text)),
        Query::And(items) | Query::Or(items) => {
            for item in items {
                body_words(item, out);
            }
        }
        // Words typed so far; the unfinished last one cannot be highlighted.
        Query::Prefix {
            field: TextField::Any,
            text,
        } => {
            let mut words = schema::tokens(text);
            words.pop();
            out.extend(words);
        }
        Query::All
        | Query::Not(_)
        | Query::Text { .. }
        | Query::Prefix { .. }
        | Query::Filter(_) => {}
    }
}

fn start_of(text: &str, max_chars: usize) -> String {
    let collapsed: Vec<&str> = text.split_whitespace().collect();
    let mut out = String::new();
    for word in collapsed {
        if out.chars().count() + word.chars().count() + 1 > max_chars {
            break;
        }
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(word);
    }
    out
}

/// Ranking factor for a message of `date` with `flags`, when the newest
/// message in the index is from `anchor`.
fn boost(anchor: i64, date: Option<i64>, flags: u64) -> f32 {
    let recency = match date {
        Some(date) => {
            let age_days = (anchor.saturating_sub(date)).max(0) as f32 / 86_400.0;
            RECENCY_WEIGHT * (-age_days / RECENCY_HALF_LIFE_DAYS).exp2()
        }
        None => 0.0,
    };
    let flagged = if flags & u64::from(MessageFlags::FLAGGED.bits()) != 0 {
        FLAGGED_BOOST
    } else {
        1.0
    };
    (1.0 + recency) * flagged
}

/// The newest message date in the index, but not in the future; recency is
/// measured from it, so an old archive (like Enron) still ranks by recency.
fn newest_date(searcher: &Searcher) -> Result<i64> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(i64::MAX, |d| d.as_secs() as i64);
    let mut newest = i64::MIN;
    for segment in searcher.segment_readers() {
        let dates = segment.fast_fields().i64("date")?;
        if dates.num_docs() > 0 {
            newest = newest.max(dates.max_value());
        }
    }
    Ok(if newest == i64::MIN {
        now
    } else {
        newest.min(now)
    })
}

fn commit(writer: &mut IndexWriter, state: &IndexState) -> Result<()> {
    let payload = serde_json::to_string(state).map_err(|err| Error::State(err.to_string()))?;
    let mut prepared = writer.prepare_commit()?;
    prepared.set_payload(&payload);
    prepared.commit()?;
    Ok(())
}

fn read_state(index: &Index) -> Result<IndexState> {
    let metas = index.load_metas()?;
    match metas.payload {
        Some(payload) => {
            serde_json::from_str(&payload).map_err(|err| Error::State(err.to_string()))
        }
        None => Ok(IndexState::default()),
    }
}

fn io_error(path: &Path, source: io::Error) -> Error {
    Error::Io {
        path: path.to_owned(),
        source,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rebuilds_an_index_of_another_schema_version() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("index");
        let index = SearchIndex::open(&dir).unwrap();
        let mut writer: IndexWriter = index.index.writer_with_num_threads(1, 15_000_000).unwrap();
        writer
            .add_document(tantivy::doc!(index.fields.msg_id => 7u64))
            .unwrap();
        let old = IndexState {
            schema_version: SCHEMA_VERSION - 1,
            ..IndexState::default()
        };
        commit(&mut writer, &old).unwrap();
        drop((writer, index));

        assert!(matches!(
            SearchIndex::open_read_only(&dir),
            Err(Error::SchemaVersion { found, .. }) if found == SCHEMA_VERSION - 1
        ));
        let index = SearchIndex::open(&dir).unwrap();
        assert_eq!(index.num_docs(), 0);
        assert_eq!(index.state().unwrap().schema_version, SCHEMA_VERSION);
    }

    #[test]
    fn clear_empties_the_index_and_starts_over() {
        let tmp = tempfile::tempdir().unwrap();
        let index = SearchIndex::open(&tmp.path().join("index")).unwrap();
        let mut writer: IndexWriter = index.index.writer_with_num_threads(1, 15_000_000).unwrap();
        writer
            .add_document(tantivy::doc!(index.fields.msg_id => 7u64))
            .unwrap();
        let done = IndexState {
            schema_version: SCHEMA_VERSION,
            change_seq: Some(42),
            ..IndexState::default()
        };
        commit(&mut writer, &done).unwrap();
        drop(writer);
        index.reload().unwrap();
        assert_eq!(index.num_docs(), 1);

        index.clear().unwrap();
        assert_eq!(index.num_docs(), 0);
        assert_eq!(index.state().unwrap().change_seq, None);
        assert_eq!(index.state().unwrap().schema_version, SCHEMA_VERSION);
    }
}
