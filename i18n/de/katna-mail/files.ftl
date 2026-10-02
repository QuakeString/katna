# Katna Mail, German (Deutsch).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = Dateien suchen

## Left side (and chips on a phone)

files-all = Alle Dateien
files-pictures = Bilder
files-pdfs = PDFs
files-documents = Dokumente
files-sheets = Tabellen
files-slides = Folien
files-other = Sonstige
files-accounts = Konten
files-drives = Laufwerke
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = Für mich freigegeben
files-shown = Angezeigt
files-received = Empfangen
files-sent = Von mir gesendet

## Over the files

files-count = { $count ->
    [one] { $count } Datei · { $size }
   *[other] { $count } Dateien · { $size }
}
files-anyone = Alle Personen
files-from-person = Von { $name }
files-time-any = Beliebige Zeit
files-time-today = Heute
files-time-yesterday = Gestern
files-time-this-week = Diese Woche
files-time-last-week = Letzte Woche
files-time-this-month = Dieser Monat
files-time-last-month = Letzter Monat
files-time-between = { $first } – { $last }
files-time-hint = Klicken Sie auf einen Tag oder ziehen Sie über mehrere Tage
files-time-summary = { $count ->
    [one] { $days } · { $count } Datei
   *[other] { $days } · { $count } Dateien
}
files-time-clear = Löschen
files-time-month-back = Vorheriger Monat
files-time-month-on = Nächster Monat
files-time-wheel = Scrollen, um diese Daten zu verschieben, bei gleicher Länge
files-sort-newest = Neueste zuerst
files-sort-oldest = Älteste zuerst
files-sort-largest = Größte zuerst
files-sort-name = Nach Name
files-grid = Karten
files-list = Liste
files-this-week = Diese Woche
files-undated = Ohne Datum
files-me = Ich
files-no-subject = (kein Betreff)
files-loading = Dateien aus Ihren E-Mails werden gesammelt…
files-empty = Dateien aus Ihren E-Mails werden hier angezeigt.
files-none-match = Keine passenden Dateien.
files-load-failed = Die Dateien konnten nicht gelesen werden: { $error }

## A file's menu and buttons

files-open = Öffnen
files-open-with = Öffnen mit…
files-save = Speichern…
files-show-mail = E-Mail anzeigen
files-mail-window = E-Mail in neuem Fenster öffnen
files-forward = Datei weiterleiten
files-from-them = Dateien von { $name }
files-copy-name = Dateinamen kopieren
files-name-copied = Dateiname kopiert
files-downloading = E-Mail wird heruntergeladen…
files-download-failed = Diese E-Mail konnte nicht heruntergeladen werden.

## A cloud drive in place of the mail files

files-drive-mine = Meine Ablage
files-drive-mine-onedrive = Meine Dateien
files-drive-results = „{ $words }“
files-drive-count = { $folders ->
    [0] { $files ->
        [one] 1 Datei
       *[other] { $files } Dateien
    }
    [one] 1 Ordner · { $files ->
        [one] 1 Datei
       *[other] { $files } Dateien
    }
   *[other] { $folders } Ordner · { $files ->
        [one] 1 Datei
       *[other] { $files } Dateien
    }
}
files-drive-folders = Ordner
files-drive-files = Dateien
files-drive-folder = Ordner
files-drive-meta = { $what } · Bearbeitet { $date }
files-drive-as-link = { $what } · als Link
files-drive-google-doc = Google-Dokument
files-drive-google-sheet = Google-Tabelle
files-drive-google-slides = Google-Präsentation
files-drive-google-drawing = Google-Zeichnung
files-drive-fetching = Wird geholt…
files-drive-loading = Laufwerk wird geöffnet…
files-drive-empty = Dieser Ordner ist leer.
files-drive-unreachable = { $drive } ist nicht erreichbar.
files-drive-try-again = Erneut versuchen
files-drive-needs-permission = Katna braucht einmalig Ihre Erlaubnis, um dieses Laufwerk anzuzeigen. Melden Sie sich erneut an und erlauben Sie Katna, Ihre Dateien zu sehen.
files-drive-allow = Erlauben
files-drive-allow-failed = Die Anmeldung wurde nicht abgeschlossen, daher bleibt das Laufwerk geschlossen.
files-drive-attach = Anhängen
files-drive-more = Mehr
files-drive-download = Herunterladen…
files-drive-open-web = In { $drive } öffnen
files-drive-copy-link = Link kopieren
files-drive-link-copied = Link kopiert
files-drive-share = Teilen…
files-drive-rename = Umbenennen
files-drive-trash = In den Papierkorb
files-drive-trashed = „{ $name }“ liegt im Papierkorb von { $drive }
files-drive-renamed = Umbenannt in „{ $name }“
files-drive-getting = { $name } wird von { $drive } geholt…
files-drive-get-failed = { $name } konnte nicht geholt werden: { $error }
files-drive-upload = Hochladen
files-drive-upload-files = Dateien hochladen
files-drive-upload-folder = Ordner hochladen
files-drive-upload-failed = { $name } konnte nicht hochgeladen werden: { $error }
files-drive-upload-needs = Zum Hochladen braucht Katna einmalig Ihre Erlaubnis: Drücken Sie „Erlauben“ unter Einstellungen › Standard-Apps › Seite Dateien.

