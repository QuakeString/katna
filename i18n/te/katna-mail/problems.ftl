# Katna Mail, Telugu (తెలుగు).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = మెయిల్ సర్వర్
problems-signed-out = { $provider } Katnaను { $address } నుండి సైన్ అవుట్ చేసింది. మెయిల్ సింక్ ఆగిపోయింది.
problems-password-refused = { $provider } { $address } పాస్‌వర్డ్‌ను తిరస్కరించింది. అది మారి ఉండవచ్చు.
problems-no-answer = { $provider } { $address } కోసం స్పందించడం లేదు. Katna ప్రయత్నిస్తూనే ఉంటుంది.
problems-offline = మీరు ఆఫ్‌లైన్‌లో ఉన్నారు. మీ మెయిల్ ఇక్కడే ఉంది, మీరు పంపే మెయిల్ మీరు తిరిగి ఆన్‌లైన్‌కు వచ్చే వరకు వేచి ఉంటుంది.
problems-accounts-need-you = { $count ->
    [one] { $count } ఖాతాకు మీ చర్య అవసరం
   *[other] { $count } ఖాతాలకు మీ చర్య అవసరం
}
problems-show = చూపండి
problems-later = తర్వాత
problems-new-password = కొత్త పాస్‌వర్డ్
problems-try-again = మళ్లీ ప్రయత్నించండి

## The New password card

problems-password-title = కొత్త పాస్‌వర్డ్
problems-password-detail = { $provider } { $address } కోసం సేవ్ చేసిన పాస్‌వర్డ్‌ను తిరస్కరించింది. కొత్తదాన్ని టైప్ చేయండి; ఉంచే ముందు Katna దాన్ని చెక్ చేస్తుంది.
problems-password-placeholder = పాస్‌వర్డ్
problems-password-show = పాస్‌వర్డ్‌ను చూపండి
problems-password-hide = పాస్‌వర్డ్‌ను దాచండి
problems-password-cancel = రద్దు చేయండి
problems-password-save = సేవ్ చేయండి
problems-password-checking = చెక్ చేస్తోంది…
problems-password-refused-again = { $provider } ఈ పాస్‌వర్డ్‌ను కూడా తిరస్కరించింది. చెక్ చేసి మళ్లీ ప్రయత్నించండి.
problems-password-saved = { $address } కోసం పాస్‌వర్డ్ సేవ్ చేయబడింది. మీ మెయిల్‌ను తెస్తోంది…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = { $address } మెయిల్ సర్వర్ { $count ->
    [one] ఒక మెసేజ్‌ను తరలించడాన్ని అంగీకరించలేదు, కాబట్టి అది మునుపటి చోటికే తిరిగి వచ్చింది.
   *[other] { $count } మెసేజ్‌లను తరలించడాన్ని అంగీకరించలేదు, కాబట్టి అవి మునుపటి చోటికే తిరిగి వచ్చాయి.
}
problems-refused-flags = { $address } మెయిల్ సర్వర్ { $count ->
    [one] ఒక మెసేజ్‌ను (చదివినది, నక్షత్రం…) గుర్తు పెట్టడాన్ని అంగీకరించలేదు, కాబట్టి అది మునుపటిలాగే ఉంది.
   *[other] { $count } మెసేజ్‌లను (చదివినవి, నక్షత్రం…) గుర్తు పెట్టడాన్ని అంగీకరించలేదు, కాబట్టి అవి మునుపటిలాగే ఉన్నాయి.
}
problems-refused-label = { $address } మెయిల్ సర్వర్ { $count ->
    [one] ఒక మెసేజ్ లేబుల్‌లను మార్చడాన్ని అంగీకరించలేదు, కాబట్టి అది మునుపటిలాగే ఉంది.
   *[other] { $count } మెసేజ్‌ల లేబుల్‌లను మార్చడాన్ని అంగీకరించలేదు, కాబట్టి అవి మునుపటిలాగే ఉన్నాయి.
}
problems-refused-delete = { $address } మెయిల్ సర్వర్ { $count ->
    [one] ఒక మెసేజ్‌ను తొలగించడాన్ని అంగీకరించలేదు, కాబట్టి అది తిరిగి వచ్చింది.
   *[other] { $count } మెసేజ్‌లను తొలగించడాన్ని అంగీకరించలేదు, కాబట్టి అవి తిరిగి వచ్చాయి.
}
problems-refused-other = { $address } మెయిల్ సర్వర్ { $count ->
    [one] ఒక మార్పును అంగీకరించలేదు, కాబట్టి Katna దాన్ని మునుపటిలాగే ఉంచింది.
   *[other] { $count } మార్పులను అంగీకరించలేదు, కాబట్టి Katna వాటిని మునుపటిలాగే ఉంచింది.
}
problems-details = వివరాలు

