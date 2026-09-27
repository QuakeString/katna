# Katna Mail, Polish (Polski).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = Język: { $language }
language-tooltip-system = Język: { $language }, zgodnie z systemem
language-search = Szukaj języka
language-system-default = Domyślny systemowy
language-system-now = Obecnie: { $language }
language-no-match = Żaden język nie pasuje do „{ $query }”
language-machine = Przetłumaczono maszynowo. Pomóż to poprawić
language-setting = Język
language-setting-detail = Język menu, przycisków i komunikatów oraz format dat i liczb. „Domyślny systemowy” jest zgodny z ustawieniami pulpitu.

## Dates and sizes

ago-just-now = przed chwilą
ago-minutes = { $count ->
    [one] { $count } minutę temu
    [few] { $count } minuty temu
    [many] { $count } minut temu
   *[other] { $count } minuty temu
}
ago-hours = { $count ->
    [one] { $count } godzinę temu
    [few] { $count } godziny temu
    [many] { $count } godzin temu
   *[other] { $count } godziny temu
}
ago-days = { $count ->
    [one] { $count } dzień temu
    [few] { $count } dni temu
    [many] { $count } dni temu
   *[other] { $count } dnia temu
}
size-bytes = { $count ->
    [one] { $count } bajt
    [few] { $count } bajty
    [many] { $count } bajtów
   *[other] { $count } bajta
}
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = Ukryj foldery
folders-show = Pokaż foldery
compose = Utwórz
search = Szukaj
search-mail = Przeszukaj pocztę
search-settings = Przeszukaj ustawienia
search-clear = Wyczyść wyszukiwanie
search-options-show = Pokaż opcje wyszukiwania
settings = Ustawienia
account-add = Dodaj konto

## App rail (and the bottom bar on a phone)

rail-mail = Poczta
rail-calendar = Kalendarz
rail-contacts = Kontakty
rail-tasks = Zadania
rail-notes = Notatki
rail-feeds = Kanały

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Wkrótce
app-calendar-promise = Twoje kalendarze CalDAV, zaproszenia na spotkania z poczty i przypomnienia – obok skrzynki odbiorczej.
app-tasks-promise = Listy zadań synchronizowane przez CalDAV i zadania tworzone z wiadomości.
app-notes-promise = Szybkie notatki oraz notatki do wiadomości lub wątku na później.
app-feeds-promise = Czytaj kanały RSS i Atom obok poczty.

## Contacts page

app-contacts-loading = Zbieranie osób z poczty…
app-contacts-empty = Tutaj pojawią się osoby, z którymi korespondujesz.
app-contacts-count = { $count ->
    [one] { $count } osoba z poczty, najczęstsi rozmówcy na początku
    [few] { $count } osoby z poczty, najczęstsi rozmówcy na początku
    [many] { $count } osób z poczty, najczęstsi rozmówcy na początku
   *[other] { $count } osoby z poczty, najczęstsi rozmówcy na początku
}
app-contacts-top = { $count ->
    [one] Najczęstszy rozmówca z poczty
    [few] Pierwsze { $count } osoby z poczty, najczęstsi rozmówcy na początku
    [many] Pierwszych { $count } osób z poczty, najczęstsi rozmówcy na początku
   *[other] Pierwsze { $count } osoby z poczty, najczęstsi rozmówcy na początku
}
app-contacts-messages = { $count ->
    [one] { $count } wiadomość
    [few] { $count } wiadomości
    [many] { $count } wiadomości
   *[other] { $count } wiadomości
}
app-contacts-last = ostatnio { $date }

## Navigation (the folders pane)

nav-labels = Etykiety
nav-folders = Foldery
nav-label-new = Utwórz nową etykietę
nav-folder-new = Utwórz nowy folder
nav-account-unnamed = Konto { $number }
nav-tab-new = { $count ->
    [one] { $count } nowa
    [few] { $count } nowe
    [many] { $count } nowych
   *[other] { $count } nowej
}

