# Katna Mail, Zulu (isiZulu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Engeza i-akhawunti yemeyili
add-account-providers-intro = Khetha umhlinzeki wakho wemeyili. I-Katna izothola okunye.
add-account-provider-other = Enye imeyili
add-account-provider-other-detail = Noma iyiphi i-akhawunti ye-IMAP noma ye-POP3
add-account-provider-google-detail = Gmail ne-Google Workspace
add-account-provider-microsoft-detail = Outlook ne-Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = Ngena ku-{ $provider }
add-account-form-title-other = I-akhawunti yakho yemeyili
add-account-form-intro = I-Katna igcina iphasiwedi yakho ku-keyring yesistimu yakho.
add-account-looking = Kufunwa amaseva emeyili ka-{ $address }…
add-account-address-intro = Faka ikheli lakho le-imeyili. I-Katna ikutholela amaseva.
add-account-servers-title = Izilungiselelo zeseva
add-account-servers-intro = Lapho i-Katna ifunda khona futhi ithumele imeyili ka-{ $address }.
add-account-signing-in = Iyangena…
add-account-browser-title = Qhubeka esipheqululini sakho
add-account-browser-intro = I-Katna ivule ikhasi lokungena le-{ $provider } esipheqululini sakho. Ngena lapho bese uvumela i-Katna ukuthi ifunde futhi ithumele imeyili yakho, bese ubuyela lapha.
add-account-browser-hint = Alivulekanga ikhasi? Hlola amawindi esiphequluli sakho, noma ubuyele emuva uzame futhi.
add-account-stage-browser = Kulindwe ukuthi ungene esipheqululini sakho…
add-account-stage-signing-in-at = Kungenwa ku-{ $server }…
add-account-help-app-password-link = Indlela yokwenza iphasiwedi ye-app
add-account-help-turn-on-imap = I-{ $provider } ivumela ama-app emeyili kuphela uma ukufinyelela kwe-IMAP ne-POP3 kuvuliwe ezilungiselelweni zemeyili yayo yewebhu.
add-account-help-turn-on-imap-link = Indlela yokukuvula

## Add a mail account: fields

add-account-field-address = Ikheli le-imeyili
add-account-receive-with = Thola imeyili nge-
add-account-imap-about = I-IMAP igcina imeyili namafolda akho kuseva, kufana kuyo yonke idivayisi. Yikhethe uma ukwazi.
add-account-pop3-about = I-POP3 ilanda imeyili yakho kule khompyutha. Imeyili oyifundayo noma oyihambisayo lapha ihlala injalo kuseva nakwamanye amadivayisi akho.
add-account-incoming = Imeyili engenayo ({ $protocol })
add-account-outgoing = Imeyili ephumayo ({ $protocol })
add-account-field-server = Iseva
add-account-field-port = Imbobo
add-account-security-none = Lutho
add-account-security-none-warning = Ayifihlwanga: iphasiwedi lakho nemeyili kungafundwa endleleni.
add-account-field-username = Igama lomsebenzisi
add-account-field-password = Iphasiwedi
add-account-show-password = Bonisa iphasiwedi
add-account-app-password-hint = I-{ $provider } idinga iphasiwedi yohlelo lokusebenza lapha, hhayi leyo oyisebenzisa kuwebhu. Yenza eyodwa ezilungiselelweni zokuphepha ze-akhawunti yakho ye-{ $provider }.
add-account-field-name = Igama lakho (ongakukhetha)
add-account-name-hint = Liboniswa abantu obabhalelayo.
add-account-servers-pair = { $imap } kanye no-{ $smtp }
add-account-servers-found = { $source ->
    [built-in] Amaseva: { $servers }, atholwe ohlwini lwabahlinzeki lwe-Katna.
    [provider] Amaseva: { $servers }, atholwe ezilungiselelweni zomhlinzeki wakho.
    [ispdb] Amaseva: { $servers }, atholwe ohlwini lwabahlinzeki lwe-Thunderbird.
    [dns] Amaseva: { $servers }, atholwe emarekhodini e-DNS esizinda sakho.
   *[other] Amaseva: { $servers }, aqagelwe; wahlole uma ukungena kwehluleka.
}
add-account-servers-entered = Amaseva: { $servers }, njengoba efakiwe.
add-account-sign-in-with = Ngena nge-{ $provider }
add-account-sign-in-instead = Ngena nge-{ $provider } esikhundleni salokho

