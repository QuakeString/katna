# Katna Mail, Dutch (Nederlands): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Nieuwe taak
tasks-all = Alle taken
tasks-today = Vandaag
tasks-upcoming = Binnenkort
tasks-starred = Met ster
tasks-completed-view = Voltooid
tasks-new-list = Nieuwe lijst maken
tasks-labels-heading = Labels
tasks-on-this-computer = Op deze computer
tasks-my-tasks = Mijn taken
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = Meld je opnieuw aan om taken te tonen
tasks-account-signed-in = Opnieuw aangemeld bij { $address }. Je taken worden opgehaald…
tasks-account-sign-in-refused = { $provider } heeft Katna niet binnengelaten. Probeer het opnieuw en geef toegang tot je taken.
tasks-account-refused = De server heeft het wachtwoord niet geaccepteerd. Yahoo, iCloud, Zoho en andere hebben een app-wachtwoord nodig.
tasks-account-change-password = Wachtwoord wijzigen
tasks-account-change-password-tooltip = Typ het nieuwe wachtwoord; Katna controleert het bij de server
tasks-account-not-enabled = Taaktoegang voor Katna is nog niet ingeschakeld.
tasks-account-failed = De takenlijsten konden niet worden gelezen.
# $reason is the server's own words, in English.
tasks-account-error = De takenlijsten konden niet worden gelezen: { $reason }
tasks-account-none = Geen takenlijsten gevonden
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = Geen takenlijsten gevonden: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } toont taken alleen aan Katna als die is aangemeld met { $provider }.
tasks-account-sign-in-with = Aanmelden met { $provider }
tasks-account-looking = Takenlijsten zoeken…
tasks-account-try-again = Opnieuw proberen
tasks-account-try-again-tooltip = De taken van dit account nu opnieuw controleren
tasks-account-fixing = Bezig…
tasks-list-name-placeholder = Naam van de lijst

## Lists and tasks

tasks-loading = Je taken worden gelezen…
tasks-no-lists = Je takenlijsten verschijnen hier.
tasks-search = Taken zoeken
tasks-search-none = Er zijn geen taken die overeenkomen met je zoekopdracht.
tasks-add = Een taak toevoegen
tasks-title-placeholder = Titel
tasks-add-step = Een subtaak toevoegen
tasks-empty = Nog geen taken. Voeg er hierboven een toe.
tasks-starred-empty = Geef een taak een ster om die hier te zien.
tasks-label-empty = Geen open taken met dit label.
tasks-today-empty = Niets met deadline vandaag.
tasks-completed-empty = Taken die je voltooit, verschijnen hier.
tasks-upcoming-add = Een taak toevoegen voor { $day }
tasks-upcoming-overdue-day = { $weekday } { $day }
tasks-from-mail-quiet = Uit e-mail
tasks-from-note-quiet = Uit notitie
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday }, { $day }
tasks-overdue = Achterstallig
tasks-completed = { $count ->
    [one] Voltooid ({ $count })
   *[other] Voltooid ({ $count })
}
tasks-list-options = Lijstopties
tasks-sort-by = Sorteren op
tasks-sort-my-order = Mijn volgorde
tasks-sort-date = Datum
tasks-sort-starred = Recent met ster
tasks-sort-title = Titel
tasks-rename-list = Lijst hernoemen
tasks-delete-list = Lijst verwijderen
tasks-mark-done = Markeren als voltooid
tasks-mark-open = Markeren als niet voltooid
tasks-star = Ster geven
tasks-unstar = Ster verwijderen
tasks-edit-title = Titel bewerken
tasks-details = Details
tasks-delete = Verwijderen
tasks-move-to = Verplaatsen naar { $list }
tasks-from-mail = E-mail
tasks-open-mail = De e-mail openen
tasks-from-note = Notitie
tasks-open-note = De notitie openen
tasks-note-gone = Die notitie is er niet meer.
tasks-no-subject = (geen onderwerp)

## Several tasks selected (Ctrl+click, Shift+click)

tasks-selected = { $count ->
    [one] { $count } geselecteerd
   *[other] { $count } geselecteerd
}
tasks-select-clear = Selectie wissen
tasks-select-move = Naar lijst verplaatsen
tasks-select-date = Datum instellen
tasks-next-week = Volgende week

## The details dialog

tasks-notes-placeholder = Details toevoegen
tasks-date = Datum
tasks-no-date = Geen datum
tasks-time-placeholder = Tijd toevoegen
tasks-repeat = Herhalen
tasks-repeat-never = Wordt niet herhaald
tasks-repeat-daily = Dagelijks
tasks-repeat-weekly = Wekelijks
tasks-repeat-monthly = Maandelijks
tasks-repeat-yearly = Jaarlijks
tasks-repeat-other = Aangepast
tasks-remind = Herinner mij
tasks-remind-off = Niet herinneren
tasks-remind-on-time = Op het tijdstip zelf
tasks-remind-morning = Op de dag zelf, { $time }
tasks-remind-hour-before = Een uur van tevoren
tasks-remind-day-before = De dag ervoor
tasks-label-add = Label toevoegen
tasks-label-task = Taak labelen
tasks-files-attach = Bestanden bijvoegen
tasks-files-pick = Bijvoegen
tasks-file-open = Openen
tasks-file-remove = Bestand verwijderen
tasks-file-here = Alleen op deze computer
tasks-cancel = Annuleren
tasks-save = Opslaan
tasks-not-a-time = “{ $text }” is geen tijd, bijvoorbeeld { $example }.

## Due days

tasks-due-today = Vandaag
tasks-due-tomorrow = Morgen
tasks-due-yesterday = Gisteren
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = Taak voltooid
tasks-toast-next = Klaar. De volgende is op { $date }
tasks-toast-deleted = Taak verwijderd
tasks-files-added = { $count ->
    [one] Bestand bijgevoegd
   *[other] { $count } bestanden bijgevoegd
}
tasks-file-removed = “{ $name }” verwijderd
tasks-files-left-out = Niet bijgevoegd: { $names }. Een taak neemt bestanden tot { $limit }, geen mappen.
tasks-file-missing = Dat bestand is er niet meer.
tasks-toast-added = { $count ->
    [one] Toegevoegd aan Taken
   *[other] { $count } taken toegevoegd
}
tasks-mail-gone = Die e-mail is er niet meer.
tasks-toast-list-deleted = Lijst verwijderd
tasks-toast-moved = Verplaatst naar { $list }
# A task dragged to another place in its own list.
tasks-toast-placed = Taak verplaatst
tasks-toast-rescheduled = Taak verplaatst
tasks-toast-rescheduled-several = { $count ->
    [one] Taak verzet
   *[other] { $count } taken verzet
}
tasks-toast-done-several = { $count ->
    [one] Taak voltooid
   *[other] { $count } taken voltooid
}
tasks-toast-open-several = { $count ->
    [one] Taak gemarkeerd als niet voltooid
   *[other] { $count } taken gemarkeerd als niet voltooid
}
tasks-toast-starred = { $count ->
    [one] Ster gegeven aan taak
   *[other] Ster gegeven aan { $count } taken
}
tasks-toast-unstarred = { $count ->
    [one] Ster verwijderd
   *[other] Sterren verwijderd van { $count } taken
}
tasks-toast-deleted-several = { $count ->
    [one] Taak verwijderd
   *[other] { $count } taken verwijderd
}
