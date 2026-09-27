# Katna Mail, Sinhala (සිංහල).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = භාෂාව: { $language }
language-tooltip-system = භාෂාව: { $language }, පද්ධතිය අනුව
language-search = භාෂාව සොයන්න
language-system-default = පද්ධති පෙරනිමිය
language-system-now = දැන් { $language }
language-no-match = “{ $query }” හා ගැළපෙන භාෂාවක් නැත
language-machine = යන්ත්‍ර පරිවර්තනයකි. වැඩිදියුණු කිරීමට උදවු කරන්න
language-setting = භාෂාව
language-setting-detail = මෙනු, බොත්තම් සහ පණිවිඩවල භාෂාව, සහ දින සහ අංකවල ආකෘතිය. පද්ධති පෙරනිමිය ඩෙස්ක්ටොප් සැකසීම් අනුගමනය කරයි.

## Dates and sizes

ago-just-now = මේ දැන්
ago-minutes = { $count ->
    [one] මිනිත්තු { $count }කට පෙර
   *[other] මිනිත්තු { $count }කට පෙර
}
ago-hours = { $count ->
    [one] පැය { $count }කට පෙර
   *[other] පැය { $count }කට පෙර
}
ago-days = { $count ->
    [one] දින { $count }කට පෙර
   *[other] දින { $count }කට පෙර
}
size-bytes = { $count ->
    [one] බයිට { $count }
   *[other] බයිට { $count }
}
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = ෆෝල්ඩර සඟවන්න
folders-show = ෆෝල්ඩර පෙන්වන්න
compose = රචනා කරන්න
search = සොයන්න
search-mail = තැපැල් සොයන්න
search-settings = සැකසීම් සොයන්න
search-clear = සෙවීම හිස් කරන්න
search-options-show = සෙවීම් විකල්ප පෙන්වන්න
settings = සැකසීම්
account-add = ගිණුමක් එක් කරන්න

## App rail (and the bottom bar on a phone)

rail-mail = තැපැල්
rail-calendar = දින දර්ශනය
rail-contacts = සම්බන්ධතා
rail-tasks = කාර්යයන්
rail-notes = සටහන්
rail-feeds = සංග්‍රහ

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = ළඟදීම
app-calendar-promise = ඔබේ CalDAV දින දර්ශන, ඔබේ තැපැල් වලින් ලැබෙන රැස්වීම් ආරාධනා සහ සිහිකැඳවීම්, ඔබේ එන ලිපි අසලම.
app-tasks-promise = CalDAV සමඟ සමමුහුර්ත වන කළ යුතු දේ ලැයිස්තු, සහ තැපැල් වලින් සාදන ලද කාර්යයන්.
app-notes-promise = ඉක්මන් සටහන්, සහ පසුව බැලීමට තැපැලක හෝ සංවාදයක සටහන්.
app-feeds-promise = RSS සහ Atom සංග්‍රහ ඔබේ තැපැල් අසලම කියවන්න.

## Contacts page

app-contacts-loading = ඔබේ තැපැල් වලින් පුද්ගලයන් එක්රැස් කරමින්…
app-contacts-empty = ඔබ ලිපි හුවමාරු කරන පුද්ගලයන් මෙහි පෙන්වයි.
app-contacts-count = { $count ->
    [one] ඔබේ තැපැල් වලින් පුද්ගලයන් { $count }, වැඩිපුරම ලිපි හුවමාරු කළ අය මුලින්
   *[other] ඔබේ තැපැල් වලින් පුද්ගලයන් { $count }, වැඩිපුරම ලිපි හුවමාරු කළ අය මුලින්
}
app-contacts-top = { $count ->
    [one] ඔබේ තැපැල් වලින් ඉහළම පුද්ගලයන් { $count }, වැඩිපුරම ලිපි හුවමාරු කළ අය මුලින්
   *[other] ඔබේ තැපැල් වලින් ඉහළම පුද්ගලයන් { $count }, වැඩිපුරම ලිපි හුවමාරු කළ අය මුලින්
}
app-contacts-messages = { $count ->
    [one] පණිවිඩ { $count }
   *[other] පණිවිඩ { $count }
}
app-contacts-last = අවසන් වරට { $date }

## Navigation (the folders pane)

nav-labels = ලේබල
nav-folders = ෆෝල්ඩර
nav-label-new = නව ලේබලයක් සාදන්න
nav-folder-new = නව ෆෝල්ඩරයක් සාදන්න
nav-account-unnamed = ගිණුම { $number }
nav-tab-new = { $count ->
    [one] නව { $count }
   *[other] නව { $count }
}

## Special folders (the user's own folders keep their names)

folder-inbox = එන ලිපි
folder-starred = තරු යෙදූ
folder-drafts = කෙටුම්පත්
folder-sent = යැවූ
folder-archive = සංරක්ෂිත
folder-spam = අයාචිත තැපැල්
folder-trash = කුණු කූඩය
folder-all-mail = සියලු තැපැල්
folder-scheduled = උපලේඛනගත

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = නව ලේබලය
label-folder-new-title = නව ෆෝල්ඩරය
label-prompt = කරුණාකර නව ලේබල නාමයක් ඇතුළු කරන්න:
label-folder-prompt = කරුණාකර නව ෆෝල්ඩර නාමයක් ඇතුළු කරන්න:
label-name-hint = ලේබල නාමය
label-folder-name-hint = ෆෝල්ඩර නාමය
label-nest = ලේබලය මේ යටතේ තබන්න:
label-folder-nest = ෆෝල්ඩරය මේ යටතේ තබන්න:
label-cancel = අවලංගු කරන්න
label-create = සාදන්න
label-creating = සාදමින්…
label-created = “{ $name }” ලේබලය සාදන ලදී.
label-folder-created = “{ $name }” ෆෝල්ඩරය සාදන ලදී.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = ප්‍රාථමික
tab-promotions = ප්‍රවර්ධන
tab-social = සමාජ
tab-updates = යාවත්කාලීන
tab-forums = සංසද
tab-focused = අවධානය යොමු කළ
tab-other = වෙනත්
tab-inbox = එන ලිපි
tab-newsletters = පුවත් හසුන්
tab-notifications = දැනුම්දීම්
tab-new = නව { $count }
tab-provider-other = Katna විසින් වර්ග කළ

## Mail list: toolbar

