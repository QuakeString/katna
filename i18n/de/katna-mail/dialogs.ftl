# Katna Mail, German (Deutsch).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = Über Katna
about-tagline = E-Mail und Kalender für den Linux-Desktop
about-whats-new = Neuigkeiten

about-update-not-checked = Es wurde noch nicht nach Updates gesucht
about-update-checking = Suche nach Updates…
about-update-up-to-date = Katna Mail ist auf dem neuesten Stand
about-update-check-failed = Es konnte nicht nach Updates gesucht werden
about-update-available = Version { $version } ist verfügbar
about-update-downloading = Version { $version } wird heruntergeladen… { $percent } %
about-update-download-failed = Der Download von Version { $version } wurde nicht abgeschlossen
about-update-ready = Version { $version } ist bereit zur Installation
about-update-ready-detail = Katna Mail startet neu, um das Update abzuschließen.
about-update-confirm = Version { $version } installieren?
about-update-confirm-detail = Katna Mail wird geschlossen, installiert das Update und öffnet sich wieder dort, wo Sie aufgehört haben. Ihr Computer fragt nach Ihrem Passwort.
about-update-installing = Version { $version } wird installiert…
about-update-installing-detail = Geben Sie Ihr Passwort in dem geöffneten Fenster ein.
about-update-cancelled = Das Update wurde nicht installiert, weil das Passwort nicht eingegeben wurde.
about-update-failed = Das Update konnte nicht installiert werden: { $error }
about-update-unsupported = Diese Kopie von Katna Mail wird von Ihrer Paketverwaltung aktualisiert.
about-update-restart-failed = Das Update ist installiert, aber Katna Mail konnte sich nicht erneut öffnen ({ $error }). Öffnen Sie es selbst.
about-update-check = Nach Updates suchen
about-update-download = Herunterladen
about-update-retry = Erneut versuchen
about-update-button = Aktualisieren
about-update-restart = Aktualisieren und neu starten
about-update-cancel = Nicht jetzt
about-changelog = Änderungsprotokoll
about-source = Quellcode
about-coffee = Spendieren Sie mir einen Kaffee
about-coffee-coffee = Kaffee?
about-coffee-tea = Tee?
about-coffee-pizza = Pizza?
about-coffee-nothing = Nichts? Gar nichts?
about-coffee-water = Ich überlebe auch von Wasser!!
about-coffee-thanks = Danke, dass du Katna nutzt
about-coming-soon = Demnächst verfügbar
about-follow-me = Folge mir auf
about-love-title = Mit Liebe für Rust, KDE und Linux gemacht
about-love-text = Mit Rust macht es Freude, eine schnelle und sichere E-Mail-App zu schreiben: Katna enthält keinen unsafe-Code. Der Plasma-Desktop von KDE und seine PIM-Suite haben Katna inspiriert, und Linux und die Freie-Software-Gemeinschaft bilden das Fundament, auf dem es steht. Danke, und danke auch den Bibliotheken unten.
about-kde-text = KDE baut die Arbeitsumgebung, in der sich Katna am meisten zu Hause fühlt. Sie wird von Freiwilligen gemacht und von Menschen wie Ihnen finanziert. Wenn Ihnen Plasma oder die Apps von KDE gefallen, denken Sie bitte über eine Spende an KDE nach.
about-donate-kde = An KDE spenden
about-gpui-title = Aufgebaut auf GPUI, aus dem Zed-Projekt
about-gpui-text = Die gesamte Oberfläche von Katna Mail ist mit GPUI gebaut, dem schnellen, GPU-beschleunigten UI-Framework, das Zed Industries für den Zed-Editor entwickelt hat. Jedes Pixel, jede Animation und jedes Fenster, das Sie sehen, wird damit gezeichnet. Danke, Zed-Team, dass ihr es offen entwickelt. Apache-2.0.
about-gpui-github = GPUI auf GitHub
about-personal-title = Ein persönliches Projekt
about-personal-text = Katna Mail will weder neu noch revolutionär sein. Es ist die E-Mail-App, die sich sein Autor gewünscht hat, und seine Funktionen und sein Aussehen sind von Gmail, Mailspring und Thunderbird entlehnt. Möglich war es nur, weil LLMs so weit gekommen sind.
about-built-on = AUF FREIER SOFTWARE AUFGEBAUT
about-credit-pimalaya = IMAP, SMTP und Anmeldung (io-imap, io-smtp, io-sasl)
about-credit-imap-codec = IMAP lesen und schreiben
about-credit-tantivy = Suche
about-credit-sqlite = Der E-Mail-Speicher
about-credit-rustls = Sichere Verbindungen
about-credit-mail-parser = E-Mails lesen, von Stalwart Labs
about-credit-html5ever = HTML-E-Mails, aus dem Servo-Projekt
about-credit-zbus = Kommunikation mit der Arbeitsumgebung über D-Bus und Portale
about-credit-oo7 = Passwörter im Schlüsselbund der Arbeitsumgebung
about-credit-hayro = PDFs ansehen und drucken
about-credit-calamine = Tabellenvorschauen
about-credit-resvg = SVG-Bilder
about-credit-jiff = Datum und Zeitzonen
about-credit-spellbook = Rechtschreibprüfung, aus dem Helix-Editor
about-credit-smol = Vieles gleichzeitig erledigen
about-all-libraries = Alle Bibliotheken, die Katna nutzt ({ $count })
about-library-authors = von { $authors }
about-license = Katna ist freie Software unter der GNU GPL, Version 3 oder neuer.
about-close = Schließen