## Special folders (the user's own folders keep their names)

folder-inbox = Odebrane
folder-starred = Oznaczone gwiazdką
folder-drafts = Wersje robocze
folder-sent = Wysłane
folder-archive = Archiwum
folder-spam = Spam
folder-trash = Kosz
folder-all-mail = Wszystkie
folder-scheduled = Zaplanowane

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Nowa etykieta
label-folder-new-title = Nowy folder
label-prompt = Wpisz nazwę nowej etykiety:
label-folder-prompt = Wpisz nazwę nowego folderu:
label-name-hint = Nazwa etykiety
label-folder-name-hint = Nazwa folderu
label-nest = Zagnieźdź etykietę pod:
label-folder-nest = Zagnieźdź folder pod:
label-cancel = Anuluj
label-create = Utwórz
label-creating = Tworzenie…
label-created = Utworzono etykietę „{ $name }”.
label-folder-created = Utworzono folder „{ $name }”.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Główne
tab-promotions = Oferty
tab-social = Społeczności
tab-updates = Powiadomienia
tab-forums = Fora
tab-focused = Priorytetowe
tab-other = Inne
tab-inbox = Odebrane
tab-newsletters = Biuletyny
tab-notifications = Powiadomienia
tab-new = { $count ->
    [one] { $count } nowa
    [few] { $count } nowe
    [many] { $count } nowych
   *[other] { $count } nowej
}
tab-provider-other = sortuje Katna

## Mail list: toolbar

list-select = Zaznacz
list-refresh = Odśwież
list-more = Więcej
list-mark-read = Oznacz jako przeczytane
list-mark-unread = Oznacz jako nieprzeczytane
list-move-to = Przenieś do
list-archive = Archiwizuj
list-spam = Zgłoś spam
list-delete = Usuń
list-newer = Nowsze
list-older = Starsze
list-range = { $first }–{ $last } z { $total }
list-range-about = { $first }–{ $last } z około { $total }
list-results = Wyniki dla „{ $query }”
list-results-corrected = Wyświetlane są wyniki dla „{ $query }”
list-search-instead = Zamiast tego szukaj „{ $query }”
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Wszystkie
list-pick-none = Żadne
list-pick-read = Przeczytane
list-pick-unread = Nieprzeczytane
list-pick-starred = Oznaczone gwiazdką
list-pick-unstarred = Bez gwiazdki

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] Zaznaczono { $count } wątek.
        [few] Zaznaczono wszystkie { $count } wątki.
        [many] Zaznaczono wszystkie { $count } wątków.
       *[other] Zaznaczono wszystkie wątki ({ $count }).
    }
   *[message] { $count ->
        [one] Zaznaczono { $count } wiadomość.
        [few] Zaznaczono wszystkie { $count } wiadomości.
        [many] Zaznaczono wszystkie { $count } wiadomości.
       *[other] Zaznaczono wszystkie wiadomości ({ $count }).
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] Zaznaczono { $count } wątek w folderze { $folder }.
        [few] Zaznaczono wszystkie { $count } wątki w folderze { $folder }.
        [many] Zaznaczono wszystkie { $count } wątków w folderze { $folder }.
       *[other] Zaznaczono wszystkie wątki ({ $count }) w folderze { $folder }.
    }
   *[message] { $count ->
        [one] Zaznaczono { $count } wiadomość w folderze { $folder }.
        [few] Zaznaczono wszystkie { $count } wiadomości w folderze { $folder }.
        [many] Zaznaczono wszystkie { $count } wiadomości w folderze { $folder }.
       *[other] Zaznaczono wszystkie wiadomości ({ $count }) w folderze { $folder }.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] Zaznaczono { $count } wątek na tej stronie.
        [few] Zaznaczono wszystkie { $count } wątki na tej stronie.
        [many] Zaznaczono wszystkie { $count } wątków na tej stronie.
       *[other] Zaznaczono wszystkie wątki ({ $count }) na tej stronie.
    }
   *[message] { $count ->
        [one] Zaznaczono { $count } wiadomość na tej stronie.
        [few] Zaznaczono wszystkie { $count } wiadomości na tej stronie.
        [many] Zaznaczono wszystkie { $count } wiadomości na tej stronie.
       *[other] Zaznaczono wszystkie wiadomości ({ $count }) na tej stronie.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] Zaznacz { $count } wątek
        [few] Zaznacz wszystkie { $count } wątki
        [many] Zaznacz wszystkie { $count } wątków
       *[other] Zaznacz wszystkie wątki ({ $count })
    }
   *[message] { $count ->
        [one] Zaznacz { $count } wiadomość
        [few] Zaznacz wszystkie { $count } wiadomości
        [many] Zaznacz wszystkie { $count } wiadomości
       *[other] Zaznacz wszystkie wiadomości ({ $count })
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] Zaznacz { $count } wątek w folderze { $folder }
        [few] Zaznacz wszystkie { $count } wątki w folderze { $folder }
        [many] Zaznacz wszystkie { $count } wątków w folderze { $folder }
       *[other] Zaznacz wszystkie wątki ({ $count }) w folderze { $folder }
    }
   *[message] { $count ->
        [one] Zaznacz { $count } wiadomość w folderze { $folder }
        [few] Zaznacz wszystkie { $count } wiadomości w folderze { $folder }
        [many] Zaznacz wszystkie { $count } wiadomości w folderze { $folder }
       *[other] Zaznacz wszystkie wiadomości ({ $count }) w folderze { $folder }
    }
}
list-clear-selection = Wyczyść zaznaczenie

