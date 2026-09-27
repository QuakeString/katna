# Katna Mail, Amharic (አማርኛ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = ቋንቋ፦ { $language }
language-tooltip-system = ቋንቋ፦ { $language }፣ ሥርዓቱን በመከተል
language-search = ቋንቋ ፈልግ
language-system-default = የሥርዓት ነባሪ
language-system-now = አሁን { $language }
language-no-match = ከ«{ $query }» ጋር የሚዛመድ ቋንቋ የለም
language-machine = በማሽን የተተረጎመ። እንዲሻሻል ያግዙ
language-setting = ቋንቋ
language-setting-detail = የምናሌዎች፣ የአዝራሮች እና የመልዕክቶች ቋንቋ፣ እንዲሁም የቀኖች እና የቁጥሮች ቅርጸት። የሥርዓት ነባሪ የዴስክቶፑን ቋንቋ ይከተላል።

## Dates and sizes

ago-just-now = አሁን
ago-minutes = { $count ->
    [one] ከ{ $count } ደቂቃ በፊት
   *[other] ከ{ $count } ደቂቃዎች በፊት
}
ago-hours = { $count ->
    [one] ከ{ $count } ሰዓት በፊት
   *[other] ከ{ $count } ሰዓታት በፊት
}
ago-days = { $count ->
    [one] ከ{ $count } ቀን በፊት
   *[other] ከ{ $count } ቀናት በፊት
}
size-bytes = { $count ->
    [one] { $count } ባይት
   *[other] { $count } ባይቶች
}
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = አቃፊዎችን ደብቅ
folders-show = አቃፊዎችን አሳይ
compose = ጻፍ
search = ፈልግ
search-mail = ደብዳቤ ፈልግ
search-settings = ቅንብሮችን ፈልግ
search-clear = ፍለጋን አጽዳ
search-options-show = የፍለጋ አማራጮችን አሳይ
settings = ቅንብሮች
account-add = መለያ አክል

## App rail (and the bottom bar on a phone)

rail-mail = ደብዳቤ
rail-calendar = ቀን መቁጠሪያ
rail-contacts = እውቂያዎች
rail-tasks = ተግባራት
rail-notes = ማስታወሻዎች
rail-feeds = ምግቦች

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = በቅርቡ ይመጣል
app-calendar-promise = የእርስዎ CalDAV ቀን መቁጠሪያዎች፣ ከደብዳቤዎ የሚመጡ የስብሰባ ግብዣዎች እና አስታዋሾች፣ ከገቢ መልዕክት ሳጥንዎ አጠገብ።
app-tasks-promise = ከCalDAV ጋር የሚሰምሩ የሚደረጉ ነገሮች ዝርዝሮች፣ እና ከደብዳቤ የተፈጠሩ ተግባራት።
app-notes-promise = ፈጣን ማስታወሻዎች፣ እና ለበኋላ በደብዳቤ ወይም በውይይት ላይ የሚያዙ ማስታወሻዎች።
app-feeds-promise = የRSS እና Atom ምግቦችን ከደብዳቤዎ አጠገብ ያንብቡ።

## Contacts page

app-contacts-loading = ከደብዳቤዎ ሰዎችን በመሰብሰብ ላይ…
app-contacts-empty = የሚጻጻፏቸው ሰዎች እዚህ ይታያሉ።
app-contacts-count = { $count ->
    [one] ከደብዳቤዎ { $count } ሰው፣ በብዛት የሚጻጻፏቸው መጀመሪያ
   *[other] ከደብዳቤዎ { $count } ሰዎች፣ በብዛት የሚጻጻፏቸው መጀመሪያ
}
app-contacts-top = { $count ->
    [one] ከደብዳቤዎ ቀዳሚው { $count } ሰው፣ በብዛት የሚጻጻፏቸው መጀመሪያ
   *[other] ከደብዳቤዎ ቀዳሚዎቹ { $count } ሰዎች፣ በብዛት የሚጻጻፏቸው መጀመሪያ
}
app-contacts-messages = { $count ->
    [one] { $count } መልዕክት
   *[other] { $count } መልዕክቶች
}
app-contacts-last = መጨረሻ { $date }

## Navigation (the folders pane)

nav-labels = መሰየሚያዎች
nav-folders = አቃፊዎች
nav-label-new = አዲስ መሰየሚያ ፍጠር
nav-folder-new = አዲስ አቃፊ ፍጠር
nav-account-unnamed = መለያ { $number }
nav-tab-new = { $count ->
    [one] { $count } አዲስ
   *[other] { $count } አዲስ
}

## Special folders (the user's own folders keep their names)

folder-inbox = ገቢ መልዕክት ሳጥን
folder-starred = ኮከብ የተደረገባቸው
folder-drafts = ረቂቆች
folder-sent = የተላኩ
folder-archive = ማህደር
folder-spam = አይፈለጌ መልዕክት
folder-trash = መጣያ
folder-all-mail = ሁሉም ደብዳቤ
folder-scheduled = መርሐግብር የተያዘላቸው

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = አዲስ መሰየሚያ
label-folder-new-title = አዲስ አቃፊ
label-prompt = እባክዎ አዲስ የመሰየሚያ ስም ያስገቡ፦
label-folder-prompt = እባክዎ አዲስ የአቃፊ ስም ያስገቡ፦
label-name-hint = የመሰየሚያ ስም
label-folder-name-hint = የአቃፊ ስም
label-nest = መሰየሚያውን ከዚህ ስር አስቀምጥ፦
label-folder-nest = አቃፊውን ከዚህ ስር አስቀምጥ፦
label-cancel = ይቅር
label-create = ፍጠር
label-creating = በመፍጠር ላይ…
label-created = መሰየሚያ «{ $name }» ተፈጥሯል።
label-folder-created = አቃፊ «{ $name }» ተፈጥሯል።

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = ዋና
tab-promotions = ማስተዋወቂያዎች
tab-social = ማህበራዊ
tab-updates = ዝማኔዎች
tab-forums = መድረኮች
tab-focused = ትኩረት የተሰጣቸው
tab-other = ሌሎች
tab-inbox = ገቢ መልዕክት ሳጥን
tab-newsletters = ጋዜጣዎች
tab-notifications = ማሳወቂያዎች
tab-new = { $count } አዲስ
tab-provider-other = በKatna የተደረደሩ

## Mail list: toolbar

list-select = ምረጥ
list-refresh = አድስ
list-more = ተጨማሪ
list-mark-read = እንደተነበበ ምልክት አድርግ
list-mark-unread = እንዳልተነበበ ምልክት አድርግ
list-move-to = ውሰድ ወደ
list-archive = ወደ ማህደር አስቀምጥ
list-spam = አይፈለጌ መልዕክት ሪፖርት አድርግ
list-delete = ሰርዝ
list-newer = አዲስ
list-older = የቆየ
list-range = { $first }–{ $last } ከ{ $total }
list-range-about = { $first }–{ $last } ከ{ $total } ገደማ
list-results = የ«{ $query }» ውጤቶች
list-results-corrected = የ«{ $query }» ውጤቶችን በማሳየት ላይ
list-search-instead = በምትኩ «{ $query }»ን ፈልግ
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = ሁሉም
list-pick-none = ምንም
list-pick-read = የተነበቡ
list-pick-unread = ያልተነበቡ
list-pick-starred = ኮከብ የተደረገባቸው
list-pick-unstarred = ኮከብ ያልተደረገባቸው

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] ሁሉም { $count } ውይይት ተመርጧል።
       *[other] ሁሉም { $count } ውይይቶች ተመርጠዋል።
    }
   *[message] { $count ->
        [one] ሁሉም { $count } መልዕክት ተመርጧል።
       *[other] ሁሉም { $count } መልዕክቶች ተመርጠዋል።
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] በ{ $folder } ውስጥ ያለው ሁሉም { $count } ውይይት ተመርጧል።
       *[other] በ{ $folder } ውስጥ ያሉት ሁሉም { $count } ውይይቶች ተመርጠዋል።
    }
   *[message] { $count ->
        [one] በ{ $folder } ውስጥ ያለው ሁሉም { $count } መልዕክት ተመርጧል።
       *[other] በ{ $folder } ውስጥ ያሉት ሁሉም { $count } መልዕክቶች ተመርጠዋል።
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] በማያ ገጹ ላይ ያለው ሁሉም { $count } ውይይት ተመርጧል።
       *[other] በማያ ገጹ ላይ ያሉት ሁሉም { $count } ውይይቶች ተመርጠዋል።
    }
   *[message] { $count ->
        [one] በማያ ገጹ ላይ ያለው ሁሉም { $count } መልዕክት ተመርጧል።
       *[other] በማያ ገጹ ላይ ያሉት ሁሉም { $count } መልዕክቶች ተመርጠዋል።
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] ሁሉንም { $count } ውይይት ምረጥ
       *[other] ሁሉንም { $count } ውይይቶች ምረጥ
    }
   *[message] { $count ->
        [one] ሁሉንም { $count } መልዕክት ምረጥ
       *[other] ሁሉንም { $count } መልዕክቶች ምረጥ
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] በ{ $folder } ውስጥ ያለውን ሁሉንም { $count } ውይይት ምረጥ
       *[other] በ{ $folder } ውስጥ ያሉትን ሁሉንም { $count } ውይይቶች ምረጥ
    }
   *[message] { $count ->
        [one] በ{ $folder } ውስጥ ያለውን ሁሉንም { $count } መልዕክት ምረጥ
       *[other] በ{ $folder } ውስጥ ያሉትን ሁሉንም { $count } መልዕክቶች ምረጥ
    }
}
list-clear-selection = ምርጫን አጽዳ

