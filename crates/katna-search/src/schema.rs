// SPDX-License-Identifier: GPL-3.0-or-later

//! The tantivy schema (`docs/ARCHITECTURE.md` §7.1) and its tokenizer.
//!
//! Documents keep only IDs and what filtering and ranking need. Display data
//! (subject, sender, snippet) comes from SQLite, and highlighted snippets are
//! made from the raw message, so the index stores no text.

use tantivy::schema::{
    FAST, Field, INDEXED, IndexRecordOption, STRING, Schema, TextFieldIndexing, TextOptions,
};
use tantivy::tokenizer::{
    AsciiFoldingFilter, Language, LowerCaser, RemoveLongFilter, Stemmer, TextAnalyzer,
    TokenizerManager,
};

use crate::words::WordTokenizer;

/// Version of the schema and of what the indexer puts into it. Raise it when
/// either changes; an index with another version must be rebuilt.
/// 3: Thai, Lao, Khmer and Burmese split into words.
pub const SCHEMA_VERSION: u32 = 3;

/// Name of the tokenizer of the text fields that match words as written.
pub const TOKENIZER: &str = "katna";

/// Name of the tokenizer of the stemmed fields.
pub const STEM_TOKENIZER: &str = "katna_en";

/// Tokens longer than this many bytes are dropped (base64 runs, hashes).
const MAX_TOKEN_BYTES: usize = 40;

/// Every field of the schema.
#[derive(Debug, Clone, Copy)]
pub struct Fields {
    /// `message.id`; the key into SQLite.
    pub msg_id: Field,
    pub account: Field,
    /// Unix seconds.
    pub date: Field,
    /// Bytes.
    pub size: Field,
    /// `message.flags` bits, for ranking.
    pub flags: Field,
    pub subject: Field,
    pub body: Field,
    /// `subject` and `body` stemmed (English), so `contract` also finds
    /// `contracts`. Without positions: phrases match the words as written.
    pub subject_stem: Field,
    pub body_stem: Field,
    /// `From` and `Sender`: names and addresses.
    pub from: Field,
    pub to: Field,
    pub cc: Field,
    pub bcc: Field,
    /// Attachment file names.
    pub attachment: Field,
    /// `List-Id`.
    pub list: Field,
    /// Domains of all participants and their parent domains, for `org:` later.
    pub domain: Field,
    /// Folder paths and each of their components, lower-cased.
    pub folder: Field,
    /// Keywords and labels, lower-cased.
    pub label: Field,
    /// System flags: `seen`, `answered`, `flagged`, `draft`, `deleted`, `forwarded`.
    pub flag: Field,
    /// `attachment` when the message has one.
    pub has: Field,
}

impl Fields {
    /// Stemmed fields that a plain word also searches, with their boosts.
    /// The stemmed subject adds half the subject's boost, so the word as
    /// written ranks first there; the stemmed body replaces the body.
    pub fn stemmed(&self) -> [(Field, f32); 2] {
        [(self.subject_stem, 1.5), (self.body_stem, 1.0)]
    }

    /// Text fields that plain words search, with their boosts.
    pub fn free_text(&self) -> [(Field, f32); 7] {
        [
            (self.subject, 3.0),
            (self.from, 2.0),
            (self.to, 1.0),
            (self.cc, 1.0),
            (self.attachment, 1.5),
            (self.list, 1.0),
            (self.body, 1.0),
        ]
    }

    /// The fields of `schema`, which must come from [`build_schema`].
    pub fn from_schema(schema: &Schema) -> tantivy::Result<Self> {
        let get = |name: &str| schema.get_field(name);
        Ok(Self {
            msg_id: get("msg_id")?,
            account: get("account")?,
            date: get("date")?,
            size: get("size")?,
            flags: get("flags")?,
            subject: get("subject")?,
            body: get("body")?,
            subject_stem: get("subject_stem")?,
            body_stem: get("body_stem")?,
            from: get("from")?,
            to: get("to")?,
            cc: get("cc")?,
            bcc: get("bcc")?,
            attachment: get("attachment")?,
            list: get("list")?,
            domain: get("domain")?,
            folder: get("folder")?,
            label: get("label")?,
            flag: get("flag")?,
            has: get("has")?,
        })
    }
}

