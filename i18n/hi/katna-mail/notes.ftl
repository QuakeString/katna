# Katna Mail, Hindi (हिन्दी): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = नोट
notes-view-reminders = रिमाइंडर
notes-view-archive = संग्रह
notes-view-trash = ट्रैश
notes-edit-labels = लेबल में बदलाव करें
notes-search = नोट खोजें
notes-loading = आपके नोट खोले जा रहे हैं…

## Board

notes-take-a-note = नोट लें…
notes-new-list = नई सूची
notes-new-note = नया नोट
notes-pinned = पिन किए गए
notes-others = अन्य
notes-empty = आपके जोड़े गए नोट यहां दिखाई देंगे
notes-archive-empty = आपके संग्रहित नोट यहां दिखाई देंगे
notes-trash-empty = ट्रैश में कोई नोट नहीं है
notes-none-found = कोई मिलता-जुलता नोट नहीं मिला
notes-label-empty = इस लेबल वाला कोई नोट अभी नहीं है
notes-reminders-empty = आने वाले रिमाइंडर वाले नोट यहां दिखेंगे
notes-trash-note = ट्रैश में मौजूद नोट 7 दिन बाद मिटा दिए जाते हैं।
notes-empty-trash = ट्रैश खाली करें
notes-ticked = { $count ->
    [one] + { $count } चिह्नित आइटम
   *[other] + { $count } चिह्नित आइटम
}
notes-select = नोट चुनें
notes-selected = { $count ->
    [one] { $count } चुना गया
   *[other] { $count } चुने गए
}
notes-select-clear = चुनाव हटाएं

## A note's buttons

notes-pin = नोट पिन करें
notes-unpin = नोट अनपिन करें
notes-archive = संग्रहित करें
notes-unarchive = संग्रह से निकालें
notes-delete = नोट मिटाएं
notes-restore = वापस लाएं
notes-delete-forever = हमेशा के लिए मिटाएं
notes-color = बैकग्राउंड का रंग
notes-checkboxes = चेकबॉक्स दिखाएं या छिपाएं
notes-labels = लेबल
notes-close = बंद करें
notes-more = ज़्यादा
notes-make-copy = कॉपी बनाएं
notes-remind = मुझे याद दिलाएं
notes-add-picture = तस्वीर जोड़ें
notes-history = संस्करण इतिहास
notes-ai = लिखने में मदद करें
notes-send-as-mail = मेल के रूप में भेजें
notes-save-markdown = Markdown के रूप में सेव करें
notes-save-pdf = PDF के रूप में सेव करें

## The open note

notes-title = शीर्षक
notes-edited = संपादित: { $date }
notes-on-this-computer = इस कंप्यूटर पर
notes-where = यह नोट कहां रखा है
notes-untitled = बिना शीर्षक का नोट

## Pictures

notes-picture-choose = तस्वीरें जोड़ें
notes-picture-remove = तस्वीर हटाएं
notes-picture-too-big = नोट में ज़्यादा से ज़्यादा { $size } तक की तस्वीरें डाली जा सकती हैं
notes-picture-kind = यह फ़ाइल ऐसी तस्वीर नहीं है जिसे Katna दिखा सके
notes-picture-unreadable = { $name } पढ़ी नहीं जा सकी: { $error }

## Reminders

notes-remind-me = मुझे याद दिलाएं
notes-remind-off = रिमाइंडर हटाएं
notes-remind-in-the-past = ऐसा समय चुनें जो अभी बीता न हो
notes-remind-today = आज, { $time }
notes-remind-tomorrow = कल, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = { $when } के लिए रिमाइंडर सेट किया गया
notes-reminder-off = रिमाइंडर हटाया गया

## Links between notes

notes-link-note = नोट लिंक करें
notes-link-new = नया नोट “{ $title }”
notes-linked-from = इनसे लिंक किया गया
notes-link-gone = वह नोट अब यहां नहीं है
notes-new-note-gone = नया नोट अब नहीं है।

## Version history

notes-versions = संस्करण
notes-version-now = अभी
notes-version-here = आप, इस कंप्यूटर पर
notes-version-yesterday = कल, { $time }
notes-version-changes = { $count ->
    [one] { $count } बदलाव
   *[other] { $count } बदलाव
}
notes-version-from = { $device } से
notes-version-elsewhere = किसी दूसरे डिवाइस से
notes-version-created = बनाया गया
notes-version-restore = यह संस्करण वापस लाएं
notes-version-restored = संस्करण वापस लाया गया
notes-history-none = अभी कोई पुराना संस्करण नहीं है

