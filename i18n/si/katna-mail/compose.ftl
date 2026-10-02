# Katna Mail, Sinhala (සිංහල).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = නව පණිවිඩය
compose-restore = ප්‍රතිසාධනය කරන්න
compose-minimize = කුඩා කරන්න
compose-exit-full-screen = පූර්ණ තිරයෙන් ඉවත් වන්න
compose-open-window = නව කවුළුවක විවෘත කරන්න
compose-save-close = සුරකා වසන්න
compose-back-to-mail = තැපැල් කවුළුවට ආපසු
compose-pop-out-reply = පිළිතුර වෙනම කවුළුවක විවෘත කරන්න
compose-edit-recipients = ලබන්නන් සංස්කරණය කරන්න
compose-summary-cc = Cc: { $names }
compose-summary-bcc = Bcc: { $names }
compose-more-recipients = තවත් { $count }
compose-show-trimmed = කපා හැරි අන්තර්ගතය පෙන්වන්න
compose-hide-trimmed = කපා හැරි අන්තර්ගතය සඟවන්න
compose-remove-trimmed = උපුටා දැක්වූ පෙළ ඉවත් කරන්න
compose-trimmed-removed = උපුටා දැක්වූ පෙළ ඉවත් කරන ලදී

## Recipients and subject

compose-to = වෙත
compose-cc = Cc
compose-bcc = Bcc
compose-from = වෙතින්
compose-from-choose = වෙනත් ගිණුමකින් යවන්න
compose-recipients = ලබන්නන්
compose-subject = විෂය

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = පළමුව විවෘත පණිවිඩය යවන්න හෝ ඉවත ලන්න.
compose-bad-address = “{ $address }” ඊමේල් ලිපිනයක් නොවේ.
compose-no-recipients = අවම වශයෙන් එක් ලබන්නෙකු එක් කරන්න.
compose-attachments-too-large = ඇමුණුම් { $size } වේ; තැපැල් සේවාදායක { $limit } දක්වා පමණක් පිළිගනී.
compose-no-account = තැපැල් යැවීමට ගිණුමක් එක් කරන්න.
compose-past-time = අනාගතයේ වේලාවක් තෝරන්න.
compose-scheduling = කාලසටහන් කරමින්…
compose-sending = යවමින්…
compose-scheduled = { $when } ට යැවීමට කාලසටහන් කළා
compose-sent-archived = යවා සංරක්ෂණය කළා
compose-sent = පණිවිඩය යැව්වා
compose-discarded = කෙටුම්පත ඉවත දැමුවා
compose-draft-saved = කෙටුම්පත සුරැකිණි
compose-draft-saving = සුරකිමින්…
compose-draft-failed = කෙටුම්පත සුරැකිය නොහැකි විය: { $error }
compose-draft-not-opened = කෙටුම්පත විවෘත කළ නොහැකි විය.

## Attachments

compose-picker-insert = ඇතුළු කරන්න
compose-picker-attach = අමුණන්න
compose-file-too-large = { $name } ඉතා විශාලයි: පණිවිඩයකට { $limit } දක්වා පමණක් රැගෙන යා හැක.
compose-forward-files-missing = ඉදිරියට යවන පණිවිඩයේ ගොනු බාගත කර නැති නිසා ඒවා අමුණා නැත.
compose-attachment-size = ({ $size })
compose-remove-attachment = ඇමුණුම ඉවත් කරන්න
compose-attachments-total = { $count ->
    [one] ගොනු { $count }, { $size }
   *[other] ගොනු { $count }, { $size }
}
compose-drive-note = { $name } { $limit } ඉක්මවන නිසා එය ඔබේ Google Drive වෙත යන අතර පණිවිඩයේ සබැඳියක් තිබේ.
compose-drive-tip = ඔබේ Google Drive හි; පණිවිඩයේ සබැඳියක් තිබේ
compose-drive-uploading = උඩුගත කරමින් { $percent }%
compose-drive-allow = Drive ඉඩ දෙන්න
compose-drive-allow-tip = විශාල ගොනු ඔබේ Drive හි තැබීමට Katna හට ඉඩ දීමට Google සමඟ නැවත පුරනය වන්න
compose-drive-retry = නැවත උත්සාහ කරන්න
compose-drive-sends-when-uploaded = { $name } උඩුගත වූ පසු යවනු ලැබේ
compose-drive-not-uploaded = { $name } තවමත් Google Drive හි නැත
compose-drive-share-failed = Google Drive හි ගොනු බෙදා ගැනීමට නොහැකි විය: { $error }
compose-drive-share-title = ගොනු සියලු දෙනා සමඟ බෙදා ගන්නද?
compose-drive-share-text = { $count ->
    [one] Google ගිණුමක් නැති { $addresses } සමඟ Google Drive හට ගොනු බෙදා ගත නොහැක. ඒ වෙනුවට, සබැඳිය ඇති ඕනෑම කෙනෙකුට ඒවා විවෘත කළ හැක.
   *[other] Google ගිණුමක් නැති { $addresses } සමඟ Google Drive හට ගොනු බෙදා ගත නොහැක. ඒ වෙනුවට, සබැඳිය ඇති ඕනෑම කෙනෙකුට ඒවා විවෘත කළ හැක.
}
compose-drive-share-link = සබැඳියෙන් බෙදා ගන්න
compose-drive-send-without = බෙදා නොගෙන යවන්න
compose-drive-share-cancel = අවලංගු කරන්න
compose-drive-card-detail = { $size } · Google Drive
compose-drive-card-name = Google Drive
compose-onedrive-note = { $name } { $limit } ඉක්මවන නිසා එය ඔබේ OneDrive වෙත යන අතර පණිවිඩයේ සබැඳියක් තිබේ.
compose-onedrive-tip = ඔබේ OneDrive හි; පණිවිඩයේ සබැඳියක් තිබේ
compose-onedrive-allow = OneDrive ඉඩ දෙන්න
compose-onedrive-allow-tip = විශාල ගොනු ඔබේ OneDrive හි තැබීමට Katna හට ඉඩ දීමට Microsoft සමඟ නැවත පුරනය වන්න
compose-onedrive-not-uploaded = { $name } තවමත් OneDrive හි නැත
compose-onedrive-share-failed = OneDrive හි ගොනු බෙදා ගැනීමට නොහැකි විය: { $error }
compose-onedrive-share-text = { $count ->
    [one] OneDrive හට { $addresses } සමඟ ගොනු බෙදා ගත නොහැක. ඒ වෙනුවට, සබැඳිය ඇති ඕනෑම කෙනෙකුට ඒවා විවෘත කළ හැක.
   *[other] OneDrive හට { $addresses } සමඟ ගොනු බෙදා ගත නොහැක. ඒ වෙනුවට, සබැඳිය ඇති ඕනෑම කෙනෙකුට ඒවා විවෘත කළ හැක.
}
compose-onedrive-card-detail = { $size } · OneDrive
compose-onedrive-card-name = OneDrive
compose-drop-files = ගොනු මෙහි දමන්න
compose-drop-here = මෙහි දමන්න

