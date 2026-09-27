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

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = यसका संलग्नकहरू पढ्न यो सन्देश खोल्नुहोस्।
text-copy = प्रतिलिपि गर्नुहोस्
text-select-all = सबै चयन गर्नुहोस्

## Settings page: its tabs

settings-tab-general = सामान्य
settings-tab-inbox = इनबक्स
settings-tab-accounts = खाताहरू
settings-tab-subscriptions = सदस्यता
settings-tab-appearance = रूप
settings-tab-shortcuts = सर्टकटहरू
settings-tab-default-apps = पूर्वनिर्धारित एपहरू
settings-tab-folders-rules = फोल्डर र नियमहरू
settings-tab-compose = रचना
settings-tab-mcp-server = MCP सर्भर
settings-tab-feedback = प्रयोगकर्ताको प्रतिक्रिया
settings-tab-experimental = प्रयोगात्मक

## Settings page: tabs still to come

settings-tab-subscriptions-coming = तपाईंले पाउने न्यूजलेटर र मेलिङ सूचीहरू हेर्नुहोस्, र एकै क्लिकमा सदस्यता हटाउनुहोस्।
settings-tab-folders-rules-coming = फोल्डर र लेबलहरू बनाउनुहोस्, तिनको नाम बदल्नुहोस्, सार्नुहोस् र लुकाउनुहोस्, अनि कुनचाहिँ सिंक हुने भनी छान्नुहोस्। नियमहरूले नयाँ मेललाई प्रेषक, विषय वा शब्दहरूका आधारमा आफैँ क्रमबद्ध गर्छन्, लेबल लगाउँछन्, फर्वार्ड गर्छन् वा मेटाउँछन्।
settings-tab-mcp-server-coming = यो कम्प्युटरका AI सहायकहरूलाई तपाईंको स्वीकृतिमा तपाईंको मेल खोज्न, पढ्न र ड्राफ्ट लेख्न दिनुहोस्।

## Settings > General

settings-general-conversations = वार्तालाप दृश्य
settings-general-conversations-group = एउटै मेलका जवाफहरू सँगै राख्नुहोस्
settings-general-conversations-group-detail = सूचीमा प्रत्येक वार्तालापका लागि एउटा पङ्क्ति
settings-general-reading = पढाइ
settings-general-newest-first = सबैभन्दा नयाँ सन्देश पहिले
settings-general-newest-first-detail = वार्तालाप यसको सबैभन्दा नयाँ जवाफबाट सुरु हुन्छ
settings-general-full-headers = पूरा हेडरहरू देखाउनुहोस्
settings-general-full-headers-detail = प्रत्येक सन्देशमा प्रेषक, प्रापक, cc, मिति र विषय खुला देखिन्छन्
settings-general-full-names = प्रापकहरूको पूरा नाम
settings-general-full-names-detail = “म, Ada लाई” को सट्टा “म, Ada Lovelace लाई”
settings-general-mark-read = पढिएको भनी चिन्ह लगाउनुहोस्
settings-general-mark-read-now = खुल्नेबित्तिकै
settings-general-mark-read-1s = 1 सेकेन्ड खुला भएपछि
settings-general-mark-read-3s = 3 सेकेन्ड खुला भएपछि
settings-general-mark-read-never = मैले पढिएको भनी चिन्ह लगाउँदा मात्र
settings-general-reply-button = जवाफ बटन
settings-general-reply-all = सबैलाई जवाफ दिनुहोस्
settings-general-reply-all-detail = प्रत्येक सन्देशको छेउको जवाफ बटनले प्रेषकलाई मात्र होइन, सबैलाई जवाफ दिन्छ
settings-general-remote-images = वेबका तस्बिरहरू
settings-general-remote-images-detail = कुनै सन्देशका तस्बिरहरू लोड गर्दा त्यसको प्रेषकलाई तपाईंले त्यो खोल्नुभयो, कहिले र लगभग कहाँबाट भन्ने थाहा हुन्छ। बन्द हुँदा प्रत्येक सन्देशले पहिले सोध्छ, र तपाईं जुनसुकै बेला कुनै प्रेषकका तस्बिरहरू देखाउन सक्नुहुन्छ।
settings-general-remote-images-always = सधैँ तस्बिरहरू देखाउनुहोस्
settings-general-remote-images-always-detail = प्रत्येक सन्देशमा, विश्वासिलो प्रेषकका सन्देशमा मात्र होइन
settings-general-sending = पठाउने
settings-general-sending-detail = पठाइएको सन्देश कति बेर पर्खन्छ, ताकि त्यसलाई फिर्ता लिन सकियोस्।
settings-general-offline = अफलाइन मेल
settings-general-offline-detail = हालको मेल पूरै डाउनलोड हुन्छ, ताकि जडानबिना पढ्न सकियोस्। पुरानो मेल खोल्दा डाउनलोड हुन्छ।
settings-general-offline-days = { $count ->
    [one] { $count } दिन
   *[other] { $count } दिन
}
settings-general-offline-years = { $count ->
    [one] { $count } वर्ष
   *[other] { $count } वर्ष
}
settings-general-offline-all = सबै मेल
settings-general-offline-note = कम दिन छान्दा पहिले नै डाउनलोड भएको मेल रहन्छ। सर्भरमा केही पनि बदलिँदैन।
settings-general-notifications = सूचनाहरू
settings-general-notifications-detail = इनबक्समा आएको नयाँ मेलका लागि, Katna Mail बन्द हुँदा पनि।
settings-general-new-mail = नयाँ मेलबारे सूचना दिनुहोस्
settings-general-new-mail-detail = सबैलाई जवाफ दिनुहोस्, पढिएको भनी चिन्ह लगाउनुहोस् र संग्रह गर्नुहोस् बटनसहित
settings-general-new-mail-sound = आवाज बजाउनुहोस्
settings-general-new-mail-sound-detail = डेस्कटपको नयाँ मेलको आवाज
settings-general-desktop = डेस्कटप
settings-general-open-at-login = लग इन गर्दा Katna Mail खोल्नुहोस्
settings-general-open-at-login-detail = सेवा चलिरहेसम्म लग इन गर्दा मेल जसरी पनि सिंक हुन्छ
settings-general-tray = प्रणाली ट्रेमा Katna देखाउनुहोस्
settings-general-tray-detail = नपढिएका सन्देशको सङ्ख्या र एउटा मेनुसहित
settings-general-unread-badge = टास्कबार आइकनमा नपढिएका सन्देशको सङ्ख्या
settings-general-unread-badge-detail = इनबक्सका कति सन्देश नपढिएका छन्

