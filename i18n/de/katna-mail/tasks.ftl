# Katna Mail, German (Deutsch): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Erstellen
tasks-all = Alle Aufgaben
tasks-starred = Markiert
tasks-new-list = Neue Liste erstellen
tasks-on-this-computer = Auf diesem Computer
tasks-my-tasks = Meine Aufgaben
tasks-list-name-placeholder = Listenname

## Lists and tasks

tasks-loading = Ihre Aufgaben werden gelesen…
tasks-no-lists = Ihre Aufgabenlisten erscheinen hier.
tasks-add = Aufgabe hinzufügen
tasks-title-placeholder = Titel
tasks-add-step = Unteraufgabe hinzufügen
tasks-empty = Noch keine Aufgaben. Fügen Sie oben eine hinzu.
tasks-starred-empty = Markieren Sie eine Aufgabe, damit sie hier erscheint.
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
tasks-toast-deleted = Aufgabe gelöscht
tasks-toast-added = { $count ->
    [one] Zu Aufgaben hinzugefügt
   *[other] { $count } Aufgaben hinzugefügt
}
tasks-mail-gone = Diese E-Mail ist nicht mehr vorhanden.
tasks-toast-list-deleted = Liste gelöscht
tasks-toast-moved = Nach { $list } verschoben
