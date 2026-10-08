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
notify-follow-up-sent = Nachfass-E-Mail gesendet
notify-follow-up-sent-to = Niemand hatte auf „{ $subject }“ geantwortet, daher hat Katna nachgefasst.
notify-follow-up-waiting = Nachfass-E-Mail nicht gesendet
notify-follow-up-waiting-to = Sie war fällig, während dieser Computer aus war. „{ $subject }“ ist wieder in Ihrem Posteingang.

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } hat { $subject } geöffnet
notify-tracking-clicked = { $who } hat auf einen Link in { $subject } geklickt

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail kann aktualisiert werden
notify-update-ready-body = Version { $version } ist heruntergeladen. Aktualisieren installiert sie und startet Katna Mail neu.
notify-update = Aktualisieren

## Something needs the user, shown once per problem

notify-signed-out = Erneut anmelden
notify-signed-out-body = { $provider } hat Katna von { $address } abgemeldet. E-Mails werden nicht mehr synchronisiert.
notify-sign-in = Anmelden
notify-password-refused = Passwort abgelehnt
notify-password-refused-body = Der E-Mail-Server hat das Passwort für { $address } abgelehnt. Es wurde vielleicht geändert.
notify-new-password = Neues Passwort
notify-not-sent = „{ $subject }“ wurde nicht gesendet
notify-not-sent-no-subject = Eine Nachricht wurde nicht gesendet
notify-not-sent-body = Sie liegt im Postausgang, dort steht der Grund.
notify-open-outbox = Postausgang öffnen

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
notify-snooze-hour = 1 Stunde zurückstellen
notify-snooze-tomorrow = Morgen
notify-copy-code = { $code } kopieren
notify-link-verify = Auf { $domain } bestätigen
notify-link-confirm = Auf { $domain } bestätigen
notify-link-activate = Auf { $domain } aktivieren

## After Archive on a notification: a short note in the same place

notify-archived = Archiviert
notify-archived-count = { $count ->
    [one] { $count } Nachricht aus dem Posteingang verschoben
   *[other] { $count } Nachrichten aus dem Posteingang verschoben
}
notify-undo = Rückgängig

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = Code kopiert
notify-code-not-copied = Der Code konnte nicht kopiert werden

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = Antwort an { $name } gesendet
notify-open-in-katna = In Katna öffnen