## Settings > Inbox

settings-inbox-tabs = इनबक्स ट्याबहरू
settings-inbox-tabs-detail = तपाईंको मेल प्रदायकको वेबसाइटले जस्तै इनबक्सलाई ट्याबहरूमा छुट्याउनुहोस्।
settings-inbox-tabs-show = इनबक्स ट्याबहरू देखाउनुहोस्
settings-inbox-tabs-show-detail = बन्द हुँदा प्रत्येक खाताका लागि एउटै सूची देखाउँछ
settings-inbox-no-accounts = ट्याबहरू छान्न एउटा खाता थप्नुहोस्।
settings-inbox-tabs-automatic = स्वचालित: { $tabs } ({ $provider })
settings-inbox-tabs-off = ट्याब छैन
settings-inbox-tabs-gmail = प्राथमिक, प्रमोसनहरू, सामाजिक, अपडेटहरू, फोरमहरू
settings-inbox-tabs-focused = केन्द्रित र अन्य
settings-inbox-tabs-zoho = इनबक्स, न्यूजलेटरहरू र सूचनाहरू
settings-inbox-tabs-shown = देखाइएका ट्याबहरू। तपाईंले बन्द गरेको ट्याबको मेल { $tab } मा रहन्छ।

## Settings > Appearance

settings-appearance-reading-pane = पढ्ने प्यान
settings-appearance-reading-pane-detail = खोलिएको वार्तालाप कहाँ देखिन्छ।
settings-appearance-pane-right = सूचीको दायाँ
settings-appearance-pane-none = विभाजन छैन
settings-appearance-density = घनत्व
settings-appearance-density-default = पूर्वनिर्धारित
settings-appearance-density-compact = कम्प्याक्ट
settings-appearance-scaling = स्केलिङ
settings-appearance-scaling-detail = डेस्कटपको आफ्नै स्केलमाथि Katna Mail का सबै कुरा ठूला वा साना बनाउँछ: पाठ, आइकन, खाली ठाउँ र विभाजकहरू। तपाईंले पठाएको मेलको फन्ट आकार उस्तै रहन्छ। धेरै साना आकारमा आइकनहरूमा क्लिक गर्न गाह्रो हुन सक्छ।
settings-appearance-theme = थिम
settings-appearance-theme-system = डेस्कटपकै जस्तो
settings-appearance-theme-light = उज्यालो
settings-appearance-theme-dark = अँध्यारो
settings-appearance-desktop-colors = डेस्कटपका रङहरू
settings-appearance-desktop-colors-use = डेस्कटपका रङहरू प्रयोग गर्नुहोस्
settings-appearance-desktop-colors-use-detail = डेस्कटपको रङ योजना र एक्सेन्ट रङ
settings-appearance-app-names = एपका नामहरू
settings-appearance-app-names-show = एपका नामहरू देखाउनुहोस्
settings-appearance-app-names-show-detail = सबैभन्दा बायाँका एप आइकनहरूमुनि नामहरू
settings-appearance-sender-pictures = प्रेषकका तस्बिरहरू
settings-appearance-sender-pictures-show = कम्पनीका लोगोहरू देखाउनुहोस्
settings-appearance-sender-pictures-show-detail = प्रेषकको डोमेनबाट खोजिन्छ, सन्देशबाट कहिल्यै होइन, र एक हप्तासम्म राखिन्छ
settings-appearance-important = महत्त्वपूर्ण चिन्हहरू
settings-appearance-important-show = महत्त्वपूर्ण चिन्हहरू देखाउनुहोस्
settings-appearance-important-show-detail = सूचीमा प्रत्येक सन्देशको छेउमा
settings-appearance-message-width = सन्देशको चौडाइ
settings-appearance-message-width-limit = सन्देशहरूको चौडाइ सीमित गर्नुहोस्
settings-appearance-message-width-limit-detail = चौडा विन्डोमा लामा पङ्क्तिहरू पढ्न सजिलो हुन्छ
settings-appearance-mail-colors = मेलका रङहरू
settings-appearance-mail-colors-detail = धेरैजसो मेल सेतो पानाका लागि डिजाइन गरिएका हुन्छन्। अँध्यारो थिममा तिनका रङहरू राम्रोसँग पढिने गाढा रङमा बदलिन्छन्; बन्द हुँदा मेलले उज्यालो पानामा आफ्नो प्रेषकका रङहरू नै राख्छ।
settings-appearance-dark-mail = मेलका लागि पनि गाढा रङहरू
settings-appearance-dark-mail-detail = थिम अँध्यारो हुँदा मात्र
settings-appearance-attachment-previews = संलग्नकका पूर्वावलोकनहरू
settings-appearance-attachment-previews-show = संलग्नकहरूको पूर्वावलोकन देखाउनुहोस्
settings-appearance-attachment-previews-show-detail = प्रत्येक फाइलको कार्डमा त्यसको सामग्रीको सानो तस्बिर

