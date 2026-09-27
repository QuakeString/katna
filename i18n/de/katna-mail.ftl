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

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Öffnen Sie diese Nachricht, um ihre Anhänge zu lesen.
text-copy = Kopieren
text-select-all = Alles auswählen

## Settings page: its tabs

settings-tab-general = Allgemein
settings-tab-inbox = Posteingang
settings-tab-accounts = Konten
settings-tab-subscriptions = Abonnements
settings-tab-appearance = Darstellung
settings-tab-shortcuts = Tastenkombinationen
settings-tab-default-apps = Standard-Apps
settings-tab-folders-rules = Ordner und Regeln
settings-tab-compose = Verfassen
settings-tab-mcp-server = MCP-Server
settings-tab-feedback = Nutzerfeedback
settings-tab-experimental = Experimentell

## Settings page: tabs still to come

settings-tab-subscriptions-coming = Sehen Sie, welche Newsletter und Mailinglisten Sie erhalten, und kündigen Sie sie mit einem Klick.
settings-tab-folders-rules-coming = Ordner und Labels erstellen, umbenennen, verschieben und ausblenden und festlegen, welche synchronisiert werden. Regeln sortieren neue E-Mails automatisch nach Absender, Betreff oder Wörtern, versehen sie mit Labels, leiten sie weiter oder löschen sie.
settings-tab-mcp-server-coming = KI-Assistenten auf diesem Computer dürfen Ihre E-Mails durchsuchen, lesen und Entwürfe verfassen – mit Ihrer Zustimmung.

## Settings > General

settings-general-conversations = Konversationsansicht
settings-general-conversations-group = Antworten auf dieselbe E-Mail gruppieren
settings-general-conversations-group-detail = Eine Zeile pro Konversation in der Liste
settings-general-reading = Lesen
settings-general-newest-first = Neueste Nachricht zuerst
settings-general-newest-first-detail = Eine Konversation beginnt mit der letzten Antwort
settings-general-full-headers = Vollständige Kopfzeilen anzeigen
settings-general-full-headers-detail = Von, An, Cc, Datum und Betreff bei jeder Nachricht aufgeklappt
settings-general-full-names = Vollständige Namen der Empfänger
settings-general-full-names-detail = „an mich, Ada Lovelace“ statt „an mich, Ada“
settings-general-mark-read = Als gelesen markieren
settings-general-mark-read-now = Sobald sie geöffnet wird
settings-general-mark-read-1s = Wenn sie 1 Sekunde lang geöffnet ist
settings-general-mark-read-3s = Wenn sie 3 Sekunden lang geöffnet ist
settings-general-mark-read-never = Nur wenn ich sie als gelesen markiere
settings-general-reply-button = Antwortschaltfläche
settings-general-reply-all = Allen antworten
settings-general-reply-all-detail = Die Antwortschaltfläche neben jeder Nachricht antwortet allen, nicht nur dem Absender
settings-general-remote-images = Bilder aus dem Web
settings-general-remote-images-detail = Wenn die Bilder einer Nachricht geladen werden, erfährt der Absender, dass Sie sie geöffnet haben, wann und ungefähr wo. Ist dies aus, fragt jede Nachricht zuerst, und Sie können die Bilder eines Absenders jederzeit anzeigen.
settings-general-remote-images-always = Bilder immer anzeigen
settings-general-remote-images-always-detail = In jeder Nachricht, nicht nur von vertrauenswürdigen Absendern
settings-general-sending = Senden
settings-general-sending-detail = Wie lange eine gesendete Nachricht wartet, damit sie zurückgenommen werden kann.
settings-general-offline = Offline-E-Mails
settings-general-offline-detail = Aktuelle E-Mails werden vollständig heruntergeladen, damit Sie sie ohne Verbindung lesen können. Ältere E-Mails werden beim Öffnen heruntergeladen.
settings-general-offline-days = { $count ->
    [one] { $count } Tag
   *[other] { $count } Tage
}
settings-general-offline-years = { $count ->
    [one] { $count } Jahr
   *[other] { $count } Jahre
}
settings-general-offline-all = Alle E-Mails
settings-general-offline-note = Bei weniger Tagen bleiben bereits heruntergeladene E-Mails erhalten. Auf dem Server ändert sich nichts.
settings-general-notifications = Benachrichtigungen
settings-general-notifications-detail = Für neue E-Mails im Posteingang, auch wenn Katna Mail geschlossen ist.
settings-general-new-mail = Bei neuen E-Mails benachrichtigen
settings-general-new-mail-detail = Mit „Allen antworten“, „Als gelesen markieren“ und „Archivieren“
settings-general-new-mail-sound = Ton abspielen
settings-general-new-mail-sound-detail = Der Ton der Arbeitsumgebung für neue E-Mails
settings-general-desktop = Arbeitsumgebung
settings-general-open-at-login = Katna Mail bei der Anmeldung öffnen
settings-general-open-at-login-detail = E-Mails werden bei der Anmeldung ohnehin synchronisiert, solange der Dienst läuft
settings-general-tray = Katna im Systemabschnitt anzeigen
settings-general-tray-detail = Mit der Anzahl ungelesener Nachrichten und einem Menü
settings-general-unread-badge = Anzahl ungelesener Nachrichten am Symbol in der Kontrollleiste
settings-general-unread-badge-detail = Wie viele Nachrichten im Posteingang ungelesen sind

