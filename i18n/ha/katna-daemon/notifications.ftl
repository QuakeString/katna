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
notify-tracking-opened = { $who } ya buɗe { $subject }
notify-tracking-clicked = { $who } ya danna mahaɗi a cikin { $subject }

notify-update-ready = Ana iya sabunta Katna Mail
notify-update-ready-body = An sauke sigar { $version }. Sabunta yana shigar da ita kuma yana sake kunna Katna Mail.
notify-update = Sabunta

## Its buttons

notify-open = Buɗe
notify-reply-all = Amsa wa kowa
notify-mark-read = Yi alama an karanta
notify-mark-all-read = Yi wa duka alama an karanta
notify-archive = Adana a ma'ajiya
