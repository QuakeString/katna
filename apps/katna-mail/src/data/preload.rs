// SPDX-License-Identifier: GPL-3.0-or-later

//! The first list, read while the window starts. Opening the window
//! (GPUI, the GPU, fonts) takes longer than reading the list it shows, so
//! `main` starts [`Preloading`] first thing, on a thread of its own, and the
//! window takes what it read instead of reading it again. The list read is
//! the one the window showed when it last started ([`ListRead`], kept in
//! the cache directory). When the window asks for another list, the mail
//! changed meanwhile, or the thread fails or is slow, the window reads as
//! it would without it.

use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;

use katna_core::{MailCategory, Paths};
use katna_store::{
    DbKind, FlagFilter, FolderId, FolderSummary, InboxThreads, MessageFlags, Mode, Store,
};

/// How long the window waits for [`Preloading`] before it reads on its own.
const WAIT: Duration = Duration::from_secs(1);

/// A list of conversations read from the store, by what was asked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListRead {
    /// An inbox's conversations, with its unread ones per tab.
    Inbox {
        folder: FolderId,
        categories: Option<Vec<MailCategory>>,
    },
    /// Another folder's conversations.
    Folder {
        folder: FolderId,
        categories: Option<Vec<MailCategory>>,
    },
    /// Conversations across folders, as the unified inbox lists them.
    Spread {
        folders: Vec<FolderId>,
        filter: FlagFilter,
    },
}

impl ListRead {
    /// Reads the list. Only [`ListRead::Inbox`] has unread counts per tab.
    pub fn read(&self, store: &Store) -> katna_store::Result<InboxThreads> {
        match self {
            Self::Inbox { folder, categories } => {
                store.inbox_threads(*folder, categories.as_deref())
            }
            Self::Folder { folder, categories } => Ok((
                store.folder_threads(*folder, categories.as_deref())?,
                Vec::new(),
            )),
            Self::Spread { folders, filter } => {
                Ok((store.spread_threads(folders, *filter)?, Vec::new()))
            }
        }
    }

    /// One line of text, read back by [`ListRead::parse`].
    fn to_line(&self) -> String {
        let ids = |ids: &mut dyn Iterator<Item = i64>| {
            ids.map(|id| id.to_string()).collect::<Vec<_>>().join(",")
        };
        let categories = |categories: &Option<Vec<MailCategory>>| match categories {
            Some(categories) => format!("c{}", ids(&mut categories.iter().map(|c| c.to_storage()))),
            None => "-".to_owned(),
        };
        match self {
            Self::Inbox {
                folder,
                categories: c,
            } => {
                format!("inbox {} {}", folder.0, categories(c))
            }
            Self::Folder {
                folder,
                categories: c,
            } => {
                format!("folder {} {}", folder.0, categories(c))
            }
            Self::Spread { folders, filter } => format!(
                "spread {} {} {}",
                ids(&mut folders.iter().map(|f| f.0)),
                filter.set.bits(),
                filter.unset.bits()
            ),
        }
    }

    fn parse(line: &str) -> Option<Self> {
        fn ids(text: &str) -> Option<Vec<i64>> {
            text.split(',').map(|id| id.parse().ok()).collect()
        }
        fn categories(text: &str) -> Option<Option<Vec<MailCategory>>> {
            let text = match text {
                "-" => return Some(None),
                "c" => return Some(Some(Vec::new())),
                text => text.strip_prefix('c')?,
            };
            let categories = ids(text)?.into_iter().map(MailCategory::from_storage);
            Some(Some(categories.collect::<Option<_>>()?))
        }
        let mut words = line.split(' ');
        let read = match (words.next()?, words.next()?, words.next()?) {
            ("inbox", folder, c) => Self::Inbox {
                folder: FolderId(folder.parse().ok()?),
                categories: categories(c)?,
            },
            ("folder", folder, c) => Self::Folder {
                folder: FolderId(folder.parse().ok()?),
                categories: categories(c)?,
            },
            ("spread", folders, set) => Self::Spread {
                folders: ids(folders)?.into_iter().map(FolderId).collect(),
                filter: FlagFilter {
                    set: MessageFlags::from_bits(set.parse().ok()?),
                    unset: MessageFlags::from_bits(words.next()?.parse().ok()?),
                },
            },
            _ => return None,
        };
        words.next().is_none().then_some(read)
    }
}

/// What was read while the window started.
#[derive(Debug)]
pub struct Preload {
    /// The mail journal's newest change when read: a newer one makes all
    /// of it stale.
    pub change: i64,
    pub list: Option<(ListRead, InboxThreads)>,
    pub folders: Option<Vec<FolderSummary>>,
}

/// The reading started by [`Preloading::start`].
pub struct Preloading(mpsc::Receiver<Preload>);

impl Preloading {
    /// Starts reading the list the window showed when it last started, and
    /// the folders.
    pub fn start(paths: &Paths) -> Self {
        let (sender, receiver) = mpsc::sync_channel(1);
        let paths = paths.clone();
        let started = std::thread::Builder::new()
            .name("preload".into())
            .spawn(move || {
                if let Some(preload) = preload(&paths) {
                    let _ = sender.send(preload);
                }
            });
        // Without the thread the sender is gone and `wait` gives nothing.
        if let Err(err) = started {
            tracing::warn!(%err, "cannot read the mail early");
        }
        Self(receiver)
    }

    /// What was read, once it is; `None` when it failed or takes too long.
    pub fn wait(self) -> Option<Preload> {
        self.0.recv_timeout(WAIT).ok()
    }
}

fn preload(paths: &Paths) -> Option<Preload> {
    let started = std::time::Instant::now();
    let store = Store::open(paths, Mode::ReadOnly).ok()?;
    // Before the reads: a change during them makes the journal newer.
    let change = store.latest_change(DbKind::Mail).ok()?;
    let list = std::fs::read_to_string(list_file(paths))
        .ok()
        .and_then(|text| ListRead::parse(text.trim()))
        .and_then(|read| {
            let threads = read.read(&store).ok()?;
            Some((read, threads))
        });
    let folders = store.folder_summaries().ok();
    tracing::info!(elapsed = ?started.elapsed(), list = list.is_some(), "mail read early");
    Some(Preload {
        change,
        list,
        folders,
    })
}

/// Remembers `read` as the list to read early on the next start.
pub fn remember(paths: &Paths, read: &ListRead) {
    let saved = std::fs::create_dir_all(paths.cache_dir())
        .and_then(|()| std::fs::write(list_file(paths), read.to_line()));
    if let Err(err) = saved {
        tracing::info!(%err, "cannot remember the first list");
    }
}

fn list_file(paths: &Paths) -> PathBuf {
    paths.cache_dir().join("mail-first-list")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_list_read_survives_its_line() {
        let reads = [
            ListRead::Inbox {
                folder: FolderId(7),
                categories: None,
            },
            ListRead::Inbox {
                folder: FolderId(7),
                categories: Some(vec![MailCategory::Primary, MailCategory::Forums]),
            },
            ListRead::Folder {
                folder: FolderId(12),
                categories: Some(Vec::new()),
            },
            ListRead::Spread {
                folders: vec![FolderId(1), FolderId(40)],
                filter: FlagFilter::UNREAD,
            },
        ];
        for read in reads {
            assert_eq!(ListRead::parse(&read.to_line()), Some(read));
        }
        for bad in [
            "",
            "inbox",
            "inbox x -",
            "inbox 7 9",
            "inbox 7 c9",
            "spread 1 0",
            "folder 1 - x",
        ] {
            assert_eq!(ListRead::parse(bad), None, "{bad}");
        }
    }
}
