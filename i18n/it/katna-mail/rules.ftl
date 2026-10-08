# Katna Mail, Italian (Italiano).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = Regole
settings-rules-summary = Ordina, etichetta, inoltra o silenzia automaticamente la nuova posta
settings-rules-intro = Le regole ordinano automaticamente la nuova posta, in questo ordine. Trascina per riordinare.
settings-rules-all-accounts = Tutti gli account
settings-rules-new = Nuova regola
settings-rules-none = Ancora nessuna regola. Una regola ordina automaticamente la nuova posta: per mittente, oggetto o parole.
settings-rules-none-account = Ancora nessuna regola per questo account.
settings-rules-drag = Trascina per riordinare
settings-rules-edit = Modifica regola
settings-rules-turn-off = Disattiva questa regola
settings-rules-turn-on = Attiva questa regola

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = Regole iniziali
settings-rules-starters-intro = Disattivate finché non ne attivi una. Valgono per tutti i tuoi account; modificane una per cambiarla.
settings-rules-starter-turning-on = Attivazione di «{ $name }»…
settings-rules-starter-failed = Impossibile attivare «{ $name }»: { $error }
rules-starter-promotions = Promozioni silenziose
rules-starter-newsletters = Newsletter in Da leggere
rules-starter-receipts = Ricevute e fatture
rules-starter-deliveries = Consegne
rules-starter-train = Biglietti del treno
rules-starter-flight = Biglietti aerei
rules-starter-codes = Codici monouso
rules-starter-security = Avvisi di sicurezza
rules-starter-social = Posta dai social
rules-starter-invites = Inviti del calendario
rules-starter-folder-reading = Da leggere
rules-starter-folder-receipts = Ricevute
rules-starter-folder-deliveries = Consegne
rules-starter-folder-travel = Viaggi
rules-starter-folder-social = Social
rules-runs-katna = Eseguita in Katna
rules-runs-gmail = Eseguita su Gmail
rules-runs-sieve = Eseguita sul server
rules-stopped = Interrotta
rules-error-folder-gone = La cartella usata da questa regola non esiste più. Modifica la regola per sceglierne un’altra.
rules-error-no-archive = Questo account non ha una cartella Archivio. Modifica la regola per fare altro.
rules-error-no-trash = Questo account non ha una cartella Cestino. Modifica la regola per fare altro.
rules-error-cannot-send = Questo account non può inviare posta, quindi la regola non può inoltrarla.
rules-error-other = { $error }. Modifica la regola e riattivala.

settings-folders = Cartelle
settings-folders-summary = Numero di messaggi da leggere nel riquadro delle cartelle
settings-folders-unread-counts = Numero di messaggi da leggere su ogni cartella
settings-folders-unread-counts-detail = Se disattivato: solo Posta in arrivo mostra quanti sono da leggere

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } e { $next }
rules-summary-or = { $first } o { $next }
rules-summary-more = altri { $count }
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = Ha un allegato
rules-summary-no-attachment = Non ha allegati
rules-summary-mailing-list = Da una mailing list
rules-summary-not-mailing-list = Non da una mailing list
rules-summary-tab = Nella scheda { $tab }
rules-summary-not-tab = Non nella scheda { $tab }
rules-summary-move = sposta in { $folder }
rules-summary-archive = salta la posta in arrivo
rules-summary-trash = sposta nel Cestino
rules-summary-mark-read = segna come già letto
rules-summary-star = aggiungi a Speciali
rules-summary-important = contrassegna come importante
rules-summary-label = etichetta { $label }
rules-summary-forward = inoltra a { $address }
rules-summary-dont-notify = non notificare
rules-summary-read-after = { $count ->
    [one] segna come già letto dopo { $count } giorno
    [many] segna come già letto dopo { $count } di giorni
   *[other] segna come già letto dopo { $count } giorni
}
rules-summary-folder-gone = una cartella che non esiste più

## The rule editor

