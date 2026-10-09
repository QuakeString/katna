// SPDX-License-Identifier: GPL-3.0-or-later

//! The folder tree of the sidebar: accounts, then their folders nested by
//! path, special folders first. No GPUI here, so it is tested directly.

use std::collections::{BTreeMap, HashMap, HashSet};

use katna_core::{Account, AccountId};
use katna_i18n::tr;
use katna_store::{FlagFilter, FolderId, FolderRole, FolderSummary};

/// Folder paths are split at this separator. Stalwart, Gmail and the
/// importers use `/`; servers with `.` show one level until the store keeps
/// each account's delimiter.
pub const SEPARATOR: char = '/';

/// Gmail keeps its system labels under one of these.
const GMAIL_ROOTS: [&str; 2] = ["[Gmail]", "[Google Mail]"];

/// The folder Katna's notes sync to (`katna_sync::notes::NOTES_FOLDER`).
const NOTES_FOLDER: &str = "Notes";

/// Accounts with at most this many folders start fully expanded.
const EXPAND_ALL_UP_TO: usize = 40;

/// What a folder is for, from `folder.role` or, failing that, its name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Role {
    Inbox,
    Flagged,
    /// Katna's `Snoozed` folder (`katna-daemon`'s snooze).
    Snoozed,
    Drafts,
    Sent,
    Archive,
    Junk,
    Trash,
    All,
    Other,
}

impl Role {
    pub fn detect(role: Option<&str>, name: &str) -> Self {
        let from_role = match role.map(str::to_ascii_lowercase).as_deref() {
            Some("inbox") => Some(Self::Inbox),
            Some("flagged") => Some(Self::Flagged),
            Some("drafts") => Some(Self::Drafts),
            Some("sent") => Some(Self::Sent),
            Some("archive") => Some(Self::Archive),
            Some("junk") => Some(Self::Junk),
            Some("trash") => Some(Self::Trash),
            Some("all") => Some(Self::All),
            _ => None,
        };
        from_role.unwrap_or_else(|| match FolderRole::from_name(name) {
            Some(FolderRole::Inbox) => Self::Inbox,
            Some(FolderRole::Flagged) => Self::Flagged,
            Some(FolderRole::Drafts) => Self::Drafts,
            Some(FolderRole::Sent) => Self::Sent,
            Some(FolderRole::Archive) => Self::Archive,
            Some(FolderRole::Junk) => Self::Junk,
            Some(FolderRole::Trash) => Self::Trash,
            Some(FolderRole::All) => Self::All,
            None if name.eq_ignore_ascii_case("snoozed") => Self::Snoozed,
            None => Self::Other,
        })
    }

    /// Whether messages in this folder are ones the user sent, so the list
    /// shows recipients instead of senders.
    pub fn shows_recipients(self) -> bool {
        matches!(self, Self::Sent | Self::Drafts)
    }

    /// The name a special folder shows in the current language, whatever
    /// the server calls it; `None` for the user's own folders, which keep
    /// their name.
    pub fn title(self) -> Option<String> {
        let id = match self {
            Self::Inbox => "folder-inbox",
            Self::Flagged => "folder-starred",
            Self::Snoozed => "folder-snoozed",
            Self::Drafts => "folder-drafts",
            Self::Sent => "folder-sent",
            Self::Archive => "folder-archive",
            Self::Junk => "folder-spam",
            Self::Trash => "folder-trash",
            Self::All => "folder-all-mail",
            Self::Other => return None,
        };
        Some(tr!(id))
    }
}

/// A list of the unified inbox, over every account: a special folder of
/// each, or the mail of all their folders with a flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Unified {
    Inbox,
    Unread,
    Starred,
    Important,
    Sent,
    AllMail,
    Junk,
    Trash,
    Drafts,
}

impl Unified {
    /// In the order the folder pane lists them.
    pub const ALL: [Self; 9] = [
        Self::Inbox,
        Self::Unread,
        Self::Starred,
        Self::Important,
        Self::Sent,
        Self::AllMail,
        Self::Junk,
        Self::Trash,
        Self::Drafts,
    ];

    /// The special folder of each account it lists, or `None` for the
    /// lists by flag, which cover every folder but trash and spam.
    pub fn role(self) -> Option<Role> {
        match self {
            Self::Inbox => Some(Role::Inbox),
            Self::Sent => Some(Role::Sent),
            Self::AllMail => Some(Role::All),
            Self::Junk => Some(Role::Junk),
            Self::Trash => Some(Role::Trash),
            Self::Drafts => Some(Role::Drafts),
            Self::Unread | Self::Starred | Self::Important => None,
        }
    }

    /// Which messages of its folders it keeps.
    pub fn filter(self) -> FlagFilter {
        match self {
            Self::Unread => FlagFilter::UNREAD,
            Self::Starred => FlagFilter::STARRED,
            Self::Important => FlagFilter::IMPORTANT,
            _ => FlagFilter::default(),
        }
    }