## AI help

notes-ai-tidy = टेक्स्ट को व्यवस्थित करें
notes-ai-checklist = इसे चेकलिस्ट में बदलें
notes-ai-summarise = सारांश बनाएं
notes-ai-empty = पहले कुछ लिखें
notes-ai-tidied = टेक्स्ट व्यवस्थित किया गया। Ctrl+Z से पहले जैसा हो जाएगा।
notes-ai-listed = चेकलिस्ट बनाई गई। Ctrl+Z से पहले जैसा हो जाएगा।
notes-ai-summarised = सबसे ऊपर सारांश जोड़ा गया

## Labels

notes-label-note = नोट को लेबल करें
notes-label-name = लेबल का नाम डालें
notes-label-create = “{ $name }” बनाएं
notes-label-remove = लेबल हटाएं
notes-label-delete = लेबल मिटाएं
notes-labels-none = अभी कोई लेबल नहीं है। किसी नोट के लेबल बटन से जोड़ें।
notes-labels-done = हो गया
notes-label-renamed = लेबल का नाम बदलकर “{ $name }” कर दिया गया
notes-label-deleted = लेबल “{ $name }” मिटा दिया गया

## A note about a mail

notes-mail = मेल
notes-open-mail = मेल खोलें
notes-open-note = नोट खोलें

## Meeting notes

notes-meeting-take = मीटिंग नोट लें
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = उपस्थित लोग: { $names }
notes-meeting-notes = नोट
notes-meeting-actions = कार्य आइटम
notes-event = इवेंट
notes-open-event = इवेंट खोलें

## Formatting

notes-format = फ़ॉर्मैटिंग
notes-format-heading-1 = शीर्षक 1
notes-format-heading-2 = शीर्षक 2
notes-format-normal = सामान्य टेक्स्ट
notes-format-bold = बोल्ड
notes-format-italic = इटैलिक
notes-format-underline = अंडरलाइन
notes-format-quote = उद्धरण
notes-format-code = कोड
notes-format-divider = विभाजक
notes-format-clear = फ़ॉर्मैटिंग हटाएं

## Tasks

notes-make-task = टास्क बनाएं

## Colors (tooltips)

notes-color-none = कोई रंग नहीं
notes-color-coral = कोरल
notes-color-peach = पीच
notes-color-sand = रेत
notes-color-mint = पुदीना
notes-color-sage = सेज
notes-color-fog = कोहरा
notes-color-storm = तूफ़ान
notes-color-dusk = शाम
notes-color-blossom = फूल
notes-color-clay = मिट्टी
notes-color-chalk = चॉक

## Messages at the foot of the window

notes-archived = नोट संग्रहित किया गया
notes-unarchived = नोट संग्रह से निकाला गया
notes-trashed = नोट ट्रैश में भेजा गया
notes-restored = नोट वापस लाया गया
notes-saved = नोट सेव किया गया
notes-pinned-count = { $count ->
    [one] नोट पिन किया गया
   *[other] { $count } नोट पिन किए गए
}
notes-unpinned-count = { $count ->
    [one] नोट से पिन हटाया गया
   *[other] { $count } नोट से पिन हटाया गया
}
notes-colored-count = { $count ->
    [one] रंग बदला गया
   *[other] { $count } नोट का रंग बदला गया
}
notes-archived-count = { $count ->
    [one] नोट संग्रहित किया गया
   *[other] { $count } नोट संग्रहित किए गए
}
notes-unarchived-count = { $count ->
    [one] नोट संग्रह से वापस लाया गया
   *[other] { $count } नोट संग्रह से वापस लाए गए
}
notes-trashed-count = { $count ->
    [one] नोट ट्रैश में ले जाया गया
   *[other] { $count } नोट ट्रैश में ले जाए गए
}
notes-restored-count = { $count ->
    [one] नोट वापस लाया गया
   *[other] { $count } नोट वापस लाए गए
}
notes-copied-count = { $count ->
    [one] कॉपी बनाई गई
   *[other] { $count } कॉपी बनाई गईं
}
notes-empty-discarded = खाली नोट हटा दिया गया
notes-mail-gone = वह मेल अब यहां नहीं है
notes-deleted-forever = { $count ->
    [one] नोट हमेशा के लिए मिटा दिया गया
   *[other] { $count } नोट हमेशा के लिए मिटा दिए गए
}
