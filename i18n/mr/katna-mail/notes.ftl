# Katna Mail, Marathi (मराठी): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = नोट्स
notes-view-reminders = रिमाइंडर
notes-view-archive = संग्रहण
notes-view-trash = ट्रॅश
notes-edit-labels = लेबल संपादित करा
notes-search = नोट्स शोधा
notes-loading = तुमचे नोट्स उघडत आहे…

## Board

notes-take-a-note = नोंद घ्या…
notes-new-list = नवीन सूची
notes-new-note = नवीन नोट
notes-pinned = पिन केलेले
notes-others = इतर
notes-empty = तुम्ही जोडलेले नोट्स येथे दिसतील
notes-archive-empty = तुमचे संग्रहित नोट्स येथे दिसतील
notes-trash-empty = ट्रॅशमध्ये कोणतेही नोट्स नाहीत
notes-none-found = जुळणारे नोट्स नाहीत
notes-label-empty = या लेबलचे अद्याप कोणतेही नोट्स नाहीत
notes-reminders-empty = आगामी रिमाइंडर असलेले नोट्स येथे दिसतील
notes-trash-note = ट्रॅशमधील नोट्स 7 दिवसांनी हटवले जातात.
notes-empty-trash = ट्रॅश रिकामा करा
notes-ticked = { $count ->
    [one] + { $count } टिक केलेला आयटम
   *[other] + { $count } टिक केलेले आयटम
}
notes-select = नोट निवडा
notes-selected = { $count ->
    [one] { $count } निवडले
   *[other] { $count } निवडले
}
notes-select-clear = निवड साफ करा

## A note's buttons

notes-pin = नोट पिन करा
notes-unpin = नोट अनपिन करा
notes-archive = संग्रहित करा
notes-unarchive = संग्रहणातून काढा
notes-delete = नोट हटवा
notes-restore = पुनर्संचयित करा
notes-delete-forever = कायमचे हटवा
notes-color = पार्श्वभूमीचा रंग
notes-checkboxes = चेकबॉक्स दाखवा किंवा लपवा
notes-labels = लेबल
notes-close = बंद करा
notes-more = आणखी
notes-make-copy = प्रत तयार करा
notes-remind = मला आठवण करून द्या
notes-add-picture = चित्र जोडा
notes-history = आवृत्ती इतिहास
notes-ai = लिहिण्यास मदत करा
notes-send-as-mail = मेल म्हणून पाठवा
notes-save-markdown = Markdown म्हणून सेव्ह करा
notes-save-pdf = PDF म्हणून सेव्ह करा

## The open note

notes-title = शीर्षक
notes-edited = संपादित: { $date }
notes-on-this-computer = या कॉंप्युटरवर
notes-where = ही नोट कुठे ठेवली आहे
notes-untitled = शीर्षक नसलेली नोट

## Pictures

notes-picture-choose = चित्रे जोडा
notes-picture-remove = चित्र काढा
notes-picture-too-big = नोटमध्ये { $size } पर्यंतची चित्रे जाऊ शकतात
notes-picture-kind = ती फाइल Katna दाखवू शकेल असे चित्र नाही
notes-picture-unreadable = { $name } वाचता आली नाही: { $error }

## Reminders

notes-remind-me = मला आठवण करून द्या
notes-remind-off = रिमाइंडर काढा
notes-remind-in-the-past = अजून न गेलेली वेळ निवडा
notes-remind-today = आज, { $time }
notes-remind-tomorrow = उद्या, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = { $when } साठी रिमाइंडर सेट केले
notes-reminder-off = रिमाइंडर काढले

## Links between notes

notes-link-note = नोट लिंक करा
notes-link-new = नवीन नोट "{ $title }"
notes-linked-from = येथून लिंक केलेले
notes-link-gone = ती नोट आता येथे नाही
notes-new-note-gone = नवीन नोट नाहीशी झाली.

## Version history

notes-versions = आवृत्त्या
notes-version-now = आता
notes-version-here = तुम्ही, या कॉम्प्युटरवर
notes-version-yesterday = काल, { $time }
notes-version-changes = { $count ->
    [one] { $count } बदल
   *[other] { $count } बदल
}
notes-version-from = { $device } वरून
notes-version-elsewhere = दुसऱ्या डिव्हाइसवरून
notes-version-created = तयार केली
notes-version-restore = ही आवृत्ती पुनर्संचयित करा
notes-version-restored = आवृत्ती पुनर्संचयित केली
notes-history-none = अजून आधीच्या आवृत्त्या नाहीत

