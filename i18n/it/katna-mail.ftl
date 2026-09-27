# Katna Mail, Italian (Italiano).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = Lingua: { $language }
language-tooltip-system = Lingua: { $language }, come nel sistema
language-search = Cerca lingua
language-system-default = Predefinita di sistema
language-system-now = Attualmente: { $language }
language-no-match = Nessuna lingua corrisponde a «{ $query }»
language-machine = Traduzione automatica. Aiutaci a migliorarla
language-setting = Lingua
language-setting-detail = La lingua di menu, pulsanti e messaggi e il formato di date e numeri. «Predefinita di sistema» segue il desktop.

## Dates and sizes

ago-just-now = adesso
ago-minutes = { $count ->
    [one] { $count } minuto fa
    [many] { $count } di minuti fa
   *[other] { $count } minuti fa
}
ago-hours = { $count ->
    [one] { $count } ora fa
    [many] { $count } di ore fa
   *[other] { $count } ore fa
}
ago-days = { $count ->
    [one] { $count } giorno fa
    [many] { $count } di giorni fa
   *[other] { $count } giorni fa
}
size-bytes = { $count ->
    [one] { $count } byte
    [many] { $count } di byte
   *[other] { $count } byte
}
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = Nascondi cartelle
folders-show = Mostra cartelle
compose = Scrivi
search = Cerca
search-mail = Cerca nella posta
search-settings = Cerca nelle impostazioni
search-clear = Cancella ricerca
search-options-show = Mostra opzioni di ricerca
settings = Impostazioni
account-add = Aggiungi un account

## App rail (and the bottom bar on a phone)

rail-mail = Posta
rail-calendar = Calendario
rail-contacts = Contatti
rail-tasks = Attività
rail-notes = Note
rail-feeds = Feed

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Prossimamente
app-calendar-promise = I tuoi calendari CalDAV, gli inviti alle riunioni ricevuti per posta e i promemoria, accanto alla Posta in arrivo.
app-tasks-promise = Elenchi di cose da fare sincronizzati con CalDAV e attività create dalla posta.
app-notes-promise = Note veloci e note su un messaggio o una conversazione per dopo.
app-feeds-promise = Leggi i feed RSS e Atom accanto alla posta.

## Contacts page

app-contacts-loading = Raccolta delle persone dalla tua posta…
app-contacts-empty = Qui compaiono le persone con cui scrivi.
app-contacts-count = { $count ->
    [one] { $count } persona dalla tua posta, prima quelle con cui scrivi di più
    [many] { $count } di persone dalla tua posta, prima quelle con cui scrivi di più
   *[other] { $count } persone dalla tua posta, prima quelle con cui scrivi di più
}
app-contacts-top = { $count ->
    [one] La persona con cui scrivi di più
    [many] Le prime { $count } di persone dalla tua posta, prima quelle con cui scrivi di più
   *[other] Le prime { $count } persone dalla tua posta, prima quelle con cui scrivi di più
}
app-contacts-messages = { $count ->
    [one] { $count } messaggio
    [many] { $count } di messaggi
   *[other] { $count } messaggi
}
app-contacts-last = ultimo: { $date }

## Navigation (the folders pane)

nav-labels = Etichette
nav-folders = Cartelle
nav-label-new = Crea nuova etichetta
nav-folder-new = Crea nuova cartella
nav-account-unnamed = Account { $number }
nav-tab-new = { $count ->
    [one] { $count } nuovo
    [many] { $count } nuovi
   *[other] { $count } nuovi
}

## Special folders (the user's own folders keep their names)

