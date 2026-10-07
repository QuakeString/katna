# Katna Mail, German (Deutsch).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Allgemein
tab-promotions = Werbung
tab-social = Soziale Netzwerke
tab-updates = Benachrichtigungen
tab-forums = Foren
tab-focused = Relevant
tab-other = Sonstige
tab-inbox = Posteingang
tab-newsletters = Newsletter
tab-notifications = Benachrichtigungen
tab-provider-other = von Katna sortiert

## Mail list: toolbar

list-select = Auswählen
list-refresh = Aktualisieren
list-back-to-top = Nach oben
list-checking = Auf neue E-Mails wird geprüft…
list-more = Mehr
list-mark-read = Als gelesen markieren
list-mark-unread = Als ungelesen markieren
list-move-to = Verschieben nach
list-archive = Archivieren
list-spam = Spam melden
list-delete = Löschen
list-snooze = Zurückstellen
list-unsnooze = Nicht mehr zurückstellen
list-newer = Neuer
list-older = Älter
list-range = { $first }–{ $last } von { $total }
list-range-about = { $first }–{ $last } von ungefähr { $total }
list-results = Ergebnisse für „{ $query }“
list-results-corrected = Ergebnisse für „{ $query }“ werden angezeigt
list-search-instead = Stattdessen nach „{ $query }“ suchen
list-files-more = +{ $count }
list-replied = Sie haben geantwortet

## Mail list: Select menu (which lines to tick)

