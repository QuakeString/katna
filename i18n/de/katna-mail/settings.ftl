# Katna Mail, German (Deutsch).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

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
settings-general-start-at-login = Katna bei der Anmeldung starten
settings-general-start-at-login-detail = Synchronisiert E-Mails und zeigt Benachrichtigungen über neue E-Mails und das Symbol im Systemabschnitt, ohne das Fenster zu öffnen
settings-general-login-window = Auch das Fenster von Katna Mail öffnen
settings-general-login-window-detail = Das Fenster öffnet sich ebenfalls bei der Anmeldung
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
settings-appearance-theme-system = System
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
settings-default-apps-documents-detail = Word (docx, doc), OpenDocument-Text (odt) und Folien (pptx, ppt, odp).
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
settings-general-desktop-summary = Katna bei der Anmeldung starten, das Symbol im Systemabschnitt und die Anzahl ungelesener Nachrichten am Symbol in der Kontrollleiste
settings-accounts-accounts-summary = Ein Konto hinzufügen oder entfernen oder sein Bild ändern
settings-appearance-density-summary = Standard- oder kompakte Zeilen in der Liste
settings-appearance-scaling-summary = Alles größer oder kleiner machen: Text, Symbole, Abstände und Trennlinien
settings-appearance-theme-summary = System, hell oder dunkel
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
settings-default-apps-documents-summary = Womit Word- und OpenDocument-Texte und Folien geöffnet werden
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

## Settings: opening at login

settings-open-at-login-failed = Starten bei der Anmeldung konnte nicht geändert werden: { $error }

## Settings > General > Time

settings-time = Uhrzeit
settings-clock-language = Wie in der Sprache üblich
settings-clock-12 = 12 Stunden, z. B. 2:05 PM
settings-clock-24 = 24 Stunden, z. B. 14:05
settings-time-summary = 12- oder 24-Stunden-Format oder wie in der Sprache üblich

## Settings > General > Default mail app, Settings > Compose > Grammar

settings-general-mail-app = Standard-Mail-App
settings-general-mail-app-detail = E-Mail-Links in anderen Apps und auf Websites öffnen hier eine neue Nachricht.
mail-app-is-default = Katna Mail ist Ihre Standard-Mail-App.
mail-app-is-other = E-Mail-Links werden in einer anderen App geöffnet.
mail-app-make-default = Als Standard festlegen
mail-app-make-default-failed = Die Standard-Mail-App konnte nicht geändert werden.
settings-general-mail-app-summary = E-Mail-Links aus anderen Apps und von Websites in Katna Mail öffnen
settings-compose-grammar = Grammatik
settings-compose-grammar-detail = Wird auf diesem Computer mit Harper geprüft. Vorerst nur Englisch: Text in anderen Sprachen bleibt unberührt.
settings-compose-grammar-check = Grammatik prüfen
settings-compose-grammar-check-detail = Grammatikfehler beim Schreiben unterstreichen, auf Englisch
settings-compose-suggestions = Schreibvorschläge
settings-compose-suggestions-detail = Auf diesem Computer aus den E-Mails gelernt, die Sie gesendet haben und auf die Sie antworten; nichts verlässt ihn. Drücken Sie Tab, um einen Vorschlag zu übernehmen, oder schreiben Sie einfach weiter.
settings-compose-suggestions-on = Beim Schreiben Vorschläge machen
settings-compose-suggestions-on-detail = Den wahrscheinlichen Rest einer Wendung beim Tippen grau anzeigen
settings-compose-grammar-summary = Grammatikfehler beim Schreiben unterstreichen, auf Englisch
settings-compose-suggestions-summary = Den wahrscheinlichen Rest einer Wendung beim Tippen grau anzeigen