    /// Its name in the current language.
    pub fn title(self) -> String {
        match self {
            Self::Unread => tr!("folder-unread"),
            Self::Starred => tr!("folder-starred"),
            Self::Important => tr!("folder-important"),
            _ => self.role().and_then(Role::title).unwrap_or_default(),
        }
    }

    /// Its key among the expanded rows of the folder pane.
    pub fn key(self) -> &'static str {
        match self {
            Self::Inbox => "all:inbox",
            Self::Unread => "all:unread",
            Self::Starred => "all:starred",
            Self::Important => "all:important",
            Self::Sent => "all:sent",
            Self::AllMail => "all:all-mail",
            Self::Junk => "all:junk",
            Self::Trash => "all:trash",
            Self::Drafts => "all:drafts",
        }
    }

    /// Whether the list shows recipients instead of senders.
    pub fn shows_recipients(self) -> bool {
        self.role().is_some_and(Role::shows_recipients)
    }
}

impl AccountNode {
    /// Every node of the account, depth first.
    fn folders(&self) -> impl Iterator<Item = &Node> {
        fn walk<'a>(nodes: &'a [Node], out: &mut Vec<&'a Node>) {
            for node in nodes {
                out.push(node);
                walk(&node.children, out);
            }
        }
        let mut out = Vec::new();
        walk(&self.roots, &mut out);
        out.into_iter()
    }
}

/// One folder (or path component without a folder of its own).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    /// Unique in the tree: account ID and path.
    pub key: String,
    /// What the user sees; `INBOX` shows as `Inbox`.
    pub name: String,
    /// The path component as stored.
    pub segment: String,
    pub path: String,
    pub folder: Option<FolderId>,
    pub role: Role,
    pub total: u64,
    pub unread: u64,
    pub children: Vec<Node>,
}

impl Node {
    /// What the folder shows as: a special folder's name in the current
    /// language, else its own name.
    pub fn label(&self) -> String {
        self.role.title().unwrap_or_else(|| self.name.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountNode {
    pub id: AccountId,
    pub name: String,
    pub unread: u64,
    pub folder_count: usize,
    pub roots: Vec<Node>,
}

/// All accounts and their folders.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Tree {
    pub accounts: Vec<AccountNode>,
    /// What the unified inbox keeps out.
    pub unified_out: UnifiedOut,
}

/// The accounts the unified inbox keeps out, from Settings and the
/// right-click menu of an account's inbox under it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct UnifiedOut {
    /// Out of every list of it, rows and mail.
    pub hidden: HashSet<AccountId>,
    /// Their inbox's mail is out of the unified Inbox and its count; its
    /// row stays, dimmed, and opens that inbox.
    pub left_out: HashSet<AccountId>,
}

/// A visible line of the sidebar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Row {
    /// The heading of the unified inbox, over the accounts.
    AllAccounts { expanded: bool },
    /// A list of the unified inbox; expanded, one row per account follows.
    Unified {
        view: Unified,
        unread: u64,
        expanded: bool,
    },
    /// One account's part of a list of the unified inbox: its special
    /// folder, or for a list by flag, `folder: None`.
    UnifiedAccount {
        view: Unified,
        account: AccountId,
        name: String,
        folder: Option<FolderId>,
        unread: u64,
        /// An inbox left out of the unified Inbox.
        left_out: bool,
    },
    Account {
        id: AccountId,
        name: String,
        unread: u64,
        /// Its folders show under it.
        expanded: bool,
    },
    /// The header over an account's own folders, Gmail's "Labels".
    Labels { account: AccountId },
    Folder {
        key: String,
        depth: usize,
        label: String,
        role: Role,
        folder: Option<FolderId>,
        unread: u64,
        has_children: bool,
        expanded: bool,
    },
}

impl Tree {
    /// Builds the tree. `folders` may be in any order; accounts without
    /// folders are left out. `unread` has the unread count of each folder
    /// that has unread mail (it may still be empty while it is counted).
    pub fn build(
        accounts: &[Account],
        folders: &[FolderSummary],
        unread: &HashMap<FolderId, u64>,
    ) -> Self {
        let mut by_account: BTreeMap<AccountId, Vec<&FolderSummary>> = BTreeMap::new();
        for folder in folders {
            by_account.entry(folder.account).or_default().push(folder);
        }
        let name_of = |id: AccountId| {
            accounts
                .iter()
                .find(|a| a.id == id)
                .map(|a| {
                    if a.display_name.is_empty() {
                        a.address.clone()
                    } else {
                        a.display_name.clone()
                    }
                })
                .unwrap_or_else(|| tr!("nav-account-unnamed", number = id.0.to_string()))
        };
        let unread_of = |id: FolderId| unread.get(&id).copied().unwrap_or(0);
        // Accounts in the order of the account list, unknown ones last.
        let mut ids: Vec<AccountId> = by_account.keys().copied().collect();
        ids.sort_by_key(|id| {
            (
                accounts
                    .iter()
                    .position(|a| a.id == *id)
                    .unwrap_or(usize::MAX),
                id.0,
            )
        });
        let accounts = ids
            .into_iter()
            .map(|id| {
                let folders = &by_account[&id];
                let mut roots = Vec::new();
                for folder in folders {
                    insert(&mut roots, id, folder, unread_of(folder.id));
                }
                lift_gmail_labels(&mut roots);
                sort(&mut roots);
                AccountNode {
                    id,
                    name: name_of(id),
                    unread: folders
                        .iter()
                        .filter(|f| Role::detect(f.role.as_deref(), last(&f.path)) == Role::Inbox)
                        .map(|f| unread_of(f.id))
                        .sum(),
                    folder_count: folders.len(),
                    roots,
                }
            })
            .collect();
        Self {
            accounts,
            unified_out: UnifiedOut::default(),
        }
    }