## What’s new (shown after an update)

whats-new-title = Neuigkeiten in Katna Mail
whats-new-updated = Auf Version { $version } aktualisiert
whats-new-version = Version { $version }
whats-new-more = { $count ->
    [one] Und eine weitere im vollständigen Änderungsprotokoll.
   *[other] Und { $count } weitere im vollständigen Änderungsprotokoll.
}
whats-new-changelog = Vollständiges Änderungsprotokoll
whats-new-got-it = Verstanden

## First run: welcome page

onboarding-welcome-title = Willkommen bei Katna Mail
onboarding-welcome-lead = Ihre E-Mails auf Ihrem eigenen Computer: schnell durchsuchbar, offline lesbar und privat.
onboarding-fast-title = Schnell, auch offline
onboarding-fast-text = Katna behält hier eine Kopie Ihrer E-Mails, sodass Öffnen und Suchen sofort gehen, mit oder ohne Verbindung.
onboarding-providers-title = Funktioniert mit Ihren E-Mails
onboarding-providers-text = Gmail, Outlook, Yahoo, iCloud und jedes andere IMAP- oder POP-Konto.
onboarding-private-title = Privat
onboarding-private-text = Ihre E-Mails gehen direkt von Ihrem Anbieter auf diesen Computer. Kein Katna-Server sieht sie.
onboarding-get-started = Los geht’s

## First run: adding an account

onboarding-service-checking = Der Katna-Hintergrunddienst wird geprüft…
onboarding-service-running = Der Katna-Hintergrunddienst läuft.
onboarding-service-missing = Der Katna-Hintergrunddienst läuft nicht
onboarding-service-start = Er ruft Ihre E-Mails ab und sendet sie. Starten Sie ihn in einem Terminal und prüfen Sie dann erneut:
onboarding-check-again = Erneut prüfen
onboarding-account-title = Ihr E-Mail-Konto hinzufügen
onboarding-account-lead = Geben Sie Ihre E-Mail-Adresse und Ihr Passwort ein, und Katna findet die Servereinstellungen. Gmail, Yahoo und iCloud brauchen ein App-Passwort, das Sie in den Sicherheitseinstellungen Ihres Kontos erstellen.
onboarding-add-account = Konto hinzufügen
onboarding-back = Zurück

