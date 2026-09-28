# Katna Mail, Hindi (हिन्दी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = मेल खाता जोड़ें
add-account-looking = { $address } के मेल सर्वर ढूँढे जा रहे हैं…
add-account-address-intro = अपना ईमेल पता डालें। Katna आपके लिए सर्वर ढूँढ लेगा।
add-account-servers-title = सर्वर सेटिंग
add-account-servers-intro = Katna, { $address } का मेल कहाँ से पढ़ता और भेजता है।
add-account-password-title = अपना पासवर्ड डालें
add-account-signing-in = साइन इन हो रहा है…
add-account-browser-title = अपने ब्राउज़र में जारी रखें
add-account-browser-intro = Katna ने आपके ब्राउज़र में { $provider } का साइन-इन पेज खोला है। वहाँ साइन इन करें और Katna को अपना मेल पढ़ने और भेजने की अनुमति दें, फिर यहाँ वापस आएँ।
add-account-browser-hint = कोई पेज नहीं खुला? अपने ब्राउज़र की विंडो देखें, या वापस जाकर फिर से कोशिश करें।

## Add a mail account: fields

add-account-field-address = ईमेल पता
add-account-incoming = आने वाला मेल ({ $protocol })
add-account-outgoing = जाने वाला मेल ({ $protocol })
add-account-field-server = सर्वर
add-account-field-port = पोर्ट
add-account-security-none = कोई नहीं
add-account-security-none-warning = एन्क्रिप्ट नहीं है: आपका पासवर्ड और मेल रास्ते में पढ़े जा सकते हैं।
add-account-field-username = यूज़र नाम
add-account-field-password = पासवर्ड
add-account-show-password = पासवर्ड दिखाएँ
add-account-app-password-hint = यहाँ { $provider } को ऐप पासवर्ड चाहिए, वह नहीं जो आप वेब पर इस्तेमाल करते हैं। अपने { $provider } खाते की सुरक्षा सेटिंग में एक ऐप पासवर्ड बनाएँ।
add-account-field-name = आपका नाम (वैकल्पिक)
add-account-name-hint = उन लोगों को दिखता है जिन्हें आप लिखते हैं।
add-account-servers-pair = { $imap } और { $smtp }
add-account-servers-found = { $source ->
    [built-in] सर्वर: { $servers }, Katna की प्रोवाइडर सूची में मिले।
    [provider] सर्वर: { $servers }, आपके प्रोवाइडर की सेटिंग में मिले।
    [ispdb] सर्वर: { $servers }, Thunderbird की प्रोवाइडर सूची में मिले।
    [dns] सर्वर: { $servers }, आपके डोमेन के DNS रिकॉर्ड में मिले।
   *[other] सर्वर: { $servers }, अनुमान से; साइन इन न हो तो इन्हें जाँचें।
}
add-account-servers-entered = सर्वर: { $servers }, जैसे डाले गए।
add-account-or = या
add-account-sign-in-with = { $provider } से साइन इन करें
add-account-sign-in-instead = इसके बजाय { $provider } से साइन इन करें

## Add a mail account: buttons

add-account-servers-button = सर्वर सेटिंग
add-account-back = वापस
add-account-add = खाता जोड़ें
add-account-next = आगे
add-account-cancel = रद्द करें

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] आने वाले मेल का सर्वर डालें।
   *[outgoing] जाने वाले मेल का सर्वर डालें।
}
add-account-server-space = { $kind ->
    [incoming] आने वाले मेल के सर्वर के नाम में स्पेस है।
   *[outgoing] जाने वाले मेल के सर्वर के नाम में स्पेस है।
}
add-account-port-invalid = { $kind ->
    [incoming] आने वाले मेल का पोर्ट { $min } से { $max } तक की कोई संख्या होना चाहिए।
   *[outgoing] जाने वाले मेल का पोर्ट { $min } से { $max } तक की कोई संख्या होना चाहिए।
}
add-account-address-empty = ईमेल पता डालें।
add-account-address-invalid = { $example } जैसा ईमेल पता डालें।
add-account-not-found = Katna को { $address } के सर्वर नहीं मिले, इसलिए उसने आम तौर पर इस्तेमाल होने वाले नाम भर दिए हैं। अपने प्रोवाइडर से इनकी पुष्टि करें।
add-account-password-empty = पासवर्ड डालें।
add-account-name-is-password = नाम और पासवर्ड एक जैसे हैं। वहां अपना नाम लिखें, जैसा लोगों को दिखना चाहिए।
add-account-added = { $address } जोड़ा गया। आपका मेल लाया जा रहा है…
add-account-app-password-refused = { $provider } ने पासवर्ड अस्वीकार कर दिया। इसके लिए ऐप पासवर्ड चाहिए, वह नहीं जो आप वेब पर इस्तेमाल करते हैं।
add-account-password-refused = सर्वर ने पासवर्ड अस्वीकार कर दिया। उसे जाँचें और फिर से कोशिश करें।
add-account-sign-in-refused = { $provider } ने Katna को अंदर नहीं आने दिया। फिर से कोशिश करें, और अपने मेल तक पहुँच की अनुमति दें।
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Katna की यह कॉपी अभी Microsoft खातों में साइन इन नहीं कर सकती।
    [Google] Katna की यह कॉपी अभी Google खातों में साइन इन नहीं कर सकती।
   *[other] यह प्रोवाइडर सिर्फ़ अपने पेज पर साइन इन करने देता है, जो Katna अभी इसके लिए नहीं कर सकता।
}
add-account-signed-in = { $provider } से साइन इन हो गया। आपका मेल लाया जा रहा है…

## The account menu (from the account button on the top bar)

add-account-menu-another = दूसरा खाता जोड़ें
add-account-menu-manage = खाते प्रबंधित करें
app-menu = मुख्य मेन्यू
