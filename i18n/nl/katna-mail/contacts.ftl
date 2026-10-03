# Katna Mail, Dutch (Nederlands): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Contacten
contacts-frequent = Vaak gebruikt
contacts-other = Overige contacten
contacts-other-about = Mensen naar wie je vanuit Gmail hebt gemaild maar die je niet hebt opgeslagen
contacts-other-email = E-mail sturen
contacts-other-empty = Geen overige contacten. Mensen naar wie je vanuit Gmail mailt maar die je niet opslaat, verschijnen hier.
contacts-other-allow = Log opnieuw in op je Gmail-account en geef Katna toestemming om overige contacten te zien.
contacts-labels = Labels
contacts-label-options = Labelopties
contacts-label-rename = Label hernoemen
contacts-label-email = Iedereen mailen
contacts-label-delete = Label verwijderen
contacts-label-new = Nieuw label
contacts-label-name = Labelnaam
contacts-label-button = Label
contacts-label-menu = Labelen als:
contacts-label-added = Toegevoegd aan { $name }
contacts-label-removed = Verwijderd uit { $name }
contacts-label-renamed = Label hernoemd naar { $name }
contacts-label-deleted = Label { $name } verwijderd
contacts-label-no-email = Niemand met dit label heeft een e-mailadres
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = Accounts
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = Meld je opnieuw aan om contacten te tonen
contacts-account-signed-in = Opnieuw aangemeld bij { $address }. Je contacten worden opgehaald…
contacts-account-sign-in-refused = { $provider } heeft Katna niet binnengelaten. Probeer het opnieuw en geef toegang tot je contacten.
contacts-account-password = De server heeft het wachtwoord niet geaccepteerd. Yahoo, iCloud, Zoho en andere hebben een app-wachtwoord nodig.
contacts-account-change-password = Wachtwoord wijzigen
contacts-account-change-password-tooltip = Instellingen > Accounts openen
contacts-account-failed = De contacten konden niet worden gelezen.
# $reason is the server's own words, in English.
contacts-account-error = De contacten konden niet worden gelezen: { $reason }
contacts-account-none = Geen adresboek gevonden
# $reason is what the server answered, in English: "CardDAV https://dav.example.in/: status 404".
contacts-account-none-why = Geen adresboek gevonden: { $reason }
# A Gmail or Outlook account added with a password: its contacts need the
# provider's sign-in.
contacts-account-use-sign-in = { $provider } toont contacten alleen aan Katna als die is aangemeld met { $provider }.
contacts-account-sign-in-with = Aanmelden met { $provider }
contacts-account-looking = Contacten zoeken…
contacts-account-try-again = Opnieuw proberen
contacts-account-try-again-tooltip = De contacten van dit account nu opnieuw controleren
contacts-account-fixing = Bezig…
contacts-manage = Herstellen en beheren
contacts-merge = Samenvoegen en herstellen
contacts-merge-about = { $count ->
    [one] { $count } suggestie: contacten die op dezelfde persoon lijken
   *[other] { $count } suggesties: contacten die op dezelfde persoon lijken
}
contacts-merge-none = Geen dubbele contacten. Contacten met dezelfde naam of hetzelfde telefoonnummer verschijnen hier.
contacts-merge-count = { $count ->
    [one] { $count } contact
   *[other] { $count } contacten
}
contacts-merge-all = Alles samenvoegen
contacts-merge-button = Samenvoegen
contacts-merge-dismiss = Negeren
contacts-merged = { $count ->
    [1] Contacten samengevoegd
    [one] { $count } samenvoeging uitgevoerd
   *[other] { $count } samenvoegingen uitgevoerd
}
contacts-import = Importeren
contacts-export = Exporteren
contacts-import-file = Contacten importeren uit een vCard- of CSV-bestand
contacts-imported = { $count ->
    [one] { $count } contact geïmporteerd naar { $place }
   *[other] { $count } contacten geïmporteerd naar { $place }
}
contacts-imported-some = { $count ->
    [one] { $count } contact geïmporteerd naar { $place }; { $skipped } al opgeslagen, overgeslagen
   *[other] { $count } contacten geïmporteerd naar { $place }; { $skipped } al opgeslagen, overgeslagen
}
contacts-import-none = Geen contacten gevonden in { $name }
contacts-import-all-saved = Iedereen in { $name } is al opgeslagen
contacts-import-failed = Kan { $name } niet lezen: { $error }
contacts-exported = { $count ->
    [one] { $count } contact geëxporteerd naar { $path }
   *[other] { $count } contacten geëxporteerd naar { $path }
}
contacts-export-none = Geen contacten om te exporteren
contacts-export-failed = Kan contacten niet exporteren: { $error }
contacts-print = Afdrukken
contacts-print-title = Contacten
contacts-print-none = Geen contacten om af te drukken
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = Verjaardag: { $day }
contacts-print-nickname = Bijnaam: { $name }
contacts-create = Nieuw contact

