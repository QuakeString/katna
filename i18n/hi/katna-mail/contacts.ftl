# Katna Mail, Hindi (हिन्दी): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = संपर्क
contacts-frequent = अक्सर
contacts-other = अन्य संपर्क
contacts-other-about = वे लोग जिन्हें आपने Gmail से मेल भेजा है, लेकिन सहेजा नहीं है
contacts-other-email = ईमेल भेजें
contacts-other-empty = कोई अन्य संपर्क नहीं है। Gmail से आप जिन्हें मेल भेजते हैं लेकिन सहेजते नहीं, वे यहाँ दिखते हैं।
contacts-other-allow = अन्य संपर्क देखने के लिए अपने Gmail खाते में फिर से साइन इन करें और Katna को उन्हें देखने की अनुमति दें।
contacts-labels = लेबल
contacts-label-options = लेबल विकल्प
contacts-label-rename = लेबल का नाम बदलें
contacts-label-email = सभी को मेल भेजें
contacts-label-delete = लेबल मिटाएं
contacts-label-new = नया लेबल
contacts-label-name = लेबल का नाम
contacts-label-button = लेबल
contacts-label-menu = इस रूप में लेबल करें:
contacts-label-added = { $name } में जोड़ा गया
contacts-label-removed = { $name } से हटाया गया
contacts-label-renamed = लेबल का नाम बदलकर { $name } किया गया
contacts-label-deleted = लेबल { $name } मिटाया गया
contacts-label-no-email = इस लेबल पर किसी का ईमेल पता नहीं है
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = खाते
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = संपर्क दिखाने के लिए फिर से साइन इन करें
contacts-account-signed-in = { $address } में फिर से साइन इन हो गया। आपके संपर्क लाए जा रहे हैं…
contacts-account-sign-in-refused = { $provider } ने Katna को अंदर नहीं आने दिया। फिर से कोशिश करें, और अपने संपर्कों तक पहुँच की अनुमति दें।
contacts-account-password = सर्वर ने पासवर्ड स्वीकार नहीं किया। Yahoo, iCloud, Zoho और दूसरों को ऐप पासवर्ड चाहिए।
contacts-account-change-password = पासवर्ड बदलें
contacts-account-change-password-tooltip = नया पासवर्ड टाइप करें; Katna सर्वर से इसकी जाँच करता है
contacts-account-failed = संपर्क पढ़े नहीं जा सके।
# $reason is the server's own words, in English.
contacts-account-error = संपर्क पढ़े नहीं जा सके: { $reason }
contacts-account-none = कोई पता पुस्तिका नहीं मिली
# $reason is what the server answered, in English: "CardDAV https://dav.example.in/: status 404".
contacts-account-none-why = कोई पता पुस्तिका नहीं मिली: { $reason }
# A Gmail or Outlook account added with a password: its contacts need the
# provider's sign-in.
contacts-account-use-sign-in = { $provider } संपर्क सिर्फ़ उसी Katna को दिखाता है जो { $provider } से साइन इन हो।
contacts-account-sign-in-with = { $provider } से साइन इन करें
contacts-account-looking = संपर्क खोजे जा रहे हैं…
contacts-account-try-again = फिर से कोशिश करें
contacts-account-try-again-tooltip = इस खाते के संपर्क अभी फिर से जाँचें
contacts-account-fixing = काम जारी है…
contacts-manage = ठीक करें और प्रबंधित करें
contacts-merge = मर्ज करें और ठीक करें
contacts-merge-about = { $count ->
    [one] { $count } सुझाव: ऐसे संपर्क जो एक ही व्यक्ति के लगते हैं
   *[other] { $count } सुझाव: ऐसे संपर्क जो एक ही व्यक्ति के लगते हैं
}
contacts-merge-none = कोई डुप्लिकेट नहीं। समान नाम या फ़ोन नंबर वाले संपर्क यहाँ दिखेंगे।
contacts-merge-count = { $count ->
    [one] { $count } संपर्क
   *[other] { $count } संपर्क
}
contacts-merge-all = सभी मर्ज करें
contacts-merge-button = मर्ज करें
contacts-merge-dismiss = खारिज करें
contacts-merged = { $count ->
    [1] संपर्क मर्ज हो गए
    [one] { $count } मर्ज पूरे हुए
   *[other] { $count } मर्ज पूरे हुए
}
contacts-import = इंपोर्ट करें
contacts-export = एक्सपोर्ट करें
contacts-import-file = vCard या CSV फ़ाइल से संपर्क इंपोर्ट करें
contacts-imported = { $count ->
    [one] { $place } में { $count } संपर्क इंपोर्ट किए गए
   *[other] { $place } में { $count } संपर्क इंपोर्ट किए गए
}
contacts-imported-some = { $count ->
    [one] { $place } में { $count } संपर्क इंपोर्ट किए गए; पहले से सहेजे होने के कारण { $skipped } छोड़ दिए गए
   *[other] { $place } में { $count } संपर्क इंपोर्ट किए गए; पहले से सहेजे होने के कारण { $skipped } छोड़ दिए गए
}
contacts-import-none = { $name } में कोई संपर्क नहीं मिला
contacts-import-all-saved = { $name } में सभी लोग पहले से सहेजे हुए हैं
contacts-import-failed = { $name } पढ़ा नहीं जा सका: { $error }
contacts-exported = { $count ->
    [one] { $path } में { $count } संपर्क एक्सपोर्ट किए गए
   *[other] { $path } में { $count } संपर्क एक्सपोर्ट किए गए
}
contacts-export-none = एक्सपोर्ट करने के लिए कोई संपर्क नहीं
contacts-export-failed = संपर्क एक्सपोर्ट नहीं किए जा सके: { $error }
contacts-print = प्रिंट करें
contacts-print-title = संपर्क
contacts-print-none = प्रिंट करने के लिए कोई संपर्क नहीं
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = जन्मदिन: { $day }
contacts-print-nickname = उपनाम: { $name }
contacts-create = नया संपर्क

