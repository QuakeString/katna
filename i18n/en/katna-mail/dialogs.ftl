# Katna Mail, English: About, What’s new, first run, tour and crash dialogs.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## About Katna (the dialog from the "i" button on the top bar)

# The top bar button's tooltip. Katna is the app's name; keep it as is.
about-tooltip = About Katna
# Under the app's name.
about-tagline = Mail and calendar for the Linux desktop
# The small button beside the version (About, What’s new, Settings): it
# copies the version, build date and system for a bug report.
about-copy-version = Copy version details
about-version-copied = Copied
# Lines of the copied version details. $date: when this build was made.
about-version-built = Built: { $date }
# $system: the operating system and desktop, such as "Arch Linux, KDE on wayland".
about-version-system = System: { $system }
# Opens the What’s new dialog.
about-whats-new = What’s new

## Updates, in a box under the version in About (only in packages that
## update themselves). $version is a version such as 0.0.0.r236.g1a2b3c4.

about-update-not-checked = Updates have not been checked yet
about-update-checking = Checking for updates…
about-update-up-to-date = Katna Mail is up to date
# A check failed; the reason, if known, shows under it.
about-update-check-failed = Could not check for updates
about-update-available = Version { $version } is available
# $percent: how much is downloaded, a whole number from 0 to 100.
about-update-downloading = Downloading version { $version }… { $percent }%
# The download failed even after trying again; the reason shows under it.
about-update-download-failed = The download of version { $version } did not finish
about-update-ready = Version { $version } is ready to install
about-update-ready-detail = Katna Mail restarts to finish the update.
# After pressing Update: what happens next, before it happens.
about-update-confirm = Install version { $version }?
about-update-confirm-detail = Katna Mail will close, install the update and open again where you left off. Your computer will ask for your password.
# The same on Windows, where Katna Setup installs it.
about-update-confirm-detail-windows = Katna Mail will close, install the update and open again in a moment.
about-update-installing = Installing version { $version }…
about-update-installing-detail = Enter your password in the window that opened.
about-update-installing-detail-windows = Katna Mail closes now and opens again once the update is installed.
# The password window was closed or the password was refused.
about-update-cancelled = The update was not installed, because the password was not given.
# $error: what the installer said.
about-update-failed = The update could not be installed: { $error }
# A build that does not update itself: built from source, or a package
# from somewhere else.
about-update-not-self-updating = This copy of Katna Mail doesn’t update itself. Update it the way you installed it.
# $error: the system's reason.
about-update-restart-failed = The update is installed, but Katna Mail could not open again ({ $error }). Open it yourself.
# Buttons.
about-update-check = Check for updates
about-update-download = Download
about-update-retry = Try again
about-update-button = Update
about-update-restart = Update and restart
about-update-cancel = Not now
# Opens the list of changes in each version, in the browser.
about-changelog = Changelog
# Opens Katna's source code on the web.
about-source = Source code
# A button to support the author with a small donation.
about-coffee = Buy me a coffee
# A little play inside "Buy me a coffee" when the pointer rests on it: it
# asks for less and less, settles for water, then thanks the reader. Keep
# each line short, to fit the button.
about-coffee-coffee = Coffee?
about-coffee-tea = Tea?
about-coffee-pizza = Pizza?
about-coffee-nothing = Nothing? At all?
about-coffee-water = I'll survive on water!!
about-coffee-thanks = Thank you for using Katna
# Tooltip on "Buy me a coffee" while it does nothing yet.
about-coming-soon = Coming soon
# Before the author's GitHub, x.com and LinkedIn links: "Follow me on GitHub"...
about-follow-me = Follow me on
# Rust is a programming language; KDE is a free software community and its
# desktop. Keep both names as they are.
about-love-title = Made with love for Rust, KDE and Linux
# Plasma is KDE's desktop and PIM means personal information management
# (mail, calendar, contacts); "unsafe code" is a Rust term.
about-love-text = Rust makes a fast and safe mail app a joy to write: Katna has no unsafe code. KDE's Plasma desktop and its PIM suite inspired Katna, and Linux and the free software community build the ground it stands on. Thank you, and thank you to the libraries below.
about-kde-text = KDE builds the desktop Katna feels most at home on, and it is made by volunteers and funded by people like you. If you enjoy Plasma or KDE's apps, please consider donating to KDE.
# Opens KDE's donation page in the browser.
about-donate-kde = Donate to KDE
# GPUI is the framework Katna Mail's interface is built with, and Zed is the
# code editor it was made for. Keep both names as they are.
about-gpui-title = Built on GPUI, from the Zed project
# Zed Industries is a company. Apache-2.0 is the name of GPUI's license.
about-gpui-text = Katna Mail's whole interface is built on GPUI, the fast, GPU-accelerated UI framework that Zed Industries made for the Zed editor. Every pixel, animation and window you see is drawn by it. Thank you, Zed team, for building it in the open. Apache-2.0.
# Opens GPUI's page on GitHub in the browser.
about-gpui-github = GPUI on GitHub
about-personal-title = A personal project
# Gmail, Mailspring and Thunderbird are other mail apps; LLMs are large
# language models, the AI behind chat assistants.
about-personal-text = Katna Mail does not try to be new or revolutionary. It is the mail app its author wanted, and its features and look are borrowed from Gmail, Mailspring and Thunderbird. It was only possible because of how far LLMs have come.
# A small heading above the libraries Katna is built on. English writes it
# in capitals; languages without capital letters write it normally.
about-built-on = BUILT ON FREE SOFTWARE
# What each library does in Katna, under its name (which stays as it is).
# IMAP and SMTP are the protocols for reading and sending mail; io-imap,
# io-smtp and io-sasl are library names.
about-credit-pimalaya = IMAP, SMTP and sign-in (io-imap, io-smtp, io-sasl)
about-credit-imap-codec = Reading and writing IMAP
about-credit-tantivy = Search
about-credit-sqlite = The mail store
about-credit-rustls = Secure connections
# Stalwart Labs is a company.
about-credit-mail-parser = Reading mail, from Stalwart Labs
# Servo is a web browser engine project.
about-credit-html5ever = HTML mail, from the Servo project
# D-Bus and portals are how Linux apps talk to the desktop.
about-credit-zbus = Talking to the desktop over D-Bus and portals
# The keyring is the desktop's store of passwords.
about-credit-oo7 = Passwords in the desktop's keyring
about-credit-hayro = Viewing and printing PDFs
about-credit-calamine = Spreadsheet previews
about-credit-resvg = SVG pictures
about-credit-jiff = Dates and time zones
# Helix is a code editor.
about-credit-spellbook = Spell check, from the Helix editor
# The library that lets Katna do many tasks at the same time.
about-credit-smol = Doing many things at once
about-credit-color-schemes = The palettes of the built-in color schemes
# Shows or hides the full list of libraries. $count: how many there are.
about-all-libraries = Every library Katna uses ({ $count })
# Under a library's name. $authors: the names of its authors.
about-library-authors = by { $authors }
# GNU GPL is the GNU General Public License; keep the name.
about-license = Katna is free software under the GNU GPL, version 3 or later.
about-close = Close

