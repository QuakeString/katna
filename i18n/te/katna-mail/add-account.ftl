# Katna Mail, Telugu (తెలుగు).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = మెయిల్ ఖాతాను జోడించండి
add-account-providers-intro = మీ మెయిల్ ప్రొవైడర్‌ను ఎంచుకోండి. మిగతాది Katna చూసుకుంటుంది.
add-account-provider-other = ఇతర మెయిల్
add-account-provider-other-detail = ఏదైనా IMAP లేదా POP3 ఖాతా
add-account-provider-google-detail = Gmail, Google Workspace
add-account-provider-microsoft-detail = Outlook, Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = { $provider }కు సైన్ ఇన్ చేయండి
add-account-form-title-other = మీ మెయిల్ ఖాతా
add-account-form-intro = Katna మీ పాస్‌వర్డ్‌ను మీ సిస్టమ్ కీరింగ్‌లో భద్రపరుస్తుంది.
add-account-looking = { $address } కోసం మెయిల్ సర్వర్‌లను వెతుకుతోంది…
add-account-address-intro = మీ ఈమెయిల్ అడ్రస్‌ను ఎంటర్ చేయండి. Katna మీ కోసం సర్వర్‌లను కనుగొంటుంది.
add-account-servers-title = సర్వర్ సెట్టింగ్‌లు
add-account-servers-intro = { $address } కోసం Katna మెయిల్‌ను ఎక్కడ చదువుతుంది, ఎక్కడి నుండి పంపుతుంది.
add-account-signing-in = సైన్ ఇన్ అవుతోంది…
add-account-browser-title = మీ బ్రౌజర్‌లో కొనసాగించండి
add-account-browser-intro = Katna మీ బ్రౌజర్‌లో { $provider } సైన్ ఇన్ పేజీని తెరిచింది. అక్కడ సైన్ ఇన్ చేసి, మీ మెయిల్‌ను చదవడానికి, పంపడానికి Katnaను అనుమతించండి, ఆ తర్వాత ఇక్కడికి తిరిగి రండి.
add-account-browser-hint = పేజీ ఏదీ తెరుచుకోలేదా? మీ బ్రౌజర్ విండోలను చూడండి, లేదా వెనుకకు వెళ్లి మళ్లీ ప్రయత్నించండి.
add-account-stage-browser = మీరు బ్రౌజర్‌లో సైన్ ఇన్ చేసే వరకు వేచి ఉంది…
add-account-stage-signing-in-at = { $server }లో సైన్ ఇన్ చేస్తోంది…
add-account-help-app-password-link = యాప్ పాస్‌వర్డ్‌ను ఎలా తయారు చేయాలి
add-account-help-turn-on-imap = { $provider } వెబ్ మెయిల్ సెట్టింగ్‌లలో IMAP, POP3 యాక్సెస్ ఆన్ చేసిన తర్వాతే మెయిల్ యాప్‌లను అనుమతిస్తుంది.
add-account-help-turn-on-imap-link = దీన్ని ఎలా ఆన్ చేయాలి

## Add a mail account: fields

add-account-field-address = ఈమెయిల్ అడ్రస్
add-account-receive-with = మెయిల్‌ను దీనితో స్వీకరించండి
add-account-imap-about = IMAP మీ మెయిల్‌ను, ఫోల్డర్‌లను సర్వర్‌లో ఉంచుతుంది, ప్రతి డివైజ్‌లోనూ ఒకేలా ఉంటాయి. వీలైతే దీన్నే ఎంచుకోండి.
add-account-pop3-about = POP3 మీ మెయిల్‌ను ఈ కంప్యూటర్‌కు డౌన్‌లోడ్ చేస్తుంది. ఇక్కడ మీరు చదివిన లేదా తరలించిన మెయిల్ సర్వర్‌లో, మీ ఇతర డివైజ్‌లలో అలాగే ఉంటుంది.
add-account-incoming = ఇన్‌కమింగ్ మెయిల్ ({ $protocol })
add-account-outgoing = అవుట్‌గోయింగ్ మెయిల్ ({ $protocol })
add-account-field-server = సర్వర్
add-account-field-port = పోర్ట్
add-account-security-none = ఏదీ లేదు
add-account-security-none-warning = ఎన్‌క్రిప్ట్ చేయబడలేదు: మీ పాస్‌వర్డ్, మెయిల్ మార్గమధ్యంలో చదవబడవచ్చు.
add-account-field-username = యూజర్‌నేమ్
add-account-field-password = పాస్‌వర్డ్
add-account-show-password = పాస్‌వర్డ్‌ను చూపండి
add-account-app-password-hint = ఇక్కడ { $provider }కు యాప్ పాస్‌వర్డ్ అవసరం, వెబ్‌లో మీరు ఉపయోగించేది కాదు. మీ { $provider } ఖాతా సెక్యూరిటీ సెట్టింగ్‌లలో ఒకటి తయారు చేయండి.
add-account-field-name = మీ పేరు (ఐచ్ఛికం)
add-account-name-hint = మీరు రాసే వ్యక్తులకు చూపబడుతుంది.
add-account-servers-pair = { $imap }, { $smtp }
add-account-servers-found = { $source ->
    [built-in] సర్వర్‌లు: { $servers }, Katna ప్రొవైడర్ల జాబితాలో కనుగొనబడ్డాయి.
    [provider] సర్వర్‌లు: { $servers }, మీ ప్రొవైడర్ సెట్టింగ్‌లలో కనుగొనబడ్డాయి.
    [ispdb] సర్వర్‌లు: { $servers }, Thunderbird ప్రొవైడర్ల జాబితాలో కనుగొనబడ్డాయి.
    [dns] సర్వర్‌లు: { $servers }, మీ డొమైన్ DNS రికార్డులలో కనుగొనబడ్డాయి.
   *[other] సర్వర్‌లు: { $servers }, ఊహించినవి; సైన్ ఇన్ విఫలమైతే వాటిని తనిఖీ చేయండి.
}
add-account-servers-entered = సర్వర్‌లు: { $servers }, ఎంటర్ చేసినట్లే.

