# Katna Mail, Italian (Italiano).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

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
notify-snooze-back = Di nuovo in Posta in arrivo
notify-no-reply = Ancora nessuna risposta
notify-no-reply-to = Nessuno ha risposto a «{ $subject }».
notify-tracking-opened = { $who } ha aperto { $subject }
notify-tracking-clicked = { $who } ha cliccato un link in { $subject }

notify-update-ready = Katna Mail può essere aggiornato
notify-update-ready-body = La versione { $version } è stata scaricata. Aggiorna la installa e riavvia Katna Mail.
notify-update = Aggiorna
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

## Its buttons

notify-open = Apri
notify-reply-all = Rispondi a tutti
notify-mark-read = Segna come già letto
notify-mark-all-read = Segna tutti come già letti
notify-archive = Archivia
