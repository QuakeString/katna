# Katna Mail, Italian (Italiano).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

chat-heading = Lettura
chat-view = Conversazioni come chat
chat-view-detail = La posta tra persone si legge come una chat di gruppo: una bolla per ogni email con solo ciò che è stato scritto, le tue a destra. Le newsletter mantengono la vista consueta.
chat-view-switch = Mostra le conversazioni come chat
chat-view-switch-detail = La posta citata e le firme restano dietro ··· in ogni bolla

chat-switch-chat = Chat
chat-switch-mail = Email
chat-people = { $names } e tu · { $count ->
    [one] { $count } email
    [many] { $count } di email
   *[other] { $count } email
}
chat-people-heading = { $count ->
    [one] In questa chat · { $count } persona
    [many] In questa chat · { $count } di persone
   *[other] In questa chat · { $count } persone
}
chat-member-mails = { $count ->
    [0] Nessuna email
    [one] { $count } email
    [many] { $count } di email
   *[other] { $count } email
}
chat-today = Oggi
chat-yesterday = Ieri
chat-added = { $who } ha aggiunto { $names }
chat-renamed = { $who } ha cambiato l’oggetto in «{ $subject }»
chat-you = Tu
chat-not-downloaded = Non ancora scaricata
chat-forwarded = Inoltrata
chat-show-quoted = Mostra la posta citata e la firma
chat-hide-quoted = Nascondi la posta citata e la firma
chat-hide-dots = Nascondi ···
chat-show-card = Mostra la sua scheda
chat-reply-all = Rispondi a tutti
chat-more = Altro
chat-reply-only = Rispondi solo a { $name }
chat-forward = Inoltra
chat-copy-text = Copia testo
chat-show-as-mail = Mostra come email
chat-go-down = Vai alla posta più recente
chat-pin = Fissa in alto
chat-pin-file = Fissa file in alto
chat-unpin = Sblocca
chat-unpin-file = Sblocca file
chat-pinned-of = Fissato { $at } di { $count }
chat-pins-all = Tutti gli elementi fissati
chat-pins-heading = Fissati · { $count } di { $most }
chat-pins-drag = Trascina per riordinare
chat-pin-from-mail = Email di { $name } · { $when }
chat-pin-from-file = File di { $name } · { $when }
chat-pin-from-text = Testo di { $name } · { $when }
chat-pins-full = Questa chat ha già 5 elementi fissati
chat-pins-replace-title = Sostituisci un elemento fissato
chat-pins-replace-hint = Una chat può avere fino a 5 elementi fissati. Scegli quello da togliere.
chat-pins-replace = Sostituisci
chat-pins-cancel = Annulla
chat-undo = Annulla

chat-reply-to = Rispondi a { $names }
chat-send = Invia (Ctrl+Enter). Clic destro o tieni premuto per altre opzioni
chat-send-now = Invia ora
chat-attach = Allega
chat-attach-photo = Foto
chat-attach-file = File
chat-attach-library = Da File
chat-attach-template = Modello
chat-attach-signature = Firma
chat-replying-to = Risposta a { $name }
chat-reply-newest = Rispondi all’email più recente

## The attach picker (paperclip > From Files)

picker-title = Allega da File
picker-search = Cerca nomi, persone, oggetti
picker-search-drive = Cerca in questa unità
picker-mail-files = File della posta
picker-this-chat = Questa conversazione
picker-this-computer = Questo computer…
picker-in-chat = IN QUESTA CONVERSAZIONE
picker-recent = RECENTI
picker-preview = Anteprima
picker-cancel = Annulla
picker-attach = Allega
picker-attach-count = Allega { $count }
picker-selected = { $count } selezionati
picker-of-limit = di { $limit }
picker-in-mail = { $size } nell’email
picker-drive-links = { $count ->
    [one] 1 come link di Google Drive
    [many] { $count } di file come link di Google Drive
   *[other] { $count } come link di Google Drive
}
picker-onedrive-links = { $count ->
    [one] 1 come link di OneDrive
    [many] { $count } di file come link di OneDrive
   *[other] { $count } come link di OneDrive
}
picker-over = { $size }, più dei { $limit } che un’email può contenere
picker-getting = { $count ->
    [one] Recupero del file dall’unità…
    [many] Recupero di { $count } di file dall’unità…
   *[other] Recupero di { $count } file dall’unità…
}
picker-some-failed = { $count ->
    [one] Impossibile leggere un file
    [many] Impossibile leggere { $count } di file
   *[other] Impossibile leggere { $count } file
}
