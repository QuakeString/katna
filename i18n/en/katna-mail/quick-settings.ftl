# Katna Mail, English: Quick settings.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## Quick settings (the panel that slides in from the right)

quick-title = Quick settings
# A button that opens the full Settings page.
quick-see-all = See all settings
# Section heading (shown in capitals).
quick-reading-pane = Reading pane
# A reading-pane choice: the open mail shows beside the list.
quick-pane-right = Right of the list
# A reading-pane choice: the open mail takes the list's place.
quick-pane-none = No split
# Section heading: how tightly the list's lines are spaced.
quick-density = Density
quick-density-default = Default
quick-density-compact = Compact
# Section heading: light or dark colors.
quick-theme = Mode
# A theme choice: light or dark, following the desktop.
quick-theme-system = System
quick-theme-light = Light
quick-theme-dark = Dark
quick-desktop-colors = Desktop colors
quick-desktop-colors-detail = The color scheme and accent color of the desktop
# Names of the apps (Mail, Calendar, …) under their icons in the rail.
quick-app-names = App names
quick-app-names-detail = Names under the app icons at the far left
quick-inbox-tabs = Inbox tabs
quick-inbox-tabs-detail = The tabs of each account's mail provider
# Opens Settings > Inbox, where each account's tabs are chosen.
quick-choose-tabs = Choose tabs
quick-choose-tabs-detail = Per account, in Settings
# Section heading.
quick-sending = Sending
# How long a sent message waits, so it can be taken back.
quick-undo-send = Undo send
# The Undo send choice that turns it off.
quick-undo-send-off = Off
# An Undo send choice: $seconds is a number of seconds; "s" is the unit's symbol.
quick-undo-send-seconds = { $seconds } s
quick-signatures = Signatures
# Under "Signatures" when there are none.
quick-signatures-none = None yet
# Under "Signatures" when there is one. $name: the signature's name.
quick-signatures-one = { $name }, used by default
# Under "Signatures". $count: how many; $name: the one used by default.
quick-signatures-many = { $count ->
    [one] { $count } signatures; { $name } by default
   *[other] { $count } signatures; { $name } by default
}
# Under "Signatures" when none is chosen as the default. $count: how many there are.
quick-signatures-no-default = { $count ->
    [one] { $count }, none by default
   *[other] { $count }, none by default
}
# The name shown for a signature the user left unnamed.
quick-signature-untitled = Untitled
# Section heading: grouping replies into conversations.
quick-threading = Email threading
quick-conversation-view = Conversation view
quick-conversation-view-detail = Group replies to the same mail
# Section heading.
quick-help = Help
# Starts the guided tour of the app.
quick-tour = Take the tour
quick-whats-new = What’s new
quick-check-updates = Check for updates
quick-about = About Katna
