# Katna Mail, Italian (Italiano).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = Cerca file

## Left side (and chips on a phone)

files-all = Tutti i file
files-pictures = Immagini
files-pdfs = PDF
files-documents = Documenti
files-sheets = Fogli di calcolo
files-slides = Presentazioni
files-other = Altro
files-accounts = Account
files-drives = Unità
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = Condivisi con me
files-shown = Mostrati
files-received = Ricevuti
files-sent = Inviati da me

## Over the files

files-count = { $count ->
    [one] { $count } file · { $size }
    [many] { $count } di file · { $size }
   *[other] { $count } file · { $size }
}
files-anyone = Chiunque
files-from-person = Da { $name }
files-time-any = Qualsiasi data
files-time-today = Oggi
files-time-yesterday = Ieri
files-time-this-week = Questa settimana
files-time-last-week = Settimana scorsa
files-time-this-month = Questo mese
files-time-last-month = Mese scorso
files-time-between = { $first } – { $last }
files-time-hint = Fai clic su un giorno o trascina su più giorni
files-time-summary = { $count ->
    [one] { $days } · { $count } file
    [many] { $days } · { $count } di file
   *[other] { $days } · { $count } file
}
files-time-clear = Cancella
files-time-month-back = Mese precedente
files-time-month-on = Mese successivo
files-time-wheel = Scorri per spostare queste date mantenendone la durata
files-sort-newest = Prima i più recenti
files-sort-oldest = Prima i meno recenti
files-sort-largest = Prima i più grandi
files-sort-name = Per nome
files-grid = Schede
files-list = Elenco
files-this-week = Questa settimana
files-undated = Senza data
files-me = Io
files-no-subject = (nessun oggetto)
files-loading = Raccolta dei file dalla tua posta…
files-empty = I file della tua posta compaiono qui.
files-none-match = Nessun file corrispondente.
files-load-failed = Lettura dei file non riuscita: { $error }

## A file's menu and buttons

files-open = Apri
files-open-with = Apri con…
files-save = Salva…
files-show-mail = Mostra l’email
files-mail-window = Apri l’email in una nuova finestra
files-forward = Inoltra il file
files-from-them = File di { $name }
files-copy-name = Copia nome del file
files-name-copied = Nome del file copiato
files-downloading = Download dell’email…
files-download-failed = Impossibile scaricare questa email.

## A cloud drive in place of the mail files

files-drive-mine = Il mio Drive
files-drive-mine-onedrive = I miei file
files-drive-results = «{ $words }»
files-drive-count = { $folders ->
    [0] { $files ->
        [one] 1 file
        [many] { $files } di file
       *[other] { $files } file
    }
    [one] 1 cartella · { $files ->
        [one] 1 file
        [many] { $files } di file
       *[other] { $files } file
    }
    [many] { $folders } di cartelle · { $files ->
        [one] 1 file
        [many] { $files } di file
       *[other] { $files } file
    }
   *[other] { $folders } cartelle · { $files ->
        [one] 1 file
        [many] { $files } di file
       *[other] { $files } file
    }
}
files-drive-folders = Cartelle
files-drive-files = File
files-drive-folder = Cartella
files-drive-meta = { $what } · Modificato il { $date }
files-drive-as-link = { $what } · come link
files-drive-google-doc = Documento Google
files-drive-google-sheet = Foglio Google
files-drive-google-slides = Presentazione Google
files-drive-google-drawing = Disegno Google
files-drive-fetching = Recupero…
files-drive-loading = Apertura dell’unità…
files-drive-empty = Questa cartella è vuota.
files-drive-unreachable = Impossibile raggiungere { $drive }.
files-drive-try-again = Riprova
files-drive-needs-permission = Katna ha bisogno del tuo permesso una sola volta per mostrare questa unità. Accedi di nuovo e consenti a Katna di vedere i tuoi file.
files-drive-allow = Consenti
files-drive-allow-failed = L’accesso non è stato completato, quindi l’unità resta chiusa.
files-drive-attach = Allega
files-drive-more = Altro
files-drive-download = Scarica…
files-drive-open-web = Apri in { $drive }
files-drive-copy-link = Copia link
files-drive-link-copied = Link copiato
files-drive-share = Condividi…
files-drive-rename = Rinomina
files-drive-trash = Sposta nel cestino
files-drive-trashed = «{ $name }» è nel cestino di { $drive }
files-drive-renamed = Rinominato in «{ $name }»
files-drive-getting = Recupero di { $name } da { $drive }…
files-drive-get-failed = Impossibile recuperare { $name }: { $error }
files-drive-upload = Carica
files-drive-upload-files = Carica file
files-drive-upload-folder = Carica cartella
files-drive-upload-failed = Impossibile caricare { $name }: { $error }
files-drive-upload-needs = Per caricare, Katna ha bisogno del tuo permesso una sola volta: premi Consenti in Impostazioni › App predefinite › Pagina File.

