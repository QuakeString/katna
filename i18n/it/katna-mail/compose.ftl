# Katna Mail, Italian (Italiano).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = Nuovo messaggio
compose-restore = Ripristina
compose-minimize = Riduci a icona
compose-exit-full-screen = Esci da schermo intero
compose-open-window = Apri in una nuova finestra
compose-save-close = Salva e chiudi
compose-back-to-mail = Torna alla finestra della posta
compose-pop-out-reply = Apri la risposta in una finestra
compose-edit-recipients = Modifica destinatari
compose-summary-cc = Cc: { $names }
compose-summary-bcc = Ccn: { $names }
compose-more-recipients = altri { $count }
compose-show-trimmed = Mostra contenuti tagliati
compose-hide-trimmed = Nascondi contenuti tagliati
compose-remove-trimmed = Rimuovi testo citato
compose-trimmed-removed = Testo citato rimosso

## Recipients and subject

compose-to = A
compose-cc = Cc
compose-bcc = Ccn
compose-from = Da
compose-from-choose = Invia da un altro account
compose-recipients = Destinatari
compose-subject = Oggetto

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = Prima invia o elimina il messaggio aperto.
compose-bad-address = «{ $address }» non è un indirizzo email.
compose-no-recipients = Aggiungi almeno un destinatario.
compose-attachments-too-large = Gli allegati occupano { $size }; i server di posta accettano fino a { $limit }.
compose-no-account = Aggiungi un account da cui inviare la posta.
compose-past-time = Scegli un orario nel futuro.
compose-scheduling = Programmazione in corso…
compose-sending = Invio in corso…
compose-scheduled = Invio programmato per { $when }
compose-sent-archived = Inviato e archiviato
compose-sent = Messaggio inviato
compose-discarded = Bozza eliminata
compose-draft-saved = Bozza salvata
compose-draft-failed = Impossibile salvare la bozza: { $error }
compose-draft-not-opened = Impossibile aprire la bozza.

## Attachments

compose-picker-insert = Inserisci
compose-picker-attach = Allega
compose-file-too-large = { $name } è troppo grande: un messaggio può contenere fino a { $limit }.
compose-attachment-size = ({ $size })
compose-remove-attachment = Rimuovi allegato
compose-attachments-total = { $count ->
    [one] { $count } file, { $size }
    [many] { $count } di file, { $size }
   *[other] { $count } file, { $size }
}
compose-drive-note = { $name } supera { $limit }, quindi va nel tuo Google Drive e il messaggio contiene un link.
compose-drive-tip = Nel tuo Google Drive; il messaggio contiene un link
compose-drive-uploading = Caricamento in corso: { $percent }%
compose-drive-allow = Consenti Drive
compose-drive-allow-tip = Accedi di nuovo con Google per permettere a Katna di mettere i file grandi nel tuo Drive
compose-drive-retry = Riprova
compose-drive-sends-when-uploaded = Invio dopo il caricamento di { $name }
compose-drive-not-uploaded = { $name } non è ancora su Google Drive
compose-drive-share-failed = Impossibile condividere i file su Google Drive: { $error }
compose-drive-share-title = Condividere i file con tutti?
compose-drive-share-text = { $count ->
    [one] Google Drive non può condividere i file con { $addresses }, che non ha un account Google. In alternativa, chiunque abbia il link potrà aprirli.
    [many] Google Drive non può condividere i file con { $addresses }, che non hanno un account Google. In alternativa, chiunque abbia il link potrà aprirli.
   *[other] Google Drive non può condividere i file con { $addresses }, che non hanno un account Google. In alternativa, chiunque abbia il link potrà aprirli.
}
compose-drive-share-link = Condividi con link
compose-drive-send-without = Invia senza condividere
compose-drive-share-cancel = Annulla
compose-drive-card-detail = { $size } · Google Drive
compose-drop-files = Trascina qui i file
compose-drop-here = Rilascia qui
compose-paste-keep-formatting = Mantieni formattazione
compose-paste-table = Tabella
compose-paste-picture = Immagine
compose-paste-plain-text = Testo semplice
compose-paste-inline = Nel testo
compose-paste-attachment = Allegato

## Encryption and signing (the toggles by the recipients)

compose-encrypt = Crittografa
compose-encrypted = Crittografato: solo i destinatari possono leggerlo
compose-sign = Firma
compose-signed = Firmato: i destinatari possono verificare che viene da te
compose-track = Traccia aperture e clic
compose-tracked = Tracciato: vedi quando ogni destinatario lo apre o segue un link
compose-track-clicks = Traccia i clic sui link (il testo semplice non può mostrare le aperture)
compose-tracked-clicks = Tracciato: vedi quando ogni destinatario segue un link
compose-track-sign-in = Accedi a un account Katna per tracciare aperture e clic
compose-receipt = Richiedi una conferma di lettura
compose-receipt-on = Conferma di lettura richiesta: l’app del destinatario potrebbe chiedergli di inviarla
compose-delivery = Richiedi una conferma di consegna
compose-delivery-on = Conferma di consegna richiesta: il tuo server di posta ti invierà un’email quando il server di ogni destinatario la accetta
compose-delivery-unavailable = Il tuo server di posta non invia conferme di consegna

## Spelling

spell-no-dictionary = Non è installato alcun dizionario ortografico per { $language } (ad esempio hunspell-en_us).
spell-dictionary-error = Dizionario ortografico: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = «{ $words }»
grammar-add = Aggiungi «{ $words }»
grammar-remove = Rimuovi «{ $words }»
grammar-ignore = Ignora

## Send checks (asked before a message goes out)

send-check-attachment-title = Volevi allegare dei file?
send-check-attachment-text = Hai menzionato un allegato, ma non è allegato nulla.
send-check-attach = Allega un file
send-check-subject-title = Inviare senza oggetto?
send-check-subject-text = Questo messaggio non ha oggetto.
send-check-add-subject = Aggiungi oggetto
send-check-send-anyway = Invia comunque
recipient-not-valid = Indirizzo email non valido
recipient-show-address = Mostra indirizzo
recipient-remove = Rimuovi
recipient-bad-title = Controlla l’indirizzo
recipient-bad-text = «{ $address }» non è un indirizzo email valido. Correggilo o rimuovilo prima di inviare.
recipient-bad-fix = Correggi
