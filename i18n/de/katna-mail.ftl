# Katna Mail, German (Deutsch).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = Sprache: { $language }
language-tooltip-system = Sprache: { $language }, wie im System
language-search = Sprache suchen
language-system-default = Systemstandard
language-system-now = Derzeit { $language }
language-no-match = Keine Sprache entspricht „{ $query }“
language-machine = Maschinell übersetzt. Verbesserungen willkommen
language-setting = Sprache
language-setting-detail = Die Sprache von Menüs, Schaltflächen und Meldungen sowie das Format von Datum und Zahlen. „Systemstandard“ übernimmt die Einstellung der Arbeitsumgebung.

## Dates and sizes

ago-just-now = gerade eben
ago-minutes = { $count ->
    [one] vor { $count } Minute
   *[other] vor { $count } Minuten
}
ago-hours = { $count ->
    [one] vor { $count } Stunde
   *[other] vor { $count } Stunden
}
ago-days = { $count ->
    [one] vor { $count } Tag
   *[other] vor { $count } Tagen
}
size-bytes = { $count ->
    [one] { $count } Byte
   *[other] { $count } Byte
}
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = Ordner ausblenden
folders-show = Ordner einblenden
compose = Schreiben
search = Suchen
search-mail = In E-Mails suchen
search-settings = Einstellungen durchsuchen
search-clear = Suche löschen
search-options-show = Suchoptionen anzeigen
settings = Einstellungen
account-add = Konto hinzufügen

## App rail (and the bottom bar on a phone)

rail-mail = E-Mail
rail-calendar = Kalender
rail-contacts = Kontakte
rail-tasks = Aufgaben
rail-notes = Notizen
rail-feeds = Feeds

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Demnächst verfügbar
app-calendar-promise = Ihre CalDAV-Kalender, Einladungen zu Besprechungen aus Ihren E-Mails und Erinnerungen – direkt neben Ihrem Posteingang.
app-tasks-promise = Aufgabenlisten, die über CalDAV synchronisiert werden, und Aufgaben aus E-Mails.
app-notes-promise = Schnelle Notizen und Notizen zu einer E-Mail oder Konversation für später.
app-feeds-promise = RSS- und Atom-Feeds direkt neben Ihren E-Mails lesen.

## Contacts page

app-contacts-loading = Personen aus Ihren E-Mails werden gesammelt…
app-contacts-empty = Personen, mit denen Sie schreiben, werden hier angezeigt.
app-contacts-count = { $count ->
    [one] { $count } Person aus Ihren E-Mails, häufigste Kontakte zuerst
   *[other] { $count } Personen aus Ihren E-Mails, häufigste Kontakte zuerst
}
app-contacts-top = { $count ->
    [one] Top { $count } Person aus Ihren E-Mails, häufigste Kontakte zuerst
   *[other] Top { $count } Personen aus Ihren E-Mails, häufigste Kontakte zuerst
}
app-contacts-messages = { $count ->
    [one] { $count } Nachricht
   *[other] { $count } Nachrichten
}
app-contacts-last = zuletzt { $date }

## Navigation (the folders pane)

nav-labels = Labels
nav-folders = Ordner
nav-label-new = Neues Label erstellen
nav-folder-new = Neuen Ordner erstellen
nav-account-unnamed = Konto { $number }
nav-tab-new = { $count ->
    [one] { $count } neu
   *[other] { $count } neu
}

## Special folders (the user's own folders keep their names)

folder-inbox = Posteingang
folder-starred = Markiert
folder-drafts = Entwürfe
folder-sent = Gesendet
folder-archive = Archiv
folder-spam = Spam
folder-trash = Papierkorb
folder-all-mail = Alle Nachrichten
folder-scheduled = Geplant

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Neues Label
label-folder-new-title = Neuer Ordner
label-prompt = Bitte geben Sie einen neuen Labelnamen ein:
label-folder-prompt = Bitte geben Sie einen neuen Ordnernamen ein:
label-name-hint = Labelname
label-folder-name-hint = Ordnername
label-nest = Label verschachteln unter:
label-folder-nest = Ordner verschachteln unter:
label-cancel = Abbrechen
label-create = Erstellen
label-creating = Wird erstellt…
label-created = Label „{ $name }“ wurde erstellt.
label-folder-created = Ordner „{ $name }“ wurde erstellt.

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
tab-new = { $count } neu
tab-provider-other = von Katna sortiert

## Mail list: toolbar

