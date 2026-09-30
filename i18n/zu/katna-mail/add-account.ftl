# Katna Mail, Zulu (isiZulu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Engeza i-akhawunti yemeyili
add-account-looking = Kufunwa amaseva emeyili ka-{ $address }…
add-account-address-intro = Faka ikheli lakho le-imeyili. I-Katna ikutholela amaseva.
add-account-servers-title = Izilungiselelo zeseva
add-account-servers-intro = Lapho i-Katna ifunda khona futhi ithumele imeyili ka-{ $address }.
add-account-signing-in = Iyangena…
add-account-browser-title = Qhubeka esipheqululini sakho
add-account-browser-intro = I-Katna ivule ikhasi lokungena le-{ $provider } esipheqululini sakho. Ngena lapho bese uvumela i-Katna ukuthi ifunde futhi ithumele imeyili yakho, bese ubuyela lapha.
add-account-browser-hint = Alivulekanga ikhasi? Hlola amawindi esiphequluli sakho, noma ubuyele emuva uzame futhi.

## Add a mail account: fields

add-account-field-address = Ikheli le-imeyili
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

## The account menu (from the account button on the top bar)

add-account-menu-another = Engeza enye i-akhawunti
add-account-menu-manage = Phatha ama-akhawunti
app-menu = Imenyu enkulu
app-menu-back = Emuva