folder-inbox = Posta in arrivo
folder-starred = Speciali
folder-drafts = Bozze
folder-sent = Inviati
folder-archive = Archivio
folder-spam = Spam
folder-trash = Cestino
folder-all-mail = Tutti i messaggi
folder-scheduled = Programmati

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Nuova etichetta
label-folder-new-title = Nuova cartella
label-prompt = Inserisci il nome della nuova etichetta:
label-folder-prompt = Inserisci il nome della nuova cartella:
label-name-hint = Nome etichetta
label-folder-name-hint = Nome cartella
label-nest = Nidifica etichetta sotto:
label-folder-nest = Nidifica cartella sotto:
label-cancel = Annulla
label-create = Crea
label-creating = Creazione…
label-created = Etichetta «{ $name }» creata.
label-folder-created = Cartella «{ $name }» creata.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Principale
tab-promotions = Promozioni
tab-social = Social
tab-updates = Aggiornamenti
tab-forums = Forum
tab-focused = Evidenziata
tab-other = Altra
tab-inbox = Posta in arrivo
tab-newsletters = Newsletter
tab-notifications = Notifiche
tab-new = { $count ->
    [one] { $count } nuovo
    [many] { $count } nuovi
   *[other] { $count } nuovi
}
tab-provider-other = ordinata da Katna

## Mail list: toolbar

list-select = Seleziona
list-refresh = Aggiorna
list-more = Altro
list-mark-read = Segna come già letto
list-mark-unread = Segna come da leggere
list-move-to = Sposta in
list-archive = Archivia
list-spam = Segnala come spam
list-delete = Elimina
list-newer = Più recenti
list-older = Meno recenti
list-range = { $first }–{ $last } di { $total }
list-range-about = { $first }–{ $last } di circa { $total }
list-results = Risultati per «{ $query }»
list-results-corrected = Sono mostrati i risultati per «{ $query }»
list-search-instead = Cerca invece «{ $query }»
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Tutti
list-pick-none = Nessuno
list-pick-read = Già letti
list-pick-unread = Da leggere
list-pick-starred = Speciali
list-pick-unstarred = Non speciali

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } conversazione selezionata.
        [many] Tutte le { $count } di conversazioni sono selezionate.
       *[other] Tutte le { $count } conversazioni sono selezionate.
    }
   *[message] { $count ->
        [one] { $count } messaggio selezionato.
        [many] Tutti i { $count } di messaggi sono selezionati.
       *[other] Tutti i { $count } messaggi sono selezionati.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $count } conversazione in { $folder } selezionata.
        [many] Tutte le { $count } di conversazioni in { $folder } sono selezionate.
       *[other] Tutte le { $count } conversazioni in { $folder } sono selezionate.
    }
   *[message] { $count ->
        [one] { $count } messaggio in { $folder } selezionato.
        [many] Tutti i { $count } di messaggi in { $folder } sono selezionati.
       *[other] Tutti i { $count } messaggi in { $folder } sono selezionati.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] { $count } conversazione in questa pagina selezionata.
        [many] Tutte le { $count } di conversazioni in questa pagina sono selezionate.
       *[other] Tutte le { $count } conversazioni in questa pagina sono selezionate.
    }
   *[message] { $count ->
        [one] { $count } messaggio in questa pagina selezionato.
        [many] Tutti i { $count } di messaggi in questa pagina sono selezionati.
       *[other] Tutti i { $count } messaggi in questa pagina sono selezionati.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] Seleziona { $count } conversazione
        [many] Seleziona tutte le { $count } di conversazioni
       *[other] Seleziona tutte le { $count } conversazioni
    }
   *[message] { $count ->
        [one] Seleziona { $count } messaggio
        [many] Seleziona tutti i { $count } di messaggi
       *[other] Seleziona tutti i { $count } messaggi
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] Seleziona { $count } conversazione in { $folder }
        [many] Seleziona tutte le { $count } di conversazioni in { $folder }
       *[other] Seleziona tutte le { $count } conversazioni in { $folder }
    }
   *[message] { $count ->
        [one] Seleziona { $count } messaggio in { $folder }
        [many] Seleziona tutti i { $count } di messaggi in { $folder }
       *[other] Seleziona tutti i { $count } messaggi in { $folder }
    }
}
list-clear-selection = Annulla selezione