## Mail list: empty states

list-empty-search = Brak wiadomości pasujących do wyszukiwania.
list-empty-tab = Brak poczty na karcie { $tab }.
list-empty-tab-unknown = Brak poczty na tej karcie.
list-empty-folder = Brak wiadomości w folderze { $folder }.
list-empty-folder-unknown = Brak wiadomości w tym folderze.
list-first-sync = Pobieranie poczty…
list-first-sync-detail = Wiadomości pojawią się tutaj, gdy tylko dotrą.

## Mail list: lines

row-removed = Ta wiadomość została usunięta.
row-starred = Oznaczone gwiazdką
row-not-starred = Bez gwiazdki
row-important = Ważne. Kliknij, aby oznaczyć jako nieważne.
row-mark-important = Oznacz jako ważne
row-pinned = Przypięte na górze
row-pin = Przypnij na górze
row-unpin = Odepnij

## Mail list: More menu and right-click menu

menu-reply = Odpowiedz
menu-reply-all = Odpowiedz wszystkim
menu-forward = Przekaż dalej
menu-archive = Archiwizuj
menu-delete = Usuń
menu-spam = Zgłoś spam
menu-mark-read = Oznacz jako przeczytane
menu-mark-unread = Oznacz jako nieprzeczytane
menu-mark-all-read = Oznacz wszystkie jako przeczytane
menu-star = Dodaj gwiazdkę
menu-unstar = Usuń gwiazdkę
menu-important = Oznacz jako ważne
menu-not-important = Oznacz jako nieważne
menu-pin = Przypnij na górze
menu-unpin = Odepnij
menu-print-all = Drukuj wszystko
menu-new-window = Otwórz w nowym oknie
menu-move-to = Przenieś do
menu-move-to-heading = Przenieś do:
menu-find-from = Znajdź e-maile od { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] Wątek zarchiwizowany.
        [few] Zarchiwizowano { $count } wątki.
        [many] Zarchiwizowano { $count } wątków.
       *[other] Zarchiwizowano { $count } wątku.
    }
   *[message] { $count ->
        [one] Wiadomość zarchiwizowana.
        [few] Zarchiwizowano { $count } wiadomości.
        [many] Zarchiwizowano { $count } wiadomości.
       *[other] Zarchiwizowano { $count } wiadomości.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] Wątek przeniesiony do kosza.
        [few] Przeniesiono { $count } wątki do kosza.
        [many] Przeniesiono { $count } wątków do kosza.
       *[other] Przeniesiono { $count } wątku do kosza.
    }
   *[message] { $count ->
        [one] Wiadomość przeniesiona do kosza.
        [few] Przeniesiono { $count } wiadomości do kosza.
        [many] Przeniesiono { $count } wiadomości do kosza.
       *[other] Przeniesiono { $count } wiadomości do kosza.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] Wątek przeniesiony.
        [few] Przeniesiono { $count } wątki.
        [many] Przeniesiono { $count } wątków.
       *[other] Przeniesiono { $count } wątku.
    }
   *[message] { $count ->
        [one] Wiadomość przeniesiona.
        [few] Przeniesiono { $count } wiadomości.
        [many] Przeniesiono { $count } wiadomości.
       *[other] Przeniesiono { $count } wiadomości.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] Wątek oznaczony gwiazdką.
        [few] Oznaczono gwiazdką { $count } wątki.
        [many] Oznaczono gwiazdką { $count } wątków.
       *[other] Oznaczono gwiazdką { $count } wątku.
    }
   *[message] { $count ->
        [one] Wiadomość oznaczona gwiazdką.
        [few] Oznaczono gwiazdką { $count } wiadomości.
        [many] Oznaczono gwiazdką { $count } wiadomości.
       *[other] Oznaczono gwiazdką { $count } wiadomości.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] Usunięto gwiazdkę z wątku.
        [few] Usunięto gwiazdkę z { $count } wątków.
        [many] Usunięto gwiazdkę z { $count } wątków.
       *[other] Usunięto gwiazdkę z { $count } wątku.
    }
   *[message] { $count ->
        [one] Usunięto gwiazdkę z wiadomości.
        [few] Usunięto gwiazdkę z { $count } wiadomości.
        [many] Usunięto gwiazdkę z { $count } wiadomości.
       *[other] Usunięto gwiazdkę z { $count } wiadomości.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] Wątek oznaczony jako ważny.
        [few] Oznaczono { $count } wątki jako ważne.
        [many] Oznaczono { $count } wątków jako ważne.
       *[other] Oznaczono { $count } wątku jako ważne.
    }
   *[message] { $count ->
        [one] Wiadomość oznaczona jako ważna.
        [few] Oznaczono { $count } wiadomości jako ważne.
        [many] Oznaczono { $count } wiadomości jako ważne.
       *[other] Oznaczono { $count } wiadomości jako ważne.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] Wątek oznaczony jako nieważny.
        [few] Oznaczono { $count } wątki jako nieważne.
        [many] Oznaczono { $count } wątków jako nieważne.
       *[other] Oznaczono { $count } wątku jako nieważne.
    }
   *[message] { $count ->
        [one] Wiadomość oznaczona jako nieważna.
        [few] Oznaczono { $count } wiadomości jako nieważne.
        [many] Oznaczono { $count } wiadomości jako nieważne.
       *[other] Oznaczono { $count } wiadomości jako nieważne.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] Wątek przypięty na górze.
        [few] Przypięto { $count } wątki na górze.
        [many] Przypięto { $count } wątków na górze.
       *[other] Przypięto { $count } wątku na górze.
    }
   *[message] { $count ->
        [one] Wiadomość przypięta na górze.
        [few] Przypięto { $count } wiadomości na górze.
        [many] Przypięto { $count } wiadomości na górze.
       *[other] Przypięto { $count } wiadomości na górze.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] Wątek odpięty.
        [few] Odpięto { $count } wątki.
        [many] Odpięto { $count } wątków.
       *[other] Odpięto { $count } wątku.
    }
   *[message] { $count ->
        [one] Wiadomość odpięta.
        [few] Odpięto { $count } wiadomości.
        [many] Odpięto { $count } wiadomości.
       *[other] Odpięto { $count } wiadomości.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] Wątek zgłoszony jako spam.
        [few] Zgłoszono { $count } wątki jako spam.
        [many] Zgłoszono { $count } wątków jako spam.
       *[other] Zgłoszono { $count } wątku jako spam.
    }
   *[message] { $count ->
        [one] Wiadomość zgłoszona jako spam.
        [few] Zgłoszono { $count } wiadomości jako spam.
        [many] Zgłoszono { $count } wiadomości jako spam.
       *[other] Zgłoszono { $count } wiadomości jako spam.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] Wątek trwale usunięty.
        [few] Trwale usunięto { $count } wątki.
        [many] Trwale usunięto { $count } wątków.
       *[other] Trwale usunięto { $count } wątku.
    }
   *[message] { $count ->
        [one] Wiadomość trwale usunięta.
        [few] Trwale usunięto { $count } wiadomości.
        [many] Trwale usunięto { $count } wiadomości.
       *[other] Trwale usunięto { $count } wiadomości.
    }
}
toast-undone = Cofnięto działanie.
toast-undo = Cofnij
toast-no-spam-folder = To konto nie ma folderu spamu.

