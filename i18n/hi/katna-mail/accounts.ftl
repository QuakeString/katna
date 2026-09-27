# Katna Mail, Hindi (हिन्दी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = फ़ोल्डर पेन
accounts-folder-pane-detail = बाईं ओर का पेन किन खातों के फ़ोल्डर दिखाए।
accounts-shown-one = एक बार में एक खाता; खाता कार्ड में बदलें
accounts-shown-all = सभी खाते, एक के बाद एक
accounts-row = खाते
accounts-row-detail = फ़ोल्डर पेन और खाता मेन्यू खातों को इसी क्रम में दिखाते हैं; पहला खाता डिफ़ॉल्ट है। कोई खाता हटाने पर इस कंप्यूटर पर मौजूद उसके मेल की Katna वाली कॉपी मिट जाती है। मेल सर्वर पर बना रहता है।
accounts-none = अभी तक कोई खाता नहीं है।
accounts-kind-imported = इंपोर्ट किया गया
accounts-picture-reset = डेस्कटॉप की तस्वीर इस्तेमाल करें
accounts-picture-change = तस्वीर बदलें
accounts-picture-remove = तस्वीर हटाएं
accounts-rename = नाम बदलें
accounts-name-save = सेव करें
accounts-name-cancel = रद्द करें
accounts-name-placeholder = आपका नाम
accounts-rename-failed = खाते का नाम नहीं बदला जा सका: { $error }
accounts-move-up = ऊपर ले जाएं
accounts-move-down = नीचे ले जाएं
accounts-drag = क्रम बदलने के लिए खींचें
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
reset-cache-about = Katna का डाउनलोड किया गया मेल और अटैचमेंट, भेजने वालों की तस्वीरें और खोज इंडेक्स मिटा देता है, फिर हाल का मेल दोबारा डाउनलोड करता है। खाते, सेटिंग और सिर्फ़ इस कंप्यूटर पर मौजूद मेल बने रहते हैं।
reset-cache-button = कैश रीसेट करें
reset-cache-title = कैश रीसेट करें?
reset-cache-deleted = मिटाया जाएगा, फिर दोबारा डाउनलोड होगा:
reset-cache-mail = आपके IMAP सर्वर से डाउनलोड किया गया मेल और अटैचमेंट: हाल का मेल अभी दोबारा डाउनलोड होता है, पुराना मेल खोलने पर
reset-cache-index = खोज इंडेक्स, जो तुरंत दोबारा बनाया जाता है
reset-cache-pictures = भेजने वालों की तस्वीरें
reset-cache-kept = बना रहता है: आपके खाते, पासवर्ड और सेटिंग; तारे, लेबल, पढ़े जाने के निशान और पिन; ड्राफ़्ट, आउटबॉक्स और वे बदलाव जो अभी सर्वर तक नहीं पहुंचे; और POP3 खातों या इंपोर्ट की गई फ़ाइलों का मेल, जिसकी शायद कोई और कॉपी न हो। आपके मेल सर्वर पर कुछ नहीं बदलता।
reset-cache-confirm = कैश रीसेट करें
reset-cache-busy = रीसेट हो रहा है…
reset-cache-done = कैश रीसेट हो गया। हाल का मेल दोबारा डाउनलोड हो रहा है।
reset-cache-done-freed = कैश रीसेट हो गया और { $size } जगह खाली हुई। हाल का मेल दोबारा डाउनलोड हो रहा है।