## Settings > Default apps

settings-default-apps-intro = क्लिक गर्दा संलग्नकहरू कहाँ खुल्छन्। दर्शकले फाइललाई सधैँ अर्को एपमा पनि खोल्न सक्छ। डेस्कटपका पूर्वनिर्धारित एपहरू त्यसकै सेटिङहरूमा तोकिन्छन्।
settings-default-apps-pdf = PDF फाइलहरू
settings-default-apps-pdf-detail = पानाहरू, जुमसहित।
settings-default-apps-pictures = तस्बिरहरू
settings-default-apps-pictures-detail = फोटोहरू (सिधा पारिएका), PNG, GIF, WebP, BMP, TIFF र SVG।
settings-default-apps-text = पाठ फाइलहरू
settings-default-apps-text-detail = सादा पाठ, लगहरू, कोड र अन्य पाठ।
settings-default-apps-sheets = स्प्रेडसिटहरू
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) र CSV।
settings-default-apps-documents = कागजातहरू
settings-default-apps-documents-detail = Word (docx) र OpenDocument पाठ (odt)।
settings-default-apps-katna = Katna Mail को दर्शक
settings-default-apps-system = डेस्कटपको पूर्वनिर्धारित एप
settings-default-apps-ask = हरेक पटक कुन एप भनी सोध्नुहोस्
settings-default-apps-after-saving = सेभ गरेपछि
settings-default-apps-show-folder = सेभ गरिएका फाइलहरू तिनको फोल्डरमा देखाउनुहोस्
settings-default-apps-show-folder-detail = सेभ गरिएका संलग्नकहरू चयन गरिएको अवस्थामा फाइल प्रबन्धक खोल्छ

## Settings > Compose

settings-compose-send-from = नयाँ सन्देशहरू यहाँबाट पठाउनुहोस्
settings-compose-send-from-detail = जवाफ र फर्वार्डहरू सधैँ तपाईं रहेको खाताबाटै जान्छन्।
settings-compose-send-from-current = तपाईं रहेको खाता
settings-compose-send-on-replies = जवाफहरूमा पठाउनुहोस्
settings-compose-send-on-replies-detail = जवाफ वा फर्वार्डमा “पठाउनुहोस्” ले के गर्छ। “पठाउनुहोस्” छेउको मेनुमा अर्को विकल्प छ।
settings-compose-send-plain = पठाउनुहोस्
settings-compose-send-archive = पठाउनुहोस् र संग्रह गर्नुहोस्
settings-compose-signatures = हस्ताक्षरहरू
settings-compose-signatures-detail = तपाईंको सन्देशको तल, “--” पङ्क्तिपछि थपिन्छ। रचना विन्डोमा अर्को छान्नुहोस्।
settings-compose-untitled = शीर्षकविहीन
settings-compose-signature-name = नाम, जस्तै कार्यालय
settings-compose-signature-first = मेरो हस्ताक्षर
settings-compose-signature-numbered = हस्ताक्षर { $number }
settings-compose-signature-delete = मेटाउनुहोस्
settings-compose-signature-deleted = हस्ताक्षर मेटाइयो
settings-compose-signature-new = नयाँ बनाउनुहोस्
settings-compose-no-signatures = अहिलेसम्म कुनै हस्ताक्षर छैन।
settings-compose-no-signature = हस्ताक्षर छैन
settings-compose-for-new-mail = नयाँ मेलका लागि
settings-compose-for-replies = जवाफ र फर्वार्डका लागि
settings-compose-for-replies-detail = तपाईंले कुनै सन्देशमा हस्ताक्षर गरेको वार्तालापमा, जवाफ त्यही हस्ताक्षरबाट सुरु हुन्छ।
settings-compose-format = ढाँचा
settings-compose-plain-text = सादा पाठमा लेख्नुहोस्
settings-compose-plain-text-detail = नयाँ मेल ढाँचाबिना सुरु हुन्छ; रचना विन्डोमा बदल्न सकिन्छ
settings-compose-spelling = हिज्जे
settings-compose-spell-check = लेख्दै गर्दा हिज्जे जाँच गर्नुहोस्
settings-compose-spell-check-detail = गलत हिज्जे भएका शब्दमुनि रेखा लाग्छ, र दायाँ-क्लिकमा सुझावहरू आउँछन्
settings-compose-spell-desktop = डेस्कटपको भाषा ({ $language })
settings-compose-templates = टेम्प्लेटहरू
settings-compose-templates-detail = तपाईंले प्रायः लेख्ने मेल सेभ गर्नुहोस्, र त्यसबाट नयाँ मेल वा जवाफ सुरु गर्नुहोस्।

## Settings > Shortcuts

