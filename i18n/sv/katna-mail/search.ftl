# Katna Mail, Swedish (Svenska).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Search options (the panel from the button at the right of the search box)

search-options = Sökalternativ
search-options-close = Stäng
search-from = Från
search-to = Till
search-subject = Ämne
search-has-words = Innehåller orden
search-without = Innehåller inte
search-date-within = Datum inom
search-has-attachment = Har bilaga
search-attachment-custom = Anpassat
search-attachment-image = Bild
search-attachment-custom-hint = Skriv ett filtillägg, till exempel png, och sedan Blanksteg
search-attachment-remove = Ta bort
search-clear-filter = Rensa filter

## Search options: "Date within" choices

search-within-any = När som helst
search-within-days = { $count ->
    [one] { $count } dag
   *[other] { $count } dagar
}
search-within-weeks = { $count ->
    [one] { $count } vecka
   *[other] { $count } veckor
}
search-within-months = { $count ->
    [one] { $count } månad
   *[other] { $count } månader
}
search-within-years = { $count ->
    [one] { $count } år
   *[other] { $count } år
}
search-within-custom = Anpassat

## Search options: custom dates (the calendar popover)

search-dates-on = Den
search-dates-before = Före
search-dates-since = Sedan
search-dates-between = Mellan
search-dates-from = Från
search-dates-to = Till
search-dates-placeholder = ÅÅÅÅ-MM-DD
search-dates-missing = Välj ett datum
search-dates-unreadable = Använd ett datum som 2026-09-01
search-dates-out-of-range = Det datumet ligger utanför intervallet
search-dates-chip-before = Före { $date }
search-dates-chip-since = Sedan { $date }
search-dates-chip-between = { $first } – { $last }
search-dates-cancel = Avbryt
search-dates-done = Klar
search-dates-month-back = Föregående månad
search-dates-month-on = Nästa månad
search-dates-year-back = Föregående år
search-dates-year-on = Nästa år
