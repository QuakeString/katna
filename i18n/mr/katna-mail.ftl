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

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = याची अटॅचमेंट वाचण्यासाठी हा मेसेज उघडा.
text-copy = कॉपी करा
text-select-all = सर्व निवडा

## Settings page: its tabs

settings-tab-general = सामान्य
settings-tab-inbox = इनबॉक्स
settings-tab-accounts = खाती
settings-tab-subscriptions = सदस्यता
settings-tab-appearance = स्वरूप
settings-tab-shortcuts = शॉर्टकट
settings-tab-default-apps = डीफॉल्ट ॲप्स
settings-tab-folders-rules = फोल्डर आणि नियम
settings-tab-compose = लिहा
settings-tab-mcp-server = MCP सर्व्हर
settings-tab-feedback = वापरकर्ता अभिप्राय
settings-tab-experimental = प्रायोगिक

## Settings page: tabs still to come

settings-tab-subscriptions-coming = तुम्हाला येणारी वृत्तपत्रे आणि मेलिंग लिस्ट पाहा, आणि एका क्लिकमध्ये सदस्यत्व रद्द करा.
settings-tab-folders-rules-coming = फोल्डर आणि लेबल तयार करा, त्यांचे नाव बदला, हलवा आणि लपवा, आणि कोणते सिंक व्हावेत ते निवडा. नियम नवीन मेल प्रेषक, विषय किंवा शब्दांनुसार आपोआप क्रमवारी लावतात, लेबल लावतात, फॉरवर्ड करतात किंवा हटवतात.
settings-tab-mcp-server-coming = या कॉम्प्युटरवरील AI सहाय्यकांना तुमच्या परवानगीने तुमचा मेल शोधू, वाचू आणि त्याचे मसुदे लिहू द्या.

## Settings > General

settings-general-conversations = संभाषण दृश्य
settings-general-conversations-group = एकाच मेलची उत्तरे एकत्र ठेवा
settings-general-conversations-group-detail = यादीत प्रत्येक संभाषणासाठी एक ओळ
settings-general-reading = वाचन
settings-general-newest-first = सर्वात नवीन मेसेज आधी
settings-general-newest-first-detail = संभाषण त्याच्या सर्वात नवीन उत्तराने सुरू होते
settings-general-full-headers = पूर्ण हेडर दाखवा
settings-general-full-headers-detail = प्रत्येक मेसेजवर प्रेषक, प्रति, cc, तारीख आणि विषय उघडे दिसतात
settings-general-full-names = प्राप्तकर्त्यांची पूर्ण नावे
settings-general-full-names-detail = “प्रति मला, Ada” ऐवजी “प्रति मला, Ada Lovelace”
settings-general-mark-read = वाचलेले म्हणून खूण करा
settings-general-mark-read-now = उघडताच
settings-general-mark-read-1s = 1 सेकंद उघडा राहिल्यानंतर
settings-general-mark-read-3s = 3 सेकंद उघडा राहिल्यानंतर
settings-general-mark-read-never = मी वाचलेले म्हणून खूण करेन तेव्हाच
settings-general-reply-button = उत्तर बटण
settings-general-reply-all = सर्वांना उत्तर द्या
settings-general-reply-all-detail = प्रत्येक मेसेजशेजारचे उत्तर बटण फक्त प्रेषकाला नव्हे, तर सर्वांना उत्तर देते
settings-general-remote-images = वेबवरील इमेज
settings-general-remote-images-detail = एखाद्या मेसेजमधील इमेज लोड केल्याने त्याच्या प्रेषकाला कळते की तुम्ही तो उघडला, केव्हा आणि साधारण कुठून. बंद असल्यास प्रत्येक मेसेज आधी विचारतो, आणि तुम्ही एखाद्या प्रेषकाच्या इमेज कधीही दाखवू शकता.
settings-general-remote-images-always = इमेज नेहमी दाखवा
settings-general-remote-images-always-detail = प्रत्येक मेसेजमध्ये, फक्त विश्वासू प्रेषकांच्या मेसेजमध्ये नव्हे
settings-general-sending = पाठवणे
settings-general-sending-detail = पाठवलेला मेसेज किती वेळ थांबतो, जेणेकरून तो परत घेता येईल.
settings-general-offline = ऑफलाइन मेल
settings-general-offline-detail = अलीकडचा मेल पूर्ण डाउनलोड केला जातो, जेणेकरून तो कनेक्शनशिवाय वाचता येईल. जुना मेल उघडल्यावर डाउनलोड होतो.
settings-general-offline-days = { $count ->
    [one] { $count } दिवस
   *[other] { $count } दिवस
}
settings-general-offline-years = { $count ->
    [one] { $count } वर्ष
   *[other] { $count } वर्षे
}
settings-general-offline-all = सर्व मेल
settings-general-offline-note = कमी दिवस निवडल्यास आधीच डाउनलोड केलेला मेल राहतो. सर्व्हरवर काहीही बदलत नाही.
settings-general-notifications = सूचना
settings-general-notifications-detail = इनबॉक्समधील नवीन मेलसाठी, Katna Mail बंद असतानाही.
settings-general-new-mail = नवीन मेलबद्दल सूचित करा
settings-general-new-mail-detail = सर्वांना उत्तर द्या, वाचलेले म्हणून खूण करा आणि संग्रहित करा या बटणांसह
settings-general-new-mail-sound = आवाज वाजवा
settings-general-new-mail-sound-detail = डेस्कटॉपचा नवीन मेलचा आवाज
settings-general-desktop = डेस्कटॉप
settings-general-open-at-login = लॉग इन केल्यावर Katna Mail उघडा
settings-general-open-at-login-detail = सेवा चालू असेपर्यंत लॉग इन केल्यावर मेल तसाही सिंक होतो
settings-general-tray = सिस्टम ट्रेमध्ये Katna दाखवा
settings-general-tray-detail = न वाचलेल्यांची संख्या आणि एका मेनूसह
settings-general-unread-badge = टास्कबार आयकनवर न वाचलेल्यांची संख्या
settings-general-unread-badge-detail = इनबॉक्समधील किती मेसेज न वाचलेले आहेत

