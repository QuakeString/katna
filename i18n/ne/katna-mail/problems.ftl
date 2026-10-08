# Katna Mail, Nepali (नेपाली).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = मेल सर्भर

problems-signed-out = { $provider } ले Katna लाई { $address } बाट साइन आउट गर्‍यो। मेल सिंक हुन रोकियो।
problems-password-refused = { $provider } ले { $address } को पासवर्ड अस्वीकार गर्‍यो। यो बदलिएको हुन सक्छ।
problems-no-answer = { $provider } ले { $address } का लागि जवाफ दिइरहेको छैन। Katna प्रयास गरिरहन्छ।
problems-offline = तपाईं अफलाइन हुनुहुन्छ। तपाईंको मेल यहीँ छ, र तपाईंले पठाउने मेल तपाईं फर्केसम्म पर्खन्छ।
problems-accounts-need-you = { $count ->
    [one] 1 खातालाई तपाईंको ध्यान चाहिन्छ
   *[other] { $count } खातालाई तपाईंको ध्यान चाहिन्छ
}
problems-show = देखाउनुहोस्
problems-later = पछि
problems-new-password = नयाँ पासवर्ड
problems-try-again = फेरि प्रयास गर्नुहोस्

## The New password card

problems-password-title = नयाँ पासवर्ड
problems-password-detail = { $provider } ले { $address } को सेभ गरिएको पासवर्ड अस्वीकार गर्‍यो। नयाँ पासवर्ड टाइप गर्नुहोस्; Katna ले राख्नुअघि जाँच गर्छ।
problems-password-placeholder = पासवर्ड
problems-password-show = पासवर्ड देखाउनुहोस्
problems-password-hide = पासवर्ड लुकाउनुहोस्
problems-password-cancel = रद्द गर्नुहोस्
problems-password-save = सेभ गर्नुहोस्
problems-password-checking = जाँच गर्दै…
problems-password-refused-again = { $provider } ले यो पासवर्ड पनि अस्वीकार गर्‍यो। जाँचेर फेरि प्रयास गर्नुहोस्।
problems-password-saved = { $address } को पासवर्ड सेभ भयो। तपाईंको मेल ल्याउँदै…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = { $address } को मेल सर्भरले { $count ->
    [one] एउटा सन्देश सार्ने काम स्वीकार गरेन, त्यसैले त्यो पहिलेकै ठाउँमा फर्कियो।
   *[other] { $count } सन्देश सार्ने काम स्वीकार गरेन, त्यसैले ती पहिलेकै ठाउँमा फर्किए।
}
problems-refused-flags = { $address } को मेल सर्भरले { $count ->
    [one] एउटा सन्देशमा चिन्ह लगाउने (पढिएको, तारा…) काम स्वीकार गरेन, त्यसैले त्यो पहिलेजस्तै भयो।
   *[other] { $count } सन्देशमा चिन्ह लगाउने (पढिएको, तारा…) काम स्वीकार गरेन, त्यसैले ती पहिलेजस्तै भए।
}
problems-refused-label = { $address } को मेल सर्भरले { $count ->
    [one] एउटा सन्देशका लेबल बदल्ने काम स्वीकार गरेन, त्यसैले त्यो पहिलेजस्तै भयो।
   *[other] { $count } सन्देशका लेबल बदल्ने काम स्वीकार गरेन, त्यसैले ती पहिलेजस्तै भए।
}
problems-refused-delete = { $address } को मेल सर्भरले { $count ->
    [one] एउटा सन्देश मेटाउने काम स्वीकार गरेन, त्यसैले त्यो फर्कियो।
   *[other] { $count } सन्देश मेटाउने काम स्वीकार गरेन, त्यसैले ती फर्किए।
}
problems-refused-other = { $address } को मेल सर्भरले { $count ->
    [one] एउटा परिवर्तन स्वीकार गरेन, त्यसैले Katna ले त्यसलाई पहिलेजस्तै बनायो।
   *[other] { $count } परिवर्तन स्वीकार गरेन, त्यसैले Katna ले तिनलाई पहिलेजस्तै बनायो।
}
problems-details = विवरण

## Katna's background service (katna-daemon) isn't running

service-starting = Katna को पृष्ठभूमि सेवा सुरु गर्दै…
service-failed = Katna को पृष्ठभूमि सेवा सुरु हुँदैन, त्यसैले मेल सिंक भइरहेको छैन।
service-start-again = फेरि सुरु गर्नुहोस्
service-started-again = Katna को पृष्ठभूमि सेवा रोकिएको थियो र फेरि सुरु गरियो।
service-details-title = सेवा किन सुरु हुँदैन
service-details-body = यसलाई कपी गरेर आफ्नो रिपोर्टसँग पठाउनुहोस्। यसमा कुनै मेल वा पासवर्ड छैन।
service-details-copy = कपी गर्नुहोस्
service-details-close = बन्द गर्नुहोस्
service-not-running = Katna को पृष्ठभूमि सेवा चलिरहेको छैन।
service-no-answer = Katna को पृष्ठभूमि सेवाले जवाफ दिएन: { $error }
service-no-session = D-Bus सत्र छैन: { $error }

## Safe mode: an update left Katna's background service unable to start

safe-line = अपडेटमा समस्या आएकाले Katna सुरक्षित मोडमा छ, त्यसैले मेल सिंक भइरहेको छैन।
safe-try-again = फेरि प्रयास गर्नुहोस्
safe-restore = पुनर्स्थापना गर्नुहोस्
safe-restoring = { $when } को तपाईंको डेटा पुनर्स्थापना गर्दै…
safe-restored = { $when } को तपाईंको डेटा पुनर्स्थापना गरियो। पहिले भएको कुरा एउटा फोल्डरमा राखिएको छ।
safe-show-folder = फोल्डर देखाउनुहोस्
safe-restore-failed = तपाईंको डेटा पुनर्स्थापना गर्न सकिएन: { $error }
safe-restore-title = अपडेटअघिको तपाईंको डेटा पुनर्स्थापना गर्ने?
safe-restore-body = Katna तपाईंले छान्नुभएको प्रतिमा फर्कन्छ। त्यसपछि आएका मेलहरू तपाईंका खाताहरूबाट फेरि डाउनलोड हुन्छन्।
safe-restore-none = अझै कुनै प्रति छैन। हरेक अपडेटले तपाईंको डेटा बदल्नुअघि Katna एउटा प्रति बनाउँछ।
safe-restore-keep = अहिले भएको कुरा, नपठाइएका मेल, ड्राफ्ट र अझै सिंक नभएका परिवर्तनहरूसहित, पहिले एउटा फोल्डरमा राखिन्छ, त्यसैले केही पनि हराउँदैन।
safe-restore-cancel = रद्द गर्नुहोस्
safe-restore-mail = मेल
safe-restore-pim = खाता र सम्पर्कहरू
safe-restore-blobs = संलग्नकहरू
safe-report-title = डिबग रिपोर्ट
safe-report-body = यसलाई कपी गरेर आफ्नो बग रिपोर्टमा संलग्न गर्नुहोस्। यसमा कुनै मेल, ठेगाना वा पासवर्ड छैन।
safe-report-restore = पुनर्स्थापना गर्नुहोस्…
safe-report-copied = डिबग रिपोर्ट कपी गरियो