## Settings > Inbox

settings-inbox-tabs = Posteingangs-Tabs
settings-inbox-tabs-detail = Sortieren Sie den Posteingang in Tabs, wie es die Website Ihres E-Mail-Anbieters tut.
settings-inbox-tabs-show = Posteingangs-Tabs anzeigen
settings-inbox-tabs-show-detail = Wenn aus, eine Liste für alle Konten
settings-inbox-no-accounts = Fügen Sie ein Konto hinzu, um seine Tabs auszuwählen.
settings-inbox-tabs-automatic = Automatisch: { $tabs } ({ $provider })
settings-inbox-tabs-off = Keine Tabs
settings-inbox-tabs-gmail = Allgemein, Werbung, Soziale Netzwerke, Benachrichtigungen, Foren
settings-inbox-tabs-focused = Relevant und Sonstige
settings-inbox-tabs-zoho = Posteingang, Newsletter und Benachrichtigungen
settings-inbox-tabs-shown = Angezeigte Tabs. E-Mails eines ausgeschalteten Tabs bleiben in { $tab }.

## Settings > Appearance

settings-appearance-reading-pane = Lesebereich
settings-appearance-reading-pane-detail = Wo eine geöffnete Konversation angezeigt wird.
settings-appearance-pane-right = Rechts neben der Liste
settings-appearance-pane-none = Keine Unterteilung
settings-appearance-density = Dichte
settings-appearance-density-default = Standard
settings-appearance-density-compact = Kompakt
settings-appearance-scaling = Skalierung
settings-appearance-scaling-detail = Macht alles in Katna Mail größer oder kleiner, zusätzlich zur Skalierung der Arbeitsumgebung: Text, Symbole, Abstände und Trennlinien. Gesendete E-Mails behalten ihre eigene Schriftgröße. Bei sehr kleinen Größen lassen sich Symbole schwer anklicken.
settings-appearance-theme = Design
settings-appearance-theme-system = Wie die Arbeitsumgebung
settings-appearance-theme-light = Hell
settings-appearance-theme-dark = Dunkel
settings-appearance-desktop-colors = Farben der Arbeitsumgebung
settings-appearance-desktop-colors-use = Farben der Arbeitsumgebung verwenden
settings-appearance-desktop-colors-use-detail = Das Farbschema und die Akzentfarbe der Arbeitsumgebung
settings-appearance-app-names = App-Namen
settings-appearance-app-names-show = App-Namen anzeigen
settings-appearance-app-names-show-detail = Namen unter den App-Symbolen ganz links
settings-appearance-sender-pictures = Absenderbilder
settings-appearance-sender-pictures-show = Firmenlogos anzeigen
settings-appearance-sender-pictures-show-detail = Anhand der Domain des Absenders gesucht, nie anhand der Nachricht, und eine Woche lang gespeichert
settings-appearance-important = Wichtig-Markierungen
settings-appearance-important-show = Wichtig-Markierungen anzeigen
settings-appearance-important-show-detail = Neben jeder Nachricht in der Liste
settings-appearance-message-width = Nachrichtenbreite
settings-appearance-message-width-limit = Breite von Nachrichten begrenzen
settings-appearance-message-width-limit-detail = Lange Zeilen sind in einem breiten Fenster leichter zu lesen
settings-appearance-mail-colors = E-Mail-Farben
settings-appearance-mail-colors-detail = Die meisten E-Mails sind für eine weiße Seite gestaltet. Bei dunklem Design werden ihre Farben in gut lesbare dunkle Farben geändert; ist dies aus, behalten sie die Farben des Absenders auf einer hellen Seite.
settings-appearance-dark-mail = Dunkle Farben auch für E-Mails
settings-appearance-dark-mail-detail = Nur bei dunklem Design
settings-appearance-attachment-previews = Anhangvorschau
settings-appearance-attachment-previews-show = Vorschau von Anhängen anzeigen
settings-appearance-attachment-previews-show-detail = Ein kleines Bild vom Inhalt jeder Datei auf ihrer Karte

