# Katna Mail, Nepali (नेपाली).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = मेल खाता थप्नुहोस्
add-account-providers-intro = आफ्नो मेल प्रदायक छान्नुहोस्। बाँकी Katna ले पत्ता लगाउँछ।
add-account-provider-other = अन्य मेल
add-account-provider-other-detail = कुनै पनि IMAP वा POP3 खाता
add-account-provider-google-detail = Gmail र Google Workspace
add-account-provider-microsoft-detail = Outlook र Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = { $provider } मा साइन इन गर्नुहोस्
add-account-form-title-other = तपाईंको मेल खाता
add-account-form-intro = Katna ले तपाईंको पासवर्ड प्रणालीको किरिङमा राख्छ।
add-account-looking = { $address } का मेल सर्भरहरू खोज्दै…
add-account-address-intro = आफ्नो इमेल ठेगाना लेख्नुहोस्। Katna ले तपाईंका लागि सर्भरहरू पत्ता लगाउँछ।
add-account-servers-title = सर्भर सेटिङहरू
add-account-servers-intro = { $address } का लागि Katna ले मेल कहाँबाट पढ्छ र पठाउँछ।
add-account-signing-in = साइन इन गर्दै…
add-account-browser-title = आफ्नो ब्राउजरमा जारी राख्नुहोस्
add-account-browser-intro = Katna ले तपाईंको ब्राउजरमा { $provider } को साइन इन पेज खोल्यो। त्यहाँ साइन इन गर्नुहोस् र Katna लाई तपाईंको मेल पढ्न र पठाउन अनुमति दिनुहोस्, अनि यहाँ फर्कनुहोस्।
add-account-browser-hint = कुनै पेज खुलेन? आफ्नो ब्राउजरका विन्डोहरू जाँच गर्नुहोस्, वा पछाडि गएर फेरि प्रयास गर्नुहोस्।
add-account-stage-browser = तपाईंले ब्राउजरमा साइन इन गर्ने प्रतीक्षा गर्दै…
add-account-stage-signing-in-at = { $server } मा साइन इन गर्दै…
add-account-help-app-password-link = एप पासवर्ड कसरी बनाउने
add-account-help-turn-on-imap = { $provider } ले आफ्नो वेब मेलको सेटिङमा IMAP र POP3 पहुँच खोलेपछि मात्र मेल एपहरूलाई भित्र आउन दिन्छ।
add-account-help-turn-on-imap-link = यसलाई कसरी खोल्ने

## Add a mail account: fields

add-account-field-address = इमेल ठेगाना
add-account-receive-with = यसबाट मेल प्राप्त गर्नुहोस्
add-account-imap-about = IMAP ले तपाईंको मेल र फोल्डरहरू सर्भरमै राख्छ, हरेक उपकरणमा उस्तै। सम्भव भएसम्म यही छान्नुहोस्।
add-account-pop3-about = POP3 ले तपाईंको मेल यो कम्प्युटरमा डाउनलोड गर्छ। यहाँ पढेको वा सारेको मेल सर्भर र अन्य उपकरणहरूमा जस्ताको तस्तै रहन्छ।
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
add-account-done = सम्पन्न
add-account-another = अर्को खाता थप्नुहोस्
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
add-account-smtp-not-found = Katna ले तपाईंको मेल कहाँबाट पढ्ने भेट्टायो, तर कहाँबाट पठाउने भेट्टाएन। बाहिर जाने सर्भर लेख्नुहोस्।

## Add a mail account: the last step

add-account-done-title = तपाईंको खाता तयार छ
add-account-done-intro = Katna ले अहिले तपाईंको मेल ल्याउँदैछ। नयाँ मेल आइपुग्दै गर्दा देखिन्छ।
add-account-done-sign-in = साइन इन
add-account-done-signed-in-with = { $provider } बाट, तपाईंको ब्राउजरमा
add-account-done-receiving = मेल प्राप्त गर्ने
add-account-done-sending = मेल पठाउने
add-account-done-on-server = सर्भरमा रहेको मेल
add-account-done-kept = Katna मा तपाईंले नमेटाएसम्म राखिन्छ
add-account-done-pop3-hint = सर्भरमा रहेको मेललाई के हुन्छ, सेटिङहरू > खाताहरू मा बदल्नुहोस्।
add-account-done-zoho-title = कार्य र पात्रोहरू
add-account-done-zoho-about = Zoho ले यिनलाई मेलबाट छुट्टै राख्छ। यिनलाई Katna मा ल्याउन एक पटक Zoho बाट साइन इन गर्नुहोस्।
add-account-done-linked = कार्य र पात्रोहरू जोडिए

## The account menu (from the account button on the top bar)

add-account-menu-another = अर्को खाता थप्नुहोस्
app-menu = मुख्य मेनु
app-menu-back = पछाडि
