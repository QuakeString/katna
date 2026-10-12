// SPDX-License-Identifier: GPL-3.0-or-later

//! Anonymous usage statistics (`docs/ARCHITECTURE.md` §19.2, plan C.6):
//! for each week, which of a fixed list of features ([`Feature`]) were
//! used, yes or no, and a few rough facts. Recorded only while
//! "Send anonymous usage statistics" is on; the daemon sends a finished
//! week once, and turning the switch off deletes what was recorded.
//!
//! Everything lives in `$XDG_STATE_HOME/katna/usage/`: one file per week
//! (`week-N`, lines `feature` or `fact=value`), `install-id` and `sent`
//! (the last week sent).

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::Paths;

/// How long one install ID is used before a new one is made.
pub const ROTATE_AFTER: Duration = Duration::from_secs(90 * 24 * 60 * 60);

const WEEK_SECS: i64 = 7 * 24 * 60 * 60;

/// Every feature that is counted. Settings > User feedback lists them
/// all, with the text of [`Feature::label_id`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Feature {
    SearchOptions,
    Pins,
    Labels,
    ScheduledSend,
    Snooze,
    Encrypted,
    Viewers,
    Calendar,
    Contacts,
    TasksNotes,
    PhoneLayout,
    OwnFrame,
}

impl Feature {
    pub const ALL: [Feature; 12] = [
        Feature::SearchOptions,
        Feature::Pins,
        Feature::Labels,
        Feature::ScheduledSend,
        Feature::Snooze,
        Feature::Encrypted,
        Feature::Viewers,
        Feature::Calendar,
        Feature::Contacts,
        Feature::TasksNotes,
        Feature::PhoneLayout,
        Feature::OwnFrame,
    ];

    /// The name sent, and written in the week's file.
    pub fn key(self) -> &'static str {
        match self {
            Feature::SearchOptions => "search_options",
            Feature::Pins => "pins",
            Feature::Labels => "labels",
            Feature::ScheduledSend => "scheduled_send",
            Feature::Snooze => "snooze",
            Feature::Encrypted => "encrypted",
            Feature::Viewers => "viewers",
            Feature::Calendar => "calendar",
            Feature::Contacts => "contacts",
            Feature::TasksNotes => "tasks_notes",
            Feature::PhoneLayout => "phone_layout",
            Feature::OwnFrame => "own_frame",
        }
    }

    /// The message id of its name in Settings (`i18n/en/katna-mail/feedback.ftl`).
    pub fn label_id(self) -> &'static str {
        match self {
            Feature::SearchOptions => "usage-feature-search-options",
            Feature::Pins => "usage-feature-pins",
            Feature::Labels => "usage-feature-labels",
            Feature::ScheduledSend => "usage-feature-scheduled-send",
            Feature::Snooze => "usage-feature-snooze",
            Feature::Encrypted => "usage-feature-encrypted",
            Feature::Viewers => "usage-feature-viewers",
            Feature::Calendar => "usage-feature-calendar",
            Feature::Contacts => "usage-feature-contacts",
            Feature::TasksNotes => "usage-feature-tasks-notes",
            Feature::PhoneLayout => "usage-feature-phone-layout",
            Feature::OwnFrame => "usage-feature-own-frame",
        }
    }

    fn from_key(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|f| f.key() == key)
    }
}

/// Facts recorded by Katna Mail, which the daemon cannot see.
pub mod fact {
    /// The screen scale, bucketed ([`super::scale_bucket`]).
    pub const SCALE: &str = "scale";
    /// `XDG_CURRENT_DESKTOP`, as Katna Mail sees it.
    pub const DESKTOP: &str = "desktop";
    /// `XDG_SESSION_TYPE`: `wayland` or `x11`.
    pub const SESSION: &str = "session";
}

/// Weeks since 1970, starting on Mondays.
pub fn week(now: SystemTime) -> i64 {
    let secs = now
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    // 1 January 1970 was a Thursday.
    (secs + 3 * 24 * 60 * 60).div_euclid(WEEK_SECS)
}