/// The schema of version [`SCHEMA_VERSION`].
pub fn build_schema() -> Schema {
    let mut builder = Schema::builder();
    let text = TextOptions::default().set_indexing_options(
        TextFieldIndexing::default()
            .set_tokenizer(TOKENIZER)
            .set_index_option(IndexRecordOption::WithFreqsAndPositions),
    );
    builder.add_u64_field("msg_id", INDEXED | FAST);
    builder.add_u64_field("account", INDEXED | FAST);
    builder.add_i64_field("date", INDEXED | FAST);
    builder.add_u64_field("size", FAST);
    builder.add_u64_field("flags", FAST);
    for name in [
        "subject",
        "body",
        "from",
        "to",
        "cc",
        "bcc",
        "attachment",
        "list",
    ] {
        builder.add_text_field(name, text.clone());
    }
    let stemmed = TextOptions::default().set_indexing_options(
        TextFieldIndexing::default()
            .set_tokenizer(STEM_TOKENIZER)
            .set_index_option(IndexRecordOption::WithFreqs),
    );
    for name in ["subject_stem", "body_stem"] {
        builder.add_text_field(name, stemmed.clone());
    }
    for name in ["domain", "folder", "label", "flag", "has"] {
        builder.add_text_field(name, STRING);
    }
    builder.build()
}

/// The analyzer of the text fields as written: Unicode words (Thai, Lao,
/// Khmer and Burmese split with dictionaries), lower-cased and folded to
/// ASCII (`café` finds `cafe`).
pub fn analyzer() -> TextAnalyzer {
    TextAnalyzer::builder(WordTokenizer::default())
        .filter(RemoveLongFilter::limit(MAX_TOKEN_BYTES))
        .filter(LowerCaser)
        .filter(AsciiFoldingFilter)
        .build()
}

/// [`analyzer`] followed by the English Snowball stemmer. Every message is
/// stemmed as English until we detect languages; on other languages it
/// mostly leaves words alone or merges a few forms.
pub fn stem_analyzer() -> TextAnalyzer {
    TextAnalyzer::builder(WordTokenizer::default())
        .filter(RemoveLongFilter::limit(MAX_TOKEN_BYTES))
        .filter(LowerCaser)
        .filter(AsciiFoldingFilter)
        .filter(Stemmer::new(Language::English))
        .build()
}

/// Registers [`TOKENIZER`] and [`STEM_TOKENIZER`] in `manager`.
pub fn register_tokenizers(manager: &TokenizerManager) {
    manager.register(TOKENIZER, analyzer());
    manager.register(STEM_TOKENIZER, stem_analyzer());
}

/// The words of `text` as the index sees them.
pub fn tokens(text: &str) -> Vec<String> {
    tokens_with(analyzer(), text)
}

/// The words of `text` as the stemmed fields see them.
pub fn stems(text: &str) -> Vec<String> {
    tokens_with(stem_analyzer(), text)
}

fn tokens_with(mut analyzer: TextAnalyzer, text: &str) -> Vec<String> {
    let mut stream = analyzer.token_stream(text);
    let mut out = Vec::new();
    while stream.advance() {
        out.push(stream.token().text.clone());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_has_every_field() {
        let schema = build_schema();
        let fields = Fields::from_schema(&schema).unwrap();
        assert_eq!(schema.get_field_name(fields.body), "body");
        assert_eq!(schema.get_field_name(fields.has), "has");
    }

    #[test]
    fn tokenizes_addresses_and_accents() {
        assert_eq!(
            tokens("Kenneth.Lay@Enron.com"),
            ["kenneth", "lay", "enron", "com"]
        );
        assert_eq!(tokens("Café crème"), ["cafe", "creme"]);
        assert!(tokens(&"x".repeat(100)).is_empty());
    }

    #[test]
    fn stems_english() {
        assert_eq!(
            stems("Contracts contracted CONTRACTING"),
            ["contract", "contract", "contract"]
        );
        assert_eq!(stems("Café meetings"), ["cafe", "meet"]);
    }
}
