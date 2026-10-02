# Katna Mail, English: reading pane.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## Reading pane: toolbar

reader-close = Close
reader-back = Back
reader-mark-unread = Mark as unread
reader-move-to = Move to
# The ⋮ button that opens more actions.
reader-more = More
# In a dark theme: shows the open mail in its sender's own colors…
reader-original-colors = Show original colors
# …and back in dark colors.
reader-dark-colors = Show in dark colors
# Prints every message of the open conversation.
reader-print-all = Print all
# Opens the conversation in a window of its own.
reader-new-window = In new window
# Where the open conversation is in the list: "3 of 120".
reader-position = { $position } of { $total }
# Go to the newer conversation in the list.
reader-newer = Newer
# Go to the older conversation in the list.
reader-older = Older

## Reading pane: the conversation

# Shown in place of a conversation that was deleted or moved meanwhile.
reader-removed = This conversation was removed.
# The title of a conversation whose messages have no subject.
reader-no-subject = (no subject)
reader-collapse-all = Collapse all
reader-expand-all = Expand all
reader-unknown-sender = (unknown sender)
# A message's date with how long ago it was. $date: the full date; $ago: "2 hours ago".
reader-date-ago = { $date } ({ $ago })
# In place of the date of a reply that has not gone out yet.
reader-sending = Sending…
# Stands for the user's own address among the recipients: "to me, Bob".
reader-me = me
# Under the sender's name. $names: the recipients, separated by commas ("me, Bob").
reader-to = to { $names }
# Before the recipients when each has a delivered or read tick.
reader-to-label = to
reader-tick-delivered = Delivered { $when }
reader-tick-no-bounce = Sent { $when }; no bounce came back, so it most likely arrived
reader-tick-bounced = Not delivered: bounced { $when }
reader-tick-read = Read { $when } (read receipt)
reader-tick-opened = Opened, last { $when } (open tracking)
# Tooltip of the star button on a starred message.
reader-starred = Starred
# Tooltip of the star button on a message that is not starred.
reader-not-starred = Not starred
reader-too-long = The message is too long to show in full.
reader-encrypted-images = Images from the web are never loaded in encrypted mail.
reader-window-failed = Could not open a new window.

## Reading pane: message details (opened from "to me")

reader-details-from = from:
reader-details-to = to:
reader-details-cc = cc:
reader-details-date = date:
reader-details-subject = subject:

## Reading pane: downloading a message

reader-downloading = Downloading this message from the server…
reader-download-failed = Could not download this message.
reader-try-again = Try again

## Reply row

reply-reply = Reply
reply-reply-all = Reply all
reply-forward = Forward

## Encrypted and signed mail

security-decrypting = Decrypting…
security-checking = Checking the signature…
security-partly-encrypted = Only part of this message is encrypted. The rest was added outside the protection and could come from anyone.
security-partly-signed = Only part of this message is signed. The rest was added outside the protection and could come from anyone.
security-encrypted = Encrypted message
security-encrypted-smime = Encrypted message (S/MIME)
security-no-key = Can't decrypt this message: it was encrypted for a key you don't have.
security-cancelled = Decrypting was cancelled.
security-damaged = Can't decrypt this message: the encrypted data is damaged or was changed.
# $tool: the program to install, such as "GnuPG (gpg)".
security-decrypt-unavailable = Can't decrypt this message: install { $tool } to read encrypted mail.
# $reason: the error GnuPG gave, in English.
security-decrypt-failed = Can't decrypt this message: { $reason }
# Takes the place of $signer below when the signer's name is not known.
security-unknown-signer = an unknown signer
# $signer: the signer's name and address, or "an unknown signer".
security-signed-verified = Signed by { $signer } · verified
security-signed-not-sender = Signed by { $signer }, who is not the sender
security-signed-untrusted = Signed by { $signer }, with a key you marked as not trusted
security-signed-unverified = Signed by { $signer } · the key is not verified
security-bad-signature = Bad signature: this message was changed after it was signed, or the signature is forged.
security-signature-expired = Signed by { $signer } · the signature has expired
security-key-expired = Signed by { $signer } · the key has expired since
security-key-revoked = Signed by { $signer } with a key that has been revoked
security-missing-key = Signed with a key you don't have, so it can't be checked
# $key: the end of the key's fingerprint, such as "658C A70C A20C 0FE0".
security-missing-key-id = Signed with a key you don't have ({ $key }), so it can't be checked
# $tool: the program to install, such as "GnuPG (gpg)".
security-signature-unavailable = Signed; install { $tool } to check the signature
security-signature-error = The signature could not be checked.

## Open and click tracking and read receipts (the eye's popover beside a
## sent message's star, and the line above a read receipt)