## Add a mail account: buttons

add-account-servers-button = Izilungiselelo zeseva
add-account-back = Emuva
add-account-add = Engeza i-akhawunti
add-account-done = Kwenziwe
add-account-another = Engeza enye i-akhawunti
add-account-cancel = Khansela

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] Faka iseva engenayo.
   *[outgoing] Faka iseva ephumayo.
}
add-account-server-space = { $kind ->
    [incoming] Igama leseva engenayo linesikhala.
   *[outgoing] Igama leseva ephumayo linesikhala.
}
add-account-port-invalid = { $kind ->
    [incoming] Imbobo engenayo kufanele ibe inombolo esuka ku-{ $min } kuya ku-{ $max }.
   *[outgoing] Imbobo ephumayo kufanele ibe inombolo esuka ku-{ $min } kuya ku-{ $max }.
}
add-account-address-empty = Faka ikheli le-imeyili.
add-account-address-invalid = Faka ikheli le-imeyili elifana no-{ $example }.
add-account-not-found = I-Katna ayikwazanga ukuthola amaseva ka-{ $address }, ngakho igcwalise amagama ajwayelekile. Wahlole nomhlinzeki wakho.
add-account-password-empty = Faka iphasiwedi.
add-account-name-is-password = Igama liyafana nephasiwedi. Esikhundleni salokho, bhala igama lakho lapho, njengoba abantu kufanele balibone.
add-account-app-password-refused = I-{ $provider } yenqabe iphasiwedi. Idinga iphasiwedi yohlelo lokusebenza, hhayi leyo oyisebenzisa kuwebhu.
add-account-password-refused = Iseva yenqabe iphasiwedi. Yihlole bese uzama futhi.
add-account-sign-in-refused = I-{ $provider } ayizange iyivumele i-Katna ukuthi ingene. Zama futhi, bese uvumela ukufinyelela kumeyili yakho.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Le khophi ye-Katna ayikakwazi ukungena kuma-akhawunti e-Microsoft.
    [Google] Le khophi ye-Katna ayikakwazi ukungena kuma-akhawunti e-Google.
   *[other] Lo mhlinzeki uvumela ukungena ekhasini lakhe kuphela, into i-Katna engakakwazi ukuyenzela yena.
}
add-account-smtp-not-found = I-Katna ithole lapho ingafunda khona imeyili yakho kodwa hhayi lapho ingayithumela khona. Faka iseva ephumayo.

## Add a mail account: the last step

add-account-done-title = I-akhawunti yakho isilungile
add-account-done-intro = I-Katna ilanda imeyili yakho manje. Imeyili entsha iyavela njengoba ifika.
add-account-done-sign-in = Ukungena
add-account-done-signed-in-with = Nge-{ $provider }, esipheqululini sakho
add-account-done-receiving = Ukwamukela imeyili
add-account-done-sending = Ukuthumela imeyili
add-account-done-on-server = Imeyili kuseva
add-account-done-kept = Igcinwa uze uyisuse ku-Katna
add-account-done-pop3-hint = Shintsha okwenzeka ngemeyili kuseva ku-Izilungiselelo > Ama-akhawunti.
add-account-done-zoho-title = Imisebenzi namakhalenda
add-account-done-zoho-about = I-Zoho igcina lokhu kwehlukile kumeyili. Ngena nge-Zoho kanye ukuze ukulethe ku-Katna.
add-account-done-linked = Imisebenzi namakhalenda kuxhunyiwe

## The account menu (from the account button on the top bar)

add-account-menu-another = Engeza enye i-akhawunti
app-menu = Imenyu enkulu
app-menu-back = Emuva
