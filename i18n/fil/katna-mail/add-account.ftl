# Katna Mail, Filipino (Filipino).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Magdagdag ng mail account
add-account-looking = Hinahanap ang mga mail server ng { $address }…
add-account-address-intro = Ilagay ang iyong email address. Hahanapin ng Katna ang mga server para sa iyo.
add-account-servers-title = Mga setting ng server
add-account-servers-intro = Kung saan nagbabasa at nagpapadala ang Katna ng mail para sa { $address }.
add-account-signing-in = Nagsa-sign in…
add-account-browser-title = Magpatuloy sa iyong browser
add-account-browser-intro = Binuksan ng Katna ang sign-in page ng { $provider } sa iyong browser. Mag-sign in doon at payagan ang Katna na magbasa at magpadala ng iyong mail, pagkatapos ay bumalik dito.
add-account-browser-hint = Walang page na bumukas? Tingnan ang mga window ng iyong browser, o bumalik at subukang muli.

## Add a mail account: fields

add-account-field-address = Email address
add-account-incoming = Papasok na mail ({ $protocol })
add-account-outgoing = Papalabas na mail ({ $protocol })
add-account-field-server = Server
add-account-field-port = Port
add-account-security-none = Wala
add-account-security-none-warning = Hindi naka-encrypt: mababasa ang iyong password at mail habang nasa daan.
add-account-field-username = Username
add-account-field-password = Password
add-account-show-password = Ipakita ang password
add-account-app-password-hint = Kailangan ng { $provider } ng app password dito, hindi ang ginagamit mo sa web. Gumawa ng isa sa mga setting ng seguridad ng iyong { $provider } account.
add-account-field-name = Ang pangalan mo (opsyonal)
add-account-name-hint = Ipinapakita sa mga taong sinusulatan mo.
add-account-servers-pair = { $imap } at { $smtp }
add-account-servers-found = { $source ->
    [built-in] Mga server: { $servers }, nakita sa listahan ng mga provider ng Katna.
    [provider] Mga server: { $servers }, nakita sa mga setting ng iyong provider.
    [ispdb] Mga server: { $servers }, nakita sa listahan ng mga provider ng Thunderbird.
    [dns] Mga server: { $servers }, nakita sa mga DNS record ng iyong domain.
   *[other] Mga server: { $servers }, hula lang; tingnan ang mga ito kung pumalya ang pag-sign in.
}
add-account-servers-entered = Mga server: { $servers }, gaya ng inilagay.
add-account-sign-in-with = Mag-sign in gamit ang { $provider }
add-account-sign-in-instead = Mag-sign in na lang gamit ang { $provider }

## Add a mail account: buttons

add-account-servers-button = Mga setting ng server
add-account-back = Bumalik
add-account-add = Idagdag ang account
add-account-cancel = Kanselahin

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] Ilagay ang papasok na server.
   *[outgoing] Ilagay ang papalabas na server.
}
add-account-server-space = { $kind ->
    [incoming] May espasyo ang pangalan ng papasok na server.
   *[outgoing] May espasyo ang pangalan ng papalabas na server.
}
add-account-port-invalid = { $kind ->
    [incoming] Dapat na numero mula { $min } hanggang { $max } ang papasok na port.
   *[outgoing] Dapat na numero mula { $min } hanggang { $max } ang papalabas na port.
}
add-account-address-empty = Maglagay ng email address.
add-account-address-invalid = Maglagay ng email address gaya ng { $example }.
add-account-not-found = Hindi mahanap ng Katna ang mga server para sa { $address }, kaya inilagay nito ang mga karaniwang pangalan. Tingnan ang mga ito sa iyong provider.
add-account-password-empty = Ilagay ang password.
add-account-name-is-password = Pareho ang pangalan at ang password. Sa halip, i-type doon ang pangalan mo, gaya ng dapat makita ng mga tao.
add-account-app-password-refused = Tinanggihan ng { $provider } ang password. Kailangan nito ng app password, hindi ang ginagamit mo sa web.
add-account-password-refused = Tinanggihan ng server ang password. Tingnan ito at subukang muli.
add-account-sign-in-refused = Hindi pinapasok ng { $provider } ang Katna. Subukang muli, at payagan ang access sa iyong mail.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Hindi pa makakapag-sign in ang kopyang ito ng Katna sa mga Microsoft account.
    [Google] Hindi pa makakapag-sign in ang kopyang ito ng Katna sa mga Google account.
   *[other] Pinapayagan lang ng provider na ito ang pag-sign in sa sarili nitong page, na hindi pa kayang gawin ng Katna para dito.
}

## The account menu (from the account button on the top bar)

add-account-menu-another = Magdagdag ng isa pang account
app-menu = Pangunahing menu
app-menu-back = Bumalik
