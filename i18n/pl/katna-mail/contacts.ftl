# Katna Mail, Polish (Polski): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Kontakty
contacts-frequent = Często używane
contacts-other = Inne kontakty
contacts-other-about = Osoby, do których wysłano wiadomości z Gmaila, ale nie zapisano ich
contacts-other-email = Wyślij e-mail
contacts-other-empty = Brak innych kontaktów. Osoby, do których piszesz z Gmaila, ale których nie zapisujesz, pojawią się tutaj.
contacts-other-allow = Aby zobaczyć inne kontakty, zaloguj się ponownie na swoje konto Gmail i zezwól Katnie na ich wyświetlanie.
contacts-labels = Etykiety
contacts-label-options = Opcje etykiety
contacts-label-rename = Zmień nazwę etykiety
contacts-label-email = Napisz do wszystkich
contacts-label-delete = Usuń etykietę
contacts-label-new = Nowa etykieta
contacts-label-name = Nazwa etykiety
contacts-label-button = Etykieta
contacts-label-menu = Oznacz etykietą:
contacts-label-added = Dodano do etykiety { $name }
contacts-label-removed = Usunięto z etykiety { $name }
contacts-label-renamed = Zmieniono nazwę etykiety na { $name }
contacts-label-deleted = Usunięto etykietę { $name }
contacts-label-no-email = Nikt z tą etykietą nie ma adresu e-mail
contacts-create = Utwórz kontakt

## Search and the list

contacts-search = Szukaj kontaktów
contacts-loading = Wczytywanie kontaktów…
contacts-empty = Nie ma jeszcze zapisanych kontaktów. Kontakty zapisane w Gmailu, Outlooku lub Twojej usłudze poczty pojawią się tutaj.
contacts-empty-no-books = Kontakty z Twoich kont pojawią się tutaj po zsynchronizowaniu.
contacts-none-found = Żadne kontakty nie pasują do wyszukiwania.
contacts-starred = { $count ->
    [one] Kontakt oznaczony gwiazdką ({ $count })
    [few] Kontakty oznaczone gwiazdką ({ $count })
    [many] Kontaktów oznaczonych gwiazdką ({ $count })
   *[other] Kontaktu oznaczonego gwiazdką ({ $count })
}
contacts-count = Kontakty ({ $count })
contacts-col-name = Nazwa
contacts-col-email = E-mail
contacts-col-phone = Numer telefonu
contacts-col-job = Stanowisko i firma
contacts-col-labels = Etykiety

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Zezwól Katnie na odczyt kontaktów z konta { $address }.
contacts-allow-many = { $more ->
    [one] Zezwól Katnie na odczyt kontaktów z konta { $address } i { $more } kolejnego konta.
    [few] Zezwól Katnie na odczyt kontaktów z konta { $address } i { $more } kolejnych kont.
    [many] Zezwól Katnie na odczyt kontaktów z konta { $address } i { $more } kolejnych kont.
   *[other] Zezwól Katnie na odczyt kontaktów z konta { $address } i { $more } kolejnego konta.
}
contacts-allow-button = Zezwól

## A contact's page

contacts-back = Wróć do kontaktów
contacts-edit = Edytuj
contacts-delete = Usuń
contacts-deleted = Usunięto: { $name }
contacts-added = Dodano { $name } do kontaktów
contacts-find-mail = Poczta
contacts-details = Dane kontaktowe
contacts-saved-in = Zapisano w
contacts-notes = Notatki
contacts-birthday = Urodziny
contacts-nickname = Pseudonim
contacts-this-computer = Ten komputer
contacts-kind-home = Dom
contacts-kind-work = Praca
contacts-kind-mobile = Komórka
contacts-kind-other = Inne
contacts-source-google = Kontakty Google
contacts-source-microsoft = Kontakty Outlook
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = Utwórz kontakt
contacts-edit-title = Edytuj kontakt
contacts-edit-save = Zapisz
contacts-edit-saving = Zapisywanie…
contacts-edit-cancel = Anuluj
contacts-saved = Kontakt zapisany
contacts-edit-save-to = Zapisz w
contacts-edit-changes-go-to = Zmiany są zapisywane w: { $place }.
contacts-edit-given = Imię
contacts-edit-family = Nazwisko
contacts-edit-company = Firma
contacts-edit-job = Stanowisko
contacts-edit-email = E-mail
contacts-edit-phone = Telefon
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = Dodaj adres e-mail
contacts-edit-add-phone = Dodaj numer telefonu
contacts-edit-street = Adres
contacts-edit-city = Miasto
contacts-edit-postcode = Kod pocztowy
contacts-edit-country = Kraj
contacts-edit-birthday = Urodziny (YYYY-MM-DD)
contacts-edit-empty = Najpierw dodaj imię i nazwisko, adres e-mail lub numer telefonu.
