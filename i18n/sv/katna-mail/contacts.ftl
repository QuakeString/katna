# Katna Mail, Swedish (Svenska): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Kontakter
contacts-frequent = Ofta kontaktade
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
