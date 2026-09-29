# Katna Mail, Swedish (Svenska).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = Om Katna
about-tagline = E-post och kalender för Linux-skrivbordet
about-whats-new = Nyheter

about-update-not-checked = Uppdateringar har inte kontrollerats än
about-update-checking = Söker efter uppdateringar…
about-update-up-to-date = Katna Mail är uppdaterat
about-update-check-failed = Det gick inte att söka efter uppdateringar
about-update-available = Version { $version } är tillgänglig
about-update-downloading = Hämtar version { $version }… { $percent } %
about-update-download-failed = Hämtningen av version { $version } slutfördes inte
about-update-ready = Version { $version } är redo att installeras
about-update-ready-detail = Katna Mail startar om för att slutföra uppdateringen.
about-update-confirm = Installera version { $version }?
about-update-confirm-detail = Katna Mail stängs, installerar uppdateringen och öppnas igen där du var. Datorn frågar efter ditt lösenord.
about-update-installing = Installerar version { $version }…
about-update-installing-detail = Ange ditt lösenord i fönstret som öppnades.
about-update-cancelled = Uppdateringen installerades inte, eftersom lösenordet inte angavs.
about-update-failed = Uppdateringen kunde inte installeras: { $error }
about-update-unsupported = Den här kopian av Katna Mail uppdateras av din pakethanterare.
about-update-restart-failed = Uppdateringen är installerad, men Katna Mail kunde inte öppnas igen ({ $error }). Öppna det själv.
about-update-check = Sök efter uppdateringar
about-update-download = Hämta
about-update-retry = Försök igen
about-update-button = Uppdatera
about-update-restart = Uppdatera och starta om
about-update-cancel = Inte nu
about-changelog = Ändringslogg
about-source = Källkod
about-coffee = Bjud mig på en kaffe
about-coffee-coffee = Kaffe?
about-coffee-tea = Te?
about-coffee-pizza = Pizza?
about-coffee-nothing = Inget? Alls inget?
about-coffee-water = Jag överlever på vatten!!
about-coffee-thanks = Tack för att du använder Katna
about-coming-soon = Kommer snart
about-follow-me = Följ mig på
about-love-title = Gjord med kärlek till Rust, KDE och Linux
about-love-text = Rust gör det till en glädje att skriva en snabb och säker e-postapp: Katna har ingen unsafe-kod. KDE:s Plasma-skrivbord och dess PIM-svit inspirerade Katna, och Linux och fri programvara-gemenskapen bygger marken den står på. Tack, och tack till biblioteken nedan.
about-kde-text = KDE bygger skrivbordet där Katna känner sig mest hemma, och det görs av frivilliga och finansieras av personer som du. Om du tycker om Plasma eller KDE:s appar, överväg att donera till KDE.
about-donate-kde = Donera till KDE
about-gpui-title = Byggd på GPUI, från Zed-projektet
about-gpui-text = Hela Katna Mails gränssnitt är byggt på GPUI, det snabba, GPU-accelererade UI-ramverket som Zed Industries gjorde för Zed-redigeraren. Varje pixel, animering och fönster du ser ritas av det. Tack, Zed-teamet, för att ni bygger det öppet. Apache-2.0.
about-gpui-github = GPUI på GitHub
about-personal-title = Ett personligt projekt
about-personal-text = Katna Mail försöker inte vara nytt eller revolutionerande. Det är e-postappen som upphovspersonen själv ville ha, och funktionerna och utseendet är lånade från Gmail, Mailspring och Thunderbird. Det var bara möjligt tack vare hur långt LLM:er har kommit.
about-built-on = BYGGT PÅ FRI PROGRAMVARA
about-credit-pimalaya = IMAP, SMTP och inloggning (io-imap, io-smtp, io-sasl)
about-credit-imap-codec = Läsa och skriva IMAP
about-credit-tantivy = Sökning
about-credit-sqlite = E-postlagringen
about-credit-rustls = Säkra anslutningar
about-credit-mail-parser = Läsa e-post, från Stalwart Labs
about-credit-html5ever = HTML-e-post, från Servo-projektet
about-credit-zbus = Prata med skrivbordet via D-Bus och portaler
about-credit-oo7 = Lösenord i skrivbordets nyckelring
about-credit-hayro = Visa och skriva ut PDF:er
about-credit-calamine = Förhandsvisning av kalkylblad
about-credit-resvg = SVG-bilder
about-credit-jiff = Datum och tidszoner
about-credit-spellbook = Stavningskontroll, från Helix-redigeraren
about-credit-smol = Göra många saker samtidigt
about-all-libraries = Alla bibliotek som Katna använder ({ $count })
about-library-authors = av { $authors }
about-license = Katna är fri programvara under GNU GPL, version 3 eller senare.
about-close = Stäng

