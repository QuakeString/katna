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
