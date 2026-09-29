# Katna Mail, German (Deutsch): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = Heute
calendar-today-tip = Zum heutigen Tag springen
calendar-view-day = Tag
calendar-view-week = Woche
calendar-view-month = Monat
calendar-view-year = Jahr
calendar-view-schedule = Terminübersicht
calendar-view-days =
    { $count ->
        [one] { $count } Tag
       *[other] { $count } Tage
    }
calendar-options = Optionen
calendar-density = Dichte
calendar-density-responsive = An Bildschirm anpassen
calendar-density-comfortable = Komfortabel
calendar-density-compact = Kompakt
calendar-custom-days = Eigene Ansicht
calendar-second-zone = Zweite Zeitzone
calendar-zone-none = Keine
calendar-zone = { $zone } ({ $offset })
calendar-share-free = Freie Zeiten teilen
calendar-free-subject = Zeiten, zu denen ich frei bin
calendar-free-intro = Hier sind einige Zeiten, zu denen ich frei bin ({ $zone }):
calendar-free-day = { $weekday }, { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = In den nächsten Werktagen habe ich keine freie Zeit.
calendar-previous-day = Vorheriger Tag
calendar-next-day = Nächster Tag
calendar-previous-week = Vorherige Woche
calendar-next-week = Nächste Woche
calendar-previous-month = Vorheriger Monat
calendar-next-month = Nächster Monat
calendar-previous-year = Vorheriges Jahr
calendar-next-year = Nächstes Jahr
calendar-previous-period = Früher
calendar-next-period = Später
calendar-title-months = { $first } – { $last }
calendar-loading = Wird geladen …
calendar-read-failed = Der Kalender konnte nicht gelesen werden: { $error }
calendar-sets = Kalendergruppen
calendar-set-add = Angezeigte Kalender als Gruppe speichern
calendar-set-name = Name der Gruppe
calendar-set-remove = Gruppe entfernen
calendar-local = Dieser Computer
calendar-account-gone = Entferntes Konto
calendar-account-sign-in = Erneut anmelden, um Kalender anzuzeigen
calendar-account-signed-in = Wieder bei { $address } angemeldet. Ihre Kalender werden abgerufen…
calendar-account-sign-in-refused = { $provider } hat Katna keinen Zugang gewährt. Versuchen Sie es erneut und erlauben Sie den Zugriff auf Ihre Kalender.
calendar-account-refused = Der Server hat das Passwort nicht akzeptiert. Yahoo, iCloud, Zoho und andere brauchen ein App-Passwort.
calendar-account-change-password = Passwort ändern
calendar-account-change-password-tooltip = Einstellungen > Konten öffnen
calendar-account-not-enabled = Der Kalenderzugriff für Katna ist noch nicht eingeschaltet.
calendar-account-failed = Die Kalender konnten nicht gelesen werden.
calendar-account-error = Die Kalender konnten nicht gelesen werden: { $reason }
calendar-account-none = Keine Kalender gefunden
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
calendar-account-none-why = Keine Kalender gefunden: { $reason }
# A Gmail or Outlook account added with a password: its calendars need the
# provider's sign-in.
calendar-account-use-sign-in = { $provider } zeigt Kalender nur Katna, wenn es mit { $provider } angemeldet ist.
calendar-account-sign-in-with = Mit { $provider } anmelden
calendar-account-looking = Kalender werden gesucht…
calendar-account-try-again = Erneut versuchen
calendar-account-try-again-tooltip = Die Kalender dieses Kontos jetzt erneut prüfen
calendar-account-fixing = Wird bearbeitet…
calendar-birthdays = Geburtstage
calendar-birthday-of = Geburtstag von { $name }
calendar-empty-title = Noch keine Kalender
calendar-empty-text = Katna zeigt hier die Kalender Ihrer Google- und Microsoft-Konten an, sobald sie synchronisiert sind, sowie die anderer Server, die CalDAV anbieten.
calendar-schedule-empty = In den nächsten zwei Monaten ist nichts geplant.
calendar-search = Termine durchsuchen
calendar-search-past = Vergangene Termine
calendar-search-none = Keine Termine entsprechen Ihrer Suche.
calendar-no-title = (Kein Titel)
calendar-all-day = Ganztägig
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count } weitere
calendar-repeats = Wiederholt sich
calendar-join = Teilnehmen
calendar-email-guests = E-Mail an Gäste
calendar-running-late = Ich verspäte mich
calendar-late-subject = Verspätung: { $title }
calendar-late-body = Entschuldigung, ich verspäte mich für { $title } um ein paar Minuten. Ich bin bald da.
calendar-guests =
    { $count ->
        [one] { $count } Gast
       *[other] { $count } Gäste
    }
calendar-guest-answers = { $yes } Zusagen, { $maybe } vielleicht, { $no } Absagen, { $waiting } ausstehend
calendar-organizer = Organisator
calendar-optional = Optional
calendar-open-web = Im Browser öffnen
calendar-open-contact = Kontakt öffnen
calendar-close = Schließen

## Adding, changing and deleting events.