list-select = තෝරන්න
list-refresh = නැවුම් කරන්න
list-more = තවත්
list-mark-read = කියවූ ලෙස සලකුණු කරන්න
list-mark-unread = නොකියවූ ලෙස සලකුණු කරන්න
list-move-to = වෙත ගෙන යන්න
list-archive = සංරක්ෂණය කරන්න
list-spam = අයාචිත තැපැල් ලෙස වාර්තා කරන්න
list-delete = මකන්න
list-newer = අලුත්
list-older = පැරණි
list-range = { $total } න් { $first }–{ $last }
list-range-about = ආසන්න වශයෙන් { $total } න් { $first }–{ $last }
list-results = “{ $query }” සඳහා ප්‍රතිඵල
list-results-corrected = “{ $query }” සඳහා ප්‍රතිඵල පෙන්වමින්
list-search-instead = ඒ වෙනුවට “{ $query }” සොයන්න
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = සියල්ල
list-pick-none = කිසිවක් නැත
list-pick-read = කියවූ
list-pick-unread = නොකියවූ
list-pick-starred = තරු යෙදූ
list-pick-unstarred = තරු නොයෙදූ

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] සංවාද { $count } ම තෝරා ඇත.
       *[other] සංවාද { $count } ම තෝරා ඇත.
    }
   *[message] { $count ->
        [one] පණිවිඩ { $count } ම තෝරා ඇත.
       *[other] පණිවිඩ { $count } ම තෝරා ඇත.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } හි සංවාද { $count } ම තෝරා ඇත.
       *[other] { $folder } හි සංවාද { $count } ම තෝරා ඇත.
    }
   *[message] { $count ->
        [one] { $folder } හි පණිවිඩ { $count } ම තෝරා ඇත.
       *[other] { $folder } හි පණිවිඩ { $count } ම තෝරා ඇත.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] තිරයේ ඇති සංවාද { $count } ම තෝරා ඇත.
       *[other] තිරයේ ඇති සංවාද { $count } ම තෝරා ඇත.
    }
   *[message] { $count ->
        [one] තිරයේ ඇති පණිවිඩ { $count } ම තෝරා ඇත.
       *[other] තිරයේ ඇති පණිවිඩ { $count } ම තෝරා ඇත.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] සංවාද { $count } ම තෝරන්න
       *[other] සංවාද { $count } ම තෝරන්න
    }
   *[message] { $count ->
        [one] පණිවිඩ { $count } ම තෝරන්න
       *[other] පණිවිඩ { $count } ම තෝරන්න
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } හි සංවාද { $count } ම තෝරන්න
       *[other] { $folder } හි සංවාද { $count } ම තෝරන්න
    }
   *[message] { $count ->
        [one] { $folder } හි පණිවිඩ { $count } ම තෝරන්න
       *[other] { $folder } හි පණිවිඩ { $count } ම තෝරන්න
    }
}
list-clear-selection = තේරීම හිස් කරන්න

## Mail list: empty states

list-empty-search = ඔබේ සෙවීමට ගැළපෙන පණිවිඩ නැත.
list-empty-tab = { $tab } හි තැපැල් නැත.
list-empty-tab-unknown = මෙම ටැබයේ තැපැල් නැත.
list-empty-folder = { $folder } හි පණිවිඩ නැත.
list-empty-folder-unknown = මෙම ෆෝල්ඩරයේ පණිවිඩ නැත.
list-first-sync = ඔබේ තැපැල් ලබා ගනිමින්…
list-first-sync-detail = ඒවා ලැබෙන විට මෙහි පෙන්වයි.

## Mail list: lines

row-removed = මෙම පණිවිඩය ඉවත් කරන ලදී.
row-starred = තරු යෙදූ
row-not-starred = තරු නොයෙදූ
row-important = වැදගත්. වැදගත් නොවන ලෙස සලකුණු කිරීමට ක්ලික් කරන්න.
row-mark-important = වැදගත් ලෙස සලකුණු කරන්න
row-pinned = ඉහළට ඇමිණූ
row-pin = ඉහළට අමුණන්න
row-unpin = ඇමිණීම ඉවත් කරන්න

## Mail list: More menu and right-click menu

menu-reply = පිළිතුරු දෙන්න
menu-reply-all = සියල්ලන්ට පිළිතුරු දෙන්න
menu-forward = ඉදිරියට යවන්න
menu-archive = සංරක්ෂණය කරන්න
menu-delete = මකන්න
menu-spam = අයාචිත තැපැල් ලෙස වාර්තා කරන්න
menu-mark-read = කියවූ ලෙස සලකුණු කරන්න
menu-mark-unread = නොකියවූ ලෙස සලකුණු කරන්න
menu-mark-all-read = සියල්ල කියවූ ලෙස සලකුණු කරන්න
menu-star = තරුවක් එක් කරන්න
menu-unstar = තරුව ඉවත් කරන්න
menu-important = වැදගත් ලෙස සලකුණු කරන්න
menu-not-important = වැදගත් නොවන ලෙස සලකුණු කරන්න
menu-pin = ඉහළට අමුණන්න
menu-unpin = ඇමිණීම ඉවත් කරන්න
menu-print-all = සියල්ල මුද්‍රණය කරන්න
menu-new-window = නව කවුළුවක විවෘත කරන්න
menu-move-to = වෙත ගෙන යන්න
menu-move-to-heading = වෙත ගෙන යන්න:
menu-find-from = { $name } ගෙන් ලැබුණු ඊමේල් සොයන්න

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] සංවාදය සංරක්ෂණය කරන ලදී.
       *[other] සංවාද { $count } ක් සංරක්ෂණය කරන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩය සංරක්ෂණය කරන ලදී.
       *[other] පණිවිඩ { $count } ක් සංරක්ෂණය කරන ලදී.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] සංවාදය කුණු කූඩයට ගෙන යන ලදී.
       *[other] සංවාද { $count } ක් කුණු කූඩයට ගෙන යන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩය කුණු කූඩයට ගෙන යන ලදී.
       *[other] පණිවිඩ { $count } ක් කුණු කූඩයට ගෙන යන ලදී.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] සංවාදය ගෙන යන ලදී.
       *[other] සංවාද { $count } ක් ගෙන යන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩය ගෙන යන ලදී.
       *[other] පණිවිඩ { $count } ක් ගෙන යන ලදී.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] සංවාදයට තරුවක් යොදන ලදී.
       *[other] සංවාද { $count } කට තරු යොදන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩයට තරුවක් යොදන ලදී.
       *[other] පණිවිඩ { $count } කට තරු යොදන ලදී.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] සංවාදයෙන් තරුව ඉවත් කරන ලදී.
       *[other] සංවාද { $count } කින් තරු ඉවත් කරන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩයෙන් තරුව ඉවත් කරන ලදී.
       *[other] පණිවිඩ { $count } කින් තරු ඉවත් කරන ලදී.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] සංවාදය වැදගත් ලෙස සලකුණු කරන ලදී.
       *[other] සංවාද { $count } ක් වැදගත් ලෙස සලකුණු කරන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩය වැදගත් ලෙස සලකුණු කරන ලදී.
       *[other] පණිවිඩ { $count } ක් වැදගත් ලෙස සලකුණු කරන ලදී.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] සංවාදය වැදගත් නොවන ලෙස සලකුණු කරන ලදී.
       *[other] සංවාද { $count } ක් වැදගත් නොවන ලෙස සලකුණු කරන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩය වැදගත් නොවන ලෙස සලකුණු කරන ලදී.
       *[other] පණිවිඩ { $count } ක් වැදගත් නොවන ලෙස සලකුණු කරන ලදී.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] සංවාදය ඉහළට අමුණන ලදී.
       *[other] සංවාද { $count } ක් ඉහළට අමුණන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩය ඉහළට අමුණන ලදී.
       *[other] පණිවිඩ { $count } ක් ඉහළට අමුණන ලදී.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] සංවාදයේ ඇමිණීම ඉවත් කරන ලදී.
       *[other] සංවාද { $count } ක ඇමිණීම ඉවත් කරන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩයේ ඇමිණීම ඉවත් කරන ලදී.
       *[other] පණිවිඩ { $count } ක ඇමිණීම ඉවත් කරන ලදී.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] සංවාදය අයාචිත තැපැල් ලෙස වාර්තා කරන ලදී.
       *[other] සංවාද { $count } ක් අයාචිත තැපැල් ලෙස වාර්තා කරන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩය අයාචිත තැපැල් ලෙස වාර්තා කරන ලදී.
       *[other] පණිවිඩ { $count } ක් අයාචිත තැපැල් ලෙස වාර්තා කරන ලදී.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] සංවාදය සදහටම මකන ලදී.
       *[other] සංවාද { $count } ක් සදහටම මකන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩය සදහටම මකන ලදී.
       *[other] පණිවිඩ { $count } ක් සදහටම මකන ලදී.
    }
}
toast-undone = ක්‍රියාව අහෝසි කරන ලදී.
toast-undo = අහෝසි කරන්න
toast-no-spam-folder = මෙම ගිණුමට අයාචිත තැපැල් ෆෝල්ඩරයක් නැත.

