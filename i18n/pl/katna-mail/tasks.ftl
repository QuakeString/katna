# Katna Mail, Polish (Polski): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Nowe zadanie
tasks-all = Wszystkie zadania
tasks-today = Dziś
tasks-upcoming = Nadchodzące
tasks-starred = Oznaczone gwiazdką
tasks-completed-view = Ukończone
tasks-new-list = Utwórz nową listę
tasks-labels-heading = Etykiety
tasks-on-this-computer = Na tym komputerze
tasks-my-tasks = Moje zadania
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = Zaloguj się ponownie, aby wyświetlić zadania
tasks-account-signed-in = Ponownie zalogowano do { $address }. Pobieranie zadań…
tasks-account-sign-in-refused = { $provider } nie wpuścił aplikacji Katna. Spróbuj ponownie i zezwól na dostęp do zadań.
tasks-account-refused = Serwer nie przyjął hasła. Yahoo, iCloud, Zoho i inne wymagają hasła do aplikacji.
tasks-account-change-password = Zmień hasło
tasks-account-change-password-tooltip = Wpisz nowe hasło; Katna sprawdzi je na serwerze
tasks-account-not-enabled = Dostęp do zadań dla aplikacji Katna nie jest jeszcze włączony.
tasks-account-failed = Nie udało się odczytać list zadań.
# $reason is the server's own words, in English.
tasks-account-error = Nie udało się odczytać list zadań: { $reason }
tasks-account-none = Nie znaleziono list zadań
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = Nie znaleziono list zadań: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } pokazuje zadania tylko aplikacji Katna zalogowanej przez { $provider }.
tasks-account-sign-in-with = Zaloguj się przez { $provider }
tasks-account-looking = Szukanie list zadań…
tasks-account-try-again = Spróbuj ponownie
tasks-account-try-again-tooltip = Sprawdź teraz ponownie zadania tego konta
tasks-account-fixing = Trwa naprawianie…
tasks-list-name-placeholder = Nazwa listy

## Lists and tasks

tasks-loading = Wczytywanie zadań…
tasks-no-lists = Tutaj pojawią się Twoje listy zadań.
tasks-search = Szukaj zadań
tasks-search-none = Brak zadań pasujących do wyszukiwania.
tasks-add = Dodaj zadanie
tasks-title-placeholder = Tytuł
tasks-add-step = Dodaj podzadanie
tasks-empty = Nie ma jeszcze zadań. Dodaj jedno powyżej.
tasks-starred-empty = Oznacz zadanie gwiazdką, aby zobaczyć je tutaj.
tasks-label-empty = Brak otwartych zadań z tą etykietą.
tasks-today-empty = Nic do zrobienia na dziś.
tasks-completed-empty = Tutaj pojawią się ukończone zadania.
tasks-upcoming-add = Dodaj zadanie na { $day }
tasks-upcoming-overdue-day = { $weekday } { $day }
tasks-from-mail-quiet = Z poczty
tasks-from-note-quiet = Z notatki
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday }, { $day }
tasks-overdue = Zaległe
tasks-completed = { $count ->
    [one] Ukończone ({ $count })
    [few] Ukończone ({ $count })
    [many] Ukończone ({ $count })
   *[other] Ukończone ({ $count })
}
tasks-list-options = Opcje listy
tasks-sort-by = Sortuj według
tasks-sort-my-order = Moja kolejność
tasks-sort-date = Data
tasks-sort-starred = Ostatnio oznaczone gwiazdką
tasks-sort-title = Tytuł
tasks-rename-list = Zmień nazwę listy
tasks-delete-list = Usuń listę
tasks-mark-done = Oznacz jako ukończone
tasks-mark-open = Oznacz jako nieukończone
tasks-star = Oznacz gwiazdką
tasks-unstar = Usuń gwiazdkę
tasks-edit-title = Edytuj tytuł
tasks-details = Szczegóły
tasks-delete = Usuń
tasks-move-to = Przenieś do: { $list }
tasks-from-mail = Poczta
tasks-open-mail = Otwórz wiadomość
tasks-from-note = Notatka
tasks-open-note = Otwórz notatkę
tasks-note-gone = Tej notatki już tu nie ma.
tasks-no-subject = (bez tematu)

## Several tasks selected (Ctrl+click, Shift+click)