    /// Keys of the nodes that start expanded: every node of small accounts.
    pub fn initially_expanded(&self) -> HashSet<String> {
        let mut keys = HashSet::new();
        for account in &self.accounts {
            if account.folder_count <= EXPAND_ALL_UP_TO {
                walk(&account.roots, &mut |node, _| {
                    if !node.children.is_empty() {
                        keys.insert(node.key.clone());
                    }
                });
            }
        }
        keys
    }

    /// The folder to open first: the first inbox with mail, else the first
    /// folder with mail, else the first folder. Also returns the keys of its
    /// ancestors, which must be expanded to show it.
    /// Among the folders of `account` only, when one is given.
    pub fn default_folder_in(&self, account: Option<AccountId>) -> Option<(FolderId, Vec<String>)> {
        let mut candidates: [Option<(FolderId, Vec<String>)>; 3] = [None, None, None];
        for node in &self.accounts {
            if account.is_none_or(|id| id == node.id) {
                find(&node.roots, &mut Vec::new(), &mut candidates);
            }
        }
        candidates.into_iter().flatten().next()
    }

    /// The node of `folder`.
    pub fn node(&self, folder: FolderId) -> Option<&Node> {
        fn go(nodes: &[Node], folder: FolderId) -> Option<&Node> {
            for node in nodes {
                if node.folder == Some(folder) {
                    return Some(node);
                }
                if let Some(found) = go(&node.children, folder) {
                    return Some(found);
                }
            }
            None
        }
        self.accounts.iter().find_map(|a| go(&a.roots, folder))
    }

    /// The account `folder` belongs to.
    pub fn account_of(&self, folder: FolderId) -> Option<AccountId> {
        self.accounts
            .iter()
            .find(|a| a.folders().any(|n| n.folder == Some(folder)))
            .map(|a| a.id)
    }

    /// The first folder of `account` with `role`.
    pub fn role_folder(&self, account: AccountId, role: Role) -> Option<FolderId> {
        self.accounts
            .iter()
            .find(|a| a.id == account)?
            .folders()
            .find(|n| n.role == role)
            .and_then(|n| n.folder)
    }

    /// The folders of `account` in tree order, with the names they show as.
    pub fn folders_of(&self, account: AccountId) -> Vec<(FolderId, String, Role)> {
        self.accounts
            .iter()
            .filter(|a| a.id == account)
            .flat_map(|a| a.folders())
            .filter_map(|n| Some((n.folder?, n.label(), n.role)))
            .collect()
    }

    /// Whether `account` is a Gmail account, whose folders are labels.
    pub fn is_gmail(&self, account: AccountId) -> bool {
        self.accounts
            .iter()
            .filter(|a| a.id == account)
            .flat_map(|a| &a.roots)
            .any(|n| GMAIL_ROOTS.contains(&root_segment(&n.path)))
    }

    /// The folders of `account` a new folder may go inside, with their
    /// paths: the user's own, not the special ones or Gmail's system
    /// labels.
    pub fn nest_targets(&self, account: AccountId) -> Vec<(FolderId, String)> {
        self.accounts
            .iter()
            .filter(|a| a.id == account)
            .flat_map(|a| a.folders())
            .filter(|n| n.role == Role::Other)
            .filter(|n| {
                let root = n.path.split(SEPARATOR).next().unwrap_or_default();
                !GMAIL_ROOTS.contains(&root)
            })
            .filter_map(|n| Some((n.folder?, n.path.clone())))
            .collect()
    }

    /// Whether `folder` is one the user made, which can be renamed and
    /// deleted: not a special folder (by role or name), not one of Gmail's
    /// system labels or the notes folder, and holding no special folder.
    /// As the daemon's rule (`katna_sync::folders::is_special`), only
    /// stricter: a special name anywhere counts.
    pub fn editable(&self, folder: FolderId) -> bool {
        let Some(node) = self.node(folder) else {
            return false;
        };
        let root = node.path.split(SEPARATOR).next().unwrap_or_default();
        let mut plain = true;
        walk(std::slice::from_ref(node), &mut |n, _| {
            plain &= n.role == Role::Other;
        });
        plain && !GMAIL_ROOTS.contains(&root) && node.path != NOTES_FOLDER
    }

