# Katna Mail, Marathi (मराठी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = भाषा: { $language }
language-tooltip-system = भाषा: { $language }, सिस्टमनुसार
language-search = भाषा शोधा
language-system-default = सिस्टम डीफॉल्ट
language-system-now = सध्या { $language }
language-no-match = “{ $query }” शी जुळणारी कोणतीही भाषा नाही
language-machine = मशीनद्वारे भाषांतरित. सुधारण्यासाठी मदत करा
language-setting = भाषा
language-setting-detail = मेनू, बटणे आणि मेसेजची भाषा, तसेच तारखा आणि संख्यांचा फॉरमॅट. सिस्टम डीफॉल्ट डेस्कटॉपच्या सेटिंगनुसार चालते.

## Dates and sizes

ago-just-now = आत्ताच
ago-minutes = { $count ->
    [one] { $count } मिनिटापूर्वी
   *[other] { $count } मिनिटांपूर्वी
}
ago-hours = { $count ->
    [one] { $count } तासापूर्वी
   *[other] { $count } तासांपूर्वी
}
ago-days = { $count ->
    [one] { $count } दिवसापूर्वी
   *[other] { $count } दिवसांपूर्वी
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

folders-hide = फोल्डर लपवा
folders-show = फोल्डर दाखवा
compose = लिहा
search = शोधा
search-mail = मेल शोधा
search-settings = सेटिंग्ज शोधा
search-clear = शोध साफ करा
search-options-show = शोध पर्याय दाखवा
settings = सेटिंग्ज
account-add = खाते जोडा

## App rail (and the bottom bar on a phone)

rail-mail = मेल
rail-calendar = कॅलेंडर
rail-contacts = संपर्क
rail-tasks = कार्ये
rail-notes = नोट्स
rail-feeds = फीड

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = लवकरच येत आहे
app-calendar-promise = तुमची CalDAV कॅलेंडर, मेलमधून आलेली मीटिंगची आमंत्रणे आणि रिमाइंडर, तुमच्या इनबॉक्सच्या शेजारीच.
app-tasks-promise = CalDAV सोबत सिंक होणाऱ्या करायच्या कामांच्या याद्या, आणि मेलमधून तयार केलेली कार्ये.
app-notes-promise = झटपट नोट्स, आणि नंतरसाठी एखाद्या मेल किंवा संभाषणावर नोट्स.
app-feeds-promise = तुमच्या मेलच्या शेजारीच RSS आणि Atom फीड वाचा.

## Contacts page

app-contacts-loading = तुमच्या मेलमधून लोक गोळा करत आहे…
app-contacts-empty = तुम्ही ज्यांच्याशी मेलने संपर्क करता ते लोक इथे दिसतील.
app-contacts-count = { $count ->
    [one] तुमच्या मेलमधील { $count } व्यक्ती, सर्वाधिक संपर्क असलेले आधी
   *[other] तुमच्या मेलमधील { $count } लोक, सर्वाधिक संपर्क असलेले आधी
}
app-contacts-top = { $count ->
    [one] तुमच्या मेलमधील शीर्ष { $count } व्यक्ती, सर्वाधिक संपर्क असलेले आधी
   *[other] तुमच्या मेलमधील शीर्ष { $count } लोक, सर्वाधिक संपर्क असलेले आधी
}
app-contacts-messages = { $count ->
    [one] { $count } मेसेज
   *[other] { $count } मेसेज
}
app-contacts-last = शेवटचा { $date }

## Navigation (the folders pane)

nav-labels = लेबल
nav-folders = फोल्डर
nav-label-new = नवीन लेबल तयार करा
nav-folder-new = नवीन फोल्डर तयार करा
nav-account-unnamed = खाते { $number }
nav-tab-new = { $count ->
    [one] { $count } नवीन
   *[other] { $count } नवीन
}

## Special folders (the user's own folders keep their names)

folder-inbox = इनबॉक्स
folder-starred = तारांकित
folder-drafts = मसुदे
folder-sent = पाठवलेले
folder-archive = संग्रहण
folder-spam = स्पॅम
folder-trash = कचरापेटी
folder-all-mail = सर्व मेल
folder-scheduled = शेड्यूल केलेले

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = नवीन लेबल
label-folder-new-title = नवीन फोल्डर
label-prompt = कृपया नवीन लेबलचे नाव एंटर करा:
label-folder-prompt = कृपया नवीन फोल्डरचे नाव एंटर करा:
label-name-hint = लेबलचे नाव
label-folder-name-hint = फोल्डरचे नाव
label-nest = लेबल याच्या खाली ठेवा:
label-folder-nest = फोल्डर याच्या खाली ठेवा:
label-cancel = रद्द करा
label-create = तयार करा
label-creating = तयार करत आहे…
label-created = “{ $name }” लेबल तयार केले.
label-folder-created = “{ $name }” फोल्डर तयार केले.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = प्राथमिक
tab-promotions = जाहिराती
tab-social = सामाजिक
tab-updates = अपडेट
tab-forums = फोरम
tab-focused = फोकस्ड
tab-other = इतर
tab-inbox = इनबॉक्स
tab-newsletters = वृत्तपत्रे
tab-notifications = सूचना
tab-new = { $count } नवीन
tab-provider-other = Katna ने क्रमवारी लावलेले

## Mail list: toolbar

list-select = निवडा
list-refresh = रिफ्रेश करा
list-more = आणखी
list-mark-read = वाचलेले म्हणून खूण करा
list-mark-unread = न वाचलेले म्हणून खूण करा
list-move-to = येथे हलवा
list-archive = संग्रहित करा
list-spam = स्पॅमचा अहवाल द्या
list-delete = हटवा
list-newer = नवीन
list-older = जुने
list-range = { $total } पैकी { $first }–{ $last }
list-range-about = सुमारे { $total } पैकी { $first }–{ $last }
list-results = “{ $query }” साठी परिणाम
list-results-corrected = “{ $query }” साठी परिणाम दाखवत आहे
list-search-instead = त्याऐवजी “{ $query }” शोधा
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = सर्व
list-pick-none = काहीही नाही
list-pick-read = वाचलेले
list-pick-unread = न वाचलेले
list-pick-starred = तारांकित
list-pick-unstarred = तारांकित नसलेले

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } संभाषण निवडले आहे.
       *[other] सर्व { $count } संभाषणे निवडली आहेत.
    }
   *[message] { $count ->
        [one] { $count } मेसेज निवडला आहे.
       *[other] सर्व { $count } मेसेज निवडले आहेत.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } मधील { $count } संभाषण निवडले आहे.
       *[other] { $folder } मधील सर्व { $count } संभाषणे निवडली आहेत.
    }
   *[message] { $count ->
        [one] { $folder } मधील { $count } मेसेज निवडला आहे.
       *[other] { $folder } मधील सर्व { $count } मेसेज निवडले आहेत.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] स्क्रीनवरील { $count } संभाषण निवडले आहे.
       *[other] स्क्रीनवरील सर्व { $count } संभाषणे निवडली आहेत.
    }
   *[message] { $count ->
        [one] स्क्रीनवरील { $count } मेसेज निवडला आहे.
       *[other] स्क्रीनवरील सर्व { $count } मेसेज निवडले आहेत.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } संभाषण निवडा
       *[other] सर्व { $count } संभाषणे निवडा
    }
   *[message] { $count ->
        [one] { $count } मेसेज निवडा
       *[other] सर्व { $count } मेसेज निवडा
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } मधील { $count } संभाषण निवडा
       *[other] { $folder } मधील सर्व { $count } संभाषणे निवडा
    }
   *[message] { $count ->
        [one] { $folder } मधील { $count } मेसेज निवडा
       *[other] { $folder } मधील सर्व { $count } मेसेज निवडा
    }
}
list-clear-selection = निवड साफ करा

