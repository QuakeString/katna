<!-- SPDX-License-Identifier: GPL-3.0-or-later -->

# What's new highlights

What's new shows these after an update (`src/whats_new.rs`,
`docs/ARCHITECTURE.md` §13.6). Each highlight is one file in
`highlights/`, which `build.rs` builds into the app:

- Name it `YYYY-MM-DD-HHMM-slug.toml`: the UTC date and time you write it,
  then a few lowercase words joined by `-`
  (`2026-09-27-0444-about-katna.toml`). Highlights are shown newest first
  by that name, and the settings file remembers the names shown, so one
  that merges after newer ones still shows.
- Never rename, renumber or edit another change's file to make room.

```toml
# SPDX-License-Identifier: GPL-3.0-or-later

title = "About Katna"
text = """
Help > About Katna, also in Quick settings, shows the version, the
changelog and every library Katna is built on.
"""
# Only for a major feature: whats-new/<name>-light.webp and -dark.webp.
animation = "reply-row"
```

A highlight about another app than Mail can say so with
`apps = ["tasks"]` (any of `calendar`, `contacts`, `tasks`, `notes`,
`files`): while every app it names is turned off in Settings › Apps, it is
left out.

The text is one paragraph; line breaks in the file become spaces. The
build stops on a bad name, an unknown key, a missing title or text, or a
missing animation file.

## Translations

Write highlights in English only. Their translations are added later, with
the rest of the languages work, one file per language beside its `.ftl`
files: `i18n/<language>/katna-mail/whats-new.toml`, with a table per
highlight named by its file name without `.toml`:

```toml
["2026-09-27-0444-about-katna"]
title = "Katna について"
text = """
ヘルプ > Katna について（クイック設定にもあります）には、バージョン、
変更履歴、Katna が使っているすべてのライブラリが表示されます。
"""
```

What's new shows a highlight in the chosen language when that file has
its table, else in English. The text is one paragraph here too. The build
stops on a table for a highlight that does not exist, a key other than
`title` and `text`, or an empty one.

# What's new animations

Short looping clips of major features, built into Katna Mail and shown in
the What's new dialog after an update (`src/whats_new.rs`,
`docs/ARCHITECTURE.md` §13.6). Each feature has two: `<name>-light.webp`
and `<name>-dark.webp`, recorded from Katna Mail itself in each theme.

Keep them small, because the app decodes every frame while the dialog is
open:

- 560 px wide (the dialog's width; drawn at their own pixel size), about
  200 px tall;
- about 50 frames: record at 20 fps, drop repeated frames and turn them
  into longer frame delays;
- at most 600 KB each (a test checks this);
- start and end on the same picture, so the loop does not jump.

How `reply-row-*.webp` was made, on Xvfb with a demo store: open a
message in a 1240×560 window, hold the mouse on the list/reader divider,
and record the reader while dragging it:

```sh
ffmpeg -f x11grab -draw_mouse 0 -framerate 20 -video_size 660x250 \
  -i :99.0+636,355 -t 6 frames/f%04d.png
# drop repeated frames, then for each kept frame:
convert frame.png -filter Lanczos -resize 560x scaled.png
img2webp -loop 0 -min_size -lossy -q 72 -m 6 -d <ms> s0000.png -d <ms> s0001.png … -o reply-row-light.webp
```
