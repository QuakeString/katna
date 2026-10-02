# Katna Mail, Dutch (Nederlands).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = Over Katna
about-tagline = E-mail en agenda voor de Linux-desktop
about-whats-new = Wat is er nieuw

about-update-not-checked = Er is nog niet op updates gecontroleerd
about-update-checking = Controleren op updates…
about-update-up-to-date = Katna Mail is up-to-date
about-update-check-failed = Kon niet op updates controleren
about-update-available = Versie { $version } is beschikbaar
about-update-downloading = Versie { $version } wordt gedownload… { $percent }%
about-update-download-failed = Het downloaden van versie { $version } is niet voltooid
about-update-ready = Versie { $version } is klaar om te installeren
about-update-ready-detail = Katna Mail herstart om de update te voltooien.
about-update-confirm = Versie { $version } installeren?
about-update-confirm-detail = Katna Mail sluit af, installeert de update en opent opnieuw waar je gebleven was. Je computer vraagt om je wachtwoord.
about-update-installing = Versie { $version } wordt geïnstalleerd…
about-update-installing-detail = Voer je wachtwoord in het geopende venster in.
about-update-cancelled = De update is niet geïnstalleerd, omdat het wachtwoord niet is opgegeven.
about-update-failed = De update kon niet worden geïnstalleerd: { $error }
about-update-unsupported = Deze versie van Katna Mail wordt bijgewerkt door je pakketbeheer.
about-update-restart-failed = De update is geïnstalleerd, maar Katna Mail kon niet opnieuw worden geopend ({ $error }). Open het zelf.
about-update-check = Controleren op updates
about-update-download = Downloaden
about-update-retry = Opnieuw proberen
about-update-button = Bijwerken
about-update-restart = Bijwerken en herstarten
about-update-cancel = Niet nu
about-changelog = Wijzigingslogboek
about-source = Broncode
about-coffee = Trakteer me op een koffie
about-coffee-coffee = Koffie?
about-coffee-tea = Thee?
about-coffee-pizza = Pizza?
about-coffee-nothing = Niks? Helemaal niks?
about-coffee-water = Ik overleef wel op water!!
about-coffee-thanks = Bedankt voor het gebruiken van Katna
about-coming-soon = Binnenkort beschikbaar
about-follow-me = Volg mij op
about-love-title = Met liefde gemaakt voor Rust, KDE en Linux
about-love-text = Met Rust is een snelle en veilige e-mailapp schrijven een plezier: Katna bevat geen unsafe code. De Plasma-desktop van KDE en zijn PIM-suite hebben Katna geïnspireerd, en Linux en de vrije-softwaregemeenschap vormen de grond waarop het staat. Dank je wel, en dank aan de bibliotheken hieronder.
about-kde-text = KDE maakt de desktop waarop Katna zich het meest thuis voelt, gebouwd door vrijwilligers en gefinancierd door mensen zoals jij. Als je Plasma of de apps van KDE waardeert, overweeg dan een donatie aan KDE.
about-donate-kde = Doneren aan KDE
about-gpui-title = Gebouwd op GPUI, van het Zed-project
about-gpui-text = De hele interface van Katna Mail is gebouwd op GPUI, het snelle, GPU-versnelde UI-framework dat Zed Industries voor de Zed-editor heeft gemaakt. Elke pixel, animatie en elk venster dat je ziet, wordt ermee getekend. Dank je wel, Zed-team, dat je het in de openbaarheid bouwt. Apache-2.0.
about-gpui-github = GPUI op GitHub
about-personal-title = Een persoonlijk project
about-personal-text = Katna Mail probeert niet nieuw of revolutionair te zijn. Het is de e-mailapp die de maker zelf wilde, en de functies en het uiterlijk zijn geleend van Gmail, Mailspring en Thunderbird. Het was alleen mogelijk doordat LLM’s zo ver zijn gekomen.
about-built-on = GEBOUWD OP VRIJE SOFTWARE
about-credit-pimalaya = IMAP, SMTP en aanmelden (io-imap, io-smtp, io-sasl)
about-credit-imap-codec = IMAP lezen en schrijven
about-credit-tantivy = Zoeken
about-credit-sqlite = De e-mailopslag
about-credit-rustls = Beveiligde verbindingen
about-credit-mail-parser = E-mail lezen, van Stalwart Labs
about-credit-html5ever = HTML-e-mail, van het Servo-project
about-credit-zbus = Praten met de desktop via D-Bus en portals
about-credit-oo7 = Wachtwoorden in de sleutelbos van de desktop
about-credit-hayro = Pdf’s bekijken en afdrukken
about-credit-calamine = Voorbeelden van spreadsheets
about-credit-resvg = SVG-afbeeldingen
about-credit-jiff = Datums en tijdzones
about-credit-spellbook = Spellingcontrole, van de Helix-editor
about-credit-smol = Veel dingen tegelijk doen
about-all-libraries = Alle bibliotheken die Katna gebruikt ({ $count })
about-library-authors = door { $authors }
about-license = Katna is vrije software onder de GNU GPL, versie 3 of later.
about-close = Sluiten

