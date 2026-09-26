// SPDX-License-Identifier: GPL-3.0-or-later

//! End-to-end: store → index → queries.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};
use std::time::{Duration, Instant};

use katna_core::Paths;
use katna_import::{Flags, IncomingMessage, MessageSink, StoreSink, parse_message};
use katna_search::{
    IndexEvent, IndexOptions, Indexer, IndexerOptions, Query, SearchIndex, SearchOptions, Sort,
};
use katna_store::{MessageId, Mode, Store};

struct Mail {
    folder: &'static str,
    flags: Flags,
    raw: String,
}

fn mail(folder: &'static str, headers: &str, body: &str) -> Mail {
    Mail {
        folder,
        flags: Flags::default(),
        raw: format!("{headers}\r\n\r\n{body}\r\n")
            .replace('\n', "\r\n")
            .replace("\r\r", "\r"),
    }
}

fn corpus() -> Vec<Mail> {
    let mut seen = mail(
        "lay-k/inbox",
        "From: Kenneth Lay <kenneth.lay@enron.com>\nTo: all.worldwide@enron.com\n\
         Subject: 2001 budget\nDate: Mon, 14 May 2001 16:39:00 -0700\n\
         Content-Type: multipart/mixed; boundary=\"b\"",
        "--b\nContent-Type: text/plain\n\nThe budget for natural gas trading is attached.\n\
         --b\nContent-Type: application/vnd.ms-excel\n\
         Content-Disposition: attachment; filename=\"budget2001.xls\"\n\nxx\n--b--",
    );
    seen.flags.seen = true;
    let mut starred = mail(
        "skilling-j/inbox",
        "From: jeff.skilling@enron.com\nTo: kenneth.lay@enron.com\nCc: tim.belden@enron.com\n\
         Subject: California power\nDate: Tue, 15 May 2001 09:00:00 -0700",
        "Power prices in California keep rising. The budget will not hold.",
    );
    starred.flags.flagged = true;
    vec![
        seen,
        starred,
        mail(
            "belden-t/sent",
            "From: tim.belden@enron.com\nTo: phillip.allen@enron.com\n\
             Subject: Re: gas\nDate: Wed, 01 Nov 2000 10:00:00 -0800",
            "Gas natural. Not a phrase match for the other one.",
        ),
        mail(
            "allen-p/inbox",
            "From: \"Caf\u{e9} Owner\" <owner@cafe.example.org>\nTo: phillip.allen@enron.com\n\
             Subject: Lunch\nList-Id: Lunch club <lunch.example.org>\n\
             Date: Fri, 01 Jun 2001 12:00:00 +0000",
            &format!("Menu for today.\n{}", "filler text ".repeat(200)),
        ),
    ]
}

fn import(store: &mut Store, mails: &[Mail]) {
    let account = StoreSink::local_account(store, "enron").unwrap();
    let batch: Vec<IncomingMessage> = mails
        .iter()
        .map(|m| IncomingMessage {
            folder: m.folder.to_owned(),
            flags: m.flags,
            raw: m.raw.as_bytes().to_vec(),
            parsed: parse_message(m.raw.as_bytes()).unwrap(),
        })
        .collect();
    StoreSink::new(store, account.id).write(&batch).unwrap();
}

fn small_options() -> IndexOptions {
    IndexOptions {
        parse_threads: 2,
        writer_threads: 1,
        memory_budget: 20 << 20,
        commit_every: 2,
        ..IndexOptions::default()
    }
}

fn subjects(index: &SearchIndex, store: &Store, query: &str) -> Vec<String> {
    found(index, store, Query::parse_at(query, 1_000_000_000).unwrap())
}

fn found(index: &SearchIndex, store: &Store, query: Query) -> Vec<String> {
    let options = SearchOptions {
        limit: 10,
        ..SearchOptions::default()
    };
    let hits = index.search(&query, &options).unwrap().hits;
    let ids: Vec<MessageId> = hits.iter().map(|h| h.message).collect();
    store
        .messages_by_id(&ids)
        .unwrap()
        .into_iter()
        .map(|m| m.subject)
        .collect()
}