list-pick-all = Alle
list-pick-none = Keine
list-pick-read = Gelesen
list-pick-unread = Ungelesen
list-pick-starred = Markiert
list-pick-unstarred = Nicht markiert

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] Die { $count } Konversation ist ausgewählt.
       *[other] Alle { $count } Konversationen sind ausgewählt.
    }
   *[message] { $count ->
        [one] Die { $count } Nachricht ist ausgewählt.
       *[other] Alle { $count } Nachrichten sind ausgewählt.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] Die { $count } Konversation in { $folder } ist ausgewählt.
       *[other] Alle { $count } Konversationen in { $folder } sind ausgewählt.
    }
   *[message] { $count ->
        [one] Die { $count } Nachricht in { $folder } ist ausgewählt.
       *[other] Alle { $count } Nachrichten in { $folder } sind ausgewählt.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] Die { $count } Konversation auf dieser Seite ist ausgewählt.
       *[other] Alle { $count } Konversationen auf dieser Seite sind ausgewählt.
    }
   *[message] { $count ->
        [one] Die { $count } Nachricht auf dieser Seite ist ausgewählt.
       *[other] Alle { $count } Nachrichten auf dieser Seite sind ausgewählt.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] Die { $count } Konversation auswählen
       *[other] Alle { $count } Konversationen auswählen
    }
   *[message] { $count ->
        [one] Die { $count } Nachricht auswählen
       *[other] Alle { $count } Nachrichten auswählen
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] Die { $count } Konversation in { $folder } auswählen
       *[other] Alle { $count } Konversationen in { $folder } auswählen
    }
   *[message] { $count ->
        [one] Die { $count } Nachricht in { $folder } auswählen
       *[other] Alle { $count } Nachrichten in { $folder } auswählen
    }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Die { $count } gelesene Konversation auf dieser Seite ist ausgewählt.
           *[other] Alle { $count } gelesenen Konversationen auf dieser Seite sind ausgewählt.
        }
       *[message] { $count ->
            [one] Die { $count } gelesene Nachricht auf dieser Seite ist ausgewählt.
           *[other] Alle { $count } gelesenen Nachrichten auf dieser Seite sind ausgewählt.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Die { $count } ungelesene Konversation auf dieser Seite ist ausgewählt.
           *[other] Alle { $count } ungelesenen Konversationen auf dieser Seite sind ausgewählt.
        }
       *[message] { $count ->
            [one] Die { $count } ungelesene Nachricht auf dieser Seite ist ausgewählt.
           *[other] Alle { $count } ungelesenen Nachrichten auf dieser Seite sind ausgewählt.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Die { $count } markierte Konversation auf dieser Seite ist ausgewählt.
           *[other] Alle { $count } markierten Konversationen auf dieser Seite sind ausgewählt.
        }
       *[message] { $count ->
            [one] Die { $count } markierte Nachricht auf dieser Seite ist ausgewählt.
           *[other] Alle { $count } markierten Nachrichten auf dieser Seite sind ausgewählt.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Die { $count } nicht markierte Konversation auf dieser Seite ist ausgewählt.
           *[other] Alle { $count } nicht markierten Konversationen auf dieser Seite sind ausgewählt.
        }
       *[message] { $count ->
            [one] Die { $count } nicht markierte Nachricht auf dieser Seite ist ausgewählt.
           *[other] Alle { $count } nicht markierten Nachrichten auf dieser Seite sind ausgewählt.
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Die { $count } gelesene Konversation auswählen
           *[other] Alle { $count } gelesenen Konversationen auswählen
        }
       *[message] { $count ->
            [one] Die { $count } gelesene Nachricht auswählen
           *[other] Alle { $count } gelesenen Nachrichten auswählen
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Die { $count } ungelesene Konversation auswählen
           *[other] Alle { $count } ungelesenen Konversationen auswählen
        }
       *[message] { $count ->
            [one] Die { $count } ungelesene Nachricht auswählen
           *[other] Alle { $count } ungelesenen Nachrichten auswählen
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Die { $count } markierte Konversation auswählen
           *[other] Alle { $count } markierten Konversationen auswählen
        }
       *[message] { $count ->
            [one] Die { $count } markierte Nachricht auswählen
           *[other] Alle { $count } markierten Nachrichten auswählen
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Die { $count } nicht markierte Konversation auswählen
           *[other] Alle { $count } nicht markierten Konversationen auswählen
        }
       *[message] { $count ->
            [one] Die { $count } nicht markierte Nachricht auswählen
           *[other] Alle { $count } nicht markierten Nachrichten auswählen
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Die { $count } gelesene Konversation in { $folder } auswählen
           *[other] Alle { $count } gelesenen Konversationen in { $folder } auswählen
        }
       *[message] { $count ->
            [one] Die { $count } gelesene Nachricht in { $folder } auswählen
           *[other] Alle { $count } gelesenen Nachrichten in { $folder } auswählen
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Die { $count } ungelesene Konversation in { $folder } auswählen
           *[other] Alle { $count } ungelesenen Konversationen in { $folder } auswählen
        }
       *[message] { $count ->
            [one] Die { $count } ungelesene Nachricht in { $folder } auswählen
           *[other] Alle { $count } ungelesenen Nachrichten in { $folder } auswählen
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Die { $count } markierte Konversation in { $folder } auswählen
           *[other] Alle { $count } markierten Konversationen in { $folder } auswählen
        }
       *[message] { $count ->
            [one] Die { $count } markierte Nachricht in { $folder } auswählen
           *[other] Alle { $count } markierten Nachrichten in { $folder } auswählen
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Die { $count } nicht markierte Konversation in { $folder } auswählen
           *[other] Alle { $count } nicht markierten Konversationen in { $folder } auswählen
        }
       *[message] { $count ->
            [one] Die { $count } nicht markierte Nachricht in { $folder } auswählen
           *[other] Alle { $count } nicht markierten Nachrichten in { $folder } auswählen
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } gelesene Konversation ist ausgewählt.
           *[other] Alle { $count } gelesenen Konversationen sind ausgewählt.
        }
       *[message] { $count ->
            [one] { $count } gelesene Nachricht ist ausgewählt.
           *[other] Alle { $count } gelesenen Nachrichten sind ausgewählt.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } ungelesene Konversation ist ausgewählt.
           *[other] Alle { $count } ungelesenen Konversationen sind ausgewählt.
        }
       *[message] { $count ->
            [one] { $count } ungelesene Nachricht ist ausgewählt.
           *[other] Alle { $count } ungelesenen Nachrichten sind ausgewählt.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } markierte Konversation ist ausgewählt.
           *[other] Alle { $count } markierten Konversationen sind ausgewählt.
        }
       *[message] { $count ->
            [one] { $count } markierte Nachricht ist ausgewählt.
           *[other] Alle { $count } markierten Nachrichten sind ausgewählt.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } nicht markierte Konversation ist ausgewählt.
           *[other] Alle { $count } nicht markierten Konversationen sind ausgewählt.
        }
       *[message] { $count ->
            [one] { $count } nicht markierte Nachricht ist ausgewählt.
           *[other] Alle { $count } nicht markierten Nachrichten sind ausgewählt.
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } gelesene Konversation in { $folder } ist ausgewählt.
           *[other] Alle { $count } gelesenen Konversationen in { $folder } sind ausgewählt.
        }
       *[message] { $count ->
            [one] { $count } gelesene Nachricht in { $folder } ist ausgewählt.
           *[other] Alle { $count } gelesenen Nachrichten in { $folder } sind ausgewählt.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } ungelesene Konversation in { $folder } ist ausgewählt.
           *[other] Alle { $count } ungelesenen Konversationen in { $folder } sind ausgewählt.
        }
       *[message] { $count ->
            [one] { $count } ungelesene Nachricht in { $folder } ist ausgewählt.
           *[other] Alle { $count } ungelesenen Nachrichten in { $folder } sind ausgewählt.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } markierte Konversation in { $folder } ist ausgewählt.
           *[other] Alle { $count } markierten Konversationen in { $folder } sind ausgewählt.
        }
       *[message] { $count ->
            [one] { $count } markierte Nachricht in { $folder } ist ausgewählt.
           *[other] Alle { $count } markierten Nachrichten in { $folder } sind ausgewählt.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } nicht markierte Konversation in { $folder } ist ausgewählt.
           *[other] Alle { $count } nicht markierten Konversationen in { $folder } sind ausgewählt.
        }
       *[message] { $count ->
            [one] { $count } nicht markierte Nachricht in { $folder } ist ausgewählt.
           *[other] Alle { $count } nicht markierten Nachrichten in { $folder } sind ausgewählt.
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] Keine gelesenen Konversationen hier.
       *[message] Keine gelesenen Nachrichten hier.
    }
   *[unread] { $kind ->
        [conversation] Keine ungelesenen Konversationen hier.
       *[message] Keine ungelesenen Nachrichten hier.
    }
    [starred] { $kind ->
        [conversation] Keine markierten Konversationen hier.
       *[message] Keine markierten Nachrichten hier.
    }
    [unstarred] { $kind ->
        [conversation] Keine nicht markierten Konversationen hier.
       *[message] Keine nicht markierten Nachrichten hier.
    }
}
list-clear-selection = Auswahl aufheben