## Settings > Default apps

settings-default-apps-intro = Womit Anhänge beim Anklicken geöffnet werden. Der Betrachter kann eine Datei jederzeit auch in einer anderen App öffnen. Die Standard-Apps der Arbeitsumgebung legen Sie in deren eigenen Einstellungen fest.
settings-default-apps-pdf = PDF-Dateien
settings-default-apps-pdf-detail = Seiten, mit Zoom.
settings-default-apps-pictures = Bilder
settings-default-apps-pictures-detail = Fotos (aufrecht gedreht), PNG, GIF, WebP, BMP, TIFF und SVG.
settings-default-apps-text = Textdateien
settings-default-apps-text-detail = Reiner Text, Protokolle, Code und anderer Text.
settings-default-apps-sheets = Tabellen
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) und CSV.
settings-default-apps-documents = Dokumente
settings-default-apps-documents-detail = Word (docx) und OpenDocument-Text (odt).
settings-default-apps-katna = Betrachter von Katna Mail
settings-default-apps-system = Standard-App der Arbeitsumgebung
settings-default-apps-ask = Jedes Mal nach der App fragen
settings-default-apps-after-saving = Nach dem Speichern
settings-default-apps-show-folder = Gespeicherte Dateien in ihrem Ordner anzeigen
settings-default-apps-show-folder-detail = Öffnet die Dateiverwaltung und wählt die gespeicherten Anhänge aus

## Settings > Compose

settings-compose-send-from = Neue Nachrichten senden von
settings-compose-send-from-detail = Antworten und Weiterleitungen werden immer von dem Konto gesendet, in dem Sie sich gerade befinden.
settings-compose-send-from-current = Dem aktuellen Konto
settings-compose-send-on-replies = Senden bei Antworten
settings-compose-send-on-replies-detail = Was „Senden“ bei einer Antwort oder Weiterleitung tut. Das Menü neben „Senden“ bietet die andere Möglichkeit.
settings-compose-send-plain = Senden
settings-compose-send-archive = Senden und archivieren
settings-compose-signatures = Signaturen
settings-compose-signatures-detail = Wird unter Ihrer Nachricht nach einer Zeile „--“ eingefügt. Im Fenster zum Verfassen können Sie eine andere wählen.
settings-compose-untitled = Unbenannt
settings-compose-signature-name = Name, z. B. Arbeit
settings-compose-signature-first = Meine Signatur
settings-compose-signature-numbered = Signatur { $number }
settings-compose-signature-delete = Löschen
settings-compose-signature-deleted = Signatur gelöscht
settings-compose-signature-new = Neu erstellen
settings-compose-no-signatures = Noch keine Signaturen.
settings-compose-no-signature = Keine Signatur
settings-compose-for-new-mail = Für neue E-Mails
settings-compose-for-replies = Für Antworten und Weiterleitungen
settings-compose-for-replies-detail = In einer Konversation, in der Sie eine Nachricht signiert haben, beginnt eine Antwort stattdessen mit dieser Signatur.
settings-compose-format = Format
settings-compose-plain-text = In reinem Text schreiben
settings-compose-plain-text-detail = Neue E-Mails beginnen ohne Formatierung; im Fenster zum Verfassen lässt sich das umschalten
settings-compose-spelling = Rechtschreibung
settings-compose-spell-check = Rechtschreibung beim Schreiben prüfen
settings-compose-spell-check-detail = Falsch geschriebene Wörter werden unterstrichen, mit Vorschlägen per Rechtsklick
settings-compose-spell-desktop = Sprache der Arbeitsumgebung ({ $language })
settings-compose-templates = Vorlagen
settings-compose-templates-detail = Speichern Sie E-Mails, die Sie oft schreiben, und beginnen Sie damit eine neue E-Mail oder eine Antwort.