## Reading pane: toolbar

reader-close = වසන්න
reader-back = ආපසු
reader-mark-unread = නොකියවූ ලෙස සලකුණු කරන්න
reader-move-to = වෙත ගෙන යන්න
reader-more = තවත්
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
print-not-downloaded = (තවම බාගත කර නැත.)
print-encrypted = (සංකේතනය කර ඇත. එහි පෙළ මුද්‍රණය කිරීමට එය Katna Mail හි විවෘත කරන්න.)
print-to = ලබන්නා: { $addresses }
print-cc = Cc: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = මෙම පණිවිඩයේ ඇමුණුම් කියවීමට එය විවෘත කරන්න.
text-copy = පිටපත් කරන්න
text-select-all = සියල්ල තෝරන්න

## Settings page: its tabs

settings-tab-general = සාමාන්‍ය
settings-tab-inbox = එන ලිපි
settings-tab-accounts = ගිණුම්
settings-tab-subscriptions = දායකත්ව
settings-tab-appearance = පෙනුම
settings-tab-shortcuts = කෙටිමං
settings-tab-default-apps = පෙරනිමි යෙදුම්
settings-tab-folders-rules = ෆෝල්ඩර සහ නීති
settings-tab-compose = රචනා කිරීම
settings-tab-mcp-server = MCP සේවාදායකය
settings-tab-feedback = පරිශීලක ප්‍රතිපෝෂණ
settings-tab-experimental = පර්යේෂණාත්මක

## Settings page: tabs still to come

settings-tab-subscriptions-coming = ඔබට ලැබෙන පුවත් හසුන් සහ තැපැල් ලැයිස්තු බලා, එක් ක්ලික් කිරීමකින් දායකත්වයෙන් ඉවත් වන්න.
settings-tab-folders-rules-coming = ෆෝල්ඩර සහ ලේබල සාදන්න, නැවත නම් කරන්න, ගෙන යන්න සහ සඟවන්න, සහ සමමුහුර්ත කළ යුතු ඒවා තෝරන්න. නීති මඟින් නව තැපැල් යවන්නා, විෂය හෝ වචන අනුව ස්වයංක්‍රීයව වර්ග කිරීම, ලේබල් කිරීම, ඉදිරියට යැවීම හෝ මැකීම කරයි.
settings-tab-mcp-server-coming = මෙම පරිගණකයේ AI සහායකයන්ට ඔබේ අවසරය ඇතිව ඔබේ තැපැල් සෙවීමට, කියවීමට සහ කෙටුම්පත් කිරීමට ඉඩ දෙන්න.

## Settings > General

settings-general-conversations = සංවාද දසුන
settings-general-conversations-group = එකම තැපැල් වෙත ලැබුණු පිළිතුරු එකට කාණ්ඩ කරන්න
settings-general-conversations-group-detail = ලැයිස්තුවේ එක් සංවාදයකට එක් පේළියක්
settings-general-reading = කියවීම
settings-general-newest-first = නවතම පණිවිඩය මුලින්
settings-general-newest-first-detail = සංවාදයක් එහි නවතම පිළිතුරෙන් ආරම්භ වේ
settings-general-full-headers = සම්පූර්ණ ශීර්ෂ පෙන්වන්න
settings-general-full-headers-detail = සෑම පණිවිඩයකම යවන්නා, ලබන්නා, cc, දිනය සහ විෂය විවෘතව පෙන්වයි
settings-general-full-names = ලබන්නන්ගේ සම්පූර්ණ නම්
settings-general-full-names-detail = “මා, Ada වෙත” වෙනුවට “මා, Ada Lovelace වෙත”
settings-general-mark-read = කියවූ ලෙස සලකුණු කරන්න
settings-general-mark-read-now = විවෘත වූ වහාම
settings-general-mark-read-1s = තත්පර 1ක් විවෘතව තිබූ පසු
settings-general-mark-read-3s = තත්පර 3ක් විවෘතව තිබූ පසු
settings-general-mark-read-never = මා කියවූ ලෙස සලකුණු කළ විට පමණි
settings-general-reply-button = පිළිතුරු බොත්තම
settings-general-reply-all = සියල්ලන්ට පිළිතුරු දෙන්න
settings-general-reply-all-detail = එක් එක් පණිවිඩය අසල ඇති පිළිතුරු බොත්තම යවන්නාට පමණක් නොව සියල්ලන්ටම පිළිතුරු දෙයි
settings-general-remote-images = වෙබයේ රූප
settings-general-remote-images-detail = පණිවිඩයක රූප පූරණය කිරීමෙන් ඔබ එය විවෘත කළ බව, කවදාද සහ ආසන්න වශයෙන් කොහේද යන්න එහි යවන්නාට දැනගත හැක. අක්‍රිය නම්, සෑම පණිවිඩයක්ම පළමුව විමසයි, සහ ඔබට ඕනෑම විටෙක යවන්නෙකුගේ රූප පෙන්විය හැක.
settings-general-remote-images-always = සැමවිටම රූප පෙන්වන්න
settings-general-remote-images-always-detail = ඔබ විශ්වාස කරන යවන්නන්ගෙන් පමණක් නොව, සෑම පණිවිඩයකම
settings-general-sending = යැවීම
settings-general-sending-detail = යැවූ පණිවිඩයක් ආපසු ගැනීමට හැකි වන පරිදි එය රැඳී සිටින කාලය.
settings-general-offline = නොබැඳි තැපැල්
settings-general-offline-detail = මෑත තැපැල් සම්බන්ධතාවක් නොමැතිව කියවීමට සම්පූර්ණයෙන් බාගත කෙරේ. පැරණි තැපැල් ඔබ විවෘත කරන විට බාගත වේ.
settings-general-offline-days = { $count ->
    [one] දින { $count }
   *[other] දින { $count }
}
settings-general-offline-years = { $count ->
    [one] වසර { $count }
   *[other] වසර { $count }
}
settings-general-offline-all = සියලු තැපැල්
settings-general-offline-note = දින ගණන අඩු කිරීමෙන් දැනටමත් බාගත කළ තැපැල් ඉවත් නොවේ. සේවාදායකයේ කිසිවක් වෙනස් නොවේ.
settings-general-notifications = දැනුම්දීම්
settings-general-notifications-detail = එන ලිපි වෙත ලැබෙන නව තැපැල් සඳහා, Katna Mail වසා ඇති විටත්.
settings-general-new-mail = නව තැපැල් ගැන මට දැනුම් දෙන්න
settings-general-new-mail-detail = සියල්ලන්ට පිළිතුරු දෙන්න, කියවූ ලෙස සලකුණු කරන්න සහ සංරක්ෂණය කරන්න යන බොත්තම් සමඟ
settings-general-new-mail-sound = ශබ්දයක් වාදනය කරන්න
settings-general-new-mail-sound-detail = ඩෙස්ක්ටොප් එකේ නව තැපැල් ශබ්දය
settings-general-desktop = ඩෙස්ක්ටොප්
settings-general-open-at-login = පිවිසීමේදී Katna Mail විවෘත කරන්න
settings-general-open-at-login-detail = සේවාව ධාවනය වන තාක් කෙසේ වෙතත් පිවිසීමේදී තැපැල් සමමුහුර්ත වේ
settings-general-tray = පද්ධති තැටියේ Katna පෙන්වන්න
settings-general-tray-detail = නොකියවූ ගණන සහ මෙනුවක් සමඟ
settings-general-unread-badge = කාර්ය තීරු අයිකනයේ නොකියවූ ගණන
settings-general-unread-badge-detail = එන ලිපි වල නොකියවූ පණිවිඩ කීයද යන්න

## Settings > Inbox

