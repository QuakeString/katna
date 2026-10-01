# Katna Mail, Hausa (Hausa).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Ƙara asusun wasiƙu
add-account-looking = Ana neman sabobin wasiƙu na { $address }…
add-account-address-intro = Shigar da adireshin imel ɗinku. Katna za ta nemo muku sabobin.
add-account-servers-title = Saitunan sabar
add-account-servers-intro = Inda Katna ke karantawa da aika wasiƙun { $address }.
add-account-signing-in = Ana shiga…
add-account-browser-title = Ku ci gaba a burauzarku
add-account-browser-intro = Katna ya buɗe shafin shiga na { $provider } a burauzarku. Ku shiga a can ku ba Katna izinin karantawa da aika wasiƙunku, sannan ku dawo nan.
add-account-browser-hint = Babu shafin da ya buɗe? Ku duba tagogin burauzarku, ko ku koma baya ku sake gwadawa.

## Add a mail account: fields

add-account-field-address = Adireshin imel
add-account-incoming = Wasiƙu masu shigowa ({ $protocol })
add-account-outgoing = Wasiƙu masu fita ({ $protocol })
add-account-field-server = Sabar
add-account-field-port = Tashar sadarwa
add-account-security-none = Babu
add-account-security-none-warning = Ba a ɓoye ba: ana iya karanta kalmar sirrinku da wasiƙunku a hanya.
add-account-field-username = Sunan mai amfani
add-account-field-password = Kalmar sirri
add-account-show-password = Nuna kalmar sirri
add-account-app-password-hint = { $provider } yana buƙatar kalmar sirrin manhaja a nan, ba wadda kuke amfani da ita a yanar gizo ba. Ku yi ɗaya a saitunan tsaro na asusunku na { $provider }.
add-account-field-name = Sunanku (na zaɓi)
add-account-name-hint = Ana nuna shi ga mutanen da kuke rubuta wa.
add-account-servers-pair = { $imap } da { $smtp }
add-account-servers-found = { $source ->
    [built-in] Sabobi: { $servers }, an samo su a jerin masu ba da sabis na Katna.
    [provider] Sabobi: { $servers }, an samo su a saitunan mai ba ku sabis.
    [ispdb] Sabobi: { $servers }, an samo su a jerin masu ba da sabis na Thunderbird.
    [dns] Sabobi: { $servers }, an samo su a bayanan DNS na yankinku.
   *[other] Sabobi: { $servers }, bisa hasashe; ku duba su idan shiga ya kasa.
}
add-account-servers-entered = Sabobi: { $servers }, kamar yadda aka shigar.
add-account-sign-in-with = Shiga da { $provider }
add-account-sign-in-instead = Shiga da { $provider } maimakon haka

## Add a mail account: buttons

add-account-servers-button = Saitunan sabar
add-account-back = Koma baya
add-account-add = Ƙara asusu
add-account-cancel = Soke

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] Shigar da sabar wasiƙu masu shigowa.
   *[outgoing] Shigar da sabar wasiƙu masu fita.
}
add-account-server-space = { $kind ->
    [incoming] Sunan sabar wasiƙu masu shigowa yana da tazara a ciki.
   *[outgoing] Sunan sabar wasiƙu masu fita yana da tazara a ciki.
}
add-account-port-invalid = { $kind ->
    [incoming] Tashar sadarwa ta wasiƙu masu shigowa dole ta zama lamba daga { $min } zuwa { $max }.
   *[outgoing] Tashar sadarwa ta wasiƙu masu fita dole ta zama lamba daga { $min } zuwa { $max }.
}
add-account-address-empty = Shigar da adireshin imel.
add-account-address-invalid = Shigar da adireshin imel kamar { $example }.
add-account-not-found = Katna ba ta iya samo sabobin { $address } ba, don haka ta cike sunayen da aka saba. Ku tabbatar da su wurin mai ba ku sabis.
add-account-password-empty = Shigar da kalmar sirri.
add-account-name-is-password = Sunan daidai yake da kalmar sirri. Rubuta sunanku a can maimakon haka, yadda mutane za su gan shi.
add-account-app-password-refused = { $provider } ya ƙi kalmar sirrin. Yana buƙatar kalmar sirrin manhaja, ba wadda kuke amfani da ita a yanar gizo ba.
add-account-password-refused = Sabar ta ƙi kalmar sirrin. Ku duba ta kuma ku sake gwadawa.
add-account-sign-in-refused = { $provider } bai bar Katna ya shiga ba. Ku sake gwadawa, kuma ku ba da izinin shiga wasiƙunku.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Wannan kwafin Katna ba zai iya shiga asusun Microsoft ba tukuna.
    [Google] Wannan kwafin Katna ba zai iya shiga asusun Google ba tukuna.
   *[other] Wannan mai ba da sabis yana ba da damar shiga ne kawai a shafinsa, abin da Katna ba zai iya yi masa ba tukuna.
}

## The account menu (from the account button on the top bar)

add-account-menu-another = Ƙara wani asusu
app-menu = Babban menu
app-menu-back = Koma baya