## Mail list: empty states

list-empty-search = Nessun messaggio corrisponde alla ricerca.
list-empty-tab = Nessun messaggio in { $tab }.
list-empty-tab-unknown = Nessun messaggio in questa scheda.
list-empty-folder = Nessun messaggio in { $folder }.
list-empty-folder-unknown = Nessun messaggio in questa cartella.
list-first-sync = Recupero della posta…
list-first-sync-detail = I messaggi compaiono qui man mano che arrivano.

## Mail list: lines

row-removed = Questo messaggio è stato rimosso.
row-starred = Speciale
row-not-starred = Non speciale
row-important = Importante. Fai clic per contrassegnare come non importante.
row-mark-important = Contrassegna come importante
row-pinned = Fissato in alto
row-pin = Fissa in alto
row-unpin = Sblocca

## Mail list: More menu and right-click menu

menu-reply = Rispondi
menu-reply-all = Rispondi a tutti
menu-forward = Inoltra
menu-archive = Archivia
menu-delete = Elimina
menu-spam = Segnala come spam
menu-mark-read = Segna come già letto
menu-mark-unread = Segna come da leggere
menu-mark-all-read = Segna tutti come già letti
menu-star = Aggiungi a Speciali
menu-unstar = Rimuovi da Speciali
menu-important = Contrassegna come importante
menu-not-important = Contrassegna come non importante
menu-pin = Fissa in alto
menu-unpin = Sblocca
menu-print-all = Stampa tutto
menu-new-window = Apri in una nuova finestra
menu-move-to = Sposta in
menu-move-to-heading = Sposta in:
menu-find-from = Trova email da { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] Conversazione archiviata.
        [many] { $count } di conversazioni archiviate.
       *[other] { $count } conversazioni archiviate.
    }
   *[message] { $count ->
        [one] Messaggio archiviato.
        [many] { $count } di messaggi archiviati.
       *[other] { $count } messaggi archiviati.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] Conversazione spostata nel Cestino.
        [many] { $count } di conversazioni spostate nel Cestino.
       *[other] { $count } conversazioni spostate nel Cestino.
    }
   *[message] { $count ->
        [one] Messaggio spostato nel Cestino.
        [many] { $count } di messaggi spostati nel Cestino.
       *[other] { $count } messaggi spostati nel Cestino.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] Conversazione spostata.
        [many] { $count } di conversazioni spostate.
       *[other] { $count } conversazioni spostate.
    }
   *[message] { $count ->
        [one] Messaggio spostato.
        [many] { $count } di messaggi spostati.
       *[other] { $count } messaggi spostati.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] Conversazione aggiunta a Speciali.
        [many] { $count } di conversazioni aggiunte a Speciali.
       *[other] { $count } conversazioni aggiunte a Speciali.
    }
   *[message] { $count ->
        [one] Messaggio aggiunto a Speciali.
        [many] { $count } di messaggi aggiunti a Speciali.
       *[other] { $count } messaggi aggiunti a Speciali.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] Conversazione rimossa da Speciali.
        [many] { $count } di conversazioni rimosse da Speciali.
       *[other] { $count } conversazioni rimosse da Speciali.
    }
   *[message] { $count ->
        [one] Messaggio rimosso da Speciali.
        [many] { $count } di messaggi rimossi da Speciali.
       *[other] { $count } messaggi rimossi da Speciali.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] Conversazione contrassegnata come importante.
        [many] { $count } di conversazioni contrassegnate come importanti.
       *[other] { $count } conversazioni contrassegnate come importanti.
    }
   *[message] { $count ->
        [one] Messaggio contrassegnato come importante.
        [many] { $count } di messaggi contrassegnati come importanti.
       *[other] { $count } messaggi contrassegnati come importanti.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] Conversazione contrassegnata come non importante.
        [many] { $count } di conversazioni contrassegnate come non importanti.
       *[other] { $count } conversazioni contrassegnate come non importanti.
    }
   *[message] { $count ->
        [one] Messaggio contrassegnato come non importante.
        [many] { $count } di messaggi contrassegnati come non importanti.
       *[other] { $count } messaggi contrassegnati come non importanti.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] Conversazione fissata in alto.
        [many] { $count } di conversazioni fissate in alto.
       *[other] { $count } conversazioni fissate in alto.
    }
   *[message] { $count ->
        [one] Messaggio fissato in alto.
        [many] { $count } di messaggi fissati in alto.
       *[other] { $count } messaggi fissati in alto.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] Conversazione sbloccata.
        [many] { $count } di conversazioni sbloccate.
       *[other] { $count } conversazioni sbloccate.
    }
   *[message] { $count ->
        [one] Messaggio sbloccato.
        [many] { $count } di messaggi sbloccati.
       *[other] { $count } messaggi sbloccati.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] Conversazione segnalata come spam.
        [many] { $count } di conversazioni segnalate come spam.
       *[other] { $count } conversazioni segnalate come spam.
    }
   *[message] { $count ->
        [one] Messaggio segnalato come spam.
        [many] { $count } di messaggi segnalati come spam.
       *[other] { $count } messaggi segnalati come spam.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] Conversazione eliminata definitivamente.
        [many] { $count } di conversazioni eliminate definitivamente.
       *[other] { $count } conversazioni eliminate definitivamente.
    }
   *[message] { $count ->
        [one] Messaggio eliminato definitivamente.
        [many] { $count } di messaggi eliminati definitivamente.
       *[other] { $count } messaggi eliminati definitivamente.
    }
}
toast-undone = Azione annullata.
toast-undo = Annulla
toast-no-spam-folder = Questo account non ha una cartella Spam.