settings-inbox-tabs = එන ලිපි ටැබ
settings-inbox-tabs-detail = ඔබේ තැපැල් සපයන්නාගේ වෙබ් අඩවිය මෙන්, එන ලිපි ටැබවලට වර්ග කරන්න.
settings-inbox-tabs-show = එන ලිපි ටැබ පෙන්වන්න
settings-inbox-tabs-show-detail = අක්‍රිය නම් සෑම ගිණුමකටම එක් ලැයිස්තුවක් පෙන්වයි
settings-inbox-no-accounts = එහි ටැබ තේරීමට ගිණුමක් එක් කරන්න.
settings-inbox-tabs-automatic = ස්වයංක්‍රීය: { $tabs } ({ $provider })
settings-inbox-tabs-off = ටැබ නැත
settings-inbox-tabs-gmail = ප්‍රාථමික, ප්‍රවර්ධන, සමාජ, යාවත්කාලීන, සංසද
settings-inbox-tabs-focused = අවධානය යොමු කළ සහ වෙනත්
settings-inbox-tabs-zoho = එන ලිපි, පුවත් හසුන් සහ දැනුම්දීම්
settings-inbox-tabs-shown = පෙන්වන ටැබ. ඔබ අක්‍රිය කරන ටැබයක තැපැල් { $tab } හි රැඳේ.

## Settings > Appearance

settings-appearance-reading-pane = කියවීමේ පැනලය
settings-appearance-reading-pane-detail = විවෘත කළ සංවාදයක් පෙන්වන ස්ථානය.
settings-appearance-pane-right = ලැයිස්තුවේ දකුණට
settings-appearance-pane-none = බෙදීමක් නැත
settings-appearance-density = ඝනත්වය
settings-appearance-density-default = පෙරනිමි
settings-appearance-density-compact = සංයුක්ත
settings-appearance-scaling = පරිමාණනය
settings-appearance-scaling-detail = ඩෙස්ක්ටොප් එකේම පරිමාණයට අමතරව, Katna Mail හි සියල්ල විශාල හෝ කුඩා කරයි: පෙළ, අයිකන, පරතරය සහ බෙදුම් රේඛා. ඔබ යවන තැපැල් එහිම අකුරු ප්‍රමාණය තබා ගනී. ඉතා කුඩා ප්‍රමාණ නිසා අයිකන ක්ලික් කිරීම අපහසු විය හැක.
settings-appearance-theme = තේමාව
settings-appearance-theme-system = ඩෙස්ක්ටොප් එකට සමාන
settings-appearance-theme-light = ලා
settings-appearance-theme-dark = අඳුරු
settings-appearance-desktop-colors = ඩෙස්ක්ටොප් වර්ණ
settings-appearance-desktop-colors-use = ඩෙස්ක්ටොප් එකේ වර්ණ භාවිත කරන්න
settings-appearance-desktop-colors-use-detail = ඩෙස්ක්ටොප් එකේ වර්ණ රටාව සහ උච්චාරණ වර්ණය
settings-appearance-app-names = යෙදුම් නම්
settings-appearance-app-names-show = යෙදුම් නම් පෙන්වන්න
settings-appearance-app-names-show-detail = වම් කෙළවරේ යෙදුම් අයිකන යට නම්
settings-appearance-sender-pictures = යවන්නාගේ පින්තූර
settings-appearance-sender-pictures-show = සමාගම් ලාංඡන පෙන්වන්න
settings-appearance-sender-pictures-show-detail = පණිවිඩය අනුව නොව යවන්නාගේ වසම අනුව සොයා, සතියක් තබා ගනී
settings-appearance-important = වැදගත් සලකුණු
settings-appearance-important-show = වැදගත් සලකුණු පෙන්වන්න
settings-appearance-important-show-detail = ලැයිස්තුවේ එක් එක් පණිවිඩය අසල
settings-appearance-message-width = පණිවිඩ පළල
settings-appearance-message-width-limit = පණිවිඩවල පළල සීමා කරන්න
settings-appearance-message-width-limit-detail = පුළුල් කවුළුවක දිගු පේළි කියවීම පහසු වේ
settings-appearance-mail-colors = තැපැල් වර්ණ
settings-appearance-mail-colors-detail = බොහෝ තැපැල් සුදු පිටුවක් සඳහා නිර්මාණය කර ඇත. අඳුරු තේමාවක් සමඟ එහි වර්ණ හොඳින් කියවිය හැකි අඳුරු වර්ණවලට වෙනස් කෙරේ; අක්‍රිය නම්, එය ලා පිටුවක යවන්නාගේ වර්ණ තබා ගනී.
settings-appearance-dark-mail = තැපැල් සඳහාත් අඳුරු වර්ණ
settings-appearance-dark-mail-detail = තේමාව අඳුරු විට පමණි
settings-appearance-attachment-previews = ඇමුණුම් පෙරදසුන්
settings-appearance-attachment-previews-show = ඇමුණුම්වල පෙරදසුන් පෙන්වන්න
settings-appearance-attachment-previews-show-detail = එක් එක් ගොනුවේ කාඩ්පතේ එහි අන්තර්ගතයේ කුඩා රූපයක්

## Settings > Default apps

settings-default-apps-intro = ඇමුණුම් ක්ලික් කළ විට ඒවා විවෘත වන තැන. දර්ශකයට ඕනෑම විටෙක ගොනුවක් වෙනත් යෙදුමක ද විවෘත කළ හැක. ඩෙස්ක්ටොප් එකේ පෙරනිමි යෙදුම් එහිම සැකසීම් තුළ සකසා ඇත.
settings-default-apps-pdf = PDF ගොනු
settings-default-apps-pdf-detail = පිටු, විශාලනය සමඟ.
settings-default-apps-pictures = පින්තූර
settings-default-apps-pictures-detail = ඡායාරූප (කෙළින් හරවා), PNG, GIF, WebP, BMP, TIFF සහ SVG.
settings-default-apps-text = පෙළ ගොනු
settings-default-apps-text-detail = සරල පෙළ, ලොග, කේතය සහ වෙනත් පෙළ.
settings-default-apps-sheets = පැතුරුම්පත්
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) සහ CSV.
settings-default-apps-documents = ලේඛන
settings-default-apps-documents-detail = Word (docx) සහ OpenDocument පෙළ (odt).
settings-default-apps-katna = Katna Mail හි දර්ශකය
settings-default-apps-system = ඩෙස්ක්ටොප් එකේ පෙරනිමි යෙදුම
settings-default-apps-ask = සෑම විටම කුමන යෙදුමදැයි අසන්න
settings-default-apps-after-saving = සුරැකීමෙන් පසු
settings-default-apps-show-folder = සුරැකි ගොනු ඒවායේ ෆෝල්ඩරයේ පෙන්වන්න
settings-default-apps-show-folder-detail = සුරැකි ඇමුණුම් තෝරා ඇති ලෙස ගොනු කළමනාකරු විවෘත කරයි

## Settings > Compose

