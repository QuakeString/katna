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

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Otwórz tę wiadomość, aby odczytać jej załączniki.
text-copy = Kopiuj
text-select-all = Zaznacz wszystko

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
## Quick settings (the panel that slides in from the right)

quick-title = Szybkie ustawienia
quick-see-all = Zobacz wszystkie ustawienia
quick-reading-pane = Okienko odczytu
quick-pane-right = Na prawo od listy
quick-pane-none = Bez podziału
quick-density = Gęstość
quick-density-default = Domyślna
quick-density-compact = Kompaktowa
quick-theme = Motyw
quick-theme-system = Taki jak pulpit
quick-theme-light = Jasny
quick-theme-dark = Ciemny
quick-desktop-colors = Kolory pulpitu
quick-desktop-colors-detail = Schemat kolorów i kolor akcentu pulpitu
quick-app-names = Nazwy aplikacji
quick-app-names-detail = Nazwy pod ikonami aplikacji po lewej stronie
quick-inbox-tabs = Karty skrzynki odbiorczej
quick-inbox-tabs-detail = Karty dostawcy poczty każdego konta
quick-choose-tabs = Wybierz karty
quick-choose-tabs-detail = Dla każdego konta, w Ustawieniach
quick-sending = Wysyłanie
quick-undo-send = Cofnij wysłanie
quick-undo-send-off = Wyłączone
quick-undo-send-seconds = { $seconds } s
quick-signatures = Podpisy
quick-signatures-none = Jeszcze brak
quick-signatures-one = { $name }, używany domyślnie
quick-signatures-many = { $count ->
    [one] { $count } podpis; domyślnie { $name }
    [few] { $count } podpisy; domyślnie { $name }
    [many] { $count } podpisów; domyślnie { $name }
   *[other] { $count } podpisu; domyślnie { $name }
}
quick-signatures-no-default = { $count ->
    [one] { $count }, żaden domyślny
    [few] { $count }, żaden domyślny
    [many] { $count }, żaden domyślny
   *[other] { $count }, żaden domyślny
}
quick-signature-untitled = Bez nazwy
quick-threading = Wątki
quick-conversation-view = Widok wątków
quick-conversation-view-detail = Grupuj odpowiedzi na ten sam e-mail
quick-help = Pomoc
quick-tour = Obejrzyj przewodnik
quick-whats-new = Co nowego
quick-about = O Katna

## Settings: opening at login

settings-open-at-login-failed = Nie udało się zmienić otwierania po zalogowaniu: { $error }

## Settings > Appearance > Scaling

scale-letter = A
scale-percent = { $percent }%
scale-reset = Przywróć { $percent }%

## Settings > Experimental > Look & Feel

look-intro = Funkcje, które są jeszcze testowane. Mogą się zmienić lub zniknąć.
look-heading = Wygląd i zachowanie
look-window-frame = Ramka okna
look-window-frame-detail = Kto rysuje pasek tytułu, przyciski okna, narożniki i cień.
look-frame-native-kde = Natywna: ramka KDE, w Twoim motywie Plasma
look-frame-native = Natywna: ramka pulpitu
look-frame-katna = Katna: górny pasek staje się paskiem tytułu
look-frame-katna-note-named = Katna rysuje zaokrąglone narożniki i własny cień. Ramka nie podąża już za motywem { $desktop }; reguły okien nadal obowiązują.
look-frame-katna-note = Katna rysuje zaokrąglone narożniki i własny cień. Ramka nie podąża już za motywem pulpitu; reguły okien nadal obowiązują.
look-frame-client-side = Twój pulpit zostawia ramkę każdej aplikacji, więc Katna już rysuje własną.
look-blurred-background = Rozmyte tło
look-blurred-background-detail = Pulpit prześwituje przez górny pasek i foldery, rozmyty, a menu i okienka wyskakujące są z matowego szkła.
look-blur = Rozmywaj to, co jest za oknem
look-blur-detail = Poczta pozostaje na nieprzezroczystych kartach, więc tekst zachowuje kontrast
look-blur-off-kde = Efekt rozmycia KDE jest wyłączony. Włącz Rozmycie w Ustawieniach systemowych, Zarządzanie oknami, Efekty pulpitu, a potem otwórz ponownie Katna Mail.
look-blur-none-gnome = GNOME nie rozmywa tego, co jest za oknami.
look-blur-none-x11 = Twój menedżer okien nie rozmywa tego, co jest za oknami.
look-blur-none-wayland = Twój kompozytor nie rozmywa tego, co jest za oknami.

## Settings > User feedback (crash reports)

