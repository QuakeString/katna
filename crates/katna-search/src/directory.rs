// SPDX-License-Identifier: GPL-3.0-or-later

//! The index directory: tantivy's memory-mapped one, with commits that wait
//! out a reader on Windows.
//!
//! A commit replaces `meta.json` (and `.managed.json`) by renaming a new
//! file over it. On Windows that rename fails with "Access is denied" while
//! another handle has the old file open, and every app that searches polls
//! `meta.json` to see new commits. The window is a few microseconds per
//! poll, so trying the rename again a moment later lets the commit through
//! instead of failing the whole update.

use std::io;
use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use tantivy::directory::error::{DeleteError, LockError, OpenReadError, OpenWriteError};
use tantivy::directory::{
    Directory, DirectoryLock, FileHandle, Lock, MmapDirectory, WatchCallback, WatchHandle, WritePtr,
};

/// Tries after the first one, and the wait before each: about a second in
/// all, twice an app's polling interval.
const RETRIES: u32 = 20;
const RETRY_WAIT: Duration = Duration::from_millis(50);

/// Opens the memory-mapped directory at `dir`, wrapped as above.
pub(crate) fn open(dir: &Path) -> tantivy::Result<Retrying<MmapDirectory>> {
    Ok(Retrying(MmapDirectory::open(dir)?))
}

/// A directory whose atomic writes are tried again while the file they
/// replace is held open elsewhere.
#[derive(Clone, Debug)]
pub(crate) struct Retrying<D>(D);

impl<D: Directory + Clone> Directory for Retrying<D> {
    fn get_file_handle(&self, path: &Path) -> Result<Arc<dyn FileHandle>, OpenReadError> {
        self.0.get_file_handle(path)
    }

    fn delete(&self, path: &Path) -> Result<(), DeleteError> {
        self.0.delete(path)
    }

    fn exists(&self, path: &Path) -> Result<bool, OpenReadError> {
        self.0.exists(path)
    }

    fn open_write(&self, path: &Path) -> Result<WritePtr, OpenWriteError> {
        self.0.open_write(path)
    }

    fn atomic_read(&self, path: &Path) -> Result<Vec<u8>, OpenReadError> {
        self.0.atomic_read(path)
    }

    fn atomic_write(&self, path: &Path, data: &[u8]) -> io::Result<()> {
        let mut tries = 0;
        loop {
            match self.0.atomic_write(path, data) {
                Err(err) if err.kind() == io::ErrorKind::PermissionDenied && tries < RETRIES => {
                    tries += 1;
                    thread::sleep(RETRY_WAIT);
                }
                result => return result,
            }
        }
    }

    fn sync_directory(&self) -> io::Result<()> {
        self.0.sync_directory()
    }

    fn acquire_lock(&self, lock: &Lock) -> Result<DirectoryLock, LockError> {
        self.0.acquire_lock(lock)
    }

    fn watch(&self, callback: WatchCallback) -> tantivy::Result<WatchHandle> {
        self.0.watch(callback)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU32, Ordering};

    use tantivy::directory::RamDirectory;

    use super::*;

    /// Refuses the first `refusals` atomic writes as Windows does while a
    /// reader has the file open.
    #[derive(Clone, Debug)]
    struct Busy {
        inner: RamDirectory,
        refusals: Arc<AtomicU32>,
    }

    impl Directory for Busy {
        fn get_file_handle(&self, path: &Path) -> Result<Arc<dyn FileHandle>, OpenReadError> {
            self.inner.get_file_handle(path)
        }
        fn delete(&self, path: &Path) -> Result<(), DeleteError> {
            self.inner.delete(path)
        }
        fn exists(&self, path: &Path) -> Result<bool, OpenReadError> {
            self.inner.exists(path)
        }
        fn open_write(&self, path: &Path) -> Result<WritePtr, OpenWriteError> {
            self.inner.open_write(path)
        }
        fn atomic_read(&self, path: &Path) -> Result<Vec<u8>, OpenReadError> {
            self.inner.atomic_read(path)
        }
        fn atomic_write(&self, path: &Path, data: &[u8]) -> io::Result<()> {
            let left = self.refusals.load(Ordering::Relaxed);
            if left > 0 {
                self.refusals.store(left - 1, Ordering::Relaxed);
                return Err(io::Error::from(io::ErrorKind::PermissionDenied));
            }
            self.inner.atomic_write(path, data)
        }
        fn sync_directory(&self) -> io::Result<()> {
            self.inner.sync_directory()
        }
        fn watch(&self, callback: WatchCallback) -> tantivy::Result<WatchHandle> {
            self.inner.watch(callback)
        }
    }

    fn busy(refusals: u32) -> Retrying<Busy> {
        Retrying(Busy {
            inner: RamDirectory::create(),
            refusals: Arc::new(AtomicU32::new(refusals)),
        })
    }

    #[test]
    fn a_held_file_is_replaced_once_it_is_let_go() {
        let dir = busy(3);
        dir.atomic_write(Path::new("meta.json"), b"new").unwrap();
        assert_eq!(dir.atomic_read(Path::new("meta.json")).unwrap(), b"new");
    }

    #[test]
    fn a_file_held_for_good_still_fails() {
        let dir = busy(RETRIES + 1);
        let err = dir
            .atomic_write(Path::new("meta.json"), b"new")
            .unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::PermissionDenied);
    }
}
