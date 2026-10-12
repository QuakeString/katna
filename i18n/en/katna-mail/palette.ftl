# Katna Mail, English: the command palette (Ctrl+Shift+P), one box under
# the top bar that finds any action or setting by name.
# Guide: i18n/README.md. Keep ids stable; change the text freely.

# What the box says before anything is typed.
palette-search = Type a command or a setting
# Heading over what was run from the palette last.
palette-recent = Recent
# Heading over what a search found.
palette-results = { $count ->
    [one] 1 result
   *[other] { $count } results
}
palette-none = Nothing matches “{ $query }”
# Before the name of a folder or page a line goes to: "Go to: Sent".
palette-go-to = Go to:
# Where a setting is, after its name.
palette-setting-place = Settings › { $page }
# On the highlighted line: Enter runs it.
palette-enter = Enter ↵
# Between the keys of a two-key shortcut, such as G then I.
palette-then = then

## The keys along the bottom

palette-move = Move
palette-run = Run
palette-close = Close