## Katna's background service (katna-daemon) isn't running

service-starting = Katna బ్యాక్‌గ్రౌండ్ సర్వీస్‌ను ప్రారంభిస్తోంది…
service-failed = Katna బ్యాక్‌గ్రౌండ్ సర్వీస్ ప్రారంభం కావడం లేదు, కాబట్టి మెయిల్ సింక్ కావడం లేదు.
service-start-again = మళ్లీ ప్రారంభించండి
service-started-again = Katna బ్యాక్‌గ్రౌండ్ సర్వీస్ ఆగిపోయింది, మళ్లీ ప్రారంభించబడింది.
service-details-title = సర్వీస్ ఎందుకు ప్రారంభం కావడం లేదు
service-details-body = దీన్ని కాపీ చేసి మీ రిపోర్ట్‌తో పంపండి. ఇందులో మెయిల్ గానీ పాస్‌వర్డ్‌లు గానీ ఉండవు.
service-details-copy = కాపీ చేయండి
service-details-close = మూసివేయండి
service-not-running = Katna బ్యాక్‌గ్రౌండ్ సర్వీస్ నడవడం లేదు.
service-no-answer = Katna బ్యాక్‌గ్రౌండ్ సర్వీస్ స్పందించలేదు: { $error }
service-no-session = D-Bus సెషన్ లేదు: { $error }

## Safe mode: an update left Katna's background service unable to start

safe-line = అప్‌డేట్‌లో సమస్య వల్ల Katna సేఫ్ మోడ్‌లో ఉంది, కాబట్టి మెయిల్ సింక్ కావడం లేదు.
safe-try-again = మళ్లీ ప్రయత్నించండి
safe-restore = పునరుద్ధరించండి
safe-restoring = { $when } నాటి మీ డేటాను పునరుద్ధరిస్తోంది…
safe-restored = { $when } నాటి మీ డేటా పునరుద్ధరించబడింది. ఇంతకు ముందు ఉన్నది ఒక ఫోల్డర్‌లో ఉంచబడింది.
safe-show-folder = ఫోల్డర్‌ను చూపండి
safe-restore-failed = మీ డేటాను పునరుద్ధరించలేకపోయాము: { $error }
safe-restore-title = అప్‌డేట్‌కు ముందు నాటి మీ డేటాను పునరుద్ధరించాలా?
safe-restore-body = మీరు ఎంచుకున్న కాపీకి Katna తిరిగి వెళ్తుంది. దాని తర్వాత వచ్చిన మెయిల్ మీ ఖాతాల నుండి మళ్లీ డౌన్‌లోడ్ అవుతుంది.
safe-restore-none = ఇంకా కాపీలు లేవు. ప్రతి అప్‌డేట్ మీ డేటాను మార్చే ముందు Katna ఒక కాపీ తయారు చేస్తుంది.
safe-restore-keep = పంపని మెయిల్, డ్రాఫ్ట్‌లు, ఇంకా సింక్ కాని మార్పులతో సహా ఇప్పుడు ఉన్నదంతా ముందుగా ఒక ఫోల్డర్‌లో ఉంచబడుతుంది, కాబట్టి ఏదీ పోదు.
safe-restore-cancel = రద్దు చేయండి
safe-restore-mail = మెయిల్
safe-restore-pim = ఖాతాలు, కాంటాక్ట్‌లు
safe-restore-blobs = అటాచ్‌మెంట్‌లు
safe-report-title = డీబగ్ రిపోర్ట్
safe-report-body = దీన్ని కాపీ చేసి మీ బగ్ రిపోర్ట్‌కు జోడించండి. ఇందులో మెయిల్, అడ్రస్‌లు లేదా పాస్‌వర్డ్‌లు ఉండవు.
safe-report-restore = పునరుద్ధరించండి…
safe-report-copied = డీబగ్ రిపోర్ట్ కాపీ అయింది