settings-shortcuts-set = सर्टकट सेट
settings-shortcuts-set-detail = तपाईंलाई परिचित मेल एपका कुञ्जीहरूबाट सुरु गर्नुहोस्। यहाँ Cmd भनेको Ctrl हो। तपाईंका आफ्नै परिवर्तनहरू सेटमाथि रहन्छन्, र “पूर्वनिर्धारित पुनर्स्थापना गर्नुहोस्” ले सेटकै कुञ्जीहरूमा फर्काउँछ।
settings-shortcuts-single = एकल-कुञ्जी सर्टकटहरू
settings-shortcuts-single-detail = Ctrl वा Alt बिनाका कुञ्जीहरू, वेबमेलमा जस्तै: e ले संग्रह गर्छ, j र k ले सार्छन्, / ले खोज्छ। यी सूची र खुला वार्तालापमा काम गर्छन्, टाइप गर्दा कहिल्यै होइन।
settings-shortcuts-single-use = एकल-कुञ्जी सर्टकटहरू प्रयोग गर्नुहोस्
settings-shortcuts-single-use-detail = Ctrl सर्टकटहरू सधैँ काम गर्छन्
settings-shortcuts-how = बदल्न कुनै कुञ्जीमा क्लिक गर्नुहोस्, वा थप्न + मा, त्यसपछि नयाँ कुञ्जीहरू थिच्नुहोस्। Esc ले रद्द गर्छ।
settings-shortcuts-restore = पूर्वनिर्धारित पुनर्स्थापना गर्नुहोस्
settings-shortcuts-no-key = कुञ्जी छैन
settings-shortcuts-press = कुञ्जीहरू थिच्नुहोस्…
settings-shortcuts-then = { $keys } त्यसपछि…
settings-shortcuts-moved = { $keys } ले अब “{ $previous }” को सट्टा “{ $action }” गर्छ।
settings-shortcuts-single-off = एकल-कुञ्जी सर्टकटहरू बन्द छन्, त्यसैले यो कुञ्जी तिनलाई खोलेपछि मात्र काम गर्छ।
settings-shortcuts-restored = प्रत्येक सर्टकटले फेरि आफ्नो सेटका कुञ्जीहरू पायो।

## Settings search: the line under a result

settings-general-language-summary = एप, मिति र सङ्ख्याहरूको भाषा
settings-general-reading-summary = सबैभन्दा नयाँ सन्देश पहिले, पूरा हेडरहरू, प्रापकहरूको पूरा नाम
settings-general-mark-read-summary = खोलिएको वार्तालाप कहिले पढिएको भनी चिन्ह लाग्छ: तुरुन्तै, 1 वा 3 सेकेन्डपछि, वा आफैँ
settings-general-reply-button-summary = प्रत्येक सन्देशको छेउको जवाफ बटनले सबैलाई जवाफ दिन्छ
settings-general-remote-images-summary = प्रत्येक सन्देशका तस्बिरहरू सधैँ देखाउनुहोस्
settings-general-sending-summary = पठाएको पूर्ववत गर्नुहोस्: पठाइएको सन्देश कति बेर पर्खन्छ, ताकि त्यसलाई फिर्ता लिन सकियोस्
settings-general-offline-summary = हालका कति दिनको मेल पूरै डाउनलोड हुन्छ, ताकि जडानबिना पढ्न सकियोस्
settings-general-notifications-summary = नयाँ मेलका सूचनाहरू र तिनको आवाज
settings-general-desktop-summary = लग इन गर्दा Katna Mail खोल्ने, प्रणाली ट्रे आइकन र टास्कबार आइकनमा नपढिएका सन्देशको सङ्ख्या
settings-accounts-accounts-summary = खाता थप्नुहोस् वा हटाउनुहोस्, वा यसको तस्बिर बदल्नुहोस्
settings-appearance-density-summary = सूचीमा पूर्वनिर्धारित वा कम्प्याक्ट पङ्क्तिहरू
settings-appearance-scaling-summary = सबै कुरा ठूला वा साना बनाउनुहोस्: पाठ, आइकन, खाली ठाउँ र विभाजकहरू
settings-appearance-theme-summary = डेस्कटपकै जस्तो, उज्यालो वा अँध्यारो
settings-appearance-sender-pictures-summary = कम्पनीका लोगोहरू, प्रेषकको डोमेनबाट खोजिएका
settings-appearance-important-summary = सूचीमा प्रत्येक सन्देशको छेउमा महत्त्वपूर्ण चिन्ह
settings-appearance-mail-colors-summary = अँध्यारो थिममा HTML मेलका लागि गाढा रङहरू, वा यसको प्रेषकका रङहरू
settings-appearance-attachment-previews-summary = प्रत्येक संलग्नकको सामग्रीको सानो तस्बिर
settings-shortcuts-set-summary = Gmail, Inbox by Gmail, Apple Mail, Outlook वा Thunderbird का कुञ्जीहरूबाट सुरु गर्नुहोस्
settings-shortcuts-single-summary = Ctrl वा Alt बिनाका कुञ्जीहरू, वेबमेलमा जस्तै
settings-default-apps-pdf-summary = PDF संलग्नकहरू कहाँ खुल्छन्
settings-default-apps-pictures-summary = फोटो र तस्बिरहरू कहाँ खुल्छन्
settings-default-apps-text-summary = सादा पाठ, लग र कोड कहाँ खुल्छन्
settings-default-apps-sheets-summary = Excel, OpenDocument र CSV फाइलहरू कहाँ खुल्छन्
settings-default-apps-documents-summary = Word र OpenDocument पाठ कहाँ खुल्छन्
settings-default-apps-after-saving-summary = सेभ गरिएका संलग्नकहरू तिनको फोल्डरमा देखाउनुहोस्
settings-compose-send-from-summary = नयाँ मेल कुन खाताबाट जान्छ: तपाईं रहेको खाता, वा सधैँ एउटै खाता
settings-compose-send-on-replies-summary = जवाफ र फर्वार्डमा पठाउनुहोस्, वा पठाउनुहोस् र वार्तालाप संग्रह गर्नुहोस्
settings-compose-signatures-summary = तपाईंको सन्देशको तल, “--” पङ्क्तिपछि थपिन्छ
settings-compose-for-new-mail-summary = नयाँ मेल सुरु हुने हस्ताक्षर
settings-compose-for-replies-summary = जवाफ र फर्वार्ड सुरु हुने हस्ताक्षर
settings-compose-format-summary = नयाँ मेल सादा पाठमा लेख्नुहोस्
settings-compose-spelling-summary = लेख्दै गर्दा हिज्जे जाँच, र शब्दकोशको भाषा
settings-compose-templates-summary = छिट्टै आउँदैछ: तपाईंले प्रायः लेख्ने मेल सेभ गर्नुहोस्, र त्यसबाट नयाँ मेल वा जवाफ सुरु गर्नुहोस्
settings-feedback-crash-reports-summary = Katna Mail वा यसको पृष्ठभूमि सेवा क्र्यास हुँदा क्र्यास रिपोर्टहरू यो कम्प्युटरमा सेभ गर्नुहोस्
settings-feedback-saved-summary = यो कम्प्युटरमा सेभ गरिएका क्र्यास रिपोर्टहरू हेर्नुहोस्, प्रतिलिपि गर्नुहोस् वा मेटाउनुहोस्
settings-feedback-help-improve-summary = के बिग्रियो भनी सुधार्न मद्दत गर्न क्र्यास रिपोर्टहरू पठाउनुहोस्; तपाईंले नखोलेसम्म बन्द
settings-experimental-blur-summary = माथिल्लो बारबाट डेस्कटप धमिलो देखिन्छ, र मेनुहरू धमिलो सिसाजस्ता देखिन्छन्
settings-search-shortcut = किबोर्ड सर्टकट
settings-search-tab = सेटिङ ट्याब
settings-search-none = “{ $query }” सँग मिल्ने कुनै सेटिङ छैन।
settings-search-results = “{ $query }” सँग मिल्ने सेटिङहरू