## What’s new (shown after an update)

whats-new-title = Wat is er nieuw in Katna Mail
whats-new-updated = Bijgewerkt naar versie { $version }
whats-new-version = Versie { $version }
whats-new-more = { $count ->
    [one] En nog één in het volledige wijzigingslogboek.
   *[other] En nog { $count } in het volledige wijzigingslogboek.
}
whats-new-changelog = Volledig wijzigingslogboek
whats-new-got-it = Begrepen

## First run: welcome page

onboarding-welcome-title = Welkom bij Katna Mail
onboarding-welcome-lead = Je e-mail op je eigen computer: snel doorzoekbaar, offline leesbaar en privé.
onboarding-fast-title = Snel, ook offline
onboarding-fast-text = Katna bewaart hier een kopie van je e-mail, zodat openen en zoeken direct gaat, met of zonder verbinding.
onboarding-providers-title = Werkt met je e-mail
onboarding-providers-text = Gmail, Outlook, Yahoo, iCloud en elk ander IMAP- of POP-account.
onboarding-private-title = Privé
onboarding-private-text = Je e-mail gaat rechtstreeks van je provider naar deze computer. Geen enkele Katna-server ziet hem.
onboarding-get-started = Aan de slag

## First run: adding an account

onboarding-service-checking = De achtergrondservice van Katna controleren…
onboarding-service-running = De achtergrondservice van Katna draait.
onboarding-service-missing = De achtergrondservice van Katna draait niet
onboarding-service-start = Deze haalt je e-mail op en verstuurt hem. Start hem vanuit een terminal en controleer dan opnieuw:
onboarding-check-again = Opnieuw controleren
onboarding-account-title = Voeg je e-mailaccount toe
onboarding-account-lead = Typ je e-mailadres en wachtwoord, en Katna vindt de serverinstellingen. Gmail, Yahoo en iCloud hebben een app-wachtwoord nodig, dat je maakt in de beveiligingsinstellingen van je account.
onboarding-add-account = Account toevoegen
onboarding-back = Terug

## First run: choosing the look

onboarding-look-title = Maak het je eigen
onboarding-look-lead = Kies hoe e-mail opent en hoe Katna eruitziet. Je kunt dit altijd wijzigen in de snelle instellingen.
onboarding-reading-pane = Leesvenster
onboarding-pane-right = Rechts van de lijst
onboarding-pane-none = Geen splitsing
onboarding-theme = Thema
onboarding-theme-system = Systeem
onboarding-theme-light = Licht
onboarding-theme-dark = Donker
onboarding-density = Dichtheid
onboarding-density-default = Standaard
onboarding-density-compact = Compact
onboarding-continue = Doorgaan

## First run: done

onboarding-ready-title = Alles is klaar
onboarding-ready-lead = Katna haalt je e-mail op. Die verschijnt zodra hij binnenkomt, en nieuwe e-mail komt er vanzelf bij.
onboarding-ready-lead-address = Katna haalt de e-mail van { $address } op. Die verschijnt zodra hij binnenkomt, en nieuwe e-mail komt er vanzelf bij.
onboarding-ready-tour = Een rondleiding van een minuut volgen om te zien waar alles zit?
onboarding-skip = Nu overslaan
onboarding-take-tour = Rondleiding volgen

## Asking to send crash reports (on its own and on the first-run pages)

share-title = Help Katna te verbeteren
share-lead = Als Katna crasht, bewaart het een rapport op deze computer. Door deze rapporten te versturen help je te herstellen wat er misging. Je kunt dit altijd wijzigen in Instellingen > Feedback.
share-sent = Wat wordt verstuurd
share-sent-detail = Het crashrapport zoals je het in Instellingen kunt bekijken: wat er crashte en waar in Katna, de versie, je Linux-systeem en desktop, en de laatste logregels van Katna, waarin namen van e-mailmappen kunnen staan.
share-never-sent = Wat nooit wordt verstuurd
share-never-sent-detail = Je berichten, contacten, wachtwoorden, IP-adres, gebruikersnaam of computernaam. E-mailadressen worden uit het rapport verwijderd.
share-where = Waar het naartoe gaat
share-where-detail = De crashtracker van Katna bij Sentry, opgeslagen in de EU. Geen ID koppelt rapporten aan jou.
share-dont-send = Niet versturen
share-send = Crashrapporten versturen
share-sending = Crashrapporten worden verstuurd. Dank je wel.
share-local = Crashrapporten blijven op deze computer.

