# Katna Mail, Afrikaans (Afrikaans): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = Vandag
calendar-today-tip = Gaan na vandag
calendar-view-day = Dag
calendar-view-week = Week
calendar-view-month = Maand
calendar-view-year = Jaar
calendar-view-schedule = Skedule
calendar-view-days =
    { $count ->
        [one] { $count } dag
       *[other] { $count } dae
    }
calendar-options = Opsies
calendar-density = Digtheid
calendar-density-responsive = Reageer op jou skerm
calendar-density-comfortable = Gemaklik
calendar-density-compact = Kompak
calendar-custom-days = Pasgemaakte aansig
calendar-second-zone = Tweede tydsone
calendar-zone-none = Geen
calendar-zone = { $zone } ({ $offset })
calendar-share-free = Deel vrye tye
calendar-free-subject = Tye wanneer ek vry is
calendar-free-intro = Hier is 'n paar tye wanneer ek vry is ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = Ek het nie vrye tyd in die volgende paar werksdae nie.
calendar-previous-day = Vorige dag
calendar-next-day = Volgende dag
calendar-previous-week = Vorige week
calendar-next-week = Volgende week
calendar-previous-month = Vorige maand
calendar-next-month = Volgende maand
calendar-previous-year = Vorige jaar
calendar-next-year = Volgende jaar
calendar-previous-period = Vroeër
calendar-next-period = Later
calendar-title-months = { $first } – { $last }
calendar-loading = Laai tans…
calendar-read-failed = Die kalender kon nie gelees word nie: { $error }
calendar-sets = Kalenderstelle
calendar-set-add = Stoor die kalenders wat gewys word as ’n stel
calendar-set-name = Naam van die stel
calendar-set-remove = Verwyder stel
calendar-local = Hierdie rekenaar
calendar-account-gone = Verwyderde rekening
calendar-account-sign-in = Meld weer aan om kalenders te wys
calendar-account-signed-in = Weer aangemeld by { $address }. Haal tans jou kalenders…
calendar-account-sign-in-refused = { $provider } het Katna nie ingelaat nie. Probeer weer, en gee toegang tot jou kalenders.
calendar-account-refused = Die bediener het die wagwoord nie aanvaar nie. Yahoo, iCloud, Zoho en ander het 'n programwagwoord nodig.
calendar-account-change-password = Verander wagwoord
calendar-account-change-password-tooltip = Tik die nuwe wagwoord; Katna gaan dit by die bediener na
calendar-account-not-enabled = Kalendertoegang vir Katna is nog nie aangeskakel nie.
calendar-account-failed = Die kalenders kon nie gelees word nie.
calendar-account-error = Die kalenders kon nie gelees word nie: { $reason }
calendar-account-none = Geen kalenders gevind nie
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
calendar-account-none-why = Geen kalenders gevind nie: { $reason }
# A Gmail or Outlook account added with a password: its calendars need the
# provider's sign-in.
calendar-account-use-sign-in = { $provider } wys kalenders net aan Katna as dit met { $provider } aangemeld is.
calendar-account-sign-in-with = Meld aan met { $provider }
calendar-account-looking = Soek tans kalenders…
calendar-account-try-again = Probeer weer
calendar-account-try-again-tooltip = Kyk nou weer na hierdie rekening se kalenders
calendar-account-fixing = Besig daarmee…
calendar-birthdays = Verjaardae
calendar-tasks = Take
calendar-birthday-of = { $name } se verjaardag
calendar-empty-title = Nog geen kalenders nie
calendar-empty-text = Katna wys hier die kalenders van jou Google- en Microsoft-rekeninge sodra hulle gesinchroniseer is, en dié van ander bedieners wat CalDAV aanbied.
calendar-schedule-empty = Niks beplan vir die volgende twee maande nie.
calendar-search = Soek geleenthede
calendar-search-past = Vorige geleenthede
calendar-search-none = Geen geleenthede pas by jou soektog nie.
calendar-no-title = (Geen titel)
calendar-all-day = Heeldag
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count } meer
calendar-peek-day = { $weekday }, { $day }
calendar-repeats = Herhaal
calendar-join = Sluit aan
calendar-join-with = Sluit aan met { $service }
calendar-email-guests = Stuur e-pos aan gaste
calendar-running-late = Ek is laat
calendar-late-subject = Laat: { $title }
calendar-late-body = Jammer, ek is ’n paar minute laat vir { $title }. Ek sal binnekort daar wees.
calendar-guests =
    { $count ->
        [one] { $count } gas
       *[other] { $count } gaste
    }