## Quick settings (the panel that slides in from the right)

quick-title = द्रुत सेटिङहरू
quick-see-all = सबै सेटिङहरू हेर्नुहोस्
quick-reading-pane = पढ्ने प्यान
quick-pane-right = सूचीको दायाँ
quick-pane-none = विभाजन छैन
quick-density = घनत्व
quick-density-default = पूर्वनिर्धारित
quick-density-compact = कम्प्याक्ट
quick-theme = थिम
quick-theme-system = डेस्कटपकै जस्तो
quick-theme-light = उज्यालो
quick-theme-dark = अँध्यारो
quick-desktop-colors = डेस्कटपका रङहरू
quick-desktop-colors-detail = डेस्कटपको रङ योजना र एक्सेन्ट रङ
quick-app-names = एपका नामहरू
quick-app-names-detail = सबैभन्दा बायाँका एप आइकनहरूमुनि नामहरू
quick-inbox-tabs = इनबक्स ट्याबहरू
quick-inbox-tabs-detail = प्रत्येक खाताको मेल प्रदायकका ट्याबहरू
quick-choose-tabs = ट्याबहरू छान्नुहोस्
quick-choose-tabs-detail = प्रत्येक खाताका लागि, सेटिङहरूमा
quick-sending = पठाउने
quick-undo-send = पठाएको पूर्ववत गर्नुहोस्
quick-undo-send-off = बन्द
quick-undo-send-seconds = { $seconds } सेकेन्ड
quick-signatures = हस्ताक्षरहरू
quick-signatures-none = अहिलेसम्म छैन
quick-signatures-one = { $name }, पूर्वनिर्धारित रूपमा प्रयोग हुन्छ
quick-signatures-many = { $count ->
    [one] { $count } हस्ताक्षर; पूर्वनिर्धारित { $name }
   *[other] { $count } हस्ताक्षरहरू; पूर्वनिर्धारित { $name }
}
quick-signatures-no-default = { $count ->
    [one] { $count }, पूर्वनिर्धारित छैन
   *[other] { $count }, कुनै पनि पूर्वनिर्धारित छैन
}
quick-signature-untitled = शीर्षकविहीन
quick-threading = इमेल थ्रेडिङ
quick-conversation-view = वार्तालाप दृश्य
quick-conversation-view-detail = एउटै मेलका जवाफहरू सँगै राख्नुहोस्
quick-help = मद्दत
quick-tour = एपको भ्रमण गर्नुहोस्
quick-whats-new = नयाँ के छ
quick-about = Katna को बारेमा

## Settings: opening at login

settings-open-at-login-failed = लग इनमा खुल्ने सेटिङ बदल्न सकिएन: { $error }

## Settings > Appearance > Scaling

scale-letter = अ
scale-percent = { $percent }%
scale-reset = फेरि { $percent }% मा

## Settings > Experimental > Look & Feel

