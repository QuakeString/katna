# Katna Mail, Swahili (Kiswahili).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Ongeza akaunti ya barua
add-account-looking = Inatafuta seva za barua za { $address }…
add-account-address-intro = Weka anwani yako ya barua pepe. Katna itakutafutia seva.
add-account-servers-title = Mipangilio ya seva
add-account-servers-intro = Mahali Katna inaposoma na kutuma barua za { $address }.
add-account-password-title = Weka nenosiri lako
add-account-signing-in = Inaingia…

## Add a mail account: fields

add-account-field-address = Anwani ya barua pepe
add-account-incoming = Barua zinazoingia ({ $protocol })
add-account-outgoing = Barua zinazotoka ({ $protocol })
add-account-field-server = Seva
add-account-field-port = Mlango
add-account-security-none = Hakuna
add-account-field-username = Jina la mtumiaji
add-account-field-password = Nenosiri
add-account-show-password = Onyesha nenosiri
add-account-app-password-hint = { $provider } inahitaji nenosiri la programu hapa, si lile unalotumia kwenye wavuti. Tengeneza moja kwenye mipangilio ya usalama ya akaunti yako ya { $provider }.
add-account-field-name = Jina lako (si lazima)
add-account-name-hint = Huonyeshwa kwa watu unaowaandikia.
add-account-servers-pair = { $imap } na { $smtp }
add-account-servers-found = { $source ->
    [built-in] Seva: { $servers }, zimepatikana kwenye orodha ya watoa huduma ya Katna.
    [provider] Seva: { $servers }, zimepatikana kwenye mipangilio ya mtoa huduma wako.
    [ispdb] Seva: { $servers }, zimepatikana kwenye orodha ya watoa huduma ya Thunderbird.
    [dns] Seva: { $servers }, zimepatikana kwenye rekodi za DNS za kikoa chako.
   *[other] Seva: { $servers }, zimekisiwa; zikague ikiwa kuingia kutashindwa.
}
add-account-servers-entered = Seva: { $servers }, kama zilivyowekwa.

## Add a mail account: buttons

add-account-servers-button = Mipangilio ya seva
add-account-back = Rudi
add-account-add = Ongeza akaunti
add-account-next = Endelea
add-account-cancel = Ghairi

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] Weka seva ya barua zinazoingia.
   *[outgoing] Weka seva ya barua zinazotoka.
}
add-account-server-space = { $kind ->
    [incoming] Jina la seva ya barua zinazoingia lina nafasi ndani yake.
   *[outgoing] Jina la seva ya barua zinazotoka lina nafasi ndani yake.
}
add-account-port-invalid = { $kind ->
    [incoming] Mlango wa barua zinazoingia lazima uwe namba kuanzia { $min } hadi { $max }.
   *[outgoing] Mlango wa barua zinazotoka lazima uwe namba kuanzia { $min } hadi { $max }.
}
add-account-address-empty = Weka anwani ya barua pepe.
add-account-address-invalid = Weka anwani ya barua pepe kama { $example }.
add-account-not-found = Katna haikuweza kupata seva za { $address }, kwa hivyo imejaza majina ya kawaida. Yakague na mtoa huduma wako.
add-account-password-empty = Weka nenosiri.
add-account-name-is-password = Jina ni sawa na nenosiri. Andika jina lako hapo badala yake, kama watu wanavyopaswa kuliona.
add-account-added = { $address } imeongezwa. Inapokea barua zako…
add-account-app-password-refused = { $provider } imekataa nenosiri. Inahitaji nenosiri la programu, si lile unalotumia kwenye wavuti.
add-account-password-refused = Seva imekataa nenosiri. Likague na ujaribu tena.

## The account menu (from the account button on the top bar)

add-account-menu-another = Ongeza akaunti nyingine
add-account-menu-manage = Dhibiti akaunti
