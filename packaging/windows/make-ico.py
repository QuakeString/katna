#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-or-later
"""Writes packaging/windows/katna.ico, the Windows icon of Katna's programs
and Setup, from the rendered hicolor PNGs (packaging/icons/render.py).
Needs Pillow. Run again after the icon changes."""
from pathlib import Path

from PIL import Image

here = Path(__file__).resolve().parent
hicolor = here.parent / "icons" / "hicolor"
sizes = [16, 24, 32, 48, 64, 128, 256]
images = [
    Image.open(hicolor / f"{s}x{s}" / "apps" / "in.invenia.katna.Mail.png").convert("RGBA")
    for s in sizes
]
images[-1].save(
    here / "katna.ico",
    sizes=[(s, s) for s in sizes],
    append_images=images[:-1],
)
