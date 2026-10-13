# Katna service, English: the system tray icon.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## The tray icon's menu
# "_" marks the mnemonic: the letter after it is underlined and pressed with Alt.
# Keep exactly one "_" in each, before a letter of your translation, and a
# different letter in each.

tray-open-inbox = Open _Inbox
tray-new-message = _New Message
# Opens the quick capture card, on Task: a small card over whatever is on
# screen for writing a new task in one line.
tray-new-task = New _task
# Opens the quick capture card on Note.
tray-new-note = New n_ote
# Opens Katna Mail's Settings page.
tray-preferences = _Preferences
# Closes Katna Mail and stops Katna until the next login.
tray-quit = _Quit

## The tray icon's tooltip, under "Katna Mail"

# How many messages in the Inbox are unread.
tray-unread = { $count ->
    [0] No unread mail
    [one] { $count } unread message
   *[other] { $count } unread messages
}

# Lines under that one, while something needs the user.
tray-password-refused = New password needed for { $address }
tray-signed-out = Sign in again to { $address }
# From three accounts.
tray-accounts-need-you = { $count } accounts need you
tray-not-sent = { $count ->
    [one] { $count } message wasn't sent
   *[other] { $count } messages weren't sent
}
