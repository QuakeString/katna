# Katna Mail, Polish (Polski): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = Notatki
notes-view-reminders = Przypomnienia
notes-view-archive = Archiwum
notes-view-trash = Kosz
notes-edit-labels = Edytuj etykiety
notes-search = Szukaj w notatkach
notes-loading = Otwieranie notatek…

## Board

notes-take-a-note = Utwórz notatkę…
notes-new-list = Nowa lista
notes-new-note = Nowa notatka
notes-pinned = Przypięte
notes-others = Inne
notes-empty = Dodane notatki pojawią się tutaj
notes-archive-empty = Zarchiwizowane notatki pojawią się tutaj
notes-trash-empty = Brak notatek w koszu
notes-none-found = Brak pasujących notatek
notes-label-empty = Nie ma jeszcze notatek z tą etykietą
notes-reminders-empty = Tutaj pojawiają się notatki z nadchodzącymi przypomnieniami
notes-trash-note = Notatki w koszu są usuwane po 7 dniach.
notes-empty-trash = Opróżnij kosz
notes-ticked = { $count ->
    [one] + { $count } zaznaczony element
    [few] + { $count } zaznaczone elementy
    [many] + { $count } zaznaczonych elementów
   *[other] + { $count } zaznaczonego elementu
}
notes-select = Zaznacz notatkę
notes-selected = { $count ->
    [one] { $count } zaznaczona
    [few] { $count } zaznaczone
    [many] { $count } zaznaczonych
   *[other] { $count } zaznaczonej
}
notes-select-clear = Wyczyść zaznaczenie

## A note's buttons

notes-pin = Przypnij notatkę
notes-unpin = Odepnij notatkę
notes-archive = Archiwizuj
notes-unarchive = Przywróć z archiwum
notes-delete = Usuń notatkę
notes-restore = Przywróć
notes-delete-forever = Usuń trwale
notes-color = Kolor tła
notes-checkboxes = Pokaż lub ukryj pola wyboru
notes-labels = Etykiety
notes-close = Zamknij
notes-more = Więcej
notes-make-copy = Utwórz kopię
notes-remind = Przypomnij mi
notes-add-picture = Dodaj obraz
notes-history = Historia wersji
notes-ai = Pomóż mi pisać
notes-send-as-mail = Wyślij jako wiadomość
notes-save-markdown = Zapisz jako Markdown
notes-save-pdf = Zapisz jako PDF

## The open note

notes-title = Tytuł
notes-edited = Edytowano: { $date }
notes-on-this-computer = Na tym komputerze
notes-where = Gdzie jest przechowywana ta notatka
notes-untitled = Notatka bez tytułu

## Pictures

notes-picture-choose = Dodaj obrazy
notes-picture-remove = Usuń obraz
notes-picture-too-big = Notatka może zawierać obrazy do { $size }
notes-picture-kind = Ten plik nie jest obrazem, który Katna potrafi wyświetlić
notes-picture-unreadable = Nie udało się odczytać { $name }: { $error }

## Reminders

notes-remind-me = Przypomnij mi
notes-remind-off = Usuń przypomnienie
notes-remind-in-the-past = Wybierz godzinę, która jeszcze nie minęła
notes-remind-today = Dzisiaj, { $time }
notes-remind-tomorrow = Jutro, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = Ustawiono przypomnienie na { $when }
notes-reminder-off = Usunięto przypomnienie

## Links between notes

notes-link-note = Połącz z notatką
notes-link-new = Nowa notatka „{ $title }”
notes-linked-from = Odwołują się tutaj
notes-link-gone = Tej notatki już tu nie ma
notes-new-note-gone = Nowa notatka zniknęła.

## Version history

notes-versions = Wersje
notes-version-now = Teraz
notes-version-here = Ty, na tym komputerze
notes-version-yesterday = Wczoraj, { $time }
notes-version-changes = { $count ->
    [one] { $count } zmiana
    [few] { $count } zmiany
    [many] { $count } zmian
   *[other] { $count } zmiany
}
notes-version-from = Z urządzenia { $device }
notes-version-elsewhere = Z innego urządzenia
notes-version-created = Utworzono
notes-version-restore = Przywróć tę wersję
notes-version-restored = Przywrócono wersję
notes-history-none = Brak wcześniejszych wersji

## AI help

notes-ai-tidy = Uporządkuj tekst
notes-ai-checklist = Zamień na listę kontrolną
notes-ai-summarise = Podsumuj
notes-ai-empty = Najpierw coś napisz
notes-ai-tidied = Tekst uporządkowany. Ctrl+Z przywraca poprzedni.
notes-ai-listed = Zamieniono na listę kontrolną. Ctrl+Z przywraca poprzedni tekst.
notes-ai-summarised = Podsumowanie dodano na górze