tasks-selected = { $count ->
    [one] Zaznaczono: { $count }
    [few] Zaznaczono: { $count }
    [many] Zaznaczono: { $count }
   *[other] Zaznaczono: { $count }
}
tasks-select-clear = Wyczyść zaznaczenie
tasks-select-move = Przenieś na listę
tasks-select-date = Ustaw datę
tasks-next-week = W przyszłym tygodniu

## The details dialog

tasks-notes-placeholder = Dodaj szczegóły
tasks-date = Data
tasks-no-date = Brak daty
tasks-time-placeholder = Dodaj godzinę
tasks-repeat = Powtarzaj
tasks-repeat-never = Nie powtarza się
tasks-repeat-daily = Codziennie
tasks-repeat-weekly = Co tydzień
tasks-repeat-monthly = Co miesiąc
tasks-repeat-yearly = Co rok
tasks-repeat-other = Niestandardowe
tasks-remind = Przypomnij
tasks-remind-off = Nie przypominaj
tasks-remind-on-time = W momencie zadania
tasks-remind-morning = W dniu zadania, { $time }
tasks-remind-hour-before = Godzinę wcześniej
tasks-remind-day-before = Dzień wcześniej
tasks-label-add = Dodaj etykietę
tasks-label-task = Etykieta zadania
tasks-files-attach = Załącz pliki
tasks-files-pick = Załącz
tasks-file-open = Otwórz
tasks-file-remove = Usuń plik
tasks-file-here = Tylko na tym komputerze
tasks-cancel = Anuluj
tasks-save = Zapisz
tasks-not-a-time = „{ $text }” to nie godzina, na przykład { $example }.

## Due days

tasks-due-today = Dziś
tasks-due-tomorrow = Jutro
tasks-due-yesterday = Wczoraj
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = Zadanie ukończone
tasks-toast-next = Gotowe. Następne: { $date }
tasks-toast-deleted = Zadanie usunięte
tasks-files-added = { $count ->
    [one] Załączono plik
    [few] Załączono { $count } pliki
    [many] Załączono { $count } plików
   *[other] Załączono { $count } pliku
}
tasks-file-removed = Usunięto „{ $name }”
tasks-files-left-out = Nie załączono: { $names }. Zadanie przyjmuje pliki do { $limit }, bez folderów.
tasks-file-missing = Tego pliku już tu nie ma.
tasks-toast-added = { $count ->
    [one] Dodano do Zadań
    [few] Dodano zadania: { $count }
    [many] Dodano zadania: { $count }
   *[other] Dodano zadania: { $count }
}
tasks-mail-gone = Tej wiadomości już tu nie ma.
tasks-toast-list-deleted = Lista usunięta
tasks-toast-moved = Przeniesiono do: { $list }
# A task dragged to another place in its own list.
tasks-toast-placed = Przeniesiono zadanie
tasks-toast-rescheduled = Zmieniono termin zadania
tasks-toast-rescheduled-several = { $count ->
    [one] Zmieniono termin zadania
    [few] Zmieniono termin { $count } zadań
    [many] Zmieniono termin { $count } zadań
   *[other] Zmieniono termin { $count } zadania
}
tasks-toast-done-several = { $count ->
    [one] Ukończono zadanie
    [few] Ukończono { $count } zadania
    [many] Ukończono { $count } zadań
   *[other] Ukończono { $count } zadania
}
tasks-toast-open-several = { $count ->
    [one] Oznaczono zadanie jako nieukończone
    [few] Oznaczono { $count } zadania jako nieukończone
    [many] Oznaczono { $count } zadań jako nieukończone
   *[other] Oznaczono { $count } zadania jako nieukończone
}
tasks-toast-starred = { $count ->
    [one] Oznaczono zadanie gwiazdką
    [few] Oznaczono gwiazdką { $count } zadania
    [many] Oznaczono gwiazdką { $count } zadań
   *[other] Oznaczono gwiazdką { $count } zadania
}
tasks-toast-unstarred = { $count ->
    [one] Usunięto gwiazdkę
    [few] Usunięto gwiazdki z { $count } zadań
    [many] Usunięto gwiazdki z { $count } zadań
   *[other] Usunięto gwiazdki z { $count } zadania
}
tasks-toast-deleted-several = { $count ->
    [one] Usunięto zadanie
    [few] Usunięto { $count } zadania
    [many] Usunięto { $count } zadań
   *[other] Usunięto { $count } zadania
}
