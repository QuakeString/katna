# Katna Mail, German (Deutsch).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

chat-heading = Lesen
chat-view = Konversationen als Chats
chat-view-detail = E-Mails zwischen Menschen lesen sich wie ein Gruppenchat: eine Blase pro E-Mail mit nur dem Geschriebenen, Ihre eigenen rechts. Newsletter behalten die übliche Ansicht.
chat-view-switch = Konversationen als Chats anzeigen
chat-view-switch-detail = Die zitierte E-Mail und Signaturen warten hinter ··· in jeder Blase

chat-switch-chat = Chat
chat-switch-mail = E-Mail
chat-people = { $names } und Sie · { $count ->
    [one] { $count } E-Mail
   *[other] { $count } E-Mails
}
chat-people-heading = { $count ->
    [one] In diesem Chat · { $count } Person
   *[other] In diesem Chat · { $count } Personen
}
chat-member-mails = { $count ->
    [0] Keine E-Mails
    [one] { $count } E-Mail
   *[other] { $count } E-Mails
}
chat-today = Heute
chat-yesterday = Gestern
chat-added = { $who } hat { $names } hinzugefügt
chat-renamed = { $who } hat den Betreff in „{ $subject }“ geändert
chat-you = Sie
chat-not-downloaded = Noch nicht heruntergeladen
chat-forwarded = Weitergeleitet
chat-show-quoted = Zitierte E-Mail und Signatur anzeigen
chat-hide-quoted = Zitierte E-Mail und Signatur ausblenden
chat-hide-dots = ··· ausblenden
chat-show-card = Visitenkarte anzeigen
chat-reply-all = Allen antworten
chat-more = Mehr
chat-reply-only = Nur { $name } antworten
chat-forward = Weiterleiten
chat-copy-text = Text kopieren
chat-show-as-mail = Als E-Mail anzeigen
chat-go-down = Zur neuesten E-Mail
chat-pin = Oben anheften
chat-pin-file = Datei oben anheften
chat-unpin = Lösen
chat-unpin-file = Datei lösen
chat-pinned-of = Angeheftet { $at } von { $count }
chat-pins-all = Alle angehefteten
chat-pins-heading = Angeheftet · { $count } von { $most }
chat-pins-drag = Zum Umsortieren ziehen
chat-pin-from-mail = E-Mail von { $name } · { $when }
chat-pin-from-file = Datei von { $name } · { $when }
chat-pin-from-text = Text von { $name } · { $when }
chat-pins-full = Dieser Chat hat bereits 5 angeheftete Elemente
chat-pins-replace-title = Angeheftetes ersetzen
chat-pins-replace-hint = Ein Chat fasst bis zu 5 angeheftete Elemente. Wählen Sie das, das weichen soll.
chat-pins-replace = Ersetzen
chat-pins-cancel = Abbrechen
chat-undo = Rückgängig

chat-reply-to = Antwort an { $names }
chat-send = Senden (Strg+Eingabe). Rechtsklick oder gedrückt halten für mehr
chat-send-now = Jetzt senden
chat-attach = Anhängen
chat-attach-photo = Foto
chat-attach-file = Datei
chat-attach-library = Aus Dateien
chat-attach-template = Vorlage
chat-attach-signature = Signatur
chat-replying-to = Antwort an { $name }
chat-reply-newest = Auf die neueste E-Mail antworten

## The attach picker (paperclip > From Files)

picker-title = Aus Dateien anhängen
picker-search = Namen, Personen, Betreffe suchen
picker-search-drive = Dieses Laufwerk durchsuchen
picker-mail-files = E-Mail-Dateien
picker-this-chat = Diese Konversation
picker-this-computer = Dieser Computer…
picker-in-chat = IN DIESER KONVERSATION
picker-recent = ZULETZT
picker-preview = Vorschau
picker-cancel = Abbrechen
picker-attach = Anhängen
picker-attach-count = { $count } anhängen
picker-selected = { $count } ausgewählt
picker-of-limit = von { $limit }
picker-in-mail = { $size } in der E-Mail
picker-drive-links = { $count ->
    [one] 1 als Google-Drive-Link
   *[other] { $count } als Google-Drive-Links
}
picker-onedrive-links = { $count ->
    [one] 1 als OneDrive-Link
   *[other] { $count } als OneDrive-Links
}
picker-over = { $size }, mehr als die { $limit }, die eine E-Mail fassen kann
picker-getting = { $count ->
    [one] Die Datei wird vom Laufwerk geholt…
   *[other] { $count } Dateien werden vom Laufwerk geholt…
}
picker-some-failed = { $count ->
    [one] Eine Datei konnte nicht gelesen werden
   *[other] { $count } Dateien konnten nicht gelesen werden
}
