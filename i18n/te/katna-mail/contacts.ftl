# Katna Mail, Telugu (తెలుగు): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = కాంటాక్ట్‌లు
contacts-frequent = తరచుగా
contacts-other = ఇతర కాంటాక్ట్‌లు
contacts-other-about = మీరు Gmail నుండి మెయిల్ చేసి, సేవ్ చేయని వ్యక్తులు
contacts-other-email = ఇమెయిల్ పంపండి
contacts-other-empty = ఇతర కాంటాక్ట్‌లు లేవు. మీరు Gmail నుండి మెయిల్ చేసి, సేవ్ చేయని వ్యక్తులు ఇక్కడ కనిపిస్తారు.
contacts-other-allow = ఇతర కాంటాక్ట్‌లను చూడటానికి, మీ Gmail ఖాతాలో మళ్లీ సైన్ ఇన్ చేసి, వాటిని చూడటానికి Katna‌కు అనుమతి ఇవ్వండి.
contacts-labels = లేబుల్‌లు
contacts-label-options = లేబుల్ ఎంపికలు
contacts-label-rename = లేబుల్ పేరు మార్చండి
contacts-label-email = అందరికీ మెయిల్ పంపండి
contacts-label-delete = లేబుల్‌ను తొలగించండి
contacts-label-new = కొత్త లేబుల్
contacts-label-name = లేబుల్ పేరు
contacts-label-button = లేబుల్
contacts-label-menu = ఇలా లేబుల్ చేయండి:
contacts-label-added = { $name }కు జోడించబడింది
contacts-label-removed = { $name } నుండి తీసివేయబడింది
contacts-label-renamed = లేబుల్ పేరు { $name }గా మార్చబడింది
contacts-label-deleted = లేబుల్ { $name } తొలగించబడింది
contacts-label-no-email = ఈ లేబుల్‌లో ఎవరికీ ఇమెయిల్ చిరునామా లేదు
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = ఖాతాలు
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = కాంటాక్ట్‌లను చూపడానికి మళ్లీ సైన్ ఇన్ చేయండి
contacts-account-signed-in = { $address }కు మళ్లీ సైన్ ఇన్ అయింది. మీ కాంటాక్ట్‌లను తెస్తోంది…
contacts-account-sign-in-refused = { $provider } Katnaను లోపలికి అనుమతించలేదు. మళ్లీ ట్రై చేసి, మీ కాంటాక్ట్‌లకు యాక్సెస్ అనుమతించండి.
contacts-account-password = సర్వర్ పాస్‌వర్డ్‌ను అంగీకరించలేదు. Yahoo, iCloud, Zoho మరియు ఇతరాలకు యాప్ పాస్‌వర్డ్ అవసరం.
contacts-account-change-password = పాస్‌వర్డ్ మార్చండి
contacts-account-change-password-tooltip = సెట్టింగ్‌లు > ఖాతాలు తెరవండి
contacts-account-failed = కాంటాక్ట్‌లను చదవలేకపోయాము.
# $reason is the server's own words, in English.
contacts-account-error = కాంటాక్ట్‌లను చదవలేకపోయాము: { $reason }
contacts-account-none = అడ్రస్ బుక్ ఏదీ కనుగొనబడలేదు
# $reason is what the server answered, in English: "CardDAV https://dav.example.in/: status 404".
contacts-account-none-why = అడ్రస్ బుక్ ఏదీ కనుగొనబడలేదు: { $reason }
# A Gmail or Outlook account added with a password: its contacts need the
# provider's sign-in.
contacts-account-use-sign-in = { $provider }తో సైన్ ఇన్ అయిన Katnaకు మాత్రమే { $provider } కాంటాక్ట్‌లను చూపుతుంది.
contacts-account-sign-in-with = { $provider }తో సైన్ ఇన్ చేయండి
contacts-account-looking = కాంటాక్ట్‌ల కోసం వెతుకుతోంది…
contacts-account-try-again = మళ్లీ ట్రై చేయండి
contacts-account-try-again-tooltip = ఈ ఖాతా కాంటాక్ట్‌లను ఇప్పుడు మళ్లీ తనిఖీ చేయండి
contacts-account-fixing = సరిచేస్తోంది…
contacts-manage = పరిష్కరించండి మరియు నిర్వహించండి
contacts-merge = విలీనం చేసి పరిష్కరించండి
contacts-merge-about = { $count ->
    [one] { $count } సూచన: ఒకే వ్యక్తిలా కనిపించే కాంటాక్ట్‌లు
   *[other] { $count } సూచన: ఒకే వ్యక్తిలా కనిపించే కాంటాక్ట్‌లు
}
contacts-merge-none = డూప్లికేట్‌లు లేవు. ఒకే పేరు లేదా ఫోన్ నంబర్ ఉన్న కాంటాక్ట్‌లు ఇక్కడ కనిపిస్తాయి.
contacts-merge-count = { $count ->
    [one] { $count } కాంటాక్ట్‌లు
   *[other] { $count } కాంటాక్ట్‌లు
}
contacts-merge-all = అన్నింటినీ విలీనం చేయండి
contacts-merge-button = విలీనం చేయండి
contacts-merge-dismiss = తీసివేయండి
contacts-merged = { $count ->
    [1] కాంటాక్ట్‌లు విలీనం అయ్యాయి
    [one] { $count } విలీనాలు పూర్తయ్యాయి
   *[other] { $count } విలీనాలు పూర్తయ్యాయి
}
contacts-import = దిగుమతి చేయండి
contacts-export = ఎగుమతి చేయండి
contacts-import-file = vCard లేదా CSV ఫైల్ నుండి కాంటాక్ట్‌లను దిగుమతి చేయండి
contacts-imported = { $count ->
    [one] { $place }లో { $count } కాంటాక్ట్‌లు దిగుమతి అయ్యాయి
   *[other] { $place }లో { $count } కాంటాక్ట్‌లు దిగుమతి అయ్యాయి
}
contacts-imported-some = { $count ->
    [one] { $place }లో { $count } కాంటాక్ట్‌లు దిగుమతి అయ్యాయి; ఇప్పటికే సేవ్ చేసిన { $skipped } వదిలివేయబడ్డాయి
   *[other] { $place }లో { $count } కాంటాక్ట్‌లు దిగుమతి అయ్యాయి; ఇప్పటికే సేవ్ చేసిన { $skipped } వదిలివేయబడ్డాయి
}
contacts-import-none = { $name }లో కాంటాక్ట్‌లు ఏవీ కనుగొనబడలేదు
contacts-import-all-saved = { $name }లోని అందరూ ఇప్పటికే సేవ్ అయి ఉన్నారు
contacts-import-failed = { $name }ను చదవడం సాధ్యం కాలేదు: { $error }
contacts-exported = { $count ->
    [one] { $path }కు { $count } కాంటాక్ట్‌లు ఎగుమతి అయ్యాయి
   *[other] { $path }కు { $count } కాంటాక్ట్‌లు ఎగుమతి అయ్యాయి
}
contacts-export-none = ఎగుమతి చేయడానికి కాంటాక్ట్‌లు లేవు
contacts-export-failed = కాంటాక్ట్‌లను ఎగుమతి చేయడం సాధ్యం కాలేదు: { $error }
contacts-print = ప్రింట్ చేయండి
contacts-print-title = కాంటాక్ట్‌లు
contacts-print-none = ప్రింట్ చేయడానికి కాంటాక్ట్‌లు లేవు
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = పుట్టినరోజు: { $day }
contacts-print-nickname = ముద్దుపేరు: { $name }
contacts-create = కాంటాక్ట్‌ను సృష్టించండి

