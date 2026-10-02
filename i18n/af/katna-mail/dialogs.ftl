# Katna Mail, Afrikaans (Afrikaans).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = Meer oor Katna
about-tagline = E-pos en kalender vir die Linux-werkskerm
about-whats-new = Wat's nuut
about-update-not-checked = Opdaterings is nog nie nagegaan nie
about-update-checking = Gaan tans opdaterings na…
about-update-up-to-date = Katna Mail is op datum
about-update-check-failed = Kon nie opdaterings nagaan nie
about-update-available = Weergawe { $version } is beskikbaar
about-update-downloading = Laai weergawe { $version } af… { $percent }%
about-update-download-failed = Die aflaai van weergawe { $version } het nie voltooi nie
about-update-ready = Weergawe { $version } is gereed om te installeer
about-update-ready-detail = Katna Mail herbegin om die opdatering te voltooi.
about-update-confirm = Installeer weergawe { $version }?
about-update-confirm-detail = Katna Mail sal toemaak, die opdatering installeer en weer oopmaak waar jy opgehou het. Jou rekenaar sal vir jou wagwoord vra.
about-update-installing = Installeer weergawe { $version }…
about-update-installing-detail = Tik jou wagwoord in die venster wat oopgegaan het.
about-update-cancelled = Die opdatering is nie geïnstalleer nie, omdat die wagwoord nie gegee is nie.
about-update-failed = Die opdatering kon nie geïnstalleer word nie: { $error }
about-update-unsupported = Hierdie kopie van Katna Mail word deur jou pakketbestuurder opgedateer.
about-update-restart-failed = Die opdatering is geïnstalleer, maar Katna Mail kon nie weer oopmaak nie ({ $error }). Maak dit self oop.
about-update-check = Gaan opdaterings na
about-update-download = Laai af
about-update-retry = Probeer weer
about-update-button = Opdateer
about-update-restart = Opdateer en herbegin
about-update-cancel = Nie nou nie
about-changelog = Veranderingslog
about-source = Bronkode
about-coffee = Koop vir my 'n koffie
about-coffee-coffee = Koffie?
about-coffee-tea = Tee?
about-coffee-pizza = Pizza?
about-coffee-nothing = Niks? Glad niks?
about-coffee-water = Ek oorleef op water!!
about-coffee-thanks = Dankie dat jy Katna gebruik
about-coming-soon = Kom binnekort
about-follow-me = Volg my op
about-love-title = Met liefde gemaak vir Rust, KDE en Linux
about-love-text = Rust maak dit 'n plesier om 'n vinnige en veilige e-posprogram te skryf: Katna het geen unsafe-kode nie. KDE se Plasma-werkskerm en sy PIM-suite het Katna geïnspireer, en Linux en die vryesagteware-gemeenskap bou die grond waarop dit staan. Dankie, en dankie aan die biblioteke hieronder.
about-kde-text = KDE bou die werkskerm waarop Katna die meeste tuis voel, en dit word deur vrywilligers gemaak en deur mense soos jy befonds. As jy van Plasma of KDE se programme hou, oorweeg asseblief 'n skenking aan KDE.
about-donate-kde = Skenk aan KDE
about-gpui-title = Gebou op GPUI, van die Zed-projek
about-gpui-text = Katna Mail se hele koppelvlak is gebou op GPUI, die vinnige, GPU-versnelde UI-raamwerk wat Zed Industries vir die Zed-redigeerder gemaak het. Elke pixel, animasie en venster wat jy sien, word daardeur geteken. Dankie, Zed-span, dat julle dit in die openbaar bou. Apache-2.0.
about-gpui-github = GPUI op GitHub
about-personal-title = 'n Persoonlike projek
about-personal-text = Katna Mail probeer nie nuut of revolusionêr wees nie. Dit is die e-posprogram wat sy outeur wou hê, en sy funksies en voorkoms is by Gmail, Mailspring en Thunderbird geleen. Dit was net moontlik omdat LLM's so ver gekom het.
about-built-on = GEBOU OP VRYE SAGTEWARE
about-credit-pimalaya = IMAP, SMTP en aanmelding (io-imap, io-smtp, io-sasl)
about-credit-imap-codec = IMAP lees en skryf
about-credit-tantivy = Soek
about-credit-sqlite = Die e-posstoor
about-credit-rustls = Veilige verbindings
about-credit-mail-parser = E-pos lees, van Stalwart Labs
about-credit-html5ever = HTML-e-pos, van die Servo-projek
about-credit-zbus = Praat met die werkskerm oor D-Bus en portale
about-credit-oo7 = Wagwoorde in die werkskerm se sleutelring
about-credit-hayro = PDF's bekyk en druk
about-credit-calamine = Voorskoue van sigblaaie
about-credit-resvg = SVG-prente
about-credit-jiff = Datums en tydsones
about-credit-spellbook = Speltoets, van die Helix-redigeerder
about-credit-smol = Baie dinge gelyk doen
about-credit-color-schemes = Die palette van die ingeboude kleurskemas
about-all-libraries = Elke biblioteek wat Katna gebruik ({ $count })
about-library-authors = deur { $authors }
about-license = Katna is vrye sagteware onder die GNU GPL, weergawe 3 of later.
about-close = Maak toe

