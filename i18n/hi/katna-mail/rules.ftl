# Katna Mail, Hindi (हिन्दी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = नियम
settings-rules-summary = नए मेल को अपने-आप छांटें, लेबल करें, फ़ॉरवर्ड करें या चुप करें
settings-rules-intro = नियम नए मेल को इसी क्रम में अपने-आप छांटते हैं। क्रम बदलने के लिए खींचें।
settings-rules-all-accounts = सभी खाते
settings-rules-new = नया नियम
settings-rules-none = अभी कोई नियम नहीं। नियम नए मेल को भेजने वाले, विषय या शब्दों के हिसाब से अपने-आप छांटता है।
settings-rules-none-account = इस खाते के लिए अभी कोई नियम नहीं।
settings-rules-drag = क्रम बदलने के लिए खींचें
settings-rules-edit = नियम में बदलाव करें
settings-rules-turn-off = यह नियम बंद करें
settings-rules-turn-on = यह नियम चालू करें

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = शुरुआती नियम
settings-rules-starters-intro = जब तक आप चालू न करें, बंद रहते हैं। ये आपके सभी खातों के लिए काम करते हैं; बदलने के लिए किसी में बदलाव करें।
settings-rules-starter-turning-on = “{ $name }” चालू किया जा रहा है…
settings-rules-starter-failed = “{ $name }” चालू नहीं किया जा सका: { $error }
rules-starter-promotions = प्रमोशन को चुप करें
rules-starter-newsletters = न्यूज़लेटर “पढ़ने के लिए” में
rules-starter-receipts = रसीदें और इनवॉइस
rules-starter-deliveries = डिलीवरी
rules-starter-train = ट्रेन टिकट
rules-starter-flight = फ़्लाइट टिकट
rules-starter-codes = वन-टाइम कोड
rules-starter-security = सुरक्षा अलर्ट
rules-starter-social = सामाजिक मेल
rules-starter-invites = कैलेंडर आमंत्रण
rules-starter-folder-reading = पढ़ने के लिए
rules-starter-folder-receipts = रसीदें
rules-starter-folder-deliveries = डिलीवरी
rules-starter-folder-travel = यात्रा
rules-starter-folder-social = सामाजिक
rules-runs-katna = Katna में चलता है
rules-runs-gmail = Gmail पर चलता है
rules-runs-sieve = सर्वर पर चलता है
rules-stopped = रुका हुआ
rules-error-folder-gone = यह नियम जिस फ़ोल्डर का उपयोग करता है, वह अब मौजूद नहीं है। दूसरा चुनने के लिए नियम में बदलाव करें।
rules-error-no-archive = इस खाते में संग्रह फ़ोल्डर नहीं है। कुछ और करने के लिए नियम में बदलाव करें।
rules-error-no-trash = इस खाते में ट्रैश फ़ोल्डर नहीं है। कुछ और करने के लिए नियम में बदलाव करें।
rules-error-cannot-send = यह खाता मेल नहीं भेज सकता, इसलिए नियम उसे फ़ॉरवर्ड नहीं कर सकता।
rules-error-other = { $error }। नियम में बदलाव करें और उसे फिर से चालू करें।

settings-folders = फ़ोल्डर
settings-folders-summary = फ़ोल्डर पैनल में न पढ़े गए की गिनती
settings-folders-unread-counts = हर फ़ोल्डर पर न पढ़े गए की गिनती
settings-folders-unread-counts-detail = बंद: सिर्फ़ इनबॉक्स दिखाता है कि कितने नहीं पढ़े गए

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } और { $next }
rules-summary-or = { $first } या { $next }
rules-summary-more = { $count } और
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = अटैचमेंट है
rules-summary-no-attachment = अटैचमेंट नहीं है
rules-summary-mailing-list = मेलिंग सूची से
rules-summary-not-mailing-list = मेलिंग सूची से नहीं
rules-summary-tab = { $tab } टैब में
rules-summary-not-tab = { $tab } टैब में नहीं
rules-summary-move = { $folder } में ले जाएं
rules-summary-archive = इनबॉक्स छोड़ें
rules-summary-trash = ट्रैश में ले जाएं
rules-summary-mark-read = पढ़ा गया मार्क करें
rules-summary-star = तारांकित करें
rules-summary-important = ज़रूरी मार्क करें
rules-summary-label = लेबल { $label }
rules-summary-forward = { $address } पर फ़ॉरवर्ड करें
rules-summary-dont-notify = सूचना न दें
rules-summary-read-after = { $count ->
    [one] { $count } दिन बाद पढ़ा गया मार्क करें
   *[other] { $count } दिन बाद पढ़ा गया मार्क करें
}
rules-summary-folder-gone = एक फ़ोल्डर जो अब नहीं है

## The rule editor

