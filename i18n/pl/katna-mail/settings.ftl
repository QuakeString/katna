# Katna Mail, Polish (Polski).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings page: its tabs

settings-tab-general = Ogólne
settings-tab-inbox = Odebrane
settings-tab-accounts = Konta
settings-tab-subscriptions = Subskrypcje
settings-tab-appearance = Wygląd
settings-tab-shortcuts = Skróty
settings-tab-default-apps = Domyślne aplikacje
settings-tab-folders-rules = Foldery i reguły
settings-tab-compose = Tworzenie
settings-tab-mcp-server = Serwer MCP
settings-tab-feedback = Opinie
settings-tab-experimental = Eksperymentalne

## Settings page: tabs still to come

settings-tab-subscriptions-coming = Zobacz biuletyny i listy mailingowe, które otrzymujesz, i wypisz się jednym kliknięciem.
settings-tab-folders-rules-coming = Twórz, zmieniaj nazwy, przenoś i ukrywaj foldery i etykiety oraz wybieraj, które mają się synchronizować. Reguły same sortują, etykietują, przekazują dalej lub usuwają nową pocztę według nadawcy, tematu lub słów.
settings-tab-mcp-server-coming = Pozwól asystentom AI na tym komputerze przeszukiwać i czytać Twoją pocztę oraz tworzyć wersje robocze – za Twoją zgodą.

## Settings > General

settings-general-conversations = Widok wątków
settings-general-conversations-group = Grupuj odpowiedzi na ten sam e-mail
settings-general-conversations-group-detail = Jeden wiersz na wątek na liście
settings-general-reading = Czytanie
settings-general-newest-first = Najnowsza wiadomość na początku
settings-general-newest-first-detail = Wątek zaczyna się od ostatniej odpowiedzi
settings-general-full-headers = Pokazuj pełne nagłówki
settings-general-full-headers-detail = Od, do, DW, data i temat rozwinięte w każdej wiadomości
settings-general-full-names = Pełne nazwy odbiorców
settings-general-full-names-detail = „do mnie, Ada Lovelace” zamiast „do mnie, Ada”
settings-general-mark-read = Oznaczanie jako przeczytane
settings-general-mark-read-now = Od razu po otwarciu
settings-general-mark-read-1s = Po 1 sekundzie od otwarcia
settings-general-mark-read-3s = Po 3 sekundach od otwarcia
settings-general-mark-read-never = Tylko gdy sam oznaczę jako przeczytane
settings-general-reply-button = Przycisk odpowiedzi
settings-general-reply-all = Odpowiadaj wszystkim
settings-general-reply-all-detail = Przycisk odpowiedzi obok każdej wiadomości odpowiada wszystkim, a nie tylko nadawcy
settings-general-remote-images = Obrazy z internetu
settings-general-remote-images-detail = Wczytanie obrazów z wiadomości informuje nadawcę, że ją otworzyłeś, kiedy i mniej więcej gdzie. Gdy ta opcja jest wyłączona, każda wiadomość najpierw pyta, a obrazy od danego nadawcy zawsze możesz pokazać.
settings-general-remote-images-always = Zawsze pokazuj obrazy
settings-general-remote-images-always-detail = W każdej wiadomości, nie tylko od zaufanych nadawców
settings-general-sending = Wysyłanie
settings-general-sending-detail = Jak długo wysłana wiadomość czeka, aby można było cofnąć wysłanie.
settings-general-offline = Poczta offline
settings-general-offline-detail = Najnowsza poczta jest pobierana w całości, aby można ją było czytać bez połączenia. Starsza poczta jest pobierana po otwarciu.
settings-general-offline-days = { $count ->
    [one] { $count } dzień
    [few] { $count } dni
    [many] { $count } dni
   *[other] { $count } dnia
}
settings-general-offline-years = { $count ->
    [one] { $count } rok
    [few] { $count } lata
    [many] { $count } lat
   *[other] { $count } roku
}
settings-general-offline-all = Cała poczta
settings-general-offline-note = Po wybraniu mniejszej liczby dni już pobrana poczta zostaje. Na serwerze nic się nie zmienia.
settings-general-notifications = Powiadomienia
settings-general-notifications-detail = O nowej poczcie w Odebranych, nawet gdy Katna Mail jest zamknięta.
settings-general-new-mail = Powiadamiaj o nowej poczcie
settings-general-new-mail-detail = Z przyciskami Odpowiedz wszystkim, Oznacz jako przeczytane i Archiwizuj
settings-general-new-mail-sound = Odtwarzaj dźwięk
settings-general-new-mail-sound-detail = Dźwięk nowej poczty ustawiony na pulpicie
settings-general-desktop = Pulpit
settings-general-open-at-login = Otwieraj Katna Mail po zalogowaniu
settings-general-open-at-login-detail = Poczta i tak synchronizuje się po zalogowaniu, dopóki działa usługa
settings-general-tray = Pokazuj Katna w zasobniku systemowym
settings-general-tray-detail = Z liczbą nieprzeczytanych i menu
settings-general-unread-badge = Liczba nieprzeczytanych na ikonie w pasku zadań
settings-general-unread-badge-detail = Ile wiadomości w Odebranych jest nieprzeczytanych

