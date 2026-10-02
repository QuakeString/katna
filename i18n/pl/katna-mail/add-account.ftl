# Katna Mail, Polish (Polski).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Dodaj konto pocztowe
add-account-providers-intro = Wybierz dostawcę poczty. Resztę Katna znajdzie sama.
add-account-provider-other = Inna poczta
add-account-provider-other-detail = Dowolne konto IMAP lub POP3
add-account-provider-google-detail = Gmail i Google Workspace
add-account-provider-microsoft-detail = Outlook i Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = Zaloguj się do { $provider }
add-account-form-title-other = Twoje konto pocztowe
add-account-form-intro = Katna przechowuje Twoje hasło w systemowym pęku kluczy.
add-account-looking = Szukanie serwerów poczty dla { $address }…
add-account-address-intro = Wpisz adres e-mail. Katna sama znajdzie serwery.
add-account-servers-title = Ustawienia serwera
add-account-servers-intro = Gdzie Katna odbiera i wysyła pocztę dla { $address }.
add-account-signing-in = Logowanie…
add-account-browser-title = Kontynuuj w przeglądarce
add-account-browser-intro = Katna otworzyła stronę logowania { $provider } w przeglądarce. Zaloguj się tam i pozwól aplikacji Katna czytać i wysyłać Twoją pocztę, a potem wróć tutaj.
add-account-browser-hint = Nie otworzyła się żadna strona? Sprawdź okna przeglądarki albo wróć i spróbuj ponownie.
add-account-stage-browser = Czekanie na zalogowanie w przeglądarce…
add-account-stage-signing-in-at = Logowanie na { $server }…
add-account-help-app-password-link = Jak utworzyć hasło aplikacji
add-account-help-turn-on-imap = { $provider } wpuszcza aplikacje pocztowe dopiero po włączeniu dostępu IMAP i POP3 w ustawieniach poczty internetowej.
add-account-help-turn-on-imap-link = Jak to włączyć

## Add a mail account: fields

add-account-field-address = Adres e-mail
add-account-receive-with = Odbieraj pocztę przez
add-account-imap-about = IMAP przechowuje pocztę i foldery na serwerze, tak samo na każdym urządzeniu. Wybierz go, jeśli możesz.
add-account-pop3-about = POP3 pobiera pocztę na ten komputer. Poczta przeczytana lub przeniesiona tutaj pozostaje bez zmian na serwerze i na innych urządzeniach.
add-account-incoming = Poczta przychodząca ({ $protocol })
add-account-outgoing = Poczta wychodząca ({ $protocol })
add-account-field-server = Serwer
add-account-field-port = Port
add-account-security-none = Brak
add-account-security-none-warning = Brak szyfrowania: hasło i wiadomości mogą zostać odczytane w trakcie przesyłania.
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

## Add a mail account: buttons

add-account-sign-in-with = Zaloguj się przez { $provider }
add-account-sign-in-instead = Zamiast tego zaloguj się przez { $provider }

add-account-servers-button = Ustawienia serwera
add-account-back = Wstecz
add-account-add = Dodaj konto
add-account-done = Gotowe
add-account-another = Dodaj kolejne konto
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
add-account-app-password-refused = { $provider } odrzucił hasło. Wymagane jest hasło do aplikacji, a nie to, którego używasz w przeglądarce.
add-account-password-refused = Serwer odrzucił hasło. Sprawdź je i spróbuj ponownie.
add-account-sign-in-refused = { $provider } nie wpuścił aplikacji Katna. Spróbuj ponownie i zezwól na dostęp do poczty.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Ta kopia Katna nie potrafi jeszcze logować się do kont Microsoft.
    [Google] Ta kopia Katna nie potrafi jeszcze logować się do kont Google.
   *[other] Ten dostawca pozwala logować się tylko na własnej stronie, a Katna jeszcze tego dla niego nie potrafi.
}
add-account-smtp-not-found = Katna wie, skąd czytać pocztę, ale nie wie, przez co ją wysyłać. Wpisz serwer poczty wychodzącej.

## Add a mail account: the last step

add-account-done-title = Twoje konto jest gotowe
add-account-done-intro = Katna pobiera teraz Twoją pocztę. Nowa poczta pojawia się na bieżąco.
add-account-done-sign-in = Logowanie
add-account-done-signed-in-with = Przez { $provider }, w przeglądarce
add-account-done-receiving = Odbieranie poczty
add-account-done-sending = Wysyłanie poczty
add-account-done-on-server = Poczta na serwerze
add-account-done-kept = Zachowywana, dopóki nie usuniesz jej w Katna
add-account-done-pop3-hint = Co dzieje się z pocztą na serwerze, zmienisz w Ustawienia > Konta.
add-account-done-zoho-title = Zadania i kalendarze
add-account-done-zoho-about = Zoho trzyma je osobno od poczty. Zaloguj się raz przez Zoho, aby przenieść je do Katna.
add-account-done-linked = Zadania i kalendarze połączone

## The account menu (from the account button on the top bar)

add-account-menu-another = Dodaj kolejne konto
app-menu = Menu główne
app-menu-back = Wstecz
