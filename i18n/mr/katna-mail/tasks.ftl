# Katna Mail, Marathi (मराठी): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = तयार करा
tasks-all = सर्व कार्ये
tasks-today = आज
tasks-starred = तारांकित
tasks-new-list = नवी सूची तयार करा
tasks-on-this-computer = या संगणकावर
tasks-my-tasks = माझी कार्ये
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = कार्ये दाखवण्यासाठी पुन्हा साइन इन करा
tasks-account-signed-in = { $address } मध्ये पुन्हा साइन इन केले. तुमची कार्ये आणत आहे…
tasks-account-sign-in-refused = { $provider } ने Katna ला आत येऊ दिले नाही. पुन्हा प्रयत्न करा, आणि तुमच्या कार्यांचा ॲक्सेस द्या.
tasks-account-refused = सर्व्हरने पासवर्ड स्वीकारला नाही. Yahoo, iCloud, Zoho आणि इतरांना ॲप पासवर्ड लागतो.
tasks-account-change-password = पासवर्ड बदला
tasks-account-change-password-tooltip = सेटिंग्ज > खाती उघडा
tasks-account-not-enabled = Katna साठी कार्यांचा ॲक्सेस अजून सुरू केलेला नाही.
tasks-account-failed = कार्य सूची वाचता आल्या नाहीत.
# $reason is the server's own words, in English.
tasks-account-error = कार्य सूची वाचता आल्या नाहीत: { $reason }
tasks-account-none = कोणतीही कार्य सूची सापडली नाही
tasks-account-looking = कार्य सूची शोधत आहे…
tasks-account-try-again = पुन्हा प्रयत्न करा
tasks-account-try-again-tooltip = या खात्याची कार्ये आता पुन्हा तपासा
tasks-account-fixing = काम सुरू आहे…
tasks-list-name-placeholder = सूचीचे नाव

## Lists and tasks

tasks-loading = तुमची कार्ये वाचत आहे…
tasks-no-lists = तुमच्या कार्यसूची येथे दिसतील.
tasks-search = कार्ये शोधा
tasks-search-none = तुमच्या शोधाशी कोणतेही कार्य जुळले नाही.
tasks-add = कार्य जोडा
tasks-title-placeholder = शीर्षक
tasks-add-step = उपकार्य जोडा
tasks-empty = अजून कोणतेही कार्य नाही. वर एक जोडा.
tasks-starred-empty = येथे पाहण्यासाठी एखाद्या कार्याला तारांकित करा.
tasks-today-empty = आजसाठी काहीही नाही.
tasks-today-date = { $weekday }, { $day }
tasks-overdue = मुदत उलटून गेलेली
tasks-completed = { $count ->
    [one] पूर्ण झालेली ({ $count })
   *[other] पूर्ण झालेली ({ $count })
}
tasks-list-options = सूची पर्याय
tasks-rename-list = सूचीचे नाव बदला
tasks-delete-list = सूची हटवा
tasks-mark-done = पूर्ण म्हणून चिन्हांकित करा
tasks-mark-open = अपूर्ण म्हणून चिन्हांकित करा
tasks-star = तारांकित करा
tasks-unstar = तारांकन काढा
tasks-edit-title = शीर्षक संपादित करा
tasks-details = तपशील
tasks-delete = हटवा
tasks-move-to = { $list } मध्ये हलवा
tasks-from-mail = मेल
tasks-open-mail = मेल उघडा
tasks-from-note = नोट
tasks-open-note = नोट उघडा
tasks-note-gone = ती नोट आता येथे नाही.
tasks-no-subject = (विषय नाही)

## The details dialog

tasks-notes-placeholder = तपशील जोडा
tasks-date = तारीख
tasks-no-date = तारीख नाही
tasks-time-placeholder = वेळ जोडा
tasks-repeat = पुनरावृत्ती
tasks-repeat-never = पुनरावृत्ती नाही
tasks-repeat-daily = दररोज
tasks-repeat-weekly = दर आठवड्याला
tasks-repeat-monthly = दर महिन्याला
tasks-repeat-yearly = दरवर्षी
tasks-repeat-other = कस्टम
tasks-remind = मला आठवण करून द्या
tasks-remind-off = आठवण करून देऊ नका
tasks-remind-on-time = त्याच वेळी
tasks-remind-morning = त्या दिवशी, { $time }
tasks-remind-hour-before = एक तास आधी
tasks-remind-day-before = एक दिवस आधी
tasks-cancel = रद्द करा
tasks-save = सेव्ह करा
tasks-not-a-time = “{ $text }” ही वेळ नाही, उदा. { $example }.

## Due days

tasks-due-today = आज
tasks-due-tomorrow = उद्या
tasks-due-yesterday = काल
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = कार्य पूर्ण झाले
tasks-toast-next = झाले. पुढचे { $date } रोजी
tasks-toast-deleted = कार्य हटवले
tasks-toast-added = { $count ->
    [one] कार्यांमध्ये जोडले
   *[other] { $count } कार्ये जोडली
}
tasks-mail-gone = तो मेल आता येथे नाही.
tasks-toast-list-deleted = सूची हटवली
tasks-toast-moved = { $list } मध्ये हलवले
tasks-toast-rescheduled = कार्याची वेळ बदलली
