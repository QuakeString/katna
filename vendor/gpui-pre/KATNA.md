# gpui-pre, patched for Katna

This is `gpui-pre` 0.3.6 from crates.io (Zed's `gpui` at `bcf6582`,
Apache-2.0, see `LICENSE-APACHE`), used through `[patch.crates-io]` in the
workspace `Cargo.toml`. Its examples and integration test are left out,
except the one picture a unit test reads (`examples/image/`).

The first commit that added this directory holds the crate unchanged, so
`git diff` against it shows the whole patch. When GPUI is upgraded, copy the
new version here and apply the same changes, or drop a patch once upstream
GPUI can do it.

## Line breaking

Upstream wrapped after spaces and before any character outside its list of
"word characters", so Arabic, Hindi (Devanagari), Thai and most other
scripts were split mid-word, even inside a letter and its marks.

- `src/text_system/line_breaks.rs` finds where a line may wrap: UAX #14
  line break opportunities from ICU4X's `icu_segmenter` (compiled data,
  `line-break: normal`), with dictionaries for Thai, Lao, Khmer and
  Burmese, which are written without spaces; and grapheme cluster
  boundaries.
- `LineWrapper::wrap_line`, `truncate_wrapped_line` and
  `LineLayout::compute_wrap_boundaries` (`line_layout.rs`) wrap at those
  opportunities. A word wider than the line is split between grapheme
  clusters only; a single cluster wider than the line overflows whole.
  Truncation never cuts a cluster either. `is_word_char` is gone; its test
  now checks the same words against UAX #14 (a hyphen is a break
  opportunity there, as in browsers).
- The `svg_renderer` tests are off: they read fonts from Zed's repository,
  which the published crate does not carry.

Run the crate's tests with
`cargo test --manifest-path vendor/gpui-pre/Cargo.toml --lib --features test-support`
(it is outside the workspace; the `Cargo.lock` that writes is ignored).
