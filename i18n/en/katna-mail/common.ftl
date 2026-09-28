# Katna Mail, English: language picker, dates and sizes, top bar.
# Guide: i18n/README.md. Keep ids stable; change the text freely.
# In plural forms, write { $count } rather than the digit, so languages
# with their own digits show them.

## Language picker (top bar and Settings > General)

# The top bar button's tooltip. $language: the language's own name.
language-tooltip = Language: { $language }
language-tooltip-system = Language: { $language }, following the system
language-search = Search language
language-system-default = System default
# Under "System default": the language it is using now.
language-system-now = Now { $language }
language-no-match = No language matches “{ $query }”
language-machine = Translated by machine. Help improve it
language-setting = Language
language-setting-detail = The language of menus, buttons and messages, and the format of dates and numbers. System default follows the desktop.

## Dates and sizes

ago-just-now = just now
ago-minutes = { $count ->
    [one] { $count } minute ago
   *[other] { $count } minutes ago
}
ago-hours = { $count ->
    [one] { $count } hour ago
   *[other] { $count } hours ago
}
ago-days = { $count ->
    [one] { $count } day ago
   *[other] { $count } days ago
}
size-bytes = { $count ->
    [one] { $count } byte
   *[other] { $count } bytes
}
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = Hide folders
folders-show = Show folders
compose = Compose
search = Search
search-mail = Search mail
# The main search box's placeholder: it finds mail, people and more.
search-awesome = Awesome bar, search anything
search-settings = Search settings
search-clear = Clear search
search-options-show = Show search options
settings = Settings
account-add = Add an account