## Settings > Shortcuts

settings-shortcuts-set = Tastenbelegung
settings-shortcuts-set-detail = Beginnen Sie mit den Tasten eines E-Mail-Programms, das Sie kennen. Cmd ist hier Strg. Ihre eigenen Änderungen bleiben über der Belegung erhalten, und „Standard wiederherstellen“ kehrt zu deren Tasten zurück.
settings-shortcuts-single = Kürzel mit einer Taste
settings-shortcuts-single-detail = Tasten ohne Strg oder Alt, wie im Webmail: e archiviert, j und k bewegen, / sucht. Sie funktionieren in der Liste und in der geöffneten Konversation, nie beim Tippen.
settings-shortcuts-single-use = Kürzel mit einer Taste verwenden
settings-shortcuts-single-use-detail = Tastenkombinationen mit Strg funktionieren immer
settings-shortcuts-how = Klicken Sie auf eine Taste, um sie zu ändern, oder auf +, um eine hinzuzufügen, und drücken Sie dann die neuen Tasten. Esc bricht ab.
settings-shortcuts-restore = Standard wiederherstellen
settings-shortcuts-no-key = Keine Taste
settings-shortcuts-press = Tasten drücken…
settings-shortcuts-then = { $keys }, dann…
settings-shortcuts-moved = { $keys } bewirkt jetzt „{ $action }“ statt „{ $previous }“.
settings-shortcuts-single-off = Kürzel mit einer Taste sind aus; diese Taste funktioniert, sobald sie eingeschaltet sind.
settings-shortcuts-restored = Alle Tastenkombinationen haben wieder die Tasten ihrer Belegung.

## Settings search: the line under a result

settings-general-language-summary = Sprache der App sowie von Datum und Zahlen
settings-general-reading-summary = Neueste Nachricht zuerst, vollständige Kopfzeilen, vollständige Namen der Empfänger
settings-general-mark-read-summary = Wann eine geöffnete Konversation als gelesen markiert wird: sofort, nach 1 oder 3 Sekunden oder von Hand
settings-general-reply-button-summary = Die Antwortschaltfläche neben jeder Nachricht antwortet allen
settings-general-remote-images-summary = Die Bilder jeder Nachricht immer anzeigen
settings-general-sending-summary = Senden rückgängig machen: wie lange eine gesendete Nachricht wartet, damit sie zurückgenommen werden kann
settings-general-offline-summary = Wie viele Tage aktueller E-Mails vollständig heruntergeladen werden, um sie ohne Verbindung zu lesen
settings-general-notifications-summary = Benachrichtigungen bei neuen E-Mails und ihr Ton
settings-general-desktop-summary = Katna Mail bei der Anmeldung öffnen, das Symbol im Systemabschnitt und die Anzahl ungelesener Nachrichten am Symbol in der Kontrollleiste
settings-accounts-accounts-summary = Ein Konto hinzufügen oder entfernen oder sein Bild ändern
settings-appearance-density-summary = Standard- oder kompakte Zeilen in der Liste
settings-appearance-scaling-summary = Alles größer oder kleiner machen: Text, Symbole, Abstände und Trennlinien
settings-appearance-theme-summary = Wie die Arbeitsumgebung, hell oder dunkel
settings-appearance-sender-pictures-summary = Firmenlogos, anhand der Domain des Absenders gesucht
settings-appearance-important-summary = Die Wichtig-Markierung neben jeder Nachricht in der Liste
settings-appearance-mail-colors-summary = Dunkle Farben für HTML-E-Mails bei dunklem Design oder die Farben des Absenders
settings-appearance-attachment-previews-summary = Ein kleines Bild vom Inhalt jedes Anhangs
settings-shortcuts-set-summary = Mit den Tasten von Gmail, Inbox by Gmail, Apple Mail, Outlook oder Thunderbird beginnen
settings-shortcuts-single-summary = Tasten ohne Strg oder Alt, wie im Webmail
settings-default-apps-pdf-summary = Womit PDF-Anhänge geöffnet werden
settings-default-apps-pictures-summary = Womit Fotos und Bilder geöffnet werden
settings-default-apps-text-summary = Womit reiner Text, Protokolle und Code geöffnet werden
settings-default-apps-sheets-summary = Womit Excel-, OpenDocument- und CSV-Dateien geöffnet werden
settings-default-apps-documents-summary = Womit Word- und OpenDocument-Texte geöffnet werden
settings-default-apps-after-saving-summary = Gespeicherte Anhänge in ihrem Ordner anzeigen
settings-compose-send-from-summary = Das Konto, von dem neue E-Mails gesendet werden: das aktuelle oder immer dasselbe
settings-compose-send-on-replies-summary = „Senden“ oder „Senden und archivieren“ bei Antworten und Weiterleitungen
settings-compose-signatures-summary = Wird unter Ihrer Nachricht nach einer Zeile „--“ eingefügt
settings-compose-for-new-mail-summary = Die Signatur, mit der neue E-Mails beginnen
settings-compose-for-replies-summary = Die Signatur, mit der Antworten und Weiterleitungen beginnen
settings-compose-format-summary = Neue E-Mails in reinem Text schreiben
settings-compose-spelling-summary = Rechtschreibung beim Schreiben prüfen und die Sprache des Wörterbuchs
settings-compose-templates-summary = Demnächst: E-Mails speichern, die Sie oft schreiben, und damit eine neue E-Mail oder eine Antwort beginnen
settings-feedback-crash-reports-summary = Absturzberichte auf diesem Computer speichern, wenn Katna Mail oder sein Hintergrunddienst abstürzt
settings-feedback-saved-summary = Die auf diesem Computer gespeicherten Absturzberichte ansehen, kopieren oder löschen
settings-feedback-help-improve-summary = Absturzberichte senden, um bei der Fehlerbehebung zu helfen; aus, solange Sie es nicht einschalten
settings-experimental-blur-summary = Die Arbeitsumgebung scheint verschwommen durch die obere Leiste, und Menüs wirken wie Milchglas
settings-search-shortcut = Tastenkombination
settings-search-tab = Einstellungs-Tab
settings-search-none = Keine Einstellungen entsprechen „{ $query }“.
settings-search-results = Einstellungen, die „{ $query }“ entsprechen