## Paste options (a small bar under what was just pasted or dropped)

compose-paste-keep-formatting = හැඩතල ගැන්වීම තබා ගන්න
compose-paste-table = වගුව
compose-paste-picture = පින්තූරය
compose-paste-plain-text = සරල පෙළ
compose-paste-inline = පෙළ තුළ
compose-paste-attachment = ඇමුණුම

## Encryption and signing (the toggles by the recipients)

compose-encrypt = සංකේතනය කරන්න
compose-encrypted = සංකේතනය කළා: ලබන්නන්ට පමණක් කියවිය හැක
compose-sign = අත්සන් කරන්න
compose-signed = අත්සන් කළා: එය ඔබෙන් බව ලබන්නන්ට පරීක්ෂා කළ හැක

## Open and click tracking and read receipts (toggles after Sign)

compose-track = විවෘත කිරීම් සහ ක්ලික් ලුහුබඳින්න
compose-tracked = ලුහුබඳිමින්: එක් එක් ලබන්නා එය විවෘත කරන විට හෝ සබැඳියක් විවෘත කරන විට ඔබට පෙනේ
compose-track-clicks = සබැඳි ක්ලික් ලුහුබඳින්න (සරල පෙළෙහි විවෘත කිරීම් පෙන්විය නොහැක)
compose-tracked-clicks = ලුහුබඳිමින්: එක් එක් ලබන්නා සබැඳියක් විවෘත කරන විට ඔබට පෙනේ
compose-track-sign-in = විවෘත කිරීම් සහ ක්ලික් ලුහුබැඳීමට Katna ගිණුමකට පුරනය වන්න
compose-receipt = කියවූ බවට රිසිට්පතක් ඉල්ලන්න
compose-receipt-on = කියවූ බවට රිසිට්පතක් ඉල්ලා ඇත: ලබන්නාගේ යෙදුම එකක් යවන ලෙස ඔවුන්ගෙන් ඉල්ලිය හැක
compose-delivery = බෙදාහැරීමේ රිසිට්පතක් ඉල්ලන්න
compose-delivery-on = බෙදාහැරීමේ රිසිට්පතක් ඉල්ලා ඇත: එක් එක් ලබන්නාගේ සේවාදායකය පණිවිඩය පිළිගත් විට ඔබේ තැපැල් සේවාදායකය ඔබට ඊමේල් කරයි
compose-delivery-unavailable = ඔබේ තැපැල් සේවාදායකය බෙදාහැරීමේ රිසිට්පත් නොයවයි

## Spelling

spell-no-dictionary = { $language } සඳහා අක්ෂර වින්‍යාස ශබ්දකෝෂයක් ස්ථාපනය කර නැත (උදාහරණයක් ලෙස hunspell-en_us).
spell-dictionary-error = අක්ෂර වින්‍යාස ශබ්දකෝෂය: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = “{ $words }”
grammar-add = “{ $words }” එක් කරන්න
grammar-remove = “{ $words }” ඉවත් කරන්න
grammar-ignore = නොසලකා හරින්න

## Send checks (asked before a message goes out)

send-check-attachment-title = ඔබට ගොනු ඇමිණීමට අවශ්‍ය වුණාද?
send-check-attachment-text = ඔබ ඇමුණුමක් ගැන ලිව්වා, නමුත් කිසිවක් අමුණා නැත.
send-check-attach = ගොනුවක් අමුණන්න
send-check-subject-title = විෂයක් නැතිව යවන්නද?
send-check-subject-text = මෙම පණිවිඩයට විෂයක් නැත.
send-check-add-subject = විෂයක් එක් කරන්න
send-check-send-anyway = කෙසේ වෙතත් යවන්න

## Recipients (To, Cc and Bcc)

recipient-not-valid = වලංගු ඊමේල් ලිපිනයක් නොවේ
recipient-show-address = ලිපිනය පෙන්වන්න
recipient-remove = ඉවත් කරන්න
recipient-bad-title = ලිපිනය පරීක්ෂා කරන්න
recipient-bad-text = “{ $address }” වලංගු ඊමේල් ලිපිනයක් නොවේ. යැවීමට පෙර එය නිවැරදි කරන්න හෝ ඉවත් කරන්න.
recipient-bad-fix = නිවැරදි කරන්න
