# Katna Mail, Italian (Italiano): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Contatti
contacts-frequent = Frequenti
contacts-other = Altri contatti
contacts-other-about = Persone a cui hai scritto da Gmail ma che non hai salvato
contacts-other-email = Invia e-mail
contacts-other-empty = Nessun altro contatto. Le persone a cui scrivi da Gmail senza salvarle compaiono qui.
contacts-other-allow = Per vedere gli altri contatti, accedi di nuovo al tuo account Gmail e consenti a Katna di vederli.
contacts-labels = Etichette
contacts-label-options = Opzioni etichetta
contacts-label-rename = Rinomina etichetta
contacts-label-email = Scrivi a tutti
contacts-label-delete = Elimina etichetta
contacts-label-new = Nuova etichetta
contacts-label-name = Nome etichetta
contacts-label-button = Etichetta
contacts-label-menu = Etichetta come:
contacts-label-added = Aggiunto a { $name }
contacts-label-removed = Rimosso da { $name }
contacts-label-renamed = Etichetta rinominata in { $name }
contacts-label-deleted = Etichetta eliminata: { $name }
contacts-label-no-email = Nessuno con questa etichetta ha un indirizzo email
contacts-manage = Correggi e gestisci
contacts-merge = Unisci e correggi
contacts-merge-about = { $count ->
    [one] { $count } suggerimento: contatti che sembrano la stessa persona
    [many] { $count } di suggerimenti: contatti che sembrano la stessa persona
   *[other] { $count } suggerimenti: contatti che sembrano la stessa persona
}
contacts-merge-none = Nessun duplicato. I contatti con lo stesso nome o numero di telefono vengono mostrati qui.
contacts-merge-count = { $count ->
    [one] { $count } contatto
    [many] { $count } di contatti
   *[other] { $count } contatti
}
contacts-merge-all = Unisci tutti
contacts-merge-button = Unisci
contacts-merge-dismiss = Ignora
contacts-merged = { $count ->
    [1] Contatti uniti
    [one] { $count } unione eseguita
    [many] { $count } di unioni eseguite
   *[other] { $count } unioni eseguite
}
contacts-import = Importa
contacts-export = Esporta
contacts-import-file = Importa contatti da un file vCard o CSV
contacts-imported = { $count ->
    [one] { $count } contatto importato in { $place }
    [many] { $count } di contatti importati in { $place }
   *[other] { $count } contatti importati in { $place }
}
contacts-imported-some = { $count ->
    [one] { $count } contatto importato in { $place }; { $skipped } già salvati, esclusi
    [many] { $count } di contatti importati in { $place }; { $skipped } già salvati, esclusi
   *[other] { $count } contatti importati in { $place }; { $skipped } già salvati, esclusi
}
contacts-import-none = Nessun contatto trovato in { $name }
contacts-import-all-saved = Tutte le persone in { $name } sono già salvate
contacts-import-failed = Impossibile leggere { $name }: { $error }
contacts-exported = { $count ->
    [one] { $count } contatto esportato in { $path }
    [many] { $count } di contatti esportati in { $path }
   *[other] { $count } contatti esportati in { $path }
}
contacts-export-none = Nessun contatto da esportare
contacts-export-failed = Impossibile esportare i contatti: { $error }
contacts-print = Stampa
contacts-print-title = Contatti
contacts-print-none = Nessun contatto da stampare
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = Compleanno: { $day }
contacts-print-nickname = Soprannome: { $name }
contacts-create = Crea contatto

## Search and the list

contacts-search = Cerca nei contatti
contacts-loading = Caricamento dei contatti…
contacts-empty = Nessun contatto salvato finora. I contatti che salvi in Gmail, Outlook o nel tuo servizio di posta compaiono qui.
contacts-empty-no-books = I contatti dei tuoi account compariranno qui dopo la sincronizzazione.
contacts-none-found = Nessun contatto corrisponde alla ricerca.
contacts-starred = { $count ->
    [one] Contatto speciale ({ $count })
    [many] Contatti speciali ({ $count })
   *[other] Contatti speciali ({ $count })
}
contacts-count = Contatti ({ $count })
contacts-col-name = Nome
contacts-col-email = Email
contacts-col-phone = Numero di telefono
contacts-col-job = Posizione e azienda
contacts-col-labels = Etichette

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Consenti a Katna di leggere i contatti di { $address }.
contacts-allow-many = { $more ->
    [one] Consenti a Katna di leggere i contatti di { $address } e di { $more } altro account.
    [many] Consenti a Katna di leggere i contatti di { $address } e di { $more } di altri account.
   *[other] Consenti a Katna di leggere i contatti di { $address } e di altri { $more } account.
}
contacts-allow-button = Consenti

## A contact's page

contacts-back = Torna ai contatti
contacts-edit = Modifica
contacts-delete = Elimina
contacts-qr = Condividi come codice QR
contacts-qr-about = Inquadra il codice con la fotocamera di un telefono per salvare il contatto.
contacts-qr-too-long = Questo contatto ha troppi dettagli per stare in un codice QR.
contacts-qr-done = Fine
contacts-deleted = Eliminato: { $name }
contacts-added = { $name } aggiunto ai contatti
contacts-find-mail = Posta
contacts-details = Dettagli del contatto
contacts-saved-in = Salvato in
contacts-notes = Note
contacts-birthday = Compleanno
contacts-nickname = Soprannome
contacts-this-computer = Questo computer
contacts-kind-home = Casa
contacts-kind-work = Lavoro
contacts-kind-mobile = Cellulare
contacts-kind-other = Altro
contacts-source-google = Google Contacts
contacts-source-microsoft = Contatti Outlook
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = Crea contatto
contacts-edit-title = Modifica contatto
contacts-edit-save = Salva
contacts-edit-saving = Salvataggio in corso…
contacts-edit-cancel = Annulla
contacts-saved = Contatto salvato
contacts-edit-save-to = Salva in
contacts-edit-changes-go-to = Le modifiche vengono salvate in { $place }.
contacts-edit-given = Nome
contacts-edit-family = Cognome
contacts-edit-company = Azienda
contacts-edit-job = Qualifica
contacts-edit-email = Email
contacts-edit-phone = Telefono
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = Aggiungi email
contacts-edit-add-phone = Aggiungi telefono
contacts-edit-street = Indirizzo
contacts-edit-city = Città
contacts-edit-postcode = CAP
contacts-edit-country = Paese
contacts-edit-birthday = Compleanno (YYYY-MM-DD)
contacts-edit-empty = Aggiungi prima un nome, un’email o un numero di telefono.
