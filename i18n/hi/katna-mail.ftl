# Katna Mail, Hindi (हिन्दी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = भाषा: { $language }
language-tooltip-system = भाषा: { $language }, सिस्टम के अनुसार
language-search = भाषा खोजें
language-system-default = सिस्टम डिफ़ॉल्ट
language-system-now = अभी { $language }
language-no-match = “{ $query }” से मेल खाने वाली कोई भाषा नहीं है
language-machine = मशीन से अनुवादित। इसे बेहतर बनाने में मदद करें
language-setting = भाषा
language-setting-detail = मेन्यू, बटन और मैसेज की भाषा, और तारीख व संख्याओं का फ़ॉर्मैट। सिस्टम डिफ़ॉल्ट, डेस्कटॉप की सेटिंग के हिसाब से चलता है।

## Dates and sizes

ago-just-now = अभी-अभी
ago-minutes = { $count ->
    [one] { $count } मिनट पहले
   *[other] { $count } मिनट पहले
}
ago-hours = { $count ->
    [one] { $count } घंटा पहले
   *[other] { $count } घंटे पहले
}
ago-days = { $count ->
    [one] { $count } दिन पहले
   *[other] { $count } दिन पहले
}
size-bytes = { $count ->
    [one] { $count } बाइट
   *[other] { $count } बाइट
}
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = फ़ोल्डर छिपाएं
folders-show = फ़ोल्डर दिखाएं
compose = लिखें
search = खोजें
search-mail = मेल खोजें
search-settings = सेटिंग खोजें
search-clear = खोज मिटाएं
search-options-show = खोज के विकल्प दिखाएं
settings = सेटिंग
account-add = खाता जोड़ें

## App rail (and the bottom bar on a phone)

rail-mail = मेल
rail-calendar = कैलेंडर
rail-contacts = संपर्क
rail-tasks = टास्क
rail-notes = नोट
rail-feeds = फ़ीड

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = जल्द आ रहा है
app-calendar-promise = आपके CalDAV कैलेंडर, मेल से आए मीटिंग के न्योते और रिमाइंडर, आपके इनबॉक्स के बगल में।
app-tasks-promise = CalDAV के साथ सिंक होने वाली टू-डू सूचियां, और मेल से बनाए गए टास्क।
app-notes-promise = झटपट नोट, और बाद के लिए किसी मेल या बातचीत पर नोट।
app-feeds-promise = अपने मेल के साथ-साथ RSS और Atom फ़ीड पढ़ें।

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

## Navigation (the folders pane)

nav-labels = लेबल
nav-folders = फ़ोल्डर
nav-label-new = नया लेबल बनाएं
nav-folder-new = नया फ़ोल्डर बनाएं
nav-account-unnamed = खाता { $number }
nav-tab-new = { $count ->
    [one] { $count } नया
   *[other] { $count } नए
}

## Special folders (the user's own folders keep their names)

folder-inbox = इनबॉक्स
folder-starred = तारांकित
folder-drafts = ड्राफ़्ट
folder-sent = भेजे गए
folder-archive = संग्रह
folder-spam = स्पैम
folder-trash = ट्रैश
folder-all-mail = सभी मेल
folder-scheduled = शेड्यूल किए गए

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = नया लेबल
label-folder-new-title = नया फ़ोल्डर
label-prompt = कृपया नए लेबल का नाम डालें:
label-folder-prompt = कृपया नए फ़ोल्डर का नाम डालें:
label-name-hint = लेबल का नाम
label-folder-name-hint = फ़ोल्डर का नाम
label-nest = लेबल को इसके अंदर रखें:
label-folder-nest = फ़ोल्डर को इसके अंदर रखें:
label-cancel = रद्द करें
label-create = बनाएं
label-creating = बनाया जा रहा है…
label-created = लेबल “{ $name }” बनाया गया।
label-folder-created = फ़ोल्डर “{ $name }” बनाया गया।

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = मुख्य
tab-promotions = प्रमोशन
tab-social = सामाजिक
tab-updates = अपडेट
tab-forums = फ़ोरम
tab-focused = फ़ोकस्ड
tab-other = अन्य
tab-inbox = इनबॉक्स
tab-newsletters = न्यूज़लेटर
tab-notifications = सूचनाएं
tab-new = { $count } नए
tab-provider-other = Katna ने छांटा

## Mail list: toolbar

list-select = चुनें
list-refresh = रीफ़्रेश करें
list-more = ज़्यादा
list-mark-read = पढ़ा गया के रूप में मार्क करें
list-mark-unread = नहीं पढ़ा गया के रूप में मार्क करें
list-move-to = इसमें ले जाएं
list-archive = संग्रह करें
list-spam = स्पैम की शिकायत करें
list-delete = मिटाएं
list-newer = नए
list-older = पुराने
list-range = { $total } में से { $first }–{ $last }
list-range-about = लगभग { $total } में से { $first }–{ $last }
list-results = “{ $query }” के नतीजे
list-results-corrected = “{ $query }” के नतीजे दिखाए जा रहे हैं
list-search-instead = इसके बजाय “{ $query }” खोजें
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = सभी
list-pick-none = कोई नहीं
list-pick-read = पढ़े गए
list-pick-unread = नहीं पढ़े गए
list-pick-starred = तारांकित
list-pick-unstarred = तारांकित नहीं

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } बातचीत चुनी गई है।
       *[other] सभी { $count } बातचीत चुनी गई हैं।
    }
   *[message] { $count ->
        [one] { $count } मैसेज चुना गया है।
       *[other] सभी { $count } मैसेज चुने गए हैं।
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } में { $count } बातचीत चुनी गई है।
       *[other] { $folder } में सभी { $count } बातचीत चुनी गई हैं।
    }
   *[message] { $count ->
        [one] { $folder } में { $count } मैसेज चुना गया है।
       *[other] { $folder } में सभी { $count } मैसेज चुने गए हैं।
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] स्क्रीन पर { $count } बातचीत चुनी गई है।
       *[other] स्क्रीन पर सभी { $count } बातचीत चुनी गई हैं।
    }
   *[message] { $count ->
        [one] स्क्रीन पर { $count } मैसेज चुना गया है।
       *[other] स्क्रीन पर सभी { $count } मैसेज चुने गए हैं।
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } बातचीत चुनें
       *[other] सभी { $count } बातचीत चुनें
    }
   *[message] { $count ->
        [one] { $count } मैसेज चुनें
       *[other] सभी { $count } मैसेज चुनें
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } में { $count } बातचीत चुनें
       *[other] { $folder } में सभी { $count } बातचीत चुनें
    }
   *[message] { $count ->
        [one] { $folder } में { $count } मैसेज चुनें
       *[other] { $folder } में सभी { $count } मैसेज चुनें
    }
}
list-clear-selection = चुनाव हटाएं

## Mail list: empty states

list-empty-search = आपकी खोज से कोई मैसेज मेल नहीं खाता।
list-empty-tab = { $tab } में कोई मेल नहीं है।
list-empty-tab-unknown = इस टैब में कोई मेल नहीं है।
list-empty-folder = { $folder } में कोई मैसेज नहीं है।
list-empty-folder-unknown = इस फ़ोल्डर में कोई मैसेज नहीं है।
list-first-sync = आपका मेल लाया जा रहा है…
list-first-sync-detail = मेल आते ही यहां दिखेगा।

## Mail list: lines

