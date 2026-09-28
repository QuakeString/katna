# Katna Mail, Polish (Polski).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Dodaj konto pocztowe
add-account-looking = Szukanie serwerów poczty dla { $address }…
add-account-address-intro = Wpisz adres e-mail. Katna sama znajdzie serwery.
add-account-servers-title = Ustawienia serwera
add-account-servers-intro = Gdzie Katna odbiera i wysyła pocztę dla { $address }.
add-account-password-title = Wpisz hasło
add-account-signing-in = Logowanie…
add-account-browser-title = Kontynuuj w przeglądarce
add-account-browser-intro = Katna otworzyła stronę logowania { $provider } w przeglądarce. Zaloguj się tam i pozwól aplikacji Katna czytać i wysyłać Twoją pocztę, a potem wróć tutaj.
add-account-browser-hint = Nie otworzyła się żadna strona? Sprawdź okna przeglądarki albo wróć i spróbuj ponownie.

## Add a mail account: fields

add-account-field-address = Adres e-mail
add-account-incoming = Poczta przychodząca ({ $protocol })
add-account-outgoing = Poczta wychodząca ({ $protocol })
add-account-field-server = Serwer
add-account-field-port = Port
add-account-security-none = Brak
add-account-field-username = Nazwa użytkownika
add-account-field-password = Hasło
add-account-show-password = Pokaż hasło
add-account-app-password-hint = { $provider } wymaga tu hasła do aplikacji, a nie tego, którego używasz w przeglądarce. Utwórz je w ustawieniach bezpieczeństwa konta { $provider }.
add-account-field-name = Twoje imię i nazwisko (opcjonalnie)
add-account-name-hint = Widoczne dla osób, do których piszesz.
add-account-servers-pair = { $imap } i { $smtp }
add-account-servers-found = { $source ->
    [built-in] Serwery: { $servers }, znalezione na liście dostawców Katna.
    [provider] Serwery: { $servers }, znalezione w ustawieniach Twojego dostawcy.
    [ispdb] Serwery: { $servers }, znalezione na liście dostawców Thunderbirda.
    [dns] Serwery: { $servers }, znalezione w rekordach DNS Twojej domeny.
   *[other] Serwery: { $servers }, odgadnięte; sprawdź je, jeśli logowanie się nie powiedzie.
}
add-account-servers-entered = Serwery: { $servers }, wpisane ręcznie.
add-account-or = lub
add-account-sign-in-with = Zaloguj się przez { $provider }
add-account-sign-in-instead = Zamiast tego zaloguj się przez { $provider }

## Add a mail account: buttons

add-account-servers-button = Ustawienia serwera
add-account-back = Wstecz
add-account-add = Dodaj konto
add-account-next = Dalej
add-account-cancel = Anuluj

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] Wpisz serwer poczty przychodzącej.
   *[outgoing] Wpisz serwer poczty wychodzącej.
}
add-account-server-space = { $kind ->
    [incoming] Nazwa serwera poczty przychodzącej zawiera spację.
   *[outgoing] Nazwa serwera poczty wychodzącej zawiera spację.
}
add-account-port-invalid = { $kind ->
    [incoming] Port poczty przychodzącej musi być liczbą od { $min } do { $max }.
   *[outgoing] Port poczty wychodzącej musi być liczbą od { $min } do { $max }.
}
add-account-address-empty = Wpisz adres e-mail.
add-account-address-invalid = Wpisz adres e-mail, na przykład { $example }.
add-account-not-found = Katna nie znalazła serwerów dla { $address }, więc wpisała typowe nazwy. Sprawdź je u swojego dostawcy.
add-account-password-empty = Wpisz hasło.
add-account-name-is-password = Nazwa jest taka sama jak hasło. Wpisz tam zamiast tego swoje imię i nazwisko, tak jak mają je widzieć inni.
add-account-added = Dodano { $address }. Pobieranie poczty…
add-account-app-password-refused = { $provider } odrzucił hasło. Wymagane jest hasło do aplikacji, a nie to, którego używasz w przeglądarce.
add-account-password-refused = Serwer odrzucił hasło. Sprawdź je i spróbuj ponownie.
add-account-sign-in-refused = { $provider } nie wpuścił aplikacji Katna. Spróbuj ponownie i zezwól na dostęp do poczty.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Ta kopia Katna nie potrafi jeszcze logować się do kont Microsoft.
    [Google] Ta kopia Katna nie potrafi jeszcze logować się do kont Google.
   *[other] Ten dostawca pozwala logować się tylko na własnej stronie, a Katna jeszcze tego dla niego nie potrafi.
}
add-account-signed-in = Zalogowano przez { $provider }. Pobieranie poczty…

## The account menu (from the account button on the top bar)

add-account-menu-another = Dodaj kolejne konto
add-account-menu-manage = Zarządzaj kontami
app-menu = Menu główne