## Search and the list

contacts-search = కాంటాక్ట్‌లను శోధించండి
contacts-loading = కాంటాక్ట్‌లు లోడ్ అవుతున్నాయి…
contacts-empty = ఇంకా సేవ్ చేసిన కాంటాక్ట్‌లు లేవు. మీరు Gmail, Outlook లేదా మీ మెయిల్ సేవలో సేవ్ చేసిన కాంటాక్ట్‌లు ఇక్కడ కనిపిస్తాయి.
contacts-empty-no-books = మీ ఖాతాల కాంటాక్ట్‌లు సింక్ అయిన తర్వాత ఇక్కడ కనిపిస్తాయి.
contacts-none-found = మీ శోధనకు సరిపోలే కాంటాక్ట్‌లు లేవు.
contacts-starred = { $count ->
    [one] నక్షత్రం ఉంచిన కాంటాక్ట్ ({ $count })
   *[other] నక్షత్రం ఉంచిన కాంటాక్ట్‌లు ({ $count })
}
contacts-count = కాంటాక్ట్‌లు ({ $count })
contacts-col-name = పేరు
contacts-col-email = ఇమెయిల్
contacts-col-phone = ఫోన్ నంబర్
contacts-col-job = ఉద్యోగ శీర్షిక & కంపెనీ
contacts-col-labels = లేబుల్‌లు

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = { $address } యొక్క కాంటాక్ట్‌లను చదవడానికి Katna‌ను అనుమతించండి.
contacts-allow-many = { $more ->
    [one] { $address } మరియు మరో { $more } ఖాతా యొక్క కాంటాక్ట్‌లను చదవడానికి Katna‌ను అనుమతించండి.
   *[other] { $address } మరియు మరో { $more } ఖాతాల కాంటాక్ట్‌లను చదవడానికి Katna‌ను అనుమతించండి.
}
contacts-allow-button = అనుమతించండి

