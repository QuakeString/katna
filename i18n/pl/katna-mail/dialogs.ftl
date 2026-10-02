# Katna Mail, Polish (Polski).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = O Katna
about-tagline = Poczta i kalendarz dla pulpitu Linux
about-whats-new = Co nowego

## Updates, in a box under the version in About (only in packages that
## update themselves). $version is a version such as 0.0.0.r236.g1a2b3c4.

about-update-not-checked = Aktualizacje nie zostały jeszcze sprawdzone
about-update-checking = Sprawdzanie aktualizacji…
about-update-up-to-date = Katna Mail jest aktualna
about-update-check-failed = Nie udało się sprawdzić aktualizacji
about-update-available = Dostępna jest wersja { $version }
about-update-downloading = Pobieranie wersji { $version }… { $percent }%
about-update-download-failed = Pobieranie wersji { $version } nie zostało ukończone
about-update-ready = Wersja { $version } jest gotowa do instalacji
about-update-ready-detail = Katna Mail uruchomi się ponownie, aby zakończyć aktualizację.
about-update-confirm = Zainstalować wersję { $version }?
about-update-confirm-detail = Katna Mail zamknie się, zainstaluje aktualizację i otworzy się ponownie w tym samym miejscu co wcześniej. Komputer poprosi o hasło.
about-update-installing = Instalowanie wersji { $version }…
about-update-installing-detail = Wpisz hasło w otwartym oknie.
about-update-cancelled = Aktualizacja nie została zainstalowana, ponieważ nie podano hasła.
about-update-failed = Nie udało się zainstalować aktualizacji: { $error }
about-update-unsupported = Ta kopia Katna Mail jest aktualizowana przez menedżera pakietów.
about-update-restart-failed = Aktualizacja jest zainstalowana, ale Katna Mail nie mogła się ponownie otworzyć ({ $error }). Otwórz ją samodzielnie.
about-update-check = Sprawdź aktualizacje
about-update-download = Pobierz
about-update-retry = Spróbuj ponownie
about-update-button = Aktualizuj
about-update-restart = Zaktualizuj i uruchom ponownie
about-update-cancel = Nie teraz
about-changelog = Lista zmian
about-source = Kod źródłowy
about-coffee = Postaw mi kawę
about-coffee-coffee = Kawa?
about-coffee-tea = Herbata?
about-coffee-pizza = Pizza?
about-coffee-nothing = Nic? Zupełnie nic?
about-coffee-water = Przeżyję na wodzie!!
about-coffee-thanks = Dziękuję, że używasz Katna
about-coming-soon = Wkrótce
about-follow-me = Śledź mnie na
about-love-title = Stworzone z miłością do Rusta, KDE i Linuksa
about-love-text = Dzięki Rustowi pisanie szybkiej i bezpiecznej aplikacji pocztowej to przyjemność: Katna nie ma kodu unsafe. Pulpit Plasma od KDE i jego pakiet PIM zainspirowały Katna, a Linux i społeczność wolnego oprogramowania tworzą grunt, na którym stoi. Dziękuję – i dziękuję bibliotekom wymienionym poniżej.
about-kde-text = KDE tworzy pulpit, na którym Katna czuje się najlepiej, a robią to wolontariusze finansowani przez ludzi takich jak Ty. Jeśli lubisz Plasmę lub aplikacje KDE, rozważ wsparcie KDE darowizną.
about-donate-kde = Wesprzyj KDE
about-gpui-title = Zbudowane na GPUI z projektu Zed
about-gpui-text = Cały interfejs Katna Mail jest zbudowany na GPUI – szybkim, akcelerowanym przez GPU frameworku interfejsu, który Zed Industries stworzyło dla edytora Zed. Każdy piksel, animacja i okno, które widzisz, są przez niego rysowane. Dziękuję, zespole Zed, za rozwijanie go otwarcie. Apache-2.0.
about-gpui-github = GPUI na GitHubie
about-personal-title = Osobisty projekt
about-personal-text = Katna Mail nie próbuje być nowa ani rewolucyjna. To aplikacja pocztowa, jakiej chciał jej autor, a jej funkcje i wygląd są zapożyczone z Gmaila, Mailspringa i Thunderbirda. Powstała tylko dzięki temu, jak daleko zaszły LLM-y.
about-built-on = ZBUDOWANE NA WOLNYM OPROGRAMOWANIU
about-credit-pimalaya = IMAP, SMTP i logowanie (io-imap, io-smtp, io-sasl)
about-credit-imap-codec = Odczyt i zapis IMAP
about-credit-tantivy = Wyszukiwanie
about-credit-sqlite = Magazyn poczty
about-credit-rustls = Bezpieczne połączenia
about-credit-mail-parser = Odczyt poczty, od Stalwart Labs
about-credit-html5ever = Poczta HTML, z projektu Servo
about-credit-zbus = Komunikacja z pulpitem przez D-Bus i portale
about-credit-oo7 = Hasła w bazie kluczy pulpitu
about-credit-hayro = Wyświetlanie i drukowanie PDF-ów
about-credit-calamine = Podgląd arkuszy kalkulacyjnych
about-credit-resvg = Obrazy SVG
about-credit-jiff = Daty i strefy czasowe
about-credit-spellbook = Sprawdzanie pisowni, z edytora Helix
about-credit-smol = Robienie wielu rzeczy naraz
about-credit-color-schemes = Palety wbudowanych schematów kolorów
about-all-libraries = Wszystkie biblioteki używane przez Katna ({ $count })
about-library-authors = autorzy: { $authors }
about-license = Katna to wolne oprogramowanie na licencji GNU GPL w wersji 3 lub nowszej.
about-close = Zamknij

