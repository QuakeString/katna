# Katna Mail, Polish (Polski).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = Serwer poczty

problems-signed-out = { $provider } wylogował Katna z konta { $address }. Poczta przestała się synchronizować.
problems-password-refused = { $provider } odrzucił hasło do konta { $address }. Mogło zostać zmienione.
problems-no-answer = { $provider } nie odpowiada dla konta { $address }. Katna próbuje dalej.
problems-offline = Jesteś offline. Twoja poczta nadal tu jest, a wysyłane wiadomości poczekają, aż wrócisz online.
problems-accounts-need-you = { $count ->
    [one] 1 konto wymaga Twojej uwagi
    [few] { $count } konta wymagają Twojej uwagi
    [many] { $count } kont wymaga Twojej uwagi
   *[other] { $count } konta wymaga Twojej uwagi
}
problems-show = Pokaż
problems-later = Później
problems-new-password = Nowe hasło
problems-try-again = Spróbuj ponownie

## The New password card

problems-password-title = Nowe hasło
problems-password-detail = { $provider } odrzucił zapisane hasło do konta { $address }. Wpisz nowe; Katna sprawdzi je przed zapisaniem.
problems-password-placeholder = Hasło
problems-password-show = Pokaż hasło
problems-password-hide = Ukryj hasło
problems-password-cancel = Anuluj
problems-password-save = Zapisz
problems-password-checking = Sprawdzanie…
problems-password-refused-again = { $provider } odrzucił także to hasło. Sprawdź je i spróbuj ponownie.
problems-password-saved = Zapisano hasło do konta { $address }. Pobieranie poczty…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = Serwer poczty konta { $address } nie przyjął przeniesienia { $count ->
    [one] wiadomości, więc wróciła na swoje miejsce.
    [few] { $count } wiadomości, więc wróciły na swoje miejsce.
    [many] { $count } wiadomości, więc wróciły na swoje miejsce.
   *[other] { $count } wiadomości, więc wróciły na swoje miejsce.
}
problems-refused-flags = Serwer poczty konta { $address } nie przyjął oznaczenia { $count ->
    [one] wiadomości (przeczytana, gwiazdka…), więc wróciła do poprzedniego stanu.
    [few] { $count } wiadomości (przeczytane, gwiazdka…), więc wróciły do poprzedniego stanu.
    [many] { $count } wiadomości (przeczytane, gwiazdka…), więc wróciły do poprzedniego stanu.
   *[other] { $count } wiadomości (przeczytane, gwiazdka…), więc wróciły do poprzedniego stanu.
}
problems-refused-label = Serwer poczty konta { $address } nie przyjął zmiany etykiet { $count ->
    [one] wiadomości, więc wróciła do poprzedniego stanu.
    [few] { $count } wiadomości, więc wróciły do poprzedniego stanu.
    [many] { $count } wiadomości, więc wróciły do poprzedniego stanu.
   *[other] { $count } wiadomości, więc wróciły do poprzedniego stanu.
}
problems-refused-delete = Serwer poczty konta { $address } nie przyjął usunięcia { $count ->
    [one] wiadomości, więc wróciła.
    [few] { $count } wiadomości, więc wróciły.
    [many] { $count } wiadomości, więc wróciły.
   *[other] { $count } wiadomości, więc wróciły.
}
problems-refused-other = Serwer poczty konta { $address } nie przyjął { $count ->
    [one] zmiany, więc Katna przywróciła poprzedni stan.
    [few] { $count } zmian, więc Katna przywróciła poprzedni stan.
    [many] { $count } zmian, więc Katna przywróciła poprzedni stan.
   *[other] { $count } zmiany, więc Katna przywróciła poprzedni stan.
}
problems-details = Szczegóły

## Katna's background service (katna-daemon) isn't running

service-starting = Uruchamianie usługi Katna działającej w tle…
service-failed = Usługa Katna działająca w tle nie chce się uruchomić, więc poczta się nie synchronizuje.
service-start-again = Uruchom ponownie
service-started-again = Usługa Katna działająca w tle zatrzymała się i została uruchomiona ponownie.
service-details-title = Dlaczego usługa się nie uruchamia
service-details-body = Skopiuj to i wyślij razem ze zgłoszeniem. Nie zawiera poczty ani haseł.
service-details-copy = Kopiuj
service-details-close = Zamknij