calendar-guest-answers = { $yes } ja, { $maybe } miskien, { $no } nee, { $waiting } wag
calendar-organizer = Organiseerder
calendar-optional = Opsioneel
calendar-open-web = Maak in die blaaier oop
calendar-open-mail = Maak die e-pos oop
calendar-open-contact = Maak kontak oop
calendar-close = Maak toe

## Adding, changing and deleting events.

calendar-add-title = Voeg titel by
calendar-add-location = Voeg ligging by
calendar-add-notes = Voeg beskrywing by
calendar-add-guests = Voeg gaste by
calendar-remove-guest = Verwyder
calendar-add-meet = Voeg Google Meet-videogesprek by
calendar-add-teams = Voeg Teams-vergadering by
calendar-has-call = Videogesprek bygevoeg
calendar-weekday-day = { $weekday }, { $day }
calendar-schedule-day = { $weekday }, { $month }
calendar-all-day-box = Heeldag
calendar-more-options = Meer opsies
calendar-save = Stoor
calendar-saved = Geleentheid gestoor
calendar-deleted = Geleentheid uitgevee
calendar-discard = Verwerp veranderinge
calendar-edit = Wysig geleentheid
calendar-delete = Vee geleentheid uit
calendar-event-details = Geleentheidbesonderhede
# Right-click menus on the calendar: on a free time or day, an event and
# a task.
calendar-menu-new-event = Nuwe geleentheid
calendar-event-window-title = Nuwe gebeurtenis
# Shows the day right-clicked on its own, in the Day view.
calendar-menu-open-day = Maak dag oop
calendar-menu-duplicate = Dupliseer
calendar-menu-color = Kleur
# The event takes its calendar's color.
calendar-menu-color-calendar = Kalenderkleur
# A task's new due day, a week from today.
calendar-menu-in-a-week = Oor 'n week
# Event colors, by the names Google Calendar gives them.
calendar-color-tomato = Tamatie
calendar-color-flamingo = Flamink
calendar-color-tangerine = Nartjie
calendar-color-banana = Piesang
calendar-color-sage = Salie
calendar-color-basil = Basilie
calendar-color-peacock = Pou
calendar-color-blueberry = Bloubessie
calendar-color-lavender = Laventel
calendar-color-grape = Druif
calendar-color-graphite = Grafiet
calendar-menu-only-this = Wys net hierdie
calendar-menu-rename = Hernoem
calendar-menu-remove = Verwyder van lys
calendar-menu-delete = Vee uit
calendar-menu-new-calendar = Nuwe kalender
calendar-menu-show-all = Wys alles
calendar-menu-hide-all = Versteek alles
calendar-menu-account-settings = Rekeninginstellings
calendar-why-main = Hoofkalender
calendar-why-last = Enigste een hier
calendar-why-owner = Net eienaar
calendar-why-contacts = Uit Kontakte
calendar-why-unreached = Nie bereik nie
calendar-name-placeholder = Kalendernaam
calendar-toast-added = “{ $name }” bygevoeg
calendar-toast-renamed = Kalender hernoem
calendar-toast-recolored = Kalenderkleur verander
calendar-toast-deleted = “{ $name }” uitgevee
calendar-toast-removed = “{ $name }” van jou lys verwyder
calendar-edit-failed = Die kalender is nie verander nie: { $reason }
calendar-delete-title = Vee “{ $name }” uit?
calendar-delete-confirm = Vee uit
calendar-deleting = Vee tans uit…
calendar-delete-heading = Uitgevee:
calendar-delete-events = Die kalender en al sy geleenthede
calendar-delete-shared = Vir almal met wie dit gedeel is
calendar-delete-server = Dit word van { $account } op die e-posdiens uitgevee, nie net in Katna nie.
calendar-delete-local = Dit word van hierdie rekenaar uitgevee.
calendar-remove-title = Verwyder “{ $name }” van jou lys?
calendar-remove-confirm = Verwyder
calendar-removing = Verwyder tans…
calendar-remove-heading = Wat verander:
calendar-remove-events = Jy sien nie meer sy geleenthede nie, hier en in jou ander programme
calendar-remove-server = Die kalender bly by sy eienaar, wat dit weer met jou kan deel.
calendar-kind-event = Geleentheid
calendar-kind-task = Taak
calendar-kind-focus = Fokustyd
calendar-kind-out-of-office = Uit die kantoor
calendar-kind-working-location = Werkplek
calendar-task-added = Taak bygevoeg
calendar-task-added-to = Taak by { $list } gevoeg
calendar-task-list-local = Op hierdie rekenaar
calendar-working-home = Tuis
calendar-busy = Besig
calendar-free = Beskikbaar
calendar-cancel = Kanselleer
calendar-ok = OK
calendar-read-only = Jy kan nie geleenthede in hierdie kalender verander nie
calendar-none-editable = Nog geen kalender waarby jy geleenthede kan voeg nie
calendar-no-such-time = Daardie tyd bestaan nie in jou tydsone nie
calendar-end-before-start = Die geleentheid eindig voordat dit begin
calendar-repeat-never = Herhaal nie
calendar-repeat-daily = Daagliks
calendar-repeat-weekly = Weekliks op { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] Maandeliks op die eerste { $weekday }
        [2] Maandeliks op die tweede { $weekday }
        [3] Maandeliks op die derde { $weekday }
        [4] Maandeliks op die vierde { $weekday }
       *[other] Maandeliks op die laaste { $weekday }
    }
