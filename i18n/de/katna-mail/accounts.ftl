# Katna Mail, German (Deutsch).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

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