## The Share dialog of a drive file or folder

files-share-title = Condividi «{ $name }»
files-share-add = Aggiungi persone per nome o indirizzo
files-share-not-address = «{ $text }» non è un indirizzo email
files-share-notify = Lascia che { $drive } invii anche a loro un’email
files-share-people = Persone con accesso
files-share-general = Accesso generale
files-share-loading = Lettura di chi ha accesso…
files-share-restricted = Limitato
files-share-restricted-about = Solo le persone con accesso possono aprirlo con il link
files-share-anyone = Chiunque abbia il link
files-share-anyone-can = { $role ->
    [editor] Chiunque abbia il link può modificare
    [commenter] Chiunque abbia il link può commentare
   *[viewer] Chiunque abbia il link può visualizzare
}
files-share-anyone-about = { $role ->
    [editor] Chiunque su internet abbia il link può modificare
    [commenter] Chiunque su internet abbia il link può commentare
   *[viewer] Chiunque su internet abbia il link può visualizzare
}
files-share-role-owner = Proprietario
files-share-role-editor = Editor
files-share-role-commenter = Commentatore
files-share-role-viewer = Visualizzatore
files-share-you = { $name } (tu)
files-share-domain = Tutti in { $domain }
files-share-inherited = Accesso da una cartella che lo contiene
files-share-remove = Rimuovi accesso
files-share-copy-link = Copia link
files-share-share = Condividi
files-share-done = Fine
files-share-close = Chiudi
files-share-sharing = Condivisione…
files-share-shared = { $count ->
    [one] Condiviso con 1 persona
    [many] Condiviso con { $count } di persone
   *[other] Condiviso con { $count } persone
}
files-share-refused = { $drive } non è riuscito a condividere con { $addresses }
files-share-failed = Impossibile cambiare la condivisione: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] Caricamento di 1 elemento
    [many] Caricamento di { $count } di elementi
   *[other] Caricamento di { $count } elementi
}
files-tray-done = { $count ->
    [one] 1 caricamento completato
    [many] { $count } di caricamenti completati
   *[other] { $count } caricamenti completati
}
files-tray-some-failed = { $done } caricati, { $failed } non riusciti
files-tray-minutes-left = { $minutes ->
    [one] Circa un minuto rimanente
    [many] Circa { $minutes } di minuti rimanenti
   *[other] Circa { $minutes } minuti rimanenti
}
files-tray-seconds-left = Meno di un minuto rimanente
files-tray-starting = Avvio…
files-tray-cancel-all = Annulla tutto
files-tray-cancel = Annulla
files-tray-fold = Nascondi l’elenco
files-tray-unfold = Mostra l’elenco
files-tray-close = Chiudi
files-tray-progress = { $place } · { $sent } di { $size }
files-tray-in = In { $place }
files-tray-cancelled = Annullato
