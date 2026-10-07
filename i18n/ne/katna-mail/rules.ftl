# Katna Mail, Nepali (नेपाली).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = नियमहरू
settings-rules-summary = नयाँ मेललाई आफैँ क्रमबद्ध गर्नुहोस्, लेबल लगाउनुहोस्, फर्वार्ड गर्नुहोस् वा शान्त राख्नुहोस्
settings-rules-intro = नियमहरूले नयाँ मेललाई यही क्रममा आफैँ क्रमबद्ध गर्छन्। क्रम बदल्न तान्नुहोस्।
settings-rules-all-accounts = सबै खाताहरू
settings-rules-new = नयाँ नियम
settings-rules-none = अहिलेसम्म कुनै नियम छैन। नियमले नयाँ मेललाई प्रेषक, विषय वा शब्दहरूका आधारमा आफैँ क्रमबद्ध गर्छ।
settings-rules-none-account = यो खाताका लागि अहिलेसम्म कुनै नियम छैन।
settings-rules-drag = क्रम बदल्न तान्नुहोस्
settings-rules-edit = नियम सम्पादन गर्नुहोस्
settings-rules-turn-off = यो नियम बन्द गर्नुहोस्
settings-rules-turn-on = यो नियम खोल्नुहोस्

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = सुरुका नियमहरू
settings-rules-starters-intro = तपाईंले नखोलेसम्म बन्द रहन्छन्। यी तपाईंका सबै खाताहरूमा लागू हुन्छन्; बदल्न कुनै एउटा सम्पादन गर्नुहोस्।
settings-rules-starter-turning-on = “{ $name }” खोल्दै…
settings-rules-starter-failed = “{ $name }” खोल्न सकिएन: { $error }
rules-starter-promotions = शान्त प्रमोसनहरू
rules-starter-newsletters = न्युजलेटरहरू पढाइमा
rules-starter-receipts = रसिद र बिलहरू
rules-starter-deliveries = डेलिभरीहरू
rules-starter-train = रेल टिकटहरू
rules-starter-flight = उडान टिकटहरू
rules-starter-codes = एक पटके कोडहरू
rules-starter-security = सुरक्षा सतर्कताहरू
rules-starter-social = सामाजिक मेल
rules-starter-invites = पात्रोका निम्तोहरू
rules-starter-folder-reading = पढाइ
rules-starter-folder-receipts = रसिदहरू
rules-starter-folder-deliveries = डेलिभरीहरू
rules-starter-folder-travel = यात्रा
rules-starter-folder-social = सामाजिक
rules-runs-katna = Katna मा चल्छ
rules-runs-gmail = Gmail मा चल्छ
rules-runs-sieve = सर्भरमा चल्छ
rules-stopped = रोकिएको
rules-error-folder-gone = यो नियमले प्रयोग गर्ने फोल्डर अब छैन। अर्को छान्न नियम सम्पादन गर्नुहोस्।
rules-error-no-archive = यो खातामा संग्रह फोल्डर छैन। अरू केही गर्न नियम सम्पादन गर्नुहोस्।
rules-error-no-trash = यो खातामा ट्र्यास फोल्डर छैन। अरू केही गर्न नियम सम्पादन गर्नुहोस्।
rules-error-cannot-send = यो खाताले मेल पठाउन सक्दैन, त्यसैले नियमले यसलाई फर्वार्ड गर्न सक्दैन।
rules-error-other = { $error }। नियम सम्पादन गरेर फेरि खोल्नुहोस्।

