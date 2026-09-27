# Katna Mail, Nepali (नेपाली).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = भाषा: { $language }
language-tooltip-system = भाषा: { $language }, प्रणालीअनुसार
language-search = भाषा खोज्नुहोस्
language-system-default = प्रणालीको पूर्वनिर्धारित
language-system-now = अहिले { $language }
language-no-match = “{ $query }” सँग मिल्ने कुनै भाषा छैन
language-machine = मेसिनद्वारा अनुवाद गरिएको। सुधार गर्न मद्दत गर्नुहोस्
language-setting = भाषा
language-setting-detail = मेनु, बटन र सन्देशहरूको भाषा, र मिति तथा सङ्ख्याको ढाँचा। प्रणालीको पूर्वनिर्धारितले डेस्कटपको सेटिङ पछ्याउँछ।

## Dates and sizes

ago-just-now = भर्खरै
ago-minutes = { $count ->
    [one] { $count } मिनेटअघि
   *[other] { $count } मिनेटअघि
}
ago-hours = { $count ->
    [one] { $count } घण्टाअघि
   *[other] { $count } घण्टाअघि
}
ago-days = { $count ->
    [one] { $count } दिनअघि
   *[other] { $count } दिनअघि
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

folders-hide = फोल्डरहरू लुकाउनुहोस्
folders-show = फोल्डरहरू देखाउनुहोस्
compose = रचना गर्नुहोस्
search = खोज्नुहोस्
search-mail = मेल खोज्नुहोस्
search-settings = सेटिङहरू खोज्नुहोस्
search-clear = खोज हटाउनुहोस्
search-options-show = खोजका विकल्पहरू देखाउनुहोस्
settings = सेटिङहरू
account-add = खाता थप्नुहोस्

## App rail (and the bottom bar on a phone)

rail-mail = मेल
rail-calendar = पात्रो
rail-contacts = सम्पर्कहरू
rail-tasks = कार्यहरू
rail-notes = टिपोटहरू
rail-feeds = फिडहरू

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = छिट्टै आउँदैछ
app-calendar-promise = तपाईंका CalDAV पात्रोहरू, मेलबाट आएका बैठकका निम्तो र रिमाइन्डरहरू, तपाईंको इनबक्सकै छेउमा।
app-tasks-promise = CalDAV सँग सिंक हुने गर्नुपर्ने कामका सूचीहरू, र मेलबाट बनाइएका कार्यहरू।
app-notes-promise = छिटो टिपोटहरू, र पछिका लागि मेल वा वार्तालापमा टिपोटहरू।
app-feeds-promise = RSS र Atom फिडहरू आफ्नो मेलकै छेउमा पढ्नुहोस्।

## Contacts page

app-contacts-loading = तपाईंको मेलबाट मानिसहरू सङ्कलन गर्दै…
app-contacts-empty = तपाईंले मेल आदानप्रदान गर्ने मानिसहरू यहाँ देखिन्छन्।
app-contacts-count = { $count ->
    [one] तपाईंको मेलबाट { $count } जना व्यक्ति, सबैभन्दा धेरै लेखापढी गरिएका पहिले
   *[other] तपाईंको मेलबाट { $count } जना मानिसहरू, सबैभन्दा धेरै लेखापढी गरिएका पहिले
}
app-contacts-top = { $count ->
    [one] तपाईंको मेलबाट शीर्ष { $count } जना व्यक्ति, सबैभन्दा धेरै लेखापढी गरिएका पहिले
   *[other] तपाईंको मेलबाट शीर्ष { $count } जना मानिसहरू, सबैभन्दा धेरै लेखापढी गरिएका पहिले
}
app-contacts-messages = { $count ->
    [one] { $count } सन्देश
   *[other] { $count } सन्देशहरू
}
app-contacts-last = पछिल्लो पटक { $date }

## Navigation (the folders pane)

nav-labels = लेबलहरू
nav-folders = फोल्डरहरू
nav-label-new = नयाँ लेबल बनाउनुहोस्
nav-folder-new = नयाँ फोल्डर बनाउनुहोस्
nav-account-unnamed = खाता { $number }
nav-tab-new = { $count ->
    [one] { $count } नयाँ
   *[other] { $count } नयाँ
}

## Special folders (the user's own folders keep their names)

folder-inbox = इनबक्स
folder-starred = तारा लगाइएको
folder-drafts = ड्राफ्टहरू
folder-sent = पठाइएको
folder-archive = संग्रह
folder-spam = स्प्याम
folder-trash = ट्र्यास
folder-all-mail = सबै मेल
folder-scheduled = तालिकाबद्ध गरिएको

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = नयाँ लेबल
label-folder-new-title = नयाँ फोल्डर
label-prompt = कृपया नयाँ लेबलको नाम प्रविष्ट गर्नुहोस्:
label-folder-prompt = कृपया नयाँ फोल्डरको नाम प्रविष्ट गर्नुहोस्:
label-name-hint = लेबलको नाम
label-folder-name-hint = फोल्डरको नाम
label-nest = लेबललाई यसभित्र राख्नुहोस्:
label-folder-nest = फोल्डरलाई यसभित्र राख्नुहोस्:
label-cancel = रद्द गर्नुहोस्
label-create = बनाउनुहोस्
label-creating = बनाउँदै…
label-created = लेबल “{ $name }” बनाइयो।
label-folder-created = फोल्डर “{ $name }” बनाइयो।

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = प्राथमिक
tab-promotions = प्रमोसनहरू
tab-social = सामाजिक
tab-updates = अपडेटहरू
tab-forums = फोरमहरू
tab-focused = केन्द्रित
tab-other = अन्य
tab-inbox = इनबक्स
tab-newsletters = न्यूजलेटरहरू
tab-notifications = सूचनाहरू
tab-new = { $count } नयाँ
tab-provider-other = Katna द्वारा क्रमबद्ध

## Mail list: toolbar

list-select = चयन गर्नुहोस्
list-refresh = रिफ्रेस गर्नुहोस्
list-more = थप
list-mark-read = पढिएको भनी चिन्ह लगाउनुहोस्
list-mark-unread = नपढिएको भनी चिन्ह लगाउनुहोस्
list-move-to = यहाँ सार्नुहोस्
list-archive = संग्रह गर्नुहोस्
list-spam = स्प्याम भनी रिपोर्ट गर्नुहोस्
list-delete = मेटाउनुहोस्
list-newer = नयाँ
list-older = पुरानो
list-range = { $total } मध्ये { $first }–{ $last }
list-range-about = लगभग { $total } मध्ये { $first }–{ $last }
list-results = “{ $query }” का लागि नतिजाहरू
list-results-corrected = “{ $query }” का लागि नतिजाहरू देखाउँदै
list-search-instead = यसको सट्टा “{ $query }” खोज्नुहोस्
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = सबै
list-pick-none = कुनै पनि होइन
list-pick-read = पढिएको
list-pick-unread = नपढिएको
list-pick-starred = तारा लगाइएको
list-pick-unstarred = तारा नलगाइएको

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] सबै { $count } वार्तालाप चयन गरिएको छ।
       *[other] सबै { $count } वार्तालापहरू चयन गरिएका छन्।
    }
   *[message] { $count ->
        [one] सबै { $count } सन्देश चयन गरिएको छ।
       *[other] सबै { $count } सन्देशहरू चयन गरिएका छन्।
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } का सबै { $count } वार्तालाप चयन गरिएको छ।
       *[other] { $folder } का सबै { $count } वार्तालापहरू चयन गरिएका छन्।
    }
   *[message] { $count ->
        [one] { $folder } का सबै { $count } सन्देश चयन गरिएको छ।
       *[other] { $folder } का सबै { $count } सन्देशहरू चयन गरिएका छन्।
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] स्क्रिनमा भएका सबै { $count } वार्तालाप चयन गरिएको छ।
       *[other] स्क्रिनमा भएका सबै { $count } वार्तालापहरू चयन गरिएका छन्।
    }
   *[message] { $count ->
        [one] स्क्रिनमा भएका सबै { $count } सन्देश चयन गरिएको छ।
       *[other] स्क्रिनमा भएका सबै { $count } सन्देशहरू चयन गरिएका छन्।
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] सबै { $count } वार्तालाप चयन गर्नुहोस्
       *[other] सबै { $count } वार्तालापहरू चयन गर्नुहोस्
    }
   *[message] { $count ->
        [one] सबै { $count } सन्देश चयन गर्नुहोस्
       *[other] सबै { $count } सन्देशहरू चयन गर्नुहोस्
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } का सबै { $count } वार्तालाप चयन गर्नुहोस्
       *[other] { $folder } का सबै { $count } वार्तालापहरू चयन गर्नुहोस्
    }
   *[message] { $count ->
        [one] { $folder } का सबै { $count } सन्देश चयन गर्नुहोस्
       *[other] { $folder } का सबै { $count } सन्देशहरू चयन गर्नुहोस्
    }
}
list-clear-selection = चयन हटाउनुहोस्

