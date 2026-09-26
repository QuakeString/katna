// SPDX-License-Identifier: GPL-3.0-or-later

//! A synthetic corpus with Enron's shape, for machines without the real one.
//!
//! Numbers follow the real corpus roughly: ~150 mailbox owners, folders like
//! `lay-k/inbox`, a few thousand correspondents with a few very active ones,
//! Zipf-distributed words, 50–600-word bodies, 5 % attachments of 1–60 KB. The text is
//! nonsense made of real and made-up words, which is what the index sees
//! anyway.

use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use katna_core::Paths;
use katna_import::{Flags, IncomingMessage, MessageSink, StoreSink, parse_message};
use katna_store::{Mode, Store};

use crate::usage_error;

const BATCH: usize = 1_000;

/// Frequent words first; the synthetic vocabulary follows them.
const COMMON_WORDS: &[&str] = &[
    "the",
    "to",
    "and",
    "of",
    "a",
    "in",
    "for",
    "is",
    "on",
    "that",
    "this",
    "with",
    "be",
    "will",
    "i",
    "you",
    "we",
    "have",
    "are",
    "it",
    "at",
    "enron",
    "please",
    "from",
    "as",
    "by",
    "not",
    "or",
    "your",
    "if",
    "can",
    "our",
    "gas",
    "power",
    "energy",
    "would",
    "any",
    "all",
    "has",
    "meeting",
    "deal",
    "price",
    "market",
    "call",
    "know",
    "let",
    "thanks",
    "information",
    "new",
    "trading",
    "contract",
    "california",
    "business",
    "time",
    "company",
    "natural",
    "agreement",
    "electricity",
    "budget",
    "report",
    "attached",
    "review",
    "credit",
    "risk",
    "legal",
    "capacity",
    "pipeline",
    "transmission",
    "schedule",
    "forecast",
    "financial",
    "regulatory",
    "utility",
    "customers",
    "demand",
    "supply",
    "storage",
    "volume",
    "offer",
    "invoice",
    "project",
    "plan",
];

/// Very active senders; Enron's real addresses, which the default queries use.
const KNOWN_PEOPLE: &[&str] = &[
    "jeff.dasovich",
    "kay.mann",
    "vince.kaminski",
    "tana.jones",
    "sara.shackleton",
    "chris.germany",
    "kenneth.lay",
    "jeff.skilling",
    "tim.belden",
    "phillip.allen",
    "steven.kean",
    "richard.shapiro",
    "john.arnold",
    "louise.kitchen",
    "mark.taylor",
];

const FOLDERS: &[&str] = &[
    "inbox",
    "all_documents",
    "sent",
    "sent_items",
    "_sent_mail",
    "discussion_threads",
    "deleted_items",
    "notes_inbox",
    "calendar",
    "personal",
];

const ATTACHMENT_TYPES: &[(&str, &str)] = &[
    ("xls", "application/vnd.ms-excel"),
    ("doc", "application/msword"),
    ("pdf", "application/pdf"),
    ("ppt", "application/vnd.ms-powerpoint"),
];

pub fn run(args: &[String]) -> ExitCode {
    let mut data_dir = None;
    let mut messages: u64 = 500_000;
    let mut seed: u64 = 1;
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        let value = args.next();
        let ok = match arg.as_str() {
            "--data-dir" => {
                data_dir = value.map(PathBuf::from);
                data_dir.is_some()
            }
            "--messages" => value
                .and_then(|v| v.parse().ok())
                .map(|v| messages = v)
                .is_some(),
            "--seed" => value
                .and_then(|v| v.parse().ok())
                .map(|v| seed = v)
                .is_some(),
            _ => false,
        };
        if !ok {
            return usage_error();
        }
    }
    let Some(data_dir) = data_dir else {
        return usage_error();
    };
    match generate(&Paths::with_root(&data_dir), messages, seed) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}

fn generate(paths: &Paths, count: u64, seed: u64) -> Result<(), Box<dyn std::error::Error>> {
    let mut store = Store::open(paths, Mode::ReadWrite)?;
    let account = StoreSink::local_account(&mut store, "synthetic")?;
    let mut sink = StoreSink::new(&mut store, account.id);
    let mut corpus = Corpus::new(seed);
    let started = Instant::now();
    let mut bytes = 0u64;
    let mut done = 0u64;
    while done < count {
        let n = BATCH.min((count - done) as usize);
        let batch: Vec<IncomingMessage> = (0..n)
            .filter_map(|_| {
                let (folder, raw) = corpus.message();
                bytes += raw.len() as u64;
                let parsed = parse_message(&raw)?;
                Some(IncomingMessage {
                    folder,
                    flags: Flags::default(),
                    raw,
                    parsed,
                })
            })
            .collect();
        sink.write(&batch)?;
        done += n as u64;
        if done % 50_000 < BATCH as u64 {
            eprintln!("  {done} messages");
        }
    }
    let seconds = started.elapsed().as_secs_f64();
    println!(
        "{done} synthetic messages ({:.1} MiB) imported in {seconds:.1} s",
        bytes as f64 / (1024.0 * 1024.0)
    );
    Ok(())
}

