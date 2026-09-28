# Katna Mail, German (Deutsch).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Schließen
reader-back = Zurück
reader-mark-unread = Als ungelesen markieren
reader-move-to = Verschieben nach
reader-more = Mehr
reader-original-colors = Originalfarben anzeigen
reader-dark-colors = In dunklen Farben anzeigen
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
reader-sending = Wird gesendet…
reader-me = mich
reader-to = an { $names }
reader-to-label = an
reader-tick-delivered = Zugestellt { $when }
reader-tick-no-bounce = Gesendet { $when }; keine Unzustellbarkeitsnachricht kam zurück, sie ist also sehr wahrscheinlich angekommen
reader-tick-bounced = Nicht zugestellt: unzustellbar { $when }
reader-tick-read = Gelesen { $when } (Lesebestätigung)
reader-tick-opened = Geöffnet, zuletzt { $when } (Öffnungsverfolgung)
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
tracking-opened = { $who } hat sie { $count ->
    [one] einmal
   *[other] { $count }-mal
} geöffnet, zuletzt { $when }
tracking-opens-clicks = { $who } hat sie { $opens ->
    [one] einmal
   *[other] { $opens }-mal
} geöffnet und { $clicks ->
    [one] einmal
   *[other] { $clicks }-mal
} einen Link aufgerufen, zuletzt { $when }
tracking-clicked = { $who } hat { $clicks ->
    [one] einmal
   *[other] { $clicks }-mal
} einen Link aufgerufen, zuletzt { $when }
tracking-maybe-opened = { $who } hat sie vielleicht geöffnet (Apple Mail lädt Bilder zum Schutz der Privatsphäre)
tracking-seen-none = Noch hat niemand sie geöffnet oder einen Link aufgerufen
tracking-receipt = { $who } hat eine Lesebestätigung gesendet
tracking-receipt-displayed = Lesebestätigung: { $who } hat Ihre Nachricht geöffnet
tracking-receipt-other = Lesebestätigung: { $who } hat Ihre Nachricht gelöscht oder bearbeitet, ohne sie zu öffnen

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
print-preview-title = Druckvorschau
print-preview-laying-out = Seiten werden aufgebaut…
print-preview-pages = { $count ->
    [one] { $count } Seite
   *[other] { $count } Seiten
}
print-preview-more = { $count ->
    [one] und { $count } weitere Seite
   *[other] und { $count } weitere Seiten
}
print-preview-failed = die Seiten konnten nicht angezeigt werden
print-preview-paper = Papier
print-preview-a4 = A4
print-preview-letter = US-Letter
print-preview-layout = Layout
print-preview-as-shown = Wie angezeigt
print-preview-simple = Einfacher Text
print-preview-backgrounds = Hintergründe
print-preview-cancel = Abbrechen
print-preview-print = Drucken
print-not-downloaded = (Noch nicht heruntergeladen.)
print-encrypted = (Verschlüsselt. Öffnen Sie die Nachricht in Katna Mail, um ihren Text zu drucken.)
print-to = An: { $addresses }
print-cc = Cc: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Öffnen Sie diese Nachricht, um ihre Anhänge zu lesen.
text-copy = Kopieren
text-select-all = Alles auswählen
