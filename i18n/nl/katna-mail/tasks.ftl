# Katna Mail, Dutch (Nederlands): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Maken
tasks-all = Alle taken
tasks-today = Vandaag
tasks-starred = Met ster
tasks-new-list = Nieuwe lijst maken
tasks-on-this-computer = Op deze computer
tasks-my-tasks = Mijn taken
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = Meld je opnieuw aan om taken te tonen
tasks-account-signed-in = Opnieuw aangemeld bij { $address }. Je taken worden opgehaald…
tasks-account-sign-in-refused = { $provider } heeft Katna niet binnengelaten. Probeer het opnieuw en geef toegang tot je taken.
tasks-account-refused = De server heeft het wachtwoord niet geaccepteerd. Yahoo, iCloud, Zoho en andere hebben een app-wachtwoord nodig.
tasks-account-change-password = Wachtwoord wijzigen
tasks-account-change-password-tooltip = Instellingen > Accounts openen
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
tasks-today-empty = Niets met deadline vandaag.
tasks-today-date = { $weekday }, { $day }
tasks-overdue = Achterstallig
tasks-completed = { $count ->
    [one] Voltooid ({ $count })
   *[other] Voltooid ({ $count })
}
tasks-list-options = Lijstopties
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
tasks-toast-added = { $count ->
    [one] Toegevoegd aan Taken
   *[other] { $count } taken toegevoegd
}
tasks-mail-gone = Die e-mail is er niet meer.
tasks-toast-list-deleted = Lijst verwijderd
tasks-toast-moved = Verplaatst naar { $list }
tasks-toast-rescheduled = Taak verplaatst
