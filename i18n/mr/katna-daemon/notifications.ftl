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
notify-follow-up-sent = पाठपुरावा पाठवला
notify-follow-up-sent-to = “{ $subject }” ला कोणीही उत्तर दिले नव्हते, म्हणून Katna ने पाठपुरावा केला.
notify-follow-up-waiting = पाठपुरावा पाठवला नाही
notify-follow-up-waiting-to = हा कॉम्प्युटर बंद असताना त्याची वेळ झाली. “{ $subject }” तुमच्या इनबॉक्समध्ये परत आला आहे.
notify-tracking-opened = { $who } यांनी { $subject } उघडले
notify-tracking-clicked = { $who } यांनी { $subject } मधील लिंकवर क्लिक केले
notify-update-ready = Katna Mail अपडेट करता येते
notify-update-ready-body = आवृत्ती { $version } डाउनलोड झाली आहे. अपडेट ती इंस्टॉल करते आणि Katna Mail रीस्टार्ट करते.
notify-update = अपडेट

## Something needs the user, shown once per problem

notify-signed-out = पुन्हा साइन इन करा
notify-signed-out-body = { $provider } ने Katna ला { $address } मधून साइन आउट केले. मेल सिंक होणे थांबले.
notify-sign-in = साइन इन करा
notify-password-refused = पासवर्ड नाकारला
notify-password-refused-body = मेल सर्व्हरने { $address } चा पासवर्ड नाकारला. तो बदललेला असू शकतो.
notify-new-password = नवीन पासवर्ड
notify-not-sent = “{ $subject }” पाठवला गेला नाही
notify-not-sent-no-subject = एक मेसेज पाठवला गेला नाही
notify-not-sent-body = तो आउटबॉक्समध्ये आहे, तिथे कारण दिले आहे.
notify-open-outbox = आउटबॉक्स उघडा
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
notify-snooze-hour = 1 तास स्नूझ करा
notify-snooze-tomorrow = उद्या
notify-copy-code = { $code } कॉपी करा
notify-link-verify = { $domain } वर पडताळा
notify-link-confirm = { $domain } वर पुष्टी करा
notify-link-activate = { $domain } वर सक्रिय करा

## After Archive on a notification: a short note in the same place

notify-archived = संग्रहित केले
notify-archived-count = { $count ->
    [one] { $count } मेसेज इनबॉक्समधून हलवला
   *[other] { $count } मेसेज इनबॉक्समधून हलवले
}
notify-undo = पूर्ववत करा

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = कोड कॉपी केला
notify-code-not-copied = कोड कॉपी करता आला नाही

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = { $name } यांना उत्तर पाठवले
notify-open-in-katna = Katna मध्ये उघडा