row-removed = यह मैसेज हटा दिया गया।
row-starred = तारांकित
row-not-starred = तारांकित नहीं
row-important = ज़रूरी। ज़रूरी नहीं के रूप में मार्क करने के लिए क्लिक करें।
row-mark-important = ज़रूरी के रूप में मार्क करें
row-pinned = सबसे ऊपर पिन किया गया
row-pin = सबसे ऊपर पिन करें
row-unpin = अनपिन करें

## Mail list: More menu and right-click menu

menu-reply = जवाब दें
menu-reply-all = सभी को जवाब दें
menu-forward = फ़ॉरवर्ड करें
menu-archive = संग्रह करें
menu-delete = मिटाएं
menu-spam = स्पैम की शिकायत करें
menu-mark-read = पढ़ा गया के रूप में मार्क करें
menu-mark-unread = नहीं पढ़ा गया के रूप में मार्क करें
menu-mark-all-read = सभी को पढ़ा गया के रूप में मार्क करें
menu-star = तारांकित करें
menu-unstar = तारांकन हटाएं
menu-important = ज़रूरी के रूप में मार्क करें
menu-not-important = ज़रूरी नहीं के रूप में मार्क करें
menu-pin = सबसे ऊपर पिन करें
menu-unpin = अनपिन करें
menu-print-all = सभी प्रिंट करें
menu-new-window = नई विंडो में खोलें
menu-move-to = इसमें ले जाएं
menu-move-to-heading = इसमें ले जाएं:
menu-find-from = { $name } से आए ईमेल ढूंढें

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] बातचीत संग्रहित की गई।
       *[other] { $count } बातचीत संग्रहित की गईं।
    }
   *[message] { $count ->
        [one] मैसेज संग्रहित किया गया।
       *[other] { $count } मैसेज संग्रहित किए गए।
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] बातचीत ट्रैश में ले जाई गई।
       *[other] { $count } बातचीत ट्रैश में ले जाई गईं।
    }
   *[message] { $count ->
        [one] मैसेज ट्रैश में ले जाया गया।
       *[other] { $count } मैसेज ट्रैश में ले जाए गए।
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] बातचीत ले जाई गई।
       *[other] { $count } बातचीत ले जाई गईं।
    }
   *[message] { $count ->
        [one] मैसेज ले जाया गया।
       *[other] { $count } मैसेज ले जाए गए।
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] बातचीत तारांकित की गई।
       *[other] { $count } बातचीत तारांकित की गईं।
    }
   *[message] { $count ->
        [one] मैसेज तारांकित किया गया।
       *[other] { $count } मैसेज तारांकित किए गए।
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] बातचीत से तारांकन हटाया गया।
       *[other] { $count } बातचीत से तारांकन हटाया गया।
    }
   *[message] { $count ->
        [one] मैसेज से तारांकन हटाया गया।
       *[other] { $count } मैसेज से तारांकन हटाया गया।
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] बातचीत ज़रूरी के रूप में मार्क की गई।
       *[other] { $count } बातचीत ज़रूरी के रूप में मार्क की गईं।
    }
   *[message] { $count ->
        [one] मैसेज ज़रूरी के रूप में मार्क किया गया।
       *[other] { $count } मैसेज ज़रूरी के रूप में मार्क किए गए।
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] बातचीत ज़रूरी नहीं के रूप में मार्क की गई।
       *[other] { $count } बातचीत ज़रूरी नहीं के रूप में मार्क की गईं।
    }
   *[message] { $count ->
        [one] मैसेज ज़रूरी नहीं के रूप में मार्क किया गया।
       *[other] { $count } मैसेज ज़रूरी नहीं के रूप में मार्क किए गए।
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] बातचीत सबसे ऊपर पिन की गई।
       *[other] { $count } बातचीत सबसे ऊपर पिन की गईं।
    }
   *[message] { $count ->
        [one] मैसेज सबसे ऊपर पिन किया गया।
       *[other] { $count } मैसेज सबसे ऊपर पिन किए गए।
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] बातचीत अनपिन की गई।
       *[other] { $count } बातचीत अनपिन की गईं।
    }
   *[message] { $count ->
        [one] मैसेज अनपिन किया गया।
       *[other] { $count } मैसेज अनपिन किए गए।
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] बातचीत की स्पैम के रूप में शिकायत की गई।
       *[other] { $count } बातचीत की स्पैम के रूप में शिकायत की गई।
    }
   *[message] { $count ->
        [one] मैसेज की स्पैम के रूप में शिकायत की गई।
       *[other] { $count } मैसेज की स्पैम के रूप में शिकायत की गई।
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] बातचीत हमेशा के लिए मिटा दी गई।
       *[other] { $count } बातचीत हमेशा के लिए मिटा दी गईं।
    }
   *[message] { $count ->
        [one] मैसेज हमेशा के लिए मिटा दिया गया।
       *[other] { $count } मैसेज हमेशा के लिए मिटा दिए गए।
    }
}
toast-undone = कार्रवाई पहले जैसी कर दी गई।
toast-undo = पहले जैसा करें
toast-no-spam-folder = इस खाते में कोई स्पैम फ़ोल्डर नहीं है।

## Reading pane: toolbar

reader-close = बंद करें
reader-back = वापस जाएं
reader-mark-unread = नहीं पढ़ा गया के रूप में मार्क करें
reader-move-to = इसमें ले जाएं
reader-more = ज़्यादा
reader-print-all = सभी प्रिंट करें
reader-new-window = नई विंडो में
reader-position = { $total } में से { $position }
reader-newer = नई
reader-older = पुरानी

## Reading pane: the conversation

reader-removed = यह बातचीत हटा दी गई।
reader-no-subject = (कोई विषय नहीं)
reader-collapse-all = सभी को छोटा करें
reader-expand-all = सभी को बड़ा करें
reader-unknown-sender = (अज्ञात भेजने वाला)
reader-date-ago = { $date } ({ $ago })
reader-me = मैं
reader-to = पाने वाले: { $names }
reader-starred = तारांकित
reader-not-starred = तारांकित नहीं
reader-too-long = मैसेज इतना लंबा है कि पूरा नहीं दिखाया जा सकता।
reader-encrypted-images = एन्क्रिप्ट किए गए मेल में वेब से इमेज कभी लोड नहीं की जातीं।
reader-window-failed = नई विंडो नहीं खोली जा सकी।

## Reading pane: message details (opened from "to me")

reader-details-from = भेजने वाला:
reader-details-to = पाने वाला:
reader-details-cc = cc:
reader-details-date = तारीख:
reader-details-subject = विषय:

## Reading pane: downloading a message

reader-downloading = यह मैसेज सर्वर से डाउनलोड किया जा रहा है…
reader-download-failed = यह मैसेज डाउनलोड नहीं किया जा सका।
reader-try-again = फिर से कोशिश करें

## Reply row

reply-reply = जवाब दें
reply-reply-all = सभी को जवाब दें
reply-forward = फ़ॉरवर्ड करें

## Encrypted and signed mail