## Settings > Inbox

settings-inbox-tabs = Karty skrzynki odbiorczej
settings-inbox-tabs-detail = Sortuj skrzynkę odbiorczą na karty, tak jak robi to witryna Twojego dostawcy poczty.
settings-inbox-tabs-show = Pokazuj karty skrzynki odbiorczej
settings-inbox-tabs-show-detail = Po wyłączeniu każde konto ma jedną listę
settings-inbox-no-accounts = Dodaj konto, aby wybrać jego karty.
settings-inbox-tabs-automatic = Automatycznie: { $tabs } ({ $provider })
settings-inbox-tabs-off = Bez kart
settings-inbox-tabs-gmail = Główne, Oferty, Społeczności, Powiadomienia, Fora
settings-inbox-tabs-focused = Priorytetowe i Inne
settings-inbox-tabs-zoho = Odebrane, Biuletyny i Powiadomienia
settings-inbox-tabs-shown = Widoczne karty. Poczta z wyłączonej karty zostaje w karcie { $tab }.

## Settings > Appearance

settings-appearance-reading-pane = Okienko odczytu
settings-appearance-reading-pane-detail = Gdzie wyświetla się otwarty wątek.
settings-appearance-pane-right = Na prawo od listy
settings-appearance-pane-none = Bez podziału
settings-appearance-density = Gęstość
settings-appearance-density-default = Domyślna
settings-appearance-density-compact = Kompaktowa
settings-appearance-scaling = Skalowanie
settings-appearance-scaling-detail = Powiększa lub zmniejsza wszystko w Katna Mail, niezależnie od skalowania samego pulpitu: tekst, ikony, odstępy i linie podziału. Wysyłana poczta zachowuje własny rozmiar czcionki. Przy bardzo małych rozmiarach ikony mogą być trudne do kliknięcia.
settings-appearance-theme = Motyw
settings-appearance-theme-system = Taki jak pulpit
settings-appearance-theme-light = Jasny
settings-appearance-theme-dark = Ciemny
settings-appearance-desktop-colors = Kolory pulpitu
settings-appearance-desktop-colors-use = Używaj kolorów pulpitu
settings-appearance-desktop-colors-use-detail = Schemat kolorów i kolor akcentu pulpitu
settings-appearance-app-names = Nazwy aplikacji
settings-appearance-app-names-show = Pokazuj nazwy aplikacji
settings-appearance-app-names-show-detail = Nazwy pod ikonami aplikacji po lewej stronie
settings-appearance-sender-pictures = Zdjęcia nadawców
settings-appearance-sender-pictures-show = Pokazuj logo firm
settings-appearance-sender-pictures-show-detail = Wyszukiwane według domeny nadawcy, nigdy według wiadomości, i przechowywane przez tydzień
settings-appearance-important = Znaczniki ważności
settings-appearance-important-show = Pokazuj znaczniki ważności
settings-appearance-important-show-detail = Obok każdej wiadomości na liście
settings-appearance-message-width = Szerokość wiadomości
settings-appearance-message-width-limit = Ogranicz szerokość wiadomości
settings-appearance-message-width-limit-detail = W szerokim oknie długie wiersze czyta się łatwiej
settings-appearance-mail-colors = Kolory poczty
settings-appearance-mail-colors-detail = Większość poczty jest projektowana na białą stronę. Przy ciemnym motywie jej kolory są zamieniane na ciemne, dobrze czytelne; po wyłączeniu poczta zachowuje kolory nadawcy na jasnej stronie.
settings-appearance-dark-mail = Ciemne kolory także dla poczty
settings-appearance-dark-mail-detail = Tylko gdy motyw jest ciemny
settings-appearance-attachment-previews = Podgląd załączników
settings-appearance-attachment-previews-show = Pokazuj podgląd załączników
settings-appearance-attachment-previews-show-detail = Mały obraz zawartości każdego pliku na jego karcie

## Settings > Default apps