## Mail list: empty states

list-empty-search = तपाईंको खोजसँग मिल्ने कुनै सन्देश भेटिएन।
list-empty-tab = { $tab } मा कुनै मेल छैन।
list-empty-tab-unknown = यो ट्याबमा कुनै मेल छैन।
list-empty-folder = { $folder } मा कुनै सन्देश छैन।
list-empty-folder-unknown = यो फोल्डरमा कुनै सन्देश छैन।
list-first-sync = तपाईंको मेल ल्याउँदै…
list-first-sync-detail = मेल आइपुग्दै गर्दा यहाँ देखिन्छ।

## Mail list: lines

row-removed = यो सन्देश हटाइयो।
row-starred = तारा लगाइएको
row-not-starred = तारा नलगाइएको
row-important = महत्त्वपूर्ण। महत्त्वपूर्ण होइन भनी चिन्ह लगाउन क्लिक गर्नुहोस्।
row-mark-important = महत्त्वपूर्ण भनी चिन्ह लगाउनुहोस्
row-pinned = माथि पिन गरिएको
row-pin = माथि पिन गर्नुहोस्
row-unpin = अनपिन गर्नुहोस्

## Mail list: More menu and right-click menu

menu-reply = जवाफ दिनुहोस्
menu-reply-all = सबैलाई जवाफ दिनुहोस्
menu-forward = फर्वार्ड गर्नुहोस्
menu-archive = संग्रह गर्नुहोस्
menu-delete = मेटाउनुहोस्
menu-spam = स्प्याम भनी रिपोर्ट गर्नुहोस्
menu-mark-read = पढिएको भनी चिन्ह लगाउनुहोस्
menu-mark-unread = नपढिएको भनी चिन्ह लगाउनुहोस्
menu-mark-all-read = सबैलाई पढिएको भनी चिन्ह लगाउनुहोस्
menu-star = तारा लगाउनुहोस्
menu-unstar = तारा हटाउनुहोस्
menu-important = महत्त्वपूर्ण भनी चिन्ह लगाउनुहोस्
menu-not-important = महत्त्वपूर्ण होइन भनी चिन्ह लगाउनुहोस्
menu-pin = माथि पिन गर्नुहोस्
menu-unpin = अनपिन गर्नुहोस्
menu-print-all = सबै प्रिन्ट गर्नुहोस्
menu-new-window = नयाँ विन्डोमा खोल्नुहोस्
menu-move-to = यहाँ सार्नुहोस्
menu-move-to-heading = यहाँ सार्नुहोस्:
menu-find-from = { $name } बाट आएका इमेलहरू खोज्नुहोस्

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] वार्तालाप संग्रह गरियो।
       *[other] { $count } वार्तालापहरू संग्रह गरिए।
    }
   *[message] { $count ->
        [one] सन्देश संग्रह गरियो।
       *[other] { $count } सन्देशहरू संग्रह गरिए।
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] वार्तालाप ट्र्यासमा सारियो।
       *[other] { $count } वार्तालापहरू ट्र्यासमा सारिए।
    }
   *[message] { $count ->
        [one] सन्देश ट्र्यासमा सारियो।
       *[other] { $count } सन्देशहरू ट्र्यासमा सारिए।
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] वार्तालाप सारियो।
       *[other] { $count } वार्तालापहरू सारिए।
    }
   *[message] { $count ->
        [one] सन्देश सारियो।
       *[other] { $count } सन्देशहरू सारिए।
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] वार्तालापमा तारा लगाइयो।
       *[other] { $count } वार्तालापहरूमा तारा लगाइयो।
    }
   *[message] { $count ->
        [one] सन्देशमा तारा लगाइयो।
       *[other] { $count } सन्देशहरूमा तारा लगाइयो।
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] वार्तालापबाट तारा हटाइयो।
       *[other] { $count } वार्तालापहरूबाट तारा हटाइयो।
    }
   *[message] { $count ->
        [one] सन्देशबाट तारा हटाइयो।
       *[other] { $count } सन्देशहरूबाट तारा हटाइयो।
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] वार्तालापलाई महत्त्वपूर्ण भनी चिन्ह लगाइयो।
       *[other] { $count } वार्तालापहरूलाई महत्त्वपूर्ण भनी चिन्ह लगाइयो।
    }
   *[message] { $count ->
        [one] सन्देशलाई महत्त्वपूर्ण भनी चिन्ह लगाइयो।
       *[other] { $count } सन्देशहरूलाई महत्त्वपूर्ण भनी चिन्ह लगाइयो।
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] वार्तालापलाई महत्त्वपूर्ण होइन भनी चिन्ह लगाइयो।
       *[other] { $count } वार्तालापहरूलाई महत्त्वपूर्ण होइन भनी चिन्ह लगाइयो।
    }
   *[message] { $count ->
        [one] सन्देशलाई महत्त्वपूर्ण होइन भनी चिन्ह लगाइयो।
       *[other] { $count } सन्देशहरूलाई महत्त्वपूर्ण होइन भनी चिन्ह लगाइयो।
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] वार्तालाप माथि पिन गरियो।
       *[other] { $count } वार्तालापहरू माथि पिन गरिए।
    }
   *[message] { $count ->
        [one] सन्देश माथि पिन गरियो।
       *[other] { $count } सन्देशहरू माथि पिन गरिए।
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] वार्तालाप अनपिन गरियो।
       *[other] { $count } वार्तालापहरू अनपिन गरिए।
    }
   *[message] { $count ->
        [one] सन्देश अनपिन गरियो।
       *[other] { $count } सन्देशहरू अनपिन गरिए।
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] वार्तालापलाई स्प्याम भनी रिपोर्ट गरियो।
       *[other] { $count } वार्तालापहरूलाई स्प्याम भनी रिपोर्ट गरियो।
    }
   *[message] { $count ->
        [one] सन्देशलाई स्प्याम भनी रिपोर्ट गरियो।
       *[other] { $count } सन्देशहरूलाई स्प्याम भनी रिपोर्ट गरियो।
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] वार्तालाप सधैँका लागि मेटाइयो।
       *[other] { $count } वार्तालापहरू सधैँका लागि मेटाइए।
    }
   *[message] { $count ->
        [one] सन्देश सधैँका लागि मेटाइयो।
       *[other] { $count } सन्देशहरू सधैँका लागि मेटाइए।
    }
}
toast-undone = कार्य पूर्ववत गरियो।
toast-undo = पूर्ववत गर्नुहोस्
toast-no-spam-folder = यो खातामा स्प्याम फोल्डर छैन।