/// Deterministic pseudo-random numbers (xorshift64*).
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n.max(1) as u64) as usize
    }

    /// True with probability `percent` / 100.
    fn chance(&mut self, percent: u64) -> bool {
        self.next() % 100 < percent
    }
}

/// Picks index `i` of `n` with probability ∝ 1 / (i + 1): Zipf's law.
struct Zipf {
    cumulative: Vec<f64>,
}

impl Zipf {
    fn new(n: usize) -> Self {
        let mut total = 0.0;
        let cumulative = (0..n)
            .map(|i| {
                total += 1.0 / (i as f64 + 1.0);
                total
            })
            .collect();
        Self { cumulative }
    }

    fn pick(&self, rng: &mut Rng) -> usize {
        let total = self.cumulative.last().copied().unwrap_or(1.0);
        let x = (rng.next() >> 11) as f64 / (1u64 << 53) as f64 * total;
        self.cumulative
            .partition_point(|&c| c < x)
            .min(self.cumulative.len() - 1)
    }
}

struct Corpus {
    rng: Rng,
    words: Vec<String>,
    word_dist: Zipf,
    people: Vec<String>,
    people_dist: Zipf,
    owners: Vec<String>,
    next_id: u64,
}

impl Corpus {
    fn new(seed: u64) -> Self {
        let mut rng = Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
        const SYLLABLES: &[&str] = &[
            "ka", "ren", "to", "mi", "sal", "ve", "dor", "an", "lu", "pre", "tra", "con", "gen",
            "ex", "ma", "ri", "ol", "sto", "ber", "in", "qu", "al", "tes", "mon",
        ];
        let mut words: Vec<String> = COMMON_WORDS.iter().map(|w| (*w).to_owned()).collect();
        while words.len() < 40_000 {
            let n = 2 + rng.below(3);
            let word: String = (0..n)
                .map(|_| SYLLABLES[rng.below(SYLLABLES.len())])
                .collect();
            words.push(word);
        }
        let mut people: Vec<String> = KNOWN_PEOPLE
            .iter()
            .map(|p| format!("{p}@enron.com"))
            .collect();
        for i in 0..6_000 {
            let first = &words[COMMON_WORDS.len() + rng.below(3_000)];
            let last = &words[COMMON_WORDS.len() + rng.below(3_000)];
            let domain = if i % 3 == 0 {
                format!("{}.com", words[COMMON_WORDS.len() + rng.below(500)])
            } else {
                "enron.com".to_owned()
            };
            people.push(format!("{first}.{last}@{domain}"));
        }
        let owners = people
            .iter()
            .filter(|p| p.ends_with("@enron.com"))
            .take(150)
            .map(|p| {
                let local = p.split('@').next().unwrap_or_default();
                let (first, last) = local.split_once('.').unwrap_or((local, "x"));
                format!("{last}-{}", &first[..1])
            })
            .collect();
        Self {
            word_dist: Zipf::new(words.len()),
            people_dist: Zipf::new(people.len()),
            rng,
            words,
            people,
            owners,
            next_id: 0,
        }
    }

    fn word(&mut self) -> &str {
        let i = self.word_dist.pick(&mut self.rng);
        &self.words[i]
    }

    fn person(&mut self) -> String {
        let i = self.people_dist.pick(&mut self.rng);
        self.people[i].clone()
    }

    fn text(&mut self, words: usize) -> String {
        let mut out = String::new();
        for i in 0..words {
            if i > 0 {
                out.push(if i % 14 == 0 { '\n' } else { ' ' });
            }
            let word = self.word().to_owned();
            out.push_str(&word);
        }
        out
    }