## Reading pane: toolbar

reader-close = Zamknij
reader-back = Wstecz
reader-mark-unread = Oznacz jako nieprzeczytane
reader-move-to = Przenieś do
reader-more = Więcej
reader-print-all = Drukuj wszystko
reader-new-window = W nowym oknie
reader-position = { $position } z { $total }
reader-newer = Nowszy
reader-older = Starszy

## Reading pane: the conversation

reader-removed = Ten wątek został usunięty.
reader-no-subject = (bez tematu)
reader-collapse-all = Zwiń wszystko
reader-expand-all = Rozwiń wszystko
reader-unknown-sender = (nieznany nadawca)
reader-date-ago = { $date } ({ $ago })
reader-me = mnie
reader-to = do { $names }
reader-starred = Oznaczone gwiazdką
reader-not-starred = Bez gwiazdki
reader-too-long = Wiadomość jest zbyt długa, aby wyświetlić ją w całości.
reader-encrypted-images = Obrazy z internetu nigdy nie są wczytywane w zaszyfrowanej poczcie.
reader-window-failed = Nie udało się otworzyć nowego okna.

## Reading pane: message details (opened from "to me")

reader-details-from = od:
reader-details-to = do:
reader-details-cc = dw:
reader-details-date = data:
reader-details-subject = temat:

