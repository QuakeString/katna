# Katna Mail, Sinhala (සිංහල).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = වසන්න
reader-back = ආපසු
reader-mark-unread = නොකියවූ ලෙස සලකුණු කරන්න
reader-move-to = වෙත ගෙන යන්න
reader-more = තවත්
reader-original-colors = මුල් වර්ණ පෙන්වන්න
reader-dark-colors = අඳුරු වර්ණවලින් පෙන්වන්න
reader-print-all = සියල්ල මුද්‍රණය කරන්න
reader-new-window = නව කවුළුවක
reader-position = { $total } න් { $position }
reader-newer = අලුත්
reader-older = පැරණි

## Reading pane: the conversation

reader-removed = මෙම සංවාදය ඉවත් කරන ලදී.
reader-no-subject = (විෂයක් නැත)
reader-collapse-all = සියල්ල හකුළන්න
reader-expand-all = සියල්ල දිග හරින්න
reader-unknown-sender = (නොදන්නා යවන්නා)
reader-date-ago = { $date } ({ $ago })
reader-me = මා
reader-to = { $names } වෙත
reader-to-label = ලබන්නන්:
reader-tick-delivered = බෙදාහැරුණා: { $when }
reader-tick-no-bounce = යැව්වා: { $when }; බවුන්ස් එකක් ආපසු ආවේ නැති නිසා එය බොහෝ විට ලැබෙන්නට ඇත
reader-tick-bounced = බෙදාහැරුණේ නැත: බවුන්ස් විය, { $when }
reader-tick-read = කියෙව්වා: { $when } (කියවූ බවට රිසිට්පත)
reader-tick-opened = විවෘත කළා, අවසන් වරට { $when } (විවෘත කිරීම් ලුහුබැඳීම)
reader-starred = තරු යෙදූ
reader-not-starred = තරු නොයෙදූ
reader-too-long = පණිවිඩය සම්පූර්ණයෙන් පෙන්වීමට තරම් දිග වැඩිය.
reader-encrypted-images = සංකේතනය කළ තැපැල් වල වෙබයේ රූප කිසි විටෙකත් පූරණය නොකෙරේ.
reader-window-failed = නව කවුළුවක් විවෘත කළ නොහැකි විය.

## Reading pane: message details (opened from "to me")

reader-details-from = යවන්නා:
reader-details-to = ලබන්නා:
reader-details-cc = cc:
reader-details-date = දිනය:
reader-details-subject = විෂය:

## Reading pane: downloading a message

reader-downloading = මෙම පණිවිඩය සේවාදායකයෙන් බාගනිමින්…
reader-download-failed = මෙම පණිවිඩය බාගත කළ නොහැකි විය.
reader-try-again = නැවත උත්සාහ කරන්න

## Reply row

reply-reply = පිළිතුරු දෙන්න
reply-reply-all = සියල්ලන්ට පිළිතුරු දෙන්න
reply-forward = ඉදිරියට යවන්න

## Encrypted and signed mail