## What’s new (shown after an update)

whats-new-title = Wat's nuut in Katna Mail
whats-new-updated = Opgedateer na weergawe { $version }
whats-new-version = Weergawe { $version }
whats-new-more = { $count ->
    [one] En nog een in die volledige veranderingslog.
   *[other] En nog { $count } in die volledige veranderingslog.
}
whats-new-changelog = Volledige veranderingslog
whats-new-got-it = Reg so

## First run: welcome page

onboarding-welcome-title = Welkom by Katna Mail
onboarding-welcome-lead = Jou e-pos op jou eie rekenaar: vinnig om te deursoek, leesbaar vanlyn en privaat.
onboarding-fast-title = Vinnig, selfs vanlyn
onboarding-fast-text = Katna hou 'n kopie van jou e-pos hier, sodat dit oombliklik oopmaak en deursoek word, met of sonder 'n verbinding.
onboarding-providers-title = Werk met jou e-pos
onboarding-providers-text = Gmail, Outlook, Yahoo, iCloud en enige ander IMAP- of POP-rekening.
onboarding-private-title = Privaat
onboarding-private-text = Jou e-pos gaan reguit van jou verskaffer na hierdie rekenaar. Geen Katna-bediener sien dit nie.
onboarding-get-started = Begin

## First run: adding an account

onboarding-service-checking = Kontroleer tans die Katna-agtergronddiens…
onboarding-service-running = Die Katna-agtergronddiens loop.
onboarding-service-missing = Die Katna-agtergronddiens loop nie
onboarding-service-start = Dit haal en stuur jou e-pos. Begin dit vanuit 'n terminaal en kontroleer dan weer:
onboarding-check-again = Kontroleer weer
onboarding-account-title = Voeg jou e-posrekening by
onboarding-account-lead = Tik jou e-posadres en wagwoord, en Katna vind die bedienerinstellings. Gmail, Yahoo en iCloud het 'n programwagwoord nodig, wat jy in jou rekening se sekuriteitsinstellings maak.
onboarding-add-account = Voeg 'n rekening by
onboarding-back = Terug

## First run: choosing the look

onboarding-look-title = Maak dit joune
onboarding-look-lead = Kies hoe e-pos oopmaak en hoe Katna lyk. Jy kan dit enige tyd in vinnige instellings verander.
onboarding-reading-pane = Leesvenster
onboarding-pane-right = Regs van die lys
onboarding-pane-none = Geen verdeling
onboarding-theme = Tema
onboarding-theme-system = Stelsel
onboarding-theme-light = Lig
onboarding-theme-dark = Donker
onboarding-density = Digtheid
onboarding-density-default = Verstek
onboarding-density-compact = Kompak
onboarding-continue = Gaan voort

