# SPDX-License-Identifier: GPL-3.0-or-later

# The Update dialog, from Help > Check for Updates and the notification
# that an update is ready. Its states and buttons are the about-update-*
# messages in dialogs.ftl.

# Its title when this copy is updated by the package manager.
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