## Reading pane: toolbar

reader-close = Chiudi
reader-back = Indietro
reader-mark-unread = Segna come da leggere
reader-move-to = Sposta in
reader-more = Altro
reader-print-all = Stampa tutto
reader-new-window = In una nuova finestra
reader-position = { $position } di { $total }
reader-newer = Più recente
reader-older = Meno recente

## Reading pane: the conversation

reader-removed = Questa conversazione è stata rimossa.
reader-no-subject = (nessun oggetto)
reader-collapse-all = Comprimi tutto
reader-expand-all = Espandi tutto
reader-unknown-sender = (mittente sconosciuto)
reader-date-ago = { $date } ({ $ago })
reader-me = me
reader-to = a { $names }
reader-starred = Speciale
reader-not-starred = Non speciale
reader-too-long = Il messaggio è troppo lungo per essere mostrato per intero.
reader-encrypted-images = Le immagini dal web non vengono mai caricate nella posta crittografata.
reader-window-failed = Impossibile aprire una nuova finestra.

## Reading pane: message details (opened from "to me")

reader-details-from = da:
reader-details-to = a:
reader-details-cc = cc:
reader-details-date = data:
reader-details-subject = oggetto:

## Reading pane: downloading a message

reader-downloading = Download di questo messaggio dal server…
reader-download-failed = Impossibile scaricare questo messaggio.
reader-try-again = Riprova

## Reply row

reply-reply = Rispondi
reply-reply-all = Rispondi a tutti
reply-forward = Inoltra

## Encrypted and signed mail