## Quick settings (the panel that slides in from the right)

quick-title = Schnelleinstellungen
quick-see-all = Alle Einstellungen aufrufen
quick-reading-pane = Lesebereich
quick-pane-right = Rechts neben der Liste
quick-pane-none = Keine Unterteilung
quick-density = Dichte
quick-density-default = Standard
quick-density-compact = Kompakt
quick-theme = Design
quick-theme-system = Wie die Arbeitsumgebung
quick-theme-light = Hell
quick-theme-dark = Dunkel
quick-desktop-colors = Farben der Arbeitsumgebung
quick-desktop-colors-detail = Das Farbschema und die Akzentfarbe der Arbeitsumgebung
quick-app-names = App-Namen
quick-app-names-detail = Namen unter den App-Symbolen ganz links
quick-inbox-tabs = Posteingangs-Tabs
quick-inbox-tabs-detail = Die Tabs des E-Mail-Anbieters jedes Kontos
quick-choose-tabs = Tabs auswählen
quick-choose-tabs-detail = Pro Konto, in den Einstellungen
quick-sending = Senden
quick-undo-send = Senden rückgängig machen
quick-undo-send-off = Aus
quick-undo-send-seconds = { $seconds } s
quick-signatures = Signaturen
quick-signatures-none = Noch keine
quick-signatures-one = { $name }, standardmäßig verwendet
quick-signatures-many = { $count ->
    [one] { $count } Signatur; standardmäßig { $name }
   *[other] { $count } Signaturen; standardmäßig { $name }
}
quick-signatures-no-default = { $count ->
    [one] { $count }, keine als Standard
   *[other] { $count }, keine als Standard
}
quick-signature-untitled = Unbenannt
quick-threading = E-Mail-Threads
quick-conversation-view = Konversationsansicht
quick-conversation-view-detail = Antworten auf dieselbe E-Mail gruppieren
quick-help = Hilfe
quick-tour = Tour starten
quick-whats-new = Neuigkeiten
quick-about = Über Katna

## Settings: opening at login

settings-open-at-login-failed = Öffnen bei der Anmeldung konnte nicht geändert werden: { $error }

