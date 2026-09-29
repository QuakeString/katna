# Katna Mail, Polish (Polski): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Kontakty
contacts-frequent = Często używane
contacts-labels = Etykiety

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