## Reading pane: toolbar

reader-close = बन्द गर्नुहोस्
reader-back = पछाडि
reader-mark-unread = नपढिएको भनी चिन्ह लगाउनुहोस्
reader-move-to = यहाँ सार्नुहोस्
reader-more = थप
reader-print-all = सबै प्रिन्ट गर्नुहोस्
reader-new-window = नयाँ विन्डोमा
reader-position = { $total } मध्ये { $position }
reader-newer = नयाँ
reader-older = पुरानो

## Reading pane: the conversation

reader-removed = यो वार्तालाप हटाइयो।
reader-no-subject = (विषय छैन)
reader-collapse-all = सबै खुम्च्याउनुहोस्
reader-expand-all = सबै विस्तार गर्नुहोस्
reader-unknown-sender = (अज्ञात प्रेषक)
reader-date-ago = { $date } ({ $ago })
reader-me = म
reader-to = { $names } लाई
reader-starred = तारा लगाइएको
reader-not-starred = तारा नलगाइएको
reader-too-long = सन्देश पूरै देखाउन धेरै लामो छ।
reader-encrypted-images = इन्क्रिप्ट गरिएको मेलमा वेबका तस्बिरहरू कहिल्यै लोड गरिँदैनन्।
reader-window-failed = नयाँ विन्डो खोल्न सकिएन।