## The Share dialog of a drive file or folder

files-share-title = „{ $name }“ teilen
files-share-add = Personen per Name oder Adresse hinzufügen
files-share-not-address = „{ $text }“ ist keine E-Mail-Adresse
files-share-notify = { $drive } soll sie auch per E-Mail benachrichtigen
files-share-people = Personen mit Zugriff
files-share-general = Allgemeiner Zugriff
files-share-loading = Zugriffsberechtigte werden gelesen…
files-share-restricted = Eingeschränkt
files-share-restricted-about = Nur Personen mit Zugriff können es über den Link öffnen
files-share-anyone = Jeder mit dem Link
files-share-anyone-can = { $role ->
    [editor] Jeder mit dem Link kann bearbeiten
    [commenter] Jeder mit dem Link kann kommentieren
   *[viewer] Jeder mit dem Link kann ansehen
}
files-share-anyone-about = { $role ->
    [editor] Jeder im Internet mit dem Link kann bearbeiten
    [commenter] Jeder im Internet mit dem Link kann kommentieren
   *[viewer] Jeder im Internet mit dem Link kann ansehen
}
files-share-role-owner = Eigentümer
files-share-role-editor = Bearbeiter
files-share-role-commenter = Kommentator
files-share-role-viewer = Betrachter
files-share-you = { $name } (Sie)
files-share-domain = Alle bei { $domain }
files-share-inherited = Zugriff über einen übergeordneten Ordner
files-share-remove = Zugriff entfernen
files-share-copy-link = Link kopieren
files-share-share = Teilen
files-share-done = Fertig
files-share-sharing = Wird geteilt…
files-share-shared = { $count ->
    [one] Mit 1 Person geteilt
   *[other] Mit { $count } Personen geteilt
}
files-share-refused = { $drive } konnte nicht mit { $addresses } teilen
files-share-failed = Freigabe konnte nicht geändert werden: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] 1 Element wird hochgeladen
   *[other] { $count } Elemente werden hochgeladen
}
files-tray-done = { $count ->
    [one] 1 Upload fertig
   *[other] { $count } Uploads fertig
}
files-tray-some-failed = { $done } hochgeladen, { $failed } fehlgeschlagen
files-tray-minutes-left = { $minutes ->
    [one] Noch etwa eine Minute
   *[other] Noch etwa { $minutes } Minuten
}
files-tray-seconds-left = Noch weniger als eine Minute
files-tray-starting = Wird gestartet…
files-tray-cancel-all = Alle abbrechen
files-tray-cancel = Abbrechen
files-tray-fold = Liste ausblenden
files-tray-unfold = Liste anzeigen
files-tray-close = Schließen
files-tray-progress = { $place } · { $sent } von { $size }
files-tray-in = In { $place }
files-tray-cancelled = Abgebrochen
