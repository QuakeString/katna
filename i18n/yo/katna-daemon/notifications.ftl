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
notify-follow-up-sent = A ti fi ìtẹ̀lé ránṣẹ́
notify-follow-up-sent-to = Kò sí ẹni tó fèsì sí “{ $subject }”, nítorí náà Katna ti tẹ̀lé e.
notify-follow-up-waiting = A kò fi ìtẹ̀lé ránṣẹ́
notify-follow-up-waiting-to = Àkókò rẹ̀ tó nígbà tí kọ̀ǹpútà yìí wà ní pípa. “{ $subject }” ti padà sínú Àpótí-ìwọlé rẹ.
notify-tracking-opened = { $who } ti ṣí { $subject }
notify-tracking-clicked = { $who } ti tẹ ìjápọ̀ kan nínú { $subject }

notify-update-ready = A lè ṣe ìmúdójúìwọ̀n Katna Mail
notify-update-ready-body = A ti gba ẹ̀yà { $version } sílẹ̀. Ìmúdójúìwọ̀n ń fi í sórí ẹrọ tí ó sì tún Katna Mail bẹ̀rẹ̀.
notify-update = Ìmúdójúìwọ̀n

## Something needs the user, shown once per problem

notify-signed-out = Wọlé lẹ́ẹ̀kan sí i
notify-signed-out-body = { $provider } ti mú Katna jáde kúrò nínú { $address }. Lẹ́tà ti dáwọ́ ìbámu dúró.
notify-sign-in = Wọlé
notify-password-refused = A kọ ọ̀rọ̀ aṣínà
notify-password-refused-body = Sáfà lẹ́tà kọ ọ̀rọ̀ aṣínà fún { $address }. Ó lè jẹ́ pé ó ti yí padà.
notify-new-password = Ọ̀rọ̀ aṣínà tuntun
notify-not-sent = A kò fi “{ $subject }” ránṣẹ́
notify-not-sent-no-subject = A kò fi ìfiránṣẹ́ kan ránṣẹ́
notify-not-sent-body = Ó wà nínú Àpótí-ìjáde, tí ó sọ ìdí rẹ̀.
notify-open-outbox = Ṣí Àpótí-ìjáde
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
notify-snooze-hour = Sún síwájú fún wákàtí kan
notify-snooze-tomorrow = Ọ̀la
notify-copy-code = Ṣẹ̀dà { $code }
notify-link-verify = Fìdí rẹ̀ múlẹ̀ lórí { $domain }
notify-link-confirm = Jẹ́rìí sí i lórí { $domain }
notify-link-activate = Mú un ṣiṣẹ́ lórí { $domain }

## After Archive on a notification: a short note in the same place

notify-archived = A ti fi pamọ́
notify-archived-count = { $count ->
   *[other] A ti gbé ìfiránṣẹ́ { $count } kúrò nínú àpótí-ìwọlé
}
notify-undo = Dá padà

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = A ti ṣẹ̀dà kóòdù náà
notify-code-not-copied = A kò lè ṣẹ̀dà kóòdù náà

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = A ti fi èsì ránṣẹ́ sí { $name }
notify-open-in-katna = Ṣí i nínú Katna