## Settings > Inbox

settings-inbox-tabs = इनबॉक्स टॅब
settings-inbox-tabs-detail = तुमच्या मेल प्रदात्याच्या वेबसाइटप्रमाणे इनबॉक्स टॅबमध्ये विभागा.
settings-inbox-tabs-show = इनबॉक्स टॅब दाखवा
settings-inbox-tabs-show-detail = बंद असल्यास प्रत्येक खात्यासाठी एकच यादी दिसते
settings-inbox-no-accounts = त्याचे टॅब निवडण्यासाठी खाते जोडा.
settings-inbox-tabs-automatic = आपोआप: { $tabs } ({ $provider })
settings-inbox-tabs-off = टॅब नाहीत
settings-inbox-tabs-gmail = प्राथमिक, जाहिराती, सामाजिक, अपडेट, फोरम
settings-inbox-tabs-focused = फोकस्ड आणि इतर
settings-inbox-tabs-zoho = इनबॉक्स, वृत्तपत्रे आणि सूचना
settings-inbox-tabs-shown = दाखवलेले टॅब. तुम्ही बंद केलेल्या टॅबमधील मेल { $tab } मध्ये राहतो.

## Settings > Appearance

settings-appearance-reading-pane = वाचन पेन
settings-appearance-reading-pane-detail = उघडलेले संभाषण कुठे दिसते.
settings-appearance-pane-right = यादीच्या उजवीकडे
settings-appearance-pane-none = विभाजन नाही
settings-appearance-density = घनता
settings-appearance-density-default = डीफॉल्ट
settings-appearance-density-compact = कॉम्पॅक्ट
settings-appearance-scaling = स्केलिंग
settings-appearance-scaling-detail = डेस्कटॉपच्या स्वतःच्या स्केलवर, Katna Mail मधील सर्व काही मोठे किंवा लहान करते: मजकूर, आयकन, अंतर आणि विभाजक. तुम्ही पाठवलेल्या मेलचा फॉन्ट आकार तसाच राहतो. खूप लहान आकारात आयकनवर क्लिक करणे कठीण होऊ शकते.
settings-appearance-theme = थीम
settings-appearance-theme-system = डेस्कटॉपप्रमाणे
settings-appearance-theme-light = लाइट
settings-appearance-theme-dark = डार्क
settings-appearance-desktop-colors = डेस्कटॉपचे रंग
settings-appearance-desktop-colors-use = डेस्कटॉपचे रंग वापरा
settings-appearance-desktop-colors-use-detail = डेस्कटॉपची रंगसंगती आणि ॲक्सेंट रंग
settings-appearance-app-names = ॲप्सची नावे
settings-appearance-app-names-show = ॲप्सची नावे दाखवा
settings-appearance-app-names-show-detail = अगदी डावीकडील ॲप आयकनखाली नावे
settings-appearance-sender-pictures = प्रेषकाची चित्रे
settings-appearance-sender-pictures-show = कंपनीचे लोगो दाखवा
settings-appearance-sender-pictures-show-detail = प्रेषकाच्या डोमेनवरून शोधले जातात, कधीही मेसेजवरून नाही, आणि एक आठवडा ठेवले जातात
settings-appearance-important = महत्त्वाचे चिन्ह
settings-appearance-important-show = महत्त्वाचे चिन्ह दाखवा
settings-appearance-important-show-detail = यादीत प्रत्येक मेसेजच्या शेजारी
settings-appearance-message-width = मेसेजची रुंदी
settings-appearance-message-width-limit = मेसेजची रुंदी मर्यादित करा
settings-appearance-message-width-limit-detail = रुंद विंडोमध्ये लांब ओळी वाचणे सोपे होते
settings-appearance-mail-colors = मेलचे रंग
settings-appearance-mail-colors-detail = बहुतेक मेल पांढऱ्या पानासाठी डिझाइन केलेले असतात. डार्क थीममध्ये त्यांचे रंग सहज वाचता येतील अशा गडद रंगांमध्ये बदलले जातात; बंद असल्यास मेल फिकट पानावर प्रेषकाचे रंगच ठेवतो.
settings-appearance-dark-mail = मेलसाठीही गडद रंग
settings-appearance-dark-mail-detail = फक्त थीम डार्क असताना
settings-appearance-attachment-previews = अटॅचमेंट पूर्वावलोकन
settings-appearance-attachment-previews-show = अटॅचमेंटचे पूर्वावलोकन दाखवा
settings-appearance-attachment-previews-show-detail = प्रत्येक फाइलच्या कार्डवर तिच्या आशयाचे एक छोटे चित्र