list-select = Auswählen
list-refresh = Aktualisieren
list-more = Mehr
list-mark-read = Als gelesen markieren
list-mark-unread = Als ungelesen markieren
list-move-to = Verschieben nach
list-archive = Archivieren
list-spam = Spam melden
list-delete = Löschen
list-newer = Neuer
list-older = Älter
list-range = { $first }–{ $last } von { $total }
list-range-about = { $first }–{ $last } von ungefähr { $total }
list-results = Ergebnisse für „{ $query }“
list-results-corrected = Ergebnisse für „{ $query }“ werden angezeigt
list-search-instead = Stattdessen nach „{ $query }“ suchen
list-files-more = +{ $count }

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
list-clear-selection = Auswahl aufheben

## Mail list: empty states

list-empty-search = Keine Nachrichten entsprechen Ihrer Suche.
list-empty-tab = Keine E-Mails in { $tab }.
list-empty-tab-unknown = Keine E-Mails in diesem Tab.
list-empty-folder = Keine Nachrichten in { $folder }.
list-empty-folder-unknown = Keine Nachrichten in diesem Ordner.
list-first-sync = Ihre E-Mails werden abgerufen…
list-first-sync-detail = Sie werden hier angezeigt, sobald sie eintreffen.

## Mail list: lines

row-removed = Diese Nachricht wurde entfernt.
row-starred = Markiert
row-not-starred = Nicht markiert
row-important = Wichtig. Klicken, um als nicht wichtig zu markieren.
row-mark-important = Als wichtig markieren
row-pinned = Oben angeheftet
row-pin = Oben anheften
row-unpin = Nicht mehr anheften

## Mail list: More menu and right-click menu

menu-reply = Antworten
menu-reply-all = Allen antworten
menu-forward = Weiterleiten
menu-archive = Archivieren
menu-delete = Löschen
menu-spam = Spam melden
menu-mark-read = Als gelesen markieren
menu-mark-unread = Als ungelesen markieren
menu-mark-all-read = Alle als gelesen markieren
menu-star = Markierung hinzufügen
menu-unstar = Markierung entfernen
menu-important = Als wichtig markieren
menu-not-important = Als nicht wichtig markieren
menu-pin = Oben anheften
menu-unpin = Nicht mehr anheften
menu-print-all = Alle drucken
menu-new-window = In neuem Fenster öffnen
menu-move-to = Verschieben nach
menu-move-to-heading = Verschieben nach:
menu-find-from = E-Mails von { $name } suchen

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
toast-undone = Aktion rückgängig gemacht.
toast-undo = Rückgängig
toast-no-spam-folder = Dieses Konto hat keinen Spam-Ordner.

## Reading pane: toolbar

reader-close = Schließen
reader-back = Zurück
reader-mark-unread = Als ungelesen markieren
reader-move-to = Verschieben nach
reader-more = Mehr
reader-print-all = Alle drucken
reader-new-window = In neuem Fenster
reader-position = { $position } von { $total }
reader-newer = Neuer
reader-older = Älter

## Reading pane: the conversation

reader-removed = Diese Konversation wurde entfernt.
reader-no-subject = (kein Betreff)
reader-collapse-all = Alle einklappen
reader-expand-all = Alle ausklappen
reader-unknown-sender = (unbekannter Absender)
reader-date-ago = { $date } ({ $ago })
reader-me = mich
reader-to = an { $names }
reader-starred = Markiert
reader-not-starred = Nicht markiert
reader-too-long = Die Nachricht ist zu lang, um vollständig angezeigt zu werden.
reader-encrypted-images = In verschlüsselten E-Mails werden Bilder aus dem Web nie geladen.
reader-window-failed = Neues Fenster konnte nicht geöffnet werden.

## Reading pane: message details (opened from "to me")

reader-details-from = von:
reader-details-to = an:
reader-details-cc = Cc:
reader-details-date = Datum:
reader-details-subject = Betreff:

## Reading pane: downloading a message

reader-downloading = Diese Nachricht wird vom Server heruntergeladen…
reader-download-failed = Diese Nachricht konnte nicht heruntergeladen werden.
reader-try-again = Erneut versuchen

## Reply row

reply-reply = Antworten
reply-reply-all = Allen antworten
reply-forward = Weiterleiten

## Encrypted and signed mail