## Settings > Appearance > Scaling

scale-letter = A
scale-percent = { $percent } %
scale-reset = Zurück auf { $percent } %

## Settings > Experimental > Look & Feel

look-intro = Funktionen, die noch erprobt werden. Sie können sich ändern oder wegfallen.
look-heading = Erscheinungsbild
look-window-frame = Fensterrahmen
look-window-frame-detail = Wer die Titelleiste, die Fensterknöpfe, die Ecken und den Schatten zeichnet.
look-frame-native-kde = Nativ: der Rahmen von KDE, in Ihrem Plasma-Design
look-frame-native = Nativ: der Rahmen der Arbeitsumgebung
look-frame-katna = Katna: Die obere Leiste wird zur Titelleiste
look-frame-katna-note-named = Katna zeichnet abgerundete Ecken und einen eigenen Schatten. Der Rahmen folgt nicht mehr dem Design von { $desktop }; Fensterregeln gelten weiterhin.
look-frame-katna-note = Katna zeichnet abgerundete Ecken und einen eigenen Schatten. Der Rahmen folgt nicht mehr dem Design der Arbeitsumgebung; Fensterregeln gelten weiterhin.
look-frame-client-side = Ihre Arbeitsumgebung überlässt den Rahmen jeder App selbst, daher zeichnet Katna bereits einen eigenen.
look-blurred-background = Verschwommener Hintergrund
look-blurred-background-detail = Die Arbeitsumgebung scheint verschwommen durch die obere Leiste und die Ordner, und Menüs und Popups wirken wie Milchglas.
look-blur = Hintergrund hinter dem Fenster verwischen
look-blur-detail = E-Mails bleiben auf undurchsichtigen Karten, damit der Text seinen Kontrast behält
look-blur-off-kde = Der Verwischen-Effekt von KDE ist aus. Schalten Sie „Verwischen“ unter Systemeinstellungen, Fensterverwaltung, Arbeitsflächen-Effekte ein und öffnen Sie Katna Mail dann erneut.
look-blur-none-gnome = GNOME verwischt nicht, was hinter Fenstern liegt.
look-blur-none-x11 = Ihr Fenstermanager verwischt nicht, was hinter Fenstern liegt.
look-blur-none-wayland = Ihr Compositor verwischt nicht, was hinter Fenstern liegt.

## Settings > User feedback (crash reports)

