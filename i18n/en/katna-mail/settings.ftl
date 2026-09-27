# Katna Mail, English: Settings page.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## Settings page: its tabs

settings-tab-general = General
settings-tab-inbox = Inbox
settings-tab-accounts = Accounts
settings-tab-katna-account = Katna account
# The tab listing newsletters and mailing lists, to unsubscribe from.
settings-tab-subscriptions = Subscription
settings-tab-appearance = Appearance
# The tab of keyboard shortcuts.
settings-tab-shortcuts = Shortcuts
# The tab choosing which app opens each kind of attachment.
settings-tab-default-apps = Default apps
settings-tab-folders-rules = Folders & rules
# The tab with signatures and other settings for writing mail.
settings-tab-compose = Compose
# MCP: the Model Context Protocol, which AI assistants use.
settings-tab-mcp-server = MCP server
settings-tab-feedback = User feedback
settings-tab-experimental = Experimental

## Settings page: tabs still to come

settings-tab-subscriptions-coming = See the newsletters and mailing lists you get, and unsubscribe in one click.
settings-tab-folders-rules-coming = Create, rename, move and hide folders and labels, and choose which ones sync. Rules sort, label, forward or delete new mail by itself, by sender, subject or words.
settings-tab-mcp-server-coming = Let AI assistants on this computer search, read and draft your mail, with your say.

## Settings > General
# The Language row uses language-setting and language-setting-detail.

settings-general-conversations = Conversation view
settings-general-conversations-group = Group replies to the same mail
settings-general-conversations-group-detail = One line per conversation in the list
# The heading of the switches for how an opened conversation shows.
settings-general-reading = Reading
settings-general-newest-first = Newest message first
settings-general-newest-first-detail = A conversation starts with its latest reply
settings-general-full-headers = Show full headers
settings-general-full-headers-detail = From, to, cc, date and subject open on every message
settings-general-full-names = Full names of recipients
# An example of the recipients line with and without full names.
settings-general-full-names-detail = “to me, Ada Lovelace” rather than “to me, Ada”
settings-general-mark-read = Mark as read
settings-general-mark-read-now = As soon as it opens
settings-general-mark-read-1s = After it is open for 1 second
settings-general-mark-read-3s = After it is open for 3 seconds
settings-general-mark-read-never = Only when I mark it read
# Gmail's name for the setting.
settings-general-auto-advance = Auto-advance
settings-general-auto-advance-detail = After you delete, archive or move the open conversation
settings-general-auto-advance-next = Open the next conversation
settings-general-auto-advance-previous = Open the previous conversation
settings-general-auto-advance-list = Go back to the list
settings-general-reply-button = Reply button
settings-general-reply-all = Reply to everyone
settings-general-reply-all-detail = The reply button beside each message replies to all, not only the sender
settings-general-remote-images = Images from the web
settings-general-remote-images-detail = Loading a message's images tells its sender that you opened it, when, and roughly where. Off, each message asks first, and you can always show a sender's images.
settings-general-remote-images-always = Always show images
settings-general-remote-images-always-detail = In every message, not only from senders you trust
# The row of the undo-send delay.
settings-general-sending = Sending
settings-general-sending-detail = How long a sent message waits, so it can be taken back.
settings-general-offline = Offline mail
settings-general-offline-detail = Recent mail is downloaded whole, to read without a connection. Older mail downloads when you open it.
# A choice of how much mail is kept for offline reading.
settings-general-offline-days = { $count ->
    [one] { $count } day
   *[other] { $count } days
}
settings-general-offline-years = { $count ->
    [one] { $count } year
   *[other] { $count } years
}
settings-general-offline-all = All mail
settings-general-offline-note = Choosing fewer days keeps mail already downloaded. Nothing changes on the server.
settings-general-notifications = Notifications
settings-general-notifications-detail = For new mail in the Inbox, even while Katna Mail is closed.
settings-general-new-mail = Notify me about new mail
# The buttons a new-mail notification has.
settings-general-new-mail-detail = With Reply all, Mark as read and Archive
settings-general-new-mail-sound = Play a sound
settings-general-new-mail-sound-detail = The desktop's new-mail sound
settings-general-reset-cache = Reset cache
settings-general-reset-cache-detail = When mail looks wrong or out of date, or to free disk space. Nothing changes on your mail servers.
# The row of settings about the desktop: login, tray and taskbar.
settings-general-desktop = Desktop
settings-general-start-at-login = Start Katna at login
settings-general-start-at-login-detail = Syncs mail and shows new-mail notifications and the tray icon, without opening the window
# Under "Start Katna at login", while it is on.
settings-general-login-window = Open the Katna Mail window too
settings-general-login-window-detail = The window opens at login as well
settings-general-tray = Show Katna in the system tray
settings-general-tray-detail = With the unread count and a menu
settings-general-unread-badge = Unread count on the taskbar icon
settings-general-unread-badge-detail = How many Inbox messages are unread
# The row saying which app opens email (mailto:) links.
settings-general-mail-app = Default mail app
settings-general-mail-app-detail = Email links in other apps and on websites open a new message here.
mail-app-is-default = Katna Mail is your default mail app.
mail-app-is-other = Email links open in another app.
mail-app-make-default = Make default
mail-app-make-default-failed = Couldn't change the default mail app.

