# Katna Mail, English: account problems (a line at the top of the mail
# list, a mark on the account's heading and picture, and the New password
# card). Guide: i18n/README.md. Keep ids stable; change the text freely.

## Lines at the top of the mail list

# Stands for $provider below when the provider is not one Katna knows.
problems-the-server = The mail server

# Google or Microsoft stopped letting Katna in. $provider: Google or
# Microsoft; $address: the account's email address.
problems-signed-out = { $provider } signed Katna out of { $address }. Mail stopped syncing.
# The server refused the saved password (it was changed, or an app password
# was removed). $provider: like "Zoho Mail" or the server's name.
problems-password-refused = { $provider } refused the password for { $address }. It may have changed.
# One server has not answered for half an hour; Katna keeps trying.
problems-no-answer = { $provider } isn't answering for { $address }. Katna keeps trying.
# Every account is unreachable: the computer is offline.
problems-offline = You're offline. Your mail is still here, and mail you send waits until you're back.
# Three or more problems fold into one line.
problems-accounts-need-you = { $count ->
    [one] 1 account needs you
   *[other] { $count } accounts need you
}
# Unfolds the line above into one line per account.
problems-show = Show
# Hides a line for a day.
problems-later = Later
# Opens the New password card.
problems-new-password = New password
# Asks the server again now.
problems-try-again = Try again

## The New password card

problems-password-title = New password
# $provider: like "Zoho Mail"; $address: the account's email address.
problems-password-detail = { $provider } refused the saved password for { $address }. Type the new one; Katna checks it before keeping it.
problems-password-placeholder = Password
problems-password-show = Show password
problems-password-hide = Hide password
problems-password-cancel = Cancel
problems-password-save = Save
# On the Save button while the server checks the password.
problems-password-checking = Checking…
# The server refused the typed password as well.
problems-password-refused-again = { $provider } refused this password too. Check it and try again.
# Snackbar once the new password works.
problems-password-saved = Password saved for { $address }. Getting your mail…

## When a mail server refuses a change for good (a note at the bottom)

# Katna undid the change. $count: how many messages; $address: the
# account's email address.
problems-refused-move = The mail server of { $address } didn't accept moving { $count ->
    [one] a message, so it's back where it was.
   *[other] { $count } messages, so they're back where they were.
}
problems-refused-flags = The mail server of { $address } didn't accept marking { $count ->
    [one] a message (read, starred…), so it's back as it was.
   *[other] { $count } messages (read, starred…), so they're back as they were.
}
problems-refused-label = The mail server of { $address } didn't accept changing the labels of { $count ->
    [one] a message, so it's back as it was.
   *[other] { $count } messages, so they're back as they were.
}
problems-refused-delete = The mail server of { $address } didn't accept deleting { $count ->
    [one] a message, so it's back.
   *[other] { $count } messages, so they're back.
}
# Anything else, or a mix of changes.
problems-refused-other = The mail server of { $address } didn't accept { $count ->
    [one] a change, so Katna put it back as it was.
   *[other] { $count } changes, so Katna put them back as they were.
}
# The note's button: shows the server's own words.
problems-details = Details

## Katna's background service (katna-daemon) isn't running

# A grey line at the top of the list while Katna Mail starts the service,
# shown only when that takes more than a few seconds.
service-starting = Starting Katna's background service…
# An amber line at the top of the list when the service still won't start.
service-failed = Katna's background service won't start, so mail isn't syncing.
# The amber line's and the Details dialog's button: tries to start it again.
service-start-again = Start again
# A note after Katna Mail started the service again, when it had stopped
# while the window was open.
service-started-again = Katna's background service stopped and was started again.
# The Details dialog: what went wrong, ready to copy.
service-details-title = Why the service won't start
service-details-body = Copy this and send it with your report. It has no mail or passwords in it.
service-details-copy = Copy
service-details-close = Close
# Why a change could not be made, when Katna's service is not running or
# did not answer. $error: what went wrong, usually in English.
service-not-running = The Katna background service is not running.
service-no-answer = The Katna background service did not answer: { $error }
# There is no desktop session bus (D-Bus) to reach the service over.
service-no-session = No D-Bus session: { $error }
