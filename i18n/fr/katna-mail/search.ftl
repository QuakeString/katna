# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Search options (the panel from the button at the right of the search box)

search-options = Options de recherche
search-options-close = Fermer
search-from = De
search-to = À
search-subject = Objet
search-has-words = Contient les mots
search-without = Ne contient pas
search-date-within = Période
search-has-attachment = Contient une pièce jointe
search-attachment-custom = Personnalisé
search-attachment-image = Image
search-attachment-custom-hint = Saisissez une extension, par exemple png, puis Espace
search-attachment-remove = Retirer
search-clear-filter = Effacer le filtre

## Search options: "Date within" choices

search-within-any = N’importe quand
search-within-days = { $count ->
    [one] { $count } jour
    [many] { $count } de jours
   *[other] { $count } jours
}
search-within-weeks = { $count ->
    [one] { $count } semaine
    [many] { $count } de semaines
   *[other] { $count } semaines
}
search-within-months = { $count ->
    [one] { $count } mois
    [many] { $count } de mois
   *[other] { $count } mois
}
search-within-years = { $count ->
    [one] { $count } an
    [many] { $count } d’années
   *[other] { $count } ans
}
search-within-custom = Personnalisé

## Search options: custom dates (the calendar popover)

search-dates-on = Le
search-dates-before = Avant
search-dates-since = Depuis
search-dates-between = Entre
search-dates-from = Du
search-dates-to = Au
search-dates-placeholder = AAAA-MM-JJ
search-dates-missing = Choisissez une date
search-dates-unreadable = Utilisez une date comme 2026-09-01
search-dates-out-of-range = Cette date est hors limites
search-dates-chip-before = Avant le { $date }
search-dates-chip-since = Depuis le { $date }
search-dates-chip-between = { $first } – { $last }
search-dates-cancel = Annuler
search-dates-done = OK
search-dates-month-back = Mois précédent
search-dates-month-on = Mois suivant
search-dates-year-back = Année précédente
search-dates-year-on = Année suivante