security-decrypting = डिक्रिप्ट किया जा रहा है…
security-checking = हस्ताक्षर की जांच की जा रही है…
security-partly-encrypted = इस मैसेज का सिर्फ़ एक हिस्सा एन्क्रिप्ट किया गया है। बाकी हिस्सा सुरक्षा के बाहर जोड़ा गया था और किसी ने भी भेजा हो सकता है।
security-partly-signed = इस मैसेज के सिर्फ़ एक हिस्से पर हस्ताक्षर है। बाकी हिस्सा सुरक्षा के बाहर जोड़ा गया था और किसी ने भी भेजा हो सकता है।
security-encrypted = एन्क्रिप्ट किया गया मैसेज
security-encrypted-smime = एन्क्रिप्ट किया गया मैसेज (S/MIME)
security-no-key = यह मैसेज डिक्रिप्ट नहीं किया जा सकता: इसे ऐसी कुंजी के लिए एन्क्रिप्ट किया गया था जो आपके पास नहीं है।
security-cancelled = डिक्रिप्ट करना रद्द कर दिया गया।
security-damaged = यह मैसेज डिक्रिप्ट नहीं किया जा सकता: एन्क्रिप्ट किया गया डेटा खराब है या बदला गया है।
security-decrypt-unavailable = यह मैसेज डिक्रिप्ट नहीं किया जा सकता: एन्क्रिप्ट किया गया मेल पढ़ने के लिए { $tool } इंस्टॉल करें।
security-decrypt-failed = यह मैसेज डिक्रिप्ट नहीं किया जा सकता: { $reason }
security-unknown-signer = अज्ञात हस्ताक्षरकर्ता
security-signed-verified = { $signer } के हस्ताक्षर · सत्यापित
security-signed-not-sender = { $signer } के हस्ताक्षर, जो भेजने वाले नहीं हैं
security-signed-untrusted = { $signer } के हस्ताक्षर, ऐसी कुंजी से जिसे आपने भरोसेमंद नहीं के रूप में मार्क किया है
security-signed-unverified = { $signer } के हस्ताक्षर · कुंजी सत्यापित नहीं है
security-bad-signature = गलत हस्ताक्षर: हस्ताक्षर के बाद इस मैसेज को बदला गया, या हस्ताक्षर जाली है।
security-signature-expired = { $signer } के हस्ताक्षर · हस्ताक्षर की समय-सीमा खत्म हो गई है
security-key-expired = { $signer } के हस्ताक्षर · तब से कुंजी की समय-सीमा खत्म हो गई है
security-key-revoked = { $signer } के हस्ताक्षर, ऐसी कुंजी से जिसे रद्द कर दिया गया है
security-missing-key = ऐसी कुंजी से हस्ताक्षर किया गया जो आपके पास नहीं है, इसलिए इसकी जांच नहीं हो सकती
security-missing-key-id = ऐसी कुंजी ({ $key }) से हस्ताक्षर किया गया जो आपके पास नहीं है, इसलिए इसकी जांच नहीं हो सकती
security-signature-unavailable = हस्ताक्षरित; हस्ताक्षर की जांच के लिए { $tool } इंस्टॉल करें
security-signature-error = हस्ताक्षर की जांच नहीं हो सकी।

## Remote images and pictures

remote-hidden = इस मैसेज की इमेज छिपाई गई हैं।
remote-show = इमेज दिखाएं
remote-always-show = इस भेजने वाले की इमेज हमेशा दिखाएं
remote-picture-use = इस्तेमाल करें
remote-picture-too-big = 8 MB या उससे छोटी तस्वीर चुनें।
remote-picture-type = PNG, JPEG, GIF, WebP या SVG तस्वीर चुनें।
remote-picture-read-failed = तस्वीर पढ़ी नहीं जा सकी: { $error }
remote-picture-keep-failed = तस्वीर सेव नहीं की जा सकी: { $error }
remote-picture-remove-failed = तस्वीर हटाई नहीं जा सकी: { $error }

## Attachments

attachment-count = { $count ->
    [one] एक अटैचमेंट
   *[other] { $count } अटैचमेंट
}
attachment-save = सेव करें
attachment-save-all = सभी सेव करें
attachment-save-all-tooltip = सभी अटैचमेंट किसी फ़ोल्डर में सेव करें
attachment-save-here = यहां सेव करें
attachment-not-downloaded = यह मैसेज डाउनलोड नहीं किया गया है।
attachment-not-found = यह अटैचमेंट मैसेज में नहीं मिला।
attachment-read-failed = { $name } पढ़ी नहीं जा सकी
attachment-numbered = अटैचमेंट { $number }
attachment-saved-all = { $count ->
    [one] { $count } फ़ाइल { $place } में सेव की गई
   *[other] { $count } फ़ाइलें { $place } में सेव की गईं
}
attachment-saved-some = { $total ->
    [one] { $total } में से { $saved } फ़ाइल { $place } में सेव की गई। { $failed } सेव नहीं की जा सकी
   *[other] { $total } में से { $saved } फ़ाइलें { $place } में सेव की गईं। { $failed } सेव नहीं की जा सकी
}
attachment-saved-to = { $path } में सेव किया गया
attachment-save-failed = { $name } सेव नहीं की जा सकी: { $error }
attachment-open-failed = { $name } खोली नहीं जा सकी: { $error }
attachment-risky = यह फ़ाइल कोई प्रोग्राम चला सकती है, इसलिए Katna इसे नहीं खोलता। इसके बजाय इसे सेव करें।
attachment-encrypted-open = यह फ़ाइल एन्क्रिप्ट होकर आई थी। इसे कहीं और खोलने के लिए सेव करें।

## Printing

print-failed = प्रिंट नहीं किया जा सका: { $error }
print-no-font = कोई फ़ॉन्ट नहीं मिला
print-opened-as-pdf = PDF के रूप में खोला गया, ताकि वहां से प्रिंट किया जा सके।
print-not-downloaded = (अभी तक डाउनलोड नहीं किया गया।)
print-encrypted = (एन्क्रिप्ट किया गया। इसका टेक्स्ट प्रिंट करने के लिए इसे Katna Mail में खोलें।)
print-to = पाने वाले: { $addresses }
print-cc = Cc: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = इसके अटैचमेंट पढ़ने के लिए यह मैसेज खोलें।
text-copy = कॉपी करें
text-select-all = सभी चुनें

## Settings page: its tabs

settings-tab-general = सामान्य
settings-tab-inbox = इनबॉक्स
settings-tab-accounts = खाते
settings-tab-subscriptions = सदस्यताएं
settings-tab-appearance = रूप-रंग
settings-tab-shortcuts = शॉर्टकट
settings-tab-default-apps = डिफ़ॉल्ट ऐप
settings-tab-folders-rules = फ़ोल्डर और नियम
settings-tab-compose = लिखें
settings-tab-mcp-server = MCP सर्वर
settings-tab-feedback = उपयोगकर्ता फ़ीडबैक
settings-tab-experimental = प्रयोगात्मक

## Settings page: tabs still to come

settings-tab-subscriptions-coming = आपको मिलने वाले न्यूज़लेटर और मेलिंग लिस्ट देखें, और एक क्लिक में सदस्यता छोड़ें।
settings-tab-folders-rules-coming = फ़ोल्डर और लेबल बनाएं, उनका नाम बदलें, उन्हें ले जाएं और छिपाएं, और चुनें कि कौन-से सिंक हों। नियम नए मेल को भेजने वाले, विषय या शब्दों के हिसाब से अपने-आप छांटते हैं, उन पर लेबल लगाते हैं, उन्हें फ़ॉरवर्ड करते हैं या मिटाते हैं।
settings-tab-mcp-server-coming = इस कंप्यूटर पर मौजूद AI असिस्टेंट को, आपकी मंज़ूरी से, आपका मेल खोजने, पढ़ने और उसके ड्राफ़्ट लिखने दें।

## Settings > General

