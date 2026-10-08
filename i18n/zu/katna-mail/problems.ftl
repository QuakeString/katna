# Katna Mail, Zulu (isiZulu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = Iseva yemeyili

problems-signed-out = { $provider } ikhiphe i-Katna ku-{ $address }. Imeyili iyekile ukuvumelanisa.
problems-password-refused = { $provider } yenqabe iphasiwedi ka-{ $address }. Kungenzeka ishintshile.
problems-no-answer = { $provider } ayiphenduli ku-{ $address }. I-Katna iyaqhubeka izama.
problems-offline = Awuxhunyiwe. Imeyili yakho isekhona lapha, futhi imeyili oyithumelayo ilinda uze uxhume futhi.
problems-accounts-need-you = { $count ->
    [one] I-akhawunti engu-{ $count } iyakudinga
   *[other] Ama-akhawunti angu-{ $count } ayakudinga
}
problems-show = Bonisa
problems-later = Kamuva
problems-new-password = Iphasiwedi entsha
problems-try-again = Zama futhi

## The New password card

problems-password-title = Iphasiwedi entsha
problems-password-detail = { $provider } yenqabe iphasiwedi elondoloziwe ka-{ $address }. Thayipha entsha; i-Katna iyayihlola ngaphambi kokuyigcina.
problems-password-placeholder = Iphasiwedi
problems-password-show = Bonisa iphasiwedi
problems-password-hide = Fihla iphasiwedi
problems-password-cancel = Khansela
problems-password-save = Londoloza
problems-password-checking = Iyahlola…
problems-password-refused-again = { $provider } yenqabe nale phasiwedi. Yihlole bese uzama futhi.
problems-password-saved = Iphasiwedi ilondolozwe ku-{ $address }. Kulandwa imeyili yakho…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = Iseva yemeyili ka-{ $address } ayikwamukelanga ukuhambisa { $count ->
    [one] umlayezo, ngakho usubuyele lapho ubukhona.
   *[other] imilayezo engu-{ $count }, ngakho isibuyele lapho ibikhona.
}
problems-refused-flags = Iseva yemeyili ka-{ $address } ayikwamukelanga ukumaka { $count ->
    [one] umlayezo (ofundiwe, onenkanyezi…), ngakho usubuyele njengoba ubunjalo.
   *[other] imilayezo engu-{ $count } (efundiwe, enenkanyezi…), ngakho isibuyele njengoba ibinjalo.
}
problems-refused-label = Iseva yemeyili ka-{ $address } ayikwamukelanga ukushintsha amalebula { $count ->
    [one] omlayezo, ngakho usubuyele njengoba ubunjalo.
   *[other] emilayezo engu-{ $count }, ngakho isibuyele njengoba ibinjalo.
}
problems-refused-delete = Iseva yemeyili ka-{ $address } ayikwamukelanga ukususa { $count ->
    [one] umlayezo, ngakho usubuyile.
   *[other] imilayezo engu-{ $count }, ngakho isibuyile.
}
problems-refused-other = Iseva yemeyili ka-{ $address } ayizamukelanga { $count ->
    [one] uguquko, ngakho i-Katna ilubuyisele njengoba belunjalo.
   *[other] izinguquko ezingu-{ $count }, ngakho i-Katna izibuyisele njengoba bezinjalo.
}
problems-details = Imininingwane

## Katna's background service (katna-daemon) isn't running

service-starting = Kuqalwa isevisi ye-Katna esebenza ngemuva…
service-failed = Isevisi ye-Katna esebenza ngemuva ayiqali, ngakho imeyili ayivumelanisi.
service-start-again = Qala futhi
service-started-again = Isevisi ye-Katna esebenza ngemuva ibimile futhi iqalwe kabusha.
service-details-title = Kungani isevisi ingaqali
service-details-body = Kopisha lokhu bese ukuthumela nombiko wakho. Akunayo imeyili noma amaphasiwedi.
service-details-copy = Kopisha
service-details-close = Vala
service-not-running = Isevisi yangemuva ye-Katna ayisebenzi.
service-no-answer = Isevisi yangemuva ye-Katna ayiphendulanga: { $error }
service-no-session = Ayikho iseshini ye-D-Bus: { $error }