## Add a mail account: buttons

add-account-sign-in-with = { $provider }తో సైన్ ఇన్ చేయండి
add-account-sign-in-instead = బదులుగా { $provider }తో సైన్ ఇన్ చేయండి
add-account-servers-button = సర్వర్ సెట్టింగ్‌లు
add-account-back = వెనుకకు
add-account-add = ఖాతాను జోడించండి
add-account-done = పూర్తయింది
add-account-another = మరొక ఖాతాను జోడించండి
add-account-cancel = రద్దు చేయండి

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] ఇన్‌కమింగ్ సర్వర్‌ను ఎంటర్ చేయండి.
   *[outgoing] అవుట్‌గోయింగ్ సర్వర్‌ను ఎంటర్ చేయండి.
}
add-account-server-space = { $kind ->
    [incoming] ఇన్‌కమింగ్ సర్వర్ పేరులో ఖాళీ ఉంది.
   *[outgoing] అవుట్‌గోయింగ్ సర్వర్ పేరులో ఖాళీ ఉంది.
}
add-account-port-invalid = { $kind ->
    [incoming] ఇన్‌కమింగ్ పోర్ట్ { $min } నుండి { $max } వరకు ఉన్న సంఖ్య అయి ఉండాలి.
   *[outgoing] అవుట్‌గోయింగ్ పోర్ట్ { $min } నుండి { $max } వరకు ఉన్న సంఖ్య అయి ఉండాలి.
}
add-account-address-empty = ఈమెయిల్ అడ్రస్‌ను ఎంటర్ చేయండి.
add-account-address-invalid = { $example } వంటి ఈమెయిల్ అడ్రస్‌ను ఎంటర్ చేయండి.
add-account-not-found = Katna { $address } కోసం సర్వర్‌లను కనుగొనలేకపోయింది, కాబట్టి సాధారణ పేర్లను నింపింది. వాటిని మీ ప్రొవైడర్‌తో తనిఖీ చేయండి.
add-account-password-empty = పాస్‌వర్డ్‌ను ఎంటర్ చేయండి.
add-account-name-is-password = పేరు పాస్‌వర్డ్‌లాగే ఉంది. బదులుగా అక్కడ మీ పేరును, ఇతరులు చూడాల్సిన విధంగా టైప్ చేయండి.
add-account-app-password-refused = { $provider } పాస్‌వర్డ్‌ను తిరస్కరించింది. దానికి యాప్ పాస్‌వర్డ్ అవసరం, వెబ్‌లో మీరు ఉపయోగించేది కాదు.
add-account-password-refused = సర్వర్ పాస్‌వర్డ్‌ను తిరస్కరించింది. దాన్ని తనిఖీ చేసి మళ్లీ ప్రయత్నించండి.
add-account-sign-in-refused = { $provider } Katnaను లోపలికి అనుమతించలేదు. మళ్లీ ప్రయత్నించి, మీ మెయిల్‌కు యాక్సెస్ అనుమతించండి.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Katna యొక్క ఈ కాపీ ఇంకా Microsoft ఖాతాలకు సైన్ ఇన్ చేయలేదు.
    [Google] Katna యొక్క ఈ కాపీ ఇంకా Google ఖాతాలకు సైన్ ఇన్ చేయలేదు.
   *[other] ఈ ప్రొవైడర్ తన సొంత పేజీలో మాత్రమే సైన్ ఇన్ చేయడానికి అనుమతిస్తుంది, దాని కోసం Katna ఇంకా అలా చేయలేదు.
}
add-account-smtp-not-found = మీ మెయిల్‌ను ఎక్కడ చదవాలో Katna కనుగొంది, కానీ ఎక్కడ నుండి పంపాలో కనుగొనలేదు. అవుట్‌గోయింగ్ సర్వర్‌ను ఎంటర్ చేయండి.

## Add a mail account: the last step

add-account-done-title = మీ ఖాతా సిద్ధంగా ఉంది
add-account-done-intro = Katna ఇప్పుడు మీ మెయిల్‌ను తీసుకువస్తోంది. కొత్త మెయిల్ వచ్చిన వెంటనే కనిపిస్తుంది.
add-account-done-sign-in = సైన్ ఇన్
add-account-done-signed-in-with = { $provider }తో, మీ బ్రౌజర్‌లో
add-account-done-receiving = మెయిల్ స్వీకరించడం
add-account-done-sending = మెయిల్ పంపడం
add-account-done-on-server = సర్వర్‌లో మెయిల్
add-account-done-kept = మీరు Katnaలో తొలగించే వరకు ఉంచబడుతుంది
add-account-done-pop3-hint = సర్వర్‌లోని మెయిల్‌కు ఏమి జరగాలో సెట్టింగ్‌లు > ఖాతాలులో మార్చండి.
add-account-done-zoho-title = టాస్క్‌లు, క్యాలెండర్‌లు
add-account-done-zoho-about = Zoho వీటిని మెయిల్ నుండి వేరుగా ఉంచుతుంది. వీటిని Katnaలోకి తీసుకురావడానికి Zohoతో ఒకసారి సైన్ ఇన్ చేయండి.
add-account-done-linked = టాస్క్‌లు, క్యాలెండర్‌లు కనెక్ట్ అయ్యాయి

## The account menu (from the account button on the top bar)

add-account-menu-another = మరో ఖాతాను జోడించండి
app-menu = ప్రధాన మెనూ
app-menu-back = వెనుకకు