## Mail list: empty states

list-empty-search = ከፍለጋዎ ጋር የሚዛመድ መልዕክት የለም።
list-empty-tab = በ{ $tab } ውስጥ ምንም ደብዳቤ የለም።
list-empty-tab-unknown = በዚህ ትር ውስጥ ምንም ደብዳቤ የለም።
list-empty-folder = በ{ $folder } ውስጥ ምንም መልዕክት የለም።
list-empty-folder-unknown = በዚህ አቃፊ ውስጥ ምንም መልዕክት የለም።
list-first-sync = ደብዳቤዎን በማምጣት ላይ…
list-first-sync-detail = ሲደርስ እዚህ ይታያል።

## Mail list: lines

row-removed = ይህ መልዕክት ተወግዷል።
row-starred = ኮከብ የተደረገበት
row-not-starred = ኮከብ ያልተደረገበት
row-important = አስፈላጊ። አስፈላጊ እንዳልሆነ ምልክት ለማድረግ ጠቅ ያድርጉ።
row-mark-important = እንደ አስፈላጊ ምልክት አድርግ
row-pinned = ከላይ ተሰክቷል
row-pin = ከላይ ሰካ
row-unpin = ንቀል

## Mail list: More menu and right-click menu

menu-reply = መልስ
menu-reply-all = ለሁሉም መልስ
menu-forward = አስተላልፍ
menu-archive = ወደ ማህደር አስቀምጥ
menu-delete = ሰርዝ
menu-spam = አይፈለጌ መልዕክት ሪፖርት አድርግ
menu-mark-read = እንደተነበበ ምልክት አድርግ
menu-mark-unread = እንዳልተነበበ ምልክት አድርግ
menu-mark-all-read = ሁሉንም እንደተነበቡ ምልክት አድርግ
menu-star = ኮከብ አክል
menu-unstar = ኮከብ አስወግድ
menu-important = እንደ አስፈላጊ ምልክት አድርግ
menu-not-important = አስፈላጊ እንዳልሆነ ምልክት አድርግ
menu-pin = ከላይ ሰካ
menu-unpin = ንቀል
menu-print-all = ሁሉንም አትም
menu-new-window = በአዲስ መስኮት ክፈት
menu-move-to = ውሰድ ወደ
menu-move-to-heading = ውሰድ ወደ፦
menu-find-from = ከ{ $name } የመጡ ኢሜይሎችን ፈልግ

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] ውይይቱ ወደ ማህደር ተቀምጧል።
       *[other] { $count } ውይይቶች ወደ ማህደር ተቀምጠዋል።
    }
   *[message] { $count ->
        [one] መልዕክቱ ወደ ማህደር ተቀምጧል።
       *[other] { $count } መልዕክቶች ወደ ማህደር ተቀምጠዋል።
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] ውይይቱ ወደ መጣያ ተወስዷል።
       *[other] { $count } ውይይቶች ወደ መጣያ ተወስደዋል።
    }
   *[message] { $count ->
        [one] መልዕክቱ ወደ መጣያ ተወስዷል።
       *[other] { $count } መልዕክቶች ወደ መጣያ ተወስደዋል።
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] ውይይቱ ተወስዷል።
       *[other] { $count } ውይይቶች ተወስደዋል።
    }
   *[message] { $count ->
        [one] መልዕክቱ ተወስዷል።
       *[other] { $count } መልዕክቶች ተወስደዋል።
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] ውይይቱ ኮከብ ተደርጎበታል።
       *[other] { $count } ውይይቶች ኮከብ ተደርጎባቸዋል።
    }
   *[message] { $count ->
        [one] መልዕክቱ ኮከብ ተደርጎበታል።
       *[other] { $count } መልዕክቶች ኮከብ ተደርጎባቸዋል።
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] ከውይይቱ ኮከብ ተወግዷል።
       *[other] ከ{ $count } ውይይቶች ኮከብ ተወግዷል።
    }
   *[message] { $count ->
        [one] ከመልዕክቱ ኮከብ ተወግዷል።
       *[other] ከ{ $count } መልዕክቶች ኮከብ ተወግዷል።
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] ውይይቱ አስፈላጊ ተብሎ ምልክት ተደርጎበታል።
       *[other] { $count } ውይይቶች አስፈላጊ ተብለው ምልክት ተደርጎባቸዋል።
    }
   *[message] { $count ->
        [one] መልዕክቱ አስፈላጊ ተብሎ ምልክት ተደርጎበታል።
       *[other] { $count } መልዕክቶች አስፈላጊ ተብለው ምልክት ተደርጎባቸዋል።
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] ውይይቱ አስፈላጊ አይደለም ተብሎ ምልክት ተደርጎበታል።
       *[other] { $count } ውይይቶች አስፈላጊ አይደሉም ተብለው ምልክት ተደርጎባቸዋል።
    }
   *[message] { $count ->
        [one] መልዕክቱ አስፈላጊ አይደለም ተብሎ ምልክት ተደርጎበታል።
       *[other] { $count } መልዕክቶች አስፈላጊ አይደሉም ተብለው ምልክት ተደርጎባቸዋል።
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] ውይይቱ ከላይ ተሰክቷል።
       *[other] { $count } ውይይቶች ከላይ ተሰክተዋል።
    }
   *[message] { $count ->
        [one] መልዕክቱ ከላይ ተሰክቷል።
       *[other] { $count } መልዕክቶች ከላይ ተሰክተዋል።
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] ውይይቱ ተነቅሏል።
       *[other] { $count } ውይይቶች ተነቅለዋል።
    }
   *[message] { $count ->
        [one] መልዕክቱ ተነቅሏል።
       *[other] { $count } መልዕክቶች ተነቅለዋል።
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] ውይይቱ አይፈለጌ መልዕክት ተብሎ ሪፖርት ተደርጓል።
       *[other] { $count } ውይይቶች አይፈለጌ መልዕክት ተብለው ሪፖርት ተደርገዋል።
    }
   *[message] { $count ->
        [one] መልዕክቱ አይፈለጌ መልዕክት ተብሎ ሪፖርት ተደርጓል።
       *[other] { $count } መልዕክቶች አይፈለጌ መልዕክት ተብለው ሪፖርት ተደርገዋል።
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] ውይይቱ እስከመጨረሻው ተሰርዟል።
       *[other] { $count } ውይይቶች እስከመጨረሻው ተሰርዘዋል።
    }
   *[message] { $count ->
        [one] መልዕክቱ እስከመጨረሻው ተሰርዟል።
       *[other] { $count } መልዕክቶች እስከመጨረሻው ተሰርዘዋል።
    }
}
toast-undone = እርምጃው ተቀልብሷል።
toast-undo = ቀልብስ
toast-no-spam-folder = ይህ መለያ የአይፈለጌ መልዕክት አቃፊ የለውም።

## Reading pane: toolbar

reader-close = ዝጋ
reader-back = ተመለስ
reader-mark-unread = እንዳልተነበበ ምልክት አድርግ
reader-move-to = ውሰድ ወደ
reader-more = ተጨማሪ
reader-print-all = ሁሉንም አትም
reader-new-window = በአዲስ መስኮት
reader-position = { $position } ከ{ $total }
reader-newer = አዲስ
reader-older = የቆየ

## Reading pane: the conversation

reader-removed = ይህ ውይይት ተወግዷል።
reader-no-subject = (ርዕሰ ጉዳይ የለም)
reader-collapse-all = ሁሉንም ሰብስብ
reader-expand-all = ሁሉንም ዘርጋ
reader-unknown-sender = (ያልታወቀ ላኪ)
reader-date-ago = { $date } ({ $ago })
reader-me = እኔ
reader-to = ለ{ $names }
reader-starred = ኮከብ የተደረገበት
reader-not-starred = ኮከብ ያልተደረገበት
reader-too-long = መልዕክቱ ሙሉ በሙሉ ለማሳየት በጣም ረጅም ነው።
reader-encrypted-images = በተመሰጠረ ደብዳቤ ውስጥ ከድር የሚመጡ ምስሎች በጭራሽ አይጫኑም።
reader-window-failed = አዲስ መስኮት መክፈት አልተቻለም።

