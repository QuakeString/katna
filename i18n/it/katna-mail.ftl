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

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Apri questo messaggio per leggerne gli allegati.
text-copy = Copia
text-select-all = Seleziona tutto

## Settings page: its tabs

settings-tab-general = Generali
settings-tab-inbox = Posta in arrivo
settings-tab-accounts = Account
settings-tab-subscriptions = Iscrizioni
settings-tab-appearance = Aspetto
settings-tab-shortcuts = Scorciatoie
settings-tab-default-apps = App predefinite
settings-tab-folders-rules = Cartelle e regole
settings-tab-compose = Scrittura
settings-tab-mcp-server = Server MCP
settings-tab-feedback = Feedback degli utenti
settings-tab-experimental = Sperimentali

## Settings page: tabs still to come

settings-tab-subscriptions-coming = Visualizza le newsletter e le mailing list che ricevi e annulla l’iscrizione con un clic.
settings-tab-folders-rules-coming = Crea, rinomina, sposta e nascondi cartelle ed etichette e scegli quali sincronizzare. Le regole ordinano, etichettano, inoltrano o eliminano automaticamente la nuova posta, in base a mittente, oggetto o parole.
settings-tab-mcp-server-coming = Consenti agli assistenti IA su questo computer di cercare, leggere e scrivere bozze della tua posta, con il tuo consenso.

## Settings > General

settings-general-conversations = Visualizzazione per conversazione
settings-general-conversations-group = Raggruppa le risposte alla stessa email
settings-general-conversations-group-detail = Una riga per conversazione nell’elenco
settings-general-reading = Lettura
settings-general-newest-first = Prima il messaggio più recente
settings-general-newest-first-detail = Una conversazione inizia con l’ultima risposta
settings-general-full-headers = Mostra intestazioni complete
settings-general-full-headers-detail = Da, a, cc, data e oggetto visibili in ogni messaggio
settings-general-full-names = Nomi completi dei destinatari
settings-general-full-names-detail = «a me, Ada Lovelace» anziché «a me, Ada»
settings-general-mark-read = Segna come già letto
settings-general-mark-read-now = Appena si apre
settings-general-mark-read-1s = Dopo 1 secondo di apertura
settings-general-mark-read-3s = Dopo 3 secondi di apertura
settings-general-mark-read-never = Solo quando la segno come già letta
settings-general-reply-button = Pulsante Rispondi
settings-general-reply-all = Rispondi a tutti
settings-general-reply-all-detail = Il pulsante di risposta accanto a ogni messaggio risponde a tutti, non solo al mittente
settings-general-remote-images = Immagini dal web
settings-general-remote-images-detail = Caricare le immagini di un messaggio fa sapere al mittente che l’hai aperto, quando e più o meno dove. Se disattivato, ogni messaggio chiede prima, e puoi sempre mostrare le immagini di un mittente.
settings-general-remote-images-always = Mostra sempre le immagini
settings-general-remote-images-always-detail = In ogni messaggio, non solo da mittenti attendibili
settings-general-sending = Invio
settings-general-sending-detail = Quanto attende un messaggio inviato, per poterlo annullare.
settings-general-offline = Posta offline
settings-general-offline-detail = La posta recente viene scaricata per intero, per leggerla senza connessione. La posta meno recente viene scaricata quando la apri.
settings-general-offline-days = { $count ->
    [one] { $count } giorno
    [many] { $count } di giorni
   *[other] { $count } giorni
}
settings-general-offline-years = { $count ->
    [one] { $count } anno
    [many] { $count } di anni
   *[other] { $count } anni
}
settings-general-offline-all = Tutta la posta
settings-general-offline-note = Scegliendo meno giorni, la posta già scaricata viene mantenuta. Sul server non cambia nulla.
settings-general-notifications = Notifiche
settings-general-notifications-detail = Per la nuova posta in Posta in arrivo, anche quando Katna Mail è chiuso.
settings-general-new-mail = Avvisami quando arriva nuova posta
settings-general-new-mail-detail = Con Rispondi a tutti, Segna come già letto e Archivia
settings-general-new-mail-sound = Riproduci un suono
settings-general-new-mail-sound-detail = Il suono di nuova posta del desktop
settings-general-desktop = Desktop
settings-general-open-at-login = Apri Katna Mail all’accesso
settings-general-open-at-login-detail = La posta si sincronizza comunque all’accesso, finché il servizio è in esecuzione
settings-general-tray = Mostra Katna nell’area di notifica
settings-general-tray-detail = Con il numero di messaggi da leggere e un menu
settings-general-unread-badge = Numero di messaggi da leggere sull’icona nella barra delle applicazioni
settings-general-unread-badge-detail = Quanti messaggi in Posta in arrivo sono da leggere