## Mail list: empty states

list-empty-search = Keine Nachrichten entsprechen Ihrer Suche.
list-empty-tab = Keine E-Mails in { $tab }.
list-empty-tab-unknown = Keine E-Mails in diesem Tab.
list-empty-folder = Keine Nachrichten in { $folder }.
list-empty-folder-unknown = Keine Nachrichten in diesem Ordner.
list-empty-waiting = Nichts wartet auf eine Antwort.
list-empty-reminders = Keine Erinnerungen. Drücken Sie H bei einer E-Mail, um eine hinzuzufügen.
list-first-sync = Ihre E-Mails werden abgerufen…
list-first-sync-detail = Sie werden hier angezeigt, sobald sie eintreffen.

## Mail list: lines

row-removed = Diese Nachricht wurde entfernt.
row-starred = Markiert
row-not-starred = Nicht markiert
row-important = Wichtig. Klicken, um als nicht wichtig zu markieren.
row-mark-important = Als wichtig markieren
row-pinned = Oben angeheftet
row-task = Aufgabe
row-task-open = Aufgabe öffnen: { $title }
row-tracking-none = Verfolgt. Noch nicht geöffnet
row-tracking-opened = Geöffnet: { $opened } von { $recipients }
row-tracking-clicked = Geöffnet: { $opened } von { $recipients }, Link aufgerufen: { $clicked }
row-pin = Oben anheften
row-unpin = Nicht mehr anheften
row-snoozed-until = Zurückgestellt bis { $when }
row-snoozed-day-time = { $day } { $time }
snoozed-group-today = Heute
snoozed-group-tomorrow = Morgen
snoozed-group-this-week = Diese Woche
snoozed-group-later = Später
row-follow-up-step = Nachfass-E-Mail { $step } von { $steps } · { $date }
row-follow-up-waiting = Nachfass-E-Mail wartet
row-reminder = Erinnerung { $date }