/// When week `week` ends (the Monday after it, 00:00 UTC).
pub fn week_end(week: i64) -> SystemTime {
    let secs = (week + 1) * WEEK_SECS - 3 * 24 * 60 * 60;
    UNIX_EPOCH + Duration::from_secs(secs.max(0) as u64)
}

/// `1–1.5` and so on, for a scale factor.
pub fn scale_bucket(scale: f32) -> &'static str {
    if scale < 1.25 {
        "1"
    } else if scale < 1.75 {
        "1.5"
    } else if scale < 2.5 {
        "2"
    } else {
        "3+"
    }
}

/// `1`, `2-3` or `4+`.
pub fn accounts_bucket(accounts: usize) -> &'static str {
    match accounts {
        0 => "0",
        1 => "1",
        2 | 3 => "2-3",
        _ => "4+",
    }
}

fn dir(paths: &Paths) -> PathBuf {
    paths.state_dir().join("usage")
}

fn week_file(paths: &Paths, week: i64) -> PathBuf {
    dir(paths).join(format!("week-{week}"))
}

fn add_line(paths: &Paths, line: &str, now: SystemTime) -> io::Result<()> {
    let path = week_file(paths, week(now));
    let known = fs::read_to_string(&path).unwrap_or_default();
    if known.lines().any(|l| l == line) {
        return Ok(());
    }
    fs::create_dir_all(dir(paths))?;
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;
    writeln!(file, "{line}")
}

/// Notes that `feature` was used this week. The caller checks that usage
/// statistics are on.
pub fn record(paths: &Paths, feature: Feature, now: SystemTime) {
    if let Err(err) = add_line(paths, feature.key(), now) {
        tracing::debug!(%err, "could not record a used feature");
    }
}

/// Notes a fact ([`fact`]) for this week; the first value of the week wins.
pub fn record_fact(paths: &Paths, key: &str, value: &str, now: SystemTime) {
    let value: String = value
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || "-.+:_ ".contains(*c))
        .take(40)
        .collect();
    if value.is_empty() || recorded(paths, week(now)).1.contains_key(key) {
        return;
    }
    if let Err(err) = add_line(paths, &format!("{key}={value}"), now) {
        tracing::debug!(%err, "could not record a usage fact");
    }
}

/// The features used and the facts recorded in `week`.
pub fn recorded(paths: &Paths, week: i64) -> (BTreeSet<Feature>, BTreeMap<String, String>) {
    let text = fs::read_to_string(week_file(paths, week)).unwrap_or_default();
    let mut used = BTreeSet::new();
    let mut facts = BTreeMap::new();
    for line in text.lines() {
        match line.split_once('=') {
            Some((key, value)) => {
                facts
                    .entry(key.to_owned())
                    .or_insert_with(|| value.to_owned());
            }
            None => {
                if let Some(feature) = Feature::from_key(line) {
                    used.insert(feature);
                }
            }
        }
    }
    (used, facts)
}

/// Deletes everything recorded and the install ID, as when usage
/// statistics are turned off.
pub fn forget(paths: &Paths) {
    let _ = fs::remove_dir_all(dir(paths));
}

/// The last week sent, if any.
pub fn last_sent(paths: &Paths) -> Option<i64> {
    fs::read_to_string(dir(paths).join("sent"))
        .ok()?
        .trim()
        .parse()
        .ok()
}

/// Remembers that `week` was sent and deletes the files of that week and
/// older ones.
pub fn mark_sent(paths: &Paths, week: i64) -> io::Result<()> {
    fs::create_dir_all(dir(paths))?;
    fs::write(dir(paths).join("sent"), format!("{week}\n"))?;
    if let Ok(entries) = fs::read_dir(dir(paths)) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let old = name
                .to_str()
                .and_then(|n| n.strip_prefix("week-"))
                .and_then(|n| n.parse::<i64>().ok())
                .is_some_and(|w| w <= week);
            if old {
                let _ = fs::remove_file(entry.path());
            }
        }
    }
    Ok(())
}

