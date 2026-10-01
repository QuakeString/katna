# Katna Mail, Nepali (नेपाली).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = मेल खाता थप्नुहोस्
add-account-looking = { $address } का मेल सर्भरहरू खोज्दै…
add-account-address-intro = आफ्नो इमेल ठेगाना लेख्नुहोस्। Katna ले तपाईंका लागि सर्भरहरू पत्ता लगाउँछ।
add-account-servers-title = सर्भर सेटिङहरू
add-account-servers-intro = { $address } का लागि Katna ले मेल कहाँबाट पढ्छ र पठाउँछ।
add-account-signing-in = साइन इन गर्दै…
add-account-browser-title = आफ्नो ब्राउजरमा जारी राख्नुहोस्
add-account-browser-intro = Katna ले तपाईंको ब्राउजरमा { $provider } को साइन इन पेज खोल्यो। त्यहाँ साइन इन गर्नुहोस् र Katna लाई तपाईंको मेल पढ्न र पठाउन अनुमति दिनुहोस्, अनि यहाँ फर्कनुहोस्।
add-account-browser-hint = कुनै पेज खुलेन? आफ्नो ब्राउजरका विन्डोहरू जाँच गर्नुहोस्, वा पछाडि गएर फेरि प्रयास गर्नुहोस्।

## Add a mail account: fields

add-account-field-address = इमेल ठेगाना
add-account-incoming = आउने मेल ({ $protocol })
add-account-outgoing = जाने मेल ({ $protocol })
add-account-field-server = सर्भर
add-account-field-port = पोर्ट
add-account-security-none = कुनै पनि होइन
add-account-security-none-warning = इन्क्रिप्ट गरिएको छैन: तपाईंको पासवर्ड र मेल बाटोमै पढ्न सकिन्छ।
add-account-field-username = प्रयोगकर्ता नाम
add-account-field-password = पासवर्ड
add-account-show-password = पासवर्ड देखाउनुहोस्
add-account-app-password-hint = { $provider } लाई यहाँ एप पासवर्ड चाहिन्छ, तपाईंले वेबमा प्रयोग गर्ने पासवर्ड होइन। आफ्नो { $provider } खाताको सुरक्षा सेटिङमा एउटा बनाउनुहोस्।
add-account-field-name = तपाईंको नाम (ऐच्छिक)
add-account-name-hint = तपाईंले लेख्ने मानिसहरूलाई देखाइन्छ।
add-account-servers-pair = { $imap } र { $smtp }
add-account-servers-found = { $source ->
    [built-in] सर्भरहरू: { $servers }, Katna को प्रदायक सूचीमा भेटिए।
    [provider] सर्भरहरू: { $servers }, तपाईंको प्रदायकको सेटिङमा भेटिए।
    [ispdb] सर्भरहरू: { $servers }, Thunderbird को प्रदायक सूचीमा भेटिए।
    [dns] सर्भरहरू: { $servers }, तपाईंको डोमेनका DNS रेकर्डहरूमा भेटिए।
   *[other] सर्भरहरू: { $servers }, अनुमान गरिएको; साइन इन असफल भए जाँच गर्नुहोस्।
}
add-account-servers-entered = सर्भरहरू: { $servers }, लेखिएअनुसार।
add-account-sign-in-with = { $provider } बाट साइन इन गर्नुहोस्
add-account-sign-in-instead = बरु { $provider } बाट साइन इन गर्नुहोस्

## Add a mail account: buttons

add-account-servers-button = सर्भर सेटिङहरू
add-account-back = पछाडि
add-account-add = खाता थप्नुहोस्
add-account-cancel = रद्द गर्नुहोस्

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] आउने मेलको सर्भर लेख्नुहोस्।
   *[outgoing] जाने मेलको सर्भर लेख्नुहोस्।
}
add-account-server-space = { $kind ->
    [incoming] आउने मेलको सर्भरको नाममा खाली ठाउँ छ।
   *[outgoing] जाने मेलको सर्भरको नाममा खाली ठाउँ छ।
}
add-account-port-invalid = { $kind ->
    [incoming] आउने मेलको पोर्ट { $min } देखि { $max } सम्मको सङ्ख्या हुनुपर्छ।
   *[outgoing] जाने मेलको पोर्ट { $min } देखि { $max } सम्मको सङ्ख्या हुनुपर्छ।
}
add-account-address-empty = इमेल ठेगाना लेख्नुहोस्।
add-account-address-invalid = { $example } जस्तो इमेल ठेगाना लेख्नुहोस्।
add-account-not-found = Katna ले { $address } का लागि सर्भरहरू भेटेन, त्यसैले सामान्य नामहरू भरिदियो। तिनलाई आफ्नो प्रदायकसँग जाँच गर्नुहोस्।
add-account-password-empty = पासवर्ड लेख्नुहोस्।
add-account-name-is-password = नाम र पासवर्ड उस्तै छन्। त्यहाँ बरु मानिसहरूले देख्नुपर्ने गरी आफ्नो नाम लेख्नुहोस्।
add-account-app-password-refused = { $provider } ले पासवर्ड अस्वीकार गर्‍यो। यसलाई एप पासवर्ड चाहिन्छ, तपाईंले वेबमा प्रयोग गर्ने पासवर्ड होइन।
add-account-password-refused = सर्भरले पासवर्ड अस्वीकार गर्‍यो। यसलाई जाँच गरेर फेरि प्रयास गर्नुहोस्।
add-account-sign-in-refused = { $provider } ले Katna लाई भित्र आउन दिएन। फेरि प्रयास गर्नुहोस्, र आफ्नो मेलमा पहुँच दिनुहोस्।
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Katna को यो प्रतिले अझै Microsoft खाताहरूमा साइन इन गर्न सक्दैन।
    [Google] Katna को यो प्रतिले अझै Google खाताहरूमा साइन इन गर्न सक्दैन।
   *[other] यो प्रदायकले आफ्नै पेजमा मात्र साइन इन गर्न दिन्छ, जुन Katna ले यसका लागि अझै गर्न सक्दैन।
}

## The account menu (from the account button on the top bar)

add-account-menu-another = अर्को खाता थप्नुहोस्
app-menu = मुख्य मेनु
app-menu-back = पछाडि
