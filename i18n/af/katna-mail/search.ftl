# Katna Mail, Afrikaans (Afrikaans).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Search options (the panel from the button at the right of the search box)

search-options = Soekopsies
search-options-close = Maak toe
search-from = Van
search-to = Aan
search-subject = Onderwerp
search-has-words = Bevat die woorde
search-without = Bevat nie
search-date-within = Datum binne
search-has-attachment = Het aanhegsel
search-clear-filter = Vee filter uit

## Search options: "Date within" choices

search-within-any = Enige tyd
search-within-days = { $count ->
    [one] { $count } dag
   *[other] { $count } dae
}
search-within-weeks = { $count ->
    [one] { $count } week
   *[other] { $count } weke
}
search-within-months = { $count ->
    [one] { $count } maand
   *[other] { $count } maande
}
search-within-years = { $count ->
    [one] { $count } jaar
   *[other] { $count } jaar
}
search-within-custom = Pasgemaak

## Search options: custom dates (the calendar popover)

search-dates-on = Op
search-dates-before = Voor
search-dates-since = Sedert
search-dates-between = Tussen
search-dates-from = Van
search-dates-to = Tot
search-dates-placeholder = JJJJ-MM-DD
search-dates-missing = Kies 'n datum
search-dates-unreadable = Gebruik 'n datum soos 2026-09-01
search-dates-out-of-range = Daardie datum is buite bereik
search-dates-chip-before = Voor { $date }
search-dates-chip-since = Sedert { $date }
search-dates-chip-between = { $first } – { $last }
search-dates-cancel = Kanselleer
search-dates-done = Klaar
search-dates-month-back = Vorige maand
search-dates-month-on = Volgende maand
search-dates-year-back = Vorige jaar
search-dates-year-on = Volgende jaar
