# Katna Mail, Dutch (Nederlands).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count ->
    [one] { $count } nieuwe e-mail
   *[other] { $count } nieuwe e-mails
}
notify-and-more = en nog { $count }
notify-no-subject = (geen onderwerp)
notify-unknown-sender = Onbekende afzender
notify-snooze-back = Terug van snooze
notify-no-reply = Nog geen antwoord
notify-no-reply-to = Niemand heeft geantwoord op ‘{ $subject }’.
notify-tracking-opened = { $who } heeft { $subject } geopend
notify-tracking-clicked = { $who } heeft op een link in { $subject } geklikt

notify-update-ready = Katna Mail kan worden bijgewerkt
notify-update-ready-body = Versie { $version } is gedownload. Bijwerken installeert deze en herstart Katna Mail.
notify-update = Bijwerken
notify-event-now = Nu
notify-event-in-minutes = { $count ->
    [one] Over { $count } minuut
   *[other] Over { $count } minuten
}
notify-event-in-hours = { $count ->
   *[other] Over { $count } uur
}
notify-event-in-days = { $count ->
    [1] Morgen
    [one] Over { $count } dag
   *[other] Over { $count } dagen
}
notify-event-all-day = Hele dag
notify-event-join = Deelnemen
notify-event-snooze = 5 min snoozen
notify-task-done = Markeren als voltooid

## Its buttons

notify-open = Openen
notify-reply-all = Allen beantwoorden
notify-mark-read = Markeren als gelezen
notify-mark-all-read = Alles markeren als gelezen
notify-archive = Archiveren
