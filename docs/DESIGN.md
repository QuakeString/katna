# Katna design system

Every Katna window is built from the same named values and the same shared
controls, so a hover, a corner or a gap looks alike wherever it appears.
Decided 2026-10-03 (Design system study); the values come from what the
code already used most, so moving code onto them changes little on screen.

## Where things live

| What | Where |
| --- | --- |
| Radii, spacing, text sizes, state opacities, elevation levels, durations | `katna_ui::tokens` (`crates/katna-ui/src/tokens.rs`) |
| Springs | `katna_ui::motion` (`SMOOTH`, `GENTLE`, `SLIDE`, `QUICK`) |
| Colour roles | Katna Mail's `Theme` (`apps/katna-mail/src/theme.rs`), built from the schemes in `katna_ui::schemes` |
| Lengths | always through `katna_ui::px` (interface scale) |
| Shared controls | `apps/katna-mail/src/widgets.rs`, moving to `katna-ui` as they are shared beyond Mail |

## Tokens

**Radius:** `XS` 4 (checkboxes, tags, inline code), `SM` 8 (fields, menus
and their rows, thumbnails), `MD` 12 (tiles and cards inside a card), `LG`
16 (cards, dialogs, popovers, sheets, Compose; `PANEL_RADIUS`), `FULL`
(pills, avatars). A shape inside another takes
`radius::inner(outer, padding)`. The title bar's roundness setting stays the
user's.

**Spacing:** `S1` 2 (optical nudges only), `S2` 4, `S3` 8, `S4` 12, `S5` 16,
`S6` 24, `S7` 32, `S8` 48. Odd values only for 1-2 px optical alignment,
with a comment saying why.

**Text:** `MICRO` 11, `CAPTION` 12, `SMALL` 13, `BODY` 14, `SUBTITLE` 16,
`TITLE` 20, `DISPLAY` 24, each with `text::line_height`. Weights: normal,
medium, semibold, bold.

**State layers** (the text colour at this opacity over the element): hover
7%, pressed 14%, selected 10% (12% dark), dragged 16%, disabled 38%.
Separating lines are a quarter of an edge's strength.

**Lines:** `faint` (today's `th.divider`, `th.faint_line`) separates;
`edge` (today's `th.outline`) outlines what can be clicked or typed in:
fields and chips; outlined and pill buttons keep their stronger edge
(`text_faint` at 70%, his call to keep them at full strength); a strong
line marks focus and errors.

**Elevation:** five levels, each fixing surface, shadow and edge together,
in light and dark alike.
Level 1 is `Theme::card_edge` and `widgets::card_shadow(th, t)` (#647):
light `card_edge` is the shadow ink at 10% (`0x3c40431a`), drawn as a 1 px
ring (at `CARD_REST` = 0.15 of full strength while the card rests) under a
0,1 / blur 2 shadow at 47% of `th.shadow`; dark has no `card_edge` and
keeps one 0,1 / blur 3 shadow at 30%.

| Level | Used for | Dark (#448) | Light |
| --- | --- | --- | --- |
| 0 Page | the page | page colour | page colour |
| 1 Card | list, open mail, contact card, agenda | surface, one soft shadow | white, `card_edge` ring + short shadow |
| 2 Float | floating buttons, dragging | `raised` + rim | white, edge + short shadow |
| 3 Menu | menus, dialogs (`widgets::dialog`) | `menu` + rim | white, edge + shadow |
| 4 Popover | notched popovers (`notched::popover`), the tour | `menu` + rim | white, edge + deeper shadow |

A tile (`widgets::tile`) rests at level 1 at full strength; a file card
rises to level 2 under the pointer in `FAST` (`widgets::tile_lift`), and
the buttons on its corners are level 2 on frosted glass
(`attachments::Lifted`).

**Motion:** a hover never switches on at once: round and pill
buttons carry `katna_ui::Glow`, and any other box that tints on hover
puts `widgets::hover_fade` first among its children. Springs for movement (`SLIDE` is the one with a little
overshoot). Timed fades: `FAST` 140 ms (hover), `BASE` 220 ms (fades,
folds), `SLOW` 400 ms (page swaps), `LINGER` 900 ms (slow reveals).
Every spring goes through `motion::scaled` and every timed animation
through `motion::time`, so Settings > Appearance > Animation speed
(the desktop's speed, KDE's `AnimationDurationFactor`, or Katna's own
50-200%) stretches them all. Reduce motion (the desktop's, or always or
never in Settings) sets GPUI's `reduce_motion`: `with_animation`,
`with_spring` and `Spring::tick` then jump to the end.
A menu fades out in `FAST` when it closes, out of reach while it fades
(the right-click menu keeps itself, marked closing, until the fade ends;
a toolbar menu is noted as it closes, by `track_menu_fade`, and
`with_menu` draws it fading).
A notched popover does the same: it notes when it closed
(`notched::fade_out`), draws itself through `notched::fading` until
`notched::faded`, and treats a fading popover as closed.
Something that opens and closes in place (a card that folds to a line, a
section that unfolds) glides with `widgets::fold_box` and turns its
`widgets::fold_arrow`, sharing one `widgets::Fold`: the height glides on
`SLIDE`, the arrow turns half round on `SMOOTH`, and nothing that stays
fades or blinks. An arrow never swaps for another icon: a
`fold_arrow` turns by itself whenever its open state changes, however it
changed. A box that folds to nothing glides open from 0.

## Shared controls

The same element in two places is one shared widget. Built: icon button,
pill button, `widgets::button` with `ButtonStyle::Filled`, `Outlined` and
`Text` (`filled_button`, `outlined_button`, `text_button`),
`tonal_icon_button` (a contact's actions), `choice_chip` (32 px, `SM`,
edge at rest, the selected tint with a check while picked), `tag` (a grey label
pill), `row` (a clickable line: 40 px at least, `SM`, hover, the
selected tint while open, ripple; `ticked_row` has the ticked tint and
`count_pill` a count at its end), `field` and `line_field` (an edge at rest, a 2 px
accent ring inside it while it has the keys; 40 px for one line), menu
and menu item, switch,
checkbox, radio, colour swatch and wheel, avatar, tooltip, snackbar, scroll
bar, skeleton, `notched::popover` (opens at the click, notch, level 4,
`LG`), `widgets::dialog` (level 3, `LG`, frosted), `widgets::card` (level
1: rounds, fills through `pane` and adds `card_shadow`; the mail list, open
mail, person card, agenda and Settings), `widgets::tile` (level 1 at full
strength, `MD`, `th.surface`: a file, folder, attachment, invitation,
summary, inline reply or mail service to pick; a clickable one adds
`tile_hover` first). A card or tile never draws its own outline
in place of the shared edge and shadow. To build: `Field`'s error state and
suggestions. The Gallery (`katna-mail --page gallery`, development
builds only, `window/gallery.rs`) shows every shared control in light and
dark; add a new control to it.

## Moving code over

1. Tokens and this file, no visible change.
2. `ci/check-tokens.sh` counts radii, text sizes and spacing typed as raw
   numbers in the GPUI crates; the counts in `ci/token-budgets.txt` only go
   down. Lower the budget in the PR that lowers the count.
3. Light elevation, from the card-edge decision.
4. The shared controls above, one PR each.
5. Area by area, by the thread that owns the area. Values that change on
   screen (15 to 16, 12.5 to 13) are shown as pictures before merging.
   Setup and the window frame join last.