rules-editor-new-title = Nuova regola
rules-editor-edit-title = Modifica regola
rules-editor-name-hint = Nome della regola
rules-editor-when = Quando una nuova email soddisfa
rules-editor-of-these = queste condizioni:
rules-mode-all = tutte
rules-mode-any = una qualsiasi di
rules-field-from = Da
rules-field-to = A
rules-field-cc = Cc
rules-field-any-recipient = A o Cc
rules-field-reply-to = Rispondi a
rules-field-subject = Oggetto
rules-field-body = Testo
rules-field-attachment-name = Nome dell’allegato
rules-field-has-attachment = Ha un allegato
rules-field-mailing-list = Da una mailing list
rules-field-tab = Scheda della posta in arrivo
rules-comparator-contains = contiene
rules-comparator-not-contains = non contiene
rules-comparator-begins-with = inizia con
rules-comparator-ends-with = finisce con
rules-comparator-equals = è esattamente
rules-comparator-matches = corrisponde al modello
rules-has-yes = sì
rules-has-no = no
rules-editor-value-hint = Parole o un indirizzo
rules-editor-add-condition = Aggiungi una condizione
rules-editor-remove = Rimuovi
rules-editor-then = Allora:
rules-action-move = Sposta in
rules-action-archive = Salta la posta in arrivo (archivia)
rules-action-trash = Sposta nel Cestino
rules-action-mark-read = Segna come già letto
rules-action-star = Aggiungi a Speciali
rules-action-important = Contrassegna come importante
rules-action-label = Aggiungi etichetta
rules-action-forward = Inoltra a
rules-action-dont-notify = Non notificare
rules-action-read-after = Segna come già letto dopo
rules-editor-choose-folder = Scegli una cartella
rules-editor-choose-label = Scegli un’etichetta
rules-editor-new-folder = Nuova: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = Indirizzo email
rules-editor-days = giorni
rules-editor-add-action = Aggiungi un’azione
rules-editor-stop = Fermati qui: le regole successive non vengono eseguite su questa email
rules-editor-accounts = Account:
rules-editor-accounts-none = Scegli gli account
rules-editor-accounts-many = { $count ->
    [one] { $count } account
    [many] { $count } di account
   *[other] { $count } account
}
rules-editor-matches = Corrisponde a { $mails } degli ultimi { $days } giorni
rules-editor-mails = { $count ->
    [one] { $count } email
    [many] { $count } di email
   *[other] { $count } email
}
rules-editor-counting = Conteggio della posta corrispondente…
rules-editor-show = Mostrale
rules-editor-also-apply = Applica anche a queste { $count }
rules-editor-runs-katna = Eseguita in Katna, mentre questo computer è acceso.
rules-editor-runs-gmail = Eseguita su Gmail, quindi funziona anche sul telefono e a computer spento.
rules-editor-runs-sieve = Eseguita sul tuo server di posta, quindi funziona anche sul telefono e a computer spento.
rules-note-gmail-action = Eseguita in Katna: i filtri di Gmail non possono «{ $action }».
rules-note-sieve-action = Eseguita in Katna: le regole del tuo server di posta non possono «{ $action }».
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = Eseguita in Katna: i filtri di Gmail non possono verificare «{ $test }» come fa Katna.
rules-note-sieve-condition = Eseguita in Katna: le regole del tuo server di posta non possono verificare «{ $test }» come fa Katna.
rules-note-order = Eseguita in Katna, come una regola precedente dell’account: le regole vengono eseguite nell’ordine dell’elenco.
rules-note-gmail-stop = Eseguita in Katna: i filtri di Gmail non possono impedire l’esecuzione delle regole successive.
rules-note-gmail-forward = Eseguita in Katna: Gmail inoltra solo agli indirizzi verificati nelle sue impostazioni, e { $address } non è tra questi.
rules-note-gmail-folder = Eseguita in Katna: Gmail non ha un’etichetta per una cartella usata da questa regola.
rules-note-sieve-folder = Eseguita in Katna: il tuo server di posta non ha una cartella usata da questa regola.
rules-note-gmail-sign-in = Eseguita in Katna finché non accedi di nuovo a Google e consenti a Katna di creare filtri di Gmail.
rules-note-sieve-other-script = Eseguita in Katna: sul tuo server di posta è attivo un altro script di regole («{ $name }»).
rules-note-gmail-failed = Eseguita in Katna: Gmail non l’ha accettata ({ $error }).
rules-note-sieve-failed = Eseguita in Katna: il tuo server di posta non l’ha accettata ({ $error }).
rules-editor-cancel = Annulla
rules-editor-save = Salva
rules-editor-saving = Salvataggio…
rules-editor-delete = Elimina regola
rules-editor-delete-ask = Eliminare questa regola?
rules-editor-delete-keep = Mantienila
rules-editor-delete-confirm = Elimina
rules-editor-needs-folder = Scegli una cartella per ogni «Sposta in» e un’etichetta per ogni «Aggiungi etichetta».
rules-editor-needs-days = «Segna come già letto dopo» richiede un numero di giorni, da 1 a 3650.
rules-saved = Regola salvata
rules-saved-applied = { $count ->
    [one] Regola salvata e applicata a { $count } email
    [many] Regola salvata e applicata a { $count } di email
   *[other] Regola salvata e applicata a { $count } email
}
rules-apply-failed = Regola salvata, ma la sua applicazione non è riuscita: { $error }
rules-deleted = Regola eliminata
rules-delete-failed = Impossibile eliminare la regola: { $error }
rules-change-failed = Impossibile modificare le regole: { $error }
