# Katna Mail, Hindi (हिन्दी): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = नया टास्क
tasks-all = सभी टास्क
tasks-today = आज
tasks-upcoming = आने वाले
tasks-starred = तारांकित
tasks-completed-view = पूरे हुए
tasks-new-list = नई सूची बनाएं
tasks-labels-heading = लेबल
tasks-on-this-computer = इस कंप्यूटर पर
tasks-my-tasks = मेरे टास्क
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = टास्क दिखाने के लिए फिर से साइन इन करें
tasks-account-signed-in = { $address } में फिर से साइन इन हो गया। आपके टास्क लाए जा रहे हैं…
tasks-account-sign-in-refused = { $provider } ने Katna को अंदर नहीं आने दिया। फिर से कोशिश करें, और अपने टास्क तक पहुँच की अनुमति दें।
tasks-account-refused = सर्वर ने पासवर्ड स्वीकार नहीं किया। Yahoo, iCloud, Zoho और दूसरों को ऐप पासवर्ड चाहिए।
tasks-account-change-password = पासवर्ड बदलें
tasks-account-change-password-tooltip = नया पासवर्ड टाइप करें; Katna सर्वर से इसकी जाँच करता है
tasks-account-not-enabled = Katna के लिए टास्क पहुँच अभी चालू नहीं है।
tasks-account-failed = टास्क सूचियां पढ़ी नहीं जा सकीं।
# $reason is the server's own words, in English.
tasks-account-error = टास्क सूचियां पढ़ी नहीं जा सकीं: { $reason }
tasks-account-none = कोई टास्क सूची नहीं मिली
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = कोई टास्क सूची नहीं मिली: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } टास्क सिर्फ़ उसी Katna को दिखाता है जो { $provider } से साइन इन हो।
tasks-account-sign-in-with = { $provider } से साइन इन करें
tasks-account-looking = टास्क सूचियां खोजी जा रही हैं…
tasks-account-try-again = फिर से कोशिश करें
tasks-account-try-again-tooltip = इस खाते के टास्क अभी फिर से जाँचें
tasks-account-fixing = काम जारी है…
tasks-list-name-placeholder = सूची का नाम

## Lists and tasks

tasks-loading = आपके टास्क पढ़े जा रहे हैं…
tasks-no-lists = आपकी टास्क सूचियां यहां दिखेंगी।
tasks-search = टास्क खोजें
tasks-search-none = आपकी खोज से कोई टास्क मेल नहीं खाता।
tasks-add = टास्क जोड़ें
tasks-title-placeholder = शीर्षक
tasks-add-step = सबटास्क जोड़ें
tasks-empty = अभी कोई टास्क नहीं है। ऊपर से एक जोड़ें।
tasks-starred-empty = किसी टास्क को तारांकित करें, वह यहां दिखेगा।
tasks-label-empty = इस लेबल वाला कोई खुला टास्क नहीं।
tasks-today-empty = आज के लिए कुछ नहीं है।
tasks-completed-empty = आप जो टास्क पूरे करेंगे, वे यहां दिखेंगे।
tasks-upcoming-add = { $day } के लिए टास्क जोड़ें
tasks-upcoming-overdue-day = { $weekday } { $day }
tasks-from-mail-quiet = मेल से
tasks-from-note-quiet = नोट से
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday }, { $day }
tasks-overdue = अतिदेय
tasks-completed = { $count ->
    [one] पूरे हुए ({ $count })
   *[other] पूरे हुए ({ $count })
}
tasks-list-options = सूची के विकल्प
tasks-sort-by = इसके अनुसार क्रम
tasks-sort-my-order = मेरा क्रम
tasks-sort-date = तारीख
tasks-sort-starred = हाल में तारांकित
tasks-sort-title = शीर्षक
tasks-rename-list = सूची का नाम बदलें
tasks-delete-list = सूची मिटाएं
tasks-mark-done = पूरा हुआ चिह्नित करें
tasks-mark-open = अधूरा चिह्नित करें
tasks-star = तारांकित करें
tasks-unstar = तारांकन हटाएं
tasks-edit-title = शीर्षक बदलें
tasks-details = ब्योरा
tasks-delete = मिटाएं
tasks-move-to = { $list } में ले जाएं
tasks-from-mail = मेल
tasks-open-mail = मेल खोलें
tasks-from-note = नोट
tasks-open-note = नोट खोलें
tasks-note-gone = वह नोट अब यहां नहीं है।
tasks-no-subject = (कोई विषय नहीं)