## Settings > Default apps

settings-default-apps-intro = क्लिक केल्यावर अटॅचमेंट कुठे उघडतात. व्ह्यूअर फाइल नेहमी दुसऱ्या ॲपमध्येही उघडू शकतो. डेस्कटॉपची डीफॉल्ट ॲप्स त्याच्या स्वतःच्या सेटिंग्जमध्ये सेट केली जातात.
settings-default-apps-pdf = PDF फाइल
settings-default-apps-pdf-detail = पाने, झूमसह.
settings-default-apps-pictures = चित्रे
settings-default-apps-pictures-detail = फोटो (सरळ फिरवलेले), PNG, GIF, WebP, BMP, TIFF आणि SVG.
settings-default-apps-text = मजकूर फाइल
settings-default-apps-text-detail = साधा मजकूर, लॉग, कोड आणि इतर मजकूर.
settings-default-apps-sheets = स्प्रेडशीट
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) आणि CSV.
settings-default-apps-documents = दस्तऐवज
settings-default-apps-documents-detail = Word (docx) आणि OpenDocument मजकूर (odt).
settings-default-apps-katna = Katna Mail चा व्ह्यूअर
settings-default-apps-system = डेस्कटॉपचे डीफॉल्ट ॲप
settings-default-apps-ask = प्रत्येक वेळी कोणते ॲप ते विचारा
settings-default-apps-after-saving = सेव्ह केल्यानंतर
settings-default-apps-show-folder = सेव्ह केलेल्या फाइल त्यांच्या फोल्डरमध्ये दाखवा
settings-default-apps-show-folder-detail = सेव्ह केलेली अटॅचमेंट निवडलेल्या स्थितीत फाइल व्यवस्थापक उघडते

## Settings > Compose

settings-compose-send-from = नवीन मेसेज यावरून पाठवा
settings-compose-send-from-detail = उत्तरे आणि फॉरवर्ड नेहमी तुम्ही ज्या खात्यात आहात त्यावरूनच जातात.
settings-compose-send-from-current = तुम्ही ज्या खात्यात आहात ते
settings-compose-send-on-replies = उत्तरांवर पाठवा
settings-compose-send-on-replies-detail = उत्तर किंवा फॉरवर्डवर “पाठवा” काय करते. “पाठवा” शेजारील मेनूमध्ये दुसरा पर्याय मिळतो.
settings-compose-send-plain = पाठवा
settings-compose-send-archive = पाठवा आणि संग्रहित करा
settings-compose-signatures = स्वाक्षऱ्या
settings-compose-signatures-detail = तुमच्या मेसेजच्या खाली, “--” ओळीनंतर जोडली जाते. लिहिण्याच्या विंडोमध्ये दुसरी निवडा.
settings-compose-untitled = शीर्षक नसलेले
settings-compose-signature-name = नाव, उदा. कार्यालय
settings-compose-signature-first = माझी स्वाक्षरी
settings-compose-signature-numbered = स्वाक्षरी { $number }
settings-compose-signature-delete = हटवा
settings-compose-signature-deleted = स्वाक्षरी हटवली
settings-compose-signature-new = नवीन तयार करा
settings-compose-no-signatures = अजून स्वाक्षऱ्या नाहीत.
settings-compose-no-signature = स्वाक्षरी नाही
settings-compose-for-new-mail = नवीन मेलसाठी
settings-compose-for-replies = उत्तरे आणि फॉरवर्डसाठी
settings-compose-for-replies-detail = ज्या संभाषणात तुम्ही एखाद्या मेसेजवर स्वाक्षरी केली आहे, त्यात उत्तर त्याच स्वाक्षरीने सुरू होते.
settings-compose-format = फॉरमॅट
settings-compose-plain-text = साध्या मजकुरात लिहा
settings-compose-plain-text-detail = नवीन मेल फॉरमॅटिंगशिवाय सुरू होतो; लिहिण्याच्या विंडोमध्ये बदलता येते
settings-compose-spelling = शुद्धलेखन
settings-compose-spell-check = लिहिताना शुद्धलेखन तपासा
settings-compose-spell-check-detail = चुकीच्या शब्दांखाली रेघ येते, आणि राइट-क्लिकवर सूचना मिळतात
settings-compose-spell-desktop = डेस्कटॉपची भाषा ({ $language })
settings-compose-templates = टेम्पलेट
settings-compose-templates-detail = तुम्ही वारंवार लिहिता तो मेल सेव्ह करा, आणि त्यावरून नवीन मेल किंवा उत्तर सुरू करा.

