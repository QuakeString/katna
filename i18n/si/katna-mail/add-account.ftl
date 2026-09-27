# Katna Mail, Sinhala (සිංහල).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = තැපැල් ගිණුමක් එක් කරන්න
add-account-looking = { $address } සඳහා තැපැල් සේවාදායක සොයමින්…
add-account-address-intro = ඔබේ ඊමේල් ලිපිනය ඇතුළත් කරන්න. Katna ඔබ වෙනුවෙන් සේවාදායක සොයා ගනී.
add-account-servers-title = සේවාදායක සැකසීම්
add-account-servers-intro = { $address } සඳහා Katna තැපැල් කියවන සහ යවන තැන.
add-account-password-title = ඔබේ මුරපදය ඇතුළත් කරන්න
add-account-signing-in = පුරනය වෙමින්…

## Add a mail account: fields

add-account-field-address = ඊමේල් ලිපිනය
add-account-incoming = එන තැපැල් ({ $protocol })
add-account-outgoing = යන තැපැල් ({ $protocol })
add-account-field-server = සේවාදායකය
add-account-field-port = තොට
add-account-security-none = කිසිවක් නැත
add-account-field-username = පරිශීලක නාමය
add-account-field-password = මුරපදය
add-account-show-password = මුරපදය පෙන්වන්න
add-account-app-password-hint = { $provider } සඳහා මෙහි යෙදුම් මුරපදයක් අවශ්‍යයි, ඔබ වෙබයේ භාවිත කරන මුරපදය නොවේ. ඔබේ { $provider } ගිණුමේ ආරක්ෂක සැකසීම්වල එකක් සාදන්න.
add-account-field-name = ඔබේ නම (විකල්ප)
add-account-name-hint = ඔබ ලියන අයට පෙන්වයි.
add-account-servers-pair = { $imap } සහ { $smtp }
add-account-servers-found = { $source ->
    [built-in] සේවාදායක: { $servers }, Katna හි සපයන්නන්ගේ ලැයිස්තුවෙන් හමු විය.
    [provider] සේවාදායක: { $servers }, ඔබේ සපයන්නාගේ සැකසීම්වලින් හමු විය.
    [ispdb] සේවාදායක: { $servers }, Thunderbird හි සපයන්නන්ගේ ලැයිස්තුවෙන් හමු විය.
    [dns] සේවාදායක: { $servers }, ඔබේ වසමේ DNS වාර්තාවලින් හමු විය.
   *[other] සේවාදායක: { $servers }, අනුමානයෙන්; පුරනය අසාර්ථක වුවහොත් ඒවා පරීක්ෂා කරන්න.
}
add-account-servers-entered = සේවාදායක: { $servers }, ඇතුළත් කළ පරිදි.

## Add a mail account: buttons

add-account-servers-button = සේවාදායක සැකසීම්
add-account-back = ආපසු
add-account-add = ගිණුම එක් කරන්න
add-account-next = ඊළඟ
add-account-cancel = අවලංගු කරන්න

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] එන තැපැල් සේවාදායකය ඇතුළත් කරන්න.
   *[outgoing] යන තැපැල් සේවාදායකය ඇතුළත් කරන්න.
}
add-account-server-space = { $kind ->
    [incoming] එන තැපැල් සේවාදායකයේ නමේ හිස්තැනක් ඇත.
   *[outgoing] යන තැපැල් සේවාදායකයේ නමේ හිස්තැනක් ඇත.
}
add-account-port-invalid = { $kind ->
    [incoming] එන තැපැල් තොට { $min } සිට { $max } දක්වා අංකයක් විය යුතුයි.
   *[outgoing] යන තැපැල් තොට { $min } සිට { $max } දක්වා අංකයක් විය යුතුයි.
}
add-account-address-empty = ඊමේල් ලිපිනයක් ඇතුළත් කරන්න.
add-account-address-invalid = { $example } වැනි ඊමේල් ලිපිනයක් ඇතුළත් කරන්න.
add-account-not-found = Katna ට { $address } සඳහා සේවාදායක සොයා ගත නොහැකි වූ බැවින්, සාමාන්‍ය නම් පුරවා ඇත. ඒවා ඔබේ සපයන්නා සමඟ පරීක්ෂා කරන්න.
add-account-password-empty = මුරපදය ඇතුළත් කරන්න.
add-account-name-is-password = නම මුරපදයට සමානයි. ඒ වෙනුවට, අනෙක් අයට පෙනිය යුතු ආකාරයට ඔබේ නම එහි ටයිප් කරන්න.
add-account-added = { $address } එක් කළා. ඔබේ තැපැල් ලබා ගනිමින්…
add-account-app-password-refused = { $provider } මුරපදය ප්‍රතික්ෂේප කළා. ඔබ වෙබයේ භාවිත කරන මුරපදය නොව, යෙදුම් මුරපදයක් අවශ්‍යයි.
add-account-password-refused = සේවාදායකය මුරපදය ප්‍රතික්ෂේප කළා. එය පරීක්ෂා කර නැවත උත්සාහ කරන්න.

## The account menu (from the account button on the top bar)

add-account-menu-another = තවත් ගිණුමක් එක් කරන්න
add-account-menu-manage = ගිණුම් කළමනාකරණය කරන්න
