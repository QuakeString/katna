Katna for Linux (x86_64)
========================

Katna Mail and the Katna background service, built from main.

Install for you only (into ~/.local):

    ./install.sh

Or for everyone on this computer:

    sudo ./install.sh --prefix /usr/local

Then open Katna Mail from your apps menu. D-Bus starts the background
service (katna-daemon) when Katna Mail or katnactl needs it.

Katna needs a desktop with the usual libraries (libxkbcommon, fontconfig,
freetype, Wayland or X11, and Vulkan or OpenGL drivers) and a password
store (GNOME Keyring, KWallet or KeePassXC) for account passwords.

To remove it: ./install.sh --uninstall (with the same --prefix).
Your mail and settings in ~/.local/share/katna and ~/.config/katna stay.

Katna is free software under the GNU GPL, version 3 or later (LICENSE in
share/licenses/katna).
