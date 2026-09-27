# Katna Mail, Telugu (తెలుగు).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = మెయిల్ ఖాతాను జోడించండి
add-account-looking = { $address } కోసం మెయిల్ సర్వర్‌లను వెతుకుతోంది…
add-account-address-intro = మీ ఈమెయిల్ అడ్రస్‌ను ఎంటర్ చేయండి. Katna మీ కోసం సర్వర్‌లను కనుగొంటుంది.
add-account-servers-title = సర్వర్ సెట్టింగ్‌లు
add-account-servers-intro = { $address } కోసం Katna మెయిల్‌ను ఎక్కడ చదువుతుంది, ఎక్కడి నుండి పంపుతుంది.
add-account-password-title = మీ పాస్‌వర్డ్‌ను ఎంటర్ చేయండి
add-account-signing-in = సైన్ ఇన్ అవుతోంది…

## Add a mail account: fields

add-account-field-address = ఈమెయిల్ అడ్రస్
add-account-incoming = ఇన్‌కమింగ్ మెయిల్ ({ $protocol })
add-account-outgoing = అవుట్‌గోయింగ్ మెయిల్ ({ $protocol })
add-account-field-server = సర్వర్
add-account-field-port = పోర్ట్
add-account-security-none = ఏదీ లేదు
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

add-account-servers-button = సర్వర్ సెట్టింగ్‌లు
add-account-back = వెనుకకు
add-account-add = ఖాతాను జోడించండి
add-account-next = తర్వాత
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
add-account-added = { $address } జోడించబడింది. మీ మెయిల్‌ను తెస్తోంది…
add-account-app-password-refused = { $provider } పాస్‌వర్డ్‌ను తిరస్కరించింది. దానికి యాప్ పాస్‌వర్డ్ అవసరం, వెబ్‌లో మీరు ఉపయోగించేది కాదు.
add-account-password-refused = సర్వర్ పాస్‌వర్డ్‌ను తిరస్కరించింది. దాన్ని తనిఖీ చేసి మళ్లీ ప్రయత్నించండి.

## The account menu (from the account button on the top bar)

add-account-menu-another = మరో ఖాతాను జోడించండి
add-account-menu-manage = ఖాతాలను నిర్వహించండి
