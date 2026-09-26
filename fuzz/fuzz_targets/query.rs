// SPDX-License-Identifier: GPL-3.0-or-later

//! Query parser and compiler: any input must parse or fail cleanly, and
//! whatever parses must compile to a tantivy query.

#![no_main]

use katna_search::Query;
use katna_search::compile::compile;
use katna_search::schema::{Fields, build_schema};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(input) = std::str::from_utf8(data) else {
        return;
    };
    let fields = Fields::from_schema(&build_schema()).expect("schema has every field");
    for query in [
        Query::parse_at(input, 1_000_000_000),
        Query::parse_as_you_type(input, 1_000_000_000),
    ]
    .into_iter()
    .flatten()
    {
        let _ = query.has_free_text();
        let _ = compile(&fields, &query);
    }
});
