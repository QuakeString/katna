# Katna Mail, Polish (Polski): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Utwórz
tasks-all = Wszystkie zadania
tasks-today = Dziś
tasks-starred = Oznaczone gwiazdką
tasks-new-list = Utwórz nową listę
tasks-on-this-computer = Na tym komputerze
tasks-my-tasks = Moje zadania
tasks-list-name-placeholder = Nazwa listy

## Lists and tasks

tasks-loading = Wczytywanie zadań…
tasks-no-lists = Tutaj pojawią się Twoje listy zadań.
tasks-add = Dodaj zadanie
tasks-title-placeholder = Tytuł
tasks-add-step = Dodaj podzadanie
tasks-empty = Nie ma jeszcze zadań. Dodaj jedno powyżej.
tasks-starred-empty = Oznacz zadanie gwiazdką, aby zobaczyć je tutaj.
tasks-today-empty = Nic do zrobienia na dziś.
tasks-today-date = { $weekday }, { $day }
tasks-overdue = Zaległe
tasks-completed = { $count ->
    [one] Ukończone ({ $count })
    [few] Ukończone ({ $count })
    [many] Ukończone ({ $count })
   *[other] Ukończone ({ $count })
}
tasks-list-options = Opcje listy
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
tasks-toast-added = { $count ->
    [one] Dodano do Zadań
    [few] Dodano zadania: { $count }
    [many] Dodano zadania: { $count }
   *[other] Dodano zadania: { $count }
}
tasks-mail-gone = Tej wiadomości już tu nie ma.
tasks-toast-list-deleted = Lista usunięta
tasks-toast-moved = Przeniesiono do: { $list }
tasks-toast-rescheduled = Zmieniono termin zadania
