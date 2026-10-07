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
# The title when Katna sent a follow-up for the user because nobody
# replied. Under it: notify-follow-up-sent-to.
notify-follow-up-sent = Follow-up sent
# $subject: the subject of the sent message.
notify-follow-up-sent-to = Nobody had replied to “{ $subject }”, so Katna followed up.
# The title when a follow-up fell due while the computer was off: Katna
# did not send it late, and the conversation is back in the Inbox.
notify-follow-up-waiting = Follow-up not sent
# Under it. $subject: the subject of the sent message.
notify-follow-up-waiting-to = It was due while this computer was off. “{ $subject }” is back in your Inbox.

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

## Something needs the user, shown once per problem

# A Google or Microsoft sign-in ended; mail stopped syncing.
notify-signed-out = Sign in again
# Under it. $provider: like "Google".
notify-signed-out-body = { $provider } signed Katna out of { $address }. Mail stopped syncing.
# Its button: opens Katna Mail's sign-in for the account.
notify-sign-in = Sign in
# The server refused the saved password (it was changed, or an app
# password was removed).
notify-password-refused = Password refused
notify-password-refused-body = The mail server refused the password for { $address }. It may have changed.
# Its button: opens Katna Mail's New password card for the account.
notify-new-password = New password
# A message the server refused for good. $subject: its subject.
notify-not-sent = “{ $subject }” wasn't sent
notify-not-sent-no-subject = A message wasn't sent
notify-not-sent-body = It's in the Outbox, which says why.
# Its button: opens Katna Mail's Outbox.
notify-open-outbox = Open Outbox

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
# Button on a task's reminder: ticks the task off.
notify-task-done = Mark as done

## The buttons of new-mail notifications and reminders

notify-open = Open
# Only on a notification about one message: shows more of it in the same
# notification.
notify-peek = Peek
# Only on a notification about one message: a field to type a reply into,
# where the desktop has one, else Katna Mail's reply window.
notify-reply = Reply
# The reply field's grey text before anything is typed. $name: the sender.
notify-reply-placeholder = Reply to { $name }…
# The reply field's button.
notify-send = Send
# Only on a notification about one message.
notify-reply-all = Reply all
notify-mark-read = Mark as read
# On a notification about several messages.
notify-mark-all-read = Mark all as read
notify-archive = Archive
# Only on a notification about one message with a one-time code (a sign-in
# or verification code): copies it. $code: the code, such as 482913.
notify-copy-code = Copy { $code }
# Only on a notification about one message with a link to verify an
# address, confirm something or activate an account: opens the link in the
# browser. $domain: where the link goes, such as accounts.example.com, so
# a link that only looks like a company's shows where it really goes.
notify-link-verify = Verify on { $domain }
notify-link-confirm = Confirm on { $domain }
notify-link-activate = Activate on { $domain }

## After Archive on a notification: a short note in the same place

# Its title. Under it, the subject of the archived message.
notify-archived = Archived
# Under the title when several messages were archived.
notify-archived-count = { $count ->
    [one] { $count } message moved out of the inbox
   *[other] { $count } messages moved out of the inbox
}
# Its button: puts the mail back in the inbox. Also on the note below: keeps
# the reply from going and opens it in Katna Mail to write on.
notify-undo = Undo

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = Code copied
# When the code could not be put on the clipboard: it stays shown, to copy
# by hand.
notify-code-not-copied = Could not copy the code

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

# Its title. Under it, the start of the reply. $name: who it goes to.
notify-reply-sent = Reply sent to { $name }
# Its button: shows the conversation in Katna Mail.
notify-open-in-katna = Open in Katna
