# Katna Mail, Hausa (Hausa).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Ƙara asusun wasiƙu
add-account-providers-intro = Zaɓi mai ba ku sabis na wasiƙu. Katna za ta nemo sauran.
add-account-provider-other = Wasu wasiƙu
add-account-provider-other-detail = Kowane asusun IMAP ko POP3
add-account-provider-google-detail = Gmail da Google Workspace
add-account-provider-microsoft-detail = Outlook da Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = Shiga { $provider }
add-account-form-title-other = Asusun wasiƙunku
add-account-form-intro = Katna tana ajiye kalmar sirrinku a ma'ajiyar maɓallan tsarinku.
add-account-looking = Ana neman sabobin wasiƙu na { $address }…
add-account-address-intro = Shigar da adireshin imel ɗinku. Katna za ta nemo muku sabobin.
add-account-servers-title = Saitunan sabar
add-account-servers-intro = Inda Katna ke karantawa da aika wasiƙun { $address }.
add-account-signing-in = Ana shiga…
add-account-browser-title = Ku ci gaba a burauzarku
add-account-browser-intro = Katna ya buɗe shafin shiga na { $provider } a burauzarku. Ku shiga a can ku ba Katna izinin karantawa da aika wasiƙunku, sannan ku dawo nan.
add-account-browser-hint = Babu shafin da ya buɗe? Ku duba tagogin burauzarku, ko ku koma baya ku sake gwadawa.
add-account-stage-browser = Ana jiran ku shiga a burauzarku…
add-account-stage-signing-in-at = Ana shiga a { $server }…
add-account-help-app-password-link = Yadda ake yin kalmar sirrin manhaja
add-account-help-turn-on-imap = { $provider } tana barin manhajojin wasiƙa su shiga ne kawai idan an kunna damar IMAP da POP3 a saitunan wasiƙunta na yanar gizo.
add-account-help-turn-on-imap-link = Yadda ake kunna shi

## Add a mail account: fields

add-account-field-address = Adireshin imel
add-account-receive-with = Karɓi wasiƙu ta
add-account-imap-about = IMAP tana ajiye wasiƙunku da folda a kan sabar, iri ɗaya a kowace na'ura. Zaɓi ta idan za ku iya.
add-account-pop3-about = POP3 tana sauke wasiƙunku zuwa wannan kwamfuta. Wasiƙun da kuka karanta ko kuka matsar a nan suna zama yadda suke a kan sabar da sauran na'urorinku.
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
add-account-done = An gama
add-account-another = Ƙara wani asusu
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
add-account-smtp-not-found = Katna ta gano inda za ta karanta wasiƙunku amma ba inda za ta aika su ba. Shigar da sabar fita.

## Add a mail account: the last step

add-account-done-title = Asusunku ya shirya
add-account-done-intro = Katna tana samo wasiƙunku yanzu. Sababbin wasiƙu suna bayyana yayin da suke isowa.
add-account-done-sign-in = Shiga
add-account-done-signed-in-with = Da { $provider }, a burauzarku
add-account-done-receiving = Karɓar wasiƙu
add-account-done-sending = Aika wasiƙu
add-account-done-on-server = Wasiƙu a kan sabar
add-account-done-kept = Ana ajiye su har sai kun share su a Katna
add-account-done-pop3-hint = Canza abin da ke faruwa da wasiƙu a kan sabar a Saituna > Asusu.
add-account-done-zoho-title = Ayyuka da kalandoji
add-account-done-zoho-about = Zoho tana ajiye waɗannan dabam da wasiƙu. Shiga da Zoho sau ɗaya don kawo su cikin Katna.
add-account-done-linked = An haɗa ayyuka da kalandoji

## The account menu (from the account button on the top bar)

add-account-menu-another = Ƙara wani asusu
app-menu = Babban menu
app-menu-back = Koma baya
