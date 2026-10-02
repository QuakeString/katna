# Katna Mail, Polish (Polski).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = Szukaj plików

## Left side (and chips on a phone)

files-all = Wszystkie pliki
files-pictures = Obrazy
files-pdfs = PDF-y
files-documents = Dokumenty
files-sheets = Arkusze kalkulacyjne
files-slides = Prezentacje
files-other = Inne
files-accounts = Konta
files-drives = Dyski
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = Udostępnione mi
files-shown = Widoczne
files-received = Odebrane
files-sent = Wysłane przeze mnie

## Over the files

files-count = { $count ->
    [one] { $count } plik · { $size }
    [few] { $count } pliki · { $size }
    [many] { $count } plików · { $size }
   *[other] { $count } pliku · { $size }
}
files-anyone = Ktokolwiek
files-from-person = Od: { $name }
files-time-any = Dowolny czas
files-time-today = Dzisiaj
files-time-yesterday = Wczoraj
files-time-this-week = Ten tydzień
files-time-last-week = Zeszły tydzień
files-time-this-month = Ten miesiąc
files-time-last-month = Zeszły miesiąc
files-time-between = { $first } – { $last }
files-time-hint = Kliknij dzień albo przeciągnij przez kilka dni
files-time-summary = { $count ->
    [one] { $days } · { $count } plik
    [few] { $days } · { $count } pliki
    [many] { $days } · { $count } plików
   *[other] { $days } · { $count } pliku
}
files-time-clear = Wyczyść
files-time-month-back = Poprzedni miesiąc
files-time-month-on = Następny miesiąc
files-time-wheel = Przewiń, aby przesunąć te daty z zachowaniem ich długości
files-sort-newest = Najnowsze na początku
files-sort-oldest = Najstarsze na początku
files-sort-largest = Największe na początku
files-sort-name = Według nazwy
files-grid = Karty
files-list = Lista
files-this-week = Ten tydzień
files-undated = Bez daty
files-me = Ja
files-no-subject = (bez tematu)
files-loading = Zbieranie plików z poczty…
files-empty = Tutaj pojawiają się pliki z Twojej poczty.
files-none-match = Żaden plik nie pasuje.
files-load-failed = Nie udało się odczytać plików: { $error }

## A file's menu and buttons

files-open = Otwórz
files-open-with = Otwórz za pomocą…
files-save = Zapisz…
files-show-mail = Pokaż wiadomość
files-mail-window = Otwórz wiadomość w nowym oknie
files-forward = Przekaż plik dalej
files-from-them = Pliki od: { $name }
files-copy-name = Kopiuj nazwę pliku
files-name-copied = Skopiowano nazwę pliku
files-downloading = Pobieranie wiadomości…
files-download-failed = Nie udało się pobrać tej wiadomości.

## A cloud drive in place of the mail files

files-drive-mine = Mój dysk
files-drive-mine-onedrive = Moje pliki
files-drive-results = „{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] 1 plik
        [few] { $files } pliki
        [many] { $files } plików
       *[other] { $files } pliku
    }
    [one] 1 folder · { $files ->
        [one] 1 plik
        [few] { $files } pliki
        [many] { $files } plików
       *[other] { $files } pliku
    }
    [few] { $folders } foldery · { $files ->
        [one] 1 plik
        [few] { $files } pliki
        [many] { $files } plików
       *[other] { $files } pliku
    }
    [many] { $folders } folderów · { $files ->
        [one] 1 plik
        [few] { $files } pliki
        [many] { $files } plików
       *[other] { $files } pliku
    }
   *[other] { $folders } folderu · { $files ->
        [one] 1 plik
        [few] { $files } pliki
        [many] { $files } plików
       *[other] { $files } pliku
    }
}
files-drive-folders = Foldery
files-drive-files = Pliki
files-drive-folder = Folder
files-drive-meta = { $what } · Edytowano { $date }
files-drive-as-link = { $what } · jako link
files-drive-google-doc = Dokument Google
files-drive-google-sheet = Arkusz Google
files-drive-google-slides = Prezentacja Google
files-drive-google-drawing = Rysunek Google
files-drive-fetching = Pobieranie…
files-drive-loading = Otwieranie dysku…
files-drive-empty = Ten folder jest pusty.
files-drive-unreachable = Brak połączenia z { $drive }.
files-drive-try-again = Spróbuj ponownie
files-drive-needs-permission = Katna potrzebuje jednorazowo Twojej zgody, aby pokazać ten dysk. Zaloguj się ponownie i pozwól Katna widzieć Twoje pliki.
files-drive-allow = Zezwól
files-drive-allow-failed = Logowanie nie zostało ukończone, więc dysk pozostaje zamknięty.
files-drive-attach = Załącz
files-drive-more = Więcej
files-drive-download = Pobierz…
files-drive-open-web = Otwórz w { $drive }
files-drive-copy-link = Kopiuj link
files-drive-link-copied = Skopiowano link
files-drive-share = Udostępnij…
files-drive-rename = Zmień nazwę
files-drive-trash = Przenieś do kosza
files-drive-trashed = „{ $name }” jest w koszu { $drive }
files-drive-renamed = Zmieniono nazwę na „{ $name }”
files-drive-getting = Pobieranie { $name } z { $drive }…
files-drive-get-failed = Nie udało się pobrać { $name }: { $error }
files-drive-upload = Prześlij
files-drive-upload-files = Prześlij pliki
files-drive-upload-folder = Prześlij folder
files-drive-upload-failed = Nie udało się przesłać { $name }: { $error }
files-drive-upload-needs = Aby przesyłać, Katna potrzebuje jednorazowo Twojej zgody: naciśnij Zezwól w Ustawienia › Domyślne aplikacje › Strona Pliki.

