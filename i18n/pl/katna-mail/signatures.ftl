# Katna Mail, Polish (Polski).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The signature being edited

signature-placeholder = Twoje imię i nazwisko, a pod nimi wszystko, co chcesz dodać

## Its formatting bar

signature-bold = Pogrubienie
signature-italic = Kursywa
signature-underline = Podkreślenie
signature-link = Link
signature-link-apply = Zastosuj
signature-picture = Wstaw obraz
signature-align-left = Wyrównaj do lewej
signature-align-center = Wyśrodkuj
signature-align-right = Wyrównaj do prawej
signature-numbered-list = Lista numerowana
signature-bulleted-list = Lista punktowana
signature-remove-formatting = Wyczyść formatowanie

## Adding a picture

signature-picture-choose = Wstaw
signature-picture-too-big = Obrazy w podpisie mogą mieć maksymalnie { $size }.
signature-picture-kind = Wybierz obraz PNG, JPEG, GIF lub WebP.
signature-picture-unreadable = { $name }: { $error }

## Layouts

signature-layout = Układ
signature-layout-own = Własny
signature-layout-classic = Klasyczny
signature-layout-logo-left = Logo po lewej
signature-layout-photo = Zdjęcie
signature-layout-band = Kolorowy pas
signature-layout-one-line = Jedna linia
signature-layout-centred = Wyśrodkowany
signature-layout-banner = Z banerem
signature-layout-underline = Podkreślenie
signature-layout-side-bar = Pasek boczny
signature-layout-card = Wizytówka
signature-layout-monogram = Monogram
signature-layout-plain = Zwykły tekst
signature-layout-mobile-label = Kom.:
signature-layout-office-label = Tel.:
signature-layout-email-label = E:
signature-layout-name = Imię i nazwisko
signature-layout-job = Stanowisko
signature-layout-company = Firma
signature-layout-mobile = Komórka
signature-layout-office = Telefon służbowy
signature-layout-email = E-mail
signature-layout-website = Strona internetowa
signature-layout-address = Adres
signature-layout-pictures = Obrazy
signature-layout-logo = Logo
signature-layout-photo-picture = Zdjęcie
signature-layout-banner-picture = Baner
signature-layout-remove-picture = Usuń
signature-layout-pages = Strony
signature-layout-page-placeholder = Dodaj adres strony
signature-layout-colour = Kolor
signature-layout-picture-failed = Nie udało się użyć pliku { $name } jako obrazu.
signature-layout-preview = Jak widzi to odbiorca
signature-layout-light = Jasny
signature-layout-dark = Ciemny
signature-layout-text = Zwykły tekst
signature-layout-inside = Obrazy są wysyłane wewnątrz wiadomości, więc wyświetlają się nawet tam, gdzie obrazy z internetu są wyłączone. Ten dodaje { $size } do każdej wiadomości.
signature-layout-edit = Edytuj ręcznie
signature-layout-edit-confirm = Edytować ręcznie? Pola i układ znikną, a podpis zachowa wygląd na tyle, na ile pozwala edytor.
signature-layout-use-confirm = Użyć układu { $layout }? Zastąpi ten podpis i zostanie wypełniony jego danymi.
signature-layout-use = Użyj układu
signature-layout-cancel = Anuluj

## Paste HTML

signature-html-title = Wklej HTML
signature-html-subtitle = Dla podpisu zaprojektowanego gdzie indziej
signature-html-placeholder = Wklej tutaj kod HTML podpisu
signature-html-name = Wklejony
signature-html-new = Zapisany jako nowy podpis „{ $name }”
signature-html-replaces = Zastępuje „{ $name }”
signature-html-cancel = Anuluj
signature-html-save = Zapisz
signature-html-fetching = Pobieranie obrazów…
signature-html-pictures-inside = { $count ->
    [one] { $count } obraz pobrany i umieszczony wewnątrz wiadomości ({ $size })
    [few] { $count } obrazy pobrane i umieszczone wewnątrz wiadomości ({ $size })
    [many] { $count } obrazów pobranych i umieszczonych wewnątrz wiadomości ({ $size })
   *[other] { $count } obrazu pobrane i umieszczone wewnątrz wiadomości ({ $size })
}
signature-html-pictures-web = { $count ->
    [one] Nie udało się pobrać { $count } obrazu, więc odbiorcy wczytają go z internetu
    [few] Nie udało się pobrać { $count } obrazów, więc odbiorcy wczytają je z internetu
    [many] Nie udało się pobrać { $count } obrazów, więc odbiorcy wczytają je z internetu
   *[other] Nie udało się pobrać { $count } obrazu, więc odbiorcy wczytają je z internetu
}
signature-html-removed = Usunięto skrypty, formularze i piksele śledzące, które aplikacje pocztowe i tak blokują
signature-html-style-sheet = Pominięto arkusz stylów: poczta zachowuje tylko style zapisane przy każdym elemencie
signature-html-links = Usunięto linki prowadzące gdzie indziej niż do strony internetowej, adresu e-mail lub telefonu
signature-html-plain-text = Utworzono z niego wersję tekstową dla aplikacji pocztowych, które pokazują tylko tekst

## Import

signature-import-title = Importuj
signature-import-subtitle = Z Gmail, Thunderbird, Evolution i KMail
signature-import-looking = Szukanie podpisów…
signature-import-none = Nie znaleziono podpisów. Dla innej aplikacji skopiuj HTML jej podpisu i użyj Wklej HTML.
signature-import-from = Z { $app }
signature-import-already = już jest w Katna
signature-import-gmail-sign-in = { $address }: zaloguj się ponownie w Ustawienia > Konta, aby Katna mogła odczytać podpisy z Gmail.
signature-import-gmail-failed = { $address }: { $error }
signature-import-cancel = Anuluj
signature-import-do = { $count ->
    [one] Importuj { $count } podpis
    [few] Importuj { $count } podpisy
    [many] Importuj { $count } podpisów
   *[other] Importuj { $count } podpisu
}
signature-import-name = { $name } ({ $app })
