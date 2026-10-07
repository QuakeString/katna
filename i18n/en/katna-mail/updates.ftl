# SPDX-License-Identifier: GPL-3.0-or-later

# The Update dialog, from Help > Check for Updates and the notification
# that an update is ready. Its states and buttons are the about-update-*
# messages in dialogs.ftl.

# Its title when this copy does not update itself.
update-dialog-title = Updates
# While downloading.
update-dialog-downloading-detail = The download goes on when you close this window.
# $done and $total: sizes such as "12 MB".
update-dialog-progress = { $done } of { $total }
# The two version boxes.
update-dialog-installed = Installed
update-dialog-new = New version
# $date: when the version was built, such as "28 September 2026, 21:16".
update-dialog-built = Built { $date }
# The newest change in a build: the first line of its commit, in English.
update-dialog-latest-change = Latest change: { $title }
# Where the installed build came from.
update-dialog-source-arch = Arch package, nightly channel
update-dialog-source-windows = Katna Setup for Windows, nightly channel
update-dialog-source-appimage = AppImage, nightly channel
update-dialog-source-tarball = Linux tarball, nightly channel
update-dialog-source-rpm = Fedora package, nightly channel
update-dialog-source-deb = Ubuntu and Debian package, nightly channel
update-dialog-source-snap = Snap, nightly channel
update-dialog-source-flatpak = Flatpak, nightly channel
update-dialog-source-nix = Nix, nightly channel
# For a package Katna cannot install itself, above the command that does.
update-dialog-command-detail = Run this command in a terminal to install it.
# Copies that command.
update-dialog-copy = Copy
# Under "Katna Mail is up to date"; $ago is like "5 minutes ago".
update-dialog-checked = Checked { $ago }
# $commit: the version's commit, such as "1ef4594"; opens it on GitHub.
update-dialog-commit = Commit { $commit }
# $size: such as "39 MB".
update-dialog-size = Download size { $size }
update-dialog-whats-new = What’s new in this version
update-dialog-changes-title = Changes
# Shows or hides the list of every change since the installed version.
update-dialog-changes = { $count ->
    [one] 1 change since your version
   *[other] { $count } changes since your version
}
# When the installed version is older than the list goes back.
update-dialog-latest-changes = The latest { $count } changes
# Opens the changes between the two versions on GitHub.
update-dialog-compare = Compare on GitHub
# The app cannot reach katna-daemon, which checks and downloads.
update-dialog-no-service = The Katna background service is not running.
update-dialog-later = Later
update-dialog-close = Close
