// SPDX-License-Identifier: GPL-3.0-or-later

//! Content-addressed blob store (`docs/ARCHITECTURE.md` §5.2).
//!
//! Blobs are keyed by their blake3 hash, so the same message in several
//! folders is stored once. Blobs up to [`FILE_THRESHOLD`] are stored
//! zstd-compressed inside `blobs.db`; larger ones are stored as files in
//! `attachments/`, because SQLite is faster than the filesystem for small
//! blobs and slower for large ones.
//!
//! Write order: store the blob first, then the metadata row that references
//! it. Unreferenced blobs are removed later by a background job.

use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use rusqlite::{Connection, OptionalExtension, params};

use crate::db::Mode;
use crate::error::{Error, Result};

/// Blobs larger than this are stored as files instead of in `blobs.db`.
pub const FILE_THRESHOLD: usize = 256 * 1024;

/// zstd level: fast to compress, most of the gain for mail text.
const ZSTD_LEVEL: i32 = 3;

/// `blob.storage` values.
const STORAGE_INLINE: i64 = 0;
const STORAGE_FILE: i64 = 1;

/// The blake3 hash that identifies a blob.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BlobHash([u8; 32]);

impl BlobHash {
    /// Hashes `data`.
    pub fn of(data: &[u8]) -> Self {
        Self(*blake3::hash(data).as_bytes())
    }

    /// Wraps 32 raw hash bytes.
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// The raw hash bytes, as stored in `blob_hash` columns.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Lowercase hex, as used for file names.
    pub fn to_hex(&self) -> String {
        blake3::Hash::from_bytes(self.0).to_hex().to_string()
    }
}

impl fmt::Display for BlobHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl fmt::Debug for BlobHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BlobHash({self})")
    }
}

/// Returned when parsing a hash that is not 64 hex digits.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("invalid blob hash {0:?}")]
pub struct InvalidBlobHash(pub String);

impl FromStr for BlobHash {
    type Err = InvalidBlobHash;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        blake3::Hash::from_hex(s)
            .map(|hash| Self(*hash.as_bytes()))
            .map_err(|_| InvalidBlobHash(s.to_owned()))
    }
}

/// `blobs.db` plus the `attachments/` directory.
#[derive(Debug)]
pub struct BlobStore {
    conn: Connection,
    dir: PathBuf,
    mode: Mode,
}

impl BlobStore {
    pub(crate) fn new(conn: Connection, dir: PathBuf, mode: Mode) -> Self {
        Self { conn, dir, mode }
    }

    /// Starts a transaction that the following [`put`](Self::put)s join, so
    /// a [`MailBatch`](crate::MailBatch) commits its blobs at once.
    pub(crate) fn begin(&self) -> Result<()> {
        self.check_writable()?;
        self.conn.execute_batch("BEGIN IMMEDIATE")?;
        Ok(())
    }

    /// Ends the transaction started by [`begin`](Self::begin).
    pub(crate) fn end(&self, commit: bool) -> Result<()> {
        self.conn
            .execute_batch(if commit { "COMMIT" } else { "ROLLBACK" })?;
        Ok(())
    }

    /// Stores `data` and returns its hash. Storing the same bytes again is a
    /// cheap no-op.
    pub fn put(&self, data: &[u8]) -> Result<BlobHash> {
        self.check_writable()?;
        let hash = BlobHash::of(data);
        if self.contains(&hash)? {
            return Ok(hash);
        }
        let size = i64::try_from(data.len())
            .map_err(|_| Error::InvalidData(format!("blob of {} bytes", data.len())))?;
        if data.len() > FILE_THRESHOLD {
            self.write_file(&hash, data)?;
            self.conn.execute(
                "INSERT OR IGNORE INTO blob (hash, size, storage, data) VALUES (?1, ?2, ?3, NULL)",
                params![hash.as_bytes(), size, STORAGE_FILE],
            )?;
        } else {
            let compressed = zstd::bulk::compress(data, ZSTD_LEVEL)
                .map_err(|err| Error::io(self.db_path(), err))?;
            self.conn.execute(
                "INSERT OR IGNORE INTO blob (hash, size, storage, data) VALUES (?1, ?2, ?3, ?4)",
                params![hash.as_bytes(), size, STORAGE_INLINE, compressed],
            )?;
        }
        Ok(hash)
    }

    /// Returns the bytes of a blob, or `None` if it is not stored.
    ///
    /// The content is checked against the hash, so corruption on disk is
    /// reported as [`Error::CorruptBlob`] instead of returning wrong data.
    pub fn get(&self, hash: &BlobHash) -> Result<Option<Vec<u8>>> {
        let row = self
            .conn
            .query_row(
                "SELECT size, storage, data FROM blob WHERE hash = ?1",
                [hash.as_bytes()],
                |row| {
                    Ok((
                        row.get::<_, i64>(0)?,
                        row.get::<_, i64>(1)?,
                        row.get::<_, Option<Vec<u8>>>(2)?,
                    ))
                },
            )
            .optional()?;
        let Some((size, storage, data)) = row else {
            return Ok(None);
        };
        let size = usize::try_from(size).map_err(|_| Error::CorruptBlob(*hash))?;
        let bytes = match (storage, data) {
            (STORAGE_INLINE, Some(compressed)) => {
                zstd::bulk::decompress(&compressed, size).map_err(|_| Error::CorruptBlob(*hash))?
            }
            (STORAGE_FILE, None) => {
                let path = self.file_path(hash);
                fs::read(&path).map_err(|err| Error::io(path, err))?
            }
            _ => return Err(Error::CorruptBlob(*hash)),
        };
        if bytes.len() != size || BlobHash::of(&bytes) != *hash {
            return Err(Error::CorruptBlob(*hash));
        }
        Ok(Some(bytes))
    }

