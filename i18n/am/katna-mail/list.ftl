# Katna Mail, Amharic (አማርኛ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

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
