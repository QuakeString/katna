# Katna Mail, Marathi (मराठी): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = संपर्क
contacts-frequent = वारंवार
contacts-other = इतर संपर्क
contacts-other-about = ज्यांना तुम्ही Gmail वरून मेल पाठवला आहे पण जतन केले नाही
contacts-other-email = ईमेल पाठवा
contacts-other-empty = इतर संपर्क नाहीत. Gmail वरून तुम्ही ज्यांना मेल पाठवता पण जतन करत नाही, ते येथे दिसतात.
contacts-other-allow = इतर संपर्क पाहण्यासाठी तुमच्या Gmail खात्यात पुन्हा साइन इन करा आणि Katna ला ते पाहण्याची परवानगी द्या.
contacts-labels = लेबल
contacts-label-options = लेबल पर्याय
contacts-label-rename = लेबलचे नाव बदला
contacts-label-email = सर्वांना मेल पाठवा
contacts-label-delete = लेबल हटवा
contacts-label-new = नवीन लेबल
contacts-label-name = लेबलचे नाव
contacts-label-button = लेबल
contacts-label-menu = असे लेबल लावा:
contacts-label-added = { $name } मध्ये जोडले
contacts-label-removed = { $name } मधून काढले
contacts-label-renamed = लेबलचे नाव बदलून { $name } केले
contacts-label-deleted = लेबल { $name } हटवले
contacts-label-no-email = या लेबलवरील कोणाकडेही ईमेल पत्ता नाही
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = खाती
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = संपर्क दाखवण्यासाठी पुन्हा साइन इन करा
contacts-account-signed-in = { $address } मध्ये पुन्हा साइन इन केले. तुमचे संपर्क आणत आहे…
contacts-account-sign-in-refused = { $provider } ने Katna ला आत येऊ दिले नाही. पुन्हा प्रयत्न करा, आणि तुमच्या संपर्कांचा ॲक्सेस द्या.
contacts-account-password = सर्व्हरने पासवर्ड स्वीकारला नाही. Yahoo, iCloud, Zoho आणि इतरांना ॲप पासवर्ड लागतो.
contacts-account-change-password = पासवर्ड बदला
contacts-account-change-password-tooltip = सेटिंग्ज > खाती उघडा
contacts-account-failed = संपर्क वाचता आले नाहीत.
# $reason is the server's own words, in English.
contacts-account-error = संपर्क वाचता आले नाहीत: { $reason }
contacts-account-none = कोणतीही पत्ता पुस्तिका सापडली नाही
# $reason is what the server answered, in English: "CardDAV https://dav.example.in/: status 404".
contacts-account-none-why = कोणतीही पत्ता पुस्तिका सापडली नाही: { $reason }
# A Gmail or Outlook account added with a password: its contacts need the
# provider's sign-in.
contacts-account-use-sign-in = { $provider } फक्त { $provider } ने साइन इन केलेल्या Katna लाच संपर्क दाखवते.
contacts-account-sign-in-with = { $provider } ने साइन इन करा
contacts-account-looking = संपर्क शोधत आहे…
contacts-account-try-again = पुन्हा प्रयत्न करा
contacts-account-try-again-tooltip = या खात्याचे संपर्क आता पुन्हा तपासा
contacts-account-fixing = काम सुरू आहे…
contacts-manage = दुरुस्त करा आणि व्यवस्थापित करा
contacts-merge = विलीन करा आणि दुरुस्त करा
contacts-merge-about = { $count ->
    [one] { $count } सूचना: एकाच व्यक्तीचे वाटणारे संपर्क
   *[other] { $count } सूचना: एकाच व्यक्तीचे वाटणारे संपर्क
}
contacts-merge-none = डुप्लिकेट नाहीत. समान नाव किंवा फोन नंबर असलेले संपर्क येथे दिसतील.
contacts-merge-count = { $count ->
    [one] { $count } संपर्क
   *[other] { $count } संपर्क
}
contacts-merge-all = सर्व विलीन करा
contacts-merge-button = विलीन करा
contacts-merge-dismiss = रद्द करा
contacts-merged = { $count ->
    [1] संपर्क विलीन केले
    [one] { $count } विलीनीकरणे पूर्ण झाली
   *[other] { $count } विलीनीकरणे पूर्ण झाली
}
contacts-import = आयात करा
contacts-export = निर्यात करा
contacts-import-file = vCard किंवा CSV फाइलमधून संपर्क आयात करा
contacts-imported = { $count ->
    [one] { $place } मध्ये { $count } संपर्क आयात केले
   *[other] { $place } मध्ये { $count } संपर्क आयात केले
}
contacts-imported-some = { $count ->
    [one] { $place } मध्ये { $count } संपर्क आयात केले; आधीच सेव्ह केलेले { $skipped } वगळले
   *[other] { $place } मध्ये { $count } संपर्क आयात केले; आधीच सेव्ह केलेले { $skipped } वगळले
}
contacts-import-none = { $name } मध्ये कोणतेही संपर्क आढळले नाहीत
contacts-import-all-saved = { $name } मधील सर्वजण आधीच सेव्ह केलेले आहेत
contacts-import-failed = { $name } वाचता आले नाही: { $error }
contacts-exported = { $count ->
    [one] { $path } मध्ये { $count } संपर्क निर्यात केले
   *[other] { $path } मध्ये { $count } संपर्क निर्यात केले
}
contacts-export-none = निर्यात करण्यासाठी संपर्क नाहीत
contacts-export-failed = संपर्क निर्यात करता आले नाहीत: { $error }
contacts-print = प्रिंट करा
contacts-print-title = संपर्क
contacts-print-none = प्रिंट करण्यासाठी संपर्क नाहीत
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = वाढदिवस: { $day }
contacts-print-nickname = टोपणनाव: { $name }
contacts-create = नवीन संपर्क