## Settings > Inbox

settings-inbox-tabs = Schede della posta in arrivo
settings-inbox-tabs-detail = Suddividi la posta in arrivo in schede, come fa il sito web del tuo provider di posta.
settings-inbox-tabs-show = Mostra le schede della posta in arrivo
settings-inbox-tabs-show-detail = Se disattivato, un unico elenco per tutti gli account
settings-inbox-no-accounts = Aggiungi un account per sceglierne le schede.
settings-inbox-tabs-automatic = Automatico: { $tabs } ({ $provider })
settings-inbox-tabs-off = Nessuna scheda
settings-inbox-tabs-gmail = Principale, Promozioni, Social, Aggiornamenti, Forum
settings-inbox-tabs-focused = Evidenziata e Altra
settings-inbox-tabs-zoho = Posta in arrivo, Newsletter e Notifiche
settings-inbox-tabs-shown = Schede mostrate. La posta di una scheda disattivata resta in { $tab }.

## Settings > Appearance

settings-appearance-reading-pane = Riquadro di lettura
settings-appearance-reading-pane-detail = Dove viene mostrata una conversazione aperta.
settings-appearance-pane-right = A destra dell’elenco
settings-appearance-pane-none = Nessuna suddivisione
settings-appearance-density = Densità
settings-appearance-density-default = Predefinita
settings-appearance-density-compact = Compatta
settings-appearance-scaling = Scala
settings-appearance-scaling-detail = Ingrandisce o rimpicciolisce tutto in Katna Mail, oltre alla scala del desktop: testo, icone, spaziatura e divisori. La posta che invii mantiene la propria dimensione del carattere. Con dimensioni molto piccole può essere difficile fare clic sulle icone.
settings-appearance-theme = Tema
settings-appearance-theme-system = Come il desktop
settings-appearance-theme-light = Chiaro
settings-appearance-theme-dark = Scuro
settings-appearance-desktop-colors = Colori del desktop
settings-appearance-desktop-colors-use = Usa i colori del desktop
settings-appearance-desktop-colors-use-detail = Lo schema di colori e il colore di risalto del desktop
settings-appearance-app-names = Nomi delle app
settings-appearance-app-names-show = Mostra i nomi delle app
settings-appearance-app-names-show-detail = Nomi sotto le icone delle app all’estrema sinistra
settings-appearance-sender-pictures = Immagini dei mittenti
settings-appearance-sender-pictures-show = Mostra i loghi aziendali
settings-appearance-sender-pictures-show-detail = Cercati in base al dominio del mittente, mai in base al messaggio, e conservati per una settimana
settings-appearance-important = Indicatori di importanza
settings-appearance-important-show = Mostra gli indicatori di importanza
settings-appearance-important-show-detail = Accanto a ogni messaggio nell’elenco
settings-appearance-message-width = Larghezza dei messaggi
settings-appearance-message-width-limit = Limita la larghezza dei messaggi
settings-appearance-message-width-limit-detail = Le righe lunghe sono più facili da leggere in una finestra larga
settings-appearance-mail-colors = Colori della posta
settings-appearance-mail-colors-detail = La maggior parte della posta è pensata per una pagina bianca. Con un tema scuro i suoi colori vengono sostituiti da colori scuri ben leggibili; se disattivato, mantiene i colori del mittente su una pagina chiara.
settings-appearance-dark-mail = Colori scuri anche per la posta
settings-appearance-dark-mail-detail = Solo con il tema scuro
settings-appearance-attachment-previews = Anteprime degli allegati
settings-appearance-attachment-previews-show = Mostra le anteprime degli allegati
settings-appearance-attachment-previews-show-detail = Una piccola immagine del contenuto di ogni file nel suo riquadro

