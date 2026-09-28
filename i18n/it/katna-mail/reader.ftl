# Katna Mail, Italian (Italiano).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Chiudi
reader-back = Indietro
reader-mark-unread = Segna come da leggere
reader-move-to = Sposta in
reader-more = Altro
reader-original-colors = Mostra i colori originali
reader-dark-colors = Mostra in colori scuri
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
reader-sending = Invio in corso…
reader-me = me
reader-to = a { $names }
reader-to-label = a
reader-tick-delivered = Consegnato { $when }
reader-tick-no-bounce = Inviato { $when }; non è tornato alcun avviso di mancata consegna, quindi è molto probabile che sia arrivato
reader-tick-bounced = Non consegnato: respinto { $when }
reader-tick-read = Letto { $when } (conferma di lettura)
reader-tick-opened = Aperto, l’ultima volta { $when } (tracciamento delle aperture)
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
tracking-opened = { $who } l’ha aperto { $count ->
    [one] una volta
    [many] { $count } di volte
   *[other] { $count } volte
}, l’ultima { $when }
tracking-opens-clicks = { $who } l’ha aperto { $opens ->
    [one] una volta
    [many] { $opens } di volte
   *[other] { $opens } volte
} e ha seguito un link { $clicks ->
    [one] una volta
    [many] { $clicks } di volte
   *[other] { $clicks } volte
}, l’ultima { $when }
tracking-clicked = { $who } ha seguito un link { $clicks ->
    [one] una volta
    [many] { $clicks } di volte
   *[other] { $clicks } volte
}, l’ultima { $when }
tracking-maybe-opened = { $who } potrebbe averlo aperto (Apple Mail carica le immagini per la privacy)
tracking-seen-none = Nessuno l’ha ancora aperto né ha seguito un link
tracking-receipt = { $who } ha inviato una conferma di lettura
tracking-receipt-displayed = Conferma di lettura: { $who } ha aperto il tuo messaggio
tracking-receipt-other = Conferma di lettura: { $who } ha eliminato o gestito il tuo messaggio senza aprirlo

## Remote images and pictures

remote-hidden = Le immagini in questo messaggio sono nascoste.
remote-hidden-unconfirmed = Immagini nascoste: non è stato possibile confermare il mittente.
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
print-preview-title = Anteprima di stampa
print-preview-laying-out = Impaginazione in corso…
print-preview-pages = { $count ->
    [one] { $count } pagina
    [many] { $count } di pagine
   *[other] { $count } pagine
}
print-preview-more = { $count ->
    [one] e { $count } pagina in più
    [many] e altre { $count } di pagine
   *[other] e altre { $count } pagine
}
print-preview-failed = impossibile mostrare le pagine
print-preview-paper = Carta
print-preview-a4 = A4
print-preview-letter = Lettera
print-preview-layout = Impaginazione
print-preview-as-shown = Come mostrato
print-preview-simple = Testo semplice
print-preview-backgrounds = Sfondi
print-preview-cancel = Annulla
print-preview-print = Stampa
print-not-downloaded = (Non ancora scaricato.)
print-encrypted = (Crittografato. Aprilo in Katna Mail per stamparne il testo.)
print-to = A: { $addresses }
print-cc = Cc: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Apri questo messaggio per leggerne gli allegati.
text-copy = Copia
text-select-all = Seleziona tutto