rules-editor-new-title = नया नियम
rules-editor-edit-title = नियम में बदलाव करें
rules-editor-name-hint = नियम का नाम
rules-editor-when = जब कोई नया मेल इनमें से
rules-editor-of-these = से मेल खाए:
rules-mode-all = सभी
rules-mode-any = किसी भी
rules-field-from = भेजने वाला
rules-field-to = पाने वाले
rules-field-cc = Cc
rules-field-any-recipient = पाने वाले या Cc
rules-field-reply-to = जवाब का पता
rules-field-subject = विषय
rules-field-body = टेक्स्ट
rules-field-attachment-name = अटैचमेंट का नाम
rules-field-has-attachment = अटैचमेंट है
rules-field-mailing-list = मेलिंग सूची से
rules-field-tab = इनबॉक्स टैब
rules-comparator-contains = में शामिल है
rules-comparator-not-contains = में शामिल नहीं है
rules-comparator-begins-with = से शुरू होता है
rules-comparator-ends-with = पर खत्म होता है
rules-comparator-equals = ठीक यही है
rules-comparator-matches = पैटर्न से मेल खाता है
rules-has-yes = हां
rules-has-no = नहीं
rules-editor-value-hint = शब्द या पता
rules-editor-add-condition = शर्त जोड़ें
rules-editor-remove = हटाएं
rules-editor-then = तब:
rules-action-move = इसमें ले जाएं
rules-action-archive = इनबॉक्स छोड़ें (संग्रह करें)
rules-action-trash = ट्रैश में ले जाएं
rules-action-mark-read = पढ़ा गया मार्क करें
rules-action-star = तारांकित करें
rules-action-important = ज़रूरी मार्क करें
rules-action-label = लेबल जोड़ें
rules-action-forward = इस पर फ़ॉरवर्ड करें
rules-action-dont-notify = सूचना न दें
rules-action-read-after = इसके बाद पढ़ा गया मार्क करें
rules-editor-choose-folder = फ़ोल्डर चुनें
rules-editor-choose-label = लेबल चुनें
rules-editor-new-folder = नया: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = ईमेल पता
rules-editor-days = दिन
rules-editor-add-action = कार्रवाई जोड़ें
rules-editor-stop = यहीं रुकें: बाद के नियम इस मेल पर नहीं चलते
rules-editor-accounts = खाते:
rules-editor-accounts-none = खाते चुनें
rules-editor-accounts-many = { $count ->
    [one] { $count } खाता
   *[other] { $count } खाते
}
rules-editor-matches = पिछले { $days } दिनों के { $mails } से मेल खाता है
rules-editor-mails = { $count ->
    [one] { $count } मेल
   *[other] { $count } मेल
}
rules-editor-counting = मेल खाने वाले मेल गिने जा रहे हैं…
rules-editor-show = उन्हें दिखाएं
rules-editor-also-apply = इन { $count } पर भी लागू करें
rules-editor-runs-katna = Katna में चलता है, जब तक यह कंप्यूटर चालू है।
rules-editor-runs-gmail = Gmail पर चलता है, इसलिए आपके फ़ोन पर और इस कंप्यूटर के बंद होने पर भी काम करता है।
rules-editor-runs-sieve = आपके मेल सर्वर पर चलता है, इसलिए आपके फ़ोन पर और इस कंप्यूटर के बंद होने पर भी काम करता है।
rules-note-gmail-action = Katna में चलता है: Gmail फ़िल्टर “{ $action }” नहीं कर सकते।
rules-note-sieve-action = Katna में चलता है: आपके मेल सर्वर के नियम “{ $action }” नहीं कर सकते।
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = Katna में चलता है: Gmail फ़िल्टर “{ $test }” को Katna की तरह नहीं जाँच सकते।
rules-note-sieve-condition = Katna में चलता है: आपके मेल सर्वर के नियम “{ $test }” को Katna की तरह नहीं जाँच सकते।
rules-note-order = Katna में चलता है, क्योंकि खाते का एक पहले वाला नियम भी वहीं चलता है: नियम सूची के क्रम में चलते हैं।
rules-note-gmail-stop = Katna में चलता है: Gmail फ़िल्टर बाद के नियमों को चलने से नहीं रोक सकते।
rules-note-gmail-forward = Katna में चलता है: Gmail सिर्फ़ अपनी सेटिंग में सत्यापित पतों पर फ़ॉरवर्ड करता है, और { $address } उनमें से नहीं है।
rules-note-gmail-folder = Katna में चलता है: इस नियम के एक फ़ोल्डर के लिए Gmail में कोई लेबल नहीं है।
rules-note-sieve-folder = Katna में चलता है: इस नियम का एक फ़ोल्डर आपके मेल सर्वर पर नहीं है।
rules-note-gmail-sign-in = Katna में चलता है, जब तक आप Google में फिर से साइन इन करके Katna को Gmail फ़िल्टर बनाने की अनुमति नहीं देते।
rules-note-sieve-other-script = Katna में चलता है: आपके मेल सर्वर पर नियमों की दूसरी स्क्रिप्ट (“{ $name }”) चालू है।
rules-note-gmail-failed = Katna में चलता है: Gmail ने इसे स्वीकार नहीं किया ({ $error })।
rules-note-sieve-failed = Katna में चलता है: आपके मेल सर्वर ने इसे स्वीकार नहीं किया ({ $error })।
rules-editor-cancel = रद्द करें
rules-editor-save = सेव करें
rules-editor-saving = सेव किया जा रहा है…
rules-editor-delete = नियम मिटाएं
rules-editor-delete-ask = यह नियम मिटाएं?
rules-editor-delete-keep = रहने दें
rules-editor-delete-confirm = मिटाएं
rules-editor-needs-folder = हर “इसमें ले जाएं” के लिए फ़ोल्डर और हर “लेबल जोड़ें” के लिए लेबल चुनें।
rules-editor-needs-days = “इसके बाद पढ़ा गया मार्क करें” के लिए 1 से 3650 तक दिनों की संख्या चाहिए।
rules-saved = नियम सेव किया गया
rules-saved-applied = { $count ->
    [one] नियम सेव किया गया और { $count } मेल पर लागू किया गया
   *[other] नियम सेव किया गया और { $count } मेल पर लागू किया गया
}
rules-apply-failed = नियम सेव किया गया, लेकिन उसे लागू करना विफल रहा: { $error }
rules-deleted = नियम मिटाया गया
rules-delete-failed = नियम मिटाया नहीं जा सका: { $error }
rules-change-failed = नियम बदले नहीं जा सके: { $error }