## Settings > Default apps

settings-default-apps-intro = Dove si aprono gli allegati quando fai clic su di essi. Il visualizzatore può sempre aprire un file anche in un’altra app. Le app predefinite del desktop si impostano nelle sue impostazioni.
settings-default-apps-pdf = File PDF
settings-default-apps-pdf-detail = Pagine, con zoom.
settings-default-apps-pictures = Immagini
settings-default-apps-pictures-detail = Foto (raddrizzate), PNG, GIF, WebP, BMP, TIFF e SVG.
settings-default-apps-text = File di testo
settings-default-apps-text-detail = Testo semplice, log, codice e altro testo.
settings-default-apps-sheets = Fogli di calcolo
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) e CSV.
settings-default-apps-documents = Documenti
settings-default-apps-documents-detail = Word (docx) e testo OpenDocument (odt).
settings-default-apps-katna = Visualizzatore di Katna Mail
settings-default-apps-system = App predefinita del desktop
settings-default-apps-ask = Chiedi ogni volta quale app usare
settings-default-apps-after-saving = Dopo il salvataggio
settings-default-apps-show-folder = Mostra i file salvati nella loro cartella
settings-default-apps-show-folder-detail = Apre il gestore di file con gli allegati salvati selezionati

## Settings > Compose

settings-compose-send-from = Invia nuovi messaggi da
settings-compose-send-from-detail = Le risposte e gli inoltri partono sempre dall’account in cui ti trovi.
settings-compose-send-from-current = L’account in cui ti trovi
settings-compose-send-on-replies = Invio nelle risposte
settings-compose-send-on-replies-detail = Cosa fa Invia in una risposta o un inoltro. Il menu accanto a Invia offre l’altra opzione.
settings-compose-send-plain = Invia
settings-compose-send-archive = Invia e archivia
settings-compose-signatures = Firme
settings-compose-signatures-detail = Aggiunta sotto il tuo messaggio, dopo una riga «--». Scegline un’altra nella finestra di scrittura.
settings-compose-untitled = Senza titolo
settings-compose-signature-name = Nome, ad esempio Lavoro
settings-compose-signature-first = La mia firma
settings-compose-signature-numbered = Firma { $number }
settings-compose-signature-delete = Elimina
settings-compose-signature-deleted = Firma eliminata
settings-compose-signature-new = Crea nuova
settings-compose-no-signatures = Ancora nessuna firma.
settings-compose-no-signature = Nessuna firma
settings-compose-for-new-mail = Per i nuovi messaggi
settings-compose-for-replies = Per risposte e inoltri
settings-compose-for-replies-detail = In una conversazione in cui hai firmato un messaggio, una risposta inizia invece con quella firma.
settings-compose-format = Formato
settings-compose-plain-text = Scrivi in testo semplice
settings-compose-plain-text-detail = La nuova posta inizia senza formattazione; la finestra di scrittura può cambiarlo
settings-compose-spelling = Ortografia
settings-compose-spell-check = Controlla l’ortografia durante la scrittura
settings-compose-spell-check-detail = Le parole errate vengono sottolineate, con suggerimenti facendo clic con il tasto destro
settings-compose-spell-desktop = Lingua del desktop ({ $language })
settings-compose-templates = Modelli
settings-compose-templates-detail = Salva i messaggi che scrivi spesso e usali per iniziare un nuovo messaggio o una risposta.

## Settings > Shortcuts

