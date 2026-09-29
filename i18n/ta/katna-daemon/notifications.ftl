# Katna Mail, Tamil (தமிழ்).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count ->
    [one] { $count } புதிய மின்னஞ்சல்
   *[other] { $count } புதிய மின்னஞ்சல்கள்
}
notify-and-more = மேலும் { $count }
notify-no-subject = (பொருள் இல்லை)
notify-unknown-sender = அறியாத அனுப்புநர்
notify-snooze-back = உறக்கநிலையிலிருந்து திரும்பியவை
notify-no-reply = இன்னும் பதில் இல்லை
notify-no-reply-to = “{ $subject }” என்பதற்கு யாரும் பதிலளிக்கவில்லை.
notify-tracking-opened = { $who } { $subject } மெயிலைத் திறந்தார்
notify-tracking-clicked = { $who } { $subject } மெயிலில் ஒரு லிங்க்கைக் கிளிக் செய்தார்

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail-ஐ புதுப்பிக்கலாம்
notify-update-ready-body = பதிப்பு { $version } பதிவிறக்கப்பட்டுள்ளது. புதுப்பி அதை நிறுவி Katna Mail-ஐ மறுதொடக்கம் செய்யும்.
notify-update = புதுப்பி
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

## Its buttons

notify-open = திற
notify-reply-all = அனைவருக்கும் பதிலளி
notify-mark-read = படித்ததாகக் குறி
notify-mark-all-read = அனைத்தையும் படித்ததாகக் குறி
notify-archive = காப்பகப்படுத்து
