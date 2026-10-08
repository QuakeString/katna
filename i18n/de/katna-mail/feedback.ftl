# Katna Mail, German (Deutsch).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

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

## Settings > User feedback (usage statistics)

feedback-usage = Anonyme Nutzungsstatistiken senden
feedback-usage-detail = Einmal pro Woche: welche Funktionen Sie genutzt haben, ja oder nein. Niemals Anzahlen, Adressen, Namen oder Suchbegriffe
feedback-intro-sending-usage = Absturzberichte und wöchentliche Nutzungsstatistiken werden gesendet. Nichts anderes verlässt diesen Computer.
feedback-intro-usage-only = Wöchentliche Nutzungsstatistiken werden gesendet. Absturzberichte bleiben auf diesem Computer.
feedback-counted = Was gezählt wird
feedback-counted-detail = Jeder Punkt ist ja oder nein für die Woche.
feedback-counted-also = Außerdem: die Version von Katna, die Linux-Familie, die Arbeitsumgebung, die Bildschirmskalierung und die Anzahl der Konten (1, 2–3, 4+)
feedback-see-report = Bericht dieser Woche ansehen
feedback-hide-report = Bericht dieser Woche ausblenden
feedback-report-goes = Wird nach Ende der Woche am { $date } gesendet, wenn Nutzungsstatistiken dann noch eingeschaltet sind.
feedback-install-id = Installations-ID { $id }
feedback-install-id-tooltip = Zufällig, damit ein Computer in einer Woche nicht doppelt gezählt wird. Sie ändert sich alle 90 Tage und wird nie mit Absturzberichten oder Feedback gesendet
feedback-install-id-reset = Zurücksetzen
feedback-install-id-new = Neue Installations-ID erstellt.
feedback-report-copied = Bericht kopiert.
feedback-send-feedback = Feedback
feedback-send-feedback-detail = Ein Problem, eine Idee, alles Mögliche.
feedback-send-feedback-button = Feedback senden…
usage-feature-search-options = Suchoptionen
usage-feature-pins = Angeheftete E-Mails
usage-feature-labels = Labels
usage-feature-scheduled-send = Geplantes Senden
usage-feature-snooze = Zurückstellen und Erinnerungen
usage-feature-encrypted = Verschlüsselte E-Mails
usage-feature-viewers = Integrierte Betrachter
usage-feature-calendar = Kalender
usage-feature-contacts = Kontakte
usage-feature-tasks-notes = Aufgaben und Notizen
usage-feature-phone-layout = Layout für Telefonbreite
usage-feature-own-frame = Eigener Fensterrahmen von Katna

## Help > Send feedback

send-feedback-title = Feedback senden
send-feedback-about = Thema
send-feedback-problem = Problem
send-feedback-idea = Idee
send-feedback-other = Etwas anderes
send-feedback-message = Ihre Nachricht
send-feedback-message-placeholder = Was ist passiert, oder was wünschen Sie sich?
send-feedback-reply = E-Mail-Adresse für eine Antwort (optional)
send-feedback-reply-placeholder = name@example.org
send-feedback-system = Version von Katna und Ihr System mitsenden
send-feedback-what-is-sent = Was gesendet wird
send-feedback-show = Anzeigen
send-feedback-hide = Ausblenden
send-feedback-where = Geht an den Feedback-Posteingang von Katna bei Sentry (EU). Keine IP-Adresse, keine Konten, keine Nachrichten, keine Installations-ID.
send-feedback-cancel = Abbrechen
send-feedback-send = Senden
send-feedback-sending = Wird gesendet…
send-feedback-sent = Feedback gesendet. Vielen Dank
send-feedback-failed = Feedback konnte nicht gesendet werden: { $error }