settings-compose-send-from = නව පණිවිඩ යැවිය යුත්තේ
settings-compose-send-from-detail = පිළිතුරු සහ ඉදිරියට යැවීම් සැමවිටම ඔබ සිටින ගිණුමෙන් යවයි.
settings-compose-send-from-current = ඔබ සිටින ගිණුම
settings-compose-send-on-replies = පිළිතුරුවලදී යවන්න
settings-compose-send-on-replies-detail = පිළිතුරක හෝ ඉදිරියට යැවීමක යවන්න බොත්තම කරන දේ. යවන්න අසල ඇති මෙනුව අනෙක ලබා දෙයි.
settings-compose-send-plain = යවන්න
settings-compose-send-archive = යවා සංරක්ෂණය කරන්න
settings-compose-signatures = අත්සන්
settings-compose-signatures-detail = ඔබේ පණිවිඩයට පහළින්, “--” පේළියකට පසුව එක් කෙරේ. රචනා කවුළුවේ වෙනත් එකක් තෝරන්න.
settings-compose-untitled = මාතෘකා රහිත
settings-compose-signature-name = නම, උදා: රැකියාව
settings-compose-signature-first = මගේ අත්සන
settings-compose-signature-numbered = අත්සන { $number }
settings-compose-signature-delete = මකන්න
settings-compose-signature-deleted = අත්සන මකන ලදී
settings-compose-signature-new = නව එකක් සාදන්න
settings-compose-no-signatures = තවම අත්සන් නැත.
settings-compose-no-signature = අත්සනක් නැත
settings-compose-for-new-mail = නව තැපැල් සඳහා
settings-compose-for-replies = පිළිතුරු සහ ඉදිරියට යැවීම් සඳහා
settings-compose-for-replies-detail = ඔබ පණිවිඩයක් අත්සන් කළ සංවාදයක, පිළිතුරක් ඒ වෙනුවට එම අත්සනින් ආරම්භ වේ.
settings-compose-format = ආකෘතිය
settings-compose-plain-text = සරල පෙළින් ලියන්න
settings-compose-plain-text-detail = නව තැපැල් හැඩතල ගැන්වීමකින් තොරව ආරම්භ වේ; රචනා කවුළුවට මාරු කළ හැක
settings-compose-spelling = අක්ෂර වින්‍යාසය
settings-compose-spell-check = මා ලියන අතරතුර අක්ෂර වින්‍යාසය පරීක්ෂා කරන්න
settings-compose-spell-check-detail = වැරදි අක්ෂර වින්‍යාසය ඇති වචන යටින් ඉරි ඇඳේ, දකුණු-ක්ලික් කිරීමෙන් යෝජනා සමඟ
settings-compose-spell-desktop = ඩෙස්ක්ටොප් එකේ භාෂාව ({ $language })
settings-compose-templates = අච්චු
settings-compose-templates-detail = ඔබ නිතර ලියන තැපැල් සුරකින්න, සහ එයින් නව තැපැලක් හෝ පිළිතුරක් ආරම්භ කරන්න.

## Settings > Shortcuts

settings-shortcuts-set = කෙටිමං කට්ටලය
settings-shortcuts-set-detail = ඔබ දන්නා තැපැල් යෙදුමක යතුරුවලින් ආරම්භ කරන්න. මෙහි Cmd යනු Ctrl ය. ඔබේම වෙනස්කම් කට්ටලයට ඉහළින් පවතින අතර, පෙරනිමි ප්‍රතිසාධනය කරන්න කට්ටලයේ යතුරු වෙත ආපසු යයි.
settings-shortcuts-single = තනි-යතුරු කෙටිමං
settings-shortcuts-single-detail = වෙබ් තැපැල් මෙන් Ctrl හෝ Alt නොමැති යතුරු: e සංරක්ෂණය කරයි, j සහ k ගෙන යයි, / සොයයි. ඒවා ලැයිස්තුවේ සහ විවෘත සංවාදයේ ක්‍රියා කරයි, ටයිප් කරන අතරතුර කිසි විටෙකත් නොවේ.
settings-shortcuts-single-use = තනි-යතුරු කෙටිමං භාවිත කරන්න
settings-shortcuts-single-use-detail = Ctrl කෙටිමං සැමවිටම ක්‍රියා කරයි
settings-shortcuts-how = යතුරක් වෙනස් කිරීමට එය ක්ලික් කරන්න, නැතහොත් එකක් එක් කිරීමට + ක්ලික් කරන්න, ඉන්පසු නව යතුරු ඔබන්න. Esc අවලංගු කරයි.
settings-shortcuts-restore = පෙරනිමි ප්‍රතිසාධනය කරන්න
settings-shortcuts-no-key = යතුරක් නැත
settings-shortcuts-press = යතුරු ඔබන්න…
settings-shortcuts-then = { $keys } ඉන්පසු…
settings-shortcuts-moved = { $keys } දැන් “{ $previous }” වෙනුවට “{ $action }” කරයි.
settings-shortcuts-single-off = තනි-යතුරු කෙටිමං අක්‍රියයි, එබැවින් මෙම යතුර ඒවා සක්‍රිය කළ පසු ක්‍රියා කරයි.
settings-shortcuts-restored = සෑම කෙටිමඟකටම නැවතත් එහි කට්ටලයේ යතුරු ඇත.

## Settings search: the line under a result

settings-general-language-summary = යෙදුමේ, දිනවල සහ අංකවල භාෂාව
settings-general-reading-summary = නවතම පණිවිඩය මුලින්, සම්පූර්ණ ශීර්ෂ, ලබන්නන්ගේ සම්පූර්ණ නම්
settings-general-mark-read-summary = විවෘත කළ සංවාදයක් කියවූ ලෙස සලකුණු වන්නේ කවදාද: වහාම, තත්පර 1කට හෝ 3කට පසු, නැතහොත් අතින්
settings-general-reply-button-summary = එක් එක් පණිවිඩය අසල ඇති පිළිතුරු බොත්තම සියල්ලන්ට පිළිතුරු දෙයි
settings-general-remote-images-summary = සෑම පණිවිඩයකම රූප සැමවිටම පෙන්වන්න
settings-general-sending-summary = යැවීම අහෝසි කිරීම: යැවූ පණිවිඩයක් ආපසු ගැනීමට හැකි වන පරිදි එය රැඳී සිටින කාලය
settings-general-offline-summary = සම්බන්ධතාවක් නොමැතිව කියවීමට මෑත තැපැල් දින කීයක් සම්පූර්ණයෙන් බාගත කෙරේද
settings-general-notifications-summary = නව තැපැල් දැනුම්දීම් සහ ඒවායේ ශබ්දය
settings-general-desktop-summary = පිවිසීමේදී Katna Mail විවෘත කිරීම, පද්ධති තැටි අයිකනය සහ කාර්ය තීරු අයිකනයේ නොකියවූ ගණන
settings-accounts-accounts-summary = ගිණුමක් එක් කරන්න හෝ ඉවත් කරන්න, නැතහොත් එහි පින්තූරය වෙනස් කරන්න
settings-appearance-density-summary = ලැයිස්තුවේ පෙරනිමි හෝ සංයුක්ත පේළි
settings-appearance-scaling-summary = සියල්ල විශාල හෝ කුඩා කරන්න: පෙළ, අයිකන, පරතරය සහ බෙදුම් රේඛා
settings-appearance-theme-summary = ඩෙස්ක්ටොප් එකට සමාන, ලා හෝ අඳුරු
settings-appearance-sender-pictures-summary = යවන්නාගේ වසම අනුව සොයන සමාගම් ලාංඡන
settings-appearance-important-summary = ලැයිස්තුවේ එක් එක් පණිවිඩය අසල ඇති වැදගත් සලකුණ
settings-appearance-mail-colors-summary = අඳුරු තේමාවක HTML තැපැල් සඳහා අඳුරු වර්ණ, නැතහොත් එහි යවන්නාගේ වර්ණ
settings-appearance-attachment-previews-summary = එක් එක් ඇමුණුමේ අන්තර්ගතයේ කුඩා රූපයක්
settings-shortcuts-set-summary = Gmail, Inbox by Gmail, Apple Mail, Outlook හෝ Thunderbird හි යතුරුවලින් ආරම්භ කරන්න
settings-shortcuts-single-summary = වෙබ් තැපැල් මෙන් Ctrl හෝ Alt නොමැති යතුරු
settings-default-apps-pdf-summary = PDF ඇමුණුම් විවෘත වන තැන
settings-default-apps-pictures-summary = ඡායාරූප සහ පින්තූර විවෘත වන තැන
settings-default-apps-text-summary = සරල පෙළ, ලොග සහ කේතය විවෘත වන තැන
settings-default-apps-sheets-summary = Excel, OpenDocument සහ CSV ගොනු විවෘත වන තැන
settings-default-apps-documents-summary = Word සහ OpenDocument පෙළ විවෘත වන තැන
settings-default-apps-after-saving-summary = සුරැකි ඇමුණුම් ඒවායේ ෆෝල්ඩරයේ පෙන්වන්න
settings-compose-send-from-summary = නව තැපැල් යවන ගිණුම: ඔබ සිටින එක, නැතහොත් සැමවිටම එකම එක
settings-compose-send-on-replies-summary = පිළිතුරු සහ ඉදිරියට යැවීම්වලදී යවන්න, නැතහොත් යවා සංවාදය සංරක්ෂණය කරන්න
settings-compose-signatures-summary = ඔබේ පණිවිඩයට පහළින්, “--” පේළියකට පසුව එක් කෙරේ
settings-compose-for-new-mail-summary = නව තැපැල් ආරම්භ වන අත්සන
settings-compose-for-replies-summary = පිළිතුරු සහ ඉදිරියට යැවීම් ආරම්භ වන අත්සන
settings-compose-format-summary = නව තැපැල් සරල පෙළින් ලියන්න
settings-compose-spelling-summary = ලියන අතරතුර අක්ෂර වින්‍යාසය පරීක්ෂා කිරීම, සහ ශබ්දකෝෂයේ භාෂාව
settings-compose-templates-summary = ළඟදීම: ඔබ නිතර ලියන තැපැල් සුරකින්න, සහ එයින් නව තැපැලක් හෝ පිළිතුරක් ආරම්භ කරන්න
settings-feedback-crash-reports-summary = Katna Mail හෝ එහි පසුබිම් සේවාව බිඳ වැටුණු විට බිඳවැටීම් වාර්තා මෙම පරිගණකයේ සුරකින්න
settings-feedback-saved-summary = මෙම පරිගණකයේ සුරැකි බිඳවැටීම් වාර්තා බලන්න, පිටපත් කරන්න හෝ මකන්න
settings-feedback-help-improve-summary = වැරදුණු දේ නිවැරදි කිරීමට උදවු වීමට බිඳවැටීම් වාර්තා යවන්න; ඔබ සක්‍රිය කරන තුරු අක්‍රියයි
settings-experimental-blur-summary = ඩෙස්ක්ටොප් එක ඉහළ තීරුව හරහා බොඳව පෙනෙන අතර, මෙනු මීදුම් වීදුරු ලෙස පෙනේ
settings-search-shortcut = යතුරුපුවරු කෙටිමඟ
settings-search-tab = සැකසීම් ටැබය
settings-search-none = “{ $query }” හා ගැළපෙන සැකසීම් නැත.
settings-search-results = “{ $query }” හා ගැළපෙන සැකසීම්

