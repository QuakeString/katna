// SPDX-License-Identifier: GPL-3.0-or-later

//! The reply tree of one thread, after Jamie Zawinski's threading algorithm
//! (<https://www.jwz.org/doc/threading.html>), steps 1 to 4: link messages
//! by `Message-ID` and `References`, then drop the messages we never saw
//! and hang their replies on the nearest ancestor we have. Grouping
//! messages into threads is done in the store as they arrive
//! (`threads.rs`); this only orders one thread for display.

use std::collections::HashMap;

use crate::mail::MessageId;

/// A message as the tree needs it.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Item<'a> {
    pub id: MessageId,
    pub message_id: Option<&'a str>,
    /// Ancestors, oldest first; the last is the parent.
    pub refs: &'a [String],
    pub date: Option<i64>,
}

/// One message in a thread's reply tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThreadNode {
    pub message: MessageId,
    /// The nearest ancestor in the thread, if any.
    pub parent: Option<MessageId>,
    /// 0 for a message that starts a branch.
    pub depth: u32,
}

#[derive(Default)]
struct Container {
    /// Index into the items; `None` for a message only referred to.
    item: Option<usize>,
    parent: Option<usize>,
    children: Vec<usize>,
}

/// The messages in depth-first order, each branch sorted by date.
pub(crate) fn tree(items: &[Item<'_>]) -> Vec<ThreadNode> {
    let mut containers: Vec<Container> = Vec::new();
    let mut by_id: HashMap<&str, usize> = HashMap::new();
    for (index, item) in items.iter().enumerate() {
        // Step 1A: the message's own container. A second copy of the same
        // Message-ID (the message in two folders) gets its own.
        let own = match item.message_id {
            Some(id) if !by_id.contains_key(id) => {
                let at = new_container(&mut containers);
                by_id.insert(id, at);
                at
            }
            Some(id) if containers[by_id[id]].item.is_none() => by_id[id],
            _ => new_container(&mut containers),
        };
        containers[own].item = Some(index);

        // Step 1B: link the references in order, never making a loop and
        // never moving a container that already has a parent.
        let mut previous: Option<usize> = None;
        for reference in item.refs {
            let at = match by_id.get(reference.as_str()) {
                Some(&at) => at,
                None => {
                    let at = new_container(&mut containers);
                    by_id.insert(reference.as_str(), at);
                    at
                }
            };
            if let Some(parent) = previous
                && containers[at].parent.is_none()
                && !reaches(&containers, at, parent)
            {
                link(&mut containers, parent, at);
            }
            previous = Some(at);
        }

        // Step 1C: the last reference is the parent, whatever step 1B of an
        // earlier message guessed.
        if let Some(parent) = previous
            && !reaches(&containers, own, parent)
        {
            unlink(&mut containers, own);
            link(&mut containers, parent, own);
        }
    }

    // Earliest date in each subtree, for ordering branches.
    let mut earliest: Vec<Option<i64>> = vec![None; containers.len()];
    let roots: Vec<usize> = (0..containers.len())
        .filter(|&c| containers[c].parent.is_none())
        .collect();
    for &root in &roots {
        fill_earliest(&containers, items, root, &mut earliest);
    }
    let key = |c: usize| (earliest[c].unwrap_or(i64::MAX), c);

    let mut out = Vec::with_capacity(items.len());
    let mut roots = roots;
    roots.sort_by_key(|&c| key(c));
    // (container, depth, parent message), popped in order.
    let mut stack: Vec<(usize, u32, Option<MessageId>)> =
        roots.iter().rev().map(|&c| (c, 0, None)).collect();
    while let Some((c, depth, parent)) = stack.pop() {
        let (child_depth, child_parent) = match containers[c].item {
            Some(index) => {
                let message = items[index].id;
                out.push(ThreadNode {
                    message,
                    parent,
                    depth,
                });
                (depth + 1, Some(message))
            }
            // Step 4: a message we never saw; its replies take its place.
            None => (depth, parent),
        };
        let mut children = containers[c].children.clone();
        children.sort_by_key(|&child| key(child));
        stack.extend(
            children
                .into_iter()
                .rev()
                .map(|child| (child, child_depth, child_parent)),
        );
    }
    out
}

fn new_container(containers: &mut Vec<Container>) -> usize {
    containers.push(Container::default());
    containers.len() - 1
}

/// Whether `from` is `to` or one of its ancestors.
fn reaches(containers: &[Container], from: usize, to: usize) -> bool {
    let mut at = Some(to);
    while let Some(c) = at {
        if c == from {
            return true;
        }
        at = containers[c].parent;
    }
    false
}

fn link(containers: &mut [Container], parent: usize, child: usize) {
    containers[child].parent = Some(parent);
    containers[parent].children.push(child);
}

fn unlink(containers: &mut [Container], child: usize) {
    if let Some(parent) = containers[child].parent.take() {
        containers[parent].children.retain(|&c| c != child);
    }
}

fn fill_earliest(
    containers: &[Container],
    items: &[Item<'_>],
    root: usize,
    earliest: &mut [Option<i64>],
) {
    // Post-order without recursion: threads can be very deep.
    let mut stack = vec![(root, false)];
    while let Some((c, done)) = stack.pop() {
        if done {
            let own = containers[c].item.and_then(|i| items[i].date);
            earliest[c] = containers[c]
                .children
                .iter()
                .filter_map(|&child| earliest[child])
                .chain(own)
                .min();
        } else {
            stack.push((c, true));
            stack.extend(containers[c].children.iter().map(|&child| (child, false)));
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// xorshift64*: deterministic randomness without a dependency.
    pub(crate) struct Rng(pub u64);

    impl Rng {
        pub(crate) fn next(&mut self) -> u64 {
            self.0 ^= self.0 >> 12;
            self.0 ^= self.0 << 25;
            self.0 ^= self.0 >> 27;
            self.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
        }

        pub(crate) fn below(&mut self, n: usize) -> usize {
            (self.next() % n as u64) as usize
        }

        pub(crate) fn shuffle<T>(&mut self, items: &mut [T]) {
            for i in (1..items.len()).rev() {
                items.swap(i, self.below(i + 1));
            }
        }
    }

    /// A random reply forest: `parents[i]` is an earlier message or none.
    pub(crate) fn forest(rng: &mut Rng, n: usize) -> Vec<Option<usize>> {
        (0..n)
            .map(|i| (i > 0 && rng.below(4) != 0).then(|| rng.below(i)))
            .collect()
    }

    pub(crate) fn ancestors(parents: &[Option<usize>], mut i: usize) -> Vec<usize> {
        let mut chain = Vec::new();
        while let Some(p) = parents[i] {
            chain.push(p);
            i = p;
        }
        chain.reverse();
        chain
    }

    fn check_shape(nodes: &[ThreadNode], ids: &[MessageId]) {
        assert_eq!(nodes.len(), ids.len(), "every message once");
        let mut seen = std::collections::HashSet::new();
        let mut depth_of = HashMap::new();
        for node in nodes {
            assert!(seen.insert(node.message), "{:?} twice", node.message);
            match node.parent {
                None => assert_eq!(node.depth, 0),
                Some(parent) => {
                    // Parents come first, one level up.
                    assert_eq!(depth_of.get(&parent).map(|d| d + 1), Some(node.depth));
                }
            }
            depth_of.insert(node.message, node.depth);
        }
    }

    #[test]
    fn replies_hang_under_their_nearest_known_ancestor() {
        for seed in 1..300u64 {
            let mut rng = Rng(seed.wrapping_mul(0x9e37_79b9_7f4a_7c15));
            let n = 1 + rng.below(40);
            let parents = forest(&mut rng, n);
            let msgid = |i: usize| format!("m{i}@test");
            // Some messages were never downloaded.
            let present: Vec<usize> = (0..n).filter(|_| rng.below(5) != 0).collect();
            let refs: Vec<Vec<String>> = (0..n)
                .map(|i| ancestors(&parents, i).into_iter().map(msgid).collect())
                .collect();
            let ids: Vec<String> = (0..n).map(msgid).collect();
            let mut order = present.clone();
            rng.shuffle(&mut order);
            let items: Vec<Item<'_>> = order
                .iter()
                .map(|&i| Item {
                    id: MessageId(i as i64),
                    message_id: Some(&ids[i]),
                    refs: &refs[i],
                    date: Some(i as i64),
                })
                .collect();
            let nodes = tree(&items);
            let expected_ids: Vec<MessageId> =
                present.iter().map(|&i| MessageId(i as i64)).collect();
            check_shape(&nodes, &expected_ids);
            for node in &nodes {
                let i = node.message.0 as usize;
                let nearest = ancestors(&parents, i)
                    .into_iter()
                    .rev()
                    .find(|a| present.contains(a))
                    .map(|a| MessageId(a as i64));
                assert_eq!(node.parent, nearest, "seed {seed}: message {i}");
            }
        }
    }

    #[test]
    fn hostile_references_make_no_loops() {
        for seed in 1..300u64 {
            let mut rng = Rng(seed.wrapping_mul(0xd1b5_4a32_d192_ed03) | 1);
            let n = 1 + rng.below(20);
            let pool: Vec<String> = (0..n + 3).map(|i| format!("k{i}")).collect();
            let refs: Vec<Vec<String>> = (0..n)
                .map(|_| {
                    (0..rng.below(6))
                        .map(|_| pool[rng.below(pool.len())].clone())
                        .collect()
                })
                .collect();
            let own: Vec<Option<&str>> = (0..n)
                .map(|_| match rng.below(4) {
                    0 => None,
                    _ => Some(pool[rng.below(pool.len())].as_str()),
                })
                .collect();
            let items: Vec<Item<'_>> = (0..n)
                .map(|i| Item {
                    id: MessageId(i as i64),
                    message_id: own[i],
                    refs: &refs[i],
                    date: (rng.below(3) != 0).then(|| rng.below(100) as i64),
                })
                .collect();
            let nodes = tree(&items);
            check_shape(&nodes, &items.iter().map(|i| i.id).collect::<Vec<_>>());
        }
    }

    #[test]
    fn branches_are_ordered_by_date() {
        let refs_a: Vec<String> = vec![];
        let refs_b = vec!["a".to_owned()];
        let refs_c = vec!["a".to_owned()];
        let refs_d = vec!["a".to_owned(), "b".to_owned()];
        let item = |id, message_id, refs, date| Item {
            id: MessageId(id),
            message_id: Some(message_id),
            refs,
            date: Some(date),
        };
        let items = [
            item(4, "d", &refs_d, 40),
            item(3, "c", &refs_c, 20),
            item(2, "b", &refs_b, 30),
            item(1, "a", &refs_a, 10),
        ];
        let order: Vec<(i64, u32)> = tree(&items)
            .iter()
            .map(|n| (n.message.0, n.depth))
            .collect();
        // a, then c (earlier) before b's branch.
        assert_eq!(order, [(1, 0), (3, 1), (2, 1), (4, 2)]);
    }
}