security-decrypting = Wird entschlüsselt…
security-checking = Signatur wird geprüft…
security-partly-encrypted = Nur ein Teil dieser Nachricht ist verschlüsselt. Der Rest wurde außerhalb des Schutzes hinzugefügt und könnte von jedem stammen.
security-partly-signed = Nur ein Teil dieser Nachricht ist signiert. Der Rest wurde außerhalb des Schutzes hinzugefügt und könnte von jedem stammen.
security-encrypted = Verschlüsselte Nachricht
security-encrypted-smime = Verschlüsselte Nachricht (S/MIME)
security-no-key = Diese Nachricht kann nicht entschlüsselt werden: Sie wurde für einen Schlüssel verschlüsselt, den Sie nicht haben.
security-cancelled = Die Entschlüsselung wurde abgebrochen.
security-damaged = Diese Nachricht kann nicht entschlüsselt werden: Die verschlüsselten Daten sind beschädigt oder wurden verändert.
security-decrypt-unavailable = Diese Nachricht kann nicht entschlüsselt werden: Installieren Sie { $tool }, um verschlüsselte E-Mails zu lesen.
security-decrypt-failed = Diese Nachricht kann nicht entschlüsselt werden: { $reason }
security-unknown-signer = einem unbekannten Unterzeichner
security-signed-verified = Signiert von { $signer } · verifiziert
security-signed-not-sender = Signiert von { $signer } – nicht vom Absender
security-signed-untrusted = Signiert von { $signer } mit einem Schlüssel, den Sie als nicht vertrauenswürdig markiert haben
security-signed-unverified = Signiert von { $signer } · der Schlüssel ist nicht verifiziert
security-bad-signature = Ungültige Signatur: Diese Nachricht wurde nach dem Signieren verändert oder die Signatur ist gefälscht.
security-signature-expired = Signiert von { $signer } · die Signatur ist abgelaufen
security-key-expired = Signiert von { $signer } · der Schlüssel ist inzwischen abgelaufen
security-key-revoked = Signiert von { $signer } mit einem widerrufenen Schlüssel
security-missing-key = Mit einem Schlüssel signiert, den Sie nicht haben, daher nicht prüfbar
security-missing-key-id = Mit einem Schlüssel signiert, den Sie nicht haben ({ $key }), daher nicht prüfbar
security-signature-unavailable = Signiert; installieren Sie { $tool }, um die Signatur zu prüfen
security-signature-error = Die Signatur konnte nicht geprüft werden.

## Remote images and pictures

remote-hidden = Bilder in dieser Nachricht sind ausgeblendet.
remote-show = Bilder anzeigen
remote-always-show = Von diesem Absender immer anzeigen
remote-picture-use = Verwenden
remote-picture-too-big = Wählen Sie ein Bild mit höchstens 8 MB.
remote-picture-type = Wählen Sie ein PNG-, JPEG-, GIF-, WebP- oder SVG-Bild.
remote-picture-read-failed = Das Bild kann nicht gelesen werden: { $error }
remote-picture-keep-failed = Das Bild kann nicht gespeichert werden: { $error }
remote-picture-remove-failed = Das Bild kann nicht entfernt werden: { $error }

## Attachments

attachment-count = { $count ->
    [one] Ein Anhang
   *[other] { $count } Anhänge
}
attachment-save = Speichern
attachment-save-all = Alle speichern
attachment-save-all-tooltip = Alle Anhänge in einem Ordner speichern
attachment-save-here = Hier speichern
attachment-not-downloaded = Diese Nachricht ist nicht heruntergeladen.
attachment-not-found = Dieser Anhang wurde in der Nachricht nicht gefunden.
attachment-read-failed = { $name } konnte nicht gelesen werden
attachment-numbered = Anhang { $number }
attachment-saved-all = { $count ->
    [one] { $count } Datei in { $place } gespeichert
   *[other] { $count } Dateien in { $place } gespeichert
}
attachment-saved-some = { $total ->
    [one] { $saved } von { $total } Datei in { $place } gespeichert. { $failed } konnte nicht gespeichert werden
   *[other] { $saved } von { $total } Dateien in { $place } gespeichert. { $failed } konnte nicht gespeichert werden
}
attachment-saved-to = Gespeichert in { $path }
attachment-save-failed = { $name } konnte nicht gespeichert werden: { $error }
attachment-open-failed = { $name } konnte nicht geöffnet werden: { $error }
attachment-risky = Diese Datei könnte ein Programm ausführen, daher öffnet Katna sie nicht. Speichern Sie sie stattdessen.
attachment-encrypted-open = Diese Datei wurde verschlüsselt empfangen. Speichern Sie sie, um sie anderswo zu öffnen.

## Printing

print-failed = Drucken nicht möglich: { $error }
print-no-font = keine Schriftart gefunden
print-opened-as-pdf = Als PDF geöffnet, um von dort aus zu drucken.
print-not-downloaded = (Noch nicht heruntergeladen.)
print-encrypted = (Verschlüsselt. Öffnen Sie die Nachricht in Katna Mail, um ihren Text zu drucken.)
print-to = An: { $addresses }
print-cc = Cc: { $addresses }
