# Katna Mail, Nepali (नेपाली).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = लेबलहरू
nav-folders = फोल्डरहरू
nav-label-new = नयाँ लेबल बनाउनुहोस्
nav-folder-new = नयाँ फोल्डर बनाउनुहोस्
nav-menu-check-mail = नयाँ मेल जाँच गर्नुहोस्
nav-menu-check-inbox = यो इनबक्स जाँच गर्नुहोस्
nav-unified-leave-out = एकीकृत इनबक्सबाट बाहिर राख्नुहोस्
nav-unified-bring-back = एकीकृत इनबक्समा फिर्ता ल्याउनुहोस्
nav-menu-sign-in-again = फेरि साइन इन गर्नुहोस्
nav-menu-new-mail = यो खाताबाट नयाँ मेल
nav-menu-account-settings = खाता सेटिङहरू
nav-account-checked = सिंक भएको · { $ago } जाँचियो
nav-account-in-sync = सिंक भएको
nav-account-connecting = जडान हुँदै…
nav-account-offline = अफलाइन, फेरि प्रयास गर्दै
nav-account-signed-out = { $provider } साइन इनको म्याद सकियो
nav-account-password-refused = पासवर्ड अस्वीकार गरियो
nav-account-storage = { $total } मध्ये { $used } प्रयोग भयो
nav-menu-new-subfolder = भित्र नयाँ फोल्डर
nav-menu-new-sublabel = भित्र नयाँ लेबल
nav-menu-rename = नाम बदल्नुहोस्
nav-menu-delete = मेटाउनुहोस्
nav-menu-empty-trash = ट्र्यास खाली गर्नुहोस्
nav-account-unnamed = खाता { $number }
nav-all-accounts = सबै खाताहरू
nav-expand = फोल्डरहरू देखाउनुहोस्
nav-collapse = फोल्डरहरू लुकाउनुहोस्
storage-used = { $total } मध्ये { $percent }% प्रयोग भयो
storage-used-detail = { $address }: { $total } मध्ये { $used } प्रयोग भयो

## Special folders (the user's own folders keep their names)

folder-inbox = इनबक्स
folder-starred = तारा लगाइएको
folder-snoozed = स्नुज गरिएको
folder-unread = नपढिएको
folder-important = महत्त्वपूर्ण
folder-drafts = ड्राफ्टहरू
folder-sent = पठाइएको
folder-archive = संग्रह
folder-spam = स्प्याम
folder-trash = ट्र्यास
folder-all-mail = सबै मेल
folder-scheduled = तालिकाबद्ध गरिएको
folder-waiting = जवाफको पर्खाइमा
folder-waiting-short = पर्खाइमा
folder-reminders = रिमाइन्डरहरू
folder-outbox = आउटबक्स
folder-activity = गतिविधि
folder-not-on-account = यो खातामा त्यस्तो फोल्डर छैन।

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
label-rename-title = लेबलको नाम बदल्नुहोस्
label-folder-rename-title = फोल्डरको नाम बदल्नुहोस्
label-rename = नाम बदल्नुहोस्
label-renaming = नाम बदल्दै…
label-renamed = लेबलको नाम बदलेर “{ $name }” बनाइयो।
label-folder-renamed = फोल्डरको नाम बदलेर “{ $name }” बनाइयो।

## Deleting a folder or label (asked first)

folder-delete-title = “{ $name }” मेटाउने?
folder-delete-body = { $count ->
    [0] यसमा कुनै मेल छैन। फोल्डर सर्भरबाट हटाइन्छ, त्यसैले वेबमेल र तपाईंको फोनबाट पनि यो हराउँछ।
   *[other] { $kind ->
        [conversation] { $count ->
            [one] यसको { $count } वार्तालाप ट्र्यासमा जान्छ, त्यसैले तपाईं अझै यसलाई फिर्ता ल्याउन सक्नुहुन्छ।
           *[other] यसका { $count } वार्तालापहरू ट्र्यासमा जान्छन्, त्यसैले तपाईं अझै तिनलाई फिर्ता ल्याउन सक्नुहुन्छ।
        }
       *[message] { $count ->
            [one] यसको { $count } सन्देश ट्र्यासमा जान्छ, त्यसैले तपाईं अझै यसलाई फिर्ता ल्याउन सक्नुहुन्छ।
           *[other] यसका { $count } सन्देशहरू ट्र्यासमा जान्छन्, त्यसैले तपाईं अझै तिनलाई फिर्ता ल्याउन सक्नुहुन्छ।
        }
    } फोल्डर सर्भरबाट हटाइन्छ, त्यसैले वेबमेल र तपाईंको फोनबाट पनि यो हराउँछ।
}
folder-delete-forever-body = { $count ->
    [0] यसमा कुनै मेल छैन। फोल्डर सर्भरबाट हटाइन्छ, त्यसैले वेबमेल र तपाईंको फोनबाट पनि यो हराउँछ।
   *[other] { $kind ->
        [conversation] { $count ->
            [one] यसको { $count } वार्तालाप सधैँका लागि मेटाइन्छ; यो खातामा ट्र्यास छैन।
           *[other] यसका { $count } वार्तालापहरू सधैँका लागि मेटाइन्छन्; यो खातामा ट्र्यास छैन।
        }
       *[message] { $count ->
            [one] यसको { $count } सन्देश सधैँका लागि मेटाइन्छ; यो खातामा ट्र्यास छैन।
           *[other] यसका { $count } सन्देशहरू सधैँका लागि मेटाइन्छन्; यो खातामा ट्र्यास छैन।
        }
    } फोल्डर सर्भरबाट हटाइन्छ, त्यसैले वेबमेल र तपाईंको फोनबाट पनि यो हराउँछ।
}
folder-delete-label-body = लेबल हटाइन्छ। यसको मेल सबै मेल र यसका अन्य लेबलहरूमा रहन्छ।
folder-delete-confirm = फोल्डर मेटाउनुहोस्
folder-delete-label-confirm = लेबल मेटाउनुहोस्
folder-deleted = फोल्डर “{ $name }” मेटाइयो
label-deleted = लेबल “{ $name }” मेटाइयो