## What’s new (shown after an update)

whats-new-title = Co nowego w Katna Mail
whats-new-updated = Zaktualizowano do wersji { $version }
whats-new-version = Wersja { $version }
whats-new-more = { $count ->
    [one] I jeszcze jedna zmiana na pełnej liście zmian.
    [few] I jeszcze { $count } zmiany na pełnej liście zmian.
    [many] I jeszcze { $count } zmian na pełnej liście zmian.
   *[other] I jeszcze { $count } zmiany na pełnej liście zmian.
}
whats-new-changelog = Pełna lista zmian
whats-new-got-it = Rozumiem

## First run: welcome page

onboarding-welcome-title = Witaj w Katna Mail
onboarding-welcome-lead = Twoja poczta na Twoim własnym komputerze: szybka do przeszukania, dostępna offline i prywatna.
onboarding-fast-title = Szybko, nawet offline
onboarding-fast-text = Katna przechowuje tu kopię Twojej poczty, więc otwieranie i wyszukiwanie działa natychmiast, z połączeniem lub bez.
onboarding-providers-title = Działa z Twoją pocztą
onboarding-providers-text = Gmail, Outlook, Yahoo, iCloud i każde inne konto IMAP lub POP.
onboarding-private-title = Prywatnie
onboarding-private-text = Poczta trafia prosto od Twojego dostawcy na ten komputer. Żaden serwer Katna jej nie widzi.
onboarding-get-started = Zaczynajmy

## First run: adding an account

onboarding-service-checking = Sprawdzanie usługi Katna działającej w tle…
onboarding-service-running = Usługa Katna działa w tle.
onboarding-service-missing = Usługa Katna działająca w tle nie jest uruchomiona
onboarding-service-start = Pobiera i wysyła Twoją pocztę. Uruchom ją z terminala, a potem sprawdź ponownie:
onboarding-check-again = Sprawdź ponownie
onboarding-account-title = Dodaj konto pocztowe
onboarding-account-lead = Wpisz adres e-mail i hasło, a Katna znajdzie ustawienia serwera. Gmail, Yahoo i iCloud wymagają hasła do aplikacji, które utworzysz w ustawieniach bezpieczeństwa konta.
onboarding-add-account = Dodaj konto
onboarding-back = Wstecz

## First run: choosing the look

onboarding-look-title = Dopasuj do siebie
onboarding-look-lead = Wybierz, jak otwiera się poczta i jak wygląda Katna. Możesz to zmienić w każdej chwili w szybkich ustawieniach.
onboarding-reading-pane = Okienko odczytu
onboarding-pane-right = Na prawo od listy
onboarding-pane-none = Bez podziału
onboarding-theme = Motyw
onboarding-theme-system = Systemowy
onboarding-theme-light = Jasny
onboarding-theme-dark = Ciemny
onboarding-density = Gęstość
onboarding-density-default = Domyślna
onboarding-density-compact = Kompaktowa
onboarding-continue = Dalej

## First start: the Katna account page. A Katna account is an account on
## Katna's own server, not a mail account; see katna-account.ftl.