settings-shortcuts-set = Set di scorciatoie
settings-shortcuts-set-detail = Parti dai tasti di un’app di posta che conosci. Qui Cmd corrisponde a Ctrl. Le tue modifiche restano applicate sopra il set, e Ripristina predefinite torna ai tasti del set.
settings-shortcuts-single = Scorciatoie a tasto singolo
settings-shortcuts-single-detail = Tasti senza Ctrl o Alt, come nella webmail: e archivia, j e k spostano, / cerca. Funzionano nell’elenco e nella conversazione aperta, mai mentre scrivi.
settings-shortcuts-single-use = Usa le scorciatoie a tasto singolo
settings-shortcuts-single-use-detail = Le scorciatoie con Ctrl funzionano sempre
settings-shortcuts-how = Fai clic su un tasto per cambiarlo, o su + per aggiungerne uno, quindi premi i nuovi tasti. Esc annulla.
settings-shortcuts-restore = Ripristina predefinite
settings-shortcuts-no-key = Nessun tasto
settings-shortcuts-press = Premi i tasti…
settings-shortcuts-then = { $keys } poi…
settings-shortcuts-moved = { $keys } ora esegue «{ $action }» invece di «{ $previous }».
settings-shortcuts-single-off = Le scorciatoie a tasto singolo sono disattivate, quindi questo tasto funzionerà quando saranno attive.
settings-shortcuts-restored = Tutte le scorciatoie hanno di nuovo i tasti del loro set.

## Settings search: the line under a result

settings-general-language-summary = Lingua dell’app, delle date e dei numeri
settings-general-reading-summary = Prima il messaggio più recente, intestazioni complete, nomi completi dei destinatari
settings-general-mark-read-summary = Quando una conversazione aperta viene segnata come già letta: subito, dopo 1 o 3 secondi, o a mano
settings-general-reply-button-summary = Il pulsante di risposta accanto a ogni messaggio risponde a tutti
settings-general-remote-images-summary = Mostra sempre le immagini di ogni messaggio
settings-general-sending-summary = Annulla invio: quanto attende un messaggio inviato, per poterlo annullare
settings-general-offline-summary = Quanti giorni di posta recente vengono scaricati per intero, per leggerla senza connessione
settings-general-notifications-summary = Notifiche di nuova posta e il loro suono
settings-general-desktop-summary = Apri Katna Mail all’accesso, l’icona nell’area di notifica e il numero di messaggi da leggere sull’icona nella barra delle applicazioni
settings-accounts-accounts-summary = Aggiungi o rimuovi un account, o cambiane l’immagine
settings-appearance-density-summary = Righe predefinite o compatte nell’elenco
settings-appearance-scaling-summary = Ingrandisci o rimpicciolisci tutto: testo, icone, spaziatura e divisori
settings-appearance-theme-summary = Come il desktop, chiaro o scuro
settings-appearance-sender-pictures-summary = Loghi aziendali, cercati in base al dominio del mittente
settings-appearance-important-summary = L’indicatore di importanza accanto a ogni messaggio nell’elenco
settings-appearance-mail-colors-summary = Colori scuri per la posta HTML con un tema scuro, o i colori del mittente
settings-appearance-attachment-previews-summary = Una piccola immagine del contenuto di ogni allegato
settings-shortcuts-set-summary = Parti dai tasti di Gmail, Inbox by Gmail, Apple Mail, Outlook o Thunderbird
settings-shortcuts-single-summary = Tasti senza Ctrl o Alt, come nella webmail
settings-default-apps-pdf-summary = Dove si aprono gli allegati PDF
settings-default-apps-pictures-summary = Dove si aprono foto e immagini
settings-default-apps-text-summary = Dove si aprono testo semplice, log e codice
settings-default-apps-sheets-summary = Dove si aprono i file Excel, OpenDocument e CSV
settings-default-apps-documents-summary = Dove si aprono i testi Word e OpenDocument
settings-default-apps-after-saving-summary = Mostra gli allegati salvati nella loro cartella
settings-compose-send-from-summary = L’account da cui parte la nuova posta: quello in cui ti trovi o sempre lo stesso
settings-compose-send-on-replies-summary = Invia, o Invia e archivia la conversazione, nelle risposte e negli inoltri
settings-compose-signatures-summary = Aggiunta sotto il tuo messaggio, dopo una riga «--»
settings-compose-for-new-mail-summary = La firma con cui inizia la nuova posta
settings-compose-for-replies-summary = La firma con cui iniziano risposte e inoltri
settings-compose-format-summary = Scrivi la nuova posta in testo semplice
settings-compose-spelling-summary = Controlla l’ortografia durante la scrittura, e la lingua del dizionario
settings-compose-templates-summary = Prossimamente: salva i messaggi che scrivi spesso e usali per iniziare un nuovo messaggio o una risposta
settings-feedback-crash-reports-summary = Salva i rapporti sugli arresti anomali su questo computer quando Katna Mail o il suo servizio in background si arresta in modo anomalo
settings-feedback-saved-summary = Visualizza, copia o elimina i rapporti sugli arresti anomali salvati su questo computer
settings-feedback-help-improve-summary = Invia i rapporti sugli arresti anomali per aiutare a risolvere il problema; disattivato finché non lo attivi
settings-experimental-blur-summary = Il desktop traspare sfocato attraverso la barra superiore, e i menu sono in vetro smerigliato
settings-search-shortcut = Scorciatoia da tastiera
settings-search-tab = Scheda delle impostazioni
settings-search-none = Nessuna impostazione corrisponde a «{ $query }».
settings-search-results = Impostazioni corrispondenti a «{ $query }»

