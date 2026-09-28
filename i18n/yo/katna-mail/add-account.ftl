# Katna Mail, Yoruba (Yorùbá).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Ṣàfikún àkáǹtì lẹ́tà
add-account-looking = À ń wá àwọn sáfà lẹ́tà { $address }…
add-account-address-intro = Tẹ àdírẹ́sì ímeèlì rẹ. Katna yóò wá àwọn sáfà fún ọ.
add-account-servers-title = Ètò sáfà
add-account-servers-intro = Ibi tí Katna ti ń ka lẹ́tà, tí ó sì ti ń fi ránṣẹ́ fún { $address }.
add-account-password-title = Tẹ ọ̀rọ̀ aṣínà rẹ
add-account-signing-in = À ń wọlé…
add-account-browser-title = Tẹ̀síwájú nínú aṣàwákiri rẹ
add-account-browser-intro = Katna ti ṣí ojú-ìwé ìwọlé { $provider } nínú aṣàwákiri rẹ. Wọlé níbẹ̀ kí o sì gba Katna láàyè láti ka lẹ́tà rẹ àti láti fi ránṣẹ́, lẹ́yìn náà padà wá síbí.
add-account-browser-hint = Kò sí ojú-ìwé tó ṣí? Ṣàyẹ̀wò àwọn fèrèsé aṣàwákiri rẹ, tàbí padà sẹ́yìn kí o sì gbìyànjú lẹ́ẹ̀kan sí i.

## Add a mail account: fields

add-account-field-address = Àdírẹ́sì ímeèlì
add-account-incoming = Lẹ́tà tó ń wọlé ({ $protocol })
add-account-outgoing = Lẹ́tà tó ń jáde ({ $protocol })
add-account-field-server = Sáfà
add-account-field-port = Pọ́ọ̀tù
add-account-security-none = Kò sí
add-account-security-none-warning = A kò fi àṣírí bò ó: a lè ka ọ̀rọ̀ìpamọ́ àti lẹ́tà rẹ lójú ọ̀nà.
add-account-field-username = Orúkọ oníṣe
add-account-field-password = Ọ̀rọ̀ aṣínà
add-account-show-password = Fi ọ̀rọ̀ aṣínà hàn
add-account-app-password-hint = { $provider } nílò ọ̀rọ̀ aṣínà áàpù níbí, kì í ṣe èyí tí o ń lò lórí wẹ́ẹ̀bù. Ṣe ọ̀kan nínú ètò ààbò àkáǹtì { $provider } rẹ.
add-account-field-name = Orúkọ rẹ (kò pọndandan)
add-account-name-hint = A ń fi hàn fún àwọn ènìyàn tí o kọ̀wé sí.
add-account-servers-pair = { $imap } àti { $smtp }
add-account-servers-found = { $source ->
    [built-in] Àwọn sáfà: { $servers }, a rí wọn nínú àkójọ àwọn olùpèsè Katna.
    [provider] Àwọn sáfà: { $servers }, a rí wọn nínú ètò olùpèsè rẹ.
    [ispdb] Àwọn sáfà: { $servers }, a rí wọn nínú àkójọ àwọn olùpèsè Thunderbird.
    [dns] Àwọn sáfà: { $servers }, a rí wọn nínú àkọsílẹ̀ DNS àgbègbè rẹ.
   *[other] Àwọn sáfà: { $servers }, láti inú àfojúsùn; ṣàyẹ̀wò wọn tí wíwọlé bá kùnà.
}
add-account-servers-entered = Àwọn sáfà: { $servers }, bí a ṣe tẹ̀ wọ́n.
add-account-or = tàbí
add-account-sign-in-with = Wọlé pẹ̀lú { $provider }
add-account-sign-in-instead = Wọlé pẹ̀lú { $provider } dípò èyí

## Add a mail account: buttons

add-account-servers-button = Ètò sáfà
add-account-back = Padà
add-account-add = Ṣàfikún àkáǹtì
add-account-next = Èyí tó kàn
add-account-cancel = Fagilé

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] Tẹ sáfà lẹ́tà tó ń wọlé.
   *[outgoing] Tẹ sáfà lẹ́tà tó ń jáde.
}
add-account-server-space = { $kind ->
    [incoming] Àlàfo wà nínú orúkọ sáfà lẹ́tà tó ń wọlé.
   *[outgoing] Àlàfo wà nínú orúkọ sáfà lẹ́tà tó ń jáde.
}
add-account-port-invalid = { $kind ->
    [incoming] Pọ́ọ̀tù lẹ́tà tó ń wọlé gbọ́dọ̀ jẹ́ nọ́ńbà láti { $min } sí { $max }.
   *[outgoing] Pọ́ọ̀tù lẹ́tà tó ń jáde gbọ́dọ̀ jẹ́ nọ́ńbà láti { $min } sí { $max }.
}
add-account-address-empty = Tẹ àdírẹ́sì ímeèlì kan.
add-account-address-invalid = Tẹ àdírẹ́sì ímeèlì bí { $example }.
add-account-not-found = Katna kò rí àwọn sáfà fún { $address }, nítorí náà ó kọ àwọn orúkọ tí a sábà ń lò sí i. Ṣàyẹ̀wò wọn pẹ̀lú olùpèsè rẹ.
add-account-password-empty = Tẹ ọ̀rọ̀ aṣínà.
add-account-name-is-password = Orúkọ náà bá ọ̀rọ̀ aṣínà mu. Dípò bẹ́ẹ̀, tẹ orúkọ rẹ síbẹ̀, bí ó ṣe yẹ kí àwọn ènìyàn rí i.
add-account-added = A ti ṣàfikún { $address }. À ń gba lẹ́tà rẹ…
add-account-app-password-refused = { $provider } kọ ọ̀rọ̀ aṣínà náà. Ó nílò ọ̀rọ̀ aṣínà áàpù, kì í ṣe èyí tí o ń lò lórí wẹ́ẹ̀bù.
add-account-password-refused = Sáfà kọ ọ̀rọ̀ aṣínà náà. Ṣàyẹ̀wò rẹ̀ kí o sì gbìyànjú lẹ́ẹ̀kan sí i.
add-account-sign-in-refused = { $provider } kò jẹ́ kí Katna wọlé. Gbìyànjú lẹ́ẹ̀kan sí i, kí o sì fàyè gba wíwọlé sí lẹ́tà rẹ.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Ẹ̀dà Katna yìí kò tíì lè wọlé sí àwọn àkáǹtì Microsoft.
    [Google] Ẹ̀dà Katna yìí kò tíì lè wọlé sí àwọn àkáǹtì Google.
   *[other] Olùpèsè yìí gba wíwọlé láàyè lórí ojú-ìwé tirẹ̀ nìkan, èyí tí Katna kò tíì lè ṣe fún un.
}
add-account-signed-in = O ti wọlé pẹ̀lú { $provider }. À ń gba lẹ́tà rẹ…

## The account menu (from the account button on the top bar)

add-account-menu-another = Ṣàfikún àkáǹtì míì
add-account-menu-manage = Ṣàkóso àwọn àkáǹtì
app-menu = Mẹ́nù pàtàkì