## Search and the list

contacts-search = Contacten zoeken
contacts-loading = Contacten laden…
contacts-empty = Nog geen opgeslagen contacten. Contacten die je opslaat in Gmail, Outlook of je e-mailservice verschijnen hier.
contacts-empty-no-books = Contacten van je accounts verschijnen hier zodra ze zijn gesynchroniseerd.
contacts-none-found = Geen contacten gevonden voor je zoekopdracht.
contacts-starred = { $count ->
    [one] Contact met ster ({ $count })
   *[other] Contacten met ster ({ $count })
}
contacts-count = Contacten ({ $count })
contacts-col-name = Naam
contacts-col-email = E-mail
contacts-col-phone = Telefoonnummer
contacts-col-job = Functie en bedrijf
contacts-col-labels = Labels

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Katna toestaan de contacten van { $address } te lezen.
contacts-allow-many = { $more ->
    [one] Katna toestaan de contacten van { $address } en { $more } ander account te lezen.
   *[other] Katna toestaan de contacten van { $address } en { $more } andere accounts te lezen.
}
contacts-allow-button = Toestaan

## A contact's page

contacts-back = Terug naar contacten
contacts-edit = Bewerken
contacts-delete = Verwijderen
contacts-qr = Delen als QR-code
contacts-qr-about = Scan dit met de camera van een telefoon om het contact op te slaan.
contacts-qr-too-long = Dit contact heeft te veel gegevens voor een QR-code.
contacts-qr-done = Klaar
contacts-deleted = { $name } verwijderd
contacts-added = { $name } toegevoegd aan contacten
contacts-find-mail = E-mail
contacts-details = Contactgegevens
contacts-saved-in = Opgeslagen in
contacts-notes = Notities
contacts-birthday = Verjaardag
contacts-nickname = Bijnaam
contacts-this-computer = Deze computer
contacts-kind-home = Thuis
contacts-kind-work = Werk
contacts-kind-mobile = Mobiel
contacts-kind-other = Overig
contacts-source-google = Google Contacten
contacts-source-microsoft = Outlook-contacten
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = Contact maken
contacts-edit-title = Contact bewerken
contacts-edit-save = Opslaan
contacts-edit-saving = Opslaan…
contacts-edit-cancel = Annuleren
contacts-saved = Contact opgeslagen
contacts-edit-save-to = Opslaan in
contacts-edit-changes-go-to = Wijzigingen worden opgeslagen in { $place }.
contacts-edit-given = Voornaam
contacts-edit-family = Achternaam
contacts-edit-company = Bedrijf
contacts-edit-job = Functie
contacts-edit-email = E-mail
contacts-edit-phone = Telefoon
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = E-mailadres toevoegen
contacts-edit-add-phone = Telefoonnummer toevoegen
contacts-edit-street = Straatadres
contacts-edit-city = Plaats
contacts-edit-postcode = Postcode
contacts-edit-country = Land
contacts-edit-birthday = Verjaardag (YYYY-MM-DD)
contacts-edit-empty = Voeg eerst een naam, e-mailadres of telefoonnummer toe.