## Settings > Shortcuts

settings-shortcuts-set = शॉर्टकट संच
settings-shortcuts-set-detail = तुमच्या ओळखीच्या मेल ॲपच्या कीजपासून सुरुवात करा. इथे Cmd म्हणजे Ctrl. तुमचे स्वतःचे बदल संचावर कायम राहतात, आणि “डीफॉल्ट पुनर्संचयित करा” संचाच्या कीजवर परत जाते.
settings-shortcuts-single = एका कीचे शॉर्टकट
settings-shortcuts-single-detail = Ctrl किंवा Alt शिवाय की, वेबमेलप्रमाणे: e संग्रहित करते, j आणि k हलवतात, / शोधते. हे यादीत आणि उघडलेल्या संभाषणात काम करतात, टाइप करताना कधीच नाही.
settings-shortcuts-single-use = एका कीचे शॉर्टकट वापरा
settings-shortcuts-single-use-detail = Ctrl शॉर्टकट नेहमी काम करतात
settings-shortcuts-how = बदलण्यासाठी की वर क्लिक करा, किंवा नवीन जोडण्यासाठी + वर, मग नवीन की दाबा. Esc रद्द करते.
settings-shortcuts-restore = डीफॉल्ट पुनर्संचयित करा
settings-shortcuts-no-key = की नाही
settings-shortcuts-press = की दाबा…
settings-shortcuts-then = { $keys } नंतर…
settings-shortcuts-moved = { $keys } आता “{ $previous }” ऐवजी “{ $action }” करते.
settings-shortcuts-single-off = एका कीचे शॉर्टकट बंद आहेत, त्यामुळे ते चालू केल्यावरच ही की काम करेल.
settings-shortcuts-restored = प्रत्येक शॉर्टकटला त्याच्या संचाच्या की पुन्हा मिळाल्या.

## Settings search: the line under a result

settings-general-language-summary = ॲप, तारखा आणि संख्यांची भाषा
settings-general-reading-summary = सर्वात नवीन मेसेज आधी, पूर्ण हेडर, प्राप्तकर्त्यांची पूर्ण नावे
settings-general-mark-read-summary = उघडलेले संभाषण वाचलेले म्हणून केव्हा खूण होते: लगेच, 1 किंवा 3 सेकंदांनंतर, किंवा स्वतः
settings-general-reply-button-summary = प्रत्येक मेसेजशेजारचे उत्तर बटण सर्वांना उत्तर देते
settings-general-remote-images-summary = प्रत्येक मेसेजमधील इमेज नेहमी दाखवा
settings-general-sending-summary = पाठवणे पूर्ववत करा: पाठवलेला मेसेज किती वेळ थांबतो, जेणेकरून तो परत घेता येईल
settings-general-offline-summary = अलीकडच्या किती दिवसांचा मेल पूर्ण डाउनलोड होतो, जेणेकरून तो कनेक्शनशिवाय वाचता येईल
settings-general-notifications-summary = नवीन मेलच्या सूचना आणि त्यांचा आवाज
settings-general-desktop-summary = लॉग इन केल्यावर Katna Mail उघडणे, सिस्टम ट्रे आयकन आणि टास्कबार आयकनवर न वाचलेल्यांची संख्या
settings-accounts-accounts-summary = खाते जोडा किंवा काढा, किंवा त्याचे चित्र बदला
settings-appearance-density-summary = यादीत डीफॉल्ट किंवा कॉम्पॅक्ट ओळी
settings-appearance-scaling-summary = सर्व काही मोठे किंवा लहान करा: मजकूर, आयकन, अंतर आणि विभाजक
settings-appearance-theme-summary = डेस्कटॉपप्रमाणे, लाइट किंवा डार्क
settings-appearance-sender-pictures-summary = कंपनीचे लोगो, प्रेषकाच्या डोमेनवरून शोधलेले
settings-appearance-important-summary = यादीत प्रत्येक मेसेजशेजारचे महत्त्वाचे चिन्ह
settings-appearance-mail-colors-summary = डार्क थीममध्ये HTML मेलसाठी गडद रंग, किंवा प्रेषकाचे रंग
settings-appearance-attachment-previews-summary = प्रत्येक अटॅचमेंटच्या आशयाचे एक छोटे चित्र
settings-shortcuts-set-summary = Gmail, Inbox by Gmail, Apple Mail, Outlook किंवा Thunderbird च्या कीजपासून सुरुवात करा
settings-shortcuts-single-summary = Ctrl किंवा Alt शिवाय की, वेबमेलप्रमाणे
settings-default-apps-pdf-summary = PDF अटॅचमेंट कुठे उघडतात
settings-default-apps-pictures-summary = फोटो आणि चित्रे कुठे उघडतात
settings-default-apps-text-summary = साधा मजकूर, लॉग आणि कोड कुठे उघडतात
settings-default-apps-sheets-summary = Excel, OpenDocument आणि CSV फाइल कुठे उघडतात
settings-default-apps-documents-summary = Word आणि OpenDocument मजकूर कुठे उघडतो
settings-default-apps-after-saving-summary = सेव्ह केलेली अटॅचमेंट त्यांच्या फोल्डरमध्ये दाखवा
settings-compose-send-from-summary = नवीन मेल कोणत्या खात्यावरून जातो: तुम्ही ज्यात आहात ते, किंवा नेहमी एकच
settings-compose-send-on-replies-summary = उत्तरे आणि फॉरवर्डवर पाठवा, किंवा पाठवा आणि संभाषण संग्रहित करा
settings-compose-signatures-summary = तुमच्या मेसेजच्या खाली, “--” ओळीनंतर जोडली जाते
settings-compose-for-new-mail-summary = नवीन मेल ज्या स्वाक्षरीने सुरू होतो
settings-compose-for-replies-summary = उत्तरे आणि फॉरवर्ड ज्या स्वाक्षरीने सुरू होतात
settings-compose-format-summary = नवीन मेल साध्या मजकुरात लिहा
settings-compose-spelling-summary = लिहिताना शुद्धलेखन तपासा, आणि शब्दकोशाची भाषा
settings-compose-templates-summary = लवकरच येत आहे: तुम्ही वारंवार लिहिता तो मेल सेव्ह करा, आणि त्यावरून नवीन मेल किंवा उत्तर सुरू करा
settings-feedback-crash-reports-summary = Katna Mail किंवा त्याची बॅकग्राउंड सेवा क्रॅश झाल्यावर क्रॅश अहवाल या कॉम्प्युटरवर सेव्ह करा
settings-feedback-saved-summary = या कॉम्प्युटरवर सेव्ह केलेले क्रॅश अहवाल पाहा, कॉपी करा किंवा हटवा
settings-feedback-help-improve-summary = काय चुकले ते दुरुस्त करण्यात मदत म्हणून क्रॅश अहवाल पाठवा; तुम्ही चालू करेपर्यंत बंद
settings-experimental-blur-summary = वरच्या बारमधून डेस्कटॉप धूसर दिसतो, आणि मेनू धूसर काचेसारखे दिसतात
settings-search-shortcut = कीबोर्ड शॉर्टकट
settings-search-tab = सेटिंग्ज टॅब
settings-search-none = “{ $query }” शी जुळणारी कोणतीही सेटिंग नाही.
settings-search-results = “{ $query }” शी जुळणाऱ्या सेटिंग्ज