settings-general-conversations = बातचीत व्यू
settings-general-conversations-group = एक ही मेल के जवाबों को एक साथ रखें
settings-general-conversations-group-detail = सूची में हर बातचीत के लिए एक लाइन
settings-general-reading = पढ़ना
settings-general-newest-first = सबसे नया मैसेज पहले
settings-general-newest-first-detail = बातचीत अपने सबसे नए जवाब से शुरू होती है
settings-general-full-headers = पूरे हेडर दिखाएं
settings-general-full-headers-detail = हर मैसेज पर भेजने वाला, पाने वाला, cc, तारीख और विषय खुले दिखते हैं
settings-general-full-names = पाने वालों के पूरे नाम
settings-general-full-names-detail = “पाने वाले: मैं, Ada” के बजाय “पाने वाले: मैं, Ada Lovelace”
settings-general-mark-read = पढ़ा गया के रूप में मार्क करें
settings-general-mark-read-now = खुलते ही
settings-general-mark-read-1s = 1 सेकंड तक खुला रहने के बाद
settings-general-mark-read-3s = 3 सेकंड तक खुला रहने के बाद
settings-general-mark-read-never = सिर्फ़ तब, जब मैं इसे पढ़ा गया के रूप में मार्क करूं
settings-general-reply-button = जवाब देने का बटन
settings-general-reply-all = सभी को जवाब दें
settings-general-reply-all-detail = हर मैसेज के बगल वाला जवाब बटन सिर्फ़ भेजने वाले को नहीं, सभी को जवाब देता है
settings-general-remote-images = वेब से इमेज
settings-general-remote-images-detail = किसी मैसेज की इमेज लोड करने से उसके भेजने वाले को पता चल जाता है कि आपने उसे खोला, कब खोला और मोटे तौर पर कहां से। बंद होने पर हर मैसेज पहले पूछता है, और आप किसी भेजने वाले की इमेज कभी भी दिखा सकते हैं।
settings-general-remote-images-always = इमेज हमेशा दिखाएं
settings-general-remote-images-always-detail = हर मैसेज में, सिर्फ़ भरोसेमंद भेजने वालों के मैसेज में नहीं
settings-general-sending = भेजना
settings-general-sending-detail = भेजा गया मैसेज कितनी देर रुका रहे, ताकि उसे वापस लिया जा सके।
settings-general-offline = ऑफ़लाइन मेल
settings-general-offline-detail = हाल का मेल पूरा डाउनलोड होता है, ताकि उसे बिना कनेक्शन के पढ़ा जा सके। पुराना मेल खोलने पर डाउनलोड होता है।
settings-general-offline-days = { $count ->
    [one] { $count } दिन
   *[other] { $count } दिन
}
settings-general-offline-years = { $count ->
    [one] { $count } साल
   *[other] { $count } साल
}
settings-general-offline-all = सभी मेल
settings-general-offline-note = कम दिन चुनने पर पहले से डाउनलोड किया गया मेल बना रहता है। सर्वर पर कुछ नहीं बदलता।
settings-general-notifications = सूचनाएं
settings-general-notifications-detail = इनबॉक्स में आए नए मेल के लिए, Katna Mail बंद होने पर भी।
settings-general-new-mail = नए मेल की सूचना दें
settings-general-new-mail-detail = सभी को जवाब दें, पढ़ा गया के रूप में मार्क करें और संग्रह करें बटन के साथ
settings-general-new-mail-sound = आवाज़ चलाएं
settings-general-new-mail-sound-detail = डेस्कटॉप की नए मेल वाली आवाज़
settings-general-desktop = डेस्कटॉप
settings-general-open-at-login = लॉग इन करने पर Katna Mail खोलें
settings-general-open-at-login-detail = सेवा चलती रहे, तो लॉग इन पर मेल वैसे भी सिंक होता है
settings-general-tray = सिस्टम ट्रे में Katna दिखाएं
settings-general-tray-detail = बिना पढ़े मैसेज की संख्या और एक मेन्यू के साथ
settings-general-unread-badge = टास्कबार आइकॉन पर बिना पढ़े मैसेज की संख्या
settings-general-unread-badge-detail = इनबॉक्स के कितने मैसेज नहीं पढ़े गए हैं

## Settings > Inbox

settings-inbox-tabs = इनबॉक्स टैब
settings-inbox-tabs-detail = इनबॉक्स को टैब में बांटें, जैसे आपके मेल प्रोवाइडर की वेबसाइट करती है।
settings-inbox-tabs-show = इनबॉक्स टैब दिखाएं
settings-inbox-tabs-show-detail = बंद होने पर हर खाते के लिए एक ही सूची दिखती है
settings-inbox-no-accounts = टैब चुनने के लिए कोई खाता जोड़ें।
settings-inbox-tabs-automatic = अपने-आप: { $tabs } ({ $provider })
settings-inbox-tabs-off = कोई टैब नहीं
settings-inbox-tabs-gmail = मुख्य, प्रमोशन, सामाजिक, अपडेट, फ़ोरम
settings-inbox-tabs-focused = फ़ोकस्ड और अन्य
settings-inbox-tabs-zoho = इनबॉक्स, न्यूज़लेटर और सूचनाएं
settings-inbox-tabs-shown = दिखाए जाने वाले टैब। जिस टैब को आप बंद करते हैं, उसका मेल { $tab } में रहता है।

## Settings > Appearance

settings-appearance-reading-pane = रीडिंग पेन
settings-appearance-reading-pane-detail = खोली गई बातचीत कहां दिखे।
settings-appearance-pane-right = सूची की दाईं ओर
settings-appearance-pane-none = कोई विभाजन नहीं
settings-appearance-density = डेंसिटी
settings-appearance-density-default = डिफ़ॉल्ट
settings-appearance-density-compact = कॉम्पैक्ट
settings-appearance-scaling = स्केलिंग
settings-appearance-scaling-detail = डेस्कटॉप के अपने स्केल के ऊपर, Katna Mail में सब कुछ बड़ा या छोटा करता है: टेक्स्ट, आइकॉन, खाली जगह और डिवाइडर। आपके भेजे गए मेल का फ़ॉन्ट साइज़ वही रहता है। बहुत छोटे साइज़ पर आइकॉन पर क्लिक करना मुश्किल हो सकता है।
settings-appearance-theme = थीम
settings-appearance-theme-system = डेस्कटॉप जैसी
settings-appearance-theme-light = लाइट
settings-appearance-theme-dark = डार्क
settings-appearance-desktop-colors = डेस्कटॉप के रंग
settings-appearance-desktop-colors-use = डेस्कटॉप के रंग इस्तेमाल करें
settings-appearance-desktop-colors-use-detail = डेस्कटॉप की कलर स्कीम और एक्सेंट कलर
settings-appearance-app-names = ऐप के नाम
settings-appearance-app-names-show = ऐप के नाम दिखाएं
settings-appearance-app-names-show-detail = सबसे बाईं ओर ऐप आइकॉन के नीचे नाम
settings-appearance-sender-pictures = भेजने वालों की तस्वीरें
settings-appearance-sender-pictures-show = कंपनी के लोगो दिखाएं
settings-appearance-sender-pictures-show-detail = भेजने वाले के डोमेन से खोजे जाते हैं, मैसेज से कभी नहीं, और एक हफ़्ते तक रखे जाते हैं
settings-appearance-important = ज़रूरी मार्कर
settings-appearance-important-show = ज़रूरी मार्कर दिखाएं
settings-appearance-important-show-detail = सूची में हर मैसेज के बगल में
settings-appearance-message-width = मैसेज की चौड़ाई
settings-appearance-message-width-limit = मैसेज की चौड़ाई सीमित करें
settings-appearance-message-width-limit-detail = चौड़ी विंडो में लंबी लाइनें पढ़ना आसान होता है
settings-appearance-mail-colors = मेल के रंग
settings-appearance-mail-colors-detail = ज़्यादातर मेल सफ़ेद पेज के लिए डिज़ाइन किए जाते हैं। डार्क थीम में उनके रंग ऐसे गहरे रंगों में बदल दिए जाते हैं जो आसानी से पढ़े जा सकें; बंद होने पर मेल हल्के पेज पर अपने भेजने वाले के रंग ही रखता है।
settings-appearance-dark-mail = मेल के लिए भी गहरे रंग
settings-appearance-dark-mail-detail = सिर्फ़ तब, जब थीम डार्क हो
settings-appearance-attachment-previews = अटैचमेंट की झलक
settings-appearance-attachment-previews-show = अटैचमेंट की झलक दिखाएं
settings-appearance-attachment-previews-show-detail = हर फ़ाइल के कार्ड पर उसके कॉन्टेंट की एक छोटी तस्वीर

