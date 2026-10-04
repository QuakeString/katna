# Katna Mail, English: app rail and app pages.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## Top bar

# The brand before the app's name at the top left: "Katna" + "Mail",
# "Katna" + "Contacts" and so on. Keep it as Katna.
top-brand = Katna

## App rail (and the bottom bar on a phone)

rail-mail = Mail
rail-calendar = Calendar
# Also the heading of the Contacts page.
rail-contacts = Contacts
rail-tasks = Tasks
rail-notes = Notes
# Every attachment of every account in one place.
rail-files = Files

## Rail right-click menu

# $app: the app's name from the rail, such as "Notes".
rail-menu-open = Open { $app }
rail-menu-settings = { $app } settings
rail-menu-turn-off = Turn off { $app }…

## Turning an app off (Settings > Apps)

# The sheet before an app is turned off. $app: its name, such as "Calendar".
app-off-title = Turn off { $app }?
app-off-body = Katna stops syncing { $app } and takes it out of:
app-off-kept = Nothing changes on your accounts. Turn { $app } on again in Settings › Apps whenever you like.
app-off-cancel = Cancel
app-off-confirm = Turn off
# The snackbar after, with Undo.
app-off-done = { $app } turned off
# When a key, a launcher or a link leads to an app that is off; the button turns it on.
app-off-note = { $app } is off
app-off-turn-on = Turn on
# Where an app turned off leaves, one line each.
app-off-leaves-calendar-rail = The rail and Ctrl+2
app-off-leaves-calendar-agenda = The agenda beside your mail
app-off-leaves-calendar-meeting = Schedule meeting, and Open in Calendar on invitations
app-off-leaves-calendar-reminders = Event reminders
app-off-leaves-calendar-desktop = Events in KRunner and the desktop clock
app-off-leaves-contacts-rail = The rail and Ctrl+3
app-off-leaves-contacts-card = Add to contacts on a sender's card
app-off-leaves-contacts-birthdays = Birthdays in Calendar
app-off-leaves-tasks-rail = The rail and Ctrl+4
app-off-leaves-tasks-mail = Add to Tasks on mail, and Shift+T
app-off-leaves-tasks-calendar = Tasks in Calendar
app-off-leaves-tasks-tray = New task in the tray, and Meta+Alt+T
app-off-leaves-tasks-reminders = Task reminders
app-off-leaves-notes-rail = The rail and Ctrl+5
app-off-leaves-notes-mail = Add a note on mail
app-off-leaves-notes-meetings = Meeting notes on events
app-off-leaves-notes-tray = New note in the tray, and Meta+Alt+N
app-off-leaves-notes-reminders = Note reminders
app-off-leaves-files-rail = The rail and Ctrl+7
app-off-leaves-files-compose = Files when attaching in Compose

## Pages of apps still to come

# The page's heading. $app: the app's name from the rail, such as "Calendar".
app-page-title = Katna { $app }
app-coming-soon = Coming soon
app-calendar-promise = Your CalDAV calendars, meeting invitations from your mail and reminders, next to your inbox.
app-tasks-promise = To-do lists that sync with CalDAV, and tasks made from mail.
app-notes-promise = Quick notes, and notes on a mail or conversation for later.

## Contacts page

app-contacts-loading = Gathering people from your mail…
app-contacts-empty = People you write with show up here.
# Under the heading, when every person is listed.
app-contacts-count = { $count ->
    [one] { $count } person from your mail, most written with first
   *[other] { $count } people from your mail, most written with first
}
# Under the heading, when the list stops at its limit.
app-contacts-top = { $count ->
    [one] The top { $count } person from your mail, most written with first
   *[other] The top { $count } people from your mail, most written with first
}
# How many messages there are with one person.
app-contacts-messages = { $count ->
    [one] { $count } message
   *[other] { $count } messages
}
# When you last wrote with a person. $date: a time or date, such as "14:05" or "Sep 3".
app-contacts-last = last { $date }