feedback-intro-sending = Neue Absturzberichte werden gesendet, um bei der Fehlerbehebung zu helfen. Nichts anderes verlässt diesen Computer.
feedback-intro-local = Katna sendet nichts. Absturzberichte bleiben auf diesem Computer, damit Sie sie ansehen oder einem Fehlerbericht anhängen können.
feedback-crash-reports = Absturzberichte
feedback-crash-reports-detail = Werden erstellt, wenn Katna Mail oder sein Hintergrunddienst abstürzt.
feedback-save = Absturzberichte auf diesem Computer speichern
feedback-save-detail = Ihr persönlicher Ordner, Benutzer- und Computernamen sowie E-Mail-Adressen werden weggelassen
feedback-saved = Gespeicherte Absturzberichte
feedback-saved-detail = { $count ->
    [one] Der neueste Bericht wird aufbewahrt.
   *[other] Die neuesten { $count } werden aufbewahrt.
}
feedback-help-improve = Helfen Sie, Katna zu verbessern
feedback-help-improve-detail = Aus, solange Sie es nicht einschalten, und Sie können es hier jederzeit wieder ausschalten.
feedback-send = Absturzberichte senden
feedback-send-detail = Der gespeicherte Bericht geht genau so, wie Sie ihn hier sehen können, an den Absturz-Tracker von Katna (Sentry, in der EU). Keine IP-Adresse, keine Nachrichten, keine E-Mail-Adressen
feedback-none-saved = Keine Absturzberichte gespeichert.
feedback-delete-all = Alle löschen
feedback-app-daemon = Hintergrunddienst
feedback-report-sent = { $date } · Gesendet
feedback-view = Ansehen
feedback-view-tooltip = Bericht öffnen
feedback-copy-tooltip = Kopieren, um ihn in einen Fehlerbericht einzufügen
feedback-copied = Absturzbericht kopiert.
feedback-deleted-all = Absturzberichte gelöscht.
feedback-read-failed = Der Absturzbericht konnte nicht gelesen werden: { $error }
feedback-delete-failed = Der Absturzbericht konnte nicht gelöscht werden: { $error }
feedback-delete-all-failed = Die Absturzberichte konnten nicht gelöscht werden: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _Datei
desktop-menu-new-message = _Neue Nachricht
desktop-menu-quit = _Beenden
desktop-menu-edit = _Bearbeiten
desktop-menu-undo = _Rückgängig
desktop-menu-select-all = _Alles auswählen
desktop-menu-select-none = Auswahl auf_heben
desktop-menu-find = _Suchen…
desktop-menu-view = _Ansicht
desktop-menu-folder-list = _Ordnerliste anzeigen
desktop-menu-refresh = A_ktualisieren
desktop-menu-go = _Gehe zu
desktop-menu-inbox = _Posteingang
desktop-menu-starred = _Markiert
desktop-menu-sent = _Gesendet
desktop-menu-drafts = _Entwürfe
desktop-menu-all-mail = _Alle Nachrichten
desktop-menu-next = _Nächste Konversation
desktop-menu-previous = _Vorherige Konversation
desktop-menu-message = _Nachricht
desktop-menu-open = Ö_ffnen
desktop-menu-reply = _Antworten
desktop-menu-reply-all = Alle_n antworten
desktop-menu-forward = _Weiterleiten
desktop-menu-archive = Arch_ivieren
desktop-menu-delete = _Löschen
desktop-menu-spam = Als _Spam melden
desktop-menu-move-to = _Verschieben nach…
desktop-menu-mark-read = Als _gelesen markieren
desktop-menu-mark-unread = Als _ungelesen markieren
desktop-menu-star = Mar_kieren
desktop-menu-important = Als wi_chtig markieren
desktop-menu-not-important = Als nich_t wichtig markieren
desktop-menu-settings = _Einstellungen
desktop-menu-quick-settings = _Schnelleinstellungen
desktop-menu-configure = Katna Mail ein_richten…
desktop-menu-help = _Hilfe
desktop-menu-shortcuts = _Tastenkombinationen
desktop-menu-whats-new = _Neuigkeiten
desktop-menu-about = Ü_ber Katna

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = Navigation
shortcut-group-actions = Aktionen
shortcut-group-go-to = Gehe zu
shortcut-group-app = Anwendung

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = Nächste Konversation
shortcut-previous = Vorherige Konversation
shortcut-down = In der Liste nach unten
shortcut-up = In der Liste nach oben
shortcut-first = Zum ersten Eintrag der Liste
shortcut-last = Zum letzten Eintrag der Liste
shortcut-page-down = In der Liste eine Seite nach unten
shortcut-page-up = In der Liste eine Seite nach oben
shortcut-open = Konversation öffnen
shortcut-back = Zurück zur Liste
shortcut-scroll-down = Nach unten scrollen
shortcut-scroll-up = Nach oben scrollen
shortcut-scroll-page-down = Eine Seite nach unten scrollen
shortcut-scroll-page-up = Eine Seite nach oben scrollen
shortcut-compose = Schreiben
shortcut-reply = Antworten
shortcut-reply-all = Allen antworten
shortcut-forward = Weiterleiten
shortcut-archive = Archivieren
shortcut-delete = Löschen
shortcut-spam = Spam melden
shortcut-move-to = Verschieben nach
shortcut-mark-read = Als gelesen markieren
shortcut-mark-unread = Als ungelesen markieren
shortcut-star = Markierung hinzufügen oder entfernen
shortcut-important = Als wichtig markieren
shortcut-not-important = Als nicht wichtig markieren
shortcut-check = Konversation auswählen
shortcut-select-all = Alle Konversationen auswählen
shortcut-select-none = Auswahl aller Konversationen aufheben
shortcut-undo = Letzte Aktion rückgängig machen
shortcut-go-inbox = Posteingang
shortcut-go-starred = Markiert
shortcut-go-sent = Gesendet
shortcut-go-drafts = Entwürfe
shortcut-go-all = Alle Nachrichten
shortcut-search = E-Mails durchsuchen
shortcut-navigation = Menü ein- oder ausblenden
shortcut-quick-settings = Schnelleinstellungen
shortcut-settings = Alle Einstellungen
shortcut-shortcuts = Tastenkombinationen
shortcut-reload = Auf neue E-Mails prüfen
shortcut-quit = Beenden

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first }, dann { $second }

