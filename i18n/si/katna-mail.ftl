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
