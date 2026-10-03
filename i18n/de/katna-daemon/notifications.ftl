# Katna Mail, German (Deutsch).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

notify-new-emails = { $count ->
    [one] { $count } neue E-Mail
   *[other] { $count } neue E-Mails
}
notify-and-more = und { $count } weitere
notify-no-subject = (kein Betreff)
notify-unknown-sender = Unbekannter Absender

## Reminders the user asked for (same buttons)

notify-snooze-back = Zurückgestellte E-Mails sind wieder da
notify-no-reply = Noch keine Antwort
notify-no-reply-to = Niemand hat auf „{ $subject }“ geantwortet.

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } hat { $subject } geöffnet
notify-tracking-clicked = { $who } hat auf einen Link in { $subject } geklickt

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail kann aktualisiert werden
notify-update-ready-body = Version { $version } ist heruntergeladen. Aktualisieren installiert sie und startet Katna Mail neu.
notify-update = Aktualisieren

## Reminders of calendar events

notify-event-now = Jetzt
notify-event-in-minutes = { $count ->
   *[other] In { $count } Minuten
}
notify-event-in-hours = { $count ->
   *[other] In { $count } Stunden
}
notify-event-in-days = { $count ->
    [1] Morgen
   *[other] In { $count } Tagen
}
notify-event-all-day = Ganztägig
notify-event-join = Teilnehmen
notify-event-snooze = 5 Min. zurückstellen
notify-task-done = Als erledigt markieren

## The buttons of new-mail notifications and reminders

notify-open = Öffnen
notify-peek = Vorschau
notify-reply = Antworten
notify-reply-placeholder = Antwort an { $name }…
notify-send = Senden
notify-reply-all = Allen antworten
notify-mark-read = Als gelesen markieren
notify-mark-all-read = Alle als gelesen markieren
notify-archive = Archivieren

## After Archive on a notification: a short note in the same place

notify-archived = Archiviert
notify-archived-count = { $count ->
    [one] { $count } Nachricht aus dem Posteingang verschoben
   *[other] { $count } Nachrichten aus dem Posteingang verschoben
}
notify-undo = Rückgängig

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = Antwort an { $name } gesendet
notify-open-in-katna = In Katna öffnen