## Settings > Accounts

accounts-folder-pane = Ordnerbereich
accounts-folder-pane-detail = Die Ordner welcher Konten der Bereich links anzeigt.
accounts-shown-one = Jeweils ein Konto; Wechsel über die Kontokarte
accounts-shown-all = Alle Konten nacheinander
accounts-row = Konten
accounts-row-detail = Wenn Sie ein Konto entfernen, wird die Kopie seiner E-Mails gelöscht, die Katna auf diesem Computer hat. Die E-Mails bleiben auf dem Server.
accounts-none = Noch keine Konten.
accounts-kind-imported = Importiert
accounts-picture-reset = Bild der Arbeitsumgebung verwenden
accounts-picture-change = Bild ändern
accounts-remove = Entfernen
accounts-delete-all-row = Alle Daten löschen
accounts-delete-all-row-detail = Neu beginnen, wie nach einer Neuinstallation.
accounts-delete-all-about = Löscht alle Konten, alle gespeicherten E-Mails, Kontakte und Kalender, den Suchindex, Ihre Einstellungen und gespeicherten Passwörter von diesem Computer. Auf Ihren E-Mail-Servern ändert sich nichts.
accounts-delete-all-open = Alle Katna-Daten löschen

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } wurde aus Katna entfernt.
accounts-removed = { $address } wurde aus Katna entfernt. Die E-Mails sind weiterhin auf dem Server.
accounts-all-deleted = Alle Katna-Daten wurden von diesem Computer gelöscht.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = { $address } entfernen?
accounts-remove-confirm = Konto entfernen
accounts-removing = Wird entfernt…
accounts-remove-local-mail = { $folders ->
    [0] Alle in dieses Konto importierten E-Mails
    [one] Alle in dieses Konto importierten E-Mails in seinem Ordner
   *[other] Alle in dieses Konto importierten E-Mails in seinen { $folders } Ordnern
}
accounts-remove-local-settings = Seine Katna-Einstellungen
accounts-remove-mail = { $folders ->
    [0] Alle von Katna gespeicherten E-Mails dieses Kontos
    [one] Alle von Katna gespeicherten E-Mails dieses Kontos in seinem Ordner
   *[other] Alle von Katna gespeicherten E-Mails dieses Kontos in seinen { $folders } Ordnern
}
accounts-remove-outbox = Seine Nachrichten, die im Postausgang warten
accounts-remove-settings = Sein gespeichertes Passwort und seine Katna-Einstellungen
accounts-delete-all-title = Alle Katna-Daten löschen?
accounts-delete-all-confirm = Alles löschen
accounts-deleting = Wird gelöscht…
accounts-delete-all-accounts = Alle Konten und alle von Katna gespeicherten E-Mails und Anhänge
accounts-delete-all-contacts = Kontakte, Kalender und der Suchindex
accounts-delete-all-settings = Alle Einstellungen, Signaturen und Tastenkombinationen
accounts-delete-all-passwords = Alle gespeicherten Passwörter
accounts-deleted-heading = Von diesem Computer gelöscht:
accounts-cannot-undo = Dies kann nicht rückgängig gemacht werden.
accounts-server-delete-all = Auf Ihren E-Mail-Servern ändert sich nichts: Ihre E-Mails bleiben dort, und wenn Sie ein Konto erneut hinzufügen, werden sie wieder heruntergeladen. Aus Dateien importierte E-Mails gibt es nur in Katna; die Dateien selbst bleiben unverändert.
accounts-server-local = Diese E-Mails wurden aus Dateien importiert, daher hat Katna die einzige Kopie. Die Quelldateien bleiben unverändert; importieren Sie sie erneut, um die E-Mails zurückzubekommen.
accounts-server-remove = Auf dem E-Mail-Server ändert sich nichts: Ihre E-Mails bleiben dort, und wenn Sie das Konto erneut hinzufügen, werden sie wieder heruntergeladen.
accounts-confirm-word = löschen
accounts-confirm-placeholder = „{ accounts-confirm-word }“ eingeben
accounts-confirm-prompt = Geben Sie zur Bestätigung „{ accounts-confirm-word }“ ein:
accounts-cancel = Abbrechen
