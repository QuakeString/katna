# Katna Mail, Nepali (नेपाली): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = सम्पर्कहरू
contacts-frequent = बारम्बार
contacts-labels = लेबलहरू
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
contacts-deleted = { $name } मेटाइयो
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