onboarding-katna-title = Więcej z kontem Katna
onboarding-katna-lead = Jest opcjonalne. Włącza funkcje online Katna, a założyć je możesz też później w Ustawienia > Subskrypcja.
onboarding-katna-receipts-title = Potwierdzenia odczytu
onboarding-katna-receipts-text = Zobacz, kiedy ludzie otwierają wysłaną przez Ciebie pocztę.
onboarding-katna-links-title = Śledzenie linków
onboarding-katna-links-text = Zobacz, które linki w Twojej poczcie są klikane.
onboarding-katna-activity-title = Aktywność
onboarding-katna-activity-text = Otwarcia i kliknięcia wszystkiego, co wysłałeś, w jednym miejscu.
onboarding-katna-translate-title = Automatyczne tłumaczenie
onboarding-katna-translate-text = Czytaj pocztę napisaną w innych językach w swoim własnym.
onboarding-katna-private = Ma własne hasło. Dane logowania do poczty nigdy nie opuszczają tego komputera.

## First run: done

onboarding-ready-title = Wszystko gotowe
onboarding-ready-lead = Katna pobiera Twoją pocztę. Wiadomości pojawiają się w miarę napływania, a nowa poczta dochodzi sama.
onboarding-ready-lead-address = Katna pobiera pocztę konta { $address }. Wiadomości pojawiają się w miarę napływania, a nowa poczta dochodzi sama.
onboarding-ready-tour = Obejrzeć minutowy przewodnik, żeby zobaczyć, gdzie co jest?
onboarding-skip = Na razie pomiń
onboarding-take-tour = Obejrzyj przewodnik

## Asking to send crash reports (on its own and on the first-run pages)

share-title = Pomóż ulepszyć Katna
share-lead = Gdy Katna ulegnie awarii, zapisuje raport na tym komputerze. Wysyłanie tych raportów pomaga naprawić to, co poszło nie tak. Możesz to zmienić w każdej chwili w Ustawienia > Opinie.
share-sent = Co jest wysyłane
share-sent-detail = Raport o awarii w takiej postaci, w jakiej możesz go zobaczyć w Ustawieniach: co uległo awarii i gdzie w Katna, wersja, Twój system Linux i pulpit oraz ostatnie wiersze dziennika Katna, które mogą zawierać nazwy folderów poczty.
share-never-sent = Co nigdy nie jest wysyłane
share-never-sent-detail = Twoje wiadomości, kontakty, hasła, adres IP, nazwa użytkownika ani nazwa komputera. Adresy e-mail są usuwane z raportu.
share-where = Dokąd trafia
share-where-detail = Do systemu śledzenia awarii Katna w Sentry, przechowywanego w UE. Żaden identyfikator nie wiąże raportów z Tobą.
share-dont-send = Nie wysyłaj
share-send = Wysyłaj raporty o awariach
share-sending = Raporty o awariach będą wysyłane. Dziękujemy.
share-local = Raporty o awariach zostają na tym komputerze.

## The tour (cards pointing at each part of the window)

tour-welcome-title = Witaj w Katna Mail
tour-welcome-text = Minutowy przewodnik pokaże, gdzie co jest.
tour-not-now = Nie teraz
tour-start = Obejrzyj przewodnik
tour-close = Zamknij
tour-skip = Pomiń przewodnik
tour-back = Wstecz
tour-done = Gotowe
tour-next = Dalej
tour-step = { $step } z { $total }
tour-compose-title = Napisz wiadomość
tour-compose-text = Utwórz otwiera nową wiadomość w prawym dolnym rogu, więc możesz dalej czytać, pisząc.
tour-search-title = Przeszukuj całą pocztę
tour-search-text = Wyszukiwanie działa też offline. Przycisk po prawej stronie dodaje filtry: nadawca, odbiorca, temat, daty i załączniki.
tour-menu-title = Pokaż lub ukryj foldery
tour-menu-text = Ten przycisk zwija listę folderów. Gdy jest ukryta, zatrzymaj wskaźnik na Poczcie po lewej, aby zobaczyć foldery.
tour-apps-title = Twoje aplikacje
tour-apps-text = Poczta jest tutaj, obok Kalendarza, Kontaktów, Zadań, Notatek i Plików.
tour-tabs-title = Karty skrzynki odbiorczej
tour-tabs-text = Nowa poczta jest sortowana do kart Główne, Oferty, Społeczności, Powiadomienia i Fora. Karty możesz wyłączyć w szybkich ustawieniach.
tour-list-title = Twoje wiadomości
tour-list-text = Kliknij wiadomość, aby ją przeczytać. Najedź na nią, aby zobaczyć szybkie akcje, kliknij prawym przyciskiem, aby zobaczyć więcej, lub zaznacz kilka, aby działać na nich razem.
tour-settings-title = Szybkie ustawienia
tour-settings-text = Tutaj zmienisz okienko odczytu, gęstość i motyw. Stąd możesz też ponownie uruchomić przewodnik.
tour-account-title = Twoje konto
tour-account-text = Zobacz, w którym koncie jesteś, i dodaj kolejne.

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] Usługa Katna działająca w tle nieoczekiwanie się zatrzymała.
    [one] Usługa Katna działająca w tle nieoczekiwanie się zatrzymała. Zapisano jeszcze jeden raport o awarii.
    [few] Usługa Katna działająca w tle nieoczekiwanie się zatrzymała. Zapisano jeszcze { $more } raporty o awariach.
    [many] Usługa Katna działająca w tle nieoczekiwanie się zatrzymała. Zapisano jeszcze { $more } raportów o awariach.
   *[other] Usługa Katna działająca w tle nieoczekiwanie się zatrzymała. Zapisano jeszcze { $more } raportu o awariach.
}
crash-mail = { $more ->
    [0] Katna Mail ostatnio nieoczekiwanie się zamknęła.
    [one] Katna Mail ostatnio nieoczekiwanie się zamknęła. Zapisano jeszcze jeden raport o awarii.
    [few] Katna Mail ostatnio nieoczekiwanie się zamknęła. Zapisano jeszcze { $more } raporty o awariach.
    [many] Katna Mail ostatnio nieoczekiwanie się zamknęła. Zapisano jeszcze { $more } raportów o awariach.
   *[other] Katna Mail ostatnio nieoczekiwanie się zamknęła. Zapisano jeszcze { $more } raportu o awariach.
}
crash-view = Wyświetl raport
crash-view-tooltip = Otwórz raport zapisany na tym komputerze
crash-copy = Kopiuj raport
crash-close = Zamknij