    /// `folder` and the folders inside it.
    pub fn subtree(&self, folder: FolderId) -> Vec<FolderId> {
        let mut out = Vec::new();
        if let Some(node) = self.node(folder) {
            walk(std::slice::from_ref(node), &mut |n, _| {
                out.extend(n.folder);
            });
        }
        out
    }

    /// The visible rows, given the expanded node keys: of every account,
    /// or of `only` when given. Only accounts `open` says are open show
    /// their folders.
    pub fn rows(
        &self,
        expanded: &HashSet<String>,
        only: Option<AccountId>,
        open: impl Fn(AccountId) -> bool,
    ) -> Vec<Row> {
        let mut rows = Vec::new();
        for account in &self.accounts {
            if only.is_some_and(|id| id != account.id) {
                continue;
            }
            let is_open = open(account.id);
            rows.push(Row::Account {
                id: account.id,
                name: account.name.clone(),
                unread: account.unread,
                expanded: is_open,
            });
            if !is_open {
                continue;
            }
            // The user's own folders (labels) come after the special ones,
            // under a header with the button that makes a new one.
            let special = account
                .roots
                .iter()
                .take_while(|n| n.role != Role::Other || is_gmail_label(n))
                .count();
            push_rows(&account.roots[..special], 0, expanded, &mut rows);
            rows.push(Row::Labels {
                account: account.id,
            });
            push_rows(&account.roots[special..], 0, expanded, &mut rows);
        }
        rows
    }
}

impl Tree {
    /// The rows of the unified inbox: its heading and, when `open`, its
    /// lists that some account has, each followed by its accounts when
    /// expanded (by [`Unified::key`]). Empty with fewer than two accounts.
    pub fn unified_rows(&self, expanded: &HashSet<String>, open: bool) -> Vec<Row> {
        if self.accounts.len() < 2 {
            return Vec::new();
        }
        let mut rows = vec![Row::AllAccounts { expanded: open }];
        if !open {
            return rows;
        }
        for view in Unified::ALL {
            let parts = self.unified_parts(view);
            if parts.is_empty() {
                continue;
            }
            let is_expanded = expanded.contains(view.key());
            rows.push(Row::Unified {
                view,
                // Only special folders have counts: counting mail by flag
                // across every folder is too slow to do on each change.
                unread: parts
                    .iter()
                    .map(|p| match p {
                        Row::UnifiedAccount {
                            unread,
                            left_out: false,
                            ..
                        } => *unread,
                        _ => 0,
                    })
                    .sum(),
                expanded: is_expanded,
            });
            if is_expanded {
                rows.extend(parts);
            }
        }
        rows
    }

    /// The unified Inbox's unread count, without the inboxes left out.
    pub fn unified_inbox_unread(&self) -> u64 {
        self.unified_parts(Unified::Inbox)
            .iter()
            .map(|p| match p {
                Row::UnifiedAccount {
                    unread,
                    left_out: false,
                    ..
                } => *unread,
                _ => 0,
            })
            .sum()
    }

    /// The rows of each account in `view`: every account for a list by
    /// flag, those with the folder for a special folder.
    fn unified_parts(&self, view: Unified) -> Vec<Row> {
        self.accounts
            .iter()
            .filter(|a| !self.unified_out.hidden.contains(&a.id))
            .filter_map(|account| {
                let (folder, unread) = match view.role() {
                    Some(role) => {
                        let node = account
                            .folders()
                            .find(|n| n.role == role && n.folder.is_some())?;
                        (node.folder, node.unread)
                    }
                    None => (None, 0),
                };
                Some(Row::UnifiedAccount {
                    view,
                    account: account.id,
                    name: account.name.clone(),
                    folder,
                    unread,
                    left_out: view == Unified::Inbox
                        && self.unified_out.left_out.contains(&account.id),
                })
            })
            .collect()
    }

    /// The folders `view` lists mail from, of `account` or of all those
    /// it keeps in.
    pub fn unified_folders(&self, view: Unified, account: Option<AccountId>) -> Vec<FolderId> {
        let out = &self.unified_out;
        self.accounts
            .iter()
            .filter(|a| match account {
                Some(id) => id == a.id,
                None => {
                    !out.hidden.contains(&a.id)
                        && !(view == Unified::Inbox && out.left_out.contains(&a.id))
                }
            })
            .flat_map(|a| match view.role() {
                Some(role) => a
                    .folders()
                    .find(|n| n.role == role && n.folder.is_some())
                    .and_then(|n| n.folder)
                    .into_iter()
                    .collect::<Vec<_>>(),
                None => a
                    .folders()
                    .filter(|n| !matches!(n.role, Role::Trash | Role::Junk))
                    .filter_map(|n| n.folder)
                    .collect(),
            })
            .collect()
    }
}

