# Katna Mail, Yoruba (Yorùbá).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = Ímeèlì tuntun { $count }
notify-and-more = àti { $count } míì
notify-no-subject = (kò sí àkọlé)
notify-unknown-sender = Olùfiránṣẹ́ àìmọ̀
notify-snooze-back = Àwọn lẹ́tà tí a sún síwájú ti padà
notify-no-reply = Kò tíì sí èsì
notify-no-reply-to = Kò sí ẹni tó fèsì sí “{ $subject }”.
notify-tracking-opened = { $who } ti ṣí { $subject }
notify-tracking-clicked = { $who } ti tẹ ìjápọ̀ kan nínú { $subject }

notify-update-ready = A lè ṣe ìmúdójúìwọ̀n Katna Mail
notify-update-ready-body = A ti gba ẹ̀yà { $version } sílẹ̀. Ìmúdójúìwọ̀n ń fi í sórí ẹrọ tí ó sì tún Katna Mail bẹ̀rẹ̀.
notify-update = Ìmúdójúìwọ̀n
notify-event-now = Báyìí
notify-event-in-minutes = { $count ->
   *[other] Lẹ́yìn ìṣẹ́jú { $count }
}
notify-event-in-hours = { $count ->
   *[other] Lẹ́yìn wákàtí { $count }
}
notify-event-in-days = { $count ->
    [1] Ọ̀la
   *[other] Lẹ́yìn ọjọ́ { $count }
}
notify-event-all-day = Ní gbogbo ọjọ́
notify-event-join = Darapọ̀
notify-event-snooze = Sún síwájú ní ìṣẹ́jú 5
notify-task-done = Ṣàmì sí pé ó ti parí

## Its buttons

notify-open = Ṣí
notify-peek = Yọjú wò ó
notify-reply = Fèsì
notify-reply-placeholder = Fèsì sí { $name }…
notify-send = Fi ránṣẹ́
notify-reply-all = Fèsì sí gbogbo
notify-mark-read = Sàmì sí bí kíkà
notify-mark-all-read = Sàmì sí gbogbo rẹ̀ bí kíkà
notify-archive = Fi pamọ́

## After Archive on a notification: a short note in the same place

notify-archived = A ti fi pamọ́
notify-archived-count = { $count ->
   *[other] A ti gbé ìfiránṣẹ́ { $count } kúrò nínú àpótí-ìwọlé
}
notify-undo = Dá padà

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = A ti fi èsì ránṣẹ́ sí { $name }
notify-open-in-katna = Ṣí i nínú Katna
