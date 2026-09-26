// SPDX-License-Identifier: GPL-3.0-or-later

//! Benchmark results as JSON, and the comparison with an earlier run that the
//! nightly job uses to catch regressions (`docs/IMPLEMENTATION_PLAN.md` §3.3).

use serde::{Deserialize, Serialize};

/// Latencies of one `search` run, in milliseconds.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Report {
    pub queries: Vec<QueryReport>,
    pub p50_ms: f64,
    pub p95_ms: f64,
    pub p99_ms: f64,
}

/// Latencies of one query.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct QueryReport {
    pub query: String,
    pub matches: usize,
    pub p50_ms: f64,
    pub p99_ms: f64,
}

/// Slowdowns smaller than this are noise, whatever the percentage.
pub const NOISE_FLOOR_MS: f64 = 1.0;

/// Compares `current` with `baseline`. Returns the regressions that fail the
/// run (overall p50 or p99 slower by more than `max_percent` and by more than
/// [`NOISE_FLOOR_MS`]) and notes about single queries, which vary too much
/// between runs to fail on.
pub fn compare(
    current: &Report,
    baseline: &Report,
    max_percent: f64,
) -> (Vec<String>, Vec<String>) {
    let slower = |now: f64, before: f64| {
        now > before * (1.0 + max_percent / 100.0) && now - before > NOISE_FLOOR_MS
    };
    let same_queries = current.queries.len() == baseline.queries.len()
        && current
            .queries
            .iter()
            .zip(&baseline.queries)
            .all(|(a, b)| a.query == b.query);
    let mut failures = Vec::new();
    let mut notes = Vec::new();
    if !same_queries {
        notes.push("the query set changed; not compared with the baseline".to_owned());
        return (failures, notes);
    }
    for (name, now, before) in [
        ("p50", current.p50_ms, baseline.p50_ms),
        ("p99", current.p99_ms, baseline.p99_ms),
    ] {
        if slower(now, before) {
            failures.push(format!(
                "overall {name} {now:.1} ms, was {before:.1} ms ({:+.0} %)",
                change(now, before)
            ));
        }
    }
    for (now, before) in current.queries.iter().zip(&baseline.queries) {
        if slower(now.p50_ms, before.p50_ms) {
            notes.push(format!(
                "{}: p50 {:.1} ms, was {:.1} ms ({:+.0} %)",
                now.query,
                now.p50_ms,
                before.p50_ms,
                change(now.p50_ms, before.p50_ms)
            ));
        }
    }
    (failures, notes)
}

fn change(now: f64, before: f64) -> f64 {
    if before > 0.0 {
        (now / before - 1.0) * 100.0
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn report(p50: f64, p99: f64, per_query: &[(&str, f64)]) -> Report {
        Report {
            queries: per_query
                .iter()
                .map(|(query, p50)| QueryReport {
                    query: (*query).to_owned(),
                    matches: 1,
                    p50_ms: *p50,
                    p99_ms: *p50,
                })
                .collect(),
            p50_ms: p50,
            p95_ms: p99,
            p99_ms: p99,
        }
    }

    #[test]
    fn flags_real_regressions_only() {
        let base = report(3.0, 10.0, &[("a", 3.0), ("b", 10.0)]);
        // 20 % slower but under the noise floor: fine.
        let (fail, notes) = compare(&report(3.6, 10.5, &[("a", 3.0), ("b", 10.0)]), &base, 10.0);
        assert!(fail.is_empty() && notes.is_empty());
        // p99 12.5 ms vs 10 ms: +25 % and +2.5 ms.
        let (fail, notes) = compare(&report(3.0, 12.5, &[("a", 3.0), ("b", 12.5)]), &base, 10.0);
        assert_eq!(fail, ["overall p99 12.5 ms, was 10.0 ms (+25 %)"]);
        assert_eq!(notes, ["b: p50 12.5 ms, was 10.0 ms (+25 %)"]);
        // Faster is never a regression.
        assert!(
            compare(&report(1.0, 5.0, &[("a", 1.0), ("b", 5.0)]), &base, 10.0)
                .0
                .is_empty()
        );
    }

    #[test]
    fn skips_a_changed_query_set() {
        let base = report(3.0, 10.0, &[("a", 3.0)]);
        let (fail, notes) = compare(&report(30.0, 100.0, &[("c", 30.0)]), &base, 10.0);
        assert!(fail.is_empty());
        assert_eq!(notes.len(), 1);
    }

    #[test]
    fn round_trips_through_json() {
        let report = report(3.0, 10.0, &[("from:ada", 3.0)]);
        let json = serde_json::to_string(&report).unwrap();
        assert_eq!(serde_json::from_str::<Report>(&json).unwrap(), report);
    }
}
