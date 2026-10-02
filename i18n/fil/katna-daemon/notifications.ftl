# Katna Mail, Filipino (Filipino).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

notify-new-emails = { $count } bagong email
notify-and-more = at { $count } pa
notify-no-subject = (walang subject)
notify-unknown-sender = Hindi kilalang nagpadala

## Reminders the user asked for (same buttons)

notify-snooze-back = Bumalik mula sa snooze
notify-no-reply = Wala pang sagot
notify-no-reply-to = Wala pang sumasagot sa “{ $subject }”.

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = Binuksan ni { $who } ang { $subject }
notify-tracking-clicked = Nag-click si { $who } ng link sa { $subject }

## An update of Katna is downloaded and ready to install

notify-update-ready = Puwedeng i-update ang Katna Mail
notify-update-ready-body = Na-download na ang bersyon { $version }. Ii-install ito ng I-update at ire-restart ang Katna Mail.
notify-update = I-update

## Reminders of calendar events

notify-event-now = Ngayon
notify-event-in-minutes = { $count ->
    [one] Sa loob ng { $count } minuto
   *[other] Sa loob ng { $count } minuto
}
notify-event-in-hours = { $count ->
   *[other] Sa loob ng { $count } oras
}
notify-event-in-days = { $count ->
    [1] Bukas
   *[other] Sa loob ng { $count } araw
}
notify-event-all-day = Buong araw
notify-event-join = Sumali
notify-event-snooze = I-snooze nang 5 min
notify-task-done = Markahan bilang tapos na

## The buttons of new-mail notifications and reminders

notify-open = Buksan
notify-peek = Silipin
notify-reply = Sumagot
notify-reply-placeholder = Sumagot kay { $name }…
notify-send = Ipadala
notify-reply-all = Sumagot sa lahat
notify-mark-read = Markahan bilang nabasa na
notify-mark-all-read = Markahan lahat bilang nabasa na
notify-archive = I-archive

## After Archive on a notification: a short note in the same place

notify-archived = Na-archive
notify-archived-count = { $count ->
    [one] { $count } mensahe ang inilipat palabas ng inbox
   *[other] { $count } mensahe ang inilipat palabas ng inbox
}
notify-undo = I-undo

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = Naipadala ang sagot kay { $name }
notify-open-in-katna = Buksan sa Katna
