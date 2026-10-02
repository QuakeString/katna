# Katna Mail, Tamil (தமிழ்).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

notify-new-emails = { $count ->
    [one] { $count } புதிய மின்னஞ்சல்
   *[other] { $count } புதிய மின்னஞ்சல்கள்
}
notify-and-more = மேலும் { $count }
notify-no-subject = (பொருள் இல்லை)
notify-unknown-sender = அறியாத அனுப்புநர்

## Reminders the user asked for (same buttons)

notify-snooze-back = உறக்கநிலையிலிருந்து திரும்பியவை
notify-no-reply = இன்னும் பதில் இல்லை
notify-no-reply-to = “{ $subject }” என்பதற்கு யாரும் பதிலளிக்கவில்லை.

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } { $subject } மெயிலைத் திறந்தார்
notify-tracking-clicked = { $who } { $subject } மெயிலில் ஒரு லிங்க்கைக் கிளிக் செய்தார்

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail-ஐ புதுப்பிக்கலாம்
notify-update-ready-body = பதிப்பு { $version } பதிவிறக்கப்பட்டுள்ளது. புதுப்பி அதை நிறுவி Katna Mail-ஐ மறுதொடக்கம் செய்யும்.
notify-update = புதுப்பி

## Reminders of calendar events

notify-event-now = இப்போது
notify-event-in-minutes = { $count ->
    [one] { $count } நிமிடங்களில்
   *[other] { $count } நிமிடங்களில்
}
notify-event-in-hours = { $count ->
    [one] { $count } மணி நேரத்தில்
   *[other] { $count } மணி நேரத்தில்
}
notify-event-in-days = { $count ->
    [1] நாளை
    [one] { $count } நாட்களில்
   *[other] { $count } நாட்களில்
}
notify-event-all-day = நாள் முழுவதும்
notify-event-join = சேர்
notify-event-snooze = 5 நிமிடம் உறக்கநிலையில் வை
notify-task-done = முடிந்ததாகக் குறி

## The buttons of new-mail notifications and reminders

notify-open = திற
notify-peek = பார்
notify-reply = பதிலளி
notify-reply-placeholder = { $name } க்குப் பதிலளி…
notify-send = அனுப்பு
notify-reply-all = அனைவருக்கும் பதிலளி
notify-mark-read = படித்ததாகக் குறி
notify-mark-all-read = அனைத்தையும் படித்ததாகக் குறி
notify-archive = காப்பகப்படுத்து

## After Archive on a notification: a short note in the same place

notify-archived = காப்பகப்படுத்தப்பட்டது
notify-archived-count = { $count ->
    [one] { $count } மெசேஜ் இன்பாக்ஸிலிருந்து நகர்த்தப்பட்டது
   *[other] { $count } மெசேஜ்கள் இன்பாக்ஸிலிருந்து நகர்த்தப்பட்டன
}
notify-undo = செயல்தவிர்

## it waits for the undo time

notify-reply-sent = { $name } க்குப் பதில் அனுப்பப்பட்டது
notify-open-in-katna = Katna-வில் திற