## What’s new (shown after an update)

whats-new-title = Nyheter i Katna Mail
whats-new-updated = Uppdaterad till version { $version }
whats-new-version = Version { $version }
whats-new-more = { $count ->
    [one] Och en till i den fullständiga ändringsloggen.
   *[other] Och { $count } till i den fullständiga ändringsloggen.
}
whats-new-changelog = Fullständig ändringslogg
whats-new-got-it = Uppfattat

## First run: welcome page

onboarding-welcome-title = Välkommen till Katna Mail
onboarding-welcome-lead = Din e-post på din egen dator: snabb att söka i, läsbar offline och privat.
onboarding-fast-title = Snabb, även offline
onboarding-fast-text = Katna sparar en kopia av din e-post här, så att öppna och söka i den går direkt, med eller utan anslutning.
onboarding-providers-title = Fungerar med din e-post
onboarding-providers-text = Gmail, Outlook, Yahoo, iCloud och alla andra IMAP- eller POP-konton.
onboarding-private-title = Privat
onboarding-private-text = Din e-post går direkt från din leverantör till den här datorn. Ingen Katna-server ser den.
onboarding-get-started = Kom igång

## First run: adding an account

onboarding-service-checking = Kontrollerar Katnas bakgrundstjänst…
onboarding-service-running = Katnas bakgrundstjänst körs.
onboarding-service-missing = Katnas bakgrundstjänst körs inte
onboarding-service-start = Den hämtar och skickar din e-post. Starta den från en terminal och kontrollera sedan igen:
onboarding-check-again = Kontrollera igen
onboarding-account-title = Lägg till ditt e-postkonto
onboarding-account-lead = Skriv din e-postadress och ditt lösenord, så hittar Katna serverinställningarna. Gmail, Yahoo och iCloud kräver ett applösenord, som du skapar i kontots säkerhetsinställningar.
onboarding-add-account = Lägg till ett konto
onboarding-back = Tillbaka

## First run: choosing the look

onboarding-look-title = Gör den till din
onboarding-look-lead = Välj hur e-post öppnas och hur Katna ser ut. Du kan ändra det när som helst i snabbinställningarna.
onboarding-reading-pane = Läsruta
onboarding-pane-right = Till höger om listan
onboarding-pane-none = Ingen delning
onboarding-theme = Tema
onboarding-theme-system = System
onboarding-theme-light = Ljust
onboarding-theme-dark = Mörkt
onboarding-density = Täthet
onboarding-density-default = Standard
onboarding-density-compact = Kompakt
onboarding-continue = Fortsätt

## First run: done

onboarding-ready-title = Allt är klart
onboarding-ready-lead = Katna hämtar din e-post. Den dyker upp allteftersom den kommer, och ny e-post visas av sig själv.
onboarding-ready-lead-address = Katna hämtar e-posten för { $address }. Den dyker upp allteftersom den kommer, och ny e-post visas av sig själv.
onboarding-ready-tour = Vill du ta en rundtur på en minut för att se var allt finns?
onboarding-skip = Hoppa över nu
onboarding-take-tour = Ta rundturen

## Asking to send crash reports (on its own and on the first-run pages)

share-title = Hjälp till att förbättra Katna
share-lead = När Katna kraschar sparas en rapport på den här datorn. Att skicka rapporterna hjälper till att rätta det som gick fel. Du kan ändra det när som helst i Inställningar > Feedback.
share-sent = Vad som skickas
share-sent-detail = Kraschrapporten som du kan visa den i Inställningar: vad som kraschade och var i Katna, versionen, ditt Linux-system och skrivbord samt Katnas sista loggrader, som kan nämna e-postmappar.
share-never-sent = Vad som aldrig skickas
share-never-sent-detail = Dina meddelanden, kontakter, lösenord, IP-adress, användarnamn eller datornamn. E-postadresser tas bort från rapporten.
share-where = Vart den skickas
share-where-detail = Katnas kraschspårare hos Sentry, lagrad i EU. Inget ID kopplar rapporterna till dig.
share-dont-send = Skicka inte
share-send = Skicka kraschrapporter
share-sending = Kraschrapporter kommer att skickas. Tack.
share-local = Kraschrapporter stannar på den här datorn.