## Quick settings (the panel that slides in from the right)

quick-title = Impostazioni rapide
quick-see-all = Visualizza tutte le impostazioni
quick-reading-pane = Riquadro di lettura
quick-pane-right = A destra dell’elenco
quick-pane-none = Nessuna suddivisione
quick-density = Densità
quick-density-default = Predefinita
quick-density-compact = Compatta
quick-theme = Tema
quick-theme-system = Come il desktop
quick-theme-light = Chiaro
quick-theme-dark = Scuro
quick-desktop-colors = Colori del desktop
quick-desktop-colors-detail = Lo schema di colori e il colore di risalto del desktop
quick-app-names = Nomi delle app
quick-app-names-detail = Nomi sotto le icone delle app all’estrema sinistra
quick-inbox-tabs = Schede della posta in arrivo
quick-inbox-tabs-detail = Le schede del provider di posta di ogni account
quick-choose-tabs = Scegli le schede
quick-choose-tabs-detail = Per account, nelle Impostazioni
quick-sending = Invio
quick-undo-send = Annulla invio
quick-undo-send-off = Disattivato
quick-undo-send-seconds = { $seconds } s
quick-signatures = Firme
quick-signatures-none = Ancora nessuna
quick-signatures-one = { $name }, usata come predefinita
quick-signatures-many = { $count ->
    [one] { $count } firma; { $name } predefinita
    [many] { $count } di firme; { $name } predefinita
   *[other] { $count } firme; { $name } predefinita
}
quick-signatures-no-default = { $count ->
    [one] { $count }, nessuna predefinita
    [many] { $count }, nessuna predefinita
   *[other] { $count }, nessuna predefinita
}
quick-signature-untitled = Senza titolo
quick-threading = Organizzazione in thread delle email
quick-conversation-view = Visualizzazione per conversazione
quick-conversation-view-detail = Raggruppa le risposte alla stessa email
quick-help = Guida
quick-tour = Fai il tour
quick-whats-new = Novità
quick-about = Informazioni su Katna

## Settings: opening at login

settings-open-at-login-failed = Impossibile modificare l’apertura all’accesso: { $error }

## Settings > Appearance > Scaling

scale-letter = A
scale-percent = { $percent }%
scale-reset = Torna al { $percent }%

## Settings > Experimental > Look & Feel