## Quick settings (the panel that slides in from the right)

quick-title = झटपट सेटिंग्ज
quick-see-all = सर्व सेटिंग्ज पाहा
quick-reading-pane = वाचन पेन
quick-pane-right = यादीच्या उजवीकडे
quick-pane-none = विभाजन नाही
quick-density = घनता
quick-density-default = डीफॉल्ट
quick-density-compact = कॉम्पॅक्ट
quick-theme = थीम
quick-theme-system = डेस्कटॉपप्रमाणे
quick-theme-light = लाइट
quick-theme-dark = डार्क
quick-desktop-colors = डेस्कटॉपचे रंग
quick-desktop-colors-detail = डेस्कटॉपची रंगसंगती आणि ॲक्सेंट रंग
quick-app-names = ॲप्सची नावे
quick-app-names-detail = अगदी डावीकडील ॲप आयकनखाली नावे
quick-inbox-tabs = इनबॉक्स टॅब
quick-inbox-tabs-detail = प्रत्येक खात्याच्या मेल प्रदात्याचे टॅब
quick-choose-tabs = टॅब निवडा
quick-choose-tabs-detail = प्रत्येक खात्यासाठी, सेटिंग्जमध्ये
quick-sending = पाठवणे
quick-undo-send = पाठवणे पूर्ववत करा
quick-undo-send-off = बंद
quick-undo-send-seconds = { $seconds } से.
quick-signatures = स्वाक्षऱ्या
quick-signatures-none = अजून नाहीत
quick-signatures-one = { $name }, डीफॉल्ट म्हणून वापरली जाते
quick-signatures-many = { $count ->
    [one] { $count } स्वाक्षरी; डीफॉल्ट { $name }
   *[other] { $count } स्वाक्षऱ्या; डीफॉल्ट { $name }
}
quick-signatures-no-default = { $count ->
    [one] { $count }, डीफॉल्ट नाही
   *[other] { $count }, एकही डीफॉल्ट नाही
}
quick-signature-untitled = शीर्षक नसलेली
quick-threading = ईमेल थ्रेडिंग
quick-conversation-view = संभाषण दृश्य
quick-conversation-view-detail = एकाच मेलची उत्तरे एकत्र ठेवा
quick-help = मदत
quick-tour = ॲपची सफर करा
quick-whats-new = नवीन काय आहे
quick-about = Katna बद्दल

