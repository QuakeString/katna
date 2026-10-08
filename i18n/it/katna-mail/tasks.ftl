# Katna Mail, Italian (Italiano): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Nuova attività
tasks-all = Tutte le attività
tasks-today = Oggi
tasks-upcoming = In arrivo
tasks-starred = Speciali
tasks-completed-view = Completate
tasks-new-list = Crea nuovo elenco
tasks-labels-heading = Etichette
tasks-on-this-computer = Su questo computer
tasks-my-tasks = Le mie attività
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = Accedi di nuovo per mostrare le attività
tasks-account-signed-in = Accesso a { $address } eseguito di nuovo. Recupero delle attività…
tasks-account-sign-in-refused = { $provider } non ha fatto entrare Katna. Riprova e consenti l'accesso alle tue attività.
tasks-account-refused = Il server non ha accettato la password. Yahoo, iCloud, Zoho e altri richiedono una password per le app.
tasks-account-change-password = Cambia password
tasks-account-change-password-tooltip = Digita la nuova password; Katna la verifica con il server
tasks-account-not-enabled = L'accesso alle attività per Katna non è ancora attivo.
tasks-account-failed = Impossibile leggere gli elenchi di attività.
# $reason is the server's own words, in English.
tasks-account-error = Impossibile leggere gli elenchi di attività: { $reason }
tasks-account-none = Nessun elenco di attività trovato
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = Nessun elenco di attività trovato: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } mostra le attività solo a Katna con accesso tramite { $provider }.
tasks-account-sign-in-with = Accedi con { $provider }
tasks-account-looking = Ricerca degli elenchi di attività…
tasks-account-try-again = Riprova
tasks-account-try-again-tooltip = Controlla di nuovo ora le attività di questo account
tasks-account-fixing = Operazione in corso…
tasks-list-name-placeholder = Nome dell’elenco

## Lists and tasks

tasks-loading = Lettura delle tue attività…
tasks-no-lists = I tuoi elenchi di attività compaiono qui.
tasks-search = Cerca nelle attività
tasks-search-none = Nessuna attività corrisponde alla ricerca.
tasks-add = Aggiungi un’attività
tasks-title-placeholder = Titolo
tasks-add-step = Aggiungi una sottoattività
tasks-empty = Ancora nessuna attività. Aggiungine una qui sopra.
tasks-starred-empty = Contrassegna un’attività come speciale per vederla qui.
tasks-label-empty = Nessuna attività aperta con questa etichetta.
tasks-today-empty = Nessuna scadenza oggi.
tasks-completed-empty = Le attività che completi compaiono qui.
tasks-upcoming-add = Aggiungi un’attività per { $day }
tasks-upcoming-overdue-day = { $weekday } { $day }
tasks-from-mail-quiet = Da un’email
tasks-from-note-quiet = Da una nota
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday }, { $day }
tasks-overdue = Scadute
tasks-completed = { $count ->
    [one] Completate ({ $count })
    [many] Completate ({ $count })
   *[other] Completate ({ $count })
}
tasks-list-options = Opzioni dell’elenco
tasks-sort-by = Ordina per
tasks-sort-my-order = Il mio ordine
tasks-sort-date = Data
tasks-sort-starred = Aggiunte di recente a Speciali
tasks-sort-title = Titolo
tasks-rename-list = Rinomina elenco
tasks-delete-list = Elimina elenco
tasks-mark-done = Segna come completata
tasks-mark-open = Segna come non completata
tasks-star = Contrassegna come speciale
tasks-unstar = Rimuovi da speciali
tasks-edit-title = Modifica titolo
tasks-details = Dettagli
tasks-delete = Elimina
tasks-move-to = Sposta in { $list }
tasks-from-mail = Posta
tasks-open-mail = Apri l’email
tasks-from-note = Nota
tasks-open-note = Apri la nota
tasks-note-gone = Quella nota non c’è più.
tasks-no-subject = (nessun oggetto)

## Several tasks selected (Ctrl+click, Shift+click)

tasks-selected = { $count ->
    [one] { $count } selezionata
    [many] { $count } di attività selezionate
   *[other] { $count } selezionate
}
tasks-select-clear = Cancella selezione
tasks-select-move = Sposta nell’elenco
tasks-select-date = Imposta data
tasks-next-week = La prossima settimana

## The details dialog

tasks-notes-placeholder = Aggiungi dettagli
tasks-date = Data
tasks-no-date = Nessuna data
tasks-time-placeholder = Aggiungi ora
tasks-repeat = Ripeti
tasks-repeat-never = Non si ripete
tasks-repeat-daily = Ogni giorno
tasks-repeat-weekly = Ogni settimana
tasks-repeat-monthly = Ogni mese
tasks-repeat-yearly = Ogni anno
tasks-repeat-other = Personalizzata
tasks-remind = Ricordamelo
tasks-remind-off = Non ricordare
tasks-remind-on-time = All'ora dell'attività
tasks-remind-morning = Il giorno stesso, { $time }
tasks-remind-hour-before = Un'ora prima
tasks-remind-day-before = Il giorno prima
tasks-label-add = Aggiungi etichetta
tasks-label-task = Etichetta attività
tasks-files-attach = Allega file
tasks-files-pick = Allega
tasks-file-open = Apri
tasks-file-remove = Rimuovi file
tasks-file-here = Solo su questo computer
tasks-cancel = Annulla
tasks-save = Salva
tasks-not-a-time = «{ $text }» non è un orario, ad esempio { $example }.

## Due days

tasks-due-today = Oggi
tasks-due-tomorrow = Domani
tasks-due-yesterday = Ieri
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = Attività completata
tasks-toast-next = Fatto. La prossima è il { $date }
tasks-toast-deleted = Attività eliminata
tasks-files-added = { $count ->
    [one] File allegato
    [many] { $count } di file allegati
   *[other] { $count } file allegati
}
tasks-file-removed = «{ $name }» rimosso
tasks-files-left-out = Non allegati: { $names }. Un’attività accetta file fino a { $limit }, non cartelle.
tasks-file-missing = Quel file non è più qui.
tasks-toast-added = { $count ->
    [one] Aggiunta ad Attività
    [many] { $count } attività aggiunte
   *[other] { $count } attività aggiunte
}
tasks-mail-gone = Questa email non è più qui.
tasks-toast-list-deleted = Elenco eliminato
tasks-toast-moved = Attività spostata in { $list }
# A task dragged to another place in its own list.
tasks-toast-placed = Attività spostata
tasks-toast-rescheduled = Attività riprogrammata
tasks-toast-rescheduled-several = { $count ->
    [one] Attività riprogrammata
    [many] { $count } di attività riprogrammate
   *[other] { $count } attività riprogrammate
}
tasks-toast-done-several = { $count ->
    [one] Attività completata
    [many] { $count } di attività completate
   *[other] { $count } attività completate
}
tasks-toast-open-several = { $count ->
    [one] Attività segnata come non completata
    [many] { $count } di attività segnate come non completate
   *[other] { $count } attività segnate come non completate
}
tasks-toast-starred = { $count ->
    [one] Attività aggiunta a Speciali
    [many] { $count } di attività aggiunte a Speciali
   *[other] { $count } attività aggiunte a Speciali
}
tasks-toast-unstarred = { $count ->
    [one] Attività rimossa da Speciali
    [many] { $count } di attività rimosse da Speciali
   *[other] { $count } attività rimosse da Speciali
}
tasks-toast-deleted-several = { $count ->
    [one] Attività eliminata
    [many] { $count } di attività eliminate
   *[other] { $count } attività eliminate
}