settings-folders = फोल्डरहरू
settings-folders-summary = फोल्डर प्यानमा नपढिएका सङ्ख्या
settings-folders-unread-counts = हरेक फोल्डरमा नपढिएका सङ्ख्या
settings-folders-unread-counts-detail = बन्द: इनबक्सले मात्र कति नपढिएका छन् देखाउँछ

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } र { $next }
rules-summary-or = { $first } वा { $next }
rules-summary-more = थप { $count }
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = संलग्नक छ
rules-summary-no-attachment = संलग्नक छैन
rules-summary-mailing-list = मेलिङ सूचीबाट
rules-summary-not-mailing-list = मेलिङ सूचीबाट होइन
rules-summary-tab = { $tab } ट्याबमा
rules-summary-not-tab = { $tab } ट्याबमा होइन
rules-summary-move = { $folder } मा सार्नुहोस्
rules-summary-archive = इनबक्स छोड्नुहोस्
rules-summary-trash = ट्र्यासमा सार्नुहोस्
rules-summary-mark-read = पढिएको भनी चिन्ह लगाउनुहोस्
rules-summary-star = तारा लगाउनुहोस्
rules-summary-important = महत्त्वपूर्ण भनी चिन्ह लगाउनुहोस्
rules-summary-label = { $label } लेबल लगाउनुहोस्
rules-summary-forward = { $address } मा फर्वार्ड गर्नुहोस्
rules-summary-dont-notify = सूचना नदिनुहोस्
rules-summary-read-after = { $count ->
    [one] { $count } दिनपछि पढिएको भनी चिन्ह लगाउनुहोस्
   *[other] { $count } दिनपछि पढिएको भनी चिन्ह लगाउनुहोस्
}
rules-summary-folder-gone = हटिसकेको फोल्डर

## The rule editor