## Settings: opening at login

settings-open-at-login-failed = लॉग इन केल्यावर उघडण्याची सेटिंग बदलता आली नाही: { $error }

## Settings > Appearance > Scaling

scale-letter = अ
scale-percent = { $percent }%
scale-reset = पुन्हा { $percent }% वर

## Settings > Experimental > Look & Feel

look-intro = अजून चाचणीत असलेली वैशिष्ट्ये. ती बदलू शकतात किंवा काढली जाऊ शकतात.
look-heading = रूप आणि अनुभव
look-window-frame = विंडो फ्रेम
look-window-frame-detail = शीर्षक बार, विंडोची बटणे, कोपरे आणि सावली कोण काढते.
look-frame-native-kde = नेटिव्ह: KDE ची फ्रेम, तुमच्या Plasma थीममध्ये
look-frame-native = नेटिव्ह: डेस्कटॉपची फ्रेम
look-frame-katna = Katna: वरचा बारच शीर्षक बार बनतो
look-frame-katna-note-named = Katna गोलाकार कोपरे आणि स्वतःची सावली काढते. फ्रेम आता { $desktop } थीमनुसार चालत नाही; विंडो नियम तरीही लागू होतात.
look-frame-katna-note = Katna गोलाकार कोपरे आणि स्वतःची सावली काढते. फ्रेम आता डेस्कटॉप थीमनुसार चालत नाही; विंडो नियम तरीही लागू होतात.
look-frame-client-side = तुमचा डेस्कटॉप फ्रेम प्रत्येक ॲपवर सोडतो, त्यामुळे Katna आधीच स्वतःची फ्रेम काढते.
look-blurred-background = धूसर पार्श्वभूमी
look-blurred-background-detail = वरचा बार आणि फोल्डरमधून डेस्कटॉप धूसर दिसतो, आणि मेनू व पॉपओव्हर धूसर काचेसारखे दिसतात.
look-blur = विंडोच्या मागे असलेले धूसर करा
look-blur-detail = मेल भरीव कार्डवरच राहतो, त्यामुळे मजकुराचा कॉन्ट्रास्ट टिकून राहतो
look-blur-off-kde = KDE चा धूसर (Blur) इफेक्ट बंद आहे. सिस्टम सेटिंग्ज, विंडो व्यवस्थापन, डेस्कटॉप इफेक्ट्समध्ये धूसर चालू करा, मग Katna Mail पुन्हा उघडा.
look-blur-none-gnome = GNOME विंडोंच्या मागे असलेले धूसर करत नाही.
look-blur-none-x11 = तुमचा विंडो मॅनेजर विंडोंच्या मागे असलेले धूसर करत नाही.
look-blur-none-wayland = तुमचा कंपोझिटर विंडोंच्या मागे असलेले धूसर करत नाही.

## Settings > User feedback (crash reports)

