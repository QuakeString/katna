# Katna Mail, Telugu (తెలుగు).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = ఫైల్‌లను వెతకండి

## Left side (and chips on a phone)

files-all = అన్ని ఫైల్‌లు
files-pictures = చిత్రాలు
files-pdfs = PDFలు
files-documents = డాక్యుమెంట్‌లు
files-sheets = స్ప్రెడ్‌షీట్‌లు
files-slides = స్లైడ్‌లు
files-other = ఇతరాలు
files-accounts = ఖాతాలు
files-drives = డ్రైవ్‌లు
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = నాతో షేర్ చేసినవి
files-shown = చూపబడేవి
files-received = స్వీకరించినవి
files-sent = నేను పంపినవి

## Over the files

files-count = { $count ->
    [one] { $count } ఫైల్ · { $size }
   *[other] { $count } ఫైల్‌లు · { $size }
}
files-anyone = ఎవరైనా
files-from-person = { $name } నుండి
files-time-any = ఎప్పుడైనా
files-time-today = ఈరోజు
files-time-yesterday = నిన్న
files-time-this-week = ఈ వారం
files-time-last-week = గత వారం
files-time-this-month = ఈ నెల
files-time-last-month = గత నెల
files-time-between = { $first } – { $last }
files-time-hint = ఒక రోజును క్లిక్ చేయండి, లేదా రోజుల మీదుగా లాగండి
files-time-summary = { $count ->
    [one] { $days } · { $count } ఫైల్
   *[other] { $days } · { $count } ఫైల్‌లు
}
files-time-clear = క్లియర్ చేయండి
files-time-month-back = మునుపటి నెల
files-time-month-on = తదుపరి నెల
files-time-wheel = ఈ తేదీలను జరపడానికి స్క్రోల్ చేయండి, వ్యవధి అలాగే ఉంటుంది
files-sort-newest = కొత్తవి ముందు
files-sort-oldest = పాతవి ముందు
files-sort-largest = పెద్దవి ముందు
files-sort-name = పేరు ప్రకారం
files-grid = కార్డ్‌లు
files-list = లిస్ట్
files-this-week = ఈ వారం
files-undated = తేదీ లేదు
files-me = నేను
files-no-subject = (సబ్జెక్ట్ లేదు)
files-loading = మీ మెయిల్ నుండి ఫైల్‌లను సేకరిస్తోంది…
files-empty = మీ మెయిల్‌లోని ఫైల్‌లు ఇక్కడ కనిపిస్తాయి.
files-none-match = సరిపోలే ఫైల్‌లు లేవు.
files-load-failed = ఫైల్‌లను చదవడం విఫలమైంది: { $error }

## A file's menu and buttons

files-open = తెరవండి
files-open-with = దీనితో తెరవండి…
files-save = సేవ్ చేయండి…
files-show-mail = మెయిల్‌ను చూపండి
files-mail-window = మెయిల్‌ను కొత్త విండోలో తెరవండి
files-forward = ఫైల్‌ను ఫార్వర్డ్ చేయండి
files-from-them = { $name } నుండి ఫైల్‌లు
files-copy-name = ఫైల్ పేరును కాపీ చేయండి
files-name-copied = ఫైల్ పేరు కాపీ అయింది
files-downloading = మెయిల్‌ను డౌన్‌లోడ్ చేస్తోంది…
files-download-failed = ఈ మెయిల్‌ను డౌన్‌లోడ్ చేయడం సాధ్యం కాలేదు.

## A cloud drive in place of the mail files

files-drive-mine = నా డ్రైవ్
files-drive-mine-onedrive = నా ఫైల్‌లు
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] 1 ఫైల్
       *[other] { $files } ఫైల్‌లు
    }
    [one] 1 ఫోల్డర్ · { $files ->
        [one] 1 ఫైల్
       *[other] { $files } ఫైల్‌లు
    }
   *[other] { $folders } ఫోల్డర్‌లు · { $files ->
        [one] 1 ఫైల్
       *[other] { $files } ఫైల్‌లు
    }
}
files-drive-folders = ఫోల్డర్‌లు
files-drive-files = ఫైల్‌లు
files-drive-folder = ఫోల్డర్
files-drive-meta = { $what } · { $date }న ఎడిట్ చేయబడింది
files-drive-as-link = { $what } · లింక్‌గా
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = తెస్తోంది…
files-drive-loading = డ్రైవ్‌ను తెరుస్తోంది…
files-drive-empty = ఈ ఫోల్డర్ ఖాళీగా ఉంది.
files-drive-unreachable = { $drive }ను చేరుకోలేకపోతున్నాము.
files-drive-try-again = మళ్లీ ప్రయత్నించండి
files-drive-needs-permission = ఈ డ్రైవ్‌ను చూపడానికి Katnaకు ఒకసారి మీ అనుమతి కావాలి. మళ్లీ సైన్ ఇన్ చేసి, మీ ఫైల్‌లను చూడటానికి Katnaను అనుమతించండి.
files-drive-allow = అనుమతించండి
files-drive-allow-failed = సైన్ ఇన్ పూర్తి కాలేదు, కాబట్టి డ్రైవ్ మూసివేయబడే ఉంటుంది.
files-drive-attach = అటాచ్ చేయండి
files-drive-more = మరిన్ని
files-drive-download = డౌన్‌లోడ్ చేయండి…
files-drive-open-web = { $drive }లో తెరవండి
files-drive-copy-link = లింక్‌ను కాపీ చేయండి
files-drive-link-copied = లింక్ కాపీ అయింది
files-drive-share = షేర్ చేయండి…
files-drive-rename = పేరు మార్చండి
files-drive-trash = ట్రాష్‌కు తరలించండి
files-drive-trashed = “{ $name }” { $drive } ట్రాష్‌లో ఉంది
files-drive-renamed = “{ $name }”గా పేరు మార్చబడింది
files-drive-getting = { $drive } నుండి { $name }ను తెస్తోంది…
files-drive-get-failed = { $name }ను తేలేకపోయాము: { $error }
files-drive-upload = అప్‌లోడ్ చేయండి
files-drive-upload-files = ఫైల్‌లను అప్‌లోడ్ చేయండి
files-drive-upload-folder = ఫోల్డర్‌ను అప్‌లోడ్ చేయండి
files-drive-upload-failed = { $name }ను అప్‌లోడ్ చేయలేకపోయాము: { $error }
files-drive-upload-needs = అప్‌లోడ్ చేయడానికి Katnaకు ఒకసారి మీ అనుమతి కావాలి: సెట్టింగ్‌లు › డిఫాల్ట్ యాప్‌లు › ఫైల్‌ల పేజీలో అనుమతించండి నొక్కండి.

