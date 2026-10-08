# Katna Mail, German (Deutsch).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = Labels
nav-folders = Ordner
nav-label-new = Neues Label erstellen
nav-folder-new = Neuen Ordner erstellen
nav-menu-check-mail = Auf neue E-Mails prüfen
nav-menu-check-inbox = Diesen Posteingang prüfen
nav-unified-leave-out = Aus dem gemeinsamen Posteingang herausnehmen
nav-unified-bring-back = Wieder in den gemeinsamen Posteingang aufnehmen
nav-menu-sign-in-again = Erneut anmelden
nav-menu-new-mail = Neue E-Mail von diesem Konto
nav-menu-account-settings = Kontoeinstellungen
nav-account-checked = Synchron · geprüft { $ago }
nav-account-in-sync = Synchron
nav-account-connecting = Verbindung wird hergestellt…
nav-account-offline = Offline, neuer Versuch läuft
nav-account-signed-out = Anmeldung bei { $provider } abgelaufen
nav-account-password-refused = Passwort abgelehnt
nav-account-storage = { $used } von { $total } belegt
nav-menu-new-subfolder = Neuer Ordner darin
nav-menu-new-sublabel = Neues Label darin
nav-menu-rename = Umbenennen
nav-menu-delete = Löschen
nav-menu-empty-trash = Papierkorb leeren
nav-account-unnamed = Konto { $number }
nav-all-accounts = Alle Konten
nav-expand = Ordner anzeigen
nav-collapse = Ordner ausblenden
storage-used = { $percent } % von { $total } belegt
storage-used-detail = { $address }: { $used } von { $total } belegt

## Special folders (the user's own folders keep their names)

folder-inbox = Posteingang
folder-starred = Markiert
folder-snoozed = Zurückgestellt
folder-unread = Ungelesen
folder-important = Wichtig
folder-drafts = Entwürfe
folder-sent = Gesendet
folder-archive = Archiv
folder-spam = Spam
folder-trash = Papierkorb
folder-all-mail = Alle Nachrichten
folder-scheduled = Geplant
folder-waiting = Wartet auf Antwort
folder-waiting-short = Wartend
folder-reminders = Erinnerungen
folder-outbox = Postausgang
folder-activity = Aktivität

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
label-rename-title = Label umbenennen
label-folder-rename-title = Ordner umbenennen
label-rename = Umbenennen
label-renaming = Wird umbenannt…
label-renamed = Label umbenannt in „{ $name }“.
label-folder-renamed = Ordner umbenannt in „{ $name }“.

## Deleting a folder or label (asked first)

folder-delete-title = „{ $name }“ löschen?
folder-delete-body = { $count ->
    [0] Er enthält keine E-Mails. Der Ordner wird vom Server entfernt, sodass er auch im Webmail und auf Ihrem Handy verschwindet.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Seine { $count } Konversation kommt in den Papierkorb, Sie können sie also noch zurückholen.
           *[other] Seine { $count } Konversationen kommen in den Papierkorb, Sie können sie also noch zurückholen.
        }
       *[message] { $count ->
            [one] Seine { $count } Nachricht kommt in den Papierkorb, Sie können sie also noch zurückholen.
           *[other] Seine { $count } Nachrichten kommen in den Papierkorb, Sie können sie also noch zurückholen.
        }
    } Der Ordner wird vom Server entfernt, sodass er auch im Webmail und auf Ihrem Handy verschwindet.
}
folder-delete-forever-body = { $count ->
    [0] Er enthält keine E-Mails. Der Ordner wird vom Server entfernt, sodass er auch im Webmail und auf Ihrem Handy verschwindet.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Seine { $count } Konversation wird endgültig gelöscht; dieses Konto hat keinen Papierkorb.
           *[other] Seine { $count } Konversationen werden endgültig gelöscht; dieses Konto hat keinen Papierkorb.
        }
       *[message] { $count ->
            [one] Seine { $count } Nachricht wird endgültig gelöscht; dieses Konto hat keinen Papierkorb.
           *[other] Seine { $count } Nachrichten werden endgültig gelöscht; dieses Konto hat keinen Papierkorb.
        }
    } Der Ordner wird vom Server entfernt, sodass er auch im Webmail und auf Ihrem Handy verschwindet.
}
folder-delete-label-body = Das Label wird entfernt. Seine E-Mails bleiben in „Alle Nachrichten“ und in ihren anderen Labels.
folder-delete-confirm = Ordner löschen
folder-delete-label-confirm = Label löschen
folder-deleted = Ordner „{ $name }“ gelöscht
label-deleted = Label „{ $name }“ gelöscht
