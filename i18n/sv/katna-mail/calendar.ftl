# Katna Mail, Swedish (Svenska): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = Idag
calendar-today-tip = Gå till idag
calendar-view-day = Dag
calendar-view-week = Vecka
calendar-view-month = Månad
calendar-view-year = År
calendar-view-schedule = Schema
calendar-view-days =
    { $count ->
        [one] { $count } dag
       *[other] { $count } dagar
    }
calendar-options = Alternativ
calendar-density = Densitet
calendar-density-responsive = Anpassas efter skärmen
calendar-density-comfortable = Bekväm
calendar-density-compact = Kompakt
calendar-custom-days = Anpassad vy
calendar-second-zone = Andra tidszon
calendar-zone-none = Ingen
calendar-zone = { $zone } ({ $offset })
calendar-share-free = Dela lediga tider
calendar-free-subject = Tider då jag är ledig
calendar-free-intro = Här är några tider då jag är ledig ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = Jag har ingen ledig tid de närmaste arbetsdagarna.
calendar-previous-day = Föregående dag
calendar-next-day = Nästa dag
calendar-previous-week = Föregående vecka
calendar-next-week = Nästa vecka
calendar-previous-month = Föregående månad
calendar-next-month = Nästa månad
calendar-previous-year = Föregående år
calendar-next-year = Nästa år
calendar-previous-period = Tidigare
calendar-next-period = Senare
calendar-title-months = { $first } – { $last }
calendar-loading = Läser in …
calendar-read-failed = Kalendern kunde inte läsas: { $error }
calendar-sets = Kalenderuppsättningar
calendar-set-add = Spara de visade kalendrarna som en uppsättning
calendar-set-name = Uppsättningens namn
calendar-set-remove = Ta bort uppsättning
calendar-local = Den här datorn
calendar-account-gone = Borttaget konto
calendar-account-sign-in = Logga in igen för att visa kalendrar
calendar-account-signed-in = Inloggad på { $address } igen. Hämtar dina kalendrar…
calendar-account-sign-in-refused = { $provider } släppte inte in Katna. Försök igen och ge åtkomst till dina kalendrar.
calendar-account-refused = Servern godtog inte lösenordet. Yahoo, iCloud, Zoho och andra kräver ett applösenord.
calendar-account-change-password = Ändra lösenord
calendar-account-change-password-tooltip = Öppna Inställningar > Konton
calendar-account-not-enabled = Kalenderåtkomst för Katna är inte påslagen än.
calendar-account-failed = Det gick inte att läsa kalendrarna.
calendar-account-error = Det gick inte att läsa kalendrarna: { $reason }
calendar-account-none = Inga kalendrar hittades
calendar-account-looking = Letar efter kalendrar…
calendar-account-try-again = Försök igen
calendar-account-try-again-tooltip = Kontrollera det här kontots kalendrar igen nu
calendar-account-fixing = Arbetar på det…
calendar-birthdays = Födelsedagar
calendar-birthday-of = Födelsedag för { $name }
calendar-empty-title = Inga kalendrar ännu
calendar-empty-text = Katna visar här kalendrarna för dina Google- och Microsoft-konton när de har synkroniserats, och kalendrar från andra servrar som erbjuder CalDAV.
calendar-schedule-empty = Inget planerat de kommande två månaderna.
calendar-search = Sök bland händelser
calendar-search-past = Tidigare händelser
calendar-search-none = Inga händelser matchar din sökning.
calendar-no-title = (Ingen rubrik)
calendar-all-day = Heldag
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count } till
calendar-repeats = Återkommer
calendar-join = Anslut
calendar-email-guests = Skicka e-post till gäster
calendar-running-late = Jag är försenad
calendar-late-subject = Försenad: { $title }
calendar-late-body = Ursäkta, jag är några minuter försenad till { $title }. Jag kommer snart.
calendar-guests =
    { $count ->
        [one] { $count } gäst
       *[other] { $count } gäster
    }
calendar-guest-answers = { $yes } ja, { $maybe } kanske, { $no } nej, { $waiting } väntar
calendar-organizer = Arrangör
calendar-optional = Valfri
calendar-open-web = Öppna i webbläsaren
calendar-open-contact = Öppna kontakt
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
calendar-kind-event = Händelse
calendar-kind-focus = Fokustid
calendar-kind-out-of-office = Frånvaro
calendar-kind-working-location = Arbetsplats
calendar-working-home = Hemma
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
calendar-invite-by-mail = Finns inte i din kalender: ditt svar skickas till arrangören med e-post.
calendar-mail-yes = Accepterad: { $title }
calendar-mail-yes-body = { $name } har accepterat den här inbjudan.
calendar-mail-no = Avböjd: { $title }
calendar-mail-no-body = { $name } har avböjt den här inbjudan.
calendar-mail-maybe = Preliminärt accepterad: { $title }
calendar-mail-maybe-body = { $name } har preliminärt accepterat den här inbjudan.
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
