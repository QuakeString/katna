# Katna Mail, Dutch (Nederlands): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = Vandaag
calendar-today-tip = Naar vandaag gaan
calendar-view-day = Dag
calendar-view-week = Week
calendar-view-month = Maand
calendar-view-year = Jaar
calendar-view-schedule = Agenda
calendar-view-days =
    { $count ->
        [one] { $count } dag
       *[other] { $count } dagen
    }
calendar-options = Opties
calendar-density = Dichtheid
calendar-density-responsive = Afgestemd op je scherm
calendar-density-comfortable = Comfortabel
calendar-density-compact = Compact
calendar-custom-days = Aangepaste weergave
calendar-second-zone = Tweede tijdzone
calendar-zone-none = Geen
calendar-zone = { $zone } ({ $offset })
calendar-share-free = Vrije tijden delen
calendar-free-subject = Tijden dat ik vrij ben
calendar-free-intro = Hier zijn enkele tijden dat ik vrij ben ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = Ik heb de komende werkdagen geen vrije tijd.
calendar-previous-day = Vorige dag
calendar-next-day = Volgende dag
calendar-previous-week = Vorige week
calendar-next-week = Volgende week
calendar-previous-month = Vorige maand
calendar-next-month = Volgende maand
calendar-previous-year = Vorig jaar
calendar-next-year = Volgend jaar
calendar-previous-period = Eerder
calendar-next-period = Later
calendar-title-months = { $first } – { $last }
calendar-loading = Laden…
calendar-read-failed = De agenda kon niet worden gelezen: { $error }
calendar-sets = Agendagroepen
calendar-set-add = Getoonde agenda’s als groep opslaan
calendar-set-name = Naam van de groep
calendar-set-remove = Groep verwijderen
calendar-local = Deze computer
calendar-account-gone = Verwijderd account
calendar-account-sign-in = Meld je opnieuw aan om agenda’s te tonen
calendar-account-signed-in = Opnieuw aangemeld bij { $address }. Je agenda’s worden opgehaald…
calendar-account-sign-in-refused = { $provider } heeft Katna niet binnengelaten. Probeer het opnieuw en geef toegang tot je agenda’s.
calendar-account-refused = De server heeft het wachtwoord niet geaccepteerd. Yahoo, iCloud, Zoho en andere hebben een app-wachtwoord nodig.
calendar-account-change-password = Wachtwoord wijzigen
calendar-account-change-password-tooltip = Instellingen > Accounts openen
calendar-account-not-enabled = Agendatoegang voor Katna is nog niet ingeschakeld.
calendar-account-failed = De agenda’s konden niet worden gelezen.
calendar-account-error = De agenda’s konden niet worden gelezen: { $reason }
calendar-account-none = Geen agenda’s gevonden
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
calendar-account-none-why = Geen agenda’s gevonden: { $reason }
# A Gmail or Outlook account added with a password: its calendars need the
# provider's sign-in.
calendar-account-use-sign-in = { $provider } toont agenda’s alleen aan Katna als die is aangemeld met { $provider }.
calendar-account-sign-in-with = Aanmelden met { $provider }
calendar-account-looking = Agenda’s zoeken…
calendar-account-try-again = Opnieuw proberen
calendar-account-try-again-tooltip = De agenda’s van dit account nu opnieuw controleren
calendar-account-fixing = Bezig…
calendar-birthdays = Verjaardagen
calendar-birthday-of = Verjaardag van { $name }
calendar-empty-title = Nog geen agenda’s
calendar-empty-text = Katna toont hier de agenda’s van je Google- en Microsoft-accounts zodra ze zijn gesynchroniseerd, en die van andere servers die CalDAV bieden.
calendar-schedule-empty = Er staat niets gepland voor de komende twee maanden.
calendar-search = Afspraken zoeken
calendar-search-past = Afgelopen afspraken
calendar-search-none = Er zijn geen afspraken die overeenkomen met je zoekopdracht.
calendar-no-title = (Geen titel)
calendar-all-day = Hele dag
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = nog { $count }
calendar-repeats = Herhaalt
calendar-join = Deelnemen
calendar-email-guests = Gasten e-mailen
calendar-running-late = Ik ben te laat
calendar-late-subject = Te laat: { $title }
calendar-late-body = Sorry, ik ben een paar minuten te laat voor { $title }. Ik ben er zo.
calendar-guests =
    { $count ->
        [one] { $count } gast
       *[other] { $count } gasten
    }
calendar-guest-answers = { $yes } ja, { $maybe } misschien, { $no } nee, { $waiting } in afwachting
calendar-organizer = Organisator
calendar-optional = Optioneel
calendar-open-web = Openen in de browser
calendar-open-contact = Contact openen
calendar-close = Sluiten

## Adding, changing and deleting events.