## Settings > Inbox

settings-inbox-tabs = Inbox tabs
settings-inbox-tabs-detail = Sort the inbox into tabs, as your mail provider's website does.
settings-inbox-tabs-show = Show inbox tabs
settings-inbox-tabs-show-detail = Off shows one list for every account
settings-inbox-no-accounts = Add an account to choose its tabs.
# $tabs: the tabs of that style, such as "Focused and Other". $provider: the mail provider, such as "Gmail".
settings-inbox-tabs-automatic = Automatic: { $tabs } ({ $provider })
settings-inbox-tabs-off = No tabs
settings-inbox-tabs-gmail = Primary, Promotions, Social, Updates, Forums
settings-inbox-tabs-focused = Focused and Other
settings-inbox-tabs-zoho = Inbox, Newsletters and Notifications
# $tab: the first tab, such as "Primary".
settings-inbox-tabs-shown = Tabs shown. Mail of a tab you turn off stays in { $tab }.

## Settings > Appearance

settings-appearance-reading-pane = Reading pane
settings-appearance-reading-pane-detail = Where an opened conversation shows.
# A reading pane choice: the opened conversation shows beside the list.
settings-appearance-pane-right = Right of the list
# A reading pane choice: the opened conversation takes the list's place.
settings-appearance-pane-none = No split
# How close together the lines of the list are.
settings-appearance-density = Density
settings-appearance-density-default = Default
settings-appearance-density-compact = Compact
settings-appearance-scaling = Scaling
settings-appearance-scaling-detail = Makes everything in Katna Mail bigger or smaller, on top of the desktop's own scale: text, icons, spacing and dividers. Mail you send keeps its own font size. Very small sizes can make icons hard to click.
settings-appearance-theme = Theme
# A theme choice: light or dark, following the desktop.
settings-appearance-theme-system = System
settings-appearance-theme-light = Light
settings-appearance-theme-dark = Dark
settings-appearance-desktop-colors = Desktop colors
settings-appearance-desktop-colors-use = Use the desktop's colors
settings-appearance-desktop-colors-use-detail = The color scheme and accent color of the desktop
settings-appearance-app-names = App names
settings-appearance-app-names-show = Show app names
settings-appearance-app-names-show-detail = Names under the app icons at the far left
settings-appearance-sender-pictures = Sender pictures
settings-appearance-sender-pictures-show = Show company logos
settings-appearance-sender-pictures-show-detail = Looked up by the sender's domain, never by message, and kept for a week
settings-appearance-important = Important markers
settings-appearance-important-show = Show Important markers
settings-appearance-important-show-detail = Beside each message in the list
settings-appearance-message-width = Message width
settings-appearance-message-width-limit = Limit the width of messages
settings-appearance-message-width-limit-detail = Long lines are easier to read in a wide window
settings-appearance-mail-colors = Mail colors
settings-appearance-mail-colors-detail = Most mail is designed for a white page. With a dark theme its colors are changed to dark ones that read well; off, it keeps its sender's colors on a light page.
settings-appearance-dark-mail = Dark colors for mail too
settings-appearance-dark-mail-detail = Only while the theme is dark
settings-appearance-attachment-previews = Attachment previews
settings-appearance-attachment-previews-show = Show previews of attachments
settings-appearance-attachment-previews-show-detail = A small picture of each file's content on its card

## Settings > Default apps