#[test]
fn indexes_the_store_and_answers_queries() {
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths::with_root(tmp.path());
    let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
    import(&mut store, &corpus());

    let index = SearchIndex::open(&paths.index_dir()).unwrap();
    let stats = index.update(&store, &small_options(), |_| {}).unwrap();
    assert_eq!(stats.indexed, 4);
    assert_eq!(stats.without_text, 0);
    assert_eq!(index.num_docs(), 4);
    let state = index.state().unwrap();
    assert!(state.change_seq.is_some());
    assert_eq!(state.scan_after, None);

    let reader = Store::open(&paths, Mode::ReadOnly).unwrap();
    let app = SearchIndex::open_read_only(&paths.index_dir()).unwrap();
    let q = |query: &str| subjects(&app, &reader, query);

    assert_eq!(q("from:kenneth.lay has:attachment budget"), ["2001 budget"]);
    assert_eq!(q("from:\"kenneth lay\""), ["2001 budget"]);
    // Relevance: the subject match ranks first.
    assert_eq!(q("budget"), ["2001 budget", "California power"]);
    assert_eq!(q("\"natural gas\""), ["2001 budget"]);
    assert_eq!(q("gas -subject:re"), ["2001 budget"]);
    assert_eq!(q("to:tim.belden"), ["California power"]);
    assert_eq!(q("cc:tim.belden"), ["California power"]);
    assert_eq!(q("filename:xls"), ["2001 budget"]);
    assert_eq!(q("budget2001"), ["2001 budget"]);
    assert_eq!(q("list:lunch.example.org"), ["Lunch"]);
    assert_eq!(q("cafe"), ["Lunch"]);
    assert_eq!(q("org:example.org"), ["Lunch"]);
    assert_eq!(q("org:enron.com in:sent"), ["Re: gas"]);
    assert_eq!(q("in:skilling-j"), ["California power"]);
    assert_eq!(q("is:starred"), ["California power"]);
    // Newest first when there is no free text.
    assert_eq!(q("is:unread"), ["Lunch", "California power", "Re: gas"]);
    assert_eq!(
        q("after:2001-05-15 before:2001-06-01"),
        ["California power"]
    );
    assert_eq!(q("larger:2K"), ["Lunch"]);
    assert_eq!(q("(power OR lunch) -california"), ["Lunch"]);
    assert_eq!(q("-from:enron.com"), ["Lunch"]);
    assert!(q("nothing-matches-this").is_empty());
    // Other forms of a word, in the subject and body; phrases stay exact.
    assert_eq!(q("price"), ["California power"]);
    assert_eq!(q("rises"), ["California power"]);
    assert_eq!(q("trade"), ["2001 budget"]);
    assert_eq!(q("subject:budgets"), ["2001 budget"]);
    assert!(q("\"natural gases\"").is_empty());
    assert!(q("from:skill").is_empty());
    assert_eq!(q("").len(), 4);

    // As you type: the last word is a prefix.
    let typing = |query: &str| {
        found(
            &app,
            &reader,
            Query::parse_as_you_type(query, 1_000_000_000).unwrap(),
        )
    };
    assert_eq!(typing("budg"), ["2001 budget", "California power"]);
    assert_eq!(typing("from:kenn"), ["2001 budget"]);
    assert_eq!(typing("natural ga").len(), 2);
    // An unfinished phrase keeps its word order.
    assert_eq!(typing("\"natural ga"), ["2001 budget"]);
    assert!(typing("-from:jeff califo").is_empty());
    assert_eq!(typing("-from:kenneth califo"), ["California power"]);
    assert!(typing("qzxwv").is_empty());

    let everything = Query::parse("").unwrap();
    let oldest = app
        .search(
            &everything,
            &SearchOptions {
                sort: Sort::Oldest,
                limit: 1,
                count: true,
                ..SearchOptions::default()
            },
        )
        .unwrap();
    assert_eq!(oldest.total, Some(4));
    assert_eq!(oldest.hits.len(), 1);
    assert_eq!(oldest.hits[0].date, Some(973_101_600));

    let query = Query::parse("natural gas budget -subject:trading").unwrap();
    let hit = app.search(&query, &SearchOptions::default()).unwrap().hits[0];
    let snippets = app.snippets(&reader, &query, &[hit.message], 80).unwrap();
    let snippet = &snippets[0];
    let marked: Vec<&str> = snippet
        .highlights
        .iter()
        .map(|r| &snippet.text[r.clone()])
        .collect();
    assert!(marked.contains(&"budget"), "{snippet:?}");
    assert!(marked.contains(&"natural"), "{snippet:?}");
    assert!(!marked.contains(&"trading"), "{snippet:?}");

    // Other forms of the searched word are highlighted.
    let query = Query::parse("price rise").unwrap();
    let hit = app.search(&query, &SearchOptions::default()).unwrap().hits[0];
    let snippet = &app.snippets(&reader, &query, &[hit.message], 80).unwrap()[0];
    let marked: Vec<&str> = snippet
        .highlights
        .iter()
        .map(|r| &snippet.text[r.clone()])
        .collect();
    assert_eq!(marked, ["prices", "rising"], "{snippet:?}");

    // No match in the body: the snippet is the start of the body.
    let query = Query::parse("from:owner").unwrap();
    let hit = app.search(&query, &SearchOptions::default()).unwrap().hits[0];
    let snippet = &app.snippets(&reader, &query, &[hit.message], 30).unwrap()[0];
    assert_eq!(snippet.text, "Menu for today. filler text");
    assert!(snippet.highlights.is_empty());
}