calendar-repeat-yearly = Jaarliks op { $day }
calendar-repeat-weekdays = Elke weeksdag (Maandag tot Vrydag)
calendar-repeat-custom = Pasgemaak
calendar-reminder-none = Geen kennisgewing
calendar-reminder-at-start = Met die begin
calendar-reminder-minutes =
    { $count ->
        [one] { $count } minuut voor
       *[other] { $count } minute voor
    }
calendar-reminder-hours =
    { $count ->
        [one] { $count } uur voor
       *[other] { $count } ure voor
    }
calendar-reminder-days =
    { $count ->
        [one] { $count } dag voor
       *[other] { $count } dae voor
    }
calendar-scope-edit-title = Wysig herhalende geleentheid
calendar-scope-delete-title = Vee herhalende geleentheid uit
calendar-scope-this = Hierdie geleentheid
calendar-scope-following = Hierdie en volgende geleenthede
calendar-scope-all = Alle geleenthede
calendar-scope-respond-title = Antwoord vir ’n herhalende geleentheid
calendar-going = Gaan jy?
calendar-answer-yes = Ja
calendar-answer-no = Nee
calendar-answer-maybe = Miskien
calendar-answered-yes = Jy gaan
calendar-answered-no = Jy gaan nie
calendar-answered-maybe = Jy gaan dalk

## The card at the top of a mail with an invitation.

calendar-invite = Uitnodiging
calendar-invite-cancelled = Geleentheid gekanselleer
calendar-invite-reply = { $name } het geantwoord
calendar-invite-reply-yes = { $name } het aanvaar
calendar-invite-reply-no = { $name } het van die hand gewys
calendar-invite-reply-maybe = { $name } gaan dalk
calendar-invite-organizer = Georganiseer deur { $name }
calendar-invite-open = Maak in Kalender oop
calendar-invite-not-yet = Nog nie in jou kalender nie. Jy kan antwoord sodra dit gesinkroniseer is.
calendar-invite-by-mail = Nie in jou kalender nie: jou antwoord gaan per e-pos na die organiseerder.
calendar-mail-yes = Aanvaar: { $title }
calendar-mail-yes-body = { $name } het hierdie uitnodiging aanvaar.
calendar-mail-no = Van die hand gewys: { $title }
calendar-mail-no-body = { $name } het hierdie uitnodiging van die hand gewys.
calendar-mail-maybe = Voorlopig aanvaar: { $title }
calendar-mail-maybe-body = { $name } het hierdie uitnodiging voorlopig aanvaar.
calendar-invite-your-day = Jou dag
calendar-invite-clashes =
    { $count ->
        [one] Bots met { $count } geleentheid
       *[other] Bots met { $count } geleenthede
    }

## The day's agenda beside the mail.

agenda-show = Wys die dag se agenda
agenda-hide = Versteek die agenda
agenda-today = Vandag, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = Niks beplan vir hierdie dag nie.