    /// Whether a blob is stored.
    pub fn contains(&self, hash: &BlobHash) -> Result<bool> {
        Ok(self
            .conn
            .query_row(
                "SELECT 1 FROM blob WHERE hash = ?1",
                [hash.as_bytes()],
                |_| Ok(()),
            )
            .optional()?
            .is_some())
    }

    /// Deletes a blob. Returns whether it existed.
    pub fn remove(&self, hash: &BlobHash) -> Result<bool> {
        self.check_writable()?;
        let storage: Option<i64> = self
            .conn
            .query_row(
                "DELETE FROM blob WHERE hash = ?1 RETURNING storage",
                [hash.as_bytes()],
                |row| row.get(0),
            )
            .optional()?;
        if storage == Some(STORAGE_FILE) {
            // The row is gone first: a crash here leaves an unreferenced
            // file, never a row pointing at a missing file.
            let path = self.file_path(hash);
            match fs::remove_file(&path) {
                Ok(()) => {}
                Err(err) if err.kind() == io::ErrorKind::NotFound => {}
                Err(err) => return Err(Error::io(path, err)),
            }
        }
        Ok(storage.is_some())
    }

    /// Returns up to `max_pages` free pages to the filesystem
    /// (`PRAGMA incremental_vacuum`). `None` frees all of them.
    pub fn incremental_vacuum(&self, max_pages: Option<u32>) -> Result<()> {
        self.check_writable()?;
        let sql = match max_pages {
            Some(pages) => format!("PRAGMA incremental_vacuum({pages})"),
            None => "PRAGMA incremental_vacuum".to_owned(),
        };
        // The pragma returns one row per freed page.
        let mut stmt = self.conn.prepare(&sql)?;
        let mut rows = stmt.query([])?;
        while rows.next()?.is_some() {}
        Ok(())
    }

    /// `attachments/<first two hex digits>/<hex>`
    fn file_path(&self, hash: &BlobHash) -> PathBuf {
        let hex = hash.to_hex();
        self.dir.join(&hex[..2]).join(hex)
    }

    fn db_path(&self) -> PathBuf {
        self.conn.path().map(PathBuf::from).unwrap_or_default()
    }

    /// Writes a large blob to a temporary file and renames it into place, so
    /// a crash never leaves a partial file under the final name.
    fn write_file(&self, hash: &BlobHash, data: &[u8]) -> Result<()> {
        let path = self.file_path(hash);
        let dir = path.parent().unwrap_or(&self.dir);
        fs::create_dir_all(dir).map_err(|err| Error::io(dir, err))?;
        let tmp = dir.join(format!(".{}.{}.tmp", hash.to_hex(), std::process::id()));
        let result = write_synced(&tmp, data).and_then(|()| fs::rename(&tmp, &path));
        if let Err(err) = result {
            let _ = fs::remove_file(&tmp);
            return Err(Error::io(path, err));
        }
        Ok(())
    }

    fn check_writable(&self) -> Result<()> {
        match self.mode {
            Mode::ReadWrite => Ok(()),
            Mode::ReadOnly => Err(Error::ReadOnly),
        }
    }
}

