# Katna Mail, German (Deutsch): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Kontakte
contacts-frequent = Häufig kontaktiert
contacts-other = Weitere Kontakte
contacts-other-about = Personen, denen Sie aus Gmail geschrieben, aber nicht gespeichert haben
contacts-other-email = E-Mail senden
contacts-other-empty = Keine weiteren Kontakte. Personen, denen Sie aus Gmail schreiben, die Sie aber nicht speichern, erscheinen hier.
contacts-other-allow = Um weitere Kontakte zu sehen, melden Sie sich erneut bei Ihrem Gmail-Konto an und erlauben Sie Katna den Zugriff darauf.
contacts-labels = Labels
contacts-label-options = Label-Optionen
contacts-label-rename = Label umbenennen
contacts-label-email = Allen eine E-Mail senden
contacts-label-delete = Label löschen
contacts-label-new = Neues Label
contacts-label-name = Labelname
contacts-label-button = Label
contacts-label-menu = Label zuweisen:
contacts-label-added = Zu „{ $name }“ hinzugefügt
contacts-label-removed = Aus „{ $name }“ entfernt
contacts-label-renamed = Label umbenannt in „{ $name }“
contacts-label-deleted = Label „{ $name }“ gelöscht
contacts-label-no-email = Niemand mit diesem Label hat eine E-Mail-Adresse
contacts-manage = Korrigieren und verwalten
contacts-merge = Zusammenführen und korrigieren
contacts-merge-about = { $count ->
    [one] { $count } Vorschlag: Kontakte, die wie dieselbe Person aussehen
   *[other] { $count } Vorschläge: Kontakte, die wie dieselbe Person aussehen
}
contacts-merge-none = Keine Duplikate. Kontakte mit demselben Namen oder derselben Telefonnummer erscheinen hier.
contacts-merge-count = { $count ->
    [one] { $count } Kontakt
   *[other] { $count } Kontakte
}
contacts-merge-all = Alle zusammenführen
contacts-merge-button = Zusammenführen
contacts-merge-dismiss = Ablehnen
contacts-merged = { $count ->
    [1] Kontakte zusammengeführt
    [one] { $count } Zusammenführung durchgeführt
   *[other] { $count } Zusammenführungen durchgeführt
}
contacts-import = Importieren
contacts-export = Exportieren
contacts-import-file = Kontakte aus einer vCard- oder CSV-Datei importieren
contacts-imported = { $count ->
    [one] { $count } Kontakt in { $place } importiert
   *[other] { $count } Kontakte in { $place } importiert
}
contacts-imported-some = { $count ->
    [one] { $count } Kontakt in { $place } importiert; { $skipped } bereits gespeichert, ausgelassen
   *[other] { $count } Kontakte in { $place } importiert; { $skipped } bereits gespeichert, ausgelassen
}
contacts-import-none = Keine Kontakte in { $name } gefunden
contacts-import-all-saved = Alle Personen in { $name } sind bereits gespeichert
contacts-import-failed = { $name } konnte nicht gelesen werden: { $error }
contacts-exported = { $count ->
    [one] { $count } Kontakt nach { $path } exportiert
   *[other] { $count } Kontakte nach { $path } exportiert
}
contacts-export-none = Keine Kontakte zum Exportieren
contacts-export-failed = Kontakte konnten nicht exportiert werden: { $error }
contacts-print = Drucken
contacts-print-title = Kontakte
contacts-print-none = Keine Kontakte zum Drucken
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = Geburtstag: { $day }
contacts-print-nickname = Spitzname: { $name }
contacts-create = Kontakt erstellen

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
contacts-edit = Bearbeiten
contacts-delete = Löschen
contacts-qr = Als QR-Code teilen
contacts-qr-about = Scannen Sie den Code mit der Kamera eines Smartphones, um den Kontakt zu speichern.
contacts-qr-too-long = Dieser Kontakt hat zu viele Angaben für einen QR-Code.
contacts-qr-done = Fertig
contacts-deleted = { $name } gelöscht
contacts-added = { $name } zu Kontakten hinzugefügt
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

## Creating and changing a contact

contacts-edit-new-title = Kontakt erstellen
contacts-edit-title = Kontakt bearbeiten
contacts-edit-save = Speichern
contacts-edit-saving = Wird gespeichert …
contacts-edit-cancel = Abbrechen
contacts-saved = Kontakt gespeichert
contacts-edit-save-to = Speichern unter
contacts-edit-changes-go-to = Änderungen werden in { $place } gespeichert.
contacts-edit-given = Vorname
contacts-edit-family = Nachname
contacts-edit-company = Unternehmen
contacts-edit-job = Position
contacts-edit-email = E-Mail
contacts-edit-phone = Telefon
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = E-Mail-Adresse hinzufügen
contacts-edit-add-phone = Telefonnummer hinzufügen
contacts-edit-street = Straße
contacts-edit-city = Ort
contacts-edit-postcode = Postleitzahl
contacts-edit-country = Land
contacts-edit-birthday = Geburtstag (YYYY-MM-DD)
contacts-edit-empty = Geben Sie zuerst einen Namen, eine E-Mail-Adresse oder eine Telefonnummer ein.
