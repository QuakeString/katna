# Katna Mail, Afrikaans (Afrikaans): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Kontakte
contacts-frequent = Gereeld
contacts-other = Ander kontakte
contacts-other-about = Mense aan wie jy vanaf Gmail e-pos gestuur het maar nie gestoor het nie
contacts-other-email = Stuur e-pos
contacts-other-empty = Geen ander kontakte nie. Mense aan wie jy vanaf Gmail e-pos stuur maar nie stoor nie, verskyn hier.
contacts-other-allow = Om ander kontakte te sien, meld weer by jou Gmail-rekening aan en laat Katna toe om hulle te sien.
contacts-labels = Etikette
contacts-label-options = Etiketopsies
contacts-label-rename = Hernoem etiket
contacts-label-email = E-pos almal
contacts-label-delete = Vee etiket uit
contacts-label-new = Nuwe etiket
contacts-label-name = Etiketnaam
contacts-label-button = Etiket
contacts-label-menu = Etiketteer as:
contacts-label-added = By { $name } gevoeg
contacts-label-removed = Van { $name } verwyder
contacts-label-renamed = Etiket hernoem na { $name }
contacts-label-deleted = Etiket { $name } is uitgevee
contacts-label-no-email = Niemand op hierdie etiket het ’n e-posadres nie
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = Rekeninge
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = Meld weer aan om kontakte te wys
contacts-account-signed-in = Weer aangemeld by { $address }. Haal tans jou kontakte…
contacts-account-sign-in-refused = { $provider } het Katna nie ingelaat nie. Probeer weer, en gee toegang tot jou kontakte.
contacts-account-password = Die bediener het die wagwoord nie aanvaar nie. Yahoo, iCloud, Zoho en ander het 'n programwagwoord nodig.
contacts-account-change-password = Verander wagwoord
contacts-account-change-password-tooltip = Tik die nuwe wagwoord; Katna gaan dit by die bediener na
contacts-account-failed = Die kontakte kon nie gelees word nie.
# $reason is the server's own words, in English.
contacts-account-error = Die kontakte kon nie gelees word nie: { $reason }
contacts-account-none = Geen adresboek gevind nie
# $reason is what the server answered, in English: "CardDAV https://dav.example.in/: status 404".
contacts-account-none-why = Geen adresboek gevind nie: { $reason }
# A Gmail or Outlook account added with a password: its contacts need the
# provider's sign-in.
contacts-account-use-sign-in = { $provider } wys kontakte net aan Katna as dit met { $provider } aangemeld is.
contacts-account-sign-in-with = Meld aan met { $provider }
contacts-account-looking = Soek tans kontakte…
contacts-account-try-again = Probeer weer
contacts-account-try-again-tooltip = Kyk nou weer na hierdie rekening se kontakte
contacts-account-fixing = Besig daarmee…
contacts-manage = Regstel en bestuur
contacts-merge = Voeg saam en regstel
contacts-merge-about = { $count ->
    [one] { $count } voorstel: kontakte wat lyk soos dieselfde persoon
   *[other] { $count } voorstelle: kontakte wat lyk soos dieselfde persoon
}
contacts-merge-none = Geen duplikate nie. Kontakte met dieselfde naam of foonnommer verskyn hier.
contacts-merge-count = { $count ->
    [one] { $count } kontak
   *[other] { $count } kontakte
}
contacts-merge-all = Voeg almal saam
contacts-merge-button = Voeg saam
contacts-merge-dismiss = Verwerp
contacts-merged = { $count ->
    [1] Kontakte saamgevoeg
    [one] { $count } samevoeging voltooi
   *[other] { $count } samevoegings voltooi
}
contacts-import = Voer in
contacts-export = Voer uit
contacts-import-file = Voer kontakte in vanaf ’n vCard- of CSV-lêer
contacts-imported = { $count ->
    [one] { $count } kontak in { $place } ingevoer
   *[other] { $count } kontakte in { $place } ingevoer
}
contacts-imported-some = { $count ->
    [one] { $count } kontak in { $place } ingevoer; { $skipped } reeds gestoor, weggelaat
   *[other] { $count } kontakte in { $place } ingevoer; { $skipped } reeds gestoor, weggelaat
}
contacts-import-none = Geen kontakte in { $name } gevind nie
contacts-import-all-saved = Almal in { $name } is reeds gestoor
contacts-import-failed = Kon nie { $name } lees nie: { $error }
contacts-exported = { $count ->
    [one] { $count } kontak na { $path } uitgevoer
   *[other] { $count } kontakte na { $path } uitgevoer
}
contacts-export-none = Geen kontakte om uit te voer nie
contacts-export-failed = Kon nie kontakte uitvoer nie: { $error }
contacts-print = Druk
contacts-print-title = Kontakte
contacts-print-none = Geen kontakte om te druk nie
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = Verjaardag: { $day }
contacts-print-nickname = Bynaam: { $name }
contacts-create = Nuwe kontak