feedback-intro-sending = काय चुकले ते दुरुस्त करण्यात मदत म्हणून नवीन क्रॅश अहवाल पाठवले जातात. इतर काहीही या कॉम्प्युटरबाहेर जात नाही.
feedback-intro-local = Katna कुठेही काहीही पाठवत नाही. क्रॅश अहवाल या कॉम्प्युटरवरच राहतात, तुम्ही पाहण्यासाठी किंवा बग अहवालाला जोडण्यासाठी.
feedback-crash-reports = क्रॅश अहवाल
feedback-crash-reports-detail = Katna Mail किंवा त्याची बॅकग्राउंड सेवा क्रॅश झाल्यावर लिहिले जातात.
feedback-save = क्रॅश अहवाल या कॉम्प्युटरवर सेव्ह करा
feedback-save-detail = तुमचे होम फोल्डर, वापरकर्ता आणि कॉम्प्युटरची नावे आणि ईमेल पत्ते वगळले जातात
feedback-saved = सेव्ह केलेले क्रॅश अहवाल
feedback-saved-detail = { $count ->
    [one] सर्वात नवीन { $count } ठेवला जातो.
   *[other] सर्वात नवीन { $count } ठेवले जातात.
}
feedback-help-improve = Katna सुधारण्यात मदत करा
feedback-help-improve-detail = तुम्ही चालू करेपर्यंत बंद, आणि तुम्ही ते इथे कधीही बंद करू शकता.
feedback-send = क्रॅश अहवाल पाठवा
feedback-send-detail = सेव्ह केलेला अहवाल, तुम्ही इथे पाहू शकता अगदी तसाच, Katna च्या क्रॅश ट्रॅकरकडे (Sentry, EU मध्ये) जातो. कोणताही IP पत्ता, मेसेज किंवा ईमेल पत्ते नाहीत
feedback-none-saved = कोणतेही क्रॅश अहवाल सेव्ह केलेले नाहीत.
feedback-delete-all = सर्व हटवा
feedback-app-daemon = बॅकग्राउंड सेवा
feedback-report-sent = { $date } · पाठवला
feedback-view = पाहा
feedback-view-tooltip = अहवाल उघडा
feedback-copy-tooltip = बग अहवालात पेस्ट करण्यासाठी कॉपी करा
feedback-copied = क्रॅश अहवाल कॉपी केला.
feedback-deleted-all = क्रॅश अहवाल हटवले.
feedback-read-failed = क्रॅश अहवाल वाचता आला नाही: { $error }
feedback-delete-failed = क्रॅश अहवाल हटवता आला नाही: { $error }
feedback-delete-all-failed = क्रॅश अहवाल हटवता आले नाहीत: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _फाइल
desktop-menu-new-message = _नवीन मेसेज
desktop-menu-quit = _बाहेर पडा
desktop-menu-edit = _संपादन
desktop-menu-undo = _पूर्ववत करा
desktop-menu-select-all = _सर्व निवडा
desktop-menu-select-none = _काहीही निवडू नका
desktop-menu-find = _शोधा…
desktop-menu-view = _दृश्य
desktop-menu-folder-list = _फोल्डर यादी दाखवा
desktop-menu-refresh = _रिफ्रेश करा
desktop-menu-go = _जा
desktop-menu-inbox = _इनबॉक्स
desktop-menu-starred = _तारांकित
desktop-menu-sent = _पाठवलेले
desktop-menu-drafts = _मसुदे
desktop-menu-all-mail = _सर्व मेल
desktop-menu-next = _पुढील संभाषण
desktop-menu-previous = _मागील संभाषण
desktop-menu-message = _मेसेज
desktop-menu-open = _उघडा
desktop-menu-reply = _उत्तर द्या
desktop-menu-reply-all = _सर्वांना उत्तर द्या
desktop-menu-forward = _फॉरवर्ड करा
desktop-menu-archive = _संग्रहित करा
desktop-menu-delete = _हटवा
desktop-menu-spam = _स्पॅमचा अहवाल द्या
desktop-menu-move-to = _येथे हलवा…
desktop-menu-mark-read = _वाचलेले म्हणून खूण करा
desktop-menu-mark-unread = _न वाचलेले म्हणून खूण करा
desktop-menu-star = _तारांकित करा
desktop-menu-important = _महत्त्वाचे म्हणून खूण करा
desktop-menu-not-important = _महत्त्वाचे नाही म्हणून खूण करा
desktop-menu-settings = _सेटिंग्ज
desktop-menu-quick-settings = _झटपट सेटिंग्ज
desktop-menu-configure = _Katna Mail कॉन्फिगर करा…
desktop-menu-help = _मदत
desktop-menu-shortcuts = _कीबोर्ड शॉर्टकट
desktop-menu-whats-new = _नवीन काय आहे
desktop-menu-about = _Katna बद्दल

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = नेव्हिगेशन
shortcut-group-actions = क्रिया
shortcut-group-go-to = येथे जा
shortcut-group-app = ॲप्लिकेशन

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = पुढील संभाषण
shortcut-previous = मागील संभाषण
shortcut-down = यादीत खाली जा
shortcut-up = यादीत वर जा
shortcut-first = यादीतील पहिले
shortcut-last = यादीतील शेवटचे
shortcut-page-down = यादीत एक पान खाली
shortcut-page-up = यादीत एक पान वर
shortcut-open = संभाषण उघडा
shortcut-back = यादीकडे परत जा
shortcut-scroll-down = खाली स्क्रोल करा
shortcut-scroll-up = वर स्क्रोल करा
shortcut-scroll-page-down = एक पान खाली स्क्रोल करा
shortcut-scroll-page-up = एक पान वर स्क्रोल करा
shortcut-compose = लिहा
shortcut-reply = उत्तर द्या
shortcut-reply-all = सर्वांना उत्तर द्या
shortcut-forward = फॉरवर्ड करा
shortcut-archive = संग्रहित करा
shortcut-delete = हटवा
shortcut-spam = स्पॅमचा अहवाल द्या
shortcut-move-to = येथे हलवा
shortcut-mark-read = वाचलेले म्हणून खूण करा
shortcut-mark-unread = न वाचलेले म्हणून खूण करा
shortcut-star = तारांकित करा किंवा तारांकन काढा
shortcut-important = महत्त्वाचे म्हणून खूण करा
shortcut-not-important = महत्त्वाचे नाही म्हणून खूण करा
shortcut-check = संभाषणावर टिक करा
shortcut-select-all = सर्व संभाषणांवर टिक करा
shortcut-select-none = सर्व संभाषणांवरील टिक काढा
shortcut-undo = शेवटची कृती पूर्ववत करा
shortcut-go-inbox = इनबॉक्स
shortcut-go-starred = तारांकित
shortcut-go-sent = पाठवलेले
shortcut-go-drafts = मसुदे
shortcut-go-all = सर्व मेल
shortcut-search = मेल शोधा
shortcut-navigation = मेनू दाखवा किंवा लपवा
shortcut-quick-settings = झटपट सेटिंग्ज
shortcut-settings = सर्व सेटिंग्ज
shortcut-shortcuts = कीबोर्ड शॉर्टकट
shortcut-reload = नवीन मेल तपासा
shortcut-quit = बाहेर पडा

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } नंतर { $second }

