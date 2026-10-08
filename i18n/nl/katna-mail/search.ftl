# Katna Mail, Dutch (Nederlands).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Search options (the panel from the button at the right of the search box)

search-options = Zoekopties
search-options-close = Sluiten
search-from = Van
search-to = Aan
search-subject = Onderwerp
search-has-words = Bevat de woorden
search-without = Bevat niet
search-date-within = Datum binnen
search-has-attachment = Heeft bijlage
search-attachment-custom = Aangepast
search-attachment-image = Afbeelding
search-attachment-custom-hint = Typ een extensie, zoals png, en dan Spatie
search-attachment-remove = Verwijderen
search-clear-filter = Filter wissen

## Search options: "Date within" choices

search-within-any = Altijd
search-within-days = { $count ->
    [one] { $count } dag
   *[other] { $count } dagen
}
search-within-weeks = { $count ->
    [one] { $count } week
   *[other] { $count } weken
}
search-within-months = { $count ->
    [one] { $count } maand
   *[other] { $count } maanden
}
search-within-years = { $count ->
    [one] { $count } jaar
   *[other] { $count } jaar
}
search-within-custom = Aangepast

## Search options: custom dates (the calendar popover)

search-dates-on = Op
search-dates-before = Vóór
search-dates-since = Sinds
search-dates-between = Tussen
search-dates-from = Van
search-dates-to = Tot
search-dates-placeholder = JJJJ-MM-DD
search-dates-missing = Kies een datum
search-dates-unreadable = Gebruik een datum zoals 2026-09-01
search-dates-out-of-range = Die datum valt buiten het bereik
search-dates-chip-before = Vóór { $date }
search-dates-chip-since = Sinds { $date }
search-dates-chip-between = { $first } – { $last }
search-dates-cancel = Annuleren
search-dates-done = Klaar
search-dates-month-back = Vorige maand
search-dates-month-on = Volgende maand
search-dates-year-back = Vorig jaar
search-dates-year-on = Volgend jaar

## More results on server: under the results, mail found by asking the
## mail server, for mail that is not downloaded to this computer yet.

search-server-more = Meer resultaten op de server
search-server-searching = E-mail op de server doorzoeken…
search-server-empty-searching = Hier staat nog niets. E-mail op de server doorzoeken…
search-server-nothing = Geen resultaten meer op de server
search-server-failed = Kan de server niet doorzoeken.
search-server-again = Opnieuw proberen
