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
