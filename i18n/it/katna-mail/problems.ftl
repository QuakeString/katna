# Katna Mail, Italian (Italiano).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = Il server di posta

problems-signed-out = { $provider } ha disconnesso Katna da { $address }. La posta non si sincronizza più.
problems-password-refused = { $provider } ha rifiutato la password di { $address }. Potrebbe essere cambiata.
problems-no-answer = { $provider } non risponde per { $address }. Katna continua a provare.
problems-offline = Sei offline. La tua posta è ancora qui, e quella che invii attende finché non torni online.
problems-accounts-need-you = { $count ->
    [one] 1 account richiede la tua attenzione
    [many] { $count } di account richiedono la tua attenzione
   *[other] { $count } account richiedono la tua attenzione
}
problems-show = Mostra
problems-later = Più tardi
problems-new-password = Nuova password
problems-try-again = Riprova

## The New password card

problems-password-title = Nuova password
problems-password-detail = { $provider } ha rifiutato la password salvata per { $address }. Scrivi quella nuova; Katna la verifica prima di conservarla.
problems-password-placeholder = Password
problems-password-show = Mostra password
problems-password-hide = Nascondi password
problems-password-cancel = Annulla
problems-password-save = Salva
problems-password-checking = Verifica…
problems-password-refused-again = { $provider } ha rifiutato anche questa password. Controllala e riprova.
problems-password-saved = Password salvata per { $address }. Recupero della posta…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = Il server di posta di { $address } non ha accettato lo spostamento di { $count ->
    [one] un messaggio, quindi è tornato dov’era.
    [many] { $count } di messaggi, quindi sono tornati dov’erano.
   *[other] { $count } messaggi, quindi sono tornati dov’erano.
}
problems-refused-flags = Il server di posta di { $address } non ha accettato di contrassegnare { $count ->
    [one] un messaggio (letto, speciale…), quindi è tornato com’era.
    [many] { $count } di messaggi (letti, speciali…), quindi sono tornati com’erano.
   *[other] { $count } messaggi (letti, speciali…), quindi sono tornati com’erano.
}
problems-refused-label = Il server di posta di { $address } non ha accettato di cambiare le etichette di { $count ->
    [one] un messaggio, quindi è tornato com’era.
    [many] { $count } di messaggi, quindi sono tornati com’erano.
   *[other] { $count } messaggi, quindi sono tornati com’erano.
}
problems-refused-delete = Il server di posta di { $address } non ha accettato l’eliminazione di { $count ->
    [one] un messaggio, quindi è tornato.
    [many] { $count } di messaggi, quindi sono tornati.
   *[other] { $count } messaggi, quindi sono tornati.
}
problems-refused-other = Il server di posta di { $address } non ha accettato { $count ->
    [one] una modifica, quindi Katna l’ha ripristinata com’era.
    [many] { $count } di modifiche, quindi Katna le ha ripristinate com’erano.
   *[other] { $count } modifiche, quindi Katna le ha ripristinate com’erano.
}
problems-details = Dettagli

## Katna's background service (katna-daemon) isn't running

service-starting = Avvio del servizio in background di Katna…
service-failed = Il servizio in background di Katna non si avvia, quindi la posta non si sincronizza.
service-start-again = Avvia di nuovo
service-started-again = Il servizio in background di Katna si era fermato ed è stato riavviato.
service-details-title = Perché il servizio non si avvia
service-details-body = Copia questo testo e invialo con la tua segnalazione. Non contiene posta né password.
service-details-copy = Copia
service-details-close = Chiudi
service-not-running = Il servizio in background di Katna non è in esecuzione.
service-no-answer = Il servizio in background di Katna non ha risposto: { $error }
service-no-session = Nessuna sessione D-Bus: { $error }

## Safe mode: an update left Katna's background service unable to start

safe-line = Katna è in modalità provvisoria dopo un problema con l’aggiornamento, quindi la posta non si sincronizza.
safe-try-again = Riprova
safe-restore = Ripristina
safe-restoring = Ripristino dei tuoi dati del { $when }…
safe-restored = Dati del { $when } ripristinati. Ciò che c’era prima è conservato in una cartella.
safe-show-folder = Mostra cartella
safe-restore-failed = Impossibile ripristinare i tuoi dati: { $error }
safe-restore-title = Ripristinare i tuoi dati da prima di un aggiornamento?
safe-restore-body = Katna torna alla copia che scegli. La posta arrivata dopo viene scaricata di nuovo dai tuoi account.
safe-restore-none = Non ci sono ancora copie. Katna ne crea una prima che ogni aggiornamento modifichi i tuoi dati.
safe-restore-keep = Ciò che c’è ora, compresi posta non inviata, bozze e modifiche non ancora sincronizzate, viene prima conservato in una cartella, così non si perde nulla.
safe-restore-cancel = Annulla
safe-restore-mail = Posta
safe-restore-pim = Account e contatti
safe-restore-blobs = Allegati
safe-report-title = Rapporto di debug
safe-report-body = Copialo e allegalo alla tua segnalazione di bug. Non contiene posta, indirizzi né password.
safe-report-restore = Ripristina…
safe-report-copied = Rapporto di debug copiato