## A contact's page

contacts-back = కాంటాక్ట్‌లకు తిరిగి వెళ్లండి
contacts-edit = ఎడిట్ చేయండి
contacts-delete = తొలగించండి
contacts-qr = QR కోడ్‌గా షేర్ చేయండి
contacts-qr-about = కాంటాక్ట్‌ను సేవ్ చేయడానికి దీన్ని ఫోన్ కెమెరాతో స్కాన్ చేయండి.
contacts-qr-too-long = QR కోడ్‌లో ఇమడడానికి ఈ కాంటాక్ట్‌లో చాలా ఎక్కువ వివరాలు ఉన్నాయి.
contacts-qr-done = పూర్తయింది
contacts-deleted = { $name } తొలగించబడింది
contacts-added = { $name }ను కాంటాక్ట్‌లకు జోడించారు
contacts-find-mail = మెయిల్
contacts-details = కాంటాక్ట్ వివరాలు
contacts-saved-in = సేవ్ చేసిన చోటు
contacts-notes = నోట్స్
contacts-birthday = పుట్టినరోజు
contacts-nickname = ముద్దుపేరు
contacts-this-computer = ఈ కంప్యూటర్
contacts-kind-home = ఇల్లు
contacts-kind-work = కార్యాలయం
contacts-kind-mobile = మొబైల్
contacts-kind-other = ఇతర
contacts-source-google = Google కాంటాక్ట్‌లు
contacts-source-microsoft = Outlook కాంటాక్ట్‌లు
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = కాంటాక్ట్‌ను సృష్టించండి
contacts-edit-title = కాంటాక్ట్‌ను ఎడిట్ చేయండి
contacts-edit-save = సేవ్ చేయండి
contacts-edit-saving = సేవ్ చేస్తోంది…
contacts-edit-cancel = రద్దు చేయండి
contacts-saved = కాంటాక్ట్ సేవ్ చేయబడింది
contacts-edit-save-to = ఇక్కడ సేవ్ చేయండి
contacts-edit-changes-go-to = మార్పులు { $place }లో సేవ్ చేయబడతాయి.
contacts-edit-given = మొదటి పేరు
contacts-edit-family = ఇంటి పేరు
contacts-edit-company = కంపెనీ
contacts-edit-job = ఉద్యోగ శీర్షిక
contacts-edit-email = ఇమెయిల్
contacts-edit-phone = ఫోన్
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = ఇమెయిల్‌ను జోడించండి
contacts-edit-add-phone = ఫోన్‌ను జోడించండి
contacts-edit-street = వీధి చిరునామా
contacts-edit-city = నగరం
contacts-edit-postcode = పిన్ కోడ్
contacts-edit-country = దేశం
contacts-edit-birthday = పుట్టినరోజు (YYYY-MM-DD)
contacts-edit-empty = ముందుగా పేరు, ఇమెయిల్ లేదా ఫోన్ నంబర్‌ను జోడించండి.