fn last(path: &str) -> &str {
    path.rsplit(SEPARATOR).next().unwrap_or(path)
}

fn insert(nodes: &mut Vec<Node>, account: AccountId, folder: &FolderSummary, unread: u64) {
    let mut level = nodes;
    let mut prefix = String::new();
    let parts: Vec<&str> = folder
        .path
        .split(SEPARATOR)
        .filter(|p| !p.is_empty())
        .collect();
    let parts = if parts.is_empty() {
        vec![folder.path.as_str()]
    } else {
        parts
    };
    for (i, part) in parts.iter().enumerate() {
        if i > 0 {
            prefix.push(SEPARATOR);
        }
        prefix.push_str(part);
        let at = match level.iter().position(|n| n.segment == *part) {
            Some(at) => at,
            None => {
                level.push(Node {
                    key: format!("{}:{prefix}", account.0),
                    name: (*part).to_owned(),
                    segment: (*part).to_owned(),
                    path: prefix.clone(),
                    folder: None,
                    role: Role::Other,
                    total: 0,
                    unread: 0,
                    children: Vec::new(),
                });
                level.len() - 1
            }
        };
        if i + 1 == parts.len() {
            let node = &mut level[at];
            node.folder = Some(folder.id);
            node.role = Role::detect(folder.role.as_deref(), part);
            node.total = folder.total;
            node.unread = unread;
            if node.role == Role::Inbox && node.name.eq_ignore_ascii_case("inbox") {
                node.name = "Inbox".to_owned();
            }
            return;
        }
        level = &mut level[at].children;
    }
}

/// The first part of `path`.
fn root_segment(path: &str) -> &str {
    path.split(SEPARATOR).next().unwrap_or(path)
}

/// Whether `node` is one of Gmail's own labels, such as Starred or
/// Important, lifted out of `[Gmail]`.
fn is_gmail_label(node: &Node) -> bool {
    node.path.contains(SEPARATOR) && GMAIL_ROOTS.contains(&root_segment(&node.path))
}

/// Whether the folder pane line `key` is Gmail's Important label.
pub fn is_gmail_important(key: &str) -> bool {
    let path = key.split_once(':').map_or(key, |(_, path)| path);
    GMAIL_ROOTS.iter().any(|root| {
        path.strip_prefix(root)
            .and_then(|p| p.strip_prefix(SEPARATOR))
            .is_some_and(|p| p.eq_ignore_ascii_case("important"))
    })
}

/// Brings Gmail's own labels (Starred, Important, Sent Mail, Drafts, All
/// Mail, Spam, Trash…) out of `[Gmail]` to the top, beside Inbox, as Gmail
/// lists them. `[Gmail]` itself stays only if it holds mail.
fn lift_gmail_labels(roots: &mut Vec<Node>) {
    let mut lifted = Vec::new();
    roots.retain_mut(|root| {
        if !GMAIL_ROOTS.contains(&root.segment.as_str()) {
            return true;
        }
        lifted.append(&mut root.children);
        root.folder.is_some()
    });
    roots.append(&mut lifted);
}

/// Where a top-level folder goes: special folders first, Gmail's
/// Important after Starred and Snoozed, its other labels after the
/// special ones, then the user's own.
fn rank(node: &Node) -> (Role, u8) {
    match node.role {
        Role::Other if is_gmail_label(node) && node.segment.eq_ignore_ascii_case("important") => {
            (Role::Snoozed, 1)
        }
        Role::Other if is_gmail_label(node) => (Role::All, 1),
        role => (role, 0),
    }
}

fn sort(nodes: &mut [Node]) {
    // Special folders first, then by name, ignoring case and leading
    // punctuation such as Gmail's `[Gmail]`.
    let sort_key = |n: &Node| {
        (
            rank(n),
            n.name
                .trim_start_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase(),
            n.name.clone(),
        )
    };
    nodes.sort_by_cached_key(sort_key);
    for node in nodes {
        sort(&mut node.children);
    }
}

fn walk<'a>(nodes: &'a [Node], f: &mut impl FnMut(&'a Node, usize)) {
    fn go<'a>(nodes: &'a [Node], depth: usize, f: &mut impl FnMut(&'a Node, usize)) {
        for node in nodes {
            f(node, depth);
            go(&node.children, depth + 1, f);
        }
    }
    go(nodes, 0, f);
}

fn find(
    nodes: &[Node],
    ancestors: &mut Vec<String>,
    candidates: &mut [Option<(FolderId, Vec<String>)>; 3],
) {
    for node in nodes {
        if let Some(folder) = node.folder {
            let slot = if node.role == Role::Inbox && node.total > 0 {
                0
            } else if node.total > 0 {
                1
            } else {
                2
            };
            if candidates[slot].is_none() {
                candidates[slot] = Some((folder, ancestors.clone()));
            }
        }
        ancestors.push(node.key.clone());
        find(&node.children, ancestors, candidates);
        ancestors.pop();
    }
}

