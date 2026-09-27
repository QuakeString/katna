# Katna Mail, Zulu (isiZulu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Search options (the panel from the button at the right of the search box)

search-options = Okukhethwa kukho kosesho
search-options-close = Vala
search-from = Umthumeli
search-to = Umamukeli
search-subject = Isihloko
search-has-words = Inamagama
search-without = Ayinawo
search-date-within = Usuku ngaphakathi kwesikhathi
search-has-attachment = Inokunamathiselwe
search-clear-filter = Sula isihlungi

## Search options: "Date within" choices

search-within-any = Noma nini
search-within-days = { $count ->
    [one] usuku olungu-{ $count }
   *[other] izinsuku ezingu-{ $count }
}
search-within-weeks = { $count ->
    [one] iviki elingu-{ $count }
   *[other] amaviki angu-{ $count }
}
search-within-months = { $count ->
    [one] inyanga engu-{ $count }
   *[other] izinyanga ezingu-{ $count }
}
search-within-years = { $count ->
    [one] unyaka ongu-{ $count }
   *[other] iminyaka engu-{ $count }
}
search-within-custom = Ngokwezifiso

## Search options: custom dates (the calendar popover)

search-dates-on = Ngosuku
search-dates-before = Ngaphambi
search-dates-since = Kusukela
search-dates-between = Phakathi
search-dates-from = Kusuka
search-dates-to = Kuya
search-dates-placeholder = YYYY-MM-DD
search-dates-missing = Khetha usuku
search-dates-unreadable = Sebenzisa usuku olufana no-2026-09-01
search-dates-out-of-range = Lolo suku lungaphandle kobubanzi
search-dates-chip-before = Ngaphambi kuka-{ $date }
search-dates-chip-since = Kusukela ngo-{ $date }
search-dates-chip-between = { $first } – { $last }
search-dates-cancel = Khansela
search-dates-done = Kwenziwe
search-dates-month-back = Inyanga edlule
search-dates-month-on = Inyanga ezayo
search-dates-year-back = Unyaka odlule
search-dates-year-on = Unyaka ozayo
