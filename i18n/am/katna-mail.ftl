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