fn push_rows(nodes: &[Node], depth: usize, expanded: &HashSet<String>, rows: &mut Vec<Row>) {
    for node in nodes {
        let is_expanded = expanded.contains(&node.key);
        rows.push(Row::Folder {
            key: node.key.clone(),
            depth,
            label: node.name.clone(),
            role: node.role,
            folder: node.folder,
            unread: node.unread,
            has_children: !node.children.is_empty(),
            expanded: is_expanded,
        });
        if is_expanded {
            push_rows(&node.children, depth + 1, expanded, rows);
        }
    }
}

#[cfg(test)]
mod tests {
    use katna_core::AccountKind;

    use super::*;

    fn folder(id: i64, account: i64, path: &str, total: u64) -> FolderSummary {
        FolderSummary {
            id: FolderId(id),
            account: AccountId(account),
            path: path.to_owned(),
            role: None,
            total,
        }
    }

    fn labels(rows: &[Row]) -> Vec<String> {
        rows.iter()
            .map(|row| match row {
                Row::AllAccounts { expanded } => {
                    format!("# All{}", if *expanded { " -" } else { " +" })
                }
                Row::Unified {
                    view,
                    expanded,
                    unread,
                } => format!("{:?} {unread}{}", view, if *expanded { " -" } else { " +" }),
                Row::UnifiedAccount {
                    name,
                    folder,
                    unread,
                    left_out,
                    ..
                } => {
                    let out = if *left_out { " out" } else { "" };
                    format!("  {name} {:?} {unread}{out}", folder.map(|f| f.0))
                }
                Row::Account { name, expanded, .. } => {
                    format!("# {name}{}", if *expanded { "" } else { " +" })
                }
                Row::Labels { .. } => "## Labels".to_owned(),
                Row::Folder {
                    depth,
                    label,
                    expanded,
                    has_children,
                    ..
                } => format!(
                    "{}{label}{}",
                    "  ".repeat(*depth),
                    match (has_children, expanded) {
                        (false, _) => "",
                        (true, true) => " -",
                        (true, false) => " +",
                    }
                ),
            })
            .collect()
    }

    #[test]
    fn new_folders_nest_under_the_users_own() {
        let folders = [
            folder(1, 1, "INBOX", 1),
            folder(2, 1, "[Gmail]/All Mail", 1),
            folder(3, 1, "[Gmail]/Starred", 0),
            folder(4, 1, "Work", 0),
            folder(5, 1, "Work/Clients", 0),
            folder(6, 1, "Sent", 0),
            folder(7, 2, "Projects", 0),
        ];
        let tree = Tree::build(&[], &folders, &HashMap::new());
        assert!(tree.is_gmail(AccountId(1)));
        assert!(!tree.is_gmail(AccountId(2)));
        assert_eq!(
            tree.nest_targets(AccountId(1)),
            [
                (FolderId(4), "Work".to_owned()),
                (FolderId(5), "Work/Clients".to_owned())
            ]
        );
        assert_eq!(
            tree.nest_targets(AccountId(2)),
            [(FolderId(7), "Projects".to_owned())]
        );
    }

    #[test]
    fn only_the_users_own_folders_are_editable() {
        let mut sent = folder(6, 1, "Sent", 0);
        sent.role = Some("sent".to_owned());
        let folders = [
            folder(1, 1, "INBOX", 1),
            folder(2, 1, "INBOX/Receipts", 1),
            folder(3, 1, "[Gmail]/All Mail", 1),
            folder(4, 1, "Work", 0),
            folder(5, 1, "Work/Clients", 0),
            sent,
            folder(7, 1, "Notes", 0),
            folder(8, 1, "Old", 0),
            folder(9, 1, "Old/Trash", 0),
            folder(10, 1, "Snoozed", 0),
        ];
        let tree = Tree::build(&[], &folders, &HashMap::new());
        let editable: Vec<i64> = folders
            .iter()
            .filter(|f| tree.editable(f.id))
            .map(|f| f.id.0)
            .collect();
        // Not the inbox, Gmail's labels, Sent, Notes, Snoozed, or a folder
        // holding a special one.
        assert_eq!(editable, [2, 4, 5]);
        assert!(!tree.editable(FolderId(99)));
        assert_eq!(tree.subtree(FolderId(4)), [FolderId(4), FolderId(5)]);
        assert!(tree.subtree(FolderId(99)).is_empty());
    }

    #[test]
    fn roles() {
        assert_eq!(Role::detect(Some("sent"), "Whatever"), Role::Sent);
        assert_eq!(Role::detect(None, "INBOX"), Role::Inbox);
        assert_eq!(Role::detect(None, "Sent Items"), Role::Sent);
        assert_eq!(Role::detect(None, "_sent_mail"), Role::Sent);
        assert_eq!(Role::detect(None, "[Gmail]"), Role::Other);
        assert_eq!(Role::detect(None, "Spam"), Role::Junk);
        assert_eq!(Role::detect(Some("flagged"), "Starred"), Role::Flagged);
        assert!(Role::Sent.shows_recipients() && !Role::Inbox.shows_recipients());
    }

