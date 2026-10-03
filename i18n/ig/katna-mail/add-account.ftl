# Katna Mail, Igbo (Igbo).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Tinye akaụntụ ozi
add-account-providers-intro = Họrọ onye na-enye gị ọrụ ozi. Katna ga-achọta ndị fọdụrụ.
add-account-provider-other = Ozi ọzọ
add-account-provider-other-detail = Akaụntụ IMAP ma ọ bụ POP3 ọ bụla
add-account-provider-google-detail = Gmail na Google Workspace
add-account-provider-microsoft-detail = Outlook na Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = Banye na { $provider }
add-account-form-title-other = Akaụntụ ozi gị
add-account-form-intro = Katna na-edobe okwuntughe gị n'ime igbe mkpịsị ugodi sistemụ gị.
add-account-looking = Na-achọ sava ozi nke { $address }…
add-account-address-intro = Tinye adreesị ozi-e gị. Katna ga-achọtara gị sava ndị ahụ.
add-account-servers-title = Ntọala sava
add-account-servers-intro = Ebe Katna na-agụ ma na-ezipụ ozi maka { $address }.
add-account-signing-in = Na-abanye…
add-account-browser-title = Gaa n'ihu na ihe nchọgharị gị
add-account-browser-intro = Katna emepela peeji mbanye { $provider } na ihe nchọgharị gị. Banye ebe ahụ ma kwe ka Katna gụọ ma zipụ ozi gị, wee laghachi ebe a.
add-account-browser-hint = O nweghị peeji mepere? Lelee windo nke ihe nchọgharị gị, ma ọ bụ laghachi ma nwaa ọzọ.
add-account-stage-browser = Na-eche ka i banye na brawza gị…
add-account-stage-signing-in-at = Na-abanye na { $server }…
add-account-help-app-password-link = Otu esi eme okwuntughe ngwa
add-account-help-turn-on-imap = { $provider } na-ekwe ka ngwa ozi banye naanị mgbe a gbanyere ohere IMAP na POP3 na ntọala ozi weebụ ya.
add-account-help-turn-on-imap-link = Otu esi agbanye ya

## Add a mail account: fields

add-account-field-address = Adreesị ozi-e
add-account-receive-with = Nata ozi site na
add-account-imap-about = IMAP na-edobe ozi na folda gị na sava, otu ihe ahụ na ngwaọrụ ọ bụla. Họrọ ya mgbe i nwere ike.
add-account-pop3-about = POP3 na-ebudata ozi gị na kọmputa a. Ozi ị gụrụ ma ọ bụ bugharịa ebe a na-anọ ka ọ dị na sava na ngwaọrụ gị ndị ọzọ.
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
add-account-sign-in-with = Banye na { $provider }
add-account-sign-in-instead = Kama nke ahụ, banye na { $provider }

## Add a mail account: buttons

add-account-servers-button = Ntọala sava
add-account-back = Laghachi
add-account-add = Tinye akaụntụ
add-account-done = O mechara
add-account-another = Tinye akaụntụ ọzọ
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
add-account-app-password-refused = { $provider } jụrụ okwuntughe ahụ. Ọ chọrọ okwuntughe ngwa, ọ bụghị nke ị na-eji na weebụ.
add-account-password-refused = Sava jụrụ okwuntughe ahụ. Nyochaa ya ma nwaa ọzọ.
add-account-sign-in-refused = { $provider } ekweghị ka Katna banye. Nwaa ọzọ, ma kwe ka ọ nweta ozi gị.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Mbipụta Katna a enweghị ike ịbanye n'akaụntụ Microsoft ugbu a.
    [Google] Mbipụta Katna a enweghị ike ịbanye n'akaụntụ Google ugbu a.
   *[other] Onye na-enye ọrụ a na-ekwe ka a banye naanị na peeji nke ya, nke Katna enweghị ike ime ya ugbu a.
}
add-account-smtp-not-found = Katna chọtara ebe a ga-agụ ozi gị mana ọ chọtaghị ebe a ga-ezipu ya. Tinye sava na-ezipụ ozi.

## Add a mail account: the last step

add-account-done-title = Akaụntụ gị adịla njikere
add-account-done-intro = Katna na-enweta ozi gị ugbu a. Ozi ọhụrụ na-apụta ka ọ na-abata.
add-account-done-sign-in = Ịbanye
add-account-done-signed-in-with = Site na { $provider }, na brawza gị
add-account-done-receiving = Ịnata ozi
add-account-done-sending = Izipu ozi
add-account-done-on-server = Ozi dị na sava
add-account-done-kept = Edobere ruo mgbe i hichapụrụ ya na Katna
add-account-done-pop3-hint = Gbanwee ihe na-eme ozi dị na sava na Ntọala > Akaụntụ.
add-account-done-zoho-title = Ọrụ na kalịnda
add-account-done-zoho-about = Zoho na-edobe ndị a iche na ozi. Banye na Zoho otu ugboro ka i weta ha na Katna.
add-account-done-linked = Ejikọla ọrụ na kalịnda

## The account menu (from the account button on the top bar)

add-account-menu-another = Tinye akaụntụ ọzọ
app-menu = Menu isi
app-menu-back = Laghachi
