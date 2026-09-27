# Katna Mail, German (Deutsch).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Search options (the panel from the button at the right of the search box)

search-options = Suchoptionen
search-options-close = Schließen
search-from = Von
search-to = An
search-subject = Betreff
search-has-words = Enthält die Wörter
search-without = Enthält nicht
search-date-within = Zeitraum
search-has-attachment = Mit Anhang
search-clear-filter = Filter löschen

## Search options: "Date within" choices

search-within-any = Beliebig
search-within-days = { $count ->
    [one] { $count } Tag
   *[other] { $count } Tage
}
search-within-weeks = { $count ->
    [one] { $count } Woche
   *[other] { $count } Wochen
}
search-within-months = { $count ->
    [one] { $count } Monat
   *[other] { $count } Monate
}
search-within-years = { $count ->
    [one] { $count } Jahr
   *[other] { $count } Jahre
}
search-within-custom = Benutzerdefiniert

## Search options: custom dates (the calendar popover)

search-dates-on = Am
search-dates-before = Vor
search-dates-since = Seit
search-dates-between = Zwischen
search-dates-from = Von
search-dates-to = Bis
search-dates-placeholder = JJJJ-MM-TT
search-dates-missing = Datum auswählen
search-dates-unreadable = Verwenden Sie ein Datum wie 2026-09-01
search-dates-out-of-range = Dieses Datum liegt außerhalb des gültigen Bereichs
search-dates-chip-before = Vor dem { $date }
search-dates-chip-since = Seit dem { $date }
search-dates-chip-between = { $first } – { $last }
search-dates-cancel = Abbrechen
search-dates-done = Fertig
search-dates-month-back = Vorheriger Monat
search-dates-month-on = Nächster Monat
search-dates-year-back = Vorheriges Jahr
search-dates-year-on = Nächstes Jahr