## First start: the Katna account page. A Katna account is an account on
## Katna's own server, not a mail account; see katna-account.ftl.

onboarding-katna-title = Kry meer met 'n Katna-rekening
onboarding-katna-lead = Dit is opsioneel. Dit skakel Katna se aanlyn kenmerke aan, en jy kan later een maak in Instellings > Intekening.
onboarding-katna-receipts-title = Leesbewyse
onboarding-katna-receipts-text = Sien wanneer mense die e-pos wat jy stuur oopmaak.
onboarding-katna-links-title = Skakelnasporing
onboarding-katna-links-text = Sien watter skakels in jou e-pos geklik word.
onboarding-katna-activity-title = Aktiwiteit
onboarding-katna-activity-text = Wie oopmaak en klik, vir alles wat jy gestuur het, op een plek.
onboarding-katna-translate-title = Outomatiese vertaling
onboarding-katna-translate-text = Lees e-pos in ander tale in jou eie taal.
onboarding-katna-private = Dit het sy eie wagwoord. Jou e-posaanmeldings verlaat nooit hierdie rekenaar nie.

## First run: done

onboarding-ready-title = Jy is gereed
onboarding-ready-lead = Katna haal tans jou e-pos. Dit verskyn soos dit aankom, en nuwe e-pos verskyn vanself.
onboarding-ready-lead-address = Katna haal tans die e-pos van { $address }. Dit verskyn soos dit aankom, en nuwe e-pos verskyn vanself.
onboarding-ready-tour = Neem 'n toer van een minuut om te sien waar alles is?
onboarding-skip = Slaan nou oor
onboarding-take-tour = Neem die toer

## Asking to send crash reports (on its own and on the first-run pages)

share-title = Help om Katna te verbeter
share-lead = Wanneer Katna omval, stoor dit 'n verslag op hierdie rekenaar. Om hierdie verslae te stuur, help om reg te maak wat verkeerd geloop het. Jy kan dit enige tyd in Instellings > Gebruikersterugvoer verander.
share-sent = Wat gestuur word
share-sent-detail = Die omvalverslag soos jy dit in Instellings kan bekyk: wat omgeval het en waar in Katna, die weergawe, jou Linux-stelsel en werkskerm, en Katna se laaste loglyne, wat e-posvouers kan noem.
share-never-sent = Wat nooit gestuur word nie
share-never-sent-detail = Jou boodskappe, kontakte, wagwoorde, IP-adres, gebruikersnaam of rekenaarnaam. E-posadresse word uit die verslag verwyder.
share-where = Waarheen dit gaan
share-where-detail = Katna se omvalspoorder by Sentry, gestoor in die EU. Geen ID koppel verslae aan jou nie.
share-dont-send = Moenie stuur nie
share-send = Stuur omvalverslae
share-sending = Omvalverslae sal gestuur word. Dankie.
share-local = Omvalverslae bly op hierdie rekenaar.

## The tour (cards pointing at each part of the window)

