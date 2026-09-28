// SPDX-License-Identifier: GPL-3.0-or-later

//! Simple per-key limits within a fixed window, kept in memory. Any route
//! can use one: new installs per address, or per-install use of a feature.
//!
//! Each limit holds at most a fixed number of keys, so a flood of new
//! addresses cannot grow it without bound. Keys whose window is over are
//! dropped oldest first, a few at a time on each use (no scan of the whole
//! map); when every slot still holds a key within its window, new keys are
//! refused until one expires, while keys already there keep counting.

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Keys one limit holds by default.
pub const DEFAULT_CAPACITY: usize = 100_000;

/// At most `max` uses per key in each `window`.
pub struct WindowLimit<K> {
    max: u32,
    window: Duration,
    capacity: usize,
    inner: Mutex<Inner<K>>,
}

struct Inner<K> {
    /// Uses so far and when the key's window started.
    uses: HashMap<K, (u32, Instant)>,
    /// The keys by when their window started, oldest first. A key's
    /// window only starts again after it expired and was dropped, so each
    /// key is here exactly once.
    order: VecDeque<(Instant, K)>,
}

impl<K: Eq + Hash + Clone> WindowLimit<K> {
    /// A limit of `max` uses per `window`, holding up to
    /// [`DEFAULT_CAPACITY`] keys.
    pub fn new(max: u32, window: Duration) -> Self {
        Self::with_capacity(max, window, DEFAULT_CAPACITY)
    }

    /// A limit of `max` uses per `window`, holding up to `capacity` keys.
    pub fn with_capacity(max: u32, window: Duration, capacity: usize) -> Self {
        Self {
            max,
            window,
            capacity,
            inner: Mutex::new(Inner {
                uses: HashMap::new(),
                order: VecDeque::new(),
            }),
        }
    }

    /// Counts one use of `key`; returns `false` when the key is over its
    /// limit, or is new while the limit is full (and the use is not
    /// counted).
    pub fn allow(&self, key: K) -> bool {
        let now = Instant::now();
        let mut inner = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        let Inner { uses, order } = &mut *inner;
        // Only keys at the front can have expired: the queue is in the
        // order their windows started.
        while let Some((start, _)) = order.front()
            && now.duration_since(*start) >= self.window
        {
            if let Some((_, key)) = order.pop_front() {
                uses.remove(&key);
            }
        }
        if let Some((count, _)) = uses.get_mut(&key) {
            if *count >= self.max {
                return false;
            }
            *count += 1;
            return true;
        }
        if self.max == 0 || uses.len() >= self.capacity {
            return false;
        }
        uses.insert(key.clone(), (1, now));
        order.push_back((now, key));
        true
    }

    /// How many keys the limit holds now.
    pub fn len(&self) -> usize {
        self.inner
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .uses
            .len()
    }

    /// Whether the limit holds no keys.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_per_key() {
        let limit = WindowLimit::new(2, Duration::from_secs(3600));
        assert!(limit.allow("a"));
        assert!(limit.allow("a"));
        assert!(!limit.allow("a"));
        assert!(limit.allow("b"));
    }

    #[test]
    fn a_new_window_starts_over() {
        let limit = WindowLimit::new(1, Duration::ZERO);
        assert!(limit.allow(1));
        assert!(limit.allow(1));
        // Expired keys are dropped, not kept.
        assert!(limit.len() <= 1);
    }

    #[test]
    fn a_full_limit_refuses_new_keys_but_keeps_counting_old_ones() {
        let limit = WindowLimit::with_capacity(3, Duration::from_secs(3600), 100);
        for key in 0..100 {
            assert!(limit.allow(key));
        }
        assert_eq!(limit.len(), 100);
        // Full: a new key is refused, and the map does not grow.
        assert!(!limit.allow(1000));
        assert_eq!(limit.len(), 100);
        // Keys already there still count up to their limit.
        assert!(limit.allow(7));
        assert!(limit.allow(7));
        assert!(!limit.allow(7));
    }

    #[test]
    fn expired_keys_make_room() {
        let limit = WindowLimit::with_capacity(1, Duration::from_millis(50), 10);
        for key in 0..10 {
            assert!(limit.allow(key));
        }
        assert!(!limit.allow(10));
        std::thread::sleep(Duration::from_millis(60));
        assert!(limit.allow(10));
        assert_eq!(limit.len(), 1);
        // And an expired key starts over.
        assert!(limit.allow(3));
    }

    #[test]
    fn zero_allows_nothing() {
        let limit = WindowLimit::new(0, Duration::from_secs(60));
        assert!(!limit.allow("a"));
        assert!(limit.is_empty());
    }
}
