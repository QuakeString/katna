# Katna Mail, Swahili (Kiswahili).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Ongeza akaunti ya barua
add-account-providers-intro = Chagua mtoa huduma wako wa barua. Katna itapata vingine.
add-account-provider-other = Barua nyingine
add-account-provider-other-detail = Akaunti yoyote ya IMAP au POP3
add-account-provider-google-detail = Gmail na Google Workspace
add-account-provider-microsoft-detail = Outlook na Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = Ingia kwenye { $provider }
add-account-form-title-other = Akaunti yako ya barua
add-account-form-intro = Katna huhifadhi nenosiri lako kwenye keyring ya mfumo wako.
add-account-looking = Inatafuta seva za barua za { $address }…
add-account-address-intro = Weka anwani yako ya barua pepe. Katna itakutafutia seva.
add-account-servers-title = Mipangilio ya seva
add-account-servers-intro = Mahali Katna inaposoma na kutuma barua za { $address }.
add-account-signing-in = Inaingia…
add-account-browser-title = Endelea kwenye kivinjari chako
add-account-browser-intro = Katna imefungua ukurasa wa kuingia wa { $provider } kwenye kivinjari chako. Ingia hapo na uiruhusu Katna isome na kutuma barua zako, kisha urudi hapa.
add-account-browser-hint = Hakuna ukurasa uliofunguka? Angalia madirisha ya kivinjari chako, au rudi ujaribu tena.
add-account-stage-browser = Inasubiri uingie kwenye kivinjari chako…
add-account-stage-signing-in-at = Inaingia kwenye { $server }…
add-account-help-app-password-link = Jinsi ya kutengeneza nenosiri la programu
add-account-help-turn-on-imap = { $provider } huruhusu programu za barua tu baada ya ufikiaji wa IMAP na POP3 kuwashwa kwenye mipangilio ya barua yake ya wavuti.
add-account-help-turn-on-imap-link = Jinsi ya kuuwasha

## Add a mail account: fields

add-account-field-address = Anwani ya barua pepe
add-account-receive-with = Pokea barua kwa
add-account-imap-about = IMAP huhifadhi barua na folda zako kwenye seva, sawa kwenye kila kifaa. Ichague unapoweza.
add-account-pop3-about = POP3 hupakua barua zako kwenye kompyuta hii. Barua unazosoma au kuhamisha hapa hubaki kama zilivyo kwenye seva na vifaa vyako vingine.
add-account-incoming = Barua zinazoingia ({ $protocol })
add-account-outgoing = Barua zinazotoka ({ $protocol })
add-account-field-server = Seva
add-account-field-port = Mlango
add-account-security-none = Hakuna
add-account-security-none-warning = Haijasimbwa: nenosiri na barua zako zinaweza kusomwa njiani.
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
add-account-sign-in-with = Ingia kwa { $provider }
add-account-sign-in-instead = Ingia kwa { $provider } badala yake

## Add a mail account: buttons

add-account-servers-button = Mipangilio ya seva
add-account-back = Rudi
add-account-add = Ongeza akaunti
add-account-done = Imekamilika
add-account-another = Ongeza akaunti nyingine
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
add-account-app-password-refused = { $provider } imekataa nenosiri. Inahitaji nenosiri la programu, si lile unalotumia kwenye wavuti.
add-account-password-refused = Seva imekataa nenosiri. Likague na ujaribu tena.
add-account-sign-in-refused = { $provider } haikuiruhusu Katna kuingia. Jaribu tena, na uruhusu ufikiaji wa barua zako.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Nakala hii ya Katna bado haiwezi kuingia kwenye akaunti za Microsoft.
    [Google] Nakala hii ya Katna bado haiwezi kuingia kwenye akaunti za Google.
   *[other] Mtoa huduma huyu anaruhusu kuingia kwenye ukurasa wake tu, jambo ambalo Katna bado haiwezi kulifanya kwa ajili yake.
}
add-account-smtp-not-found = Katna imepata mahali pa kusoma barua zako lakini si mahali pa kuzituma. Weka seva ya kutuma.

## Add a mail account: the last step

add-account-done-title = Akaunti yako iko tayari
add-account-done-intro = Katna inapata barua zako sasa. Barua mpya huonekana zinapowasili.
add-account-done-sign-in = Kuingia
add-account-done-signed-in-with = Kwa { $provider }, kwenye kivinjari chako
add-account-done-receiving = Kupokea barua
add-account-done-sending = Kutuma barua
add-account-done-on-server = Barua kwenye seva
add-account-done-kept = Huhifadhiwa hadi uzifute katika Katna
add-account-done-pop3-hint = Badilisha kinachofanyika kwa barua kwenye seva katika Mipangilio > Akaunti.
add-account-done-zoho-title = Majukumu na kalenda
add-account-done-zoho-about = Zoho huviweka hivi mbali na barua. Ingia kwa Zoho mara moja ili kuvileta katika Katna.
add-account-done-linked = Majukumu na kalenda vimeunganishwa

## The account menu (from the account button on the top bar)

add-account-menu-another = Ongeza akaunti nyingine
app-menu = Menyu kuu
app-menu-back = Rudi
