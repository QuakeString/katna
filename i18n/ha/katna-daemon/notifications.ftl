# Katna Mail, Hausa (Hausa).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count ->
    [one] sabon imel { $count }
   *[other] sababbin imel { $count }
}
notify-and-more = da ƙarin { $count }
notify-no-subject = (babu jigo)
notify-unknown-sender = Mai aikawa da ba a sani ba
notify-snooze-back = Ya dawo daga jinkiri
notify-no-reply = Babu amsa tukuna
notify-no-reply-to = Babu wanda ya amsa “{ $subject }”.
notify-follow-up-sent = An aika bibiya
notify-follow-up-sent-to = Ba wanda ya amsa “{ $subject }”, don haka Katna ta yi bibiya.
notify-follow-up-waiting = Ba a aika bibiya ba
notify-follow-up-waiting-to = Lokacinta ya yi yayin da wannan kwamfuta a kashe take. “{ $subject }” ya koma Akwatin saƙonku.
notify-tracking-opened = { $who } ya buɗe { $subject }
notify-tracking-clicked = { $who } ya danna mahaɗi a cikin { $subject }

notify-update-ready = Ana iya sabunta Katna Mail
notify-update-ready-body = An sauke sigar { $version }. Sabunta yana shigar da ita kuma yana sake kunna Katna Mail.
notify-update = Sabunta

## Something needs the user, shown once per problem

notify-signed-out = Sake shiga
notify-signed-out-body = { $provider } ta fitar da Katna daga { $address }. Wasiƙu sun daina daidaitawa.
notify-sign-in = Shiga
notify-password-refused = An ƙi kalmar sirri
notify-password-refused-body = Sabar wasiƙa ta ƙi kalmar sirrin { $address }. Wataƙila an canza ta.
notify-new-password = Sabuwar kalmar sirri
notify-not-sent = Ba a aika “{ $subject }” ba
notify-not-sent-no-subject = Ba a aika wani saƙo ba
notify-not-sent-body = Yana cikin Akwatin fita, wanda ke faɗin dalili.
notify-open-outbox = Buɗe Akwatin fita
notify-event-now = Yanzu
notify-event-in-minutes = { $count ->
    [one] Nan da minti { $count }
   *[other] Nan da minti { $count }
}
notify-event-in-hours = { $count ->
    [one] Nan da awa { $count }
   *[other] Nan da awa { $count }
}
notify-event-in-days = { $count ->
    [1] Gobe
    [one] Nan da kwana { $count }
   *[other] Nan da kwana { $count }
}
notify-event-all-day = Duk rana
notify-event-join = Shiga
notify-event-snooze = Jinkirta na mintuna 5
notify-task-done = Yi alama an gama

## Its buttons

notify-open = Buɗe
notify-peek = Leƙa
notify-reply = Amsa
notify-reply-placeholder = Amsa wa { $name }…
notify-send = Aika
notify-reply-all = Amsa wa kowa
notify-mark-read = Yi alama an karanta
notify-mark-all-read = Yi wa duka alama an karanta
notify-archive = Adana a ma'ajiya
notify-snooze-hour = Jinkirta awa 1
notify-snooze-tomorrow = Gobe
notify-copy-code = Kwafi { $code }
notify-link-verify = Tabbatar a { $domain }
notify-link-confirm = Amince a { $domain }
notify-link-activate = Kunna a { $domain }

## After Archive on a notification: a short note in the same place

notify-archived = An adana a ma'ajiya
notify-archived-count = { $count ->
    [one] An fitar da saƙo { $count } daga akwatin saƙo
   *[other] An fitar da saƙonni { $count } daga akwatin saƙo
}
notify-undo = Janye

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = An kwafi lamba
notify-code-not-copied = Ba a iya kwafin lambar ba

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = An aika amsa ga { $name }
notify-open-in-katna = Buɗe a Katna