## Settings > Accounts

accounts-folder-pane = फोल्डर पेन
accounts-folder-pane-detail = डावीकडील पेन कोणत्या खात्यांचे फोल्डर दाखवते.
accounts-shown-one = एका वेळी एक खाते; खाते कार्डमध्ये बदला
accounts-shown-all = सर्व खाती, एकामागोमाग एक
accounts-row = खाती
accounts-row-detail = खाते काढल्यास या कॉम्प्युटरवरील त्याच्या मेलची Katna ची प्रत हटवली जाते. मेल सर्व्हरवर राहतो.
accounts-none = अजून कोणतेही खाते नाही.
accounts-kind-imported = आयात केलेले
accounts-picture-reset = डेस्कटॉपचे चित्र वापरा
accounts-picture-change = चित्र बदला
accounts-remove = काढा
accounts-delete-all-row = सर्व डेटा हटवा
accounts-delete-all-row-detail = नव्या इंस्टॉलप्रमाणे पुन्हा सुरुवात करा.
accounts-delete-all-about = या कॉम्प्युटरवरून प्रत्येक खाते, सर्व साठवलेला मेल, संपर्क आणि कॅलेंडर, शोध इंडेक्स, तुमच्या सेटिंग्ज आणि सेव्ह केलेले पासवर्ड हटवते. तुमच्या मेल सर्व्हरवर काहीही बदलत नाही.
accounts-delete-all-open = Katna चा सर्व डेटा हटवा

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } Katna मधून काढले.
accounts-removed = { $address } Katna मधून काढले. त्याचा मेल अजूनही सर्व्हरवर आहे.
accounts-all-deleted = Katna चा सर्व डेटा या कॉम्प्युटरवरून हटवला.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = { $address } काढायचे?
accounts-remove-confirm = खाते काढा
accounts-removing = काढत आहे…
accounts-remove-local-mail = { $folders ->
    [0] या खात्यात आयात केलेला सर्व मेल
    [one] या खात्यात आयात केलेला सर्व मेल, त्याच्या फोल्डरमधील
   *[other] या खात्यात आयात केलेला सर्व मेल, त्याच्या { $folders } फोल्डरमधील
}
accounts-remove-local-settings = त्याच्या Katna सेटिंग्ज
accounts-remove-mail = { $folders ->
    [0] Katna ने साठवलेला या खात्याचा सर्व मेल
    [one] Katna ने साठवलेला या खात्याचा सर्व मेल, त्याच्या फोल्डरमधील
   *[other] Katna ने साठवलेला या खात्याचा सर्व मेल, त्याच्या { $folders } फोल्डरमधील
}
accounts-remove-outbox = आउटबॉक्समध्ये थांबलेले त्याचे मेसेज
accounts-remove-settings = त्याचा सेव्ह केलेला पासवर्ड आणि त्याच्या Katna सेटिंग्ज
accounts-delete-all-title = Katna चा सर्व डेटा हटवायचा?
accounts-delete-all-confirm = सर्व काही हटवा
accounts-deleting = हटवत आहे…
accounts-delete-all-accounts = प्रत्येक खाते, आणि Katna ने साठवलेले सर्व मेल आणि अटॅचमेंट
accounts-delete-all-contacts = संपर्क, कॅलेंडर आणि शोध इंडेक्स
accounts-delete-all-settings = सर्व सेटिंग्ज, स्वाक्षऱ्या आणि कीबोर्ड शॉर्टकट
accounts-delete-all-passwords = प्रत्येक सेव्ह केलेला पासवर्ड
accounts-deleted-heading = या कॉम्प्युटरवरून हटवले जाईल:
accounts-cannot-undo = हे पूर्ववत करता येणार नाही.
accounts-server-delete-all = तुमच्या मेल सर्व्हरवर काहीही बदलत नाही: तुमचा मेल तिथेच राहतो, आणि खाते पुन्हा जोडल्यास तो पुन्हा डाउनलोड होतो. फाइलमधून आयात केलेला मेल फक्त Katna मध्ये आहे; त्या फाइलना हात लावला जात नाही.
accounts-server-local = हा मेल फाइलमधून आयात केला होता, त्यामुळे त्याची एकमेव प्रत Katna कडे आहे. ज्या फाइलमधून तो आला त्यांना हात लावला जात नाही; तो परत मिळवण्यासाठी त्या पुन्हा आयात करा.
accounts-server-remove = मेल सर्व्हरवर काहीही बदलत नाही: तुमचा मेल तिथेच राहतो, आणि खाते पुन्हा जोडल्यास तो पुन्हा डाउनलोड होतो.
accounts-confirm-word = हटवा
accounts-confirm-placeholder = “{ accounts-confirm-word }” टाइप करा
accounts-confirm-prompt = खात्री करण्यासाठी “{ accounts-confirm-word }” टाइप करा:
accounts-cancel = रद्द करा