## Sign in again (a bar at the bottom when Google or Microsoft stopped
## letting an account in; $provider: Google or Microsoft)

sign-in-again-text = { $provider } prosi o ponowne zalogowanie się do { $address }.
sign-in-again-button = Zaloguj się
sign-in-again-tooltip = Otwórz stronę logowania { $provider } w przeglądarce
sign-in-again-waiting = Czekanie na przeglądarkę…
sign-in-again-close = Zamknij
google-api-off = { $api } jest wyłączone w projekcie Google Cloud Katna.
google-api-turn-on = Włącz
google-api-turn-on-tooltip = Otwórz Google Cloud, aby włączyć { $api }, a potem naciśnij Spróbuj ponownie
sign-in-again-done = Ponownie zalogowano do { $address }. Pobieranie poczty…

## Before deleting several conversations, or deleting for good

delete-ask-title = { $kind ->
    [conversation] { $count ->
        [one] Przenieść ten wątek do kosza?
        [few] Przenieść { $count } wątki do kosza?
        [many] Przenieść { $count } wątków do kosza?
       *[other] Przenieść { $count } wątku do kosza?
    }
   *[message] { $count ->
        [one] Przenieść tę wiadomość do kosza?
        [few] Przenieść { $count } wiadomości do kosza?
        [many] Przenieść { $count } wiadomości do kosza?
       *[other] Przenieść { $count } wiadomości do kosza?
    }
}
delete-ask-body = { $count ->
    [one] Możesz to zaraz potem cofnąć albo później przywrócić z kosza.
    [few] Możesz to zaraz potem cofnąć albo później przywrócić je z kosza.
    [many] Możesz to zaraz potem cofnąć albo później przywrócić je z kosza.
   *[other] Możesz to zaraz potem cofnąć albo później przywrócić je z kosza.
}
delete-ask-confirm = Przenieś do kosza
delete-forever-title = { $kind ->
    [conversation] { $count ->
        [one] Usunąć ten wątek trwale?
        [few] Usunąć { $count } wątki trwale?
        [many] Usunąć { $count } wątków trwale?
       *[other] Usunąć { $count } wątku trwale?
    }
   *[message] { $count ->
        [one] Usunąć tę wiadomość trwale?
        [few] Usunąć { $count } wiadomości trwale?
        [many] Usunąć { $count } wiadomości trwale?
       *[other] Usunąć { $count } wiadomości trwale?
    }
}
delete-forever-body = { $count ->
    [one] Usunięcie obejmuje też serwer. Tego nie da się cofnąć.
    [few] Usunięcie obejmuje też serwer. Tego nie da się cofnąć.
    [many] Usunięcie obejmuje też serwer. Tego nie da się cofnąć.
   *[other] Usunięcie obejmuje też serwer. Tego nie da się cofnąć.
}
delete-forever-confirm = Usuń trwale
delete-ask-dont-ask = Nie pytaj ponownie
delete-ask-cancel = Anuluj
