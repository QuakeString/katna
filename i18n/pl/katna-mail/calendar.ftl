# Katna Mail, Polish (Polski): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = Dzisiaj
calendar-today-tip = Przejdź do dzisiaj
calendar-view-day = Dzień
calendar-view-week = Tydzień
calendar-view-month = Miesiąc
calendar-view-schedule = Harmonogram
calendar-previous-day = Poprzedni dzień
calendar-next-day = Następny dzień
calendar-previous-week = Poprzedni tydzień
calendar-next-week = Następny tydzień
calendar-previous-month = Poprzedni miesiąc
calendar-next-month = Następny miesiąc
calendar-previous-period = Wcześniej
calendar-next-period = Później
calendar-title-months = { $first } – { $last }
calendar-loading = Wczytywanie…
calendar-read-failed = Nie udało się odczytać kalendarza: { $error }
calendar-local = Ten komputer
calendar-account-gone = Usunięte konto
calendar-empty-title = Brak kalendarzy
calendar-empty-text = Katna pokazuje tu kalendarze Twoich kont Google i Microsoft po ich zsynchronizowaniu oraz kalendarze innych serwerów obsługujących CalDAV.
calendar-schedule-empty = Nic nie zaplanowano na najbliższe dwa miesiące.
calendar-no-title = (Bez tytułu)
calendar-all-day = Cały dzień
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = jeszcze { $count }
calendar-repeats = Powtarza się
calendar-join = Dołącz
calendar-guests =
    { $count ->
        [one] { $count } gość
        [few] { $count } gości
        [many] { $count } gości
       *[other] { $count } gościa
    }
calendar-guest-answers = { $yes } tak, { $maybe } może, { $no } nie, { $waiting } oczekuje
calendar-organizer = Organizator
calendar-optional = Opcjonalny
calendar-open-web = Otwórz w przeglądarce
calendar-close = Zamknij

## Adding, changing and deleting events.

calendar-add-title = Dodaj tytuł
calendar-add-location = Dodaj lokalizację
calendar-add-notes = Dodaj opis
calendar-add-guests = Dodaj gości
calendar-remove-guest = Usuń
calendar-add-meet = Dodaj wideokonferencję Google Meet
calendar-add-teams = Dodaj spotkanie w Teams
calendar-has-call = Dodano połączenie wideo
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = Cały dzień
calendar-more-options = Więcej opcji
calendar-save = Zapisz
calendar-saved = Zapisano wydarzenie
calendar-deleted = Usunięto wydarzenie
calendar-discard = Odrzuć zmiany
calendar-edit = Edytuj wydarzenie
calendar-delete = Usuń wydarzenie
calendar-event-details = Szczegóły wydarzenia
calendar-kind-event = Wydarzenie
calendar-kind-focus = Czas na skupienie
calendar-kind-out-of-office = Poza biurem
calendar-kind-working-location = Miejsce pracy
calendar-working-home = Dom
calendar-busy = Zajęty
calendar-free = Dostępny
calendar-cancel = Anuluj
calendar-ok = OK
calendar-read-only = Nie możesz zmieniać wydarzeń w tym kalendarzu
calendar-none-editable = Nie ma jeszcze kalendarza, do którego możesz dodawać wydarzenia
calendar-no-such-time = Ta godzina nie istnieje w Twojej strefie czasowej
calendar-end-before-start = Wydarzenie kończy się, zanim się zacznie
calendar-repeat-never = Nie powtarza się
calendar-repeat-daily = Codziennie
calendar-repeat-weekly = Co tydzień: { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] Co miesiąc: { $weekday }, pierwszy tydzień
        [2] Co miesiąc: { $weekday }, drugi tydzień
        [3] Co miesiąc: { $weekday }, trzeci tydzień
        [4] Co miesiąc: { $weekday }, czwarty tydzień
       *[other] Co miesiąc: { $weekday }, ostatni tydzień
    }
calendar-repeat-yearly = Co roku: { $day }
calendar-repeat-weekdays = Każdego dnia roboczego (od poniedziałku do piątku)
calendar-repeat-custom = Niestandardowe
calendar-reminder-none = Brak powiadomienia
calendar-reminder-at-start = Na początku
calendar-reminder-minutes =
    { $count ->
        [one] { $count } minuta przed
        [few] { $count } minuty przed
        [many] { $count } minut przed
       *[other] { $count } minuty przed
    }
calendar-reminder-hours =
    { $count ->
        [one] { $count } godzina przed
        [few] { $count } godziny przed
        [many] { $count } godzin przed
       *[other] { $count } godziny przed
    }
calendar-reminder-days =
    { $count ->
        [one] { $count } dzień przed
        [few] { $count } dni przed
        [many] { $count } dni przed
       *[other] { $count } dnia przed
    }
calendar-scope-edit-title = Edytuj wydarzenie cykliczne
calendar-scope-delete-title = Usuń wydarzenie cykliczne
calendar-scope-this = To wydarzenie
calendar-scope-following = To i kolejne wydarzenia
calendar-scope-all = Wszystkie wydarzenia
calendar-scope-respond-title = Odpowiedź dla wydarzenia cyklicznego
calendar-going = Bierzesz udział?
calendar-answer-yes = Tak
calendar-answer-no = Nie
calendar-answer-maybe = Może
calendar-answered-yes = Bierzesz udział
calendar-answered-no = Nie bierzesz udziału
calendar-answered-maybe = Możesz wziąć udział

## The card at the top of a mail with an invitation.

calendar-invite = Zaproszenie
calendar-invite-cancelled = Wydarzenie odwołane
calendar-invite-reply = { $name }: nowa odpowiedź
calendar-invite-reply-yes = { $name }: zaproszenie przyjęte
calendar-invite-reply-no = { $name }: zaproszenie odrzucone
calendar-invite-reply-maybe = { $name } może wziąć udział
calendar-invite-organizer = Organizator: { $name }
calendar-invite-open = Otwórz w Kalendarzu
calendar-invite-not-yet = Jeszcze nie ma w Twoim kalendarzu. Odpowiedź będzie możliwa po synchronizacji.
calendar-invite-your-day = Twój dzień
calendar-invite-clashes =
    { $count ->
        [one] Koliduje z { $count } wydarzeniem
        [few] Koliduje z { $count } wydarzeniami
        [many] Koliduje z { $count } wydarzeniami
       *[other] Koliduje z { $count } wydarzenia
    }

## The day's agenda beside the mail.

agenda-show = Pokaż plan dnia
agenda-hide = Ukryj plan dnia
agenda-today = Dzisiaj, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = Nic nie zaplanowano na ten dzień.
