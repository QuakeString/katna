# Katna Mail, Nepali (नेपाली).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = फोल्डर प्यान
accounts-folder-pane-detail = बायाँको प्यानले कुन खाताहरूका फोल्डर देखाउँछ।
accounts-shown-one = एक पटकमा एउटा खाता; खाता कार्डमा बदल्नुहोस्
accounts-shown-all = सबै खाताहरू, एकपछि अर्को
accounts-unified = एकीकृत इनबक्स
accounts-unified-switch = सबै खाताहरूका मेल एकैसाथ देखाउनुहोस्
accounts-unified-switch-detail = “सबै खाताहरू” फोल्डर प्यानको सबैभन्दा माथि हुन्छ, जसमा हरेक खाताको इनबक्स, पठाइएका मेल र थप कुराहरू एउटै सूचीमा हुन्छन्। यसको तलका खाताहरू सुरुमा खुम्चिएका हुन्छन्।
accounts-row = खाताहरू
accounts-row-detail = फोल्डर प्यान र खाता मेनुले खाताहरू यही क्रममा देखाउँछन्; पहिलो खाता पूर्वनिर्धारित हो। खाता हटाउँदा यो कम्प्युटरमा भएको यसको मेलको Katna को प्रतिलिपि मेटिन्छ। मेल सर्भरमै रहन्छ।
accounts-none = अहिलेसम्म कुनै खाता छैन।
accounts-pop3-row = सर्भरमा रहेको मेल
accounts-pop3-row-detail = POP3 खाताहरूले मेल यो कम्प्युटरमा डाउनलोड गर्छन्। त्यसपछि सर्भरमा रहेको प्रतिलिपिलाई के गर्ने, छान्नुहोस्।
accounts-pop3-with-katna = Katna मा मैले नमेटाएसम्म राख्नुहोस्
accounts-pop3-at-once = डाउनलोड भएपछि मेटाउनुहोस्
accounts-pop3-after-days = { $count ->
    [one] { $count } दिनपछि मेटाउनुहोस्
   *[other] { $count } दिनपछि मेटाउनुहोस्
}
accounts-pop3-never = कहिल्यै नमेटाउनुहोस्
accounts-pop3-days-less = कम दिन
accounts-pop3-days-more = थप दिन
accounts-kind-imported = आयात गरिएको
accounts-picture-reset = डेस्कटपको तस्बिर प्रयोग गर्नुहोस्
accounts-picture-change = तस्बिर बदल्नुहोस्
accounts-picture-remove = तस्बिर हटाउनुहोस्
accounts-rename = नाम बदल्नुहोस्
accounts-name-save = सेभ गर्नुहोस्
accounts-name-cancel = रद्द गर्नुहोस्
accounts-name-placeholder = तपाईंको नाम
accounts-rename-failed = खाताको नाम बदल्न सकिएन: { $error }
accounts-move-up = माथि सार्नुहोस्
accounts-move-down = तल सार्नुहोस्
accounts-drag = क्रम बदल्न तान्नुहोस्
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
reset-cache-about = Katna ले डाउनलोड गरेका मेल र संलग्नकहरू, प्रेषकका तस्बिरहरू र खोज अनुक्रमणिका मेटाउँछ, अनि हालका मेल फेरि डाउनलोड गर्छ। खाताहरू, सेटिङहरू र यो कम्प्युटरमा मात्र भएका मेल रहन्छन्।
reset-cache-button = क्यास रिसेट गर्नुहोस्
reset-cache-title = क्यास रिसेट गर्ने?
reset-cache-deleted = मेटाइन्छ, अनि फेरि डाउनलोड हुन्छ:
reset-cache-mail = तपाईंका IMAP सर्भरहरूबाट डाउनलोड गरिएका मेल र संलग्नकहरू: हालका मेल अहिल्यै फेरि डाउनलोड हुन्छन्, पुराना मेल तपाईंले खोल्दा
reset-cache-index = खोज अनुक्रमणिका, जुन तुरुन्तै फेरि बनाइन्छ
reset-cache-pictures = प्रेषकका तस्बिरहरू
reset-cache-kept = रहन्छन्: तपाईंका खाताहरू, पासवर्डहरू र सेटिङहरू; ताराहरू, लेबलहरू, पढिएका चिन्हहरू र पिनहरू; ड्राफ्टहरू, आउटबक्स र सर्भरमा पुगिनसकेका परिवर्तनहरू; र POP3 खाताहरू वा आयात गरिएका फाइलहरूका मेल, जसको अर्को प्रति नहुन सक्छ। तपाईंका मेल सर्भरहरूमा केही पनि बदलिँदैन।
reset-cache-confirm = क्यास रिसेट गर्नुहोस्
reset-cache-busy = रिसेट गर्दै…
reset-cache-done = क्यास रिसेट भयो। हालका मेल फेरि डाउनलोड हुँदैछन्।
reset-cache-done-freed = क्यास रिसेट भयो र { $size } ठाउँ खाली भयो। हालका मेल फेरि डाउनलोड हुँदैछन्।
