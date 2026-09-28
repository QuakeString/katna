// SPDX-License-Identifier: GPL-3.0-or-later

//! Simple per-key limits within a fixed window, kept in memory. Any route
//! can use one: new installs per address, or per-install use of a feature.

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// At most `max` uses per key in each `window`.
pub struct WindowLimit<K> {
    max: u32,
    window: Duration,
    uses: Mutex<Uses<K>>,
}

struct Uses<K> {
    by_key: HashMap<K, (u32, Instant)>,
    /// When keys whose window ended were last dropped.
    swept: Instant,
}

impl<K> Default for Uses<K> {
    fn default() -> Self {
        Self {
            by_key: HashMap::new(),
            swept: Instant::now(),
        }
    }
}

/// Keys kept before ended windows are dropped.
const SWEEP_ABOVE: usize = 10_000;

/// At most one sweep this often, so a map full of live keys is not
/// scanned on every use.
const SWEEP_EVERY: Duration = Duration::from_secs(10);

impl<K: Eq + Hash> WindowLimit<K> {
    /// A limit of `max` uses per `window`.
    pub fn new(max: u32, window: Duration) -> Self {
        Self {
            max,
            window,
            uses: Mutex::default(),
        }
    }

    /// Counts one use of `key`; returns `false` when the key is over its
    /// limit (and the use is not counted).
    pub fn allow(&self, key: K) -> bool {
        let now = Instant::now();
        let mut uses = self.uses.lock().unwrap_or_else(|e| e.into_inner());
        if uses.by_key.len() > SWEEP_ABOVE && now.duration_since(uses.swept) >= SWEEP_EVERY {
            uses.by_key
                .retain(|_, (_, start)| now.duration_since(*start) < self.window);
            uses.swept = now;
        }
        let (count, start) = uses.by_key.entry(key).or_insert((0, now));
        if now.duration_since(*start) >= self.window {
            *count = 0;
            *start = now;
        }
        if *count >= self.max {
            return false;
        }
        *count += 1;
        true
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
    }
}
