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

## Open and click tracking (only for mail sent with "Track opens and clicks")

# The title when a recipient first opens a tracked message. $who is the
# recipient's name or address, $subject the message's subject.
notify-tracking-opened = { $who } opened { $subject }
# The title when a recipient first follows a link in a tracked message;
# the text under it is the link.
notify-tracking-clicked = { $who } clicked a link in { $subject }

## Its buttons

notify-open = Open
# Only on a notification about one message.
notify-reply-all = Reply all
notify-mark-read = Mark as read
# On a notification about several messages.
notify-mark-all-read = Mark all as read
notify-archive = Archive
