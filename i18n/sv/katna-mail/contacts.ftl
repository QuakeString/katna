# Katna Mail, Swedish (Svenska): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Kontakter
contacts-frequent = Ofta kontaktade
contacts-other = Övriga kontakter
contacts-other-about = Personer du har mejlat från Gmail men inte sparat
contacts-other-email = Skicka e-post
contacts-other-empty = Inga övriga kontakter. Personer du mejlar från Gmail men inte sparar visas här.
contacts-other-allow = Om du vill se övriga kontakter loggar du in på ditt Gmail-konto igen och tillåter Katna att se dem.
contacts-labels = Etiketter
contacts-label-options = Etikettalternativ
contacts-label-rename = Byt namn på etikett
contacts-label-email = Skicka e-post till alla
contacts-label-delete = Radera etikett
contacts-label-new = Ny etikett
contacts-label-name = Etikettnamn
contacts-label-button = Etikett
contacts-label-menu = Etikettera som:
contacts-label-added = Lades till i { $name }
contacts-label-removed = Togs bort från { $name }
contacts-label-renamed = Etiketten bytte namn till { $name }
contacts-label-deleted = Etiketten { $name } raderades
contacts-label-no-email = Ingen med den här etiketten har en e-postadress
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = Konton
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = Logga in igen för att visa kontakter
contacts-account-signed-in = Inloggad på { $address } igen. Hämtar dina kontakter…
contacts-account-sign-in-refused = { $provider } släppte inte in Katna. Försök igen och ge åtkomst till dina kontakter.
contacts-account-password = Servern godtog inte lösenordet. Yahoo, iCloud, Zoho och andra kräver ett applösenord.
contacts-account-change-password = Ändra lösenord
contacts-account-change-password-tooltip = Öppna Inställningar > Konton
contacts-account-failed = Det gick inte att läsa kontakterna.
# $reason is the server's own words, in English.
contacts-account-error = Det gick inte att läsa kontakterna: { $reason }
contacts-account-none = Ingen adressbok hittades
# $reason is what the server answered, in English: "CardDAV https://dav.example.in/: status 404".
contacts-account-none-why = Ingen adressbok hittades: { $reason }
# A Gmail or Outlook account added with a password: its contacts need the
# provider's sign-in.
contacts-account-use-sign-in = { $provider } visar kontakter bara för Katna när den är inloggad med { $provider }.
contacts-account-sign-in-with = Logga in med { $provider }
contacts-account-looking = Letar efter kontakter…
contacts-account-try-again = Försök igen
contacts-account-try-again-tooltip = Kontrollera det här kontots kontakter igen nu
contacts-account-fixing = Arbetar på det…
contacts-manage = Åtgärda och hantera
contacts-merge = Sammanfoga och åtgärda
contacts-merge-about = { $count ->
    [one] { $count } förslag: kontakter som verkar vara samma person
   *[other] { $count } förslag: kontakter som verkar vara samma person
}
contacts-merge-none = Inga dubbletter. Kontakter med samma namn eller telefonnummer visas här.
contacts-merge-count = { $count ->
    [one] { $count } kontakt
   *[other] { $count } kontakter
}
contacts-merge-all = Sammanfoga alla
contacts-merge-button = Sammanfoga
contacts-merge-dismiss = Avvisa
contacts-merged = { $count ->
    [1] Kontakter sammanfogade
    [one] { $count } sammanfogning klar
   *[other] { $count } sammanfogningar klara
}
contacts-import = Importera
contacts-export = Exportera
contacts-import-file = Importera kontakter från en vCard- eller CSV-fil
contacts-imported = { $count ->
    [one] { $count } kontakt importerades till { $place }
   *[other] { $count } kontakter importerades till { $place }
}
contacts-imported-some = { $count ->
    [one] { $count } kontakt importerades till { $place }; { $skipped } redan sparade, utelämnades
   *[other] { $count } kontakter importerades till { $place }; { $skipped } redan sparade, utelämnades
}
contacts-import-none = Inga kontakter hittades i { $name }
contacts-import-all-saved = Alla i { $name } är redan sparade
contacts-import-failed = Det gick inte att läsa { $name }: { $error }
contacts-exported = { $count ->
    [one] { $count } kontakt exporterades till { $path }
   *[other] { $count } kontakter exporterades till { $path }
}
contacts-export-none = Inga kontakter att exportera
contacts-export-failed = Det gick inte att exportera kontakterna: { $error }
contacts-print = Skriv ut
contacts-print-title = Kontakter
contacts-print-none = Inga kontakter att skriva ut
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = Födelsedag: { $day }
contacts-print-nickname = Smeknamn: { $name }
contacts-create = Skapa kontakt

## Search and the list

contacts-search = Sök bland kontakter
contacts-loading = Läser in kontakter …
contacts-empty = Inga sparade kontakter än. Kontakter som du sparar i Gmail, Outlook eller din e-posttjänst visas här.
contacts-empty-no-books = Kontakter från dina konton visas här när de har synkroniserats.
contacts-none-found = Inga kontakter matchar din sökning.
contacts-starred = { $count ->
    [one] Stjärnmärkt kontakt ({ $count })
   *[other] Stjärnmärkta kontakter ({ $count })
}
contacts-count = Kontakter ({ $count })
contacts-col-name = Namn
contacts-col-email = E-post
contacts-col-phone = Telefonnummer
contacts-col-job = Befattning och företag
contacts-col-labels = Etiketter

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Tillåt att Katna läser kontakterna för { $address }.
contacts-allow-many = { $more ->
    [one] Tillåt att Katna läser kontakterna för { $address } och { $more } konto till.
   *[other] Tillåt att Katna läser kontakterna för { $address } och { $more } konton till.
}
contacts-allow-button = Tillåt

## A contact's page

contacts-back = Tillbaka till kontakter
contacts-edit = Redigera
contacts-delete = Radera
contacts-qr = Dela som QR-kod
contacts-qr-about = Skanna med en telefons kamera för att spara kontakten.
contacts-qr-too-long = Kontakten har för många uppgifter för att rymmas i en QR-kod.
contacts-qr-done = Klar
contacts-deleted = { $name } raderades
contacts-added = { $name } lades till i kontakter
contacts-find-mail = E-post
contacts-details = Kontaktuppgifter
contacts-saved-in = Sparad i
contacts-notes = Anteckningar
contacts-birthday = Födelsedag
contacts-nickname = Smeknamn
contacts-this-computer = Den här datorn
contacts-kind-home = Hem
contacts-kind-work = Arbete
contacts-kind-mobile = Mobil
contacts-kind-other = Övrigt
contacts-source-google = Google Kontakter
contacts-source-microsoft = Outlook-kontakter
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = Skapa kontakt
contacts-edit-title = Redigera kontakt
contacts-edit-save = Spara
contacts-edit-saving = Sparar…
contacts-edit-cancel = Avbryt
contacts-saved = Kontakten sparades
contacts-edit-save-to = Spara i
contacts-edit-changes-go-to = Ändringar sparas i { $place }.
contacts-edit-given = Förnamn
contacts-edit-family = Efternamn
contacts-edit-company = Företag
contacts-edit-job = Befattning
contacts-edit-email = E-post
contacts-edit-phone = Telefon
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = Lägg till e-postadress
contacts-edit-add-phone = Lägg till telefonnummer
contacts-edit-street = Gatuadress
contacts-edit-city = Ort
contacts-edit-postcode = Postnummer
contacts-edit-country = Land
contacts-edit-birthday = Födelsedag (YYYY-MM-DD)
contacts-edit-empty = Lägg först till ett namn, en e-postadress eller ett telefonnummer.