settings-default-apps-intro = Gdzie otwierają się załączniki po kliknięciu. Przeglądarka zawsze może też otworzyć plik w innej aplikacji. Domyślne aplikacje pulpitu ustawia się w jego własnych ustawieniach.
settings-default-apps-pdf = Pliki PDF
settings-default-apps-pdf-detail = Strony, z powiększaniem.
settings-default-apps-pictures = Obrazy
settings-default-apps-pictures-detail = Zdjęcia (obrócone prawidłowo), PNG, GIF, WebP, BMP, TIFF i SVG.
settings-default-apps-text = Pliki tekstowe
settings-default-apps-text-detail = Zwykły tekst, logi, kod i inny tekst.
settings-default-apps-sheets = Arkusze kalkulacyjne
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) i CSV.
settings-default-apps-documents = Dokumenty
settings-default-apps-documents-detail = Word (docx) i tekst OpenDocument (odt).
settings-default-apps-katna = Przeglądarka Katna Mail
settings-default-apps-system = Domyślna aplikacja pulpitu
settings-default-apps-ask = Pytaj za każdym razem
settings-default-apps-after-saving = Po zapisaniu
settings-default-apps-show-folder = Pokazuj zapisane pliki w ich folderze
settings-default-apps-show-folder-detail = Otwiera menedżer plików z zaznaczonymi zapisanymi załącznikami

## Settings > Compose

settings-compose-send-from = Wysyłaj nowe wiadomości z
settings-compose-send-from-detail = Odpowiedzi i przekazane wiadomości zawsze wychodzą z konta, w którym jesteś.
settings-compose-send-from-current = Konto, w którym jesteś
settings-compose-send-on-replies = Wysyłanie odpowiedzi
settings-compose-send-on-replies-detail = Co robi przycisk Wyślij przy odpowiedzi lub przekazaniu. Menu obok przycisku Wyślij oferuje drugą opcję.
settings-compose-send-plain = Wyślij
settings-compose-send-archive = Wyślij i zarchiwizuj
settings-compose-signatures = Podpisy
settings-compose-signatures-detail = Dodawany pod wiadomością, po wierszu „--”. Inny możesz wybrać w oknie tworzenia wiadomości.
settings-compose-untitled = Bez nazwy
settings-compose-signature-name = Nazwa, np. Praca
settings-compose-signature-first = Mój podpis
settings-compose-signature-numbered = Podpis { $number }
settings-compose-signature-delete = Usuń
settings-compose-signature-deleted = Podpis usunięty
settings-compose-signature-new = Utwórz nowy
settings-compose-no-signatures = Nie ma jeszcze podpisów.
settings-compose-no-signature = Bez podpisu
settings-compose-for-new-mail = Dla nowych wiadomości
settings-compose-for-replies = Dla odpowiedzi i przekazywanych
settings-compose-for-replies-detail = W wątku, w którym podpisałeś wiadomość, odpowiedź zaczyna się zamiast tego od tego podpisu.
settings-compose-format = Format
settings-compose-plain-text = Pisz zwykłym tekstem
settings-compose-plain-text-detail = Nowa poczta zaczyna się bez formatowania; w oknie tworzenia można to zmienić
settings-compose-spelling = Pisownia
settings-compose-spell-check = Sprawdzaj pisownię podczas pisania
settings-compose-spell-check-detail = Błędnie napisane słowa są podkreślane, a sugestie są dostępne po kliknięciu prawym przyciskiem
settings-compose-spell-desktop = Język pulpitu ({ $language })
settings-compose-templates = Szablony
settings-compose-templates-detail = Zapisuj często pisane wiadomości i zaczynaj od nich nową wiadomość lub odpowiedź.

## Settings > Shortcuts

settings-shortcuts-set = Zestaw skrótów
settings-shortcuts-set-detail = Zacznij od klawiszy znanej Ci aplikacji pocztowej. Cmd to tutaj Ctrl. Twoje własne zmiany pozostają nałożone na zestaw, a Przywróć domyślne wraca do klawiszy zestawu.
settings-shortcuts-single = Skróty jednoklawiszowe
settings-shortcuts-single-detail = Klawisze bez Ctrl ani Alt, jak w poczcie internetowej: e archiwizuje, j i k przechodzą dalej i wstecz, / wyszukuje. Działają na liście i w otwartym wątku, nigdy podczas pisania.
settings-shortcuts-single-use = Używaj skrótów jednoklawiszowych
settings-shortcuts-single-use-detail = Skróty z Ctrl działają zawsze
settings-shortcuts-how = Kliknij klawisz, aby go zmienić, lub +, aby dodać nowy, a potem naciśnij nowe klawisze. Esc anuluje.
settings-shortcuts-restore = Przywróć domyślne
settings-shortcuts-no-key = Brak klawisza
settings-shortcuts-press = Naciśnij klawisze…
settings-shortcuts-then = { $keys }, potem…
settings-shortcuts-moved = { $keys } wykonuje teraz „{ $action }” zamiast „{ $previous }”.
settings-shortcuts-single-off = Skróty jednoklawiszowe są wyłączone, więc ten klawisz zadziała po ich włączeniu.
settings-shortcuts-restored = Wszystkie skróty mają znów klawisze swojego zestawu.

