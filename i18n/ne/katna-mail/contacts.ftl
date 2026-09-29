# Katna Mail, Nepali (नेपाली): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = सम्पर्कहरू
contacts-frequent = बारम्बार
contacts-other = अन्य सम्पर्कहरू
contacts-other-about = तपाईंले Gmail बाट मेल गर्नुभएको तर सुरक्षित नगर्नुभएका मानिसहरू
contacts-other-email = इमेल पठाउनुहोस्
contacts-other-empty = अन्य सम्पर्कहरू छैनन्। तपाईंले Gmail बाट मेल गर्ने तर सुरक्षित नगर्ने मानिसहरू यहाँ देखिन्छन्।
contacts-other-allow = अन्य सम्पर्कहरू हेर्न आफ्नो Gmail खातामा फेरि साइन इन गर्नुहोस् र Katna लाई ती हेर्न अनुमति दिनुहोस्।
contacts-labels = लेबलहरू
contacts-label-options = लेबल विकल्पहरू
contacts-label-rename = लेबलको नाम बदल्नुहोस्
contacts-label-email = सबैलाई मेल पठाउनुहोस्
contacts-label-delete = लेबल मेटाउनुहोस्
contacts-label-new = नयाँ लेबल
contacts-label-name = लेबलको नाम
contacts-label-button = लेबल
contacts-label-menu = यसरी लेबल लगाउनुहोस्:
contacts-label-added = { $name } मा थपियो
contacts-label-removed = { $name } बाट हटाइयो
contacts-label-renamed = लेबलको नाम बदलेर { $name } बनाइयो
contacts-label-deleted = लेबल { $name } मेटाइयो
contacts-label-no-email = यो लेबलमा कसैको पनि इमेल ठेगाना छैन
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = खाताहरू
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = सम्पर्कहरू देखाउन फेरि साइन इन गर्नुहोस्
contacts-account-signed-in = { $address } मा फेरि साइन इन भयो। तपाईंका सम्पर्कहरू ल्याउँदै…
contacts-account-sign-in-refused = { $provider } ले Katna लाई भित्र आउन दिएन। फेरि प्रयास गर्नुहोस्, र आफ्ना सम्पर्कहरूमा पहुँच दिनुहोस्।
contacts-account-password = सर्भरले पासवर्ड स्वीकार गरेन। Yahoo, iCloud, Zoho र अरूलाई एप पासवर्ड चाहिन्छ।
contacts-account-change-password = पासवर्ड बदल्नुहोस्
contacts-account-change-password-tooltip = सेटिङहरू > खाताहरू खोल्नुहोस्
contacts-account-failed = सम्पर्कहरू पढ्न सकिएन।
# $reason is the server's own words, in English.
contacts-account-error = सम्पर्कहरू पढ्न सकिएन: { $reason }
contacts-account-none = कुनै ठेगाना पुस्तिका भेटिएन
# $reason is what the server answered, in English: "CardDAV https://dav.example.in/: status 404".
contacts-account-none-why = कुनै ठेगाना पुस्तिका भेटिएन: { $reason }
# A Gmail or Outlook account added with a password: its contacts need the
# provider's sign-in.
contacts-account-use-sign-in = { $provider } ले { $provider } बाट साइन इन गरिएको Katna लाई मात्र सम्पर्कहरू देखाउँछ।
contacts-account-sign-in-with = { $provider } बाट साइन इन गर्नुहोस्
contacts-account-looking = सम्पर्कहरू खोज्दै…
contacts-account-try-again = फेरि प्रयास गर्नुहोस्
contacts-account-try-again-tooltip = यो खाताका सम्पर्कहरू अहिले फेरि जाँच गर्नुहोस्
contacts-account-fixing = काम हुँदैछ…
contacts-manage = ठीक गर्नुहोस् र व्यवस्थापन गर्नुहोस्
contacts-merge = गाभ्नुहोस् र ठीक गर्नुहोस्
contacts-merge-about = { $count ->
    [one] { $count } सुझाव: उही व्यक्तिजस्ता देखिने सम्पर्कहरू
   *[other] { $count } सुझाव: उही व्यक्तिजस्ता देखिने सम्पर्कहरू
}
contacts-merge-none = डुप्लिकेट छैनन्। उही नाम वा फोन नम्बर भएका सम्पर्कहरू यहाँ देखिन्छन्।
contacts-merge-count = { $count ->
    [one] { $count } सम्पर्कहरू
   *[other] { $count } सम्पर्कहरू
}
contacts-merge-all = सबै गाभ्नुहोस्
contacts-merge-button = गाभ्नुहोस्
contacts-merge-dismiss = खारेज गर्नुहोस्
contacts-merged = { $count ->
    [1] सम्पर्कहरू गाभिए
    [one] { $count } गाभ्ने काम सम्पन्न भए
   *[other] { $count } गाभ्ने काम सम्पन्न भए
}
contacts-import = आयात गर्नुहोस्
contacts-export = निर्यात गर्नुहोस्
contacts-import-file = vCard वा CSV फाइलबाट सम्पर्कहरू आयात गर्नुहोस्
contacts-imported = { $count ->
    [one] { $place } मा { $count } सम्पर्कहरू आयात गरियो
   *[other] { $place } मा { $count } सम्पर्कहरू आयात गरियो
}
contacts-imported-some = { $count ->
    [one] { $place } मा { $count } सम्पर्कहरू आयात गरियो; पहिले नै सुरक्षित गरिएका { $skipped } छोडियो
   *[other] { $place } मा { $count } सम्पर्कहरू आयात गरियो; पहिले नै सुरक्षित गरिएका { $skipped } छोडियो
}
contacts-import-none = { $name } मा कुनै सम्पर्क फेला परेन
contacts-import-all-saved = { $name } मा भएका सबै पहिले नै सुरक्षित छन्
contacts-import-failed = { $name } पढ्न सकिएन: { $error }
contacts-exported = { $count ->
    [one] { $path } मा { $count } सम्पर्कहरू निर्यात गरियो
   *[other] { $path } मा { $count } सम्पर्कहरू निर्यात गरियो
}
contacts-export-none = निर्यात गर्ने सम्पर्क छैन
contacts-export-failed = सम्पर्कहरू निर्यात गर्न सकिएन: { $error }
contacts-print = प्रिन्ट गर्नुहोस्
contacts-print-title = सम्पर्कहरू
contacts-print-none = प्रिन्ट गर्न कुनै सम्पर्क छैन
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = जन्मदिन: { $day }
contacts-print-nickname = उपनाम: { $name }
contacts-create = सम्पर्क सिर्जना गर्नुहोस्

