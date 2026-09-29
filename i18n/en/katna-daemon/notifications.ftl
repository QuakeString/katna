# Katna service, English: new-mail notifications.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

# The title of one notification about several new messages.
notify-new-emails = { $count ->
    [one] { $count } new email
   *[other] { $count } new emails
}
# The last line of that notification, under the first few messages:
# how many more are not listed.
notify-and-more = { $count ->
    [one] and { $count } more
   *[other] and { $count } more
}
# A message without a subject.
notify-no-subject = (no subject)
# The title of a notification about a message with no sender.
notify-unknown-sender = Unknown sender

## Reminders the user asked for (same buttons)

# The title of a notification about snoozed mail that is back in the inbox.
# Under it, one line per conversation: "Sender: Subject".
notify-snooze-back = Back from snooze
# The title of a follow-up reminder: nobody answered a message the user
# sent and asked to be reminded about.
notify-no-reply = No reply yet
# Under it. $subject: the subject of the sent message.
notify-no-reply-to = Nobody has replied to “{ $subject }”.

## Open and click tracking (only for mail sent with "Track opens and clicks")

# The title when a recipient first opens a tracked message. $who is the
# recipient's name or address, $subject the message's subject.
notify-tracking-opened = { $who } opened { $subject }
# The title when a recipient first follows a link in a tracked message;
# the text under it is the link.
notify-tracking-clicked = { $who } clicked a link in { $subject }

## An update of Katna is downloaded and ready to install

# The title.
notify-update-ready = Katna Mail can be updated
# Under it. $version: the new version, such as 0.0.0.r236.g1a2b3c4.
notify-update-ready-body = Version { $version } is downloaded. Update installs it and restarts Katna Mail.
# Its button: shows the update in Katna Mail, ready to install.
notify-update = Update

## Reminders of calendar events

# Under the event's title: how soon it starts.
notify-event-now = Now
notify-event-in-minutes = { $count ->
    [one] In 1 minute
   *[other] In { $count } minutes
}
notify-event-in-hours = { $count ->
    [one] In 1 hour
   *[other] In { $count } hours
}
notify-event-in-days = { $count ->
    [one] Tomorrow
   *[other] In { $count } days
}
# For an event that lasts all day.
notify-event-all-day = All day
# Its buttons: open the video call, and remind again in 5 minutes.
notify-event-join = Join
notify-event-snooze = Snooze 5 min

## The buttons of new-mail notifications and reminders

notify-open = Open
# Only on a notification about one message.
notify-reply-all = Reply all
notify-mark-read = Mark as read
# On a notification about several messages.
notify-mark-all-read = Mark all as read
notify-archive = Archive