rules-editor-new-title = नयाँ नियम
rules-editor-edit-title = नियम सम्पादन गर्नुहोस्
rules-editor-name-hint = नियमको नाम
rules-editor-when = नयाँ मेल मिल्दा
rules-editor-of-these = यीमध्ये:
rules-mode-all = सबै
rules-mode-any = कुनै पनि
rules-field-from = प्रेषक
rules-field-to = प्रापक
rules-field-cc = Cc
rules-field-any-recipient = प्रापक वा Cc
rules-field-reply-to = जवाफ ठेगाना
rules-field-subject = विषय
rules-field-body = पाठ
rules-field-attachment-name = संलग्नकको नाम
rules-field-has-attachment = संलग्नक छ
rules-field-mailing-list = मेलिङ सूचीबाट
rules-field-tab = इनबक्स ट्याब
rules-comparator-contains = समावेश छ
rules-comparator-not-contains = समावेश छैन
rules-comparator-begins-with = बाट सुरु हुन्छ
rules-comparator-ends-with = मा अन्त्य हुन्छ
rules-comparator-equals = ठ्याक्कै यही हो
rules-comparator-matches = ढाँचासँग मिल्छ
rules-has-yes = हो
rules-has-no = होइन
rules-editor-value-hint = शब्द वा ठेगाना
rules-editor-add-condition = सर्त थप्नुहोस्
rules-editor-remove = हटाउनुहोस्
rules-editor-then = त्यसपछि:
rules-action-move = यहाँ सार्नुहोस्
rules-action-archive = इनबक्स छोड्नुहोस् (संग्रह)
rules-action-trash = ट्र्यासमा सार्नुहोस्
rules-action-mark-read = पढिएको भनी चिन्ह लगाउनुहोस्
rules-action-star = तारा लगाउनुहोस्
rules-action-important = महत्त्वपूर्ण भनी चिन्ह लगाउनुहोस्
rules-action-label = लेबल थप्नुहोस्
rules-action-forward = यहाँ फर्वार्ड गर्नुहोस्
rules-action-dont-notify = सूचना नदिनुहोस्
rules-action-read-after = यति दिनपछि पढिएको भनी चिन्ह लगाउनुहोस्
rules-editor-choose-folder = फोल्डर छान्नुहोस्
rules-editor-choose-label = लेबल छान्नुहोस्
rules-editor-new-folder = नयाँ: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = इमेल ठेगाना
rules-editor-days = दिन
rules-editor-add-action = कार्य थप्नुहोस्
rules-editor-stop = यहीँ रोक्नुहोस्: पछिका नियमहरू यो मेलमा चल्दैनन्
rules-editor-accounts = खाताहरू:
rules-editor-accounts-none = खाताहरू छान्नुहोस्
rules-editor-accounts-many = { $count ->
    [one] { $count } खाता
   *[other] { $count } खाता
}
rules-editor-matches = पछिल्ला { $days } दिनका { $mails } सँग मिल्छ
rules-editor-mails = { $count ->
    [one] { $count } मेल
   *[other] { $count } मेल
}
rules-editor-counting = मिल्ने मेल गन्दै…
rules-editor-show = तिनलाई देखाउनुहोस्
rules-editor-also-apply = यी { $count } मा पनि लागू गर्नुहोस्
rules-editor-runs-katna = Katna मा चल्छ, यो कम्प्युटर खुला रहँदासम्म।
rules-editor-runs-gmail = Gmail मा चल्छ, त्यसैले तपाईंको फोनमा र यो कम्प्युटर बन्द हुँदा पनि काम गर्छ।
rules-editor-runs-sieve = तपाईंको मेल सर्भरमा चल्छ, त्यसैले तपाईंको फोनमा र यो कम्प्युटर बन्द हुँदा पनि काम गर्छ।
rules-note-gmail-action = Katna मा चल्छ: Gmail फिल्टरहरूले “{ $action }” गर्न सक्दैनन्।
rules-note-sieve-action = Katna मा चल्छ: तपाईंको मेल सर्भरका नियमहरूले “{ $action }” गर्न सक्दैनन्।
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = Katna मा चल्छ: Gmail फिल्टरहरूले “{ $test }” लाई Katna ले जस्तै जाँच्न सक्दैनन्।
rules-note-sieve-condition = Katna मा चल्छ: तपाईंको मेल सर्भरका नियमहरूले “{ $test }” लाई Katna ले जस्तै जाँच्न सक्दैनन्।
rules-note-order = Katna मा चल्छ, किनकि खाताको अघिल्लो नियम पनि त्यहीँ चल्छ: नियमहरू सूचीको क्रममा चल्छन्।
rules-note-gmail-stop = Katna मा चल्छ: Gmail फिल्टरहरूले पछिका नियमहरू चल्नबाट रोक्न सक्दैनन्।
rules-note-gmail-forward = Katna मा चल्छ: Gmail ले आफ्नो सेटिङमा प्रमाणित ठेगानाहरूमा मात्र फर्वार्ड गर्छ, र { $address } तीमध्ये एक होइन।
rules-note-gmail-folder = Katna मा चल्छ: यो नियमले प्रयोग गर्ने फोल्डरका लागि Gmail मा कुनै लेबल छैन।
rules-note-sieve-folder = Katna मा चल्छ: यो नियमले प्रयोग गर्ने फोल्डर तपाईंको मेल सर्भरमा छैन।
rules-note-gmail-sign-in = तपाईं फेरि Google मा साइन इन गरेर Katna लाई Gmail फिल्टरहरू बनाउन नदिएसम्म Katna मा चल्छ।
rules-note-sieve-other-script = Katna मा चल्छ: तपाईंको मेल सर्भरमा अर्को नियम स्क्रिप्ट (“{ $name }”) सक्रिय छ।
rules-note-gmail-failed = Katna मा चल्छ: Gmail ले यसलाई लिएन ({ $error })।
rules-note-sieve-failed = Katna मा चल्छ: तपाईंको मेल सर्भरले यसलाई लिएन ({ $error })।
rules-editor-cancel = रद्द गर्नुहोस्
rules-editor-save = सेभ गर्नुहोस्
rules-editor-saving = सेभ गर्दै…
rules-editor-delete = नियम मेटाउनुहोस्
rules-editor-delete-ask = यो नियम मेटाउने हो?
rules-editor-delete-keep = राख्नुहोस्
rules-editor-delete-confirm = मेटाउनुहोस्
rules-editor-needs-folder = प्रत्येक “यहाँ सार्नुहोस्” का लागि फोल्डर र प्रत्येक “लेबल थप्नुहोस्” का लागि लेबल छान्नुहोस्।
rules-editor-needs-days = “यति दिनपछि पढिएको भनी चिन्ह लगाउनुहोस्” मा 1 देखि 3650 सम्मको दिनको सङ्ख्या चाहिन्छ।
rules-saved = नियम सेभ भयो
rules-saved-applied = { $count ->
    [one] नियम सेभ भयो र { $count } मेलमा लागू भयो
   *[other] नियम सेभ भयो र { $count } मेलमा लागू भयो
}
rules-apply-failed = नियम सेभ भयो, तर लागू गर्न सकिएन: { $error }
rules-deleted = नियम मेटाइयो
rules-delete-failed = नियम मेटाउन सकिएन: { $error }
rules-change-failed = नियमहरू बदल्न सकिएन: { $error }