look-intro = Funzionalità ancora in prova. Potrebbero cambiare o essere rimosse.
look-heading = Aspetto e stile
look-window-frame = Cornice della finestra
look-window-frame-detail = Chi disegna la barra del titolo, i pulsanti della finestra, gli angoli e l’ombra.
look-frame-native-kde = Nativa: la cornice di KDE, nel tuo tema di Plasma
look-frame-native = Nativa: la cornice del desktop
look-frame-katna = Katna: la barra superiore diventa la barra del titolo
look-frame-katna-note-named = Katna disegna angoli arrotondati e la propria ombra. La cornice non segue più il tema di { $desktop }; le regole delle finestre restano valide.
look-frame-katna-note = Katna disegna angoli arrotondati e la propria ombra. La cornice non segue più il tema del desktop; le regole delle finestre restano valide.
look-frame-client-side = Il tuo desktop lascia la cornice a ogni app, quindi Katna disegna già la propria.
look-blurred-background = Sfondo sfocato
look-blurred-background-detail = Il desktop traspare sfocato attraverso la barra superiore e le cartelle, e menu e popup sono in vetro smerigliato.
look-blur = Sfoca ciò che sta dietro la finestra
look-blur-detail = La posta resta su riquadri opachi, così il testo mantiene il contrasto
look-blur-off-kde = L’effetto sfocatura di KDE è disattivato. Attiva Sfocatura in Impostazioni di sistema, Gestione delle finestre, Effetti del desktop, quindi riapri Katna Mail.
look-blur-none-gnome = GNOME non sfoca ciò che sta dietro le finestre.
look-blur-none-x11 = Il tuo gestore di finestre non sfoca ciò che sta dietro le finestre.
look-blur-none-wayland = Il tuo compositore non sfoca ciò che sta dietro le finestre.

## Settings > User feedback (crash reports)