tour-welcome-title = Welkom by Katna Mail
tour-welcome-text = 'n Toer van een minuut wys waar alles is.
tour-not-now = Nie nou nie
tour-start = Neem die toer
tour-close = Maak toe
tour-skip = Slaan toer oor
tour-back = Terug
tour-done = Klaar
tour-next = Volgende
tour-step = { $step } van { $total }
tour-compose-title = Skryf 'n boodskap
tour-compose-text = Skryf maak 'n nuwe boodskap regs onder oop, sodat jy kan aanhou lees terwyl jy skryf.
tour-search-title = Deursoek al jou e-pos
tour-search-text = Soek werk ook vanlyn. Die knoppie heel regs voeg filters by: sender, ontvanger, onderwerp, datums en aanhegsels.
tour-menu-title = Wys of versteek die vouers
tour-menu-text = Hierdie knoppie vou die vouerlys weg. Terwyl dit versteek is, laat die wyser op E-pos links rus om die vouers te sien.
tour-apps-title = Jou programme
tour-apps-text = E-pos woon hier, langs Kalender, Kontakte, Take, Notas en Lêers.
tour-tabs-title = Inkassie-oortjies
tour-tabs-text = Nuwe e-pos word in Primêr, Promosies, Sosiaal, Opdaterings en Forums gesorteer. Jy kan die oortjies in vinnige instellings afskakel.
tour-list-title = Jou boodskappe
tour-list-text = Klik op 'n boodskap om dit te lees. Beweeg daaroor vir vinnige aksies, regsklik vir meer, of merk verskeie om saam daarop te reageer.
tour-settings-title = Vinnige instellings
tour-settings-text = Verander die leesvenster, digtheid en tema hier. Die toer kan ook weer van daar af begin word.
tour-account-title = Jou rekening
tour-account-text = Sien in watter rekening jy is, en voeg nog een by.

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] Katna se agtergronddiens het onverwags gestop.
    [one] Katna se agtergronddiens het onverwags gestop. Nog een omvalverslag is gestoor.
   *[other] Katna se agtergronddiens het onverwags gestop. Nog { $more } omvalverslae is gestoor.
}
crash-mail = { $more ->
    [0] Katna Mail het laas onverwags toegemaak.
    [one] Katna Mail het laas onverwags toegemaak. Nog een omvalverslag is gestoor.
   *[other] Katna Mail het laas onverwags toegemaak. Nog { $more } omvalverslae is gestoor.
}
crash-view = Bekyk verslag
crash-view-tooltip = Maak die verslag oop, gestoor op hierdie rekenaar
crash-copy = Kopieer verslag
crash-close = Maak toe
sign-in-again-text = { $provider } vra dat jy weer by { $address } aanmeld.
sign-in-again-button = Meld aan
sign-in-again-tooltip = Maak die { $provider }-aanmeldbladsy in jou blaaier oop
sign-in-again-waiting = Wag tans vir jou blaaier…
sign-in-again-close = Maak toe
google-api-off = { $api } is afgeskakel in Katna se Google Cloud-projek.
google-api-turn-on = Skakel aan
google-api-turn-on-tooltip = Maak Google Cloud oop om { $api } aan te skakel, en druk dan Probeer weer
sign-in-again-done = Weer by { $address } aangemeld. Haal tans jou e-pos…
delete-ask-title = { $kind ->
    [conversation] { $count ->
        [one] Skuif hierdie gesprek na die asblik?
       *[other] Skuif { $count } gesprekke na die asblik?
    }
   *[message] { $count ->
        [one] Skuif hierdie boodskap na die asblik?
       *[other] Skuif { $count } boodskappe na die asblik?
    }
}
delete-ask-body = { $count ->
    [one] Jy kan dit net daarna ontdoen, of dit later uit die asblik terugbring.
   *[other] Jy kan dit net daarna ontdoen, of hulle later uit die asblik terugbring.
}
delete-ask-confirm = Skuif na asblik
delete-forever-title = { $kind ->
    [conversation] { $count ->
        [one] Vee hierdie gesprek permanent uit?
       *[other] Vee { $count } gesprekke permanent uit?
    }
   *[message] { $count ->
        [one] Vee hierdie boodskap permanent uit?
       *[other] Vee { $count } boodskappe permanent uit?
    }
}
delete-forever-body = { $count ->
    [one] Dit word ook op die bediener uitgevee. Dit kan nie ontdoen word nie.
   *[other] Hulle word ook op die bediener uitgevee. Dit kan nie ontdoen word nie.
}
delete-forever-confirm = Vee permanent uit
delete-ask-dont-ask = Vra nie weer nie
delete-ask-cancel = Kanselleer
