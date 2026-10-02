# Katna Mail, Sinhala (සිංහල).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = ගොනු සොයන්න

## Left side (and chips on a phone)

files-all = සියලු ගොනු
files-pictures = පින්තූර
files-pdfs = PDF
files-documents = ලේඛන
files-sheets = පැතුරුම්පත්
files-slides = ස්ලයිඩ
files-other = වෙනත්
files-accounts = ගිණුම්
files-drives = ධාවක
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = මා සමඟ බෙදා ගත්
files-shown = පෙන්වන්නේ
files-received = ලැබුණු
files-sent = මා යැවූ

## Over the files

files-count = { $count ->
    [one] ගොනු { $count } · { $size }
   *[other] ගොනු { $count } · { $size }
}
files-anyone = ඕනෑම අයෙක්
files-from-person = { $name } ගෙන්
files-time-any = ඕනෑම වේලාවක
files-time-today = අද
files-time-yesterday = ඊයේ
files-time-this-week = මෙම සතිය
files-time-last-week = පසුගිය සතිය
files-time-this-month = මෙම මාසය
files-time-last-month = පසුගිය මාසය
files-time-between = { $first } – { $last }
files-time-hint = දිනයක් ක්ලික් කරන්න, නැතහොත් දින හරහා අදින්න
files-time-summary = { $count ->
    [one] { $days } · ගොනු { $count }
   *[other] { $days } · ගොනු { $count }
}
files-time-clear = හිස් කරන්න
files-time-month-back = පෙර මාසය
files-time-month-on = ඊළඟ මාසය
files-time-wheel = දිග එලෙසම තබා මෙම දින ගෙන යාමට ස්ක්‍රෝල් කරන්න
files-sort-newest = නවතම මුලින්
files-sort-oldest = පැරණිතම මුලින්
files-sort-largest = විශාලතම මුලින්
files-sort-name = නම අනුව
files-grid = කාඩ්පත්
files-list = ලැයිස්තුව
files-this-week = මෙම සතිය
files-undated = දිනයක් නැත
files-me = මම
files-no-subject = (විෂයක් නැත)
files-loading = ඔබේ තැපැල්වලින් ගොනු එක්රැස් කරමින්…
files-empty = ඔබේ තැපැල්වල ගොනු මෙහි පෙන්වයි.
files-none-match = ගැළපෙන ගොනු නැත.
files-load-failed = ගොනු කියවීම අසාර්ථක විය: { $error }

## A file's menu and buttons

files-open = විවෘත කරන්න
files-open-with = මෙයින් විවෘත කරන්න…
files-save = සුරකින්න…
files-show-mail = තැපැල පෙන්වන්න
files-mail-window = තැපැල නව කවුළුවක විවෘත කරන්න
files-forward = ගොනුව ඉදිරියට යවන්න
files-from-them = { $name } ගෙන් ගොනු
files-copy-name = ගොනුවේ නම පිටපත් කරන්න
files-name-copied = ගොනුවේ නම පිටපත් කළා
files-downloading = තැපැල බාගත කරමින්…
files-download-failed = මෙම තැපැල බාගත කළ නොහැකි විය.

## A cloud drive in place of the mail files

files-drive-mine = මගේ ධාවකය
files-drive-mine-onedrive = මගේ ගොනු
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] ගොනු { $files }
       *[other] ගොනු { $files }
    }
    [one] ෆෝල්ඩර { $folders } · { $files ->
        [one] ගොනු { $files }
       *[other] ගොනු { $files }
    }
   *[other] ෆෝල්ඩර { $folders } · { $files ->
        [one] ගොනු { $files }
       *[other] ගොනු { $files }
    }
}
files-drive-folders = ෆෝල්ඩර
files-drive-files = ගොනු
files-drive-folder = ෆෝල්ඩරය
files-drive-meta = { $what } · සංස්කරණය කළේ { $date }
files-drive-as-link = { $what } · සබැඳියක් ලෙස
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = ලබා ගනිමින්…
files-drive-loading = ධාවකය විවෘත කරමින්…
files-drive-empty = මෙම ෆෝල්ඩරය හිස්ය.
files-drive-unreachable = { $drive } වෙත ළඟා විය නොහැක.
files-drive-try-again = නැවත උත්සාහ කරන්න
files-drive-needs-permission = මෙම ධාවකය පෙන්වීමට Katna හට එක් වරක් ඔබේ අවසරය අවශ්‍යයි. නැවත පුරනය වී ඔබේ ගොනු බැලීමට Katna හට ඉඩ දෙන්න.
files-drive-allow = ඉඩ දෙන්න
files-drive-allow-failed = පුරනය අවසන් නොවූ නිසා ධාවකය වසා ඇත.
files-drive-attach = අමුණන්න
files-drive-more = තවත්
files-drive-download = බාගන්න…
files-drive-open-web = { $drive } හි විවෘත කරන්න
files-drive-copy-link = සබැඳිය පිටපත් කරන්න
files-drive-link-copied = සබැඳිය පිටපත් කළා
files-drive-share = බෙදා ගන්න…
files-drive-rename = නැවත නම් කරන්න
files-drive-trash = කුණු කූඩයට ගෙන යන්න
files-drive-trashed = “{ $name }” { $drive } කුණු කූඩයේ ඇත
files-drive-renamed = “{ $name }” ලෙස නැවත නම් කළා
files-drive-getting = { $drive } වෙතින් { $name } ලබා ගනිමින්…
files-drive-get-failed = { $name } ලබා ගත නොහැකි විය: { $error }
files-drive-upload = උඩුගත කරන්න
files-drive-upload-files = ගොනු උඩුගත කරන්න
files-drive-upload-folder = ෆෝල්ඩරය උඩුගත කරන්න
files-drive-upload-failed = { $name } උඩුගත කළ නොහැකි විය: { $error }
files-drive-upload-needs = උඩුගත කිරීමට Katna හට එක් වරක් ඔබේ අවසරය අවශ්‍යයි: සැකසීම් › පෙරනිමි යෙදුම් › ගොනු පිටුව තුළ ඉඩ දෙන්න ඔබන්න.

