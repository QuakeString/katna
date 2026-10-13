#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-or-later
"""Writes the Store package's logos in packaging/windows/store/Assets from
the rendered hicolor PNGs (packaging/icons/render.py), as make-ico.py does
katna.ico. Needs Pillow. Run again after the icon changes."""
from pathlib import Path

from PIL import Image

here = Path(__file__).resolve().parent
hicolor = here.parent.parent / "icons" / "hicolor"
icon = Image.open(next((hicolor / "512x512" / "apps").glob("*.png"))).convert("RGBA")


def logo(name, width, height, scale):
    """The icon `scale` of the tile's height, centred on a clear tile."""
    side = round(height * scale)
    tile = Image.new("RGBA", (width, height), (0, 0, 0, 0))
    small = icon.resize((side, side), Image.LANCZOS)
    tile.alpha_composite(small, ((width - side) // 2, (height - side) // 2))
    tile.save(here / "Assets" / name, optimize=True)


# Tiles leave room around the icon, as Windows' own do; the list and
# taskbar icons fill theirs.
logo("StoreLogo.png", 50, 50, 1.0)
logo("Square44x44Logo.png", 44, 44, 1.0)
logo("Square44x44Logo.targetsize-44_altform-unplated.png", 44, 44, 1.0)
logo("Square150x150Logo.png", 150, 150, 0.66)
logo("Wide310x150Logo.png", 310, 150, 0.66)