## The tour (cards pointing at each part of the window)

tour-welcome-title = Välkommen till Katna Mail
tour-welcome-text = En rundtur på en minut visar var allt finns.
tour-not-now = Inte nu
tour-start = Ta rundturen
tour-close = Stäng
tour-skip = Hoppa över rundturen
tour-back = Tillbaka
tour-done = Klar
tour-next = Nästa
tour-step = { $step } av { $total }
tour-compose-title = Skriv ett meddelande
tour-compose-text = Skriv öppnar ett nytt meddelande längst ned till höger, så att du kan fortsätta läsa medan du skriver.
tour-search-title = Sök i all din e-post
tour-search-text = Sökning fungerar även offline. Knappen längst till höger lägger till filter: avsändare, mottagare, ämne, datum och bilagor.
tour-menu-title = Visa eller dölj mapparna
tour-menu-text = Den här knappen fäller undan mapplistan. Medan den är dold kan du vila pekaren på E-post till vänster för att se mapparna.
tour-apps-title = Dina appar
tour-apps-text = E-post bor här nu. Kalender, Kontakter, Uppgifter, Anteckningar och Flöden kommer att ansluta i det här fältet.
tour-tabs-title = Inkorgsflikar
tour-tabs-text = Ny e-post sorteras i Primär, Kampanjer, Socialt, Uppdateringar och Forum. Du kan stänga av flikarna i snabbinställningarna.
tour-list-title = Dina meddelanden
tour-list-text = Klicka på ett meddelande för att läsa det. Håll pekaren över det för snabbåtgärder, högerklicka för fler, eller markera flera för att hantera dem tillsammans.
tour-settings-title = Snabbinställningar
tour-settings-text = Ändra läsruta, täthet och tema här. Rundturen kan också startas igen därifrån.
tour-account-title = Ditt konto
tour-account-text = Se vilket konto du är i och lägg till ett till.

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] Katnas bakgrundstjänst stoppades oväntat.
    [one] Katnas bakgrundstjänst stoppades oväntat. Ytterligare en kraschrapport är sparad.
   *[other] Katnas bakgrundstjänst stoppades oväntat. Ytterligare { $more } kraschrapporter är sparade.
}
crash-mail = { $more ->
    [0] Katna Mail stängdes oväntat förra gången.
    [one] Katna Mail stängdes oväntat förra gången. Ytterligare en kraschrapport är sparad.
   *[other] Katna Mail stängdes oväntat förra gången. Ytterligare { $more } kraschrapporter är sparade.
}
crash-view = Visa rapport
crash-view-tooltip = Öppna rapporten, sparad på den här datorn
crash-copy = Kopiera rapport
crash-close = Stäng
sign-in-again-text = { $provider } ber dig logga in på { $address } igen.
sign-in-again-button = Logga in
sign-in-again-tooltip = Öppna inloggningssidan för { $provider } i webbläsaren
sign-in-again-waiting = Väntar på webbläsaren…
sign-in-again-close = Stäng
sign-in-again-done = Inloggad på { $address } igen. Hämtar din e-post…
delete-ask-title = { $kind ->
    [conversation] { $count ->
        [one] Flytta konversationen till papperskorgen?
       *[other] Flytta { $count } konversationer till papperskorgen?
    }
   *[message] { $count ->
        [one] Flytta meddelandet till papperskorgen?
       *[other] Flytta { $count } meddelanden till papperskorgen?
    }
}
delete-ask-body = { $count ->
    [one] Du kan ångra det direkt efteråt, eller hämta tillbaka det från papperskorgen senare.
   *[other] Du kan ångra det direkt efteråt, eller hämta tillbaka dem från papperskorgen senare.
}
delete-ask-confirm = Flytta till papperskorgen
delete-forever-title = { $kind ->
    [conversation] { $count ->
        [one] Radera konversationen permanent?
       *[other] Radera { $count } konversationer permanent?
    }
   *[message] { $count ->
        [one] Radera meddelandet permanent?
       *[other] Radera { $count } meddelanden permanent?
    }
}
delete-forever-body = { $count ->
    [one] Det raderas även på servern. Det går inte att ångra.
   *[other] De raderas även på servern. Det går inte att ångra.
}
delete-forever-confirm = Radera permanent
delete-ask-dont-ask = Fråga inte igen
delete-ask-cancel = Avbryt
