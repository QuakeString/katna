# Katna Mail, German (Deutsch): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = Notizen
notes-view-reminders = Erinnerungen
notes-view-archive = Archiv
notes-view-trash = Papierkorb
notes-edit-labels = Labels bearbeiten
notes-search = Notizen durchsuchen
notes-loading = Ihre Notizen werden geöffnet…

## Board

notes-take-a-note = Notiz schreiben…
notes-new-list = Neue Liste
notes-new-note = Neue Notiz
notes-pinned = Fixiert
notes-others = Sonstige
notes-empty = Hier werden Ihre Notizen angezeigt
notes-archive-empty = Hier werden Ihre archivierten Notizen angezeigt
notes-trash-empty = Keine Notizen im Papierkorb
notes-none-found = Keine passenden Notizen
notes-label-empty = Noch keine Notizen mit diesem Label
notes-reminders-empty = Notizen mit anstehenden Erinnerungen erscheinen hier
notes-trash-note = Notizen im Papierkorb werden nach 7 Tagen gelöscht.
notes-empty-trash = Papierkorb leeren
notes-ticked = { $count ->
    [one] + { $count } abgehaktes Element
   *[other] + { $count } abgehakte Elemente
}
notes-select = Notiz auswählen
notes-selected = { $count ->
    [one] { $count } ausgewählt
   *[other] { $count } ausgewählt
}
notes-select-clear = Auswahl aufheben

## A note's buttons

notes-pin = Notiz fixieren
notes-unpin = Fixierung aufheben
notes-archive = Archivieren
notes-unarchive = Aus dem Archiv holen
notes-delete = Notiz löschen
notes-restore = Wiederherstellen
notes-delete-forever = Endgültig löschen
notes-color = Hintergrundfarbe
notes-checkboxes = Kontrollkästchen ein- oder ausblenden
notes-labels = Labels
notes-close = Schließen
notes-more = Mehr
notes-make-copy = Kopie erstellen
notes-remind = Erinnern
notes-add-picture = Bild hinzufügen
notes-history = Versionsverlauf
notes-ai = Schreibhilfe
notes-send-as-mail = Als E-Mail senden
notes-save-markdown = Als Markdown speichern
notes-save-pdf = Als PDF speichern

## The open note

notes-title = Titel
notes-edited = Bearbeitet: { $date }
notes-on-this-computer = Auf diesem Computer
notes-where = Speicherort dieser Notiz
notes-untitled = Unbenannte Notiz

## Pictures

notes-picture-choose = Bilder hinzufügen
notes-picture-remove = Bild entfernen
notes-picture-too-big = Bilder bis { $size } passen in eine Notiz
notes-picture-kind = Diese Datei ist kein Bild, das Katna anzeigen kann
notes-picture-unreadable = { $name } konnte nicht gelesen werden: { $error }

## Reminders

notes-remind-me = Erinnern
notes-remind-off = Erinnerung entfernen
notes-remind-in-the-past = Wählen Sie einen Zeitpunkt, der noch nicht vorbei ist
notes-remind-today = Heute, { $time }
notes-remind-tomorrow = Morgen, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = Erinnerung für { $when } eingestellt
notes-reminder-off = Erinnerung entfernt

## Links between notes

notes-link-note = Notiz verknüpfen
notes-link-new = Neue Notiz „{ $title }“
notes-linked-from = Verknüpft von
notes-link-gone = Diese Notiz gibt es nicht mehr

## Version history

notes-versions = Versionen
notes-version-now = Jetzt
notes-version-here = Sie, auf diesem Computer
notes-version-yesterday = Gestern, { $time }
notes-version-changes = { $count ->
    [one] { $count } Änderung
   *[other] { $count } Änderungen
}
notes-version-from = Von { $device }
notes-version-elsewhere = Von einem anderen Gerät
notes-version-created = Erstellt
notes-version-restore = Diese Version wiederherstellen
notes-version-restored = Version wiederhergestellt
notes-history-none = Noch keine früheren Versionen