## Settings > Default apps

settings-default-apps-intro = क्लिक करने पर अटैचमेंट कहां खुलें। व्यूअर किसी फ़ाइल को हमेशा किसी दूसरे ऐप में भी खोल सकता है। डेस्कटॉप के डिफ़ॉल्ट ऐप उसकी अपनी सेटिंग में चुने जाते हैं।
settings-default-apps-pdf = PDF फ़ाइलें
settings-default-apps-pdf-detail = पेज, ज़ूम के साथ।
settings-default-apps-pictures = तस्वीरें
settings-default-apps-pictures-detail = फ़ोटो (सीधी करके), PNG, GIF, WebP, BMP, TIFF और SVG।
settings-default-apps-text = टेक्स्ट फ़ाइलें
settings-default-apps-text-detail = सादा टेक्स्ट, लॉग, कोड और दूसरा टेक्स्ट।
settings-default-apps-sheets = स्प्रेडशीट
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) और CSV।
settings-default-apps-documents = दस्तावेज़
settings-default-apps-documents-detail = Word (docx) और OpenDocument टेक्स्ट (odt)।
settings-default-apps-katna = Katna Mail का व्यूअर
settings-default-apps-system = डेस्कटॉप का डिफ़ॉल्ट ऐप
settings-default-apps-ask = हर बार पूछें कि कौन-सा ऐप
settings-default-apps-after-saving = सेव करने के बाद
settings-default-apps-show-folder = सेव की गई फ़ाइलें उनके फ़ोल्डर में दिखाएं
settings-default-apps-show-folder-detail = फ़ाइल मैनेजर खोलता है, जिसमें सेव किए गए अटैचमेंट चुने हुए होते हैं

## Settings > Compose

settings-compose-send-from = नए मैसेज इस खाते से भेजें
settings-compose-send-from-detail = जवाब और फ़ॉरवर्ड हमेशा उसी खाते से जाते हैं जिसमें आप हैं।
settings-compose-send-from-current = वह खाता जिसमें आप हैं
settings-compose-send-on-replies = जवाबों पर भेजें
settings-compose-send-on-replies-detail = जवाब या फ़ॉरवर्ड पर “भेजें” क्या करता है। “भेजें” के बगल वाले मेन्यू में दूसरा विकल्प मिलता है।
settings-compose-send-plain = भेजें
settings-compose-send-archive = भेजें और संग्रह करें
settings-compose-signatures = हस्ताक्षर
settings-compose-signatures-detail = आपके मैसेज के नीचे, “--” लाइन के बाद जोड़ा जाता है। लिखने वाली विंडो में कोई दूसरा चुनें।
settings-compose-untitled = बिना नाम का
settings-compose-signature-name = नाम, जैसे ऑफ़िस
settings-compose-signature-first = मेरा हस्ताक्षर
settings-compose-signature-numbered = हस्ताक्षर { $number }
settings-compose-signature-delete = मिटाएं
settings-compose-signature-deleted = हस्ताक्षर मिटाया गया
settings-compose-signature-new = नया बनाएं
settings-compose-no-signatures = अभी तक कोई हस्ताक्षर नहीं है।
settings-compose-no-signature = कोई हस्ताक्षर नहीं
settings-compose-for-new-mail = नए मेल के लिए
settings-compose-for-replies = जवाब और फ़ॉरवर्ड के लिए
settings-compose-for-replies-detail = जिस बातचीत में आपने किसी मैसेज पर हस्ताक्षर किया है, उसमें जवाब उसी हस्ताक्षर से शुरू होता है।
settings-compose-format = फ़ॉर्मैट
settings-compose-plain-text = सादे टेक्स्ट में लिखें
settings-compose-plain-text-detail = नया मेल बिना फ़ॉर्मैटिंग के शुरू होता है; लिखने वाली विंडो में इसे बदला जा सकता है
settings-compose-spelling = वर्तनी
settings-compose-spell-check = लिखते समय वर्तनी जांचें
settings-compose-spell-check-detail = गलत वर्तनी वाले शब्दों के नीचे लाइन दिखती है, और राइट-क्लिक पर सुझाव मिलते हैं
settings-compose-spell-desktop = डेस्कटॉप की भाषा ({ $language })
settings-compose-templates = टेम्प्लेट
settings-compose-templates-detail = जो मेल आप अक्सर लिखते हैं उसे सेव करें, और उससे नया मेल या जवाब शुरू करें।

## Settings > Shortcuts

settings-shortcuts-set = शॉर्टकट सेट
settings-shortcuts-set-detail = किसी जाने-पहचाने मेल ऐप की कुंजियों से शुरू करें। यहां Cmd का मतलब Ctrl है। आपके अपने बदलाव सेट के ऊपर बने रहते हैं, और “डिफ़ॉल्ट बहाल करें” सेट की कुंजियों पर लौटा देता है।
settings-shortcuts-single = एक कुंजी वाले शॉर्टकट
settings-shortcuts-single-detail = Ctrl या Alt के बिना कुंजियां, जैसे वेबमेल में: e संग्रह करता है, j और k ऊपर-नीचे ले जाते हैं, / खोजता है। ये सूची और खुली बातचीत में काम करते हैं, टाइप करते समय कभी नहीं।
settings-shortcuts-single-use = एक कुंजी वाले शॉर्टकट इस्तेमाल करें
settings-shortcuts-single-use-detail = Ctrl वाले शॉर्टकट हमेशा काम करते हैं
settings-shortcuts-how = बदलने के लिए किसी कुंजी पर क्लिक करें, या नई जोड़ने के लिए + पर, फिर नई कुंजियां दबाएं। Esc से रद्द होता है।
settings-shortcuts-restore = डिफ़ॉल्ट बहाल करें
settings-shortcuts-no-key = कोई कुंजी नहीं
settings-shortcuts-press = कुंजियां दबाएं…
settings-shortcuts-then = { $keys } फिर…
settings-shortcuts-moved = { $keys } अब “{ $previous }” के बजाय “{ $action }” करता है।
settings-shortcuts-single-off = एक कुंजी वाले शॉर्टकट बंद हैं, इसलिए यह कुंजी उन्हें चालू करने पर काम करेगी।
settings-shortcuts-restored = हर शॉर्टकट को उसके सेट की कुंजियां फिर से मिल गई हैं।

## Settings search: the line under a result