## What’s new (shown after an update)

whats-new-title = What’s new in Katna Mail
# $version: the version number, such as 0.9.0.
whats-new-updated = Updated to version { $version }
# $version: the version number, such as 0.9.0.
whats-new-version = Version { $version }
# Below the highlights. $count: how many more changes the changelog lists.
whats-new-more = { $count ->
    [one] And one more in the full changelog.
   *[other] And { $count } more in the full changelog.
}
# Opens the list of every change, in the browser.
whats-new-changelog = Full changelog
# Closes the dialog.
whats-new-got-it = Got it

## First run: welcome page

onboarding-welcome-title = Welcome to Katna Mail
onboarding-welcome-lead = Your mail on your own computer: quick to search, readable offline and private.
onboarding-fast-title = Fast, even offline
onboarding-fast-text = Katna keeps a copy of your mail here, so opening and searching it is instant, with or without a connection.
onboarding-providers-title = Works with your mail
# Gmail, Outlook, Yahoo and iCloud are mail providers; IMAP and POP are the
# protocols mail apps use. Keep the names as they are.
onboarding-providers-text = Gmail, Outlook, Yahoo, iCloud and any other IMAP or POP account.
onboarding-private-title = Private
onboarding-private-text = Your mail goes straight from your provider to this computer. No Katna server sees it.
onboarding-get-started = Get started

## First run: adding an account