## Reading pane: downloading a message

reader-downloading = Pobieranie tej wiadomości z serwera…
reader-download-failed = Nie udało się pobrać tej wiadomości.
reader-try-again = Spróbuj ponownie

## Reply row

reply-reply = Odpowiedz
reply-reply-all = Odpowiedz wszystkim
reply-forward = Przekaż dalej

## Encrypted and signed mail

security-decrypting = Odszyfrowywanie…
security-checking = Sprawdzanie podpisu…
security-partly-encrypted = Tylko część tej wiadomości jest zaszyfrowana. Reszta została dodana poza ochroną i może pochodzić od kogokolwiek.
security-partly-signed = Tylko część tej wiadomości jest podpisana. Reszta została dodana poza ochroną i może pochodzić od kogokolwiek.
security-encrypted = Zaszyfrowana wiadomość
security-encrypted-smime = Zaszyfrowana wiadomość (S/MIME)
security-no-key = Nie można odszyfrować tej wiadomości: została zaszyfrowana dla klucza, którego nie masz.
security-cancelled = Odszyfrowywanie zostało anulowane.
security-damaged = Nie można odszyfrować tej wiadomości: zaszyfrowane dane są uszkodzone lub zostały zmienione.
security-decrypt-unavailable = Nie można odszyfrować tej wiadomości: zainstaluj { $tool }, aby czytać zaszyfrowaną pocztę.
security-decrypt-failed = Nie można odszyfrować tej wiadomości: { $reason }
security-unknown-signer = nieznany podpisujący
security-signed-verified = Podpis: { $signer } · zweryfikowany
security-signed-not-sender = Podpis: { $signer } · podpisujący nie jest nadawcą
security-signed-untrusted = Podpis: { $signer } · klucz oznaczony przez Ciebie jako niezaufany
security-signed-unverified = Podpis: { $signer } · klucz nie jest zweryfikowany
security-bad-signature = Nieprawidłowy podpis: ta wiadomość została zmieniona po podpisaniu lub podpis jest sfałszowany.
security-signature-expired = Podpis: { $signer } · podpis wygasł
security-key-expired = Podpis: { $signer } · klucz od tego czasu wygasł
security-key-revoked = Podpis: { $signer } · klucz został unieważniony
security-missing-key = Podpisano kluczem, którego nie masz, więc nie można tego sprawdzić
security-missing-key-id = Podpisano kluczem, którego nie masz ({ $key }), więc nie można tego sprawdzić
security-signature-unavailable = Podpisano; zainstaluj { $tool }, aby sprawdzić podpis
security-signature-error = Nie udało się sprawdzić podpisu.