/// The finished week to send now, if one is waiting: the last week before
/// `now` with something recorded and not sent yet.
pub fn due(paths: &Paths, now: SystemTime) -> Option<i64> {
    let this = week(now);
    let sent = last_sent(paths).unwrap_or(i64::MIN);
    let entries = fs::read_dir(dir(paths)).ok()?;
    entries
        .flatten()
        .filter_map(|e| {
            e.file_name()
                .to_str()?
                .strip_prefix("week-")?
                .parse::<i64>()
                .ok()
        })
        .filter(|&w| w < this && w > sent)
        .max()
}

/// The install ID: 32 random hex digits, and when it changes. A new one
/// is made when there is none or it is [`ROTATE_AFTER`] old.
pub fn install_id(paths: &Paths, now: SystemTime) -> (String, SystemTime) {
    let path = dir(paths).join("install-id");
    let saved = fs::read_to_string(&path).ok().and_then(|text| {
        let (id, made) = text.trim().split_once(' ')?;
        let made = UNIX_EPOCH + Duration::from_secs(made.parse().ok()?);
        (id.len() == 32 && id.chars().all(|c| c.is_ascii_hexdigit())).then(|| (id.to_owned(), made))
    });
    if let Some((id, made)) = saved
        && now.duration_since(made).is_ok_and(|age| age < ROTATE_AFTER)
    {
        return (id, made + ROTATE_AFTER);
    }
    new_install_id(paths, now)
}

/// Makes a new install ID at once ("Reset" in Settings).
pub fn new_install_id(paths: &Paths, now: SystemTime) -> (String, SystemTime) {
    let id = random_hex();
    let made = now
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let path = dir(paths).join("install-id");
    let written =
        fs::create_dir_all(dir(paths)).and_then(|()| fs::write(&path, format!("{id} {made}\n")));
    if let Err(err) = written {
        tracing::debug!(%err, "could not save the install ID");
    }
    (id, now + ROTATE_AFTER)
}

fn random_hex() -> String {
    let mut bytes = [0u8; 16];
    let read = fs::File::open("/dev/urandom").and_then(|mut f| f.read_exact(&mut bytes));
    if read.is_err() {
        // Windows: the standard library's random hash keys.
        use std::hash::{BuildHasher, Hasher};
        for (i, chunk) in bytes.chunks_mut(8).enumerate() {
            let mut hasher = std::collections::hash_map::RandomState::new().build_hasher();
            hasher.write_usize(i);
            hasher.write_u128(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_nanos())
                    .unwrap_or(0),
            );
            chunk.copy_from_slice(&hasher.finish().to_le_bytes());
        }
    }
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// The Linux family from `/etc/os-release` (`arch`, `ubuntu`), or
/// `windows`.
pub fn os_family() -> String {
    if cfg!(windows) {
        return "windows".into();
    }
    let text = fs::read_to_string("/etc/os-release").unwrap_or_default();
    let field = |name: &str| {
        text.lines()
            .find_map(|line| Some(line.strip_prefix(name)?.trim_matches('"').to_owned()))
    };
    field("ID=").unwrap_or_else(|| "linux".into())
}

/// One week's statistics, as sent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeeklyReport {
    pub week: i64,
    pub version: String,
    pub os: String,
    pub desktop: String,
    pub session: String,
    pub scale: String,
    pub accounts: String,
    pub used: BTreeSet<Feature>,
    pub install_id: String,
}

impl WeeklyReport {
    /// The report for `week` from what was recorded, with `accounts`
    /// accounts set up.
    pub fn build(paths: &Paths, week: i64, accounts: usize, install_id: String) -> Self {
        let (used, facts) = recorded(paths, week);
        let fact = |key: &str| facts.get(key).cloned().unwrap_or_default();
        Self {
            week,
            version: crate::crash::VERSION.to_owned(),
            os: os_family(),
            desktop: fact(fact::DESKTOP),
            session: fact(fact::SESSION),
            scale: fact(fact::SCALE),
            accounts: accounts_bucket(accounts).to_owned(),
            used,
            install_id,
        }
    }

