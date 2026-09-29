# Katna Mail, German (Deutsch): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Erstellen
tasks-all = Alle Aufgaben
tasks-today = Heute
tasks-starred = Markiert
tasks-new-list = Neue Liste erstellen
tasks-on-this-computer = Auf diesem Computer
tasks-my-tasks = Meine Aufgaben
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = Erneut anmelden, um Aufgaben anzuzeigen
tasks-account-signed-in = Wieder bei { $address } angemeldet. Ihre Aufgaben werden abgerufen…
tasks-account-sign-in-refused = { $provider } hat Katna keinen Zugang gewährt. Versuchen Sie es erneut und erlauben Sie den Zugriff auf Ihre Aufgaben.
tasks-account-refused = Der Server hat das Passwort nicht akzeptiert. Yahoo, iCloud, Zoho und andere brauchen ein App-Passwort.
tasks-account-change-password = Passwort ändern
tasks-account-change-password-tooltip = Einstellungen > Konten öffnen
tasks-account-not-enabled = Der Aufgabenzugriff für Katna ist noch nicht eingeschaltet.
tasks-account-failed = Die Aufgabenlisten konnten nicht gelesen werden.
# $reason is the server's own words, in English.
tasks-account-error = Die Aufgabenlisten konnten nicht gelesen werden: { $reason }
tasks-account-none = Keine Aufgabenlisten gefunden
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = Keine Aufgabenlisten gefunden: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } zeigt Aufgaben nur Katna, wenn es mit { $provider } angemeldet ist.
tasks-account-sign-in-with = Mit { $provider } anmelden
tasks-account-looking = Aufgabenlisten werden gesucht…
tasks-account-try-again = Erneut versuchen
tasks-account-try-again-tooltip = Die Aufgaben dieses Kontos jetzt erneut prüfen
tasks-account-fixing = Wird bearbeitet…
tasks-list-name-placeholder = Listenname

## Lists and tasks

tasks-loading = Ihre Aufgaben werden gelesen…
tasks-no-lists = Ihre Aufgabenlisten erscheinen hier.
tasks-search = Aufgaben durchsuchen
tasks-search-none = Keine Aufgaben entsprechen Ihrer Suche.
tasks-add = Aufgabe hinzufügen
tasks-title-placeholder = Titel
tasks-add-step = Unteraufgabe hinzufügen
tasks-empty = Noch keine Aufgaben. Fügen Sie oben eine hinzu.
tasks-starred-empty = Markieren Sie eine Aufgabe, damit sie hier erscheint.
tasks-today-empty = Heute ist nichts fällig.
tasks-today-date = { $weekday }, { $day }
tasks-overdue = Überfällig
tasks-completed = { $count ->
    [one] Erledigt ({ $count })
   *[other] Erledigt ({ $count })
}
tasks-list-options = Listenoptionen
tasks-rename-list = Liste umbenennen
tasks-delete-list = Liste löschen
tasks-mark-done = Als erledigt markieren
tasks-mark-open = Als nicht erledigt markieren
tasks-star = Markieren
tasks-unstar = Markierung entfernen
tasks-edit-title = Titel bearbeiten
tasks-details = Details
tasks-delete = Löschen
tasks-move-to = Nach { $list } verschieben
tasks-from-mail = E-Mail
tasks-open-mail = Die E-Mail öffnen
tasks-from-note = Notiz
tasks-open-note = Die Notiz öffnen
tasks-note-gone = Diese Notiz gibt es nicht mehr.
tasks-no-subject = (kein Betreff)

## The details dialog

tasks-notes-placeholder = Details hinzufügen
tasks-date = Datum
tasks-no-date = Kein Datum
tasks-time-placeholder = Uhrzeit hinzufügen
tasks-repeat = Wiederholen
tasks-repeat-never = Wiederholt sich nicht
tasks-repeat-daily = Täglich
tasks-repeat-weekly = Wöchentlich
tasks-repeat-monthly = Monatlich
tasks-repeat-yearly = Jährlich
tasks-repeat-other = Benutzerdefiniert
tasks-remind = Erinnern
tasks-remind-off = Nicht erinnern
tasks-remind-on-time = Zur festgelegten Zeit
tasks-remind-morning = Am selben Tag, { $time }
tasks-remind-hour-before = Eine Stunde vorher
tasks-remind-day-before = Einen Tag vorher
tasks-cancel = Abbrechen
tasks-save = Speichern
tasks-not-a-time = „{ $text }“ ist keine Uhrzeit, zum Beispiel { $example }.

## Due days

tasks-due-today = Heute
tasks-due-tomorrow = Morgen
tasks-due-yesterday = Gestern
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = Aufgabe erledigt
tasks-toast-next = Erledigt. Nächste am { $date }
tasks-toast-deleted = Aufgabe gelöscht
tasks-toast-added = { $count ->
    [one] Zu Aufgaben hinzugefügt
   *[other] { $count } Aufgaben hinzugefügt
}
tasks-mail-gone = Diese E-Mail ist nicht mehr vorhanden.
tasks-toast-list-deleted = Liste gelöscht
tasks-toast-moved = Nach { $list } verschoben
# A task dragged to another place in its own list.
tasks-toast-placed = Aufgabe verschoben
tasks-toast-rescheduled = Aufgabe neu terminiert
