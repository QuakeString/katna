# Katna Mail, Italian (Italiano).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

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