## Quick settings (the panel that slides in from the right)

quick-title = ඉක්මන් සැකසීම්
quick-see-all = සියලු සැකසීම් බලන්න
quick-reading-pane = කියවීමේ පැනලය
quick-pane-right = ලැයිස්තුවේ දකුණට
quick-pane-none = බෙදීමක් නැත
quick-density = ඝනත්වය
quick-density-default = පෙරනිමි
quick-density-compact = සංයුක්ත
quick-theme = තේමාව
quick-theme-system = ඩෙස්ක්ටොප් එකට සමාන
quick-theme-light = ලා
quick-theme-dark = අඳුරු
quick-desktop-colors = ඩෙස්ක්ටොප් වර්ණ
quick-desktop-colors-detail = ඩෙස්ක්ටොප් එකේ වර්ණ රටාව සහ උච්චාරණ වර්ණය
quick-app-names = යෙදුම් නම්
quick-app-names-detail = වම් කෙළවරේ යෙදුම් අයිකන යට නම්
quick-inbox-tabs = එන ලිපි ටැබ
quick-inbox-tabs-detail = එක් එක් ගිණුමේ තැපැල් සපයන්නාගේ ටැබ
quick-choose-tabs = ටැබ තෝරන්න
quick-choose-tabs-detail = සැකසීම් තුළ, ගිණුමකට අනුව
quick-sending = යැවීම
quick-undo-send = යැවීම අහෝසි කරන්න
quick-undo-send-off = අක්‍රියයි
quick-undo-send-seconds = තත්පර { $seconds }
quick-signatures = අත්සන්
quick-signatures-none = තවම නැත
quick-signatures-one = { $name }, පෙරනිමියෙන් භාවිත වේ
quick-signatures-many = { $count ->
    [one] අත්සන් { $count }; පෙරනිමියෙන් { $name }
   *[other] අත්සන් { $count }; පෙරනිමියෙන් { $name }
}
quick-signatures-no-default = { $count ->
    [one] { $count }, පෙරනිමියක් නැත
   *[other] { $count }, පෙරනිමියක් නැත
}
quick-signature-untitled = මාතෘකා රහිත
quick-threading = ඊමේල් සංවාද
quick-conversation-view = සංවාද දසුන
quick-conversation-view-detail = එකම තැපැල් වෙත ලැබුණු පිළිතුරු එකට කාණ්ඩ කරන්න
quick-help = උදවු
quick-tour = චාරිකාව ගන්න
quick-whats-new = අලුත් මොනවාද
quick-about = Katna ගැන

## Settings: opening at login

settings-open-at-login-failed = පිවිසීමේදී විවෘත කිරීම වෙනස් කළ නොහැකි විය: { $error }

## Settings > Appearance > Scaling

scale-letter = අ
scale-percent = { $percent }%
scale-reset = { $percent }% වෙත ආපසු

## Settings > Experimental > Look & Feel

look-intro = තවමත් අත්හදා බලමින් පවතින විශේෂාංග. ඒවා වෙනස් විය හැක හෝ ඉවත් විය හැක.
look-heading = පෙනුම සහ හැඟීම
look-window-frame = කවුළු රාමුව
look-window-frame-detail = මාතෘකා තීරුව, කවුළු බොත්තම්, කොන් සහ සෙවනැල්ල අඳින්නේ කවුද.
look-frame-native-kde = ස්වදේශීය: ඔබේ Plasma තේමාවේ KDE රාමුව
look-frame-native = ස්වදේශීය: ඩෙස්ක්ටොප් එකේ රාමුව
look-frame-katna = Katna: ඉහළ තීරුව මාතෘකා තීරුව බවට පත් වේ
look-frame-katna-note-named = Katna වටකුරු කොන් සහ එහිම සෙවනැල්ල අඳියි. රාමුව තවදුරටත් { $desktop } තේමාව අනුගමනය නොකරයි; කවුළු නීති තවමත් අදාළ වේ.
look-frame-katna-note = Katna වටකුරු කොන් සහ එහිම සෙවනැල්ල අඳියි. රාමුව තවදුරටත් ඩෙස්ක්ටොප් තේමාව අනුගමනය නොකරයි; කවුළු නීති තවමත් අදාළ වේ.
look-frame-client-side = ඔබේ ඩෙස්ක්ටොප් එක රාමුව එක් එක් යෙදුමට භාර දෙයි, එබැවින් Katna දැනටමත් එහිම රාමුව අඳියි.
look-blurred-background = බොඳ කළ පසුබිම
look-blurred-background-detail = ඩෙස්ක්ටොප් එක ඉහළ තීරුව සහ ෆෝල්ඩර හරහා බොඳව පෙනෙන අතර, මෙනු සහ උත්පතන මීදුම් වීදුරු ලෙස පෙනේ.
look-blur = කවුළුව පිටුපස ඇති දේ බොඳ කරන්න
look-blur-detail = තැපැල් ඝන කාඩ්පත් මත රැඳේ, එබැවින් පෙළ එහි වෙනස්කම තබා ගනී
look-blur-off-kde = KDE හි බොඳ කිරීමේ ප්‍රයෝගය අක්‍රියයි. පද්ධති සැකසීම්, කවුළු කළමනාකරණය, ඩෙස්ක්ටොප් ප්‍රයෝග තුළ බොඳ කිරීම සක්‍රිය කර, ඉන්පසු Katna Mail නැවත විවෘත කරන්න.
look-blur-none-gnome = GNOME කවුළු පිටුපස ඇති දේ බොඳ නොකරයි.
look-blur-none-x11 = ඔබේ කවුළු කළමනාකරු කවුළු පිටුපස ඇති දේ බොඳ නොකරයි.
look-blur-none-wayland = ඔබේ සංයුක්තකාරකය කවුළු පිටුපස ඇති දේ බොඳ නොකරයි.