## Search and the list

contacts-search = संपर्क शोधा
contacts-loading = संपर्क लोड होत आहेत…
contacts-empty = अजून कोणतेही सेव्ह केलेले संपर्क नाहीत. Gmail, Outlook किंवा तुमच्या मेल सेवेत सेव्ह केलेले संपर्क येथे दिसतात.
contacts-empty-no-books = तुमच्या खात्यांचे संपर्क सिंक झाले की ते येथे दिसतील.
contacts-none-found = तुमच्या शोधाशी जुळणारा एकही संपर्क नाही.
contacts-starred = { $count ->
    [one] तारांकित संपर्क ({ $count })
   *[other] तारांकित संपर्क ({ $count })
}
contacts-count = संपर्क ({ $count })
contacts-col-name = नाव
contacts-col-email = ईमेल
contacts-col-phone = फोन नंबर
contacts-col-job = पदनाम आणि कंपनी
contacts-col-labels = लेबल

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Katna ला { $address } चे संपर्क वाचण्याची परवानगी द्या.
contacts-allow-many = { $more ->
    [one] Katna ला { $address } आणि आणखी { $more } खात्याचे संपर्क वाचण्याची परवानगी द्या.
   *[other] Katna ला { $address } आणि आणखी { $more } खात्यांचे संपर्क वाचण्याची परवानगी द्या.
}
contacts-allow-button = परवानगी द्या

## A contact's page

contacts-back = संपर्कांकडे परत जा
contacts-edit = संपादित करा
contacts-delete = हटवा
contacts-qr = QR कोड म्हणून शेअर करा
contacts-qr-about = संपर्क सेव्ह करण्यासाठी हे फोनच्या कॅमेऱ्याने स्कॅन करा.
contacts-qr-too-long = या संपर्कात QR कोडमध्ये मावण्यासाठी खूप जास्त तपशील आहेत.
contacts-qr-done = झाले
contacts-deleted = { $name } हटवले
contacts-added = { $name } संपर्कांमध्ये जोडले
contacts-find-mail = मेल
contacts-details = संपर्क तपशील
contacts-saved-in = येथे सेव्ह केले
contacts-notes = नोट्स
contacts-birthday = वाढदिवस
contacts-nickname = टोपणनाव
contacts-this-computer = हा संगणक
contacts-kind-home = घर
contacts-kind-work = कार्यालय
contacts-kind-mobile = मोबाइल
contacts-kind-other = इतर
contacts-source-google = Google संपर्क
contacts-source-microsoft = Outlook संपर्क
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = संपर्क तयार करा
contacts-edit-title = संपर्क संपादित करा
contacts-edit-save = सेव्ह करा
contacts-edit-saving = सेव्ह करत आहे…
contacts-edit-cancel = रद्द करा
contacts-saved = संपर्क सेव्ह केला
contacts-edit-save-to = येथे सेव्ह करा
contacts-edit-changes-go-to = बदल { $place } येथे सेव्ह केले जातात.
contacts-edit-given = पहिले नाव
contacts-edit-family = आडनाव
contacts-edit-company = कंपनी
contacts-edit-job = पदनाम
contacts-edit-email = ईमेल
contacts-edit-phone = फोन
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = ईमेल जोडा
contacts-edit-add-phone = फोन जोडा
contacts-edit-street = रस्त्याचा पत्ता
contacts-edit-city = शहर
contacts-edit-postcode = पिन कोड
contacts-edit-country = देश
contacts-edit-birthday = वाढदिवस (YYYY-MM-DD)
contacts-edit-empty = आधी नाव, ईमेल किंवा फोन नंबर जोडा.
