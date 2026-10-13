# Katna Mail, English: the Outbox (mail that has not gone out yet) and the
# note when a message is not sent. Guide: i18n/README.md. Keep ids stable;
# change the text freely.

## Why a message has not gone out (one line under its subject)

# The mail server refused it for good. $reason: one of the reasons below.
outbox-not-sent = Not sent because { $reason }.
# The mail server refused it; Katna tries again later by itself.
outbox-retrying = Not sent yet because { $reason }. Katna tries again by itself.
# The account's sign-in ended (Google, Microsoft). $address: the account's
# email address.
outbox-waiting-sign-in = Waiting for you to sign in to { $address } again. It goes out then.
# The mail server refused the saved password. $address: the account's
# email address.
outbox-waiting-password = Waiting for the new password of { $address }. It goes out then.
outbox-waiting-connection = Waiting for a connection. It goes out when you're back online.

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = it has no recipients
outbox-reason-address = an address it's sent to doesn't exist
outbox-reason-too-large = it's too large for the mail server
outbox-reason-blocked = the mail server blocked it
outbox-reason-gone = its copy on this computer is gone
outbox-reason-refused = the mail server refused it

## Buttons and notes

outbox-try-again = Try again
# Takes the message out of the Outbox and opens it to change.
outbox-edit = Edit
outbox-delete = Delete
outbox-deleted = Deleted from the Outbox
outbox-sending-again = Sending again…

# The note at the bottom of the window when a message is not sent.
# $subject: its subject; $reason: one of the reasons above.
outbox-snackbar-not-sent = “{ $subject }” wasn't sent because { $reason }.
# The note's button: opens the Outbox.
outbox-open = Outbox
