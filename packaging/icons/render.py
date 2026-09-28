#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-or-later
"""Renders Katna's icon PNGs and the tray's raw pixels from the SVGs in src/.

Needs rsvg-convert (librsvg) and Pillow. Run it after changing the logo:

    python3 packaging/icons/render.py

- 16 to 32 px use src/katna-small.svg, the k on its disc without the
  shadow, which blurs to mush that small.
- 48 px and up use src/katna.svg, with the shadow.
- src/katna-symbolic.svg, the one-colour k the tray shows, is copied to
  hicolor/symbolic/apps/ as <app ID>-symbolic.svg.

Then run packaging/windows/make-ico.py for Windows' katna.ico.
"""
import shutil
import pathlib
import subprocess
import tempfile

from PIL import Image

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parent.parent
# The app ID, from the scalable icon's name (IDs live in katna_core::ids).
APP_ID = next(HERE.glob("*.svg")).stem
HICOLOR = [16, 22, 24, 32, 48, 64, 96, 128, 256, 512]
# The tray's pixmaps (crates/katna-platform/src/tray.rs, SIZES).
TRAY = [16, 22, 24, 32, 48, 64]


def source(size):
    return HERE / "src" / ("katna-small.svg" if size <= 32 else "katna.svg")


def render(size, out):
    subprocess.run(
        ["rsvg-convert", "-w", str(size), "-h", str(size), str(source(size)), "-o", str(out)],
        check=True,
    )


def main():
    for size in HICOLOR:
        out = HERE / "hicolor" / f"{size}x{size}" / "apps" / f"{APP_ID}.png"
        out.parent.mkdir(parents=True, exist_ok=True)
        render(size, out)
    symbolic = HERE / "hicolor" / "symbolic" / "apps" / f"{APP_ID}-symbolic.svg"
    symbolic.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(HERE / "src" / "katna-symbolic.svg", symbolic)
    tray = ROOT / "crates" / "katna-platform" / "icons"
    tray.mkdir(exist_ok=True)
    with tempfile.TemporaryDirectory() as tmp:
        for size in TRAY:
            png = pathlib.Path(tmp) / f"{size}.png"
            render(size, png)
            # Straight-alpha RGBA, row by row, no header.
            (tray / f"{size}.rgba").write_bytes(Image.open(png).convert("RGBA").tobytes())


if __name__ == "__main__":
    main()
