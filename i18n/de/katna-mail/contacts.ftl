# Katna Mail, German (Deutsch): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Kontakte
contacts-frequent = Häufig kontaktiert
contacts-labels = Labels

## Search and the list

contacts-search = Kontakte suchen
contacts-loading = Kontakte werden geladen …
contacts-empty = Noch keine gespeicherten Kontakte. Kontakte, die Sie in Gmail, Outlook oder Ihrem E-Mail-Dienst speichern, erscheinen hier.
contacts-empty-no-books = Kontakte aus Ihren Konten erscheinen hier, sobald sie synchronisiert sind.
contacts-none-found = Keine Kontakte entsprechen Ihrer Suche.
contacts-starred = { $count ->
    [one] Markierter Kontakt ({ $count })
   *[other] Markierte Kontakte ({ $count })
}
contacts-count = Kontakte ({ $count })
contacts-col-name = Name
contacts-col-email = E-Mail
contacts-col-phone = Telefonnummer
contacts-col-job = Position und Unternehmen
contacts-col-labels = Labels

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Katna erlauben, die Kontakte von { $address } zu lesen.
contacts-allow-many = { $more ->
    [one] Katna erlauben, die Kontakte von { $address } und { $more } weiterem Konto zu lesen.
   *[other] Katna erlauben, die Kontakte von { $address } und { $more } weiteren Konten zu lesen.
}
contacts-allow-button = Erlauben

## A contact's page

contacts-back = Zurück zu den Kontakten
contacts-find-mail = E-Mail
contacts-details = Kontaktdetails
contacts-saved-in = Gespeichert in
contacts-notes = Notizen
contacts-birthday = Geburtstag
contacts-nickname = Spitzname
contacts-this-computer = Dieser Computer
contacts-kind-home = Privat
contacts-kind-work = Geschäftlich
contacts-kind-mobile = Mobil
contacts-kind-other = Sonstiges
contacts-source-google = Google Kontakte
contacts-source-microsoft = Outlook-Kontakte
contacts-source-carddav = CardDAV