    /// A folder and a raw message.
    fn message(&mut self) -> (String, Vec<u8>) {
        self.next_id += 1;
        let owner = self.owners[self.rng.below(self.owners.len())].clone();
        let folder = format!("{owner}/{}", FOLDERS[self.rng.below(FOLDERS.len())]);
        let from = self.person();
        let to: Vec<String> = (0..1 + self.rng.below(4)).map(|_| self.person()).collect();
        let cc: Vec<String> = if self.rng.chance(25) {
            (0..1 + self.rng.below(3)).map(|_| self.person()).collect()
        } else {
            Vec::new()
        };
        let subject_words = 2 + self.rng.below(6);
        let mut subject = self.text(subject_words).replace('\n', " ");
        if self.rng.chance(30) {
            subject = format!("Re: {subject}");
        }
        // 1999-01-01 .. 2002-07-01
        let date = 915_148_800 + self.rng.below(110_000_000) as i64;
        let body_words = 50 + self.rng.below(550);
        let body = self.text(body_words);

        let mut raw = format!(
            "Message-ID: <{}.{}.JavaMail.evans@thyme>\r\n\
             Date: {}\r\nFrom: {from}\r\nTo: {}\r\n",
            self.next_id,
            self.rng.next() % 10_000_000,
            rfc2822(date),
            to.join(", "),
        );
        if !cc.is_empty() {
            raw.push_str(&format!("Cc: {}\r\n", cc.join(", ")));
        }
        raw.push_str(&format!("Subject: {subject}\r\nMime-Version: 1.0\r\n"));
        if self.rng.chance(5) {
            let (ext, mime) = ATTACHMENT_TYPES[self.rng.below(ATTACHMENT_TYPES.len())];
            let name = format!(
                "{}_{}.{ext}",
                self.word().to_owned(),
                self.word().to_owned()
            );
            let size = 1_000 + self.rng.below(60_000);
            let data: String = (0..size / 76)
                .map(|_| {
                    let mut line: String = (0..76)
                        .map(|_| {
                            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
                                [self.rng.below(64)] as char
                        })
                        .collect();
                    line.push_str("\r\n");
                    line
                })
                .collect();
            raw.push_str(&format!(
                "Content-Type: multipart/mixed; boundary=\"b\"\r\n\r\n\
                 --b\r\nContent-Type: text/plain; charset=us-ascii\r\n\r\n{}\r\n\
                 --b\r\nContent-Type: {mime}; name=\"{name}\"\r\n\
                 Content-Disposition: attachment; filename=\"{name}\"\r\n\
                 Content-Transfer-Encoding: base64\r\n\r\n{data}--b--\r\n",
                body.replace('\n', "\r\n"),
            ));
        } else {
            raw.push_str(&format!(
                "Content-Type: text/plain; charset=us-ascii\r\n\r\n{}\r\n",
                body.replace('\n', "\r\n")
            ));
        }
        (folder, raw.into_bytes())
    }
}

/// Unix seconds → an RFC 2822 date in UTC.
fn rfc2822(time: i64) -> String {
    const DAYS: [&str; 7] = ["Thu", "Fri", "Sat", "Sun", "Mon", "Tue", "Wed"];
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let days = time.div_euclid(86_400);
    let secs = time.rem_euclid(86_400);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!(
        "{}, {day} {} {year} {:02}:{:02}:{:02} +0000",
        DAYS[days.rem_euclid(7) as usize],
        MONTHS[(month - 1) as usize],
        secs / 3600,
        secs / 60 % 60,
        secs % 60,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dates() {
        assert_eq!(rfc2822(0), "Thu, 1 Jan 1970 00:00:00 +0000");
        assert_eq!(rfc2822(989_883_540), "Mon, 14 May 2001 23:39:00 +0000");
    }

    #[test]
    fn messages_parse_and_repeat() {
        let mut a = Corpus::new(7);
        let mut b = Corpus::new(7);
        for _ in 0..200 {
            let (folder, raw) = a.message();
            assert_eq!((folder.clone(), raw.clone()), b.message());
            let parsed = parse_message(&raw).unwrap();
            assert!(parsed.date.is_some());
            assert!(!parsed.participants.is_empty());
            assert!(folder.contains('/'));
        }
    }

    #[test]
    fn zipf_prefers_small_indexes() {
        let zipf = Zipf::new(1_000);
        let mut rng = Rng(3);
        let picks: Vec<usize> = (0..10_000).map(|_| zipf.pick(&mut rng)).collect();
        let first = picks.iter().filter(|&&i| i == 0).count();
        let tenth = picks.iter().filter(|&&i| i == 9).count();
        assert!(first > 5 * tenth, "{first} vs {tenth}");
        assert!(picks.iter().all(|&i| i < 1_000));
    }
}