settings-general-language-summary = ऐप, तारीखों और संख्याओं की भाषा
settings-general-reading-summary = सबसे नया मैसेज पहले, पूरे हेडर, पाने वालों के पूरे नाम
settings-general-mark-read-summary = खोली गई बातचीत कब पढ़ी गई के रूप में मार्क हो: तुरंत, 1 या 3 सेकंड बाद, या खुद मार्क करने पर
settings-general-reply-button-summary = हर मैसेज के बगल वाला जवाब बटन सभी को जवाब देता है
settings-general-remote-images-summary = हर मैसेज की इमेज हमेशा दिखाएं
settings-general-sending-summary = भेजना पहले जैसा करें: भेजा गया मैसेज कितनी देर रुका रहे, ताकि उसे वापस लिया जा सके
settings-general-offline-summary = हाल के कितने दिनों का मेल पूरा डाउनलोड हो, ताकि उसे बिना कनेक्शन के पढ़ा जा सके
settings-general-notifications-summary = नए मेल की सूचनाएं और उनकी आवाज़
settings-general-desktop-summary = लॉग इन करने पर Katna Mail खोलें, सिस्टम ट्रे आइकॉन और टास्कबार आइकॉन पर बिना पढ़े मैसेज की संख्या
settings-accounts-accounts-summary = खाता जोड़ें या हटाएं, या उसकी तस्वीर बदलें
settings-appearance-density-summary = सूची में डिफ़ॉल्ट या कॉम्पैक्ट लाइनें
settings-appearance-scaling-summary = सब कुछ बड़ा या छोटा करें: टेक्स्ट, आइकॉन, खाली जगह और डिवाइडर
settings-appearance-theme-summary = डेस्कटॉप जैसी, लाइट या डार्क
settings-appearance-sender-pictures-summary = कंपनी के लोगो, भेजने वाले के डोमेन से खोजे गए
settings-appearance-important-summary = सूची में हर मैसेज के बगल में ज़रूरी मार्कर
settings-appearance-mail-colors-summary = डार्क थीम में HTML मेल के लिए गहरे रंग, या उसके भेजने वाले के रंग
settings-appearance-attachment-previews-summary = हर अटैचमेंट के कॉन्टेंट की एक छोटी तस्वीर
settings-shortcuts-set-summary = Gmail, Inbox by Gmail, Apple Mail, Outlook या Thunderbird की कुंजियों से शुरू करें
settings-shortcuts-single-summary = Ctrl या Alt के बिना कुंजियां, जैसे वेबमेल में
settings-default-apps-pdf-summary = PDF अटैचमेंट कहां खुलें
settings-default-apps-pictures-summary = फ़ोटो और तस्वीरें कहां खुलें
settings-default-apps-text-summary = सादा टेक्स्ट, लॉग और कोड कहां खुलें
settings-default-apps-sheets-summary = Excel, OpenDocument और CSV फ़ाइलें कहां खुलें
settings-default-apps-documents-summary = Word और OpenDocument टेक्स्ट कहां खुलें
settings-default-apps-after-saving-summary = सेव किए गए अटैचमेंट उनके फ़ोल्डर में दिखाएं
settings-compose-send-from-summary = नया मेल किस खाते से जाए: जिसमें आप हैं, या हमेशा एक ही खाते से
settings-compose-send-on-replies-summary = जवाब और फ़ॉरवर्ड पर भेजें, या भेजें और बातचीत संग्रह करें
settings-compose-signatures-summary = आपके मैसेज के नीचे, “--” लाइन के बाद जोड़ा जाता है
settings-compose-for-new-mail-summary = नया मेल किस हस्ताक्षर से शुरू हो
settings-compose-for-replies-summary = जवाब और फ़ॉरवर्ड किस हस्ताक्षर से शुरू हों
settings-compose-format-summary = नया मेल सादे टेक्स्ट में लिखें
settings-compose-spelling-summary = लिखते समय वर्तनी जांचें, और शब्दकोश की भाषा
settings-compose-templates-summary = जल्द आ रहा है: जो मेल आप अक्सर लिखते हैं उसे सेव करें, और उससे नया मेल या जवाब शुरू करें
settings-feedback-crash-reports-summary = Katna Mail या उसकी बैकग्राउंड सेवा के क्रैश होने पर क्रैश रिपोर्ट इस कंप्यूटर पर सेव करें
settings-feedback-saved-summary = इस कंप्यूटर पर सेव की गई क्रैश रिपोर्ट देखें, कॉपी करें या मिटाएं
settings-feedback-help-improve-summary = जो गड़बड़ हुई उसे ठीक करने में मदद के लिए क्रैश रिपोर्ट भेजें; जब तक आप चालू न करें, यह बंद रहता है
settings-experimental-blur-summary = ऊपरी बार में से डेस्कटॉप धुंधला दिखता है, और मेन्यू फ़्रॉस्टेड ग्लास जैसे दिखते हैं
settings-search-shortcut = कीबोर्ड शॉर्टकट
settings-search-tab = सेटिंग टैब
settings-search-none = “{ $query }” से मेल खाने वाली कोई सेटिंग नहीं है।
settings-search-results = “{ $query }” से मेल खाने वाली सेटिंग

## Quick settings (the panel that slides in from the right)

quick-title = क्विक सेटिंग
quick-see-all = सभी सेटिंग देखें
quick-reading-pane = रीडिंग पेन
quick-pane-right = सूची की दाईं ओर
quick-pane-none = कोई विभाजन नहीं
quick-density = डेंसिटी
quick-density-default = डिफ़ॉल्ट
quick-density-compact = कॉम्पैक्ट
quick-theme = थीम
quick-theme-system = डेस्कटॉप जैसी
quick-theme-light = लाइट
quick-theme-dark = डार्क
quick-desktop-colors = डेस्कटॉप के रंग
quick-desktop-colors-detail = डेस्कटॉप की कलर स्कीम और एक्सेंट कलर
quick-app-names = ऐप के नाम
quick-app-names-detail = सबसे बाईं ओर ऐप आइकॉन के नीचे नाम
quick-inbox-tabs = इनबॉक्स टैब
quick-inbox-tabs-detail = हर खाते के मेल प्रोवाइडर के टैब
quick-choose-tabs = टैब चुनें
quick-choose-tabs-detail = हर खाते के लिए, सेटिंग में
quick-sending = भेजना
quick-undo-send = भेजना पहले जैसा करें
quick-undo-send-off = बंद
quick-undo-send-seconds = { $seconds } सेकंड
quick-signatures = हस्ताक्षर
quick-signatures-none = अभी कोई नहीं
quick-signatures-one = { $name }, डिफ़ॉल्ट रूप से इस्तेमाल होता है
quick-signatures-many = { $count ->
    [one] { $count } हस्ताक्षर; डिफ़ॉल्ट रूप से { $name }
   *[other] { $count } हस्ताक्षर; डिफ़ॉल्ट रूप से { $name }
}
quick-signatures-no-default = { $count ->
    [one] { $count }, कोई डिफ़ॉल्ट नहीं
   *[other] { $count }, कोई डिफ़ॉल्ट नहीं
}
quick-signature-untitled = बिना नाम का
quick-threading = ईमेल थ्रेडिंग
quick-conversation-view = बातचीत व्यू
quick-conversation-view-detail = एक ही मेल के जवाबों को एक साथ रखें
quick-help = सहायता
quick-tour = ऐप का टूर करें
quick-whats-new = नया क्या है
quick-about = Katna के बारे में

## Settings: opening at login

settings-open-at-login-failed = लॉग इन पर खुलने की सेटिंग नहीं बदली जा सकी: { $error }

