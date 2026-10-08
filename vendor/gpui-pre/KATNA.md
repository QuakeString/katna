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

Right-to-left text wraps line by line in reading order
(`LineLayoutCache::wrap_bidi`, `line_layout.rs`). A shaped line is in the
order it shows, so a right-to-left paragraph's glyph positions fall as
their index grows, and breaks found along them cut it apart (the first
line held one letter, the rest ran out of the box). For a line holding
right-to-left letters the breaks are found on its glyphs put back in
reading order, each character as wide as it was shaped; then each wrapped
line is shaped on its own, so bidi orders it as a line, its glyphs are put
in the order they show, and the lines follow one another as a
left-to-right line's parts do. Not yet: a wrapped right-to-left line's
glyphs are in visual order, so a style run (a link's colour, an
underline) is matched to glyphs in that order and can spill into the
neighbouring text of the same line.

Run the crate's tests with
`cargo test --manifest-path vendor/gpui-pre/Cargo.toml --lib --features test-support`
(it is outside the workspace; the `Cargo.lock` that writes is ignored).

## Layout direction (right to left)

GPUI had no layout direction. Now a window, or any element's subtree, can
be laid out right to left, as Arabic, Hebrew, Persian and Urdu need:

- `LayoutDirection` (`Ltr`, `Rtl`; `style.rs`) and `Style::layout_direction`
  (`None` inherits). `Window::set_layout_direction` sets the window's
  (left to right unless set); `Styled::layout_rtl()` and
  `Styled::layout_ltr()` set a subtree's, the latter for content that keeps
  its direction in a right-to-left window (phone numbers, code, media
  controls). `Window::layout_direction()` is the current one while drawing.
- `Window::with_layout_direction` keeps a stack, like `with_text_style`;
  `Interactivity::request_layout`, `prepaint` and `paint` (`div.rs`, so
  divs, images, SVGs and uniform lists) push their style's direction.
  Deferred draws keep the direction where they were deferred.
- `TaffyLayoutEngine` (`taffy.rs`) marks nodes requested in a right-to-left
  direction and, in `layout_bounds`, mirrors each child's x inside such a
  parent's border box. Flex rows, grid columns, padding, margins, gaps and
  absolute insets therefore start on the right, without changing taffy.
- A right-to-left div paints its left border and corners on the right
  (`Style::mirrored`), so `border_l` and `rounded_l` are "start" there, like
  `pl` and `ml`.
- Text alignment reads as start and end, like padding: in a right-to-left
  layout `TextAlign::Left` (the default, `text_left`) puts text on the
  right and `Right` on the left (`TextAlign::resolve`, applied by the text
  element). Code that paints a `ShapedLine` itself passes the alignment
  as given, so it resolves it first.

- Box shadows fall the other way: `Style::mirrored` turns their x
  offsets around.
- Horizontal scrolling (`div.rs`): a right-to-left row starts at the right
  and runs out past the left edge, so its scroll offset goes from 0 up to
  the most it can scroll (positive, the content moving right) instead of
  down from 0, and a vertical wheel on a row that scrolls only sideways
  moves it toward its end, on the left. `ScrollHandle::scroll_to_item`
  works as before, on screen positions.
- `uniform_list` puts its items against the right padding and border, and
  `list` an item narrower than itself against its right edge; `list`
  also gives its items its own `layout_ltr`/`layout_rtl`.
- `anchored` reads its anchor's left and right as start and end: anchored
  by the top left corner (the default) it opens toward the left of its
  point, horizontal offsets turn around, without a position it hangs from
  the right of where it was laid out, local positions count from there
  toward the left, and one wider than the window keeps its right edge.
  Positions given in window coordinates are where they are on screen (a
  pointer's position), so a menu opens toward the left of the pointer.
- `Svg::mirror_rtl()` draws an icon flipped left to right in a
  right-to-left layout, after its own transformation (so a chevron that
  turns clockwise to point down turns the other way), for icons that
  point along the line; Katna picks which in `katna_ui::icons`.
- Scrollbars: GPUI draws none itself. Space taffy reserves for one
  (`scrollbar_width`) is mirrored with the rest, to the left; Katna's own
  scrollbar (`katna-ui`) sits on the left in a right-to-left layout.

Tests: `taffy::direction_tests`, `elements::anchored::tests` for menus and
`elements::svg::mirror_tests`.