#[test]
fn follows_the_change_journal_and_rebuilds() {
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths::with_root(tmp.path());
    let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
    let mails = corpus();
    import(&mut store, &mails[..2]);

    let index = SearchIndex::open(&paths.index_dir()).unwrap();
    assert_eq!(
        index
            .update(&store, &small_options(), |_| {})
            .unwrap()
            .indexed,
        2
    );
    // Nothing new: nothing to do.
    assert_eq!(
        index.update(&store, &small_options(), |_| {}).unwrap(),
        Default::default()
    );

    // Two new messages and a copy of an indexed one in another folder.
    let mut more: Vec<Mail> = mails.into_iter().skip(2).collect();
    let copy = corpus().remove(0);
    more.push(Mail {
        folder: "lay-k/archive",
        ..copy
    });
    import(&mut store, &more);
    let stats = index.update(&store, &small_options(), |_| {}).unwrap();
    assert_eq!(stats.indexed, 3);
    assert_eq!(index.num_docs(), 4);
    assert_eq!(subjects(&index, &store, "in:archive"), ["2001 budget"]);
    assert_eq!(subjects(&index, &store, "in:inbox budget").len(), 2);

    // Delete and rebuild from the store.
    drop(index);
    SearchIndex::delete(&paths.index_dir()).unwrap();
    assert!(SearchIndex::open_read_only(&paths.index_dir()).is_err());
    let index = SearchIndex::open(&paths.index_dir()).unwrap();
    assert_eq!(
        index
            .update(&store, &small_options(), |_| {})
            .unwrap()
            .indexed,
        4
    );
    assert_eq!(subjects(&index, &store, "in:archive"), ["2001 budget"]);
}

#[test]
fn stops_early_and_carries_on() {
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths::with_root(tmp.path());
    let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
    import(&mut store, &corpus());

    let index = SearchIndex::open(&paths.index_dir()).unwrap();
    let stop = Arc::new(AtomicBool::new(true));
    let stopping = IndexOptions {
        stop: Some(stop.clone()),
        ..small_options()
    };
    assert_eq!(index.update(&store, &stopping, |_| {}).unwrap().indexed, 0);
    assert_eq!(index.state().unwrap().change_seq, None);

    stop.store(false, Ordering::Relaxed);
    assert_eq!(index.update(&store, &stopping, |_| {}).unwrap().indexed, 4);
    assert!(index.state().unwrap().change_seq.is_some());
}

#[test]
fn indexer_follows_the_store_and_apps_see_it() {
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths::with_root(tmp.path());
    let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
    let mails = corpus();
    import(&mut store, &mails[..2]);

    let (events, received) = mpsc::channel();
    let indexer = Indexer::start(
        &paths,
        IndexerOptions {
            index: small_options(),
            poll_every: Duration::from_secs(60),
        },
        move |event| {
            let _ = events.send(event);
        },
    )
    .unwrap();
    let updated = |expected: u64| loop {
        match received.recv_timeout(Duration::from_secs(20)).unwrap() {
            IndexEvent::Updated(stats) => {
                assert_eq!(stats.indexed, expected);
                break;
            }
            IndexEvent::Progress { .. } => {}
            IndexEvent::Failed(err) => panic!("{err}"),
        }
    };
    updated(2);

    let app = SearchIndex::open_read_only(&paths.index_dir()).unwrap();
    assert_eq!(app.num_docs(), 2);

    import(&mut store, &mails[2..]);
    indexer.changed();
    updated(2);
    // The app's index reloads by itself.
    let started = Instant::now();
    while app.num_docs() < 4 {
        assert!(started.elapsed() < Duration::from_secs(20), "no reload");
        std::thread::sleep(Duration::from_millis(50));
    }
    assert_eq!(subjects(&app, &store, "cafe"), ["Lunch"]);

    // Nothing changed: no update.
    indexer.changed();
    indexer.stop();
    assert!(
        received
            .try_iter()
            .all(|event| !matches!(event, IndexEvent::Updated(_)))
    );
}