look-intro = अझै परीक्षण भइरहेका सुविधाहरू। यी बदलिन वा हटाइन सक्छन्।
look-heading = रूप र अनुभूति
look-window-frame = विन्डो फ्रेम
look-window-frame-detail = शीर्षक बार, विन्डोका बटनहरू, कुनाहरू र छाया कसले कोर्छ।
look-frame-native-kde = नेटिभ: KDE को फ्रेम, तपाईंको Plasma थिममा
look-frame-native = नेटिभ: डेस्कटपको फ्रेम
look-frame-katna = Katna: माथिल्लो बार नै शीर्षक बार बन्छ
look-frame-katna-note-named = Katna ले गोलाकार कुनाहरू र आफ्नै छाया कोर्छ। फ्रेमले अब { $desktop } थिम पछ्याउँदैन; विन्डो नियमहरू भने लागू हुन्छन्।
look-frame-katna-note = Katna ले गोलाकार कुनाहरू र आफ्नै छाया कोर्छ। फ्रेमले अब डेस्कटप थिम पछ्याउँदैन; विन्डो नियमहरू भने लागू हुन्छन्।
look-frame-client-side = तपाईंको डेस्कटपले फ्रेम प्रत्येक एपलाई छोडिदिन्छ, त्यसैले Katna ले पहिले नै आफ्नै फ्रेम कोर्छ।
look-blurred-background = धमिलो पृष्ठभूमि
look-blurred-background-detail = माथिल्लो बार र फोल्डरहरूबाट डेस्कटप धमिलो देखिन्छ, र मेनु तथा पपओभरहरू धमिलो सिसाजस्ता देखिन्छन्।
look-blur = विन्डोपछाडि भएको कुरा धमिलो बनाउनुहोस्
look-blur-detail = मेल ठोस कार्डमै रहन्छ, त्यसैले पाठको कन्ट्रास्ट कायम रहन्छ
look-blur-off-kde = KDE को धमिलो (Blur) प्रभाव बन्द छ। प्रणाली सेटिङहरू, विन्डो व्यवस्थापन, डेस्कटप प्रभावहरूमा धमिलो खोल्नुहोस्, त्यसपछि Katna Mail फेरि खोल्नुहोस्।
look-blur-none-gnome = GNOME ले विन्डोपछाडि भएको कुरा धमिलो बनाउँदैन।
look-blur-none-x11 = तपाईंको विन्डो प्रबन्धकले विन्डोपछाडि भएको कुरा धमिलो बनाउँदैन।
look-blur-none-wayland = तपाईंको कम्पोजिटरले विन्डोपछाडि भएको कुरा धमिलो बनाउँदैन।

## Settings > User feedback (crash reports)

