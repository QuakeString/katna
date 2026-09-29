# Katna Mail, Polish (Polski): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = Notatki
notes-view-archive = Archiwum
notes-view-trash = Kosz
notes-search = Szukaj w notatkach
notes-loading = Otwieranie notatek…

## Board

notes-take-a-note = Utwórz notatkę…
notes-new-list = Nowa lista
notes-pinned = Przypięte
notes-others = Inne
notes-empty = Dodane notatki pojawią się tutaj
notes-archive-empty = Zarchiwizowane notatki pojawią się tutaj
notes-trash-empty = Brak notatek w koszu
notes-none-found = Brak pasujących notatek
notes-trash-note = Notatki w koszu są usuwane po 7 dniach.
notes-empty-trash = Opróżnij kosz
notes-ticked = { $count ->
    [one] + { $count } zaznaczony element
    [few] + { $count } zaznaczone elementy
    [many] + { $count } zaznaczonych elementów
   *[other] + { $count } zaznaczonego elementu
}

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
notes-close = Zamknij

## The open note

notes-title = Tytuł
notes-edited = Edytowano: { $date }
notes-on-this-computer = Na tym komputerze
notes-where = Gdzie jest przechowywana ta notatka

## A note about a mail

notes-mail = Poczta
notes-open-mail = Otwórz wiadomość
notes-open-note = Otwórz notatkę

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
notes-empty-discarded = Pusta notatka została odrzucona
notes-mail-gone = Tej wiadomości już tu nie ma
notes-deleted-forever = { $count ->
    [one] Notatka została usunięta trwale
    [few] { $count } notatki zostały usunięte trwale
    [many] { $count } notatek zostało usuniętych trwale
   *[other] { $count } notatki została usunięta trwale
}
