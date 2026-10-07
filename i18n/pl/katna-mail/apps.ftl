# Katna Mail, Polish (Polski).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

top-brand = Katna

## App rail (and the bottom bar on a phone)

rail-mail = Poczta
rail-calendar = Kalendarz
rail-contacts = Kontakty
rail-tasks = Zadania
rail-notes = Notatki
rail-files = Pliki

## Rail right-click menu

rail-menu-open = Otwórz { $app }
rail-menu-settings = Ustawienia: { $app }
rail-menu-turn-off = Wyłącz { $app }…

## Turning an app off (Settings > Apps)

app-off-title = Wyłączyć { $app }?
app-off-body = Katna przestanie synchronizować { $app } i usunie tę aplikację z:
app-off-keep = Zachowaj kopię na tym komputerze
app-off-keep-detail = Ponowne włączenie działa od razu
app-off-remove = Usuń kopię z tego komputera
app-off-remove-detail = Na Twoich kontach nic się nie zmienia, a ponowne włączenie pobierze wszystko jeszcze raz. To, co jest tylko na tym komputerze lub nie zostało jeszcze wysłane, zostaje.
app-off-cancel = Anuluj
app-off-confirm = Wyłącz
app-off-done = Wyłączono { $app }
app-off-note = { $app } jest wyłączone
app-off-turn-on = Włącz
app-off-leaves-calendar-rail = Pasek aplikacji i Ctrl+2
app-off-leaves-calendar-agenda = Plan dnia obok Twojej poczty
app-off-leaves-calendar-meeting = Zaplanuj spotkanie oraz Otwórz w Kalendarzu przy zaproszeniach
app-off-leaves-calendar-reminders = Przypomnienia o wydarzeniach
app-off-leaves-calendar-desktop = Wydarzenia w KRunner i w zegarze pulpitu
app-off-leaves-contacts-rail = Pasek aplikacji i Ctrl+3
app-off-leaves-contacts-card = Dodaj do kontaktów na karcie nadawcy
app-off-leaves-contacts-birthdays = Urodziny w Kalendarzu
app-off-leaves-tasks-rail = Pasek aplikacji i Ctrl+4
app-off-leaves-tasks-mail = Dodaj do Zadań przy poczcie oraz Shift+T
app-off-leaves-tasks-calendar = Zadania w Kalendarzu
app-off-leaves-tasks-tray = Nowe zadanie w zasobniku oraz Meta+Alt+T
app-off-leaves-tasks-reminders = Przypomnienia o zadaniach
app-off-leaves-notes-rail = Pasek aplikacji i Ctrl+5
app-off-leaves-notes-mail = Dodaj notatkę do poczty
app-off-leaves-notes-meetings = Notatki ze spotkań przy wydarzeniach
app-off-leaves-notes-tray = Nowa notatka w zasobniku oraz Meta+Alt+N
app-off-leaves-notes-reminders = Przypomnienia o notatkach
app-off-leaves-files-rail = Pasek aplikacji i Ctrl+7
app-off-leaves-files-compose = Pliki przy dołączaniu w oknie tworzenia

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Wkrótce
app-calendar-promise = Twoje kalendarze CalDAV, zaproszenia na spotkania z poczty i przypomnienia – obok skrzynki odbiorczej.
app-tasks-promise = Listy zadań synchronizowane przez CalDAV i zadania tworzone z wiadomości.
app-notes-promise = Szybkie notatki oraz notatki do wiadomości lub wątku na później.

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