## Reading pane: message details (opened from "to me")

reader-details-from = ከ፦
reader-details-to = ለ፦
reader-details-cc = ግልባጭ፦
reader-details-date = ቀን፦
reader-details-subject = ርዕሰ ጉዳይ፦

## Reading pane: downloading a message

reader-downloading = ይህን መልዕክት ከአገልጋዩ በማውረድ ላይ…
reader-download-failed = ይህን መልዕክት ማውረድ አልተቻለም።
reader-try-again = እንደገና ሞክር

## Reply row

reply-reply = መልስ
reply-reply-all = ለሁሉም መልስ
reply-forward = አስተላልፍ

## Encrypted and signed mail

security-decrypting = ምስጠራውን በመፍታት ላይ…
security-checking = ፊርማውን በማረጋገጥ ላይ…
security-partly-encrypted = የዚህ መልዕክት ክፍል ብቻ ነው የተመሰጠረው። የተቀረው ከጥበቃው ውጭ የተጨመረ ሲሆን ከማንኛውም ሰው ሊመጣ ይችላል።
security-partly-signed = የዚህ መልዕክት ክፍል ብቻ ነው የተፈረመው። የተቀረው ከጥበቃው ውጭ የተጨመረ ሲሆን ከማንኛውም ሰው ሊመጣ ይችላል።
security-encrypted = የተመሰጠረ መልዕክት
security-encrypted-smime = የተመሰጠረ መልዕክት (S/MIME)
security-no-key = ይህን መልዕክት መፍታት አይቻልም፦ የተመሰጠረው እርስዎ በሌለዎት ቁልፍ ነው።
security-cancelled = መፍታቱ ተሰርዟል።
security-damaged = ይህን መልዕክት መፍታት አይቻልም፦ የተመሰጠረው ውሂብ ተበላሽቷል ወይም ተቀይሯል።
security-decrypt-unavailable = ይህን መልዕክት መፍታት አይቻልም፦ የተመሰጠረ ደብዳቤ ለማንበብ { $tool }ን ይጫኑ።
security-decrypt-failed = ይህን መልዕክት መፍታት አይቻልም፦ { $reason }
security-unknown-signer = ያልታወቀ ፈራሚ
security-signed-verified = በ{ $signer } የተፈረመ · የተረጋገጠ
security-signed-not-sender = ላኪው ባልሆነው በ{ $signer } የተፈረመ
security-signed-untrusted = እምነት የማይጣልበት ብለው ምልክት ባደረጉበት ቁልፍ በ{ $signer } የተፈረመ
security-signed-unverified = በ{ $signer } የተፈረመ · ቁልፉ አልተረጋገጠም
security-bad-signature = መጥፎ ፊርማ፦ ይህ መልዕክት ከተፈረመ በኋላ ተቀይሯል፣ ወይም ፊርማው የተጭበረበረ ነው።
security-signature-expired = በ{ $signer } የተፈረመ · የፊርማው ጊዜ አልፏል
security-key-expired = በ{ $signer } የተፈረመ · የቁልፉ ጊዜ ከዚያ ወዲህ አልፏል
security-key-revoked = በተሻረ ቁልፍ በ{ $signer } የተፈረመ
security-missing-key = በሌለዎት ቁልፍ የተፈረመ ስለሆነ ሊረጋገጥ አይችልም
security-missing-key-id = በሌለዎት ቁልፍ ({ $key }) የተፈረመ ስለሆነ ሊረጋገጥ አይችልም
security-signature-unavailable = የተፈረመ፤ ፊርማውን ለማረጋገጥ { $tool }ን ይጫኑ
security-signature-error = ፊርማው ሊረጋገጥ አልቻለም።

## Remote images and pictures

remote-hidden = በዚህ መልዕክት ውስጥ ያሉ ምስሎች ተደብቀዋል።
remote-show = ምስሎችን አሳይ
remote-always-show = ከዚህ ላኪ ሁልጊዜ አሳይ
remote-picture-use = ተጠቀም
remote-picture-too-big = 8 MB ወይም ከዚያ ያነሰ ሥዕል ይምረጡ።
remote-picture-type = የPNG፣ JPEG፣ GIF፣ WebP ወይም SVG ሥዕል ይምረጡ።
remote-picture-read-failed = ሥዕሉን ማንበብ አይቻልም፦ { $error }
remote-picture-keep-failed = ሥዕሉን ማስቀመጥ አይቻልም፦ { $error }
remote-picture-remove-failed = ሥዕሉን ማስወገድ አይቻልም፦ { $error }

## Attachments

attachment-count = { $count ->
    [one] አንድ አባሪ
   *[other] { $count } አባሪዎች
}
attachment-save = አስቀምጥ
attachment-save-all = ሁሉንም አስቀምጥ
attachment-save-all-tooltip = እያንዳንዱን አባሪ ወደ አቃፊ አስቀምጥ
attachment-save-here = እዚህ አስቀምጥ
attachment-not-downloaded = ይህ መልዕክት አልወረደም።
attachment-not-found = ይህ አባሪ በመልዕክቱ ውስጥ ሊገኝ አልቻለም።
attachment-read-failed = { $name }ን ማንበብ አልተቻለም
attachment-numbered = አባሪ { $number }
attachment-saved-all = { $count ->
    [one] { $count } ፋይል ወደ { $place } ተቀምጧል
   *[other] { $count } ፋይሎች ወደ { $place } ተቀምጠዋል
}
attachment-saved-some = { $total ->
    [one] ከ{ $total } ፋይል { $saved } ወደ { $place } ተቀምጧል። { $failed }ን ማስቀመጥ አልተቻለም
   *[other] ከ{ $total } ፋይሎች { $saved } ወደ { $place } ተቀምጠዋል። { $failed }ን ማስቀመጥ አልተቻለም
}
attachment-saved-to = ወደ { $path } ተቀምጧል
attachment-save-failed = { $name }ን ማስቀመጥ አልተቻለም፦ { $error }
attachment-open-failed = { $name }ን መክፈት አልተቻለም፦ { $error }
attachment-risky = ይህ ፋይል ፕሮግራም ሊያሄድ ስለሚችል Katna አይከፍተውም። በምትኩ ያስቀምጡት።
attachment-encrypted-open = ይህ ፋይል ተመስጥሮ ነው የመጣው። ሌላ ቦታ ለመክፈት ያስቀምጡት።

## Printing

