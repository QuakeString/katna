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