## Search and the list

contacts-search = Soek kontakte
contacts-loading = Laai tans kontakte …
contacts-empty = Nog geen gestoorde kontakte nie. Kontakte wat jy in Gmail, Outlook of jou e-posdiens stoor, verskyn hier.
contacts-empty-no-books = Kontakte van jou rekeninge verskyn hier sodra dit gesinkroniseer is.
contacts-none-found = Geen kontakte pas by jou soektog nie.
contacts-starred = { $count ->
    [one] Gesterde kontak ({ $count })
   *[other] Gesterde kontakte ({ $count })
}
contacts-count = Kontakte ({ $count })
contacts-col-name = Naam
contacts-col-email = E-pos
contacts-col-phone = Foonnommer
contacts-col-job = Posbenaming en maatskappy
contacts-col-labels = Etikette

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Laat Katna toe om die kontakte van { $address } te lees.
contacts-allow-many = { $more ->
    [one] Laat Katna toe om die kontakte van { $address } en { $more } ander rekening te lees.
   *[other] Laat Katna toe om die kontakte van { $address } en { $more } ander rekeninge te lees.
}
contacts-allow-button = Laat toe

## A contact's page

contacts-back = Terug na kontakte
contacts-edit = Wysig
contacts-delete = Vee uit
contacts-qr = Deel as QR-kode
contacts-qr-about = Skandeer dit met ’n foon se kamera om die kontak te stoor.
contacts-qr-too-long = Hierdie kontak het te veel besonderhede om in ’n QR-kode te pas.
contacts-qr-done = Klaar
contacts-deleted = { $name } is uitgevee
contacts-added = { $name } by kontakte gevoeg
contacts-find-mail = E-pos
contacts-details = Kontakbesonderhede
contacts-saved-in = Gestoor in
contacts-notes = Notas
contacts-birthday = Verjaarsdag
contacts-nickname = Bynaam
contacts-this-computer = Hierdie rekenaar
contacts-kind-home = Tuis
contacts-kind-work = Werk
contacts-kind-mobile = Selfoon
contacts-kind-other = Ander
contacts-source-google = Google Kontakte
contacts-source-microsoft = Outlook-kontakte
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = Skep kontak
contacts-edit-title = Wysig kontak
contacts-edit-save = Stoor
contacts-edit-saving = Stoor tans…
contacts-edit-cancel = Kanselleer
contacts-saved = Kontak gestoor
contacts-edit-save-to = Stoor in
contacts-edit-changes-go-to = Veranderinge word in { $place } gestoor.
contacts-edit-given = Voornaam
contacts-edit-family = Van
contacts-edit-company = Maatskappy
contacts-edit-job = Posbenaming
contacts-edit-email = E-pos
contacts-edit-phone = Foon
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = Voeg e-pos by
contacts-edit-add-phone = Voeg foon by
contacts-edit-street = Straatadres
contacts-edit-city = Stad
contacts-edit-postcode = Poskode
contacts-edit-country = Land
contacts-edit-birthday = Verjaarsdag (YYYY-MM-DD)
contacts-edit-empty = Voeg eers 'n naam, e-pos of foonnommer by.
