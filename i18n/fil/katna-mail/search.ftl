# Katna Mail, Filipino (Filipino).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Search options (the panel from the button at the right of the search box)

search-options = Mga opsyon sa paghahanap
search-options-close = Isara
search-from = Mula kay
search-to = Para kay
search-subject = Subject
search-has-words = May mga salitang
search-without = Walang mga salitang
search-date-within = Petsa sa loob ng
search-has-attachment = May attachment
search-attachment-custom = Custom
search-attachment-image = Larawan
search-attachment-custom-hint = Mag-type ng extension, gaya ng png, tapos Space
search-attachment-remove = Alisin
search-clear-filter = I-clear ang filter

## Search options: "Date within" choices

search-within-any = Anumang oras
search-within-days = { $count ->
    [one] { $count } araw
   *[other] { $count } araw
}
search-within-weeks = { $count ->
    [one] { $count } linggo
   *[other] { $count } linggo
}
search-within-months = { $count ->
    [one] { $count } buwan
   *[other] { $count } buwan
}
search-within-years = { $count ->
    [one] { $count } taon
   *[other] { $count } taon
}
search-within-custom = Custom

## Search options: custom dates (the calendar popover)

search-dates-on = Sa
search-dates-before = Bago ang
search-dates-since = Mula noong
search-dates-between = Sa pagitan ng
search-dates-from = Mula
search-dates-to = Hanggang
search-dates-placeholder = YYYY-MM-DD
search-dates-missing = Pumili ng petsa
search-dates-unreadable = Gumamit ng petsang gaya ng 2026-09-01
search-dates-out-of-range = Wala sa saklaw ang petsang iyan
search-dates-chip-before = Bago ang { $date }
search-dates-chip-since = Mula noong { $date }
search-dates-chip-between = { $first } – { $last }
search-dates-cancel = Kanselahin
search-dates-done = Tapos na
search-dates-month-back = Nakaraang buwan
search-dates-month-on = Susunod na buwan
search-dates-year-back = Nakaraang taon
search-dates-year-on = Susunod na taon

## More results on server: under the results, mail found by asking the
## mail server, for mail that is not downloaded to this computer yet.

search-server-more = Iba pang resulta sa server
search-server-searching = Hinahanap ang mail sa server…
search-server-empty-searching = Wala pa rito. Hinahanap ang mail sa server…
search-server-nothing = Wala nang iba pang resulta sa server
search-server-failed = Hindi makapaghanap sa server.
search-server-again = Subukan ulit