    #[test]
    fn gmail_labels_sit_beside_the_inbox() {
        let mut folders = vec![
            folder(1, 1, "INBOX", 4),
            folder(2, 1, "[Gmail]/Sent Mail", 0),
            folder(3, 1, "[Gmail]/Starred", 0),
            folder(4, 1, "[Gmail]/Important", 2),
            folder(5, 1, "[Gmail]/All Mail", 9),
            folder(6, 1, "[Gmail]/Spam", 0),
            folder(7, 1, "[Gmail]/Drafts", 0),
            folder(8, 1, "Receipts", 0),
            folder(9, 1, "[Gmail]/Chats", 0),
        ];
        for (ix, role) in [
            (1, "sent"),
            (2, "flagged"),
            (4, "all"),
            (5, "junk"),
            (6, "drafts"),
        ] {
            folders[ix].role = Some(role.to_owned());
        }
        let tree = Tree::build(&[], &folders, &HashMap::new());
        assert!(tree.is_gmail(AccountId(1)));
        let rows = tree.rows(&tree.initially_expanded(), None, |_| true);
        assert_eq!(
            labels(&rows),
            [
                "# Account 1",
                "Inbox",
                "Starred",
                "Important",
                "Drafts",
                "Sent Mail",
                "Spam",
                "All Mail",
                "Chats",
                "## Labels",
                "Receipts",
            ]
        );
        // Still Gmail's own: never renamed, deleted or a place for new ones.
        assert!(!tree.editable(FolderId(4)));
        assert!(is_gmail_important("1:[Gmail]/Important"));
        assert!(!is_gmail_important("1:Important"));
        assert_eq!(
            tree.nest_targets(AccountId(1)),
            [(FolderId(8), "Receipts".to_owned())]
        );
    }

    #[test]
    fn nested_folders_special_first() {
        let accounts = [Account {
            id: AccountId(1),
            kind: AccountKind::Imap,
            display_name: "Work".into(),
            address: "ada@example.org".into(),
        }];
        let folders = [
            folder(1, 1, "Projects/Katna", 3),
            folder(2, 1, "INBOX", 10),
            folder(3, 1, "[Gmail]/Sent Mail", 5),
            folder(4, 1, "archive", 2),
            folder(5, 1, "INBOX/Receipts", 1),
            folder(6, 2, "Inbox", 0),
        ];
        let unread = HashMap::from([(FolderId(1), 1), (FolderId(2), 4), (FolderId(5), 1)]);
        let tree = Tree::build(&accounts, &folders, &unread);
        assert_eq!(tree.node(FolderId(1)).unwrap().unread, 1);
        assert_eq!(tree.accounts.len(), 2);
        assert_eq!(tree.accounts[0].unread, 4);
        assert_eq!(tree.accounts[1].name, "Account 2");

        let rows = tree.rows(&tree.initially_expanded(), None, |_| true);
        assert_eq!(
            labels(&rows),
            [
                "# Work",
                "Inbox -",
                "  Receipts",
                "Sent Mail",
                "archive",
                "## Labels",
                "Projects -",
                "  Katna",
                "# Account 2",
                "Inbox",
                "## Labels",
            ]
        );
        let collapsed = tree.rows(&HashSet::new(), None, |_| true);
        assert_eq!(
            labels(&collapsed)[..4],
            ["# Work", "Inbox +", "Sent Mail", "archive"]
        );
        let Row::Folder { folder, role, .. } = &rows[6] else {
            panic!("expected a folder row");
        };
        assert_eq!((*folder, *role), (None, Role::Other));

        assert_eq!(
            tree.default_folder_in(None),
            Some((FolderId(2), Vec::new()))
        );

        // One account at a time.
        let only = tree.rows(&HashSet::new(), Some(AccountId(2)), |_| true);
        assert_eq!(labels(&only), ["# Account 2", "Inbox", "## Labels"]);
        assert_eq!(
            tree.default_folder_in(Some(AccountId(2))),
            Some((FolderId(6), Vec::new()))
        );
        assert_eq!(tree.default_folder_in(Some(AccountId(9))), None);
        assert_eq!(tree.node(FolderId(3)).unwrap().role, Role::Sent);
        assert_eq!(tree.node(FolderId(99)), None);
    }

