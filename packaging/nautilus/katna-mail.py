# SPDX-License-Identifier: GPL-3.0-or-later
"""Send with Katna Mail in GNOME Files' right-click menu on files and folders.

A new message in Katna Mail with them attached (folders as zips). With
several mail accounts the entry opens a submenu of them. The accounts come
from send-menu.json, which katna-daemon writes in Katna's data folder when
they change. Needs nautilus-python (Arch: python-nautilus).
"""

import json
import os
import subprocess

import gi

try:
    gi.require_version("Nautilus", "4.0")
except ValueError:
    gi.require_version("Nautilus", "3.0")
from gi.repository import GObject, Nautilus  # noqa: E402

LABEL = "Send with Katna Mail"


def menu_file():
    data = os.environ.get("XDG_DATA_HOME") or os.path.expanduser("~/.local/share")
    return os.path.join(data, "katna", "send-menu.json")


def read_menu():
    """The entry's label and the mail accounts, as the daemon wrote them."""
    try:
        with open(menu_file(), encoding="utf-8") as file:
            menu = json.load(file)
        return menu.get("label") or LABEL, menu.get("accounts") or []
    except (OSError, ValueError, AttributeError):
        return LABEL, []


def send(paths, address=None):
    command = ["katna-mail", "--attach"]
    if address:
        command += ["--from", address]
    subprocess.Popen(command + paths, start_new_session=True)


class KatnaMailMenu(GObject.GObject, Nautilus.MenuProvider):
    def get_file_items(self, *args):
        # Nautilus 43 and later pass the files alone; older ones the
        # window first.
        files = args[-1]
        paths = [f.get_location().get_path() for f in files]
        paths = [p for p in paths if p]
        if not paths:
            return []
        label, accounts = read_menu()
        item = Nautilus.MenuItem(name="KatnaMail::Send", label=label)
        if len(accounts) < 2:
            item.connect("activate", lambda _item: send(paths))
            return [item]
        submenu = Nautilus.Menu()
        item.set_submenu(submenu)
        for ix, account in enumerate(accounts):
            address = account.get("address", "")
            line = Nautilus.MenuItem(
                name=f"KatnaMail::Send{ix}",
                label=account.get("label") or address,
            )
            line.connect("activate", lambda _item, a=address: send(paths, a))
            submenu.append_item(line)
        return [item]
