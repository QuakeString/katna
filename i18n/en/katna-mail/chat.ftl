# Katna Mail, English: the chat view, a conversation between people shown
# as a group chat with a bubble for each mail (Settings > Experimental).
# Guide: i18n/README.md. Keep ids stable; change the text freely.

# Settings > Experimental: the heading over the chat view's switch.
chat-heading = Reading
chat-view = Conversations as chats
# Under "Conversations as chats".
chat-view-detail = Mail between people reads like a group chat: a bubble for each mail with only what was written, your own on the right. Newsletters keep the usual view.
chat-view-switch = Show conversations as chats
chat-view-switch-detail = The quoted mail and signatures wait behind ··· in each bubble

# The switch in the conversation's header between the chat and the usual view.
chat-switch-chat = Chat
chat-switch-mail = Mail
# Under the subject. $names: the people's first names, such as "Priya, Arjun, Sara"; $count: the mails in the conversation.
chat-people = { $names } and you · { $count ->
    [one] { $count } mail
   *[other] { $count } mails
}
# Over the list of everyone in the chat, opened from its header. $count: the people, the user included.
chat-people-heading = { $count ->
    [one] In this chat · { $count } person
   *[other] In this chat · { $count } people
}
# Beside each person in that list. $count: the mails they sent in this conversation.
chat-member-mails = { $count ->
    [0] No mails
    [one] { $count } mail
   *[other] { $count } mails
}
# The day over the bubbles of that day.
chat-today = Today
chat-yesterday = Yesterday
# Between bubbles, when a mail brings new people in. $who: a first name, or chat-you; $names: the first names of the new people.
chat-added = { $who } added { $names }
# Who added people, when it was the user.
chat-you = You
# A bubble whose mail is not downloaded yet.
chat-not-downloaded = Not downloaded yet
# The forwarded mail inside a bubble.
chat-forwarded = Forwarded
# The ··· pill of a bubble, and what it says when open.
chat-show-quoted = Show the quoted mail and signature
chat-hide-quoted = Hide the quoted mail and signature
chat-hide-dots = Hide ···
# A click on a sender's picture or name.
chat-show-card = Show their card
# The buttons beside a bubble under the pointer, and its right-click menu.
chat-reply-all = Reply to all
chat-more = More
# $name: the sender's first name.
chat-reply-only = Reply to { $name } only
chat-forward = Forward
chat-copy-text = Copy text
chat-show-as-mail = Show as mail
# Beside a reply just sent, while it can still be taken back.
chat-undo = Undo

# The reply box at the foot of the chat. $names: the people's first names.
chat-reply-to = Reply to { $names }
chat-send = Send (Ctrl+Enter)
chat-attach = Attach
chat-attach-photo = Photo
chat-attach-file = File
# Opens the attach picker: files from Katna's Files page.
chat-attach-library = From Files
chat-attach-template = Template
chat-attach-signature = Signature
# Over the reply box when the reply answers an older mail. $name: its sender's first name.
chat-replying-to = Replying to { $name }
# The cross on that strip: the reply answers the newest mail again.
chat-reply-newest = Reply to the newest mail

## The attach picker (paperclip > From Files)

picker-title = Attach from Files
picker-search = Search names, people, subjects
picker-in-chat = IN THIS CONVERSATION
picker-recent = RECENT
picker-preview = Preview
picker-cancel = Cancel
picker-attach = Attach
picker-attach-count = Attach { $count }
picker-selected = { $count } selected
# After the size of everything the mail would carry. $limit: 25 MB.
picker-of-limit = of { $limit }
# $size: how much goes through the cloud rather than in the mail.
picker-via-drive = { $size } via Google Drive
picker-via-onedrive = { $size } via OneDrive
picker-over = { $size }, more than the { $limit } a mail can carry
picker-some-failed = { $count ->
    [one] One file could not be read
   *[other] { $count } files could not be read
}
