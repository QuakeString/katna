# Katna Mail, English: the compose window.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## Compose window: title bar

# The title of a message without a subject yet.
compose-new-message = New Message
# Tooltip: brings a minimized message back to its size.
compose-restore = Restore
compose-minimize = Minimize
compose-exit-full-screen = Exit full screen
# Tooltip: opens the message in a window of its own.
compose-open-window = Open in a new window
# Tooltip of the close button.
compose-save-close = Save and close
# Tooltip in a message's own window: puts it back in the mail window.
compose-back-to-mail = Back to the mail window
# Tooltip in a reply written at the foot of a conversation: opens it in
# the compose window.
compose-pop-out-reply = Pop out reply
# A reply at the foot of a conversation shows its recipients on one line
# until it is clicked. Tooltip of that line: shows the From, To, Cc and
# Bcc rows.
compose-edit-recipients = Edit recipients
# On that line, before the names in Cc and in Bcc.
compose-summary-cc = Cc: { $names }
compose-summary-bcc = Bcc: { $names }
# After the first names of a field with many recipients, once the cursor
# left it: how many more there are. A click shows them all.
compose-more-recipients = { $count } more
# Tooltip of the "…" button under a reply: shows the quoted message.
compose-show-trimmed = Show trimmed content
# The same button once the quoted message shows: hides it again.
compose-hide-trimmed = Hide trimmed content
# The x on the corner of that button: takes the quoted message out.
compose-remove-trimmed = Remove quoted text
compose-trimmed-removed = Quoted text removed

## Recipients and subject

# Labels of the recipient rows, and the links that show the Cc and Bcc
# rows. Cc: carbon copy; Bcc: blind carbon copy, hidden from the others.
compose-to = To
compose-cc = Cc
compose-bcc = Bcc
# Label of the row showing the account the message goes out from.
compose-from = From
# Tooltip of the From row's button: lists the other accounts.
compose-from-choose = Send from another account
# Placeholder of the To field.
compose-recipients = Recipients
# Placeholder of the subject field.
compose-subject = Subject

## Sending (the notes at the bottom of the window)

# When Compose or Reply is pressed while another message is being written.
compose-open-elsewhere = Send or discard the open message first.
# $address: what was typed in a recipient field.
compose-bad-address = “{ $address }” is not an email address.
compose-no-recipients = Add at least one recipient.
# $size: the attachments' total size, such as "31 MB"; $limit: the most
# mail servers take, such as "25 MB".
compose-attachments-too-large = The attachments are { $size }; mail servers take up to { $limit }.
compose-no-account = Add an account to send mail from.
# Schedule send was given a time that has passed.
compose-past-time = Pick a time in the future.
compose-scheduling = Scheduling…
compose-sending = Sending…
# $when: the day and time it goes out, such as "Tomorrow, 8:00 AM".
compose-scheduled = Send scheduled for { $when }
# The message went out and its conversation was archived (Send and archive).
compose-sent-archived = Sent and archived
compose-sent = Message sent
compose-discarded = Draft discarded
# The message was kept in the Drafts folder: on closing, and dim beside the
# title while it is written.
compose-draft-saved = Draft saved
compose-draft-saving = Saving…
# $error: why, such as "The Katna background service is not running."
compose-draft-failed = The draft could not be saved: { $error }
# A draft picked in the Drafts folder could not be read or downloaded.
compose-draft-not-opened = The draft could not be opened.

## Attachments

