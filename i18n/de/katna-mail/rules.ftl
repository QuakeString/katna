# Katna Mail, German (Deutsch).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = Regeln
settings-rules-summary = Neue E-Mails automatisch sortieren, mit Labels versehen, weiterleiten oder stummschalten
settings-rules-intro = Regeln sortieren neue E-Mails automatisch, in dieser Reihenfolge. Zum Umsortieren ziehen.
settings-rules-all-accounts = Alle Konten
settings-rules-new = Neue Regel
settings-rules-none = Noch keine Regeln. Eine Regel sortiert neue E-Mails automatisch: nach Absender, Betreff oder Wörtern.
settings-rules-none-account = Noch keine Regeln für dieses Konto.
settings-rules-drag = Zum Umsortieren ziehen
settings-rules-edit = Regel bearbeiten
settings-rules-turn-off = Diese Regel ausschalten
settings-rules-turn-on = Diese Regel einschalten

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = Vorlagen für Regeln
settings-rules-starters-intro = Aus, bis Sie eine einschalten. Sie gelten für alle Ihre Konten; bearbeiten Sie eine, um sie zu ändern.
settings-rules-starter-turning-on = „{ $name }“ wird eingeschaltet…
settings-rules-starter-failed = „{ $name }“ konnte nicht eingeschaltet werden: { $error }
rules-starter-promotions = Werbung stummschalten
rules-starter-newsletters = Newsletter nach „Lesen“
rules-starter-receipts = Belege und Rechnungen
rules-starter-deliveries = Lieferungen
rules-starter-train = Bahntickets
rules-starter-flight = Flugtickets
rules-starter-codes = Einmalcodes
rules-starter-security = Sicherheitswarnungen
rules-starter-social = Soziale Netzwerke
rules-starter-invites = Kalendereinladungen
rules-starter-folder-reading = Lesen
rules-starter-folder-receipts = Belege
rules-starter-folder-deliveries = Lieferungen
rules-starter-folder-travel = Reisen
rules-starter-folder-social = Soziales
rules-runs-katna = Läuft in Katna
rules-runs-gmail = Läuft bei Gmail
rules-runs-sieve = Läuft auf dem Server
rules-stopped = Angehalten
rules-error-folder-gone = Der Ordner dieser Regel existiert nicht mehr. Bearbeiten Sie die Regel, um einen anderen zu wählen.
rules-error-no-archive = Dieses Konto hat keinen Archivordner. Bearbeiten Sie die Regel, damit sie etwas anderes tut.
rules-error-no-trash = Dieses Konto hat keinen Papierkorb. Bearbeiten Sie die Regel, damit sie etwas anderes tut.
rules-error-cannot-send = Dieses Konto kann keine E-Mails senden, daher kann die Regel sie nicht weiterleiten.
rules-error-other = { $error }. Bearbeiten Sie die Regel und schalten Sie sie wieder ein.

settings-folders = Ordner
settings-folders-summary = Anzahl ungelesener Nachrichten im Ordnerbereich
settings-folders-unread-counts = Anzahl ungelesener Nachrichten bei jedem Ordner
settings-folders-unread-counts-detail = Aus: Nur der Posteingang zeigt, wie viele ungelesen sind

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } und { $next }
rules-summary-or = { $first } oder { $next }
rules-summary-more = { $count } weitere
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = Hat einen Anhang
rules-summary-no-attachment = Hat keinen Anhang
rules-summary-mailing-list = Von einer Mailingliste
rules-summary-not-mailing-list = Nicht von einer Mailingliste
rules-summary-tab = Im Tab { $tab }
rules-summary-not-tab = Nicht im Tab { $tab }
rules-summary-move = nach { $folder } verschieben
rules-summary-archive = Posteingang überspringen
rules-summary-trash = in den Papierkorb verschieben
rules-summary-mark-read = als gelesen markieren
rules-summary-star = markieren
rules-summary-important = als wichtig markieren
rules-summary-label = Label { $label } hinzufügen
rules-summary-forward = an { $address } weiterleiten
rules-summary-dont-notify = nicht benachrichtigen
rules-summary-read-after = { $count ->
    [one] nach { $count } Tag als gelesen markieren
   *[other] nach { $count } Tagen als gelesen markieren
}
rules-summary-folder-gone = ein Ordner, den es nicht mehr gibt

## The rule editor

