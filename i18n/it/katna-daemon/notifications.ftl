# Katna Mail, Italian (Italiano).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

notify-new-emails = { $count ->
    [one] { $count } nuova email
    [many] { $count } di nuove email
   *[other] { $count } nuove email
}
notify-and-more = { $count ->
    [one] e { $count } altra
   *[other] e altre { $count }
}
notify-no-subject = (nessun oggetto)
notify-unknown-sender = Mittente sconosciuto

## Reminders the user asked for (same buttons)

notify-snooze-back = Di nuovo in Posta in arrivo
notify-no-reply = Ancora nessuna risposta
notify-no-reply-to = Nessuno ha risposto a «{ $subject }».

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } ha aperto { $subject }
notify-tracking-clicked = { $who } ha cliccato un link in { $subject }

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail può essere aggiornato
notify-update-ready-body = La versione { $version } è stata scaricata. Aggiorna la installa e riavvia Katna Mail.
notify-update = Aggiorna

## Reminders of calendar events

notify-event-now = Ora
notify-event-in-minutes = { $count ->
    [one] Tra { $count } minuto
   *[other] Tra { $count } minuti
}
notify-event-in-hours = { $count ->
    [one] Tra { $count } ora
   *[other] Tra { $count } ore
}
notify-event-in-days = { $count ->
    [1] Domani
    [one] Tra { $count } giorno
   *[other] Tra { $count } giorni
}
notify-event-all-day = Tutto il giorno
notify-event-join = Partecipa
notify-event-snooze = Posticipa di 5 min
notify-task-done = Segna come completata

## The buttons of new-mail notifications and reminders

notify-open = Apri
notify-peek = Anteprima
notify-reply = Rispondi
notify-reply-placeholder = Rispondi a { $name }…
notify-send = Invia
notify-reply-all = Rispondi a tutti
notify-mark-read = Segna come già letto
notify-mark-all-read = Segna tutti come già letti
notify-archive = Archivia

## After Archive on a notification: a short note in the same place

notify-archived = Archiviato
notify-archived-count = { $count ->
    [one] { $count } messaggio spostato fuori dalla posta in arrivo
    [many] { $count } di messaggi spostati fuori dalla posta in arrivo
   *[other] { $count } messaggi spostati fuori dalla posta in arrivo
}
notify-undo = Annulla

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = Risposta inviata a { $name }
notify-open-in-katna = Apri in Katna