## Labels

notes-label-note = Dodaj etykietę do notatki
notes-label-name = Wpisz nazwę etykiety
notes-label-create = Utwórz „{ $name }”
notes-label-remove = Usuń etykietę
notes-label-delete = Usuń etykietę
notes-labels-none = Nie ma jeszcze etykiet. Dodaj jedną przyciskiem etykiety w notatce.
notes-labels-done = Gotowe
notes-label-renamed = Zmieniono nazwę etykiety na „{ $name }”
notes-label-deleted = Usunięto etykietę „{ $name }”

## A note about a mail

notes-mail = Poczta
notes-open-mail = Otwórz wiadomość
notes-open-note = Otwórz notatkę

## Meeting notes

notes-meeting-take = Notuj ze spotkania
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = Uczestnicy: { $names }
notes-meeting-notes = Notatki
notes-meeting-actions = Zadania do wykonania
notes-event = Wydarzenie
notes-open-event = Otwórz wydarzenie

## Formatting

notes-format = Formatowanie
notes-format-heading-1 = Nagłówek 1
notes-format-heading-2 = Nagłówek 2
notes-format-normal = Zwykły tekst
notes-format-bold = Pogrubienie
notes-format-italic = Kursywa
notes-format-underline = Podkreślenie
notes-format-quote = Cytat
notes-format-code = Kod
notes-format-divider = Separator
notes-format-clear = Wyczyść formatowanie

## Tasks

notes-make-task = Zamień w zadanie

## Colors (tooltips)

notes-color-none = Bez koloru
notes-color-coral = Koralowy
notes-color-peach = Brzoskwiniowy
notes-color-sand = Piaskowy
notes-color-mint = Miętowy
notes-color-sage = Szałwiowy
notes-color-fog = Mgła
notes-color-storm = Burza
notes-color-dusk = Zmierzch
notes-color-blossom = Kwiat
notes-color-clay = Glina
notes-color-chalk = Kreda

## Messages at the foot of the window

notes-archived = Notatka została zarchiwizowana
notes-unarchived = Notatka została przywrócona z archiwum
notes-trashed = Notatka została przeniesiona do kosza
notes-restored = Notatka została przywrócona
notes-saved = Notatka zapisana
notes-pinned-count = { $count ->
    [one] Przypięto notatkę
    [few] Przypięto { $count } notatki
    [many] Przypięto { $count } notatek
   *[other] Przypięto { $count } notatki
}
notes-unpinned-count = { $count ->
    [one] Odpięto notatkę
    [few] Odpięto { $count } notatki
    [many] Odpięto { $count } notatek
   *[other] Odpięto { $count } notatki
}
notes-colored-count = { $count ->
    [one] Zmieniono kolor
    [few] Zmieniono kolor { $count } notatek
    [many] Zmieniono kolor { $count } notatek
   *[other] Zmieniono kolor { $count } notatki
}
notes-archived-count = { $count ->
    [one] Zarchiwizowano notatkę
    [few] Zarchiwizowano { $count } notatki
    [many] Zarchiwizowano { $count } notatek
   *[other] Zarchiwizowano { $count } notatki
}
notes-unarchived-count = { $count ->
    [one] Przywrócono notatkę z archiwum
    [few] Przywrócono { $count } notatki z archiwum
    [many] Przywrócono { $count } notatek z archiwum
   *[other] Przywrócono { $count } notatki z archiwum
}
notes-trashed-count = { $count ->
    [one] Przeniesiono notatkę do Kosza
    [few] Przeniesiono { $count } notatki do Kosza
    [many] Przeniesiono { $count } notatek do Kosza
   *[other] Przeniesiono { $count } notatki do Kosza
}
notes-restored-count = { $count ->
    [one] Przywrócono notatkę
    [few] Przywrócono { $count } notatki
    [many] Przywrócono { $count } notatek
   *[other] Przywrócono { $count } notatki
}
notes-copied-count = { $count ->
    [one] Utworzono kopię
    [few] Utworzono { $count } kopie
    [many] Utworzono { $count } kopii
   *[other] Utworzono { $count } kopii
}
notes-empty-discarded = Pusta notatka została odrzucona
notes-mail-gone = Tej wiadomości już tu nie ma
notes-deleted-forever = { $count ->
    [one] Notatka została usunięta trwale
    [few] { $count } notatki zostały usunięte trwale
    [many] { $count } notatek zostało usuniętych trwale
   *[other] { $count } notatki została usunięta trwale
}