feedback-intro-sending = I nuovi rapporti sugli arresti anomali vengono inviati per aiutare a risolvere il problema. Nient’altro lascia questo computer.
feedback-intro-local = Katna non invia nulla a nessuno. I rapporti sugli arresti anomali restano su questo computer, perché tu possa consultarli o allegarli a una segnalazione di bug.
feedback-crash-reports = Rapporti sugli arresti anomali
feedback-crash-reports-detail = Creati quando Katna Mail o il suo servizio in background si arresta in modo anomalo.
feedback-save = Salva i rapporti sugli arresti anomali su questo computer
feedback-save-detail = La tua cartella home, i nomi utente e del computer e gli indirizzi email vengono omessi
feedback-saved = Rapporti sugli arresti anomali salvati
feedback-saved-detail = { $count ->
    [one] Viene conservato solo il più recente.
    [many] Vengono conservati i { $count } di più recenti.
   *[other] Vengono conservati i { $count } più recenti.
}
feedback-help-improve = Aiuta a migliorare Katna
feedback-help-improve-detail = Disattivato finché non lo attivi, e puoi disattivarlo qui in qualsiasi momento.
feedback-send = Invia i rapporti sugli arresti anomali
feedback-send-detail = Il rapporto salvato, esattamente come puoi vederlo qui, viene inviato al sistema di tracciamento degli arresti anomali di Katna (Sentry, nell’UE). Nessun indirizzo IP, messaggio o indirizzo email
feedback-none-saved = Nessun rapporto sugli arresti anomali salvato.
feedback-delete-all = Elimina tutto
feedback-app-daemon = Servizio in background
feedback-report-sent = { $date } · Inviato
feedback-view = Visualizza
feedback-view-tooltip = Apri il rapporto
feedback-copy-tooltip = Copialo per incollarlo in una segnalazione di bug
feedback-copied = Rapporto sull’arresto anomalo copiato.
feedback-deleted-all = Rapporti sugli arresti anomali eliminati.
feedback-read-failed = Impossibile leggere il rapporto sull’arresto anomalo: { $error }
feedback-delete-failed = Impossibile eliminare il rapporto sull’arresto anomalo: { $error }
feedback-delete-all-failed = Impossibile eliminare i rapporti sugli arresti anomali: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _File
desktop-menu-new-message = _Nuovo messaggio
desktop-menu-quit = _Esci
desktop-menu-edit = _Modifica
desktop-menu-undo = _Annulla
desktop-menu-select-all = Seleziona t_utto
desktop-menu-select-none = _Deseleziona
desktop-menu-find = _Trova…
desktop-menu-view = _Visualizza
desktop-menu-folder-list = Mostra l’elenco delle _cartelle
desktop-menu-refresh = A_ggiorna
desktop-menu-go = Va_i
desktop-menu-inbox = Posta in _arrivo
desktop-menu-starred = _Speciali
desktop-menu-sent = _Inviati
desktop-menu-drafts = _Bozze
desktop-menu-all-mail = _Tutti i messaggi
desktop-menu-next = Conversazione s_uccessiva
desktop-menu-previous = Conversazione _precedente
desktop-menu-message = M_essaggio
desktop-menu-open = _Apri
desktop-menu-reply = _Rispondi
desktop-menu-reply-all = Rispondi a _tutti
desktop-menu-forward = _Inoltra
desktop-menu-archive = Arc_hivia
desktop-menu-delete = _Elimina
desktop-menu-spam = Segnala come s_pam
desktop-menu-move-to = _Sposta in…
desktop-menu-mark-read = Segna come già _letto
desktop-menu-mark-unread = Segna come da le_ggere
desktop-menu-star = Aggiungi a Spe_ciali
desktop-menu-important = Contrassegna come i_mportante
desktop-menu-not-important = Contrassegna come _non importante
desktop-menu-settings = Imp_ostazioni
desktop-menu-quick-settings = Impostazioni _rapide
desktop-menu-configure = _Configura Katna Mail…
desktop-menu-help = _Aiuto
desktop-menu-shortcuts = _Scorciatoie da tastiera
desktop-menu-whats-new = _Novità
desktop-menu-about = _Informazioni su Katna

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = Navigazione
shortcut-group-actions = Azioni
shortcut-group-go-to = Vai a
shortcut-group-app = Applicazione

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = Conversazione successiva
shortcut-previous = Conversazione precedente
shortcut-down = Scendi nell’elenco
shortcut-up = Sali nell’elenco
shortcut-first = Primo dell’elenco
shortcut-last = Ultimo dell’elenco
shortcut-page-down = Pagina giù nell’elenco
shortcut-page-up = Pagina su nell’elenco
shortcut-open = Apri conversazione
shortcut-back = Torna all’elenco
shortcut-scroll-down = Scorri verso il basso
shortcut-scroll-up = Scorri verso l’alto
shortcut-scroll-page-down = Scorri di una pagina in basso
shortcut-scroll-page-up = Scorri di una pagina in alto
shortcut-compose = Scrivi
shortcut-reply = Rispondi
shortcut-reply-all = Rispondi a tutti
shortcut-forward = Inoltra
shortcut-archive = Archivia
shortcut-delete = Elimina
shortcut-spam = Segnala come spam
shortcut-move-to = Sposta in
shortcut-mark-read = Segna come già letto
shortcut-mark-unread = Segna come da leggere
shortcut-star = Aggiungi a o rimuovi da Speciali
shortcut-important = Contrassegna come importante
shortcut-not-important = Contrassegna come non importante
shortcut-check = Seleziona la conversazione
shortcut-select-all = Seleziona tutte le conversazioni
shortcut-select-none = Deseleziona tutte le conversazioni
shortcut-undo = Annulla l’ultima azione
shortcut-go-inbox = Posta in arrivo
shortcut-go-starred = Speciali
shortcut-go-sent = Inviati
shortcut-go-drafts = Bozze
shortcut-go-all = Tutti i messaggi
shortcut-search = Cerca nella posta
shortcut-navigation = Mostra o comprimi il menu
shortcut-quick-settings = Impostazioni rapide
shortcut-settings = Tutte le impostazioni
shortcut-shortcuts = Scorciatoie da tastiera
shortcut-reload = Controlla se c’è nuova posta
shortcut-quit = Esci

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } poi { $second }