## Mail list: More menu and right-click menu

menu-reply = Antworten
menu-reply-all = Allen antworten
menu-forward = Weiterleiten
menu-archive = Archivieren
menu-delete = Löschen
menu-delete-forever = Endgültig löschen
menu-move-to-inbox = In den Posteingang verschieben
menu-spam = Spam melden
menu-not-spam = Kein Spam
menu-mark-read = Als gelesen markieren
menu-mark-unread = Als ungelesen markieren
menu-mark-all-read = Alle als gelesen markieren
menu-star = Markierung hinzufügen
menu-unstar = Markierung entfernen
menu-important = Als wichtig markieren
menu-not-important = Als nicht wichtig markieren
menu-pin = Oben anheften
menu-unpin = Nicht mehr anheften
menu-snooze = Zurückstellen
menu-remind = Erinnern
menu-unsnooze = Nicht mehr zurückstellen
menu-add-to-tasks = Zu Aufgaben hinzufügen
menu-schedule-meeting = Besprechung planen
menu-start-call = Videoanruf starten
menu-add-note = Notiz hinzufügen
menu-print-all = Alle drucken
menu-new-window = In neuem Fenster öffnen
menu-move-to = Verschieben nach
# Opens a submenu: Add to Tasks, Add a note, Schedule a meeting and Start a
# video call.
menu-follow-up = Nachverfolgen
# Opens a submenu of the rarer actions: Report spam, Mark as important and
# Pin to top.
menu-more = Mehr
menu-move-to-heading = Verschieben nach:
menu-move-to-search = Verschieben nach…
menu-label-as = Label zuweisen
menu-label-as-search = Label zuweisen…
menu-no-folder = Kein Ordner namens „{ $name }“
menu-no-label = Kein Label namens „{ $name }“
menu-create-folder = „{ $name }“ erstellen
menu-always-move = E-Mails von { $name } immer hierher verschieben
toast-always-move-failed = Die E-Mail wurde verschoben, aber die Regel wurde nicht erstellt: { $error }
drag-mail = { $kind ->
    [conversation] { $count ->
        [one] { $count } Konversation
       *[other] { $count } Konversationen
    }
   *[message] { $count ->
        [one] { $count } Nachricht
       *[other] { $count } Nachrichten
    }
}
menu-find-from = E-Mails von { $name } suchen
menu-make-rule = Regel erstellen…

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] Konversation archiviert.
       *[other] { $count } Konversationen archiviert.
    }
   *[message] { $count ->
        [one] Nachricht archiviert.
       *[other] { $count } Nachrichten archiviert.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] Konversation in den Papierkorb verschoben.
       *[other] { $count } Konversationen in den Papierkorb verschoben.
    }
   *[message] { $count ->
        [one] Nachricht in den Papierkorb verschoben.
       *[other] { $count } Nachrichten in den Papierkorb verschoben.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] Konversation verschoben.
       *[other] { $count } Konversationen verschoben.
    }
   *[message] { $count ->
        [one] Nachricht verschoben.
       *[other] { $count } Nachrichten verschoben.
    }
}
toast-label-added = Label „{ $label }“ hinzugefügt.
toast-label-removed = Label „{ $label }“ entfernt.
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] Konversation markiert.
       *[other] { $count } Konversationen markiert.
    }
   *[message] { $count ->
        [one] Nachricht markiert.
       *[other] { $count } Nachrichten markiert.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] Markierung der Konversation entfernt.
       *[other] Markierung von { $count } Konversationen entfernt.
    }
   *[message] { $count ->
        [one] Markierung der Nachricht entfernt.
       *[other] Markierung von { $count } Nachrichten entfernt.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] Konversation als wichtig markiert.
       *[other] { $count } Konversationen als wichtig markiert.
    }
   *[message] { $count ->
        [one] Nachricht als wichtig markiert.
       *[other] { $count } Nachrichten als wichtig markiert.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] Konversation als nicht wichtig markiert.
       *[other] { $count } Konversationen als nicht wichtig markiert.
    }
   *[message] { $count ->
        [one] Nachricht als nicht wichtig markiert.
       *[other] { $count } Nachrichten als nicht wichtig markiert.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] Konversation oben angeheftet.
       *[other] { $count } Konversationen oben angeheftet.
    }
   *[message] { $count ->
        [one] Nachricht oben angeheftet.
       *[other] { $count } Nachrichten oben angeheftet.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] Konversation nicht mehr angeheftet.
       *[other] { $count } Konversationen nicht mehr angeheftet.
    }
   *[message] { $count ->
        [one] Nachricht nicht mehr angeheftet.
       *[other] { $count } Nachrichten nicht mehr angeheftet.
    }
}
toast-snoozed = { $kind ->
    [conversation] { $count ->
        [one] Konversation bis { $when } zurückgestellt.
       *[other] { $count } Konversationen bis { $when } zurückgestellt.
    }
   *[message] { $count ->
        [one] Nachricht bis { $when } zurückgestellt.
       *[other] { $count } Nachrichten bis { $when } zurückgestellt.
    }
}
toast-unsnoozed = { $kind ->
    [conversation] { $count ->
        [one] Konversation wieder im Posteingang.
       *[other] { $count } Konversationen wieder im Posteingang.
    }
   *[message] { $count ->
        [one] Nachricht wieder im Posteingang.
       *[other] { $count } Nachrichten wieder im Posteingang.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] Konversation als Spam gemeldet.
       *[other] { $count } Konversationen als Spam gemeldet.
    }
   *[message] { $count ->
        [one] Nachricht als Spam gemeldet.
       *[other] { $count } Nachrichten als Spam gemeldet.
    }
}
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] Konversation als kein Spam markiert und in den Posteingang verschoben.
       *[other] { $count } Konversationen als kein Spam markiert und in den Posteingang verschoben.
    }
   *[message] { $count ->
        [one] Nachricht als kein Spam markiert und in den Posteingang verschoben.
       *[other] { $count } Nachrichten als kein Spam markiert und in den Posteingang verschoben.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] Konversation endgültig gelöscht.
       *[other] { $count } Konversationen endgültig gelöscht.
    }
   *[message] { $count ->
        [one] Nachricht endgültig gelöscht.
       *[other] { $count } Nachrichten endgültig gelöscht.
    }
}
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] Konversation als gelesen markiert.
       *[other] { $count } Konversationen als gelesen markiert.
    }
   *[message] { $count ->
        [one] Nachricht als gelesen markiert.
       *[other] { $count } Nachrichten als gelesen markiert.
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] Konversation als ungelesen markiert.
       *[other] { $count } Konversationen als ungelesen markiert.
    }
   *[message] { $count ->
        [one] Nachricht als ungelesen markiert.
       *[other] { $count } Nachrichten als ungelesen markiert.
    }
}
toast-undone = Aktion rückgängig gemacht.
toast-nothing-to-undo = Nichts rückgängig zu machen.
toast-cannot-undo-delete-forever = Endgültig gelöschte E-Mails lassen sich nicht wiederherstellen.
toast-send-undone = Senden rückgängig gemacht.
toast-too-late-to-undo-send = Zu spät zum Rückgängigmachen: Die Nachricht wurde bereits gesendet.
toast-undo = Rückgängig
toast-close = Schließen
toast-no-spam-folder = Dieses Konto hat keinen Spam-Ordner.