## AI help

notes-ai-tidy = Text aufräumen
notes-ai-checklist = In eine Checkliste umwandeln
notes-ai-summarise = Zusammenfassen
notes-ai-empty = Schreiben Sie zuerst etwas
notes-ai-tidied = Text aufgeräumt. Strg+Z macht es rückgängig.
notes-ai-listed = In eine Checkliste umgewandelt. Strg+Z macht es rückgängig.
notes-ai-summarised = Zusammenfassung oben eingefügt

## Labels

notes-label-note = Notiz mit Label versehen
notes-label-name = Labelnamen eingeben
notes-label-create = „{ $name }“ erstellen
notes-label-remove = Label entfernen
notes-label-delete = Label löschen
notes-labels-none = Noch keine Labels. Fügen Sie über die Label-Schaltfläche einer Notiz eines hinzu.
notes-labels-done = Fertig
notes-label-renamed = Label umbenannt in „{ $name }“
notes-label-deleted = Label „{ $name }“ gelöscht

## A note about a mail

notes-mail = E-Mail
notes-open-mail = Die E-Mail öffnen
notes-open-note = Die Notiz öffnen

## Meeting notes

notes-meeting-take = Besprechungsnotizen erstellen
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = Teilnehmer: { $names }
notes-meeting-notes = Notizen
notes-meeting-actions = Aufgaben
notes-event = Event
notes-open-event = Das Event öffnen

## Formatting

notes-format = Formatierung
notes-format-heading-1 = Überschrift 1
notes-format-heading-2 = Überschrift 2
notes-format-normal = Normaler Text
notes-format-bold = Fett
notes-format-italic = Kursiv
notes-format-underline = Unterstrichen
notes-format-quote = Zitat
notes-format-code = Code
notes-format-divider = Trennlinie
notes-format-clear = Formatierung löschen

## Tasks

notes-make-task = Zur Aufgabe machen

## Colors (tooltips)

notes-color-none = Keine Farbe
notes-color-coral = Koralle
notes-color-peach = Pfirsich
notes-color-sand = Sand
notes-color-mint = Minze
notes-color-sage = Salbei
notes-color-fog = Nebel
notes-color-storm = Gewitter
notes-color-dusk = Abenddämmerung
notes-color-blossom = Blüte
notes-color-clay = Ton
notes-color-chalk = Kreide

## Messages at the foot of the window

notes-archived = Notiz archiviert
notes-unarchived = Notiz aus dem Archiv geholt
notes-trashed = Notiz in den Papierkorb verschoben
notes-restored = Notiz wiederhergestellt
notes-saved = Notiz gespeichert
notes-pinned-count = { $count ->
    [one] Notiz fixiert
   *[other] { $count } Notizen fixiert
}
notes-unpinned-count = { $count ->
    [one] Fixierung aufgehoben
   *[other] Fixierung von { $count } Notizen aufgehoben
}
notes-colored-count = { $count ->
    [one] Farbe geändert
   *[other] Farbe von { $count } Notizen geändert
}
notes-archived-count = { $count ->
    [one] Notiz archiviert
   *[other] { $count } Notizen archiviert
}
notes-unarchived-count = { $count ->
    [one] Notiz aus dem Archiv geholt
   *[other] { $count } Notizen aus dem Archiv geholt
}
notes-trashed-count = { $count ->
    [one] Notiz in den Papierkorb verschoben
   *[other] { $count } Notizen in den Papierkorb verschoben
}
notes-restored-count = { $count ->
    [one] Notiz wiederhergestellt
   *[other] { $count } Notizen wiederhergestellt
}
notes-copied-count = { $count ->
    [one] Kopie erstellt
   *[other] { $count } Kopien erstellt
}
notes-empty-discarded = Leere Notiz verworfen
notes-mail-gone = Diese E-Mail ist nicht mehr vorhanden
notes-deleted-forever = { $count ->
    [one] Notiz endgültig gelöscht
   *[other] { $count } Notizen endgültig gelöscht
}
