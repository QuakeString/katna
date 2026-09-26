// SPDX-License-Identifier: GPL-3.0-or-later

//! The folder tree of the sidebar: accounts, then their folders nested by
//! path, special folders first. No GPUI here, so it is tested directly.

use std::collections::{BTreeMap, HashMap, HashSet};

use katna_core::{Account, AccountId};
use katna_store::{FolderId, FolderRole, FolderSummary};

/// Folder paths are split at this separator. Stalwart, Gmail and the
/// importers use `/`; servers with `.` show one level until the store keeps
/// each account's delimiter.
pub const SEPARATOR: char = '/';

/// Gmail keeps its system labels under one of these.
const GMAIL_ROOTS: [&str; 2] = ["[Gmail]", "[Google Mail]"];

/// Accounts with at most this many folders start fully expanded.
const EXPAND_ALL_UP_TO: usize = 40;

/// What a folder is for, from `folder.role` or, failing that, its name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Role {
    Inbox,
    Flagged,
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
            None => Self::Other,
        })
    }

    /// Whether messages in this folder are ones the user sent, so the list
    /// shows recipients instead of senders.
    pub fn shows_recipients(self) -> bool {
        matches!(self, Self::Sent | Self::Drafts)
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
}

/// A visible line of the sidebar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Row {
    Account {
        id: AccountId,
        name: String,
        unread: u64,
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
                .unwrap_or_else(|| format!("Account {}", id.0))
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
        Self { accounts }
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

    /// The folders of `account` in tree order, with their full names.
    pub fn folders_of(&self, account: AccountId) -> Vec<(FolderId, String, Role)> {
        self.accounts
            .iter()
            .filter(|a| a.id == account)
            .flat_map(|a| a.folders())
            .filter_map(|n| Some((n.folder?, n.name.clone(), n.role)))
            .collect()
    }

    /// Whether `account` is a Gmail account, whose folders are labels.
    pub fn is_gmail(&self, account: AccountId) -> bool {
        self.accounts
            .iter()
            .filter(|a| a.id == account)
            .flat_map(|a| &a.roots)
            .any(|n| GMAIL_ROOTS.contains(&n.segment.as_str()))
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

    /// The visible rows, given the expanded node keys: of every account,
    /// or of `only` when given.
    pub fn rows(&self, expanded: &HashSet<String>, only: Option<AccountId>) -> Vec<Row> {
        let mut rows = Vec::new();
        for account in &self.accounts {
            if only.is_some_and(|id| id != account.id) {
                continue;
            }
            rows.push(Row::Account {
                id: account.id,
                name: account.name.clone(),
                unread: account.unread,
            });
            // The user's own folders (labels) come after the special ones,
            // under a header with the button that makes a new one.
            let special = account
                .roots
                .iter()
                .take_while(|n| n.role != Role::Other)
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

fn sort(nodes: &mut [Node]) {
    // Special folders first, then by name, ignoring case and leading
    // punctuation such as Gmail's `[Gmail]`.
    let sort_key = |n: &Node| {
        (
            n.role,
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
                Row::Account { name, .. } => format!("# {name}"),
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

        let rows = tree.rows(&tree.initially_expanded(), None);
        assert_eq!(
            labels(&rows),
            [
                "# Work",
                "Inbox -",
                "  Receipts",
                "archive",
                "## Labels",
                "[Gmail] -",
                "  Sent Mail",
                "Projects -",
                "  Katna",
                "# Account 2",
                "Inbox",
                "## Labels",
            ]
        );
        let collapsed = tree.rows(&HashSet::new(), None);
        assert_eq!(
            labels(&collapsed)[..4],
            ["# Work", "Inbox +", "archive", "## Labels"]
        );
        let Row::Folder { folder, role, .. } = &rows[7] else {
            panic!("expected a folder row");
        };
        assert_eq!((*folder, *role), (None, Role::Other));

        assert_eq!(
            tree.default_folder_in(None),
            Some((FolderId(2), Vec::new()))
        );

        // One account at a time.
        let only = tree.rows(&HashSet::new(), Some(AccountId(2)));
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
        let rows = tree.rows(&expanded, None);
        assert_eq!(rows.len(), 2 + 30 + 2);
        assert_eq!(labels(&rows)[1], "## Labels");
        assert_eq!(labels(&rows)[5..8], ["user03 -", "  Inbox +", "  notes"]);
    }

    #[test]
    fn empty_store() {
        let tree = Tree::build(&[], &[], &HashMap::new());
        assert!(tree.rows(&HashSet::new(), None).is_empty());
        assert_eq!(tree.default_folder_in(None), None);
    }
}
