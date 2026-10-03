# Katna Mail, Afrikaans (Afrikaans).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Voeg 'n e-posrekening by
add-account-providers-intro = Kies jou e-posverskaffer. Katna vind die res.
add-account-provider-other = Ander e-pos
add-account-provider-other-detail = Enige IMAP- of POP3-rekening
add-account-provider-google-detail = Gmail en Google Workspace
add-account-provider-microsoft-detail = Outlook en Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = Meld aan by { $provider }
add-account-form-title-other = Jou e-posrekening
add-account-form-intro = Katna hou jou wagwoord in jou stelsel se sleutelring.
add-account-looking = Soek tans die e-posbedieners van { $address }…
add-account-address-intro = Voer jou e-posadres in. Katna vind die bedieners vir jou.
add-account-servers-title = Bedienerinstellings
add-account-servers-intro = Waar Katna e-pos vir { $address } lees en stuur.
add-account-signing-in = Meld tans aan…
add-account-browser-title = Gaan voort in jou blaaier
add-account-browser-intro = Katna het die { $provider }-aanmeldbladsy in jou blaaier oopgemaak. Meld daar aan en laat Katna toe om jou e-pos te lees en te stuur, en kom dan hierheen terug.
add-account-browser-hint = Geen bladsy oopgemaak nie? Kyk na jou blaaier se vensters, of gaan terug en probeer weer.
add-account-stage-browser = Wag dat jy in jou blaaier aanmeld…
add-account-stage-signing-in-at = Meld tans aan by { $server }…
add-account-help-app-password-link = Hoe om 'n programwagwoord te maak
add-account-help-turn-on-imap = { $provider } laat e-posprogramme eers toe sodra IMAP- en POP3-toegang in die instellings van sy webpos aangeskakel is.
add-account-help-turn-on-imap-link = Hoe om dit aan te skakel

## Add a mail account: fields

add-account-field-address = E-posadres
add-account-receive-with = Ontvang e-pos met
add-account-imap-about = IMAP hou jou e-pos en vouers op die bediener, dieselfde op elke toestel. Kies dit as jy kan.
add-account-pop3-about = POP3 laai jou e-pos na hierdie rekenaar af. E-pos wat jy hier lees of skuif, bly soos dit is op die bediener en jou ander toestelle.
add-account-incoming = Inkomende e-pos ({ $protocol })
add-account-outgoing = Uitgaande e-pos ({ $protocol })
add-account-field-server = Bediener
add-account-field-port = Poort
add-account-security-none = Geen
add-account-security-none-warning = Nie geënkripteer nie: jou wagwoord en pos kan onderweg gelees word.
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
add-account-sign-in-with = Meld aan met { $provider }
add-account-sign-in-instead = Meld eerder aan met { $provider }

## Add a mail account: buttons

add-account-servers-button = Bedienerinstellings
add-account-back = Terug
add-account-add = Voeg rekening by
add-account-done = Klaar
add-account-another = Voeg nog 'n rekening by
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
add-account-app-password-refused = { $provider } het die wagwoord geweier. Dit het 'n programwagwoord nodig, nie die een wat jy op die web gebruik nie.
add-account-password-refused = Die bediener het die wagwoord geweier. Kontroleer dit en probeer weer.
add-account-sign-in-refused = { $provider } het Katna nie ingelaat nie. Probeer weer, en gee toegang tot jou e-pos.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Hierdie kopie van Katna kan nog nie by Microsoft-rekeninge aanmeld nie.
    [Google] Hierdie kopie van Katna kan nog nie by Google-rekeninge aanmeld nie.
   *[other] Hierdie verskaffer laat aanmelding net op sy eie bladsy toe, wat Katna nog nie daarvoor kan doen nie.
}
add-account-smtp-not-found = Katna het gevind waar om jou e-pos te lees, maar nie waarheen om dit te stuur nie. Voer die uitgaande bediener in.

## Add a mail account: the last step

add-account-done-title = Jou rekening is gereed
add-account-done-intro = Katna haal nou jou e-pos. Nuwe e-pos verskyn soos dit aankom.
add-account-done-sign-in = Aanmelding
add-account-done-signed-in-with = Met { $provider }, in jou blaaier
add-account-done-receiving = Ontvang e-pos
add-account-done-sending = Stuur e-pos
add-account-done-on-server = E-pos op die bediener
add-account-done-kept = Gehou totdat jy dit in Katna uitvee
add-account-done-pop3-hint = Verander wat met e-pos op die bediener gebeur in Instellings > Rekeninge.
add-account-done-zoho-title = Take en kalenders
add-account-done-zoho-about = Zoho hou dit apart van e-pos. Meld een keer met Zoho aan om dit in Katna te bring.
add-account-done-linked = Take en kalenders gekoppel

## The account menu (from the account button on the top bar)

add-account-menu-another = Voeg nog 'n rekening by
app-menu = Hoofkieslys
app-menu-back = Terug