## AI help

notes-ai-tidy = मजकूर नीटनेटका करा
notes-ai-checklist = चेकलिस्टमध्ये रूपांतरित करा
notes-ai-summarise = सारांश द्या
notes-ai-empty = आधी काहीतरी लिहा
notes-ai-tidied = मजकूर नीटनेटका केला. Ctrl+Z ने तो परत येतो.
notes-ai-listed = चेकलिस्ट बनवली. Ctrl+Z ने तो परत येतो.
notes-ai-summarised = सारांश वर जोडला

## Labels

notes-label-note = नोटला लेबल लावा
notes-label-name = लेबलचे नाव टाका
notes-label-create = “{ $name }” तयार करा
notes-label-remove = लेबल काढा
notes-label-delete = लेबल हटवा
notes-labels-none = अद्याप कोणतेही लेबल नाही. नोटच्या लेबल बटणावरून एक जोडा.
notes-labels-done = झाले
notes-label-renamed = लेबलचे नाव बदलून “{ $name }” केले
notes-label-deleted = लेबल “{ $name }” हटवले

## A note about a mail

notes-mail = मेल
notes-open-mail = मेल उघडा
notes-open-note = नोट उघडा

## Meeting notes

notes-meeting-take = मीटिंग नोंद घ्या
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = उपस्थित: { $names }
notes-meeting-notes = नोंदी
notes-meeting-actions = कृती आयटम
notes-event = इव्हेंट
notes-open-event = इव्हेंट उघडा

## Formatting

notes-format = फॉरमॅटिंग
notes-format-heading-1 = शीर्षक 1
notes-format-heading-2 = शीर्षक 2
notes-format-normal = सामान्य मजकूर
notes-format-bold = ठळक
notes-format-italic = तिरपे
notes-format-underline = अधोरेखित
notes-format-quote = अवतरण
notes-format-code = कोड
notes-format-divider = विभाजक
notes-format-clear = फॉरमॅटिंग साफ करा

## Tasks

notes-make-task = कार्य बनवा

## Colors (tooltips)

notes-color-none = रंग नाही
notes-color-coral = प्रवाळी
notes-color-peach = पीच
notes-color-sand = वाळू
notes-color-mint = पुदिना
notes-color-sage = सेज
notes-color-fog = धुके
notes-color-storm = वादळ
notes-color-dusk = संधिप्रकाश
notes-color-blossom = फुलोरा
notes-color-clay = माती
notes-color-chalk = खडू

## Messages at the foot of the window

notes-archived = नोट संग्रहित केली
notes-unarchived = नोट संग्रहणातून काढली
notes-trashed = नोट ट्रॅशमध्ये हलवली
notes-restored = नोट पुनर्संचयित केली
notes-saved = नोट सेव्ह केली
notes-pinned-count = { $count ->
    [one] नोट पिन केली
   *[other] { $count } नोट्स पिन केल्या
}
notes-unpinned-count = { $count ->
    [one] नोट अनपिन केली
   *[other] { $count } नोट्स अनपिन केल्या
}
notes-colored-count = { $count ->
    [one] रंग बदलला
   *[other] { $count } नोट्सचा रंग बदलला
}
notes-archived-count = { $count ->
    [one] नोट संग्रहित केली
   *[other] { $count } नोट्स संग्रहित केल्या
}
notes-unarchived-count = { $count ->
    [one] नोट संग्रहणातून काढली
   *[other] { $count } नोट्स संग्रहणातून काढल्या
}
notes-trashed-count = { $count ->
    [one] नोट ट्रॅशमध्ये हलवली
   *[other] { $count } नोट्स ट्रॅशमध्ये हलवल्या
}
notes-restored-count = { $count ->
    [one] नोट पुनर्संचयित केली
   *[other] { $count } नोट्स पुनर्संचयित केल्या
}
notes-copied-count = { $count ->
    [one] प्रत तयार केली
   *[other] { $count } प्रती तयार केल्या
}
notes-empty-discarded = रिकामी नोट काढून टाकली
notes-mail-gone = तो मेल आता येथे नाही
notes-deleted-forever = { $count ->
    [one] नोट कायमची हटवली
   *[other] { $count } नोट्स कायमचे हटवले
}