feedback-intro-sending = Nowe raporty o awariach są wysyłane, aby pomóc naprawić błędy. Nic więcej nie opuszcza tego komputera.
feedback-intro-local = Katna niczego nigdzie nie wysyła. Raporty o awariach zostają na tym komputerze, abyś mógł je przejrzeć lub dołączyć do zgłoszenia błędu.
feedback-crash-reports = Raporty o awariach
feedback-crash-reports-detail = Tworzone, gdy Katna Mail lub jej usługa w tle ulegnie awarii.
feedback-save = Zapisuj raporty o awariach na tym komputerze
feedback-save-detail = Folder domowy, nazwy użytkownika i komputera oraz adresy e-mail są pomijane
feedback-saved = Zapisane raporty o awariach
feedback-saved-detail = { $count ->
    [one] Przechowywany jest najnowszy raport.
    [few] Przechowywane są { $count } najnowsze raporty.
    [many] Przechowywanych jest { $count } najnowszych raportów.
   *[other] Przechowywane są najnowsze raporty ({ $count }).
}
feedback-help-improve = Pomóż ulepszyć Katna
feedback-help-improve-detail = Wyłączone, dopóki tego nie włączysz, a w każdej chwili możesz to tutaj wyłączyć.
feedback-send = Wysyłaj raporty o awariach
feedback-send-detail = Zapisany raport, dokładnie taki, jaki możesz tu wyświetlić, trafia do systemu śledzenia awarii Katna (Sentry, w UE). Bez adresu IP, wiadomości i adresów e-mail
feedback-none-saved = Nie ma zapisanych raportów o awariach.
feedback-delete-all = Usuń wszystkie
feedback-app-daemon = Usługa w tle
feedback-report-sent = { $date } · Wysłano
feedback-view = Wyświetl
feedback-view-tooltip = Otwórz raport
feedback-copy-tooltip = Skopiuj, aby wkleić do zgłoszenia błędu
feedback-copied = Skopiowano raport o awarii.
feedback-deleted-all = Usunięto raporty o awariach.
feedback-read-failed = Nie udało się odczytać raportu o awarii: { $error }
feedback-delete-failed = Nie udało się usunąć raportu o awarii: { $error }
feedback-delete-all-failed = Nie udało się usunąć raportów o awariach: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _Plik
desktop-menu-new-message = _Nowa wiadomość
desktop-menu-quit = _Zakończ
desktop-menu-edit = _Edycja
desktop-menu-undo = _Cofnij
desktop-menu-select-all = Zaznacz _wszystko
desktop-menu-select-none = _Odznacz wszystko
desktop-menu-find = _Znajdź…
desktop-menu-view = _Widok
desktop-menu-folder-list = Pokaż listę _folderów
desktop-menu-refresh = _Odśwież
desktop-menu-go = _Przejdź
desktop-menu-inbox = _Odebrane
desktop-menu-starred = Oznaczone _gwiazdką
desktop-menu-sent = _Wysłane
desktop-menu-drafts = Wersje _robocze
desktop-menu-all-mail = W_szystkie
desktop-menu-next = _Następny wątek
desktop-menu-previous = _Poprzedni wątek
desktop-menu-message = _Wiadomość
desktop-menu-open = _Otwórz
desktop-menu-reply = _Odpowiedz
desktop-menu-reply-all = Odpowiedz _wszystkim
desktop-menu-forward = _Przekaż dalej
desktop-menu-archive = _Archiwizuj
desktop-menu-delete = _Usuń
desktop-menu-spam = Zgłoś _spam
desktop-menu-move-to = P_rzenieś do…
desktop-menu-mark-read = Oznacz jako p_rzeczytane
desktop-menu-mark-unread = Oznacz jako _nieprzeczytane
desktop-menu-star = _Gwiazdka
desktop-menu-important = Oznacz jako _ważne
desktop-menu-not-important = Oznacz jako nie_ważne
desktop-menu-settings = U_stawienia
desktop-menu-quick-settings = _Szybkie ustawienia
desktop-menu-configure = _Konfiguruj Katna Mail…
desktop-menu-help = Pomo_c
desktop-menu-shortcuts = Skróty _klawiszowe
desktop-menu-whats-new = Co _nowego
desktop-menu-about = _O Katna
## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = Poruszanie się
shortcut-group-actions = Działania
shortcut-group-go-to = Przejdź do
shortcut-group-app = Aplikacja

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = Następny wątek
shortcut-previous = Poprzedni wątek
shortcut-down = W dół listy
shortcut-up = W górę listy
shortcut-first = Pierwszy na liście
shortcut-last = Ostatni na liście
shortcut-page-down = Strona w dół listy
shortcut-page-up = Strona w górę listy
shortcut-open = Otwórz wątek
shortcut-back = Wróć do listy
shortcut-scroll-down = Przewiń w dół
shortcut-scroll-up = Przewiń w górę
shortcut-scroll-page-down = Przewiń o stronę w dół
shortcut-scroll-page-up = Przewiń o stronę w górę
shortcut-compose = Utwórz
shortcut-reply = Odpowiedz
shortcut-reply-all = Odpowiedz wszystkim
shortcut-forward = Przekaż dalej
shortcut-archive = Archiwizuj
shortcut-delete = Usuń
shortcut-spam = Zgłoś spam
shortcut-move-to = Przenieś do
shortcut-mark-read = Oznacz jako przeczytane
shortcut-mark-unread = Oznacz jako nieprzeczytane
shortcut-star = Dodaj lub usuń gwiazdkę
shortcut-important = Oznacz jako ważne
shortcut-not-important = Oznacz jako nieważne
shortcut-check = Zaznacz wątek
shortcut-select-all = Zaznacz wszystkie wątki
shortcut-select-none = Odznacz wszystkie wątki
shortcut-undo = Cofnij ostatnie działanie
shortcut-go-inbox = Odebrane
shortcut-go-starred = Oznaczone gwiazdką
shortcut-go-sent = Wysłane
shortcut-go-drafts = Wersje robocze
shortcut-go-all = Wszystkie
shortcut-search = Przeszukaj pocztę
shortcut-navigation = Pokaż lub zwiń menu
shortcut-quick-settings = Szybkie ustawienia
shortcut-settings = Wszystkie ustawienia
shortcut-shortcuts = Skróty klawiszowe
shortcut-reload = Sprawdź nową pocztę
shortcut-quit = Zakończ

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first }, potem { $second }