# The button of the file picker that puts the chosen pictures in the text.
compose-picker-insert = Insert
# The button of the file picker that attaches the chosen files.
compose-picker-attach = Attach
# $name: the file's name; $limit: the most a message can carry, such as "25 MB".
compose-file-too-large = { $name } is too large: a message can carry up to { $limit }.
# Forward could not bring the original files along: the message is not downloaded.
compose-forward-files-missing = The forwarded message's files are not downloaded, so they are not attached.
# An attached file's size, after its name. $size: such as "1.2 MB".
compose-attachment-size = ({ $size })
compose-remove-attachment = Remove attachment
# Above the attachments when there are two or more. $size: such as "18.4 MB".
compose-attachments-total = { $count ->
    [one] { $count } file, { $size }
   *[other] { $count } files, { $size }
}
# Google Drive: files over the mail limit. $name: the first such file;
# $limit: such as "25 MB".
compose-drive-note = { $name } is over { $limit }, so it goes to your Google Drive and the message carries a link.
compose-drive-tip = In your Google Drive; the message carries a link
# On a file's chip while it uploads. $percent: 0 to 100.
compose-drive-uploading = Uploading { $percent }%
# On the chip when the account's sign-in did not allow Google Drive.
compose-drive-allow = Allow Drive
compose-drive-allow-tip = Sign in with Google again to let Katna put large files in your Drive
compose-drive-retry = Try again
compose-drive-sends-when-uploaded = Sending once { $name } is uploaded
compose-drive-not-uploaded = { $name } is not in Google Drive yet
compose-drive-share-failed = Could not share the files in Google Drive: { $error }
compose-drive-share-title = Share the files with everyone?
# $addresses: the recipients, separated by commas.
compose-drive-share-text = { $count ->
    [one] Google Drive can't share the files with { $addresses }, who has no Google account. Anyone with the link can open them instead.
   *[other] Google Drive can't share the files with { $addresses }, who have no Google account. Anyone with the link can open them instead.
}
compose-drive-share-link = Share with link
compose-drive-send-without = Send without sharing
compose-drive-share-cancel = Cancel
# Under a Drive file's link in the sent message. $size: such as "84 MB".
compose-drive-card-detail = { $size } · Google Drive
# The same for OneDrive, for accounts that sign in with Microsoft
# (Outlook.com, Hotmail, Microsoft 365).
compose-onedrive-note = { $name } is over { $limit }, so it goes to your OneDrive and the message carries a link.
compose-onedrive-tip = In your OneDrive; the message carries a link
compose-onedrive-allow = Allow OneDrive
compose-onedrive-allow-tip = Sign in with Microsoft again to let Katna put large files in your OneDrive
compose-onedrive-not-uploaded = { $name } is not in OneDrive yet
compose-onedrive-share-failed = Could not share the files in OneDrive: { $error }
compose-onedrive-share-text = { $count ->
    [one] OneDrive can't share the files with { $addresses }. Anyone with the link can open them instead.
   *[other] OneDrive can't share the files with { $addresses }. Anyone with the link can open them instead.
}
compose-onedrive-card-detail = { $size } · OneDrive
# Shown over the message while files are dragged over it.
compose-drop-files = Drop files here
# Shown over the message while text, cells or a picture from another app
# are dragged over it.
compose-drop-here = Drop here

## Paste options (a small bar under what was just pasted or dropped)

# Pasted text keeps the fonts, colors and lists it was copied with.
compose-paste-keep-formatting = Keep formatting
# Cells copied from a spreadsheet go in as a table.
compose-paste-table = Table
# Cells go in as a picture of them.
compose-paste-picture = Picture
compose-paste-plain-text = Plain text
# A picture shows in the text of the message.
compose-paste-inline = Inline
# A picture goes as an attached file.
compose-paste-attachment = Attachment

## Encryption and signing (the toggles by the recipients)

# Tooltips: Encrypt and Sign turn it on; the longer texts say it is on.
compose-encrypt = Encrypt
compose-encrypted = Encrypted: only the recipients can read it
compose-sign = Sign
compose-signed = Signed: recipients can check it is from you

## Open and click tracking and read receipts (toggles after Sign)

# Tooltips: the short texts turn it on; the longer ones say it is on.
compose-track = Track opens and clicks
compose-tracked = Tracked: you see when each recipient opens it or follows a link
# Plain text has no picture to see opens by: only its links are tracked.
compose-track-clicks = Track link clicks (plain text can't show opens)
compose-tracked-clicks = Tracked: you see when each recipient follows a link
# Shown instead while not signed in; clicking opens Settings > Subscription.
compose-track-sign-in = Sign in to a Katna account to track opens and clicks
compose-receipt = Request a read receipt
compose-receipt-on = Read receipt requested: the recipient's app may ask them to send one
compose-delivery = Request a delivery receipt
compose-delivery-on = Delivery receipt requested: your mail server will email you when each recipient's server accepts it
compose-delivery-unavailable = Your mail server doesn't send delivery receipts

## Spelling

# $language: a language code such as "de_DE". "hunspell-en_us" is the
# name of a package to install; keep it as it is.
spell-no-dictionary = No spelling dictionary for { $language } is installed (for example hunspell-en_us).
# $error: why the dictionary could not be read, from the system.
spell-dictionary-error = Spelling dictionary: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

# A fix that puts other words in place of the marked ones.
grammar-replace = “{ $words }”
# A fix that adds words after the marked ones.
grammar-add = Add “{ $words }”
# A fix that removes the marked words.
grammar-remove = Remove “{ $words }”
grammar-ignore = Ignore

## Send checks (asked before a message goes out)

send-check-attachment-title = Did you mean to attach files?
send-check-attachment-text = You wrote about an attachment, but nothing is attached.
send-check-attach = Attach a file
send-check-subject-title = Send without a subject?
send-check-subject-text = This message has no subject.
send-check-add-subject = Add subject
send-check-send-anyway = Send anyway

## Recipients (To, Cc and Bcc)

# The tooltip of a recipient that is not an email address.
recipient-not-valid = Not a valid email address
# The tooltip of the arrow on a recipient that shows its address.
recipient-show-address = Show address
# The tooltip of the x on a recipient, which takes it out.
recipient-remove = Remove
# Asked when Send finds a recipient that is not an email address.
recipient-bad-title = Check the address
recipient-bad-text = “{ $address }” is not a valid email address. Fix it or remove it before sending.
recipient-bad-fix = Fix it
