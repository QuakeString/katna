# Katna Mail, Telugu (తెలుగు).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = లేబుల్‌లు
nav-folders = ఫోల్డర్‌లు
nav-label-new = కొత్త లేబుల్‌ను క్రియేట్ చేయండి
nav-folder-new = కొత్త ఫోల్డర్‌ను క్రియేట్ చేయండి
nav-menu-check-mail = కొత్త మెయిల్‌ను చెక్ చేయండి
nav-menu-check-inbox = ఈ ఇన్‌బాక్స్‌ను చెక్ చేయండి
nav-unified-leave-out = ఏకీకృత ఇన్‌బాక్స్ నుండి మినహాయించండి
nav-unified-bring-back = ఏకీకృత ఇన్‌బాక్స్‌లోకి తిరిగి తీసుకురండి
nav-menu-sign-in-again = మళ్లీ సైన్ ఇన్ చేయండి
nav-menu-new-mail = ఈ ఖాతా నుండి కొత్త మెయిల్
nav-menu-account-settings = ఖాతా సెట్టింగ్‌లు
nav-account-checked = సింక్‌లో ఉంది · { $ago } చెక్ చేయబడింది
nav-account-in-sync = సింక్‌లో ఉంది
nav-account-connecting = కనెక్ట్ చేస్తోంది…
nav-account-offline = ఆఫ్‌లైన్, మళ్లీ ప్రయత్నిస్తోంది
nav-account-signed-out = { $provider } సైన్ ఇన్ గడువు ముగిసింది
nav-account-password-refused = పాస్‌వర్డ్ తిరస్కరించబడింది
nav-account-storage = { $total }లో { $used } ఉపయోగించబడింది
nav-menu-new-subfolder = లోపల కొత్త ఫోల్డర్
nav-menu-new-sublabel = లోపల కొత్త లేబుల్
nav-menu-rename = పేరు మార్చండి
nav-menu-delete = తొలగించండి
nav-menu-empty-trash = ట్రాష్‌ను ఖాళీ చేయండి
nav-account-unnamed = ఖాతా { $number }
nav-all-accounts = అన్ని ఖాతాలు
nav-expand = ఫోల్డర్‌లను చూపండి
nav-collapse = ఫోల్డర్‌లను దాచండి
storage-used = { $total }లో { $percent }% ఉపయోగించబడింది
storage-used-detail = { $address }: { $total }లో { $used } ఉపయోగించబడింది

## Special folders (the user's own folders keep their names)

folder-inbox = ఇన్‌బాక్స్
folder-starred = నక్షత్రం ఉంచినవి
folder-snoozed = స్నూజ్ చేసినవి
folder-unread = చదవనివి
folder-important = ముఖ్యమైనవి
folder-drafts = డ్రాఫ్ట్‌లు
folder-sent = పంపినవి
folder-archive = ఆర్కైవ్
folder-spam = స్పామ్
folder-trash = ట్రాష్
folder-all-mail = అన్ని మెయిల్స్
folder-scheduled = షెడ్యూల్ చేసినవి
folder-waiting = రిప్లయి కోసం వేచి ఉన్నవి
folder-waiting-short = వేచి ఉన్నవి
folder-reminders = రిమైండర్‌లు
folder-outbox = అవుట్‌బాక్స్
folder-activity = యాక్టివిటీ
folder-not-on-account = ఈ అకౌంట్‌లో అలాంటి ఫోల్డర్ లేదు.

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = కొత్త లేబుల్
label-folder-new-title = కొత్త ఫోల్డర్
label-prompt = దయచేసి కొత్త లేబుల్ పేరును ఎంటర్ చేయండి:
label-folder-prompt = దయచేసి కొత్త ఫోల్డర్ పేరును ఎంటర్ చేయండి:
label-name-hint = లేబుల్ పేరు
label-folder-name-hint = ఫోల్డర్ పేరు
label-nest = లేబుల్‌ను దీని కింద ఉంచండి:
label-folder-nest = ఫోల్డర్‌ను దీని కింద ఉంచండి:
label-cancel = రద్దు చేయండి
label-create = క్రియేట్ చేయండి
label-creating = క్రియేట్ చేస్తోంది…
label-created = “{ $name }” లేబుల్ క్రియేట్ చేయబడింది.
label-folder-created = “{ $name }” ఫోల్డర్ క్రియేట్ చేయబడింది.
label-rename-title = లేబుల్ పేరు మార్చండి
label-folder-rename-title = ఫోల్డర్ పేరు మార్చండి
label-rename = పేరు మార్చండి
label-renaming = పేరు మారుస్తోంది…
label-renamed = లేబుల్ పేరు “{ $name }”గా మార్చబడింది.
label-folder-renamed = ఫోల్డర్ పేరు “{ $name }”గా మార్చబడింది.