## Settings > Accounts

accounts-folder-pane = Panel folderów
accounts-folder-pane-detail = Foldery których kont pokazuje panel po lewej.
accounts-shown-one = Jedno konto naraz; przełączaj na karcie konta
accounts-shown-all = Wszystkie konta, jedno po drugim
accounts-row = Konta
accounts-row-detail = Usunięcie konta usuwa kopię jego poczty przechowywaną przez Katna na tym komputerze. Poczta zostaje na serwerze.
accounts-none = Nie ma jeszcze kont.
accounts-kind-imported = Zaimportowane
accounts-picture-reset = Użyj zdjęcia z pulpitu
accounts-picture-change = Zmień zdjęcie
accounts-remove = Usuń
accounts-delete-all-row = Usuń wszystkie dane
accounts-delete-all-row-detail = Zacznij od nowa, jak po nowej instalacji.
accounts-delete-all-about = Usuwa z tego komputera wszystkie konta, całą zapisaną pocztę, kontakty i kalendarze, indeks wyszukiwania, Twoje ustawienia i zapisane hasła. Na Twoich serwerach poczty nic się nie zmienia.
accounts-delete-all-open = Usuń wszystkie dane Katna

## Settings > Accounts: snackbars after deleting

accounts-removed-local = Usunięto { $address } z Katna.
accounts-removed = Usunięto { $address } z Katna. Jego poczta nadal jest na serwerze.
accounts-all-deleted = Usunięto z tego komputera wszystkie dane Katna.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = Usunąć { $address }?
accounts-remove-confirm = Usuń konto
accounts-removing = Usuwanie…
accounts-remove-local-mail = { $folders ->
    [0] Cała poczta zaimportowana do tego konta
    [one] Cała poczta zaimportowana do tego konta, w jego folderze
    [few] Cała poczta zaimportowana do tego konta, w jego { $folders } folderach
    [many] Cała poczta zaimportowana do tego konta, w jego { $folders } folderach
   *[other] Cała poczta zaimportowana do tego konta, w jego { $folders } folderach
}
accounts-remove-local-settings = Jego ustawienia Katna
accounts-remove-mail = { $folders ->
    [0] Cała poczta tego konta przechowywana przez Katna
    [one] Cała poczta tego konta przechowywana przez Katna, w jego folderze
    [few] Cała poczta tego konta przechowywana przez Katna, w jego { $folders } folderach
    [many] Cała poczta tego konta przechowywana przez Katna, w jego { $folders } folderach
   *[other] Cała poczta tego konta przechowywana przez Katna, w jego { $folders } folderach
}
accounts-remove-outbox = Jego wiadomości czekające w skrzynce nadawczej
accounts-remove-settings = Jego zapisane hasło i ustawienia Katna
accounts-delete-all-title = Usunąć wszystkie dane Katna?
accounts-delete-all-confirm = Usuń wszystko
accounts-deleting = Usuwanie…
accounts-delete-all-accounts = Wszystkie konta oraz cała poczta i załączniki przechowywane przez Katna
accounts-delete-all-contacts = Kontakty, kalendarze i indeks wyszukiwania
accounts-delete-all-settings = Wszystkie ustawienia, podpisy i skróty klawiszowe
accounts-delete-all-passwords = Wszystkie zapisane hasła
accounts-deleted-heading = Zostanie usunięte z tego komputera:
accounts-cannot-undo = Tej operacji nie można cofnąć.
accounts-server-delete-all = Na Twoich serwerach poczty nic się nie zmienia: poczta tam zostaje, a ponowne dodanie konta pobierze ją znowu. Poczta zaimportowana z plików jest tylko w Katna; same pliki pozostają nietknięte.
accounts-server-local = Ta poczta została zaimportowana z plików, więc Katna ma jedyną kopię. Pliki, z których pochodzi, pozostają nietknięte; zaimportuj je ponownie, aby ją odzyskać.
accounts-server-remove = Na serwerze poczty nic się nie zmienia: poczta tam zostaje, a ponowne dodanie konta pobierze ją znowu.
accounts-confirm-word = usuń
accounts-confirm-placeholder = Wpisz „{ accounts-confirm-word }”
accounts-confirm-prompt = Aby potwierdzić, wpisz „{ accounts-confirm-word }”:
accounts-cancel = Anuluj