## Search and the list

contacts-search = सम्पर्कहरू खोज्नुहोस्
contacts-loading = सम्पर्कहरू लोड हुँदैछन्…
contacts-empty = अहिलेसम्म कुनै सुरक्षित गरिएका सम्पर्क छैनन्। Gmail, Outlook वा तपाईंको मेल सेवामा सुरक्षित गरिएका सम्पर्कहरू यहाँ देखिन्छन्।
contacts-empty-no-books = तपाईंका खाताहरूका सम्पर्कहरू सिंक भएपछि यहाँ देखिन्छन्।
contacts-none-found = तपाईंको खोजसँग मिल्ने कुनै सम्पर्क छैन।
contacts-starred = { $count ->
    [one] तारा लगाइएको सम्पर्क ({ $count })
   *[other] तारा लगाइएका सम्पर्कहरू ({ $count })
}
contacts-count = सम्पर्कहरू ({ $count })
contacts-col-name = नाम
contacts-col-email = इमेल
contacts-col-phone = फोन नम्बर
contacts-col-job = पद र कम्पनी
contacts-col-labels = लेबलहरू

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Katna लाई { $address } का सम्पर्कहरू पढ्न अनुमति दिनुहोस्।
contacts-allow-many = { $more ->
    [one] Katna लाई { $address } र थप { $more } खाताका सम्पर्कहरू पढ्न अनुमति दिनुहोस्।
   *[other] Katna लाई { $address } र थप { $more } खाताका सम्पर्कहरू पढ्न अनुमति दिनुहोस्।
}
contacts-allow-button = अनुमति दिनुहोस्

## A contact's page

contacts-back = सम्पर्कहरूमा फर्कनुहोस्
contacts-edit = सम्पादन गर्नुहोस्
contacts-delete = मेटाउनुहोस्
contacts-qr = QR कोडको रूपमा सेयर गर्नुहोस्
contacts-qr-about = सम्पर्क सुरक्षित गर्न फोनको क्यामेराले यसलाई स्क्यान गर्नुहोस्।
contacts-qr-too-long = यो सम्पर्कमा QR कोडमा अट्नका लागि धेरै विवरण छन्।
contacts-qr-done = भयो
contacts-deleted = { $name } मेटाइयो
contacts-added = { $name } लाई सम्पर्कमा थपियो
contacts-find-mail = मेल
contacts-details = सम्पर्क विवरण
contacts-saved-in = यहाँ सुरक्षित गरिएको
contacts-notes = टिपोटहरू
contacts-birthday = जन्मदिन
contacts-nickname = उपनाम
contacts-this-computer = यो कम्प्युटर
contacts-kind-home = घर
contacts-kind-work = कार्यालय
contacts-kind-mobile = मोबाइल
contacts-kind-other = अन्य
contacts-source-google = Google सम्पर्कहरू
contacts-source-microsoft = Outlook सम्पर्कहरू
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = सम्पर्क सिर्जना गर्नुहोस्
contacts-edit-title = सम्पर्क सम्पादन गर्नुहोस्
contacts-edit-save = सेभ गर्नुहोस्
contacts-edit-saving = सेभ गर्दै…
contacts-edit-cancel = रद्द गर्नुहोस्
contacts-saved = सम्पर्क सेभ गरियो
contacts-edit-save-to = यहाँ सेभ गर्नुहोस्
contacts-edit-changes-go-to = परिवर्तनहरू { $place } मा सेभ गरिन्छन्।
contacts-edit-given = पहिलो नाम
contacts-edit-family = थर
contacts-edit-company = कम्पनी
contacts-edit-job = पद
contacts-edit-email = इमेल
contacts-edit-phone = फोन
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = इमेल थप्नुहोस्
contacts-edit-add-phone = फोन थप्नुहोस्
contacts-edit-street = सडकको ठेगाना
contacts-edit-city = सहर
contacts-edit-postcode = हुलाक कोड
contacts-edit-country = देश
contacts-edit-birthday = जन्मदिन (YYYY-MM-DD)
contacts-edit-empty = पहिले नाम, इमेल वा फोन नम्बर थप्नुहोस्।