## Search and the list

contacts-search = संपर्क खोजें
contacts-loading = संपर्क लोड हो रहे हैं…
contacts-empty = अभी कोई सहेजा हुआ संपर्क नहीं है। Gmail, Outlook या आपकी मेल सेवा में सहेजे गए संपर्क यहाँ दिखते हैं।
contacts-empty-no-books = आपके खातों के संपर्क सिंक होते ही यहाँ दिखेंगे।
contacts-none-found = आपकी खोज से कोई संपर्क मेल नहीं खाता।
contacts-starred = { $count ->
    [one] तारांकित संपर्क ({ $count })
   *[other] तारांकित संपर्क ({ $count })
}
contacts-count = संपर्क ({ $count })
contacts-col-name = नाम
contacts-col-email = ईमेल
contacts-col-phone = फ़ोन नंबर
contacts-col-job = नौकरी का शीर्षक और कंपनी
contacts-col-labels = लेबल

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Katna को { $address } के संपर्क पढ़ने की अनुमति दें।
contacts-allow-many = { $more ->
    [one] Katna को { $address } और { $more } और खाते के संपर्क पढ़ने की अनुमति दें।
   *[other] Katna को { $address } और { $more } और खातों के संपर्क पढ़ने की अनुमति दें।
}
contacts-allow-button = अनुमति दें

## A contact's page

contacts-back = संपर्कों पर वापस जाएँ
contacts-edit = बदलाव करें
contacts-delete = मिटाएं
contacts-qr = QR कोड के रूप में शेयर करें
contacts-qr-about = संपर्क सेव करने के लिए इसे फ़ोन के कैमरे से स्कैन करें।
contacts-qr-too-long = इस संपर्क में इतनी ज़्यादा जानकारी है कि वह QR कोड में नहीं समा सकती।
contacts-qr-done = हो गया
contacts-deleted = { $name } मिटा दिया गया
contacts-added = { $name } को संपर्क में जोड़ा गया
contacts-find-mail = मेल
contacts-details = संपर्क का ब्योरा
contacts-saved-in = इसमें सहेजा गया
contacts-notes = नोट
contacts-birthday = जन्मदिन
contacts-nickname = उपनाम
contacts-this-computer = यह कंप्यूटर
contacts-kind-home = घर
contacts-kind-work = कार्यालय
contacts-kind-mobile = मोबाइल
contacts-kind-other = अन्य
contacts-source-google = Google संपर्क
contacts-source-microsoft = Outlook संपर्क
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = संपर्क बनाएं
contacts-edit-title = संपर्क में बदलाव करें
contacts-edit-save = सेव करें
contacts-edit-saving = सेव हो रहा है…
contacts-edit-cancel = रद्द करें
contacts-saved = संपर्क सेव हो गया
contacts-edit-save-to = इसमें सेव करें
contacts-edit-changes-go-to = बदलाव { $place } में सेव किए जाएंगे।
contacts-edit-given = पहला नाम
contacts-edit-family = अंतिम नाम
contacts-edit-company = कंपनी
contacts-edit-job = नौकरी का शीर्षक
contacts-edit-email = ईमेल
contacts-edit-phone = फ़ोन
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = ईमेल जोड़ें
contacts-edit-add-phone = फ़ोन जोड़ें
contacts-edit-street = सड़क का पता
contacts-edit-city = शहर
contacts-edit-postcode = पिन कोड
contacts-edit-country = देश
contacts-edit-birthday = जन्मदिन (YYYY-MM-DD)
contacts-edit-empty = पहले कोई नाम, ईमेल या फ़ोन नंबर जोड़ें।
