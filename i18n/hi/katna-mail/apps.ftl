# Katna Mail, Hindi (हिन्दी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## App rail (and the bottom bar on a phone)

rail-mail = मेल
rail-calendar = कैलेंडर
rail-contacts = संपर्क
rail-tasks = टास्क
rail-notes = नोट
rail-files = फ़ाइलें

## Rail right-click menu

rail-menu-open = { $app } खोलें
rail-menu-settings = { $app } की सेटिंग
rail-menu-turn-off = { $app } बंद करें…

## Turning an app off (Settings > Apps)

app-off-title = { $app } बंद करें?
app-off-body = Katna { $app } को सिंक करना बंद कर देता है और उसे यहां से हटा देता है:
app-off-keep = इस कंप्यूटर पर एक कॉपी रखें
app-off-keep-detail = फिर से चालू करना तुरंत होता है
app-off-remove = इस कंप्यूटर से कॉपी हटाएं
app-off-remove-detail = आपके खातों में कुछ नहीं बदलता, और फिर से चालू करने पर यह दोबारा डाउनलोड होता है। जो सिर्फ़ इस कंप्यूटर पर है या अभी भेजा नहीं गया, वह बना रहता है।
app-off-cancel = रद्द करें
app-off-confirm = बंद करें
app-off-done = { $app } बंद किया गया
app-off-note = { $app } बंद है
app-off-turn-on = चालू करें
app-off-leaves-calendar-rail = रेल और Ctrl+2
app-off-leaves-calendar-agenda = आपके मेल के बगल वाला एजेंडा
app-off-leaves-calendar-meeting = मीटिंग शेड्यूल करें, और न्योतों पर कैलेंडर में खोलें
app-off-leaves-calendar-reminders = इवेंट के रिमाइंडर
app-off-leaves-calendar-desktop = KRunner और डेस्कटॉप घड़ी में इवेंट
app-off-leaves-contacts-rail = रेल और Ctrl+3
app-off-leaves-contacts-card = भेजने वाले के कार्ड पर संपर्कों में जोड़ें
app-off-leaves-contacts-birthdays = कैलेंडर में जन्मदिन
app-off-leaves-tasks-rail = रेल और Ctrl+4
app-off-leaves-tasks-mail = मेल पर टास्क में जोड़ें, और Shift+T
app-off-leaves-tasks-calendar = कैलेंडर में टास्क
app-off-leaves-tasks-tray = ट्रे में नया टास्क, और Meta+Alt+T
app-off-leaves-tasks-reminders = टास्क के रिमाइंडर
app-off-leaves-notes-rail = रेल और Ctrl+5
app-off-leaves-notes-mail = मेल पर नोट जोड़ें
app-off-leaves-notes-meetings = इवेंट पर मीटिंग के नोट
app-off-leaves-notes-tray = ट्रे में नया नोट, और Meta+Alt+N
app-off-leaves-notes-reminders = नोट के रिमाइंडर
app-off-leaves-files-rail = रेल और Ctrl+7
app-off-leaves-files-compose = लिखते समय अटैच करने में फ़ाइलें

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = जल्द आ रहा है
app-calendar-promise = आपके CalDAV कैलेंडर, मेल से आए मीटिंग के न्योते और रिमाइंडर, आपके इनबॉक्स के बगल में।
app-tasks-promise = CalDAV के साथ सिंक होने वाली टू-डू सूचियां, और मेल से बनाए गए टास्क।
app-notes-promise = झटपट नोट, और बाद के लिए किसी मेल या बातचीत पर नोट।

## Contacts page

app-contacts-loading = आपके मेल से लोगों को इकट्ठा किया जा रहा है…
app-contacts-empty = जिन लोगों से आप मेल पर बात करते हैं, वे यहां दिखेंगे।
app-contacts-count = { $count ->
    [one] आपके मेल से { $count } व्यक्ति, सबसे ज़्यादा बातचीत वाले पहले
   *[other] आपके मेल से { $count } लोग, सबसे ज़्यादा बातचीत वाले पहले
}
app-contacts-top = { $count ->
    [one] आपके मेल से शीर्ष { $count } व्यक्ति, सबसे ज़्यादा बातचीत वाले पहले
   *[other] आपके मेल से शीर्ष { $count } लोग, सबसे ज़्यादा बातचीत वाले पहले
}
app-contacts-messages = { $count ->
    [one] { $count } मैसेज
   *[other] { $count } मैसेज
}
app-contacts-last = आखिरी बार { $date }
top-brand = Katna
