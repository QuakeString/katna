# Katna Mail, English: unsubscribing from mailing lists and newsletters
# (the chip after an open mail's sender, Unsubscribe on a mail line's
# right-click menu, and the note that says how it went).
# Guide: i18n/README.md. Keep ids stable; change the text freely.

## The chip after the sender's address in an open mail

unsubscribe-chip = Unsubscribe
# While it is being done in the background.
unsubscribe-working = Unsubscribing…
# Once done.
unsubscribe-done-chip = Unsubscribed

## The right-click menu of mail lines

unsubscribe-menu = Unsubscribe

## The note at the bottom of the window

# { $name } is the sender's name.
unsubscribe-done-one = Unsubscribed from { $name }
unsubscribe-done-many = Unsubscribed from { $count } senders
# Some were done; { $left } have only a web page or did not answer.
unsubscribe-some = { $left ->
    [one] Unsubscribed from { $count }. 1 needs its web page
   *[other] Unsubscribed from { $count }. { $left } need their web pages
}
# The sender gives no way to unsubscribe without its web page.
unsubscribe-needs-page = { $name } only unsubscribes on its web page
unsubscribe-failed = Couldn't unsubscribe from { $name }
unsubscribe-none-many = { $count } senders need their web pages to unsubscribe
# Nothing in the chosen mail says how to unsubscribe.
unsubscribe-nothing = This mail has no way to unsubscribe
unsubscribe-no-way = { $count ->
    [one] This sender gives no way to unsubscribe
   *[other] These { $count } senders give no way to unsubscribe
}
# The note's button: opens the list's page in the browser.
unsubscribe-open-page = Open page
unsubscribe-open-pages = Open pages