## The Share dialog of a drive file or folder

files-share-title = Udostępnij „{ $name }”
files-share-add = Dodaj osoby według nazwy lub adresu
files-share-not-address = „{ $text }” nie jest adresem e-mail
files-share-notify = Niech { $drive } też wyśle im e-mail
files-share-people = Osoby z dostępem
files-share-general = Dostęp ogólny
files-share-loading = Sprawdzanie, kto ma dostęp…
files-share-restricted = Ograniczony
files-share-restricted-about = Tylko osoby z dostępem mogą otworzyć to przez link
files-share-anyone = Każdy, kto ma link
files-share-anyone-can = { $role ->
    [editor] Każdy, kto ma link, może edytować
    [commenter] Każdy, kto ma link, może komentować
   *[viewer] Każdy, kto ma link, może wyświetlać
}
files-share-anyone-about = { $role ->
    [editor] Każdy w internecie, kto ma link, może edytować
    [commenter] Każdy w internecie, kto ma link, może komentować
   *[viewer] Każdy w internecie, kto ma link, może wyświetlać
}
files-share-role-owner = Właściciel
files-share-role-editor = Edytujący
files-share-role-commenter = Komentujący
files-share-role-viewer = Przeglądający
files-share-you = { $name } (Ty)
files-share-domain = Wszyscy w { $domain }
files-share-inherited = Dostęp z folderu, w którym się znajduje
files-share-remove = Usuń dostęp
files-share-copy-link = Kopiuj link
files-share-share = Udostępnij
files-share-done = Gotowe
files-share-close = Zamknij
files-share-sharing = Udostępnianie…
files-share-shared = { $count ->
    [one] Udostępniono 1 osobie
    [few] Udostępniono { $count } osobom
    [many] Udostępniono { $count } osobom
   *[other] Udostępniono { $count } osoby
}
files-share-refused = { $drive } nie udostępnił: { $addresses }
files-share-failed = Nie udało się zmienić udostępniania: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] Przesyłanie 1 elementu
    [few] Przesyłanie { $count } elementów
    [many] Przesyłanie { $count } elementów
   *[other] Przesyłanie { $count } elementu
}
files-tray-done = { $count ->
    [one] Przesłano 1 element
    [few] Przesłano { $count } elementy
    [many] Przesłano { $count } elementów
   *[other] Przesłano { $count } elementu
}
files-tray-some-failed = Przesłano: { $done }, niepowodzenia: { $failed }
files-tray-minutes-left = { $minutes ->
    [one] Została około minuta
    [few] Zostały około { $minutes } minuty
    [many] Zostało około { $minutes } minut
   *[other] Zostało około { $minutes } minuty
}
files-tray-seconds-left = Została niecała minuta
files-tray-starting = Rozpoczynanie…
files-tray-cancel-all = Anuluj wszystko
files-tray-cancel = Anuluj
files-tray-fold = Ukryj listę
files-tray-unfold = Pokaż listę
files-tray-close = Zamknij
files-tray-progress = { $place } · { $sent } z { $size }
files-tray-in = W { $place }
files-tray-cancelled = Anulowano
