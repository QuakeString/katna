# Katna Mail, Nepali (नेपाली).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count } नयाँ इमेल
notify-and-more = र थप { $count }
notify-no-subject = (विषय छैन)
notify-unknown-sender = अज्ञात प्रेषक
notify-snooze-back = स्नुजबाट फर्कियो
notify-no-reply = अझै जवाफ आएको छैन
notify-no-reply-to = “{ $subject }” को जवाफ कसैले दिएको छैन।
notify-tracking-opened = { $who } ले { $subject } खोल्नुभयो
notify-tracking-clicked = { $who } ले { $subject } मा भएको लिङ्कमा क्लिक गर्नुभयो
notify-update-ready = Katna Mail अपडेट गर्न सकिन्छ
notify-update-ready-body = संस्करण { $version } डाउनलोड भइसक्यो। अपडेटले यसलाई स्थापना गर्छ र Katna Mail लाई रीस्टार्ट गर्छ।
notify-update = अपडेट
notify-event-now = अहिले
notify-event-in-minutes = { $count ->
    [one] { $count } मिनेटमा
   *[other] { $count } मिनेटमा
}
notify-event-in-hours = { $count ->
    [one] { $count } घण्टामा
   *[other] { $count } घण्टामा
}
notify-event-in-days = { $count ->
    [1] भोलि
    [one] { $count } दिनमा
   *[other] { $count } दिनमा
}
notify-event-all-day = दिनभर
notify-event-join = सामेल हुनुहोस्
notify-event-snooze = 5 मिनेट स्नुज गर्नुहोस्
notify-task-done = सम्पन्न भनी चिह्नित गर्नुहोस्

## Its buttons

notify-open = खोल्नुहोस्
notify-peek = झल्को हेर्नुहोस्
notify-reply = जवाफ दिनुहोस्
notify-reply-placeholder = { $name } लाई जवाफ…
notify-send = पठाउनुहोस्
notify-reply-all = सबैलाई जवाफ दिनुहोस्
notify-mark-read = पढिएको भनी चिन्ह लगाउनुहोस्
notify-mark-all-read = सबैलाई पढिएको भनी चिन्ह लगाउनुहोस्
notify-archive = संग्रह गर्नुहोस्

## After Archive on a notification: a short note in the same place

notify-archived = संग्रह गरियो
notify-archived-count = { $count ->
    [one] { $count } सन्देश इनबक्सबाट सारियो
   *[other] { $count } सन्देशहरू इनबक्सबाट सारिए
}
notify-undo = पूर्ववत गर्नुहोस्

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = { $name } लाई जवाफ पठाइयो
notify-open-in-katna = Katna मा खोल्नुहोस्