## Settings > Appearance > Scaling

scale-letter = अ
scale-percent = { $percent }%
scale-reset = वापस { $percent }% पर

## Settings > Experimental > Look & Feel

look-intro = ऐसी सुविधाएं जिन्हें अभी आज़माया जा रहा है। ये बदल सकती हैं या हटाई जा सकती हैं।
look-heading = रूप और अनुभव
look-window-frame = विंडो फ़्रेम
look-window-frame-detail = टाइटल बार, विंडो के बटन, कोने और परछाईं कौन बनाता है।
look-frame-native-kde = नेटिव: KDE का फ़्रेम, आपकी Plasma थीम में
look-frame-native = नेटिव: डेस्कटॉप का फ़्रेम
look-frame-katna = Katna: ऊपरी बार ही टाइटल बार बन जाता है
look-frame-katna-note-named = Katna गोल कोने और अपनी परछाईं खुद बनाता है। फ़्रेम अब { $desktop } थीम के हिसाब से नहीं चलता; विंडो के नियम फिर भी लागू होते हैं।
look-frame-katna-note = Katna गोल कोने और अपनी परछाईं खुद बनाता है। फ़्रेम अब डेस्कटॉप थीम के हिसाब से नहीं चलता; विंडो के नियम फिर भी लागू होते हैं।
look-frame-client-side = आपका डेस्कटॉप फ़्रेम हर ऐप पर छोड़ देता है, इसलिए Katna पहले से ही अपना फ़्रेम खुद बनाता है।
look-blurred-background = धुंधला बैकग्राउंड
look-blurred-background-detail = ऊपरी बार और फ़ोल्डरों में से डेस्कटॉप धुंधला दिखता है, और मेन्यू व पॉपओवर फ़्रॉस्टेड ग्लास जैसे दिखते हैं।
look-blur = विंडो के पीछे की चीज़ों को धुंधला करें
look-blur-detail = मेल ठोस कार्ड पर ही रहता है, ताकि टेक्स्ट का कंट्रास्ट बना रहे
look-blur-off-kde = KDE का धुंधला (Blur) इफ़ेक्ट बंद है। सिस्टम सेटिंग्स, विंडो प्रबंधन, डेस्कटॉप प्रभाव में धुंधला चालू करें, फिर Katna Mail दोबारा खोलें।
look-blur-none-gnome = GNOME विंडो के पीछे की चीज़ों को धुंधला नहीं करता।
look-blur-none-x11 = आपका विंडो मैनेजर विंडो के पीछे की चीज़ों को धुंधला नहीं करता।
look-blur-none-wayland = आपका कंपोज़िटर विंडो के पीछे की चीज़ों को धुंधला नहीं करता।

## Settings > User feedback (crash reports)

feedback-intro-sending = जो गड़बड़ हुई उसे ठीक करने में मदद के लिए नई क्रैश रिपोर्ट भेजी जाती हैं। इसके अलावा कुछ भी इस कंप्यूटर से बाहर नहीं जाता।
feedback-intro-local = Katna कहीं कुछ नहीं भेजता। क्रैश रिपोर्ट इसी कंप्यूटर पर रहती हैं, ताकि आप उन्हें देख सकें या किसी बग रिपोर्ट के साथ अटैच कर सकें।
feedback-crash-reports = क्रैश रिपोर्ट
feedback-crash-reports-detail = Katna Mail या उसकी बैकग्राउंड सेवा के क्रैश होने पर लिखी जाती हैं।
feedback-save = क्रैश रिपोर्ट इस कंप्यूटर पर सेव करें
feedback-save-detail = आपका होम फ़ोल्डर, यूज़र और कंप्यूटर के नाम, और ईमेल पते इनमें शामिल नहीं किए जाते
feedback-saved = सेव की गई क्रैश रिपोर्ट
feedback-saved-detail = { $count ->
    [one] सबसे नई { $count } रिपोर्ट रखी जाती है।
   *[other] सबसे नई { $count } रिपोर्ट रखी जाती हैं।
}
feedback-help-improve = Katna को बेहतर बनाने में मदद करें
feedback-help-improve-detail = जब तक आप चालू न करें, यह बंद रहता है, और आप इसे यहां कभी भी बंद कर सकते हैं।
feedback-send = क्रैश रिपोर्ट भेजें
feedback-send-detail = सेव की गई रिपोर्ट, ठीक वैसी ही जैसी आप उसे यहां देख सकते हैं, Katna के क्रैश ट्रैकर (Sentry, EU में) पर जाती है। कोई IP पता, मैसेज या ईमेल पते नहीं
feedback-none-saved = कोई क्रैश रिपोर्ट सेव नहीं है।
feedback-delete-all = सभी मिटाएं
feedback-app-daemon = बैकग्राउंड सेवा
feedback-report-sent = { $date } · भेजी गई
feedback-view = देखें
feedback-view-tooltip = रिपोर्ट खोलें
feedback-copy-tooltip = बग रिपोर्ट में पेस्ट करने के लिए इसे कॉपी करें
feedback-copied = क्रैश रिपोर्ट कॉपी की गई।
feedback-deleted-all = क्रैश रिपोर्ट मिटा दी गईं।
feedback-read-failed = क्रैश रिपोर्ट पढ़ी नहीं जा सकी: { $error }
feedback-delete-failed = क्रैश रिपोर्ट मिटाई नहीं जा सकी: { $error }
feedback-delete-all-failed = क्रैश रिपोर्ट मिटाई नहीं जा सकीं: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _फ़ाइल
desktop-menu-new-message = _नया मैसेज
desktop-menu-quit = _बाहर निकलें
desktop-menu-edit = _संपादन
desktop-menu-undo = _पहले जैसा करें
desktop-menu-select-all = _सभी चुनें
desktop-menu-select-none = _कोई नहीं चुनें
desktop-menu-find = _ढूंढें…
desktop-menu-view = _देखें
desktop-menu-folder-list = _फ़ोल्डर सूची दिखाएं
desktop-menu-refresh = _रीफ़्रेश करें
desktop-menu-go = _जाएं
desktop-menu-inbox = _इनबॉक्स
desktop-menu-starred = _तारांकित
desktop-menu-sent = _भेजे गए
desktop-menu-drafts = _ड्राफ़्ट
desktop-menu-all-mail = _सभी मेल
desktop-menu-next = _अगली बातचीत
desktop-menu-previous = _पिछली बातचीत
desktop-menu-message = _मैसेज
desktop-menu-open = _खोलें
desktop-menu-reply = _जवाब दें
desktop-menu-reply-all = _सभी को जवाब दें
desktop-menu-forward = _फ़ॉरवर्ड करें
desktop-menu-archive = _संग्रह करें
desktop-menu-delete = _मिटाएं
desktop-menu-spam = _स्पैम की शिकायत करें
desktop-menu-move-to = _इसमें ले जाएं…
desktop-menu-mark-read = _पढ़ा गया के रूप में मार्क करें
desktop-menu-mark-unread = _नहीं पढ़ा गया के रूप में मार्क करें
desktop-menu-star = _तारांकित करें
desktop-menu-important = _ज़रूरी के रूप में मार्क करें
desktop-menu-not-important = _ज़रूरी नहीं के रूप में मार्क करें
desktop-menu-settings = _सेटिंग
desktop-menu-quick-settings = _क्विक सेटिंग
desktop-menu-configure = _Katna Mail कॉन्फ़िगर करें…
desktop-menu-help = _सहायता
desktop-menu-shortcuts = _कीबोर्ड शॉर्टकट
desktop-menu-whats-new = _नया क्या है
desktop-menu-about = _Katna के बारे में

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = नेविगेशन
shortcut-group-actions = कार्रवाइयां
shortcut-group-go-to = यहां जाएं
shortcut-group-app = ऐप्लिकेशन

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = अगली बातचीत
shortcut-previous = पिछली बातचीत
shortcut-down = सूची में नीचे जाएं
shortcut-up = सूची में ऊपर जाएं
shortcut-first = सूची में पहली
shortcut-last = सूची में आखिरी
shortcut-page-down = सूची में एक पेज नीचे
shortcut-page-up = सूची में एक पेज ऊपर
shortcut-open = बातचीत खोलें
shortcut-back = सूची पर वापस जाएं
shortcut-scroll-down = नीचे स्क्रॉल करें
shortcut-scroll-up = ऊपर स्क्रॉल करें
shortcut-scroll-page-down = एक पेज नीचे स्क्रॉल करें
shortcut-scroll-page-up = एक पेज ऊपर स्क्रॉल करें
shortcut-compose = लिखें
shortcut-reply = जवाब दें
shortcut-reply-all = सभी को जवाब दें
shortcut-forward = फ़ॉरवर्ड करें
shortcut-archive = संग्रह करें
shortcut-delete = मिटाएं
shortcut-spam = स्पैम की शिकायत करें
shortcut-move-to = इसमें ले जाएं
shortcut-mark-read = पढ़ा गया के रूप में मार्क करें
shortcut-mark-unread = नहीं पढ़ा गया के रूप में मार्क करें
shortcut-star = तारांकित करें या तारांकन हटाएं
shortcut-important = ज़रूरी के रूप में मार्क करें
shortcut-not-important = ज़रूरी नहीं के रूप में मार्क करें
shortcut-check = बातचीत चुनें
shortcut-select-all = सभी बातचीत चुनें
shortcut-select-none = सभी बातचीत से चुनाव हटाएं
shortcut-undo = पिछली कार्रवाई पहले जैसी करें
shortcut-go-inbox = इनबॉक्स
shortcut-go-starred = तारांकित
shortcut-go-sent = भेजे गए
shortcut-go-drafts = ड्राफ़्ट
shortcut-go-all = सभी मेल
shortcut-search = मेल खोजें
shortcut-navigation = मेन्यू दिखाएं या छिपाएं
shortcut-quick-settings = क्विक सेटिंग
shortcut-settings = सभी सेटिंग
shortcut-shortcuts = कीबोर्ड शॉर्टकट
shortcut-reload = नए मेल देखें
shortcut-quit = बाहर निकलें

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } फिर { $second }