## Reading pane: message details (opened from "to me")

reader-details-from = प्रेषक:
reader-details-to = प्रापक:
reader-details-cc = cc:
reader-details-date = मिति:
reader-details-subject = विषय:

## Reading pane: downloading a message

reader-downloading = सर्भरबाट यो सन्देश डाउनलोड गर्दै…
reader-download-failed = यो सन्देश डाउनलोड गर्न सकिएन।
reader-try-again = फेरि प्रयास गर्नुहोस्

## Reply row

reply-reply = जवाफ दिनुहोस्
reply-reply-all = सबैलाई जवाफ दिनुहोस्
reply-forward = फर्वार्ड गर्नुहोस्

## Encrypted and signed mail

security-decrypting = डिक्रिप्ट गर्दै…
security-checking = हस्ताक्षर जाँच गर्दै…
security-partly-encrypted = यो सन्देशको केही भाग मात्र इन्क्रिप्ट गरिएको छ। बाँकी भाग सुरक्षाबाहिर थपिएको हो र जोसुकैबाट आएको हुन सक्छ।
security-partly-signed = यो सन्देशको केही भागमा मात्र हस्ताक्षर गरिएको छ। बाँकी भाग सुरक्षाबाहिर थपिएको हो र जोसुकैबाट आएको हुन सक्छ।
security-encrypted = इन्क्रिप्ट गरिएको सन्देश
security-encrypted-smime = इन्क्रिप्ट गरिएको सन्देश (S/MIME)
security-no-key = यो सन्देश डिक्रिप्ट गर्न सकिँदैन: यो तपाईंसँग नभएको कुञ्जीका लागि इन्क्रिप्ट गरिएको थियो।
security-cancelled = डिक्रिप्ट गर्ने कार्य रद्द गरियो।
security-damaged = यो सन्देश डिक्रिप्ट गर्न सकिँदैन: इन्क्रिप्ट गरिएको डेटा बिग्रिएको छ वा परिवर्तन गरिएको छ।
security-decrypt-unavailable = यो सन्देश डिक्रिप्ट गर्न सकिँदैन: इन्क्रिप्ट गरिएको मेल पढ्न { $tool } स्थापना गर्नुहोस्।
security-decrypt-failed = यो सन्देश डिक्रिप्ट गर्न सकिँदैन: { $reason }
security-unknown-signer = एक अज्ञात हस्ताक्षरकर्ता
security-signed-verified = { $signer } द्वारा हस्ताक्षरित · प्रमाणित
security-signed-not-sender = { $signer } द्वारा हस्ताक्षरित, जो प्रेषक होइनन्
security-signed-untrusted = तपाईंले अविश्वसनीय भनी चिन्ह लगाएको कुञ्जीद्वारा { $signer } ले हस्ताक्षर गरेको
security-signed-unverified = { $signer } द्वारा हस्ताक्षरित · कुञ्जी प्रमाणित छैन
security-bad-signature = गलत हस्ताक्षर: यो सन्देश हस्ताक्षरपछि परिवर्तन गरिएको छ, वा हस्ताक्षर नक्कली हो।
security-signature-expired = { $signer } द्वारा हस्ताक्षरित · हस्ताक्षरको म्याद सकिएको छ
security-key-expired = { $signer } द्वारा हस्ताक्षरित · त्यसपछि कुञ्जीको म्याद सकिएको छ
security-key-revoked = रद्द गरिएको कुञ्जीद्वारा { $signer } ले हस्ताक्षर गरेको
security-missing-key = तपाईंसँग नभएको कुञ्जीद्वारा हस्ताक्षर गरिएकाले जाँच गर्न सकिँदैन
security-missing-key-id = तपाईंसँग नभएको कुञ्जी ({ $key }) द्वारा हस्ताक्षर गरिएकाले जाँच गर्न सकिँदैन
security-signature-unavailable = हस्ताक्षरित; हस्ताक्षर जाँच गर्न { $tool } स्थापना गर्नुहोस्
security-signature-error = हस्ताक्षर जाँच गर्न सकिएन।

