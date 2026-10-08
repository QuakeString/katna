# Katna Mail, Nepali (नेपाली): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = नयाँ कार्य
tasks-all = सबै कार्य
tasks-today = आज
tasks-upcoming = आगामी
tasks-starred = तारा लगाइएको
tasks-completed-view = सम्पन्न
tasks-new-list = नयाँ सूची बनाउनुहोस्
tasks-labels-heading = लेबलहरू
tasks-on-this-computer = यो कम्प्युटरमा
tasks-my-tasks = मेरा कार्य
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = कार्यहरू देखाउन फेरि साइन इन गर्नुहोस्
tasks-account-signed-in = { $address } मा फेरि साइन इन भयो। तपाईंका कार्यहरू ल्याउँदै…
tasks-account-sign-in-refused = { $provider } ले Katna लाई भित्र आउन दिएन। फेरि प्रयास गर्नुहोस्, र आफ्ना कार्यहरूमा पहुँच दिनुहोस्।
tasks-account-refused = सर्भरले पासवर्ड स्वीकार गरेन। Yahoo, iCloud, Zoho र अरूलाई एप पासवर्ड चाहिन्छ।
tasks-account-change-password = पासवर्ड बदल्नुहोस्
tasks-account-change-password-tooltip = नयाँ पासवर्ड टाइप गर्नुहोस्; Katna ले सर्भरसँग जाँच गर्छ
tasks-account-not-enabled = Katna का लागि कार्य पहुँच अझै सक्रिय गरिएको छैन।
tasks-account-failed = कार्य सूचीहरू पढ्न सकिएन।
# $reason is the server's own words, in English.
tasks-account-error = कार्य सूचीहरू पढ्न सकिएन: { $reason }
tasks-account-none = कुनै कार्य सूची भेटिएन
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = कुनै कार्य सूची भेटिएन: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } ले { $provider } बाट साइन इन गरिएको Katna लाई मात्र कार्यहरू देखाउँछ।
tasks-account-sign-in-with = { $provider } बाट साइन इन गर्नुहोस्
tasks-account-looking = कार्य सूचीहरू खोज्दै…
tasks-account-try-again = फेरि प्रयास गर्नुहोस्
tasks-account-try-again-tooltip = यो खाताका कार्यहरू अहिले फेरि जाँच गर्नुहोस्
tasks-account-fixing = काम हुँदैछ…
tasks-list-name-placeholder = सूचीको नाम

## Lists and tasks

tasks-loading = तपाईंका कार्य पढिँदै छन्…
tasks-no-lists = तपाईंका कार्य सूचीहरू यहाँ देखिन्छन्।
tasks-search = कार्यहरू खोज्नुहोस्
tasks-search-none = तपाईंको खोजसँग मिल्ने कुनै कार्य भेटिएन।
tasks-add = कार्य थप्नुहोस्
tasks-title-placeholder = शीर्षक
tasks-add-step = उप-कार्य थप्नुहोस्
tasks-empty = अहिलेसम्म कुनै कार्य छैन। माथि एउटा थप्नुहोस्।
tasks-starred-empty = यहाँ हेर्न कुनै कार्यमा तारा लगाउनुहोस्।
tasks-label-empty = यो लेबल भएको कुनै खुला कार्य छैन।
tasks-today-empty = आजका लागि केही छैन।
tasks-completed-empty = तपाईंले सम्पन्न गरेका कार्यहरू यहाँ देखिन्छन्।
tasks-upcoming-add = { $day } का लागि कार्य थप्नुहोस्
tasks-upcoming-overdue-day = { $weekday } { $day }
tasks-from-mail-quiet = मेलबाट
tasks-from-note-quiet = टिपोटबाट
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday }, { $day }
tasks-overdue = म्याद नाघेको
tasks-completed = { $count ->
    [one] सम्पन्न ({ $count })
   *[other] सम्पन्न ({ $count })
}
tasks-list-options = सूचीका विकल्पहरू
tasks-sort-by = यसअनुसार क्रमबद्ध गर्नुहोस्
tasks-sort-my-order = मेरो क्रम
tasks-sort-date = मिति
tasks-sort-starred = भर्खर तारा लगाइएका
tasks-sort-title = शीर्षक
tasks-rename-list = सूचीको नाम बदल्नुहोस्
tasks-delete-list = सूची मेटाउनुहोस्
tasks-mark-done = सम्पन्न भनी चिन्ह लगाउनुहोस्
tasks-mark-open = असम्पन्न भनी चिन्ह लगाउनुहोस्
tasks-star = तारा लगाउनुहोस्
tasks-unstar = तारा हटाउनुहोस्
tasks-edit-title = शीर्षक सम्पादन गर्नुहोस्
tasks-details = विवरण
tasks-delete = मेटाउनुहोस्
tasks-move-to = { $list } मा सार्नुहोस्
tasks-from-mail = मेल
tasks-open-mail = मेल खोल्नुहोस्
tasks-from-note = टिपोट
tasks-open-note = टिपोट खोल्नुहोस्
tasks-note-gone = त्यो टिपोट अब यहाँ छैन।
tasks-no-subject = (विषय छैन)