print-failed = ማተም አልተቻለም፦ { $error }
print-no-font = ምንም ቅርጸ-ቁምፊ አልተገኘም
print-opened-as-pdf = ከዚያ ለማተም እንደ PDF ተከፍቷል።
print-not-downloaded = (ገና አልወረደም።)
print-encrypted = (የተመሰጠረ። ጽሑፉን ለማተም በKatna Mail ውስጥ ይክፈቱት።)
print-to = ለ፦ { $addresses }
print-cc = ግልባጭ፦ { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = አባሪዎቹን ለማንበብ ይህን መልዕክት ይክፈቱ።
text-copy = ቅዳ
text-select-all = ሁሉንም ምረጥ

## Settings page: its tabs

settings-tab-general = አጠቃላይ
settings-tab-inbox = ገቢ መልዕክት ሳጥን
settings-tab-accounts = መለያዎች
settings-tab-subscriptions = ምዝገባዎች
settings-tab-appearance = መልክ
settings-tab-shortcuts = አቋራጮች
settings-tab-default-apps = ነባሪ መተግበሪያዎች
settings-tab-folders-rules = አቃፊዎች እና ደንቦች
settings-tab-compose = መጻፍ
settings-tab-mcp-server = MCP አገልጋይ
settings-tab-feedback = የተጠቃሚ ግብረመልስ
settings-tab-experimental = የሙከራ

## Settings page: tabs still to come

settings-tab-subscriptions-coming = የሚደርሱዎትን ጋዜጣዎች እና የደብዳቤ ዝርዝሮች ይመልከቱ፣ እና በአንድ ጠቅታ ከደንበኝነት ይውጡ።
settings-tab-folders-rules-coming = አቃፊዎችን እና መሰየሚያዎችን ይፍጠሩ፣ እንደገና ይሰይሙ፣ ያንቀሳቅሱ እና ይደብቁ፣ እና የትኞቹ እንደሚሰምሩ ይምረጡ። ደንቦች አዲስ ደብዳቤን በራሳቸው በላኪ፣ በርዕሰ ጉዳይ ወይም በቃላት ይለያሉ፣ ይሰይማሉ፣ ያስተላልፋሉ ወይም ይሰርዛሉ።
settings-tab-mcp-server-coming = በዚህ ኮምፒውተር ላይ ያሉ የAI ረዳቶች ደብዳቤዎን እንዲፈልጉ፣ እንዲያነቡ እና ረቂቅ እንዲጽፉ ይፍቀዱ፣ በእርስዎ ፈቃድ።

## Settings > General

settings-general-conversations = የውይይት እይታ
settings-general-conversations-group = ለአንድ ደብዳቤ የተሰጡ ምላሾችን በአንድ ላይ ሰብስብ
settings-general-conversations-group-detail = በዝርዝሩ ውስጥ ለእያንዳንዱ ውይይት አንድ መስመር
settings-general-reading = ንባብ
settings-general-newest-first = አዲሱ መልዕክት መጀመሪያ
settings-general-newest-first-detail = ውይይት በመጨረሻው ምላሹ ይጀምራል
settings-general-full-headers = ሙሉ ራስጌዎችን አሳይ
settings-general-full-headers-detail = ከ፣ ለ፣ ግልባጭ፣ ቀን እና ርዕሰ ጉዳይ በእያንዳንዱ መልዕክት ላይ ክፍት ሆነው ይታያሉ
settings-general-full-names = የተቀባዮች ሙሉ ስሞች
settings-general-full-names-detail = «ለእኔ፣ Ada Lovelace» እንጂ «ለእኔ፣ Ada» አይደለም
settings-general-mark-read = እንደተነበበ ምልክት አድርግ
settings-general-mark-read-now = ልክ እንደተከፈተ
settings-general-mark-read-1s = ለ1 ሰከንድ ከተከፈተ በኋላ
settings-general-mark-read-3s = ለ3 ሰከንዶች ከተከፈተ በኋላ
settings-general-mark-read-never = እኔ እንደተነበበ ምልክት ሳደርግበት ብቻ
settings-general-reply-button = የምላሽ አዝራር
settings-general-reply-all = ለሁሉም መልስ
settings-general-reply-all-detail = ከእያንዳንዱ መልዕክት አጠገብ ያለው የምላሽ አዝራር ለላኪው ብቻ ሳይሆን ለሁሉም ይመልሳል
settings-general-remote-images = ከድር የሚመጡ ምስሎች
settings-general-remote-images-detail = የመልዕክትን ምስሎች መጫን እንደከፈቱት፣ መቼ እና በግምት የት እንዳሉ ለላኪው ይነግረዋል። ሲጠፋ እያንዳንዱ መልዕክት መጀመሪያ ይጠይቃል፣ እና የአንድን ላኪ ምስሎች ሁልጊዜ ማሳየት ይችላሉ።
settings-general-remote-images-always = ምስሎችን ሁልጊዜ አሳይ
settings-general-remote-images-always-detail = ከሚያምኗቸው ላኪዎች ብቻ ሳይሆን በእያንዳንዱ መልዕክት ውስጥ
settings-general-sending = መላክ
settings-general-sending-detail = የተላከ መልዕክት እንዲመለስ ምን ያህል ጊዜ እንደሚጠብቅ።
settings-general-offline = ከመስመር ውጭ ደብዳቤ
settings-general-offline-detail = የቅርብ ጊዜ ደብዳቤ ያለ ግንኙነት እንዲነበብ ሙሉ በሙሉ ይወርዳል። የቆየ ደብዳቤ ሲከፍቱት ይወርዳል።
settings-general-offline-days = { $count ->
    [one] { $count } ቀን
   *[other] { $count } ቀናት
}
settings-general-offline-years = { $count ->
    [one] { $count } ዓመት
   *[other] { $count } ዓመታት
}
settings-general-offline-all = ሁሉም ደብዳቤ
settings-general-offline-note = ያነሱ ቀናትን መምረጥ አስቀድሞ የወረደውን ደብዳቤ ያቆያል። በአገልጋዩ ላይ ምንም አይቀየርም።
settings-general-notifications = ማሳወቂያዎች
settings-general-notifications-detail = በገቢ መልዕክት ሳጥን ውስጥ ላለ አዲስ ደብዳቤ፣ Katna Mail ተዘግቶ ሳለም እንኳ።
settings-general-new-mail = ስለ አዲስ ደብዳቤ አሳውቀኝ
settings-general-new-mail-detail = ከ«ለሁሉም መልስ»፣ «እንደተነበበ ምልክት አድርግ» እና «ወደ ማህደር አስቀምጥ» ጋር
settings-general-new-mail-sound = ድምፅ አጫውት
settings-general-new-mail-sound-detail = የዴስክቶፑ የአዲስ ደብዳቤ ድምፅ
settings-general-desktop = ዴስክቶፕ
settings-general-open-at-login = ሲገቡ Katna Mailን ክፈት
settings-general-open-at-login-detail = አገልግሎቱ እስከሚሠራ ድረስ፣ ደብዳቤ ሲገቡ በማንኛውም ሁኔታ ይሰምራል
settings-general-tray = Katnaን በሥርዓት ትሪ ውስጥ አሳይ
settings-general-tray-detail = ካልተነበቡ መልዕክቶች ብዛት እና ከምናሌ ጋር
settings-general-unread-badge = በተግባር አሞሌ አዶ ላይ ያልተነበቡ መልዕክቶች ብዛት
settings-general-unread-badge-detail = በገቢ መልዕክት ሳጥን ውስጥ ስንት መልዕክቶች እንዳልተነበቡ

## Settings > Inbox

settings-inbox-tabs = የገቢ መልዕክት ሳጥን ትሮች
settings-inbox-tabs-detail = የደብዳቤ አቅራቢዎ ድር ጣቢያ እንደሚያደርገው፣ ገቢ መልዕክት ሳጥኑን ወደ ትሮች ይለዩ።
settings-inbox-tabs-show = የገቢ መልዕክት ሳጥን ትሮችን አሳይ
settings-inbox-tabs-show-detail = ሲጠፋ ለእያንዳንዱ መለያ አንድ ዝርዝር ያሳያል
settings-inbox-no-accounts = ትሮቹን ለመምረጥ መለያ ያክሉ።
settings-inbox-tabs-automatic = ራስ-ሰር፦ { $tabs } ({ $provider })
settings-inbox-tabs-off = ምንም ትር የለም
settings-inbox-tabs-gmail = ዋና፣ ማስተዋወቂያዎች፣ ማህበራዊ፣ ዝማኔዎች፣ መድረኮች
settings-inbox-tabs-focused = ትኩረት የተሰጣቸው እና ሌሎች
settings-inbox-tabs-zoho = ገቢ መልዕክት ሳጥን፣ ጋዜጣዎች እና ማሳወቂያዎች
settings-inbox-tabs-shown = የሚታዩ ትሮች። ያጠፉት ትር ደብዳቤ በ{ $tab } ውስጥ ይቆያል።

## Settings > Appearance

settings-appearance-reading-pane = የንባብ ክፍል
settings-appearance-reading-pane-detail = የተከፈተ ውይይት የሚታይበት ቦታ።
settings-appearance-pane-right = ከዝርዝሩ በስተቀኝ
settings-appearance-pane-none = ክፍፍል የለም
settings-appearance-density = ጥግግት
settings-appearance-density-default = ነባሪ
settings-appearance-density-compact = የታመቀ
settings-appearance-scaling = መጠን
settings-appearance-scaling-detail = በዴስክቶፑ የራሱ መጠን ላይ ተጨምሮ በKatna Mail ውስጥ ያለውን ሁሉ ትልቅ ወይም ትንሽ ያደርጋል፦ ጽሑፍ፣ አዶዎች፣ ክፍተት እና መከፋፈያዎች። የሚልኩት ደብዳቤ የራሱን የቅርጸ-ቁምፊ መጠን ይይዛል። በጣም ትንሽ መጠኖች አዶዎችን ጠቅ ለማድረግ አስቸጋሪ ሊያደርጉ ይችላሉ።
settings-appearance-theme = ገጽታ
settings-appearance-theme-system = እንደ ዴስክቶፑ
settings-appearance-theme-light = ፈካ ያለ
settings-appearance-theme-dark = ጠቆር ያለ
settings-appearance-desktop-colors = የዴስክቶፕ ቀለሞች
settings-appearance-desktop-colors-use = የዴስክቶፑን ቀለሞች ተጠቀም
settings-appearance-desktop-colors-use-detail = የዴስክቶፑ የቀለም ገጽታ እና የአጽንዖት ቀለም
settings-appearance-app-names = የመተግበሪያ ስሞች
settings-appearance-app-names-show = የመተግበሪያ ስሞችን አሳይ
settings-appearance-app-names-show-detail = በግራ ጠርዝ ላይ ካሉት የመተግበሪያ አዶዎች በታች ያሉ ስሞች
settings-appearance-sender-pictures = የላኪ ሥዕሎች
settings-appearance-sender-pictures-show = የኩባንያ አርማዎችን አሳይ
settings-appearance-sender-pictures-show-detail = በላኪው ጎራ እንጂ በመልዕክት በጭራሽ አይፈለጉም፣ እና ለአንድ ሳምንት ይቀመጣሉ
settings-appearance-important = የአስፈላጊ ምልክቶች
settings-appearance-important-show = የአስፈላጊ ምልክቶችን አሳይ
settings-appearance-important-show-detail = በዝርዝሩ ውስጥ ከእያንዳንዱ መልዕክት አጠገብ
settings-appearance-message-width = የመልዕክት ስፋት
settings-appearance-message-width-limit = የመልዕክቶችን ስፋት ገድብ
settings-appearance-message-width-limit-detail = ረጅም መስመሮች በሰፊ መስኮት ውስጥ ለማንበብ ቀላል ናቸው
settings-appearance-mail-colors = የደብዳቤ ቀለሞች
settings-appearance-mail-colors-detail = አብዛኛው ደብዳቤ የተዘጋጀው ለነጭ ገጽ ነው። በጠቆር ያለ ገጽታ ቀለሞቹ በደንብ ወደሚነበቡ ጠቆር ያሉ ቀለሞች ይቀየራሉ፤ ሲጠፋ የላኪውን ቀለሞች በፈካ ያለ ገጽ ላይ ይይዛል።
settings-appearance-dark-mail = ለደብዳቤም ጠቆር ያሉ ቀለሞች
settings-appearance-dark-mail-detail = ገጽታው ጠቆር ያለ ሲሆን ብቻ
settings-appearance-attachment-previews = የአባሪ ቅድመ እይታዎች
settings-appearance-attachment-previews-show = የአባሪዎችን ቅድመ እይታ አሳይ
settings-appearance-attachment-previews-show-detail = በካርዱ ላይ የእያንዳንዱ ፋይል ይዘት ትንሽ ሥዕል

## Settings > Default apps

settings-default-apps-intro = አባሪዎችን ጠቅ ሲያደርጉ የሚከፈቱበት ቦታ። ተመልካቹ ሁልጊዜ ፋይልን በሌላ መተግበሪያም መክፈት ይችላል። የዴስክቶፑ ነባሪ መተግበሪያዎች በራሱ ቅንብሮች ውስጥ ይዘጋጃሉ።
settings-default-apps-pdf = የPDF ፋይሎች
settings-default-apps-pdf-detail = ገጾች፣ ከማጉላት ጋር።
settings-default-apps-pictures = ሥዕሎች
settings-default-apps-pictures-detail = ፎቶዎች (ቀጥ ተደርገው)፣ PNG፣ GIF፣ WebP፣ BMP፣ TIFF እና SVG።
settings-default-apps-text = የጽሑፍ ፋይሎች
settings-default-apps-text-detail = ግልጽ ጽሑፍ፣ ምዝግብ ማስታወሻዎች፣ ኮድ እና ሌላ ጽሑፍ።
settings-default-apps-sheets = የተመን ሉሆች
settings-default-apps-sheets-detail = Excel (xlsx፣ xls)፣ OpenDocument (ods) እና CSV።
settings-default-apps-documents = ሰነዶች
settings-default-apps-documents-detail = Word (docx) እና የOpenDocument ጽሑፍ (odt)።
settings-default-apps-katna = የKatna Mail ተመልካች
settings-default-apps-system = የዴስክቶፑ ነባሪ መተግበሪያ
settings-default-apps-ask = በእያንዳንዱ ጊዜ የትኛውን መተግበሪያ እንደሆነ ጠይቅ
settings-default-apps-after-saving = ካስቀመጡ በኋላ
settings-default-apps-show-folder = የተቀመጡ ፋይሎችን በአቃፊያቸው ውስጥ አሳይ
settings-default-apps-show-folder-detail = የፋይል አቀናባሪውን የተቀመጡት አባሪዎች ተመርጠው ይከፍታል

## Settings > Compose

settings-compose-send-from = አዲስ መልዕክቶችን ላክ ከ
settings-compose-send-from-detail = ምላሾች እና ማስተላለፎች ሁልጊዜ ካሉበት መለያ ይወጣሉ።
settings-compose-send-from-current = ያሉበት መለያ
settings-compose-send-on-replies = በምላሾች ላይ መላክ
settings-compose-send-on-replies-detail = ላክ በምላሽ ወይም በማስተላለፍ ላይ የሚያደርገው። ከላክ አጠገብ ያለው ምናሌ ሌላውን ያቀርባል።
settings-compose-send-plain = ላክ
settings-compose-send-archive = ላክ እና ወደ ማህደር አስቀምጥ
settings-compose-signatures = ፊርማዎች
settings-compose-signatures-detail = ከመልዕክትዎ በታች፣ ከ«--» መስመር በኋላ ይታከላል። በመጻፊያ መስኮቱ ውስጥ ሌላ ይምረጡ።
settings-compose-untitled = ርዕስ የሌለው
settings-compose-signature-name = ስም፣ ለምሳሌ ሥራ
settings-compose-signature-first = የእኔ ፊርማ
settings-compose-signature-numbered = ፊርማ { $number }
settings-compose-signature-delete = ሰርዝ
settings-compose-signature-deleted = ፊርማው ተሰርዟል
settings-compose-signature-new = አዲስ ፍጠር
settings-compose-no-signatures = እስካሁን ምንም ፊርማ የለም።
settings-compose-no-signature = ፊርማ የለም
settings-compose-for-new-mail = ለአዲስ ደብዳቤ
settings-compose-for-replies = ለምላሾች እና ለማስተላለፎች
settings-compose-for-replies-detail = መልዕክት በፈረሙበት ውይይት ውስጥ፣ ምላሽ በምትኩ በዚያ ፊርማ ይጀምራል።
settings-compose-format = ቅርጸት
settings-compose-plain-text = በግልጽ ጽሑፍ ጻፍ
settings-compose-plain-text-detail = አዲስ ደብዳቤ ያለ ቅርጸት ይጀምራል፤ የመጻፊያ መስኮቱ መቀየር ይችላል
settings-compose-spelling = ፊደል አጻጻፍ
settings-compose-spell-check = በምጽፍበት ጊዜ ፊደል አጻጻፍን አረጋግጥ
settings-compose-spell-check-detail = በስህተት የተጻፉ ቃላት ከስራቸው ይሰመርባቸዋል፣ በቀኝ ጠቅታ የአስተያየት ጥቆማዎች አሉ
settings-compose-spell-desktop = የዴስክቶፑ ቋንቋ ({ $language })
settings-compose-templates = አብነቶች
settings-compose-templates-detail = ብዙ ጊዜ የሚጽፉትን ደብዳቤ ያስቀምጡ፣ እና አዲስ ደብዳቤ ወይም ምላሽ ከእሱ ይጀምሩ።

## Settings > Shortcuts

settings-shortcuts-set = የአቋራጭ ስብስብ
settings-shortcuts-set-detail = ከሚያውቁት የደብዳቤ መተግበሪያ ቁልፎች ይጀምሩ። እዚህ Cmd ማለት Ctrl ነው። የራስዎ ለውጦች በስብስቡ ላይ ይቆያሉ፣ እና ነባሪዎችን መልስ ወደ ስብስቡ ቁልፎች ይመልሳል።
settings-shortcuts-single = ነጠላ-ቁልፍ አቋራጮች
settings-shortcuts-single-detail = ያለ Ctrl ወይም Alt ቁልፎች፣ እንደ ድር ደብዳቤ፦ e ወደ ማህደር ያስቀምጣል፣ j እና k ያንቀሳቅሳሉ፣ / ይፈልጋል። በዝርዝሩ እና በተከፈተው ውይይት ውስጥ ይሠራሉ፣ በሚተይቡበት ጊዜ ግን በጭራሽ።
settings-shortcuts-single-use = ነጠላ-ቁልፍ አቋራጮችን ተጠቀም
settings-shortcuts-single-use-detail = የCtrl አቋራጮች ሁልጊዜ ይሠራሉ
settings-shortcuts-how = ለመቀየር ቁልፍን ጠቅ ያድርጉ፣ ወይም አንድ ለማከል +ን፣ ከዚያ አዲሶቹን ቁልፎች ይጫኑ። Esc ይሰርዛል።
settings-shortcuts-restore = ነባሪዎችን መልስ
settings-shortcuts-no-key = ቁልፍ የለም
settings-shortcuts-press = ቁልፎችን ይጫኑ…
settings-shortcuts-then = { $keys } ከዚያ…
settings-shortcuts-moved = { $keys } አሁን ከ«{ $previous }» ይልቅ «{ $action }»ን ያደርጋል።
settings-shortcuts-single-off = ነጠላ-ቁልፍ አቋራጮች ጠፍተዋል፣ ስለዚህ ይህ ቁልፍ ሲበሩ ይሠራል።
settings-shortcuts-restored = እያንዳንዱ አቋራጭ የስብስቡን ቁልፎች እንደገና አግኝቷል።

## Settings search: the line under a result

settings-general-language-summary = የመተግበሪያው፣ የቀኖች እና የቁጥሮች ቋንቋ
settings-general-reading-summary = አዲሱ መልዕክት መጀመሪያ፣ ሙሉ ራስጌዎች፣ የተቀባዮች ሙሉ ስሞች
settings-general-mark-read-summary = የተከፈተ ውይይት እንደተነበበ ምልክት የሚደረግበት ጊዜ፦ ወዲያውኑ፣ ከ1 ወይም 3 ሰከንዶች በኋላ፣ ወይም በእጅ
settings-general-reply-button-summary = ከእያንዳንዱ መልዕክት አጠገብ ያለው የምላሽ አዝራር ለሁሉም ይመልሳል
settings-general-remote-images-summary = የእያንዳንዱን መልዕክት ምስሎች ሁልጊዜ አሳይ
settings-general-sending-summary = መላክን ቀልብስ፦ የተላከ መልዕክት እንዲመለስ ምን ያህል ጊዜ እንደሚጠብቅ
settings-general-offline-summary = ያለ ግንኙነት እንዲነበብ የስንት ቀናት የቅርብ ጊዜ ደብዳቤ ሙሉ በሙሉ እንደሚወርድ
settings-general-notifications-summary = የአዲስ ደብዳቤ ማሳወቂያዎች እና ድምፃቸው
settings-general-desktop-summary = ሲገቡ Katna Mailን መክፈት፣ የሥርዓት ትሪ አዶ እና በተግባር አሞሌ አዶ ላይ ያልተነበቡ መልዕክቶች ብዛት
settings-accounts-accounts-summary = መለያ ያክሉ ወይም ያስወግዱ፣ ወይም ሥዕሉን ይቀይሩ
settings-appearance-density-summary = በዝርዝሩ ውስጥ ነባሪ ወይም የታመቁ መስመሮች
settings-appearance-scaling-summary = ሁሉንም ነገር ትልቅ ወይም ትንሽ ያድርጉ፦ ጽሑፍ፣ አዶዎች፣ ክፍተት እና መከፋፈያዎች
settings-appearance-theme-summary = እንደ ዴስክቶፑ፣ ፈካ ያለ ወይም ጠቆር ያለ
settings-appearance-sender-pictures-summary = በላኪው ጎራ የሚፈለጉ የኩባንያ አርማዎች
settings-appearance-important-summary = በዝርዝሩ ውስጥ ከእያንዳንዱ መልዕክት አጠገብ ያለው የአስፈላጊ ምልክት
settings-appearance-mail-colors-summary = በጠቆር ያለ ገጽታ ለHTML ደብዳቤ ጠቆር ያሉ ቀለሞች፣ ወይም የላኪው ቀለሞች
settings-appearance-attachment-previews-summary = የእያንዳንዱ አባሪ ይዘት ትንሽ ሥዕል
settings-shortcuts-set-summary = ከGmail፣ Inbox by Gmail፣ Apple Mail፣ Outlook ወይም Thunderbird ቁልፎች ይጀምሩ
settings-shortcuts-single-summary = ያለ Ctrl ወይም Alt ቁልፎች፣ እንደ ድር ደብዳቤ
settings-default-apps-pdf-summary = የPDF አባሪዎች የሚከፈቱበት ቦታ
settings-default-apps-pictures-summary = ፎቶዎች እና ሥዕሎች የሚከፈቱበት ቦታ
settings-default-apps-text-summary = ግልጽ ጽሑፍ፣ ምዝግብ ማስታወሻዎች እና ኮድ የሚከፈቱበት ቦታ
settings-default-apps-sheets-summary = የExcel፣ OpenDocument እና CSV ፋይሎች የሚከፈቱበት ቦታ
settings-default-apps-documents-summary = Word እና የOpenDocument ጽሑፍ የሚከፈቱበት ቦታ
settings-default-apps-after-saving-summary = የተቀመጡ አባሪዎችን በአቃፊያቸው ውስጥ አሳይ
settings-compose-send-from-summary = አዲስ ደብዳቤ የሚወጣበት መለያ፦ ያሉበት፣ ወይም ሁልጊዜ ያው
settings-compose-send-on-replies-summary = በምላሾች እና በማስተላለፎች ላይ ላክ፣ ወይም ላክ እና ውይይቱን ወደ ማህደር አስቀምጥ
settings-compose-signatures-summary = ከመልዕክትዎ በታች፣ ከ«--» መስመር በኋላ ይታከላል
settings-compose-for-new-mail-summary = አዲስ ደብዳቤ የሚጀምርበት ፊርማ
settings-compose-for-replies-summary = ምላሾች እና ማስተላለፎች የሚጀምሩበት ፊርማ
settings-compose-format-summary = አዲስ ደብዳቤን በግልጽ ጽሑፍ ጻፍ
settings-compose-spelling-summary = በሚጽፉበት ጊዜ ፊደል አጻጻፍን ማረጋገጥ፣ እና የመዝገበ ቃላቱ ቋንቋ
settings-compose-templates-summary = በቅርቡ ይመጣል፦ ብዙ ጊዜ የሚጽፉትን ደብዳቤ ያስቀምጡ፣ እና አዲስ ደብዳቤ ወይም ምላሽ ከእሱ ይጀምሩ
settings-feedback-crash-reports-summary = Katna Mail ወይም የጀርባ አገልግሎቱ ሲበላሽ የብልሽት ሪፖርቶችን በዚህ ኮምፒውተር ላይ አስቀምጥ
settings-feedback-saved-summary = በዚህ ኮምፒውተር ላይ የተቀመጡ የብልሽት ሪፖርቶችን ይመልከቱ፣ ይቅዱ ወይም ይሰርዙ
settings-feedback-help-improve-summary = የተበላሸውን ለማስተካከል እንዲያግዙ የብልሽት ሪፖርቶችን ላክ፤ ካላበሩት በስተቀር ጠፍቷል
settings-experimental-blur-summary = ዴስክቶፑ በላይኛው አሞሌ በኩል ደብዝዞ ይታያል፣ ምናሌዎችም እንደ ጭጋጋማ መስታወት ይሆናሉ
settings-search-shortcut = የቁልፍ ሰሌዳ አቋራጭ
settings-search-tab = የቅንብሮች ትር
settings-search-none = ከ«{ $query }» ጋር የሚዛመድ ቅንብር የለም።
settings-search-results = ከ«{ $query }» ጋር የሚዛመዱ ቅንብሮች
## Quick settings (the panel that slides in from the right)

quick-title = ፈጣን ቅንብሮች
quick-see-all = ሁሉንም ቅንብሮች ይመልከቱ
quick-reading-pane = የንባብ ክፍል
quick-pane-right = ከዝርዝሩ በስተቀኝ
quick-pane-none = ክፍፍል የለም
quick-density = ጥግግት
quick-density-default = ነባሪ
quick-density-compact = የታመቀ
quick-theme = ገጽታ
quick-theme-system = እንደ ዴስክቶፑ
quick-theme-light = ፈካ ያለ
quick-theme-dark = ጠቆር ያለ
quick-desktop-colors = የዴስክቶፕ ቀለሞች
quick-desktop-colors-detail = የዴስክቶፑ የቀለም ገጽታ እና የአጽንዖት ቀለም
quick-app-names = የመተግበሪያ ስሞች
quick-app-names-detail = በግራ ጠርዝ ላይ ካሉት የመተግበሪያ አዶዎች በታች ያሉ ስሞች
quick-inbox-tabs = የገቢ መልዕክት ሳጥን ትሮች
quick-inbox-tabs-detail = የእያንዳንዱ መለያ የደብዳቤ አቅራቢ ትሮች
quick-choose-tabs = ትሮችን ምረጥ
quick-choose-tabs-detail = ለእያንዳንዱ መለያ፣ በቅንብሮች ውስጥ
quick-sending = መላክ
quick-undo-send = መላክን ቀልብስ
quick-undo-send-off = ጠፍቷል
quick-undo-send-seconds = { $seconds } ሰ
quick-signatures = ፊርማዎች
quick-signatures-none = እስካሁን የለም
quick-signatures-one = { $name }፣ በነባሪ ጥቅም ላይ የሚውል
quick-signatures-many = { $count ->
    [one] { $count } ፊርማ፤ { $name } ነባሪ ነው
   *[other] { $count } ፊርማዎች፤ { $name } ነባሪ ነው
}
quick-signatures-no-default = { $count ->
    [one] { $count }፣ ምንም ነባሪ የለም
   *[other] { $count }፣ ምንም ነባሪ የለም
}
quick-signature-untitled = ርዕስ የሌለው
quick-threading = የኢሜይል ውይይት ስብስብ
quick-conversation-view = የውይይት እይታ
quick-conversation-view-detail = ለአንድ ደብዳቤ የተሰጡ ምላሾችን በአንድ ላይ ሰብስብ
quick-help = እገዛ
quick-tour = ጉብኝቱን ጀምር
quick-whats-new = ምን አዲስ ነገር አለ
quick-about = ስለ Katna

## Settings: opening at login

settings-open-at-login-failed = ሲገቡ መክፈትን መቀየር አልተቻለም፦ { $error }

## Settings > Appearance > Scaling

scale-letter = አ
scale-percent = { $percent }%
scale-reset = ወደ { $percent }% መልስ

## Settings > Experimental > Look & Feel

look-intro = አሁንም በሙከራ ላይ ያሉ ባህሪያት። ሊቀየሩ ወይም ሊወገዱ ይችላሉ።
look-heading = መልክ እና ስሜት
look-window-frame = የመስኮት ፍሬም
look-window-frame-detail = የርዕስ አሞሌውን፣ የመስኮት አዝራሮቹን፣ ማዕዘኖቹን እና ጥላውን የሚስለው።
look-frame-native-kde = የሥርዓቱ፦ የKDE ፍሬም፣ በPlasma ገጽታዎ
look-frame-native = የሥርዓቱ፦ የዴስክቶፑ ፍሬም
look-frame-katna = Katna፦ የላይኛው አሞሌ የርዕስ አሞሌ ይሆናል
look-frame-katna-note-named = Katna ክብ ማዕዘኖችን እና የራሱን ጥላ ይስላል። ፍሬሙ ከእንግዲህ የ{ $desktop } ገጽታን አይከተልም፤ የመስኮት ደንቦች አሁንም ይሠራሉ።
look-frame-katna-note = Katna ክብ ማዕዘኖችን እና የራሱን ጥላ ይስላል። ፍሬሙ ከእንግዲህ የዴስክቶፑን ገጽታ አይከተልም፤ የመስኮት ደንቦች አሁንም ይሠራሉ።
look-frame-client-side = ዴስክቶፕዎ ፍሬሙን ለእያንዳንዱ መተግበሪያ ይተዋል፣ ስለዚህ Katna አስቀድሞ የራሱን ይስላል።
look-blurred-background = የደበዘዘ ዳራ
look-blurred-background-detail = ዴስክቶፑ በላይኛው አሞሌ እና በአቃፊዎቹ በኩል ደብዝዞ ይታያል፣ ምናሌዎች እና ብቅ-ባዮችም እንደ ጭጋጋማ መስታወት ይሆናሉ።
look-blur = ከመስኮቱ ጀርባ ያለውን አደብዝዝ
look-blur-detail = ደብዳቤ በጠንካራ ካርዶች ላይ ይቆያል፣ ስለዚህ ጽሑፍ ንፅፅሩን ይይዛል
look-blur-off-kde = የKDE Blur ውጤት ጠፍቷል። በSystem Settings፣ Window Management፣ Desktop Effects ውስጥ Blurን ያብሩ፣ ከዚያ Katna Mailን እንደገና ይክፈቱ።
look-blur-none-gnome = GNOME ከመስኮቶች ጀርባ ያለውን አያደበዝዝም።
look-blur-none-x11 = የመስኮት አቀናባሪዎ ከመስኮቶች ጀርባ ያለውን አያደበዝዝም።
look-blur-none-wayland = ኮምፖዚተርዎ ከመስኮቶች ጀርባ ያለውን አያደበዝዝም።

## Settings > User feedback (crash reports)

feedback-intro-sending = የተበላሸውን ለማስተካከል እንዲያግዙ አዲስ የብልሽት ሪፖርቶች ይላካሉ። ሌላ ምንም ነገር ከዚህ ኮምፒውተር አይወጣም።
feedback-intro-local = Katna ምንም ነገር ወደ የትም አይልክም። የብልሽት ሪፖርቶች እንዲመለከቷቸው ወይም ከሳንካ ሪፖርት ጋር እንዲያያይዟቸው በዚህ ኮምፒውተር ላይ ይቆያሉ።
feedback-crash-reports = የብልሽት ሪፖርቶች
feedback-crash-reports-detail = Katna Mail ወይም የጀርባ አገልግሎቱ ሲበላሽ ይጻፋሉ።
feedback-save = የብልሽት ሪፖርቶችን በዚህ ኮምፒውተር ላይ አስቀምጥ
feedback-save-detail = የቤት አቃፊዎ፣ የተጠቃሚ እና የኮምፒውተር ስሞች እና የኢሜይል አድራሻዎች አይካተቱም
feedback-saved = የተቀመጡ የብልሽት ሪፖርቶች
feedback-saved-detail = { $count ->
    [one] አዲሱ { $count } ይቀመጣል።
   *[other] አዲሶቹ { $count } ይቀመጣሉ።
}
feedback-help-improve = Katnaን ለማሻሻል ያግዙ
feedback-help-improve-detail = ካላበሩት በስተቀር ጠፍቷል፣ እና በማንኛውም ጊዜ እዚህ ማጥፋት ይችላሉ።
feedback-send = የብልሽት ሪፖርቶችን ላክ
feedback-send-detail = የተቀመጠው ሪፖርት፣ እዚህ ማየት እንደሚችሉት በትክክል፣ ወደ Katna የብልሽት መከታተያ (Sentry፣ በEU) ይሄዳል። የIP አድራሻ፣ መልዕክቶች ወይም የኢሜይል አድራሻዎች የሉም
feedback-none-saved = ምንም የተቀመጠ የብልሽት ሪፖርት የለም።
feedback-delete-all = ሁሉንም ሰርዝ
feedback-app-daemon = የጀርባ አገልግሎት
feedback-report-sent = { $date } · ተልኳል
feedback-view = ተመልከት
feedback-view-tooltip = ሪፖርቱን ክፈት
feedback-copy-tooltip = በሳንካ ሪፖርት ውስጥ ለመለጠፍ ቅዳው
feedback-copied = የብልሽት ሪፖርቱ ተቀድቷል።
feedback-deleted-all = የብልሽት ሪፖርቶቹ ተሰርዘዋል።
feedback-read-failed = የብልሽት ሪፖርቱን ማንበብ አልተቻለም፦ { $error }
feedback-delete-failed = የብልሽት ሪፖርቱን መሰረዝ አልተቻለም፦ { $error }
feedback-delete-all-failed = የብልሽት ሪፖርቶቹን መሰረዝ አልተቻለም፦ { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _ፋይል
desktop-menu-new-message = _አዲስ መልዕክት
desktop-menu-quit = _ውጣ
desktop-menu-edit = _አርትዕ
desktop-menu-undo = _ቀልብስ
desktop-menu-select-all = _ሁሉንም ምረጥ
desktop-menu-select-none = _ምንም አትምረጥ
desktop-menu-find = _ፈልግ…
desktop-menu-view = _እይታ
desktop-menu-folder-list = _የአቃፊ ዝርዝርን አሳይ
desktop-menu-refresh = _አድስ
desktop-menu-go = _ሂድ
desktop-menu-inbox = _ገቢ መልዕክት ሳጥን
desktop-menu-starred = _ኮከብ የተደረገባቸው
desktop-menu-sent = _የተላኩ
desktop-menu-drafts = _ረቂቆች
desktop-menu-all-mail = _ሁሉም ደብዳቤ
desktop-menu-next = _ቀጣይ ውይይት
desktop-menu-previous = _ቀዳሚ ውይይት
desktop-menu-message = _መልዕክት
desktop-menu-open = _ክፈት
desktop-menu-reply = _መልስ
desktop-menu-reply-all = _ለሁሉም መልስ
desktop-menu-forward = _አስተላልፍ
desktop-menu-archive = _ወደ ማህደር አስቀምጥ
desktop-menu-delete = _ሰርዝ
desktop-menu-spam = _አይፈለጌ መልዕክት ሪፖርት አድርግ
desktop-menu-move-to = _ውሰድ ወደ…
desktop-menu-mark-read = _እንደተነበበ ምልክት አድርግ
desktop-menu-mark-unread = _እንዳልተነበበ ምልክት አድርግ
desktop-menu-star = _ኮከብ አክል
desktop-menu-important = _እንደ አስፈላጊ ምልክት አድርግ
desktop-menu-not-important = _አስፈላጊ እንዳልሆነ ምልክት አድርግ
desktop-menu-settings = _ቅንብሮች
desktop-menu-quick-settings = _ፈጣን ቅንብሮች
desktop-menu-configure = _Katna Mailን አዋቅር…
desktop-menu-help = _እገዛ
desktop-menu-shortcuts = _የቁልፍ ሰሌዳ አቋራጮች
desktop-menu-whats-new = _ምን አዲስ ነገር አለ
desktop-menu-about = _ስለ Katna
## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = መንቀሳቀስ
shortcut-group-actions = እርምጃዎች
shortcut-group-go-to = ሂድ ወደ
shortcut-group-app = መተግበሪያ

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = ቀጣይ ውይይት
shortcut-previous = ቀዳሚ ውይይት
shortcut-down = በዝርዝሩ ወደ ታች ውረድ
shortcut-up = በዝርዝሩ ወደ ላይ ውጣ
shortcut-first = በዝርዝሩ ውስጥ የመጀመሪያው
shortcut-last = በዝርዝሩ ውስጥ የመጨረሻው
shortcut-page-down = በዝርዝሩ አንድ ገጽ ወደ ታች
shortcut-page-up = በዝርዝሩ አንድ ገጽ ወደ ላይ
shortcut-open = ውይይት ክፈት
shortcut-back = ወደ ዝርዝሩ ተመለስ
shortcut-scroll-down = ወደ ታች ሸብልል
shortcut-scroll-up = ወደ ላይ ሸብልል
shortcut-scroll-page-down = አንድ ገጽ ወደ ታች ሸብልል
shortcut-scroll-page-up = አንድ ገጽ ወደ ላይ ሸብልል
shortcut-compose = ጻፍ
shortcut-reply = መልስ
shortcut-reply-all = ለሁሉም መልስ
shortcut-forward = አስተላልፍ
shortcut-archive = ወደ ማህደር አስቀምጥ
shortcut-delete = ሰርዝ
shortcut-spam = አይፈለጌ መልዕክት ሪፖርት አድርግ
shortcut-move-to = ውሰድ ወደ
shortcut-mark-read = እንደተነበበ ምልክት አድርግ
shortcut-mark-unread = እንዳልተነበበ ምልክት አድርግ
shortcut-star = ኮከብ አክል ወይም አስወግድ
shortcut-important = እንደ አስፈላጊ ምልክት አድርግ
shortcut-not-important = አስፈላጊ እንዳልሆነ ምልክት አድርግ
shortcut-check = ውይይቱን ምልክት አድርግ
shortcut-select-all = ሁሉንም ውይይቶች ምልክት አድርግ
shortcut-select-none = ከሁሉም ውይይቶች ምልክት አንሳ
shortcut-undo = የመጨረሻውን እርምጃ ቀልብስ
shortcut-go-inbox = ገቢ መልዕክት ሳጥን
shortcut-go-starred = ኮከብ የተደረገባቸው
shortcut-go-sent = የተላኩ
shortcut-go-drafts = ረቂቆች
shortcut-go-all = ሁሉም ደብዳቤ
shortcut-search = ደብዳቤ ፈልግ
shortcut-navigation = ምናሌውን አሳይ ወይም አጣጥፍ
shortcut-quick-settings = ፈጣን ቅንብሮች
shortcut-settings = ሁሉም ቅንብሮች
shortcut-shortcuts = የቁልፍ ሰሌዳ አቋራጮች
shortcut-reload = አዲስ ደብዳቤ ፈትሽ
shortcut-quit = ውጣ

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } ከዚያ { $second }

## Settings > Accounts

accounts-folder-pane = የአቃፊ ክፍል
accounts-folder-pane-detail = በግራ በኩል ያለው ክፍል የየትኞቹን መለያዎች አቃፊዎች እንደሚያሳይ።
accounts-shown-one = በአንድ ጊዜ አንድ መለያ፤ በመለያ ካርዱ ውስጥ ይቀያይሩ
accounts-shown-all = ሁሉም መለያዎች፣ አንዱ ከሌላው በኋላ
accounts-row = መለያዎች
accounts-row-detail = መለያን ማስወገድ በዚህ ኮምፒውተር ላይ ያለውን የKatna የደብዳቤው ቅጂ ይሰርዛል። ደብዳቤው በአገልጋዩ ላይ ይቆያል።
accounts-none = እስካሁን ምንም መለያ የለም።
accounts-kind-imported = የመጣ
accounts-picture-reset = የዴስክቶፕ ሥዕልን ተጠቀም
accounts-picture-change = ሥዕል ቀይር
accounts-remove = አስወግድ
accounts-delete-all-row = ሁሉንም ውሂብ ሰርዝ
accounts-delete-all-row-detail = እንደ አዲስ ጭነት፣ እንደገና ይጀምሩ።
accounts-delete-all-about = እያንዳንዱን መለያ፣ ሁሉንም የተከማቸ ደብዳቤ፣ እውቂያዎችን እና ቀን መቁጠሪያዎችን፣ የፍለጋ ማውጫውን፣ ቅንብሮችዎን እና የተቀመጡ የይለፍ ቃላትን ከዚህ ኮምፒውተር ይሰርዛል። በደብዳቤ አገልጋዮችዎ ላይ ምንም አይቀየርም።
accounts-delete-all-open = ሁሉንም የKatna ውሂብ ሰርዝ

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } ከKatna ተወግዷል።
accounts-removed = { $address } ከKatna ተወግዷል። ደብዳቤው አሁንም በአገልጋዩ ላይ አለ።
accounts-all-deleted = ሁሉም የKatna ውሂብ ከዚህ ኮምፒውተር ተሰርዟል።

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = { $address } ይወገድ?
accounts-remove-confirm = መለያን አስወግድ
accounts-removing = በማስወገድ ላይ…
accounts-remove-local-mail = { $folders ->
    [0] ወደዚህ መለያ የመጣ ሁሉም ደብዳቤ
    [one] በአቃፊው ውስጥ ወደዚህ መለያ የመጣ ሁሉም ደብዳቤ
   *[other] በ{ $folders } አቃፊዎቹ ውስጥ ወደዚህ መለያ የመጣ ሁሉም ደብዳቤ
}
accounts-remove-local-settings = የKatna ቅንብሮቹ
accounts-remove-mail = { $folders ->
    [0] በKatna የተከማቸ የዚህ መለያ ሁሉም ደብዳቤ
    [one] በአቃፊው ውስጥ በKatna የተከማቸ የዚህ መለያ ሁሉም ደብዳቤ
   *[other] በ{ $folders } አቃፊዎቹ ውስጥ በKatna የተከማቸ የዚህ መለያ ሁሉም ደብዳቤ
}
accounts-remove-outbox = በወጪ መልዕክት ሳጥን ውስጥ የሚጠብቁ መልዕክቶቹ
accounts-remove-settings = የተቀመጠው የይለፍ ቃሉ እና የKatna ቅንብሮቹ
accounts-delete-all-title = ሁሉም የKatna ውሂብ ይሰረዝ?
accounts-delete-all-confirm = ሁሉንም ሰርዝ
accounts-deleting = በመሰረዝ ላይ…
accounts-delete-all-accounts = እያንዳንዱ መለያ፣ እና በKatna የተከማቹ ሁሉም ደብዳቤዎች እና አባሪዎች
accounts-delete-all-contacts = እውቂያዎች፣ ቀን መቁጠሪያዎች እና የፍለጋ ማውጫው
accounts-delete-all-settings = ሁሉም ቅንብሮች፣ ፊርማዎች እና የቁልፍ ሰሌዳ አቋራጮች
accounts-delete-all-passwords = እያንዳንዱ የተቀመጠ የይለፍ ቃል
accounts-deleted-heading = ከዚህ ኮምፒውተር የሚሰረዙ፦
accounts-cannot-undo = ይህ ሊቀለበስ አይችልም።
accounts-server-delete-all = በደብዳቤ አገልጋዮችዎ ላይ ምንም አይቀየርም፦ ደብዳቤዎ እዚያ ይቆያል፣ እና መለያ እንደገና ማከል እንደገና ያወርደዋል። ከፋይሎች የመጣ ደብዳቤ ያለው በKatna ውስጥ ብቻ ነው፤ ፋይሎቹ አይነኩም።
accounts-server-local = ይህ ደብዳቤ ከፋይሎች የመጣ ስለሆነ ብቸኛው ቅጂ ያለው በKatna ነው። የመጣባቸው ፋይሎች አይነኩም፤ መልሰው ለማግኘት እንደገና ያስመጧቸው።
accounts-server-remove = በደብዳቤ አገልጋዩ ላይ ምንም አይቀየርም፦ ደብዳቤዎ እዚያ ይቆያል፣ እና መለያውን እንደገና ማከል እንደገና ያወርደዋል።
accounts-confirm-word = ሰርዝ
accounts-confirm-placeholder = «{ accounts-confirm-word }» ብለው ይተይቡ
accounts-confirm-prompt = ለማረጋገጥ «{ accounts-confirm-word }» ብለው ይተይቡ፦
accounts-cancel = ይቅር
