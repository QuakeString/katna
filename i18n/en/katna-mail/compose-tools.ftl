# Katna Mail, English: the compose window's bars, menus and dialogs.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## The Send row

# The Send button's tooltip when it also archives the conversation.
compose-tool-send-archive-tip = Send and archive (Ctrl+Enter)
compose-tool-send-tip = Send (Ctrl+Enter)
compose-tool-send = Send
# The arrow beside Send.
compose-tool-send-more = More send options
# The Send menu on a reply, when Send also archives the conversation.
compose-tool-send-without-archiving = Send without archiving
# The Send menu on a reply, when Send does not archive.
compose-tool-send-and-archive = Send and archive
# Shows and hides the formatting bar.
compose-tool-formatting = Formatting options
compose-tool-attach = Attach files
compose-tool-link = Insert link (Ctrl+K)
compose-tool-emoji = Insert emoji
compose-tool-photo = Insert photo
compose-tool-event = Insert calendar event
compose-tool-event-coming = Calendar events come with Katna Calendar.
compose-tool-more = More options
# In a separate compose window: puts the message back in the main window.
compose-tool-dock = Back to the mail window
compose-tool-discard = Discard draft

## The formatting bar

# Shown instead of the formatting buttons.
compose-tool-plain-note = Plain text mode: formatting is off. Turn it on in More options.
compose-tool-undo = Undo (Ctrl+Z)
compose-tool-redo = Redo (Ctrl+Y)
compose-tool-font = Font
compose-tool-size = Size
compose-tool-bold = Bold (Ctrl+B)
compose-tool-italic = Italic (Ctrl+I)
compose-tool-underline = Underline (Ctrl+U)
compose-tool-text-color = Text color
# The colour button: the text colour and the highlight, side by side.
compose-tool-colors = Text and highlight color
compose-tool-background-color = Background color
compose-tool-default-color = Default color
compose-tool-no-background = No background
compose-tool-align = Align
compose-tool-align-left = Align left (Ctrl+Shift+L)
compose-tool-align-center = Align center (Ctrl+Shift+E)
compose-tool-align-right = Align right (Ctrl+Shift+R)
compose-tool-numbered-list = Numbered list (Ctrl+Shift+7)
compose-tool-bulleted-list = Bulleted list (Ctrl+Shift+8)
compose-tool-indent-less = Indent less (Ctrl+[)
compose-tool-indent-more = Indent more (Ctrl+])
# Makes the paragraph read left to right (English, say). Ctrl+Shift+X
# turns it the other way.
compose-tool-ltr = Left-to-right (Ctrl+Shift+X)
# Makes the paragraph read right to left (Arabic, Hebrew, Persian, Urdu).
compose-tool-rtl = Right-to-left (Ctrl+Shift+X)
# Makes the paragraph a quote.
compose-tool-quote = Quote (Ctrl+Shift+9)
compose-tool-strikethrough = Strikethrough (Alt+Shift+5)
compose-tool-remove-formatting = Remove formatting (Ctrl+\)
# The table button when the cursor is in a table.
compose-tool-table = Table
compose-tool-insert-table = Insert table
# The table size under the pointer in the grid of a new table.
compose-tool-table-size = { $columns } × { $rows } table
# On a narrow bar, the buttons that do not fit.
compose-tool-more-formatting = More formatting options

## Changing a table

compose-tool-row-above = Insert row above
compose-tool-row-below = Insert row below
compose-tool-column-left = Insert column left
compose-tool-column-right = Insert column right
compose-tool-delete-row = Delete row
compose-tool-delete-column = Delete column
compose-tool-delete-table = Delete table

## The emoji picker

compose-tool-emoji-search = Search emoji
# The picker's tabs.
compose-tool-emoji-smileys = Smileys & emotion
compose-tool-emoji-people = People
compose-tool-emoji-animals = Animals & nature
compose-tool-emoji-food = Food & drink
compose-tool-emoji-travel = Travel & places
compose-tool-emoji-activities = Activities
compose-tool-emoji-objects = Objects
compose-tool-emoji-symbols = Symbols
compose-tool-emoji-flags = Flags
compose-tool-emoji-results = Search results
compose-tool-emoji-none = No emoji found

## Links

# The bubble under a link the cursor is in; the address follows.
compose-tool-link-go = Go to link:
compose-tool-link-change = Change
compose-tool-link-remove = Remove
# The link dialog's title for a link that is there; also a right-click item.
compose-tool-edit-link = Edit link
# The link dialog's title for a new link.
compose-tool-insert-link = Insert link
compose-tool-link-text = Text to display
compose-tool-link-address = Web address or email address
compose-tool-link-hint = Not sure where to link? Search for the page, then copy its address.
compose-tool-ok = OK
compose-tool-cancel = Cancel

## The right-click menu

# A misspelled word that the dictionary has nothing like.
compose-tool-no-suggestions = No suggestions
compose-tool-add-to-dictionary = Add to dictionary
# $error: why, from the system.
compose-tool-dictionary-failed = Could not save the word: { $error }
compose-tool-cut = Cut
compose-tool-copy = Copy
compose-tool-paste = Paste
compose-tool-select-all = Select all
compose-tool-remove-link = Remove link

## More options

# When on, new messages open full screen.
compose-tool-full-screen = Default to full screen
# Puts a new video call's link (Google Meet, or Jitsi Meet) in the message.
compose-tool-video-call = Add a video call
compose-tool-label = Label
compose-tool-label-coming = Labels on sent mail are coming soon. Label the message in Sent once it is out.
compose-tool-plain-mode = Plain text mode
compose-tool-print = Print
compose-tool-check-spelling = Check spelling
# The printed page's title for a message without a subject.
compose-tool-print-no-subject = (no subject)
# The printed page's header lines; the addresses follow.
compose-tool-print-to = To:
compose-tool-print-cc = Cc:
compose-tool-print-bcc = Bcc:

## Switching to plain text

compose-tool-plain-title = Switch to plain text?
compose-tool-plain-text = Plain text mode removes the formatting, pictures and tables of this message. Its text stays.
compose-tool-plain-switch = Switch

## The signature menu

compose-tool-signature = Insert signature
compose-tool-signature-none = No signature
# A signature without a name.
compose-tool-signature-untitled = Untitled
# Opens Settings at the signatures.
compose-tool-signature-manage = Manage signatures
# The faint tag naming the signature beside it in the text.
compose-signature-tag-tip = Choose another signature
# After choosing another account in From, which starts with its own
# signature. $name: the signature's name.
compose-signature-changed = Signature changed to { $name }
# After choosing another account in From, which starts without a signature.
compose-signature-taken-out = Signature taken out

## The templates menu, and saving a message as a template

compose-tool-templates = Templates
compose-tool-templates-none = No templates yet
compose-tool-template-save = Save as template…
# Opens Settings at the templates.
compose-tool-templates-manage = Manage templates
compose-tool-template-save-title = Save as template
compose-tool-template-save-text = The subject, text and attachments are saved. In the text, {"{"}first name{"}"}, {"{"}name{"}"} and {"{"}my name{"}"} are filled in when you use it. A template with the same name is replaced.
compose-tool-template-name = Template name
compose-tool-template-save-ok = Save
# $name: the template's name.
compose-template-saved = Saved as template “{ $name }”
# $error: why it failed.
compose-template-save-failed = Could not save the template: { $error }
compose-template-open-failed = Could not open the template
