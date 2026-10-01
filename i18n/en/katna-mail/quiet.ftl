# Katna Mail, English: muting (the bell on the list's toolbar, Mute… on a
# folder's or an account's right-click menu, and its notes).
# Guide: i18n/README.md. Keep ids stable; change the text freely.

## The bell on the list's toolbar, for the open folder or inbox tab

# Tooltip while new mail here notifies and counts on the taskbar.
quiet-tip-rings = Mute notifications
# Tooltip while it does not; clicking turns it back on.
quiet-tip-off = Muted. Click to notify again
# Tooltip while it is muted for a while; { $when } is a date and time.
quiet-tip-muted-until = Muted until { $when }. Click to notify again

## The right-click menu of a folder or an account

quiet-mute = Mute…
quiet-unmute = Unmute
# For a folder that does not notify, such as Spam.
quiet-notify = Notify for new mail

## How long to mute

quiet-for-hour = For 1 hour
quiet-until-tomorrow = Until tomorrow morning
quiet-until-unmuted = Until I turn it back on
quiet-turn-on = Notify for new mail

## The note after a change, with Undo; { $name } is a folder, tab or account

quiet-off = { $name } muted
quiet-muted-until = { $name } muted until { $when }
quiet-on = { $name } notifies again

## Conversations and senders (the reading pane's bell, the More menus and
## the right-click menu)

quiet-mute-conversation = Mute conversation
quiet-unmute-conversation = Unmute conversation
quiet-mute-sender = Mute sender
quiet-unmute-sender = Unmute sender
# Tooltip of the crossed bell on a muted conversation's line.
quiet-row-muted = Muted
# Over a muted conversation in the reading pane.
quiet-conversation-strip = Muted. New replies won't notify or count.
quiet-conversation-muted = { $count ->
    [one] Conversation muted
   *[other] { $count } conversations muted
}
quiet-conversation-unmuted = { $count ->
    [one] Conversation unmuted
   *[other] { $count } conversations unmuted
}
quiet-sender-muted = Mail from { $address } muted
quiet-sender-unmuted = Mail from { $address } notifies again
