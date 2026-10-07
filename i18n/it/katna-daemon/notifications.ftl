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
notify-follow-up-sent = Sollecito inviato
notify-follow-up-sent-to = Nessuno aveva risposto a «{ $subject }», così Katna ha inviato un sollecito.
notify-follow-up-waiting = Sollecito non inviato
notify-follow-up-waiting-to = Era previsto mentre questo computer era spento. «{ $subject }» è di nuovo nella tua Posta in arrivo.

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } ha aperto { $subject }
notify-tracking-clicked = { $who } ha cliccato un link in { $subject }

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail può essere aggiornato
notify-update-ready-body = La versione { $version } è stata scaricata. Aggiorna la installa e riavvia Katna Mail.
notify-update = Aggiorna

## Something needs the user, shown once per problem

notify-signed-out = Accedi di nuovo
notify-signed-out-body = { $provider } ha disconnesso Katna da { $address }. La posta non si sincronizza più.
notify-sign-in = Accedi
notify-password-refused = Password rifiutata
notify-password-refused-body = Il server di posta ha rifiutato la password di { $address }. Potrebbe essere cambiata.
notify-new-password = Nuova password
notify-not-sent = «{ $subject }» non è stato inviato
notify-not-sent-no-subject = Un messaggio non è stato inviato
notify-not-sent-body = È in Posta in uscita, che ne spiega il motivo.
notify-open-outbox = Apri Posta in uscita

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
notify-snooze-hour = Posticipa di 1 ora
notify-snooze-tomorrow = Domani
notify-copy-code = Copia { $code }
notify-link-verify = Verifica su { $domain }
notify-link-confirm = Conferma su { $domain }
notify-link-activate = Attiva su { $domain }

## After Archive on a notification: a short note in the same place

notify-archived = Archiviato
notify-archived-count = { $count ->
    [one] { $count } messaggio spostato fuori dalla posta in arrivo
    [many] { $count } di messaggi spostati fuori dalla posta in arrivo
   *[other] { $count } messaggi spostati fuori dalla posta in arrivo
}
notify-undo = Annulla

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = Codice copiato
notify-code-not-copied = Impossibile copiare il codice

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = Risposta inviata a { $name }
notify-open-in-katna = Apri in Katna