## Mail list: empty states

list-empty-search = तुमच्या शोधाशी कोणताही मेसेज जुळला नाही.
list-empty-tab = { $tab } मध्ये कोणताही मेल नाही.
list-empty-tab-unknown = या टॅबमध्ये कोणताही मेल नाही.
list-empty-folder = { $folder } मध्ये कोणताही मेसेज नाही.
list-empty-folder-unknown = या फोल्डरमध्ये कोणताही मेसेज नाही.
list-first-sync = तुमचा मेल आणत आहे…
list-first-sync-detail = मेल येईल तसा इथे दिसेल.

## Mail list: lines

row-removed = हा मेसेज काढून टाकला.
row-starred = तारांकित
row-not-starred = तारांकित नाही
row-important = महत्त्वाचे. महत्त्वाचे नाही म्हणून खूण करण्यासाठी क्लिक करा.
row-mark-important = महत्त्वाचे म्हणून खूण करा
row-pinned = सर्वात वर पिन केलेले
row-pin = सर्वात वर पिन करा
row-unpin = अनपिन करा

## Mail list: More menu and right-click menu

menu-reply = उत्तर द्या
menu-reply-all = सर्वांना उत्तर द्या
menu-forward = फॉरवर्ड करा
menu-archive = संग्रहित करा
menu-delete = हटवा
menu-spam = स्पॅमचा अहवाल द्या
menu-mark-read = वाचलेले म्हणून खूण करा
menu-mark-unread = न वाचलेले म्हणून खूण करा
menu-mark-all-read = सर्व वाचलेले म्हणून खूण करा
menu-star = तारांकित करा
menu-unstar = तारांकन काढा
menu-important = महत्त्वाचे म्हणून खूण करा
menu-not-important = महत्त्वाचे नाही म्हणून खूण करा
menu-pin = सर्वात वर पिन करा
menu-unpin = अनपिन करा
menu-print-all = सर्व प्रिंट करा
menu-new-window = नवीन विंडोमध्ये उघडा
menu-move-to = येथे हलवा
menu-move-to-heading = येथे हलवा:
menu-find-from = { $name } कडून आलेले ईमेल शोधा

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] संभाषण संग्रहित केले.
       *[other] { $count } संभाषणे संग्रहित केली.
    }
   *[message] { $count ->
        [one] मेसेज संग्रहित केला.
       *[other] { $count } मेसेज संग्रहित केले.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] संभाषण कचरापेटीत हलवले.
       *[other] { $count } संभाषणे कचरापेटीत हलवली.
    }
   *[message] { $count ->
        [one] मेसेज कचरापेटीत हलवला.
       *[other] { $count } मेसेज कचरापेटीत हलवले.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] संभाषण हलवले.
       *[other] { $count } संभाषणे हलवली.
    }
   *[message] { $count ->
        [one] मेसेज हलवला.
       *[other] { $count } मेसेज हलवले.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] संभाषण तारांकित केले.
       *[other] { $count } संभाषणे तारांकित केली.
    }
   *[message] { $count ->
        [one] मेसेज तारांकित केला.
       *[other] { $count } मेसेज तारांकित केले.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] संभाषणावरील तारांकन काढले.
       *[other] { $count } संभाषणांवरील तारांकन काढले.
    }
   *[message] { $count ->
        [one] मेसेजवरील तारांकन काढले.
       *[other] { $count } मेसेजवरील तारांकन काढले.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] संभाषणावर महत्त्वाचे म्हणून खूण केली.
       *[other] { $count } संभाषणांवर महत्त्वाचे म्हणून खूण केली.
    }
   *[message] { $count ->
        [one] मेसेजवर महत्त्वाचे म्हणून खूण केली.
       *[other] { $count } मेसेजवर महत्त्वाचे म्हणून खूण केली.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] संभाषणावर महत्त्वाचे नाही म्हणून खूण केली.
       *[other] { $count } संभाषणांवर महत्त्वाचे नाही म्हणून खूण केली.
    }
   *[message] { $count ->
        [one] मेसेजवर महत्त्वाचे नाही म्हणून खूण केली.
       *[other] { $count } मेसेजवर महत्त्वाचे नाही म्हणून खूण केली.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] संभाषण सर्वात वर पिन केले.
       *[other] { $count } संभाषणे सर्वात वर पिन केली.
    }
   *[message] { $count ->
        [one] मेसेज सर्वात वर पिन केला.
       *[other] { $count } मेसेज सर्वात वर पिन केले.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] संभाषण अनपिन केले.
       *[other] { $count } संभाषणे अनपिन केली.
    }
   *[message] { $count ->
        [one] मेसेज अनपिन केला.
       *[other] { $count } मेसेज अनपिन केले.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] संभाषणाचा स्पॅम म्हणून अहवाल दिला.
       *[other] { $count } संभाषणांचा स्पॅम म्हणून अहवाल दिला.
    }
   *[message] { $count ->
        [one] मेसेजचा स्पॅम म्हणून अहवाल दिला.
       *[other] { $count } मेसेजचा स्पॅम म्हणून अहवाल दिला.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] संभाषण कायमचे हटवले.
       *[other] { $count } संभाषणे कायमची हटवली.
    }
   *[message] { $count ->
        [one] मेसेज कायमचा हटवला.
       *[other] { $count } मेसेज कायमचे हटवले.
    }
}
toast-undone = कृती पूर्ववत केली.
toast-undo = पूर्ववत करा
toast-no-spam-folder = या खात्यात स्पॅम फोल्डर नाही.