## Remote images and pictures

remote-hidden = Obrazy w tej wiadomości są ukryte.
remote-show = Pokaż obrazy
remote-always-show = Zawsze pokazuj od tego nadawcy
remote-picture-use = Użyj
remote-picture-too-big = Wybierz obraz o rozmiarze do 8 MB.
remote-picture-type = Wybierz obraz PNG, JPEG, GIF, WebP lub SVG.
remote-picture-read-failed = Nie można odczytać obrazu: { $error }
remote-picture-keep-failed = Nie można zachować obrazu: { $error }
remote-picture-remove-failed = Nie można usunąć obrazu: { $error }

## Attachments

attachment-count = { $count ->
    [one] Jeden załącznik
    [few] { $count } załączniki
    [many] { $count } załączników
   *[other] { $count } załącznika
}
attachment-save = Zapisz
attachment-save-all = Zapisz wszystkie
attachment-save-all-tooltip = Zapisz wszystkie załączniki w folderze
attachment-save-here = Zapisz tutaj
attachment-not-downloaded = Ta wiadomość nie jest pobrana.
attachment-not-found = Nie znaleziono tego załącznika w wiadomości.
attachment-read-failed = Nie udało się odczytać pliku { $name }
attachment-numbered = załącznik { $number }
attachment-saved-all = { $count ->
    [one] Zapisano { $count } plik w folderze { $place }
    [few] Zapisano { $count } pliki w folderze { $place }
    [many] Zapisano { $count } plików w folderze { $place }
   *[other] Zapisano { $count } pliku w folderze { $place }
}
attachment-saved-some = { $total ->
    [one] Zapisano { $saved } z { $total } pliku w folderze { $place }. Nie udało się zapisać: { $failed }
    [few] Zapisano { $saved } z { $total } plików w folderze { $place }. Nie udało się zapisać: { $failed }
    [many] Zapisano { $saved } z { $total } plików w folderze { $place }. Nie udało się zapisać: { $failed }
   *[other] Zapisano { $saved } z { $total } pliku w folderze { $place }. Nie udało się zapisać: { $failed }
}
attachment-saved-to = Zapisano w { $path }
attachment-save-failed = Nie udało się zapisać pliku { $name }: { $error }
attachment-open-failed = Nie udało się otworzyć pliku { $name }: { $error }
attachment-risky = Ten plik może uruchomić program, więc Katna go nie otwiera. Zamiast tego zapisz go.
attachment-encrypted-open = Ten plik dotarł zaszyfrowany. Zapisz go, aby otworzyć go w innym miejscu.

## Printing

print-failed = Nie udało się wydrukować: { $error }
print-no-font = nie znaleziono czcionki
print-opened-as-pdf = Otwarto jako PDF, aby wydrukować z tego miejsca.
print-not-downloaded = (Jeszcze nie pobrano.)
print-encrypted = (Zaszyfrowana. Otwórz ją w Katna Mail, aby wydrukować jej treść.)
print-to = Do: { $addresses }
print-cc = DW: { $addresses }