## First run: choosing the look

onboarding-look-title = Ganz nach Ihrem Geschmack
onboarding-look-lead = Wählen Sie, wie E-Mails geöffnet werden und wie Katna aussieht. Sie können das jederzeit in den Schnelleinstellungen ändern.
onboarding-reading-pane = Lesebereich
onboarding-pane-right = Rechts neben der Liste
onboarding-pane-none = Keine Unterteilung
onboarding-theme = Design
onboarding-theme-system = System
onboarding-theme-light = Hell
onboarding-theme-dark = Dunkel
onboarding-density = Dichte
onboarding-density-default = Standard
onboarding-density-compact = Kompakt
onboarding-continue = Weiter

## First run: done

onboarding-ready-title = Alles bereit
onboarding-ready-lead = Katna ruft Ihre E-Mails ab. Sie erscheinen, sobald sie ankommen, und neue E-Mails kommen von selbst hinzu.
onboarding-ready-lead-address = Katna ruft die E-Mails von { $address } ab. Sie erscheinen, sobald sie ankommen, und neue E-Mails kommen von selbst hinzu.
onboarding-ready-tour = Eine einminütige Tour machen, um zu sehen, wo alles ist?
onboarding-skip = Vorerst überspringen
onboarding-take-tour = Tour starten

## Asking to send crash reports (on its own and on the first-run pages)

share-title = Helfen Sie, Katna zu verbessern
share-lead = Wenn Katna abstürzt, speichert es einen Bericht auf diesem Computer. Wenn Sie diese Berichte senden, hilft das, den Fehler zu beheben. Sie können das jederzeit unter Einstellungen > Nutzerfeedback ändern.
share-sent = Was gesendet wird
share-sent-detail = Der Absturzbericht, so wie Sie ihn in den Einstellungen ansehen können: was wo in Katna abgestürzt ist, die Version, Ihr Linux-System und Ihre Arbeitsumgebung sowie die letzten Protokollzeilen von Katna, in denen E-Mail-Ordner vorkommen können.
share-never-sent = Was nie gesendet wird
share-never-sent-detail = Ihre Nachrichten, Kontakte, Passwörter, IP-Adresse, Ihr Benutzername oder Computername. E-Mail-Adressen werden aus dem Bericht entfernt.
share-where = Wohin er geht
share-where-detail = An Katnas Absturz-Tracker bei Sentry, gespeichert in der EU. Keine ID verknüpft die Berichte mit Ihnen.
share-dont-send = Nicht senden
share-send = Absturzberichte senden
share-sending = Absturzberichte werden gesendet. Danke.
share-local = Absturzberichte bleiben auf diesem Computer.

## The tour (cards pointing at each part of the window)