## Settings search: the line under a result

settings-general-language-summary = Język aplikacji, dat i liczb
settings-general-reading-summary = Najnowsza wiadomość na początku, pełne nagłówki, pełne nazwy odbiorców
settings-general-mark-read-summary = Kiedy otwarty wątek jest oznaczany jako przeczytany: od razu, po 1 lub 3 sekundach albo ręcznie
settings-general-reply-button-summary = Przycisk odpowiedzi obok każdej wiadomości odpowiada wszystkim
settings-general-remote-images-summary = Zawsze pokazuj obrazy w każdej wiadomości
settings-general-sending-summary = Cofnij wysłanie: jak długo wysłana wiadomość czeka, aby można było ją cofnąć
settings-general-offline-summary = Z ilu dni najnowsza poczta jest pobierana w całości, aby czytać ją bez połączenia
settings-general-notifications-summary = Powiadomienia o nowej poczcie i ich dźwięk
settings-general-desktop-summary = Otwieranie Katna Mail po zalogowaniu, ikona w zasobniku systemowym i liczba nieprzeczytanych na ikonie w pasku zadań
settings-accounts-accounts-summary = Dodaj lub usuń konto albo zmień jego zdjęcie
settings-appearance-density-summary = Domyślne lub kompaktowe wiersze na liście
settings-appearance-scaling-summary = Powiększ lub zmniejsz wszystko: tekst, ikony, odstępy i linie podziału
settings-appearance-theme-summary = Taki jak pulpit, jasny lub ciemny
settings-appearance-sender-pictures-summary = Logo firm wyszukiwane według domeny nadawcy
settings-appearance-important-summary = Znacznik ważności obok każdej wiadomości na liście
settings-appearance-mail-colors-summary = Ciemne kolory poczty HTML w ciemnym motywie albo kolory nadawcy
settings-appearance-attachment-previews-summary = Mały obraz zawartości każdego załącznika
settings-shortcuts-set-summary = Zacznij od klawiszy Gmail, Inbox by Gmail, Apple Mail, Outlook lub Thunderbird
settings-shortcuts-single-summary = Klawisze bez Ctrl ani Alt, jak w poczcie internetowej
settings-default-apps-pdf-summary = Gdzie otwierają się załączniki PDF
settings-default-apps-pictures-summary = Gdzie otwierają się zdjęcia i obrazy
settings-default-apps-text-summary = Gdzie otwierają się zwykły tekst, logi i kod
settings-default-apps-sheets-summary = Gdzie otwierają się pliki Excel, OpenDocument i CSV
settings-default-apps-documents-summary = Gdzie otwierają się dokumenty Word i tekst OpenDocument
settings-default-apps-after-saving-summary = Pokazuj zapisane załączniki w ich folderze
settings-compose-send-from-summary = Konto, z którego wychodzi nowa poczta: to, w którym jesteś, albo zawsze to samo
settings-compose-send-on-replies-summary = Wyślij albo Wyślij i zarchiwizuj wątek przy odpowiedziach i przekazywaniu
settings-compose-signatures-summary = Dodawany pod wiadomością, po wierszu „--”
settings-compose-for-new-mail-summary = Podpis, od którego zaczyna się nowa wiadomość
settings-compose-for-replies-summary = Podpis, od którego zaczynają się odpowiedzi i przekazywane wiadomości
settings-compose-format-summary = Pisz nowe wiadomości zwykłym tekstem
settings-compose-spelling-summary = Sprawdzanie pisowni podczas pisania i język słownika
settings-compose-templates-summary = Wkrótce: zapisuj często pisane wiadomości i zaczynaj od nich nową wiadomość lub odpowiedź
settings-feedback-crash-reports-summary = Zapisuj raporty o awariach na tym komputerze, gdy Katna Mail lub jej usługa w tle ulegnie awarii
settings-feedback-saved-summary = Wyświetl, skopiuj lub usuń raporty o awariach zapisane na tym komputerze
settings-feedback-help-improve-summary = Wysyłaj raporty o awariach, aby pomóc naprawić błędy; wyłączone, dopóki tego nie włączysz
settings-experimental-blur-summary = Pulpit prześwituje przez górny pasek, rozmyty, a menu są z matowego szkła
settings-search-shortcut = Skrót klawiszowy
settings-search-tab = Karta ustawień
settings-search-none = Żadne ustawienia nie pasują do „{ $query }”.
settings-search-results = Ustawienia pasujące do „{ $query }”

## Settings: opening at login

settings-open-at-login-failed = Nie udało się zmienić otwierania po zalogowaniu: { $error }
