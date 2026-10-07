# Katna Mail, Italian (Italiano).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = Etichette
nav-folders = Cartelle
nav-label-new = Crea nuova etichetta
nav-folder-new = Crea nuova cartella
nav-menu-check-mail = Controlla nuova posta
nav-menu-check-inbox = Controlla questa posta in arrivo
nav-unified-leave-out = Escludi dalla Posta in arrivo unificata
nav-unified-bring-back = Includi di nuovo nella Posta in arrivo unificata
nav-menu-sign-in-again = Accedi di nuovo
nav-menu-new-mail = Nuova email da questo account
nav-menu-account-settings = Impostazioni dell’account
nav-account-checked = Sincronizzato · controllato { $ago }
nav-account-in-sync = Sincronizzato
nav-account-connecting = Connessione…
nav-account-offline = Offline, nuovo tentativo
nav-account-signed-out = Accesso a { $provider } scaduto
nav-account-password-refused = Password rifiutata
nav-account-storage = { $used } di { $total } usati
nav-menu-new-subfolder = Nuova cartella al suo interno
nav-menu-new-sublabel = Nuova etichetta al suo interno
nav-menu-rename = Rinomina
nav-menu-delete = Elimina
nav-menu-empty-trash = Svuota il Cestino
nav-account-unnamed = Account { $number }
nav-all-accounts = Tutti gli account
nav-expand = Mostra cartelle
nav-collapse = Nascondi cartelle
storage-used = { $percent }% di { $total } in uso
storage-used-detail = { $address }: { $used } di { $total } in uso

## Special folders (the user's own folders keep their names)

folder-inbox = Posta in arrivo
folder-starred = Speciali
folder-snoozed = Posticipati
folder-unread = Da leggere
folder-important = Importanti
folder-drafts = Bozze
folder-sent = Inviati
folder-archive = Archivio
folder-spam = Spam
folder-trash = Cestino
folder-all-mail = Tutti i messaggi
folder-scheduled = Programmati
folder-waiting = In attesa di risposta
folder-waiting-short = In attesa
folder-reminders = Promemoria
folder-outbox = Posta in uscita
folder-activity = Attività

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
label-rename-title = Rinomina etichetta
label-folder-rename-title = Rinomina cartella
label-rename = Rinomina
label-renaming = Ridenominazione…
label-renamed = Etichetta rinominata in «{ $name }».
label-folder-renamed = Cartella rinominata in «{ $name }».

## Deleting a folder or label (asked first)

folder-delete-title = Eliminare «{ $name }»?
folder-delete-body = { $count ->
    [0] Non contiene posta. La cartella viene rimossa dal server, quindi scompare anche dalla webmail e dal telefono.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] La sua { $count } conversazione va nel Cestino, così puoi ancora recuperarla.
            [many] Le sue { $count } di conversazioni vanno nel Cestino, così puoi ancora recuperarle.
           *[other] Le sue { $count } conversazioni vanno nel Cestino, così puoi ancora recuperarle.
        }
       *[message] { $count ->
            [one] Il suo { $count } messaggio va nel Cestino, così puoi ancora recuperarlo.
            [many] I suoi { $count } di messaggi vanno nel Cestino, così puoi ancora recuperarli.
           *[other] I suoi { $count } messaggi vanno nel Cestino, così puoi ancora recuperarli.
        }
    } La cartella viene rimossa dal server, quindi scompare anche dalla webmail e dal telefono.
}
folder-delete-forever-body = { $count ->
    [0] Non contiene posta. La cartella viene rimossa dal server, quindi scompare anche dalla webmail e dal telefono.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] La sua { $count } conversazione viene eliminata definitivamente; questo account non ha un Cestino.
            [many] Le sue { $count } di conversazioni vengono eliminate definitivamente; questo account non ha un Cestino.
           *[other] Le sue { $count } conversazioni vengono eliminate definitivamente; questo account non ha un Cestino.
        }
       *[message] { $count ->
            [one] Il suo { $count } messaggio viene eliminato definitivamente; questo account non ha un Cestino.
            [many] I suoi { $count } di messaggi vengono eliminati definitivamente; questo account non ha un Cestino.
           *[other] I suoi { $count } messaggi vengono eliminati definitivamente; questo account non ha un Cestino.
        }
    } La cartella viene rimossa dal server, quindi scompare anche dalla webmail e dal telefono.
}
folder-delete-label-body = L’etichetta viene rimossa. La sua posta resta in Tutti i messaggi e nelle sue altre etichette.
folder-delete-confirm = Elimina cartella
folder-delete-label-confirm = Elimina etichetta
folder-deleted = Cartella «{ $name }» eliminata
label-deleted = Etichetta «{ $name }» eliminata
