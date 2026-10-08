# Katna Mail, Sinhala (සිංහල).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = තැපැල් සේවාදායකය
problems-signed-out = { $provider } විසින් Katna, { $address } වෙතින් ඉවත් කළා. තැපැල් සමමුහුර්ත වීම නැවතුණා.
problems-password-refused = { $provider } විසින් { $address } සඳහා මුරපදය ප්‍රතික්ෂේප කළා. එය වෙනස් වී ඇති විය හැක.
problems-no-answer = { $provider } විසින් { $address } සඳහා ප්‍රතිචාර නොදක්වයි. Katna දිගටම උත්සාහ කරයි.
problems-offline = ඔබ නොබැඳියි. ඔබේ තැපැල් තවමත් මෙහි ඇත, ඔබ යවන තැපැල් ඔබ නැවත සබැඳි වන තුරු රැඳේ.
problems-accounts-need-you = { $count ->
    [one] ගිණුම් { $count }කට ඔබේ ක්‍රියාව අවශ්‍යයි
   *[other] ගිණුම් { $count }කට ඔබේ ක්‍රියාව අවශ්‍යයි
}
problems-show = පෙන්වන්න
problems-later = පසුව
problems-new-password = නව මුරපදය
problems-try-again = නැවත උත්සාහ කරන්න

## The New password card

problems-password-title = නව මුරපදය
problems-password-detail = { $provider } විසින් { $address } සඳහා සුරැකි මුරපදය ප්‍රතික්ෂේප කළා. නව මුරපදය ටයිප් කරන්න; Katna එය තබා ගැනීමට පෙර පරීක්ෂා කරයි.
problems-password-placeholder = මුරපදය
problems-password-show = මුරපදය පෙන්වන්න
problems-password-hide = මුරපදය සඟවන්න
problems-password-cancel = අවලංගු කරන්න
problems-password-save = සුරකින්න
problems-password-checking = පරීක්ෂා කරමින්…
problems-password-refused-again = { $provider } මෙම මුරපදයද ප්‍රතික්ෂේප කළා. එය පරීක්ෂා කර නැවත උත්සාහ කරන්න.
problems-password-saved = { $address } සඳහා මුරපදය සුරැකුණා. ඔබේ තැපැල් ලබා ගනිමින්…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = { $address } හි තැපැල් සේවාදායකය { $count ->
    [one] පණිවිඩයක් ගෙන යාම පිළිගත්තේ නැත, එබැවින් එය පෙර තිබූ තැනටම ආපසු ගියා.
   *[other] පණිවිඩ { $count } ක් ගෙන යාම පිළිගත්තේ නැත, එබැවින් ඒවා පෙර තිබූ තැනටම ආපසු ගියා.
}
problems-refused-flags = { $address } හි තැපැල් සේවාදායකය { $count ->
    [one] පණිවිඩයක් සලකුණු කිරීම (කියවූ, තරු යෙදූ…) පිළිගත්තේ නැත, එබැවින් එය පෙර තිබූ ලෙසම ඇත.
   *[other] පණිවිඩ { $count } ක් සලකුණු කිරීම (කියවූ, තරු යෙදූ…) පිළිගත්තේ නැත, එබැවින් ඒවා පෙර තිබූ ලෙසම ඇත.
}
problems-refused-label = { $address } හි තැපැල් සේවාදායකය { $count ->
    [one] පණිවිඩයක ලේබල වෙනස් කිරීම පිළිගත්තේ නැත, එබැවින් එය පෙර තිබූ ලෙසම ඇත.
   *[other] පණිවිඩ { $count } ක ලේබල වෙනස් කිරීම පිළිගත්තේ නැත, එබැවින් ඒවා පෙර තිබූ ලෙසම ඇත.
}
problems-refused-delete = { $address } හි තැපැල් සේවාදායකය { $count ->
    [one] පණිවිඩයක් මැකීම පිළිගත්තේ නැත, එබැවින් එය ආපසු ආවා.
   *[other] පණිවිඩ { $count } ක් මැකීම පිළිගත්තේ නැත, එබැවින් ඒවා ආපසු ආවා.
}
problems-refused-other = { $address } හි තැපැල් සේවාදායකය { $count ->
    [one] වෙනස්කමක් පිළිගත්තේ නැත, එබැවින් Katna එය පෙර තිබූ ලෙසම යළි සැකසුවා.
   *[other] වෙනස්කම් { $count } ක් පිළිගත්තේ නැත, එබැවින් Katna ඒවා පෙර තිබූ ලෙසම යළි සැකසුවා.
}
problems-details = විස්තර

## Katna's background service (katna-daemon) isn't running

service-starting = Katna හි පසුබිම් සේවාව ආරම්භ කරමින්…
service-failed = Katna හි පසුබිම් සේවාව ආරම්භ නොවන නිසා, තැපැල් සමමුහුර්ත නොවේ.
service-start-again = නැවත ආරම්භ කරන්න
service-started-again = Katna හි පසුබිම් සේවාව නැවතී, නැවත ආරම්භ කළා.
service-details-title = සේවාව ආරම්භ නොවන්නේ ඇයි
service-details-body = මෙය පිටපත් කර ඔබේ වාර්තාව සමඟ යවන්න. එහි තැපැල් හෝ මුරපද නැත.
service-details-copy = පිටපත් කරන්න
service-details-close = වසන්න
service-not-running = Katna පසුබිම් සේවාව ක්‍රියාත්මක නොවේ.
service-no-answer = Katna පසුබිම් සේවාව පිළිතුරු දුන්නේ නැත: { $error }
service-no-session = D-Bus සැසියක් නැත: { $error }