settings-default-apps-intro = Where attachments open when you click them. The viewer can always open a file in another app too. The desktop's default apps are set in its own settings.
settings-default-apps-pdf = PDF files
settings-default-apps-pdf-detail = Pages, with zoom.
settings-default-apps-pictures = Pictures
settings-default-apps-pictures-detail = Photos (turned upright), PNG, GIF, WebP, BMP, TIFF and SVG.
settings-default-apps-text = Text files
settings-default-apps-text-detail = Plain text, logs, code and other text.
settings-default-apps-sheets = Spreadsheets
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) and CSV.
settings-default-apps-documents = Documents
settings-default-apps-documents-detail = Word (docx, doc), OpenDocument text (odt) and slides (pptx, ppt, odp).
settings-default-apps-katna = Katna Mail's viewer
settings-default-apps-system = The desktop's default app
settings-default-apps-ask = Ask which app each time
settings-default-apps-after-saving = After saving
settings-default-apps-show-folder = Show saved files in their folder
settings-default-apps-show-folder-detail = Opens the file manager with the saved attachments picked

## Settings > Compose

settings-compose-send-from = Send new messages from
settings-compose-send-from-detail = New messages start from this account; the From row picks another. Replies and forwards always go out from the account they answer.
# The choice to send new mail from whichever account is open.
settings-compose-send-from-current = The account you are in
settings-compose-send-on-replies = Send on replies
settings-compose-send-on-replies-detail = What Send does on a reply or forward. The menu beside Send offers the other.
# The choice of what the Send button does on a reply: only send.
settings-compose-send-plain = Send
# The choice of what the Send button does on a reply: send, then archive the conversation.
settings-compose-send-archive = Send and archive
settings-compose-signatures = Signatures
settings-compose-signatures-detail = Added below your message, after a “--” line. Pick another one in the compose window.
# A signature, or another thing, without a name.
settings-compose-untitled = Untitled
# The placeholder of a signature's name.
settings-compose-signature-name = Name, such as Work
# The name given to the first signature made.
settings-compose-signature-first = My signature
# The name given to a new signature. $number: how many there are with it.
settings-compose-signature-numbered = Signature { $number }
settings-compose-signature-delete = Delete
settings-compose-signature-deleted = Signature deleted
# The button that makes a new signature.
settings-compose-signature-new = Create new
settings-compose-no-signatures = No signatures yet.
settings-compose-no-signature = No signature
settings-compose-for-new-mail = For new mail
settings-compose-for-replies = For replies and forwards
settings-compose-for-replies-detail = In a conversation where you signed a message, a reply starts with that signature instead.
# The row choosing between plain text and formatted mail.
settings-compose-format = Format
settings-compose-plain-text = Write in plain text
settings-compose-plain-text-detail = New mail starts without formatting; the compose window can switch
settings-compose-spelling = Spelling
settings-compose-spell-check = Check spelling while I write
settings-compose-spell-check-detail = Misspelled words are underlined, with suggestions on right-click
# The spelling dictionary of the desktop's language. $language: its code, such as "en_US".
settings-compose-spell-desktop = Desktop's language ({ $language })
# Harper: the grammar checker's name.
settings-compose-grammar = Grammar
settings-compose-grammar-detail = Checked on this computer with Harper. English only for now: text in other languages is left alone.
settings-compose-grammar-check = Check grammar
settings-compose-grammar-check-detail = Underline grammar mistakes while writing, in English
settings-compose-suggestions = Writing suggestions
settings-compose-suggestions-detail = Learned on this computer from the mail you sent and the mail you are answering; nothing leaves it. Press Tab to take a suggestion, or keep typing.
settings-compose-suggestions-on = Suggest while writing
settings-compose-suggestions-on-detail = Show the likely rest of a phrase in grey as you type
settings-compose-templates = Templates
settings-compose-templates-detail = Save mail you write often, and start new mail or a reply from it.

## Settings > Shortcuts

settings-shortcuts-set = Shortcut set
settings-shortcuts-set-detail = Start from the keys of a mail app you know. Cmd is Ctrl here. Your own changes stay on top of the set, and Restore defaults goes back to the set's keys.
settings-shortcuts-single = Single-key shortcuts
settings-shortcuts-single-detail = Keys without Ctrl or Alt, as in webmail: e archives, j and k move, / searches. They work in the list and the open conversation, never while typing.
settings-shortcuts-single-use = Use single-key shortcuts
settings-shortcuts-single-use-detail = Ctrl shortcuts always work
settings-shortcuts-how = Click a key to change it, or + to add one, then press the new keys. Esc cancels.
settings-shortcuts-restore = Restore defaults
# Beside a shortcut that has no key.
settings-shortcuts-no-key = No key
# While recording a shortcut's keys.
settings-shortcuts-press = Press keys…
# While recording a shortcut of two keys in a row, after the first. $keys: the first key.
settings-shortcuts-then = { $keys } then…
# $keys: the keys pressed. $action: the shortcut they now do. $previous: the shortcut they did before.
settings-shortcuts-moved = { $keys } now does “{ $action }” instead of “{ $previous }”.
settings-shortcuts-single-off = Single-key shortcuts are off, so this key works once they are on.
settings-shortcuts-restored = Every shortcut has its set's keys again.

