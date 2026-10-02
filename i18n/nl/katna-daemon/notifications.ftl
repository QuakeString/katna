# Katna Mail, Dutch (Nederlands).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

notify-new-emails = { $count ->
    [one] { $count } nieuwe e-mail
   *[other] { $count } nieuwe e-mails
}
notify-and-more = en nog { $count }
notify-no-subject = (geen onderwerp)
notify-unknown-sender = Onbekende afzender

## Reminders the user asked for (same buttons)

notify-snooze-back = Terug van snooze
notify-no-reply = Nog geen antwoord
notify-no-reply-to = Niemand heeft geantwoord op ‘{ $subject }’.

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } heeft { $subject } geopend
notify-tracking-clicked = { $who } heeft op een link in { $subject } geklikt

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail kan worden bijgewerkt
notify-update-ready-body = Versie { $version } is gedownload. Bijwerken installeert deze en herstart Katna Mail.
notify-update = Bijwerken

## Reminders of calendar events

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

## The buttons of new-mail notifications and reminders

notify-open = Openen
notify-peek = Bekijken
notify-reply = Beantwoorden
notify-reply-placeholder = Antwoord aan { $name }…
notify-send = Versturen
notify-reply-all = Allen beantwoorden
notify-mark-read = Markeren als gelezen
notify-mark-all-read = Alles markeren als gelezen
notify-archive = Archiveren

## After Archive on a notification: a short note in the same place

notify-archived = Gearchiveerd
notify-archived-count = { $count ->
    [one] { $count } bericht uit de inbox gehaald
   *[other] { $count } berichten uit de inbox gehaald
}
notify-undo = Ongedaan maken

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = Antwoord verstuurd aan { $name }
notify-open-in-katna = Openen in Katna
