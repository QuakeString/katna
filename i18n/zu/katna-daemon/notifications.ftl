# Katna Mail, Zulu (isiZulu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count ->
    [one] { $count } i-imeyili entsha
   *[other] ama-imeyili amasha angu-{ $count }
}
notify-and-more = nokunye okungu-{ $count }
notify-no-subject = (asikho isihloko)
notify-unknown-sender = Umthumeli ongaziwa
notify-snooze-back = Kubuyile ngemva kokulibaziswa
notify-no-reply = Ayikabikho impendulo
notify-no-reply-to = Akekho ophendule “{ $subject }”.
notify-follow-up-sent = Ukulandelela kuthunyelwe
notify-follow-up-sent-to = Akekho obephendule ku-“{ $subject }”, ngakho i-Katna ilandelele.
notify-follow-up-waiting = Ukulandelela akuthunyelwanga
notify-follow-up-waiting-to = Isikhathi sakho sifike ngesikhathi le khompyutha ivaliwe. “{ $subject }” sesibuyele ebhokisini lakho lokungenayo.
notify-tracking-opened = U-{ $who } uvule { $subject }
notify-tracking-clicked = U-{ $who } uchofoze isixhumanisi ku-{ $subject }

## An update of Katna is downloaded and ready to install

notify-update-ready = I-Katna Mail ingabuyekezwa
notify-update-ready-body = Inguqulo { $version } ilandiwe. Ukubuyekeza kuyifaka bese kuqala kabusha i-Katna Mail.
notify-update = Buyekeza

## Something needs the user, shown once per problem

notify-signed-out = Ngena futhi
notify-signed-out-body = { $provider } ikhiphe i-Katna ku-{ $address }. Imeyili iyekile ukuvumelanisa.
notify-sign-in = Ngena
notify-password-refused = Iphasiwedi yenqatshiwe
notify-password-refused-body = Iseva yemeyili yenqabe iphasiwedi ka-{ $address }. Kungenzeka ishintshile.
notify-new-password = Iphasiwedi entsha
notify-not-sent = “{ $subject }” akuthunyelwanga
notify-not-sent-no-subject = Umlayezo awuthunyelwanga
notify-not-sent-body = Ukubhokisi eliphumayo, elichaza ukuthi kungani.
notify-open-outbox = Vula ibhokisi eliphumayo
notify-event-now = Manje
notify-event-in-minutes = { $count ->
    [one] Emizuzwini engu-{ $count }
   *[other] Emizuzwini engu-{ $count }
}
notify-event-in-hours = { $count ->
    [one] Emahoreni angu-{ $count }
   *[other] Emahoreni angu-{ $count }
}
notify-event-in-days = { $count ->
    [1] Kusasa
    [one] Ezinsukwini ezingu-{ $count }
   *[other] Ezinsukwini ezingu-{ $count }
}
notify-event-all-day = Usuku lonke
notify-event-join = Joyina
notify-event-snooze = Libazisa imizuzu emi-5
notify-task-done = Maka njengokwenziwe

## Its buttons

notify-open = Vula
notify-peek = Lunguza
notify-reply = Phendula
notify-reply-placeholder = Phendula u-{ $name }…
notify-send = Thumela
notify-reply-quote-header = Mhla { $date }, { $from } wabhala:
notify-reply-quote-header-no-date = { $from } wabhala:
notify-reply-all = Phendula bonke
notify-mark-read = Maka njengokufundiwe
notify-mark-all-read = Maka konke njengokufundiwe
notify-archive = Faka kungobo yomlando
notify-snooze-hour = Libazisa ihora eli-1
notify-snooze-tomorrow = Kusasa
notify-copy-code = Kopisha { $code }
notify-link-verify = Qinisekisa ku-{ $domain }
notify-link-confirm = Qinisekisa ku-{ $domain }
notify-link-activate = Yenza kusebenze ku-{ $domain }

## After Archive on a notification: a short note in the same place

notify-archived = Kufakwe engobeni yomlando
notify-archived-count = { $count ->
    [one] Umlayezo o-{ $count } ukhishwe ebhokisini lokungenayo
   *[other] Imilayezo engu-{ $count } ikhishwe ebhokisini lokungenayo
}
notify-undo = Hlehlisa

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = Ikhodi ikopishiwe
notify-code-not-copied = Akukwazekanga ukukopisha ikhodi

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = Impendulo ithunyelwe ku-{ $name }
notify-open-in-katna = Vula ku-Katna