calendar-add-title = Titel hinzufügen
calendar-add-location = Ort hinzufügen
calendar-add-notes = Beschreibung hinzufügen
calendar-add-guests = Gäste hinzufügen
calendar-remove-guest = Entfernen
calendar-add-meet = Google Meet-Videokonferenz hinzufügen
calendar-add-teams = Teams-Besprechung hinzufügen
calendar-has-call = Videokonferenz hinzugefügt
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = Ganztägig
calendar-more-options = Weitere Optionen
calendar-save = Speichern
calendar-saved = Termin gespeichert
calendar-deleted = Termin gelöscht
calendar-discard = Änderungen verwerfen
calendar-edit = Termin bearbeiten
calendar-delete = Termin löschen
calendar-event-details = Termindetails
# Right-click menus on the calendar: on a free time or day, an event and
# a task.
calendar-menu-new-event = Neuer Termin
# Shows the day right-clicked on its own, in the Day view.
calendar-menu-open-day = Tag öffnen
calendar-menu-duplicate = Duplizieren
calendar-menu-color = Farbe
# The event takes its calendar's color.
calendar-menu-color-calendar = Kalenderfarbe
# A task's new due day, a week from today.
calendar-menu-in-a-week = In einer Woche
# Event colors, by the names Google Calendar gives them.
calendar-color-tomato = Tomate
calendar-color-flamingo = Flamingo
calendar-color-tangerine = Mandarine
calendar-color-banana = Banane
calendar-color-sage = Salbei
calendar-color-basil = Basilikum
calendar-color-peacock = Pfau
calendar-color-blueberry = Heidelbeere
calendar-color-lavender = Lavendel
calendar-color-grape = Weintraube
calendar-color-graphite = Graphit
calendar-kind-event = Event
calendar-kind-focus = Fokuszeit
calendar-kind-out-of-office = Abwesend
calendar-kind-working-location = Arbeitsort
calendar-working-home = Zuhause
calendar-busy = Beschäftigt
calendar-free = Verfügbar
calendar-cancel = Abbrechen
calendar-ok = OK
calendar-read-only = Termine in diesem Kalender können Sie nicht ändern
calendar-none-editable = Noch kein Kalender, zu dem Sie Termine hinzufügen können
calendar-no-such-time = Diese Uhrzeit gibt es in Ihrer Zeitzone nicht
calendar-end-before-start = Der Termin endet, bevor er beginnt
calendar-repeat-never = Wiederholt sich nicht
calendar-repeat-daily = Täglich
calendar-repeat-weekly = Wöchentlich am { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] Monatlich am ersten { $weekday }
        [2] Monatlich am zweiten { $weekday }
        [3] Monatlich am dritten { $weekday }
        [4] Monatlich am vierten { $weekday }
       *[other] Monatlich am letzten { $weekday }
    }
calendar-repeat-yearly = Jährlich am { $day }
calendar-repeat-weekdays = Jeden Wochentag (Montag bis Freitag)
calendar-repeat-custom = Benutzerdefiniert
calendar-reminder-none = Keine Benachrichtigung
calendar-reminder-at-start = Zu Beginn
calendar-reminder-minutes =
    { $count ->
        [one] { $count } Minute vorher
       *[other] { $count } Minuten vorher
    }
calendar-reminder-hours =
    { $count ->
        [one] { $count } Stunde vorher
       *[other] { $count } Stunden vorher
    }
calendar-reminder-days =
    { $count ->
        [one] { $count } Tag vorher
       *[other] { $count } Tage vorher
    }
calendar-scope-edit-title = Serientermin bearbeiten
calendar-scope-delete-title = Serientermin löschen
calendar-scope-this = Dieser Termin
calendar-scope-following = Dieser und alle folgenden Termine
calendar-scope-all = Alle Termine
calendar-scope-respond-title = Antwort für einen Serientermin
calendar-going = Teilnehmen?
calendar-answer-yes = Ja
calendar-answer-no = Nein
calendar-answer-maybe = Vielleicht
calendar-answered-yes = Sie nehmen teil
calendar-answered-no = Sie nehmen nicht teil
calendar-answered-maybe = Sie nehmen vielleicht teil

## The card at the top of a mail with an invitation.

calendar-invite = Einladung
calendar-invite-cancelled = Termin abgesagt
calendar-invite-reply = { $name } hat geantwortet
calendar-invite-reply-yes = { $name } hat zugesagt
calendar-invite-reply-no = { $name } hat abgesagt
calendar-invite-reply-maybe = { $name } hat vielleicht zugesagt
calendar-invite-organizer = Organisiert von { $name }
calendar-invite-open = In Kalender öffnen
calendar-invite-not-yet = Noch nicht in Ihrem Kalender. Sie können antworten, sobald die Synchronisierung erfolgt ist.
calendar-invite-by-mail = Nicht in Ihrem Kalender: Ihre Antwort geht per E-Mail an den Organisator.
calendar-mail-yes = Zugesagt: { $title }
calendar-mail-yes-body = { $name } hat diese Einladung angenommen.
calendar-mail-no = Abgesagt: { $title }
calendar-mail-no-body = { $name } hat diese Einladung abgelehnt.
calendar-mail-maybe = Vorläufig zugesagt: { $title }
calendar-mail-maybe-body = { $name } hat diese Einladung vorläufig angenommen.
calendar-invite-your-day = Ihr Tag
calendar-invite-clashes =
    { $count ->
        [one] Überschneidet sich mit { $count } Termin
       *[other] Überschneidet sich mit { $count } Terminen
    }

## The day's agenda beside the mail.

agenda-show = Tagesübersicht anzeigen
agenda-hide = Tagesübersicht ausblenden
agenda-today = Heute, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = An diesem Tag ist nichts geplant.