## Settings > Accounts

accounts-folder-pane = फ़ोल्डर पेन
accounts-folder-pane-detail = बाईं ओर का पेन किन खातों के फ़ोल्डर दिखाए।
accounts-shown-one = एक बार में एक खाता; खाता कार्ड में बदलें
accounts-shown-all = सभी खाते, एक के बाद एक
accounts-row = खाते
accounts-row-detail = कोई खाता हटाने पर इस कंप्यूटर पर मौजूद उसके मेल की Katna वाली कॉपी मिट जाती है। मेल सर्वर पर बना रहता है।
accounts-none = अभी तक कोई खाता नहीं है।
accounts-kind-imported = इंपोर्ट किया गया
accounts-picture-reset = डेस्कटॉप की तस्वीर इस्तेमाल करें
accounts-picture-change = तस्वीर बदलें
accounts-remove = हटाएं
accounts-delete-all-row = सारा डेटा मिटाएं
accounts-delete-all-row-detail = नए सिरे से शुरू करें, जैसे नए इंस्टॉल पर।
accounts-delete-all-about = इस कंप्यूटर से हर खाता, सारा सेव किया गया मेल, संपर्क और कैलेंडर, खोज इंडेक्स, आपकी सेटिंग और सेव किए गए पासवर्ड मिटा देता है। आपके मेल सर्वर पर कुछ नहीं बदलता।
accounts-delete-all-open = Katna का सारा डेटा मिटाएं

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } को Katna से हटा दिया गया।
accounts-removed = { $address } को Katna से हटा दिया गया। इसका मेल अब भी सर्वर पर है।
accounts-all-deleted = Katna का सारा डेटा इस कंप्यूटर से मिटा दिया गया।

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = { $address } हटाएं?
accounts-remove-confirm = खाता हटाएं
accounts-removing = हटाया जा रहा है…
accounts-remove-local-mail = { $folders ->
    [0] इस खाते में इंपोर्ट किया गया सारा मेल
    [one] इस खाते में इंपोर्ट किया गया सारा मेल, उसके फ़ोल्डर में
   *[other] इस खाते में इंपोर्ट किया गया सारा मेल, उसके { $folders } फ़ोल्डरों में
}
accounts-remove-local-settings = इसकी Katna सेटिंग
accounts-remove-mail = { $folders ->
    [0] Katna में सेव किया गया इस खाते का सारा मेल
    [one] Katna में सेव किया गया इस खाते का सारा मेल, उसके फ़ोल्डर में
   *[other] Katna में सेव किया गया इस खाते का सारा मेल, उसके { $folders } फ़ोल्डरों में
}
accounts-remove-outbox = आउटबॉक्स में भेजे जाने का इंतज़ार कर रहे इसके मैसेज
accounts-remove-settings = इसका सेव किया गया पासवर्ड और इसकी Katna सेटिंग
accounts-delete-all-title = Katna का सारा डेटा मिटाएं?
accounts-delete-all-confirm = सब कुछ मिटाएं
accounts-deleting = मिटाया जा रहा है…
accounts-delete-all-accounts = हर खाता, और Katna में सेव किए गए सभी मेल और अटैचमेंट
accounts-delete-all-contacts = संपर्क, कैलेंडर और खोज इंडेक्स
accounts-delete-all-settings = सभी सेटिंग, हस्ताक्षर और कीबोर्ड शॉर्टकट
accounts-delete-all-passwords = सेव किया गया हर पासवर्ड
accounts-deleted-heading = इस कंप्यूटर से मिटाया जाएगा:
accounts-cannot-undo = इसे पहले जैसा नहीं किया जा सकता।
accounts-server-delete-all = आपके मेल सर्वर पर कुछ नहीं बदलता: आपका मेल वहीं रहता है, और खाता फिर से जोड़ने पर वह फिर से डाउनलोड हो जाता है। फ़ाइलों से इंपोर्ट किया गया मेल सिर्फ़ Katna में है; उन फ़ाइलों को छुआ नहीं जाता।
accounts-server-local = यह मेल फ़ाइलों से इंपोर्ट किया गया था, इसलिए इसकी इकलौती कॉपी Katna के पास है। जिन फ़ाइलों से यह आया था उन्हें छुआ नहीं जाता; इसे वापस पाने के लिए उन्हें फिर से इंपोर्ट करें।
accounts-server-remove = मेल सर्वर पर कुछ नहीं बदलता: आपका मेल वहीं रहता है, और खाता फिर से जोड़ने पर वह फिर से डाउनलोड हो जाता है।
accounts-confirm-word = मिटाएं
accounts-confirm-placeholder = “{ accounts-confirm-word }” टाइप करें
accounts-confirm-prompt = पुष्टि करने के लिए “{ accounts-confirm-word }” टाइप करें:
accounts-cancel = रद्द करें