## Settings > User feedback (crash reports)

feedback-intro-sending = වැරදුණු දේ නිවැරදි කිරීමට උදවු වීමට නව බිඳවැටීම් වාර්තා යවනු ලැබේ. වෙන කිසිවක් මෙම පරිගණකයෙන් පිටව නොයයි.
feedback-intro-local = Katna කිසිවක් කොහේවත් නොයවයි. බිඳවැටීම් වාර්තා මෙම පරිගණකයේ රැඳේ, ඔබට බැලීමට හෝ දෝෂ වාර්තාවකට ඇමිණීමට.
feedback-crash-reports = බිඳවැටීම් වාර්තා
feedback-crash-reports-detail = Katna Mail හෝ එහි පසුබිම් සේවාව බිඳ වැටුණු විට ලියනු ලැබේ.
feedback-save = බිඳවැටීම් වාර්තා මෙම පරිගණකයේ සුරකින්න
feedback-save-detail = ඔබේ නිවාස ෆෝල්ඩරය, පරිශීලක සහ පරිගණක නම් සහ ඊමේල් ලිපින ඉවත් කෙරේ
feedback-saved = සුරැකි බිඳවැටීම් වාර්තා
feedback-saved-detail = { $count ->
    [one] නවතම { $count } තබා ගනී.
   *[other] නවතම { $count } තබා ගනී.
}
feedback-help-improve = Katna වැඩිදියුණු කිරීමට උදවු කරන්න
feedback-help-improve-detail = ඔබ සක්‍රිය කරන තුරු අක්‍රියයි, සහ ඔබට ඕනෑම විටෙක මෙහි එය අක්‍රිය කළ හැක.
feedback-send = බිඳවැටීම් වාර්තා යවන්න
feedback-send-detail = සුරැකි වාර්තාව, ඔබට මෙහි බැලිය හැකි ආකාරයටම, Katna හි බිඳවැටීම් ලුහුබැඳීමට (Sentry, EU හි) යයි. IP ලිපිනයක්, පණිවිඩ හෝ ඊමේල් ලිපින නැත
feedback-none-saved = සුරැකි බිඳවැටීම් වාර්තා නැත.
feedback-delete-all = සියල්ල මකන්න
feedback-app-daemon = පසුබිම් සේවාව
feedback-report-sent = { $date } · යවන ලදී
feedback-view = බලන්න
feedback-view-tooltip = වාර්තාව විවෘත කරන්න
feedback-copy-tooltip = දෝෂ වාර්තාවකට ඇලවීමට එය පිටපත් කරන්න
feedback-copied = බිඳවැටීම් වාර්තාව පිටපත් කරන ලදී.
feedback-deleted-all = බිඳවැටීම් වාර්තා මකන ලදී.
feedback-read-failed = බිඳවැටීම් වාර්තාව කියවිය නොහැකි විය: { $error }
feedback-delete-failed = බිඳවැටීම් වාර්තාව මැකිය නොහැකි විය: { $error }
feedback-delete-all-failed = බිඳවැටීම් වාර්තා මැකිය නොහැකි විය: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _ගොනුව
desktop-menu-new-message = _නව පණිවිඩය
desktop-menu-quit = _ඉවත් වන්න
desktop-menu-edit = _සංස්කරණය
desktop-menu-undo = _අහෝසි කරන්න
desktop-menu-select-all = _සියල්ල තෝරන්න
desktop-menu-select-none = _කිසිවක් නොතෝරන්න
desktop-menu-find = _සොයන්න…
desktop-menu-view = _දසුන
desktop-menu-folder-list = _ෆෝල්ඩර ලැයිස්තුව පෙන්වන්න
desktop-menu-refresh = _නැවුම් කරන්න
desktop-menu-go = _යන්න
desktop-menu-inbox = _එන ලිපි
desktop-menu-starred = _තරු යෙදූ
desktop-menu-sent = _යැවූ
desktop-menu-drafts = _කෙටුම්පත්
desktop-menu-all-mail = _සියලු තැපැල්
desktop-menu-next = _ඊළඟ සංවාදය
desktop-menu-previous = _පෙර සංවාදය
desktop-menu-message = _පණිවිඩය
desktop-menu-open = _විවෘත කරන්න
desktop-menu-reply = _පිළිතුරු දෙන්න
desktop-menu-reply-all = _සියල්ලන්ට පිළිතුරු දෙන්න
desktop-menu-forward = _ඉදිරියට යවන්න
desktop-menu-archive = _සංරක්ෂණය කරන්න
desktop-menu-delete = _මකන්න
desktop-menu-spam = _අයාචිත තැපැල් ලෙස වාර්තා කරන්න
desktop-menu-move-to = _වෙත ගෙන යන්න…
desktop-menu-mark-read = _කියවූ ලෙස සලකුණු කරන්න
desktop-menu-mark-unread = _නොකියවූ ලෙස සලකුණු කරන්න
desktop-menu-star = _තරුවක් එක් කරන්න
desktop-menu-important = _වැදගත් ලෙස සලකුණු කරන්න
desktop-menu-not-important = _වැදගත් නොවන ලෙස සලකුණු කරන්න
desktop-menu-settings = _සැකසීම්
desktop-menu-quick-settings = _ඉක්මන් සැකසීම්
desktop-menu-configure = _Katna Mail වින්‍යාස කරන්න…
desktop-menu-help = _උදවු
desktop-menu-shortcuts = _යතුරුපුවරු කෙටිමං
desktop-menu-whats-new = _අලුත් මොනවාද
desktop-menu-about = _Katna ගැන

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = සැරිසැරීම
shortcut-group-actions = ක්‍රියා
shortcut-group-go-to = යන්න
shortcut-group-app = යෙදුම

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = ඊළඟ සංවාදය
shortcut-previous = පෙර සංවාදය
shortcut-down = ලැයිස්තුවේ පහළට යන්න
shortcut-up = ලැයිස්තුවේ ඉහළට යන්න
shortcut-first = ලැයිස්තුවේ පළමුවැන්න
shortcut-last = ලැයිස්තුවේ අවසානය
shortcut-page-down = ලැයිස්තුවේ පිටුවක් පහළට
shortcut-page-up = ලැයිස්තුවේ පිටුවක් ඉහළට
shortcut-open = සංවාදය විවෘත කරන්න
shortcut-back = ලැයිස්තුවට ආපසු
shortcut-scroll-down = පහළට අනුචලනය කරන්න
shortcut-scroll-up = ඉහළට අනුචලනය කරන්න
shortcut-scroll-page-down = පිටුවක් පහළට අනුචලනය කරන්න
shortcut-scroll-page-up = පිටුවක් ඉහළට අනුචලනය කරන්න
shortcut-compose = රචනා කරන්න
shortcut-reply = පිළිතුරු දෙන්න
shortcut-reply-all = සියල්ලන්ට පිළිතුරු දෙන්න
shortcut-forward = ඉදිරියට යවන්න
shortcut-archive = සංරක්ෂණය කරන්න
shortcut-delete = මකන්න
shortcut-spam = අයාචිත තැපැල් ලෙස වාර්තා කරන්න
shortcut-move-to = වෙත ගෙන යන්න
shortcut-mark-read = කියවූ ලෙස සලකුණු කරන්න
shortcut-mark-unread = නොකියවූ ලෙස සලකුණු කරන්න
shortcut-star = තරුව එක් කරන්න හෝ ඉවත් කරන්න
shortcut-important = වැදගත් ලෙස සලකුණු කරන්න
shortcut-not-important = වැදගත් නොවන ලෙස සලකුණු කරන්න
shortcut-check = සංවාදය සලකුණු කරන්න
shortcut-select-all = සියලු සංවාද සලකුණු කරන්න
shortcut-select-none = සියලු සංවාදවල සලකුණු ඉවත් කරන්න
shortcut-undo = අවසන් ක්‍රියාව අහෝසි කරන්න
shortcut-go-inbox = එන ලිපි
shortcut-go-starred = තරු යෙදූ
shortcut-go-sent = යැවූ
shortcut-go-drafts = කෙටුම්පත්
shortcut-go-all = සියලු තැපැල්
shortcut-search = තැපැල් සොයන්න
shortcut-navigation = මෙනුව පෙන්වන්න හෝ හකුළන්න
shortcut-quick-settings = ඉක්මන් සැකසීම්
shortcut-settings = සියලු සැකසීම්
shortcut-shortcuts = යතුරුපුවරු කෙටිමං
shortcut-reload = නව තැපැල් සඳහා පරීක්ෂා කරන්න
shortcut-quit = ඉවත් වන්න

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } ඉන්පසු { $second }

