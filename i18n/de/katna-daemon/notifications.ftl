# Katna Mail, German (Deutsch).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count ->
    [one] { $count } neue E-Mail
   *[other] { $count } neue E-Mails
}
notify-and-more = und { $count } weitere
notify-no-subject = (kein Betreff)
notify-unknown-sender = Unbekannter Absender
notify-snooze-back = Zurückgestellte E-Mails sind wieder da
notify-no-reply = Noch keine Antwort
notify-no-reply-to = Niemand hat auf „{ $subject }“ geantwortet.
notify-tracking-opened = { $who } hat { $subject } geöffnet
notify-tracking-clicked = { $who } hat auf einen Link in { $subject } geklickt

notify-update-ready = Katna Mail kann aktualisiert werden
notify-update-ready-body = Version { $version } ist heruntergeladen. Aktualisieren installiert sie und startet Katna Mail neu.
notify-update = Aktualisieren
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

## Its buttons

notify-open = Öffnen
notify-reply-all = Allen antworten
notify-mark-read = Als gelesen markieren
notify-mark-all-read = Alle als gelesen markieren
notify-archive = Archivieren
