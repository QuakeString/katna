<!-- SPDX-License-Identifier: GPL-3.0-or-later -->

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