## The tour (cards pointing at each part of the window)

tour-welcome-title = Welkom bij Katna Mail
tour-welcome-text = Een rondleiding van een minuut laat zien waar alles zit.
tour-not-now = Niet nu
tour-start = Rondleiding volgen
tour-close = Sluiten
tour-skip = Rondleiding overslaan
tour-back = Terug
tour-done = Klaar
tour-next = Volgende
tour-step = { $step } van { $total }
tour-compose-title = Een bericht schrijven
tour-compose-text = Opstellen opent een nieuw bericht rechtsonder, zodat je kunt blijven lezen terwijl je schrijft.
tour-search-title = Al je e-mail doorzoeken
tour-search-text = Zoeken werkt ook offline. De knop aan de rechterkant voegt filters toe: afzender, ontvanger, onderwerp, datums en bijlagen.
tour-menu-title = De mappen tonen of verbergen
tour-menu-text = Deze knop klapt de mappenlijst weg. Terwijl die verborgen is, laat je de aanwijzer op E-mail links rusten om de mappen te zien.
tour-apps-title = Je apps
tour-tabs-title = Inbox-tabbladen
tour-tabs-text = Nieuwe e-mail wordt gesorteerd in Primair, Reclame, Sociaal, Updates en Forums. Je kunt de tabbladen uitzetten in de snelle instellingen.
tour-list-title = Je berichten
tour-list-text = Klik op een bericht om het te lezen. Beweeg erover voor snelle acties, klik met rechts voor meer, of vink er meerdere aan om ze samen te bewerken.
tour-settings-title = Snelle instellingen
tour-settings-text = Wijzig hier het leesvenster, de dichtheid en het thema. Ook de rondleiding kun je daar opnieuw starten.
tour-account-title = Je account
tour-account-text = Zie in welk account je zit en voeg er nog een toe.

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] De achtergrondservice van Katna is onverwacht gestopt.
    [one] De achtergrondservice van Katna is onverwacht gestopt. Er is nog één crashrapport bewaard.
   *[other] De achtergrondservice van Katna is onverwacht gestopt. Er zijn nog { $more } crashrapporten bewaard.
}
crash-mail = { $more ->
    [0] Katna Mail is de vorige keer onverwacht gesloten.
    [one] Katna Mail is de vorige keer onverwacht gesloten. Er is nog één crashrapport bewaard.
   *[other] Katna Mail is de vorige keer onverwacht gesloten. Er zijn nog { $more } crashrapporten bewaard.
}
crash-view = Rapport bekijken
crash-view-tooltip = Het rapport openen, bewaard op deze computer
crash-copy = Rapport kopiëren
crash-close = Sluiten
sign-in-again-text = { $provider } vraagt je om je opnieuw aan te melden bij { $address }.
sign-in-again-button = Aanmelden
sign-in-again-tooltip = Open de aanmeldpagina van { $provider } in je browser
sign-in-again-waiting = Wachten op je browser…
sign-in-again-close = Sluiten
sign-in-again-done = Opnieuw aangemeld bij { $address }. Je e-mail wordt opgehaald…
delete-ask-title = { $kind ->
    [conversation] { $count ->
        [one] Dit gesprek naar de Prullenbak verplaatsen?
       *[other] { $count } gesprekken naar de Prullenbak verplaatsen?
    }
   *[message] { $count ->
        [one] Dit bericht naar de Prullenbak verplaatsen?
       *[other] { $count } berichten naar de Prullenbak verplaatsen?
    }
}
delete-ask-body = { $count ->
    [one] Je kunt het daarna meteen ongedaan maken, of het later uit de Prullenbak terughalen.
   *[other] Je kunt het daarna meteen ongedaan maken, of ze later uit de Prullenbak terughalen.
}
delete-ask-confirm = Naar Prullenbak
delete-forever-title = { $kind ->
    [conversation] { $count ->
        [one] Dit gesprek definitief verwijderen?
       *[other] { $count } gesprekken definitief verwijderen?
    }
   *[message] { $count ->
        [one] Dit bericht definitief verwijderen?
       *[other] { $count } berichten definitief verwijderen?
    }
}
delete-forever-body = { $count ->
    [one] Het wordt ook op de server verwijderd. Dit kan niet ongedaan worden gemaakt.
   *[other] Ze worden ook op de server verwijderd. Dit kan niet ongedaan worden gemaakt.
}
delete-forever-confirm = Definitief verwijderen
delete-ask-dont-ask = Niet meer vragen
delete-ask-cancel = Annuleren
