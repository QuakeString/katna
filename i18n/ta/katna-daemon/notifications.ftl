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
notify-follow-up-sent = ஃபாலோ-அப் அனுப்பப்பட்டது
notify-follow-up-sent-to = “{ $subject }” க்கு யாரும் பதிலளிக்கவில்லை, எனவே Katna ஃபாலோ-அப் அனுப்பியது.
notify-follow-up-waiting = ஃபாலோ-அப் அனுப்பப்படவில்லை
notify-follow-up-waiting-to = இந்தக் கணினி அணைந்திருந்தபோது அதன் நேரம் வந்தது. “{ $subject }” உங்கள் இன்பாக்ஸுக்குத் திரும்பியுள்ளது.

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } { $subject } மெயிலைத் திறந்தார்
notify-tracking-clicked = { $who } { $subject } மெயிலில் ஒரு லிங்க்கைக் கிளிக் செய்தார்

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail-ஐ புதுப்பிக்கலாம்
notify-update-ready-body = பதிப்பு { $version } பதிவிறக்கப்பட்டுள்ளது. புதுப்பி அதை நிறுவி Katna Mail-ஐ மறுதொடக்கம் செய்யும்.
notify-update = புதுப்பி

## Something needs the user, shown once per problem

notify-signed-out = மீண்டும் உள்நுழையுங்கள்
notify-signed-out-body = { $provider } { $address } இலிருந்து Katna-வை வெளியேற்றியது. அஞ்சல் ஒத்திசைவு நின்றுவிட்டது.
notify-sign-in = உள்நுழை
notify-password-refused = கடவுச்சொல் ஏற்கப்படவில்லை
notify-password-refused-body = அஞ்சல் சர்வர் { $address } க்கான கடவுச்சொல்லை ஏற்கவில்லை. அது மாறியிருக்கலாம்.
notify-new-password = புதிய கடவுச்சொல்
notify-not-sent = “{ $subject }” அனுப்பப்படவில்லை
notify-not-sent-no-subject = ஒரு மெசேஜ் அனுப்பப்படவில்லை
notify-not-sent-body = அது அவுட்பாக்ஸில் உள்ளது; ஏன் என்று அங்கே சொல்லப்பட்டுள்ளது.
notify-open-outbox = அவுட்பாக்ஸைத் திற

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
notify-snooze-hour = 1 மணிநேரம் உறக்கநிலையில் வை
notify-snooze-tomorrow = நாளை
notify-copy-code = { $code } ஐ நகலெடு
notify-link-verify = { $domain } இல் சரிபார்
notify-link-confirm = { $domain } இல் உறுதிப்படுத்து
notify-link-activate = { $domain } இல் செயல்படுத்து

## After Archive on a notification: a short note in the same place

notify-archived = காப்பகப்படுத்தப்பட்டது
notify-archived-count = { $count ->
    [one] { $count } மெசேஜ் இன்பாக்ஸிலிருந்து நகர்த்தப்பட்டது
   *[other] { $count } மெசேஜ்கள் இன்பாக்ஸிலிருந்து நகர்த்தப்பட்டன
}
notify-undo = செயல்தவிர்

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = குறியீடு நகலெடுக்கப்பட்டது
notify-code-not-copied = குறியீட்டை நகலெடுக்க முடியவில்லை

## it waits for the undo time

notify-reply-sent = { $name } க்குப் பதில் அனுப்பப்பட்டது
notify-open-in-katna = Katna-வில் திற
