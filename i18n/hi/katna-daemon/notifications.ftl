# Katna Mail, Hindi (हिन्दी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count ->
    [one] { $count } नया ईमेल
   *[other] { $count } नए ईमेल
}
notify-and-more = और { $count }
notify-no-subject = (कोई विषय नहीं)
notify-unknown-sender = अज्ञात भेजने वाला
notify-snooze-back = स्नूज़ से वापस
notify-no-reply = अभी तक कोई जवाब नहीं
notify-no-reply-to = “{ $subject }” का किसी ने जवाब नहीं दिया।
notify-tracking-opened = { $who } ने { $subject } खोला
notify-tracking-clicked = { $who } ने { $subject } में एक लिंक पर क्लिक किया
notify-update-ready = Katna Mail को अपडेट किया जा सकता है
notify-update-ready-body = संस्करण { $version } डाउनलोड हो चुका है। अपडेट इसे इंस्टॉल करता है और Katna Mail को रीस्टार्ट करता है।
notify-update = अपडेट
notify-event-now = अभी
notify-event-in-minutes = { $count ->
    [one] { $count } मिनट में
   *[other] { $count } मिनट में
}
notify-event-in-hours = { $count ->
    [one] { $count } घंटे में
   *[other] { $count } घंटे में
}
notify-event-in-days = { $count ->
    [1] कल
    [one] { $count } दिन में
   *[other] { $count } दिन में
}
notify-event-all-day = पूरे दिन
notify-event-join = शामिल हों
notify-event-snooze = 5 मिनट स्नूज़ करें
notify-task-done = पूरा हुआ चिह्नित करें

## Its buttons

notify-open = खोलें
notify-peek = झलक देखें
notify-reply = जवाब दें
notify-reply-placeholder = { $name } को जवाब दें…
notify-send = भेजें
notify-reply-all = सभी को जवाब दें
notify-mark-read = पढ़ा गया के रूप में मार्क करें
notify-mark-all-read = सभी को पढ़ा गया के रूप में मार्क करें
notify-archive = संग्रह करें

## After Archive on a notification: a short note in the same place

notify-archived = संग्रह किया गया
notify-archived-count = { $count ->
    [one] { $count } मैसेज इनबॉक्स से हटाया गया
   *[other] { $count } मैसेज इनबॉक्स से हटाए गए
}
notify-undo = पहले जैसा करें

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = { $name } को जवाब भेजा गया
notify-open-in-katna = Katna में खोलें