# $who: a recipient's name or address. $count: how many times.
# $when: the latest time, as the list shows dates ("3:04 PM", "Tue").
tracking-opened = { $who } opened it { $count ->
    [one] once
   *[other] { $count } times
}, last { $when }
# $opens and $clicks: how many times, as $count above.
tracking-opens-clicks = { $who } opened it { $opens ->
    [one] once
   *[other] { $opens } times
} and followed a link { $clicks ->
    [one] once
   *[other] { $clicks } times
}, last { $when }
# Followed a link with pictures turned off, so no open was seen.
tracking-clicked = { $who } followed a link { $clicks ->
    [one] once
   *[other] { $clicks } times
}, last { $when }
# Apple Mail fetches pictures for privacy whether or not the mail is read.
tracking-maybe-opened = { $who } may have opened it (Apple Mail loads pictures for privacy)
# In the eye's popover when no one has opened it or followed a link.
tracking-seen-none = No one has opened it or followed a link yet
# A read receipt came back from $who.
tracking-receipt = { $who } sent a read receipt
# On a read receipt itself.
tracking-receipt-displayed = Read receipt: { $who } opened your message
tracking-receipt-other = Read receipt: { $who } deleted or handled your message without opening it

## Remote images and pictures

remote-hidden = Images in this message are hidden.
# For a sender whose images are always shown, when the mail provider could
# not confirm that the message really comes from that address.
remote-hidden-unconfirmed = Images are hidden: the sender could not be confirmed.
remote-show = Show images
remote-always-show = Always show from this sender
# The button of the file chooser that picks an account's picture.
remote-picture-use = Use
remote-picture-too-big = Pick a picture of 8 MB or less.
remote-picture-type = Pick a PNG, JPEG, GIF, WebP or SVG picture.
# $error: the system's error, in English.
remote-picture-read-failed = Cannot read the picture: { $error }
remote-picture-keep-failed = Cannot keep the picture: { $error }
remote-picture-remove-failed = Cannot remove the picture: { $error }

## Attachments

# Above a message's attachment cards.
attachment-count = { $count ->
    [one] One attachment
   *[other] { $count } attachments
}
# Tooltip of the download button on an attachment card.
attachment-save = Save
attachment-forward = Forward
attachment-save-all = Save all
attachment-save-all-tooltip = Save every attachment to a folder
# The button of the folder chooser that saves every attachment.
attachment-save-here = Save here
attachment-not-downloaded = This message is not downloaded.
attachment-open-message = Open this message to read its attachments.
attachment-not-found = This attachment could not be found in the message.
# $name: the file's name.
attachment-read-failed = Could not read { $name }
# Names an attachment that has no name, by its place in the message.
attachment-numbered = attachment { $number }
# $place: the folder, such as "Downloads".
attachment-saved-all = { $count ->
    [one] Saved { $count } file to { $place }
   *[other] Saved { $count } files to { $place }
}
# $failed: the first file that could not be saved, with the error.
attachment-saved-some = { $total ->
    [one] Saved { $saved } of { $total } file to { $place }. Could not save { $failed }
   *[other] Saved { $saved } of { $total } files to { $place }. Could not save { $failed }
}
# $path: where the file was saved.
attachment-saved-to = Saved to { $path }
# $error: the system's error, in English.
attachment-save-failed = Could not save { $name }: { $error }
attachment-open-failed = Could not open { $name }: { $error }
attachment-risky = This file could run a program, so Katna does not open it. Save it instead.
attachment-encrypted-open = This file came encrypted. Save it to open it elsewhere.

## Printing

# $error: why, in English.
print-failed = Could not print: { $error }
print-no-font = no font was found
print-opened-as-pdf = Opened as a PDF to print from there.

# The print preview, before the desktop's print dialog.
print-preview-title = Print preview
print-preview-laying-out = Laying out the pages…
print-preview-pages = { $count ->
    [one] { $count } page
   *[other] { $count } pages
}
print-preview-more = { $count ->
    [one] and { $count } more page
   *[other] and { $count } more pages
}
print-preview-failed = the pages could not be shown
print-preview-paper = Paper
print-preview-a4 = A4
print-preview-letter = Letter
# How HTML mail prints: as the reading pane shows it, or its text alone.
print-preview-layout = Layout
print-preview-as-shown = As shown
print-preview-simple = Simple text
# Switch: print the background colors of HTML mail (off saves ink).
print-preview-backgrounds = Backgrounds
print-preview-cancel = Cancel
print-preview-print = Print
# In the printed page, in place of a message's text.
print-not-downloaded = (Not downloaded yet.)
print-encrypted = (Encrypted. Open it in Katna Mail to print its text.)
# In the printed page, above a message. $addresses: its recipients.
print-to = To: { $addresses }
print-cc = Cc: { $addresses }

## Message text (right-click menu in the reading pane)

# Text selected in one bubble of a chat, pinned to the top of the chat.
text-pin = Pin to top
# On the right-click menu of an address in a mail's details.
text-copy-address = Copy address
text-copy = Copy
text-select-all = Select all
