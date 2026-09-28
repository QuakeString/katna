# Katna Mail, Afrikaans (Afrikaans).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Voeg 'n e-posrekening by
add-account-looking = Soek tans die e-posbedieners van { $address }…
add-account-address-intro = Voer jou e-posadres in. Katna vind die bedieners vir jou.
add-account-servers-title = Bedienerinstellings
add-account-servers-intro = Waar Katna e-pos vir { $address } lees en stuur.
add-account-password-title = Voer jou wagwoord in
add-account-signing-in = Meld tans aan…
add-account-browser-title = Gaan voort in jou blaaier
add-account-browser-intro = Katna het die { $provider }-aanmeldbladsy in jou blaaier oopgemaak. Meld daar aan en laat Katna toe om jou e-pos te lees en te stuur, en kom dan hierheen terug.
add-account-browser-hint = Geen bladsy oopgemaak nie? Kyk na jou blaaier se vensters, of gaan terug en probeer weer.

## Add a mail account: fields

add-account-field-address = E-posadres
add-account-incoming = Inkomende e-pos ({ $protocol })
add-account-outgoing = Uitgaande e-pos ({ $protocol })
add-account-field-server = Bediener
add-account-field-port = Poort
add-account-security-none = Geen
add-account-field-username = Gebruikersnaam
add-account-field-password = Wagwoord
add-account-show-password = Wys wagwoord
add-account-app-password-hint = { $provider } het hier 'n programwagwoord nodig, nie die een wat jy op die web gebruik nie. Maak een in die sekuriteitsinstellings van jou { $provider }-rekening.
add-account-field-name = Jou naam (opsioneel)
add-account-name-hint = Word gewys aan die mense aan wie jy skryf.
add-account-servers-pair = { $imap } en { $smtp }
add-account-servers-found = { $source ->
    [built-in] Bedieners: { $servers }, gevind in Katna se lys van verskaffers.
    [provider] Bedieners: { $servers }, gevind in jou verskaffer se instellings.
    [ispdb] Bedieners: { $servers }, gevind in Thunderbird se lys van verskaffers.
    [dns] Bedieners: { $servers }, gevind in jou domein se DNS-rekords.
   *[other] Bedieners: { $servers }, geraai; kontroleer hulle as aanmelding misluk.
}
add-account-servers-entered = Bedieners: { $servers }, soos ingevoer.
add-account-or = of
add-account-sign-in-with = Meld aan met { $provider }
add-account-sign-in-instead = Meld eerder aan met { $provider }

## Add a mail account: buttons

add-account-servers-button = Bedienerinstellings
add-account-back = Terug
add-account-add = Voeg rekening by
add-account-next = Volgende
add-account-cancel = Kanselleer

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] Voer die inkomende bediener in.
   *[outgoing] Voer die uitgaande bediener in.
}
add-account-server-space = { $kind ->
    [incoming] Die inkomende bediener se naam het 'n spasie daarin.
   *[outgoing] Die uitgaande bediener se naam het 'n spasie daarin.
}
add-account-port-invalid = { $kind ->
    [incoming] Die inkomende poort moet 'n getal van { $min } tot { $max } wees.
   *[outgoing] Die uitgaande poort moet 'n getal van { $min } tot { $max } wees.
}
add-account-address-empty = Voer 'n e-posadres in.
add-account-address-invalid = Voer 'n e-posadres in soos { $example }.
add-account-not-found = Katna kon nie die bedieners vir { $address } vind nie, en het dus die gewone name ingevul. Kontroleer hulle by jou verskaffer.
add-account-password-empty = Voer die wagwoord in.
add-account-name-is-password = Die naam is dieselfde as die wagwoord. Tik eerder jou naam daar, soos mense dit moet sien.
add-account-added = { $address } bygevoeg. Haal tans jou e-pos…
add-account-app-password-refused = { $provider } het die wagwoord geweier. Dit het 'n programwagwoord nodig, nie die een wat jy op die web gebruik nie.
add-account-password-refused = Die bediener het die wagwoord geweier. Kontroleer dit en probeer weer.
add-account-sign-in-refused = { $provider } het Katna nie ingelaat nie. Probeer weer, en gee toegang tot jou e-pos.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Hierdie kopie van Katna kan nog nie by Microsoft-rekeninge aanmeld nie.
    [Google] Hierdie kopie van Katna kan nog nie by Google-rekeninge aanmeld nie.
   *[other] Hierdie verskaffer laat aanmelding net op sy eie bladsy toe, wat Katna nog nie daarvoor kan doen nie.
}
add-account-signed-in = Aangemeld met { $provider }. Haal tans jou e-pos…

## The account menu (from the account button on the top bar)

add-account-menu-another = Voeg nog 'n rekening by
add-account-menu-manage = Bestuur rekeninge