security-decrypting = විකේතනය කරමින්…
security-checking = අත්සන පරීක්ෂා කරමින්…
security-partly-encrypted = මෙම පණිවිඩයේ කොටසක් පමණක් සංකේතනය කර ඇත. ඉතිරි කොටස ආරක්ෂාවෙන් පිටත එක් කළ එකක් වන අතර ඕනෑම අයෙකුගෙන් පැමිණි එකක් විය හැක.
security-partly-signed = මෙම පණිවිඩයේ කොටසක් පමණක් අත්සන් කර ඇත. ඉතිරි කොටස ආරක්ෂාවෙන් පිටත එක් කළ එකක් වන අතර ඕනෑම අයෙකුගෙන් පැමිණි එකක් විය හැක.
security-encrypted = සංකේතනය කළ පණිවිඩය
security-encrypted-smime = සංකේතනය කළ පණිවිඩය (S/MIME)
security-no-key = මෙම පණිවිඩය විකේතනය කළ නොහැක: එය ඔබ සතු නැති යතුරක් සඳහා සංකේතනය කර ඇත.
security-cancelled = විකේතනය අවලංගු කරන ලදී.
security-damaged = මෙම පණිවිඩය විකේතනය කළ නොහැක: සංකේතනය කළ දත්ත හානි වී ඇත හෝ වෙනස් කර ඇත.
security-decrypt-unavailable = මෙම පණිවිඩය විකේතනය කළ නොහැක: සංකේතනය කළ තැපැල් කියවීමට { $tool } ස්ථාපනය කරන්න.
security-decrypt-failed = මෙම පණිවිඩය විකේතනය කළ නොහැක: { $reason }
security-unknown-signer = නොදන්නා අත්සන්කරුවෙකු
security-signed-verified = { $signer } විසින් අත්සන් කළ · තහවුරු කළ
security-signed-not-sender = යවන්නා නොවන { $signer } විසින් අත්සන් කළ
security-signed-untrusted = ඔබ විශ්වාස නොකරන ලෙස සලකුණු කළ යතුරකින් { $signer } විසින් අත්සන් කළ
security-signed-unverified = { $signer } විසින් අත්සන් කළ · යතුර තහවුරු කර නැත
security-bad-signature = නරක අත්සනක්: මෙම පණිවිඩය අත්සන් කිරීමෙන් පසු වෙනස් කර ඇත, නැතහොත් අත්සන ව්‍යාජය.
security-signature-expired = { $signer } විසින් අත්සන් කළ · අත්සන කල් ඉකුත් වී ඇත
security-key-expired = { $signer } විසින් අත්සන් කළ · එතැන් සිට යතුර කල් ඉකුත් වී ඇත
security-key-revoked = අවලංගු කළ යතුරකින් { $signer } විසින් අත්සන් කළ
security-missing-key = ඔබ සතු නැති යතුරකින් අත්සන් කර ඇති නිසා පරීක්ෂා කළ නොහැක
security-missing-key-id = ඔබ සතු නැති යතුරකින් ({ $key }) අත්සන් කර ඇති නිසා පරීක්ෂා කළ නොහැක
security-signature-unavailable = අත්සන් කර ඇත; අත්සන පරීක්ෂා කිරීමට { $tool } ස්ථාපනය කරන්න
security-signature-error = අත්සන පරීක්ෂා කළ නොහැකි විය.
tracking-opened = { $who } එය { $count ->
    [one] එක් වරක්
   *[other] වාර { $count }ක්
} විවෘත කළා, අවසන් වරට { $when }
tracking-opens-clicks = { $who } එය { $opens ->
    [one] එක් වරක්
   *[other] වාර { $opens }ක්
} විවෘත කර සබැඳියක් { $clicks ->
    [one] එක් වරක්
   *[other] වාර { $clicks }ක්
} විවෘත කළා, අවසන් වරට { $when }
tracking-clicked = { $who } සබැඳියක් { $clicks ->
    [one] එක් වරක්
   *[other] වාර { $clicks }ක්
} විවෘත කළා, අවසන් වරට { $when }
tracking-maybe-opened = { $who } එය විවෘත කළා විය හැක (Apple Mail පෞද්ගලිකත්වය සඳහා පින්තූර පූරණය කරයි)
tracking-receipt = { $who } කියවූ බවට රිසිට්පතක් එව්වා
tracking-receipt-displayed = කියවූ බවට රිසිට්පත: { $who } ඔබේ පණිවිඩය විවෘත කළා
tracking-receipt-other = කියවූ බවට රිසිට්පත: { $who } ඔබේ පණිවිඩය විවෘත නොකර මැකුවා හෝ හැසිරෙව්වා

## Remote images and pictures