fn write_synced(path: &Path, data: &[u8]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(data)?;
    file.sync_all()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{self, DbKind};

    fn store(root: &Path, mode: Mode) -> BlobStore {
        let conn = db::open(&root.join("blobs.db"), DbKind::Blobs, mode).unwrap();
        BlobStore::new(conn, root.join("attachments"), mode)
    }

    #[test]
    fn hash_hex_round_trip() {
        let hash = BlobHash::of(b"hello");
        assert_eq!(hash.to_hex().len(), 64);
        assert_eq!(hash.to_hex().parse::<BlobHash>().unwrap(), hash);
        assert!("xyz".parse::<BlobHash>().is_err());
        // Known blake3 test vector for the empty input.
        assert_eq!(
            BlobHash::of(b"").to_string(),
            "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
        );
    }

    #[test]
    fn small_blob_round_trip_is_compressed_inline() {
        let tmp = tempfile::tempdir().unwrap();
        let blobs = store(tmp.path(), Mode::ReadWrite);
        let message = b"Subject: hi\r\n\r\nhello hello hello hello hello hello\r\n".repeat(50);
        let hash = blobs.put(&message).unwrap();
        assert_eq!(hash, BlobHash::of(&message));
        assert!(blobs.contains(&hash).unwrap());
        assert_eq!(blobs.get(&hash).unwrap().unwrap(), message);

        let stored: Vec<u8> = blobs
            .conn
            .query_row("SELECT data FROM blob", [], |row| row.get(0))
            .unwrap();
        assert!(stored.len() < message.len() / 4, "not compressed");
        assert!(!blobs.dir.exists(), "small blobs must not create files");
    }

    #[test]
    fn identical_content_is_stored_once() {
        let tmp = tempfile::tempdir().unwrap();
        let blobs = store(tmp.path(), Mode::ReadWrite);
        let a = blobs.put(b"same").unwrap();
        let b = blobs.put(b"same").unwrap();
        assert_eq!(a, b);
        let count: i64 = blobs
            .conn
            .query_row("SELECT count(*) FROM blob", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn empty_blob() {
        let tmp = tempfile::tempdir().unwrap();
        let blobs = store(tmp.path(), Mode::ReadWrite);
        let hash = blobs.put(b"").unwrap();
        assert_eq!(blobs.get(&hash).unwrap().unwrap(), b"");
    }

    #[test]
    fn large_blob_is_a_file() {
        let tmp = tempfile::tempdir().unwrap();
        let blobs = store(tmp.path(), Mode::ReadWrite);
        let data: Vec<u8> = (0..FILE_THRESHOLD + 1)
            .map(|i| (i * 7 % 251) as u8)
            .collect();
        let hash = blobs.put(&data).unwrap();
        let path = blobs.file_path(&hash);
        assert!(path.is_file());
        assert_eq!(fs::read(&path).unwrap(), data);
        assert_eq!(blobs.get(&hash).unwrap().unwrap(), data);

        // Only the final file remains in the shard directory.
        let entries = fs::read_dir(path.parent().unwrap()).unwrap().count();
        assert_eq!(entries, 1);

        assert!(blobs.remove(&hash).unwrap());
        assert!(!path.exists());
        assert!(blobs.get(&hash).unwrap().is_none());
    }

    #[test]
    fn missing_and_removed_blobs() {
        let tmp = tempfile::tempdir().unwrap();
        let blobs = store(tmp.path(), Mode::ReadWrite);
        let hash = BlobHash::of(b"never stored");
        assert!(blobs.get(&hash).unwrap().is_none());
        assert!(!blobs.remove(&hash).unwrap());
        let hash = blobs.put(b"short-lived").unwrap();
        assert!(blobs.remove(&hash).unwrap());
        assert!(!blobs.contains(&hash).unwrap());
    }

    #[test]
    fn detects_corruption() {
        let tmp = tempfile::tempdir().unwrap();
        let blobs = store(tmp.path(), Mode::ReadWrite);
        let data = vec![b'x'; FILE_THRESHOLD + 10];
        let hash = blobs.put(&data).unwrap();
        let mut bad = data.clone();
        bad[0] = b'y';
        fs::write(blobs.file_path(&hash), &bad).unwrap();
        assert!(matches!(blobs.get(&hash), Err(Error::CorruptBlob(h)) if h == hash));

        let hash = blobs.put(b"inline").unwrap();
        blobs
            .conn
            .execute(
                "UPDATE blob SET data = ?1 WHERE hash = ?2",
                params![
                    zstd::bulk::compress(b"tampered", 3).unwrap(),
                    hash.as_bytes()
                ],
            )
            .unwrap();
        assert!(matches!(blobs.get(&hash), Err(Error::CorruptBlob(_))));
    }

    #[test]
    fn read_only_store_reads_but_does_not_write() {
        let tmp = tempfile::tempdir().unwrap();
        let hash = store(tmp.path(), Mode::ReadWrite).put(b"message").unwrap();
        let blobs = store(tmp.path(), Mode::ReadOnly);
        assert_eq!(blobs.get(&hash).unwrap().unwrap(), b"message");
        assert!(matches!(blobs.put(b"new"), Err(Error::ReadOnly)));
        assert!(matches!(blobs.remove(&hash), Err(Error::ReadOnly)));
    }

    #[test]
    fn incremental_vacuum_shrinks_the_file() {
        let tmp = tempfile::tempdir().unwrap();
        let blobs = store(tmp.path(), Mode::ReadWrite);
        // Incompressible data, below the file threshold.
        let hashes: Vec<BlobHash> = (0u32..200)
            .map(|i| {
                let data: Vec<u8> = (0..16 * 1024u32)
                    .map(|j| {
                        blake3::hash(&[i.to_le_bytes(), j.to_le_bytes()].concat()).as_bytes()[0]
                    })
                    .collect();
                blobs.put(&data).unwrap()
            })
            .collect();
        for hash in &hashes {
            blobs.remove(hash).unwrap();
        }
        let free_pages = |b: &BlobStore| -> i64 {
            b.conn
                .pragma_query_value(None, "freelist_count", |row| row.get(0))
                .unwrap()
        };
        assert!(free_pages(&blobs) > 0);
        blobs.incremental_vacuum(None).unwrap();
        assert_eq!(free_pages(&blobs), 0);
    }
}