## The Share dialog of a drive file or folder

files-share-title = “{ $name }”ను షేర్ చేయండి
files-share-add = పేరు లేదా అడ్రస్‌తో వ్యక్తులను జోడించండి
files-share-not-address = “{ $text }” ఈమెయిల్ అడ్రస్ కాదు
files-share-notify = { $drive } వారికి ఈమెయిల్ కూడా పంపనివ్వండి
files-share-people = యాక్సెస్ ఉన్న వ్యక్తులు
files-share-general = సాధారణ యాక్సెస్
files-share-loading = ఎవరికి యాక్సెస్ ఉందో చదువుతోంది…
files-share-restricted = పరిమితం
files-share-restricted-about = యాక్సెస్ ఉన్నవారు మాత్రమే లింక్‌తో దీన్ని తెరవగలరు
files-share-anyone = లింక్ ఉన్న ఎవరైనా
files-share-anyone-can = { $role ->
    [editor] లింక్ ఉన్న ఎవరైనా ఎడిట్ చేయవచ్చు
    [commenter] లింక్ ఉన్న ఎవరైనా కామెంట్ చేయవచ్చు
   *[viewer] లింక్ ఉన్న ఎవరైనా చూడవచ్చు
}
files-share-anyone-about = { $role ->
    [editor] ఇంటర్నెట్‌లో లింక్ ఉన్న ఎవరైనా ఎడిట్ చేయవచ్చు
    [commenter] ఇంటర్నెట్‌లో లింక్ ఉన్న ఎవరైనా కామెంట్ చేయవచ్చు
   *[viewer] ఇంటర్నెట్‌లో లింక్ ఉన్న ఎవరైనా చూడవచ్చు
}
files-share-role-owner = యజమాని
files-share-role-editor = ఎడిటర్
files-share-role-commenter = కామెంటర్
files-share-role-viewer = వ్యూయర్
files-share-you = { $name } (మీరు)
files-share-domain = { $domain }లోని అందరూ
files-share-inherited = ఇది ఉన్న ఫోల్డర్ నుండి యాక్సెస్
files-share-remove = యాక్సెస్ తీసివేయండి
files-share-copy-link = లింక్‌ను కాపీ చేయండి
files-share-share = షేర్ చేయండి
files-share-done = పూర్తయింది
files-share-sharing = షేర్ చేస్తోంది…
files-share-shared = { $count ->
    [one] 1 వ్యక్తితో షేర్ చేయబడింది
   *[other] { $count } మందితో షేర్ చేయబడింది
}
files-share-refused = { $drive } { $addresses }తో షేర్ చేయలేకపోయింది
files-share-failed = షేరింగ్‌ను మార్చలేకపోయాము: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] 1 అంశాన్ని అప్‌లోడ్ చేస్తోంది
   *[other] { $count } అంశాలను అప్‌లోడ్ చేస్తోంది
}
files-tray-done = { $count ->
    [one] 1 అప్‌లోడ్ పూర్తయింది
   *[other] { $count } అప్‌లోడ్‌లు పూర్తయ్యాయి
}
files-tray-some-failed = { $done } అప్‌లోడ్ అయ్యాయి, { $failed } విఫలమయ్యాయి
files-tray-minutes-left = { $minutes ->
    [one] సుమారు ఒక నిమిషం మిగిలి ఉంది
   *[other] సుమారు { $minutes } నిమిషాలు మిగిలి ఉన్నాయి
}
files-tray-seconds-left = ఒక నిమిషం కంటే తక్కువ మిగిలి ఉంది
files-tray-starting = ప్రారంభిస్తోంది…
files-tray-cancel-all = అన్నీ రద్దు చేయండి
files-tray-cancel = రద్దు చేయండి
files-tray-fold = లిస్ట్‌ను దాచండి
files-tray-unfold = లిస్ట్‌ను చూపండి
files-tray-close = మూసివేయండి
files-tray-progress = { $place } · { $size }లో { $sent }
files-tray-in = { $place }లో
files-tray-cancelled = రద్దు చేయబడింది