# The background service (katna-daemon) fetches and sends mail for the app.
onboarding-service-checking = Checking the Katna background service…
onboarding-service-running = The Katna background service is running.
onboarding-service-missing = The Katna background service is not running
# Followed by the command to type in a terminal.
onboarding-service-start = It fetches and sends your mail. Start it from a terminal, then check again:
onboarding-check-again = Check again
onboarding-account-title = Add your mail account
# An app password is a separate password a provider makes for mail apps.
onboarding-account-lead = Type your email address and password, and Katna finds the server settings. Gmail, Yahoo and iCloud need an app password, made in your account's security settings.
onboarding-add-account = Add an account
# Goes back to the welcome page.
onboarding-back = Back

## First run: choosing the look

onboarding-look-title = Make it yours
onboarding-look-lead = Pick how mail opens and how Katna looks. You can change these any time in quick settings.
# Where an opened message is shown.
onboarding-reading-pane = Reading pane
# The message opens to the right of the mail list.
onboarding-pane-right = Right of the list
# The message opens in place of the mail list.
onboarding-pane-none = No split
# Light or dark colors.
onboarding-theme = Theme
# Light or dark, following the desktop's setting.
onboarding-theme-system = System
onboarding-theme-light = Light
onboarding-theme-dark = Dark
# How tightly the mail list is packed.
onboarding-density = Density
onboarding-density-default = Default
onboarding-density-compact = Compact
onboarding-continue = Continue

## First start: the Katna account page. A Katna account is an account on
## Katna's own server, not a mail account; see katna-account.ftl.

onboarding-katna-title = Get more with a Katna account
onboarding-katna-lead = It's optional. It turns on Katna's online features, and you can make one later in Settings > Subscription.
onboarding-katna-receipts-title = Read receipts
onboarding-katna-receipts-text = See when people open the mail you send.
onboarding-katna-links-title = Link tracking
onboarding-katna-links-text = See which links in your mail get clicked.
onboarding-katna-activity-title = Activity
onboarding-katna-activity-text = Opens and clicks for everything you sent, in one place.
onboarding-katna-translate-title = Automatic translation
onboarding-katna-translate-text = Read mail written in other languages in your own.
# Under the list, beside a lock.
onboarding-katna-private = It has its own password. Your mail logins never leave this computer.

## First run: done

onboarding-ready-title = You're all set
onboarding-ready-lead = Katna is getting your mail. It shows up as it arrives, and new mail appears on its own.
# $address: the email address of the account just added.
onboarding-ready-lead-address = Katna is getting the mail of { $address }. It shows up as it arrives, and new mail appears on its own.
onboarding-ready-tour = Take a one-minute tour to see where everything is?
# Closes the first-run pages without the tour.
onboarding-skip = Skip for now
onboarding-take-tour = Take the tour

## Asking to send crash reports (on its own and on the first-run pages)

share-title = Help improve Katna
share-lead = When Katna crashes, it saves a report on this computer. Sending these reports helps fix what went wrong. You can change this any time in Settings > User feedback.
share-sent = What is sent
# The log lines are the last lines of Katna's own record of what it did.
share-sent-detail = The crash report as you can view it in Settings: what crashed and where in Katna, the version, your Linux system and desktop, and Katna's last log lines, which can name mail folders.
share-never-sent = What is never sent
share-never-sent-detail = Your messages, contacts, passwords, IP address, user name or computer name. Email addresses are removed from the report.
share-where = Where it goes
# Sentry is the name of the crash tracker; EU is the European Union.
share-where-detail = Katna's crash tracker at Sentry, stored in the EU. No ID ties reports to you.
share-dont-send = Don’t send
share-send = Send crash reports
# Shown briefly after choosing "Send crash reports".
share-sending = Crash reports will be sent. Thank you.
# Shown briefly after choosing "Don’t send".
share-local = Crash reports stay on this computer.

## The tour (cards pointing at each part of the window)