## Settings search: the line under a result

settings-general-language-summary = Language of the app, dates and numbers
settings-general-reading-summary = Newest message first, full headers, full names of recipients
settings-general-mark-read-summary = When an opened conversation is marked read: at once, after 1 or 3 seconds, or by hand
settings-general-auto-advance-summary = What opens after you delete, archive or move the open conversation: the next one, the previous one, or the list
settings-general-reply-button-summary = The reply button beside each message replies to everyone
settings-general-remote-images-summary = Always show the images of every message
settings-general-sending-summary = Undo send: how long a sent message waits, so it can be taken back
settings-general-offline-summary = How many days of recent mail are downloaded whole, to read without a connection
settings-general-notifications-summary = New-mail notifications and their sound
settings-general-reset-cache-summary = Delete downloaded mail, sender pictures and the search index, and download them again
settings-general-desktop-summary = Start Katna at login, the system tray icon and the unread count on the taskbar icon
settings-accounts-accounts-summary = Add or remove an account, or change its picture
settings-appearance-density-summary = Default or compact lines in the list
settings-appearance-scaling-summary = Make everything bigger or smaller: text, icons, spacing and dividers
settings-appearance-theme-summary = System, light or dark
settings-appearance-sender-pictures-summary = Company logos, looked up by the sender's domain
settings-appearance-important-summary = The Important marker beside each message in the list
settings-appearance-mail-colors-summary = Dark colors for HTML mail in a dark theme, or its sender's colors
settings-appearance-attachment-previews-summary = A small picture of each attachment's content
settings-shortcuts-set-summary = Start from the keys of Gmail, Inbox by Gmail, Apple Mail, Outlook or Thunderbird
settings-shortcuts-single-summary = Keys without Ctrl or Alt, as in webmail
settings-default-apps-pdf-summary = Where PDF attachments open
settings-default-apps-pictures-summary = Where photos and pictures open
settings-default-apps-text-summary = Where plain text, logs and code open
settings-default-apps-sheets-summary = Where Excel, OpenDocument and CSV files open
settings-default-apps-documents-summary = Where Word and OpenDocument text and slides open
settings-default-apps-after-saving-summary = Show saved attachments in their folder
settings-compose-send-from-summary = The account new mail goes out from: the first one, another one, or the one you are in
settings-compose-send-on-replies-summary = Send, or Send and archive the conversation, on replies and forwards
settings-compose-signatures-summary = Added below your message, after a “--” line
settings-compose-for-new-mail-summary = The signature new mail starts with
settings-compose-for-replies-summary = The signature replies and forwards start with
settings-compose-format-summary = Write new mail in plain text
settings-compose-spelling-summary = Check spelling while writing, and the dictionary's language
settings-general-mail-app-summary = Open email links from other apps and websites in Katna Mail
settings-compose-grammar-summary = Underline grammar mistakes while writing, in English
settings-compose-suggestions-summary = Show the likely rest of a phrase in grey as you type
settings-compose-templates-summary = Coming soon: save mail you write often, and start new mail or a reply from it
settings-feedback-crash-reports-summary = Save crash reports on this computer when Katna Mail or its background service crashes
settings-feedback-saved-summary = View, copy or delete the crash reports saved on this computer
settings-feedback-help-improve-summary = Send crash reports to help fix what went wrong; off unless you turn it on
settings-experimental-blur-summary = The desktop shows through the top bar, blurred, and menus are frosted
# The line under a keyboard shortcut found by the settings search.
settings-search-shortcut = Keyboard shortcut
# The line under a tab of the Settings page found by the search.
settings-search-tab = Settings tab
# $query: what was typed in the search box.
settings-search-none = No settings match “{ $query }”.
settings-search-results = Settings that match “{ $query }”

## Settings: starting at login

# $error: the system's error, in English.
settings-open-at-login-failed = Could not change starting at login: { $error }

## Settings > General > Time

settings-time = Time
settings-clock-language = As the language writes it
settings-clock-12 = 12-hour, like 2:05 PM
settings-clock-24 = 24-hour, like 14:05
# The line under "Time" in a settings search result.
settings-time-summary = 12-hour or 24-hour clock, or as the language writes it
