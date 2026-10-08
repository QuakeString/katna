# Katna Mail, Italian (Italiano).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Search options (the panel from the button at the right of the search box)

search-options = Opzioni di ricerca
search-options-close = Chiudi
search-from = Da
search-to = A
search-subject = Oggetto
search-has-words = Contiene le parole
search-without = Non contiene
search-date-within = Data entro
search-has-attachment = Con allegato
search-attachment-custom = Personalizzato
search-attachment-image = Immagine
search-attachment-custom-hint = Digita un'estensione, come png, poi Spazio
search-attachment-remove = Rimuovi
search-clear-filter = Cancella filtro

## Search options: "Date within" choices

search-within-any = In qualsiasi momento
search-within-days = { $count ->
    [one] { $count } giorno
    [many] { $count } di giorni
   *[other] { $count } giorni
}
search-within-weeks = { $count ->
    [one] { $count } settimana
    [many] { $count } di settimane
   *[other] { $count } settimane
}
search-within-months = { $count ->
    [one] { $count } mese
    [many] { $count } di mesi
   *[other] { $count } mesi
}
search-within-years = { $count ->
    [one] { $count } anno
    [many] { $count } di anni
   *[other] { $count } anni
}
search-within-custom = Personalizzato

## Search options: custom dates (the calendar popover)

search-dates-on = Il
search-dates-before = Prima del
search-dates-since = Dal
search-dates-between = Tra
search-dates-from = Da
search-dates-to = A
search-dates-placeholder = AAAA-MM-GG
search-dates-missing = Scegli una data
search-dates-unreadable = Usa una data come 2026-09-01
search-dates-out-of-range = Quella data è fuori intervallo
search-dates-chip-before = Prima del { $date }
search-dates-chip-since = Dal { $date }
search-dates-chip-between = { $first } – { $last }
search-dates-cancel = Annulla
search-dates-done = Fine
search-dates-month-back = Mese precedente
search-dates-month-on = Mese successivo
search-dates-year-back = Anno precedente
search-dates-year-on = Anno successivo
