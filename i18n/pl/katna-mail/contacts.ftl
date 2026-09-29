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
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = Konta
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = Zaloguj się ponownie, aby wyświetlić kontakty
contacts-account-signed-in = Ponownie zalogowano do { $address }. Pobieranie kontaktów…
contacts-account-sign-in-refused = { $provider } nie wpuścił aplikacji Katna. Spróbuj ponownie i zezwól na dostęp do kontaktów.
contacts-account-password = Serwer nie przyjął hasła. Yahoo, iCloud, Zoho i inne wymagają hasła do aplikacji.
contacts-account-change-password = Zmień hasło
contacts-account-change-password-tooltip = Otwórz Ustawienia > Konta
contacts-account-failed = Nie udało się odczytać kontaktów.
# $reason is the server's own words, in English.
contacts-account-error = Nie udało się odczytać kontaktów: { $reason }
contacts-account-none = Nie znaleziono książki adresowej
# $reason is what the server answered, in English: "CardDAV https://dav.example.in/: status 404".
contacts-account-none-why = Nie znaleziono książki adresowej: { $reason }
# A Gmail or Outlook account added with a password: its contacts need the
# provider's sign-in.
contacts-account-use-sign-in = { $provider } pokazuje kontakty tylko aplikacji Katna zalogowanej przez { $provider }.
contacts-account-sign-in-with = Zaloguj się przez { $provider }
contacts-account-looking = Szukanie kontaktów…
contacts-account-try-again = Spróbuj ponownie
contacts-account-try-again-tooltip = Sprawdź teraz ponownie kontakty tego konta
contacts-account-fixing = Trwa naprawianie…
contacts-manage = Napraw i zarządzaj
contacts-merge = Scal i napraw
contacts-merge-about = { $count ->
    [one] { $count } sugestia: kontakty, które wyglądają na tę samą osobę
    [few] { $count } sugestie: kontakty, które wyglądają na tę samą osobę
    [many] { $count } sugestii: kontakty, które wyglądają na tę samą osobę
   *[other] { $count } sugestii: kontakty, które wyglądają na tę samą osobę
}
contacts-merge-none = Brak duplikatów. Kontakty o tej samej nazwie lub tym samym numerze telefonu pojawią się tutaj.
contacts-merge-count = { $count ->
    [one] { $count } kontakt
    [few] { $count } kontakty
    [many] { $count } kontaktów
   *[other] { $count } kontaktu
}
contacts-merge-all = Scal wszystkie
contacts-merge-button = Scal
contacts-merge-dismiss = Odrzuć
contacts-merged = { $count ->
    [1] Kontakty scalone
    [one] Liczba scaleń: { $count }
    [few] Liczba scaleń: { $count }
    [many] Liczba scaleń: { $count }
   *[other] Liczba scaleń: { $count }
}
contacts-import = Importuj
contacts-export = Eksportuj
contacts-import-file = Importuj kontakty z pliku vCard lub CSV
contacts-imported = { $count ->
    [one] Zaimportowano { $count } kontakt do: { $place }
    [few] Zaimportowano { $count } kontakty do: { $place }
    [many] Zaimportowano { $count } kontaktów do: { $place }
   *[other] Zaimportowano { $count } kontaktu do: { $place }
}
contacts-imported-some = { $count ->
    [one] Zaimportowano { $count } kontakt do: { $place }; pominięto { $skipped } już zapisanych
    [few] Zaimportowano { $count } kontakty do: { $place }; pominięto { $skipped } już zapisanych
    [many] Zaimportowano { $count } kontaktów do: { $place }; pominięto { $skipped } już zapisanych
   *[other] Zaimportowano { $count } kontaktu do: { $place }; pominięto { $skipped } już zapisanych
}
contacts-import-none = Nie znaleziono kontaktów w pliku { $name }
contacts-import-all-saved = Wszystkie osoby z pliku { $name } są już zapisane
contacts-import-failed = Nie można odczytać pliku { $name }: { $error }
contacts-exported = { $count ->
    [one] Wyeksportowano { $count } kontakt do: { $path }
    [few] Wyeksportowano { $count } kontakty do: { $path }
    [many] Wyeksportowano { $count } kontaktów do: { $path }
   *[other] Wyeksportowano { $count } kontaktu do: { $path }
}
contacts-export-none = Brak kontaktów do wyeksportowania
contacts-export-failed = Nie można wyeksportować kontaktów: { $error }
contacts-print = Drukuj
contacts-print-title = Kontakty
contacts-print-none = Brak kontaktów do wydrukowania
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = Urodziny: { $day }
contacts-print-nickname = Pseudonim: { $name }
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
contacts-qr = Udostępnij jako kod QR
contacts-qr-about = Zeskanuj to aparatem telefonu, aby zapisać kontakt.
contacts-qr-too-long = Ten kontakt ma zbyt wiele danych, aby zmieścić się w kodzie QR.
contacts-qr-done = Gotowe
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
