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
notify-tracking-opened = U-{ $who } uvule { $subject }
notify-tracking-clicked = U-{ $who } uchofoze isixhumanisi ku-{ $subject }

## An update of Katna is downloaded and ready to install

notify-update-ready = I-Katna Mail ingabuyekezwa
notify-update-ready-body = Inguqulo { $version } ilandiwe. Ukubuyekeza kuyifaka bese kuqala kabusha i-Katna Mail.
notify-update = Buyekeza
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

## Its buttons

notify-open = Vula
notify-reply-all = Phendula bonke
notify-mark-read = Maka njengokufundiwe
notify-mark-all-read = Maka konke njengokufundiwe
notify-archive = Faka kungobo yomlando
