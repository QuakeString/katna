# Katna Mail, German (Deutsch).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

top-brand = Katna

## App rail (and the bottom bar on a phone)

rail-mail = E-Mail
rail-calendar = Kalender
rail-contacts = Kontakte
rail-tasks = Aufgaben
rail-notes = Notizen
rail-files = Dateien

## Rail right-click menu

rail-menu-open = { $app } öffnen
rail-menu-settings = Einstellungen für { $app }
rail-menu-turn-off = { $app } ausschalten…

## Turning an app off (Settings > Apps)

app-off-title = { $app } ausschalten?
app-off-body = Katna synchronisiert { $app } nicht mehr und entfernt es aus:
app-off-keep = Eine Kopie auf diesem Computer behalten
app-off-keep-detail = Das Wiedereinschalten geht sofort
app-off-remove = Die Kopie auf diesem Computer entfernen
app-off-remove-detail = In Ihren Konten ändert sich nichts, und beim Wiedereinschalten wird alles erneut heruntergeladen. Was nur auf diesem Computer liegt oder noch nicht gesendet ist, bleibt erhalten.
app-off-cancel = Abbrechen
app-off-confirm = Ausschalten
app-off-done = { $app } ausgeschaltet
app-off-note = { $app } ist ausgeschaltet
app-off-turn-on = Einschalten
app-off-leaves-calendar-rail = Der App-Leiste und Strg+2
app-off-leaves-calendar-agenda = Der Terminübersicht neben Ihren E-Mails
app-off-leaves-calendar-meeting = „Besprechung planen“ und „Im Kalender öffnen“ bei Einladungen
app-off-leaves-calendar-reminders = Terminerinnerungen
app-off-leaves-calendar-desktop = Terminen in KRunner und in der Uhr der Arbeitsumgebung
app-off-leaves-contacts-rail = Der App-Leiste und Strg+3
app-off-leaves-contacts-card = „Zu Kontakten hinzufügen“ auf der Karte eines Absenders
app-off-leaves-contacts-birthdays = Geburtstagen im Kalender
app-off-leaves-tasks-rail = Der App-Leiste und Strg+4
app-off-leaves-tasks-mail = „Zu Aufgaben hinzufügen“ bei E-Mails und Umschalt+T
app-off-leaves-tasks-calendar = Aufgaben im Kalender
app-off-leaves-tasks-tray = „Neue Aufgabe“ in der Kontrollleiste und Meta+Alt+T
app-off-leaves-tasks-reminders = Aufgabenerinnerungen
app-off-leaves-notes-rail = Der App-Leiste und Strg+5
app-off-leaves-notes-mail = „Notiz hinzufügen“ bei E-Mails
app-off-leaves-notes-meetings = Besprechungsnotizen bei Terminen
app-off-leaves-notes-tray = „Neue Notiz“ in der Kontrollleiste und Meta+Alt+N
app-off-leaves-notes-reminders = Notizerinnerungen
app-off-leaves-files-rail = Der App-Leiste und Strg+7
app-off-leaves-files-compose = Dateien beim Anhängen im Verfassen-Fenster

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Demnächst verfügbar
app-calendar-promise = Ihre CalDAV-Kalender, Einladungen zu Besprechungen aus Ihren E-Mails und Erinnerungen – direkt neben Ihrem Posteingang.
app-tasks-promise = Aufgabenlisten, die über CalDAV synchronisiert werden, und Aufgaben aus E-Mails.
app-notes-promise = Schnelle Notizen und Notizen zu einer E-Mail oder Konversation für später.

## Contacts page

app-contacts-loading = Personen aus Ihren E-Mails werden gesammelt…
app-contacts-empty = Personen, mit denen Sie schreiben, werden hier angezeigt.
app-contacts-count = { $count ->
    [one] { $count } Person aus Ihren E-Mails, häufigste Kontakte zuerst
   *[other] { $count } Personen aus Ihren E-Mails, häufigste Kontakte zuerst
}
app-contacts-top = { $count ->
    [one] Top { $count } Person aus Ihren E-Mails, häufigste Kontakte zuerst
   *[other] Top { $count } Personen aus Ihren E-Mails, häufigste Kontakte zuerst
}
app-contacts-messages = { $count ->
    [one] { $count } Nachricht
   *[other] { $count } Nachrichten
}
app-contacts-last = zuletzt { $date }