## Deleting a folder or label (asked first)

folder-delete-title = “{ $name }”ను తొలగించాలా?
folder-delete-body = { $count ->
    [0] ఇందులో మెయిల్ ఏదీ లేదు. ఫోల్డర్ సర్వర్ నుండి తీసివేయబడుతుంది, కాబట్టి వెబ్‌మెయిల్, మీ ఫోన్ నుండి కూడా అది పోతుంది.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] ఇందులోని { $count } సంభాషణ ట్రాష్‌కు వెళ్తుంది, కాబట్టి దాన్ని మళ్లీ పొందవచ్చు.
           *[other] ఇందులోని { $count } సంభాషణలు ట్రాష్‌కు వెళ్తాయి, కాబట్టి వాటిని మళ్లీ పొందవచ్చు.
        }
       *[message] { $count ->
            [one] ఇందులోని { $count } మెసేజ్ ట్రాష్‌కు వెళ్తుంది, కాబట్టి దాన్ని మళ్లీ పొందవచ్చు.
           *[other] ఇందులోని { $count } మెసేజ్‌లు ట్రాష్‌కు వెళ్తాయి, కాబట్టి వాటిని మళ్లీ పొందవచ్చు.
        }
    } ఫోల్డర్ సర్వర్ నుండి తీసివేయబడుతుంది, కాబట్టి వెబ్‌మెయిల్, మీ ఫోన్ నుండి కూడా అది పోతుంది.
}
folder-delete-forever-body = { $count ->
    [0] ఇందులో మెయిల్ ఏదీ లేదు. ఫోల్డర్ సర్వర్ నుండి తీసివేయబడుతుంది, కాబట్టి వెబ్‌మెయిల్, మీ ఫోన్ నుండి కూడా అది పోతుంది.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] ఇందులోని { $count } సంభాషణ శాశ్వతంగా తొలగించబడుతుంది; ఈ ఖాతాకు ట్రాష్ లేదు.
           *[other] ఇందులోని { $count } సంభాషణలు శాశ్వతంగా తొలగించబడతాయి; ఈ ఖాతాకు ట్రాష్ లేదు.
        }
       *[message] { $count ->
            [one] ఇందులోని { $count } మెసేజ్ శాశ్వతంగా తొలగించబడుతుంది; ఈ ఖాతాకు ట్రాష్ లేదు.
           *[other] ఇందులోని { $count } మెసేజ్‌లు శాశ్వతంగా తొలగించబడతాయి; ఈ ఖాతాకు ట్రాష్ లేదు.
        }
    } ఫోల్డర్ సర్వర్ నుండి తీసివేయబడుతుంది, కాబట్టి వెబ్‌మెయిల్, మీ ఫోన్ నుండి కూడా అది పోతుంది.
}
folder-delete-label-body = లేబుల్ తీసివేయబడుతుంది. దాని మెయిల్ అన్ని మెయిల్స్‌లో, దాని ఇతర లేబుల్‌లలో అలాగే ఉంటుంది.
folder-delete-confirm = ఫోల్డర్‌ను తొలగించండి
folder-delete-label-confirm = లేబుల్‌ను తొలగించండి
folder-deleted = “{ $name }” ఫోల్డర్ తొలగించబడింది
label-deleted = “{ $name }” లేబుల్ తొలగించబడింది
