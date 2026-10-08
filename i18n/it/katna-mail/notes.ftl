# Katna Mail, Italian (Italiano): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = Note
notes-view-reminders = Promemoria
notes-view-archive = Archivio
notes-view-trash = Cestino
notes-edit-labels = Modifica etichette
notes-search = Cerca nelle note
notes-loading = Apertura delle note in corso…

## Board

notes-take-a-note = Crea una nota…
notes-new-list = Nuovo elenco
notes-new-note = Nuova nota
notes-pinned = Fissate
notes-others = Altre
notes-empty = Le note che aggiungi vengono visualizzate qui
notes-archive-empty = Le note archiviate vengono visualizzate qui
notes-trash-empty = Nessuna nota nel cestino
notes-none-found = Nessuna nota corrispondente
notes-label-empty = Ancora nessuna nota con questa etichetta
notes-reminders-empty = Le note con promemoria in arrivo vengono visualizzate qui
notes-trash-note = Le note nel cestino vengono eliminate dopo 7 giorni.
notes-empty-trash = Svuota cestino
notes-ticked = { $count ->
    [one] + { $count } elemento selezionato
    [many] + { $count } elementi selezionati
   *[other] + { $count } elementi selezionati
}
notes-select = Seleziona nota
notes-selected = { $count ->
    [one] { $count } selezionata
    [many] { $count } di selezionate
   *[other] { $count } selezionate
}
notes-select-clear = Annulla selezione

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
notes-more = Altro
notes-make-copy = Crea una copia
notes-remind = Ricordamelo
notes-add-picture = Aggiungi immagine
notes-history = Cronologia versioni
notes-ai = Aiutami a scrivere
notes-send-as-mail = Invia come email
notes-save-markdown = Salva come Markdown
notes-save-pdf = Salva come PDF

## The open note

notes-title = Titolo
notes-edited = Modificata: { $date }
notes-on-this-computer = Su questo computer
notes-where = Dove è salvata questa nota
notes-untitled = Nota senza titolo

## Pictures

notes-picture-choose = Aggiungi immagini
notes-picture-remove = Rimuovi immagine
notes-picture-too-big = In una nota possono andare immagini fino a { $size }
notes-picture-kind = Quel file non è un’immagine che Katna può mostrare
notes-picture-unreadable = Impossibile leggere { $name }: { $error }

## Reminders

notes-remind-me = Ricordamelo
notes-remind-off = Rimuovi promemoria
notes-remind-in-the-past = Scegli un’ora non ancora passata
notes-remind-today = Oggi, { $time }
notes-remind-tomorrow = Domani, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = Promemoria impostato per { $when }
notes-reminder-off = Promemoria rimosso

## Links between notes

notes-link-note = Collega una nota
notes-link-new = Nuova nota «{ $title }»
notes-linked-from = Collegata da
notes-link-gone = Quella nota non c’è più
notes-new-note-gone = La nuova nota non c’è più.

## Version history

notes-versions = Versioni
notes-version-now = Ora
notes-version-here = Tu, su questo computer
notes-version-yesterday = Ieri, { $time }
notes-version-changes = { $count ->
    [one] { $count } modifica
    [many] { $count } di modifiche
   *[other] { $count } modifiche
}
notes-version-from = Da { $device }
notes-version-elsewhere = Da un altro dispositivo
notes-version-created = Creata
notes-version-restore = Ripristina questa versione
notes-version-restored = Versione ripristinata
notes-history-none = Ancora nessuna versione precedente

## AI help

notes-ai-tidy = Riordina il testo
notes-ai-checklist = Trasforma in elenco di controllo
notes-ai-summarise = Riassumi
notes-ai-empty = Scrivi prima qualcosa
notes-ai-tidied = Testo riordinato. Ctrl+Z lo ripristina.
notes-ai-listed = Trasformato in elenco di controllo. Ctrl+Z lo ripristina.
notes-ai-summarised = Riassunto aggiunto in cima

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

## Meeting notes

notes-meeting-take = Prendi appunti riunione
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = Partecipanti: { $names }
notes-meeting-notes = Appunti
notes-meeting-actions = Azioni da svolgere
notes-event = Evento
notes-open-event = Apri l’evento

## Formatting

notes-format = Formattazione
notes-format-heading-1 = Titolo 1
notes-format-heading-2 = Titolo 2
notes-format-normal = Testo normale
notes-format-bold = Grassetto
notes-format-italic = Corsivo
notes-format-underline = Sottolineato
notes-format-quote = Citazione
notes-format-code = Codice
notes-format-divider = Separatore
notes-format-clear = Cancella formattazione

## Tasks

notes-make-task = Trasforma in attività

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
notes-saved = Nota salvata
notes-pinned-count = { $count ->
    [one] Nota fissata
    [many] { $count } di note fissate
   *[other] { $count } note fissate
}
notes-unpinned-count = { $count ->
    [one] Nota sbloccata
    [many] { $count } di note sbloccate
   *[other] { $count } note sbloccate
}
notes-colored-count = { $count ->
    [one] Colore cambiato
    [many] Colore cambiato su { $count } di note
   *[other] Colore cambiato su { $count } note
}
notes-archived-count = { $count ->
    [one] Nota archiviata
    [many] { $count } di note archiviate
   *[other] { $count } note archiviate
}
notes-unarchived-count = { $count ->
    [one] Nota rimossa dall’archivio
    [many] { $count } di note rimosse dall’archivio
   *[other] { $count } note rimosse dall’archivio
}
notes-trashed-count = { $count ->
    [one] Nota spostata nel cestino
    [many] { $count } di note spostate nel cestino
   *[other] { $count } note spostate nel cestino
}
notes-restored-count = { $count ->
    [one] Nota ripristinata
    [many] { $count } di note ripristinate
   *[other] { $count } note ripristinate
}
notes-copied-count = { $count ->
    [one] Copia creata
    [many] { $count } di copie create
   *[other] { $count } copie create
}
notes-empty-discarded = Nota vuota eliminata
notes-mail-gone = Questa email non è più qui
notes-deleted-forever = { $count ->
    [one] Nota eliminata definitivamente
    [many] { $count } note eliminate definitivamente
   *[other] { $count } note eliminate definitivamente
}