    #[test]
    fn large_accounts_start_collapsed_at_their_first_inbox() {
        let mut folders = Vec::new();
        for owner in 0..30 {
            folders.push(folder(
                owner * 2 + 1,
                1,
                &format!("user{owner:02}/notes"),
                1,
            ));
            folders.push(folder(
                owner * 2 + 2,
                1,
                &format!("user{owner:02}/inbox"),
                0,
            ));
        }
        // The first inbox with mail is user03's.
        folders.push(folder(1000, 1, "user03/inbox/sub", 0));
        folders[7].total = 5;
        assert_eq!(folders[7].path, "user03/inbox");
        let tree = Tree::build(&[], &folders, &HashMap::new());
        assert!(tree.initially_expanded().is_empty());
        let (id, ancestors) = tree.default_folder_in(None).unwrap();
        assert_eq!(id, FolderId(8));
        assert_eq!(ancestors, ["1:user03"]);
        let expanded: HashSet<String> = ancestors.into_iter().collect();
        let rows = tree.rows(&expanded, None, |_| true);
        assert_eq!(rows.len(), 2 + 30 + 2);
        assert_eq!(labels(&rows)[1], "## Labels");
        assert_eq!(labels(&rows)[5..8], ["user03 -", "  Inbox +", "  notes"]);
    }

    #[test]
    fn empty_store() {
        let tree = Tree::build(&[], &[], &HashMap::new());
        assert!(tree.rows(&HashSet::new(), None, |_| true).is_empty());
        assert_eq!(tree.default_folder_in(None), None);
    }

    #[test]
    fn unified_inbox_over_the_accounts() {
        let accounts = [
            Account {
                id: AccountId(1),
                kind: AccountKind::Imap,
                display_name: String::new(),
                address: "ada@example.org".into(),
            },
            Account {
                id: AccountId(2),
                kind: AccountKind::Imap,
                display_name: String::new(),
                address: "kay@example.org".into(),
            },
        ];
        let folders = [
            folder(1, 1, "INBOX", 3),
            folder(2, 1, "Sent", 1),
            folder(3, 1, "Trash", 1),
            folder(4, 1, "Work", 1),
            folder(5, 2, "INBOX", 2),
            folder(6, 2, "Spam", 0),
        ];
        let unread = HashMap::from([(FolderId(1), 2), (FolderId(5), 1), (FolderId(4), 7)]);
        let tree = Tree::build(&accounts, &folders, &unread);

        let folded = tree.unified_rows(&HashSet::new(), false);
        assert_eq!(labels(&folded), ["# All +"]);
        let rows = tree.unified_rows(&HashSet::from(["all:inbox".to_owned()]), true);
        assert_eq!(
            labels(&rows),
            [
                "# All -",
                "Inbox 3 -",
                "  ada@example.org Some(1) 2",
                "  kay@example.org Some(5) 1",
                "Unread 0 +",
                "Starred 0 +",
                "Important 0 +",
                "Sent 0 +",
                "Junk 0 +",
                "Trash 0 +",
            ]
        );
        let unread_parts = tree.unified_rows(&HashSet::from(["all:unread".to_owned()]), true);
        assert_eq!(
            labels(&unread_parts)[3..5],
            ["  ada@example.org None 0", "  kay@example.org None 0"]
        );

        assert_eq!(
            tree.unified_folders(Unified::Inbox, None),
            [FolderId(1), FolderId(5)]
        );
        assert_eq!(tree.unified_folders(Unified::Sent, Some(AccountId(2))), []);
        // Lists by flag cover every folder but trash and spam.
        assert_eq!(
            tree.unified_folders(Unified::Unread, None),
            [FolderId(1), FolderId(2), FolderId(4), FolderId(5)]
        );

        // A left-out inbox keeps its row, out of the Inbox's count and
        // mail, and still opens on its own.
        let mut out = tree.clone();
        out.unified_out.left_out.insert(AccountId(1));
        let rows = out.unified_rows(&HashSet::from(["all:inbox".to_owned()]), true);
        assert_eq!(
            labels(&rows)[1..4],
            [
                "Inbox 1 -",
                "  ada@example.org Some(1) 2 out",
                "  kay@example.org Some(5) 1"
            ]
        );
        assert_eq!(out.unified_folders(Unified::Inbox, None), [FolderId(5)]);
        assert_eq!(
            out.unified_folders(Unified::Inbox, Some(AccountId(1))),
            [FolderId(1)]
        );
        assert_eq!(out.unified_folders(Unified::Sent, None), [FolderId(2)]);
        // A hidden account is out of every list.
        let mut hidden = tree.clone();
        hidden.unified_out.hidden.insert(AccountId(1));
        let rows = hidden.unified_rows(&HashSet::from(["all:inbox".to_owned()]), true);
        assert_eq!(
            labels(&rows)[1..3],
            ["Inbox 1 -", "  kay@example.org Some(5) 1"]
        );
        assert_eq!(hidden.unified_folders(Unified::Unread, None), [FolderId(5)]);

        // Folded accounts show only their name.
        let rows = tree.rows(&HashSet::new(), None, |id| id == AccountId(2));
        assert_eq!(
            labels(&rows),
            [
                "# ada@example.org +",
                "# kay@example.org",
                "Inbox",
                "Spam",
                "## Labels"
            ]
        );

        // One account has no unified inbox.
        let one = Tree::build(&accounts, &folders[..4], &unread);
        assert!(one.unified_rows(&HashSet::new(), true).is_empty());
    }
}