## Settings > Accounts

accounts-folder-pane = ෆෝල්ඩර පැනලය
accounts-folder-pane-detail = වම් පස පැනලය පෙන්වන්නේ කුමන ගිණුම්වල ෆෝල්ඩරද යන්න.
accounts-shown-one = වරකට එක් ගිණුමක්; ගිණුම් කාඩ්පතෙන් මාරු වන්න
accounts-shown-all = සියලු ගිණුම්, එකකට පසු එකක්
accounts-row = ගිණුම්
accounts-row-detail = ගිණුමක් ඉවත් කිරීමෙන් මෙම පරිගණකයේ ඇති Katna සතු එහි තැපැල් පිටපත මැකේ. තැපැල් සේවාදායකයේ රැඳේ.
accounts-none = තවම ගිණුම් නැත.
accounts-kind-imported = ආයාත කළ
accounts-picture-reset = ඩෙස්ක්ටොප් පින්තූරය භාවිත කරන්න
accounts-picture-change = පින්තූරය වෙනස් කරන්න
accounts-remove = ඉවත් කරන්න
accounts-delete-all-row = සියලු දත්ත මකන්න
accounts-delete-all-row-detail = නව ස්ථාපනයක මෙන් මුල සිට අරඹන්න.
accounts-delete-all-about = සෑම ගිණුමක්ම, සියලු ගබඩා කළ තැපැල්, සම්බන්ධතා සහ දින දර්ශන, සෙවුම් සුචිය, ඔබේ සැකසීම් සහ සුරැකි මුරපද මෙම පරිගණකයෙන් මකයි. ඔබේ තැපැල් සේවාදායකවල කිසිවක් වෙනස් නොවේ.
accounts-delete-all-open = සියලු Katna දත්ත මකන්න

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } Katna වෙතින් ඉවත් කරන ලදී.
accounts-removed = { $address } Katna වෙතින් ඉවත් කරන ලදී. එහි තැපැල් තවමත් සේවාදායකයේ ඇත.
accounts-all-deleted = සියලු Katna දත්ත මෙම පරිගණකයෙන් මකන ලදී.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = { $address } ඉවත් කරන්නද?
accounts-remove-confirm = ගිණුම ඉවත් කරන්න
accounts-removing = ඉවත් කරමින්…
accounts-remove-local-mail = { $folders ->
    [0] මෙම ගිණුමට ආයාත කළ සියලු තැපැල්
    [one] මෙම ගිණුමට එහි ෆෝල්ඩරයට ආයාත කළ සියලු තැපැල්
   *[other] මෙම ගිණුමට එහි ෆෝල්ඩර { $folders } තුළට ආයාත කළ සියලු තැපැල්
}
accounts-remove-local-settings = එහි Katna සැකසීම්
accounts-remove-mail = { $folders ->
    [0] Katna ගබඩා කළ මෙම ගිණුමේ සියලු තැපැල්
    [one] Katna විසින් එහි ෆෝල්ඩරයේ ගබඩා කළ මෙම ගිණුමේ සියලු තැපැල්
   *[other] Katna විසින් එහි ෆෝල්ඩර { $folders } තුළ ගබඩා කළ මෙම ගිණුමේ සියලු තැපැල්
}
accounts-remove-outbox = පිටතට යන ලිපි තුළ යැවීමට බලා සිටින එහි පණිවිඩ
accounts-remove-settings = එහි සුරැකි මුරපදය සහ එහි Katna සැකසීම්
accounts-delete-all-title = සියලු Katna දත්ත මකන්නද?
accounts-delete-all-confirm = සියල්ල මකන්න
accounts-deleting = මකමින්…
accounts-delete-all-accounts = සෑම ගිණුමක්ම, සහ Katna ගබඩා කළ සියලු තැපැල් සහ ඇමුණුම්
accounts-delete-all-contacts = සම්බන්ධතා, දින දර්ශන සහ සෙවුම් සුචිය
accounts-delete-all-settings = සියලු සැකසීම්, අත්සන් සහ යතුරුපුවරු කෙටිමං
accounts-delete-all-passwords = සෑම සුරැකි මුරපදයක්ම
accounts-deleted-heading = මෙම පරිගණකයෙන් මැකෙන දේ:
accounts-cannot-undo = මෙය අහෝසි කළ නොහැක.
accounts-server-delete-all = ඔබේ තැපැල් සේවාදායකවල කිසිවක් වෙනස් නොවේ: ඔබේ තැපැල් එහි රැඳේ, සහ ගිණුමක් නැවත එක් කිරීමෙන් එය නැවත බාගත වේ. ගොනුවලින් ආයාත කළ තැපැල් ඇත්තේ Katna හි පමණි; ගොනු ස්පර්ශ නොකෙරේ.
accounts-server-local = මෙම තැපැල් ගොනුවලින් ආයාත කළ නිසා, එකම පිටපත ඇත්තේ Katna සතුවය. එය පැමිණි ගොනු ස්පර්ශ නොකෙරේ; එය ආපසු ලබා ගැනීමට ඒවා නැවත ආයාත කරන්න.
accounts-server-remove = තැපැල් සේවාදායකයේ කිසිවක් වෙනස් නොවේ: ඔබේ තැපැල් එහි රැඳේ, සහ ගිණුම නැවත එක් කිරීමෙන් එය නැවත බාගත වේ.
accounts-confirm-word = මකන්න
accounts-confirm-placeholder = “{ accounts-confirm-word }” ටයිප් කරන්න
accounts-confirm-prompt = තහවුරු කිරීමට, “{ accounts-confirm-word }” ටයිප් කරන්න:
accounts-cancel = අවලංගු කරන්න