## The Share dialog of a drive file or folder

files-share-title = “{ $name }” බෙදා ගන්න
files-share-add = නම හෝ ලිපිනය අනුව පුද්ගලයන් එක් කරන්න
files-share-not-address = “{ $text }” ඊමේල් ලිපිනයක් නොවේ
files-share-notify = { $drive } ඔවුන්ට ඊමේල් කරන්නත් ඉඩ දෙන්න
files-share-people = ප්‍රවේශය ඇති පුද්ගලයන්
files-share-general = සාමාන්‍ය ප්‍රවේශය
files-share-loading = ප්‍රවේශය ඇත්තේ කාටදැයි කියවමින්…
files-share-restricted = සීමිත
files-share-restricted-about = ප්‍රවේශය ඇති අයට පමණක් සබැඳියෙන් එය විවෘත කළ හැක
files-share-anyone = සබැඳිය ඇති ඕනෑම අයෙක්
files-share-anyone-can = { $role ->
    [editor] සබැඳිය ඇති ඕනෑම අයෙකුට සංස්කරණය කළ හැක
    [commenter] සබැඳිය ඇති ඕනෑම අයෙකුට අදහස් දැක්විය හැක
   *[viewer] සබැඳිය ඇති ඕනෑම අයෙකුට බැලිය හැක
}
files-share-anyone-about = { $role ->
    [editor] සබැඳිය ඇති අන්තර්ජාලයේ ඕනෑම අයෙකුට සංස්කරණය කළ හැක
    [commenter] සබැඳිය ඇති අන්තර්ජාලයේ ඕනෑම අයෙකුට අදහස් දැක්විය හැක
   *[viewer] සබැඳිය ඇති අන්තර්ජාලයේ ඕනෑම අයෙකුට බැලිය හැක
}
files-share-role-owner = හිමිකරු
files-share-role-editor = සංස්කාරක
files-share-role-commenter = අදහස් දක්වන්නා
files-share-role-viewer = නරඹන්නා
files-share-you = { $name } (ඔබ)
files-share-domain = { $domain } හි සියලු දෙනා
files-share-inherited = එය ඇති ෆෝල්ඩරයකින් ලැබෙන ප්‍රවේශය
files-share-remove = ප්‍රවේශය ඉවත් කරන්න
files-share-copy-link = සබැඳිය පිටපත් කරන්න
files-share-share = බෙදා ගන්න
files-share-done = හරි
files-share-sharing = බෙදා ගනිමින්…
files-share-shared = { $count ->
    [one] පුද්ගලයන් { $count } සමඟ බෙදා ගත්තා
   *[other] පුද්ගලයන් { $count } සමඟ බෙදා ගත්තා
}
files-share-refused = { $drive } හට { $addresses } සමඟ බෙදා ගත නොහැකි විය
files-share-failed = බෙදා ගැනීම වෙනස් කළ නොහැකි විය: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] අයිතම { $count } උඩුගත කරමින්
   *[other] අයිතම { $count } උඩුගත කරමින්
}
files-tray-done = { $count ->
    [one] උඩුගත කිරීම් { $count } අවසන්
   *[other] උඩුගත කිරීම් { $count } අවසන්
}
files-tray-some-failed = { $done } උඩුගත විය, { $failed } අසාර්ථකයි
files-tray-minutes-left = { $minutes ->
    [one] මිනිත්තුවක් පමණ ඉතිරියි
   *[other] මිනිත්තු { $minutes }ක් පමණ ඉතිරියි
}
files-tray-seconds-left = මිනිත්තුවකට අඩුවෙන් ඉතිරියි
files-tray-starting = ආරම්භ කරමින්…
files-tray-cancel-all = සියල්ල අවලංගු කරන්න
files-tray-cancel = අවලංගු කරන්න
files-tray-fold = ලැයිස්තුව සඟවන්න
files-tray-unfold = ලැයිස්තුව පෙන්වන්න
files-tray-close = වසන්න
files-tray-progress = { $place } · { $size } න් { $sent }
files-tray-in = { $place } තුළ
files-tray-cancelled = අවලංගු කළා