/// Mail from Hasina Banu among people whose names start with b, as in a
/// real mailbox, where more than a handful of words start with each letter.
fn banu_corpus() -> Vec<Mail> {
    let mut mails = vec![
        mail(
            "inbox",
            "From: Hasina Banu <hasina.banu@example.com>\nTo: me@example.com\n\
             Subject: School fees\nDate: Mon, 14 May 2001 16:39:00 +0000",
            "Please pay the school fees by Friday.",
        ),
        mail(
            "inbox",
            "From: Hasina Banu <hasina.banu@example.com>\nTo: me@example.com\n\
             Subject: Dinner\nDate: Tue, 15 May 2001 16:39:00 +0000",
            "Dinner at eight.",
        ),
        mail(
            "inbox",
            "From: Hasina Rahman <hasina@example.net>\nTo: me@example.com\n\
             Subject: Hello\nDate: Wed, 16 May 2001 16:39:00 +0000",
            "Hello from Rahman.",
        ),
        mail(
            "inbox",
            "From: Kerston Lee <kerston@example.net>\nTo: me@example.com\n\
             Subject: Tea\nDate: Wed, 16 May 2001 17:39:00 +0000",
            "Tea at four.",
        ),
    ];
    // Twenty-six names that sort before "banu": baaa, baab, …, baaz.
    for letter in 'a'..='z' {
        mails.push(Mail {
            folder: "inbox",
            flags: Flags::default(),
            raw: format!(
                "From: Baa{letter} Person <baa{letter}@example.org>\r\nTo: me@example.com\r\n\
                 Subject: Note {letter}\r\nDate: Thu, 17 May 2001 16:39:00 +0000\r\n\r\n\
                 bab{letter} bad{letter} bag{letter}\r\n"
            ),
        });
    }
    mails
}

#[test]
fn forgives_typos_and_short_prefixes() {
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths::with_root(tmp.path());
    let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
    import(&mut store, &banu_corpus());
    let index = SearchIndex::open(&paths.index_dir()).unwrap();
    index.update(&store, &small_options(), |_| {}).unwrap();
    let typing = |query: &str| {
        let query = Query::parse_as_you_type(query, 1_000_000_000).unwrap();
        let options = SearchOptions {
            limit: 50,
            ..SearchOptions::default()
        };
        let ids: Vec<MessageId> = index
            .search(&query, &options)
            .unwrap()
            .hits
            .iter()
            .map(|h| h.message)
            .collect();
        let mut found: Vec<String> = store
            .messages_by_id(&ids)
            .unwrap()
            .into_iter()
            .map(|m| m.subject)
            .collect();
        found.sort();
        found
    };

    // A short last word finds every longer word, not only the first few.
    assert_eq!(typing("hasina b"), ["Dinner", "School fees"]);
    assert_eq!(typing("hasina ba"), ["Dinner", "School fees"]);
    assert_eq!(typing("b").len(), 28);
    // A misspelled name still finds the person.
    assert_eq!(typing("haskina banu"), ["Dinner", "School fees"]);
    assert_eq!(typing("hasina bano"), ["Dinner", "School fees"]);
    assert_eq!(typing("hasna banu"), ["Dinner", "School fees"]);
    assert_eq!(typing("from:hasnia"), ["Dinner", "Hello", "School fees"]);
    // Misspelled words anywhere, when nothing matches exactly.
    assert_eq!(typing("scool fees"), ["School fees"]);
    let fuzzy = |text: &str| {
        let query = Query::parse_as_you_type(text, 1_000_000_000).unwrap();
        index
            .search(&query, &SearchOptions::default())
            .unwrap()
            .fuzzy
    };
    assert!(fuzzy("scool fees"));

    // "Did you mean": the nearest words that are in the mail.
    let suggest = |text: &str| index.suggest(text, true).unwrap();
    assert_eq!(suggest("haskina banu").as_deref(), Some("hasina banu"));
    assert_eq!(suggest("Haskina Banu").as_deref(), Some("Hasina Banu"));
    assert_eq!(suggest("hasina bano").as_deref(), Some("hasina banu"));
    assert_eq!(suggest("from:Hasnia").as_deref(), Some("from:Hasina"));
    assert_eq!(suggest("scool fees").as_deref(), Some("school fees"));
    assert_eq!(suggest("hasina banu"), None);
    assert_eq!(suggest("hasina b"), None);
    assert_eq!(suggest("\"scool fees\" -haskina"), None);
    assert_eq!(suggest("qzxwvq"), None);
    // One typo from both; the first letter wins over the 26 "Person" mails.
    assert_eq!(suggest("kerson").as_deref(), Some("kerston"));
    // An unfinished last word is completed from the nearest start.
    assert_eq!(suggest("haskin").as_deref(), Some("hasina"));
    assert!(!fuzzy("haskina banu"));
    assert!(!fuzzy("school fees"));
}