## Reading pane: toolbar

reader-close = बंद करा
reader-back = मागे
reader-mark-unread = न वाचलेले म्हणून खूण करा
reader-move-to = येथे हलवा
reader-more = आणखी
reader-print-all = सर्व प्रिंट करा
reader-new-window = नवीन विंडोमध्ये
reader-position = { $total } पैकी { $position }
reader-newer = नवीन
reader-older = जुने

## Reading pane: the conversation

reader-removed = हे संभाषण काढून टाकले.
reader-no-subject = (विषय नाही)
reader-collapse-all = सर्व संकुचित करा
reader-expand-all = सर्व विस्तृत करा
reader-unknown-sender = (अज्ञात प्रेषक)
reader-date-ago = { $date } ({ $ago })
reader-me = मला
reader-to = प्रति { $names }
reader-starred = तारांकित
reader-not-starred = तारांकित नाही
reader-too-long = मेसेज खूप मोठा असल्यामुळे पूर्ण दाखवता येत नाही.
reader-encrypted-images = एन्क्रिप्ट केलेल्या मेलमध्ये वेबवरील इमेज कधीही लोड केल्या जात नाहीत.
reader-window-failed = नवीन विंडो उघडता आली नाही.

## Reading pane: message details (opened from "to me")

reader-details-from = प्रेषक:
reader-details-to = प्रति:
reader-details-cc = cc:
reader-details-date = तारीख:
reader-details-subject = विषय:

