# Katna Mail, German (Deutsch): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Neue Aufgabe
tasks-all = Alle Aufgaben
tasks-today = Heute
tasks-upcoming = Demnächst
tasks-starred = Markiert
tasks-completed-view = Erledigt
tasks-new-list = Neue Liste erstellen
tasks-labels-heading = Labels
tasks-on-this-computer = Auf diesem Computer
tasks-my-tasks = Meine Aufgaben
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = Erneut anmelden, um Aufgaben anzuzeigen
tasks-account-signed-in = Wieder bei { $address } angemeldet. Ihre Aufgaben werden abgerufen…
tasks-account-sign-in-refused = { $provider } hat Katna keinen Zugang gewährt. Versuchen Sie es erneut und erlauben Sie den Zugriff auf Ihre Aufgaben.
tasks-account-refused = Der Server hat das Passwort nicht akzeptiert. Yahoo, iCloud, Zoho und andere brauchen ein App-Passwort.
tasks-account-change-password = Passwort ändern
tasks-account-change-password-tooltip = Geben Sie das neue Passwort ein; Katna prüft es beim Server
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
tasks-label-empty = Keine offenen Aufgaben mit diesem Label.
tasks-today-empty = Heute ist nichts fällig.
tasks-completed-empty = Erledigte Aufgaben erscheinen hier.
tasks-upcoming-add = Aufgabe für { $day } hinzufügen
tasks-upcoming-overdue-day = { $weekday }, { $day }
tasks-from-mail-quiet = Aus E-Mail
tasks-from-note-quiet = Aus Notiz
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday }, { $day }
tasks-overdue = Überfällig
tasks-completed = { $count ->
    [one] Erledigt ({ $count })
   *[other] Erledigt ({ $count })
}
tasks-list-options = Listenoptionen
tasks-sort-by = Sortieren nach
tasks-sort-my-order = Meine Reihenfolge
tasks-sort-date = Datum
tasks-sort-starred = Zuletzt markiert
tasks-sort-title = Titel
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

## Several tasks selected (Ctrl+click, Shift+click)

tasks-selected = { $count ->
    [one] { $count } ausgewählt
   *[other] { $count } ausgewählt
}
tasks-select-clear = Auswahl aufheben
tasks-select-move = In Liste verschieben
tasks-select-date = Datum festlegen
tasks-next-week = Nächste Woche

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
tasks-label-add = Label hinzufügen
tasks-label-task = Aufgabe mit Label versehen
tasks-files-attach = Dateien anhängen
tasks-files-pick = Anhängen
tasks-file-open = Öffnen
tasks-file-remove = Datei entfernen
tasks-file-here = Nur auf diesem Computer
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
tasks-files-added = { $count ->
    [one] Datei angehängt
   *[other] { $count } Dateien angehängt
}
tasks-file-removed = „{ $name }“ entfernt
tasks-files-left-out = Nicht angehängt: { $names }. Eine Aufgabe nimmt Dateien bis { $limit } auf, keine Ordner.
tasks-file-missing = Diese Datei ist nicht mehr vorhanden.
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
tasks-toast-rescheduled-several = { $count ->
    [one] Aufgabe neu terminiert
   *[other] { $count } Aufgaben neu terminiert
}
tasks-toast-done-several = { $count ->
    [one] Aufgabe erledigt
   *[other] { $count } Aufgaben erledigt
}
tasks-toast-open-several = { $count ->
    [one] Aufgabe als nicht erledigt markiert
   *[other] { $count } Aufgaben als nicht erledigt markiert
}
tasks-toast-starred = { $count ->
    [one] Aufgabe markiert
   *[other] { $count } Aufgaben markiert
}
tasks-toast-unstarred = { $count ->
    [one] Markierung entfernt
   *[other] Markierung von { $count } Aufgaben entfernt
}
tasks-toast-deleted-several = { $count ->
    [one] Aufgabe gelöscht
   *[other] { $count } Aufgaben gelöscht
}