## Several tasks selected (Ctrl+click, Shift+click)

tasks-selected = { $count ->
    [one] { $count } चयन गरिएको
   *[other] { $count } चयन गरिएका
}
tasks-select-clear = चयन हटाउनुहोस्
tasks-select-move = सूचीमा सार्नुहोस्
tasks-select-date = मिति राख्नुहोस्
tasks-next-week = अर्को हप्ता

## The details dialog

tasks-notes-placeholder = विवरण थप्नुहोस्
tasks-date = मिति
tasks-no-date = मिति छैन
tasks-time-placeholder = समय थप्नुहोस्
tasks-repeat = दोहोर्याउनुहोस्
tasks-repeat-never = दोहोरिँदैन
tasks-repeat-daily = दैनिक
tasks-repeat-weekly = साप्ताहिक
tasks-repeat-monthly = मासिक
tasks-repeat-yearly = वार्षिक
tasks-repeat-other = कस्टम
tasks-remind = मलाई सम्झाउनुहोस्
tasks-remind-off = नसम्झाउनुहोस्
tasks-remind-on-time = त्यही समयमा
tasks-remind-morning = सोही दिन, { $time }
tasks-remind-hour-before = एक घण्टा अघि
tasks-remind-day-before = एक दिन अघि
tasks-label-add = लेबल थप्नुहोस्
tasks-label-task = कार्यमा लेबल लगाउनुहोस्
tasks-files-attach = फाइलहरू संलग्न गर्नुहोस्
tasks-files-pick = संलग्न गर्नुहोस्
tasks-file-open = खोल्नुहोस्
tasks-file-remove = फाइल हटाउनुहोस्
tasks-file-here = यो कम्प्युटरमा मात्र
tasks-cancel = रद्द गर्नुहोस्
tasks-save = सेभ गर्नुहोस्
tasks-not-a-time = “{ $text }” समय होइन, जस्तै { $example }।

## Due days

tasks-due-today = आज
tasks-due-tomorrow = भोलि
tasks-due-yesterday = हिजो
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = कार्य सम्पन्न भयो
tasks-toast-next = भयो। अर्को { $date } मा
tasks-toast-deleted = कार्य मेटियो
tasks-files-added = { $count ->
    [one] फाइल संलग्न गरियो
   *[other] { $count } फाइलहरू संलग्न गरिए
}
tasks-file-removed = “{ $name }” हटाइयो
tasks-files-left-out = संलग्न गरिएन: { $names }। कार्यमा { $limit } सम्मका फाइलहरू राख्न सकिन्छ, फोल्डरहरू होइन।
tasks-file-missing = त्यो फाइल अब यहाँ छैन।
tasks-toast-added = { $count ->
    [one] कार्यमा थपियो
   *[other] { $count } कार्य थपिए
}
tasks-mail-gone = त्यो मेल अब यहाँ छैन।
tasks-toast-list-deleted = सूची मेटियो
tasks-toast-moved = { $list } मा सारियो
# A task dragged to another place in its own list.
tasks-toast-placed = कार्य सारियो
tasks-toast-rescheduled = कार्यको समय परिवर्तन गरियो
tasks-toast-rescheduled-several = { $count ->
    [one] कार्यको समय परिवर्तन गरियो
   *[other] { $count } कार्यहरूको समय परिवर्तन गरियो
}
tasks-toast-done-several = { $count ->
    [one] कार्य सम्पन्न भयो
   *[other] { $count } कार्यहरू सम्पन्न भए
}
tasks-toast-open-several = { $count ->
    [one] कार्यलाई असम्पन्न भनी चिन्ह लगाइयो
   *[other] { $count } कार्यहरूलाई असम्पन्न भनी चिन्ह लगाइयो
}
tasks-toast-starred = { $count ->
    [one] कार्यमा तारा लगाइयो
   *[other] { $count } कार्यहरूमा तारा लगाइयो
}
tasks-toast-unstarred = { $count ->
    [one] तारा हटाइयो
   *[other] { $count } कार्यहरूबाट तारा हटाइयो
}
tasks-toast-deleted-several = { $count ->
    [one] कार्य मेटियो
   *[other] { $count } कार्यहरू मेटिए
}
