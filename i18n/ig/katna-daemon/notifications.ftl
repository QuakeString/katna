# Katna Mail, Igbo (Igbo).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = Email ọhụrụ { $count }
notify-and-more = na { $count } ọzọ
notify-no-subject = (enweghị isiokwu)
notify-unknown-sender = Onye zitere amaghị
notify-snooze-back = Alọghachila site na iyigharị
notify-no-reply = Azịza apụtabeghị
notify-no-reply-to = Ọ dịghị onye zaghachiri “{ $subject }”.
notify-tracking-opened = { $who } mepere { $subject }
notify-tracking-clicked = { $who } pịrị njikọ dị na { $subject }

## An update of Katna is downloaded and ready to install

notify-update-ready = Enwere ike imelite Katna Mail
notify-update-ready-body = Ụdị { $version } abudatala. Imelite na-awụnye ya wee malitegharịa Katna Mail.
notify-update = Melite
notify-event-now = Ugbu a
notify-event-in-minutes = { $count ->
   *[other] N’ime nkeji { $count }
}
notify-event-in-hours = { $count ->
   *[other] N’ime awa { $count }
}
notify-event-in-days = { $count ->
    [1] Echi
   *[other] N’ime ụbọchị { $count }
}
notify-event-all-day = Ụbọchị niile
notify-event-join = Sonye
notify-event-snooze = Yigharịa nkeji 5
notify-task-done = Maa ka emechara

## Its buttons

notify-open = Mepee
notify-peek = Lelee
notify-reply = Zaa
notify-reply-placeholder = Zaa { $name }…
notify-send = Zipu
notify-reply-all = Zaa mmadụ niile
notify-mark-read = Kaa akara dị ka agụrụ
notify-mark-all-read = Kaa akara na niile dị ka agụrụ
notify-archive = Chekwaa

## After Archive on a notification: a short note in the same place

notify-archived = Echekwala
notify-archived-count = { $count ->
   *[other] Ozi { $count } apụla n'igbe ozi mbata
}
notify-undo = Megharịa

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = Ezigara { $name } nzaghachi
notify-open-in-katna = Mepee na Katna
