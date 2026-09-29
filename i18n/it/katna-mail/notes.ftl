# Katna Mail, Italian (Italiano): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = Note
notes-view-archive = Archivio
notes-view-trash = Cestino
notes-edit-labels = Modifica etichette
notes-search = Cerca nelle note
notes-loading = Apertura delle note in corso…

## Board

notes-take-a-note = Crea una nota…
notes-new-list = Nuovo elenco
notes-pinned = Fissate
notes-others = Altre
notes-empty = Le note che aggiungi vengono visualizzate qui
notes-archive-empty = Le note archiviate vengono visualizzate qui
notes-trash-empty = Nessuna nota nel cestino
notes-none-found = Nessuna nota corrispondente
notes-label-empty = Ancora nessuna nota con questa etichetta
notes-trash-note = Le note nel cestino vengono eliminate dopo 7 giorni.
notes-empty-trash = Svuota cestino
notes-ticked = { $count ->
    [one] + { $count } elemento selezionato
    [many] + { $count } elementi selezionati
   *[other] + { $count } elementi selezionati
}

## A note's buttons

notes-pin = Fissa nota
notes-unpin = Sblocca nota
notes-archive = Archivia
notes-unarchive = Rimuovi dall’archivio
notes-delete = Elimina nota
notes-restore = Ripristina
notes-delete-forever = Elimina definitivamente
notes-color = Colore di sfondo
notes-checkboxes = Mostra o nascondi le caselle di controllo
notes-labels = Etichette
notes-close = Chiudi

## The open note

notes-title = Titolo
notes-edited = Modificata: { $date }
notes-on-this-computer = Su questo computer
notes-where = Dove è salvata questa nota

## Labels

notes-label-note = Etichetta nota
notes-label-name = Inserisci il nome dell'etichetta
notes-label-create = Crea “{ $name }”
notes-label-remove = Rimuovi etichetta
notes-label-delete = Elimina etichetta
notes-labels-none = Ancora nessuna etichetta. Aggiungine una dal pulsante etichetta di una nota.
notes-labels-done = Fine
notes-label-renamed = Etichetta rinominata in “{ $name }”
notes-label-deleted = Etichetta “{ $name }” eliminata

## A note about a mail

notes-mail = Posta
notes-open-mail = Apri l’email
notes-open-note = Apri la nota

## Colors (tooltips)

notes-color-none = Nessun colore
notes-color-coral = Corallo
notes-color-peach = Pesca
notes-color-sand = Sabbia
notes-color-mint = Menta
notes-color-sage = Salvia
notes-color-fog = Nebbia
notes-color-storm = Tempesta
notes-color-dusk = Crepuscolo
notes-color-blossom = Fiore
notes-color-clay = Argilla
notes-color-chalk = Gesso

## Messages at the foot of the window

notes-archived = Nota archiviata
notes-unarchived = Nota rimossa dall’archivio
notes-trashed = Nota spostata nel cestino
notes-restored = Nota ripristinata
notes-empty-discarded = Nota vuota eliminata
notes-mail-gone = Questa email non è più qui
notes-deleted-forever = { $count ->
    [one] Nota eliminata definitivamente
    [many] { $count } note eliminate definitivamente
   *[other] { $count } note eliminate definitivamente
}