## Reading pane: downloading a message

reader-downloading = सर्व्हरवरून हा मेसेज डाउनलोड करत आहे…
reader-download-failed = हा मेसेज डाउनलोड करता आला नाही.
reader-try-again = पुन्हा प्रयत्न करा

## Reply row

reply-reply = उत्तर द्या
reply-reply-all = सर्वांना उत्तर द्या
reply-forward = फॉरवर्ड करा

## Encrypted and signed mail

security-decrypting = डिक्रिप्ट करत आहे…
security-checking = स्वाक्षरी तपासत आहे…
security-partly-encrypted = या मेसेजचा फक्त काही भाग एन्क्रिप्ट केलेला आहे. उरलेला भाग संरक्षणाच्या बाहेर जोडला गेला होता आणि तो कोणाकडूनही आलेला असू शकतो.
security-partly-signed = या मेसेजच्या फक्त काही भागावर स्वाक्षरी आहे. उरलेला भाग संरक्षणाच्या बाहेर जोडला गेला होता आणि तो कोणाकडूनही आलेला असू शकतो.
security-encrypted = एन्क्रिप्ट केलेला मेसेज
security-encrypted-smime = एन्क्रिप्ट केलेला मेसेज (S/MIME)
security-no-key = हा मेसेज डिक्रिप्ट करता येत नाही: तो तुमच्याकडे नसलेल्या कीसाठी एन्क्रिप्ट केला होता.
security-cancelled = डिक्रिप्ट करणे रद्द केले.
security-damaged = हा मेसेज डिक्रिप्ट करता येत नाही: एन्क्रिप्ट केलेला डेटा खराब झाला आहे किंवा बदलला गेला आहे.
security-decrypt-unavailable = हा मेसेज डिक्रिप्ट करता येत नाही: एन्क्रिप्ट केलेला मेल वाचण्यासाठी { $tool } इंस्टॉल करा.
security-decrypt-failed = हा मेसेज डिक्रिप्ट करता येत नाही: { $reason }
security-unknown-signer = अज्ञात स्वाक्षरीकर्ता
security-signed-verified = { $signer } यांची स्वाक्षरी · पडताळलेली
security-signed-not-sender = { $signer } यांची स्वाक्षरी, जे प्रेषक नाहीत
security-signed-untrusted = { $signer } यांची स्वाक्षरी, तुम्ही विश्वसनीय नाही म्हणून खूण केलेल्या कीसह
security-signed-unverified = { $signer } यांची स्वाक्षरी · की पडताळलेली नाही
security-bad-signature = चुकीची स्वाक्षरी: स्वाक्षरी केल्यानंतर हा मेसेज बदलला गेला, किंवा स्वाक्षरी बनावट आहे.
security-signature-expired = { $signer } यांची स्वाक्षरी · स्वाक्षरीची मुदत संपली आहे
security-key-expired = { $signer } यांची स्वाक्षरी · त्यानंतर कीची मुदत संपली आहे
security-key-revoked = { $signer } यांची स्वाक्षरी, रद्द केलेल्या कीसह
security-missing-key = तुमच्याकडे नसलेल्या कीने स्वाक्षरी केली आहे, त्यामुळे ती तपासता येत नाही
security-missing-key-id = तुमच्याकडे नसलेल्या कीने ({ $key }) स्वाक्षरी केली आहे, त्यामुळे ती तपासता येत नाही
security-signature-unavailable = स्वाक्षरी केलेले; स्वाक्षरी तपासण्यासाठी { $tool } इंस्टॉल करा
security-signature-error = स्वाक्षरी तपासता आली नाही.

