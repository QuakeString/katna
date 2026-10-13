# Katna Mail, Swahili (Kiswahili).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Search options (the panel from the button at the right of the search box)

search-options = Chaguo za utafutaji
search-options-close = Funga
search-from = Kutoka
search-to = Kwa
search-subject = Mada
search-has-words = Ina maneno
search-without = Haina
search-date-within = Tarehe ndani ya
search-has-attachment = Ina kiambatisho
search-attachment-custom = Maalum
search-attachment-image = Picha
search-attachment-custom-hint = Andika kiendelezi, kama png, kisha Space
search-attachment-remove = Ondoa
search-clear-filter = Futa kichujio

## Search options: "Date within" choices

search-within-any = Wakati wowote
search-within-days = { $count ->
    [one] siku { $count }
   *[other] siku { $count }
}
search-within-weeks = { $count ->
    [one] wiki { $count }
   *[other] wiki { $count }
}
search-within-months = { $count ->
    [one] mwezi { $count }
   *[other] miezi { $count }
}
search-within-years = { $count ->
    [one] mwaka { $count }
   *[other] miaka { $count }
}
search-within-custom = Maalum

## Search options: custom dates (the calendar popover)

search-dates-on = Siku ya
search-dates-before = Kabla ya
search-dates-since = Tangu
search-dates-between = Kati ya
search-dates-from = Kuanzia
search-dates-to = Hadi
search-dates-placeholder = YYYY-MM-DD
search-dates-missing = Chagua tarehe
search-dates-unreadable = Tumia tarehe kama 2026-09-01
search-dates-out-of-range = Tarehe hiyo iko nje ya masafa
search-dates-chip-before = Kabla ya { $date }
search-dates-chip-since = Tangu { $date }
search-dates-chip-between = { $first } – { $last }
search-dates-cancel = Ghairi
search-dates-done = Imekamilika
search-dates-month-back = Mwezi uliopita
search-dates-month-on = Mwezi ujao
search-dates-year-back = Mwaka uliopita
search-dates-year-on = Mwaka ujao

## More results on server: under the results, mail found by asking the
## mail server, for mail that is not downloaded to this computer yet.

search-server-more = Matokeo zaidi kwenye seva
search-server-searching = Inatafuta barua kwenye seva…
search-server-empty-searching = Bado hakuna kitu hapa. Inatafuta barua kwenye seva…
search-server-nothing = Hakuna matokeo zaidi kwenye seva
search-server-failed = Imeshindwa kutafuta kwenye seva.
search-server-again = Jaribu tena