## Several tasks selected (Ctrl+click, Shift+click)

tasks-selected = { $count ->
    [one] { $count } चुना गया
   *[other] { $count } चुने गए
}
tasks-select-clear = चुनाव हटाएं
tasks-select-move = सूची में ले जाएं
tasks-select-date = तारीख सेट करें
tasks-next-week = अगले हफ़्ते

## The details dialog

tasks-notes-placeholder = ब्योरा जोड़ें
tasks-date = तारीख़
tasks-no-date = कोई तारीख़ नहीं
tasks-time-placeholder = समय जोड़ें
tasks-repeat = दोहराएं
tasks-repeat-never = दोहराव नहीं
tasks-repeat-daily = रोज़
tasks-repeat-weekly = हर हफ़्ते
tasks-repeat-monthly = हर महीने
tasks-repeat-yearly = हर साल
tasks-repeat-other = कस्टम
tasks-remind = मुझे याद दिलाएं
tasks-remind-off = याद न दिलाएं
tasks-remind-on-time = उसी समय
tasks-remind-morning = उसी दिन, { $time }
tasks-remind-hour-before = एक घंटा पहले
tasks-remind-day-before = एक दिन पहले
tasks-label-add = लेबल जोड़ें
tasks-label-task = टास्क को लेबल करें
tasks-files-attach = फ़ाइलें अटैच करें
tasks-files-pick = अटैच करें
tasks-file-open = खोलें
tasks-file-remove = फ़ाइल हटाएं
tasks-file-here = सिर्फ़ इस कंप्यूटर पर
tasks-cancel = रद्द करें
tasks-save = सेव करें
tasks-not-a-time = “{ $text }” समय नहीं है, जैसे { $example }।

## Due days

tasks-due-today = आज
tasks-due-tomorrow = कल
tasks-due-yesterday = बीता कल
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = टास्क पूरा हुआ
tasks-toast-next = हो गया। अगला { $date } को
tasks-toast-deleted = टास्क मिटाया गया
tasks-files-added = { $count ->
    [one] फ़ाइल अटैच की गई
   *[other] { $count } फ़ाइलें अटैच की गईं
}
tasks-file-removed = “{ $name }” हटाई गई
tasks-files-left-out = अटैच नहीं हुईं: { $names }। टास्क में { $limit } तक की फ़ाइलें जुड़ सकती हैं, फ़ोल्डर नहीं।
tasks-file-missing = वह फ़ाइल अब यहां नहीं है।
tasks-toast-added = { $count ->
    [one] टास्क में जोड़ा गया
   *[other] { $count } टास्क जोड़े गए
}
tasks-mail-gone = वह मेल अब यहां नहीं है।
tasks-toast-list-deleted = सूची मिटाई गई
tasks-toast-moved = { $list } में ले जाया गया
# A task dragged to another place in its own list.
tasks-toast-placed = टास्क ले जाया गया
tasks-toast-rescheduled = कार्य का समय बदला गया
tasks-toast-rescheduled-several = { $count ->
    [one] टास्क का समय बदला गया
   *[other] { $count } टास्क का समय बदला गया
}
tasks-toast-done-several = { $count ->
    [one] टास्क पूरा हुआ
   *[other] { $count } टास्क पूरे हुए
}
tasks-toast-open-several = { $count ->
    [one] टास्क अधूरा मार्क किया गया
   *[other] { $count } टास्क अधूरे मार्क किए गए
}
tasks-toast-starred = { $count ->
    [one] टास्क तारांकित किया गया
   *[other] { $count } टास्क तारांकित किए गए
}
tasks-toast-unstarred = { $count ->
    [one] तारा हटाया गया
   *[other] { $count } टास्क से तारे हटाए गए
}
tasks-toast-deleted-several = { $count ->
    [one] टास्क मिटाया गया
   *[other] { $count } टास्क मिटाए गए
}
