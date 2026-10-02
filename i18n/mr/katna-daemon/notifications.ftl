# Katna Mail, Marathi (मराठी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count } नवीन ईमेल
notify-and-more = आणि आणखी { $count }
notify-no-subject = (विषय नाही)
notify-unknown-sender = अज्ञात प्रेषक
notify-snooze-back = स्नूझमधून परत आले
notify-no-reply = अजून उत्तर नाही
notify-no-reply-to = “{ $subject }” ला अजून कोणीही उत्तर दिलेले नाही.
notify-tracking-opened = { $who } यांनी { $subject } उघडले
notify-tracking-clicked = { $who } यांनी { $subject } मधील लिंकवर क्लिक केले
notify-update-ready = Katna Mail अपडेट करता येते
notify-update-ready-body = आवृत्ती { $version } डाउनलोड झाली आहे. अपडेट ती इंस्टॉल करते आणि Katna Mail रीस्टार्ट करते.
notify-update = अपडेट
notify-event-now = आत्ता
notify-event-in-minutes = { $count ->
    [one] { $count } मिनिटांत
   *[other] { $count } मिनिटांत
}
notify-event-in-hours = { $count ->
    [one] { $count } तासात
   *[other] { $count } तासात
}
notify-event-in-days = { $count ->
    [1] उद्या
    [one] { $count } दिवसांत
   *[other] { $count } दिवसांत
}
notify-event-all-day = पूर्ण दिवस
notify-event-join = सहभागी व्हा
notify-event-snooze = 5 मिनिटे स्नूझ करा
notify-task-done = पूर्ण म्हणून खूण करा

## Its buttons

notify-open = उघडा
notify-peek = झलक
notify-reply = उत्तर द्या
notify-reply-placeholder = { $name } यांना उत्तर द्या…
notify-send = पाठवा
notify-reply-all = सर्वांना उत्तर द्या
notify-mark-read = वाचलेले म्हणून खूण करा
notify-mark-all-read = सर्व वाचलेले म्हणून खूण करा
notify-archive = संग्रहित करा

## After Archive on a notification: a short note in the same place

notify-archived = संग्रहित केले
notify-archived-count = { $count ->
    [one] { $count } मेसेज इनबॉक्समधून हलवला
   *[other] { $count } मेसेज इनबॉक्समधून हलवले
}
notify-undo = पूर्ववत करा

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = { $name } यांना उत्तर पाठवले
notify-open-in-katna = Katna मध्ये उघडा