calendar-add-title = Titel toevoegen
calendar-add-location = Locatie toevoegen
calendar-add-notes = Beschrijving toevoegen
calendar-add-guests = Gasten toevoegen
calendar-remove-guest = Verwijderen
calendar-add-meet = Google Meet-videovergadering toevoegen
calendar-add-teams = Teams-vergadering toevoegen
calendar-has-call = Videovergadering toegevoegd
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = Hele dag
calendar-more-options = Meer opties
calendar-save = Opslaan
calendar-saved = Afspraak opgeslagen
calendar-deleted = Afspraak verwijderd
calendar-discard = Wijzigingen negeren
calendar-edit = Afspraak bewerken
calendar-delete = Afspraak verwijderen
calendar-event-details = Afspraakdetails
calendar-kind-event = Afspraak
calendar-kind-focus = Focustijd
calendar-kind-out-of-office = Niet op kantoor
calendar-kind-working-location = Werklocatie
calendar-working-home = Thuis
calendar-busy = Bezet
calendar-free = Beschikbaar
calendar-cancel = Annuleren
calendar-ok = OK
calendar-read-only = Je kunt afspraken in deze agenda niet wijzigen
calendar-none-editable = Nog geen agenda waaraan je afspraken kunt toevoegen
calendar-no-such-time = Dat tijdstip bestaat niet in je tijdzone
calendar-end-before-start = De afspraak eindigt voordat deze begint
calendar-repeat-never = Herhaalt niet
calendar-repeat-daily = Dagelijks
calendar-repeat-weekly = Wekelijks op { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] Maandelijks op de eerste { $weekday }
        [2] Maandelijks op de tweede { $weekday }
        [3] Maandelijks op de derde { $weekday }
        [4] Maandelijks op de vierde { $weekday }
       *[other] Maandelijks op de laatste { $weekday }
    }
calendar-repeat-yearly = Jaarlijks op { $day }
calendar-repeat-weekdays = Elke weekdag (maandag t/m vrijdag)
calendar-repeat-custom = Aangepast
calendar-reminder-none = Geen melding
calendar-reminder-at-start = Bij aanvang
calendar-reminder-minutes =
    { $count ->
        [one] { $count } minuut ervoor
       *[other] { $count } minuten ervoor
    }
calendar-reminder-hours =
    { $count ->
        [one] { $count } uur ervoor
       *[other] { $count } uur ervoor
    }
calendar-reminder-days =
    { $count ->
        [one] { $count } dag ervoor
       *[other] { $count } dagen ervoor
    }
calendar-scope-edit-title = Terugkerende afspraak bewerken
calendar-scope-delete-title = Terugkerende afspraak verwijderen
calendar-scope-this = Deze afspraak
calendar-scope-following = Deze en volgende afspraken
calendar-scope-all = Alle afspraken
calendar-scope-respond-title = Antwoord voor een terugkerende afspraak
calendar-going = Ga je?
calendar-answer-yes = Ja
calendar-answer-no = Nee
calendar-answer-maybe = Misschien
calendar-answered-yes = Je gaat
calendar-answered-no = Je gaat niet
calendar-answered-maybe = Je gaat misschien

## The card at the top of a mail with an invitation.

calendar-invite = Uitnodiging
calendar-invite-cancelled = Afspraak geannuleerd
calendar-invite-reply = { $name } heeft geantwoord
calendar-invite-reply-yes = { $name } heeft geaccepteerd
calendar-invite-reply-no = { $name } heeft afgewezen
calendar-invite-reply-maybe = { $name } komt misschien
calendar-invite-organizer = Georganiseerd door { $name }
calendar-invite-open = Openen in Agenda
calendar-invite-not-yet = Nog niet in je agenda. Je kunt antwoorden zodra de synchronisatie klaar is.
calendar-invite-by-mail = Niet in je agenda: je antwoord gaat per e-mail naar de organisator.
calendar-mail-yes = Geaccepteerd: { $title }
calendar-mail-yes-body = { $name } heeft deze uitnodiging geaccepteerd.
calendar-mail-no = Geweigerd: { $title }
calendar-mail-no-body = { $name } heeft deze uitnodiging geweigerd.
calendar-mail-maybe = Voorlopig geaccepteerd: { $title }
calendar-mail-maybe-body = { $name } heeft deze uitnodiging voorlopig geaccepteerd.
calendar-invite-your-day = Jouw dag
calendar-invite-clashes =
    { $count ->
        [one] Overlapt met { $count } afspraak
       *[other] Overlapt met { $count } afspraken
    }

## The day's agenda beside the mail.

agenda-show = Agenda van de dag tonen
agenda-hide = Agenda verbergen
agenda-today = Vandaag, { $date }
agenda-day = { $weekday } { $date }
agenda-empty = Er staat niets gepland op deze dag.