remote-hidden = මෙම පණිවිඩයේ රූප සඟවා ඇත.
remote-show = රූප පෙන්වන්න
remote-always-show = මෙම යවන්නාගෙන් සැමවිටම පෙන්වන්න
remote-picture-use = භාවිත කරන්න
remote-picture-too-big = 8 MB හෝ ඊට අඩු පින්තූරයක් තෝරන්න.
remote-picture-type = PNG, JPEG, GIF, WebP හෝ SVG පින්තූරයක් තෝරන්න.
remote-picture-read-failed = පින්තූරය කියවිය නොහැක: { $error }
remote-picture-keep-failed = පින්තූරය තබා ගත නොහැක: { $error }
remote-picture-remove-failed = පින්තූරය ඉවත් කළ නොහැක: { $error }

## Attachments

attachment-count = { $count ->
    [one] ඇමුණුමක්
   *[other] ඇමුණුම් { $count }
}
attachment-save = සුරකින්න
attachment-save-all = සියල්ල සුරකින්න
attachment-save-all-tooltip = සියලු ඇමුණුම් ෆෝල්ඩරයකට සුරකින්න
attachment-save-here = මෙහි සුරකින්න
attachment-not-downloaded = මෙම පණිවිඩය බාගත කර නැත.
attachment-not-found = මෙම ඇමුණුම පණිවිඩයේ සොයාගත නොහැකි විය.
attachment-read-failed = { $name } කියවිය නොහැකි විය
attachment-numbered = ඇමුණුම { $number }
attachment-saved-all = { $count ->
    [one] ගොනු { $count } ක් { $place } වෙත සුරකින ලදී
   *[other] ගොනු { $count } ක් { $place } වෙත සුරකින ලදී
}
attachment-saved-some = { $total ->
    [one] ගොනු { $total } න් { $saved } ක් { $place } වෙත සුරකින ලදී. { $failed } සුරැකිය නොහැකි විය
   *[other] ගොනු { $total } න් { $saved } ක් { $place } වෙත සුරකින ලදී. { $failed } සුරැකිය නොහැකි විය
}
attachment-saved-to = { $path } වෙත සුරකින ලදී
attachment-save-failed = { $name } සුරැකිය නොහැකි විය: { $error }
attachment-open-failed = { $name } විවෘත කළ නොහැකි විය: { $error }
attachment-risky = මෙම ගොනුවට වැඩසටහනක් ධාවනය කළ හැකි නිසා Katna එය විවෘත නොකරයි. ඒ වෙනුවට එය සුරකින්න.
attachment-encrypted-open = මෙම ගොනුව සංකේතනය කර ලැබුණකි. වෙනත් තැනක විවෘත කිරීමට එය සුරකින්න.

## Printing

print-failed = මුද්‍රණය කළ නොහැකි විය: { $error }
print-no-font = අකුරු මුහුණතක් හමු නොවීය
print-opened-as-pdf = එතැනින් මුද්‍රණය කිරීමට PDF ලෙස විවෘත කරන ලදී.
print-preview-title = මුද්‍රණ පෙරදසුන
print-preview-laying-out = පිටු සකසමින්…
print-preview-pages = { $count ->
    [one] පිටු { $count }
   *[other] පිටු { $count }
}
print-preview-more = { $count ->
    [one] සහ තවත් පිටු { $count }
   *[other] සහ තවත් පිටු { $count }
}
print-preview-failed = පිටු පෙන්විය නොහැකි විය
print-preview-paper = කඩදාසි
print-preview-a4 = A4
print-preview-letter = ලෙටර්
print-preview-layout = සැකැස්ම
print-preview-as-shown = පෙන්වන ආකාරයට
print-preview-simple = පෙළ පමණි
print-preview-backgrounds = පසුබිම්
print-preview-cancel = අවලංගු කරන්න
print-preview-print = මුද්‍රණය කරන්න
print-not-downloaded = (තවම බාගත කර නැත.)
print-encrypted = (සංකේතනය කර ඇත. එහි පෙළ මුද්‍රණය කිරීමට එය Katna Mail හි විවෘත කරන්න.)
print-to = ලබන්නා: { $addresses }
print-cc = Cc: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = මෙම පණිවිඩයේ ඇමුණුම් කියවීමට එය විවෘත කරන්න.
text-copy = පිටපත් කරන්න
text-select-all = සියල්ල තෝරන්න
