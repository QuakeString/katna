# Katna Mail, Polish (Polski): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = Dzisiaj
calendar-today-tip = Przejdź do dzisiaj
calendar-view-day = Dzień
calendar-view-week = Tydzień
calendar-view-month = Miesiąc
calendar-view-year = Rok
calendar-view-schedule = Harmonogram
calendar-view-days =
    { $count ->
        [one] { $count } dzień
        [few] { $count } dni
        [many] { $count } dni
       *[other] { $count } dnia
    }
calendar-options = Opcje
calendar-density = Gęstość
calendar-density-responsive = Dopasowana do ekranu
calendar-density-comfortable = Wygodna
calendar-density-compact = Kompaktowa
calendar-custom-days = Widok niestandardowy
calendar-second-zone = Druga strefa czasowa
calendar-zone-none = Brak
calendar-zone = { $zone } ({ $offset })
calendar-share-free = Udostępnij wolne terminy
calendar-free-subject = Kiedy jestem wolny
calendar-free-intro = Oto kilka terminów, kiedy jestem wolny ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = W najbliższych dniach roboczych nie mam wolnego czasu.
calendar-previous-day = Poprzedni dzień
calendar-next-day = Następny dzień
calendar-previous-week = Poprzedni tydzień
calendar-next-week = Następny tydzień
calendar-previous-month = Poprzedni miesiąc
calendar-next-month = Następny miesiąc
calendar-previous-year = Poprzedni rok
calendar-next-year = Następny rok
calendar-previous-period = Wcześniej
calendar-next-period = Później
calendar-title-months = { $first } – { $last }
calendar-loading = Wczytywanie…
calendar-read-failed = Nie udało się odczytać kalendarza: { $error }
calendar-sets = Zestawy kalendarzy
calendar-set-add = Zapisz widoczne kalendarze jako zestaw
calendar-set-name = Nazwa zestawu
calendar-set-remove = Usuń zestaw
calendar-local = Ten komputer
calendar-account-gone = Usunięte konto
calendar-account-sign-in = Zaloguj się ponownie, aby wyświetlić kalendarze
calendar-account-signed-in = Ponownie zalogowano do { $address }. Pobieranie kalendarzy…
calendar-account-sign-in-refused = { $provider } nie wpuścił aplikacji Katna. Spróbuj ponownie i zezwól na dostęp do kalendarzy.
calendar-account-refused = Serwer nie przyjął hasła. Yahoo, iCloud, Zoho i inne wymagają hasła do aplikacji.
calendar-account-change-password = Zmień hasło
calendar-account-change-password-tooltip = Otwórz Ustawienia > Konta
calendar-account-not-enabled = Dostęp do kalendarzy dla aplikacji Katna nie jest jeszcze włączony.
calendar-account-failed = Nie udało się odczytać kalendarzy.
calendar-account-error = Nie udało się odczytać kalendarzy: { $reason }
calendar-account-none = Nie znaleziono kalendarzy
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
calendar-account-none-why = Nie znaleziono kalendarzy: { $reason }
# A Gmail or Outlook account added with a password: its calendars need the
# provider's sign-in.
calendar-account-use-sign-in = { $provider } pokazuje kalendarze tylko aplikacji Katna zalogowanej przez { $provider }.
calendar-account-sign-in-with = Zaloguj się przez { $provider }
calendar-account-looking = Szukanie kalendarzy…
calendar-account-try-again = Spróbuj ponownie
calendar-account-try-again-tooltip = Sprawdź teraz ponownie kalendarze tego konta
calendar-account-fixing = Trwa naprawianie…
calendar-birthdays = Urodziny
calendar-birthday-of = Urodziny: { $name }
calendar-empty-title = Brak kalendarzy
calendar-empty-text = Katna pokazuje tu kalendarze Twoich kont Google i Microsoft po ich zsynchronizowaniu oraz kalendarze innych serwerów obsługujących CalDAV.
calendar-schedule-empty = Nic nie zaplanowano na najbliższe dwa miesiące.
calendar-search = Szukaj wydarzeń
calendar-search-past = Minione wydarzenia
calendar-search-none = Brak wydarzeń pasujących do wyszukiwania.
calendar-no-title = (Bez tytułu)
calendar-all-day = Cały dzień
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = jeszcze { $count }
calendar-peek-day = { $weekday }, { $day }
calendar-repeats = Powtarza się
calendar-join = Dołącz
calendar-join-with = Dołącz przez { $service }
calendar-email-guests = Wyślij e-mail do gości
calendar-running-late = Spóźniam się
calendar-late-subject = Spóźnienie: { $title }
calendar-late-body = Przepraszam, spóźnię się o kilka minut na { $title }. Będę wkrótce.
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
calendar-open-mail = Otwórz wiadomość
calendar-open-contact = Otwórz kontakt
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
# Right-click menus on the calendar: on a free time or day, an event and
# a task.
calendar-menu-new-event = Nowe wydarzenie
# Shows the day right-clicked on its own, in the Day view.
calendar-menu-open-day = Otwórz dzień
calendar-menu-duplicate = Duplikuj
calendar-menu-color = Kolor
# The event takes its calendar's color.
calendar-menu-color-calendar = Kolor kalendarza
# A task's new due day, a week from today.
calendar-menu-in-a-week = Za tydzień
# Event colors, by the names Google Calendar gives them.
calendar-color-tomato = Pomidor
calendar-color-flamingo = Flaming
calendar-color-tangerine = Mandarynka
calendar-color-banana = Banan
calendar-color-sage = Szałwia
calendar-color-basil = Bazylia
calendar-color-peacock = Paw
calendar-color-blueberry = Borówka
calendar-color-lavender = Lawenda
calendar-color-grape = Winogrono
calendar-color-graphite = Grafit
calendar-menu-only-this = Pokaż tylko ten
calendar-menu-rename = Zmień nazwę
calendar-menu-remove = Usuń z listy
calendar-menu-delete = Usuń
calendar-menu-new-calendar = Nowy kalendarz
calendar-menu-show-all = Pokaż wszystkie
calendar-menu-hide-all = Ukryj wszystkie
calendar-menu-account-settings = Ustawienia konta
calendar-why-main = Główny kalendarz
calendar-why-last = Jedyny tutaj
calendar-why-owner = Tylko właściciel
calendar-why-contacts = Z Kontaktów
calendar-why-unreached = Brak połączenia
calendar-name-placeholder = Nazwa kalendarza
calendar-toast-added = Dodano „{ $name }”
calendar-toast-renamed = Zmieniono nazwę kalendarza
calendar-toast-recolored = Zmieniono kolor kalendarza
calendar-toast-deleted = Usunięto „{ $name }”
calendar-toast-removed = Usunięto „{ $name }” z Twojej listy
calendar-edit-failed = Kalendarz nie został zmieniony: { $reason }
calendar-delete-title = Usunąć „{ $name }”?
calendar-delete-confirm = Usuń
calendar-deleting = Usuwanie…
calendar-delete-heading = Zostanie usunięte:
calendar-delete-events = Kalendarz i wszystkie jego wydarzenia
calendar-delete-shared = Dla wszystkich, którym go udostępniono
calendar-delete-server = Zostanie usunięty z { $account } w usłudze pocztowej, nie tylko w Katna.
calendar-delete-local = Zostanie usunięty z tego komputera.
calendar-remove-title = Usunąć „{ $name }” z Twojej listy?
calendar-remove-confirm = Usuń
calendar-removing = Usuwanie…
calendar-remove-heading = Co się zmieni:
calendar-remove-events = Przestaniesz widzieć jego wydarzenia, tutaj i w innych aplikacjach
calendar-remove-server = Kalendarz zostaje u właściciela, który może udostępnić Ci go ponownie.
calendar-kind-event = Wydarzenie
calendar-kind-task = Zadanie
calendar-kind-focus = Czas na skupienie
calendar-kind-out-of-office = Poza biurem
calendar-kind-working-location = Miejsce pracy
calendar-task-added = Dodano zadanie
calendar-task-added-to = Dodano zadanie do: { $list }
calendar-task-list-local = Na tym komputerze
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
calendar-invite-by-mail = Nie ma w Twoim kalendarzu: Twoja odpowiedź trafi do organizatora e-mailem.
calendar-mail-yes = Zaakceptowano: { $title }
calendar-mail-yes-body = { $name }: zaproszenie zostało zaakceptowane.
calendar-mail-no = Odrzucono: { $title }
calendar-mail-no-body = { $name }: zaproszenie zostało odrzucone.
calendar-mail-maybe = Wstępnie zaakceptowano: { $title }
calendar-mail-maybe-body = { $name }: zaproszenie zostało wstępnie zaakceptowane.
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
