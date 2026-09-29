# Katna Mail, Swedish (Svenska): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = Idag
calendar-today-tip = Gå till idag
calendar-view-day = Dag
calendar-view-week = Vecka
calendar-view-month = Månad
calendar-view-schedule = Schema
calendar-previous-day = Föregående dag
calendar-next-day = Nästa dag
calendar-previous-week = Föregående vecka
calendar-next-week = Nästa vecka
calendar-previous-month = Föregående månad
calendar-next-month = Nästa månad
calendar-previous-period = Tidigare
calendar-next-period = Senare
calendar-title-months = { $first } – { $last }
calendar-loading = Läser in …
calendar-read-failed = Kalendern kunde inte läsas: { $error }
calendar-local = Den här datorn
calendar-account-gone = Borttaget konto
calendar-empty-title = Inga kalendrar ännu
calendar-empty-text = Katna visar här kalendrarna för dina Google- och Microsoft-konton när de har synkroniserats, och kalendrar från andra servrar som erbjuder CalDAV.
calendar-schedule-empty = Inget planerat de kommande två månaderna.
calendar-no-title = (Ingen rubrik)
calendar-all-day = Heldag
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count } till
calendar-repeats = Återkommer
calendar-join = Anslut
calendar-guests =
    { $count ->
        [one] { $count } gäst
       *[other] { $count } gäster
    }
calendar-guest-answers = { $yes } ja, { $maybe } kanske, { $no } nej, { $waiting } väntar
calendar-organizer = Arrangör
calendar-optional = Valfri
calendar-open-web = Öppna i webbläsaren
calendar-close = Stäng

## Adding, changing and deleting events.

calendar-add-title = Lägg till rubrik
calendar-add-location = Lägg till plats
calendar-add-notes = Lägg till beskrivning
calendar-add-guests = Lägg till gäster
calendar-remove-guest = Ta bort
calendar-add-meet = Lägg till videomöte i Google Meet
calendar-add-teams = Lägg till Teams-möte
calendar-has-call = Videomöte har lagts till
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = Heldag
calendar-more-options = Fler alternativ
calendar-save = Spara
calendar-saved = Händelsen har sparats
calendar-deleted = Händelsen har raderats
calendar-discard = Ignorera ändringar
calendar-edit = Redigera händelse
calendar-delete = Radera händelse
calendar-event-details = Händelseinformation
calendar-busy = Upptagen
calendar-free = Ledig
calendar-cancel = Avbryt
calendar-ok = OK
calendar-read-only = Du kan inte ändra händelser i den här kalendern
calendar-none-editable = Ännu ingen kalender där du kan lägga till händelser
calendar-no-such-time = Den tiden finns inte i din tidszon
calendar-end-before-start = Händelsen slutar innan den börjar
calendar-repeat-never = Upprepas inte
calendar-repeat-daily = Varje dag
calendar-repeat-weekly = Varje vecka på { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] Varje månad den första { $weekday }
        [2] Varje månad den andra { $weekday }
        [3] Varje månad den tredje { $weekday }
        [4] Varje månad den fjärde { $weekday }
       *[other] Varje månad den sista { $weekday }
    }
calendar-repeat-yearly = Varje år den { $day }
calendar-repeat-weekdays = Varje vardag (måndag till fredag)
calendar-repeat-custom = Anpassad
calendar-reminder-none = Ingen avisering
calendar-reminder-at-start = Vid start
calendar-reminder-minutes =
    { $count ->
        [one] { $count } minut innan
       *[other] { $count } minuter innan
    }
calendar-reminder-hours =
    { $count ->
        [one] { $count } timme innan
       *[other] { $count } timmar innan
    }
calendar-reminder-days =
    { $count ->
        [one] { $count } dag innan
       *[other] { $count } dagar innan
    }
calendar-scope-edit-title = Redigera återkommande händelse
calendar-scope-delete-title = Radera återkommande händelse
calendar-scope-this = Den här händelsen
calendar-scope-following = Den här och följande händelser
calendar-scope-all = Alla händelser
calendar-scope-respond-title = Svar för en återkommande händelse
calendar-going = Kommer du?
calendar-answer-yes = Ja
calendar-answer-no = Nej
calendar-answer-maybe = Kanske
calendar-answered-yes = Du kommer
calendar-answered-no = Du kommer inte
calendar-answered-maybe = Du kommer kanske

## The card at the top of a mail with an invitation.

calendar-invite = Inbjudan
calendar-invite-cancelled = Händelse inställd
calendar-invite-reply = { $name } har svarat
calendar-invite-reply-yes = { $name } har tackat ja
calendar-invite-reply-no = { $name } har tackat nej
calendar-invite-reply-maybe = { $name } kommer kanske
calendar-invite-organizer = Arrangeras av { $name }
calendar-invite-open = Öppna i Kalender
calendar-invite-not-yet = Finns inte i din kalender än. Du kan svara när den har synkroniserats.
calendar-invite-your-day = Din dag
calendar-invite-clashes =
    { $count ->
        [one] Krockar med { $count } händelse
       *[other] Krockar med { $count } händelser
    }

## The day's agenda beside the mail.

agenda-show = Visa dagens agenda
agenda-hide = Dölj agendan
agenda-today = Idag, { $date }
agenda-day = { $weekday } { $date }
agenda-empty = Inget planerat den här dagen.