feedback-intro-sending = के बिग्रियो भनी सुधार्न मद्दत गर्न नयाँ क्र्यास रिपोर्टहरू पठाइन्छन्। अरू केही पनि यो कम्प्युटरबाहिर जाँदैन।
feedback-intro-local = Katna ले कतै केही पनि पठाउँदैन। क्र्यास रिपोर्टहरू यही कम्प्युटरमा रहन्छन्, तपाईंले हेर्न वा बग रिपोर्टमा संलग्न गर्नका लागि।
feedback-crash-reports = क्र्यास रिपोर्टहरू
feedback-crash-reports-detail = Katna Mail वा यसको पृष्ठभूमि सेवा क्र्यास हुँदा लेखिन्छन्।
feedback-save = क्र्यास रिपोर्टहरू यो कम्प्युटरमा सेभ गर्नुहोस्
feedback-save-detail = तपाईंको होम फोल्डर, प्रयोगकर्ता र कम्प्युटरका नामहरू, र इमेल ठेगानाहरू हटाइन्छन्
feedback-saved = सेभ गरिएका क्र्यास रिपोर्टहरू
feedback-saved-detail = { $count ->
    [one] सबैभन्दा नयाँ { $count } राखिन्छ।
   *[other] सबैभन्दा नयाँ { $count } वटा राखिन्छन्।
}
feedback-help-improve = Katna सुधार्न मद्दत गर्नुहोस्
feedback-help-improve-detail = तपाईंले नखोलेसम्म बन्द, र तपाईं यसलाई यहाँ जुनसुकै बेला बन्द गर्न सक्नुहुन्छ।
feedback-send = क्र्यास रिपोर्टहरू पठाउनुहोस्
feedback-send-detail = सेभ गरिएको रिपोर्ट, तपाईंले यहाँ हेर्न सक्ने ठ्याक्कै त्यस्तै, Katna को क्र्यास ट्र्याकर (Sentry, EU मा) मा जान्छ। कुनै IP ठेगाना, सन्देश वा इमेल ठेगाना जाँदैन
feedback-none-saved = कुनै क्र्यास रिपोर्ट सेभ गरिएको छैन।
feedback-delete-all = सबै मेटाउनुहोस्
feedback-app-daemon = पृष्ठभूमि सेवा
feedback-report-sent = { $date } · पठाइयो
feedback-view = हेर्नुहोस्
feedback-view-tooltip = रिपोर्ट खोल्नुहोस्
feedback-copy-tooltip = बग रिपोर्टमा टाँस्न यसको प्रतिलिपि गर्नुहोस्
feedback-copied = क्र्यास रिपोर्टको प्रतिलिपि गरियो।
feedback-deleted-all = क्र्यास रिपोर्टहरू मेटाइए।
feedback-read-failed = क्र्यास रिपोर्ट पढ्न सकिएन: { $error }
feedback-delete-failed = क्र्यास रिपोर्ट मेटाउन सकिएन: { $error }
feedback-delete-all-failed = क्र्यास रिपोर्टहरू मेटाउन सकिएन: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _फाइल
desktop-menu-new-message = _नयाँ सन्देश
desktop-menu-quit = _बाहिरिनुहोस्
desktop-menu-edit = _सम्पादन
desktop-menu-undo = _पूर्ववत गर्नुहोस्
desktop-menu-select-all = _सबै चयन गर्नुहोस्
desktop-menu-select-none = _कुनै पनि चयन नगर्नुहोस्
desktop-menu-find = _खोज्नुहोस्…
desktop-menu-view = _दृश्य
desktop-menu-folder-list = _फोल्डर सूची देखाउनुहोस्
desktop-menu-refresh = _रिफ्रेस गर्नुहोस्
desktop-menu-go = _जानुहोस्
desktop-menu-inbox = _इनबक्स
desktop-menu-starred = _तारा लगाइएको
desktop-menu-sent = _पठाइएको
desktop-menu-drafts = _ड्राफ्टहरू
desktop-menu-all-mail = _सबै मेल
desktop-menu-next = _अर्को वार्तालाप
desktop-menu-previous = _अघिल्लो वार्तालाप
desktop-menu-message = _सन्देश
desktop-menu-open = _खोल्नुहोस्
desktop-menu-reply = _जवाफ दिनुहोस्
desktop-menu-reply-all = _सबैलाई जवाफ दिनुहोस्
desktop-menu-forward = _फर्वार्ड गर्नुहोस्
desktop-menu-archive = _संग्रह गर्नुहोस्
desktop-menu-delete = _मेटाउनुहोस्
desktop-menu-spam = _स्प्याम भनी रिपोर्ट गर्नुहोस्
desktop-menu-move-to = _यहाँ सार्नुहोस्…
desktop-menu-mark-read = _पढिएको भनी चिन्ह लगाउनुहोस्
desktop-menu-mark-unread = _नपढिएको भनी चिन्ह लगाउनुहोस्
desktop-menu-star = _तारा लगाउनुहोस्
desktop-menu-important = _महत्त्वपूर्ण भनी चिन्ह लगाउनुहोस्
desktop-menu-not-important = _महत्त्वपूर्ण होइन भनी चिन्ह लगाउनुहोस्
desktop-menu-settings = _सेटिङहरू
desktop-menu-quick-settings = _द्रुत सेटिङहरू
desktop-menu-configure = _Katna Mail कन्फिगर गर्नुहोस्…
desktop-menu-help = _मद्दत
desktop-menu-shortcuts = _किबोर्ड सर्टकटहरू
desktop-menu-whats-new = _नयाँ के छ
desktop-menu-about = _Katna को बारेमा

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = नेभिगेसन
shortcut-group-actions = कार्यहरू
shortcut-group-go-to = यहाँ जानुहोस्
shortcut-group-app = एप्लिकेसन

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = अर्को वार्तालाप
shortcut-previous = अघिल्लो वार्तालाप
shortcut-down = सूचीमा तल जानुहोस्
shortcut-up = सूचीमा माथि जानुहोस्
shortcut-first = सूचीको पहिलो
shortcut-last = सूचीको अन्तिम
shortcut-page-down = सूचीमा एक पाना तल
shortcut-page-up = सूचीमा एक पाना माथि
shortcut-open = वार्तालाप खोल्नुहोस्
shortcut-back = सूचीमा फर्कनुहोस्
shortcut-scroll-down = तल स्क्रोल गर्नुहोस्
shortcut-scroll-up = माथि स्क्रोल गर्नुहोस्
shortcut-scroll-page-down = एक पाना तल स्क्रोल गर्नुहोस्
shortcut-scroll-page-up = एक पाना माथि स्क्रोल गर्नुहोस्
shortcut-compose = रचना गर्नुहोस्
shortcut-reply = जवाफ दिनुहोस्
shortcut-reply-all = सबैलाई जवाफ दिनुहोस्
shortcut-forward = फर्वार्ड गर्नुहोस्
shortcut-archive = संग्रह गर्नुहोस्
shortcut-delete = मेटाउनुहोस्
shortcut-spam = स्प्याम भनी रिपोर्ट गर्नुहोस्
shortcut-move-to = यहाँ सार्नुहोस्
shortcut-mark-read = पढिएको भनी चिन्ह लगाउनुहोस्
shortcut-mark-unread = नपढिएको भनी चिन्ह लगाउनुहोस्
shortcut-star = तारा लगाउनुहोस् वा हटाउनुहोस्
shortcut-important = महत्त्वपूर्ण भनी चिन्ह लगाउनुहोस्
shortcut-not-important = महत्त्वपूर्ण होइन भनी चिन्ह लगाउनुहोस्
shortcut-check = वार्तालापमा टिक लगाउनुहोस्
shortcut-select-all = सबै वार्तालापमा टिक लगाउनुहोस्
shortcut-select-none = सबै वार्तालापबाट टिक हटाउनुहोस्
shortcut-undo = अन्तिम कार्य पूर्ववत गर्नुहोस्
shortcut-go-inbox = इनबक्स
shortcut-go-starred = तारा लगाइएको
shortcut-go-sent = पठाइएको
shortcut-go-drafts = ड्राफ्टहरू
shortcut-go-all = सबै मेल
shortcut-search = मेल खोज्नुहोस्
shortcut-navigation = मेनु देखाउनुहोस् वा लुकाउनुहोस्
shortcut-quick-settings = द्रुत सेटिङहरू
shortcut-settings = सबै सेटिङहरू
shortcut-shortcuts = किबोर्ड सर्टकटहरू
shortcut-reload = नयाँ मेल जाँच गर्नुहोस्
shortcut-quit = बाहिरिनुहोस्

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } त्यसपछि { $second }

