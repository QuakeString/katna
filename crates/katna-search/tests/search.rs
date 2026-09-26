// SPDX-License-Identifier: GPL-3.0-or-later

//! End-to-end: store → index → queries.

use katna_core::Paths;
use katna_import::{Flags, IncomingMessage, MessageSink, StoreSink, parse_message};
use katna_search::{IndexOptions, Query, SearchIndex, SearchOptions, Sort};
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
    assert!(typing("budgetx").is_empty());

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
