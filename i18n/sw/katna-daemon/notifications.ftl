# Katna Mail, Swahili (Kiswahili).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = Barua pepe { $count } mpya
notify-and-more = na { $count } zaidi
notify-no-subject = (hakuna mada)
notify-unknown-sender = Mtumaji asiyejulikana
notify-snooze-back = Zimerudi baada ya kuahirishwa
notify-no-reply = Bado hakuna jibu
notify-no-reply-to = Hakuna aliyejibu “{ $subject }”.
notify-follow-up-sent = Ufuatiliaji umetumwa
notify-follow-up-sent-to = Hakuna aliyekuwa amejibu “{ $subject }”, hivyo Katna ilifuatilia.
notify-follow-up-waiting = Ufuatiliaji haukutumwa
notify-follow-up-waiting-to = Muda wake ulifika wakati kompyuta hii ilikuwa imezimwa. “{ $subject }” imerudi kwenye Kikasha chako.
notify-tracking-opened = { $who } amefungua { $subject }
notify-tracking-clicked = { $who } amebofya kiungo katika { $subject }

notify-update-ready = Katna Mail inaweza kusasishwa
notify-update-ready-body = Toleo { $version } limepakuliwa. Sasisha huliweka na kuanzisha upya Katna Mail.
notify-update = Sasisha

## Something needs the user, shown once per problem

notify-signed-out = Ingia tena
notify-signed-out-body = { $provider } imeitoa Katna kwenye { $address }. Barua zimeacha kusawazishwa.
notify-sign-in = Ingia
notify-password-refused = Nenosiri limekataliwa
notify-password-refused-body = Seva ya barua imekataa nenosiri la { $address }. Huenda limebadilika.
notify-new-password = Nenosiri jipya
notify-not-sent = “{ $subject }” haikutumwa
notify-not-sent-no-subject = Ujumbe haukutumwa
notify-not-sent-body = Uko kwenye Kikasha toezi, kinachoeleza sababu.
notify-open-outbox = Fungua Kikasha toezi
notify-event-now = Sasa
notify-event-in-minutes = { $count ->
    [one] Baada ya dakika { $count }
   *[other] Baada ya dakika { $count }
}
notify-event-in-hours = { $count ->
    [one] Baada ya saa { $count }
   *[other] Baada ya saa { $count }
}
notify-event-in-days = { $count ->
    [1] Kesho
    [one] Baada ya siku { $count }
   *[other] Baada ya siku { $count }
}
notify-event-all-day = Siku nzima
notify-event-join = Jiunge
notify-event-snooze = Ahirisha dakika 5
notify-task-done = Weka alama kuwa imekamilika

## Its buttons

notify-open = Fungua
notify-peek = Chungulia
notify-reply = Jibu
notify-reply-placeholder = Mjibu { $name }…
notify-send = Tuma
notify-reply-all = Jibu wote
notify-mark-read = Tia alama kuwa imesomwa
notify-mark-all-read = Tia alama zote kuwa zimesomwa
notify-archive = Weka kwenye kumbukumbu
notify-snooze-hour = Ahirisha saa 1
notify-snooze-tomorrow = Kesho
notify-copy-code = Nakili { $code }
notify-link-verify = Thibitisha kwenye { $domain }
notify-link-confirm = Kubali kwenye { $domain }
notify-link-activate = Amilisha kwenye { $domain }

## After Archive on a notification: a short note in the same place

notify-archived = Imewekwa kwenye kumbukumbu
notify-archived-count = { $count ->
    [one] Ujumbe { $count } umeondolewa kwenye kikasha
   *[other] Jumbe { $count } zimeondolewa kwenye kikasha
}
notify-undo = Tendua

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = Msimbo umenakiliwa
notify-code-not-copied = Imeshindwa kunakili msimbo

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = Jibu limetumwa kwa { $name }
notify-open-in-katna = Fungua katika Katna
