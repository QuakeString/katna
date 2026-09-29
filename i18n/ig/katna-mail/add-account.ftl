# Katna Mail, Igbo (Igbo).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Tinye akaụntụ ozi
add-account-looking = Na-achọ sava ozi nke { $address }…
add-account-address-intro = Tinye adreesị ozi-e gị. Katna ga-achọtara gị sava ndị ahụ.
add-account-servers-title = Ntọala sava
add-account-servers-intro = Ebe Katna na-agụ ma na-ezipụ ozi maka { $address }.
add-account-password-title = Tinye okwuntughe gị
add-account-signing-in = Na-abanye…
add-account-browser-title = Gaa n'ihu na ihe nchọgharị gị
add-account-browser-intro = Katna emepela peeji mbanye { $provider } na ihe nchọgharị gị. Banye ebe ahụ ma kwe ka Katna gụọ ma zipụ ozi gị, wee laghachi ebe a.
add-account-browser-hint = O nweghị peeji mepere? Lelee windo nke ihe nchọgharị gị, ma ọ bụ laghachi ma nwaa ọzọ.

## Add a mail account: fields

add-account-field-address = Adreesị ozi-e
add-account-incoming = Ozi mbata ({ $protocol })
add-account-outgoing = Ozi mpụta ({ $protocol })
add-account-field-server = Sava
add-account-field-port = Ọdụ
add-account-security-none = Ọ dịghị
add-account-security-none-warning = Adịghị ezoro ezo: a pụrụ ịgụ okwuntughe gị na ozi ebe ọ na-aga.
add-account-field-username = Aha onye ọrụ
add-account-field-password = Okwuntughe
add-account-show-password = Gosi okwuntughe
add-account-app-password-hint = { $provider } chọrọ okwuntughe ngwa ebe a, ọ bụghị nke ị na-eji na weebụ. Mee otu na ntọala nchekwa nke akaụntụ { $provider } gị.
add-account-field-name = Aha gị (nhọrọ)
add-account-name-hint = A na-egosi ya ndị ị na-edegara ozi.
add-account-servers-pair = { $imap } na { $smtp }
add-account-servers-found = { $source ->
    [built-in] Sava: { $servers }, a chọtara ha na ndepụta ndị na-enye ọrụ nke Katna.
    [provider] Sava: { $servers }, a chọtara ha na ntọala nke onye na-enye gị ọrụ.
    [ispdb] Sava: { $servers }, a chọtara ha na ndepụta ndị na-enye ọrụ nke Thunderbird.
    [dns] Sava: { $servers }, a chọtara ha na ndekọ DNS nke ngalaba gị.
   *[other] Sava: { $servers }, site n'ịkọ nkọ; nyochaa ha ma ọ bụrụ na ịbanye dara.
}
add-account-servers-entered = Sava: { $servers }, dịka e tinyere ha.
add-account-or = ma ọ bụ
add-account-sign-in-with = Banye na { $provider }
add-account-sign-in-instead = Kama nke ahụ, banye na { $provider }

## Add a mail account: buttons

add-account-servers-button = Ntọala sava
add-account-back = Laghachi
add-account-add = Tinye akaụntụ
add-account-next = Nke ọzọ
add-account-cancel = Kagbuo

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] Tinye sava ozi mbata.
   *[outgoing] Tinye sava ozi mpụta.
}
add-account-server-space = { $kind ->
    [incoming] Aha sava ozi mbata nwere oghere n'ime ya.
   *[outgoing] Aha sava ozi mpụta nwere oghere n'ime ya.
}
add-account-port-invalid = { $kind ->
    [incoming] Ọdụ ozi mbata ga-abụrịrị nọmba site na { $min } ruo { $max }.
   *[outgoing] Ọdụ ozi mpụta ga-abụrịrị nọmba site na { $min } ruo { $max }.
}
add-account-address-empty = Tinye adreesị ozi-e.
add-account-address-invalid = Tinye adreesị ozi-e dị ka { $example }.
add-account-not-found = Katna enweghị ike ịchọta sava nke { $address }, ya mere o tinyere aha a na-ejikarị. Nyochaa ha n'aka onye na-enye gị ọrụ.
add-account-password-empty = Tinye okwuntughe.
add-account-name-is-password = Aha ahụ yiri okwuntughe. Pịnye aha gị ebe ahụ kama, dị ka ndị mmadụ kwesịrị ịhụ ya.
add-account-added = Etinyela { $address }. Na-ebute ozi gị…
add-account-app-password-refused = { $provider } jụrụ okwuntughe ahụ. Ọ chọrọ okwuntughe ngwa, ọ bụghị nke ị na-eji na weebụ.
add-account-password-refused = Sava jụrụ okwuntughe ahụ. Nyochaa ya ma nwaa ọzọ.
add-account-sign-in-refused = { $provider } ekweghị ka Katna banye. Nwaa ọzọ, ma kwe ka ọ nweta ozi gị.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Mbipụta Katna a enweghị ike ịbanye n'akaụntụ Microsoft ugbu a.
    [Google] Mbipụta Katna a enweghị ike ịbanye n'akaụntụ Google ugbu a.
   *[other] Onye na-enye ọrụ a na-ekwe ka a banye naanị na peeji nke ya, nke Katna enweghị ike ime ya ugbu a.
}
add-account-signed-in = Abanyela na { $provider }. Na-enweta ozi gị…

## The account menu (from the account button on the top bar)

add-account-menu-another = Tinye akaụntụ ọzọ
add-account-menu-manage = Jikwaa akaụntụ
app-menu = Menu isi
app-menu-back = Laghachi
