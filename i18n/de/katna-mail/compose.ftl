# Katna Mail, German (Deutsch).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = Neue Nachricht
compose-restore = Wiederherstellen
compose-minimize = Minimieren
compose-exit-full-screen = Vollbild beenden
compose-open-window = In neuem Fenster öffnen
compose-save-close = Speichern und schließen
compose-back-to-mail = Zurück zum Mail-Fenster
compose-pop-out-reply = Antwort in eigenem Fenster
compose-edit-recipients = Empfänger bearbeiten
compose-summary-cc = Cc: { $names }
compose-summary-bcc = Bcc: { $names }
compose-show-trimmed = Gekürzten Inhalt anzeigen
compose-hide-trimmed = Gekürzten Inhalt ausblenden
compose-remove-trimmed = Zitierten Text entfernen
compose-trimmed-removed = Zitierter Text entfernt

## Recipients and subject

compose-to = An
compose-cc = Cc
compose-bcc = Bcc
compose-from = Von
compose-from-choose = Von einem anderen Konto senden
compose-recipients = Empfänger
compose-subject = Betreff

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = Senden oder verwerfen Sie zuerst die geöffnete Nachricht.
compose-bad-address = „{ $address }“ ist keine E-Mail-Adresse.
compose-no-recipients = Fügen Sie mindestens einen Empfänger hinzu.
compose-attachments-too-large = Die Anhänge sind { $size } groß; Mailserver nehmen bis zu { $limit } an.
compose-no-account = Fügen Sie ein Konto hinzu, von dem aus Sie E-Mails senden.
compose-past-time = Wählen Sie einen Zeitpunkt in der Zukunft.
compose-scheduling = Wird geplant…
compose-sending = Wird gesendet…
compose-scheduled = Senden geplant für { $when }
compose-sent-archived = Gesendet und archiviert
compose-sent = Nachricht gesendet
compose-discarded = Entwurf verworfen
compose-draft-saved = Entwurf gespeichert
compose-draft-failed = Der Entwurf konnte nicht gespeichert werden: { $error }
compose-draft-not-opened = Der Entwurf konnte nicht geöffnet werden.

## Attachments

compose-picker-insert = Einfügen
compose-picker-attach = Anhängen
compose-file-too-large = { $name } ist zu groß: Eine Nachricht kann bis zu { $limit } enthalten.
compose-attachment-size = ({ $size })
compose-remove-attachment = Anhang entfernen
compose-attachments-total = { $count ->
    [one] { $count } Datei, { $size }
   *[other] { $count } Dateien, { $size }
}
compose-drop-files = Dateien hier ablegen
compose-drop-here = Hier ablegen
compose-paste-keep-formatting = Formatierung beibehalten
compose-paste-table = Tabelle
compose-paste-picture = Bild
compose-paste-plain-text = Nur-Text
compose-paste-inline = Im Text
compose-paste-attachment = Anhang

## Encryption and signing (the toggles by the recipients)

compose-encrypt = Verschlüsseln
compose-encrypted = Verschlüsselt: Nur die Empfänger können sie lesen
compose-sign = Signieren
compose-signed = Signiert: Empfänger können prüfen, dass sie von Ihnen stammt
compose-track = Öffnungen und Klicks verfolgen
compose-tracked = Verfolgt: Sie sehen, wann jeder Empfänger sie öffnet oder einem Link folgt
compose-track-clicks = Linkklicks verfolgen (Nur-Text kann keine Öffnungen anzeigen)
compose-tracked-clicks = Verfolgt: Sie sehen, wann jeder Empfänger einem Link folgt
compose-track-sign-in = Melden Sie sich bei einem Katna-Konto an, um Öffnungen und Klicks zu verfolgen
compose-receipt = Lesebestätigung anfordern
compose-receipt-on = Lesebestätigung angefordert: Die App des Empfängers fragt ihn möglicherweise, ob er eine sendet
compose-delivery = Zustellbestätigung anfordern
compose-delivery-on = Zustellbestätigung angefordert: Ihr E-Mail-Server schickt Ihnen eine E-Mail, sobald der Server jedes Empfängers sie annimmt
compose-delivery-unavailable = Ihr E-Mail-Server sendet keine Zustellbestätigungen

## Spelling

spell-no-dictionary = Kein Rechtschreibwörterbuch für { $language } installiert (zum Beispiel hunspell-en_us).
spell-dictionary-error = Rechtschreibwörterbuch: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = „{ $words }“
grammar-add = „{ $words }“ hinzufügen
grammar-remove = „{ $words }“ entfernen
grammar-ignore = Ignorieren

## Send checks (asked before a message goes out)

send-check-attachment-title = Wollten Sie Dateien anhängen?
send-check-attachment-text = Sie erwähnen einen Anhang, aber es ist nichts angehängt.
send-check-attach = Datei anhängen
send-check-subject-title = Ohne Betreff senden?
send-check-subject-text = Diese Nachricht hat keinen Betreff.
send-check-add-subject = Betreff hinzufügen
send-check-send-anyway = Trotzdem senden
recipient-not-valid = Keine gültige E-Mail-Adresse
recipient-show-address = Adresse anzeigen
recipient-remove = Entfernen
recipient-bad-title = Adresse prüfen
recipient-bad-text = „{ $address }“ ist keine gültige E-Mail-Adresse. Korrigieren oder entfernen Sie sie vor dem Senden.
recipient-bad-fix = Korrigieren