## Remote images and pictures

remote-hidden = या मेसेजमधील इमेज लपवल्या आहेत.
remote-show = इमेज दाखवा
remote-always-show = या प्रेषकाकडील इमेज नेहमी दाखवा
remote-picture-use = वापरा
remote-picture-too-big = 8 MB किंवा त्यापेक्षा लहान चित्र निवडा.
remote-picture-type = PNG, JPEG, GIF, WebP किंवा SVG चित्र निवडा.
remote-picture-read-failed = चित्र वाचता येत नाही: { $error }
remote-picture-keep-failed = चित्र ठेवता येत नाही: { $error }
remote-picture-remove-failed = चित्र काढता येत नाही: { $error }

## Attachments

attachment-count = { $count ->
    [one] एक अटॅचमेंट
   *[other] { $count } अटॅचमेंट
}
attachment-save = सेव्ह करा
attachment-save-all = सर्व सेव्ह करा
attachment-save-all-tooltip = सर्व अटॅचमेंट एका फोल्डरमध्ये सेव्ह करा
attachment-save-here = इथे सेव्ह करा
attachment-not-downloaded = हा मेसेज डाउनलोड केलेला नाही.
attachment-not-found = हे अटॅचमेंट मेसेजमध्ये सापडले नाही.
attachment-read-failed = { $name } वाचता आली नाही
attachment-numbered = अटॅचमेंट { $number }
attachment-saved-all = { $count ->
    [one] { $count } फाइल { $place } मध्ये सेव्ह केली
   *[other] { $count } फाइल { $place } मध्ये सेव्ह केल्या
}
attachment-saved-some = { $total ->
    [one] { $total } पैकी { $saved } फाइल { $place } मध्ये सेव्ह केली. { $failed } सेव्ह करता आली नाही
   *[other] { $total } पैकी { $saved } फाइल { $place } मध्ये सेव्ह केल्या. { $failed } सेव्ह करता आली नाही
}
attachment-saved-to = { $path } मध्ये सेव्ह केले
attachment-save-failed = { $name } सेव्ह करता आली नाही: { $error }
attachment-open-failed = { $name } उघडता आली नाही: { $error }
attachment-risky = ही फाइल एखादा प्रोग्राम चालवू शकते, म्हणून Katna ती उघडत नाही. त्याऐवजी ती सेव्ह करा.
attachment-encrypted-open = ही फाइल एन्क्रिप्ट केलेली आली होती. ती इतरत्र उघडण्यासाठी सेव्ह करा.

## Printing

print-failed = प्रिंट करता आले नाही: { $error }
print-no-font = कोणताही फॉन्ट सापडला नाही
print-opened-as-pdf = तिथून प्रिंट करण्यासाठी PDF म्हणून उघडले.
print-not-downloaded = (अजून डाउनलोड केलेले नाही.)
print-encrypted = (एन्क्रिप्ट केलेले. त्याचा मजकूर प्रिंट करण्यासाठी तो Katna Mail मध्ये उघडा.)
print-to = प्रति: { $addresses }
print-cc = Cc: { $addresses }