## Settings > Accounts

accounts-folder-pane = फोल्डर प्यान
accounts-folder-pane-detail = बायाँको प्यानले कुन खाताहरूका फोल्डर देखाउँछ।
accounts-shown-one = एक पटकमा एउटा खाता; खाता कार्डमा बदल्नुहोस्
accounts-shown-all = सबै खाताहरू, एकपछि अर्को
accounts-row = खाताहरू
accounts-row-detail = खाता हटाउँदा यो कम्प्युटरमा भएको यसको मेलको Katna को प्रतिलिपि मेटिन्छ। मेल सर्भरमै रहन्छ।
accounts-none = अहिलेसम्म कुनै खाता छैन।
accounts-kind-imported = आयात गरिएको
accounts-picture-reset = डेस्कटपको तस्बिर प्रयोग गर्नुहोस्
accounts-picture-change = तस्बिर बदल्नुहोस्
accounts-remove = हटाउनुहोस्
accounts-delete-all-row = सबै डेटा मेटाउनुहोस्
accounts-delete-all-row-detail = नयाँ स्थापनाजस्तै फेरि सुरु गर्नुहोस्।
accounts-delete-all-about = यो कम्प्युटरबाट प्रत्येक खाता, सबै भण्डार गरिएका मेल, सम्पर्क र पात्रोहरू, खोज अनुक्रमणिका, तपाईंका सेटिङहरू र सेभ गरिएका पासवर्डहरू मेटाउँछ। तपाईंका मेल सर्भरहरूमा केही पनि बदलिँदैन।
accounts-delete-all-open = Katna का सबै डेटा मेटाउनुहोस्

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } लाई Katna बाट हटाइयो।
accounts-removed = { $address } लाई Katna बाट हटाइयो। यसको मेल अझै सर्भरमा छ।
accounts-all-deleted = Katna का सबै डेटा यो कम्प्युटरबाट मेटाइयो।

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = { $address } हटाउने हो?
accounts-remove-confirm = खाता हटाउनुहोस्
accounts-removing = हटाउँदै…
accounts-remove-local-mail = { $folders ->
    [0] यो खातामा आयात गरिएका सबै मेल
    [one] यो खातामा आयात गरिएका सबै मेल, यसको फोल्डरमा
   *[other] यो खातामा आयात गरिएका सबै मेल, यसका { $folders } फोल्डरहरूमा
}
accounts-remove-local-settings = यसका Katna सेटिङहरू
accounts-remove-mail = { $folders ->
    [0] Katna ले भण्डार गरेका यो खाताका सबै मेल
    [one] Katna ले भण्डार गरेका यो खाताका सबै मेल, यसको फोल्डरमा
   *[other] Katna ले भण्डार गरेका यो खाताका सबै मेल, यसका { $folders } फोल्डरहरूमा
}
accounts-remove-outbox = आउटबक्समा पर्खिरहेका यसका सन्देशहरू
accounts-remove-settings = यसको सेभ गरिएको पासवर्ड र यसका Katna सेटिङहरू
accounts-delete-all-title = Katna का सबै डेटा मेटाउने हो?
accounts-delete-all-confirm = सबै कुरा मेटाउनुहोस्
accounts-deleting = मेटाउँदै…
accounts-delete-all-accounts = प्रत्येक खाता, र Katna ले भण्डार गरेका सबै मेल र संलग्नकहरू
accounts-delete-all-contacts = सम्पर्क, पात्रोहरू र खोज अनुक्रमणिका
accounts-delete-all-settings = सबै सेटिङहरू, हस्ताक्षरहरू र किबोर्ड सर्टकटहरू
accounts-delete-all-passwords = सेभ गरिएका प्रत्येक पासवर्ड
accounts-deleted-heading = यो कम्प्युटरबाट मेटाइने:
accounts-cannot-undo = यसलाई पूर्ववत गर्न सकिँदैन।
accounts-server-delete-all = तपाईंका मेल सर्भरहरूमा केही पनि बदलिँदैन: तपाईंको मेल त्यहीँ रहन्छ, र फेरि खाता थप्दा त्यो फेरि डाउनलोड हुन्छ। फाइलहरूबाट आयात गरिएको मेल Katna मा मात्र छ; ती फाइलहरू छोइँदैनन्।
accounts-server-local = यो मेल फाइलहरूबाट आयात गरिएको थियो, त्यसैले यसको एकमात्र प्रतिलिपि Katna सँग छ। यो आएका फाइलहरू छोइँदैनन्; यसलाई फिर्ता पाउन ती फेरि आयात गर्नुहोस्।
accounts-server-remove = मेल सर्भरमा केही पनि बदलिँदैन: तपाईंको मेल त्यहीँ रहन्छ, र फेरि खाता थप्दा त्यो फेरि डाउनलोड हुन्छ।
accounts-confirm-word = मेटाउनुहोस्
accounts-confirm-placeholder = “{ accounts-confirm-word }” टाइप गर्नुहोस्
accounts-confirm-prompt = पुष्टि गर्न “{ accounts-confirm-word }” टाइप गर्नुहोस्:
accounts-cancel = रद्द गर्नुहोस्
