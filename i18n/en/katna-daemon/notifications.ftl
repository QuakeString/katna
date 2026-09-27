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

## Its buttons

notify-open = Open
# Only on a notification about one message.
notify-reply-all = Reply all
notify-mark-read = Mark as read
# On a notification about several messages.
notify-mark-all-read = Mark all as read
notify-archive = Archive