## Remote images and pictures

remote-hidden = यो सन्देशका तस्बिरहरू लुकाइएका छन्।
remote-show = तस्बिरहरू देखाउनुहोस्
remote-always-show = यो प्रेषकबाट सधैँ देखाउनुहोस्
remote-picture-use = प्रयोग गर्नुहोस्
remote-picture-too-big = 8 MB वा सोभन्दा सानो तस्बिर छान्नुहोस्।
remote-picture-type = PNG, JPEG, GIF, WebP वा SVG तस्बिर छान्नुहोस्।
remote-picture-read-failed = तस्बिर पढ्न सकिँदैन: { $error }
remote-picture-keep-failed = तस्बिर राख्न सकिँदैन: { $error }
remote-picture-remove-failed = तस्बिर हटाउन सकिँदैन: { $error }

## Attachments

attachment-count = { $count ->
    [one] एउटा संलग्नक
   *[other] { $count } संलग्नकहरू
}
attachment-save = सेभ गर्नुहोस्
attachment-save-all = सबै सेभ गर्नुहोस्
attachment-save-all-tooltip = सबै संलग्नकहरू एउटा फोल्डरमा सेभ गर्नुहोस्
attachment-save-here = यहीँ सेभ गर्नुहोस्
attachment-not-downloaded = यो सन्देश डाउनलोड गरिएको छैन।
attachment-not-found = यो संलग्नक सन्देशमा फेला पार्न सकिएन।
attachment-read-failed = { $name } पढ्न सकिएन
attachment-numbered = संलग्नक { $number }
attachment-saved-all = { $count ->
    [one] { $count } फाइल { $place } मा सेभ गरियो
   *[other] { $count } फाइलहरू { $place } मा सेभ गरिए
}
attachment-saved-some = { $total ->
    [one] { $total } मध्ये { $saved } फाइल { $place } मा सेभ गरियो। { $failed } सेभ गर्न सकिएन
   *[other] { $total } मध्ये { $saved } फाइलहरू { $place } मा सेभ गरिए। { $failed } सेभ गर्न सकिएन
}
attachment-saved-to = { $path } मा सेभ गरियो
attachment-save-failed = { $name } सेभ गर्न सकिएन: { $error }
attachment-open-failed = { $name } खोल्न सकिएन: { $error }
attachment-risky = यो फाइलले कुनै प्रोग्राम चलाउन सक्छ, त्यसैले Katna ले यसलाई खोल्दैन। बरु यसलाई सेभ गर्नुहोस्।
attachment-encrypted-open = यो फाइल इन्क्रिप्ट भएर आएको हो। अन्यत्र खोल्न यसलाई सेभ गर्नुहोस्।

## Printing

print-failed = प्रिन्ट गर्न सकिएन: { $error }
print-no-font = कुनै फन्ट फेला परेन
print-opened-as-pdf = त्यहाँबाट प्रिन्ट गर्न PDF को रूपमा खोलियो।
print-not-downloaded = (अझै डाउनलोड गरिएको छैन।)
print-encrypted = (इन्क्रिप्ट गरिएको। यसको पाठ प्रिन्ट गर्न यसलाई Katna Mail मा खोल्नुहोस्।)
print-to = प्रापक: { $addresses }
print-cc = Cc: { $addresses }