tour-welcome-title = Willkommen bei Katna Mail
tour-welcome-text = Eine einminütige Tour zeigt, wo alles ist.
tour-not-now = Nicht jetzt
tour-start = Tour starten
tour-close = Schließen
tour-skip = Tour überspringen
tour-back = Zurück
tour-done = Fertig
tour-next = Weiter
tour-step = { $step } von { $total }
tour-compose-title = Eine Nachricht schreiben
tour-compose-text = „Schreiben“ öffnet unten rechts eine neue Nachricht, sodass Sie beim Schreiben weiterlesen können.
tour-search-title = Alle E-Mails durchsuchen
tour-search-text = Die Suche funktioniert auch offline. Die Schaltfläche ganz rechts fügt Filter hinzu: Absender, Empfänger, Betreff, Datum und Anhänge.
tour-menu-title = Ordner ein- oder ausblenden
tour-menu-text = Diese Schaltfläche klappt die Ordnerliste weg. Solange sie ausgeblendet ist, halten Sie den Zeiger links auf „E-Mail“, um die Ordner zu sehen.
tour-apps-title = Ihre Apps
tour-apps-text = E-Mail ist jetzt hier zu Hause. Kalender, Kontakte, Aufgaben, Notizen und Feeds kommen in dieser Leiste hinzu.
tour-tabs-title = Posteingangs-Tabs
tour-tabs-text = Neue E-Mails werden in Allgemein, Werbung, Soziale Netzwerke, Benachrichtigungen und Foren sortiert. Sie können die Tabs in den Schnelleinstellungen ausschalten.
tour-list-title = Ihre Nachrichten
tour-list-text = Klicken Sie auf eine Nachricht, um sie zu lesen. Zeigen Sie darauf für Schnellaktionen, klicken Sie mit der rechten Maustaste für mehr, oder haken Sie mehrere an, um sie gemeinsam zu bearbeiten.
tour-settings-title = Schnelleinstellungen
tour-settings-text = Ändern Sie hier Lesebereich, Dichte und Design. Auch die Tour lässt sich dort erneut starten.
tour-account-title = Ihr Konto
tour-account-text = Sehen Sie, in welchem Konto Sie sind, und fügen Sie ein weiteres hinzu.

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] Katnas Hintergrunddienst wurde unerwartet beendet.
    [one] Katnas Hintergrunddienst wurde unerwartet beendet. Ein weiterer Absturzbericht ist gespeichert.
   *[other] Katnas Hintergrunddienst wurde unerwartet beendet. { $more } weitere Absturzberichte sind gespeichert.
}
crash-mail = { $more ->
    [0] Katna Mail wurde beim letzten Mal unerwartet beendet.
    [one] Katna Mail wurde beim letzten Mal unerwartet beendet. Ein weiterer Absturzbericht ist gespeichert.
   *[other] Katna Mail wurde beim letzten Mal unerwartet beendet. { $more } weitere Absturzberichte sind gespeichert.
}
crash-view = Bericht ansehen
crash-view-tooltip = Den auf diesem Computer gespeicherten Bericht öffnen
crash-copy = Bericht kopieren
crash-close = Schließen
sign-in-again-text = { $provider } bittet Sie, sich erneut bei { $address } anzumelden.
sign-in-again-button = Anmelden
sign-in-again-tooltip = Anmeldeseite von { $provider } im Browser öffnen
sign-in-again-waiting = Warten auf Ihren Browser…
sign-in-again-close = Schließen
sign-in-again-done = Erneut bei { $address } angemeldet. E-Mails werden abgerufen…
delete-ask-title = { $kind ->
    [conversation] { $count ->
        [one] Diese Konversation in den Papierkorb verschieben?
       *[other] { $count } Konversationen in den Papierkorb verschieben?
    }
   *[message] { $count ->
        [one] Diese Nachricht in den Papierkorb verschieben?
       *[other] { $count } Nachrichten in den Papierkorb verschieben?
    }
}
delete-ask-body = { $count ->
    [one] Sie können das gleich danach rückgängig machen oder sie später aus dem Papierkorb zurückholen.
   *[other] Sie können das gleich danach rückgängig machen oder sie später aus dem Papierkorb zurückholen.
}
delete-ask-confirm = In den Papierkorb
delete-forever-title = { $kind ->
    [conversation] { $count ->
        [one] Diese Konversation endgültig löschen?
       *[other] { $count } Konversationen endgültig löschen?
    }
   *[message] { $count ->
        [one] Diese Nachricht endgültig löschen?
       *[other] { $count } Nachrichten endgültig löschen?
    }
}
delete-forever-body = { $count ->
    [one] Sie wird auch auf dem Server gelöscht. Das lässt sich nicht rückgängig machen.
   *[other] Sie werden auch auf dem Server gelöscht. Das lässt sich nicht rückgängig machen.
}
delete-forever-confirm = Endgültig löschen
delete-ask-dont-ask = Nicht mehr fragen
delete-ask-cancel = Abbrechen