## Settings > Accounts

accounts-folder-pane = Riquadro delle cartelle
accounts-folder-pane-detail = Di quali account il riquadro a sinistra mostra le cartelle.
accounts-shown-one = Un account alla volta; cambialo dal riquadro dell’account
accounts-shown-all = Tutti gli account, uno dopo l’altro
accounts-row = Account
accounts-row-detail = Rimuovere un account elimina la copia della sua posta che Katna conserva su questo computer. La posta resta sul server.
accounts-none = Ancora nessun account.
accounts-kind-imported = Importato
accounts-picture-reset = Usa l’immagine del desktop
accounts-picture-change = Cambia immagine
accounts-remove = Rimuovi
accounts-delete-all-row = Elimina tutti i dati
accounts-delete-all-row-detail = Ricomincia da capo, come dopo una nuova installazione.
accounts-delete-all-about = Elimina da questo computer tutti gli account, tutta la posta salvata, i contatti e i calendari, l’indice di ricerca, le tue impostazioni e le password salvate. Sui tuoi server di posta non cambia nulla.
accounts-delete-all-open = Elimina tutti i dati di Katna

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } è stato rimosso da Katna.
accounts-removed = { $address } è stato rimosso da Katna. La sua posta è ancora sul server.
accounts-all-deleted = Tutti i dati di Katna sono stati eliminati da questo computer.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = Rimuovere { $address }?
accounts-remove-confirm = Rimuovi account
accounts-removing = Rimozione…
accounts-remove-local-mail = { $folders ->
    [0] Tutta la posta importata in questo account
    [one] Tutta la posta importata in questo account, nella sua cartella
    [many] Tutta la posta importata in questo account, nelle sue { $folders } di cartelle
   *[other] Tutta la posta importata in questo account, nelle sue { $folders } cartelle
}
accounts-remove-local-settings = Le sue impostazioni di Katna
accounts-remove-mail = { $folders ->
    [0] Tutta la posta di questo account salvata da Katna
    [one] Tutta la posta di questo account salvata da Katna nella sua cartella
    [many] Tutta la posta di questo account salvata da Katna nelle sue { $folders } di cartelle
   *[other] Tutta la posta di questo account salvata da Katna nelle sue { $folders } cartelle
}
accounts-remove-outbox = I suoi messaggi in attesa in Posta in uscita
accounts-remove-settings = La sua password salvata e le sue impostazioni di Katna
accounts-delete-all-title = Eliminare tutti i dati di Katna?
accounts-delete-all-confirm = Elimina tutto
accounts-deleting = Eliminazione…
accounts-delete-all-accounts = Tutti gli account, e tutta la posta e gli allegati salvati da Katna
accounts-delete-all-contacts = I contatti, i calendari e l’indice di ricerca
accounts-delete-all-settings = Tutte le impostazioni, le firme e le scorciatoie da tastiera
accounts-delete-all-passwords = Tutte le password salvate
accounts-deleted-heading = Eliminato da questo computer:
accounts-cannot-undo = Questa operazione non può essere annullata.
accounts-server-delete-all = Sui tuoi server di posta non cambia nulla: la tua posta resta lì, e se aggiungi di nuovo un account viene scaricata di nuovo. La posta importata da file è solo in Katna; i file non vengono toccati.
accounts-server-local = Questa posta è stata importata da file, quindi Katna ne ha l’unica copia. I file di origine non vengono toccati; importali di nuovo per recuperarla.
accounts-server-remove = Sul server di posta non cambia nulla: la tua posta resta lì, e se aggiungi di nuovo l’account viene scaricata di nuovo.
accounts-confirm-word = elimina
accounts-confirm-placeholder = Digita «{ accounts-confirm-word }»
accounts-confirm-prompt = Per confermare, digita «{ accounts-confirm-word }»:
accounts-cancel = Annulla