security-decrypting = Decrittografia…
security-checking = Verifica della firma…
security-partly-encrypted = Solo una parte di questo messaggio è crittografata. Il resto è stato aggiunto fuori dalla protezione e potrebbe provenire da chiunque.
security-partly-signed = Solo una parte di questo messaggio è firmata. Il resto è stato aggiunto fuori dalla protezione e potrebbe provenire da chiunque.
security-encrypted = Messaggio crittografato
security-encrypted-smime = Messaggio crittografato (S/MIME)
security-no-key = Impossibile decrittografare questo messaggio: è stato crittografato per una chiave che non possiedi.
security-cancelled = La decrittografia è stata annullata.
security-damaged = Impossibile decrittografare questo messaggio: i dati crittografati sono danneggiati o sono stati modificati.
security-decrypt-unavailable = Impossibile decrittografare questo messaggio: installa { $tool } per leggere la posta crittografata.
security-decrypt-failed = Impossibile decrittografare questo messaggio: { $reason }
security-unknown-signer = un firmatario sconosciuto
security-signed-verified = Firmato da { $signer } · verificato
security-signed-not-sender = Firmato da { $signer }, che non è il mittente
security-signed-untrusted = Firmato da { $signer }, con una chiave che hai contrassegnato come non attendibile
security-signed-unverified = Firmato da { $signer } · la chiave non è verificata
security-bad-signature = Firma non valida: questo messaggio è stato modificato dopo la firma, oppure la firma è contraffatta.
security-signature-expired = Firmato da { $signer } · la firma è scaduta
security-key-expired = Firmato da { $signer } · la chiave nel frattempo è scaduta
security-key-revoked = Firmato da { $signer } con una chiave revocata
security-missing-key = Firmato con una chiave che non possiedi, quindi non può essere verificato
security-missing-key-id = Firmato con una chiave che non possiedi ({ $key }), quindi non può essere verificato
security-signature-unavailable = Firmato; installa { $tool } per verificare la firma
security-signature-error = Impossibile verificare la firma.

## Remote images and pictures

remote-hidden = Le immagini in questo messaggio sono nascoste.
remote-show = Mostra immagini
remote-always-show = Mostra sempre da questo mittente
remote-picture-use = Usa
remote-picture-too-big = Scegli un’immagine di 8 MB o meno.
remote-picture-type = Scegli un’immagine PNG, JPEG, GIF, WebP o SVG.
remote-picture-read-failed = Impossibile leggere l’immagine: { $error }
remote-picture-keep-failed = Impossibile conservare l’immagine: { $error }
remote-picture-remove-failed = Impossibile rimuovere l’immagine: { $error }

## Attachments

attachment-count = { $count ->
    [one] Un allegato
    [many] { $count } di allegati
   *[other] { $count } allegati
}
attachment-save = Salva
attachment-save-all = Salva tutto
attachment-save-all-tooltip = Salva tutti gli allegati in una cartella
attachment-save-here = Salva qui
attachment-not-downloaded = Questo messaggio non è stato scaricato.
attachment-not-found = Impossibile trovare questo allegato nel messaggio.
attachment-read-failed = Impossibile leggere { $name }
attachment-numbered = allegato { $number }
attachment-saved-all = { $count ->
    [one] { $count } file salvato in { $place }
    [many] { $count } di file salvati in { $place }
   *[other] { $count } file salvati in { $place }
}
attachment-saved-some = { $total ->
    [one] Salvati { $saved } file su { $total } in { $place }. Impossibile salvare { $failed }
    [many] Salvati { $saved } file su { $total } in { $place }. Impossibile salvare { $failed }
   *[other] Salvati { $saved } file su { $total } in { $place }. Impossibile salvare { $failed }
}
attachment-saved-to = Salvato in { $path }
attachment-save-failed = Impossibile salvare { $name }: { $error }
attachment-open-failed = Impossibile aprire { $name }: { $error }
attachment-risky = Questo file potrebbe eseguire un programma, quindi Katna non lo apre. Salvalo invece.
attachment-encrypted-open = Questo file è arrivato crittografato. Salvalo per aprirlo altrove.

## Printing

print-failed = Impossibile stampare: { $error }
print-no-font = nessun carattere trovato
print-opened-as-pdf = Aperto come PDF per stamparlo da lì.
print-not-downloaded = (Non ancora scaricato.)
print-encrypted = (Crittografato. Aprilo in Katna Mail per stamparne il testo.)
print-to = A: { $addresses }
print-cc = Cc: { $addresses }
