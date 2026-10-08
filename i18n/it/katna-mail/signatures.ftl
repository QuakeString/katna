# Katna Mail, Italian (Italiano).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The signature being edited

signature-placeholder = Il tuo nome e tutto ciò che vuoi aggiungere sotto

## Its formatting bar

signature-bold = Grassetto
signature-italic = Corsivo
signature-underline = Sottolineato
signature-link = Link
signature-link-apply = Applica
signature-picture = Inserisci immagine
signature-align-left = Allinea a sinistra
signature-align-center = Allinea al centro
signature-align-right = Allinea a destra
signature-numbered-list = Elenco numerato
signature-bulleted-list = Elenco puntato
signature-remove-formatting = Rimuovi formattazione

## Adding a picture

signature-picture-choose = Inserisci
signature-picture-too-big = Le immagini in una firma possono arrivare al massimo a { $size }.
signature-picture-kind = Scegli un’immagine PNG, JPEG, GIF o WebP.
signature-picture-unreadable = { $name }: { $error }

## Layouts

signature-layout = Layout
signature-layout-own = Il tuo
signature-layout-classic = Classico
signature-layout-logo-left = Logo a sinistra
signature-layout-photo = Foto
signature-layout-band = Fascia di colore
signature-layout-one-line = Una riga
signature-layout-centred = Centrato
signature-layout-banner = Con banner
signature-layout-underline = Sottolineatura
signature-layout-side-bar = Barra laterale
signature-layout-card = Biglietto
signature-layout-monogram = Monogramma
signature-layout-plain = Testo semplice
signature-layout-mobile-label = Cell:
signature-layout-office-label = Uff:
signature-layout-email-label = E:
signature-layout-name = Nome
signature-layout-job = Ruolo
signature-layout-company = Azienda
signature-layout-mobile = Cellulare
signature-layout-office = Ufficio
signature-layout-email = Email
signature-layout-website = Sito web
signature-layout-address = Indirizzo
signature-layout-pictures = Immagini
signature-layout-logo = Logo
signature-layout-photo-picture = Foto
signature-layout-banner-picture = Banner
signature-layout-remove-picture = Rimuovi
signature-layout-pages = Pagine
signature-layout-page-placeholder = Aggiungi l’indirizzo di una pagina
signature-layout-colour = Colore
signature-layout-picture-failed = Impossibile usare { $name } come immagine.
signature-layout-preview = Come la vede chi legge
signature-layout-light = Chiaro
signature-layout-dark = Scuro
signature-layout-text = Testo semplice
signature-layout-inside = Le immagini vengono inviate dentro l’email, quindi si vedono anche dove le immagini dal web sono bloccate. Questa aggiunge { $size } a ogni email.
signature-layout-free = Vuoi qualcos’altro?
signature-layout-edit = Modifica a mano
signature-layout-edit-confirm = Modificarla a mano? I campi e il layout scompaiono, e mantiene il suo aspetto per quanto l’editor riesce a conservarlo.
signature-layout-use-confirm = Usare il layout { $layout }? Sostituisce questa firma, compilato a partire da essa.
signature-layout-use = Usa layout
signature-layout-cancel = Annulla

## Paste HTML

signature-html-title = Incolla HTML
signature-html-subtitle = Per una firma progettata altrove
signature-html-placeholder = Incolla qui l’HTML della firma
signature-html-name = Incollata
signature-html-new = Salvata come nuova firma, «{ $name }»
signature-html-replaces = Sovrascrive «{ $name }»
signature-html-cancel = Annulla
signature-html-save = Salva
signature-html-fetching = Scaricamento delle immagini…
signature-html-pictures-inside = { $count ->
    [one] { $count } immagine scaricata e inserita nell’email ({ $size })
    [many] { $count } di immagini scaricate e inserite nell’email ({ $size })
   *[other] { $count } immagini scaricate e inserite nell’email ({ $size })
}
signature-html-pictures-web = { $count ->
    [one] Impossibile scaricare { $count } immagine, quindi chi legge la carica dal web
    [many] Impossibile scaricare { $count } di immagini, quindi chi legge le carica dal web
   *[other] Impossibile scaricare { $count } immagini, quindi chi legge le carica dal web
}
signature-html-removed = Rimossi script, moduli e pixel di tracciamento, che le app di posta bloccano comunque
signature-html-style-sheet = Escluso un foglio di stile: la posta mantiene solo gli stili scritti su ogni elemento
signature-html-links = Rimossi i link che non portavano a un sito web, a un indirizzo o a un telefono
signature-html-plain-text = Creata una versione in testo semplice, per le app di posta che mostrano solo testo

## Import

signature-import-title = Importa
signature-import-subtitle = Da Gmail, Thunderbird, Evolution e KMail
signature-import-looking = Ricerca delle firme…
signature-import-none = Nessuna firma trovata. Per un’altra app, copia l’HTML della sua firma e usa Incolla HTML.
signature-import-from = Da { $app }
signature-import-already = già in Katna
signature-import-gmail-sign-in = { $address }: accedi di nuovo in Impostazioni > Account perché Katna possa leggere le firme di Gmail.
signature-import-gmail-failed = { $address }: { $error }
signature-import-cancel = Annulla
signature-import-do = { $count ->
    [one] Importa { $count } firma
    [many] Importa { $count } di firme
   *[other] Importa { $count } firme
}
signature-import-name = { $name } ({ $app })
