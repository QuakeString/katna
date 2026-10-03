# Katna Mail, Hindi (हिन्दी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = मेल खाता जोड़ें
add-account-providers-intro = अपना मेल प्रोवाइडर चुनें। बाकी Katna खुद ढूंढ लेता है।
add-account-provider-other = अन्य मेल
add-account-provider-other-detail = कोई भी IMAP या POP3 खाता
add-account-provider-google-detail = Gmail और Google Workspace
add-account-provider-microsoft-detail = Outlook और Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = { $provider } में साइन इन करें
add-account-form-title-other = आपका मेल खाता
add-account-form-intro = Katna आपका पासवर्ड आपके सिस्टम के कीरिंग में रखता है।
add-account-looking = { $address } के मेल सर्वर ढूँढे जा रहे हैं…
add-account-address-intro = अपना ईमेल पता डालें। Katna आपके लिए सर्वर ढूँढ लेगा।
add-account-servers-title = सर्वर सेटिंग
add-account-servers-intro = Katna, { $address } का मेल कहाँ से पढ़ता और भेजता है।
add-account-signing-in = साइन इन हो रहा है…
add-account-browser-title = अपने ब्राउज़र में जारी रखें
add-account-browser-intro = Katna ने आपके ब्राउज़र में { $provider } का साइन-इन पेज खोला है। वहाँ साइन इन करें और Katna को अपना मेल पढ़ने और भेजने की अनुमति दें, फिर यहाँ वापस आएँ।
add-account-browser-hint = कोई पेज नहीं खुला? अपने ब्राउज़र की विंडो देखें, या वापस जाकर फिर से कोशिश करें।
add-account-stage-browser = आपके ब्राउज़र में साइन इन करने का इंतज़ार है…
add-account-stage-signing-in-at = { $server } पर साइन इन हो रहा है…
add-account-help-app-password-link = ऐप पासवर्ड कैसे बनाएं
add-account-help-turn-on-imap = { $provider } मेल ऐप्स को तभी आने देता है जब उसके वेब मेल की सेटिंग में IMAP और POP3 ऐक्सेस चालू हो।
add-account-help-turn-on-imap-link = इसे कैसे चालू करें

## Add a mail account: fields

add-account-field-address = ईमेल पता
add-account-receive-with = मेल पाने का तरीका
add-account-imap-about = IMAP आपके मेल और फ़ोल्डर सर्वर पर रखता है, हर डिवाइस पर एक जैसे। हो सके तो इसे चुनें।
add-account-pop3-about = POP3 आपके मेल को इस कंप्यूटर पर डाउनलोड करता है। यहां पढ़ा या कहीं ले जाया गया मेल सर्वर और आपके दूसरे डिवाइस पर जैसा है वैसा ही रहता है।
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
add-account-sign-in-with = { $provider } से साइन इन करें
add-account-sign-in-instead = इसके बजाय { $provider } से साइन इन करें

## Add a mail account: buttons

add-account-servers-button = सर्वर सेटिंग
add-account-back = वापस
add-account-add = खाता जोड़ें
add-account-done = हो गया
add-account-another = दूसरा खाता जोड़ें
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
add-account-app-password-refused = { $provider } ने पासवर्ड अस्वीकार कर दिया। इसके लिए ऐप पासवर्ड चाहिए, वह नहीं जो आप वेब पर इस्तेमाल करते हैं।
add-account-password-refused = सर्वर ने पासवर्ड अस्वीकार कर दिया। उसे जाँचें और फिर से कोशिश करें।
add-account-sign-in-refused = { $provider } ने Katna को अंदर नहीं आने दिया। फिर से कोशिश करें, और अपने मेल तक पहुँच की अनुमति दें।
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Katna की यह कॉपी अभी Microsoft खातों में साइन इन नहीं कर सकती।
    [Google] Katna की यह कॉपी अभी Google खातों में साइन इन नहीं कर सकती।
   *[other] यह प्रोवाइडर सिर्फ़ अपने पेज पर साइन इन करने देता है, जो Katna अभी इसके लिए नहीं कर सकता।
}
add-account-smtp-not-found = Katna को पता चल गया कि आपका मेल कहां से पढ़ना है, पर यह नहीं कि कहां से भेजना है। आउटगोइंग सर्वर डालें।

## Add a mail account: the last step

add-account-done-title = आपका खाता तैयार है
add-account-done-intro = Katna अभी आपका मेल ला रहा है। नया मेल आते ही दिखता है।
add-account-done-sign-in = साइन इन
add-account-done-signed-in-with = { $provider } से, आपके ब्राउज़र में
add-account-done-receiving = मेल पाना
add-account-done-sending = मेल भेजना
add-account-done-on-server = सर्वर पर मेल
add-account-done-kept = जब तक आप Katna में न मिटाएं, तब तक रखा जाता है
add-account-done-pop3-hint = सर्वर पर मेल का क्या हो, यह सेटिंग > खाते में बदलें।
add-account-done-zoho-title = टास्क और कैलेंडर
add-account-done-zoho-about = Zoho इन्हें मेल से अलग रखता है। इन्हें Katna में लाने के लिए एक बार Zoho से साइन इन करें।
add-account-done-linked = टास्क और कैलेंडर जुड़ गए

## The account menu (from the account button on the top bar)

add-account-menu-another = दूसरा खाता जोड़ें
app-menu = मुख्य मेन्यू
app-menu-back = वापस