rules-editor-new-title = Neue Regel
rules-editor-edit-title = Regel bearbeiten
rules-editor-name-hint = Name der Regel
rules-editor-when = Wenn eine neue E-Mail
rules-editor-of-these = dieser Bedingungen erfüllt:
rules-mode-all = alle
rules-mode-any = eine
rules-field-from = Von
rules-field-to = An
rules-field-cc = Cc
rules-field-any-recipient = An oder Cc
rules-field-reply-to = Antwort an
rules-field-subject = Betreff
rules-field-body = Text
rules-field-attachment-name = Name des Anhangs
rules-field-has-attachment = Hat Anhang
rules-field-mailing-list = Von einer Mailingliste
rules-field-tab = Posteingangs-Tab
rules-comparator-contains = enthält
rules-comparator-not-contains = enthält nicht
rules-comparator-begins-with = beginnt mit
rules-comparator-ends-with = endet auf
rules-comparator-equals = ist genau
rules-comparator-matches = entspricht dem Muster
rules-has-yes = ja
rules-has-no = nein
rules-editor-value-hint = Wörter oder eine Adresse
rules-editor-add-condition = Bedingung hinzufügen
rules-editor-remove = Entfernen
rules-editor-then = Dann:
rules-action-move = Verschieben nach
rules-action-archive = Posteingang überspringen (archivieren)
rules-action-trash = In den Papierkorb verschieben
rules-action-mark-read = Als gelesen markieren
rules-action-star = Markieren
rules-action-important = Als wichtig markieren
rules-action-label = Label hinzufügen
rules-action-forward = Weiterleiten an
rules-action-dont-notify = Nicht benachrichtigen
rules-action-read-after = Als gelesen markieren nach
rules-editor-choose-folder = Ordner wählen
rules-editor-choose-label = Label wählen
rules-editor-new-folder = Neu: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = E-Mail-Adresse
rules-editor-days = Tagen
rules-editor-add-action = Aktion hinzufügen
rules-editor-stop = Hier anhalten: Spätere Regeln werden auf diese E-Mail nicht angewendet
rules-editor-accounts = Konten:
rules-editor-accounts-none = Konten wählen
rules-editor-accounts-many = { $count ->
    [one] { $count } Konto
   *[other] { $count } Konten
}
rules-editor-matches = Trifft auf { $mails } der letzten { $days } Tage zu
rules-editor-mails = { $count ->
    [one] { $count } E-Mail
   *[other] { $count } E-Mails
}
rules-editor-counting = Passende E-Mails werden gezählt…
rules-editor-show = Anzeigen
rules-editor-also-apply = Auch auf diese { $count } anwenden
rules-editor-runs-katna = Läuft in Katna, solange dieser Computer eingeschaltet ist.
rules-editor-runs-gmail = Läuft bei Gmail und wirkt daher auch auf Ihrem Smartphone und bei ausgeschaltetem Computer.
rules-editor-runs-sieve = Läuft auf Ihrem E-Mail-Server und wirkt daher auch auf Ihrem Smartphone und bei ausgeschaltetem Computer.
rules-note-gmail-action = Läuft in Katna: Gmail-Filter können „{ $action }“ nicht.
rules-note-sieve-action = Läuft in Katna: Die Regeln Ihres E-Mail-Servers können „{ $action }“ nicht.
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = Läuft in Katna: Gmail-Filter können „{ $test }“ nicht so prüfen wie Katna.
rules-note-sieve-condition = Läuft in Katna: Die Regeln Ihres E-Mail-Servers können „{ $test }“ nicht so prüfen wie Katna.
rules-note-order = Läuft in Katna, wie eine frühere Regel des Kontos: Regeln laufen in der Reihenfolge der Liste.
rules-note-gmail-stop = Läuft in Katna: Gmail-Filter können spätere Regeln nicht anhalten.
rules-note-gmail-forward = Läuft in Katna: Gmail leitet nur an Adressen weiter, die in seinen Einstellungen bestätigt sind, und { $address } gehört nicht dazu.
rules-note-gmail-folder = Läuft in Katna: Gmail hat kein Label für einen Ordner dieser Regel.
rules-note-sieve-folder = Läuft in Katna: Ihr E-Mail-Server hat einen Ordner dieser Regel nicht.
rules-note-gmail-sign-in = Läuft in Katna, bis Sie sich erneut bei Google anmelden und Katna erlauben, Gmail-Filter zu erstellen.
rules-note-sieve-other-script = Läuft in Katna: Auf Ihrem E-Mail-Server ist ein anderes Regelskript („{ $name }“) aktiv.
rules-note-gmail-failed = Läuft in Katna: Gmail hat sie nicht übernommen ({ $error }).
rules-note-sieve-failed = Läuft in Katna: Ihr E-Mail-Server hat sie nicht übernommen ({ $error }).
rules-editor-cancel = Abbrechen
rules-editor-save = Speichern
rules-editor-saving = Wird gespeichert…
rules-editor-delete = Regel löschen
rules-editor-delete-ask = Diese Regel löschen?
rules-editor-delete-keep = Behalten
rules-editor-delete-confirm = Löschen
rules-editor-needs-folder = Wählen Sie für jedes „Verschieben nach“ einen Ordner und für jedes „Label hinzufügen“ ein Label.
rules-editor-needs-days = „Als gelesen markieren nach“ braucht eine Anzahl Tage von 1 bis 3650.
rules-saved = Regel gespeichert
rules-saved-applied = { $count ->
    [one] Regel gespeichert und auf { $count } E-Mail angewendet
   *[other] Regel gespeichert und auf { $count } E-Mails angewendet
}
rules-apply-failed = Regel gespeichert, aber das Anwenden ist fehlgeschlagen: { $error }
rules-deleted = Regel gelöscht
rules-delete-failed = Die Regel konnte nicht gelöscht werden: { $error }
rules-change-failed = Die Regeln konnten nicht geändert werden: { $error }
