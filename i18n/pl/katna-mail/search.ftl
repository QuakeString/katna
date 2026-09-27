# Katna Mail, Polish (Polski).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Search options (the panel from the button at the right of the search box)

search-options = Opcje wyszukiwania
search-options-close = Zamknij
search-from = Od
search-to = Do
search-subject = Temat
search-has-words = Zawiera słowa
search-without = Nie zawiera
search-date-within = Data w ciągu
search-has-attachment = Ma załącznik
search-clear-filter = Wyczyść filtr

## Search options: "Date within" choices

search-within-any = Dowolna data
search-within-days = { $count ->
    [one] { $count } dnia
    [few] { $count } dni
    [many] { $count } dni
   *[other] { $count } dnia
}
search-within-weeks = { $count ->
    [one] { $count } tygodnia
    [few] { $count } tygodni
    [many] { $count } tygodni
   *[other] { $count } tygodnia
}
search-within-months = { $count ->
    [one] { $count } miesiąca
    [few] { $count } miesięcy
    [many] { $count } miesięcy
   *[other] { $count } miesiąca
}
search-within-years = { $count ->
    [one] { $count } roku
    [few] { $count } lat
    [many] { $count } lat
   *[other] { $count } roku
}
search-within-custom = Niestandardowy

## Search options: custom dates (the calendar popover)

search-dates-on = W dniu
search-dates-before = Przed
search-dates-since = Od
search-dates-between = Pomiędzy
search-dates-from = Od
search-dates-to = Do
search-dates-placeholder = RRRR-MM-DD
search-dates-missing = Wybierz datę
search-dates-unreadable = Użyj daty w formacie 2026-09-01
search-dates-out-of-range = Ta data jest poza zakresem
search-dates-chip-before = Przed { $date }
search-dates-chip-since = Od { $date }
search-dates-chip-between = { $first } – { $last }
search-dates-cancel = Anuluj
search-dates-done = Gotowe
search-dates-month-back = Poprzedni miesiąc
search-dates-month-on = Następny miesiąc
search-dates-year-back = Poprzedni rok
search-dates-year-on = Następny rok
