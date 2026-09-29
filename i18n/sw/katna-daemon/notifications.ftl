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
notify-tracking-opened = { $who } amefungua { $subject }
notify-tracking-clicked = { $who } amebofya kiungo katika { $subject }

notify-update-ready = Katna Mail inaweza kusasishwa
notify-update-ready-body = Toleo { $version } limepakuliwa. Sasisha huliweka na kuanzisha upya Katna Mail.
notify-update = Sasisha
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
notify-reply-all = Jibu wote
notify-mark-read = Tia alama kuwa imesomwa
notify-mark-all-read = Tia alama zote kuwa zimesomwa
notify-archive = Weka kwenye kumbukumbu