tour-welcome-title = Welcome to Katna Mail
tour-welcome-text = A one-minute tour shows where everything is.
# Closes the tour before it starts.
tour-not-now = Not now
tour-start = Take the tour
# Ends the tour, on its last card.
tour-close = Close
# Ends the tour before its last card.
tour-skip = Skip tour
tour-back = Back
# Ends the tour, on its last card.
tour-done = Done
tour-next = Next
# Which card this is. $step: its number; $total: how many cards there are.
tour-step = { $step } of { $total }
tour-compose-title = Write a message
tour-compose-text = Compose opens a new message at the bottom right, so you can keep reading while you write.
tour-search-title = Search all your mail
tour-search-text = Search works offline too. The button at the right end adds filters: sender, recipient, subject, dates and attachments.
tour-menu-title = Show or hide the folders
# "Mail" is the name of the Mail app's button in the bar at the left.
tour-menu-text = This button folds the folder list away. While it is hidden, rest the pointer on Mail at the left to see the folders.
tour-apps-title = Your apps
tour-apps-text = Mail lives here, beside Calendar, Contacts, Tasks, Notes and Files.
tour-tabs-title = Inbox tabs
# Primary, Promotions, Social, Updates and Forums are the names of the
# Inbox tabs; use the same words as those tabs.
tour-tabs-text = New mail is sorted into Primary, Promotions, Social, Updates and Forums. You can turn the tabs off in quick settings.
tour-list-title = Your messages
tour-list-text = Click a message to read it. Hover it for quick actions, right-click it for more, or tick several to act on them together.
tour-settings-title = Quick settings
tour-settings-text = Change the reading pane, density and theme here. The tour can be started again from there too.
tour-account-title = Your account
tour-account-text = See which account you are in, and add another one.

## Crash notice (a bar at the bottom after a crash)

# The background service (katna-daemon) crashed. $more: how many other
# crash reports are saved besides this one.
crash-daemon = { $more ->
    [0] Katna's background service stopped unexpectedly.
    [one] Katna's background service stopped unexpectedly. One more crash report is saved.
   *[other] Katna's background service stopped unexpectedly. { $more } more crash reports are saved.
}
# The app itself crashed the last time it ran. $more: how many other crash
# reports are saved besides this one.
crash-mail = { $more ->
    [0] Katna Mail closed unexpectedly last time.
    [one] Katna Mail closed unexpectedly last time. One more crash report is saved.
   *[other] Katna Mail closed unexpectedly last time. { $more } more crash reports are saved.
}
crash-view = View report
crash-view-tooltip = Open the report, saved on this computer
crash-copy = Copy report
# Closes the crash notice.
crash-close = Close

## Sign in again (a bar at the bottom when Google or Microsoft stopped
## letting an account in; $provider: Google or Microsoft)

# $address: the account's email address.
sign-in-again-text = { $provider } asks you to sign in to { $address } again.
# Opens the provider's sign-in page in the browser.
sign-in-again-button = Sign in
sign-in-again-tooltip = Open the { $provider } sign-in page in your browser
# In place of the button while the browser page is open.
sign-in-again-waiting = Waiting for your browser…
sign-in-again-close = Close
# Under an account in Calendar, Tasks or Contacts, and on a Drive file in a
# message, when Google has one of its APIs (People API, Google Drive API…)
# switched off in the Google Cloud project Katna signs in with.
google-api-off = { $api } is turned off in Katna's Google Cloud project.
# Opens Google's page that turns the API on.
google-api-turn-on = Turn on
google-api-turn-on-tooltip = Open Google Cloud to turn on { $api }, then press Try again
# Shown briefly after signing in again. $address: the account's email address.
sign-in-again-done = Signed in to { $address } again. Getting your mail…

## Before deleting several conversations, or deleting for good
# $kind: "conversation" or "message", as the list groups mail. $count: how many.

delete-ask-title = { $kind ->
    [conversation] { $count ->
        [one] Move this conversation to Trash?
       *[other] Move { $count } conversations to Trash?
    }
   *[message] { $count ->
        [one] Move this message to Trash?
       *[other] Move { $count } messages to Trash?
    }
}
delete-ask-body = { $count ->
    [one] You can undo it right after, or bring it back from Trash later.
   *[other] You can undo it right after, or bring them back from Trash later.
}
delete-ask-confirm = Move to Trash
delete-forever-title = { $kind ->
    [conversation] { $count ->
        [one] Delete this conversation forever?
       *[other] Delete { $count } conversations forever?
    }
   *[message] { $count ->
        [one] Delete this message forever?
       *[other] Delete { $count } messages forever?
    }
}
delete-forever-body = { $count ->
    [one] It is deleted on the server too. This can’t be undone.
   *[other] They are deleted on the server too. This can’t be undone.
}
delete-forever-confirm = Delete forever
# A tick box in the dialog; the same as the switch in Settings > General.
delete-ask-dont-ask = Don’t ask again
delete-ask-cancel = Cancel