    /// The report as text: what Settings shows and what is sent.
    pub fn text(&self) -> String {
        let names = |used: bool| {
            let list: Vec<&str> = Feature::ALL
                .into_iter()
                .filter(|f| self.used.contains(f) == used)
                .map(Feature::key)
                .collect();
            if list.is_empty() {
                "-".to_owned()
            } else {
                list.join(", ")
            }
        };
        let or_dash = |s: &str| {
            if s.is_empty() {
                "-".to_owned()
            } else {
                s.to_owned()
            }
        };
        format!(
            "version     {}\nsystem      {}\ndesktop     {}\nsession     {}\nscale       {}\naccounts    {}\nused        {}\nnot used    {}\ninstall ID  {}\n",
            self.version,
            self.os,
            or_dash(&self.desktop),
            or_dash(&self.session),
            or_dash(&self.scale),
            self.accounts,
            names(true),
            names(false),
            self.install_id,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn paths(dir: &tempfile::TempDir) -> Paths {
        Paths::with_root(dir.path())
    }

    #[test]
    fn weeks_start_on_monday() {
        // Monday 5 October 2026 00:00 UTC and the Sunday before it.
        let monday = UNIX_EPOCH + Duration::from_secs(1_791_158_400);
        let sunday = monday - Duration::from_secs(1);
        assert_eq!(week(monday), week(sunday) + 1);
        assert_eq!(week_end(week(sunday)), monday);
    }

    #[test]
    fn records_features_and_facts_once() {
        let dir = tempfile::tempdir().unwrap();
        let paths = paths(&dir);
        let now = SystemTime::now();
        record(&paths, Feature::Labels, now);
        record(&paths, Feature::Labels, now);
        record_fact(&paths, fact::SCALE, "1.5", now);
        record_fact(&paths, fact::SCALE, "2", now);
        let (used, facts) = recorded(&paths, week(now));
        assert_eq!(used.into_iter().collect::<Vec<_>>(), vec![Feature::Labels]);
        assert_eq!(facts.get(fact::SCALE).map(String::as_str), Some("1.5"));
        let text = fs::read_to_string(week_file(&paths, week(now))).unwrap();
        assert_eq!(text, "labels\nscale=1.5\n");
    }

    #[test]
    fn sends_a_finished_week_once() {
        let dir = tempfile::tempdir().unwrap();
        let paths = paths(&dir);
        let last_week = SystemTime::now() - Duration::from_secs(WEEK_SECS as u64);
        record(&paths, Feature::Snooze, last_week);
        record(&paths, Feature::Pins, SystemTime::now());
        let due_week = due(&paths, SystemTime::now()).unwrap();
        assert_eq!(due_week, week(last_week));
        mark_sent(&paths, due_week).unwrap();
        assert_eq!(due(&paths, SystemTime::now()), None);
        // This week's file stays.
        assert!(
            recorded(&paths, week(SystemTime::now()))
                .0
                .contains(&Feature::Pins)
        );
    }

    #[test]
    fn install_id_rotates() {
        let dir = tempfile::tempdir().unwrap();
        let paths = paths(&dir);
        let now = SystemTime::now();
        let (id, until) = install_id(&paths, now);
        assert_eq!(id.len(), 32);
        assert_eq!(install_id(&paths, now).0, id);
        assert_ne!(install_id(&paths, until).0, id);
        assert_ne!(new_install_id(&paths, now).0, id);
    }

    #[test]
    fn report_text_lists_every_feature() {
        let report = WeeklyReport {
            week: 2961,
            version: "0.0.0".into(),
            os: "arch".into(),
            desktop: "KDE".into(),
            session: String::new(),
            scale: "1".into(),
            accounts: "2-3".into(),
            used: [Feature::Labels, Feature::Calendar].into(),
            install_id: "ab".repeat(16),
        };
        let text = report.text();
        assert!(text.contains("used        labels, calendar\n"), "{text}");
        assert!(text.contains("session     -\n"));
        for feature in Feature::ALL {
            assert!(text.contains(feature.key()), "{}", feature.key());
        }
    }

    #[test]
    fn buckets() {
        assert_eq!(scale_bucket(1.0), "1");
        assert_eq!(scale_bucket(1.5), "1.5");
        assert_eq!(scale_bucket(2.0), "2");
        assert_eq!(accounts_bucket(3), "2-3");
        assert_eq!(accounts_bucket(9), "4+");
    }
}
