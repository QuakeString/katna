# Katna Mail, Dutch (Nederlands).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

top-brand = Katna

## App rail (and the bottom bar on a phone)

rail-mail = E-mail
rail-calendar = Agenda
rail-contacts = Contacten
rail-tasks = Taken
rail-notes = Notities
rail-files = Bestanden

## Rail right-click menu

rail-menu-open = { $app } openen
rail-menu-settings = Instellingen van { $app }
rail-menu-turn-off = { $app } uitzetten…

## Turning an app off (Settings > Apps)

app-off-title = { $app } uitzetten?
app-off-body = Katna stopt met het synchroniseren van { $app } en haalt het weg uit:
app-off-keep = Een kopie op deze computer bewaren
app-off-keep-detail = Weer aanzetten gaat direct
app-off-remove = De kopie op deze computer verwijderen
app-off-remove-detail = Er verandert niets in je accounts, en bij weer aanzetten wordt alles opnieuw gedownload. Wat alleen op deze computer staat of nog niet is verzonden, blijft.
app-off-cancel = Annuleren
app-off-confirm = Uitzetten
app-off-done = { $app } uitgezet
app-off-note = { $app } staat uit
app-off-turn-on = Aanzetten
app-off-leaves-calendar-rail = De appbalk en Ctrl+2
app-off-leaves-calendar-agenda = De agenda naast je e-mail
app-off-leaves-calendar-meeting = Vergadering plannen, en Openen in Agenda bij uitnodigingen
app-off-leaves-calendar-reminders = Herinneringen voor afspraken
app-off-leaves-calendar-desktop = Afspraken in KRunner en de bureaubladklok
app-off-leaves-contacts-rail = De appbalk en Ctrl+3
app-off-leaves-contacts-card = Toevoegen aan contacten op de kaart van een afzender
app-off-leaves-contacts-birthdays = Verjaardagen in Agenda
app-off-leaves-tasks-rail = De appbalk en Ctrl+4
app-off-leaves-tasks-mail = Toevoegen aan Taken bij e-mail, en Shift+T
app-off-leaves-tasks-calendar = Taken in Agenda
app-off-leaves-tasks-tray = Nieuwe taak in het systeemvak, en Meta+Alt+T
app-off-leaves-tasks-reminders = Taakherinneringen
app-off-leaves-notes-rail = De appbalk en Ctrl+5
app-off-leaves-notes-mail = Een notitie toevoegen bij e-mail
app-off-leaves-notes-meetings = Vergadernotities bij afspraken
app-off-leaves-notes-tray = Nieuwe notitie in het systeemvak, en Meta+Alt+N
app-off-leaves-notes-reminders = Notitieherinneringen
app-off-leaves-files-rail = De appbalk en Ctrl+7
app-off-leaves-files-compose = Bestanden bij het bijvoegen in Opstellen

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Binnenkort beschikbaar
app-calendar-promise = Je CalDAV-agenda’s, vergaderuitnodigingen uit je e-mail en herinneringen, naast je inbox.
app-tasks-promise = Takenlijsten die synchroniseren met CalDAV, en taken gemaakt van e-mail.
app-notes-promise = Snelle notities, en notities bij een e-mail of gesprek voor later.

## Contacts page

app-contacts-loading = Mensen uit je e-mail verzamelen…
app-contacts-empty = Mensen met wie je mailt, verschijnen hier.
app-contacts-count = { $count ->
    [one] { $count } persoon uit je e-mail, meest gemaild eerst
   *[other] { $count } mensen uit je e-mail, meest gemaild eerst
}
app-contacts-top = { $count ->
    [one] De top { $count } persoon uit je e-mail, meest gemaild eerst
   *[other] De top { $count } mensen uit je e-mail, meest gemaild eerst
}
app-contacts-messages = { $count ->
    [one] { $count } bericht
   *[other] { $count } berichten
}
app-contacts-last = laatst { $date }
